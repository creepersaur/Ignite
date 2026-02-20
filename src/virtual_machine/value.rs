use bincode::{Decode, Encode};
use std::{fmt::Debug, rc::Rc};

#[allow(unused)]
#[derive(Encode, Decode, Debug, Clone)]
pub enum Value {
    Number(f32),
    Bool(bool),
    String(Rc<String>),
    Function(Function),
}

#[derive(Encode, Decode, Clone)]
pub struct Function {
    pub entry: usize,
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
            Self::Number(x) => x.to_string(),
            Self::Bool(x) => x.to_string(),
            Self::String(x) => x.to_string(),
            Self::Function(_) => String::from("<function>"),
        }
    }

    pub fn is_truthy(&self) -> bool {
        match self {
            Self::Bool(x) => *x,
            _ => true,
        }
    }
}
