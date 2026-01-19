use crate::language::lexer::Lexer;

mod language;

fn main() {
    let text = "'c'";
    let mut lex = Lexer::new(text);

    println!(
        "{:?}",
        lex.get_tokens().iter().map(|x| x.kind).collect::<Vec<_>>()
    );
}
