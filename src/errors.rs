use std::io;
use git2;

#[derive(Debug)]
pub enum CliError {
    TicketExists(String),
    NoCommandDefined,
    CantOpenWorkDir,
    Io(io::Error),
    Git(git2::Error),
    CmdError(String),
}

impl From<io::Error> for CliError {
    fn from(err: io::Error) -> CliError {
        CliError::Io(err)
    }
}

impl From<git2::Error> for CliError {
    fn from(err: git2::Error) -> CliError {
        CliError::Git(err)
    }
}