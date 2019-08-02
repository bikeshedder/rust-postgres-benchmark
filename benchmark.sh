#!/bin/bash

set -e

DIRS=(
    "postgres-0.15.2"
    "postgres-0.16-rc.2"
    "tokio-postgres-0.3.0"
    "tokio-postgres-0.4.0-rc.3"
)

echo "Compiling..."
for DIR in ${DIRS[@]}; do
    (cd $DIR; cargo build --release)
done

echo "Running benchmarks..."
for DIR in ${DIRS[@]}; do
    echo
    echo "*** $DIR"
    (cd $DIR; cargo run --release --quiet)
done
