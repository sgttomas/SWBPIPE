# Absolute-sum and SRSS combinations, and combinations of combinations

**Row:** RT-A4 (Must), serving exit criterion 1 ("three-axis static seismic
with SRSS and absolute-sum combination") and RT-V1. **Basis:** main
`8cab1b3e5` (2026-10-30). **Status:** revision 3, after Codex reviews r1 (on
`bba44aac7`: one P1 and six P2s, marked "r1") and r2 (on `95b12033e`: F2-F5
and F7 resolved; one P1 and seven P2s, marked "r2"; no objection to the OQ-1
rule); the algebra kernel (§3) is the first build slice. Risk class: numerical meaning
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
    combination. Ids are unique per entity kind, not across kinds
    (`validation.rs` `detect_duplicate_ids`), so one id can name both. **It
    resolves to the load case when one has that id**, as every such reference
    does today, and to a combination only when no load case has it (r1 F3). A
    flat document with a combination and a load case sharing an id therefore
    keeps its meaning and its bytes. A combination shadowed this way is named
    through `CombinationTerm.combination`, or renamed; the desktop shows each
    reference's resolved target. No diagnostic is added, since one would change
    a flat document's envelope.
  - **Declared operating comparisons name load cases only** (r1 F7). A
    subtraction with `difference_basis` (and so with `subtrahend_support_state`)
    whose minuend or subtrahend resolves to a combination is refused, blocking,
    as `LOAD_COMBINATION_DIFFERENCE_BASIS_OPERAND_NOT_LOAD_CASE`: G-1 needs each
    operand's primitive-load multiset and `minuend_final` needs a final support
    status, which a combination has neither of.
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
  largest |zᵢ| (`round_scaled`), so the scaled value is finite, times 2ᵏ; a
  candidate that is infinite or above MAX (the largest finite binary64) is
  clamped to MAX before adjustment (r1 F6). It is then moved one ulp at a time
  on the destination grid (subnormal results included) until the exact sign of
  S − m², at each neighbouring midpoint m, says it is nearest, ties to even.
  - **Midpoints with finite products.** Each midpoint is m = r + g/2 above r
    or m = r − g′/2 below it, where g and g′ are the upper and lower gaps at r
    (g′ = g/2 at the bottom of a normal binade, except at the smallest
    normal 2⁻¹⁰²², where both gaps are 2⁻¹⁰⁷⁴, r2 F1), both powers of two, finite,
    nonzero and representable, including the subnormal gap 2⁻¹⁰⁷⁴ (half of it
    is not representable, so the gap itself is used). Then
    4(S − m²) = 4S − 4r² ∓ 4·r·g − g·g, a sum of products of finite binary64
    values (4S and 4r² as four additions each), so its sign is exact in a
    second accumulator; g·g ≥ 2⁻²¹⁴⁸, the accumulator's quantum. No neighbour
    r⁺ is formed, so the identity holds at r = MAX, whose upper gap is 2⁹⁷¹,
    where r⁺ would be infinite.
  - **Overflow.** R is `NonRepresentable` exactly when S ≥ (MAX + 2⁹⁷⁰)²,
    the virtual midpoint above MAX, tested by that identity at r = MAX. At
    equality the tie goes to the even neighbour 2¹⁰²⁴, which overflows, so
    equality is refused (IEEE 754 rounding to nearest). V-1 tests S just
    below, at and just above the threshold; SRSS(MAX, 1) rounds to MAX and
    SRSS(MAX, MAX) is refused.
  - **Regression (r2 F1).** Operands 2⁻¹⁰²² − 2⁻¹⁰⁷⁴ and 2⁻¹⁰⁴⁸, unit
    factors: the candidate 2⁻¹⁰²² is the correctly rounded result, and the
    adjustment must stop there (a g/2 lower gap would oscillate).

  This is the method of `correct_norm::norm3` step 3, carried to n terms on the
  exact accumulator, and lives in LCA beside the factor-sum rule; FK is not
  edited.
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
  above, since each component |Σ sᵢfᵢvᵢ,c| ≤ Σ |fᵢvᵢ,c|. The evidence states
  which.
- **`never_combined`** kinds publish no row, as today.
- **Sign convention.** A and R are unsigned. Row metadata reads, for example,
  "unsigned value: the absolute sum of |factor × source result| over explicit
  user terms; not an equilibrium state", and "unsigned value: the square root
  of the sum of squares of factor × source result over explicit user terms; not
  an equilibrium state". `metadata.basis` takes `explicit_user_absolute_sum` and
  `explicit_user_srss`, except on EX's combined stress rows, which keep their
  operand rows' whole `metadata.basis` (recovery token and `; section_basis=`
  suffix included), exactly as `combined_row` does today, so their structural
  key and pressure-2 signature survive (r1 F5). On every row the derivation is
  stated by `basis_ref` (`ref_type` `combination`) and the sign convention.
  These texts enter the reader vocabulary (DEL-08-04).
