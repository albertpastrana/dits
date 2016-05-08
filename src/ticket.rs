use util::*;
use errors::*;

use git2::{BranchType, Repository};
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

#[derive(Debug)]
pub struct Ticket {
    pub name: String,
    pub ttype: String,
    pub location: String,
}

impl Ticket {
	pub fn create(&self, repo: &Repository, main_workdir: &Path, abs_tickets_dir: &Path) -> Result<String, CliError> {
	    let repo_workdir = try!(workdir(repo));

	    // let abs_tickets_dir = try!(tickets_dir(repo, tickets_di));
	    let location = Path::new(abs_tickets_dir).join(&self.location);

	    // Try to create the location for the ticket
	    try!(fs::create_dir_all(location.as_path()));

	    // Check if there is a ticket with that name already
	    let existing_ticket = try!(self.find_me(main_workdir, abs_tickets_dir));
	    if !existing_ticket.is_empty() {
	    	return Err(CliError::TicketExists(existing_ticket));
	    }

	    // Create the file
	    let abs_path = location.join(&self.fname());
	    let rel_path = abs_path.strip_prefix(&repo_workdir).unwrap();
	    let mut f = try!(File::create(&abs_path));
		try!(f.write_all(b"---\n---\n# \n"));

	    // Add the file to the git index
	    let mut index = try!(repo.index());
	    try!(index.add_path(rel_path));
	    index.write().map(|_| format!("Ticket created: '{}'", rel_path.display())).map_err(|err| CliError::Git(err))
	}

    pub fn fname(&self) -> String {
        format!("{}.{}.md", self.name, self.ttype)
    }

	pub fn checkout(&self, repo: &Repository) -> Result<String, CliError> {
	    debug!("Checking out ticket: {:?}", self);

	    let tname = self.name.to_owned();

	    try!(repo.find_branch(&tname, BranchType::Local).or(
	        repo.refname_to_id("HEAD")
	            .and_then(|oid| repo.find_commit(oid))
	            .and_then(|head| repo.branch(&tname, &head, false))
	    ));

	    try!(repo.set_head(&format!("refs/heads/{}", tname)));

	    Ok(tname)
	}

