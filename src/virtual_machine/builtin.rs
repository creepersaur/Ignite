use crate::virtual_machine::{value::Value, vm::VM};

pub const BUILTINS: [&str; 2] = ["print", "println"];

pub fn builtin_print(vm: &mut VM, arg_count: usize, newline: bool) {
    let args = (0..arg_count).map(|_| vm.pop()).collect::<Vec<_>>();
    let string = args
        .iter()
        .rev()
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join(" ");

    if newline {
		println!("{string}");
	} else {
		print!("{string}");
	}

    vm.stack.push(Value::NIL);
}
