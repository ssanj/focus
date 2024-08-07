use std::path::PathBuf;
use std::fs::File;
use std::io::{Cursor, Read, Write};
use rodio::{source::Source, Decoder, OutputStream, Sink};

use crate::error::{FocusAction, FocusError};

const DEFAULT_SOUND: &'static [u8; 81970] = include_bytes!("assets/sounds/default.mp3");

// See: https://soundbible.com for free sounds
pub fn play_sound(sound_file: Option<PathBuf>) -> FocusAction {
  let (_stream, stream_handle) =
    OutputStream::try_default()
      .map_err(|e| FocusError::CouldNotOpenSoundDevice(e.to_string()))?;

  let sink =
    Sink::try_new(&stream_handle)
      .map_err(|e| FocusError::CouldNotCreateSinkForAudioPlayback(e.to_string()))?;

  let mut sound_buffer = vec![];

  let _ = sound_file
    .map(|file_name| {
      File::open(&file_name)
        .map_err(|e| FocusError::CouldNotOpenSoundFile(file_name.to_string_lossy().to_string(), e.to_string()))
        .map(|mut file| {
          file.read_to_end(&mut sound_buffer).expect("could not read sound file into buffer");
        })
    })
    .unwrap_or_else(|| {
      Ok(
          sound_buffer
            .write_all(DEFAULT_SOUND)
            .expect("could not write default sound file into buffer")
      )
    })?;


  let cursor = Cursor::new(sound_buffer);
  let source =
    Decoder::new(cursor)
      .map_err(|e| FocusError::CouldNotCreateDecoder(e.to_string()))?;
  sink.append(source.convert_samples::<f32>());
  sink.sleep_until_end();

  Ok(())
}
