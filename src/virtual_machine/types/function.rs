use bincode::{Decode, Encode};
use std::{
    fmt::Debug,
    hash::{Hash, Hasher},
    rc::Rc,
};

use crate::{hash_u64, virtual_machine::value::Value};

#[derive(Encode, Decode, Clone, PartialEq, PartialOrd)]
pub struct TFunction {
    pub entry: usize,
    pub handler: Option<(u64, u64)>,
    pub this: Option<Box<Value>>,
}

impl TFunction {
    pub fn new(entry: usize) -> Self {
        Self {
            entry,
            handler: None,
            this: None,
        }
    }
    pub fn with_lib(
        lib: Rc<str>,
        method: Rc<str>,
        this: Option<Box<Value>>,
    ) -> Self {
        Self {
            entry: 0,
            handler: Some((hash_u64!(lib.as_ref()), hash_u64!(method.as_ref()))),
            this,
        }
    }
}

impl Debug for TFunction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("entry: {}", self.entry)).unwrap();
        Ok(())
    }
}

impl Eq for TFunction {}

impl Hash for TFunction {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.entry.hash(state);
        self.handler.hash(state);
        self.this.hash(state);
    }
}
