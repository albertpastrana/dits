use util::*;
use errors::*;

use git2::Repository;
use std::fs;
use std::fs::File;
use std::path::Path;

#[derive(Debug)]
pub struct Ticket {
    pub name: String,
    pub ttype: String,
    pub location: String,
}

impl Ticket {
	pub fn create(&self, repo: &Repository, tickets_dir: &str) -> Result<String, CliError> {
	    let workdir = match repo.workdir() {
	        Some(workdir) => workdir,
	        None => return Err(CliError::CantOpenWorkDir),
	    };

	    let location = Path::new(tickets_dir).join(&self.location);

	    // Try to create the location for the ticket
	    try!(fs::create_dir_all(&workdir.join(&location)));

	    // Check if there is a ticket with that name already
	    let existing_ticket = try!(self.find_ticket(tickets_dir));
	    if !existing_ticket.is_empty() {
	    	return Err(CliError::TicketExists(existing_ticket));
	    }

	    // Create the file
	    let rel_path = location.join(&self.fname());
	    let abs_path = workdir.join(&rel_path);
	    try!(File::create(&abs_path));

	    // Add the file to the git index
	    let mut index = try!(repo.index());
	    try!(index.add_path(rel_path.as_path()));
	    let abs_fname = format!("{}", abs_path.display());
	    index.write().map(|_| abs_fname).map_err(|err| CliError::Git(err))
	}

    pub fn fname(&self) -> String {
        format!("{}.{}.md", self.name, self.ttype)
    }

    pub fn find_ticket(&self, tickets_dir: &str) -> Result<String, CliError> {
      run_cmd("find", &[tickets_dir, "-iname", &self.fname()])
    }
}
