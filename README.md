# Alloy Examples

These examples demonstrate the main features of [alloy](https://github.com/alloy-rs/alloy) and [alloy-core](https://github.com/alloy-rs/core) as well as how to use them.

[![Telegram chat][telegram-badge]][telegram-url]

[`ethers-rs`]: https://github.com/gakonst/ethers-rs
[telegram-badge]: https://img.shields.io/endpoint?color=neon&style=for-the-badge&url=https%3A%2F%2Ftg.sumanjay.workers.dev%2Fethers_rs
[telegram-url]: https://t.me/ethers_rs

## Usage

To run an example, use the command `cargo run --example <Example>`:

```sh
cargo run --example mnemonic_signer
```

## Overview

This repository contains the following examples:

- [x] Anvil
  - [x] [Deploy contract](./examples/anvil/examples/deploy_contract_anvil.rs)
  - [x] [Fork](./examples/anvil/examples/fork_anvil.rs)
  - [x] [Local](./examples/anvil/examples/local_anvil.rs)
- [x] Big numbers
  - [x] [Comparison and equivalence](./examples/big-numbers/examples/comparison_equivalence.rs)
  - [x] [Conversion](./examples/big-numbers/examples/conversion.rs)
  - [x] [Creating Instances](./examples/big-numbers/examples/create_instances.rs)
  - [x] [Math operations](./examples/big-numbers/examples/math_operations.rs)
  - [x] [Math utilities](./examples/big-numbers/examples/math_utilities.rs)
- [x] Contracts
  - [x] [Deploy from artifact](./examples/contracts/examples/deploy_from_artifact.rs)
  - [x] [Deploy from contract](./examples/contracts/examples/deploy_from_contract.rs)
  - [x] [Generate](./examples/contracts/examples/generate.rs)
- [x] Layers
  - [x] [Recommended layers](./examples/layers/examples/recommended_layers.rs)
  - [x] [Nonce manager](./examples/layers/examples/nonce_layer.rs)
  - [x] [Signature manager](./examples/layers/examples/signer_layer.rs)
- [x] Subscriptions
  - [x] [Subscribe and watch blocks](./examples/subscriptions/examples/subscribe_blocks.rs)
  - [x] [Subscribe to contract events and watch logs](./examples/subscriptions/examples/watch_contract_event.rs)
  - [x] [Event multiplexer](./examples/subscriptions/examples/event_multiplexer.rs)
- [x] Providers
  - [x] [Builder](./examples/providers/examples/builder.rs)
  - [x] [HTTP](./examples/providers/examples/http.rs)
  - [x] [IPC](./examples/providers/examples/ipc.rs)
  - [x] [WS](./examples/providers/examples/ws.rs)
- [x] Queries
  - [x] [Contract storage](./examples/queries/examples/query_contract_storage.rs)
  - [x] [Contract deployed bytecode](./examples/queries/examples/query_deployed_bytecode.rs)
  - [x] [Logs](./examples/queries/examples/query_logs.rs)
- [x] Transactions
  - [x] [Decode input](./examples/transactions/examples/decode_input.rs)
  - [x] [Get gas price in USD](./examples/transactions/examples/gas_price_usd.rs)
  - [x] [Trace call](./examples/transactions/examples/trace_call.rs)
  - [x] [Trace transaction](./examples/transactions/examples/trace_transaction.rs)
  - [x] [Transfer ERC20 token](./examples/transactions/examples/transfer_erc20.rs)
  - [x] [Transfer ETH](./examples/transactions/examples/transfer_eth.rs)
- [ ] Wallets
  - [ ] AWS signer
  - [ ] GCP signer
  - [x] [Ledger signer](./examples/wallets/examples/ledger_signer.rs)
  - [x] [Private key signer](./examples/wallets/examples/private_key_signer.rs)
  - [x] [Mnemonic signer](./examples/wallets/examples/mnemonic_signer.rs)
  - [x] [Sign message](./examples/wallets/examples/sign_message.rs)
  - [x] [Sign permit hash](./examples/wallets/examples/sign_permit_hash.rs)
  - [x] [Trezor signer](./examples/wallets/examples/trezor_signer.rs)
  - [x] [Yubi signer](./examples/wallets/examples/yubi_signer.rs)
  - [x] [Keystore signer](./examples/wallets/examples/keystore_signer.rs)
  - [x] [Create keystore](./examples/wallets/examples/create_keystore.rs)

## Contributing

Thanks for your help improving the project! We are so happy to have you! We have
[a contributing guide](./CONTRIBUTING.md) to help you get involved in the
Alloy project.

Pull requests will not be merged unless CI passes, so please ensure that your
contribution follows the linting rules and passes clippy.

#### License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in these crates by you, as defined in the Apache-2.0 license,
shall be dual licensed as above, without any additional terms or conditions.
</sub>