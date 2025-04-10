//! Example of creating instances of `U256` from strings and numbers.

use std::str::FromStr;

use alloy::primitives::{
    utils::{parse_units, ParseUnits},
    U256, I208, I216, I224, I232, I240, I248, I256, I512,
};
use eyre::Result;

fn main() -> Result<()> {
    // From strings
    let a = U256::from_str("42")?;
    assert_eq!(a.to_string(), "42");

    let amount = "42";
    let units = 4;
    let b: ParseUnits = parse_units(amount, units)?;
    assert_eq!(b.to_string(), "420000");

    // From numbers
    let c = U256::from(42_u8);
    assert_eq!(c.to_string(), "42");

    let d = U256::from(42_u16);
    assert_eq!(d.to_string(), "42");

    let e = U256::from(42_u32);
    assert_eq!(e.to_string(), "42");

    let f = U256::from(42_u64);
    assert_eq!(f.to_string(), "42");

    let g = U256::from(42_u128);
    assert_eq!(g.to_string(), "42");

    let h = U256::from(0x2a);
    assert_eq!(h.to_string(), "42");

    let i = U256::from(42);
    assert_eq!(i.to_string(), "42");

    let e = I256::unchecked_from(99999_i128);
    assert_eq!(e.to_string(), "99999");

    let f = I256::unchecked_from(-99999_i128);
    assert_eq!(f.to_string(), "-99999");

    let a = I208::unchecked_from(42_i128);
    assert_eq!(a.to_string(), "42");

    let b = I208::unchecked_from(-42_i128);
    assert_eq!(b.to_string(), "-42");

    let c = I216::unchecked_from(123456789_i128);
    assert_eq!(c.to_string(), "123456789");

    let d = I216::unchecked_from(-123456789_i128);
    assert_eq!(d.to_string(), "-123456789");

    let e = I224::unchecked_from(987654321_i128);
    assert_eq!(e.to_string(), "987654321");

    let f = I224::unchecked_from(-987654321_i128);
    assert_eq!(f.to_string(), "-987654321");

    let g = I232::unchecked_from(111111111111_i128);
    assert_eq!(g.to_string(), "111111111111");

    let h = I232::unchecked_from(-111111111111_i128);
    assert_eq!(h.to_string(), "-111111111111");

    let i = I240::unchecked_from(99999999999999_i128);
    assert_eq!(i.to_string(), "99999999999999");

    let j = I240::unchecked_from(-99999999999999_i128);
    assert_eq!(j.to_string(), "-99999999999999");

    let k = I248::unchecked_from(1234567890123456789_i128);
    assert_eq!(k.to_string(), "1234567890123456789");

    let l = I248::unchecked_from(-1234567890123456789_i128);
    assert_eq!(l.to_string(), "-1234567890123456789");

    let m = I256::unchecked_from(99999_i128);
    assert_eq!(m.to_string(), "99999");

    let n = I256::unchecked_from(-99999_i128);
    assert_eq!(n.to_string(), "-99999");

    let o = I512::unchecked_from(314159265358979_i128);
    assert_eq!(o.to_string(), "314159265358979");

    let p = I512::unchecked_from(-314159265358979_i128);
    assert_eq!(p.to_string(), "-314159265358979");

    Ok(())
}
