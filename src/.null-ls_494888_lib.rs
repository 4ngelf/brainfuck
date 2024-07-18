//! Brainfuck interpreter library
use std::io::{self, Read, Write};
use std::{default, fmt};

#[derive(PartialEq, Debug)]
pub enum Token {
    FowardPos,
    BackwardPos,
    Increment,
    Decrement,
    Read,
    Write,
    RepeatIn,
    RepeatOut,
    Comment(char),
}

impl Default for Token {
    fn default() -> Self {
        Token::Comment('\0')
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Token::*;
        match self {
            FowardPos => write!(f, ">"),
            BackwardPos => write!(f, "<"),
            Increment => write!(f, "+"),
            Decrement => write!(f, "-"),
            Read => write!(f, "."),
            Write => write!(f, ","),
            RepeatIn => write!(f, "["),
            RepeatOut => write!(f, "]"),
            Comment(char) => write!(f, "{}", char),
        }
    }
}

impl From<u8> for Token {
    fn from(value: u8) -> Self {
        match value as char {
            '>' => Token::FowardPos,
            '<' => Token::BackwardPos,
            '+' => Token::Increment,
            '-' => Token::Decrement,
            '.' => Token::Read,
            ',' => Token::Write,
            '[' => Token::RepeatIn,
            ']' => Token::RepeatOut,
            _ => Token::Comment(value as char),
        }
    }
}

#[derive(Debug, Default)]
pub struct Instructions {
    tokens: Vec<Token>,
    current: usize,
}

impl Instructions {
    pub fn from_code<T: AsRef<[u8]>>(code: &T) -> Self {
        let tokens: Vec<Token> = code.as_ref().iter().map(|c: &u8| Token::from(*c)).collect();

        Instructions {
            tokens,
            ..Default::default()
        }
    }
}
