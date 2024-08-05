use std::{
    fmt::Debug,
    io::{self, Write},
    process::Command,
};

use shell_utils::shell_error::ShellError;

use crate::token::Token;

pub struct Chunk<'a> {
    command: Token<'a>,
    options: Vec<Token<'a>>,
}

impl<'a> Chunk<'a> {
    const BUILTINS: [&'a str; 1] = ["cd"];
    pub fn new(root_command: &'a str) -> Chunk<'a> {
        if Self::BUILTINS.contains(&root_command) {
            Chunk {
                command: Token::LanguageBuiltin(root_command),
                options: Vec::new(),
            }
        } else {
            Chunk {
                command: Token::Command(root_command),
                options: Vec::new(),
            }
        }
    }

    pub fn add_option(&mut self, option: &'a str) {
        self.options.push(Token::CommandOption(option));
    }

    pub fn execute_chunk(&self) -> Result<(), ShellError> {
        let mut args: Vec<&str> = Vec::with_capacity(self.options.len());
        for arg in self.options.iter() {
            args.push(arg.stringify());
        }
        let mut process = Command::new(self.command.stringify());
        process.args(args);
        let output = process.output();
        match output {
            Err(e) => {
                eprintln!("Error executing {}: {}", self.command, e);
                return Err(ShellError::ProcessError(e.to_string()));
            }
            Ok(o) => match io::stdout().write_all(o.stdout.as_slice()) {
                Ok(_) => {}
                Err(e) => {
                    eprintln!("Error printing output from process: {}", e);
                    return Err(ShellError::ProcessError(e.to_string()));
                }
            },
        };

        Ok(())
    }
}

impl<'a> Debug for Chunk<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Base Command: {}", self.command)?;
        for option in &self.options {
            write!(f, "\n\tOption for Command {}: {}", self.command, option)?
        }

        Ok(())
    }
}

#[cfg(test)]
mod chunk_tests {
    use super::Chunk;

    #[test]
    fn test_chunk_execution() {
        let mut echo = Chunk::new("echo");
        echo.add_option("test");
        assert!(echo.execute_chunk().is_ok());
    }
}
