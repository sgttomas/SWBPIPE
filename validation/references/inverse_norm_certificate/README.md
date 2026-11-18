# Inverse-norm certificate: exact cases and the stated procedure

Small binary64 matrices whose inverse infinity norm is known exactly, for the
frame kernel's inverse-norm certificate arithmetic
(`core/solver/frame_kernel/src/structural/inverse_certificate.rs`).

The integrity check of DEL-04-01 takes Q = `mul_up(1024, est)` as an upper
bound on ‖A⁻¹‖∞ for its scaled binary64 matrix A without establishing it
(REQ-015's OUT-001 row in `execution/PKG-04/DEL-04-01/ScopeOfWork.md`; exit
criterion 18 of `docs/RELEASE_TARGET_v1.0.md`). The certificate states a
procedure that either establishes ‖A⁻¹‖∞ ≤ B ≤ Q or reports the matrix not
verified. This package tests that arithmetic. No production code calls the
certificate yet, so nothing here changes the check or establishes product
conformance, acceptance or release.

## Independence

`generators/inverse_norm_certificate_reference.py` uses only the Python
standard library. It reads, imports and runs no product code, product output,
fixture or schema. It builds each matrix itself, computes its truth in exact
fractions, and emulates the procedure below from its statement in Python's
binary64 arithmetic (each operation rounded to nearest, ties to even, gradual
underflow, no fused multiply-add). C5 is built from the construction stated in
`validation/references/inverse_norm_witness/README.md` (K0 = 2²⁰, λ = 2⁻²⁰,
w = (1, 1, −1, −1, 0, 0)/2, and its scaling rule e_i = −⌊⌊log₂ K_ii⌋/2⌋), not
from that package's generator or files; the generator checks that it gives the
exponents that README states and the C2–C4 family at λ = 2⁻²⁰.

```sh
python3 -I -B generators/inverse_norm_certificate_reference.py          # rebuild in memory and compare bytes
python3 -I -B generators/inverse_norm_certificate_reference.py --write  # rewrite cases.txt, directed.txt, SHA256SUMS
```

`SHA256SUMS` lists this README, the generator, `cases.txt` and `directed.txt`.
The generator writes it.

## Kinds of value

`cases.txt` keeps them on separately tagged lines.

| Tag | What it states | Its ground |
|---|---|---|
| `truth` | The exact rational ‖A⁻¹‖∞ of the case and of each block, and what truth alone requires: `truth requires not_verified` when the exact norm exceeds Q; otherwise `bound_at_least_norm` (any reported B is at least the exact norm). Every bound reported, verified or candidate, is at least the exact norm of what it bounds. | Exact rational inversion. Independent of the procedure. |
| `avail` | Availability expectations: the blocks, each attempt (E at σ₁, E at σ₂, D) with its outcome and every figure it forms, and the verdict with B, all as binary64 bits. | This generator's binary64 emulation of the procedure. They say what the stated procedure does on the matrix, not what is true of it: a case the procedure does not verify may still satisfy ‖A⁻¹‖∞ ≤ Q (C4). |
| `direction` | For each directed figure, the binary64 number next to the exact quantity it stands for, on the side the figure must lie (`ge`: figure ≥ edge, so figure ≥ exact; `le`: figure ≤ exact). The exact quantities are computed in fractions from the emulated factor: γ_m = mu/(1 − mu), N′, ν, 2u·max\|d̃\|, 2⁻¹⁰⁷⁴n(n + ν), the loss, σ − loss, k/σ′, U, t, 1 − t and U/(1 − t). | Exact arithmetic on the emulated factor. The generator refuses to write a figure that is on the wrong side. |
| `operand` | For γ, β, the loss, σ′, t and Uc, the binary64 number next to the exact result of the figure's own last operation on that operation's binary64 operands as the emulation formed them (for t, the exact product of U and the already formed error sum), on the side the figure must lie. A figure can lie on the right side of the ideal quantity only because its operands were already rounded outward and still be on the wrong side of the operation that formed it; this line catches that for the last operation. The earlier operations inside one figure (for example the product γ·N′ within the loss) are not tested separately. | Exact arithmetic on the emulation's own operands. The generator refuses to write a figure that is on the wrong side. |

`directed.txt` gives the directed operations against exact results,
including RN(1/3) < 1/3 and RN(h/2) = 0 < h/2
for h = 2⁻¹⁰⁷⁴, and the case [[2, h], [h, 2]]: the loop computes l₂₁ = RN(h/2)
= 0, so the exact residual at (2,1) is h ≠ 0 while the relative term
γ·|l₂₁|·d₁ is 0, and only the absolute allowance 2⁻¹⁰⁷⁴(n + ν) covers it.

## Procedure (as emulated)

For a symmetric n×n binary64 matrix A, an elimination order (`order[i]` the
free position eliminated at row i) with envelope `first` (in elimination
coordinates), and Q finite and positive. u = 2⁻⁵³; m = 2n + 2 ≤ 2⁵²;
γ = `div_up(mul_up(m, u), sub_dn(1, mul_up(m, u)))`. `add_up`, `mul_up` and
`div_up` round to nearest and step once up, `sub_dn` once down; a zero operand
is exact.

