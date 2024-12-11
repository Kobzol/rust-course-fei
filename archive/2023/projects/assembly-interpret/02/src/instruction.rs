use crate::memory::{ReadExpr, WriteExpr};

#[derive(Debug)]
pub enum Instruction {
    Set { dest: WriteExpr, src: ReadExpr },
    Print(ReadExpr),
}
