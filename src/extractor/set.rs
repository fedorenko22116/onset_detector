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
        let samples = self.samples();
        let duration = duration.as_millis() as usize;
        let rate = self.data[0].sample_rate as usize;
        let pos = samples.len() / (duration * rate / 1000) as usize;

        (0..pos)
            .enumerate()
            .map(|(i, _val)| samples[i * (duration * rate / 1000)])
            .collect()
    }

    pub fn samples(&self) -> Vec<f64> {
        let mut res = Vec::new();

        for frame in self.data.iter() {
            let mut data: Vec<f64> = Default::default();

            for channel in frame.to_owned().samples.iter() {
                for (j, d) in channel.iter().enumerate() {
                    if let Some(_val) = data.get(j) {
                        data[j] += d.to_f64();
                    } else {
                        data.push(d.to_f64());
                    }
                }
            }

            for d in data.iter() {
                res.push(*d);
            }
        }

        res
    }
}
