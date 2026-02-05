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

Commits follow the same naming strategy as the branches, but in a slightly different manner.

For example:

```bash
git commit -m "chore: moved the images into an image folder for documentation."
```


## Pull Request

It is not possible to push to the main repository, neither will it be possible to review your own pull requests.

When you are done with the issue at hand, push the changes, go to the repository and select:

1. Compare & pull request

![alt text](2026-02-05_20-02.png)

The following is extremely important, so follow the numbered list with numbers on the image closely:

## 1. Make sure the compare is your branch.
## 2. That the head repository is ripmarkus/whoknows_ripmarkus
## 3. Base = main
## 4. Base repository = ripmarkus/whoknows_ripmarkus

![alt text](2026-02-05_20-06.png)

When the PR has been created, a fellow teammate will review it and merge it, if possible.

Like stated earlier, we like commits! But we dont like reviewing PR's all the time, so make sure you are done before you open a pull request - rather have one more commit than having to reject a pull request.

# Questions?

We like making sure everyone is onboard for this strategy, there are no stupid questions, rather ask and learn, than assume and fail.

# THERE ARE NO STUPID QUESTIONS