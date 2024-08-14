# Whoknows Variations

## Get started

To get started, copy the `.env.sample` file in `src/backend` to `.env` and fill in the values. 

Then run the following command to start the application:

```bash
$ docker-compose -f docker-compose.dev.yml up --build
```

You can now access the application at `http://localhost:8080`.

## Github Packages

There are many container registries to choose from. This repository uses the Github Packages:

https://github.com/features/packages

The workflow can be modified to deploy to another container registry such as Docker Hub etc. 

## Tutorials

[01. Setup overview and what you need to change](./tutorials/01._Overview.md)

[02. Generating a CR PAT](./tutorials/02._Generating_CR_PAT.md)

[03. CLI Example](./tutorials/03._CLI_Example.md)

[04. Workflow File](./tutorials/04._Workflow_File.md)

[05. Environment Variables](./tutorials/05._Environment_Variables.md)