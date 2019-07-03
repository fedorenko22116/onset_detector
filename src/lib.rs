extern crate simplemad;
extern crate rustfft;
extern crate num;
extern crate chrono;

use chrono::prelude::*;

pub mod detector;
pub mod extractor;
pub mod utils;

#[macro_export]
macro_rules! printtm {
    () => (println!("{}", onset_detection::now()));
    ($($arg:tt)*) => ({
        print!("{} ", onset_detection::now());
        $(print!("{}", $arg);)*
        println!();
    })
}

pub fn now() -> String {
    Utc::now().format("[%H:%M:%S]").to_string()
}
