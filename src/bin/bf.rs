use brainfuck::{evaluate, BadExpressionError};
use clap::Parser;
use derive_more::{Display, From};
use std::{
    fs::File,
    io::{self, Read},
    path::{Path, PathBuf},
};

/// BrainFuck Interpreter
///
/// This is an implementation made in rust. Expected to be performant enough.
#[derive(Parser, Debug)]
#[command(version, about, long_about)]
struct Arguments {
    /// script to read from
    file: PathBuf,
}

#[derive(From, Display)]
enum CliError {
    IO(io::Error),
    Runtime(BadExpressionError),
}

impl std::fmt::Debug for CliError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self}")
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

    evaluate(&code)?;

    Ok(())
}
