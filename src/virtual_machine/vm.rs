use crate::virtual_machine::{chunk::Chunk, inst::Inst, value::Value};
use bincode::config;
use simply_colored::*;
use std::{collections::HashMap, rc::Rc};

const ORANGE: &str = "\x1b[38;2;255;150;60m";

pub struct VM {
    pub pos: usize,
    pub instructions: Vec<Inst>,
    pub stack: Vec<Value>,
	pub function_stack: Vec<usize>,
    pub constants: Vec<Value>,
    pub globals: HashMap<Rc<String>, Value>,
    pub locals: Vec<HashMap<Rc<String>, Value>>,
}

#[allow(unused)]
impl VM {
    pub fn new() -> Self {
        Self {
            pos: 0,
            instructions: vec![],
            stack: Vec::with_capacity(100),
			function_stack: Vec::with_capacity(100),
            constants: Vec::with_capacity(100),
            globals: HashMap::new(),
            locals: vec![HashMap::new()],
        }
    }

    pub fn advance(&mut self) {
        self.pos += 1;
    }

    pub fn push_inst(&mut self, instruction: Inst) {
        self.instructions.push(instruction);
    }

    #[inline]
    pub fn pop(&mut self) -> Value {
        return self.stack.pop().unwrap();
    }

    #[inline]
    pub fn pop_two(&mut self) -> (Value, Value) {
        let right = self.stack.pop().unwrap();
        let left = self.stack.pop().unwrap();
        (left, right)
    }

    #[allow(unused)]
    pub fn push_constants(&mut self, constants: Vec<Value>) {
        self.constants.extend(constants);
    }

    pub fn print_instructions(&self) {
        for (i, v) in self.instructions.iter().enumerate() {
            if matches!(v, Inst::NOP) {
                println!("{MAGENTA}{i:>2}\t{BLACK}NOP{RESET}");
                continue;
            } else if matches!(v, Inst::EXIT) {
                println!("{MAGENTA}{i:>2}\t{RED}EXIT{RESET}");
                continue;
            }

            let s = format!("{:?}", v);

            // Split the first word (the opcode) from the rest
            let mut parts = s.splitn(2, '(');
            let opcode = parts.next().unwrap();
            let rest = parts.next().map_or("", |r| r);

            if rest.is_empty() {
                println!("{MAGENTA}{:>2}{RESET}\t{ORANGE}{}{RESET}", i, opcode);
            } else {
                println!(
                    "{MAGENTA}{:>2}{RESET}\t{ORANGE}{}{RESET}({BLUE}{}{RESET})",
                    i,
                    opcode,
                    &rest[0..rest.len() - 1]
                );
            }
        }
    }

    pub fn to_chunk(&self) -> Chunk {
        Chunk::new(self.constants.clone(), self.instructions.clone())
    }

    pub fn read_bytecode_file(&mut self, path: &str) {
        let bytecode_file = std::fs::read(path).unwrap();

        let decoded: (Chunk, _) =
            bincode::decode_from_slice(&bytecode_file, config::standard()).unwrap();

        self.constants = decoded.0.constants;
        self.instructions = decoded.0.instructions;
    }

