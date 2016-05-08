#[macro_use] extern crate clap;
extern crate git2;
#[macro_use] extern crate log;
#[macro_use] extern crate env_logger;

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
    env_logger::init().unwrap();

    let workdir = match Repository::discover(".").ok().and_then(|repo| repo.workdir().map(|wd| wd.to_owned())) {
        Some(dir) => dir,
        None => panic!("Failed to open git repo workdir. Need to run dits inside a git repository."),
    };

    let abs_tickets_dir = workdir.join(TICKETS_DIR);
    debug!("Absolute ticket dir: {}", &abs_tickets_dir.as_path().display());

    // Just in case we are using a different repository for the issues
    let repo = match Repository::discover(&abs_tickets_dir.as_path().as_os_str()) {
        Ok(repo) => repo,
        Err(e) => panic!("Failed to open git repo. Need to run dits inside a git repository. {}", e),
    };
    debug!("Issues repository: {}", repo.path().display());

    let options = options();

	let result = match options.subcommand_name() {
        Some("create") => ticket_from_args(options.subcommand_matches("create").unwrap()).create(&repo, &workdir, &abs_tickets_dir.as_path()),
        Some("work") => {
            //clap + the pattern matchin ensure that we will have the work command and the mandatory name argument
            let name = normalise_str(options.subcommand_matches("work").unwrap().value_of("name").unwrap());
            Ticket::find_one(&workdir, &abs_tickets_dir.as_path(), &name).and_then(|t| t.checkout(&repo))
        },
		_ => Err(CliError::NoCommandDefined),
	};

	match result {
		Ok(m) => println!("{}", m),
        Err(CliError::Io(err)) => error!("{:?}", err),
        Err(CliError::TicketExists(tickets)) => error!("A ticket with that name already exists in this repo:\n{:?}\n", tickets),
        Err(CliError::NoCommandDefined) => error!("Need to provide at least one command. Type --help for help.\n"),
        Err(CliError::CantOpenWorkDir) => error!("Can't find workdir for git repository."),
        Err(CliError::Git(err)) => error!("Git error: {}", err.message()),
        Err(CliError::CmdError(msg)) => error!("Error running command: {}", msg),
        Err(CliError::PathError(msg)) => error!("Error with path: {}", msg),
        Err(CliError::CantFindTicket) => error!("Can't find ticket"),
        Err(CliError::MoreThanOneTicket(tickets)) => error!("Ambiguous search, more than one ticket found: {:?}", tickets),
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
        .subcommand(
            SubCommand::with_name("work")
                .about("Starts the work on a ticket")
                .arg(Arg::with_name("name")
                    .help("The name (or a portion of a name) of the ticket.")
                    .required(true)))
        .get_matches()
}

fn ticket_from_args(args: &ArgMatches) -> Ticket {
    Ticket { name: normalise_str(args.value_of("name").unwrap()),
             ttype: normalise_str(args.value_of("type").unwrap_or("task")),
             location: normalise_str(args.value_of("location").unwrap_or("backlog")) }
}