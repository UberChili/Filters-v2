use std::{collections::HashMap, env::args, fs, process};

use chunk::Chunk;
use png::SignatureHeader;

mod chunk;
mod png;

fn main() {
    let mut department: HashMap<String, u32> = HashMap::new();
    department.insert(String::from("Blue"), 32);

    // Ensure correct usage
    let args: Vec<String> = args().collect();
    if args.len() != 2 {
        eprintln!("Usage: ./filter <png file>");
        process::exit(1);
    }
    let filename = &args[1];
    let _new_filename = format!("{filename}_copy.png");

    // Open file
    let mut fileptr = match fs::File::open(filename) {
        Err(err) => {
            eprintln!("Error opening file: {err}.");
            process::exit(1);
        }
        Ok(file) => file,
    };

    // Read Signature Header and ensure if valid file
    let _signature_header: SignatureHeader = match SignatureHeader::new(&mut fileptr) {
        Err(err) => {
            eprintln!("{err} Invalid PNG File.");
            process::exit(1);
        }
        Ok(signature) => {
            println!("Valid PNG File.");
            signature
        }
    };

    // Reading chunks until we find IEND
    // Be sure to print info from IHDR
    let mut chunks: Vec<Chunk> = Vec::new();
    loop {
        // Huh, I'm not sure about this:
        let curr_chunk: Chunk = match Chunk::new(&mut fileptr) {
            Err(err) => {
                eprintln!("Error reading Chunk: {}", err);
                process::exit(1);
            }
            Ok(chunk) => {
                // Pushing the chunk into the chunks Vec by cloning
                chunks.push(chunk.clone());
                chunk
            }
        };

        // print some info from IHDR
        // Just realizing maybe I need to think of a different approach when using the chunks

        println!("Chunk Type: {}", curr_chunk.chunk_type_as_str());
        if curr_chunk.chunk_type_as_str() == "IEND" {
            println!("IEND Reached!");
            break;
        }
    }
}
