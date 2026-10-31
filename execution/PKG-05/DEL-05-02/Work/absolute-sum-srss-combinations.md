# Absolute-sum and SRSS combinations, and combinations of combinations

**Row:** RT-A4 (Must), serving exit criterion 1 ("three-axis static seismic
with SRSS and absolute-sum combination") and RT-V1. **Basis:** main
`8cab1b3e5` (2026-10-30). **Status:** revision 1, design for review; the
algebra kernel (§3) is the first build slice. Risk class: numerical meaning
and persistence, so every slice takes targeted regression checks and
independent scrutiny (AGENTS.md, Work and verification).

**Decisions applied** (SWBPIPE Agent 0, 2026-10-30, on the load-case plan):
`absolute_sum` and `srss` join model document 0.9.0 before S1; a combination
may take earlier combinations as inputs, acyclic; a constant-effort gate like
the unit factor-sum rule is approved in principle, its exact form settled here
(§5) and reviewed. Each new member is refused by name until its slice lands.

Paths are relative to the repository root. PP is `core/product_physics/src`,
LCA is `core/loads/load_case_algebra/src`, FK is `core/solver/frame_kernel/src`.
Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081). Nothing
here records or implies acceptance, issuance or release.

## 1. What exists

- **Bases.** `combinations[].basis` is one of `mechanics` (terms with factors
  over load cases), `result_state_subtraction` (minuend minus subtrahend) and
  `range_envelope` (min, max, min_abs or max_abs over operand ids). Operands are
  load-case ids only (`schemas/model.schema.yaml` `Combination`,
  `CombinationTerm`; PP `lib.rs` `CombinationExpression`).
- **Arithmetic.** LCA `evaluate_linear_combination` forms one exact sum of the
  exact products f·x in FK's `ExactAccumulator`, rounded once (E13, S11).
  Subtraction is one binary64 subtraction; an envelope selects a row.
- **Rows** (PF `preview_physics.rs` `append_combination_results`; EX
  `exact_combinations.rs` `combined_row`). Rows pair by base id (PF) or by a
  typed structural key (EX). `displacement_magnitude` and support-force and
  -moment magnitudes are recomputed as the correctly rounded norm of the
  combined components (`core/numerics/correct_norm` `norm3`), except in an
  envelope, which selects them. Support groups publish whole or not at all.
  Kinds the pressure-2 table marks `never_combined` publish no combination row.
