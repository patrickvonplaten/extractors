use crate::utils::parallelism::*;
use crate::audio::resample::resample;

use hound::WavReader;
use minimp3::{Decoder, Frame, Error};
use claxon::FlacReader;

use std::fs::File;
use std::path::Path;
use std::ffi::OsStr;


pub fn load_wav_file(path: &str) -> (Vec<f32>, usize) {
    let mut reader = WavReader::open(path).unwrap();

    let sample_rate = reader.spec().sample_rate as usize;
    let samples = reader.samples::<i16>().map(|x| x.unwrap() as f32).collect();
    (samples, sample_rate)
}

pub fn load_mp3_file(path: &str) -> (Vec<f32>, usize) {
    let mut decoder = Decoder::new(File::open(path).unwrap());

    // let mut samples: Option<Vec<f32>> = None;
    let mut samples = vec![];
    let mut sampling_rate: Option<usize> = None;
    loop {
        match decoder.next_frame() {
            Ok(Frame { mut data, sample_rate, channels: _, .. }) => {
                samples.append(&mut data);

                sampling_rate = Some(sample_rate as usize);
            },
            Err(Error::Eof) => break,
            Err(e) => panic!("{:?}", e),
        }
    }

    let casted_samples: Vec<f32> = samples.iter().map(|x| *x as f32).collect();

    (casted_samples, sampling_rate.unwrap())
}

pub fn load_flac_file(path: &str) -> (Vec<f32>, usize) {
    let mut reader = FlacReader::open(path).unwrap();

    let sample_rate = reader.streaminfo().sample_rate as usize;

    let samples = reader.samples().map(|x| x.unwrap() as f32).collect();

    (samples, sample_rate)
}

fn get_extension_from_filename(filename: &str) -> Option<&str> {
    Path::new(filename)
        .extension()
        .and_then(OsStr::to_str)
}

pub fn load_file(path: &str, to_sample_rate: Option<usize>) -> (Vec<f32>, usize) {
    // let allowed_formats = vec!["wav", "mp2", "flac"];

    let extension = get_extension_from_filename(path);

    let (mut samples, mut sample_rate) = match extension.unwrap() {
        "wav" => load_wav_file(path),
        "mp3" => load_mp3_file(path),
        "flac" => load_flac_file(path),
        _ => panic!("Make sure that format or file extension is one of ['wav', 'mp3', 'flac']"),
    };

    if to_sample_rate.is_some() {
        samples = resample(samples, to_sample_rate.unwrap() as u32, sample_rate as u32);
        sample_rate = to_sample_rate.unwrap();
    }

    (samples, sample_rate)
}

pub fn batch_load_file(paths: Vec<&str>, to_sample_rate: Option<usize>) -> Vec<Vec<f32>> {
    let audio_vectors = paths
        .into_maybe_par_iter()
        .map(|input| {
            let (samples, _) = load_file(input, to_sample_rate);
            samples
        })
        .collect::<Vec<Vec<f32>>>();

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
        assert_eq!(&samples[0..5], vec![0.0, 2052.0, 4097.0, 6126.0, 8130.0]);
    }

    #[test]
    fn test_load_mp3_file() {
        let filename = "./audio_files/mp3/common_voice_ab_19904194.mp3";

        let (samples, sample_rate) = load_file(&filename, None);

        assert_eq!(samples.len(), 200448);
        assert_eq!(sample_rate, 48000);
    }

    #[test]
    fn test_load_flac_file() {
        let filename = "./audio_files/flac/1272-128104-0000.flac";

        let (samples, sample_rate) = load_file(&filename, None);

        assert_eq!(samples.len(), 93680);
        assert_eq!(sample_rate, 16000);
    }

    #[test]
    fn test_load_with_upsample() {
        let filename = "./audio_files/flac/1272-128104-0000.flac";

        let (samples, sample_rate) = load_file(&filename, Some(48000));

        assert_eq!(samples.len(), 31227);
        assert_eq!(sample_rate, 48000);
    }

    #[test]
    fn test_load_with_downsample() {
        let filename = "./audio_files/mp3/common_voice_ab_19904194.mp3";

        let (samples, sample_rate) = load_file(&filename, Some(16000));

        assert_eq!(samples.len(), 601344);
        assert_eq!(sample_rate, 16000);
    }
}
