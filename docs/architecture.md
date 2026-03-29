# Durability

## Recovery
In order to truly be able to recover from an interrupted processing,
we need to be able to replay events - but we also have to verify the
side effects of those events.

This probably involves adding an API to the registry for Event Kinds


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
