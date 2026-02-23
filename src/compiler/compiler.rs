use std::rc::Rc;

use crate::{
    language::{nodes::Node, token::TokenKind},
    virtual_machine::{
        builtin::BUILTINS, inst::Inst, value::{Function, Value}
    },
};

pub struct Compiler {
    pub constants: Vec<Value>,
    pub offset: usize,
    // pub functions: Vec<FunctionDef>,
    pub instructions: Vec<Inst>,
    // pub function_entries: HashMap<Rc<String>, usize>,
}

impl Compiler {
    pub fn new() -> Self {
        Self {
            offset: 0,
            constants: vec![],
            instructions: vec![],
            // functions: vec![],
            // function_entries: HashMap::new(),
        }
    }

    pub fn compile_node(&mut self, node: &Node) -> Vec<Inst> {
        match node {
            Node::NIL => vec![Inst::PUSH(Value::NIL)],
            Node::Variable(x) => vec![Inst::LOAD(x.clone())],
            Node::NumberLiteral(x) => vec![Inst::PUSH(Value::Number(*x))],
            Node::BooleanLiteral(x) => vec![Inst::PUSH(Value::Bool(*x))],
            Node::StringLiteral(x) => {
                if let Some((idx, _)) = self
                    .constants
                    .iter()
                    .enumerate()
                    .find(|(_, thing)| thing == &&Value::String(Rc::new(x.clone())))
                {
                    return vec![Inst::LOADCONST(idx)];
                }
                self.constants.push(Value::String(Rc::new(x.to_string())));
                return vec![Inst::LOADCONST(self.constants.len() - 1)];
            }

            Node::ExprStmt(x) => [self.compile_node(&*x), vec![Inst::POP]].concat(),

            Node::BinOp { left, right, op } => self.compile_bin_op(left, right, op),

            Node::LetStatement {
                name,
                value,
                is_const,
            } => self.compile_let(name, value, *is_const),

            Node::SetVariable { target, value } => self.compile_set_variable(target, value),

            Node::FunctionCall { target, args } => self.compile_function_call(target, args),

            Node::FunctionDefinition {
                name,
                return_type,
                args,
                block,
            } => self.compile_function_def(name, return_type, args, block),

            Node::ReturnStatement(value) => self.compile_return(value),

            Node::IfStatement {
                condition,
                block,
                elifs,
                else_block,
            } => self.compile_if_statement(condition, block, elifs, else_block),

            Node::Block { body } => [
                vec![Inst::PUSHSCOPE],
                body.iter()
                    .map(|x| self.compile_node(x))
                    .collect::<Vec<_>>()
                    .concat(),
                vec![Inst::POPSCOPE],
            ]
            .concat(),

            _ => panic!("Unknown node: `{node:?}`"),
        }
    }
}

impl Compiler {
    pub fn compile_bin_op(
        &mut self,
        left: &Box<Node>,
        right: &Box<Node>,
        op: &TokenKind,
    ) -> Vec<Inst> {
        let mut values = [self.compile_node(&**left), self.compile_node(&**right)].concat();
        values.push(match op {
            TokenKind::PLUS => Inst::ADD,
            TokenKind::MINUS => Inst::SUB,
            TokenKind::STAR => Inst::MUL,
            TokenKind::SLASH => Inst::DIV,

            TokenKind::GT => Inst::GT,
            TokenKind::LT => Inst::LT,
            TokenKind::GE => Inst::GE,
            TokenKind::LE => Inst::LE,

            TokenKind::EQ => Inst::EQ,
            TokenKind::NEQ => Inst::NEQ,

            TokenKind::AND => Inst::AND,
            TokenKind::OR => Inst::OR,

            _ => panic!("Cannot compile unknown bin-op: `{op:?}`"),
        });
        values
    }

    pub fn compile_let(
        &mut self,
        name: &Rc<String>,
        value: &Option<Box<Node>>,
        is_const: bool,
    ) -> Vec<Inst> {
        let mut instructions = if let Some(val) = value {
            self.compile_node(&**val)
        } else {
            vec![Inst::PUSH(Value::NIL)]
        };

        if is_const {
            instructions.push(Inst::DEFCONST(name.clone()));
        } else {
            instructions.push(Inst::STORELOCAL(name.clone()));
        }
        instructions
    }

    pub fn compile_function_call(&mut self, target: &Box<Node>, args: &Vec<Node>) -> Vec<Inst> {
        let mut instructions = vec![];

        for i in args {
            instructions.extend(self.compile_node(i));
        }

        if let Node::Variable(x) = &**target
            && BUILTINS.contains(&&***x)
        {
			instructions.push(Inst::CALLBUILTIN(x.clone(), args.len()));
        } else {
            instructions.extend(self.compile_node(&**target));
            instructions.push(Inst::CALL);
        }

        instructions
    }

