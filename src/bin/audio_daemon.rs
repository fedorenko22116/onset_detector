extern crate jsonrpc_core;
extern crate jsonrpc_http_server;
extern crate ordered_float;

use std::path::Path;
use onset_detection::extractor::Music;
use onset_detection::utils::get_path;
use onset_detection::detector::Detector;
use jsonrpc_core::{IoHandler, Value, Params, ErrorCode};
use jsonrpc_http_server::{ServerBuilder};
use ordered_float::OrderedFloat;

fn main() {
    let http = get_path();

    let mut io = IoHandler::new();

    io.add_method("beats", |params: Params| {
        let params = match params.parse() {
            Ok(Params::Map(param)) => param,
            _ => return Err(jsonrpc_core::Error::new(ErrorCode::InvalidParams)),
        };

        let name = params.get("name").unwrap().as_str().unwrap();

        let path = Path::new(&name);
        let music = Music::from_file(&path).unwrap();

        let samples = music.frames.samples();
        let res = samples.fft().beats();

        println!("Incoming beats request for '{}'", music.name);

        Ok(Value::String(serde_json::to_string(&res).unwrap()))
    });

    io.add_method("pcm", |params: Params| {
        let params = match params.parse() {
            Ok(Params::Map(param)) => param,
            _ => return Err(jsonrpc_core::Error::new(ErrorCode::InvalidParams)),
        };

        let name = params.get("name").unwrap().as_str().unwrap();

        let path = Path::new(&name);
        let music = Music::from_file(&path).unwrap();

        let samples = music.frames.samples();

        println!("Incoming PCM request for '{}'", music.name);

        Ok(Value::String(serde_json::to_string(&samples).unwrap()))
    });

    io.add_method("fft", |params: Params| {
        let params = match params.parse() {
            Ok(Params::Map(param)) => param,
            _ => return Err(jsonrpc_core::Error::new(ErrorCode::InvalidParams)),
        };

        let name = params.get("name").unwrap().as_str().unwrap();

        let path = Path::new(&name);
        let music = Music::from_file(&path).unwrap();

        let samples = music.frames.samples();
        let res = samples.fft();

        println!("Incoming FFT request for '{}'", music.name);

        Ok(Value::String(serde_json::to_string(&res).unwrap()))
    });

    io.add_method("serialize", |params: Params| {
        let params = match params.parse() {
            Ok(Params::Map(param)) => param,
            _ => return Err(jsonrpc_core::Error::new(ErrorCode::InvalidParams)),
        };

        let name = params.get("name").unwrap().as_str().unwrap();

        let path = Path::new(&name);
        let music = Music::from_file(&path).unwrap();

        let samples = music.frames.samples();
        let fft = samples.fft();

        let index = music.rate / 4;

        let mut res: (Vec<f64>, Vec<f64>) = Default::default();

        fft.chunks(index as usize).into_iter()
            .for_each(|el| res.0.push(el.into_iter().sum::<f64>() / el.len() as f64));

        let max = *res.0.iter().max_by_key(|n| OrderedFloat(n.abs())).unwrap();

        if max > 0. {
            res.0 = res.0.iter().map(|el| *el / max).collect();
        }

        let beats = fft.beats();

        beats.chunks(index as usize).into_iter()
            .for_each(|el| res.1.push(el.into_iter().sum::<f64>() / el.len() as f64));

        let max = *res.1.iter().max_by_key(|n| OrderedFloat(n.abs())).unwrap();

        if max > 0. {
            res.1 = res.1.iter().map(|el| *el / max).collect();
        }

        println!("Incoming FFT request for '{}'", music.name);

        Ok(Value::String(serde_json::to_string(&res).unwrap()))
    });

    let server = ServerBuilder::new(io)
        .threads(3)
        .start_http(&http.as_str().parse().unwrap())
        .unwrap();

    server.wait();
}
