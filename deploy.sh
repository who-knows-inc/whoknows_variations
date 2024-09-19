## This is the deployment script for the WhoKnows project that is used to bundle the application into a tarball and deploy it to the server

#!/bin/bash

# Load environment variables from .env file
if [ -f .env ]; then
  set -o allexport
  source .env
  set +o allexport
fi

# 1.1 Configuration

## configure these for your environment
PKG="whoknows_nooneknows"                       # cargo package name
TARGET="x86_64-unknown-linux-gnu"               # remote target
ASSETS=("Rocket.toml" "static" "templates")     # list of assets to bundle
BUILD_DIR="target/${TARGET}/release"            # cargo build directory

# Ensure required environment variables are set
: "${DEPLOY_DIR:?DEPLOY_DIR is not set}"
: "${VM_USER:?VM_USER is not set}"
: "${VM_HOST:?VM_HOST is not set}"
: "${DEPLOY_KEY_PATH:?DEPLOY_KEY_PATH is not set}"

# Ensure target toolchain is present
echo "Adding rust target $TARGET..."
rustup target add $TARGET

# Cross-compile
echo "Building project..."
cargo zigbuild --target $TARGET --release

# Bundle
echo "Creating tarball..."
tar -cvzf "${PKG}.tar.gz" "${ASSETS[@]}" -C "${BUILD_DIR}" "${PKG}"

echo "Deployment bundle ${PKG}.tar.gz created successfully."

# 2. Deployment
## copy the tarball to the remote server
echo "Copying tarball to remote server..."
scp -i "$DEPLOY_KEY" "${PKG}.tar.gz" ${VM_USER}@${VM_HOST}:${DEPLOY_DIR}

## extract and run the app on the VM
ssh -i "$DEPLOY_KEY" ${VM_USER}@${VM_HOST} << EOF
  cd ${DEPLOY_DIR}
  tar xzvf ${PKG}.tar.gz
  sudo systemctl restart whoknows.service
EOF

echo "Deployment completed successfully."

