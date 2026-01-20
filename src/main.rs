use crate::language::lexer::Lexer;

mod language;

fn main() {
    let text = "100.5";
    let mut lex = Lexer::new(text);

    println!(
        "{:?}",
        lex.get_tokens().iter().map(|x| x.get_text(text)).collect::<Vec<_>>()
    );
}
