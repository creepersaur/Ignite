use std::rc::Rc;
use bincode::{Encode, Decode};

use crate::virtual_machine::value::Value;

#[allow(unused)]
#[derive(Encode, Decode, Debug, Clone)]
pub enum Inst {
    EXIT,        // ✅
	NOP,         // ✅
    PRINT,       // ✅
    PUSH(Value), // ✅
    POP,         // ✅

    ADD, // ✅
    SUB, // ✅
    MUL, // ✅
    DIV, // ✅

	GT,  // ✅
	LT,  // ✅
	GE,  // ✅
	LE,  // ✅
	EQ,  // ✅
	NEQ, // ✅

    LOADCONST(usize),        // ✅
    LOADGLOBAL(Rc<String>),  // ✅
    STOREGLOBAL(Rc<String>), // ✅
    
	PUSHSCOPE,
	POPSCOPE,
	LOADLOCAL(Rc<String>),
    STORELOCAL(Rc<String>),

    JUMP(usize),        // ✅
    JUMPIFFALSE(usize), // ✅

    CALL,
    RETURN,
}
