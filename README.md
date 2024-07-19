# BrainFuck Interpreter

A simple brainfuck implementation in rust. It just works.

### Example

```bash
bf examples/hello_world.bf
# Hello world!
```

## Installation

Ensure that [cargo](https://cargo) is installed and run:

```
cargo install --git https://github.com/4ngelf/brainfuck
```

## Library

You can the interpreter as a library to parse, manipulate and execute
BrainFuck code.

### Example

```rust
use brainfuck::{BrainFuckInterpreter, BadExpressionError};

fn main() -> Result<(), BadExpressionError> {
    let mut bf = BrainFuckInterpreter::new();
    bf.feed_string(">++++[<+++++++>-]<---.....")?;

    bf.execute();

    Ok(())
}
```
