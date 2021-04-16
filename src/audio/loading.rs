use hound::WavReader;
use minimp3::{Decoder, Frame, Error};
use claxon::FlacReader;

use std::fs::File;
use std::path::Path;
use std::ffi::OsStr;


pub fn load_wav_file(path: &str) -> (Vec<i16>, u32) {
    let mut reader =WavReader::open(path).unwrap();

    let sample_rate = reader.spec().sample_rate;
    let samples = reader.samples::<i16>().map(|x| x.unwrap() as i16).collect();
    (samples, sample_rate)
}

pub fn load_mp3_file(path: &str) -> (Vec<i16>, u32) {
    let mut decoder = Decoder::new(File::open(path).unwrap());

    // let mut samples: Option<Vec<i16>> = None;
    let mut samples = vec![];
    let mut sampling_rate: Option<u32> = None;
    loop {
        match decoder.next_frame() {
            Ok(Frame { mut data, sample_rate, channels, .. }) => {
                samples.append(&mut data);

                sampling_rate = Some(sample_rate as u32);
            },
            Err(Error::Eof) => break,
            Err(e) => panic!("{:?}", e),
        }
    }

    (samples, sampling_rate.unwrap())
}

pub fn load_flac_file(path: &str) -> (Vec<i16>, u32) {
    let mut reader = FlacReader::open(path).unwrap();

    let sample_rate = reader.streaminfo().sample_rate;

    let samples = reader.samples().map(|x| x.unwrap() as i16).collect();

    (samples, sample_rate)
}

fn get_extension_from_filename(filename: &str) -> Option<&str> {
    Path::new(filename)
        .extension()
        .and_then(OsStr::to_str)
}

pub fn load_file<'a, 'b>(path: &'a str, format: Option<&'b str>) -> (Vec<i16>, u32) {
    // let allowed_formats = vec!["wav", "mp3", "flac"];

    if format.is_none() {
        format = get_extension_from_filename(path);
    }

    match format.unwrap() {
        "wav" => load_wav_file(path),
        "mp3" => load_mp3_file(path),
        "flac" => load_flac_file(path),
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::f32::consts::PI;

    #[test]
    fn test_load_wav_file() {
        let filename = "./sine.wav";

        let (samples, sample_rate) = load_wav_file(&filename);

        assert_eq!(samples.len(), 44100);
        assert_eq!(sample_rate, 44100);
        assert_eq!(&samples[0..5], vec![0, 2052, 4097, 6126, 8130]);
    }

    #[test]
    fn test_load_mp3_file() {
        let filename = "./audio_files/clips/common_voice_ab_19904194.mp3";

        let (samples, sample_rate) = load_mp3_file(&filename);

        assert_eq!(samples.len(), 200448);
        assert_eq!(sample_rate, 48000);
    }

    #[test]
    fn test_load_flac_file() {
        let filename = "./audio_files/flac/example.flac";

        let (samples, sample_rate) = load_flac_file(&filename);

        assert_eq!(samples.len(), 93680);
        assert_eq!(sample_rate, 16000);
    }
}
