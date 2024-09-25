## This is the deployment script for the WhoKnows project that is used to bundle the application into a tarball and deploy it to the server

#!/bin/bash

# 1.1 Configuration

## configure these for your environment
PKG="whoknows_nooneknows"                       # cargo package name
TARGET="x86_64-unknown-linux-gnu"               # remote target
ASSETS=("Rocket.toml" "static" "templates" "migrations") # list of assets to bundle
BUILD_DIR="target/${TARGET}/release"            # cargo build directory

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

