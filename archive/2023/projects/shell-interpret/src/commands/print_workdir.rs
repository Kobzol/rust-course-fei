use crate::command::{Command, CommandResponse};
use crate::state::InterpreterState;
use std::io::Write;

pub struct PrintWorkdir;

impl Command for PrintWorkdir {
    fn execute(
        &self,
        line: &str,
        state: &mut InterpreterState,
        output: &mut dyn Write,
    ) -> CommandResponse {
        let line = line.trim();
        if line != "pwd" {
            return CommandResponse::Unhandled;
        }

        writeln!(output, "{}", state.workdir.display()).unwrap();
        CommandResponse::Handled(Ok(()))
    }
}

#[cfg(test)]
mod tests {
    use crate::command::Command;
    use crate::state::InterpreterState;
    use crate::PrintWorkdir;
    use std::path::PathBuf;

    #[test]
    fn test_pwd_1() {
        let pwd = PrintWorkdir;
        let mut state = InterpreterState::new(PathBuf::from("/foo"));
        let mut buffer = vec![];
        pwd.execute("pwd", &mut state, &mut buffer);
        assert_eq!(String::from_utf8(buffer).unwrap(), "/foo\n");
    }
}
