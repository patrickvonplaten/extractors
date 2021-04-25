use crate::utils::parallelism::*;

use hound::WavReader;
use minimp3::{Decoder, Frame, Error};
use claxon::FlacReader;

use std::fs::File;
use std::path::Path;
use std::ffi::OsStr;

use std::io::BufReader;
use rodio::Decoder as RodDecoder;


pub fn load_wav_file(path: &str) -> (Vec<i16>, u32) {
    let mut reader = WavReader::open(path).unwrap();

    let sample_rate = reader.spec().sample_rate;
    let samples = reader.samples::<i16>().map(|x| x.unwrap()).collect();
    (samples, sample_rate)
}

pub fn load_mp3_file(path: &str) -> (Vec<i16>, u32) {
    let mut decoder = Decoder::new(File::open(path).unwrap());

    // let mut samples: Option<Vec<i16>> = None;
    let mut samples = vec![];
    let mut sampling_rate: Option<u32> = None;
    loop {
        match decoder.next_frame() {
            Ok(Frame { mut data, sample_rate, channels: _, .. }) => {
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

pub fn load_file(path: &str, format: Option<&str>) -> (Vec<i16>, u32) {
    // let allowed_formats = vec!["wav", "mp2", "flac"];

    let extension = if format.is_none() { get_extension_from_filename(path) } else { format };

    match extension.unwrap() {
        "wav" => load_wav_file(path),
        "mp3" => load_mp3_file(path),
        "flac" => load_flac_file(path),
        _ => panic!("Make sure that format or file extension is one of ['wav', 'mp3', 'flac']"),
    }
}

pub fn load_file_new(path: &str) -> Vec<i16> {
    let file = BufReader::new(File::open(path).unwrap());
    let samples: Vec<i16> = RodDecoder::new(file).unwrap().map(|x| x as i16).collect();
    samples
}

pub fn batch_load_file_new(paths: Vec<&str>, format: Option<&str>) -> Vec<Vec<i16>> {
    let audio_vectors = paths
        .into_maybe_par_iter()
        .map(|input| load_file_new(input))
        .collect::<Vec<Vec<i16>>>();

    audio_vectors
}

pub fn batch_load_file(paths: Vec<&str>, format: Option<&str>) -> Vec<Vec<i16>> {
    let audio_vectors = paths
        .into_maybe_par_iter()
        .map(|input| {
            let (samples, _) = load_file(input, format);
            samples
        })
        .collect::<Vec<Vec<i16>>>();

    audio_vectors
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_wav_file() {
        let filename = "./sine.wav";

        let (samples, sample_rate) = load_file(&filename, None);

        assert_eq!(samples.len(), 44100);
        assert_eq!(sample_rate, 44100);
        assert_eq!(&samples[0..5], vec![0, 2052, 4097, 6126, 8130]);
    }

    #[test]
    fn test_load_mp3_file() {
        let filename = "./audio_files/clips/common_voice_ab_19904194.mp3";

        let (samples, sample_rate) = load_file(&filename, None);

        assert_eq!(samples.len(), 200448);
        assert_eq!(sample_rate, 48000);
    }

    #[test]
    fn test_load_flac_file() {
        let filename = "./audio_files/flac/example.flac";

        let (samples, sample_rate) = load_file(&filename, None);

        assert_eq!(samples.len(), 93680);
        assert_eq!(sample_rate, 16000);
    }
}
