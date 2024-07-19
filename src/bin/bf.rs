use brainfuck::{evaluate, BadExpressionError as BFError};
use clap::Parser;
use std::fs::File;
use std::io::{self, Read};
use std::path::{Path, PathBuf};

/// BrainFuck Interpreter
///
/// This is an implementation made in rust. Expected to be performant enough.
#[derive(Parser, Debug)]
#[command(version, about, long_about)]
struct Arguments {
    /// script to read from
    file: PathBuf,
}

#[derive(Debug)]
enum CliError {
    IO(io::Error),
    Runtime(BFError),
}

impl std::fmt::Display for CliError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CliError::IO(e) => e.fmt(f),
            CliError::Runtime(e) => e.fmt(f),
        }
    }
}

impl From<io::Error> for CliError {
    fn from(value: io::Error) -> Self {
        CliError::IO(value)
    }
}

impl From<BFError> for CliError {
    fn from(value: BFError) -> Self {
        CliError::Runtime(value)
    }
}

fn read_file<T: AsRef<Path>>(path: &T) -> io::Result<String> {
    let mut f = File::open(path)?;
    let mut output = String::new();
    f.read_to_string(&mut output)?;

    Ok(output)
}

fn main() -> Result<(), CliError> {
    let args = Arguments::parse();
    let code = read_file(&args.file)?;

    if let Err(e) = evaluate(&code) {
        eprintln!("Error: {e}")
    }

    Ok(())
}
