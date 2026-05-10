import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';

const statusDot = document.getElementById('status-dot');
const statusText = document.getElementById('status-text');
const hotkeyDisplay = document.getElementById('hotkey-display');
const languageSelect = document.getElementById('language');
const codemixCheck = document.getElementById('codemix');
const providerSelect = document.getElementById('provider');

async function loadConfig() {
  try {
    const cfg = await invoke('get_config');
    if (cfg.language) languageSelect.value = cfg.language;
    if (cfg.codemix !== undefined) codemixCheck.checked = cfg.codemix;
    if (cfg.hotkey) hotkeyDisplay.textContent = cfg.hotkey.replace('CmdOrCtrl+', 'Cmd+');
  } catch (e) {
    console.error('Failed to load config:', e);
  }
}

listen('dictation-triggered', async () => {
  statusDot.className = 'status-dot recording';
  statusText.textContent = 'Recording... speak now';

  try {
    const result = await invoke('toggle_dictation');
    statusDot.className = 'status-dot ready';
    const hotkey = hotkeyDisplay.textContent;
    statusText.innerHTML = `Pasted: "${result.text.substring(0, 40)}${result.text.length > 40 ? '...' : ''}" — press <span class="hotkey">${hotkey}</span> to dictate`;
  } catch (e) {
    statusDot.className = 'status-dot ready';
    statusText.textContent = `Error: ${e}`;
  }
});

loadConfig();
