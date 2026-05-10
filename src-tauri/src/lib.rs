use enigo::{Enigo, Keyboard, Settings};
use std::sync::Arc;
use tauri::menu::{Menu, MenuItem};
use tauri::tray::TrayIconBuilder;
use tauri::{Emitter, Manager, State};
use tauri_plugin_clipboard_manager::ClipboardExt;
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};
use tokio::sync::Mutex;

mod audio;
mod config;
mod stt;

use audio::AudioRecorder;
use config::Config;
use stt::{LocalWhisperProvider, SaarasProvider, SharedProvider, SttResult};

struct AppState {
    provider: SharedProvider,
    config: Config,
    recording: Arc<Mutex<bool>>,
}

#[tauri::command]
async fn get_config(state: State<'_, AppState>) -> Result<Config, String> {
    Ok(state.config.clone())
}

#[tauri::command]
async fn get_provider_name(state: State<'_, AppState>) -> Result<String, String> {
    let guard = state.provider.lock().await;
    Ok(guard.name().to_string())
}

#[tauri::command]
async fn toggle_dictation(state: State<'_, AppState>, app: tauri::AppHandle) -> Result<SttResult, String> {
    let mut rec = state.recording.lock().await;
    if *rec {
        return Err("Already recording".into());
    }
    *rec = true;
    drop(rec);

    // Record audio to temp file
    let temp_dir = std::env::temp_dir();
    let audio_path = temp_dir.join("saaras-tray-recording.wav");

    let recorder = AudioRecorder::new();
    match recorder.record_to_file(&audio_path, 5) {
        Ok(_) => {}
        Err(e) => {
            let mut rec = state.recording.lock().await;
            *rec = false;
            return Err(format!("Recording failed: {}", e));
        }
    }

    // Transcribe
    let guard = state.provider.lock().await;
    let result = match guard.transcribe(&audio_path).await {
        Ok(r) => r,
        Err(e) => {
            let mut rec = state.recording.lock().await;
            *rec = false;
            return Err(format!("Transcription failed: {}", e));
        }
    };
    drop(guard);

    // Copy to clipboard and paste
    let text = result.text.clone();
    let _ = app.clipboard().write_text(text.clone());
    std::thread::sleep(std::time::Duration::from_millis(100));

    // Paste using enigo in a scoped block so it's dropped before await
    {
        let mut enigo = Enigo::new(&Settings::default()).map_err(|e| e.to_string())?;

        #[cfg(target_os = "macos")]
        {
            enigo.key(enigo::Key::Meta, enigo::Direction::Press).map_err(|e| e.to_string())?;
            enigo.key(enigo::Key::Unicode('v'), enigo::Direction::Click).map_err(|e| e.to_string())?;
            enigo.key(enigo::Key::Meta, enigo::Direction::Release).map_err(|e| e.to_string())?;
        }

        #[cfg(not(target_os = "macos"))]
        {
            enigo.key(enigo::Key::Control, enigo::Direction::Press).map_err(|e| e.to_string())?;
            enigo.key(enigo::Key::Unicode('v'), enigo::Direction::Click).map_err(|e| e.to_string())?;
            enigo.key(enigo::Key::Control, enigo::Direction::Release).map_err(|e| e.to_string())?;
        }
    }

    let mut rec = state.recording.lock().await;
    *rec = false;

    Ok(result)
}

#[tauri::command]
async fn is_recording(state: State<'_, AppState>) -> Result<bool, String> {
    Ok(*state.recording.lock().await)
}

pub fn run() {
    let config = Config::load("saaras-tray");
    let hotkey_str = config.hotkey.clone().unwrap_or_else(|| "CmdOrCtrl+Shift+S".into());

    let provider: Box<dyn stt::SttProvider> = if config.provider_name.as_deref() == Some("local") {
        Box::new(LocalWhisperProvider)
    } else {
        Box::new(SaarasProvider::new(
            config.endpoint.clone().unwrap_or_default(),
            config.api_key(),
            config.language.clone().unwrap_or_default(),
            config.codemix.unwrap_or(true),
        ))
    };

    let state = AppState {
        provider: Arc::new(Mutex::new(provider)),
        config,
        recording: Arc::new(Mutex::new(false)),
    };

    tauri::Builder::default()
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_global_shortcut::Builder::new()
            .with_handler(|app, _shortcut, event| {
                if event.state == ShortcutState::Pressed {
                    let app_clone = app.clone();
                    tauri::async_runtime::spawn(async move {
                        let _ = app_clone.emit("dictation-triggered", ());
                    });
                }
            })
            .build())
        .plugin(tauri_plugin_positioner::init())
        .manage(state)
        .invoke_handler(tauri::generate_handler![
            get_config,
            get_provider_name,
            toggle_dictation,
            is_recording,
        ])
        .setup(move |app| {
            // Tray icon
            let quit_i = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
            let settings_i = MenuItem::with_id(app, "settings", "Settings", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&settings_i, &quit_i])?;

            let _tray = TrayIconBuilder::new()
                .icon(app.default_window_icon().unwrap().clone())
                .menu(&menu)
                .show_menu_on_left_click(true)
                .on_menu_event(|app, event| match event.id.as_ref() {
                    "quit" => {
                        app.exit(0);
                    }
                    "settings" => {
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                    }
                    _ => {}
                })
                .build(app)?;

            // Global shortcut
            let _ = app.global_shortcut().register(hotkey_str.as_str());

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
