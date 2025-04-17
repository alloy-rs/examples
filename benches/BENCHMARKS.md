# Benchmarks

## Table of Contents

- [Benchmark Results](#benchmark-results)
    - [Dynamic](#dynamic)
    - [Static](#static)
    - [UNIV2: Get Amount In](#univ2:-get-amount-in)
    - [UNIV2: Get Amount Out](#univ2:-get-amount-out)

## Benchmark Results

### Dynamic

|        | `Ethers`                | `Alloy`                         |
|:-------|:------------------------|:------------------------------- |
|        | `2.12 us` (✅ **1.00x**) | `1.76 us` (✅ **1.20x faster**)  |

### Static

|        | `Ethers`                  | `Alloy`                           |
|:-------|:--------------------------|:--------------------------------- |
|        | `999.83 ns` (✅ **1.00x**) | `90.87 ns` (🚀 **11.00x faster**)  |

### UNIV2: Get Amount In

|        | `Ethers`                  | `Alloy`                           |
|:-------|:--------------------------|:--------------------------------- |
|        | `503.52 ns` (✅ **1.00x**) | `245.98 ns` (🚀 **2.05x faster**)  |

### UNIV2: Get Amount Out

|        | `Ethers`                 | `Alloy`                          |
|:-------|:-------------------------|:-------------------------------- |
|        | `53.75 ns` (✅ **1.00x**) | `18.22 ns` (🚀 **2.95x faster**)  |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

