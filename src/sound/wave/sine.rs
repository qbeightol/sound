use constants;
use sound;
use sound::samples;
use std::u32;
// use std::f64;

struct T {
    frequency : f64,
    duration : f64
}

impl sound::T for T {
    fn samples(&self) -> samples::T {
        let mut data = Vec::with_capacity((constants::FRAMES_PER_SECOND as f64 * self.duration) as usize);
        for i in 0 .. data.capacity() {
            let unscaled_sample : f64 = (i as f64).sin();
            let scaled_sample = (unscaled_sample * (u32::MAX as f64)) as i32;
            data.push(scaled_sample);
        }
        return samples::of(data);
    }
}
