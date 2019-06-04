extern crate serde_json;
extern crate ordered_float;

use std::path::Path;
use onset_detection::extractor::Music;
use onset_detection::utils::get_path;
use onset_detection::detector::Detector;
use ordered_float::OrderedFloat;

fn main() {
    let path_str = get_path();
    let path = Path::new(&path_str);
    let music = Music::from_file(&path)
        .expect("Error occured during parsing");

    let fft = music.frames.samples().fft();

    let index = music.rate / 4;

    let mut res: Vec<f64> = Default::default();

    fft.iter().enumerate().step_by(index as usize)
        .for_each(|(_i, el)| res.push(*el));

    println!("{}", serde_json::to_string(&res).unwrap());

    let max = *res.iter().max_by_key(|n| OrderedFloat(n.abs())).unwrap();

    if max > 0. {
        res = res.iter().map(|el| *el / max).collect();
    }

    println!("{}", serde_json::to_string(&res).unwrap());
}
