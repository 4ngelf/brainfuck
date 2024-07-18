//! Brainfuck interpreter library
use std::fmt;

/// Represents possible tokens found in a BrainFuck script
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum Token {
    MoveRight,
    MoveLeft,
    Increment,
    Decrement,
    ReadByte,
    WriteByte,
    LoopStart,
    LoopEnd,
    Comment(u8),
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::MoveRight => write!(f, ">"),
            Token::MoveLeft => write!(f, "<"),
            Token::Increment => write!(f, "+"),
            Token::Decrement => write!(f, "-"),
            Token::ReadByte => write!(f, "."),
            Token::WriteByte => write!(f, ","),
            Token::LoopStart => write!(f, "["),
            Token::LoopEnd => write!(f, "]"),
            Token::Comment(char) => write!(f, "{char}"),
        }
    }
}

impl From<u8> for Token {
    fn from(value: u8) -> Self {
        match value {
            b'>' => Token::MoveRight,
            b'<' => Token::MoveLeft,
            b'+' => Token::Increment,
            b'-' => Token::Decrement,
            b'.' => Token::ReadByte,
            b',' => Token::WriteByte,
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

/// Syntactic error while parsing Brainfuck code
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum BadExpressionError {
    LoopNotClosed,
    LoopNotOpened,
}

impl fmt::Display for BadExpressionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::LoopNotClosed => write!(f, "'[' was never closed"),
            Self::LoopNotOpened => write!(f, "unmatched ']' symbol"),
        }
    }
}

/// This represents one unit of execution in the program
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
enum Expression {
    Forward,
    Backward,
    Increment,
    Decrement,
    Input,
    Output,
    Loop(Vec<Expression>),
}

#[derive(Debug, PartialEq, Clone, Hash)]
struct InstructionTree(Vec<Expression>);

impl InstructionTree {
    pub fn new() -> Self {
        Default::default()
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
            Token::LoopStart => InstructionTree::parse_next_loop_token(tokens)?,
            Token::LoopEnd => Err(BadExpressionError::LoopNotOpened),
            Token::Comment(_) => InstructionTree::parse_next_generic_token(tokens)?,
        };

        Some(token)
    }

    fn parse_next_loop_token<T>(tokens: &mut T) -> Option<Result<Expression, BadExpressionError>>
    where
        T: Iterator<Item = Token>,
    {
        use BadExpressionError as BE;
        use Expression as E;

        let mut expressions = Vec::new();
        loop {
            let expr = match InstructionTree::parse_next_generic_token(tokens) {
                Some(expr) => expr,
                None => return Some(Err(BE::LoopNotClosed)),
            };

            match expr {
                Ok(expr) => expressions.push(expr),
                Err(BE::LoopNotOpened) => return Some(Ok(E::Loop(expressions))),
                Err(err) => return Some(Err(err)),
            }
        }
    }
}

impl std::default::Default for InstructionTree {
    fn default() -> Self {
        InstructionTree(Vec::new())
    }
}

impl std::ops::Deref for InstructionTree {
    type Target = Vec<Expression>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for InstructionTree {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl IntoIterator for InstructionTree {
    type Item = Expression;
    type IntoIter = <Vec<Self::Item> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        IntoIterator::into_iter(self.0)
    }
}

impl std::ops::Add for InstructionTree {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        let mut output = self;
        output.0.extend(rhs.0);

        output
    }
}

impl std::ops::AddAssign for InstructionTree {
    fn add_assign(&mut self, rhs: Self) {
        self.0.extend(rhs.0)
    }
}

impl std::str::FromStr for InstructionTree {
    type Err = BadExpressionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut tokens = s.bytes().map(Token::from);
        let mut expressions = Vec::new();

        while let Some(expr) = InstructionTree::parse_next_generic_token(&mut tokens) {
            expressions.push(expr?);
        }

        Ok(InstructionTree(expressions))
    }
}

/// It holds the memory of the program
type Memory = Vec<u8>;

/// The default amount of memory allowed for a BrainFuck program
const DEFAULT_BRAINFUCK_STACK_SIZE: usize = 32_768;

/// This represents the running context of a BrainFuck program
#[derive(Debug, Hash)]
struct MemoryContext {
    memory: Memory,
    pointer_index: usize,
}

