


If you get permission denied error while trying to run commands on the server that work locally create a docker group on the server:

```bash
$ sudo usermod -aG docker $(whoami)
$ newgrp docker
```