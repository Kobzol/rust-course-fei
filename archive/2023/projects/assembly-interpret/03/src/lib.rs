use crate::instruction::{ExecutionError, Program};
use std::fmt::Display;
use std::ops::{Add, Sub};

pub mod cpu;
pub mod instruction;
pub mod memory;
pub mod parser;

use crate::cpu::Cpu;
pub use cpu::CpuBuilder;
pub use instruction::Instruction;
pub use memory::{ReadableExpr, WritableExpr};
pub use parser::{annotate_error, parse_program};

pub trait Value: Clone + Add<Output = Self> + Sub<Output = Self> + Display {
    fn parse(input: &str) -> Result<Self, String>;
}

impl Value for u8 {
    fn parse(input: &str) -> Result<Self, String> {
        input
            .parse::<u8>()
            .map_err(|error| format!("Cannot parse u8: {error:?}"))
    }
}

pub fn execute_instruction<T: Value>(
    cpu: &mut Cpu<T>,
    instruction: Instruction<T>,
) -> Result<(), ExecutionError> {
    match instruction {
        Instruction::Set { src, dest } => {
            let src_val = cpu.read(src)?;
            cpu.write(dest, src_val)?;
        }
        Instruction::Add { src, dest } => {
            let src_val = cpu.read(src)?;
            let dest_val = cpu.read(dest.as_read())?;
            let result = src_val + dest_val;
            cpu.write(dest, result)?;
        }
        Instruction::Sub { src, dest } => {
            let src_val = cpu.read(src)?;
            let dest_val = cpu.read(dest.as_read())?;
            let result = dest_val - src_val;
            cpu.write(dest, result)?;
        }
        Instruction::Print { expr } => {
            let value = cpu.read(expr)?;
            println!("{value}");
        }
    };

    Ok(())
}

pub fn execute_program<T: Value>(
    cpu: &mut Cpu<T>,
    program: Program<T>,
) -> Result<(), ExecutionError> {
    for inst in program.instructions {
        execute_instruction(cpu, inst)?;
    }

    Ok(())
}