- **Support groups** publish whole or not at all, and **`LOAD_COMBINATION_SOURCE_RESULT_MISSING`**
  and **`LOAD_COMBINATION_ROW_WITHHELD`** apply unchanged.
- **Nested operands.** A combination operand supplies its own published rows
  under the same base ids (PF) or structural keys (EX). A row the operand
  withheld or omitted is missing for every combination that names it, under the
  existing missing-source code.
  - **PF pairing (r1 F4).** PF matches a source row to the reference row by
    kind, unit, entity and the whole metadata (`lib.rs`
    `combination_source_identity_matches`), and a combination row's metadata
    carries its derivation (basis and sign convention), so a load-case row and
    a combination row of the same quantity would not pair. When any operand is
    a combination, rows pair by kind, unit, entity and every metadata field
    except `basis` and `sign_convention`, which describe derivation, not the
    physical quantity; any section-basis suffix within `basis` is still
    compared. When every operand is a load case, the match is today's, byte for
    byte.
  - **EX pairing.** EX's typed structural key already excludes derivation
    fields; with F5's rule a combined stress row keeps the section basis the
    key reads, so nested EX rows pair as primitive ones do. An operand combination withheld whole by a gate
  withholds its dependants whole: `LOAD_COMBINATION_OPERAND_WITHHELD` (warning),
  naming the operand and its own code.
- **Headlines and maxima** stay primitive-case only, as today.

## 5. Gates

Gates apply to every basis as stated; the flat cases reduce to today's gates
exactly.

### 5.1 The fixed-load count κ (constant effort and installed references)

A consuming constant-effort support's force acts in every case state (RT-A3
D-2), and so does a connector's or tie rod's nonzero installed reference
(K·q_ref, EX gate 3, `exact-combinations.md` §4), so each case carries every
fixed load once (r1 F2). Give each operand a **fixed-load count** κ,
an exact dyadic rational:
- a load case: κ = 1;
- `mechanics`: κ = Σᵢ fᵢκᵢ, exact;
- `result_state_subtraction`: κ = κₘ − κₛ;
- `range_envelope`: κ is the operands' common κ when all are equal exactly;
  otherwise κ is **undefined**;
- `absolute_sum`, `srss`: κ = 0 when every operand's κ is exactly 0, and
  undefined otherwise, **whether or not the model has a fixed-load source**
  (r2 F3). An unsigned combination with an undefined κ still publishes on its
  own where no gate applies; it is only refused as an operand of a gated use
  (below and §5.2).

A combination with an undefined operand has an undefined κ, except a
subtraction or envelope, which is not gated and propagates it.

**Representation (r2 F2).** κ is held as an exact dyadic rational: a sign, an
unsigned integer significand of at most 4,096 bits and a binary exponent in
`i64`, with every operation (a product by a binary64 factor, a sum, a
difference, a comparison) exact. It is not held in `ExactAccumulator`, whose
quantum 2⁻²¹⁴⁸ cannot represent nested products such as (2⁻¹⁰⁷⁴)³. A result
needing more than 4,096 significand bits makes κ undefined and names it
(`COMBINATION_FIXED_LOAD_COUNT_UNREPRESENTABLE`, info), so no residual is
discarded; real documents need far fewer bits (each nesting level adds at
most 53).

