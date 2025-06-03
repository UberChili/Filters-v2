use core::str;
use std::{
    fs::File,
    io::{Read, Write},
};

#[allow(dead_code)]
#[derive(Clone)]
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

    // Writes contents of chunk to a file, in order
    pub fn write_to_file(&self, file: &mut File) -> Result<usize, Box<dyn std::error::Error>> {
        let mut total_written = 0;

        // Write the length (as big-endian)
        let length_bytes = self.length.to_be_bytes();
        total_written += file.write(&length_bytes)?;

        // Write the chunk type
        total_written += file.write(&self.chunk_type())?;

        // Write the data
        total_written += file.write(&self.data())?;

        // Write the CRC
        total_written += file.write(&self.crc)?;

        Ok(total_written)
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

    //return the chunk's data
    pub fn data(&self) -> &Vec<u8> {
        &self.data
    }
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct IhdrChunk {
    width: u32,
    height: u32,
    bit_depth: u8,
    colour_type: u8,
    compression_method: u8,
    filter_method: u8,
    interlace_method: u8,
}

impl IhdrChunk {
    pub fn new(chunk: &Chunk) -> Self {
        let width: u32 = u32::from_be_bytes(chunk.data()[0..4].try_into().unwrap());
        let height: u32 = u32::from_be_bytes(chunk.data()[4..8].try_into().unwrap());
        let bit_depth: u8 = chunk.data()[8];
        let colour_type: u8 = chunk.data()[9];
        let compression_method: u8 = chunk.data()[10];
        let filter_method: u8 = chunk.data()[11];
        let interlace_method: u8 = chunk.data()[12];

        IhdrChunk {
            width,
            height,
            bit_depth,
            colour_type,
            compression_method,
            filter_method,
            interlace_method,
        }
    }
}
