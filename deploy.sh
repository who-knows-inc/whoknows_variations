## This is the deployment script for the WhoKnows project that is used to bundle the application into a tarball and deploy it to the server

#!/bin/bash

# 1.1 Configuration

## configure these for your environment
PKG="whoknows_nooneknows"                       # cargo package name
TARGET="x86_64-unknown-linux-gnu"               # remote target
ASSETS=("Rocket.toml" "static" "templates" "migrations") # list of assets to bundle
BUILD_DIR="target/${TARGET}/release"            # cargo build directory
DEPLOY_DIR="deploy"                             # Temporary deployment directory

# Ensure target toolchain is present
echo "Adding rust target $TARGET..."
rustup target add $TARGET
# Cross-compile
echo "Building project..."
cargo zigbuild --target $TARGET --release

# Check if the binary was built successfully
if [ ! -f "${BUILD_DIR}/${PKG}" ]; then
    echo "Error: Binary ${PKG} not found in ${BUILD_DIR}."
    exit 1
fi

# Create a temporary deployment directory
echo "Creating deployment directory..."
mkdir -p "${DEPLOY_DIR}"

# Copy assets to the deployment directory
echo "Copying assets..."
cp -r "${ASSETS[@]}" "${DEPLOY_DIR}/"

# Copy the binary to the deployment directory
echo "Copying binary..."
cp "${BUILD_DIR}/${PKG}" "${DEPLOY_DIR}/"

# Create the tarball from the deployment directory
echo "Creating tarball..."
tar -cvzf "${PKG}.tar.gz" -C "${DEPLOY_DIR}" .

# Clean up the deployment directory
echo "Cleaning up..."
rm -rf "${DEPLOY_DIR}"

echo "Deployment bundle ${PKG}.tar.gz created successfully."