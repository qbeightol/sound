
extern crate byteorder;
// extern crate num;

mod constants;

/// Modules for manipulating sound-related files
pub mod io {
    /// A module for creating single-channel .aif files from a vector of 32-bit samples.
    pub mod aif;
}

/// Libraries for generating sounds
pub mod sound;

//
// mod util {
//     mod complex;
// }


#[test]
fn write_sine() {
    use io::aif;
    use sound::wave::sine;
    let sine = sine::T { frequency : 440., duration : 1. };
    let aif = aif::T { audio = sine.samples()}
}

fn it_works() {
    // complex::it_works();
}
