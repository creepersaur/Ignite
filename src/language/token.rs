#![allow(unused)]

#[derive(Debug, Clone, Copy)]
pub struct TokenRange {
    pub start: usize,
    pub end: usize,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub range: TokenRange,
}

impl Token {
    pub fn new(kind: TokenKind, range: TokenRange) -> Self {
        Self { kind, range }
    }

    pub fn get_text<'a>(&self, source: &'a str) -> &'a str {
        &source[self.range.start..self.range.end]
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    NEWLINE,
    Identifier,

    // LITERALS
    IntLiteral(i32),
    FloatLiteral(f32),
    StringLiteral(String),
    BooleanLiteral(bool),
    CharLiteral(char),

    // Keywords
    RETURN,
    LET,
    FUNC,
    FOR,
    BREAK,
    CONTINUE,
    WHILE,
    IF,
    ELSE,

    // Punctuation
    LPAREN, // Parenthesis ()
    RPAREN,
    LBRACK, // Brackets []
    RBRACK,
    LBRACE, // Braces {}
    RBRACE,
    PLUS,
    MINUS,
    STAR,
    SLASH,
    MOD,
    POW,
    DOLLAR,
    HASH,
    AT,
    BANG,
    EQUAL, // ==
    GR, // >
    LT, // <
    GE, // >=
    LE, // <=
	NE, // !=
	OR, // or, ||
	AND, // and, &&
    COLON,
    SEMI,
    QUESTION,
    TILDA,
    BACKTICK,
    PIPE,
    DOT,
    COMMA,
    ARROW,
}
