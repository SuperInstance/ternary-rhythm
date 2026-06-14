# Generate clean merged lib.rs for ternary-rhythm
# Uses local Ternary enum with Positive/Neutral/Negative naming

lib = []
def L(s): lib.append(s)

L('#![forbid(unsafe_code)]')
L('')
L('//! Temporal pattern recognition and generation using ternary {-1, 0, +1} time patterns.')
L('')
L('mod ternary;')
L('')
L('pub type RhythmPattern = Vec<Ternary>;')
L('')
L('use crate::ternary::Ternary;')
L('pub use crate::ternary::Ternary as TernaryVal;')
L('use Ternary::{Negative, Neutral, Positive};')
L('')
L('#[cfg(feature = "simd")]')
L('mod attractor;')
L('')
L('/// Extension trait for Ternary.')
L('pub trait TernaryExt {')
L('    fn from_i8(v: i8) -> Option<Self> where Self: Sized;')
L('    fn to_i8(self) -> i8;')
L('    fn random(seed: &mut u64) -> Self where Self: Sized;')
L('}')
L('')
L('impl TernaryExt for Ternary {')
L('    fn from_i8(v: i8) -> Option<Self> {')
L('        match v { -1 => Some(Negative), 0 => Some(Neutral), 1 => Some(Positive), _ => None }')
L('    }')
L('    fn to_i8(self) -> i8 { self.into() }')
L('    fn random(seed: &mut u64) -> Self {')
L('        *seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1);')
L('        match (*seed % 3) as i8 { 0 => Negative, 1 => Neutral, _ => Positive }')
L('    }')
L('}')

# Save the base
with open('/tmp/lib-base.py', 'w') as f:
    f.write('\n'.join(lib))
print(f"Wrote {len(lib)} lines base")
