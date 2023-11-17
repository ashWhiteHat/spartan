# Spartan

[Spartan](https://eprint.iacr.org/2019/550.pdf) implementation

## R1cs

$
A,B,C \in \mathbb F^{m \times n}$ and $Z = (w,1,x) \in \mathbb F^n
$

- $x \in \mathbb F^l$
- $w \in \mathbb F^{n - l - 1}$

## Commitment Scheme

For $X = \mathbb F^{n \times m}$ and $M_{ij} \in X$

### $Setup(1^λ) \rightarrow pp$

**pp**

- $Γ_1 \leftarrow \mathbb G_1^m$
- $H_1 \leftarrow \mathbb G_1$
- $Γ_2 \leftarrow \mathbb G_2^n$
- $H_2 \leftarrow \mathbb G_2$

### $Commit(pp;M_{ij}) \rightarrow (C,S)$

$(C,S) := (C,(r_{rows},r_{fin},V))$ for

- $r_{rows} \leftarrow \mathbb F^n$
- $r_{fin} \leftarrow \mathbb F$
- $H_T \leftarrow e(H_1,H_2)$
- $V_i \leftarrow Commit_{Pedersen}((Γ_1,H_1);M_{ij},r_{rows,i})$
- $C \leftarrow Commit_{AFGHO}((Γ_2,H_T);V,r_{fin})$
