# Benchmarks

## Table of Contents

- [ABI Encoding](#abi-encoding)
  - [Dynamic ABI Encoding](#dynamic)
  - [Static ABIEncoding](#static)
- [U256 Operations](#u256-operations)
  - [UNIV2: Get Amount In](#univ2:-get-amount-in)
  - [UNIV2: Get Amount Out](#univ2:-get-amount-out)

## ABI Encoding

For this benchmark, we are abi-encoding the [`swap` function call](https://github.com/Uniswap/v2-core/blob/ee547b17853e71ed4e0101ccfd52e70d5acded58/contracts/UniswapV2Pair.sol#L159) from Uniswap V2, both statically and dynamically.

```solidity
function swap(uint amount0Out, uint amount1Out, address to, bytes calldata data) external;
```

### Dynamic

|     | `Ethers`                 | `Alloy`                         |
| :-- | :----------------------- | :------------------------------ |
|     | `2.12 us` (✅ **1.00x**) | `1.76 us` (✅ **1.20x faster**) |

### Static

|     | `Ethers`                   | `Alloy`                           |
| :-- | :------------------------- | :-------------------------------- |
|     | `999.83 ns` (✅ **1.00x**) | `90.87 ns` (🚀 **11.00x faster**) |

## U256 Operations

For this benchmark, we are computing the `amountIn` and `amountOut` for the [`swap` function call](https://github.com/Uniswap/v2-core/blob/ee547b17853e71ed4e0101ccfd52e70d5acded58/contracts/UniswapV2Pair.sol#L159) from the current reserves of the Uniswap V2 pair, demonstrating the use of `U256` operations.

### UNIV2: Get Amount In

|     | `Ethers`                   | `Alloy`                           |
| :-- | :------------------------- | :-------------------------------- |
|     | `503.52 ns` (✅ **1.00x**) | `245.98 ns` (🚀 **2.05x faster**) |

### UNIV2: Get Amount Out

|     | `Ethers`                  | `Alloy`                          |
| :-- | :------------------------ | :------------------------------- |
|     | `53.75 ns` (✅ **1.00x**) | `18.22 ns` (🚀 **2.95x faster**) |
