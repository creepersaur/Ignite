use crate::{
    lib_function, namespace_lib_function, rc,
    virtual_machine::{
        namespaces::namespace::TNamespace, types::function::TFunction, value::Value,
    },
};
use std::cell::RefCell;

pub fn std_math() -> Value {
    let mut namespace = TNamespace::new("Math", true);

    // Constants
    namespace.set_const("PI", Value::Number(std::f64::consts::PI));
    namespace.set_const("E", Value::Number(std::f64::consts::E));
    namespace.set_const("TAU", Value::Number(std::f64::consts::TAU));
    namespace.set_const("SQRT2", Value::Number(std::f64::consts::SQRT_2));
    namespace.set_const("LN2", Value::Number(std::f64::consts::LN_2));
    namespace.set_const("LN10", Value::Number(std::f64::consts::LN_10));
    namespace.set_const("LOG2E", Value::Number(std::f64::consts::LOG2_E));
    namespace.set_const("LOG10E", Value::Number(std::f64::consts::LOG10_E));
    namespace.set_const("INFINITY", Value::Number(f64::INFINITY));
    namespace.set_const("NEG_INFINITY", Value::Number(f64::NEG_INFINITY));
    namespace.set_const("NAN", Value::Number(f64::NAN));

    // Basic
    namespace_lib_function!(namespace, "math", "abs", 1, Value::Number);
    namespace_lib_function!(namespace, "math", "ceil", 1, Value::Number);
    namespace_lib_function!(namespace, "math", "floor", 1, Value::Number);
    namespace_lib_function!(namespace, "math", "trunc", 1, Value::Number);
    namespace_lib_function!(namespace, "math", "fract", 1, Value::Number);
    namespace_lib_function!(namespace, "math", "sign", 1, Value::Number);
    namespace_lib_function!(namespace, "math", "sqrt", 1, Value::Number);
    namespace_lib_function!(namespace, "math", "cbrt", 1, Value::Number);
    namespace_lib_function!(namespace, "math", "exp", 1, Value::Number);
    namespace_lib_function!(namespace, "math", "exp2", 1, Value::Number);
    namespace_lib_function!(namespace, "math", "ln", 1, Value::Number);
    namespace_lib_function!(namespace, "math", "log2", 1, Value::Number);
    namespace_lib_function!(namespace, "math", "log10", 1, Value::Number);
    namespace_lib_function!(namespace, "math", "recip", 1, Value::Number);

    // Two-argument
    namespace_lib_function!(namespace, "math", "pow", 2, Value::Number);
    namespace_lib_function!(namespace, "math", "log", 2, Value::Number); // log(x, base)
    namespace_lib_function!(namespace, "math", "hypot", 2, Value::Number);
    namespace_lib_function!(namespace, "math", "atan2", 2, Value::Number);
    namespace_lib_function!(namespace, "math", "min", 2, Value::Number);
    namespace_lib_function!(namespace, "math", "max", 2, Value::Number);
    namespace_lib_function!(namespace, "math", "clamp", 3, Value::Number); // clamp(x, min, max)
    namespace_lib_function!(namespace, "math", "copysign", 2, Value::Number);

    // Trig
    namespace_lib_function!(namespace, "math", "sin", 1, Value::Number);
    namespace_lib_function!(namespace, "math", "cos", 1, Value::Number);
    namespace_lib_function!(namespace, "math", "tan", 1, Value::Number);
    namespace_lib_function!(namespace, "math", "sinh", 1, Value::Number);
    namespace_lib_function!(namespace, "math", "cosh", 1, Value::Number);
    namespace_lib_function!(namespace, "math", "tanh", 1, Value::Number);

    // Inverse trig
    namespace_lib_function!(namespace, "math", "asin", 1, Value::Number);
    namespace_lib_function!(namespace, "math", "acos", 1, Value::Number);
    namespace_lib_function!(namespace, "math", "atan", 1, Value::Number);
    namespace_lib_function!(namespace, "math", "asinh", 1, Value::Number);
    namespace_lib_function!(namespace, "math", "acosh", 1, Value::Number);
    namespace_lib_function!(namespace, "math", "atanh", 1, Value::Number);

    // Conversion
    namespace_lib_function!(namespace, "math", "to_radians", 1, Value::Number);
    namespace_lib_function!(namespace, "math", "to_degrees", 1, Value::Number);
    namespace_lib_function!(namespace, "math", "to_celsius", 1, Value::Number);
    namespace_lib_function!(namespace, "math", "to_fahrenheit", 1, Value::Number);

    // Predicates — return Bool
    namespace_lib_function!(namespace, "math", "is_nan", 1, Value::Bool);
    namespace_lib_function!(namespace, "math", "is_infinite", 1, Value::Bool);
    namespace_lib_function!(namespace, "math", "is_finite", 1, Value::Bool);

	// Rounding
    namespace_lib_function!(namespace, "math", "round", 1, Value::Number);
    namespace_lib_function!(namespace, "math", "round_to", 2, Value::Number);

    namespace_lib_function!(namespace, "math", "lerp", 3, Value::Number);
    namespace_lib_function!(namespace, "math", "inv_lerp", 3, Value::Number);
    namespace_lib_function!(namespace, "math", "remap", 5, Value::Number);
    namespace_lib_function!(namespace, "math", "smoothstep", 3, Value::Number);

    namespace_lib_function!(namespace, "math", "gcd", 2, Value::Number);
    namespace_lib_function!(namespace, "math", "lcm", 2, Value::Number);
    namespace_lib_function!(namespace, "math", "factorial", 1, Value::Number);
    namespace_lib_function!(namespace, "math", "is_prime", 1, Value::Bool);

    namespace_lib_function!(namespace, "math", "fma", 3, Value::Number);
    namespace_lib_function!(namespace, "math", "mid", 2, Value::Number);
    namespace_lib_function!(namespace, "math", "wrap", 3, Value::Number);
    namespace_lib_function!(namespace, "math", "snap", 2, Value::Number);
    namespace_lib_function!(namespace, "math", "ping_pong", 2, Value::Number);

	// Geometry
    namespace_lib_function!(namespace, "math", "dist", 4, Value::Number);

    Value::Namespace(rc!(RefCell::new(namespace)))
}
