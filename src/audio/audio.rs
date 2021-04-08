pub struct Audio {
    data: Vec<f32>,
    sampling_rate: u32,
}


impl Audio {

    pub fn new(
        data: Vec<f32>,
        sampling_rate: u32,
    ) -> Self {
        Audio {
            data,
            sampling_rate,
        }
    }

    pub fn from_file(path: &str) -> Self {
        Audio {
            data,
            sampling_rate,
        }
    }

    pub fn resample(&self, target_sr: u32) -> Self {
        Audio {
            data,
            target_sr,
        }
    }
}


pub fn batch_load(paths: Vec<&str>) -> Vec<Audio> {

}

pub fn batch_resample(audios: &Vec<Audio>, target_sr: u32) -> Vec<Audio> {

}
