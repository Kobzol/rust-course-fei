mod command;
mod commands;
mod state;

use crate::command::{Command, CommandResponse};
use state::InterpreterState;
use std::io::Write;

pub use commands::change_dir::ChangeWorkdir;
pub use commands::print_workdir::PrintWorkdir;

#[derive(Default)]
pub struct Shell {
    commands: Vec<Box<dyn Command>>,
    state: InterpreterState,
}

impl Shell {
    pub fn add_command<T: Command + 'static>(&mut self, command: T) {
        self.commands.push(Box::new(command));
    }

    pub fn execute_line(&mut self, line: &str, output: &mut dyn Write) {
        for command in &self.commands {
            match command.execute(line, &mut self.state, output) {
                CommandResponse::Handled(res) => {
                    if let Err(error) = res {
                        writeln!(output, "Command `{line}` has failed: {error:?}").unwrap();
                    }

                    break;
                }
                CommandResponse::Unhandled => {}
            }
        }
    }

    pub fn print_prompt(&self, output: &mut dyn Write) -> std::io::Result<()> {
        write!(output, "{}$ ", self.state.workdir.display())?;
        output.flush()?;
        Ok(())
    }
}
