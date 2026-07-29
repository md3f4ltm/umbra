mod ast;
mod lexer;
use std::error::Error;
use std::fmt;

pub use ast::Command;
pub use lexer::LexError;

use lexer::Token;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseError {
    Lexer(LexError),
}
impl ParseError {
    pub fn is_incomplete(&self) -> bool {
        match self {
            Self::Lexer(error) => error.is_incomplete(),
        }
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Lexer(error) => write!(formatter, "{error}"),
        }
    }
}

impl Error for ParseError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Lexer(error) => Some(error),
        }
    }
}
impl From<LexError> for ParseError {
    fn from(error: LexError) -> Self {
        Self::Lexer(error)
    }
}

pub fn parse(input: &str) -> Result<Option<Command>, ParseError> {
    let tokens = lexer::tokenize(input)?;
    Ok(parse_tokens(tokens))
}

fn parse_tokens(tokens: Vec<Token>) -> Option<Command> {
    let mut words = tokens.into_iter().map(|token| match token {
        Token::Word(word) => word,
    });

    let program = words.next()?;
    let args = words.collect();

    Some(Command { program, args })
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_simple_command() {
        let command = parse("ls -la src").unwrap().unwrap();

        assert_eq!(
            command,
            Command {
                program: "ls".to_string(),
                args: vec!["-la".to_string(), "src".to_string()],
            }
        );
    }

    #[test]
    fn empty_input_returns_none() {
        assert_eq!(parse("").unwrap(), None);
        assert_eq!(parse("        ").unwrap(), None);
        assert_eq!(parse("\n\t").unwrap(), None);
    }

    #[test]
    fn parses_no_arguments() {
        let command = parse("pwd").unwrap().unwrap();

        assert_eq!(
            command,
            Command {
                program: "pwd".to_string(),
                args: Vec::new(),
            }
        );
    }

    #[test]
    fn parse_quoted_arguments() {
        let command = parse("echo \"Hello World\" 'from umbra'").unwrap().unwrap();
        assert_eq!(
            command,
            Command {
                program: "echo".to_string(),
                args: vec!["Hello World".to_string(), "from umbra".to_string()],
            }
        );
    }
}
