#!/bin/bash

function run_build() {
    local HERE=$(cd `dirname $0`; pwd -P)
    local PROJECT_ROOT=$(cd $HERE/..; pwd -P)
    cd "$PROJECT_ROOT" || die "Failed to cd to $PROJECT_ROOT"
    # remove target dir
    rm -rf ./target
    
    # bundle - mac
    # ( cargo bundle --release --target rustup x86_64-apple-darwin ) || die "Failed to bundle for macos"

    # bundle - windows (msvc) (only works if I'm in a windows (not ubuntu) running this script)
    # ( cargo bundle --release --target x86_64-pc-windows-msvc ) || die "Failed to bundle for windows (msvc)"
    # ( cross build --release --target x86_64-pc-windows-msvc ) || die "Failed to build for windows (msvc)"

    # not working - bundle - windows (gnu) (works in ubuntu)
    # dependencies:
    #  sudo apt-get install mingw-w64 
    # ( cargo bundle --release --target x86_64-pc-windows-gnu ) || die "Failed to bundle for windows (gnu)"
    # ( cross build --release --target x86_64-pc-windows-gnu ) || die "Failed to build for windows (gnu)"

    # bundle - linux
    # ( cargo bundle --release --target x86_64-unknown-linux-gnu ) || die "Failed to bundle for linux"
    ( cross bundle --release --target x86_64-unknown-linux-gnu ) || die "Failed to build for linux"
}

function die() {
    echo "$@" >&2
    exit 1
}

( run_build )
