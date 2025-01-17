//! Example showing how complex return values such as tuple, structs, etc. are returned from a call
//! to a contract using the `sol!` macro.

use alloy::{
    hex,
    primitives::{Uint, I256, U256},
    sol,
    sol_types::{SolCall, SolValue},
};
use eyre::Result;

// Complex return demonstrating the new API that directly yields the values, enabling rust pattern
// matching.
// Note: The names of return are now ignored.
sol! {
    function getNamedTuple() external view returns (uint256 a, uint256 b, uint256 c);
    function getUnamedTuple() external view returns (uint256, uint256, uint256);
    function getPartialNamedTuple() external view returns (uint256 , uint256 b, uint256 );

    struct MyStruct {
        uint256 a;
        uint256 b;
        uint256 c;
    }

    function getStructWithBytes() external view returns (MyStruct, bytes32);
    function getCompoundTupleStruct() external view returns ((MyStruct , bytes32), (MyStruct, bytes32));
}

fn main() -> Result<()> {
    let data = vec![1, 2, 3].abi_encode_sequence();
    // Names are ignored.
    let (a, b, c) = getNamedTupleCall::abi_decode_returns(&data, true)?;

    assert_eq!(a, U256::from(1));
    assert_eq!(b, U256::from(2));
    assert_eq!(c, U256::from(3));

    let (a, b, c) = getUnamedTupleCall::abi_decode_returns(&data, true)?;

    assert_eq!(a, U256::from(1));
    assert_eq!(b, U256::from(2));
    assert_eq!(c, U256::from(3));

    // Names are ignored.
    let (a, b, c) = getPartialNamedTupleCall::abi_decode_returns(&data, true)?;

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
    let (MyStruct { a, b, c }, bytes) = getStructWithBytesCall::abi_decode_returns(&data, true)?;

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

    let ((MyStruct { a, b, c }, bytes), (MyStruct { a: a2, b: b2, c: c2 }, bytes2)) =
        getCompoundTupleStructCall::abi_decode_returns(&data, true)?;

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
