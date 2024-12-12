use std::collections::HashMap;

use crate::cpu::{ReadError, WriteError};
use crate::memory::{ReadableExpr, WritableExpr};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
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
    JumpIfNotZero {
        src: ReadableExpr<T>,
        label: String,
    },
}

#[derive(Debug)]
pub enum ExecutionError {
    Read(ReadError),
    Write(WriteError),
    InstructionOutOfBounds(u64),
    InvalidLabel(String),
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

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Program<T> {
    pub instructions: Vec<Instruction<T>>,
    pub labels: HashMap<String, usize>,
}

impl<T> Program<T> {
    pub fn new(instructions: Vec<Instruction<T>>, labels: HashMap<String, usize>) -> Self {
        Self {
            instructions,
            labels,
        }
    }

    pub fn resolve_label(&self, label: &str) -> Option<usize> {
        self.labels.get(label).copied()
    }
}
