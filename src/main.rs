mod cli;
mod crypto;
mod error;
mod log;
mod prelude;
mod sbox;

use crate::prelude::*;
use std::{
    fs::{File, OpenOptions},
    io::{BufReader, BufWriter},
    time::Instant,
};
use structopt::StructOpt;

fn main() {
    let result = run();

    if result.is_err() {
        error!("{}", result.unwrap_err());
    }
}

fn run() -> Result<()> {
    let opt = Options::from_args();

    let instant = Instant::now();

    match opt {
        // encrypt subcommand
        Options::Encrypt {
            input: input_path,
            key: key_path,
            output: output_path,
            rounds,
        } => {
            info!("Encrypting {} with {}", input_path, key_path);

            // Setup buffered reader and writer
            let input_file = File::open(input_path)?;
            let input_length = input_file.metadata()?.len() as usize;
            let mut buf_reader = BufReader::new(input_file);

            let output_file = OpenOptions::new()
                .write(true)
                .create(true)
                .truncate(true)
                .open(output_path)?;
            let mut buf_writer = BufWriter::new(output_file);

            // Read in key
            let key = std::fs::read(key_path)?;
            if key.len() < 128 {
                return Err(Error::KeyTooShort(key.len()));
            }

            // Encrypt in chunks
            crypto::encrypt(&mut buf_reader, input_length, &mut buf_writer, &key, rounds)?;

            info!("Finished in {} ms", instant.elapsed().as_millis());

            Ok(())
        }

        // decrypt subcommand
        Options::Decrypt {
            input: input_path,
            key: key_path,
            output: output_path,
            rounds,
        } => {
            info!("Decrypting {} with {}", input_path, key_path);

            // Setup buffered reader and writer
            let input_file = File::open(input_path)?;
            let input_length = input_file.metadata()?.len() as usize;
            let mut buf_reader = BufReader::new(input_file);

            let output_file = OpenOptions::new()
                .write(true)
                .create(true)
                .truncate(true)
                .open(output_path)?;
            let mut buf_writer = BufWriter::new(output_file);

            // Read in key
            let key = std::fs::read(key_path)?;
            if key.len() < 128 {
                return Err(Error::KeyTooShort(key.len()));
            }

            // Decrypt in chunks
            crypto::decrypt(&mut buf_reader, input_length, &mut buf_writer, &key, rounds)?;

            info!("Finished in {} ms", instant.elapsed().as_millis());

            Ok(())
        }

        // keygen subcommand
        Options::Keygen { size, output } => {
            info!("Generating key of size {}", size);

            if size < 128 {
                warn!("A key of length {} is too short to be used with rca", size);
            }

            let key = crypto::generate_key(size);

            std::fs::write(output, key)?;

            info!("Finished in {} ms", instant.elapsed().as_millis());

            Ok(())
        }
    }
}
