#!/usr/bin/env bash
set -e

# Check if version argument is provided
if [ "$#" -ne 1 ]; then
    echo "Usage: $0 <new_version>"
    exit 1
fi
new_version=$1

echo ""
echo "  ⏫ Update version in Cargo.toml"
echo ""
sed -i "s/^version = \"[0-9]*\.[0-9]*\.[0-9]*\"\$/version = \"$new_version\"/" "Cargo.toml"
git add Cargo.toml
git commit -S -m "chore: Bump version to $new_version"
git push

echo ""
echo "  🏷️  Create and push tag"
echo ""
git tag -a v$new_version -m "v$new_version"
git push origin v$new_version

# Update changelog
echo ""
echo "  🏔️  Update CHANGELOG.md"
echo ""
git cliff -o CHANGELOG.md
git add CHANGELOG.md
git commit -S -m "chore: Update CHANGELOG.md"
git push

echo ""
echo "  🎉 Version $new_version released"
