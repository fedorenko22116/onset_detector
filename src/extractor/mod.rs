pub mod error;
pub mod set;

use std::path::Path;
use std::fs::File;
use simplemad::{Decoder};
use std::error::Error;

use crate::extractor::error::MusicError;
use crate::extractor::set::FrameSet;
use std::time::Duration;

#[derive(Debug)]
pub struct Music {
    pub name: String,
    pub frames: FrameSet,
    pub duration: Duration,
    pub rate: u32,
    pub frame_duration: Duration,
    pub bpm: u32,
}

impl Music {
    pub fn from_file(path: &Path) -> Result<Self, Box<dyn Error>> {
        let file = File::open(path)?;
        let decoder = match Decoder::decode(file) {
            Ok(f) => f,
            Err(_e) => return Err(Box::new(MusicError::ParseAudio)),
        };
        let mut frames  = Vec::new();

        for item in decoder {
            match item {
                Err(_e) => continue,
                Ok(frame) => {
                    frames.push(frame);
                }
            }
        }

        let time: u64 = frames.iter().map(|f| f.duration.as_millis() as u64).sum();

        let f_frame = match frames.get(0) {
            Some(f) => f,
            None => return Err(Box::new(MusicError::EmptyAudio))
        };

        Ok(
            Music {
                name: String::from(
                    format!("{:?}", path.file_stem().unwrap()).trim_matches(&['"'] as &[_])
                ),
                frames: FrameSet::new(&frames)?,
                duration: Duration::from_millis(time),
                rate: f_frame.sample_rate,
                frame_duration: f_frame.duration,
                bpm: f_frame.bit_rate,
            }
        )
    }
}
