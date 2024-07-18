//! Brainfuck interpreter library
use std::fmt::{self};
use std::io::{self, Read, Write};

/// The default amount of memory allowed for a BrainFuck program
const DEFAULT_BRAINFUCK_STACK_SIZE: usize = 32_768;

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

/// This represents one unit of execution in the program
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
enum Instruction {
    Forward,
    Backward,
    Increment,
    Decrement,
    Input,
    Output,
    Loop(Vec<Instruction>),
}

type Memory = Vec<u8>;

enum SyntaxError {
    Empty,
    LoopNeverClosed,
    LoopNeverOpened,
}

/// This is a sequence of instructions to run the program
#[derive(Debug, PartialEq, Eq, Hash)]
struct BrainFuckScript(Vec<Instruction>);

impl BrainFuckScript {
    pub fn new() -> Self {
        BrainFuckScript(Vec::new())
    }

    // pub fn parse<T: TryInto<Self>>(value: T) -> Result<Self, SyntaxError> {
    //     todo!()
    // }
}

impl std::str::FromStr for BrainFuckScript {
    type Err = SyntaxError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        TryFrom::try_from(s.bytes().map(From::from).collect::<Vec<Token>>())
    }
}

impl TryFrom<Vec<Token>> for BrainFuckScript {
    type Error = SyntaxError;

    fn try_from(value: Vec<Token>) -> Result<Self, Self::Error> {
        let mut instructions = Vec::with_capacity(value.len());
        instructions.push(Instruction::BeginBrainFuck);

        if value.is_empty() {
            return Err(SyntaxError::Empty);
        }

        for token in value {
            match token {
                Token::MoveRight => instructions.push(Instruction::Forward),
                Token::MoveLeft => instructions.push(Instruction::Backward),
                Token::Increment => instructions.push(Instruction::Increment),
                Token::Decrement => instructions.push(Instruction::Decrement),
                Token::ReadByte => instructions.push(Instruction::Input),
                Token::WriteByte => instructions.push(Instruction::Output),
                Token::LoopStart => {}
                Token::LoopEnd => return Err(SyntaxError::LoopNeverOpened),
                Token::Comment(_) => continue,
            }
        }

        Ok(BrainFuckScript(instructions))
    }
}

