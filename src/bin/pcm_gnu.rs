extern crate gnuplot;

use std::path::Path;
use onset_detection::extractor::Music;
use gnuplot::{Figure, Graph, Caption, AxesCommon};
use onset_detection::utils::get_path;

fn main() {
    let path_str = get_path();
    let path = Path::new(&path_str);
    let music = Music::from_file(&path)
        .expect("Error occured during parsing");

    let samples = music.frames.samples();

    let mut x = Vec::new();
    let mut y = Vec::new();

    for i in 1..samples.len() {
        x.push((i + 10) as f32);
        y.push(*samples.get(i).unwrap() as f32);
    }

    let mut fg = Figure::new();

    fg.axes2d()
        .set_title("PCM", &[])
        .set_legend(Graph(0.2), Graph(0.2), &[], &[])
        .set_x_label("x - час", &[])
        .set_y_label("y - амплітуда семплу", &[])
        .lines(&x, &y, &[Caption(&music.name)]);

    fg.show();
}
