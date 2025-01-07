#!/bin/bash

# Build the project
cargo build

# Check if build was successful
if [ $? -eq 0 ]; then
    # Run the CLI tool with any provided arguments
    ./target/debug/fussy-food "$@"
else
    echo "Build failed"
    exit 1
fi
