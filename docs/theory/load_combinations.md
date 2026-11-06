---
doc_id: OPS-THEORY-LOAD-COMBINATIONS
doc_kind: theory.note
status: draft
created: 2026-11-02
updated: 2026-11-02
deliverable_id: DEL-05-02
package_id: PKG-05
subject_deliverable: DEL-05-02
release_target_rows:
  - RT-A4
  - RT-V1
---

# Load Combinations

## Purpose and boundary

This note explains how SWBPIPE forms load combinations from solved load cases:
linear (`mechanics`) combinations, result-state subtractions, range envelopes,
absolute sums and SRSS, and combinations whose operands are other combinations. It
covers the inputs, the method, its assumptions and its limitations. It is written
from the implemented kernel (`core/loads/load_case_algebra`), the two product
routes (`core/product_physics`: `preview_physics.rs`, `exact_combinations.rs`,
`combination_plan.rs`) and their governing designs (DEL-05-02 Work
`absolute-sum-srss-combinations.md`, `exact-combinations.md` and
`pf-nonlinear-operand-standing.md`); where they differ, the design governs.

Every rule is a mechanics or arithmetic statement written for SWBPIPE. No design
code's combination rules, factors or acceptance limits are encoded or shipped. The
combinations, their factors and their purpose are the user's inputs. SWBPIPE forms
the combinations a user authors; it does not determine which combinations a code
requires and does not determine code compliance.

## Inputs

A combination has an id, a basis and the basis's operands:

- `mechanics`, `absolute_sum`, `srss`: `terms`, each naming exactly one of a load
  case (`load_case`) or a combination (`combination`), with a finite binary64
  factor. At least one term; no operand repeated.
- `result_state_subtraction`: `minuend_id` and `subtrahend_id`.
- `range_envelope`: `operand_ids` and a `mode` (`min`, `max`, `min_abs`,
  `max_abs`).

An id in `minuend_id`, `subtrahend_id` or `operand_ids` names the load case when one
has it, and a combination only otherwise, so a document whose operands are all load
cases keeps its meaning. The combinations' operand graph must be acyclic. A
subtraction declaring `difference_basis` names load cases only.

**Declared operating comparison (operating-case slice 1b).** On the pressure-free
route a `result_state_subtraction` may declare `difference_basis =
"operating_minus_sustained"` (minuend operating, subtrahend sustained). It is the
same subtraction, fl(O − S) row by row, gated by two warnings that withhold it:
G-1, `EXPANSION_BASIS_SUSTAINED_NOT_IN_OPERATING`, when the subtrahend case's
primitive loads, as a multiset by content (category, target, direction,
dimension, and the normalized magnitude's value bits and unit; ids ignored; read
after the solve's lowering, so condition-generated primitives count), are not
contained in the minuend case's; then G-2, `EXPANSION_BASIS_MODULUS_MIXED`, when
the two cases' modulus-basis keys differ. Each case keeps its own solved support
state; equal final states are not required. Its rows read "operating minus
sustained difference: a signed difference of two independently solved states,
… not an equilibrium state" and are never certified. The document states both
multisets (`difference_basis_derivation`). A reader with the model re-derives
G-1 from the model; a reader without it re-derives the verdict from those two
multisets, which checks the record's internal derivation, not that the record is
true to the model: a coherent forgery cannot be authenticated without the model
(DEL-05-02 Work `operating-case.md` §6). The
label "expansion" and any code-range meaning belong to the user's rule pack.

## Method

**Evaluation order.** Combinations are evaluated in the topological order of the
operand graph that takes, among the combinations whose operands are ready, the one
earliest in the list. A combination operand supplies its own published rows.

**Pairing.** A combined row is formed from one row of each operand for the same
quantity: on the pressure-free route by result base id, kind, unit, entity and
metadata (with a combination operand, every metadata field except the derivation's
`basis` and `sign_convention`; a section-basis suffix is still compared); on the
exact route by a typed structural key of kind, unit, entity, station, frame,
component and section basis. A missing operand row omits the combined row by name.

**Values.** For operand values xᵢ and factors fᵢ, in term order:

- linear: RN(Σ fᵢ·xᵢ), the exact sum of exact products rounded once to nearest,
  ties to even;
- subtraction: x_m − x_s (one binary64 subtraction); envelope: the mode's
  selection, ties to the smallest operand id;
- absolute sum: A = RN(Σ |fᵢ·xᵢ|), the exact sum rounded once;
- SRSS: zᵢ = RN(fᵢ·xᵢ), R = RN(√Σ zᵢ²), the exact sum of squares with its square
  root correctly rounded. zᵢ is the row the user would get by scaling the case
  by fᵢ; for factors ±1 or powers of two without under- or overflow zᵢ = fᵢ·xᵢ
  exactly.

A and R are +0.0 for an exact zero, never −0.0, and depend only on the multiset of
(fᵢ, xᵢ). A result outside the binary64 range is withheld by name, never an
infinity. Displacement and support-force and -moment magnitudes are the correctly
rounded norm of the combination's own combined components (an envelope selects
them instead). For an SRSS this equals the SRSS of the operands' magnitudes in
exact arithmetic; for an absolute sum each component |Σ sᵢfᵢvᵢ,c| ≤ Σ|fᵢvᵢ,c|, so
the magnitude bounds every signed combination's magnitude from above.

