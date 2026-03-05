use crate::{
    compiler::compiler::Compiler,
    language::{lexer::Lexer, parser::Parser},
    virtual_machine::vm::VM,
};
#[allow(unused)]
use std::{error::Error, fs, rc::Rc};

mod compiler;
mod language;
mod macros;
mod misc;
mod virtual_machine;

fn main() -> Result<(), Box<dyn Error>> {
    let text = fs::read_to_string("sigma.ignite")?;
    let mut lex = Lexer::new(&text);
    let tokens = lex.get_tokens();
    // println!("{:#?}", tokens);

    /////////////////////
    // NODES
    /////////////////////

    let mut parser = Parser::new(text, tokens);
    let mut nodes = vec![];

    while parser.current().is_ok() {
        nodes.push(parser.parse()?);
    }

    // println!("Generated Nodes:");
    // println!("---------------------------");
    // println!("{nodes:#?}");

    /////////////////////
    // COMPILER
    /////////////////////

    let mut compiler = Compiler::new();
    for i in nodes.iter() {
        compiler.compile_node(i);
    }

    let mut vm = VM::new();
    vm.constants = compiler.constants;
    vm.instructions = compiler.instructions;

    // println!("\nCompiled instructions:");
    // println!("---------------------------");
    // vm.print_instructions();

    println!("\nRunning:");
    println!("---------------------------");
    vm.run(false, false);

    Ok(())
}
