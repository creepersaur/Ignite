use crate::{
    lib_function, namespace_lib_function, rc,
    virtual_machine::{
        namespaces::namespace::TNamespace, types::function::TFunction, value::Value,
    },
};
use std::cell::RefCell;

pub fn std_io() -> Value {
    let mut namespace = TNamespace::new("IO", true);

    // Input
    namespace_lib_function!(namespace, "io", "read_line");
    namespace_lib_function!(namespace, "io", "read_line_raw");

    // Output
    namespace_lib_function!(namespace, "io", "clear");
    namespace_lib_function!(namespace, "io", "reset");
    namespace_lib_function!(namespace, "io", "write");
    namespace_lib_function!(namespace, "io", "write_line");

    Value::Namespace(rc!(RefCell::new(namespace)))
}
