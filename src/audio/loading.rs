use hound;

pub fn load_wav_file(path: &str) -> (Vec<i16>, u32) {
    let mut reader = hound::WavReader::open(path).unwrap();
    let sample_rate = reader.spec().sample_rate;

    // Define later -> those define the type to be used when loading the file
    // println!("{:?}", reader.spec().sample_format);
    // println!("{:?}", reader.spec().bits_per_sample);
    
    let samples = reader.samples::<i16>().map(|x| x.unwrap() as i16).collect();
    (samples, sample_rate)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f32::consts::PI;

    #[test]
    fn test_load_wav_file() {
        let filename = "sine.wav";

        // # Make test write sound file
        // let spec = hound::WavSpec {
        //     channels: 1,
        //     sample_rate: 44100,
        //     bits_per_sample: 16,
        //     sample_format: hound::SampleFormat::Int,
        // };
        // let mut writer = hound::WavWriter::create(&filename, spec).unwrap();
        // for t in (0 .. 44100).map(|x| x as f32 / 44100.0) {
        //     let sample = (t * 440.0 * 2.0 * PI).sin();
        //     let amplitude = i16::MAX as f32;
        //     writer.write_sample((sample * amplitude) as i16).unwrap();
        // }

        let (samples, sample_rate) = load_wav_file(&filename);

        assert_eq!(samples.len(), 44100);
        assert_eq!(sample_rate, 44100);
    }
}
