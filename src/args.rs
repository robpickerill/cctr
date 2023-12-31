use std::error::Error;

use clap::{arg, Parser};

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Delete the characters in string1 from the input
    #[arg(short, long)]
    pub delete: bool,

    /// Replace the characters in string1 with the characters in string2
    pub string1: String,

    /// Set the string1 characters to be replaced with string2
    pub string2: Option<String>,
}

pub fn parse_validate_cli() -> Result<Cli, Box<dyn Error>> {
    let cli = Cli::parse();

    if cli.delete && cli.string2.is_some() {
        return Err(Box::from(
            "cannot specify --delete and string2 at the same time",
        ));
    }

    if !cli.delete && cli.string2.is_none() {
        return Err(Box::from("string2 cannot be empty"));
    }

    Ok(cli)
}
