//! Example showing how to deconstruct complex return values such as tuple, structs, etc. are
//! returned from a call to a contract using the `sol!` macro.

use alloy::{
    hex,
    primitives::{Uint, I256, U256},
    sol,
    sol_types::{SolCall, SolValue},
};
use eyre::Result;

sol! {
    function getNamedTuple() external view returns (uint256 a, uint256 b, uint256 c);
    function getUnamedTuple() external view returns (uint256, uint256, uint256);
    function getPartialNamedTuple() external view returns (uint256 , uint256 b, uint256 );

    struct MyStruct {
        uint256 a;
        uint256 b;
        uint256 c;
    }

    function getStructWithBytes() external view returns (MyStruct my_struct, bytes32);
    function getCompoundTupleStruct() external view returns ((MyStruct , bytes32), (MyStruct, bytes32));
}

fn main() -> Result<()> {
    let data = vec![1, 2, 3].abi_encode_sequence();

    // Return param names are retained as field names in the struct.
    let getNamedTupleReturn { a, b, c } = getNamedTupleCall::abi_decode_returns(&data, true)?;

    assert_eq!(a, U256::from(1));
    assert_eq!(b, U256::from(2));
    assert_eq!(c, U256::from(3));

    // Struct fields are named `_{index}` in case a return param is left unnamed.
    let getUnamedTupleReturn { _0: a, _1: b, _2: c } =
        getUnamedTupleCall::abi_decode_returns(&data, true)?;

    assert_eq!(a, U256::from(1));
    assert_eq!(b, U256::from(2));
    assert_eq!(c, U256::from(3));

    // Indicates a case where only one of the return param  is named and the rest are unnamed.
    let getPartialNamedTupleReturn { _0: a, b, _2: c } =
        getPartialNamedTupleCall::abi_decode_returns(&data, true)?;

    assert_eq!(a, U256::from(1));
    assert_eq!(b, U256::from(2));
    assert_eq!(c, U256::from(3));

    let data = hex!(
        // MyStruct.a (uint256)
        "0000000000000000000000000000000000000000000000000000000000000001"
        // MyStruct.b (uint256)
        "0000000000000000000000000000000000000000000000000000000000000002"
        // MyStruct.c (uint256)
        "0000000000000000000000000000000000000000000000000000000000000003"
        // bytes32
        "0102030400000000000000000000000000000000000000000000000000000000"
    );

    // Deconstruct a struct and bytes32 return value.
    let getStructWithBytesReturn { my_struct: MyStruct { a, b, c }, _1 } =
        getStructWithBytesCall::abi_decode_returns(&data, true)?;

    assert_eq!(a, U256::from(1));
    assert_eq!(b, U256::from(2));
    assert_eq!(c, U256::from(3));
    assert_eq!(bytes, b256!("0102030400000000000000000000000000000000000000000000000000000000"));

    let data = hex!(
        // First tuple: (MyStruct, bytes32)
        // MyStruct.a (uint256)
        "0000000000000000000000000000000000000000000000000000000000000001"
        // MyStruct.b (uint256)
        "0000000000000000000000000000000000000000000000000000000000000002"
        // MyStruct.c (uint256)
        "0000000000000000000000000000000000000000000000000000000000000003"
        // First bytes32
        "0102030400000000000000000000000000000000000000000000000000000000"
        // Second tuple: (MyStruct, bytes32)
        // MyStruct.a (uint256)
        "0000000000000000000000000000000000000000000000000000000000000004"
        // MyStruct.b (uint256)
        "0000000000000000000000000000000000000000000000000000000000000005"
        // MyStruct.c (uint256)
        "0000000000000000000000000000000000000000000000000000000000000006"
        // Second bytes32
        "0506070800000000000000000000000000000000000000000000000000000000"
    );

    let getCompoundTupleStructReturn { _0, _1 } =
        getCompoundTupleStructCall::abi_decode_returns(&data, true)?;

    let (MyStruct { a, b, c }, bytes) = _0;
    let (MyStruct { a: a2, b: b2, c: c2 }, bytes2) = _1;

    assert_eq!(a, U256::from(1));
    assert_eq!(b, U256::from(2));
    assert_eq!(c, U256::from(3));
    assert_eq!(bytes, b256!("0102030400000000000000000000000000000000000000000000000000000000"));

    assert_eq!(a2, U256::from(4));
    assert_eq!(b2, U256::from(5));
    assert_eq!(c2, U256::from(6));
    assert_eq!(bytes2, b256!("0506070800000000000000000000000000000000000000000000000000000000"));

    Ok(())
}
