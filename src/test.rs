use crate::matrix::{DenseVectors, Entry, SparseMatrix};
use crate::r1cs::{R1cs, R1csWitness};
use crate::wire::Wire;

use zkstd::common::PrimeField;

fn array_to_witnessess<F: PrimeField>(witnesses: Vec<u64>) -> Vec<F> {
    witnesses
        .iter()
        .skip(1)
        .map(|witness| F::from(*witness))
        .collect::<Vec<_>>()
}

fn dense_to_sparse<F: PrimeField>(value: Vec<Vec<u64>>, l: usize) -> SparseMatrix<F> {
    let sparse_matrix = value
        .iter()
        .map(|elements| {
            elements
                .iter()
                .enumerate()
                .map(|(index, element)| {
                    if index == 0 {
                        Entry(Wire::One, F::from(*element))
                    } else if index <= l {
                        let index = index - 1;
                        Entry(Wire::instance(index), F::from(*element))
                    } else {
                        let index = index - 1 - l;
                        Entry(Wire::witness(index), F::from(*element))
                    }
                })
                .filter(|element| element.1 != F::zero())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    SparseMatrix(sparse_matrix)
}

fn example_z_witness<F: PrimeField>(input: u64, l: usize) -> R1csWitness<F> {
    let z = array_to_witnessess(vec![
        1,
        input,
        input * input * input + input + 5,
        input * input,
        input * input * input,
        input * input * input + input,
    ]);
    let (public_inputs, witness) = z.split_at(l);
    R1csWitness::new(
        DenseVectors(public_inputs.to_vec()),
        DenseVectors(witness.to_vec()),
    )
}

pub(crate) fn example_r1cs<F: PrimeField>(input: u64) -> R1cs<F> {
    let m = 4;
    let l = 1;
    let a = dense_to_sparse(
        vec![
            vec![0, 1, 0, 0, 0, 0],
            vec![0, 0, 0, 1, 0, 0],
            vec![0, 1, 0, 0, 1, 0],
            vec![5, 0, 0, 0, 0, 1],
        ],
        l,
    );
    let b = dense_to_sparse(
        vec![
            vec![0, 1, 0, 0, 0, 0],
            vec![0, 1, 0, 0, 0, 0],
            vec![1, 0, 0, 0, 0, 0],
            vec![1, 0, 0, 0, 0, 0],
        ],
        l,
    );
    let c = dense_to_sparse(
        vec![
            vec![0, 0, 0, 1, 0, 0],
            vec![0, 0, 0, 0, 1, 0],
            vec![0, 0, 0, 0, 0, 1],
            vec![0, 0, 1, 0, 0, 0],
        ],
        l,
    );
    let z = example_z_witness(input, l);
    R1cs { m, l, a, b, c, z }
}
