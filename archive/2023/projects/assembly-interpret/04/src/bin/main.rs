use std::fmt::{Debug, Display, Formatter};
use std::ops::{Add, Sub};

use ikea::instruction::Program;
use ikea::parser::ParseError;
use ikea::{execute_program, parse_program, CpuBuilder, Value};

#[derive(Clone, Debug, Default)]
struct Vec2D {
    x: f32,
    y: f32,
}

impl Display for Vec2D {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl Value for Vec2D {
    /// 1.0_2.0
    fn parse(input: &str) -> Result<Self, String> {
        let Some((left, right)) = input.split_once("_") else {
            return Err("Wrong Vec2D".to_string());
        };
        let x = left
            .parse::<f32>()
            .map_err(|error| format!("Error: {error:?}"))?;
        let y = right
            .parse::<f32>()
            .map_err(|error| format!("Error: {error:?}"))?;
        Ok(Self { x, y })
    }

    fn is_zero(&self) -> bool {
        self.x == 0.0 && self.y == 0.0
    }
}

macro_rules! impl_add_sub {
    ($ty: ty) => {
        impl Add for $ty {
            type Output = Self;

            fn add(self, rhs: Self) -> Self::Output {
                Self {
                    x: self.x + rhs.x,
                    y: self.y + rhs.y,
                }
            }
        }

        impl Sub for $ty {
            type Output = Self;

            fn sub(self, rhs: Self) -> Self::Output {
                Self {
                    x: self.x - rhs.x,
                    y: self.y - rhs.y,
                }
            }
        }
    };
}

impl_add_sub!(Vec2D);

struct Range {
    value: u32,
}

impl Iterator for Range {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.value == 0 {
            None
        } else {
            let value = self.value;
            self.value -= 1;
            Some(value)
        }
    }
}

fn main() {
    let mut cpu = CpuBuilder::new()
        .register_count(16)
        .memory_size(1024)
        .default::<u8>();
    let input = r#"
    MOV R0, 3
    PRINT R0x
    SUB R0, 1
    JNZ R0, start
    "#;
    let program = match parse_program(input).map_err(|error| annotate_line(input, error)) {
        Ok(program) => program,
        Err(error) => {
            eprintln!("{error}");
            std::process::exit(1);
        }
    };

    execute_program(&mut cpu, program).unwrap();
}

fn annotate_line(input: &str, error: ParseError) -> String {
    let lines: Vec<String> = input
        .lines()
        .enumerate()
        .map(|(index, line)| {
            if index == error.line {
                format!("> {line}\nLine {index}: {:?}", error)
            } else {
                line.to_string()
            }
        })
        .collect();
    lines.join("\n")
}