- **Gates** (PF `gate_reason`; EX §4 of `exact-combinations.md`), mechanics
  basis only: nonlinear supports withhold (`NONLINEAR_COMBINATION_REQUIRES_SOLVE`);
  a consuming constant-effort support withholds unless the unit factor-sum rule
  holds (`CONSTANT_EFFORT_COMBINATION_REQUIRES_SOLVE`, LCA
  `factor_sum_within_unit_tolerance`); mixed modulus bases withhold.
  Subtractions and envelopes are formed from rows in nonlinear models, with
  operand standing (#1695, slice 1a).

## 2. The members (model document 0.9.0)

- **`basis: "absolute_sum"`** and **`basis: "srss"`**. Both take `terms`, with
  the `mechanics` shape: at least one term, finite factors, no repeated operand.
  `minuend_id`, `subtrahend_id`, `operand_ids`, `mode` and `difference_basis`
  are refused on them under the existing shape codes.
- **Combinations as inputs.**
  - `CombinationTerm` gains `combination` (`Id`). A term carries exactly one of
    `load_case` and `combination` (`LOAD_COMBINATION_SHAPE_INVALID` otherwise).
    `load_case` stays required in the schema except where `combination` is
    present (an `oneOf` on `CombinationTerm`); no member is renamed.
  - `minuend_id`, `subtrahend_id` and `operand_ids` may name a load case or a
    combination. An id used as an operand that names both a load case and a
    combination is refused, blocking, as `LOAD_COMBINATION_OPERAND_AMBIGUOUS`
    (ids are unique per entity kind, not across kinds; `validation.rs`
    `detect_duplicate_ids`).
  - **Acyclic.** The graph whose edges run from a combination to each
    combination it names must have no cycle; a cycle is
    `LOAD_COMBINATION_OPERAND_CYCLE` (blocking, naming every combination on it).
    A combination may name one later in `combinations[]`: order in the array is
    not a dependency rule. Evaluation runs in the topological order that takes,
    among the ready combinations, the one earliest in the array, so the result
    order is a function of the document.
  - A combination naming an unknown id keeps `LOAD_COMBINATION_LOAD_CASE_UNKNOWN`'s
    rule under a kind-neutral code, `LOAD_COMBINATION_OPERAND_UNKNOWN`; the old
    code remains for a term's `load_case`.
- **Interim (S1 until the RT-A4 wiring slice),** as for `difference_basis`
  (0.9.0 §2.2): a document carrying either basis or a combination operand opens,
  validates and edits, and the applier refuses authoring them. The solve
  withholds each such combination, and each combination that names one, with a
  warning: `LOAD_COMBINATION_UNSIGNED_BASIS_UNSUPPORTED` or
  `LOAD_COMBINATION_NESTED_UNSUPPORTED`. The cases and other combinations
  publish. `LOAD_COMBINATION_BASIS_UNSUPPORTED` stays the blocking code for a
  token outside the closed set.

A flat document (every operand a load case, no new basis) solves byte for byte
as at 0.8.0. That is the regression the wiring slice keeps (§7, V-8).

## 3. Numerical definition (the algebra kernel)

For a row with operand values xᵢ (binary64, finite) and factors fᵢ (binary64,
finite), in authored term order:

- **Absolute sum:** A = RN( Σᵢ |fᵢ·xᵢ| ), the exact sum of the exact products
  |fᵢ|·|xᵢ|, rounded once to nearest, ties to even, at the result's binade
  (`ExactAccumulator::add_product`, `round`). An exact zero gives +0.0. A sum
  outside the binary64 range is `NonRepresentable` and the row is withheld
  (§6). This has no intermediate rounding.
- **SRSS:** zᵢ = RN(fᵢ·xᵢ), then R = RN( √( Σᵢ zᵢ² ) ): the exact sum of the
  exact squares of the zᵢ, its square root rounded once. An exact zero gives
  +0.0. A zᵢ that overflows, or an R above the largest finite binary64, is
  `NonRepresentable`.
  - **Why zᵢ is rounded.** The exact square of an exact product is a product of
    four binary64 values, which the accumulator (two-factor products, quantum
    2⁻²¹⁴⁸) cannot hold. zᵢ is exactly the value the row would have if the user
    had scaled the case by fᵢ, and for the usual factors (±1, or any power of
    two without under- or overflow) zᵢ = fᵢ·xᵢ exactly, so R is the correctly
    rounded root of the exact sum of squares. Rejected: a wider four-factor
    accumulator (about 8,600 bits) for a difference of at most half an ulp per
    term at non-dyadic factors.
  - **Order independence.** Both results depend only on the multiset of
    (fᵢ, xᵢ); term order cannot move a bit. The tests show it (V-3).
- **The square root, exactly.** S = Σ zᵢ² is held exactly (`add_product(zᵢ, zᵢ)`).
  A candidate r comes from √ of S rounded once at a scale 2⁻²ᵏ chosen from the
  largest |zᵢ| (`round_scaled`), so the scaled value is finite, times 2ᵏ. It is
  then moved one ulp at a time on the destination grid (subnormal results
  included) until the exact sign of S − m², at each neighbouring midpoint m, says
  it is nearest, ties to even. With r and its neighbour r⁺ binary64,
  4(S − m²) = 4S − r² − 2r·r⁺ − r⁺², a sum of products of binary64 values, so its
  sign is exact in a second accumulator (4S as each zᵢ² added four times). This
  is the method of `correct_norm::norm3` step 3, carried to n terms on the exact
  accumulator, and lives in LCA beside the factor-sum rule; FK is not edited.
  Bound: every product is at most 2²⁰⁵⁰ in magnitude and the accumulator carries
  64 bits beyond 2⁴¹⁹⁵, so at most 2⁶² terms; the kernel refuses more than 2³²
  terms by name rather than relying on it.
- **Agreement with the norm.** For two or three terms with unit factors,
  R = `norm3(z₀, z₁, z₂)` bit for bit (both are RN of the same exact root); V-2
  checks it.
- **Kernel API (LCA).** `AlgebraOperation` and `AlgebraExpression` gain
  `AbsoluteSum { terms }` and `Srss { terms }`, evaluated by
  `evaluate_absolute_sum` and `evaluate_srss` with the same findings as
  `evaluate_linear_combination` (empty, non-finite factor, duplicate, missing,
  dimension mismatch, status) plus `NonRepresentable` as a finding, never an
  infinity. Results carry the operand's dimension.

## 4. Rows

- **Signed rows** (PF `LINEAR_STATE_KINDS`; EX `signed_linear` signatures):
  each row is A or R of its operand rows, paired as today.
- **Recomputed magnitudes** (`displacement_magnitude`, support-force and
  -moment magnitudes; EX `recomputed_from_combined`): `norm3` of the combined
  components, as for a linear combination. For SRSS this equals the SRSS of the
  operands' own magnitudes in exact arithmetic. For absolute sums it bounds
  every signed combination's magnitude, max over signs of ‖Σ sᵢfᵢvᵢ‖, from
  above. The evidence states which.
- **`never_combined`** kinds publish no row, as today.
- **Sign convention.** A and R are unsigned. Row metadata reads, for example,
  "unsigned value: the absolute sum of |factor × source result| over explicit
  user terms; not an equilibrium state", and "unsigned value: the square root
  of the sum of squares of factor × source result over explicit user terms; not
  an equilibrium state". `metadata.basis` takes `explicit_user_absolute_sum` and
  `explicit_user_srss`. These texts enter the reader vocabulary (DEL-08-04).
- **Support groups** publish whole or not at all, and **`LOAD_COMBINATION_SOURCE_RESULT_MISSING`**
  and **`LOAD_COMBINATION_ROW_WITHHELD`** apply unchanged.
- **Nested operands.** A combination operand supplies its own published rows
  under the same base ids (PF) or structural keys (EX). A row the operand
  withheld or omitted is missing for every combination that names it, under the
  existing missing-source code. An operand combination withheld whole by a gate
  withholds its dependants whole: `LOAD_COMBINATION_OPERAND_WITHHELD` (warning),
  naming the operand and its own code.
- **Headlines and maxima** stay primitive-case only, as today.

## 5. Gates

Gates apply to every basis as stated; the flat cases reduce to today's gates
exactly.

### 5.1 The fixed-load count κ (constant effort)

A consuming constant-effort support's force acts in every case state (RT-A3
D-2), so each case carries it once. Give each operand a **fixed-load count** κ,
an exact dyadic rational:
- a load case: κ = 1;
- `mechanics`: κ = Σᵢ fᵢκᵢ, exact;
- `result_state_subtraction`: κ = κₘ − κₛ;
- `range_envelope`: κ is the operands' common κ when all are equal exactly;
  otherwise κ is mixed;
