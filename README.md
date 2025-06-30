# Whoknows Variations - Continuous Deployment

This tutorial focuses on how you can achieve Continuous Deployment of Docker images with Github Actions. 

Continuous Deployment can be achieved in many different ways, which will be discussed in [Ways to achieve Continuous Deployment](./tutorials/01._Ways_to_Achieve_Continuous_Deployment.md).

[![Continuous Delivery Deployment](https://github.com/who-knows-inc/whoknows_variations/actions/workflows/continuous_delivery_deployment.yaml/badge.svg?branch=continuous_deployment)](https://github.com/who-knows-inc/whoknows_variations/actions/workflows/continuous_delivery_deployment.yaml)

**Note**: The workflow works and has been thoroughly tested even though it appears as having failed. The failure is because there isn't a server set up to deploy to. Every time I want to make changes to the documentation I don't see the point in setting up a new server just to make the workflow pass.

---

## Prerequisites

This example has the required set up to deploy docker images to Github Packages. You can learn more about this in the [Continuous Delivery](https://github.com/who-knows-inc/whoknows_variations/tree/continuous_delivery) branch. 

In the case of this tutorial, the images have been made public so that anyone can pull them and try out the tutorial on their own servers.

---

## Tutorials

1. [Ways to achieve Continuous Deployment](./tutorials/01._Ways_to_Achieve_Continuous_Deployment.md)

2. [Setting up Github Actions](./tutorials/02._Continuous_Delivery.md)

3. [Setting up the Deployment Server](./tutorials/03._The_Continuous_Deployment_Workflow.md)

4. [Environment Variables](./tutorials/04._Environment_Variables.md)


---

### Setting up the deployment server

For convenience, there is a Terraform script in the `infrastructure` directory. Assuming that you are looking at this tutorial before we go over Terraform in the course, you can ignore this part. Instead, here are the requirements for the server:

You must create a Virtual Machine with Docker installed.

1. You must have an IP address.

2. You must have a user with SSH access. (Generating new SSH key pairs for a new user specifically for Github Action is recommended.)

2. The user must be allowed to execute Docker commands. This is how you add them to the Docker group:

```bash
sudo usermod -aG docker <user>
```

You can look in [inline_commands.sh](./infrastructure/inline_commands.sh) for inspiration. 

---

### `setup.sh`

There is a script in the root directory called `setup.sh`. It will help you define the necessary Github Secrets. Run this script after you have created the server. 

Otherwise, you can also define the secrets manually. The required values are:


| Secret Name | Description |
| ----------- | ----------- |
| `CR_PAT` | The Personal Access Token (PAT) generated in Github |
| `SSH_HOST` | The IP address of the server. |
| `SSH_USER` | The user that has SSH access to the server. |
| `SSH_PRIVATE_KEY` | The private key of the user that has SSH access to the server. |



