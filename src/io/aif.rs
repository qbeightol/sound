/// DOCUMENTATION

// extern crate byteorder;
use std::io::SeekFrom;
use std::io::prelude::*;
use std::fs::File;
use byteorder::{ByteOrder, BigEndian};

const NUM_CHANNELS : u32 = 1;
const SAMPLE_SIZE_BITS : u32 = 32;
const SAMPLE_SIZE_BYTES : u32 = 32 / 8;

/// A single-channel .aif audio file, where each sample is 32 bits.
pub struct Aif {
    audio : Vec<u32>
}

impl Aif {

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
            + self.audio.len() as u32 // size of audio bytes
            + 8;   // offset size, block size ?
    }

    /// appends a serialized version of the aif file to `file`.
    pub fn write_to_file(&self, file : &mut File) {

        let mut byte_buffer : [u8; 4] = [0; 4];

        // write form chunk ///////////////////////////////
        file.write_all(b"FORM");
        // it might seem like we should use native endian-ness, but it looks the aif standard
        // always uses big-endian representations
        BigEndian::write_u32(&mut byte_buffer, self.form_chunk_size());
        file.write_all(&byte_buffer);
        file.write_all(b"AIFF");

        // write common chunk /////////////////////////////
        file.write_all(b"COMM");
        // size of common chunk
        BigEndian::write_u32(&mut byte_buffer, 18);
        file.write_all(&byte_buffer);
        BigEndian::write_u32(&mut byte_buffer, NUM_CHANNELS);
        file.write_all(&byte_buffer);
        BigEndian::write_u32(&mut byte_buffer, self.audio.len() as u32);
        file.write_all(&byte_buffer);
        BigEndian::write_u32(&mut byte_buffer, SAMPLE_SIZE_BITS);
        file.write_all(&byte_buffer);
        // this is the sample rate (44.1k) using an 'extended' format (I think this is an
        // Apple-developed float representation)
        file.write_all(&[0x40, 0xe, 0xac, 0x44, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0]);

        // write sound chunk //////////////////////////////
        file.write_all(b"SSND");
        // size of sound chunk
        BigEndian::write_u32(&mut byte_buffer, SAMPLE_SIZE_BYTES * self.audio.len() as u32 + 8);
        file.write_all(&byte_buffer);
        // offset size
        BigEndian::write_u32(&mut byte_buffer, 0);
        file.write_all(&byte_buffer);
        // block size
        BigEndian::write_u32(&mut byte_buffer, 0);
        file.write_all(&byte_buffer);



    }
}

#[test]
fn test_write_to_file() {
    let mut file = File::create("test_write_to_file.aif")
        .expect("unable to create 'test_write_to_file.aif'");
    let mut audio = Vec::new();
    audio.push(42);
    audio.push(17);
    let aif = Aif { audio : audio };
    aif.write_to_file(&mut file);

    let file = File::open("test_write_to_file.aif").expect("impossible");


    // println!("{}", file.seek(SeekFrom::Start(0)).expect("unable to seek"));

    for byte in file.bytes() {
        println!("{}", byte.unwrap());
    }


}
