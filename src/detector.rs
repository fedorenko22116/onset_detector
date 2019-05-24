use rustfft::num_complex::Complex;
use rustfft::{FFTplanner, FFTnum, num_traits::Float};
use rustfft::num_traits::Zero;
use std::slice::Chunks;

pub trait Detector {
    fn fft(&self) -> Self;
    fn peak(&self) -> Self;
}

impl<T> Detector for Vec<T>
    where T: FFTnum + Float {
    fn fft(&self) -> Self {
        let mut input: Vec<Complex<T>> = self.iter().map(|n| Complex::from(n)).collect();
        let mut output: Vec<Complex<T>> = vec![Complex::zero(); input.len()];

        let mut planner = FFTplanner::new(false);
        let fft = planner.plan_fft(input.len());
        fft.process(&mut input, &mut output);

        output.iter()
            .map(|c| (c.im.powi(2) + c.re.powi(2).sqrt()) as T)
            .collect::<Vec<T>>()[..output.len() / 2]
            .to_vec()
    }

    fn peak(&self) -> Self {
        unimplemented!()
    }
}
