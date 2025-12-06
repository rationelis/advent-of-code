#!/usr/bin/env bash
set -euo pipefail

for rs_file in src/day*.rs; do
    day=$(basename "$rs_file" .rs)
    input_file="src/${day}.txt"

    echo "=== Running $day ==="

    cargo run --bin "$day" < "$input_file"

    echo
done
