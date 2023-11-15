use crate::matrix::{DenseVectors, SparseMatrix};

use zkstd::common::PrimeField;

#[derive(Clone, Debug, Default)]
pub struct R1csWitness<F: PrimeField> {
    // 1. Instance
    pub(crate) x: DenseVectors<F>,

    // 2. Constant
    pub(crate) one: F,

    // 3. Witness
    pub(crate) w: DenseVectors<F>,
}

impl<F: PrimeField> R1csWitness<F> {
    pub(crate) fn new(x: DenseVectors<F>, w: DenseVectors<F>) -> Self {
        Self {
            x,
            one: F::one(),
            w,
        }
    }

    pub(crate) fn z(&self, m: usize) -> DenseVectors<F> {
        let l = self.x.0.len();
        let m_l_1 = self.w.0.len();
        assert_eq!(m, l + m_l_1 + 1);

        DenseVectors(
            self.x
                .0
                .iter()
                .chain(vec![F::one()].iter())
                .chain(self.w.0.iter())
                .map(|e| *e)
                .collect::<Vec<F>>(),
        )
    }
}
