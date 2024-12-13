#!/bin/bash

APP_NAME=$(grep name Cargo.toml | cut -d"\"" -f2)
args=()
extra_args=false
for arg in "$@"; do
    if [[ "$arg" == "--" ]]; then
        extra_args=true
        continue
    fi
    if [ "$extra_args" = true ]; then
        args+=("$arg")
    fi
done

if [ "$1" == "dev" ]; then
    cargo run -- ${args[@]}
elif [ "$1" == "build" ]; then
    cargo build --release
elif [ "$1" == "prod" ]; then
    if [ ! -f ./target/release/$APP_NAME ]; then
        cargo build --release
    fi
    ./target/release/$APP_NAME ${args[@]}
elif [ "$1" == "watch" ]; then
    set -x
    arg="run -- ${args[@]}"
    cargo watch -x "$arg"
else
    cargo build --release
    ./target/release/$APP_NAME ${args[@]}
fi
