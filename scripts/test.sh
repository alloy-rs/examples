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
# 2. Filter out the examples that have external dependencies or are not meant to be run.
# 3. Pre-build the filtered examples prior to running them.
# 4. Run all the examples in parallel (up to 10) that are left after filtering.
function main () {
    local cargo_args=("$@")
    local examples=()
    local metadata
    metadata="$(cargo metadata "${cargo_args[@]}" --format-version 1 --no-deps)"

    while IFS= read -r example; do
        case "$example" in
            any_network|aws_signer|builtin|debug_trace_call_many|ethereum_wallet|\
            foundry_fork_db|gcp_signer|geth_local_instance|ipc|keystore_signer|\
            ledger_signer|permit2_signature_transfer|reth_local_instance|\
            send_eip7594_transaction|subscribe_all_logs|subscribe_logs|\
            subscribe_pending_transactions|trace_call_many|trace_call|\
            trace_transaction|trezor_signer|urgent_filler|ws_auth|ws|yubi_signer)
                continue
                ;;
        esac
        examples+=("$example")
    done < <(
        printf '%s' "$metadata" \
            | jq -r '.packages[].targets[] | select(.kind | index("example")) | .name' \
            | sort -u
    )

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
