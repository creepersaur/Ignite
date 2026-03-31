use crate::virtual_machine::{libs::lib::Library, value::Value, vm::VM};
use std::rc::Rc;

pub struct MathLib;

impl MathLib {
    fn pop_num(vm: &mut VM, fn_name: &str) -> f64 {
        match vm.pop() {
            Value::Number(x) => x,
            _ => panic!("math.{fn_name} expects a number"),
        }
    }

    // Basic
    fn abs(vm: &mut VM) -> Value {
        Value::Number(Self::pop_num(vm, "abs").abs())
    }
    fn ceil(vm: &mut VM) -> Value {
        Value::Number(Self::pop_num(vm, "ceil").ceil())
    }
    fn floor(vm: &mut VM) -> Value {
        Value::Number(Self::pop_num(vm, "floor").floor())
    }
    fn round(vm: &mut VM) -> Value {
        Value::Number(Self::pop_num(vm, "round").round())
    }
    fn trunc(vm: &mut VM) -> Value {
        Value::Number(Self::pop_num(vm, "trunc").trunc())
    }
    fn fract(vm: &mut VM) -> Value {
        Value::Number(Self::pop_num(vm, "fract").fract())
    }
    fn sign(vm: &mut VM) -> Value {
        Value::Number(Self::pop_num(vm, "sign").signum())
    }
    fn sqrt(vm: &mut VM) -> Value {
        Value::Number(Self::pop_num(vm, "sqrt").sqrt())
    }
    fn cbrt(vm: &mut VM) -> Value {
        Value::Number(Self::pop_num(vm, "cbrt").cbrt())
    }
    fn exp(vm: &mut VM) -> Value {
        Value::Number(Self::pop_num(vm, "exp").exp())
    }
    fn exp2(vm: &mut VM) -> Value {
        Value::Number(Self::pop_num(vm, "exp2").exp2())
    }
    fn ln(vm: &mut VM) -> Value {
        Value::Number(Self::pop_num(vm, "ln").ln())
    }
    fn log2(vm: &mut VM) -> Value {
        Value::Number(Self::pop_num(vm, "log2").log2())
    }
    fn log10(vm: &mut VM) -> Value {
        Value::Number(Self::pop_num(vm, "log10").log10())
    }
    fn recip(vm: &mut VM) -> Value {
        Value::Number(Self::pop_num(vm, "recip").recip())
    }

    // Two-argument (note: args are popped in reverse — second arg first)
    fn pow(vm: &mut VM) -> Value {
        let exp = Self::pop_num(vm, "pow");
        let base = Self::pop_num(vm, "pow");
        Value::Number(base.powf(exp))
    }
    fn log(vm: &mut VM) -> Value {
        let base = Self::pop_num(vm, "log");
        let x = Self::pop_num(vm, "log");
        Value::Number(x.log(base))
    }
    fn hypot(vm: &mut VM) -> Value {
        let b = Self::pop_num(vm, "hypot");
        let a = Self::pop_num(vm, "hypot");
        Value::Number(a.hypot(b))
    }
    fn atan2(vm: &mut VM) -> Value {
        let x = Self::pop_num(vm, "atan2");
        let y = Self::pop_num(vm, "atan2");
        Value::Number(y.atan2(x))
    }
    fn min(vm: &mut VM) -> Value {
        let b = Self::pop_num(vm, "min");
        let a = Self::pop_num(vm, "min");
        Value::Number(a.min(b))
    }
    fn max(vm: &mut VM) -> Value {
        let b = Self::pop_num(vm, "max");
        let a = Self::pop_num(vm, "max");
        Value::Number(a.max(b))
    }
    fn clamp(vm: &mut VM) -> Value {
        let max = Self::pop_num(vm, "clamp");
        let min = Self::pop_num(vm, "clamp");
        let x = Self::pop_num(vm, "clamp");
        Value::Number(x.clamp(min, max))
    }
    fn copysign(vm: &mut VM) -> Value {
        let sign = Self::pop_num(vm, "copysign");
        let x = Self::pop_num(vm, "copysign");
        Value::Number(x.copysign(sign))
    }

    // Trig
    fn sin(vm: &mut VM) -> Value {
        Value::Number(Self::pop_num(vm, "sin").sin())
    }
    fn cos(vm: &mut VM) -> Value {
        Value::Number(Self::pop_num(vm, "cos").cos())
    }
    fn tan(vm: &mut VM) -> Value {
        Value::Number(Self::pop_num(vm, "tan").tan())
    }
    fn sinh(vm: &mut VM) -> Value {
        Value::Number(Self::pop_num(vm, "sinh").sinh())
    }
    fn cosh(vm: &mut VM) -> Value {
        Value::Number(Self::pop_num(vm, "cosh").cosh())
    }
    fn tanh(vm: &mut VM) -> Value {
        Value::Number(Self::pop_num(vm, "tanh").tanh())
    }

