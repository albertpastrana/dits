# Dits [![Build Status](https://travis-ci.org/albertpastrana/dits.svg?branch=master)](https://travis-ci.org/albertpastrana/dits)

Dits (pronounced as deets) is a free and open source Distributed Issue Tracking System designed to handle projects in an easy, flexible and extensible way.

Dits can be used for any kind of projects, from small with just a team to bigger projects, with multiple remote teams and complex workflows.

## Why another issue tracking system?

There are lots of issue tracking systems out there, most of them are heavily used, have a long history, are stable, provide a huge amount of features and configuration options... Why then create another one?

Although the above is true and we've been reasonably satisfied using several of those systems, we were mainly frustated by two limitations we found on those systems:

- Most of them are centralized (SPOF)
- The source code and the issues reside in different places

Dits main design principle was to use the same git repository[^1] that stores the source code of the project to store the issues related to it.

## Design principles

When we were thinking about what characteristics Dits should have we wrote the following whish list:

- Technical users should be able to use it without context switching. They shouldn't need to install or use any new tool they are not already using for doing their job (editor, command line...)
- Non technical users should be able to use it easily, preferably via a web interface with minimum setup
- Markdown should be used to write the specs, as it provides a text-based, non-propietary, human-readable format that is easy of use for both technical and non-technical users
- Commits in the code should be easy to relate to a ticket
- The system shouldn't be opinionated, so everybody can use it and adapt to their own workflow
- The system should not need any configuration and minimal-to-none maintenance
- The system should be distributed
- The system should support the most common features present in issue trackers:
  - Creation of different issue types (bugs, tasks, features requests...)
  - Assignment of issues to people
  - Ability to run queries and reports
  - Tracking of all changes made to tickets
  - Ability add detailed descriptions

## Installation

### For technical users

There are three different ways a technical user can work with Dits:

- Just create files, move them around, commit and push. Easy, isn't it?
- Install the command line tool and use it to create, move tickets...
- Just use the web interface like the non-technical users

### For non-technical users

If somebody in your team has been nice and has set up a server for you, just point your browser to it.

If not, it's a bit more complicated, but you should be able to do it.

- Download the latest binary from: ???
- Open the application and type the url of the git repository you want to use.
- Point your browser to http://localhost:8080/

## Concepts and structure of the repository

### Issues directory

There has to be a special directory in your working dir that contains all the issues and its related data. By default this directory is `issues`, but you can change it in the [configuration](#configuration).

This directory may be a git submodule if having the issues in a separate git repository is more convenient for you.

### Issues

An issue in Dits is simply a Markdown file with a special file name. The file name must follow this pattern:

```{issue-title}.{issue-type}.md```

- Issue title: it is any valid string that can be a file name (ideally without spaces).
- Issue type: any string that can represent an issue type. If no issue type is provided then `task` will be used.
- Issue [container](#issue-containers): where to locate the issue when it's created. The default container is `backlog`.

The content of the file consists of an optional metadata header written in YAML enclosed by `---` and the content written in Markdown. For example:

```
---
points: 5
tags: important
affects-version: 1.0
---
# Important stuff to be done

In order to have our customers happy
We should do some important stuff

## Acceptance criteria
- Do important stuff
- Do it properly
- While smiling

```

### Issue containers

Issues are grouped by issue containers. Containers are simply directories under the `issues` directory.

In Dits, there can be as many containers as desired. Each of those containers will have the meaning the team decides, there are no special rules for that.

When an issue is created, it must be placed under a container, if no container is specified, then `backlog` is used by default.

In order to create a new container (for example, if the team is about to start a new sprint), it's as simple as creating a new directory:

```
$ mkdir issues/sprint
```

Moving an issue from the backlog to a sprint is something as simple as moving a file from one directory to another. Let's see it in action 

```
$ mv issues/backlog/important-issue issues/sprint/
```

Btw, if you were thinking if you can have containers inside other containers, you are correct! It's as simple as creating a nested directory.

### Creating an issue

As one can figure out, creating an issue is a simple as creating a file in a specific directory with a name that follows some specific conventions.

Let's see some examples on how an issue could be created using the `dits` command line tool or directly with the existing tools present in most *nix shells.

Create an issue in the default location (backlog) with the default type (task):

```
$ dits create important-issue
```
```
$ touch issues/backlog/important-issue.task.md
```
Note that the `dits` command will add the file to the git index and create any needed directory. So you may need to run `$ mkdir -p issues/backlog` and `$ git add issues/backlog/important-issue.task.md` if you are not using `dits`.

Create an issue inside a container with a specific type:

```
$ dits create important-issue -t bug -l sprint
```
```
$ touch issues/sprint/important-issue.bug.md
```

Create an issue inside a nested container:

```
$ dits create important-issue -l backlog/urgent
```
```
$ touch issues/backlog/urgent/important-issue.task.md
```

### Issue types

Each issue is of specific type. Types are "created" on the fly

### Finding issues

```
dits find
git grep
```

## Example workflows

### Scrum

### Kanban

## Configuration

If you want to override any of the configuration options you can do it by placing a `.dits-config` file in the root of your working dir.

The following configuration options are allowed:

- `issues_dir`: relative directory where the issues will be stored. Default value is `issues`.
- `valid_issue_types`: comma-separated list of issue types. By default any issue type is valid, but this option is helpful for those teams that only want to use a subset of issue types in their project (e.g. `bug,task`).
- `default_issue_type`:
- `default_issue_container`:

## Githooks

We provide some githooks that will make sure the issues have a valid format (a valid issue type, proper markdown...).

## How do we synchronize the work?

Remember we commented that this software is non-opinionated and distributed like git? Then, you have the answer. You can synchronize the work however you want, it's your decision.

Some people will prefer to push any single change to a central server, for some others will work better to send patches via email, some teams will push to each others machines...

It's pretty easy to configure a cronjob, 

We won't pollute your history

## Why Rust and TypeScript?

Just because we wanted to learn both. Building this project seemed like a good opportunity to work with them both.

## Future

- Jira/Trello/Github issues import

## Alternatives

There are some similar projects you can check:

- [https://github.com/jwiegley/git-issues]()
- [https://github.com/jeffWelling/ticgit]()
- [https://github.com/schacon/ticgit]()

## Contributing

## License

TBD


[^1]: This is not entirely true as you can easily use a different repository and include it as a [submodule](https://git-scm.com/docs/git-submodule) in your project if you don't want to have issues and code mixed.
