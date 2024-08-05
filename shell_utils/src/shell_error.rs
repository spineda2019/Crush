use std::{
    fmt::{Debug, Display},
    path::Path,
};

pub enum ShellError<'a> {
    InvalidDirectory(String),
    UnicodeError(&'a Path),
    InvalidSyntax(&'a str),
    UnexpectedToken(&'a str),
    ProcessError(String),
}

impl<'a> ShellError<'a> {
    fn stringify(&self) -> String {
        match self {
            ShellError::InvalidDirectory(dir) => {
                format!("ERROR: {} is an invalid directory", dir)
            }
            ShellError::UnicodeError(err) => {
                format!("ERROR: directory {:?} is not valid unicode", err)
            }
            ShellError::InvalidSyntax(token) => {
                format!("ERROR: Invalid Syntax at token: {}", token)
            }
            ShellError::UnexpectedToken(token) => {
                format!("ERROR: Unexpected token: {}", token)
            }
            ShellError::ProcessError(process) => {
                format!("ERROR: Error ocurred while executing process: {}", process)
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

#[cfg(test)]
mod shell_util_tests {
    use super::ShellError;

    #[test]
    fn test_stringification() {
        let bad_directory_error: ShellError = ShellError::InvalidDirectory(String::from("foobar"));
        assert_eq!(
            bad_directory_error.stringify(),
            String::from("ERROR: foobar is an invalid directory")
        );

        let unicode_error: ShellError = ShellError::UnicodeError("test".as_ref());
        assert_eq!(
            unicode_error.stringify(),
            String::from("ERROR: directory \"test\" is not valid unicode")
        );

        let syntax_error: ShellError = ShellError::InvalidSyntax("&&");
        assert_eq!(
            syntax_error.stringify(),
            String::from("ERROR: Invalid Syntax at token: &&")
        );

        let unexpected_token_error: ShellError = ShellError::UnexpectedToken("&&");
        assert_eq!(
            unexpected_token_error.stringify(),
            String::from("ERROR: Unexpected token: &&")
        );

        let process_error: ShellError = ShellError::ProcessError(String::from("echo"));
        assert_eq!(
            process_error.stringify(),
            String::from("ERROR: Error ocurred while executing process: echo")
        );
    }
}
