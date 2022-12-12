#!/bin/bash

BLUE='\033[0;34m'
RED='\033[0;31m'
RESET='\033[0;0m'

if [ -z "$1" ]; then
    echo "Usage: $0 [number]"
    exit 1
fi

if [ -d "day${1}" ]; then
    echo -e "${RED}day${1}/${RESET} already exists"
    exit 1
fi

cp -r dayN "day${1}"  # Copy from template
find "day${1}" -type f -exec sed -i "s/dayN/day${1}/g" {} +  # Replace file content
find "day${1}/src" -exec rename "s/dayN/day${1}/" {} \;  # Replace filenames
sed -i "\$i \    \"day${1}\"," Cargo.toml  # Add to Cargo.toml workspace list
echo -e "Created directory ${BLUE}day$1/"
