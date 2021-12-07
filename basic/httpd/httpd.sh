#!/bin/bash
# Builds the cgi-bin scripts and copies them
# into Apache's cgi-bin directory.
set -e

if [[ ! -r WORKSPACE ]]; then
    echo "Must be run from the workspace root"
    exit 1
fi

TARGET="/usr/lib/cgi-bin"
if [[ ! -d $TARGET ]]; then
    echo "Target $TARGET does not exist or is not a directory"
    exit 2
fi

function copy_binary {
    bazel build //basic/$1

    local BAZEL_BIN=$(bazel info bazel-bin)
    if [[ ! -d $BAZEL_BIN ]]; then
        echo "$BAZEL_BIN does not exist or is not a directory"
        exit 3
    fi

    local BINARY="$BAZEL_BIN/basic/$1/$1"
    if [[ ! -r $BINARY ]]; then
        echo "$BINARY does not exist or is not a file"
        exit 4
    fi

    cp $BINARY /usr/lib/cgi-bin/
    chmod 555 /usr/lib/cgi-bin/$1
}

sudo rm -f /usr/lib/cgi-bin/*
copy_binary cgi_test
copy_binary gwbasic_cgi_bin
