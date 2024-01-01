mod args;
mod charmap;
mod deletion;
mod replacement;

use std::io::Read;
use std::{error::Error, io::stdin};

use crate::args::parse_validate_cli;
use crate::deletion::Deleter;
use crate::replacement::Replacer;

/*
TODO:
 -c and -C : compliment chars
 extend classes
 -s : squeeze chars
*/

fn main() -> Result<(), Box<dyn Error>> {
    let args = parse_validate_cli()?;

    match args.delete {
        true => delete(args),
        false => replace(args),
    }

    Ok(())
}

fn delete(args: args::Cli) {
    let deleter = Deleter::new(&args.string1);

    let input = read_input().unwrap();
    println!("{}", deleter.delete(&input));
}

fn replace(args: args::Cli) {
    let replacer = Replacer::new(&args.string1, &args.string2.unwrap());

    let input = read_input().unwrap();
    println!("{}", replacer.replace(&input));
}

fn read_input() -> Result<String, Box<dyn Error>> {
    let mut input = String::new();

    // Read until EOF
    stdin().lock().read_to_string(&mut input)?;

    Ok(input)
}
