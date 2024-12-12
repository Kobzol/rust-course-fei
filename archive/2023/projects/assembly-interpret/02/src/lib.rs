use crate::cpu::{Cpu, ReadExprError, WriteExprError};

mod cpu;
mod instruction;
mod memory;
mod parsing;

pub use cpu::CpuBuilder;
pub use instruction::Instruction;
pub use memory::{ReadExpr, WriteExpr};
pub use parsing::{parse, ParseErrorKind};

#[derive(Debug)]
pub enum ExecuteError {
    ReadExpr(ReadExprError),
    WriteExpr(WriteExprError),
}

impl From<ReadExprError> for ExecuteError {
    fn from(value: ReadExprError) -> Self {
        Self::ReadExpr(value)
    }
}

impl From<WriteExprError> for ExecuteError {
    fn from(value: WriteExprError) -> Self {
        Self::WriteExpr(value)
    }
}

pub fn execute_instruction(cpu: &mut Cpu, inst: Instruction) -> Result<(), ExecuteError> {
    match inst {
        Instruction::Print(expr) => {
            let value = cpu.read(expr)?;
            println!("{value}");
        }
        Instruction::Set { dest, src } => {
            let value = cpu.read(src)?;
            cpu.write(dest, value)?;
        }
    };
    Ok(())
}
