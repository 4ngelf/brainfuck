use derive_more::Display;

/// Represents possible tokens found in a BrainFuck script
#[derive(Debug, Display, PartialEq, Eq, Clone, Copy, Hash)]
pub enum Token {
    #[display(fmt = ">")]
    MoveRight,

    #[display(fmt = "<")]
    MoveLeft,

    #[display(fmt = "+")]
    Increment,

    #[display(fmt = "-")]
    Decrement,

    #[display(fmt = ",")]
    ReadByte,

    #[display(fmt = ".")]
    WriteByte,

    #[display(fmt = "[")]
    LoopStart,

    #[display(fmt = "]")]
    LoopEnd,

    #[display(fmt = "{}", "*_0 as char")]
    Comment(u8),
}

impl From<u8> for Token {
    fn from(value: u8) -> Self {
        match value {
            b'>' => Token::MoveRight,
            b'<' => Token::MoveLeft,
            b'+' => Token::Increment,
            b'-' => Token::Decrement,
            b',' => Token::ReadByte,
            b'.' => Token::WriteByte,
            b'[' => Token::LoopStart,
            b']' => Token::LoopEnd,
            _ => Token::Comment(value),
        }
    }
}

impl From<char> for Token {
    fn from(value: char) -> Self {
        From::from(value as u8)
    }
}

#[cfg(test)]
mod tests {
    use super::Token as TO;

    const TOKENS: [TO; 9] = [
        TO::MoveRight,
        TO::MoveLeft,
        TO::Increment,
        TO::Decrement,
        TO::ReadByte,
        TO::WriteByte,
        TO::LoopStart,
        TO::LoopEnd,
        TO::Comment(b' '),
    ];

    const TOKEN_STR: &str = "><+-,.[] ";

    #[test]
    fn token_char_conversions() {
        let pairs = TOKEN_STR.chars().zip(TOKENS);

        for (ch, token) in pairs {
            assert_eq!(String::from(ch), format!("{token}"));
            assert_eq!(token, ch.into());
        }
    }

    #[test]
    fn token_byte_conversions() {
        let pairs = TOKEN_STR.bytes().zip(TOKENS);

        for (byte, token) in pairs {
            assert_eq!(token, byte.into());
        }
    }
}