- `absolute_sum`, `srss`: κ = 0 (they are admitted only as below).

Where the model has a consuming constant-effort support (PF
`consuming_constant_effort`; EX once RT-A3 admits the family), a combination is
withheld when:
- `mechanics`: |κ − 1| > 2⁻⁴⁶, the unit factor-sum rule with κᵢ in place of 1
  (`CONSTANT_EFFORT_COMBINATION_REQUIRES_SOLVE`, unchanged for flat
  combinations);
- `absolute_sum`, `srss`: any operand has |κᵢ| > 2⁻⁴⁶ or a mixed κ
  (`CONSTANT_EFFORT_UNSIGNED_COMBINATION_OPERAND`, warning). An unsigned
  combination of states that each carry the fixed force would count it once per
  term with its sign discarded, which no state of the system has;
- `mechanics`, `absolute_sum` or `srss` naming an operand with a mixed κ
  (`CONSTANT_EFFORT_COMBINATION_OPERAND_MIXED`, warning).

Subtractions and envelopes are not gated, as today. The comparison is exact on
the dyadic κ and the bound is the factor-sum rule's 64ε, by the same argument
(`pressure-contract.md`, "Constant-effort combinations"). It is implemented once,
in LCA beside `factor_sum_within_unit_tolerance`.

**Consequence for authoring.** In a model with a consuming constant-effort
support, the operands of an SRSS or absolute sum are differences: for each
axis, (base + EQx) − base, which has κ = 0 and is the seismic response about
the base state. That is also the form that makes sense with nonlinear supports
(§5.2). The desktop's seismic template writes this form (S3b).

### 5.2 Nonlinear supports

Absolute sums and SRSS are row operations, like envelopes: they claim no state.
In a model with nonlinear supports they are formed from rows, with each
operand's standing recorded in slice 1a's shape (`pf-nonlinear-operand-standing.md`),
and their row text says "not an equilibrium state". `mechanics` keeps
`NONLINEAR_COMBINATION_REQUIRES_SOLVE`.

**Open question OQ-1 (Agent 0).** A rule pack's occasional check wants
sustained ± E, with E an SRSS. In a nonlinear model that is a `mechanics`
combination and is withheld. Recommendation: in a nonlinear model, admit a
`mechanics` combination exactly when one term is a load case with factor 1 and
every other term has κ = 0 (a difference or an unsigned combination), publish
it from rows with operand standing, and label it "a solved state plus derived
differences or bounds; not an equilibrium state". It changes the meaning of an
existing gate for documents that nest, so it is Agent 0's call. Not adopting it
leaves E published and sustained ± E for the rule pack to form.

