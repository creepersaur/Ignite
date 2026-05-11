use crate::{
    lib_function, namespace_lib_function, rc,
    virtual_machine::{
        namespaces::namespace::TNamespace, types::function::TFunction, value::Value,
    },
};
use std::cell::RefCell;

pub fn std_fs() -> Value {
    let mut namespace = TNamespace::new("FS", true);

    namespace_lib_function!(namespace, "fs", "read");
    namespace_lib_function!(namespace, "fs", "write");

    Value::Namespace(rc!(RefCell::new(namespace)))
}
