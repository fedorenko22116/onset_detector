extern crate chrono;
extern crate textplots;

use std::path::Path;
use onset_detection::extractor::Music;
use chrono::prelude::*;
use std::time::Duration;
use textplots::{Chart, Plot, Shape};
use onset_detection::utils::get_path;

fn main() {
    println!("Onset beat detection!");
    println!("{} Starting processing..", now());

    let path_str = get_path();
    let path = Path::new(&path_str);
    let music = Music::from_file(&path)
        .expect("Error occured during parsing");

    let frames = music.frames;
    let samples = frames.per(&Duration::from_secs(100));

    println!();
    println!("Music:");
    println!("  name: {}", music.name);
    println!("  rate: {}", music.rate);
    println!("  duration: {} seconds", music.duration.as_secs());
    println!();
    println!("Approximate chart:");

    let mut data = Vec::new();

    for i in 1..samples.len() {
        data.push((i as f32 - (samples.len() / 2) as f32, *samples.get(i).unwrap() as f32));
    }

    Chart::default()
        .lineplot( Shape::Lines(&data) )
        .display();

    println!();
    println!("Samples: {:?}", samples);

    println!();
    println!("{} Finished", now());
}

fn now() -> String {
    Utc::now().format("[%H:%M:%S]").to_string()
}
