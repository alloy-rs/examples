#!/usr/bin/env bash

# Exit if anything fails.
set -eo pipefail

# Utilities
GREEN="\033[00;32m"

function log () {
  echo -e "$1"
  echo "################################################################################"
  echo "#### $2 "
  echo "################################################################################"
  echo -e "\033[0m"
}

# This script will do the following:
#
# 1. Gather all example targets from Cargo metadata.
# 2. Validate the deterministic runtime allowlist against those targets.
# 3. Pre-build the allowlisted examples prior to running them.
# 4. Run the examples in parallel.
function main () {
    local cargo_args=("$@")
    local examples=()
    local metadata
    metadata="$(cargo metadata "${cargo_args[@]}" --format-version 1 --no-deps)"
    local all_examples
    all_examples="$(
        printf '%s' "$metadata" \
            | jq -r '.packages[].targets[] | select(.kind | index("example")) | .name' \
            | sort -u
    )"
    local script_dir
    script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" >/dev/null 2>&1 && pwd)"

    while IFS= read -r example; do
        [[ -z "$example" || "$example" == \#* ]] && continue
        if ! grep -Fqx "$example" <<< "$all_examples"; then
            echo "Unknown example in runtime allowlist: $example" >&2
            exit 1
        fi
        examples+=("$example")
    done < "$script_dir/runtime-examples.txt"

    log "$GREEN" "Building..."

    # Pre-build the filtered examples prior to running them.
    local build_args=()
    for example in "${examples[@]}"; do
        build_args+=(--example "$example")
    done
    cargo build "${cargo_args[@]}" "${build_args[@]}"

    log "$GREEN" "Running..."

    # shellcheck disable=SC2016
    printf '%s\n' "${examples[@]}" | xargs -P4 -I{} bash -c '
        name="$1"
        bin="./target/debug/examples/$name"

        if [[ -x "$bin" ]]; then
            if "$bin" >/dev/null; then
                echo "Successfully ran: $name"
            else
                echo "Failed to run: $name" >&2
                exit 1
            fi
        else
            echo "Missing binary: $bin" >&2
            exit 1
        fi
    ' -- {}

    log "$GREEN" "Done"
}

# Run the main function.
# This prevents partial execution in case of incomplete downloads.
main "$@"
