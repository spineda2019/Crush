use std::{
    fmt::{Debug, Display},
    path::Path,
};

pub enum ShellError<'a> {
    InvalidDirectory(String),
    UnicodeError(&'a Path),
    InvalidSyntax(&'a str),
}

impl<'a> ShellError<'a> {
    fn stringify(&self) -> String {
        match self {
            ShellError::InvalidDirectory(dir) => {
                format!("ERROR: {} is an invalid directory", dir)
            }
            ShellError::UnicodeError(err) => {
                format!("ERROR: directory \"{:?}\" is not valid unicode", err)
            }
            ShellError::InvalidSyntax(token) => {
                format!("ERROR: Unexpected token: {}", token)
            }
        }
    }
}

impl<'a> Display for ShellError<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let message = self.stringify();
        write!(f, "{}", message)
    }
}

impl<'a> Debug for ShellError<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let message = self.stringify();
        write!(
            f,
            "{{ file: {}, line: {}, error: {} }}",
            file!(),
            line!(),
            message
        )
    }
}
