//! Example of creating, converting, and calculating with signed integers such as `I24` and `I256`.

use alloy::primitives::{aliases::I24, I256, U256};
use eyre::Result;

fn main() -> Result<()> {
    // Checked conversions reject values outside the signed type's range.
    let tick = I24::try_from(-120)?;
    assert_eq!(tick.to_string(), "-120");
    assert_eq!(i32::try_from(tick)?, -120);
    assert!(I24::try_from(8_388_608_i32).is_err());
    assert_eq!(I24::MIN.to_string(), "-8388608");
    assert_eq!(I24::MAX.to_string(), "8388607");

    // unchecked_from panics on an out-of-range conversion; use it only for known-valid values.
    assert_eq!(I24::unchecked_from(-120), tick);
    let negative = "-42".parse::<I256>()?;
    let positive = I256::try_from(100)?;
    assert!(negative.is_negative());
    assert!(negative < positive);
    assert_eq!((positive + negative).to_string(), "58");
    assert_eq!((negative * I256::try_from(2)?).to_string(), "-84");
    assert_eq!(I256::MAX.checked_add(I256::ONE), None);
    assert_eq!(I256::MIN.checked_neg(), None);

    // Numeric conversion differs from reinterpreting the underlying two's-complement bits.
    assert!(U256::try_from(negative).is_err());
    assert_eq!(I256::MINUS_ONE.into_raw(), U256::MAX);
    assert_eq!(I256::from_raw(U256::MAX), I256::MINUS_ONE);
    assert_eq!(I256::from_be_bytes(negative.to_be_bytes::<32>()), negative);

    Ok(())
}
