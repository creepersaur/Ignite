use std::{collections::HashMap, fmt::Debug, rc::Rc};

use crate::virtual_machine::{traits::member_accessible::IMemberAccessible};
use bincode::{Decode, Encode};

#[derive(Encode, Decode, Clone, PartialEq)]
pub struct TStructDef {
	pub name: String,
    pub fields: Rc<HashMap<String, String>>,
}

impl PartialOrd for TStructDef {
    fn partial_cmp(&self, _other: &Self) -> Option<std::cmp::Ordering> {
        None
    }
}

impl TStructDef {
    pub fn new(name: String, fields: Rc<HashMap<String, String>>) -> Self {
        Self { name, fields }
    }
}

impl Debug for TStructDef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("TStructDef:{}", self.name)).unwrap();
        Ok(())
    }
}

// MEMBER ACCESS
impl IMemberAccessible for TStructDef {

}
