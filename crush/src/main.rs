use shell::Shell;
use shell_error::ShellError;

mod shell;
mod shell_error;

fn main() -> Result<(), ShellError<'static>> {
    /* TODO(SEP): accept file as input */

    let shell: Shell = Shell::new()?;
    let _ = shell.start();

    Ok(())
}
