echo "============================================================================================"
echo "Update the system"
echo "============================================================================================"
sudo dnf -y upgrade --refresh
echo "============================================================================================"
echo "Install Docker (includes the compose plugin) and give user permission"
echo "============================================================================================"
sudo dnf install -y dnf-plugins-core
sudo dnf config-manager --add-repo https://download.docker.com/linux/centos/docker-ce.repo
sudo dnf install -y docker-ce docker-ce-cli containerd.io docker-buildx-plugin docker-compose-plugin
sudo systemctl enable --now docker
sudo usermod -aG docker $(whoami)
echo "============================================================================================"
echo "Create a new user for GitHub Actions. Reuses the same SSH key as the admin user,"
echo "copied from the admin's authorized_keys that Azure provisioned with var.ssh_public_key."
echo "============================================================================================"
USERNAME="githubactionsuser"
sudo useradd -m $USERNAME
sudo usermod -aG wheel $USERNAME
sudo mkdir -p /home/$USERNAME/.ssh
sudo chmod 700 /home/$USERNAME/.ssh
sudo cp ~/.ssh/authorized_keys /home/$USERNAME/.ssh/authorized_keys
sudo chmod 600 /home/$USERNAME/.ssh/authorized_keys
sudo chown -R $USERNAME:$USERNAME /home/$USERNAME/.ssh
sudo systemctl restart sshd
sudo usermod -aG docker $USERNAME
echo "============================================================================================"
echo "Install Fail2Ban"
echo "============================================================================================"
sudo dnf install -y epel-release
sudo dnf install -y fail2ban
sudo systemctl enable fail2ban
sudo systemctl start fail2ban
echo "============================================================================================"