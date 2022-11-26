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

            let mut data = std::fs::read(input_path)?;
            let key = std::fs::read(key_path)?;

            if key.len() < 128 {
                return Err(Error::KeyTooShort(key.len()));
            }

            crypto::encrypt(&mut data, &key, rounds);

            std::fs::write(output_path, data)?;

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

            let mut data = std::fs::read(input_path)?;
            let key = std::fs::read(key_path)?;

            if key.len() < 128 {
                return Err(Error::KeyTooShort(key.len()));
            }

            crypto::decrypt(&mut data, &key, rounds);

            std::fs::write(output_path, data)?;

            println!("Finished in {} ms", instant.elapsed().as_millis());

            Ok(())
        }

        // keygen subcommand
        Options::Keygen { size, output } => {
            println!("Generating key of size {}", size);

            if size < 128 {
                return Err(Error::KeyTooShort(size));
            }

            let key = crypto::generate_key(size);

            std::fs::write(output, key)?;

            println!("Finished in {} ms", instant.elapsed().as_millis());

            Ok(())
        }
    }
}
