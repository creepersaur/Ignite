use std::collections::HashMap;

use inkwell::{
    builder::Builder,
    context::{self, Context},
    module::Module,
    types::{BasicTypeEnum, FunctionType},
    values::{BasicValueEnum, IntValue, PointerValue},
};

use crate::language::nodes::Node;

#[derive(Debug)]
pub struct Compiler<'ctx> {
    pub context: &'ctx Context,
    pub module: Module<'ctx>,
    pub builder: Builder<'ctx>,

    pub vars: HashMap<String, PointerValue<'ctx>>,
}

impl<'ctx> Compiler<'ctx> {
    pub fn new(context: &'ctx Context) -> Self {
        let module = context.create_module("ignite_module");
        let builder = context.create_builder();
        let vars = HashMap::new();

        Self {
            context,
            module,
            builder,
            vars,
        }
    }

    pub fn compile_program(&mut self, ast: &Node) {
        match ast {
            Node::LetStatement { name, value } => {
                self.compile_variable(name);
            }
            _ => {}
        }
    }

    pub fn compile_let(&mut self, name: &str, value: &Node) {
        let val = self.compile_expr(value);
        let ptr = self
            .builder
            .build_alloca(val.get_type(), name)
            .expect("Failed to allocate memory for variable");

        self.builder
            .build_store(ptr, val)
            .expect("Failed to store a variable");

        self.vars.insert(name.to_string(), ptr);
    }

    fn compile_variable(&mut self, name: &str) -> inkwell::values::BasicValueEnum<'ctx> {
        let ptr = self
            .vars
            .get(name)
            .expect(&format!("Unknown variable `{}`", name));

        self.builder.build_load(*ptr, name)
    }

    pub fn compile_expr(&mut self, node: &Node) -> inkwell::values::BasicValueEnum<'ctx> {
        match node {
            Node::IntLiteral(x) => self.context.i32_type().const_int(*x as u64, true).into(),
            Node::Variable(name) => self.compile_variable(name),
            Node::BinOp { left, right, op } => {
                let lhs = self.compile_expr(left);
                let rhs = self.compile_expr(right);
                self.build_binop(lhs, rhs, *op)
            }
            _ => unimplemented!(),
        }
    }

    pub fn get_type(&self, datatype: &str) -> Option<BasicTypeEnum<'ctx>> {
        match datatype {
            "int" => Some(self.context.i32_type().into()),
            "float" => Some(self.context.f64_type().into()),
            "bool" => Some(self.context.bool_type().into()),
            _ => None,
        }
    }
}