    pub fn compile_if_statement(
        &mut self,
        condition: &Box<Node>,
        block: &Box<Node>,
        elifs: &Vec<(Node, Node)>,
        else_block: &Option<Box<Node>>,
    ) -> Vec<Inst> {
        let offset = self.offset;
        let mut instructions = vec![];
        let base = self.instructions.len();

        // helper: patches a jump placeholder
        let patch = |instrs: &mut Vec<Inst>, global_idx: usize, target: usize| {
            instrs[global_idx - base] = match instrs[global_idx - base] {
                Inst::JUMPIFFALSE(_) => Inst::JUMPIFFALSE(target + offset),
                Inst::JUMP(_) => Inst::JUMP(target + offset),
                _ => unreachable!(),
            };
        };

        // all jumps that must skip to the END after any branch executes
        let mut end_jumps: Vec<usize> = vec![];

        // ---------- IF ----------
        instructions.extend(self.compile_node(condition));

        let if_false_jump = base + instructions.len();
        instructions.push(Inst::JUMPIFFALSE(0 + self.offset));

        instructions.extend(self.compile_node(block));

        let if_end_jump = base + instructions.len();
        instructions.push(Inst::JUMP(0 + self.offset));
        end_jumps.push(if_end_jump);

        // next branch starts here
        let mut next_branch_start = base + instructions.len();
        patch(&mut instructions, if_false_jump, next_branch_start);

        // ---------- ELIFS ----------
        for (elif_cond, elif_block) in elifs {
            instructions.extend(self.compile_node(elif_cond));

            let elif_false_jump = base + instructions.len();
            instructions.push(Inst::JUMPIFFALSE(0 + self.offset));

            instructions.extend(self.compile_node(elif_block));

            let elif_end_jump = base + instructions.len();
            instructions.push(Inst::JUMP(0 + self.offset));
            end_jumps.push(elif_end_jump);

            next_branch_start = base + instructions.len();
            patch(&mut instructions, elif_false_jump, next_branch_start);
        }

        // ---------- ELSE ----------
        if let Some(else_block) = else_block {
            instructions.extend(self.compile_node(else_block));
        }

        // ---------- PATCH ALL END JUMPS ----------
        let end = base + instructions.len();
        for j in end_jumps {
            patch(&mut instructions, j, end);
        }

        instructions
    }

    pub fn compile_set_variable(&mut self, target: &Box<Node>, value: &Box<Node>) -> Vec<Inst> {
        let mut instructions = vec![];

        if let Node::Variable(x) = &**target {
            instructions.extend(self.compile_node(&**value));
            instructions.push(Inst::SETVAR(x.clone()));
        } else {
            panic!("Cannot set equal a value to `{:?}`", **target);
        }

        instructions
    }

    pub fn compile_return(&mut self, value: &Option<Box<Node>>) -> Vec<Inst> {
        if let Some(val) = value {
            [self.compile_node(val), vec![Inst::RETURN]].concat()
        } else {
            vec![Inst::PUSH(Value::NIL), Inst::RETURN]
        }
    }

    pub fn compile_function_def(
        &mut self,
        name: &Rc<String>,
        _return_type: &Option<Rc<String>>,
        args: &Vec<(Rc<String>, Option<Rc<String>>, Option<Node>)>,
        block: &Box<Node>,
    ) -> Vec<Inst> {
        self.offset += 4;
        let block_instructions = self.compile_node(block);
        self.offset -= 4;

        let mut pre_call_instructions = vec![];
        for (arg_name, _, default_value) in args {
			pre_call_instructions.push(Inst::DEFAULTNIL);

			if let Some(def) = default_value {
				pre_call_instructions.extend(self.compile_node(def));
				pre_call_instructions.push(Inst::DEFAULT);
			}
			
			pre_call_instructions.push(Inst::STORELOCAL(arg_name.clone()));
        }

        let mut func_instructions = [pre_call_instructions, block_instructions].concat();
        func_instructions.push(Inst::PUSH(Value::NIL));
        func_instructions.push(Inst::RETURN);

        let mut instructions = vec![];

        instructions.push(Inst::PUSH(Value::Function(Function::new(
            self.instructions.len() + self.offset + 3,
			args.len()
        ))));
        instructions.push(Inst::STORELOCAL(name.clone()));
        instructions.push(Inst::JUMP(
            self.instructions.len()
                + func_instructions.len()
                + instructions.len()
                + self.offset
                + 1,
        ));

        [instructions, func_instructions].concat()
    }
}
