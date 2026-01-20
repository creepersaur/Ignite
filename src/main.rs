use crate::language::{lexer::Lexer, parser::Parser};

mod language;

fn main() {
    let text = String::from("(1 + 1) * 5");
    let mut lex = Lexer::new(&text);
	let tokens = lex.get_tokens();
	
	let mut parser = Parser::new(text, tokens);

    println!(
        "{:#?}",
        parser.parse_expression()
    );
}
