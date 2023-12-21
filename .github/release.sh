#!/usr/bin/env bash
set -e

# Check if version argument is provided
if [ "$#" -ne 1 ]; then
    echo "Usage: $0 <new_version>"
    exit 1
fi
new_version=$1

# Update version in Cargo.toml
sed -i "s/^version = \"[0-9]*\.[0-9]*\.[0-9]*\"\$/version = \"$new_version\"/" "Cargo.toml"

# Create and push tag
git tag -a v$new_version -m "v$new_version"
git push origin v$new_version

# Update changelog
git cliff -o CHANGELOG.md
git commit -S -m "chore: Update changelog"
git push

echo "🎉 $new_version released"
