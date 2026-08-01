use std::env;
use std::io;
use std::path::Path;

use crate::parser::Command;

pub fn execute(command: &Command) -> Option<io::Result<i32>> {
    match command.program.as_str() {
        "cd" => Some(cd(&command.args)),
        "pwd" => Some(pwd(&command.args)),
        _ => None,
    }
}
fn pwd(args: &[String]) -> io::Result<i32> {
    match args {
        [] => {
            let dir =
                env::var("PWD").map_err(|error| io::Error::new(io::ErrorKind::NotFound, error))?;
            println!("{}", dir);
            Ok(0)
        }

        [option] if option == "-L" => {
            println!("{}", env::current_dir()?.canonicalize()?.display());
            Ok(0)
        }

        [option] if option == "-P" => {
            let dir =
                env::var("PWD").map_err(|error| io::Error::new(io::ErrorKind::NotFound, error))?;
            println!("{}", dir);
            Ok(0)
        }

        [option] if option.starts_with("-") => {
            eprintln!("pwd: Argument not found: {option}");
            Ok(2)
        }
        _ => Ok(0),
    }
}

fn cd(args: &[String]) -> io::Result<i32> {
    if args.len() > 1 {
        eprintln!("cd: too many arguments ");
        return Ok(1);
    }

    let destination = match args.first() {
        Some(path) => path.clone(),
        None => match env::var("HOME") {
            Ok(home) => home,
            Err(_) => {
                eprintln!("cd: Home is not set");
                return Ok(1);
            }
        },
    };

    change_directory(&destination)
}

fn change_directory(destination: &str) -> io::Result<i32> {
    match env::set_current_dir(Path::new(destination)) {
        Ok(()) => Ok(0),
        Err(error) => {
            eprintln!("cd: {destination}: {error}");
            Ok(1)
        }
    }
}
