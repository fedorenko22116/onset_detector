use std::env;

pub fn get_path() -> String {
    let args: Vec<String> = env::args().collect();
    return args.get(1)
        .expect("Path required")
        .clone();
}
