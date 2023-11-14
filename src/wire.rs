#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Wire {
    /// wire for constant one, public input and output
    Instance(usize),
    /// wire for private input and intermediate value
    Witness(usize),
    /// wire for first element one
    One,
}

impl Wire {
    #[cfg(test)]
    pub(crate) fn instance(index: usize) -> Self {
        Self::Instance(index)
    }

    #[cfg(test)]
    pub(crate) fn witness(index: usize) -> Self {
        Self::Witness(index)
    }

    pub(crate) fn one() -> Self {
        Self::One
    }
}
