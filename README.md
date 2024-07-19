# BrainFuck Interpreter

A simple brainfuck implementation in rust. It just works.

### Example

```bash
bf examples/hello_world.bf
# Hello world!
```

## Installation

Ensure that [cargo][cargo.url] is installed and run:

[cargo.url]: https://doc.rust-lang.org/cargo/getting-started/installation.html

```sh
cargo install --git https://github.com/4ngelf/brainfuck
```

## Library

You can the interpreter as a library to parse, manipulate and execute
BrainFuck code.

You can view the [documentation here][docs.url].

[docs.url]: https://4ngelf.github.io/brainfuck/

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
