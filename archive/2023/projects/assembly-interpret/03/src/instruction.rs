use crate::cpu::{ReadError, WriteError};
use crate::memory::{ReadableExpr, WritableExpr};

#[derive(Debug, Clone)]
pub enum Instruction<T> {
    Set {
        dest: WritableExpr,
        src: ReadableExpr<T>,
    },
    Print {
        expr: ReadableExpr<T>,
    },
    Add {
        dest: WritableExpr,
        src: ReadableExpr<T>,
    },
    Sub {
        dest: WritableExpr,
        src: ReadableExpr<T>,
    },
}

#[derive(Debug)]
pub enum ExecutionError {
    Read(ReadError),
    Write(WriteError),
    InstructionOutOfBounds(u64),
}

impl From<ReadError> for ExecutionError {
    fn from(value: ReadError) -> Self {
        Self::Read(value)
    }
}

impl From<WriteError> for ExecutionError {
    fn from(value: WriteError) -> Self {
        Self::Write(value)
    }
}

#[derive(Debug)]
pub struct Program<T> {
    pub instructions: Vec<Instruction<T>>,
}

impl<T> Program<T> {
    pub fn new(instructions: Vec<Instruction<T>>) -> Self {
        Self { instructions }
    }
}
