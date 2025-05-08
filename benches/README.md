# Benchmarks

## Table of Contents

- [ABI Encoding](#abi-encoding)
- [U256 Operations](#u256-operations)
- [Rlp Encoding and Decoding](#rlp-encoding-and-decoding)
- [JSON-ABI](#json-abi)
  - [Serialization](#serialization)
  - [Deserialization](#deserialization)

## ABI Encoding

For this benchmark, we are abi-encoding the [`swap` function call](https://github.com/Uniswap/v2-core/blob/ee547b17853e71ed4e0101ccfd52e70d5acded58/contracts/UniswapV2Pair.sol#L159) from Uniswap V2, both statically and dynamically.

```solidity
function swap(uint amount0Out, uint amount1Out, address to, bytes calldata data) external;
```

|               | `Ethers`                 | `Alloy`                           |
| :------------ | :----------------------- | :-------------------------------- |
| **`Static`**  | `1.12 us` (✅ **1.00x**) | `90.89 ns` (🚀 **12.32x faster**) |
| **`Dynamic`** | `2.20 us` (✅ **1.00x**) | `1.88 us` (✅ **1.17x faster**)   |

## U256 Operations

For this benchmark, we are computing the `amountIn` and `amountOut` for the [`swap` function call](https://github.com/Uniswap/v2-core/blob/ee547b17853e71ed4e0101ccfd52e70d5acded58/contracts/UniswapV2Pair.sol#L159) from the current reserves of the Uniswap V2 pair, demonstrating the use of `U256` operations.

|                 | `Ethers`                   | `Alloy`                           |
| :-------------- | :------------------------- | :-------------------------------- |
| **`amountIn`**  | `512.47 ns` (✅ **1.00x**) | `216.32 ns` (🚀 **2.37x faster**) |
| **`amountOut`** | `53.82 ns` (✅ **1.00x**)  | `18.19 ns` (🚀 **2.96x faster**)  |

## Rlp Encoding and Decoding

Rlp encoding and decoding comparison against [Parity tech rlp](https://crates.io/crates/rlp).
For this benchmark, we shall derive the `Encodable` and `Decodable` traits for a simple struct:

```rust
#[derive(alloy_rlp::RlpEncodable, alloy_rlp::RlpDecodable, rlp_derive::RlpDecodable,rlp_derive::RlpEncodable)]
pub struct MyStruct {
    pub a: u128,
    pub b: Vec<u8>,
}
```

|                | `Parity-Rlp`              | `Alloy-Rlp`                      |
| :------------- | :------------------------ | :------------------------------- |
| **`Encoding`** | `86.70 ns` (✅ **1.00x**) | `26.88 ns` (🚀 **3.23x faster**) |
| **`Decoding`** | `88.79 ns` (✅ **1.00x**) | `21.43 ns` (🚀 **4.14x faster**) |

## JSON-ABI

For this benchmark we shall compare ABI serialize and deserialize performance against [ethabi](https://crates.io/crates/ethabi) for widely used contracts such as [Seaport](https://etherscan.io/address/0x00000000000000adc04c56bf30ac9d3c0aaf14dc), [Uniswap V4 Pool Manager](https://etherscan.io/address/0x000000000004444c5dc75cB358380D2e3dE08A90) and [Uniswap V3 Pool](https://etherscan.io/address/0x99ac8cA7087fA4A2A1FB6357269965A2014ABc35).

### Serialization

|                     | `EthAbi`                  | `Alloy`                          |
| :------------------ | :------------------------ | :------------------------------- |
| **`Seaport`**       | `35.43 us` (✅ **1.00x**) | `38.68 us` (✅ **1.09x slower**) |
| **`PoolManager`**   | `18.33 us` (✅ **1.00x**) | `17.94 us` (✅ **1.02x faster**) |
| **`UniswapV3Pool`** | `14.61 us` (✅ **1.00x**) | `12.99 us` (✅ **1.12x faster**) |

### Deserialization

|                     | `EthAbi`                   | `Alloy`                           |
| :------------------ | :------------------------- | :-------------------------------- |
| **`Seaport`**       | `209.43 us` (✅ **1.00x**) | `210.67 us` (✅ **1.01x slower**) |
| **`PoolManager`**   | `89.05 us` (✅ **1.00x**)  | `93.31 us` (✅ **1.05x slower**)  |
| **`UniswapV3Pool`** | `63.24 us` (✅ **1.00x**)  | `68.50 us` (✅ **1.08x slower**)  |

## Serialize/Deserialize Function Signature

For this benchmark, we'll compare the serde performance of a large function signature.

|                   | `EthAbi`                  | `Alloy`                            |
| :---------------- | :------------------------ | :--------------------------------- |
| **`Serialize`**   | `5.03 us` (✅ **1.00x**)  | `247.82 ns` (🚀 **20.29x faster**) |
| **`Deserialize`** | `14.10 us` (✅ **1.00x**) | `14.05 us` (✅ **1.00x faster**)   |
