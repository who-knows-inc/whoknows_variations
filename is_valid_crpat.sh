#!/bin/bash

RED='\033[0;31m'
GREEN='\033[0;32m'
BLUE='\033[0;34m'
GREY='\033[0;90m'
NC='\033[0m'

function print_divider() {
    TERM_WIDTH=$(tput cols)

    # Create a full-width divider using dashes
    printf -v divider '%*s' "$TERM_WIDTH" '' 
    echo "${divider// /-}"
}

echo -e "${GREEN}This script will test if your cr pat is valid for this repository!${NC}"
print_divider

if ! command -v gh &> /dev/null; then
  echo -e "${RED}Error: gh (GitHub CLI) is not installed."
  exit 1
fi


# Try to extract GitHub URL from current repo
AUTO_URL=$(git config --get remote.origin.url 2>/dev/null | sed -E 's#(git@|https://)github.com[:/]#https://github.com/#' | sed 's/.git$//')

if [ -n "$AUTO_URL" ]; then
	echo -e "${GREY}Detected GitHub URL: $AUTO_URL${NC}"
	read -p "Use this repo? [Y/n]: " CONFIRM
	if [[ "$CONFIRM" =~ ^[Nn]$ ]]; then
		read -p "Paste GitHub repository URL: " REPOSITORY_URL
	else
		REPOSITORY_URL="$AUTO_URL"
	fi
else
	read -p "Paste GitHub repository URL: " REPOSITORY_URL
fi


read -p "Who is the owner of the CR_PAT (GitHub username): " TOKEN_OWNER
echo

read -p "Enter your GitHub token (CR_PAT): " CR_PAT
echo

REPO_OWNER=$(echo "$REPOSITORY_URL" | cut -d '/' -f 4)
REPO_NAME=$(echo "$REPOSITORY_URL" | cut -d '/' -f 5)

# Test Docker login
if echo "$CR_PAT" | docker login ghcr.io -u "$TOKEN_OWNER" --password-stdin &>/dev/null; then
	echo -e "${GREEN}✔ Docker login succeeded.${NC}"
else
	echo -e "${RED}✘ Docker login failed. Token may lack 'write:packages' or be invalid.${NC}"
fi

# Check if repo is accessible with the token
repo_status=$(curl -s -o /dev/null -w "%{http_code}" \
	-H "Authorization: Bearer $CR_PAT" \
	"https://api.github.com/repos/$REPO_OWNER/$REPO_NAME")

if [ "$repo_status" -eq 200 ]; then
	echo -e "${GREEN}✔ Token has access to repository $REPO_OWNER/$REPO_NAME.${NC}"
elif [ "$repo_status" -eq 404 ]; then
	echo -e "${RED}✘ Repository $REPO_OWNER/$REPO_NAME not found (or token lacks access).${NC}"
elif [ "$repo_status" -eq 403 ]; then
	echo -e "${RED}✘ Forbidden: token lacks repo access or is scoped improperly.${NC}"
else
	echo -e "${RED}✘ Unexpected response $repo_status when accessing repo.${NC}"
fi

# Check effective permissions (works for user + org-owned repos)
perm_json=$(curl -s -H "Authorization: Bearer $CR_PAT" \
	"https://api.github.com/repos/$REPO_OWNER/$REPO_NAME" | jq '.permissions')

can_push=$(echo "$perm_json" | jq -r '.push')

if [ "$can_push" == "true" ]; then
	echo -e "${GREEN}✔ Token user '$TOKEN_OWNER' has push access to the repo.${NC}"
else
	echo -e "${RED}✘ Token user '$TOKEN_OWNER' does not have push access to the repo.${NC}"
fi

# Check token scopes (requires 'gh' CLI)
scopes=$(GH_TOKEN="$CR_PAT" gh api -H "Accept: application/vnd.github+json" /user -i 2>/dev/null | grep -i "x-oauth-scopes")

if echo "$scopes" | grep -q "write:packages"; then
	echo -e "${GREEN}✔ Token has write:packages scope.${NC}"
else
	echo -e "${RED}✘ Token missing write:packages scope.${NC}"
fi

if echo "$scopes" | grep -q "repo"; then
	echo -e "${GREEN}✔ Token has repo scope (required for private repos).${NC}"
else
	echo -e "${RED}✘ Token missing repo scope (required for private repos).${NC}"
fi