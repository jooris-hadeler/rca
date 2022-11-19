use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "rca", about = "A simple AES-ish crypto algorithm.")]
pub enum Options {
    /// Subcommand options for encryption
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
        #[structopt(short, long, default_value = "a.out")]
        output: String,
    },

    /// Subcommand options for decryption
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
        #[structopt(short, long, default_value = "a.out")]
        output: String,
    },
}
