use core::str;
use std::{fs::File, io::Read};

#[allow(dead_code)]
pub struct Chunk {
    length: u32,
    chunk_type: [u8; 4],
    data: Vec<u8>,
    crc: [u8; 4],
}

impl Chunk {
    pub fn new(file: &mut File) -> Result<Self, Box<dyn std::error::Error>> {
        let mut length_bytes = [0u8; 4];
        match file.read_exact(&mut length_bytes) {
            Err(err) => return Err(format!("Error reading Chunk's Length: {}", err).into()),
            _ => (),
        }
        // Convert length to u32
        let length: u32 = u32::from_be_bytes(length_bytes);

        // Read Chunk Type
        let mut chunk_type = [0u8; 4];
        match file.read_exact(&mut chunk_type) {
            Err(err) => return Err(format!("Error reading Chunk Type: {}", err).into()),
            _ => (),
        }

        // Read data, using the length we obtained before
        let mut data: Vec<u8> = vec![0; length as usize];
        match file.read_exact(&mut data) {
            Err(err) => return Err(format!("Error reading Data: {}", err).into()),
            _ => (),
        }

        // Read CRC
        let mut crc = [0u8; 4];
        match file.read_exact(&mut crc) {
            Err(err) => return Err(format!("Error reading CRC: {}", err).into()),
            _ => (),
        }

        let chunk = Chunk {
            length,
            chunk_type,
            data,
            crc,
        };
        Ok(chunk)
    }

    // return the chunk type as an array
    #[allow(dead_code)]
    pub fn chunk_type(&self) -> [u8; 4] {
        self.chunk_type
    }

    // return the chunk type as a string slice
    pub fn chunk_type_as_str(&self) -> &str {
        // self.chunk_type.into()
        str::from_utf8(&self.chunk_type).unwrap()
    }
}
