# Flask Variations

## Setup

Bumped up the server code in `app.py` to Python version 3. This will allow us to use the latest linting libraries for Python. 





## Flake8

### What is flake8?

https://pypi.org/project/flake8/

### Running flake8 locally

There is no need to run flake8 locally, since it runs in the workflow and you might not be using Python for you project. 

But if you are interested then here is how. 

This setup is done with [poetry](https://python-poetry.org/). There are many reason for why it's a good idea to use it. 

0. Without poetry you would want to install flake8 and run the following command in the `src/backend` directory:

```bash
$ flake8
```

1. Here is how you do it with poetry. First install poetry:

```bash
$ curl -sSL https://install.python-poetry.org | python3 -
```

2. Everything has already been setup in the `pyproject.toml` file in the root of the project.

Flake8 has been installed as a dev dependency. You only have to install the dependencies with poetry by running the following in the `src/backend` folder:

```bash
$ poetry install
```

3. In root, you can run it with the following:

```bash
$ poetry run flake8 src/backend
```

## Black

 (Code Formatter)

```bash
$ poetry run black src/backend
```

In root you can define your style rules in the `.flake8` file.

## Ruff

Consider using [Ruff](https://github.com/astral-sh/ruff)

```bash
$ poetry run ruff check src/backend
```

```bash
$ poetry run ruff check src/backend --fix
```