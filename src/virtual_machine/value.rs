use bincode::{Decode, Encode};
use std::{fmt::Debug, rc::Rc};

#[allow(unused)]
#[derive(Encode, Decode, Debug, Clone, PartialEq)]
pub enum Value {
    NIL,
    Number(f32),
    Bool(bool),
    String(Rc<String>),
    Function(Function),
}

#[derive(Encode, Decode, Clone, PartialEq)]
pub struct Function {
    pub entry: usize,
	pub args: usize,
}

impl Function {
    pub fn new(entry: usize, args: usize) -> Self {
        Self { entry, args }
    }
}

impl Debug for Function {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("entry: {}", self.entry)).unwrap();
        Ok(())
    }
}

impl Value {
    pub fn to_string(&self) -> String {
        match self {
            Self::NIL => "nil".to_string(),
            Self::Number(x) => x.to_string(),
            Self::Bool(x) => x.to_string(),
            Self::String(x) => x.to_string(),
            Self::Function(_) => String::from("<function>"),
        }
    }

    pub fn is_truthy(&self) -> bool {
        match self {
            Self::NIL => false,
            Self::Bool(x) => *x,
            _ => true,
        }
    }
}
