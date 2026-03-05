use bincode::{Decode, Encode};
use std::{fmt::Debug, rc::Rc};

use crate::virtual_machine::types::{function::TFunction, list::TList};

#[allow(unused)]
#[derive(Encode, Decode, Debug, Clone, PartialEq, PartialOrd)]
pub enum Value {
    NIL,
    Number(f32),
    Bool(bool),
    String(Rc<String>),

    Function(TFunction),

    // Collections
    List(TList),

    Range {
        start: Box<Value>,
        end: Box<Value>,
        step: Box<Value>,
        inclusive: bool,
    },
}

impl Value {
    pub fn to_string(&self, debug: bool) -> String {
        match self {
            Self::NIL => "nil".to_string(),
            Self::Number(x) => x.to_string(),
            Self::Bool(x) => x.to_string(),
            Self::String(x) => {
                if debug {
                    format!("\"{}\"", x.to_string())
                } else {
                    x.to_string()
                }
            }

            Self::Function(_) => String::from("<function>"),

            Self::List(list) => format!(
                "[{}]",
                list.values
                    .borrow()
                    .iter()
                    .map(|x| if let Value::List(v) = x {
                        if list.values.as_ptr() == v.values.as_ptr() {
                            String::from("[...]")
                        } else {
                            x.to_string(true)
                        }
                    } else {
                        x.to_string(true)
                    })
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
			Self::Range {start, end, step, inclusive} => format!(
				"Range<{}..{}{}..{}>",
				start.to_string(true),
				if *inclusive { "=" } else {""},
				end.to_string(true),
				step.to_string(true),
			)
        }
    }

    pub fn is_truthy(&self) -> bool {
        match self {
            Self::NIL => false,
            Self::Bool(x) => *x,
            _ => true,
        }
    }

    pub fn as_number(&self) -> f32 {
        if let Value::Number(x) = self {
            *x
        } else {
            panic!("Cannot convert `{self:?}` to number.")
        }
    }
}
