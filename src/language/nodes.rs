use crate::language::token::TokenKind;

#[allow(unused)]
#[derive(Debug, Clone)]
pub enum Node {
    // LITERALS
    NULL,
    Variable(String),

    IntLiteral(i32),
    FloatLiteral(f32),
    StringLiteral(String),
    CharLiteral(String),
    BooleanLiteral(bool),

    // COLLECTIONS
    ListNode(Vec<Node>),

    // OPERATORS
    BinOp {
        left: Box<Node>,
        right: Box<Node>,
        op: TokenKind,
    },
    UnaryOp {
        op: TokenKind,
        right: Box<Node>,
    },

    // STATEMENTS
    // use a Box<Node> because Node by itself makes it infinitely big
    LetStatement {
        name: String,
        value: Option<Box<Node>>,
    },

    Block {
        body: Vec<Node>,
    },

    // Arguments are in the tuple -> (name: String, type: Option<String>)
    FunctionDefinition {
        name: String,
        return_type: Option<String>,
        args: Vec<(String, Option<String>)>,
        block: Box<Node>,
    },

    FunctionCall {
        target: Box<Node>,
        args: Vec<Node>,
    },

    ReturnStatement(Option<Box<Node>>),
    BreakStatement,
    ContinueStatement,

    // Loops
    WhileLoop {
        condition: Box<Node>,
        block: Box<Node>,
    },
    RangedForLoop {
        var_name: String,
        start: Box<Node>,
        end: Box<Node>,
        step: Option<Box<Node>>,
    },
    IterableForLoop {
        var_name: String,
        iterable: Box<Node>,
    },

    // Logical Operations
    IfStatement {
        condition: Box<Node>,
        block: Box<Node>,
        elifs: Vec<(Node, Node)>,
        else_block: Option<Box<Node>>,
    },

    // Class stuff
    ClassDef {
        name: String,
		interfaces: Vec<String>,
		let_statements: Vec<Node>,
        functions: Vec<Node>,
    },

	StructDef {
		name: String,
		types: Vec<(String, String)> // (key, type)
	},

	InterfaceDef {
        name: String,
		let_statements: Vec<Node>,
        functions: Vec<Node>,
	}
}
