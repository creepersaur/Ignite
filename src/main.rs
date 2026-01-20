use std::fs;

use crate::language::{lexer::Lexer, parser::Parser};

mod language;

fn main() {
    let text = fs::read_to_string("sigma.ignite").unwrap();
    let mut lex = Lexer::new(&text);
    let tokens = lex.get_tokens();

    println!("{:#?}", tokens.iter().map(|x| x.kind).collect::<Vec<_>>());

    let mut parser = Parser::new(text, tokens);

    println!("{:#?}", parser.parse());
}
