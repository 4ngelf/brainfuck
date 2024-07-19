use crate::syntax::Expression;

/// The default amount of memory allowed for a BrainFuck program
const DEFAULT_BRAINFUCK_STACK_SIZE: usize = 32_768;

/// It holds the memory of the program
pub type Memory = Vec<u8>;

/// This represents the running context of a BrainFuck program
#[derive(Debug, Hash)]
pub struct MemoryContext {
    memory: Memory,
    pointer_index: usize,
}

impl MemoryContext {
    pub fn new() -> Self {
        Self::with_capacity(DEFAULT_BRAINFUCK_STACK_SIZE)
    }

    pub fn with_capacity(capacity: usize) -> Self {
        let memory = vec![0; capacity];
        let pointer_index = capacity / 2;

        MemoryContext {
            memory,
            pointer_index,
        }
    }

    #[inline]
    pub fn move_forward(&mut self) {
        let pointer_index = self.pointer_index.wrapping_add(1);

        if pointer_index < self.memory.len() {
            self.pointer_index = pointer_index;
        } else {
            self.pointer_index = 0;
        }
    }

    #[inline]
    pub fn move_backward(&mut self) {
        let (pointer_index, overflow) = self.pointer_index.overflowing_sub(1);

        if !overflow {
            self.pointer_index = pointer_index;
        } else {
            self.pointer_index = self.memory.len() - 1;
        }
    }

    #[inline]
    pub fn set(&mut self, value: u8) {
        self.memory[self.pointer_index] = value;
    }

    #[inline]
    pub fn get(&self) -> u8 {
        self.memory[self.pointer_index]
    }

    #[inline]
    pub fn increment(&mut self) {
        self.set(self.get().wrapping_add(1))
    }

    #[inline]
    pub fn decrement(&mut self) {
        self.set(self.get().wrapping_sub(1))
    }

    #[inline]
    pub fn execute_expression(&mut self, expr: &Expression) {
        match expr {
            Expression::Increment => self.increment(),
            Expression::Decrement => self.decrement(),
            Expression::Forward => self.move_forward(),
            Expression::Backward => self.move_backward(),
            Expression::Input => self.set(get_byte()),
            Expression::Output => print_byte(self.get()),
            Expression::Loop(expressions) => {
                while self.get() != 0 {
                    for expr in expressions {
                        self.execute_expression(expr);
                    }
                }
            }
        }
    }
}

impl std::default::Default for MemoryContext {
    fn default() -> Self {
        Self::new()
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
    print!("{}", character as char);
    let _ = io::stdout().flush();
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::syntax::SyntaxTree as ET;

    fn tiny_memory() -> MemoryContext {
        MemoryContext::with_capacity(6)
    }

    #[test]
    fn memory_pointer_movement() {
        let mut m = tiny_memory();

        m.move_backward();
        assert_eq!(m.pointer_index, 2);

        m.move_forward();
        m.move_forward();
        m.move_forward();
        assert_eq!(m.pointer_index, 5);

        m.move_forward();
        assert_eq!(m.pointer_index, 0);

        m.move_backward();
        assert_eq!(m.pointer_index, 5);
    }

    #[test]
    fn memory_wraps_on_increment_or_decrement() {
        let mut m = tiny_memory();

        m.set(u8::MAX);
        assert_eq!(m.get(), u8::MAX);

        m.increment();
        assert_eq!(m.get(), 0);

        m.decrement();
        assert_eq!(m.get(), u8::MAX);
    }

    #[test]
    fn memory_execute_expression() {
        let mut m = tiny_memory();
        let exprs = "++>--<<<++[>+++<-]+++<+<---".parse::<ET>().unwrap();

        for expr in exprs {
            m.execute_expression(&expr);
        }

        assert_eq!(m.memory, vec![1, 3, 6, 2, u8::MAX - 1, u8::MAX - 2]);
    }
}
