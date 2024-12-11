use crate::instruction::Program;
use crate::{Instruction, ReadableExpr, Value, WritableExpr};
use std::collections::HashMap;

#[derive(Debug)]
pub struct ParseError<'a> {
    pub line: usize,
    pub error: ParseErrorKind<'a>,
}

impl<'a> ParseError<'a> {
    fn unknown_cmd(line: usize, cmd: &'a str) -> Self {
        Self {
            line,
            error: ParseErrorKind::UnknownCommand(cmd),
        }
    }

    fn unexpected_args(line: usize, args: &'a str) -> Self {
        Self {
            line,
            error: ParseErrorKind::UnexpectedArgs(args),
        }
    }
}

#[derive(Debug)]
pub enum ParseErrorKind<'a> {
    UnknownCommand(&'a str),
    UnexpectedArgs(&'a str),
    InvalidReadableExpr(&'a str),
    InvalidWritableExpr(&'a str),
    EmptyLabel,
    DuplicatedLabel(&'a str),
}

pub fn parse_program<T: Value>(input: &str) -> Result<Program<T>, ParseError> {
    let mut instructions = vec![];
    let mut labels = HashMap::new();

    for (line, command) in input.lines().enumerate() {
        let command = command.trim();
        if command.is_empty() {
            continue;
        }

        let (command, args) = match command.split_once(' ') {
            Some(parsed) => parsed,
            None => (command, ""),
        };

        let instruction = match command {
            "MOV" => {
                let (dest, src) = parse_dest_src(line, args)?;
                Instruction::Set { src, dest }
            }
            "ADD" => {
                let (dest, src) = parse_dest_src(line, args)?;
                Instruction::Add { src, dest }
            }
            "SUB" => {
                let (dest, src) = parse_dest_src(line, args)?;
                Instruction::Sub { src, dest }
            }
            "PRINT" => {
                let expr = add_line(line, parse_readable_expr(args))?;
                Instruction::Print { expr }
            }
            "JNZ" => {
                let mut args = args.split(",");
                let src = add_line(line, parse_readable_expr(args.next().unwrap()))?;
                let label = args.next().unwrap().trim().to_string();
                Instruction::JumpIfNotZero { src, label }
            }
            _ if command.ends_with(":") => {
                let label = command.trim_end_matches(":");
                if label.is_empty() {
                    return Err(ParseError {
                        line,
                        error: ParseErrorKind::EmptyLabel,
                    });
                }
                if labels
                    .insert(label.to_string(), instructions.len())
                    .is_some()
                {
                    return Err(ParseError {
                        line,
                        error: ParseErrorKind::DuplicatedLabel(label),
                    });
                }
                continue;
            }
            _ => {
                return Err(ParseError::unknown_cmd(line, command));
            }
        };

        instructions.push(instruction);
    }

    Ok(Program::new(instructions, labels))
}

fn parse_dest_src<T: Value>(
    line: usize,
    args: &str,
) -> Result<(WritableExpr, ReadableExpr<T>), ParseError> {
    let args = args.trim();
    let Some((dst, src)) = args.split_once(",") else {
        return Err(ParseError::unexpected_args(line, args));
    };
    let src = add_line(line, parse_readable_expr(src))?;
    let dest = add_line(line, parse_writable_expr(dst))?;
    Ok((dest, src))
}

fn add_line<T>(line: usize, result: Result<T, ParseErrorKind>) -> Result<T, ParseError> {
    result.map_err(|error| ParseError { line, error })
}

fn parse_readable_expr<T: Value>(input: &str) -> Result<ReadableExpr<T>, ParseErrorKind> {
    let input = input.trim();
    if let Ok(value) = T::parse(input) {
        return Ok(ReadableExpr::constant(value));
    }
    let Some(rest) = input.strip_prefix("R") else {
        return Err(ParseErrorKind::InvalidReadableExpr(input));
    };
    if let Ok(value) = rest.parse::<u8>() {
        return Ok(ReadableExpr::register(value));
    }

    Err(ParseErrorKind::InvalidReadableExpr(input))
}

fn parse_writable_expr(input: &str) -> Result<WritableExpr, ParseErrorKind> {
    let input = input.trim();
    let Some(rest) = input.strip_prefix("R") else {
        return Err(ParseErrorKind::InvalidReadableExpr(input));
    };
    if let Ok(value) = rest.parse::<u8>() {
        return Ok(WritableExpr::register(value));
    }
    Err(ParseErrorKind::InvalidWritableExpr(input))
}
