use crate::command::{Command, CommandError, CommandResponse};
use crate::state::InterpreterState;
use std::io::Write;
use std::path::PathBuf;

pub struct ChangeWorkdir;

struct ParsedCommand<'a> {
    command: &'a str,
    args: Vec<&'a str>,
}

impl Command for ChangeWorkdir {
    fn execute(
        &self,
        line: &str,
        state: &mut InterpreterState,
        output: &mut dyn Write,
    ) -> CommandResponse {
        let line = line.trim();
        let Some((cmd, args)) = line.split_once(' ') else {
            return CommandResponse::Unhandled;
        };

        if cmd != "cd" {
            return CommandResponse::Unhandled;
        }

        let dir = PathBuf::from(args);
        let res = match std::fs::canonicalize(dir) {
            Ok(dir) => {
                state.workdir = dir;
                Ok(())
            }
            Err(err) => Err(CommandError::IO(err)),
        };
        CommandResponse::Handled(res)
    }
}

#[cfg(test)]
mod tests {
    use crate::command::Command;
    use crate::commands::change_dir::ChangeWorkdir;
    use crate::state::InterpreterState;
    use std::path::PathBuf;

    #[test]
    fn test_cd_1() {
        let cd = ChangeWorkdir;
        let mut state = InterpreterState::new(PathBuf::from("/foo"));
        let mut buffer = vec![];
        let res = cd.execute("cd /bar", &mut state, &mut buffer);
        assert!(matches!(res, crate::CommandResponse::Handled));
        assert_eq!(state.workdir, PathBuf::from("/bar"));
    }
}
