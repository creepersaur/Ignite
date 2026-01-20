#[derive(Debug, Clone, Copy)]
pub enum Operator {
    PLUS,
    MINUS,
    MUL,
    DIV,
    MOD,
    POW,
}

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
        add: Operator,
    },
}
