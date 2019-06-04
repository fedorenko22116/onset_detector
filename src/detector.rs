use rustfft::num_complex::Complex;
use rustfft::{FFTplanner, FFTnum};
use rustfft::num_traits::Zero;
use std::ops::{AddAssign, DivAssign};
use num;
use num::Float;
use std::fmt::Debug;
use std::cmp;

pub trait Detector {
    fn fft(&self) -> Self;
    fn peak(&self) -> Self;
    fn beats(&self) -> Self;
}

impl<T> Detector for Vec<T>
    where T: FFTnum + Float + Default + AddAssign + DivAssign + Debug + PartialEq {
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

    fn peak(&self) -> Self {
        const TWIN_SIZE: isize = 10;
        const MULTIPLIER: f64 = 1.5f64;

        let mut spectral_flux: Vec<T> = Default::default();
        let mut right: Vec<T> = Default::default();

        for chunk in self.chunks(513).into_iter() {
            let left = right;
            right = chunk.to_vec();

            let mut flux: T = num::NumCast::from(0).unwrap();

            for (i, _val) in right.iter().enumerate() {
                match (left.get(i), right.get(i)) {
                    (Some(l), Some(r)) => match *r - *l {
                        val if val > num::NumCast::from(0.).unwrap() => flux += val,
                        _ => continue,
                    }
                    _ => continue,
                }
            }

            spectral_flux.push(flux);
        }


        let mut threshold: Vec<T> = Default::default();

        for (i, _val) in spectral_flux.iter().enumerate() {
            let start = cmp::max(0, i as isize - TWIN_SIZE);
            let end = cmp::max((spectral_flux.len() - 1) as isize, i as isize + TWIN_SIZE);

            let mut mean: T = num::NumCast::from(0).unwrap();

            for j in start..end {
                if let Some(val) = spectral_flux.get(j as usize) {
                    mean += *val;
                }
            }

            mean /= num::NumCast::from(end - start).unwrap();

            for _i in 0..513 {
                threshold.push(mean * num::NumCast::from(MULTIPLIER).unwrap())
            }
        }

        threshold
    }

    fn beats(&self) -> Self {
        let fft = self.to_owned();
        let peak = fft.peak();
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
