use crate::token::Token;
use derive_more::{Deref, DerefMut, Display, Error};

/// Syntactic error while parsing Brainfuck code
#[derive(Debug, Display, Error, PartialEq, Eq, Clone, Copy, Hash)]
pub enum BadExpressionError {
    #[display(fmt = "'[' was never closed")]
    LoopNotClosed,

    #[display(fmt = "unmatched ']' symbol")]
    LoopNotOpened,
}

/// This represents one unit of execution in the program
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum Expression {
    Forward,
    Backward,
    Increment,
    Decrement,
    Input,
    Output,
    Loop(Vec<Expression>),
}

/// This represents a tree of expressions for a valid BrainFuck script
#[derive(Default, Debug, PartialEq, Clone, Hash, Deref, DerefMut)]
pub struct SyntaxTree(Vec<Expression>);

impl SyntaxTree {
    pub fn new() -> Self {
        Default::default()
    }

    /// Parse a collection of tokens into a valid [`SyntaxTree`]
    pub fn parse_tokens<T>(tokens: T) -> Result<Self, BadExpressionError>
    where
        T: IntoIterator<Item = Token>,
    {
        let mut tokens = tokens.into_iter();
        let mut expressions = Vec::new();

        while let Some(expr) = SyntaxTree::parse_next_generic_token(&mut tokens) {
            expressions.push(expr?);
        }

        Ok(SyntaxTree(expressions))
    }

    fn parse_next_generic_token<T>(tokens: &mut T) -> Option<Result<Expression, BadExpressionError>>
    where
        T: Iterator<Item = Token>,
    {
        let token = match tokens.next()? {
            Token::MoveRight => Ok(Expression::Forward),
            Token::MoveLeft => Ok(Expression::Backward),
            Token::Increment => Ok(Expression::Increment),
            Token::Decrement => Ok(Expression::Decrement),
            Token::ReadByte => Ok(Expression::Input),
            Token::WriteByte => Ok(Expression::Output),
            Token::LoopStart => SyntaxTree::parse_next_loop_token(tokens)?,
            Token::LoopEnd => Err(BadExpressionError::LoopNotOpened),
            Token::Comment(_) => SyntaxTree::parse_next_generic_token(tokens)?,
        };

        Some(token)
    }

    fn parse_next_loop_token<T>(tokens: &mut T) -> Option<Result<Expression, BadExpressionError>>
    where
        T: Iterator<Item = Token>,
    {
        use BadExpressionError as Error;
        use Expression as E;

        let mut expressions = Vec::new();
        loop {
            let expr = match SyntaxTree::parse_next_generic_token(tokens) {
                Some(expr) => expr,
                None => return Some(Err(Error::LoopNotClosed)),
            };

            match expr {
                Ok(expr) => expressions.push(expr),
                Err(Error::LoopNotOpened) => return Some(Ok(E::Loop(expressions))),
                Err(err) => return Some(Err(err)),
            }
        }
    }
}

impl std::str::FromStr for SyntaxTree {
    type Err = BadExpressionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::parse_tokens(s.bytes().map(Token::from))
    }
}

impl IntoIterator for SyntaxTree {
    type Item = Expression;
    type IntoIter = <Vec<Expression> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

#[cfg(test)]
mod tests {
    use super::{BadExpressionError as Bad, Expression as E, SyntaxTree as ET};

    #[test]
    fn parse_valid_string() {
        let tree: ET = "+++>[<--->]<.".parse().unwrap();

        assert_eq!(
            tree,
            ET(vec![
                E::Increment,
                E::Increment,
                E::Increment,
                E::Forward,
                E::Loop(vec![
                    E::Backward,
                    E::Decrement,
                    E::Decrement,
                    E::Decrement,
                    E::Forward,
                ]),
                E::Backward,
                E::Output
            ])
        );
    }

    #[test]
    fn parse_error_loop_not_opened() {
        let tree_error: Result<ET, Bad> = "+++><--->]<.".parse();

        assert_eq!(tree_error, Err(Bad::LoopNotOpened));
    }

    #[test]
    fn parse_error_loop_not_closed() {
        let tree_error: Result<ET, Bad> = "+++>[<---><.".parse();

        assert_eq!(tree_error, Err(Bad::LoopNotClosed));
    }
}

