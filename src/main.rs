mod builtins;
mod executor;
mod parser;
mod shell;

fn main() {
    let mut shell = shell::Shell::new();
    match shell.run() {
        Ok(status) => std::process::exit(status),
        Err(error) => {
            println!("umbra error: {error}");
            std::process::exit(1);
        }
    }
}
