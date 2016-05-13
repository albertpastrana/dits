use errors::*;
use std::ffi::OsStr;
use std::process::Command;
use std::path::Path;
use git2::Repository;

pub fn normalise_str(s: &str) -> String {
    s.replace(" ", "-")
}

pub fn workdir<'a>(repo: &'a Repository) -> Result<&'a Path, CliError> {
    match repo.workdir() {
        Some(workdir) => Ok(workdir),
        None => Err(CliError::CantOpenWorkDir),
    }
}

pub fn run_cmd<S: AsRef<OsStr>>(cmd: S, args: &[S]) -> Result<String, CliError> {
    let output = try!(Command::new(cmd).args(args).output());
    if output.status.success() {
        Ok(format!("{}", &String::from_utf8_lossy(&output.stdout)))
    } else {
        Err(CliError::CmdError(format!("{}", &String::from_utf8_lossy(&output.stderr))))
    }
}
