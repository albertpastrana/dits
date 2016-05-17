---
---
# Order issues in a container

As a user  
I want to order the issues inside a container
so that I can see them sorted according to my criteria

## Background

Ideally the sorting should be done in a way that we don't need to
store any extra metadata for the issues and that you can still use
the command line tools (like `tree` or `ls`) to see the issues
and its order.

This implies that we should probably do it adding some kind of
numeric prefix to the filename so we can easily sort them. The
problem with that is when you want to move issues around. Should
we use something similar to basic line numbering O_o?