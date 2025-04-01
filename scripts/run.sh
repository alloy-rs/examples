#!/usr/bin/env bash

# Exit if anything fails.
set -eo pipefail

# This script will do the following:
#
# 1. Gather all the examples from the output of `cargo run --example` command.
# 2. Filter out the examples that have external dependencies or are not meant to be run.
# 1. Run all examples that are left after filtering.
function main () {
    export examples="$(
        cargo run --example 2>&1 \
            | grep -E '^ ' \
            | grep -v \
            -e 'any_network' \
            -e 'aws_signer' \
            -e 'builtin' \
            -e 'debug_trace_call_many' \
            -e 'gcp_signer' \
            -e 'geth_local_instance' \
            -e 'ipc' \
            -e 'ledger_signer' \
            -e 'reth_db_layer' \
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
            -e 'foundry_fork_db' \
            -e 'reth_db_layer' \
            -e 'reth_db_provider' \
            | xargs -n1 echo
    )"

    for example in $examples; do
        cargo run --example $example --quiet 1>/dev/null

        if [ $? -ne 0 ]; then
            echo "Failed to run: $example"
            exit 1
        else
            echo "Successfully ran: $example"
        fi
    done
}

# Run the main function.
# This prevents partial execution in case of incomplete downloads.
main