extern crate jsonrpc_core;
extern crate jsonrpc_http_server;

use std::path::Path;
use onset_detection::extractor::Music;
use onset_detection::utils::get_path;
use onset_detection::detector::Detector;
use jsonrpc_core::{IoHandler, Value, Params, ErrorCode};
use jsonrpc_http_server::{ServerBuilder};
use std::time::Duration;

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

        let samples = music.frames.per(&Duration::from_secs(100));
        let res = samples.fft().beats(&50);

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

        let samples = music.frames.per(&Duration::from_secs(100));

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

        let samples = music.frames.per(&Duration::from_secs(100));
        let res = samples.fft();

        Ok(Value::String(serde_json::to_string(&res).unwrap()))
    });

    let server = ServerBuilder::new(io)
        .threads(3)
        .start_http(&http.as_str().parse().unwrap())
        .unwrap();

    server.wait();
}