### 5.3 Modulus basis

The mixed-modulus gate (`COMBINATION_MODULUS_BASIS_MIXED`) extends to absolute
sums and SRSS, over the load cases reached through nested operands.

## 6. Refusals and diagnostics (summary)

| Code | Where | Severity |
|---|---|---|
| `LOAD_COMBINATION_OPERAND_AMBIGUOUS` | validator, solve | blocking |
| `LOAD_COMBINATION_OPERAND_CYCLE` | validator, solve | blocking |
| `LOAD_COMBINATION_OPERAND_UNKNOWN` | validator, solve | blocking |
| `LOAD_COMBINATION_UNSIGNED_BASIS_UNSUPPORTED`, `LOAD_COMBINATION_NESTED_UNSUPPORTED` | solve, interim until wiring; the applier refuses authoring | warning, withholds |
| `LOAD_COMBINATION_OPERAND_WITHHELD` | solve | warning |
| `CONSTANT_EFFORT_UNSIGNED_COMBINATION_OPERAND`, `CONSTANT_EFFORT_COMBINATION_OPERAND_MIXED` | solve | warning |
| `NonRepresentable` | row withheld as "load-case algebra refused" | in `LOAD_COMBINATION_ROW_WITHHELD` |

## 7. Verification

The reference is `validation/references/rt_a4_combinations/`, with its own
generator (exact `fractions.Fraction`, integer square root, no product code).

- **V-1, kernel table.** (factors, values) → expected bits for A and R:
  zeros and −0.0; one term; opposite signs; dyadic and non-dyadic factors;
  values spanning 2⁻¹⁰⁷⁴ to 2¹⁰²³, including subnormal results and an R in the
  subnormal binade; overflow of zᵢ and of R (refused); near-midpoint roots
  (S = m² ± one quantum); exact squares (R exact). The generator rounds the
  exact root with `math.isqrt` on a scaled integer and a remainder test, which is
  independent of the midpoint method.
- **V-2, norm agreement.** R equals `norm3` for two and three unit-factor terms
  on a random sample, both signs.
- **V-3, order.** Every permutation of a five-term table gives the same bits.
- **V-4, mutants killed:** a binary64 fold in place of the exact sum; `hypot`
  chains; rounding each square; ties away from zero; −0.0 for a zero result.
- **V-5, closed form, three-axis static seismic.** An invented cantilever
  under uniform g in X, Y and Z separately. Tip displacements and root
  reactions are the textbook closed forms (wL⁴/8EI, wL), and their SRSS and
  absolute sums are formed exactly by the generator. Both routes, both solver
  modes (wiring slice).
- **V-6, κ gate.** A model with a consuming constant-effort support: SRSS over
  load cases withheld; SRSS over (base + EQ) − base admitted; sustained + SRSS
  (κ = 1) admitted on a linear model; a mixed envelope operand withheld.
- **V-7, nesting.** Cycles of length 1, 2 and 3 refused; ambiguity refused; a
  later-declared operand evaluated first; a withheld operand withholding its
  dependants; result order a function of the document.
- **V-8, flat regression.** Every committed combination fixture and frozen
  reference solves byte for byte as on main (wiring slice).

## 8. Slices, pricing and readers

| Slice | Content | Gates |
|---|---|---|
| K (now) | LCA kernel (§3), V-1–V-4, the reference generator | `cargo test` for LCA; independent numerical review |
| F (S1) | Schema, wire, validator codes, interim refusals (§2) | with 0.9.0 S1 |
| W (after S1) | PF and EX rows (§4), gates (§5), nesting, reader vocabulary, V-5–V-8, theory note and user-guide section | Numerical review; D3 if D1-reachable code moves (`exact_combinations.rs` and PP `lib.rs` are priced files, so it is expected) |

The kernel slice adds functions to LCA only. LCA is reached from the retained
route through `exact_combinations.rs`, but the new functions are called only
from the wiring slice, so slice K is expected to price coefficient-identical;
`compare` confirms it at slice W's registration, not before.

**Readers.** The CAEPIPE SI writer refuses the new bases and nested operands
by name in S1 (MBF combination mapping is DEL-17-04's). The independent
result-envelope readers (RE `preview_physics_evidence.rs`,
`exact_combination_contract`) re-derive A and R from the published operand
rows with the same definitions in the wiring slice.

## 9. Shared with the dynamics lane

Response-spectrum modal and directional combination (RT-A8, DEL-04-09) also
needs an n-term SRSS. The kernel's `srss` is offered for reuse; its contract is
§3. Whether DEL-04-09 adopts it is that lane's choice.
