---
---
# Ability to define templates for issue types

As a user  
I would like to be able to define a different markdown template for each issue type  
So that I spend less time writing the issues and everybody shares the same template

## Acceptance criteria

- The template should be written in some kind of templating engine that allows the insertion of variables
- The system should provide a set of variables that can be inserted in the template.
  - `name`: the issue name
- The system should allow the creation of a different template per issue type
- The system should allow the creation of a default template that is applied if no specific template has been defined for that issue type

## Background

This can be very useful whenever a team wants to use a common template for writting user stories, bugs, tasks... so they can pre-populate the issue with some specific fields or contents.

See below some examples.

### An example bug template
```
---
severity: 
affects-version: 
environment: 
priority: 
---
# {{name}}

## Steps to reproduce

1. 

## Expected results

## Actual results


```

### An example user story template
```
---
priority: 
---
# {{name}}

As a  
I want to  
So that  

## Acceptance criteria


```