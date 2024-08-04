use std::{
    fmt::{Debug, Display},
    path::Path,
};

enum ShellErrorCause<'a> {
    InvalidDirectory(String),
    UnicodeError(&'a Path),
}

impl<'a> ShellErrorCause<'a> {
    fn stringify(&self) -> String {
        match self {
            ShellErrorCause::InvalidDirectory(dir) => {
                format!("ERROR: {} is an invalid directory", dir)
            }
            ShellErrorCause::UnicodeError(err) => {
                format!("ERROR: directory \"{:?}\" is not valid unicode", err)
            }
        }
    }
}

pub struct ShellError<'a> {
    cause: ShellErrorCause<'a>,
}

impl<'a> ShellError<'a> {
    pub fn directory_error(dir: String) -> Self {
        Self {
            cause: ShellErrorCause::InvalidDirectory(dir),
        }
    }

    pub fn unicode_error(dir: &'a Path) -> Self {
        Self {
            cause: ShellErrorCause::UnicodeError(dir),
        }
    }
}

impl<'a> Display for ShellError<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let message = self.cause.stringify();
        write!(f, "{}", message)
    }
}

impl<'a> Debug for ShellError<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let message = self.cause.stringify();
        write!(
            f,
            "{{ file: {}, line: {}, error: {} }}",
            file!(),
            line!(),
            message
        )
    }
}
