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

    let samples = music.frames.samples();
    let res = samples.fft();

    let mut x = Vec::new();
    let mut y = Vec::new();

    for i in 1..res.len() {
        x.push(i);
        y.push(*res.get(i).unwrap() as f64);
    }

    let res = res.peak();

    let mut x1 = Vec::new();
    let mut y1 = Vec::new();

    for i in 1..res.len() {
        x1.push(i);
        y1.push(*res.get(i).unwrap() as f64);
    }

    let mut fg = Figure::new();

    fg.axes2d()
        .set_title("Алгоритм звукової енергії", &[])
        .set_legend(Graph(0.5), Graph(0.5), &[], &[])
        .set_x_label("x - крок", &[])
        .set_y_label("y - діапазон", &[])
        .lines(&x, &y, &[Caption(&music.name)])
        .lines(&x1, &y1, &[Caption(&music.name)]);

    fg.show();
}
