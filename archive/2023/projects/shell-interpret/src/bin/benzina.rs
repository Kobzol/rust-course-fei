use benzina::{ChangeWorkdir, PrintWorkdir, Shell};
use std::io::BufRead;

fn main() {
    let mut shell = Shell::default();
    shell.add_command(PrintWorkdir);
    shell.add_command(ChangeWorkdir);

    let stdin = std::io::stdin().lock();
    let mut stdout = std::io::stdout().lock();

    shell.print_prompt(&mut stdout).unwrap();
    for line in stdin.lines() {
        let line = line.unwrap();
        shell.execute_line(&line, &mut stdout);
        shell.print_prompt(&mut stdout).unwrap();
    }
}
