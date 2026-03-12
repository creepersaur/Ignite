use std::{cell::RefCell, fmt::Debug, rc::Rc};

use crate::{
    lib_function,
    misc::to_index::to_index,
    rc,
    virtual_machine::{
        traits::member_accessible::IMemberAccessible, types::function::TFunction, value::Value,
        vm::VM,
    },
};
use bincode::{Decode, Encode};

#[derive(Encode, Decode, Clone, PartialEq, PartialOrd)]
pub struct TList {
    pub values: Rc<RefCell<Vec<Value>>>,
}

impl TList {
    pub fn new(values: Rc<RefCell<Vec<Value>>>) -> Self {
        Self { values }
    }
}

impl Debug for TList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("List").unwrap();
        Ok(())
    }
}

// MEMBER ACCESS
impl IMemberAccessible for TList {
    fn get_member(&self, _vm: &mut VM, member: &Value) -> Value {
        if let Value::Number(index) = member {
            let len = self.values.borrow().len();
            let target_index = to_index(*index, len);

            return self.values.borrow()[target_index].clone();
        }

        if let Value::String(member) = member {
            let functions = [
                "len", "push", "insert", "remove", "map", "pop", "clear", "append", "concat",
                "copy", "count", "sort", "reverse", "fill", "rep", "push_n",
            ];

            if functions.contains(&member.as_str()) {
                return lib_function!(self, "list", member, 1, Value::List);
            }
        }

        panic!("Cannot get member `{}` on {self:?}", member.to_string(true));
    }

    fn set_member(&self, member: &Value, value: Value) {
        if let Value::Number(index) = member {
            let len = self.values.borrow().len();
            let target_index = to_index(*index, len);

            self.values.borrow_mut()[target_index] = value;
            return;
        }

        panic!("Cannot set member `{}` on {self:?}", member.to_string(true));
    }
}