impl MemoryContext {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn with_capacity(capacity: usize) -> Self {
        let memory = vec![0; capacity];
        let pointer_index = capacity / 2;

        MemoryContext {
            memory,
            pointer_index,
        }
    }

    pub fn move_forward(&mut self) {
        let pointer_index = self.pointer_index.wrapping_add(1);

        if pointer_index < self.memory.len() {
            self.pointer_index = pointer_index;
        } else {
            self.pointer_index = 0;
        }
    }

    pub fn move_backward(&mut self) {
        let (pointer_index, overflow) = self.pointer_index.overflowing_sub(1);

        if !overflow {
            self.pointer_index = pointer_index;
        } else {
            self.pointer_index = self.memory.len() - 1;
        }
    }

    pub fn set(&mut self, value: u8) {
        self.memory[self.pointer_index] = value;
    }

    pub fn get(&self) -> u8 {
        self.memory[self.pointer_index]
    }

    pub fn increment(&mut self) {
        self.set(self.get().wrapping_add(1))
    }

    pub fn decrement(&mut self) {
        self.set(self.get().wrapping_sub(1))
    }
}

impl std::default::Default for MemoryContext {
    fn default() -> Self {
        MemoryContext::with_capacity(DEFAULT_BRAINFUCK_STACK_SIZE)
    }
}

#[inline]
fn get_byte() -> u8 {
    use std::io::{self, Read};
    let mut byte: [u8; 1] = [0];

    match io::stdin().read_exact(&mut byte) {
        Ok(_) => byte[0],
        Err(_) => 0,
    }
}

#[inline]
fn print_byte(character: u8) {
    use std::io::{self, Write};
    print!("{character}");
    let _ = io::stdout().flush();
}

#[inline]
fn execute_expression(memory: &mut MemoryContext, expr: &Expression) {
    match expr {
        Expression::Increment => memory.increment(),
        Expression::Decrement => memory.decrement(),
        Expression::Forward => memory.move_forward(),
        Expression::Backward => memory.move_backward(),
        Expression::Input => memory.set(get_byte()),
        Expression::Output => print_byte(memory.get()),
        Expression::Loop(expressions) => {
            while memory.get() != 0 {
                for expr in expressions {
                    execute_expression(memory, expr);
                }
            }
        }
    }
}

/// Some error during runtime.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum RuntimeError {
    BadExpression(BadExpressionError),
}

impl From<BadExpressionError> for RuntimeError {
    fn from(value: BadExpressionError) -> Self {
        RuntimeError::BadExpression(value)
    }
}

impl fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            RuntimeError::BadExpression(err) => err.fmt(f),
        }
    }
}

pub type RuntimeResult = Result<(), RuntimeError>;

/// A Brainfuck interpreter
///
/// # Example
/// ```
/// use brainfuck::BrainFuckInterpreter;
///
/// let mut bf = BrainFuckInterpreter::new();
/// bf.feed_code(r##"
/// ++++++.---^
/// "##).unwrap();
///
/// bf.execute().unwrap();
///
/// ```
pub struct BrainFuckInterpreter {
    memory: MemoryContext,
    instructions: InstructionTree,
}

impl BrainFuckInterpreter {
    pub fn new() -> Self {
        BrainFuckInterpreter {
            memory: MemoryContext::new(),
            instructions: InstructionTree::new(),
        }
    }

    pub fn with_memory_size(size: usize) -> Self {
        BrainFuckInterpreter {
            memory: MemoryContext::with_capacity(size),
            instructions: InstructionTree::new(),
        }
    }

    pub fn feed_code(&mut self, code: &str) -> Result<(), BadExpressionError> {
        self.instructions += code.parse()?;

        Ok(())
    }

    pub fn execute(&mut self) -> RuntimeResult {
        for expr in self.instructions.iter() {
            execute_expression(&mut self.memory, expr)
        }

        Ok(())
    }
}

/// Run some Brainfuck code
pub fn evaluate(code: &str) -> RuntimeResult {
    let mut interpreter = BrainFuckInterpreter::new();
    interpreter.feed_code(code)?;

    interpreter.execute()
}