Where the model has a fixed-load source (PF: a consuming constant-effort
support, `consuming_constant_effort`; EX: a nonzero installed reference, and a
constant-effort support once RT-A3 admits the family), a combination is
withheld when:
- `mechanics`: |κ − 1| > 2⁻⁴⁶, the unit factor-sum rule with κᵢ in place of 1.
  The code is today's for the source: `CONSTANT_EFFORT_COMBINATION_REQUIRES_SOLVE`
  or `COMBINATION_INSTALLED_REFERENCE_REQUIRES_SOLVE` (both when both apply),
  unchanged for flat combinations. Today EX's gate 3 reads the outer factors
  only; nested, it reads κ, so `mechanics(1·(A − B))` (κ = 0) is withheld, as it
  must be;
- `absolute_sum`, `srss`: any operand's κ is not exactly 0, or is undefined
  (`CONSTANT_EFFORT_UNSIGNED_COMBINATION_OPERAND` or
  `INSTALLED_REFERENCE_UNSIGNED_COMBINATION_OPERAND`, warning). An unsigned
  combination of states that each carry the fixed load would count it once per
  term with its sign discarded, which no state of the system has. **Exact zero,
  not the 2⁻⁴⁶ tolerance (r1 F1):** with a tolerance, an operand
  D = (1 + 2⁻⁴⁶)A − B has κ = 2⁻⁴⁶ and carries 2⁻⁴⁶ of the fixed load; an
  absolute sum of 2⁴⁶·D would carry the whole fixed load once and still be
  given κ = 0, and B plus it would publish the fixed load twice under κ = 1.
  Requiring exact zero makes the reset to κ = 0 true. A difference of two load
  cases, (base + EQx) − base, has κ = 0 exactly;
- `mechanics` naming an operand with an undefined κ
  (`CONSTANT_EFFORT_COMBINATION_OPERAND_MIXED`, warning).

Subtractions and envelopes are not gated, as today. Every κ is propagated
exactly (the dyadic representation above), never rounded, so a scaled operand's
residual is carried, not lost; the only tolerance is the unit factor-sum rule's
64ε at a `mechanics` combination's own κ, by that rule's argument
(`pressure-contract.md`, "Constant-effort combinations"). It is implemented
once, in LCA beside `factor_sum_within_unit_tolerance`.

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

**Decision OQ-1 (SWBPIPE Agent 0, 2026-10-31, under the engineering-judgment
delegation; adopted as recommended).** A rule pack's occasional check wants
sustained ± E, with E an SRSS or absolute sum of seismic responses, and in a
nonlinear model such a `mechanics` combination was withheld. In a model with
nonlinear supports, a `mechanics` combination is admitted exactly when:
- one term names a solved load case with factor exactly 1,
- it has at least one other term (r2 F4: a single load case with factor 1 is
  a flat combination, which stays withheld), and
- every other term names a combination whose κ is exactly 0 (§5.1, r1 F1): a
  difference, an absolute sum or an SRSS of κ-zero operands, or a combination
  of those. An undefined κ does not qualify (r2 F3), so srss(A) of a load case
  A is not an increment.

κ here is computed whether or not the model has a fixed-load source. The
precise predicate (at least one other term; combination operands; defined
κ = 0) carries the adopted rule's stated intent, flat documents unchanged; it
was reported to Agent 0 with r2.

It is formed from rows, with each operand's standing recorded in slice 1a's
shape, and its row text reads "a solved state plus derived differences or
bounds; not an equilibrium state". Every other `mechanics` combination in a
nonlinear model keeps `NONLINEAR_COMBINATION_REQUIRES_SOLVE`, so a flat
document's outcome does not change. **Reason** (Agent 0): response
superposition of this kind is how occasional cases are formed in practice; the
product publishes the sum of one solved state and explicitly derived
increments, and claims no equilibrium state and no certification. Its build
takes targeted regression tests that a combination outside the rule is still
withheld (V-9).

### 5.3 Modulus basis

