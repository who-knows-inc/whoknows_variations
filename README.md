# Flask Variations

## Setup

Bumped up the server code in `app.py` to Python version 3. This will allow us to use the latest linting libraries for Python. 

### The workflow

The jobs will be discussed in the sections below. 

As part of the setup it's worth mentioning how the workflow should be setup to be triggered from all branches. 

```yaml
on:
  push:
    branches:
      - "**"
  pull_request:
    branches:
      - "**"
```

Currently the workflow is set to only trigger on this branch since the linting has only been setup with Python 3 here. 


## Testing and Linting

[Testing](./tutorials/testing.md)

[Linting](./tutorials/linting.md)



