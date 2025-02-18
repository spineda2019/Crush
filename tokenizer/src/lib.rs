use std::collections::VecDeque;

use chunk::Chunk;
use shell_utils::shell_error::ShellError;

pub mod chunk;
mod token;

pub fn parse_line<'a>(line: &'a str) -> Result<Vec<Chunk<'a>>, ShellError<'a>> {
    let mut lexemes: VecDeque<&'a str> = VecDeque::new();
    let mut chunks: Vec<Chunk<'a>> = Vec::new();

    for lexeme in line.split_whitespace() {
        if lexeme == "&&" {
            match lexemes.pop_front() {
                Some(lex) => {
                    let mut chunk: Chunk<'a> = Chunk::new(lex);
                    while let Some(option) = lexemes.pop_front() {
                        chunk.add_option(option);
                    }
                    chunks.push(chunk);
                }
                None => return Err(ShellError::InvalidSyntax("&&")),
            }
        } else {
            lexemes.push_back(lexeme);
        }
    }

    if !lexemes.is_empty() {
        match lexemes.pop_front() {
            Some(lex) => {
                let mut chunk: Chunk<'a> = Chunk::new(lex);
                for option in lexemes.iter() {
                    chunk.add_option(option);
                }
                chunks.push(chunk);
            }
            None => return Err(ShellError::UnexpectedToken("")),
        }
    }

    Ok(chunks)
}

#[cfg(test)]
mod tests {
    use crate::parse_line;

    #[test]
    fn test_parse() {
        assert!(parse_line("&& bad").is_err());
    }
}
