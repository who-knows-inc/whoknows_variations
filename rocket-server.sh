## This was used under the creation of the Hetzner Cloud Server
## and is just here to show how we configured the server
#cloud-config

# Add a user named 'example'
users:
  - name: whoknows
    sudo: ["ALL=(ALL) NOPASSWD:ALL"] # Grants sudo privileges without a password
    groups: sudo
    shell: /bin/bash
    lock_passwd: true # This locks the user's password, forcing password reset at first login
    ssh-authorized-keys:
      - ssh-ed25519 example key
 
# Package update and upgrade
package_update: true
package_upgrade: true
package_reboot_if_required: true # Automatically reboots if necessary (e.g., new kernel)

# Replace the sshd_config file
write_files:
  - path: /etc/ssh/sshd_config
    permissions: "0600"
    owner: root:root
    content: |
      Include /etc/ssh/sshd_config.d/*.conf
      LogLevel VERBOSE
      PermitRootLogin no
      MaxAuthTries 3
      MaxSessions 2
      PasswordAuthentication no
      PermitEmptyPasswords no
      KbdInteractiveAuthentication no
      UsePAM yes
      AllowAgentForwarding no
      AllowTcpForwarding no
      X11Forwarding no
      PrintMotd no
      TCPKeepAlive no
      Compression no
      ClientAliveCountMax 2
      AcceptEnv LANG LC_*
      Subsystem	sftp	/usr/lib/openssh/sftp-server

# install and configure
runcmd:
  # Configure firewall

# Display status and information
  - sudo ufw status verbosey
  - sudo ufw allow 80
  - sudo ufw allow 443
  # Allow SSH
  - ufw allow ssh
  - ufw --force enable
   - sudo ufw status

  # disable root login
  - passwd -l root

  # install auditd
  - sudo apt install auditd -y
  # enable auditd
  - sudo systemctl enable auditd
  - sudo systemctl start auditd
