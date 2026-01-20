use crate::language::token::TokenKind;

#[derive(Debug, Clone)]
pub enum Node {
    // LITERALS
    IntLiteral(i32),
    FloatLiteral(f32),
    StringLiteral(String),
    CharLiteral(String),
    BooleanLiteral(bool),

    // OPERATORS
    BinOp {
        left: Box<Node>,
        right: Box<Node>,
        op: TokenKind,
    },
	UnaryOp {
		op: TokenKind,
		right: Box<Node>
	}
}
