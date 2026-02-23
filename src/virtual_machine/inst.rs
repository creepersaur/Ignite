use bincode::{Decode, Encode};
use std::rc::Rc;

use crate::virtual_machine::value::Value;

#[allow(unused)]
#[derive(Encode, Decode, Debug, Clone)]
pub enum Inst {
    EXIT,        // ✅
    NOP,         // ✅
    PRINT,       // ✅
    DEFAULT,     // ✅
    DEFAULTNIL,  // ✅
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
    AND, // ✅
    OR,  // ✅
    NOT, // ✅

    LOADCONST(usize),        // ✅
    LOADGLOBAL(Rc<String>),  // ✅
    STOREGLOBAL(Rc<String>), // ✅
	DEFCONST(Rc<String>),    // ✅
	SETVAR(Rc<String>),      // ✅

    PUSHSCOPE,              // ✅
    POPSCOPE,               // ✅
    LOADLOCAL(Rc<String>),  // ✅
    STORELOCAL(Rc<String>), // ✅

    // Load from local or global
    LOAD(Rc<String>), // ✅

    JUMP(usize),        // ✅
    JUMPIFFALSE(usize), // ✅

    CALL,
    CALLBUILTIN(Rc<String>, usize),
    RETURN,
}
