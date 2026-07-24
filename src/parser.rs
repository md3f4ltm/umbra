#[derive(Debug, Clone)]
pub struct ParseCommand {
    pub program: String,
    pub args: Vec<String>,
}

pub fn parse(line: &str) -> Option<ParseCommand> {
    let mut parts = line.split_whitespace();

    let program = parts.next()?.to_string();
    let args = parts.map(str::to_string).collect();

    Some(ParseCommand { program, args })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_simple_command() {
        let command = parse("ls -la src").unwrap();

        assert_eq!(command.program, "ls");
        assert_eq!(command.args, vec!["-la", "src"]);
    }

    #[test]
    fn empty_input_returns_none() {
        assert!(parse("").is_none());
    }
}
