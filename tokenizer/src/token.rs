use std::fmt::Display;

pub enum Token<'a> {
    Command(&'a str),
    Redirection(&'a str),
    Combination(&'a str),
    CommandOption(&'a str),
    LanguageBuiltin(&'a str),
}

impl<'a> Token<'a> {
    pub fn stringify(&self) -> &str {
        match self {
            Token::Command(x) => x,
            Token::Redirection(x) => x,
            Token::Combination(x) => x,
            Token::CommandOption(x) => x,
            Token::LanguageBuiltin(x) => x,
        }
    }
}

impl<'a> Display for Token<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.stringify())
    }
}
