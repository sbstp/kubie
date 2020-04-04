#!/bin/bash

set -eux

if [[ "$OSTYPE" == "linux-gnu" ]] ; then
    KUBIE_OS="linux"
elif [[ "$OSTYPE" == "darwin"* ]] ; then
    KUBIE_OS="darwin"
else
    echo "Unsupported OS '$OSTYPE'" >&2
    exit 1
fi

OSARCH=$(uname -m)
if [[ "$OSARCH" == "x86_64" ]] ; then
    KUBIE_ARCH="amd64"
else
    echo "Unsupported arch '$OSARCH'" >?2
    exit 1
fi

cargo build --release
strip target/release/kubie
mkdir -p binaries
cp target/release/kubie "binaries/kubie-$KUBIE_OS-$KUBIE_ARCH"
