use brainfuck::evaluate;
use clap::Parser;
use std::fs::File;
use std::io::{self, Read};
use std::path::PathBuf;

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
    Runtime(brainfuck::RuntimeError),
}

impl std::fmt::Display for CliError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
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

impl From<brainfuck::RuntimeError> for CliError {
    fn from(value: brainfuck::RuntimeError) -> Self {
        CliError::Runtime(value)
    }
}

type CliResult = Result<(), CliError>;

fn main() -> CliResult {
    let args = Arguments::parse();
    let mut code = String::new();

    File::from(args.file);

    let path = args.file.to_str().unwrap_or("InvalidPath");

    println!("Executing script from: {path}");
    Ok(())
}
