extern crate extractors;

// use std::env;
// use std::fs::read_dir;

// use extractors::audio::loading::batch_load_file as load;
use rubato::{Resampler, SincFixedIn, InterpolationType, InterpolationParameters, WindowFunction};

fn main() {
//    let args: Vec<String> = env::args().collect();
//
//    let dir_path: &str = &args[1];
//
//    let paths: Vec<String> = read_dir(dir_path).unwrap().map(|x| x.unwrap().path().display().to_string()).collect();
//    let files: Vec<&str> = paths.iter().map(|x| &x[..]).collect();
//
//
//    let samples = load(files, None);
    let params = InterpolationParameters {
        sinc_len: 256,
        f_cutoff: 0.95,
        interpolation: InterpolationType::Nearest,
        oversampling_factor: 160,
        window: WindowFunction::BlackmanHarris2,
    };
    let mut resampler = SincFixedIn::<f64>::new(
        16000 as f64 / 48000 as f64,
        params,
        1024,
        2,
    );

    let waves_in = vec![vec![0.0f64; 1024];2];
    let waves_out = resampler.process(&waves_in).unwrap();
}
