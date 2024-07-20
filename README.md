# 💀 BrainFuck Interpreter &emsp; [![Build Status]][CI Actions] [![Docs Status]][Docs Actions] [![Latest Release]][Releases]

[Build Status]: https://img.shields.io/github/actions/workflow/status/4ngelf/brainfuck/ci.yaml?branch=main&label=Tests
[CI Actions]: https://github.com/4ngelf/brainfuck/actions/workflows/ci.yaml
[Docs Status]: https://img.shields.io/github/actions/workflow/status/4ngelf/brainfuck/documentation.yaml?label=Docs
[Docs Actions]: https://4ngelf.github.io/brainfuck/
[Latest Release]: https://img.shields.io/github/v/release/4ngelf/brainfuck?label=latest
[Releases]: https://github.com/4ngelf/brainfuck/releases/latest

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

You can use the interpreter as a library to parse, manipulate and execute
BrainFuck code.

See the [documentation here][Docs.url].

[Docs.url]: https://4ngelf.github.io/brainfuck/

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
