mod cli;
mod crypto;
mod error;
mod prelude;
mod sbox;

use crate::prelude::*;
use std::time::Instant;
use structopt::StructOpt;

fn main() -> Result<()> {
    let opt = Options::from_args();

    let instant = Instant::now();

    // match the subcommand
    match opt {
        // encrypt subcommand
        Options::Encrypt {
            input: input_path,
            key: key_path,
            output: output_path,
            rounds,
        } => {
            println!("Encrypting {} with {}", input_path, key_path);

            let maybe_data = std::fs::read(input_path);
            if maybe_data.is_err() {
                return Err(Error::IOError(maybe_data.unwrap_err()));
            }

            let mut data = maybe_data.unwrap();

            let maybe_key = std::fs::read(key_path);
            if maybe_key.is_err() {
                return Err(Error::IOError(maybe_key.unwrap_err()));
            }

            let key = maybe_key.unwrap();
            if key.len() < 128 {
                return Err(Error::Generic(format!(
                    "key of size {} is to small",
                    key.len()
                )));
            }

            crypto::encrypt(&mut data, &key, rounds);

            let result = std::fs::write(output_path, data);
            if result.is_err() {
                return Err(Error::IOError(result.unwrap_err()));
            }

            println!("Finished in {} ms", instant.elapsed().as_millis());

            Ok(())
        }

        // decrypt subcommand
        Options::Decrypt {
            input: input_path,
            key: key_path,
            output: output_path,
            rounds,
        } => {
            println!("Decrypting {} with {}", input_path, key_path);

            let maybe_data = std::fs::read(input_path);
            if maybe_data.is_err() {
                return Err(Error::IOError(maybe_data.unwrap_err()));
            }

            let mut data = maybe_data.unwrap();

            let maybe_key = std::fs::read(key_path);
            if maybe_key.is_err() {
                return Err(Error::IOError(maybe_key.unwrap_err()));
            }

            let key = maybe_key.unwrap();
            if key.len() < 128 {
                return Err(Error::Generic(format!(
                    "key of size {} is to small",
                    key.len()
                )));
            }

            crypto::decrypt(&mut data, &key, rounds);

            let result = std::fs::write(output_path, data);
            if result.is_err() {
                return Err(Error::IOError(result.unwrap_err()));
            }

            println!("Finished in {} ms", instant.elapsed().as_millis());

            Ok(())
        }

        // keygen subcommand
        Options::Keygen { size, output } => {
            println!("Generating key of size {}", size);

            let key = crypto::generate_key(size);

            let result = std::fs::write(output, key);
            if result.is_err() {
                return Err(Error::IOError(result.unwrap_err()));
            }

            println!("Finished in {} ms", instant.elapsed().as_millis());

            Ok(())
        }
    }
}
