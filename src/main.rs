extern crate feature_extractor;
use std::env;

use feature_extractor::audio::loading::batch_load_file as load;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_names: Vec<&str> = args[1].split(" ").collect();

    let samples = load(file_names, None);

    let mut i = 1;
    for vec in samples {
         println!("Len {}: {}", i, vec.len());
         i += 1;
    }
}
