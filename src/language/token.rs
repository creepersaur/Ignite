#![allow(unused)]

#[derive(Debug, Clone, Copy)]
pub struct TokenRange {
    pub start: usize,
    pub end: usize,
}

#[derive(Debug, Clone, Copy)]
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

#[derive(Debug, Clone, Copy)]
pub enum TokenKind {
    NEWLINE,
    Identifier,

    // LITERALS
    IntLiteral,
    FloatLiteral,
    StringLiteral,
    BooleanLiteral,
	CharLiteral,

    // Keywords
    RETURN,
    LET,
	FUNC,
	FOR,

	// Punctuation
    LPAREN, // Parenthesis ()
    RPAREN,
    LBRACK, // Brackets []
    RBRACK,
    LBRACE, // Braces {}
    RBRACE,
	PLUS,
	MINUS,
	MUL,
	DIV,
	MOD,
	POW,
	DOLLAR,
	HASH,
	AT,
	BANG,
	EQUAL,
	GR,
	LT,
	GE,
	LE,
	COLON,
	SEMI,
	QUESTION,
	TILDA,
	BACKTICK,
	PIPE,
}
