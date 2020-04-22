extern crate clap;
extern crate pest;
#[macro_use]
extern crate pest_derive;

mod cddl;
mod formats;

use crate::cddl::Ruleset;
use clap::Clap;
use std::fs::read_to_string;
use std::io::Read;
use std::path::PathBuf;
use std::str::FromStr;

#[derive(Clap)]
enum OutputFormat {
    Debug,
}

impl FromStr for OutputFormat {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "debug" => Ok(OutputFormat::Debug),
            _ => Err(format!("Invalid format: {}", s)),
        }
    }
}

#[derive(Clap)]
struct Opts {
    #[clap(help = "The input file path, or '-' for using STDIN.")]
    input: PathBuf,

    #[clap(
        short = "f",
        long = "format",
        default_value = "debug",
        help = "The output format of the interface."
    )]
    format: OutputFormat,

    #[clap(
        long = "check",
        help = "Only validate the CDDL, don't output anything."
    )]
    check: bool,
}

fn main() {
    let opt: Opts = Opts::parse();

    let content = if opt.input.as_os_str().eq("-") {
        let mut buf = String::new();
        std::io::stdin()
            .read_to_string(&mut buf)
            .expect("Could not read STDIN.");
        buf
    } else {
        read_to_string(&opt.input).expect("Could not read file.")
    };

    match Ruleset::from_str(&content).and_then(|ruleset| ruleset.validate().map(|_| ruleset)) {
        Ok(cddl) => {
            if !opt.check {
                match opt.format {
                    OutputFormat::Debug => formats::debug::print(cddl),
                }
            }
        }
        Err(e) => match e {
            crate::cddl::error::Error::ParseError(msg) => eprintln!("Syntax error:\n{}", msg),
        },
    }
}
