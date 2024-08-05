use std::path::{Path, PathBuf};
use std::fs::File;
use std::io::BufReader;
use rodio::{source::Source, Decoder, OutputStream, Sink};

use crate::error::{FocusAction, FocusError};

pub fn play_sound(sound_file: Option<PathBuf>) -> FocusAction {

  // See: https://soundbible.com for free sounds
  let (_stream, stream_handle) =
    OutputStream::try_default()
      .map_err(|e| FocusError::CouldNotOpenSoundDevice(e.to_string()))?;

  let sink =
    Sink::try_new(&stream_handle)
      .map_err(|e| FocusError::CouldNotCreateSinkForAudioPlayback(e.to_string()))?;

  let sound_to_play = sound_file.unwrap_or_else(|| Path::new("src/assets/sounds/default.mp3").to_owned());

  let sound =
    File::open(&sound_to_play)
      .map_err(|e| FocusError::CouldNotOpenSoundFile(sound_to_play.to_string_lossy().to_string(), e.to_string()))?;

  let file = BufReader::new(sound);
  let source =
    Decoder::new(file)
      .map_err(|e| FocusError::CouldNotCreateDecoder(e.to_string()))?;
  sink.append(source.convert_samples::<f32>());
  sink.sleep_until_end();

  Ok(())
}
