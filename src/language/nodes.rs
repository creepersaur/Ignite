use crate::language::token::TokenKind;

#[derive(Debug, Clone)]
pub enum Node {
    // LITERALS
    IntLiteral(i32),
    FloatLiteral(f32),
    StringLiteral(String),
    CharLiteral(String),
    BooleanLiteral(bool),
    Null,

    // OPERATORS
    BinOp {
        left: Box<Node>,
        right: Box<Node>,
        op: TokenKind,
    },
	UnaryOp {
		op: TokenKind,
		right: Box<Node>
	},

	// STATEMENTS
	LetStatement {
		name: String,
		value: Box<Node>
	},

	Block {
		body: Vec<Node>
	},

    FuncDeclaration {
        name: String,
        params: Box<Node>,
        return_type: Box<Node>,
        block: Box<Node>,
    }
}
