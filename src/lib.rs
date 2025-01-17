mod cli_args;

use std::{fmt::Display, str::FromStr};

use anyhow::{Context, Result};
pub use cli_args::*;

fn should_encode(c: char) -> bool {
    !(c != 'x' && c != ':' && c.is_ascii_graphic() && !c.is_ascii_whitespace())
}

pub struct ProgramOutput(pub String);
impl Display for ProgramOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, ":{}:", self.0)
    }
}

pub struct EncodedChar(char);

impl ToString for EncodedChar {
    fn to_string(&self) -> String {
        format!("x{:02x}", self.0 as u8)
    }
}

impl FromStr for EncodedChar {
    type Err = anyhow::Error;

    fn from_str(str: &str) -> Result<Self, Self::Err> {
        let str_chars: Vec<char> = str.chars().collect();
        let chars: String = str_chars[1..=2].iter().collect();
        let c = u8::from_str_radix(&chars, 16)
            .with_context(|| format!("Unable to convert `{}` to u8", &chars))?
            as char;

        Ok(Self(c))
    }
}

pub fn encode(str: String) -> String {
    str.chars()
        .map(|c| {
            if should_encode(c) {
                EncodedChar(c).to_string()
            } else {
                c.to_string()
            }
        })
        .collect()
}

pub fn decode(str: String) -> Result<String> {
    let str_chars: Vec<char> = str.chars().collect();

    Ok(str
        .match_indices('x')
        .map(|(i, _)| -> Result<(String, String)> {
            let chars: String = str_chars[i..=i + 2].iter().collect();
            let encoded = EncodedChar::from_str(&chars)?;
            Ok((chars, encoded.0.to_string()))
        })
        .filter_map(Result::ok)
        .fold(str.to_owned(), |str, (replace, replacement)| {
            str.replace(&replace, &replacement)
        }))
}
