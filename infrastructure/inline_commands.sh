echo "============================================================================================"
echo "Update packages"
echo "============================================================================================"
sudo apt-get update && sudo apt-get install -y software-properties-common
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
sudo apt install -y python3.6
sudo update-alternatives --install /usr/bin/python3 python3 /usr/bin/python3.6 1
echo "============================================================================================"
echo "Install Ansible related packages"
echo "============================================================================================"
sudo pip3 install docker requests urllib3
echo "============================================================================================"
echo "Install Docker and give user permission"
echo "============================================================================================"
sudo apt install -y docker.io
sudo usermod -aG docker $(whoami)
sudo systemctl restart docker
echo "============================================================================================"
echo "Set host key checking to false, otherwise it will wait for a 'yes' that blocks the execution"
echo "============================================================================================"
export ANSIBLE_HOST_KEY_CHECKING=False
echo "============================================================================================"
echo "Install Ansible"
echo "============================================================================================"
sudo apt-add-repository --yes --update ppa:ansible/ansible
sudo apt-get install -y ansible
echo "============================================================================================"
echo "Add SSH key to the authorized_keys file"
echo "============================================================================================"
chmod 600 /home/adminuser/.ssh/id_rsa
echo "//todo undo this later on: ansible-playbook -i 'localhost,' /home/adminuser/playbook.yml"