The mixed-modulus gate (`COMBINATION_MODULUS_BASIS_MIXED`) and EX's
resolved-stiffness gate (`COMBINATION_RESOLVED_STIFFNESS_MIXED`) extend to
absolute sums and SRSS. For `mechanics`, `absolute_sum` and `srss` they compare
the facts of **every load case reached through the operand graph**,
transitively, including through subtractions and envelopes (r1 F2). Today the
stiffness facts are looked up by primitive case id, so a combination id would
yield none and pass. A subtraction or envelope spanning different stiffness
stays publishable on its own, as today, but cannot then be an operand of a
gated combination.

## 6. Refusals and diagnostics (summary)

| Code | Where | Severity |
|---|---|---|
| `LOAD_COMBINATION_DIFFERENCE_BASIS_OPERAND_NOT_LOAD_CASE` | validator, solve | blocking |
| `LOAD_COMBINATION_OPERAND_CYCLE` | validator, solve | blocking |
| `LOAD_COMBINATION_OPERAND_UNKNOWN` | validator, solve | blocking |
| `LOAD_COMBINATION_UNSIGNED_BASIS_UNSUPPORTED`, `LOAD_COMBINATION_NESTED_UNSUPPORTED` | solve, interim until wiring; the applier refuses authoring | warning, withholds |
| `LOAD_COMBINATION_OPERAND_WITHHELD` | solve | warning |
| `COMBINATION_FIXED_LOAD_COUNT_UNREPRESENTABLE` | solve | info |
| `CONSTANT_EFFORT_UNSIGNED_COMBINATION_OPERAND`, `INSTALLED_REFERENCE_UNSIGNED_COMBINATION_OPERAND`, `CONSTANT_EFFORT_COMBINATION_OPERAND_MIXED` | solve | warning |
| `NonRepresentable` | row withheld as "load-case algebra refused" | in `LOAD_COMBINATION_ROW_WITHHELD` |

## 7. Verification

The reference is `validation/references/rt_a4_combinations/`, with its own
generator (exact `fractions.Fraction`, integer square root, no product code).

- **V-1, kernel table.** (factors, values) → expected bits for A and R:
  zeros and −0.0; one term; opposite signs; dyadic and non-dyadic factors;
  values spanning 2⁻¹⁰⁷⁴ to 2¹⁰²³, including subnormal results and an R in the
  subnormal binade; overflow of zᵢ and of R (refused); near-midpoint roots
  (S = m² ± one quantum); exact squares (R exact); the overflow threshold
  S = (MAX + 2⁹⁷⁰)² and one quantum either side, SRSS(MAX, 1) = MAX and
  SRSS(MAX, MAX) refused. The generator rounds the
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
  (κ = 1) admitted on a linear model; a mixed envelope operand withheld; r1
  F1's chain, absolute_sum(2⁴⁶·((1 + 2⁻⁴⁶)A − B)), withheld. On EX with a
  nonzero installed reference: mechanics(1·(A − B)) withheld under gate 3, and
  an SRSS over load cases withheld. Gates 1-2 over a nested operand whose
  reached cases differ in stiffness: withheld.
- **V-7, nesting.** Cycles of length 1, 2 and 3 refused; an id shared by a
  load case and a combination resolves to the load case, byte for byte as on
  main; `difference_basis` naming a combination refused; sustained (a load
  case) plus an SRSS pairs every ordinary row on PF; a later-declared operand
  evaluated first; a withheld operand withholding its
  dependants; result order a function of the document.
- **V-9, OQ-1 rule.** In a nonlinear model: sustained + SRSS of differences
  admitted, with its label and operand standing; withheld under
  `NONLINEAR_COMBINATION_REQUIRES_SOLVE` when the load-case term's factor is
  not exactly 1, when two terms name load cases, when a remaining term has
  κ ≠ 0 or an undefined κ (B + srss(A)), for the singleton mechanics(1·A), and
  for every flat mechanics combination.
- **V-10, κ representation.** The r2 chain M₀ = A + tB, Mₙ₊₁ = A + t(Mₙ − A),
  t = 2⁻¹⁰⁷⁴: κ(M₂ − A) = t³ exactly, not 0, so an unsigned combination of it
  is withheld; a chain exceeding 4,096 bits is undefined and named.
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
