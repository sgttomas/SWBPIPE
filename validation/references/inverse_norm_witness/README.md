# Inverse-norm premise: known-limitation witness

Invented models on which the integrity check's inverse-norm premise fails, and
their truth in exact rational arithmetic.

The check of DEL-04-01 (REQ-015, REQ-016, AC-013) takes 1024 times a
Hager–Higham estimate of the scaled system's inverse norm as an upper bound on
that norm. The OUT-001 row of REQ-015 in
`execution/PKG-04/DEL-04-01/ScopeOfWork.md` records that this premise is
unproved and has failed. These models show the failure. Exit criterion 18 of
`docs/RELEASE_TARGET_v1.0.md` is the governing text.

**Status.** This is a known-limitation witness and not a requirement. Its
consumer, `core/product_physics/tests/inverse_norm_premise_known_limitation_witness.rs`,
asserts today's wrong behaviour. A change in that outcome needs the repair's
own independent evidence, and the OUT-001 row reconciled in the same change.
Nothing here establishes product conformance, acceptance or release.

## Independence

`generators/inverse_norm_witness_reference.py` uses only the Python standard
library. It reads, imports and runs no product code, product output, fixture or
schema. It writes the request documents itself and derives every value from
them in exact fractions. Its assumptions about how the product forms the system
are listed in its `ASSUMPTIONS` table. Each one names the source it was checked
against, and the generator checks the assumptions that can be checked by
arithmetic: exactness in binary64, the free block, rigid-body motion and the
scaling.

```sh
python3 -I -B generators/inverse_norm_witness_reference.py          # rebuild in memory and compare bytes
python3 -I -B generators/inverse_norm_witness_reference.py --write  # rewrite requests, references.json, SHA256SUMS
```

`SHA256SUMS` lists this README, the generator, the three requests and
`references.json`. The generator writes it.

## Model

One expansion joint (an objective connector) replaces the span A–B. Node A
(0, 0, 0) is anchored. Node B (1, 0, 0) m is free. The connector's axes are
global, its offsets are zero, its translation scale is 1 m and its reference
state is stress-free. Its user-entered 6×6 work matrix H is coupled. The route
is exact (`exact_pressure_v3`, Euler–Bernoulli), and there is no pressure and
no self-weight. The pressure-free route refuses a connector, so it is not used.

| Number | Value | Why |
|---|---|---|
| Free stiffness K_free | K0·(I − (1 − λ)·w wᵀ) | Eigenvalue K0·λ along w and K0 elsewhere. H is chosen to give it (H = B_j⁻ᵀ K_free B_j⁻¹). |
| K0 | 2²⁰ N/m (the same number in N/rad and N·m/rad) | Every scaled diagonal is a power of two times a short mantissa, so the equilibration is known exactly: exponents −9 on UX–RX and −10 on RY and RZ. |
| λ | 2⁻²⁰ | The scaled inverse norm is exactly 2¹⁸ = 262144. The condition number (2²⁰) stays far inside the check's contraction test. |
| w | (1, 1, −1, −1, 0, 0)/2 on (UX, UY, UZ, RX, RY, RZ) | Orthogonal to the vector of ones and to the alternating vector (−1)ⁱ(1 + i/5). It is zero on RZ, the index the estimator's one unit-vector step takes. Every start vector the estimator uses misses w, and its exact replay gives 1. |
| Chord | 1 m along x | Every entry of B, K and Ke is a short dyadic rational, exact in binary64, so the formed system is the intended system. |
| X_MAIN | (3, 1, 0, −5, 2, 1)/4096 m, rad | The displacement the six nodal loads produce. Its UZ is exactly 0, so the stop sits at the main loads' exact displacement. Both factorizations solve this displacement inexactly, with errors of opposite sign at UZ. |
| One-way stop | B's UZ, ground 0, initial state inactive | The DOF lies on w, so the check's uncorrected error reaches the stop. |
| Tiny term | +2⁻¹¹² N (negative-reaction stop), −2⁻¹²¹ N (positive-reaction stop) | It alone decides the stop's exact state, making it active. Its response at the stop, (2²⁰ + 3)/2²² times the term, is smaller than the error the twice-corrected solution keeps along w, which is about (κu)³ times the solution. It is larger than the radius the check claims. The magnitude was located by observing that error in each mode. The truth does not depend on that observation. |

