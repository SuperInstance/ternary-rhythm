#![forbid(unsafe_code)]

/// A ternary value representing {-1, 0, +1} in time patterns.
///
/// Variants:
/// - `Negative` (-1): ghost note or weak accent
/// - `Neutral` (0): silence or rest
/// - `Positive` (+1): accented hit or strong onset
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Ternary {
    Negative = -1,
    Neutral = 0,
    Positive = 1,
}

impl From<Ternary> for i8 {
    fn from(v: Ternary) -> i8 {
        v as i8
    }
}

impl TryFrom<i8> for Ternary {
    type Error = &'static str;
    fn try_from(v: i8) -> Result<Self, Self::Error> {
        match v {
            -1 => Ok(Ternary::Negative),
            0 => Ok(Ternary::Neutral),
            1 => Ok(Ternary::Positive),
            _ => Err("invalid ternary value"),
        }
    }
}
