
Create a user and add their public key to the server:

```bash
$ sudo adduser githubactionsuser
$ sudo usermod -aG sudo githubactionsuser
$ sudo mkdir /home/adminuser/.ssh

```


If you get permission denied error while trying to run commands on the server that work locally create a docker group on the server:

```bash
$ sudo usermod -aG docker $(whoami)
$ newgrp docker
```