

/// A library for generating primitive waves
pub mod wave {
    pub mod sine;
}
//
// /// Ways to combine sounds into new sounds
// mod combinator {
//
// }


/// An abstract sound.
pub trait T {
    /// Convert the sound into a sequence of 32 bit samples.
    fn samples(&self) -> samples::T;
}

pub mod samples {
    /// a sequence of 32 bit samples
    ///
    /// Each sample indicates where a speaker's diaphragm should be at a point in time (specifically
    /// sample t occurs at t / 44100 seconds). Samples with larger magnitudes correspond to larger
    /// displacements of the the diaphragm.
    pub struct T(Vec<i32>);

    pub fn of(v : Vec<i32>) -> T {
        return T(v);
    }

    impl T {
        pub fn to_vec(self) -> Vec<i32> {
            return self.0;
        }

        pub fn len(&self) -> usize {
            return self.0.len();
        }
    }

}
