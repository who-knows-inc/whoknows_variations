# Self-hosted runner

Rather than using Github's servers you can also define your own runner. You can read more about the concept [here](https://docs.github.com/en/actions/hosting-your-own-runners/managing-self-hosted-runners/about-self-hosted-runners).

## How to setup a self-hosted runner

1. Go to your repository on Github.

2. Click on the `Settings` tab.

3. Click on the `Actions` tab in the left side and choose `Runners` under it. 

4. Click on the `New self-hosted runner` button.

5. Choose the correct OS and architecture. Follow the instructions to download and configure the runner.

Once you are done, `run.sh` should be running and you are ready to try it out. 

## Create a basic workflow for your self-hosted runner

You can find it under [basic_self-hosted.yml](../.github/workflows/basic_self-hosted.yml).

```yaml
name: Test self-hosted runner

on:
  workflow_dispatch:
  push:
    branches:
        - main

jobs:
  echo_hello_world:
    runs-on: self-hosted

    steps:
    - name: "Echo Hello World"
      run: echo "Hello World"

  create_file:
    runs-on: self-hosted

    steps:
    - name: "Create a file"
      run: |
        touch test.txt
        echo "Hello World" > test.txt
```

The workflow:

1. Outputs `Hello world` on your own server. 
2. Creates a `test.txt` file with the content `Hello World`.



## Avoid having to `run.sh` script constantly

The way Github instructs you, you must run the `run.sh` script constantly. 

Even if you run it in the background so that you can log out of the server, it is still not ideal because it won't be started when the server restarts.

Solution: [Create a systemd service for it](./always_run_run.sh.md). 
