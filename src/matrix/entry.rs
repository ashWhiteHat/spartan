use crate::wire::Wire;

use zkstd::common::PrimeField;

#[derive(Clone, Debug)]
pub(crate) struct Entry<F: PrimeField>(pub(crate) Wire, pub(crate) F);

impl<F: PrimeField> Entry<F> {
    pub(crate) fn get(&self) -> (Wire, F) {
        (self.0, self.1)
    }
}

impl<F: PrimeField> From<Wire> for Entry<F> {
    fn from(value: Wire) -> Self {
        Self(value, F::one())
    }
}

impl<F: PrimeField> From<F> for Entry<F> {
    fn from(value: F) -> Self {
        Self(Wire::one(), value)
    }
}
