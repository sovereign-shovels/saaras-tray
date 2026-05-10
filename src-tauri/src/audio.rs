use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use hound::{WavSpec, WavWriter};
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;
use std::sync::{Arc, Mutex};

pub struct AudioRecorder {
    sample_rate: u32,
    channels: u16,
}

impl AudioRecorder {
    pub fn new() -> Self {
        Self {
            sample_rate: 16000,
            channels: 1,
        }
    }

    pub fn record_to_file(&self, path: &Path, duration_secs: u64) -> Result<(), String> {
        let host = cpal::default_host();
        let device = host
            .default_input_device()
            .ok_or("No input device available")?;
        let config = device
            .default_input_config()
            .map_err(|e| e.to_string())?;

        let spec = WavSpec {
            channels: self.channels,
            sample_rate: self.sample_rate,
            bits_per_sample: 16,
            sample_format: hound::SampleFormat::Int,
        };

        let writer = Arc::new(Mutex::new(Some(WavWriter::new(
            BufWriter::new(File::create(path).map_err(|e| e.to_string())?),
            spec,
        ).map_err(|e| e.to_string())?)));

        let writer_clone = writer.clone();

        let err_fn = |err| eprintln!("audio error: {}", err);
        let stream = device
            .build_input_stream(
                &config.config(),
                move |data: &[f32], _: &cpal::InputCallbackInfo| {
                    if let Ok(mut guard) = writer_clone.lock() {
                        if let Some(ref mut w) = *guard {
                            for &sample in data {
                                let sample_i16 = (sample * i16::MAX as f32) as i16;
                                let _ = w.write_sample(sample_i16);
                            }
                        }
                    }
                },
                err_fn,
                None,
            )
            .map_err(|e| e.to_string())?;

        stream.play().map_err(|e| e.to_string())?;
        std::thread::sleep(std::time::Duration::from_secs(duration_secs));
        drop(stream);

        if let Ok(mut guard) = writer.lock() {
            if let Some(w) = guard.take() {
                w.finalize().map_err(|e| e.to_string())?;
            }
        }

        Ok(())
    }
}