1024 × estimate = 1024 against the true 262144, a ratio of exactly 1/256.

## Variants and truth (`references.json`)

| Variant | Mode where it is a witness | Exact truth |
|---|---|---|
| `linear` | both | B's displacement is X_MAIN, so UZ = 0 |
| `one_way_negative` | dense | Exactly one admissible state, active. Inactive is inadmissible: UZ = +(2²⁰ + 3)/2¹³⁴ m |
| `one_way_positive` | sparse | Exactly one admissible state, active. Inactive is inadmissible: UZ = −(2²⁰ + 3)/2¹⁴³ m |

In the mode where a one-way request is not a witness, the product reached the
active state when it was observed, which is correct.

## Magnitudes

Each statement is marked by its basis. *Computed exactly* means computed by
this generator. *Observed* means seen in a product run on main 946553b52.
*Reasoned* means neither.

**The wrong states.**

- The deciding displacements at which a wrong state was published are of order
  10⁻³⁴ to 10⁻³⁶ m. The exact inactive-state stop displacements are
  +4.8·10⁻³⁵ m (dense) and −9.4·10⁻³⁸ m (sparse); this is computed exactly. The
  product decided on +1.3·10⁻³⁶ m (sparse) and −8.3·10⁻³⁴ m (dense); this was
  observed.
- The tiny load terms that make those states wrong are 36 to 39 decimal orders
  below the main loads, which are about 190 to 700 N. This is computed exactly.
  - Dense: 1.9·10⁻³⁴ N, which is 2.7·10⁻³⁷ to 1.0·10⁻³⁶ of each main load.
  - Sparse: 3.8·10⁻³⁷ N, which is 5.3·10⁻⁴⁰ to 2.0·10⁻³⁹ of each main load.
- No witness without such a term was built. A wrong state needs a deciding
  quantity below the twice-corrected solution's accuracy, which is about (κu)³
  relative. This is reasoned.

**The understated bound.**

- The understatement of the inverse-norm bound does not depend on the tiny
  term.
  - It is a property of the matrix. 1024 × the estimate is 1/256 of the true
    scaled inverse norm in every variant, including `linear`, which has no
    tiny term. This is computed exactly, and the estimate was observed to be 1
    in every run.
- For the linear variant, the check's internal displacement-row radii are
  exceeded by the actual error by 137 times (dense) and 256 times (sparse) on
  UX, UY, UZ and RX. This was observed in a run with a print-only probe, and the
  error was computed exactly from the corrections it printed.
- The error is within 256 times the claimed radius, as the factor allows. This
  is computed exactly from the same corrections.
- The anchor's reaction bounds are not understated on this model. Their actual
  error is about 10⁻⁴ to 10⁻⁵ of the claimed remainder. This is computed
  exactly from the same corrections.
  - The reason: a reaction through the connector reads K times the error, which
    the weak direction does not amplify. This is reasoned.
- This model has no member, because the span is replaced. Its result documents
  print no bound. This was observed.
- In general, a printed bound on a quantity cᵀu can be understated by up to the
  factor (256 here) only in so far as c has a component along the weak
  direction. This is reasoned.

## What is not established

How far the failure reaches into ordinary models. The smallness of the tiny
term is a property of this witness. A wrong one-way state needs a deciding
quantity below the corrected solution's accuracy, which is about (κu)³ relative.
How that smallness is produced (a tiny load term, or cancellation) is a choice
of construction.
