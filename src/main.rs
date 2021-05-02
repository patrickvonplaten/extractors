// An example of using `sample` to efficiently perform decent quality sample rate conversion on a
// WAV file entirely on the stack.

use std::env;
use std::fs::read_dir;

use extractors::audio::loading::batch_load_file as load;


fn main() {
   let args: Vec<String> = env::args().collect();

   let dir_path: &str = &args[1];

   let paths: Vec<String> = read_dir(dir_path).unwrap().map(|x| x.unwrap().path().display().to_string()).collect();
   let files: Vec<&str> = paths.iter().map(|x| &x[..]).collect();

    let samples = &load(files, Some(16000))[0];

    let normalized_samples: Vec<f32> = samples.iter().map(|x| *x / 32767.0).collect();

    println!("Sample Len {}", normalized_samples.len());
}
