use std::{error::Error, fs};

// use inkwell::context::{self, Context};

use crate::language::{lexer::Lexer, parser::Parser};

mod language;

fn main() -> Result<(), Box<dyn Error>> {
    let text = fs::read_to_string("sigma.ignite")?;
    let mut lex = Lexer::new(&text);
    let tokens = lex.get_tokens();

    println!(
        "{:#?}",
        tokens.iter().map(|x| x.kind.clone()).collect::<Vec<_>>()
    );

    let mut parser = Parser::new(text, tokens);
    let mut nodes = vec![];

    while parser.peek().is_some() {
        nodes.push(parser.parse()?);
    }

    println!("{:#?}", nodes);

    // let ctx = Context::create();
    // let compiler = Compiler::new(&ctx);

    Ok(())
}
