use crate::utils::parallelism::*;
use samplerate::{convert, ConverterType};


pub fn resample(samples: Vec<f32>, from_sample_rate: u32, to_sample_rate: u32) -> Vec<f32> {
    return convert(from_sample_rate, to_sample_rate, 1, ConverterType::SincBestQuality, &samples).unwrap();
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn upsample_wav() {
        let waves_in = vec![0.0f32; 100000];

        let waves_out = resample(waves_in, 16000, 48000);
        assert_eq!(waves_out.len(), 300000);
    }

    #[test]
    fn downsample_wav() {
        let waves_in = vec![0.0f32; 100000];

        let waves_out = resample(waves_in, 48000, 16000);
        
        assert_eq!(waves_out.len(), 33334);
    }
}
