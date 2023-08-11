#!/bin/bash

function main() {
    # Set the download URL
    URL="https://github.com/LiliGPT/lili/releases/download/v0.0.1/lili_v0.0.1.tar.gz"

    # Set the destination directory
    DEST_DIR="/usr/local/bin"

    # Create a temporary directory
    TEMP_DIR=$(mktemp -d)

    # Change to the temporary directory
    cd $TEMP_DIR

    # Download the tar.gz file
    curl -L $URL -o lili.tar.gz

    # Extract the tar.gz file
    tar -xzf lili.tar.gz

    # Move the binary to the destination directory
    sudo mv lili $DEST_DIR

    # Make the binary executable
    sudo chmod +x $DEST_DIR/lili

    # Clean up
    rm -rf $TEMP_DIR

    echo "Installation complete! You can now run 'lili' from the command line."
}

( main )
