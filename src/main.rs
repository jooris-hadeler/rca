mod cli;
mod crypto;
mod error;
mod prelude;
mod sbox;
mod log;

use crate::prelude::*;
use std::time::Instant;
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

            let mut data = std::fs::read(input_path)?;
            let key = std::fs::read(key_path)?;

            if key.len() < 128 {
                return Err(Error::KeyTooShort(key.len()));
            }

            crypto::encrypt(&mut data, &key, rounds);

            std::fs::write(output_path, data)?;

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

            let mut data = std::fs::read(input_path)?;
            let key = std::fs::read(key_path)?;

            if key.len() < 128 {
                return Err(Error::KeyTooShort(key.len()));
            }

            crypto::decrypt(&mut data, &key, rounds);

            std::fs::write(output_path, data)?;

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
