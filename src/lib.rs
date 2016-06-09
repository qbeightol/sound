
extern crate byteorder;
extern crate num;

/// Modules for manipulating sound-related files
pub mod io {
    /// A module for creating single-channel .aif files from a vector of 32-bit samples
    pub mod aif;
}

/// Libraries for generating sounds
mod sound {
    /// A library for generating primitive waves
    mod wave {

    }

    /// Ways to combine sounds into new sounds
    mod combinator {

    }
}
//
// mod util {
//     mod complex;
// }


#[test]
fn it_works() {
    // complex::it_works();
}
