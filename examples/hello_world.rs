fn main() -> Result<(), brainfuck::BadExpressionError> {
    let program = include_str!("./hello_world.bf");
    brainfuck::evaluate(program)
}
