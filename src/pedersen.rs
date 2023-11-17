use zkstd::common::{CurveAffine, Group, RngCore};

pub(crate) struct Pedersen<G: CurveAffine> {
    g: G,
    h: G,
}

impl<G: CurveAffine> Pedersen<G> {
    pub(crate) fn setup(r: impl RngCore) -> Self {
        let g = G::ADDITIVE_GENERATOR;
        let h = G::from(g * G::Scalar::random(r));
        Self { g, h }
    }

    pub(crate) fn commit(&self, x: G::Scalar, r: impl RngCore) -> G {
        let r = G::Scalar::random(r);
        G::from(self.g * x + self.h * r)
    }
}
