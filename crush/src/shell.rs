use std::{
    env::current_dir,
    io::{stdin, stdout, Write},
    path::PathBuf,
    process::exit,
};

use shell_utils::shell_error::ShellError;

pub struct Shell {
    current_working_directory: PathBuf,
    last_exit_code: isize,
}

impl<'b> Shell {
    pub fn new() -> Result<Self, ShellError<'b>> {
        let cwd: PathBuf = match current_dir() {
            Ok(dir) => dir,
            Err(_) => return Err(ShellError::InvalidSyntax(".")),
        };

        Ok(Self {
            current_working_directory: cwd,
            last_exit_code: 0,
        })
    }

    fn print_prompt(&self) -> Result<(), ShellError> {
        let cwd: &str = match self.current_working_directory.to_str() {
            Some(c) => c,
            None => return Err(ShellError::UnicodeError(&self.current_working_directory)),
        };

        print!("{} >> ", cwd);

        Ok(())
    }

    fn eval(&self, command: &str) {
        if command == "exit" {
            println!("Exiting...");
            exit(0);
        }
        println!("Currently in development... command: {}", command);
    }

    pub fn start(&self) -> Result<(), ShellError> {
        let mut input: String = String::new();

        loop {
            if let Err(e) = self.print_prompt() {
                eprintln!("Shell Error Detected: {}", e);
                continue;
            };

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
            self.eval(input.trim());
            input.clear();
        }
    }
}