    // Inverse trig
    fn asin(vm: &mut VM) -> Value {
        Value::Number(Self::pop_num(vm, "asin").asin())
    }
    fn acos(vm: &mut VM) -> Value {
        Value::Number(Self::pop_num(vm, "acos").acos())
    }
    fn atan(vm: &mut VM) -> Value {
        Value::Number(Self::pop_num(vm, "atan").atan())
    }
    fn asinh(vm: &mut VM) -> Value {
        Value::Number(Self::pop_num(vm, "asinh").asinh())
    }
    fn acosh(vm: &mut VM) -> Value {
        Value::Number(Self::pop_num(vm, "acosh").acosh())
    }
    fn atanh(vm: &mut VM) -> Value {
        Value::Number(Self::pop_num(vm, "atanh").atanh())
    }

    // Conversion
    fn to_radians(vm: &mut VM) -> Value {
        Value::Number(Self::pop_num(vm, "to_radians").to_radians())
    }
    fn to_degrees(vm: &mut VM) -> Value {
        Value::Number(Self::pop_num(vm, "to_degrees").to_degrees())
    }
    fn to_celsius(vm: &mut VM) -> Value {
        let f = Self::pop_num(vm, "to_celsius");
        Value::Number((f - 32.0) * 5.0 / 9.0)
    }

    fn to_fahrenheit(vm: &mut VM) -> Value {
        let c = Self::pop_num(vm, "to_fahrenheit");
        Value::Number((c * 9.0 / 5.0) + 32.0)
    }

    // Predicates
    fn is_nan(vm: &mut VM) -> Value {
        Value::Bool(Self::pop_num(vm, "is_nan").is_nan())
    }
    fn is_infinite(vm: &mut VM) -> Value {
        Value::Bool(Self::pop_num(vm, "is_infinite").is_infinite())
    }
    fn is_finite(vm: &mut VM) -> Value {
        Value::Bool(Self::pop_num(vm, "is_finite").is_finite())
    }

    // Rounding
    fn round_to(vm: &mut VM) -> Value {
        let decimals = Self::pop_num(vm, "round_to");
        let x = Self::pop_num(vm, "round_to");
        let factor = 10f64.powi(decimals as i32);
        Value::Number((x * factor).round() / factor)
    }

    // Interpolation
    fn lerp(vm: &mut VM) -> Value {
        let t = Self::pop_num(vm, "lerp");
        let b = Self::pop_num(vm, "lerp");
        let a = Self::pop_num(vm, "lerp");
        Value::Number(a + (b - a) * t)
    }
    fn inv_lerp(vm: &mut VM) -> Value {
        let x = Self::pop_num(vm, "inv_lerp");
        let b = Self::pop_num(vm, "inv_lerp");
        let a = Self::pop_num(vm, "inv_lerp");
        Value::Number((x - a) / (b - a))
    }
    fn remap(vm: &mut VM) -> Value {
        let out_max = Self::pop_num(vm, "remap");
        let out_min = Self::pop_num(vm, "remap");
        let in_max = Self::pop_num(vm, "remap");
        let in_min = Self::pop_num(vm, "remap");
        let x = Self::pop_num(vm, "remap");
        let t = (x - in_min) / (in_max - in_min);
        Value::Number(out_min + t * (out_max - out_min))
    }
    fn smoothstep(vm: &mut VM) -> Value {
        let x = Self::pop_num(vm, "smoothstep");
        let edge1 = Self::pop_num(vm, "smoothstep");
        let edge0 = Self::pop_num(vm, "smoothstep");
        let t = ((x - edge0) / (edge1 - edge0)).clamp(0.0, 1.0);
        Value::Number(t * t * (3.0 - 2.0 * t))
    }

    // Number theory
    fn gcd(vm: &mut VM) -> Value {
        let mut b = Self::pop_num(vm, "gcd") as u64;
        let mut a = Self::pop_num(vm, "gcd") as u64;
        while b != 0 {
            let t = b;
            b = a % b;
            a = t;
        }
        Value::Number(a as f64)
    }
    fn lcm(vm: &mut VM) -> Value {
        let b = Self::pop_num(vm, "lcm") as u64;
        let a = Self::pop_num(vm, "lcm") as u64;
        let mut tb = b;
        let mut ta = a;
        while tb != 0 {
            let t = tb;
            tb = ta % tb;
            ta = t;
        }
        Value::Number((a / ta * b) as f64)
    }
    fn factorial(vm: &mut VM) -> Value {
        let n = Self::pop_num(vm, "factorial") as u64;
        Value::Number((1..=n).product::<u64>() as f64)
    }
    fn is_prime(vm: &mut VM) -> Value {
        let n = Self::pop_num(vm, "is_prime") as u64;
        if n < 2 {
            return Value::Bool(false);
        }
        if n == 2 {
            return Value::Bool(true);
        }
        if n % 2 == 0 {
            return Value::Bool(false);
        }
        let limit = (n as f64).sqrt() as u64;
        Value::Bool((3..=limit).step_by(2).all(|i| n % i != 0))
    }