/******

/// This holds the context or state of the program during execution
#[derive(Debug)]
pub struct ExecutionContext {
    memory: Vec<u8>,
    memory_position: usize,
}

impl ExecutionContext {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn with_capacity(capacity: usize) -> Self {
        let memory = vec![0; capacity];
        ExecutionContext {
            memory,
            memory_position: 0,
        }
    }

    pub fn get(&self) -> u8 {
        self.memory[self.memory_position]
    }

    pub fn set(&mut self, value: u8) {
        self.memory[self.memory_position] = value;
    }

    pub fn forward(&mut self) {
        let output = self.memory_position.wrapping_add(1);
        self.memory_position = if output < self.memory.len() {
            output
        } else {
            0
        };
    }

    pub fn backward(&mut self) {
        let (output, overflow) = self.memory_position.overflowing_sub(1);
        self.memory_position = if !overflow {
            output
        } else {
            self.memory.len() - 1
        };
    }

    pub fn increment(&mut self) {
        self.set(self.get().wrapping_add(1))
    }

    pub fn decrement(&mut self) {
        self.set(self.get().wrapping_sub(1))
    }
}

impl std::default::Default for ExecutionContext {
    fn default() -> Self {
        let memory = vec![0; DEFAULT_BRAINFUCK_STACK_SIZE];
        ExecutionContext {
            memory,
            memory_position: 0,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum Instruction {
    Execute(Token),
    OpenLoop(u16),
    CloseLoop(u16),
}

impl std::default::Default for Instruction {
    fn default() -> Self {
        Instruction::Execute(Default::default())
    }
}

#[derive(Default, Debug, PartialEq, Eq, Clone, Hash)]
pub struct InstructionSetError;

impl fmt::Display for InstructionSetError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Error while manipulating instruction set")
    }
}

/// This holds the instructions a program should follow until the end
#[derive(Default, Debug)]
pub struct InstructionScript {
    instructions: Vec<Instruction>,
    current: usize,
}

impl InstructionScript {
    pub fn new() -> Self {
        Default::default()
    }

    // pub fn extend(&mut self, other: InstructionScript) {
    //     self.instructions.extend(other.instructions)
    // }

    /// Add a new statement to the instruction set
    ///
    /// # Example:
    /// ```
    /// use brainfuck::{Token::Increment, Instruction::Execute, InstructionSet};
    /// let mut instructions = InstructionSet::new();
    /// let _ = instructions.add_statement("+++++".into()).unwrap();
    ///
    /// assert_eq!(Execute(Increment), instructions[0])
    /// ```
    pub fn add_statement(&mut self, statement: &str) -> Result<(), InstructionSetError> {
        let new_instructions = Self::try_from(statement)?;
        self.extend(new_instructions);

        Ok(())
    }

    pub fn find_loop_pair_index(&self, pair_index: usize) -> Option<usize> {
        use Instruction::{CloseLoop, OpenLoop};

        let criteria = match self.instructions[pair_index] {
            OpenLoop(order) => CloseLoop(order),
            CloseLoop(order) => OpenLoop(order),
            _ => return None,
        };

        self.instructions.iter().position(|ins| *ins == criteria)
    }

    pub fn reset(&mut self) {
        self.current = 0
    }
}

impl std::ops::Index<usize> for InstructionScript {
    type Output = Instruction;
    fn index(&self, index: usize) -> &Self::Output {
        self.instructions.index(index)
    }
}

impl std::ops::IndexMut<usize> for InstructionScript {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.instructions.index_mut(index)
    }
}

impl Iterator for InstructionScript {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        match self.instructions.get(self.current)? {
            Instruction::Execute(ref t) => {
                self.current += 1;
                Some(*t)
            }
            Instruction::OpenLoop(_) => {
                self.current = self.find_loop_pair_index(self.current)? + 1;
                self.next()
            }
            Instruction::CloseLoop(_) => {
                self.current = self.find_loop_pair_index(self.current)? + 1;
                self.next()
            }
        }
    }
}

impl TryFrom<Vec<Token>> for InstructionScript {
    type Error = InstructionSetError;

    fn try_from(value: Vec<Token>) -> Result<Self, Self::Error> {
        let mut instructions = Vec::with_capacity(value.len());
        let mut order = 0;

        for token in value {
            match token {
                Token::Comment(_) => continue,
                Token::RepeatIn => {
                    instructions.push(Instruction::OpenLoop(order));
                    order += 1;
                }
                Token::RepeatOut => {
                    if order == 0 {
                        return Err(InstructionSetError);
                    }
                    instructions.push(Instruction::CloseLoop(order));
                    order -= 1;
                }
                token => instructions.push(Instruction::Execute(token)),
            }
        }

        if order > 0 {
            return Err(InstructionSetError);
        }

        Ok(InstructionScript { instructions })
    }
}

impl TryFrom<&str> for InstructionScript {
    type Error = InstructionSetError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        TryFrom::try_from(value.bytes().map(From::from).collect::<Vec<Token>>())
    }
}

#[derive(Default, Debug)]
pub struct BrainFuckInterpreter {
    memory: ExecutionContext,
    instructions: InstructionScript,
    position: usize,
}

impl BrainFuckInterpreter {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn with_memory(memory_size: usize) -> Self {
        BrainFuckInterpreter {
            memory: ExecutionContext::with_capacity(memory_size),
            ..Default::default()
        }
    }

    pub fn from_file(file: std::fs::File) -> Result<Self, InstructionSetError> {
        todo!()
    }

    pub fn from_script(script: &str) -> Result<Self, InstructionSetError> {
        todo!()
    }

    fn execute_instruction(&mut self, instruction: Instruction) -> Result<(), ()> {
        match instruction {
            Instruction::Execute(Token::Increment) => self.memory.increment(),
            Instruction::Execute(Token::Decrement) => self.memory.decrement(),
            Instruction::Execute(Token::ForwardPos) => self.memory.forward(),
            Instruction::Execute(Token::BackwardPos) => self.memory.backward(),
            Instruction::OpenLoop(_) => {
                self.position = if self.memory.get() == 0 {
                    self.instructions
                        .find_loop_pair_index(self.position)
                        .ok_or(())?
                        + 1
                } else {
                    self.position + 1
                }
            }
            Instruction::CloseLoop(_) => {
                self.position = if self.memory.get() == 0 {
                    self.position + 1
                } else {
                    self.instructions
                        .find_loop_pair_index(self.position)
                        .ok_or(())?
                        + 1
                }
            }
            Instruction::Execute(Token::Read) => {
                let mut one_byte: [u8; 1] = [0];
                if let Err(_) = io::stdin().read_exact(&mut one_byte) {
                    return Err(());
                }

                self.memory.set(one_byte[0])
            }
            Instruction::Execute(Token::Write) => {
                let one_byte: [u8; 1] = [self.memory.get()];
                let mut stdout = io::stdout();

                match stdout.write(&one_byte) {
                    Ok(v) if v == 1 => (),
                    _ => return Err(()),
                };

                if let Err(_) = stdout.flush() {
                    return Err(());
                }
            }
            _ => return Err(()),
        }

        Ok(())
    }

    pub fn execute(&mut self) {
        let mut current_execution = self.instructions[self.position];
        while let Ok(_) = self.execute_instruction(current_execution) {
            self.position += 1;
            current_execution = self.instructions[self.position];
        }
    }

    // pub fn execute_once

    // pub fn execute(&mut self) -> Result<(), ()> {
    //     match self.instructions[self.instruction_position] {
    //         Token::Read => {
    //             let mut buf: [u8; 1] = [0];
    //             io::stdin().read_exact(&mut buf);
    //             self.memory.set(buf[0])
    //         }
    //         Token::Write => print!("{}", self.memory.get()),
    //         Token::FowardPos => self.memory.forward(),
    //         Token::BackwardPos => self.memory.backward(),
    //         Token::Increment => self.memory.set(self.memory.get().wrapping_add(1)),
    //         Token::Decrement => self.memory.set(self.memory.get().wrapping_sub(1)),
    //         Token::RepeatIn => {
    //             if self.memory.get() == 0 {
    //                 if let Some(repeatIn) =
    //                     self.instructions.iter().position(|x| *x == Token::RepeatIn)
    //                 {
    //                 }
    //             } else {
    //                 self.last_loop_list.push(self.script_position)
    //             }
    //         }
    //         Token::RepeatOut => {}
    //         _ => unimplemented!(),
    //     }
    //     Ok(())
    // }
}

*******/
