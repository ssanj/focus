use std::path::{Path, PathBuf};
use std::fs::File;
use std::io::BufReader;
use rodio::{source::Source, Decoder, OutputStream, Sink};

pub fn play_sound(sound_file: Option<PathBuf>) {

  // See: https://soundbible.com for free sounds
  // TODO: Handle errors safely
  let (_stream, stream_handle) = OutputStream::try_default().unwrap();
  let sink = Sink::try_new(&stream_handle).unwrap();

  let sound_to_play = sound_file.unwrap_or_else(|| Path::new("src/assets/sounds/default.mp3").to_owned());

  let file = BufReader::new(File::open(sound_to_play).unwrap());
  let source = Decoder::new(file).unwrap();
  sink.append(source.convert_samples::<f32>());
  sink.sleep_until_end()
}
