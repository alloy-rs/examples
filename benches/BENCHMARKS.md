# Benchmarks

## Table of Contents

- [Benchmark Results](#benchmark-results)
  - [dyn_encoding](#dyn_encoding)
  - [static_encoding](#static_encoding)
  - [get_amount_in](#get_amount_in)
  - [get_amount_out](#get_amount_out)

## Benchmark Results

### dyn_encoding

|     | `Ethers`                 | `Alloy`                         |
| :-- | :----------------------- | :------------------------------ |
|     | `2.17 us` (✅ **1.00x**) | `1.78 us` (✅ **1.22x faster**) |

### static_encoding

|     | `Ethers`                 | `Alloy`                           |
| :-- | :----------------------- | :-------------------------------- |
|     | `1.05 us` (✅ **1.00x**) | `92.43 ns` (🚀 **11.31x faster**) |

### get_amount_in

|     | `Ethers`                   | `Alloy`                           |
| :-- | :------------------------- | :-------------------------------- |
|     | `504.23 ns` (✅ **1.00x**) | `247.20 ns` (🚀 **2.04x faster**) |

### get_amount_out

|     | `Ethers`                  | `Alloy`                          |
| :-- | :------------------------ | :------------------------------- |
|     | `53.82 ns` (✅ **1.00x**) | `18.21 ns` (🚀 **2.96x faster**) |
