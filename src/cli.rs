use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "rca", about = "A simple AES-ish crypto algorithm.")]
pub enum Options {
    /// Encrypt files
    #[structopt(name = "encrypt")]
    Encrypt {
        /// Number of rounds
        #[structopt(short, long, default_value = "4")]
        rounds: usize,

        /// Input file
        #[structopt()]
        input: String,

        /// Key file
        #[structopt(short, long, default_value = "a.key")]
        key: String,

        /// Output file
        #[structopt(short, long, default_value = "a.enc")]
        output: String,
    },

    /// Decrypt files
    #[structopt(name = "decrypt")]
    Decrypt {
        /// Number of rounds
        #[structopt(short, long, default_value = "4")]
        rounds: usize,

        /// Input file
        #[structopt()]
        input: String,

        /// Key file
        #[structopt(short, long, default_value = "a.key")]
        key: String,

        /// Output file
        #[structopt(short, long, default_value = "a.dec")]
        output: String,
    },

    /// Generate keys
    #[structopt(name = "keygen")]
    Keygen {
        /// Size of the key
        size: usize,

        /// Output file
        #[structopt(short, long, default_value = "a.key")]
        output: String,
    },
}
