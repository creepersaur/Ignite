use crate::language::token::TokenKind;

#[allow(unused)]
#[derive(Debug, Clone)]
pub enum Node {
    // LITERALS
    IntLiteral(i32),
    FloatLiteral(f32),
    StringLiteral(String),
    CharLiteral(String),
    BooleanLiteral(bool),

	Variable(String),
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
	// use a Box<Node> because Node by itself makes it infinitely big
	LetStatement {
		name: String,
		value: Box<Node>
	},

	Block {
		body: Vec<Node>
	},

	// Arguments are in the tuple -> (name: String, type: Option<String>)
	FunctionDefinition {
		name: String,
		return_type: Option<String>,
		args: Vec<(String, Option<String>)>,
		block: Box<Node>
	},

	ReturnStatement(Option<Box<Node>>),
	BreakStatement,
	ContinueStatement,

	// Loops
	WhileLoop {
		condition: Box<Node>,
		block: Box<Node>
	},
	RangedForLoop {
		var_name: String,
		start: Box<Node>,
		end: Box<Node>,
		step: Option<Box<Node>>
	},
	IterableForLoop {
		var_name: String,
		iterable: Box<Node>,
	},
}
