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
# 1. Gather all the examples from the output of `cargo run --example` command.
# 2. Filter out the examples that have external dependencies or are not meant to be run.
# 3. Pre-build the filtered examples prior to running them.
# 4. Run all the examples in parallel (up to 10) that are left after filtering.
function main () {
    export examples="$(
        cargo run --example 2>&1 \
            | grep -E '^ ' \
            | grep -v \
            -e 'any_network' \
            -e 'aws_signer' \
            -e 'builtin' \
            -e 'debug_trace_call_many' \
            -e 'ethereum_wallet' \
            -e 'foundry_fork_db' \
            -e 'gcp_signer' \
            -e 'geth_local_instance' \
            -e 'ipc' \
            -e 'keystore_signer' \
            -e 'ledger_signer' \
            -e 'permit2_signature_transfer' \
            -e 'reth_db_layer' \
            -e 'reth_db_layer' \
            -e 'reth_db_provider' \
            -e 'reth_db_provider' \
            -e 'reth_local_instance' \
            -e 'subscribe_all_logs' \
            -e 'subscribe_logs' \
            -e 'subscribe_pending_transactions' \
            -e 'trace_call_many' \
            -e 'trace_call' \
            -e 'trace_transaction' \
            -e 'trezor_signer' \
            -e 'ws_auth' \
            -e 'ws' \
            -e 'yubi_signer' \
            | xargs -n1 echo
    )"

    log $GREEN "Building..."

    # Pre-build the filtered examples prior to running them.
    cargo build $(printf -- '--example %s ' $examples)

    log $GREEN "Running..."

    # Run all the examples that are left after filtering.
    printf '%s\n' $examples \
    | xargs -P4 -I{} bash -c '
        bin="./target/debug/examples/{}"
        if [[ -x "$bin" ]]; then
            "$bin" >/dev/null \
            && echo "Successfully ran: {}" \
            || { echo "Failed to run: {}" >&2; exit 1; }
        else
            echo "Missing binary: $bin" >&2
            exit 1
        fi
        '

    log $GREEN "Done"
}

# Run the main function.
# This prevents partial execution in case of incomplete downloads.
main