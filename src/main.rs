mod builtins;
mod executor;
mod parser;
mod shell;

fn main() {
    let mut shell = shell::Shell::new();
    if let Err(error) = shell.run() {
        eprint!("umbra error: {error}");
        std::process::exit(1);
    }
}
