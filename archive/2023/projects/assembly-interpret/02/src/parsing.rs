use crate::{Instruction, ReadExpr};
use std::num::ParseIntError;

#[derive(Debug)]
pub struct ParseError<'a> {
    pub line_number: usize,
    pub line: &'a str,
    pub kind: ParseErrorKind<'a>,
}

impl<'a> ParseError<'a> {
    fn missing_args(line_number: usize, line: &'a str) -> Self {
        Self {
            line_number,
            line,
            kind: ParseErrorKind::MissingArguments,
        }
    }

    fn unknown_instruction(line_number: usize, line: &'a str, instruction: &'a str) -> Self {
        Self {
            line_number,
            line,
            kind: ParseErrorKind::UnknownInstruction(instruction),
        }
    }

    fn invalid_constant(line_number: usize, line: &'a str, constant: &'a str) -> Self {
        Self {
            line_number,
            line,
            kind: ParseErrorKind::InvalidConstant(constant),
        }
    }

    fn invalid_register(line_number: usize, line: &'a str, register: &'a str) -> Self {
        Self {
            line_number,
            line,
            kind: ParseErrorKind::InvalidRegister(register),
        }
    }
}

#[derive(Debug)]
pub enum ParseErrorKind<'a> {
    MissingArguments,
    UnknownInstruction(&'a str),
    InvalidConstant(&'a str),
    InvalidRegister(&'a str),
}

pub fn parse(input: &str) -> Result<Vec<Instruction>, ParseError> {
    let mut instructions = vec![];

    for (line_number, line) in input.lines().enumerate() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        let (inst, args) = match line.split_once(" ") {
            Some(ret) => ret,
            None => {
                return Err(ParseError::missing_args(line_number, line));
            }
        };
        let parsed_inst = match inst {
            "PRINT" => {
                let read_expr = with_line(parse_read_expr(args), line_number, line)?;
                Instruction::Print(read_expr)
            }
            _ => {
                return Err(ParseError::unknown_instruction(line_number, line, inst));
            }
        };
        instructions.push(parsed_inst);
    }

    Ok(instructions)
}

fn with_line<'a, T>(
    result: Result<T, ParseErrorKind<'a>>,
    line_number: usize,
    line: &'a str,
) -> Result<T, ParseError<'a>> {
    result.map_err(|kind| ParseError {
        line,
        line_number,
        kind,
    })
}

fn parse_read_expr(input: &str) -> Result<ReadExpr, ParseErrorKind> {
    match input.strip_prefix("R") {
        Some(register_num) => {
            let num = register_num.parse::<u8>();
            match num {
                Ok(num) => Ok(ReadExpr::register(num)),
                Err(error) => Err(ParseErrorKind::InvalidRegister(register_num)),
            }
        }
        None => {
            let num = input.parse::<u8>();
            match num {
                Ok(num) => Ok(ReadExpr::constant(num)),
                Err(error) => Err(ParseErrorKind::InvalidConstant(input)),
            }
        }
    }
}
