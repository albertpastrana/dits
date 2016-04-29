use errors::*;
use std::ffi::OsStr;
use std::process::Command;

pub fn normalise_str(s: &str) -> String {
	// s.to_lowercase().replace(" ", "-")
	s.replace(" ", "-")
}

pub fn run_cmd<S: AsRef<OsStr>>(cmd: S, args: &[S]) -> Result<String, CliError> {
    let output = try!(Command::new(cmd).args(args).output());
    if output.status.success() {
        Ok(format!("{}", &String::from_utf8_lossy(&output.stdout)))
    } else {
        Err(CliError::CmdError(format!("{}", &String::from_utf8_lossy(&output.stderr))))
    }
}
