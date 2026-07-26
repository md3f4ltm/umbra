use std::env;
use std::io;
use std::path::Path;

use crate::parser::ParseCommand;

pub fn execute(command: &ParseCommand) -> Option<io::Result<()>> {
    match command.program.as_str() {
        "cd" => Some(cd(&command.args)),
        _ => None,
    }
}

fn cd(args: &[String]) -> io::Result<()> {
    if args.len() > 1 {
        eprintln!("cd: too many arguments ");
        return Ok(());
    }

    let destination = match args.first() {
        Some(path) => path.as_str(),
        None => match env::var("HOME") {
            Ok(home) => return change_directory(&home),
            Err(_) => {
                eprintln!("cd: Home is not set");
                return Ok(());
            }
        },
    };
    change_directory(destination)
}

fn change_directory(destination: &str) -> io::Result<()> {
    if let Err(error) = env::set_current_dir(Path::new(destination)) {
        eprintln!("cd: {destination}: {error}");
    }
    Ok(())
}
