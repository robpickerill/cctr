mod args;
mod charmap;
mod classes;
mod deleter;
mod replacer;
mod squeezer;

use std::io::Read;
use std::{error::Error, io::stdin};

use crate::args::parse_validate_cli;
use crate::deleter::Deleter;
use crate::replacer::Replacer;
use crate::squeezer::Squeezer;

/*
TODO:
 -c and -C : compliment chars
 extend classes
*/

fn main() -> Result<(), Box<dyn Error>> {
    let args = parse_validate_cli()?;

    if args.delete {
        delete(&args);
    }

    if args.squeeze {
        squeeze(&args);
    }

    replace(&args);

    Ok(())
}

fn delete(args: &args::Cli) {
    let deleter = Deleter::new(&args.string1);

    let input = read_input().unwrap();
    println!("{}", deleter.delete(&input));
    std::process::exit(0);
}

fn squeeze(args: &args::Cli) {
    let squeezer = Squeezer::new(&args.string1);

    let input = read_input().unwrap();
    println!("{}", squeezer.squeeze(&input));
    std::process::exit(0);
}

fn replace(args: &args::Cli) {
    let dest = args.string2.as_ref().unwrap();
    let replacer = Replacer::new(&args.string1, dest);

    let input = read_input().unwrap();
    println!("{}", replacer.replace(&input));
    std::process::exit(0);
}

fn read_input() -> Result<String, Box<dyn Error>> {
    let mut input = String::new();

    // Read until EOF
    stdin().lock().read_to_string(&mut input)?;

    Ok(input)
}
