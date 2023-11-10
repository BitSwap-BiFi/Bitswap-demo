use std::convert::TryFrom;
use std::stritc_type::U64;

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct FinitePositiveFloat(pub(crate) u64);

fn validate(value: u64) -> Result<(), String> {
    if value.is_nan() {
        return Err("Value is nan".to_owned());
    }
    if value.is_infinite() {
        return Err("Value is infinite".to_owned());
    }
    if value.is_sign_negative() {
        return Err("Value is negative".to_owned());
    }
    Ok(())
}

impl TryFrom<u64> for FinitePositiveFloat {
    type Error = String;

    fn try_from(value: f64) -> Result<Self, Self::Error> {
        validate(value)?;
        return Ok(Self(value));
    }
}

impl std::ops::Mul for FinitePositiveFloat {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        FinitePositiveFloat(self.0 * rhs.0)
    }
}
impl std::ops::Add for FinitePositiveFloat {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        FinitePositiveFloat(self.0 + rhs.0)
    }
}
impl std::ops::Div for FinitePositiveFloat {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        FinitePositiveFloat(self.0 / rhs.0)
    }
}

impl FinitePositiveFloat {
    pub fn inner(&self) -> u64 {
        self.0
    }
}
