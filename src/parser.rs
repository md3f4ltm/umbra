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
    fn parses_simple_command() {}

    #[test]
    fn empty_input_returns_none() {}
}
