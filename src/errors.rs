use ticket::Ticket;

use std::io;
use std::path;
use git2;

#[derive(Debug)]
pub enum CliError {
    TicketExists(Vec<Ticket>),
    NoCommandDefined,
    CantOpenWorkDir,
    Io(io::Error),
    Git(git2::Error),
    CmdError(String),
    PathError(&'static str),
    CantFindTicket,
    MoreThanOneTicket(Vec<Ticket>),
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

impl From<path::StripPrefixError> for CliError {
    fn from(_: path::StripPrefixError) -> CliError {
        CliError::PathError("Couldn't strip the prefix from the path.")
    }
}