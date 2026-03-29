echo "============================================================================================"
echo "Disable Prompts and Update the system"
echo "============================================================================================"
echo "\$nrconf{kernelhints} = 0;" | sudo tee -a /etc/needrestart/needrestart.conf
echo "\$nrconf{restart} = 'a';" | sudo tee -a /etc/needrestart/needrestart.conf
sudo DEBIAN_FRONTEND=noninteractive NEEDRESTART_MODE=a NEEDRESTART_SUSPEND=1 apt-get update
sudo DEBIAN_FRONTEND=noninteractive NEEDRESTART_MODE=a NEEDRESTART_SUSPEND=1 apt-get dist-upgrade --yes --allow-downgrades --allow-remove-essential --allow-change-held-packages -o Dpkg::Options::="--force-confnew" -o Dpkg::Options::="--force-confdef"
sudo apt-get install -y software-properties-common
sudo rm -f /var/run/reboot-required /var/run/reboot-required.pkgs
echo "============================================================================================"
echo "Install Node.js"
echo "============================================================================================"
curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.2/install.sh | bash
chmod +x ~/.nvm/nvm.sh
source ~/.bashrc
nvm install 16
echo "============================================================================================"
echo "Set python3.6 as default"
echo "============================================================================================"
sudo apt install -y python3-pip
echo "============================================================================================"
sudo pip3 install docker requests urllib3
echo "============================================================================================"
echo "Install Docker and give user permission"
echo "============================================================================================"
sudo install -m 0755 -d /etc/apt/keyrings
sudo curl -fsSL https://download.docker.com/linux/ubuntu/gpg -o /etc/apt/keyrings/docker.asc
sudo chmod a+r /etc/apt/keyrings/docker.asc
echo "deb [arch=$(dpkg --print-architecture) signed-by=/etc/apt/keyrings/docker.asc] https://download.docker.com/linux/ubuntu $(. /etc/os-release && echo "$VERSION_CODENAME") stable" | sudo tee /etc/apt/sources.list.d/docker.list > /dev/null
sudo apt-get update
sudo apt-get install -y docker-ce docker-ce-cli containerd.io docker-buildx-plugin docker-compose-plugin
sudo usermod -aG docker $(whoami)
echo "============================================================================================"
echo "Create a new user for GitHub Actions. Assuming that they will reuse the same SSH key as the admin user."
echo "It is recommended that you generate a new SSH key for the GitHub Actions user and add it to `~/.ssh/authorized_keys`."
echo "Remember to change SSH_KEY= Github Secret to the correct value"
echo "============================================================================================"
USERNAME="githubactionsuser"
SSH_KEY="ssh-rsa AAAAB3NzaC1yc2EAAAADAQABAAACAQC5DBQ/Jtcq6WP5rvVLXe9B0BEi9NpkGiFh9IZA+s4BptPaxFJL8lB/9ZLkiXYqRGjsZIPE0zBTAyaCyciRDpDIoo1TIeP7Sn+4XR0LXSS91bQ+o4Iq0e6izlrcXA5fqVlDyrjODmJvrBV8+EkG+WhUPWrlj+7fb/zrTdVj0aEnALLFxkUoj7H2z6bfwOca0qQJnTioEYXi6/z6c3MiaWz7QByDPYA9zuQ7FE9HRjmEZEf1f6pCx8dU8s2HCzawHzjujEJaEgC4Ts/d91NT8wyRSuLz4TX+ZgGHcWZ4ilIHr4Ytg2cSr2Ccm+K1Z/V5+EFB+hA1wuNAPSlRK8H4Ooqpd2OMV01JNbCHjJLVnQRqB7F6JA41RqBn3z1qt7/j945wcK3m+KxBe23iDJpksOcnUg1YiWOaCiGOVzUJsK9TLdnmR7jywrUOC6Lxv9egEasso0l2AB2og38S30njDg/cGg0ec7dD6pAhlUHiF/OqBoxCJ5aq6KmfE1ETTzCQtrbMSyf15r/0/OqIoxRFDNBkfKKF+/ZoypAwTGQejOLRKx9IAYdzrG+qJC96ZNJCZkmL3jQnqG7CnEYMmJkLZ4sTsvsuAJAehPFXfTecbJQBEcT9i81mn4YXwt68HRG8YXAISptXbtbDvBekbnZifQbiEkNQbNKUvS+DInXSspYefQ== andl@kea.dk"
sudo adduser --disabled-password --gecos "" $USERNAME
sudo usermod -aG sudo $USERNAME
sudo mkdir /home/$USERNAME/.ssh
sudo chmod 700 /home/$USERNAME/.ssh
echo $SSH_KEY | sudo tee /home/$USERNAME/.ssh/authorized_keys
sudo chmod 600 /home/$USERNAME/.ssh/authorized_keys
sudo chown -R $USERNAME:$USERNAME /home/$USERNAME/.ssh
sudo systemctl restart sshd
sudo usermod -aG docker $USERNAME
echo "============================================================================================"
echo "Install Fail2Ban"
echo "============================================================================================"
sudo apt install -y fail2ban
sudo systemctl enable fail2ban
sudo systemctl start fail2ban
echo "============================================================================================"