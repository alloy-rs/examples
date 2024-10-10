#!/usr/bin/env bash

# Exit if anything fails
set -eo pipefail

# This script will do the following:
#
# 1. Run all examples with some exceptions.
function main () {
    export examples="$(
        cargo run --example 2>&1 \
            | grep -E '^ ' \
            | grep -v \
            -e 'any_network' \
            -e 'builtin' \
            -e 'geth_local_instance' \
            -e 'ipc' \
            -e 'ledger_signer' \
            -e 'reth_db_layer' \
            -e 'reth_db_provider' \
            -e 'reth_local_instance' \
            -e 'subscribe_all_logs' \
            -e 'subscribe_logs' \
            -e 'subscribe_pending_transactions' \
            -e 'trace_call' \
            -e 'trace_transaction' \
            -e 'trezor_signer' \
            -e 'ws_auth' \
            -e 'ws' \
            -e 'yubi_signer' \
            | xargs -n1 echo
    )"

    # Run the examples with the current version of Alloy
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

# Run the main function
# This prevents partial execution in case of incomplete downloads
main