To ensure that `./run.sh` always runs on startup, you can create a systemd service. Here are the steps to create and enable a systemd service for your GitHub Actions runner:

1. Create a systemd service file:

```bash
$ sudo nano /etc/systemd/system/github-actions-runner.service
```

2. Add the following content to the file (ensure that the paths and user are correct):

```ini
[Unit]
Description=GitHub Actions Runner
After=network.target

[Service]
ExecStart=/home/adminuser/actions-runner/run.sh
WorkingDirectory=/home/adminuser/actions-runner
User=adminuser
Restart=always
RestartSec=10

[Install]
WantedBy=multi-user.target
```

3. Reload systemd:

```bash
$ sudo systemctl daemon-reload
```

4. Enable the service on boot:

```bash
$ sudo systemctl enable github-actions-runner
```

5. Start the service:

```bash
$ sudo systemctl start github-actions-runner
```

6. [Optional] Check the service status:

```bash
$ sudo systemctl status github-actions-runner
```
