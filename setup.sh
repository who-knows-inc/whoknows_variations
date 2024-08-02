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
read -p "Enter the DOCKER_GITHUB_USERNAME: " DOCKER_GITHUB_USERNAME
gh secret set DOCKER_GITHUB_USERNAME -b"$DOCKER_GITHUB_USERNAME"

print_divider
echo "Write any random greeting. The goal is that it can be saved to a .env file on the deployed server and used in production." 
read -p "Enter the ENV_GREETING: " ENV_GREETING
gh secret set ENV_GREETING -b"$ENV_GREETING"

print_divider
echo -e "${GREEN}Success!${NC} Next step is to set the production environment variables${NC}"
