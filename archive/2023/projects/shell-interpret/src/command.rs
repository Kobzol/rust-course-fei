// PrintWorkdir
// ChangeWorkdir

use crate::state::InterpreterState;
use std::io::Write;

#[derive(Debug)]
pub enum CommandError {
    IO(std::io::Error),
}

pub enum CommandResponse {
    Unhandled,
    Handled(Result<(), CommandError>),
}

pub trait Command {
    fn execute(
        &self,
        line: &str,
        state: &mut InterpreterState,
        output: &mut dyn Write,
    ) -> CommandResponse;
}
