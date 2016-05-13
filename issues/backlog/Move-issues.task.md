---
---
# Move issues

As a user  
I want to move an issue from one container to another

## Acceptance criteria

There is a new subcommand (`move`) that accepts the following arguments:

- `name`: name of the ticket the user wants to start working with. It has to be case insensitive and the user should be able to provide just a portion of the name (as a regular expression, like in `find`)
- `location`: the location where to move the ticket (e.g. `sprint`)

For example, if we have an issue named `issue.task.md` in the `backlog`, running the command:

```
$ dits move issue sprint
```

would be exactly the same as running the following command(s):

```
$ mkdir issues/sprint (if it doesn't exist yet)
$ mv issues/backlog/issue.task.md issues/sprint/
```