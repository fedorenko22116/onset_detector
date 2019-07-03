use std::fmt::{Display, Formatter, self};
use std::error::Error;

#[derive(Debug, Clone)]
pub enum MusicError<'a> {
    EmptyAudio,
    ParseAudio,
    Other(&'a str)
}

impl<'a> Display for MusicError<'a> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", match self {
            MusicError::EmptyAudio => "Empty audio",
            MusicError::ParseAudio => "Can not parse a file",
            MusicError::Other(msg) => msg
        })
    }
}

impl<'a> Error for MusicError<'a> {
    fn description(&self) -> &str {
        match self {
            MusicError::EmptyAudio => "Empty audio",
            MusicError::ParseAudio => "Can not parse a file",
            MusicError::Other(msg) => msg
        }
    }

    fn cause(&self) -> Option<& dyn Error> {
        None
    }
}
