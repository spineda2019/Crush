use shell::Shell;
use shell_utils::shell_error::ShellError;

mod shell;

fn main() -> Result<(), ShellError<'static>> {
    /* TODO(SEP): accept file as input */

    let mut shell: Shell = Shell::new()?;
    let _ = shell.start();

    Ok(())
}