    fn find_me<'a>(&self, workdir: &Path, tickets_dir: &Path) -> Result<Vec<Ticket>, CliError> {
      Ticket::find_ticket(workdir, tickets_dir, &self.fname())
    }

    /// Finds the tickets in the `tickets_dir` with a name that matches `name`.
    ///
    /// This function is actually calling directly to `find <tickets_dir> -type f -iname <name>`
    /// so you can pass anything you would pass to `find`.
    ///
    /// It returns the absolute filename(s) as output by `find`
    ///
    /// # Errors
    ///
    /// It can return a `CmdError` error as is using `run_cmd` to find the file.
    pub fn find_ticket(workdir: &Path, tickets_dir: &Path, name: &String) -> Result<Vec<Ticket>, CliError> {
      let res: String = try!(run_cmd("find", &[&tickets_dir.to_string_lossy(), "-type", "f", "-iname", name]));
      let vec = res.split('\n')
                   // .map(|s| s.to_owned())
                   .filter(|s| !s.is_empty())
                   .map(|s| Ticket::from_path(workdir, tickets_dir, s).unwrap()) //TODO remove unwrap
                   .collect::<Vec<Ticket>>();
      println!("find_ticket: res: {}", res);
      Ok(vec)
    }

    /// Finds one ticket in the `tickets_dir` with a name that matches `name`.
    ///
    /// This function uses `find_ticket` and only checks if the result is exactly
    /// one ticket, otherwise it returns error.
    ///
    /// # Errors
    ///
    /// Same errors as `find_ticket` plus a `CantFindTicket` if no ticket found or
    /// a `MoreThanOneTicket` if more than one ticket is found.
    pub fn find_one(workdir: &Path, tickets_dir: &Path, name: &String) -> Result<Ticket, CliError> {
	    let mut tickets: Vec<Ticket> = try!(Ticket::find_ticket(workdir, tickets_dir, name));

	    if tickets.len() > 1 { return Err(CliError::MoreThanOneTicket(tickets)); }

	    tickets.pop().ok_or(CliError::CantFindTicket)
    }

    /// Creates a Ticket struct from an absolute path pointing to a ticket.
    ///
    /// It follows the naming conventions so this function will remove the workdir
    /// and the ticket dir from the path, then it will split the remaining between
    /// the dir and the file stem, the dir will be the location while the file stem
    /// will contain the name and the type.
    pub fn from_path(workdir: &Path, tickets_dir: &Path, ticket_path: &str) -> Result<Ticket, CliError> {
    	let rtp = try!(Path::new(ticket_path).strip_prefix(workdir));
    	let location = try!(Path::new(ticket_path).strip_prefix(tickets_dir));
    	let parent = location.parent()
    				   .and_then(|p| p.to_str())
    				   .unwrap(); //TODO remove unwrap
    	let fname: Vec<&str> = rtp.file_stem()
    							 .and_then(|osstr| osstr.to_str())
    							 .map(|s| s.rsplitn(2, '.').collect())
    							 .unwrap(); //TODO remove unwrap

    	if fname.len() != 2 {
    		return Err(CliError::PathError("Ticket filename doesn't appear to have a type"));
    	}

    	Ok(Ticket {
    		name: String::from(fname[1].to_owned()),
    		ttype: String::from(fname[0].to_owned()),
    		location: String::from(parent.to_owned())
    	})
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn from_path_should_extract_correctly_the_ticket_attributes() {
    	let workdir = Path::new("/some/workdir");
    	let tickets_dir = Path::new("/some/workdir/issues");
    	let ticket_path = "/some/workdir/issues/backlog/name.type.md";

        let result = Ticket::from_path(&workdir, &tickets_dir, &ticket_path).unwrap();

		assert_eq!(result.name, "name");
		assert_eq!(result.ttype, "type");
		assert_eq!(result.location, "backlog");
    }
    #[test]
    fn from_path_should_extract_correctly_a_multiple_location() {
    	let workdir = Path::new("/some/workdir");
    	let tickets_dir = Path::new("/some/workdir/issues");
    	let ticket_path = "/some/workdir/issues/sprint/in-progress/name.type.md";

        let result = Ticket::from_path(&workdir, &tickets_dir, &ticket_path).unwrap();

		assert_eq!(result.location, "sprint/in-progress");
    }
    #[test]
    fn from_path_should_fail_if_the_filename_is_not_well_constructed() {
    	let workdir = Path::new("/some/workdir");
    	let tickets_dir = Path::new("/some/workdir/issues");
    	let ticket_path = "/some/workdir/issues/sprint/in-progress/name-type.md";

        let result = Ticket::from_path(&workdir, &tickets_dir, &ticket_path);

		assert_eq!(result.is_err(), true);
    }
    #[test]
    fn from_path_should_fail_if_tickets_dir_is_not_below_the_workdir() {
    	let workdir = Path::new("/some/workdir");
    	let tickets_dir = Path::new("/some/not-workdir/issues");
    	let ticket_path = "/some/workdir/issues/sprint/in-progress/name.type.md";

        let result = Ticket::from_path(&workdir, &tickets_dir, &ticket_path);

		assert_eq!(result.is_err(), true);
    }
    #[test]
    fn from_path_should_fail_if_ticket_path_is_not_below_the_workdir() {
    	let workdir = Path::new("/some/workdir");
    	let tickets_dir = Path::new("/some/workdir/issues");
    	let ticket_path = "/some/not-workdir/issues/sprint/in-progress/name.type.md";

        let result = Ticket::from_path(&workdir, &tickets_dir, &ticket_path);

		assert_eq!(result.is_err(), true);
    }
}