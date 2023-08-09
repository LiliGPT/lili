#!/bin/bash

function run_build() {
    local HERE=$(cd `dirname $0`; pwd -P)
    local PROJECT_ROOT=$(cd $HERE/..; pwd -P)
    # remove target dir
    cd "$PROJECT_ROOT" || die "Failed to cd to $PROJECT_ROOT"
    rm -rf ./target
    # bundle
    # ( cargo bundle --release --target rustup x86_64-apple-darwin ) || die "Failed to bundle for macos"
    # ( cargo bundle --release --target x86_64-pc-windows-msvc ) || die "Failed to bundle for windows"
    ( cargo bundle --release --target x86_64-unknown-linux-gnu ) || die "Failed to bundle for linux"
}

function die() {
    echo "$@" >&2
    exit 1
}

( run_build )
