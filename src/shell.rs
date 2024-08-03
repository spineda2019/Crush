use std::{
    env::current_dir,
    io::{stdin, stdout, Write},
    path::PathBuf,
};

use crate::shell_error::ShellError;

pub struct Shell {
    current_working_directory: PathBuf,
}

impl<'b> Shell {
    pub fn new() -> Result<Self, ShellError<'b>> {
        let cwd: PathBuf = match current_dir() {
            Ok(dir) => dir,
            Err(_) => return Err(ShellError::directory_error(".".to_string())),
        };

        Ok(Self {
            current_working_directory: cwd,
        })
    }

    fn print_prompt(&self) -> Result<(), ShellError> {
        let cwd: &str = match self.current_working_directory.to_str() {
            Some(c) => c,
            None => return Err(ShellError::unicode_error(&self.current_working_directory)),
        };

        print!("{} >> ", cwd);

        Ok(())
    }

    fn eval(&self, command: &str) {
        println!("Currently in development... command: {}", command);
    }

    pub fn start(&self) -> Result<(), ShellError> {
        let mut input: String = String::new();

        loop {
            self.print_prompt()?;

            /* ********************************************************************************** */
            /*                                        Read                                        */
            /* ********************************************************************************** */
            let _ = stdout().flush();
            if let Err(e) = stdin().read_line(&mut input) {
                println!("Error reading line from stdin: {}", e);
            }

            /* ********************************************************************************** */
            /*                                        Eval                                        */
            /* ********************************************************************************** */
            self.eval(&input);
            input.clear();
        }
    }
}
