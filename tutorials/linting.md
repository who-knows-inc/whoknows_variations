# Linting in Python

## Introduction

This is a forray into the linting world in Python. 3 libraries as showcased here:

1. Flake8 - Linter
2. Black - Code formatter (fixes the code)
3. Ruff - Linter and code formatter 

The workflow is setup with Ruff, only. 

## Preconditions

**Note**: Linting has already been run on the codebase to fix errors to show a passing GitHub Actions workflow. It would be more interesting to take the unlinted code from other branches and replace app.py with it. 

The tutorial assumes using [Poetry](https://python-poetry.org/). You can install it with the following command:

```bash
$ curl -sSL https://install.python-poetry.org | python3 -
```

Everything has setup in the `pyproject.toml` file in the root of the project already.

## Flake8

### What is flake8?

https://pypi.org/project/flake8/

### Running flake8 locally


1. Ensure you have poetry installed:

```bash
$ poetry --version
```

2. Everything has already been setup in the `pyproject.toml` file in the root of the project. From now on all Poetry commands are to be run where the `pyproject.toml` file is located.

Since Flake8 has been setup as a dev dependency but it needs to be installed 

```bash
$ poetry install
```

3. Run it with the following:

```bash
$ poetry run flake8 src/backend
```

In root you can define your style rules in the `.flake8` file.

### Black

[Black](https://black.readthedocs.io/en/stable/index.html) is a code formatter for Python. While Flake8 only points out the errors, Black fixes them. 

1. If you haven't installed from previously:

```bash
$ poetry install
```

2. You can now run it with the following:

```bash
$ poetry run black src/backend
```

### Ruff

As of time of writing [Ruff](https://github.com/astral-sh/ruff) is the new toy that combines linting and code formatting. But a major reason for its rise is how much faster it is compared to other options. 

```bash
$ poetry run ruff check src/backend
```

```bash
$ poetry run ruff check src/backend --fix
```

## The workflow file

The linting job checks out the code, sets up python 3 and installs the requirements.

Then it uses the Github marketplace action for Ruff to run the linting. (Remember that it does not make sense to try and fix the code in a CI/CD pipeline.)

```yaml
      - name: Run linter
        uses: chartboost/ruff-action@v1
        with:
          args: check src/backend
```

The args ensure that the linter is run on the backend code. They are not strictly necessary and you could just run it on the whole repository. 
