use std::{
    fmt::Display,
    io::{Read, Write},
    path::Path,
};

use alphabet_encoding::*;
use anyhow::{Context, Result};
use clap::Parser;

mod cli_args;
use cli_args::*;

fn main() -> Result<()> {
    let _args = CliArgs::parse();

    match _args.command {
        Command::Encode { input } => {
            println!("OUTPUT {}", ProgramOutput(encode(read_file(&input)?)))
        }
        Command::Decode { encoded, output } => write_file(&output, decode(encoded)?)?,
    }

    Ok(())
}

fn read_file(p: &Path) -> Result<String> {
    let mut file = std::fs::File::open(p).context("Unable to open file!")?;

    let mut res = String::new();
    file.read_to_string(&mut res)
        .context("Unable to read file contents to string!")?;

    Ok(res)
}

fn write_file(p: &Path, contents: String) -> Result<()> {
    let mut file = std::fs::File::create(p).context("Unable to create file!")?;

    file.write(&contents.as_bytes())
        .context("Unable to write to file!")?;

    Ok(())
}

pub struct ProgramOutput(pub String);
impl Display for ProgramOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, ":{}:", self.0)
    }
}
