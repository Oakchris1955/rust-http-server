#!/bin/bash
# Format all .rs files under the tutorial/src directory

find src/**/*.rs -type f -exec rustfmt "{}" \;
echo "Done";