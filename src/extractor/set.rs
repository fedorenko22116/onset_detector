use simplemad::Frame;
use std::time::Duration;
use std::error::Error;
use crate::extractor::error::MusicError;

#[derive(Debug)]
pub struct FrameSet {
    pub data: Vec<Frame>,
    pub duration: Duration,
}

impl FrameSet {
    pub fn new(samples: &Vec<Frame>) -> Result<Self, Box<dyn Error>> {
        let frame = match samples.get(0) {
            Some(s) => s,
            None => return Err(Box::new(MusicError::EmptyAudio))
        };

        Ok(
            FrameSet {
                data: samples.to_vec(),
                duration: frame.duration,
            }
        )
    }

    pub fn per(&self, duration: &Duration) -> Vec<f64> {
        let mut res = Vec::new();
        let frame_duration = self.duration.as_millis() as usize;

        assert!(frame_duration >= self.data[0].duration.as_millis() as usize);

        let pos = ((frame_duration as f64 / duration.as_millis() as f64) * self.data.len() as f64) as usize;

        for i in 0..pos {
            let channels = self.data[i * (self.data.len() / pos)].to_owned().samples;
            let mut sample = 0.;

            for channel in channels.iter() {
                sample += channel[0].to_f64()
            }

            res.push(sample / channels.len() as f64);
        }

        res.push(0.0);

        res
    }

    pub fn extract(&self) -> Vec<f64> {
        self.per(&self.data.get(0).unwrap().duration)
    }
}
