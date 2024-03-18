# Alloy Examples

Example code using [alloy](https://github.com/alloy-rs/alloy) and [alloy-core](https://github.com/alloy-rs/core).

These examples demonstrate the main features of [Alloy](https://github.com/alloy-rs/alloy) and how to use them. 
To run an example, use the command `cargo run --example <Example>`.

```sh
cargo run --example mnemonic-signer
```

---

## Table of Contents

- [ ] Address book
- [ ] Anvil
    - [ ] Boot anvil
    - [ ] Deploy contracts
    - [ ] Fork
    - [ ] Testing
- [x] Big numbers
    - [x] [Comparison and equivalence](./examples/big-numbers/examples/comparison_equivalence.rs)
    - [x] [Conversion](./examples/big-numbers/examples/conversion.rs)
    - [x] [Creating Instances](./examples/big-numbers/examples/create_instances.rs)
    - [x] [Math operations](./examples/big-numbers/examples/math_operations.rs)
    - [x] [Math utilities](./examples/big-numbers/examples/math_utilities.rs)
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
- [ ] Queries
  - [ ] Blocks
  - [ ] Contracts
  - [ ] Events
  - [ ] Paginated logs
  - [ ] UniswapV2 pair
  - [ ] Transactions
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
  - [x] [Ledger signer](./examples/wallets/ledger_signer.rs)
  - [x] [Private key signer](./examples/wallets/private_key_signer.rs)
  - [x] [Mnemonic signer](./examples/wallets/mnemonic_signer.rs)
  - [x] [Sign message](./examples/wallets/sign_message.rs)
  - [x] [Sign permit hash](./examples/wallets/sign_permit_hash.rs)
  - [x] [Trezor signer](./examples/wallets/trezor_signer.rs)
  - [x] [Yubi signer](./examples/wallets/yubi_signer.rs)
  - [ ] Keystore signer