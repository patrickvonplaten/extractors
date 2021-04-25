extern crate feature_extractor;

use std::env;
use std::fs::read_dir;

use feature_extractor::audio::loading::batch_load_file as load;

fn main() {
    let args: Vec<String> = env::args().collect();

    let dir_path: &str = &args[1];

    let paths: Vec<String> = read_dir(dir_path).unwrap().map(|x| x.unwrap().path().display().to_string()).collect();
    let files: Vec<&str> = paths.iter().map(|x| &x[..]).collect();


    let samples = load(files, None);
}
