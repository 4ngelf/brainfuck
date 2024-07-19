use crate::{
    execution::MemoryContext,
    syntax::{BadExpressionError, SyntaxTree},
    token::Token,
};

/// A Brainfuck interpreter
///
/// This is the main driver for executing Brainfuck code.
/// The interpreter is initialized first with some memory
/// (default 32K), then parses the given BrainFuck code into a
/// valid syntax tree, which then executes step by step.
///
/// # Example
/// ```
/// # use brainfuck::{BrainFuckInterpreter, BadExpressionError};
/// # fn main() -> Result<(), BadExpressionError> {
/// #
/// let mut bf = BrainFuckInterpreter::new();
/// bf.feed_string(">++++[<+++++++>-]<---.....")?;
///
/// bf.execute();
/// #
/// # Ok(())
/// # }
///
/// ```
#[derive(Debug)]
pub struct BrainFuckInterpreter {
    memory: MemoryContext,
    instructions: SyntaxTree,
}

impl BrainFuckInterpreter {
    /// Starts a new interpreter with default memory size of 32K
    pub fn new() -> Self {
        BrainFuckInterpreter {
            memory: MemoryContext::new(),
            instructions: SyntaxTree::new(),
        }
    }

    /// Starts a new interpreter with given memory size
    pub fn with_memory_size(size: usize) -> Self {
        BrainFuckInterpreter {
            memory: MemoryContext::with_capacity(size),
            instructions: SyntaxTree::new(),
        }
    }

    /// Feeds the interpreter some code as stream of bytes
    ///
    /// Updates the internal syntax tree only if the code is valid
    pub fn feed<T>(&mut self, bytes: T) -> Result<(), BadExpressionError>
    where
        T: IntoIterator<Item = u8>,
    {
        let tokens = bytes.into_iter().map(Token::from);
        let tree = SyntaxTree::parse_tokens(tokens)?;
        self.instructions.extend(tree);

        Ok(())
    }

    /// Feeds the interpreter some code
    ///
    /// Updates the internal syntax tree only if the code is valid
    pub fn feed_string(&mut self, code: &str) -> Result<(), BadExpressionError> {
        self.feed(code.bytes())
    }

    /// Get this interpreter [`SyntaxTree`]
    pub fn syntax_tree(&self) -> &SyntaxTree {
        &self.instructions
    }

    /// Clears the internal syntax tree
    pub fn clear(&mut self) {
        self.instructions.clear();
    }

    /// Executes the internal syntax tree
    pub fn execute(&mut self) {
        for expr in self.instructions.iter() {
            self.memory.execute_expression(expr);
        }
    }
}

impl std::default::Default for BrainFuckInterpreter {
    fn default() -> Self {
        Self::new()
    }
}

/// Run some Brainfuck code
///
/// This is a fast way to initialize, feed and execute a [`BrainFuckInterpreter`].
pub fn evaluate(code: &str) -> Result<(), BadExpressionError> {
    let mut interpreter = BrainFuckInterpreter::new();
    interpreter.feed_string(code)?;
    interpreter.execute();

    Ok(())
}
