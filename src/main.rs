extern crate feature_extractor;

use feature_extractor::audio::loading::load_wav_file as load;

fn main() {
    let (samples, sr) = load("./../audio_files/LJ001-0001.wav");

    println!("{:?}", samples);
    println!("{:?}", sr);
}
