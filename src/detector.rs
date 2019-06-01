use rustfft::num_complex::Complex;
use rustfft::{FFTplanner, FFTnum};
use rustfft::num_traits::Zero;
use std::ops::AddAssign;
use num;
use num::Float;
use std::fmt::Debug;

pub trait Detector {
    fn fft(&self) -> Self;
    fn peak(&self, period: &usize) -> Self;
    fn beats(&self, period: &usize) -> Self;
}

impl<T> Detector for Vec<T>
    where T: FFTnum + Float + Default + AddAssign + Debug + PartialEq {
    fn fft(&self) -> Self {
        let mut res: Self = Default::default();

        for chunk in self.chunks(1024) {
            let mut input: Vec<Complex<T>> = vec![Complex::zero(); 1024];
            let mut output: Vec<Complex<T>> = vec![Complex::zero(); 1024];

            chunk.iter().enumerate().for_each(|(i, val)| input[i] = Complex::from(val));

            let mut planner = FFTplanner::new(false);
            let fft = planner.plan_fft(1024);
            fft.process(&mut input, &mut output);

            res = [
                res,
                output.iter()
                    .map(|c| (c.im.powi(2) + c.re.powi(2)).sqrt() as T)
                    .collect::<Vec<T>>()[..output.len() / 2 + 1]
                    .to_vec()
            ].concat();
        }

        res
    }

    fn peak(&self, period: &usize) -> Self {
        let mut threshold: Vec<T> = Default::default();

        for el in self.chunks(self.len() / period).into_iter() {
            let mut p: T = Default::default();

            for val in el.iter() {
                p += *val;
            }

            for _val in el.iter() {
                threshold.push(p / num::NumCast::from(el.len()).unwrap() * num::NumCast::from(1.8).unwrap());
            }
        }

        threshold
    }

    fn beats(&self, period: &usize) -> Self {
        let fft = self.fft();
        let peak = fft.peak(period);
        let mut res: Self = Default::default();

        for i in 0..fft.len() {
            let val = *fft.get(i).unwrap();
            let diff = val - *peak.get(i).unwrap();

            if diff > num::NumCast::from(0.).unwrap() {
                res.push(*peak.get(i).unwrap());
            } else {
                res.push(num::NumCast::from(0.).unwrap());
            }
        }

        res
    }
}