## Assumptions: the fixed-load count κ

A fixed load acts in every case state: a consuming constant-effort support's force,
or a connector's or tie rod's nonzero installed reference. Superposing states
counts it κ times, where κ is computed exactly from the operand tree: a load case
has κ = 1; `mechanics` κ = Σ fᵢκᵢ; a subtraction κ_m − κ_s; an envelope the
operands' common κ when all are exactly equal, otherwise undefined; an absolute
sum or SRSS κ = 0 when every operand's κ is exactly 0, otherwise undefined. An
undefined operand makes its dependants undefined. κ is held as an exact dyadic
rational with an odd significand of at most 4,096 bits; a result needing more is
undefined and named (`COMBINATION_FIXED_LOAD_COUNT_UNREPRESENTABLE`).

Where the model has a fixed-load source:

- a `mechanics` combination publishes only when |κ − 1| ≤ 2⁻⁴⁶, compared exactly
  (for a flat combination this is the unit factor-sum rule: 0.3 + 0.7 and
  fl(1/3) + fl(2/3) pass; 2, 1 + 2⁻⁴⁵ and 2⁵² − 2⁵² do not);
- an absolute sum or SRSS publishes only when every operand's κ is exactly 0. A
  tolerance would not do: with D = (1 + 2⁻⁴⁶)A − B (κ = 2⁻⁴⁶), an absolute sum
  of 2⁴⁶·D would carry the whole fixed load once and still pass.

The practical consequence: in such a model, the operands of a seismic SRSS or
absolute sum are differences, (base + EQx) − base, each with κ = 0.

## Assumptions: modulus basis, stiffness and nonlinear supports

A linear, absolute-sum or SRSS combination assumes one stiffness: every load case it
reaches through the operand graph, subtractions and envelopes included, must share
one modulus basis (and, on the exact route, one resolved member stiffness).

Absolute sums, SRSS, subtractions and envelopes are row operations and claim no
state, so they are formed in models with nonlinear supports, with each operand's
numerical standing recorded. A `mechanics` combination in such a model is withheld,
except decision OQ-1: one solved load case with factor exactly 1 plus at least one
other term, every other term naming a combination with κ exactly 0. Its rows are a
solved state plus derived differences or bounds, which is how occasional load cases
are commonly formed in practice; they are labelled as not an equilibrium state.

## Limitations

- Combination rows are derived inspection results: no numerical check of their
  own, never part of headlines, maxima or numerical eligibility.
- Unsigned rows have no sign or direction; no signed SRSS is formed.
- The retained routes and the CAEPIPE SI writer decline absolute sums, SRSS and
  combination operands by name.
- A declared operating comparison (`difference_basis`) is gated on the
  pressure-free route only (EX refuses it by name), its G-1 compares primitive
  loads only, and a `minuend_final` subtrahend is solved with the minuend's final
  support status held, a held-constraint comparison that can need a reaction a
  unilateral support cannot supply and is not an admissible sustained state. A reader without the model checks a declared subtraction's verdict
  against the published evidence only, which a coherent forgery can satisfy; with
  the model, G-1 over an operand that uses `condition_use` is refused as
  unverifiable.
- SWBPIPE does not decide which combinations or factors a code requires, and does
  not determine code compliance.

## Verification

`validation/references/rt_a4_combinations/` holds the kernel table (V-1) and the
three-axis static seismic cantilever (V-5, sub-directory `v5/`), each with an
independent standard-library generator. `core/loads/load_case_algebra` tests V-1
to V-4 and κ (V-6, V-9, V-10 at the kernel); `core/product_physics/tests/rt_a4_combinations.rs`
runs V-5 to V-7, V-9 and V-10 on both routes and both solver modes, each envelope
read by the product's own reader; the Rust readers re-derive A and R with the
kernel and the Python verifiers with an independent exact implementation.
