use std::{fs::File, io::Read};

const SIGNATURE_HEADER: [u8; 8] = [137, 80, 78, 71, 13, 10, 26, 10];

#[allow(dead_code)]
pub struct SignatureHeader {
    values: [u8; 8],
}

impl SignatureHeader {
    // Creates a Signature Header
    // If an error occurs, we can say that we have an invalid PNG File
    pub fn new(file: &mut File) -> Result<Self, Box<dyn std::error::Error>> {
        let mut filebuff = [0u8; 8];

        // read into buffer
        match file.read_exact(&mut filebuff) {
            Err(err) => return Err(format!("Error reading Signature Header: {err}").into()),
            _ => (),
        };

        // Compare values
        if filebuff.to_vec().iter().eq(&SIGNATURE_HEADER.to_vec()) {
            Ok(SignatureHeader { values: filebuff })
        } else {
            Err("Mismatch in Signature Header".into())
        }
    }

    // Returns the bytes from the signature header
    #[allow(dead_code)]
    pub fn values(&self) -> [u8; 8] {
        self.values
    }
}
