use interpret::{execute_instruction, parse, CpuBuilder, ParseErrorKind};

fn main() {
    let code = r#"
PRINT R0
PRINT 8
"#;
    let instructions = match parse(code) {
        Ok(instructions) => instructions,
        Err(error) => match error.kind {
            ParseErrorKind::MissingArguments => {
                println!(
                    "Missing arguments at line {}:\n> {}",
                    error.line_number, error.line
                );
                return;
            }
            ParseErrorKind::UnknownInstruction(inst) => {
                println!(
                    "Unknown instruction '{inst}' at line {}:\n> {}",
                    error.line_number, error.line
                );
                return;
            }
            ParseErrorKind::InvalidConstant(constant) => {
                println!(
                    "Invalid constant '{constant}' at line {}:\n> {}",
                    error.line_number, error.line
                );
                return;
            }
            ParseErrorKind::InvalidRegister(register) => {
                println!(
                    "Invalid register '{register}' at line {}:\n> {}",
                    error.line_number, error.line
                );
                return;
            }
        },
    };

    let mut cpu = CpuBuilder::new()
        .register_count(16)
        .memory_size(128)
        .zeroed();

    for instruction in instructions {
        execute_instruction(&mut cpu, instruction).unwrap();
    }
}
