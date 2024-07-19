//! Library for the Brainfuck interpreter.
//!
//! You can run BrainFuck code in two ways:
//!
//! 1. Using the [`BrainFuckInterpreter`] to get more control over
//! the interpreter.
//! 2. Or simply run code with [`evaluate`].
//!
//! ## Example
//! ```
//! # fn main() -> Result<(), brainfuck::BadExpressionError> {
//! use brainfuck::evaluate;
//!
//! let hello_world_program = r###"
//!    >++++++++[<+++++++++>-]<.     H
//!    >++++[<+++++++>-]<+.          e
//!    +++++++..                     ll
//!    +++.                          o
//!    >>++++++[<+++++++>-]<++.      {comma}
//!    ------------.                 {space}
//!    >++++++[<+++++++++>-]<+.      W
//!    <.                            o
//!    +++.                          r
//!    ------.                       l
//!    --------.                     d
//!    >>>++++[<++++++++>-]<+.       !
//!    >>>>>++++++++++.              {line-break}
//! "###;
//!
//! evaluate(hello_world_program)?;
//! # Ok(())
//! # }
//! ```

mod execution;
mod interpreter;
mod syntax;
mod token;

pub use interpreter::{evaluate, BrainFuckInterpreter};
pub use syntax::{BadExpressionError, Expression, SyntaxTree};
pub use token::Token;
