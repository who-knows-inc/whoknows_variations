#!/bin/bash

# 1.1 Configuration

## configure these for your environment
PKG="whoknows_nooneknows"                       # cargo package name
TARGET="x86_64-unknown-linux-gnu"               # remote target
ASSETS=("Rocket.toml" "static" "templates")     # list of assets to bundle
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

# 2. Deployment
## copy the tarball to the remote server
echo "Copying tarball to remote server..."
scp whoknows_nooneknows.tar.gz whoknows@49.13.163.245:/var/www/whoknows

## cmds to run on vm to extract and run the app
# ssh whoknows@49.13.163.245 "cd /home/whoknows && tar -xvzf whoknows_nooneknows.tar.gz && cd whoknows_nooneknows && ./whoknows_nooneknows"


