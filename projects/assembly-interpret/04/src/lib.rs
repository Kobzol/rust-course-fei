use crate::instruction::{ExecutionError, Program};
use std::fmt::Display;
use std::ops::{Add, Sub};

pub mod cpu;
pub mod instruction;
pub mod memory;
pub mod parser;

use crate::cpu::Cpu;
use crate::instruction::ExecutionError::InvalidLabel;
pub use cpu::CpuBuilder;
pub use instruction::Instruction;
pub use memory::{ReadableExpr, WritableExpr};
pub use parser::parse_program;

pub trait Value: Clone + Add<Output = Self> + Sub<Output = Self> + Display {
    fn parse(input: &str) -> Result<Self, String>;
    fn is_zero(&self) -> bool;
}

impl Value for u8 {
    fn parse(input: &str) -> Result<Self, String> {
        input
            .parse::<u8>()
            .map_err(|error| format!("Cannot parse u8: {error:?}"))
    }

    fn is_zero(&self) -> bool {
        *self == 0
    }
}

pub fn execute_instruction<T: Value>(
    cpu: &mut Cpu<T>,
    program: &Program<T>,
    instruction: Instruction<T>,
) -> Result<(), ExecutionError> {
    let ip_target = match instruction {
        Instruction::Set { src, dest } => {
            let src_val = cpu.read(src)?;
            cpu.write(dest, src_val)?;
            None
        }
        Instruction::Add { src, dest } => {
            let src_val = cpu.read(src)?;
            let dest_val = cpu.read(dest.as_read())?;
            let result = src_val + dest_val;
            cpu.write(dest, result)?;
            None
        }
        Instruction::Sub { src, dest } => {
            let src_val = cpu.read(src)?;
            let dest_val = cpu.read(dest.as_read())?;
            let result = dest_val - src_val;
            cpu.write(dest, result)?;
            None
        }
        Instruction::Print { expr } => {
            let value = cpu.read(expr)?;
            println!("{value}");
            None
        }
        Instruction::JumpIfNotZero { src, label } => {
            let value = cpu.read(src)?;
            if !value.is_zero() {
                match program.resolve_label(&label) {
                    Some(index) => Some(index),
                    None => {
                        return Err(InvalidLabel(label));
                    }
                }
            } else {
                None
            }
        }
    };

    match ip_target {
        Some(ip) => {
            cpu.set_ip(ip as u64);
        }
        None => {
            cpu.set_ip(cpu.get_ip() + 1);
        }
    }

    Ok(())
}

pub fn execute_program<T: Value>(
    cpu: &mut Cpu<T>,
    program: Program<T>,
) -> Result<(), ExecutionError> {
    while cpu.get_ip() < program.instructions.len() as u64 {
        let inst = program.instructions[cpu.get_ip() as usize].clone();
        execute_instruction(cpu, &program, inst)?;
    }

    Ok(())
}
