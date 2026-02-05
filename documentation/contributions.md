# How to contribute

There are very strict rules on how to contribute to this codebase, even if you are a from the *ripmarkus* organization.

Below we will list how to contribute, how NOT to contribute and why we do it like so.

## Issue

Find an issue and assign yourself to it, make sure that you flag it as *In progress* in the Kanban Board under the projects tab on github.

## Branching strategy

We keep all of our code in one repository, with divergent branches for features, fixes, chores and documentation.


For example:

- chore/refactor-car-endpoint
- feat/car-endpoint
- fix/car-endpoint
- documentation/car-endpoint

Each of these will state which type of commits the branch will be.

To create a new branch, use the follow command in the cloned repo:

```bash
git checkout -b type/<branch-name>
```

## Commits

We love commits and we love to commit every time we have added a new:

- function
- fix
- refactored line

*Dont mindlessly add commits but also dont hesitate to commit.*

## Pull Request

It is not possible to push to the main repository, neither will it be possible to review your own pull requests.

When you are done with the issue at hand, go to the repository &rarr; select contribute &rarr; open pull request


Next, assign the branch you made in the **head** repository.

Pretty pretty please, make sure to assign 