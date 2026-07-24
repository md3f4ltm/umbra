use std::io;
use std::process::Command;

use crate::parser::ParseCommand;

pub fn execute(command: &ParseCommand) -> io::Result<()> {
    match Command::new(&command.program).args(&command.args).status() {
        Ok(status) => {
            if !status.success() {
                eprintln!("umbra: {} exit with status {}", command.program, status)
            }
        }
        Err(error) if error.kind() == io::ErrorKind::NotFound => {
            eprintln!("umbra: command not found: {}", command.program)
        }
        Err(error) => return Err(error),
    }
    Ok(())
}
