use std::rc::Rc;

use crate::language::token::TokenKind;

#[allow(unused)]
#[derive(Debug, Clone)]
pub enum Node {
	// Expressions vs Statements
	ExprStmt(Box<Node>),

    // LITERALS
    NIL,
    Variable(Rc<String>),

    NumberLiteral(f32),
    StringLiteral(String),
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
    LetStatement {
        name: Rc<String>,
        value: Option<Box<Node>>,
		is_const: bool,
    },

	SetVariable {
		target: Box<Node>,
		value: Box<Node>,
	},

    Block {
        body: Vec<Node>,
    },

    // Arguments are in the tuple -> (name: String, type: Option<String>)
    FunctionDefinition {
        name: Rc<String>,
        return_type: Option<Rc<String>>,
        args: Vec<(Rc<String>, Option<Rc<String>>, Option<Node>)>,
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
        var_name: Rc<String>,
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
		interfaces: Vec<Rc<String>>,
		let_statements: Vec<Node>,
        functions: Vec<Node>,
    },

	StructDef {
		name: String,
		types: Vec<(Rc<String>, Rc<String>)> // (key, type)
	},

	InterfaceDef {
        name: Rc<String>,
		let_statements: Vec<Node>,
        functions: Vec<Node>,
	}
}
