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
}
