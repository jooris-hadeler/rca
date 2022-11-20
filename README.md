# RCA (RustCryptoAlgorithm)
RCA is a rewrite of one of my [older](https://github.com/Jooarye/jca) Projects in Rust. 
Back then I was fairly new to programming and only published a few Projects on Github.

## Requirements
In order of building RCA you need the following:
- Rust Compiler Toolchain ~= 1.65.0
- Git

## Building
You can easily build RCA by running these commands:
```bash
$ git clone https://github.com/jooris-hadeler/rca
$ cd rca
$ cargo build --release
```

## Usage
```bash
$ rca
rca 0.2.0
A simple AES-ish crypto algorithm.

USAGE:
    rca <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    decrypt    Decrypt files
    encrypt    Encrypt files
    help       Prints this message or the help of the given subcommand(s)
    keygen     Generate keys
```

## Contributing
If you find anything that needs changing, feel free to create an issue.
If you already have a fix for the issue you can also create a pull request.
