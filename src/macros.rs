#[macro_export]
macro_rules! rc {
    ($x: expr) => {
        std::rc::Rc::new($x)
    };
}

#[macro_export]
macro_rules! hashmap {
	{$($key:expr => $value:expr),*} => {{
		let x = std::collections::HashMap::new();

		$(
			x.insert($key, $value);
		)*

		x
	}}
}

#[macro_export]
macro_rules! lib_function {
    ($this:expr, $lib:expr, $member:expr, $args:expr) => {
        Value::Function(TFunction::with_lib(
            rc!($lib.to_string()),
            $member.clone(),
            $args,
            Some(Box::new(Value::List($this.clone()))),
        ))
    };
}
