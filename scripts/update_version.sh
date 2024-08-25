#!/bin/bash

# Read version from version.txt
VERSION=$(cat ../version.txt)

# Update the version in the package.json file
sed -i '' -e "s/\"version\": \".*\"/\"version\": \"$VERSION\"/" ../chrome-extension/package.json

# Update the version in the Cargo.toml file
sed -i '' -e "s/^version = \".*\"/version = \"$VERSION\"/" ../server/bin-shuttle/Cargo.toml
sed -i '' -e "s/^version = \".*\"/version = \"$VERSION\"/" ../server/engine/Cargo.toml

# Update the version in the docs
sed -i '' -e "s/^version = \".*\"/version = \"$VERSION\"/" ../docs/book.toml