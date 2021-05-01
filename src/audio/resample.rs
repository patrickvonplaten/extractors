use crate::utils::parallelism::*;
use rubato::{Resampler, SincFixedIn, InterpolationType, InterpolationParameters, WindowFunction};

pub fn resample(samples: Vec<f32>, to_sample_rate: u32, from_sample_rate: u32) -> Vec<f32> {
    let params = InterpolationParameters {
        sinc_len: 256,
        f_cutoff: 0.95,
        interpolation: InterpolationType::Nearest,
        oversampling_factor: 160,
        window: WindowFunction::BlackmanHarris2,
    };
    let mut resampler = SincFixedIn::<f32>::new(
        (to_sample_rate as f32 / from_sample_rate as f32).into(),
        params,
        samples.len(),
        1,
    );

    let resampled_vectors = resampler.process(&vec![samples]).unwrap();
    (&resampled_vectors[0]).to_vec()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn upsample_wav() {
        let waves_in = vec![0.0f32; 100000];

        let waves_out = resample(waves_in, 48000, 16000);
        assert_eq!(waves_out.len(), 299614);
    }

    #[test]
    fn downsample_wav() {
        let waves_in = vec![0.0f32; 100000];

        let waves_out = resample(waves_in, 16000, 48000);
        assert_eq!(waves_out.len(), 2685);
    }
}
