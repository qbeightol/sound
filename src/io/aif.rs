/// DOCUMENTATION

// extern crate byteorder;
use byteorder::{ByteOrder, BigEndian};
use sound::samples;
use std::io;
use std::io::prelude::*;
use std::fs::File;


const NUM_CHANNELS : u32 = 1;
const SAMPLE_SIZE_BITS : u32 = 32;
const SAMPLE_SIZE_BYTES : u32 = 32 / 8;

/// A single-channel .aif audio file, where each sample is 32 bits.
pub struct T {
    audio : samples::T
}

// pub fn from_sound<S : Sound>(sound : S) -> Aif {
//     // I doubt that sounds will get allocated often---its probably okay to put them on the heap
//     return Aif { audio : sound.samples() };
// }

impl T {

    fn audio_len(&self) -> u32 {
        return self.audio.len() as u32;
    }

    /// returns the size of the form chunk's data measured in bytes.
    ///
    /// The form chunk's data includes both the form type (i.e. AIFF) and the nested chunks.
    /// Currently, the nested chunks are a common chunk and a sound chunk.
    pub fn form_chunk_size(&self) -> u32 {
        return 12 // ??? (I used to assume that this "FORM", form size, and "AIFF", but that
                  // contradicts the spec---those first 2 values wouldn't be included)
            + 8   // "COMM", size of common chunk
            + 18  // Common chunk
            + 8   // "SSND", size of chunk
            + self.audio_len() // size of audio bytes
            + 8;   // offset size, block size ?
    }



    // fn audio_iter(&self) -> Iterator

    /// appends a serialized version of the aif file to `file`.
    pub fn write_to_file(&self, file : &mut File) -> Result<(), io::Error> {

        let mut byte_buffer : [u8; 4] = [0; 4];

        // write form chunk ///////////////////////////////
        try!(file.write_all(b"FORM"));
        // it might seem like we should use native endian-ness, but it looks the aif standard
        // always uses big-endian representations
        BigEndian::write_u32(&mut byte_buffer, self.form_chunk_size());
        try!(file.write_all(&byte_buffer));
        try!(file.write_all(b"AIFF"));

        // write common chunk /////////////////////////////
        try!(file.write_all(b"COMM"));
        // size of common chunk
        BigEndian::write_u32(&mut byte_buffer, 18);
        try!(file.write_all(&byte_buffer));
        BigEndian::write_u32(&mut byte_buffer, NUM_CHANNELS);
        try!(file.write_all(&byte_buffer));
        BigEndian::write_u32(&mut byte_buffer, self.audio_len());
        try!(file.write_all(&byte_buffer));
        BigEndian::write_u32(&mut byte_buffer, SAMPLE_SIZE_BITS);
        try!(file.write_all(&byte_buffer));
        // this is the sample rate (44.1k) using an 'extended' format (I think this is an
        // Apple-developed float representation)
        try!(file.write_all(&[0x40, 0xe, 0xac, 0x44, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0]));

        // write sound chunk //////////////////////////////
        try!(file.write_all(b"SSND"));
        // size of sound chunk
        BigEndian::write_u32(&mut byte_buffer, SAMPLE_SIZE_BYTES * self.audio_len() + 8);
        try!(file.write_all(&byte_buffer));
        // offset size
        BigEndian::write_u32(&mut byte_buffer, 0);
        try!(file.write_all(&byte_buffer));
        // block size
        BigEndian::write_u32(&mut byte_buffer, 0);
        try!(file.write_all(&byte_buffer));

        // let
        // let iterator =
        //
        // for sample in self.audio.iter() {
        //     BigEndian::write_i32(&mut byte_buffer, *sample);
        //     file.write_all(&byte_buffer);
        // }

        return Ok(());

    }
}

#[test]
fn test_write_to_file() {
    let mut file = File::create("test_write_to_file.aif")
        .expect("unable to create 'test_write_to_file.aif'");
    let mut audio = Vec::new();
    audio.push(42);
    audio.push(17);
    let aif = T { audio : audio };
    aif.write_to_file(&mut file);

    let file = File::open("test_write_to_file.aif").expect("impossible");


    // println!("{}", file.seek(SeekFrom::Start(0)).expect("unable to seek"));

    for byte in file.bytes() {
        println!("{}", byte.unwrap());
    }


}
