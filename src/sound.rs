use std::fs::File;
use std::io::BufReader;
use rodio::{source::Source, Decoder, OutputStream, Sink};

pub fn play_sound() {

  // See: https://soundbible.com for free sounds
  // TODO: Handle errors safely
  let (_stream, stream_handle) = OutputStream::try_default().unwrap();
  let sink = Sink::try_new(&stream_handle).unwrap();

  // TODO: Include bytes for this sound in the executable
  let file = BufReader::new(File::open("/Users/sanj/Downloads/Computer_Magic-Microsift-1901299923.mp3").unwrap());
  let source = Decoder::new(file).unwrap();
  sink.append(source.convert_samples::<f32>());
  sink.sleep_until_end()
}
