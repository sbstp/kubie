#!/bin/bash

set -Eeuo pipefail

if [[ "$#" -ne 1 ]] ; then
    echo "Usage: release.bash [version]" >&2
    exit 1
fi

if [[ "$(git rev-parse --abbrev-ref HEAD)" != "master" ]] ; then
    echo "Not in master branch" >&2
    exit 1
fi

VERSION="$1"
CURRENT_VERSION="$(cat Cargo.toml | egrep 'version = "[0-9]+\.[0-9]+\.[0-9]+"' | egrep -o "[0-9]+\.[0-9]+\.[0-9]+")"

if [[ "$VERSION" == "$CURRENT_VERSION" ]] ; then
    echo "Version has not changed: $VERSION" >&2
    exit 1
fi

sed -i "s/version = \"[0-9]\\+\\.[0-9]\\+\\.[0-9]\\+\"/version = \"$VERSION\"/" Cargo.toml
cargo check
git add Cargo.*
git commit -m "Release version $VERSION"
git tag "v$VERSION"
git push origin master
git push origin master --tags
cargo publish
