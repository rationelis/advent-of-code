#!/usr/bin/env bash
set -euo pipefail

default_limit=25

# All years have 25 days except 2025 (only 12)
special_year=2025
special_limit=12

echo "# Advent of Code"
echo
echo "https://adventofcode.com"
echo
echo "| Year | Progress | % |"
echo "|---|---|---|"

# find directories named aocYYYY
for dir in aoc20*; do
    [[ -d "$dir" ]] || continue

    year="${dir#aoc}"

    # count solved days
    # assume files named dayXX.rs inside src or root
    # we check recursively just in case
    solved=$(find "$dir" -type f -name 'day*.rs' | grep -o 'day[0-9][0-9]\.rs' | wc -l)

    # determine total days of that year
    if [[ "$year" == "$special_year" ]]; then
        total=$special_limit
    else
        total=$default_limit
    fi

    # percent
    percent=$((100 * solved / total))

    # progress bar (20 chars wide)
    barwidth=20
    filled=$((barwidth * solved / total))
    bar=$(printf "%${filled}s" | tr ' ' '#')
    bar="$bar$(printf "%$((barwidth-filled))s" | tr ' ' '.')"

    echo "| $year | \`[$bar]\` $solved/$total | $percent% |"
done

