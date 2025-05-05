# Benchmarks

## Table of Contents

- [Benchmark Results](#benchmark-results)
  - [ABI Encoding](#abi-encoding)
  - [JSON-ABI Serialization](#json-abi-serialization)
  - [JSON-ABI Deserialization](#json-abi-deserialization)
  - [Serde Function Signature](#serde-function-signature)
  - [Rlp Encoding and Decoding](#rlp-encoding-and-decoding)
  - [U256 Operations](#u256-operations)

## Benchmark Results

### ABI Encoding

|               | `Ethers`                 | `Alloy`                           |
| :------------ | :----------------------- | :-------------------------------- |
| **`Static`**  | `1.12 us` (✅ **1.00x**) | `90.89 ns` (🚀 **12.32x faster**) |
| **`Dynamic`** | `2.20 us` (✅ **1.00x**) | `1.88 us` (✅ **1.17x faster**)   |

### JSON-ABI Serialization

|                     | `EthAbi`                  | `Alloy`                          |
| :------------------ | :------------------------ | :------------------------------- |
| **`Seaport`**       | `35.43 us` (✅ **1.00x**) | `38.68 us` (✅ **1.09x slower**) |
| **`PoolManager`**   | `18.33 us` (✅ **1.00x**) | `17.94 us` (✅ **1.02x faster**) |
| **`UniswapV3Pool`** | `14.61 us` (✅ **1.00x**) | `12.99 us` (✅ **1.12x faster**) |

### JSON-ABI Deserialization

|                     | `EthAbi`                   | `Alloy`                           |
| :------------------ | :------------------------- | :-------------------------------- |
| **`Seaport`**       | `209.43 us` (✅ **1.00x**) | `210.67 us` (✅ **1.01x slower**) |
| **`PoolManager`**   | `89.05 us` (✅ **1.00x**)  | `93.31 us` (✅ **1.05x slower**)  |
| **`UniswapV3Pool`** | `63.24 us` (✅ **1.00x**)  | `68.50 us` (✅ **1.08x slower**)  |

### Serde Function Signature

|                   | `EthAbi`                  | `Alloy`                            |
| :---------------- | :------------------------ | :--------------------------------- |
| **`Serialize`**   | `5.03 us` (✅ **1.00x**)  | `247.82 ns` (🚀 **20.29x faster**) |
| **`Deserialize`** | `14.10 us` (✅ **1.00x**) | `14.05 us` (✅ **1.00x faster**)   |

### Rlp Encoding and Decoding

|                | `Parity-Rlp`              | `Alloy-Rlp`                      |
| :------------- | :------------------------ | :------------------------------- |
| **`Encoding`** | `86.70 ns` (✅ **1.00x**) | `26.88 ns` (🚀 **3.23x faster**) |
| **`Decoding`** | `88.79 ns` (✅ **1.00x**) | `21.43 ns` (🚀 **4.14x faster**) |

### U256 Operations

|                 | `Ethers`                   | `Alloy`                           |
| :-------------- | :------------------------- | :-------------------------------- |
| **`amountIn`**  | `512.47 ns` (✅ **1.00x**) | `216.32 ns` (🚀 **2.37x faster**) |
| **`amountOut`** | `53.82 ns` (✅ **1.00x**)  | `18.19 ns` (🚀 **2.96x faster**)  |

---

Made with [criterion-table](https://github.com/nu11ptr/criterion-table)
