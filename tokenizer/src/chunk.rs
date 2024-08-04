use std::{fmt::Debug, process::Command};

use crate::token::Token;

pub struct Chunk<'a> {
    command: Token<'a>,
    options: Vec<Token<'a>>,
}

impl<'a> Chunk<'a> {
    pub fn new(root_command: &'a str) -> Chunk<'a> {
        Chunk {
            command: Token::Command(root_command),
            options: Vec::new(),
        }
    }

    pub fn add_option(&mut self, option: &'a str) {
        self.options.push(Token::CommandOption(option));
    }

    pub fn execute_chunk(&self) {
        let mut args: Vec<&str> = Vec::with_capacity(self.options.len());
        for arg in self.options.iter() {
            args.push(arg.stringify());
        }
        let mut process = Command::new(self.command.stringify());
        process.args(args);
        process.output();
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