    pub fn run(&mut self) {
        self.pos = 0;

        while self.pos < self.instructions.len() {
            let current = &self.instructions[self.pos];

            match current {
                Inst::EXIT => return,
                Inst::NOP => {}
                Inst::PRINT => println!("{}", self.pop().to_string()),
                Inst::POP => {
                    self.pop();
                }
                Inst::PUSH(value) => self.stack.push(value.clone()),

                Inst::ADD => {
                    if let (Value::Number(a), Value::Number(b)) = self.pop_two() {
                        self.stack.push(Value::Number(a + b));
                    } else {
                        panic!("ADD expects numbers");
                    }
                }
                Inst::SUB => {
                    if let (Value::Number(a), Value::Number(b)) = self.pop_two() {
                        self.stack.push(Value::Number(a - b));
                    } else {
                        panic!("SUB expects numbers");
                    }
                }
                Inst::MUL => {
                    if let (Value::Number(a), Value::Number(b)) = self.pop_two() {
                        self.stack.push(Value::Number(a * b));
                    } else {
                        panic!("MUL expects numbers");
                    }
                }
                Inst::DIV => {
                    if let (Value::Number(a), Value::Number(b)) = self.pop_two() {
                        self.stack.push(Value::Number(a / b));
                    } else {
                        panic!("DIV expects numbers");
                    }
                }

                Inst::EQ => {
                    let result = match self.pop_two() {
                        (Value::Number(x), Value::Number(y)) => x == y,
                        (Value::Bool(x), Value::Bool(y)) => x == y,

                        (Value::String(x), Value::String(y)) => Rc::ptr_eq(&x, &y) || *x == *y,

                        _ => panic!("Cannot EQ"),
                    };

                    self.stack.push(Value::Bool(result));
                }
                Inst::NEQ => {
                    let result = match self.pop_two() {
                        (Value::Number(x), Value::Number(y)) => x != y,
                        (Value::Bool(x), Value::Bool(y)) => x != y,

                        (Value::String(x), Value::String(y)) => !Rc::ptr_eq(&x, &y) || *x != *y,

                        _ => panic!("Cannot NEQ"),
                    };

                    self.stack.push(Value::Bool(result));
                }
                Inst::GT => {
                    if let (Value::Number(a), Value::Number(b)) = self.pop_two() {
                        self.stack.push(Value::Bool(a > b));
                    } else {
                        panic!("GT expects numbers");
                    }
                }
                Inst::LT => {
                    if let (Value::Number(a), Value::Number(b)) = self.pop_two() {
                        self.stack.push(Value::Bool(a < b));
                    } else {
                        panic!("LT expects numbers");
                    }
                }
                Inst::GE => {
                    if let (Value::Number(a), Value::Number(b)) = self.pop_two() {
                        self.stack.push(Value::Bool(a >= b));
                    } else {
                        panic!("GE expects numbers");
                    }
                }
                Inst::LE => {
                    if let (Value::Number(a), Value::Number(b)) = self.pop_two() {
                        self.stack.push(Value::Bool(a <= b));
                    } else {
                        panic!("LE expects numbers");
                    }
                }

                Inst::LOADCONST(idx) => self.stack.push(self.constants[*idx].clone()),
                Inst::STOREGLOBAL(name) => {
                    let name = name.clone();
                    let value = self.pop();
                    self.globals.insert(name, value);
                }
                Inst::LOADGLOBAL(name) => {
                    self.stack.push(
                        self.globals
                            .get(name)
                            .expect("Global `{name}` doesn't exist.")
                            .clone(),
                    );
                }

                Inst::PUSHSCOPE => self.locals.push(HashMap::new()),
                Inst::POPSCOPE => {
                    self.locals.pop();
                }
                Inst::STORELOCAL(name) => {
                    let name = name.clone();
                    let value = self.pop();
                    self.locals.last_mut().unwrap().insert(name, value);
                }
                Inst::LOADLOCAL(name) => {
                    let mut found = None;

                    for scope in self.locals.iter().rev() {
                        if let Some(val) = scope.get(name) {
                            found = Some(val.clone());
                            break;
                        }
                    }

                    if let Some(val) = found {
                        self.stack.push(val);
                    } else {
                        panic!("Unknown local variable: {name}");
                    }
                }

                Inst::JUMP(idx) => {
                    self.pos = *idx;
                    continue;
                }
                Inst::JUMPIFFALSE(idx) => {
                    let idx = *idx;
                    if !self.pop().is_truthy() {
                        self.pos = idx;
                        continue;
                    }
                }

				Inst::CALL => {
					let func = self.pop();
					if let Value::Function(f) = func {
						self.function_stack.push(self.pos);
						self.pos = f.entry;
						continue;
					} else {
						panic!("Tried calling non-function")
					}
				}
				Inst::RETURN => {
					if let Some(last) = self.function_stack.last() {
						self.pos = *last;
					}
				}

                _ => panic!("Unimplemented instruction: {current:?}"),
            }

            self.advance();
        }
    }
}