1. **Blocks**: connected components of the graph of nonzero off-diagonal
   stored values (an explicit zero of either sign is not an edge), numbered by
   least free position; k = ⌈√n_c⌉.
2. **Loop**: the profile LDLᵀ, row by row in elimination order: for j in
   first[i]..i, s = a_ij, s = RN(s − RN(w_k·l_jk)) for k in
   max(first[i], first[j])..j, w_j = s, l_ij = RN(w_j/d_j); then
   d_i = RN(a_ii − σ_c) and d_i = RN(d_i − RN(w_k·l_ik)) for k in first[i]..i.
   A non-finite result marks its block and stands as 0; a pivot not > 0 marks
   its block and is replaced by 1.
3. **Passes** on a block's rows (upward, zero multipliers skipped):
   a′_j = 1 + Σ_{i>j}|l_ij|, b′ = d·a′, N′ = max(b′_i + Σ_{j<i}|l_ij|b′_j);
   ν = max(d_i + Σ_{j<i}|l_ij|d_j); a_i = 1 + Σ_{j<i}|l_ij|a_j, c_i = a_i/d_i,
   then c_j += |l_ij|c_i for rows i descending, U = max c.
   β = `mul_up(mul_up(2⁻¹⁰⁷⁴, n), add_up(n, ν))`.
4. **Lemma E** at σ₁ = `div_up(mul_up(2, k), Q)`: with all pivots positive,
   δ = `mul_up(2u, max|d̃_i|)`, loss = `add_up(add_up(mul_up(γ, N′), δ), β)`,
   σ′ = `sub_dn(σ, loss)`, S = `div_up(k, σ′)`; verified when σ′ > 0 and S ≤ Q.
   Only a block with a non-positive pivot at σ₁ is refactored at
   σ₂ = `div_up(mul_up(1.125, k), Q)`.
5. **Lemma D** for blocks E did not verify, on the σ = 0 factor:
   t = `mul_up(U, add_up(mul_up(γ, N′), β))`; when t < 1,
   Uc = `div_up(U, sub_dn(1, t))`; verified when Uc ≤ Q.
6. B = max of the block bounds; verified only when every block is.

## Cases

Q is an input of the certificate. Each case from C1 to C7 states it as
`mul_up(1024, est)` for a stated est; for C2 to C5 the est values are 1, and
1 + 2⁻⁵² at λ = 2⁻¹¹. C8 states Q = 2⁶⁰ directly. They are inputs stated here, not results this package reproduces. The truth
does not depend on where Q came from.

| Case | Matrix | Exact ‖A⁻¹‖∞ | Q | Expected (avail) |
|---|---|---|---|---|
| C1 (success control) | 12-DOF chain, diagonal 4, neighbours −1 | 1065/2131 | mul_up(1024, 1/2) | verified, E at σ₁ |
| C2 | witness family, λ = 2⁻¹⁰ | 256 | mul_up(1024, 1) | verified; the coupled block by E at σ₂ only |
| C3 | witness family, λ = 2⁻¹¹ | 512 | mul_up(1024, 1 + 2⁻⁵²) | verified; the coupled block by D only |
| C4 | witness family, λ = 2⁻¹² | 1024 ≤ Q | mul_up(1024, 1) | not verified (D's bound exceeds Q), although the premise holds |
| C5 (underestimate control) | the witness's scaled free matrix | 262144 > Q | mul_up(1024, 1) | not verified (truth requires it) |
| C6 | blocks of 5 and 4 on interleaved free positions, order 8 1 6 3 4 5 2 7 0, full envelope | 4/5 | mul_up(1024, 1) | verified, both blocks by E at σ₁, with different shifts |
| C7 | two 2×2 blocks joined only by explicit stored zeros (+0.0 at (2,1), −0.0 at (3,0)) inside the envelope | 1 | mul_up(1024, 1) | verified, two blocks |
| C8 | the 1×1 matrix [3] | 1/3 | 2⁶⁰ | verified by D (E's shift 2⁻⁵⁹ is below its loss); the round-to-nearest quotient for Uc lies below the exact one |

The witness family is blockdiag(4(I − (1 − λ)wwᵀ), 1, 1), w = (1, 1, −1, −1)/2;
its exact norm is 1/(4λ). The emulated bounds, to the digits shown:
C2's S = 910.2222222281, C3's Uc = 512.0000000035, C4's Uc = 1024.0000000142,
C5's Uc = 262144.0009.

## Data format

`cases.txt`, one item per line; binary64 values are `f64::to_bits` in hex,
rationals `p/q`. Per case: `case`, `role` (`control_success`,
`control_underestimate` or `-`), `about`, `n`, `order`, `first`, `q`, then
`entry r c bits` for every stored entry of the lower triangle in free
positions (explicit zeros included), then the `truth`, `avail` and
`direction` lines, then `end`. `avail attempt <block> <e1|e2|d> <outcome>
<bound or candidate, or ->`; `avail figure <block> <attempt> <name> <bits>`;
`direction <block> <attempt> <name> <ge|le> <bits>`, and `operand` lines of
the same form; blocks are named by
their least free position.

## What is not established

Whether ordinary models are verified by this procedure is not established
here; these cases are constructed. The availability expectations depend on the
procedure's stated constants (schedule T's 2 and 9/8, m = 2n + 2) and would
change with them; the truth lines would not.
