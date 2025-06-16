use std::{fs::File, io::Read};

const SIGNATURE_HEADER: [u8; 8] = [137, 80, 78, 71, 13, 10, 26, 10];

pub struct SignatureHeader {
    values: [u8; 8],
}

impl SignatureHeader {
    // Creates a Signature Header
    // If an error occurs, we can say that we have an invalid PNG File
    pub fn build(file: &mut File) -> Result<Self, Box<dyn std::error::Error>> {
        let mut filebuff = [0u8; 8];

        // read into buffer
        file.read_exact(&mut filebuff)?;

        // Compare values
        if filebuff.to_vec().iter().eq(&SIGNATURE_HEADER.to_vec()) {
            Ok(SignatureHeader { values: filebuff })
        } else {
            Err("Mismatch in Signature Header".into())
        }
    }

    // Returns the bytes from the signature header
    pub fn values(&self) -> [u8; 8] {
        self.values
    }
}
