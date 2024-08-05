use std::fmt;

#[allow(clippy::enum_variant_names)]
pub enum FocusError {
  CouldNotOpenSoundDevice(String),
  CouldNotOpenSoundFile(String, String),
  CouldNotCreateSinkForAudioPlayback(String),
  CouldNotCreateDecoder(String),
  CouldNotLoadDefaultFont(String),
  CouldNotOpenCustomFigletFont(String, String),
  CouldNotConvertStringToFiglet(String),
  CouldNotCreateNotification(String),
}


impl fmt::Display for FocusError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      let item = match self {
        FocusError::CouldNotOpenSoundDevice(error) => format!("Could not open hardware audio device due to: {error}"),
        FocusError::CouldNotOpenSoundFile(file, error) => format!("Could not open sound file: {file}, due to: {error}"),
        FocusError::CouldNotCreateSinkForAudioPlayback(error) => format!("Could not create an audio stream for audio playback due to: {error}"),
        FocusError::CouldNotCreateDecoder(error) => format!("Could not create an audio decoder for format due to: {error}"),
        FocusError::CouldNotLoadDefaultFont(error) => format!("Could not load default Figlet font due to: {error}"),
        FocusError::CouldNotOpenCustomFigletFont(file, error) => format!("Could not open custom Figlet font: {file}, due to: {error}"),
        FocusError::CouldNotConvertStringToFiglet(string) => format!("Could not covert string: {string} to Figlet"),
        FocusError::CouldNotCreateNotification(error) => format!("Could not create notification due to: {error}"),
    };

      write!(f, "{}", item)
    }
}

pub type FocusAction = Result<(), FocusError>;
