extern crate gnuplot;

use std::path::Path;
use onset_detection::extractor::Music;
use gnuplot::{Figure, Graph, Caption, AxesCommon};
use onset_detection::utils::get_path;
use onset_detection::detector::Detector;

fn main() {
    let path_str = get_path();
    let path = Path::new(&path_str);
    let music = Music::from_file(&path)
        .expect("Error occured during parsing");

    let samples = music.frames.extract();
    let fft = samples.fft();

    let mut x = Vec::new();
    let mut y = Vec::new();

    for i in 1..fft.len() {
        x.push(i);
        y.push(*fft.get(i).unwrap() as f32);
    }

    let mut fg = Figure::new();

    fg.axes2d()
        .set_title("A plot", &[])
        .set_legend(Graph(0.5), Graph(0.5), &[], &[])
        .set_x_label("x", &[])
        .set_y_label("y^2", &[])
        .lines(&x, &y, &[Caption(&music.name)]);

    fg.show();
}
