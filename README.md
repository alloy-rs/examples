# Alloy Examples

Example code using [alloy](https://github.com/alloy-rs/alloy) and [alloy-core](https://github.com/alloy-rs/core).

These examples demonstrate the main features of [Alloy](https://github.com/alloy-rs/alloy) and how to use them. 
To run an example, use the command `cargo run --example <Example>`.

```sh
cargo run --example mnemonic_signer
```

---

## Table of Contents

- [ ] Address book
- [ ] Anvil
    - [ ] Boot anvil
    - [ ] Deploy contracts
    - [ ] Fork
    - [ ] Testing
- [ ] Big numbers
    - [ ] Comparison and equivalence
    - [ ] Conversion
    - [ ] Creating Instances
    - [ ] Math operations
    - [ ] Utilities
- [ ] Contracts
    - [ ] Abigen
    - [ ] Compile
    - [ ] Creating Instances
    - [ ] Deploy Anvil
    - [ ] Deploy from ABI and bytecode
    - [ ] Deploy Moonbeam
    - [ ] Events
    - [ ] Events with meta
    - [ ] Methods
- [ ] Events
  - [ ] Logs and filtering
  - [ ] Solidity topics
- [ ] Middleware
  - [ ] Builder
  - [ ] Create custom middleware
  - [ ] Gas escalator
  - [ ] Gas oracle
  - [ ] Nonce manager
  - [ ] Policy
  - [ ] Signer
  - [ ] Time lag
  - [ ] Transformer
- [ ] Providers
  - [ ] Http
  - [ ] IPC
  - [ ] Mock 
  - [ ] Quorum
  - [ ] Retry
  - [ ] RW
  - [ ] WS
- [x] Queries
  - [x] [Contract storage](./examples/queries/examples/query_contract_storage.rs)
  - [x] [Contract deployed bytecode](./examples/queries/examples/query_deployed_bytecode.rs)
  - [x] [Logs](./examples/queries/examples/query_logs.rs)
- [ ] Subscriptions
  - [ ] Watch blocks
  - [ ] Subscribe events by type
  - [ ] Subscribe logs
- [ ] Transactions
  - [ ] Call override
  - [ ] Create raw transaction
  - [ ] Create typed transaction
  - [ ] Decode input
  - [ ] EIP-1559
  - [ ] ENS
  - [ ] Estimate gas
  - [ ] Get gas price
  - [ ] Get gas price USD
  - [ ] Remove liquidity
  - [ ] Set gas for a transaction
  - [ ] Send raw transaction
  - [ ] Send typed transaction
  - [ ] Trace
  - [ ] Transaction receipt
  - [ ] Transaction status
  - [ ] Transfer ETH
  - [ ] Transfer ERC20 token
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
  - [ ] Keystore signer