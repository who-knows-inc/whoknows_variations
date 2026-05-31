#!/bin/bash

RED='\033[0;31m'
GREEN='\033[0;32m'
BLUE='\033[0;34m'
NC='\033[0m'

function print_divider() {
    TERM_WIDTH=$(tput cols)

    # Create a full-width divider using dashes
    printf -v divider '%*s' "$TERM_WIDTH" '' 
    echo "${divider// /-}"
}

echo -e "${GREEN}Welcome to the interactive setup script!${NC}"
print_divider

# if [ ! -f "./infrastructure/terraform.tfstate" ]; then
#   echo -e "${RED}Error: The file ./infrastructure/terraform.tfstate does not exist."
#   echo "Error: This means that you have not run the terraform script to create the infrastructure."
#   exit 1
# fi

if ! command -v gh &> /dev/null; then
  echo -e "${RED}Error: gh (GitHub CLI) is not installed."
  exit 1
fi

if ! gh auth status &> /dev/null; then
  echo -e "${RED}Error: You are not logged in to GitHub CLI.${NC}"
  echo "Please log in using 'gh auth login' and try again."
  exit 1
fi

echo "This script helps you interactively define secrets for Github Action"

print_divider
echo "Create a new Personal Access Token (PAT)."
echo "More info: https://docs.github.com/en/authentication/keeping-your-account-and-data-secure/managing-your-personal-access-tokens"

print_divider
read -p "Enter the CR_PAT: " CR_PAT
gh secret set CR_PAT -b"$CR_PAT"

print_divider
echo "The SSH host is the IP Address of your virtual machine."
read -p "Enter the SSH_HOST: " SSH_HOST
gh secret set SSH_HOST -b"$SSH_HOST"

print_divider
read -p "Enter the SSH_USER: " SSH_USER
gh secret set SSH_USER -b"$SSH_USER"

print_divider
read -p "Is your SSH private key located in ~/.ssh/id_rsa? (Y/n): " SSH_PRIVATE_KEY_DEFAULT
SSH_PRIVATE_KEY_DEFAULT=$(echo "$SSH_PRIVATE_KEY_DEFAULT" | tr '[:upper:]' '[:lower:]')  # Converts to lowercase
if [ -z "$SSH_PRIVATE_KEY_DEFAULT" ] || [ "$SSH_PRIVATE_KEY_DEFAULT" == "y" ] || [ "$SSH_PRIVATE_KEY_DEFAULT" == "yes" ]; then
  SSH_PRIVATE_KEY=$(cat ~/.ssh/id_rsa)
else
  echo "Enter the SSH_PRIVATE_KEY, followed by [ENTER] and then [CTRL+D]:"
  echo -e "${RED}${BOLD}Paste as a multi-line comment. Once done press [ENTER] followed by [CTRL+D]${NC}"
  SSH_PRIVATE_KEY=$(cat)
fi
gh secret set SSH_PRIVATE_KEY -b"$SSH_PRIVATE_KEY"


print_divider
echo -e "${GREEN}Success!${NC} Next step is to set the production environment variables${NC}"
echo "You can do that by running: gh secret set ENV_<VAR_NAME>"
