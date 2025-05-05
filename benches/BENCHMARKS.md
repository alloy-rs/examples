# Benchmarks

## Table of Contents

- [Benchmark Results](#benchmark-results)
    - [Dynamic](#dynamic)
    - [JSON-ABI Serialization](#json-abi-serialization)
    - [JSON-ABI Deserialization](#json-abi-deserialization)
    - [Serialize Function Sig](#serialize-function-sig)
    - [Deserialize Function](#deserialize-function)
    - [Rlp Encoding and Decoding](#rlp-encoding-and-decoding)
    - [Static](#static)
    - [UNIV2-Get Amount In](#univ2-get-amount-in)
    - [UNIV2-Get Amount Out](#univ2-get-amount-out)

## Benchmark Results

### Dynamic

|        | `Ethers`                | `Alloy`                         |
|:-------|:------------------------|:------------------------------- |
|        | `2.29 us` (✅ **1.00x**) | `1.77 us` (✅ **1.29x faster**)  |

### JSON-ABI Serialization

|                     | `EthAbi`                 | `Alloy`                          |
|:--------------------|:-------------------------|:-------------------------------- |
| **`Seaport`**       | `41.35 us` (✅ **1.00x**) | `41.32 us` (✅ **1.00x faster**)  |
| **`PoolManager`**   | `17.98 us` (✅ **1.00x**) | `17.45 us` (✅ **1.03x faster**)  |
| **`UniswapV3Pool`** | `14.64 us` (✅ **1.00x**) | `12.98 us` (✅ **1.13x faster**)  |

### JSON-ABI Deserialization

|                     | `EthAbi`                  | `Alloy`                           |
|:--------------------|:--------------------------|:--------------------------------- |
| **`Seaport`**       | `220.47 us` (✅ **1.00x**) | `211.97 us` (✅ **1.04x faster**)  |
| **`PoolManager`**   | `90.51 us` (✅ **1.00x**)  | `94.59 us` (✅ **1.04x slower**)   |
| **`UniswapV3Pool`** | `70.76 us` (✅ **1.00x**)  | `70.14 us` (✅ **1.01x faster**)   |

### Serialize Function Sig

|        | `EthAbi`                | `Alloy`                            |
|:-------|:------------------------|:---------------------------------- |
|        | `5.10 us` (✅ **1.00x**) | `298.37 ns` (🚀 **17.10x faster**)  |

### Deserialize Function

|        | `EthAbi`                 | `Alloy`                          |
|:-------|:-------------------------|:-------------------------------- |
|        | `13.91 us` (✅ **1.00x**) | `14.27 us` (✅ **1.03x slower**)  |

### Rlp Encoding and Decoding

|                | `Parity-Rlp`             | `Alloy-Rlp`                      |
|:---------------|:-------------------------|:-------------------------------- |
| **`Encoding`** | `85.85 ns` (✅ **1.00x**) | `24.74 ns` (🚀 **3.47x faster**)  |
| **`Decoding`** | `89.95 ns` (✅ **1.00x**) | `22.07 ns` (🚀 **4.08x faster**)  |

### Static

|        | `Ethers`                | `Alloy`                           |
|:-------|:------------------------|:--------------------------------- |
|        | `1.04 us` (✅ **1.00x**) | `94.68 ns` (🚀 **11.00x faster**)  |

### UNIV2-Get Amount In

|        | `Ethers`                  | `Alloy`                           |
|:-------|:--------------------------|:--------------------------------- |
|        | `512.03 ns` (✅ **1.00x**) | `222.05 ns` (🚀 **2.31x faster**)  |

### UNIV2-Get Amount Out

|        | `Ethers`                 | `Alloy`                          |
|:-------|:-------------------------|:-------------------------------- |
|        | `56.84 ns` (✅ **1.00x**) | `18.30 ns` (🚀 **3.11x faster**)  |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

