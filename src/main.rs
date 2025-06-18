use std::{env, fs, io::Write, process};

use chunk::{Chunk, IhdrChunk};
use filter::Config;
use png::SignatureHeader;

mod chunk;
mod png;

fn main() {
    // Ensure correct usage and initializing config
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Error parsing arguments: {}", err);
        process::exit(1);
    });

    // Opening file
    let mut fileptr = match fs::File::open(&config.file_path()) {
        Err(err) => {
            eprintln!("Error opening file: {err}. Incorrect filename?");
            process::exit(1);
        }
        Ok(file) => file,
    };

    // Read Signature Header and ensure if valid file
    // TODO Start moving this to run() function in lib.rs
    let signature_header: SignatureHeader = match SignatureHeader::build(&mut fileptr) {
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
        if curr_chunk.chunk_type_as_str() == "IHDR" {
            println!("IHDR Chunk found!");
            let ihdr: IhdrChunk = IhdrChunk::new(&curr_chunk);
            println!("{:?}", ihdr);
        }

        println!("Chunk Type: {}", curr_chunk.chunk_type_as_str());
        if curr_chunk.chunk_type_as_str() == "IEND" {
            println!("IEND Reached!");
            break;
        }
    }

    println!("\nChecking if the Chunks are the same in the Vec:");
    for chunk in &chunks {
        if chunk.chunk_type_as_str() == "IHDR" {
            let ihdr: IhdrChunk = IhdrChunk::new(&chunk);
            println!("{:?}", ihdr)
        } else {
            println!("{}", chunk.chunk_type_as_str());
        }
    }

    // At this point, should we try and write the same chunks
    // to an output file and see if the output is correct?
    let mut out_fileptr = match fs::File::create(&new_filename) {
        Err(err) => {
            eprintln!("Error opening output file: {err}.");
            process::exit(1);
        }
        Ok(file) => file,
    };
    // write signature header
    match out_fileptr.write(&signature_header.values()) {
        Err(err) => {
            eprintln!("Error writing to file {}: {}", &new_filename, err);
            process::exit(1);
        }
        Ok(size) => println!("{} bytes correctly written to {}", size, &new_filename),
    }
    // write chunks one by one
    // We could refactor this function out of main
    // TODO think of a new .rs file to move this function and maybe others
    // run function???? See above when reading Signature Header!
    for chunk in chunks {
        match chunk.write_to_file(&mut out_fileptr) {
            Err(err) => {
                eprintln!("Error writing data to {}: {}.", &new_filename, err);
                process::exit(1);
            }
            Ok(size) => println!("Written {} bytes to {}.", size, &new_filename),
        }
    }
}
