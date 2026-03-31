use crate::virtual_machine::{value::Value, vm::VM};

pub trait Library {
    fn get_name(&self) -> &str;

    fn get_function(&self, name: u64) -> Box<dyn Fn(&mut VM) -> Value>;
}
