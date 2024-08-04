use std::{
    env::current_dir,
    io::{stdin, stdout, Write},
    path::PathBuf,
    process::exit,
};

use shell_utils::shell_error::ShellError;
use tokenizer::chunk::Chunk;

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

    fn eval<'line>(&self, line: &'line str) -> Result<(), ShellError<'line>> {
        let chunks: Vec<Chunk<'line>> = tokenizer::parse_line(line)?;
        dbg!(&chunks);

        if line == "exit" {
            println!("Exiting...");
            exit(0);
        }
        println!("Currently in development... command: {}", line);

        Ok(())
    }

    pub fn start(&mut self) -> Result<(), ShellError> {
        let mut input: String = String::new();

        loop {
            /* ********************************************************************************** */
            /*                                       Print                                        */
            /* ********************************************************************************** */
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
            if let Err(e) = self.eval(input.trim()) {
                eprintln!("{}", e);
                self.last_exit_code = 1;
            }
            input.clear();
        }
    }
}
