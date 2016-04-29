#[macro_use]
extern crate clap;
extern crate git2;
mod errors;
mod ticket;
mod util;

use ticket::*;
use errors::*;
use util::*;

use clap::{App, Arg, ArgMatches, SubCommand};
use git2::Repository;

const TICKETS_DIR: &'static str = "issues";

fn main() {
    let repo = match Repository::discover(".") {
        Ok(repo) => repo,
        Err(e) => panic!("Failed to open git repo. Need to run dits inside a git repository. {}", e),
    };

    //TODO we should probably try to discover the repo under the TICKETS_DIR
    //as it could be that is imported as a git submodule. It'd be something like:
    //Repository::discover(repo.workdir.join(TICKETS_DIR))

    let options = options();

	let result = match options.subcommand_name() {
		Some("create") => ticket_from_args(options.subcommand_matches("create").unwrap()).create(&repo, TICKETS_DIR),
		_ => Err(CliError::NoCommandDefined),
	};

	match result {
		Ok(m) => println!("Ticket created '{}'", m),
        Err(CliError::Io(err)) => println!("{:?}", err),
        Err(CliError::TicketExists(ticket)) => println!("A ticket with that name already exists in this repo:\n{}", ticket),
        Err(CliError::NoCommandDefined) => println!("Need to provide at least one command. Type --help for help."),
        Err(CliError::CantOpenWorkDir) => println!("Can't find workdir for git repository."),
        Err(CliError::Git(err)) => println!("Git error: {}", err.message()),
        Err(CliError::CmdError(msg)) => println!("Error running command: {}", msg),
	}
}

fn options<'a>() -> ArgMatches<'a> {
    App::new("dits")
        .version(crate_version!())
        .author("Albert Pastrana <albert.pastrana@gmail.com>")
        .about("Distributed Issue Tracking System.")
        .subcommand(
            SubCommand::with_name("create")
                .about("Creates a new ticket")
                .arg(Arg::with_name("name")
                    .help("The name of the ticket. It will be normalised.")
                    .required(true))
                .arg(Arg::with_name("type")
                    .short("t")
                    .takes_value(true)
                    .help("The type of the ticket. Default is task."))
                .arg(Arg::with_name("location")
                    .short("l")
                    .takes_value(true)
                    .help("Where to create the ticket. Default is backlog.")))
        .get_matches()
}

fn ticket_from_args(args: &ArgMatches) -> Ticket {
    Ticket { name: normalise_str(args.value_of("name").unwrap()),
             ttype: normalise_str(args.value_of("type").unwrap_or("task")),
             location: normalise_str(args.value_of("location").unwrap_or("backlog")) }
}