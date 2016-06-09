// extern crate byteorder;
use std::io::SeekFrom;
use std::io::prelude::*;
use std::fs::File;
use byteorder::{ByteOrder, BigEndian};


/// A single-channel .aif audio file, where each sample is 32 bits.
struct Aif {
    audio : Vec<u32>
}

impl Aif {

    /// returns the size of the form chunk's data (measured in bytes). The form chunk's data
    /// includes both the form type (i.e. AIFF) and the nested chunks (in this case, just the
    /// common chunk and the data chunk).
    fn form_chunk_size(&self) -> u32 {
        return 12 // ??? (I used to assume that this "FORM", form size, and "AIFF", but that
                  // contradicts the spec---those first 2 values wouldn't be included)
            + 8   // "COMM", size of common chunk
            + 18  // Common chunk
            + 8   // "SSND", size of chunk
            + self.audio.len() as u32 // size of audio bytes
            + 8;   // offset size, block size ?
    }

    /// appends the a serialized version of itself to file.
    fn write_to_file(&self, file : &mut File) {
        // write form chunk
        file.write_all(b"FORM");

        let mut total_size_as_bytes : [u8; 4] = [0; 4];
        // it looks the aif standard always uses big endian representations
        BigEndian::write_u32(&mut total_size_as_bytes, self.form_chunk_size());
        file.write_all(&total_size_as_bytes);
        file.write_all(b"AIFF");



        // write common chunk

        // write sound chunk
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
