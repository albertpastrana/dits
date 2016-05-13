---
---
# Start working on an issue

As a user  
I want to start working on an issue  
So that the rest of the team know what I'm doing

## Acceptance criteria

There is a new subcommand (`checkout`) that accepts the following arguments:

- `name`: name of the ticket the user wants to start working with. It has to be case insensitive and the user should be able to provide just a portion of the name (as a regular expression, like in `find`)

The following steps define how to mark that you are working on an issue:

- A branch named like the issue is created
- If there is a remote/origin defined, the branch is pushed
- The branch is checked out
- The issue is moved to the `../in-progress` directory

For the following exceptional situations, the program will exit with error and the user will get a message informing aboug the problem.

- No issue can be found with the specified name
- More than one issue can be found with the specified name