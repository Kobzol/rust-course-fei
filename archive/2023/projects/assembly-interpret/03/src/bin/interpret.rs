use interpret::instruction::Program;
use interpret::{
    annotate_error, execute_program, parse_program, CpuBuilder, Instruction, ReadableExpr, Value,
};
use std::fmt::{Debug, Display, Formatter};
use std::ops::{Add, Sub};

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

impl Add for Vec2D {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for Vec2D {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
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
}

fn main() {
    let mut cpu = CpuBuilder::new()
        .register_count(16)
        .memory_size(1024)
        .default::<u8>();
    let input = r#"
    PRINT 5
    "#;
    let program = match parse_program(input).map_err(|e| annotate_error(input, e)) {
        Ok(program) => program,
        Err(error) => {
            eprintln!("{error}");
            std::process::exit(1);
        }
    };
    // println!("{:?}", program);
    execute_program(&mut cpu, program).unwrap();
}
