#!/bin/bash

set -e

DIRS=(
    #"postgres-0.15.2" # build error
    "postgres-0.16-rc.2"
    "postgres-0.17.5"
    "postgres-0.19.4"
    #"tokio-postgres-0.3.0" # build error
    #"tokio-postgres-0.4.0-rc.3" # build error
    "tokio-postgres-0.5.0-alpha.2"
    "tokio-postgres-0.5.5"
    "tokio-postgres-0.7.7"
    "tokio-postgres-0.7.7_current_thread"
)

for DIR in ${DIRS[@]}; do
    echo "Compiling $DIR..."
    (cd $DIR; cargo build --release)
done

echo "Running benchmarks..."
for DIR in ${DIRS[@]}; do
    echo
    echo "### $DIR"
    (cd $DIR; cargo run --release --quiet)
done
