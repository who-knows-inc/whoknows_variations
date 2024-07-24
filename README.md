# Flask Variations - Continuous Deployment

This tutorial focuses on how you can achieve Continuous Deployment of Docker images with Github Actions. 

There are two ways to solve this with Github Actions:

1. SSH from the runner into your server and execute commands remotely.

2. Treat your server as a Github Actions runner which allows you execute commands on it. 

There is a third category that excludes Github Actions and requires third-party services. 



<!-- TODO create a setup.sh script and run git update-index --chmod=+x setup.sh on it -->


## SSH and execute commands remotely

[Tutorial](./tutorials/ssh_and_execute_commands_remotely.md)


## Your deployment server as a Github Actions runner

<!-- todo complete this -->

[This tutorial is not yet complete, but it might inspire you on how to approach this problem.](./tutorials/self_hosted_runner.md)


## Things to consider

The deployment workflow should not run on push but instead be dependent on another workflow. This can be achieved like this:

```yaml
on:
  workflow_run:
    workflows: ["<name_of_workflow>"]
    types:
      - completed
  workflow_dispatch:
```

