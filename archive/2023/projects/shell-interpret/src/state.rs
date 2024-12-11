use std::io::Write;
use std::path::PathBuf;

pub struct InterpreterState {
    pub workdir: PathBuf,
}

impl Default for InterpreterState {
    fn default() -> Self {
        Self {
            workdir: std::env::current_dir().unwrap(),
        }
    }
}

impl InterpreterState {
    pub fn new(workdir: PathBuf) -> Self {
        Self { workdir }
    }
}
