use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(about, version)]
pub struct CliArgs {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    Encode {
        /// The input file path name containing the data to encode
        input: PathBuf,
    },
    Decode {
        /// A single token (no whitespace) of encoded data
        encoded: String,

        /// The output path name to store the decoded result
        /// Pre-existing files will be overwritten
        output: PathBuf,
    },
}
