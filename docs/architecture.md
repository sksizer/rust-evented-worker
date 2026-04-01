
# Durability

## Recovery
In order to truly be able to recover from an interrupted processing,
we need to be able to replay events - but we also have to verify the
side effects of those events.

This probably involves adding an API to the registry for Event Kinds



## Internal Data Model

### Execution State Model
This gives a snapshot of the state of the system at a given point in time.

It is derived from the event log. 

Sub-elements
- top level policy definitions
- summary of execution state (ie what is the state of the execution tree at this time that it is modeling)
- the state tree of sub-components - namely the activities and the execution graph between them


## Glossary
*command*
unimplemented. Would indicate intention to change system state

*event*
TODO: something that has happened in the system. Indicates a change in
system state


*activity*
a function that takes an event and produces a new event

*activity kind*
a discriminator for the functional type of the activity. Eg. `create-worktree`,
`create-commit`, `create-tag`
