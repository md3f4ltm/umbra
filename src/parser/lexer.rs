use std::error::Error;
use std::fmt;
use std::iter::Peekable;
use std::str::Chars;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
    Word(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LexError {
    UnclosedSingleQuote,
    UnclosedDoubleQuote,
    TrailingEscape,
}

impl LexError {
    pub fn is_incomplete(&self) -> bool {
        matches!(
            self,
            Self::UnclosedSingleQuote | Self::UnclosedDoubleQuote | Self::TrailingEscape
        )
    }
}

impl fmt::Display for LexError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnclosedSingleQuote => {
                write!(formatter, "single quote was not closed")
            }
            Self::UnclosedDoubleQuote => {
                write!(formatter, "double quote was not closed")
            }
            Self::TrailingEscape => {
                write!(formatter, "escape character requires another character")
            }
        }
    }
}

impl Error for LexError {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum QuoteMode {
    Single,
    Double,
}

pub fn tokenize(input: &str) -> Result<Vec<Token>, LexError> {
    let mut tokens = Vec::new();
    let mut current_word = String::new();
    let mut word_started = false;
    let mut quote_mode = None;
    let mut characters = input.chars().peekable();

    while let Some(character) = characters.next() {
        match quote_mode {
            Some(QuoteMode::Single) => {
                if character == '\'' {
                    quote_mode = None;
                } else {
                    current_word.push(character);
                }
            }

            Some(QuoteMode::Double) => match character {
                '"' => {
                    quote_mode = None;
                }

                '\\' => {
                    consume_escape(&mut characters, &mut current_word)?;
                }

                _ => {
                    current_word.push(character);
                }
            },

            None => match character {
                character if character.is_whitespace() => {
                    finish_word(&mut tokens, &mut current_word, &mut word_started);
                }

                '\'' => {
                    quote_mode = Some(QuoteMode::Single);
                    word_started = true;
                }

                '"' => {
                    quote_mode = Some(QuoteMode::Double);
                    word_started = true;
                }

                '\\' => {
                    consume_escape(&mut characters, &mut current_word)?;

                    word_started = true;
                }

                _ => {
                    current_word.push(character);
                    word_started = true;
                }
            },
        }
    }

    match quote_mode {
        Some(QuoteMode::Single) => {
            return Err(LexError::UnclosedSingleQuote);
        }

        Some(QuoteMode::Double) => {
            return Err(LexError::UnclosedDoubleQuote);
        }

        None => {}
    }

    finish_word(&mut tokens, &mut current_word, &mut word_started);

    Ok(tokens)
}

fn consume_escape(
    characters: &mut Peekable<Chars<'_>>,
    current_word: &mut String,
) -> Result<(), LexError> {
    let escaped = characters.next().ok_or(LexError::TrailingEscape)?;

    if escaped == '\n' {
        // A backslash followed by a newline is a line continuation.
        //
        // If the newline is currently the last character, the shell
        // needs to read another line.
        if characters.peek().is_none() {
            return Err(LexError::TrailingEscape);
        }

        // Do not add either the backslash or newline to the word.
        return Ok(());
    }

    current_word.push(escaped);

    Ok(())
}

fn finish_word(tokens: &mut Vec<Token>, current_word: &mut String, word_started: &mut bool) {
    if !*word_started {
        return;
    }

    tokens.push(Token::Word(std::mem::take(current_word)));
    *word_started = false;
}

#[cfg(test)]

mod tests {
    use super::*;
}