    // Numeric utilities
    fn fma(vm: &mut VM) -> Value {
        let c = Self::pop_num(vm, "fma");
        let b = Self::pop_num(vm, "fma");
        let a = Self::pop_num(vm, "fma");
        Value::Number(a.mul_add(b, c))
    }
    fn mid(vm: &mut VM) -> Value {
        let b = Self::pop_num(vm, "mid");
        let a = Self::pop_num(vm, "mid");
        Value::Number((a + b) / 2.0)
    }
    fn wrap(vm: &mut VM) -> Value {
        let max = Self::pop_num(vm, "wrap");
        let min = Self::pop_num(vm, "wrap");
        let x = Self::pop_num(vm, "wrap");
        let range = max - min;
        Value::Number(min + ((x - min) % range + range) % range)
    }
    fn snap(vm: &mut VM) -> Value {
        let step = Self::pop_num(vm, "snap");
        let x = Self::pop_num(vm, "snap");
        Value::Number((x / step).round() * step)
    }
    fn ping_pong(vm: &mut VM) -> Value {
        let length = Self::pop_num(vm, "ping_pong");
        let x = Self::pop_num(vm, "ping_pong");
        let t = x % (length * 2.0);
        Value::Number(if t > length { length * 2.0 - t } else { t })
    }

    // Geometry
    fn dist(vm: &mut VM) -> Value {
        let y2 = Self::pop_num(vm, "dist");
        let x2 = Self::pop_num(vm, "dist");
        let y1 = Self::pop_num(vm, "dist");
        let x1 = Self::pop_num(vm, "dist");
        Value::Number((x2 - x1).hypot(y2 - y1))
    }
}

// LIBRARY
impl Library for MathLib {
    fn get_name(&self) -> &str {
        "math"
    }

    fn get_function(&self, name: Rc<String>) -> Box<dyn Fn(&mut VM) -> Value> {
        match name.as_str() {
            // Basic
            "abs" => Box::new(Self::abs),
            "ceil" => Box::new(Self::ceil),
            "floor" => Box::new(Self::floor),
            "round" => Box::new(Self::round),
            "trunc" => Box::new(Self::trunc),
            "fract" => Box::new(Self::fract),
            "sign" => Box::new(Self::sign),
            "sqrt" => Box::new(Self::sqrt),
            "cbrt" => Box::new(Self::cbrt),
            "exp" => Box::new(Self::exp),
            "exp2" => Box::new(Self::exp2),
            "ln" => Box::new(Self::ln),
            "log2" => Box::new(Self::log2),
            "log10" => Box::new(Self::log10),
            "recip" => Box::new(Self::recip),

            // Two-argument
            "pow" => Box::new(Self::pow),
            "log" => Box::new(Self::log),
            "hypot" => Box::new(Self::hypot),
            "atan2" => Box::new(Self::atan2),
            "min" => Box::new(Self::min),
            "max" => Box::new(Self::max),
            "clamp" => Box::new(Self::clamp),
            "copysign" => Box::new(Self::copysign),

            // Trig
            "sin" => Box::new(Self::sin),
            "cos" => Box::new(Self::cos),
            "tan" => Box::new(Self::tan),
            "sinh" => Box::new(Self::sinh),
            "cosh" => Box::new(Self::cosh),
            "tanh" => Box::new(Self::tanh),

            // Inverse trig
            "asin" => Box::new(Self::asin),
            "acos" => Box::new(Self::acos),
            "atan" => Box::new(Self::atan),
            "asinh" => Box::new(Self::asinh),
            "acosh" => Box::new(Self::acosh),
            "atanh" => Box::new(Self::atanh),

            // Conversion
            "to_radians" => Box::new(Self::to_radians),
            "to_degrees" => Box::new(Self::to_degrees),
            "to_celsius" => Box::new(Self::to_celsius),
            "to_fahrenheit" => Box::new(Self::to_fahrenheit),

            // Predicates
            "is_nan" => Box::new(Self::is_nan),
            "is_infinite" => Box::new(Self::is_infinite),
            "is_finite" => Box::new(Self::is_finite),

            "round_to" => Box::new(Self::round_to),
            "lerp" => Box::new(Self::lerp),
            "inv_lerp" => Box::new(Self::inv_lerp),
            "remap" => Box::new(Self::remap),
            "smoothstep" => Box::new(Self::smoothstep),
            "gcd" => Box::new(Self::gcd),
            "lcm" => Box::new(Self::lcm),
            "factorial" => Box::new(Self::factorial),
            "is_prime" => Box::new(Self::is_prime),
            "fma" => Box::new(Self::fma),
            "mid" => Box::new(Self::mid),
            "wrap" => Box::new(Self::wrap),
            "snap" => Box::new(Self::snap),
            "ping_pong" => Box::new(Self::ping_pong),
            "dist" => Box::new(Self::dist),

            _ => panic!("Unknown function `{name}` on lib {}", self.get_name()),
        }
    }
}
