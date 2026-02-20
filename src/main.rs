#[allow(unused)]
use std::{fs, error::Error, rc::Rc};
use crate::language::{lexer::Lexer, parser::Parser};

mod language;
mod virtual_machine;

fn main() -> Result<(), Box<dyn Error>> {
    let text = fs::read_to_string("sigma.ignite")?;
    let mut lex = Lexer::new(&text);
    let tokens = lex.get_tokens();

    let mut parser = Parser::new(text, tokens);
    let mut nodes = vec![];

    while parser.current().is_ok() {
        nodes.push(parser.parse()?);
    }

	println!("{nodes:#?}");

    Ok(())
}
