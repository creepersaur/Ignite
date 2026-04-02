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
    namespace_lib_function!(namespace, "math", "abs");
    namespace_lib_function!(namespace, "math", "ceil");
    namespace_lib_function!(namespace, "math", "floor");
    namespace_lib_function!(namespace, "math", "trunc");
    namespace_lib_function!(namespace, "math", "fract");
    namespace_lib_function!(namespace, "math", "sign");
    namespace_lib_function!(namespace, "math", "sqrt");
    namespace_lib_function!(namespace, "math", "cbrt");
    namespace_lib_function!(namespace, "math", "exp");
    namespace_lib_function!(namespace, "math", "exp2");
    namespace_lib_function!(namespace, "math", "ln");
    namespace_lib_function!(namespace, "math", "log2");
    namespace_lib_function!(namespace, "math", "log10");
    namespace_lib_function!(namespace, "math", "recip");

    // Two-argument
    namespace_lib_function!(namespace, "math", "pow");
    namespace_lib_function!(namespace, "math", "log"); // log(x, base)
    namespace_lib_function!(namespace, "math", "hypot");
    namespace_lib_function!(namespace, "math", "atan2");
    namespace_lib_function!(namespace, "math", "min");
    namespace_lib_function!(namespace, "math", "max");
    namespace_lib_function!(namespace, "math", "clamp"); // clamp(x, min, max)
    namespace_lib_function!(namespace, "math", "copysign");

    // Trig
    namespace_lib_function!(namespace, "math", "sin");
    namespace_lib_function!(namespace, "math", "cos");
    namespace_lib_function!(namespace, "math", "tan");
    namespace_lib_function!(namespace, "math", "sinh");
    namespace_lib_function!(namespace, "math", "cosh");
    namespace_lib_function!(namespace, "math", "tanh");

    // Inverse trig
    namespace_lib_function!(namespace, "math", "asin");
    namespace_lib_function!(namespace, "math", "acos");
    namespace_lib_function!(namespace, "math", "atan");
    namespace_lib_function!(namespace, "math", "asinh");
    namespace_lib_function!(namespace, "math", "acosh");
    namespace_lib_function!(namespace, "math", "atanh");

    // Conversion
    namespace_lib_function!(namespace, "math", "to_radians");
    namespace_lib_function!(namespace, "math", "to_degrees");
    namespace_lib_function!(namespace, "math", "to_celsius");
    namespace_lib_function!(namespace, "math", "to_fahrenheit");

    // Predicates — return Bool
    namespace_lib_function!(namespace, "math", "is_nan");
    namespace_lib_function!(namespace, "math", "is_infinite");
    namespace_lib_function!(namespace, "math", "is_finite");

	// Rounding
    namespace_lib_function!(namespace, "math", "round");
    namespace_lib_function!(namespace, "math", "round_to");

    namespace_lib_function!(namespace, "math", "lerp");
    namespace_lib_function!(namespace, "math", "inv_lerp");
    namespace_lib_function!(namespace, "math", "remap");
    namespace_lib_function!(namespace, "math", "smoothstep");

    namespace_lib_function!(namespace, "math", "gcd");
    namespace_lib_function!(namespace, "math", "lcm");
    namespace_lib_function!(namespace, "math", "factorial");
    namespace_lib_function!(namespace, "math", "is_prime");

    namespace_lib_function!(namespace, "math", "fma");
    namespace_lib_function!(namespace, "math", "mid");
    namespace_lib_function!(namespace, "math", "wrap");
    namespace_lib_function!(namespace, "math", "snap");
    namespace_lib_function!(namespace, "math", "ping_pong");

	// Geometry
    namespace_lib_function!(namespace, "math", "dist");

    Value::Namespace(rc!(RefCell::new(namespace)))
}
