# Absolute-sum and SRSS combinations, and combinations of combinations

**Exit criteria:** 1 ("three-axis static seismic
with SRSS and absolute-sum combination") and 2. The design basis was main
`8cab1b3e5` (2026-10-30). The algebra kernel is specified in §3 and the product
wiring in §10. Numerical meaning and persistence require targeted regression
checks and independent scrutiny (AGENTS.md, Work and verification).

**Decisions applied** (SWBPIPE Agent 0, 2026-10-30, on the load-case plan):
`absolute_sum` and `srss` join model document 0.9.0 before S1; a combination
may take earlier combinations as inputs, acyclic; a constant-effort gate like
the unit factor-sum rule is approved in principle, its exact form settled here
(§5) and reviewed. Each new member is refused by name until its slice lands.

Paths are relative to the repository root. PP is `core/product_physics/src`,
LCA is `core/loads/load_case_algebra/src`, FK is `core/solver/frame_kernel/src`.
Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081). Nothing
here records or implies acceptance, issuance or release.

## 1. What existed at the design basis

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
discarded. A product by a factor adds at most 53 significand bits, but a sum
aligns exponents and can add as many as the exponents differ (r3 P3: the r2
chain needs 1,075, 2,149, 3,223 and 4,297 bits for M₀ to M₃), so no
per-level bound is claimed; real documents, with factors of ordinary size,
need few bits.

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
  (`CONSTANT_EFFORT_COMBINATION_OPERAND_MIXED`, warning);
- `mechanics` whose own κ is undefined although its operands' are defined,
  because the exact result exceeds the representation (r3 P2): the unit
  factor-sum rule cannot be decided, so it is withheld under the source's code
  (`CONSTANT_EFFORT_COMBINATION_REQUIRES_SOLVE` or
  `COMBINATION_INSTALLED_REFERENCE_REQUIRES_SOLVE`), beside the info
  `COMBINATION_FIXED_LOAD_COUNT_UNREPRESENTABLE`. A gated use never treats an
  undefined κ as passing.

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
| `LOAD_COMBINATION_UNSIGNED_BASIS_UNSUPPORTED`, `LOAD_COMBINATION_NESTED_UNSUPPORTED` | solve, interim until wiring (removed by it, §10); the applier refused authoring | warning, withholds |
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
  is withheld; a chain exceeding 4,096 bits is undefined and named; with a
  fixed-load source, Q = mechanics(2A + t(M₂ − A)), whose κ = 2 + t⁴ needs
  4,298 bits, is withheld under the source's code (r3 P2).
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

## 10. As built (wiring)

Slice W lands on branch `claude/rt-a4-wiring` from main `a8299f918`. It implements
§2-§6 and removes S1's interim unsigned and nested withholding; `difference_basis`
and `minuend_final` stay withheld.

- **κ (§5.1).** LCA `src/fixed_load_count.rs`: `Dyadic` (sign, odd significand of
  at most 4,096 bits, `i64` exponent; exact products, sums and comparisons),
  `FixedLoadCount` with the tree rules, `mechanics_fixed_load_gate` (|κ − 1| ≤ 2⁻⁴⁶
  exactly; an undefined own κ or operand never passes),
  `unsigned_fixed_load_gate` (exact zero) and `solved_state_plus_increments`
  (OQ-1). Each caller maps the withholding to its own source's code. The flat
  decisions equal `factor_sum_within_unit_tolerance` (named sets and a 4,000-case
  sample).
- **Plan.** PP `src/combination_plan.rs` resolves operands (a term by kind; an id
  to the load case first), orders evaluation (Kahn, earliest ready in the array),
  and computes every combination's κ and the load cases it reaches.
  `CombinationExpression` gains `AbsoluteSum` and `Srss` and evaluates term bases
  positionally, so a load case and a combination may share an id.
- **Rows (§4).** Both routes evaluate in that order and read a combination
  operand's own published rows (PF: by base id, kept only while a later
  combination names it; EX: by structural key). Gates, EX records, rows, the
  combinations' modulus-basis records and the source-recovery exclusion's refs
  follow the evaluation order; a flat document's is its array order, so its bytes
  are unchanged.
- **Gates (§5).** Precedence: a withheld operand combination
  (`LOAD_COMBINATION_OPERAND_WITHHELD`, naming each withheld operand and its code);
  then PF: nonlinear (except OQ-1), κ on a consuming constant effort, modulus over
  every reached case; EX: modulus and resolved stiffness over every reached case,
  then gate 3 on κ, then gate 4 on κ (below). Decision OQ-1 is one pure predicate over the operand tree and κ
  (LCA `solved_state_plus_increments`, read through `Plan::solved_state_plus_increments`),
  used by both routes: on EX, a model with nonlinear supports withholds a `mechanics`
  combination outside it with `NONLINEAR_COMBINATION_REQUIRES_SOLVE` (after the
  withheld-operand rule, before gates 1-3) and admits one inside it, formed from rows
  with its operands' standing and the OQ-1 sign convention. EX cannot reach a
  nonlinear model before RT-A3 increment 4, so that path is tested at the gate and
  predicate (RT-A3's reference shape `mechanics(1·A + 1·srss(A − B))` admitted; a
  singleton, two load cases, a factor other than 1, an undefined κ and a nonzero κ
  withheld), and both EX readers mirror it, taking a published nonlinear support state
  row as the model condition. `COMBINATION_FIXED_LOAD_COUNT_UNREPRESENTABLE` (info) names
  every combination whose own κ overflows, gated or not, with ids
  `diagnostic:preview-physics:combination-fixed-load-count:<identity>` and
  `diagnostic:exact-combination:fixed-load-count:<identity>`.
- **EX gate 4 (the RT-A3 seam, agreed by RT-A3 and the load-cases manager when RT-A3
  increment 3's candidate `ac7fea603` was merged).** A consuming constant-effort support
  (DEC-049) is a second EX fixed-load source of the same κ rule, after gate 3: `mechanics`
  by |κ − 1| ≤ 2⁻⁴⁶ (`CONSTANT_EFFORT_COMBINATION_REQUIRES_SOLVE`), `absolute_sum` and
  `srss` by every operand's κ exactly 0 (`CONSTANT_EFFORT_UNSIGNED_COMBINATION_OPERAND`),
  and a `mechanics` combination naming an operand whose κ is undefined is withheld as
  `CONSTANT_EFFORT_COMBINATION_OPERAND_MIXED` (warning), as on PF (§5.1); PF's codes and
  texts. Each source keeps its codes; gate 3 names the reason where both apply. Flat, κ is
  the factor sum, so CE's flat decisions and messages are unchanged (RT-A3's CE2 cases pass
  as written). The producer and both EX readers (RE `exact_combination_evidence.rs`, Python
  `exact_combinations.py`) run one rule per source, and `results_pressure_2.v0.3` admits
  the three gate-4 reasons (`CONSTANT_EFFORT_COMBINATION_REQUIRES_SOLVE` had been missing
  from its enum). Tests: `rt_a4_combinations.rs`
  `v6_gate_4_joins_the_fixed_load_count_on_the_exact_route`,
  `v6_gate_4_forgeries_are_refused_on_the_exact_route`,
  `v6_undefined_kappa_withholdings_on_the_exact_route_by_source` and
  `v6_swapped_undefined_kappa_codes_are_refused_on_the_exact_route`;
  `tests/test_rt_a4_exact_constant_effort_seam.py` (mirror gate, CLI and mirror end to end,
  schema).
- **Undefined κ on EX, by source (Agent 0's decision, replacing this slice's first reading
  under gate 4).** §5.1 distinguishes a `mechanics` combination naming an operand whose κ is
  undefined from one whose own κ is undefined although its operands' are defined. On EX:
  - a constant-effort source alone: an undefined operand is withheld as
    `CONSTANT_EFFORT_COMBINATION_OPERAND_MIXED`, as on PF; an undefined own κ under
    `CONSTANT_EFFORT_COMBINATION_REQUIRES_SOLVE`;
  - an installed reference alone: both are withheld under
    `COMBINATION_INSTALLED_REFERENCE_REQUIRES_SOLVE`. §5.1 names no operand-mixed code for
    this source, so gate 3 keeps one `mechanics` code, and an undefined κ never passes;
  - both sources: gate 3 is checked before gate 4 (the producer's `gate_reason` and both
    readers test the installed reference first and return), so both are withheld under
    `COMBINATION_INSTALLED_REFERENCE_REQUIRES_SOLVE`, and
    `CONSTANT_EFFORT_COMBINATION_OPERAND_MIXED` never appears beside an installed reference.
  The own-undefined case is unchanged (the source's code, beside the info
  `COMBINATION_FIXED_LOAD_COUNT_UNREPRESENTABLE`). Only the constant-effort-alone undefined
  operand moved (from `CONSTANT_EFFORT_COMBINATION_REQUIRES_SOLVE`); no flat decision and no
  numeric output moves. The pressure-2 table's `mechanics_gates` list does not yet name
  `CONSTANT_EFFORT_COMBINATION_OPERAND_MIXED`; that text belongs with §10.2.
- **Choices this slice made where §2-§6 are silent** (reported to Agent 0; each is
  the narrowest reading):
  - On EX, a `mechanics` operand with an undefined κ, where an installed reference
    applies, is withheld under `COMBINATION_INSTALLED_REFERENCE_REQUIRES_SOLVE`:
    §6 names `CONSTANT_EFFORT_COMBINATION_OPERAND_MIXED` only, whose source could not
    arise on EX when this was built, and an undefined κ never passes (§5.1). With gate 4,
    where only a constant-effort support applies, such an operand is withheld as
    `CONSTANT_EFFORT_COMBINATION_OPERAND_MIXED` (Agent 0's decision; "Undefined κ on EX,
    by source" above).
  - Evidence shapes: a term `{combination_id, factor}`; an operand
    `{combination_id}` (its own record, earlier in order, states its standing); a
    row source `{combination_id, result_id}`; `missing_load_case_ids` may name a
    combination operand. PF's operand-standing notices read load-case operands only.
  - Sign conventions: §4's two example texts verbatim; an OQ-1 row's is the linear
    text followed by "; a solved state plus derived differences or bounds; not an
    equilibrium state".
  - A dependant of an interim-withheld combination (`difference_basis`,
    `minuend_final`) is removed at the solve, as S1 removes the withheld one, with
    `LOAD_COMBINATION_OPERAND_WITHHELD`.
- **Policy texts.** PF's published limitation and the preview-physics-2 and pressure-2
  tables' `combination_policy` still state the pre-RT-A4 policy; Agent 0 chose to make them
  state what is built (option (a)). The exact texts, the order they land in with #98's D3
  and the digests they move are §10.2; until then the OQ-1 exception and the unsigned bases
  are stated here, in the user guide §6.2 and in `docs/theory/load_combinations.md`.
- **Applier.** `absolute_sum` and `srss` join the closed basis set (create and basis
  edit); terms and subtraction/envelope operands may name combinations; the
  document validator refuses cycles and unknown operands as before.
- **Readers.** RE `combination_graph.rs` (a new dependency on LCA, whose kernel
  re-derives A, R and κ), `exact_combination_evidence.rs`,
  `preview_combination_operands.rs`, `preview_physics_evidence.rs` (gate codes;
  unsigned rows' texts and sign bit; with the model, A and R bit for bit) and
  `global_end_evidence.rs`. Python: `tests/verifiers/combination_graph.py` (an
  independent exact implementation: Fractions and an integer square root; it
  reproduces every V-1 row) and the three mirrors; `qualification_exact_pressure.py`
  binds the evaluation order. The desktop labels unsigned and OQ-1 rows, keeps them
  out of maxima and minima, and never draws them as one state.
- **Schemas.** `results.v0.3`, `results_pressure_2.v0.3` and
  `stress_neutral_export.v0.3` take the two basis tokens, the new gate codes and the
  shapes above; no version changes.
- **Verification.** LCA: κ tests (flat decisions, V-6, V-9, V-10 at the kernel).
  PP `tests/rt_a4_combinations.rs`: V-5 on both routes and both solver modes
  against `validation/references/rt_a4_combinations/v5/` (a cantilever along
  (3, 4, 0)/5 so the X and Y cases share components), V-6, V-7, V-9, V-10 and a
  non-representable row, every envelope read by the product reader with and
  without its model. V-8: the existing byte-identity suites
  (`pressure_free_byte_identity`, the w16 and op1a corpora, the retained
  successors) are unchanged.
- **Retained routes.** They keep declining the members
  (`FamilyFact::CombinationMember`); admitting them is a later slice with its D3.

### 10.1 Review r1 repairs (persistence and numerical review, five P2s)

- **Magnitude overflow (P1).** On both routes an absolute sum's or SRSS's recomputed
  magnitude (`displacement_magnitude`, support force and moment magnitudes) whose norm
  of finite combined components leaves the binary64 range withholds that row, or its
  whole support group, under `LOAD_COMBINATION_ROW_WITHHELD` as "load-case algebra
  refused" (§3, §6); every other result publishes. A linear combination keeps today's
  rule (a signed infinity blocks the envelope), so no flat outcome moves. Both EX
  readers mirror it.
- **Reader re-derivation (P2).** RE `preview_combination_gates.rs` and its Python mirror
  `tests/verifiers/preview_combination_gates.py`. With the model, every PF gate is
  re-derived in evaluation order after S1's interim removal (withheld operand; nonlinear
  except OQ-1; κ on a consuming constant-effort support; modulus over every reached case)
  and every unsigned row bit for bit; this replaces `unsigned_model_checks`. Without the
  model: `LOAD_COMBINATION_OPERAND_WITHHELD` needs its diagnostic naming earlier withheld
  combinations; `CONSTANT_EFFORT_UNSIGNED_COMBINATION_OPERAND` and
  `CONSTANT_EFFORT_COMBINATION_OPERAND_MIXED` are refused as unverifiable
  (`SOURCE_PREVIEW_PHYSICS_COMBINATION_GATE_UNVERIFIED`), since the document states no
  withheld combination's expression; an unsigned row is re-derived from the factors of its
  `combination_operands` record (nonlinear models) and otherwise refused
  (`..._UNSIGNED_UNVERIFIED`). The flat gates keep their model-free reading (the shared
  retained corpus's `t_gate_ok` must pass). **Consequence:** model-free callers of
  `semantic_contract::for_source` (report packaging `bindings.rs`, derivatives
  `derivative.rs`, the runner's result-envelope binding) refuse a linear-model PF document
  carrying an absolute sum or SRSS until they pass the model or PF publishes the factors.
  Resolved by §10.3: PF publishes the factors and κ inputs.
- **CLI (P3).** `openpipestress-runner read preview-physics` admits `model`.
- **Derivation metadata (P4).** A `displacement_magnitude` row of an unsigned or nested
  combination carries metadata `{component: displacement_magnitude, coordinate_system:
  global, location: node, basis, sign_convention}`; a flat combination's carries none, as
  before (V-8). A magnitude is paired and keyed by kind, unit and node on both routes and
  in every reader. The component token joins the result schemas' metadata enums; the
  tables take it with §10.2. The desktop recognizes an unsigned, OQ-1 or envelope row by
  its sign convention on a combination `basis_ref`, and a row stating no derivation takes
  its combination's (`combinationNotes`).
- **Nested rows (r2 repair, P2).** Both PF readers (RE `preview_combination_gates.rs`,
  Python mirror) now re-derive every row of an admitted recorded combination that is not
  unsigned and has a combination operand (mechanics, `result_state_subtraction`,
  `range_envelope`): from its operands' rows (a load case's rows, or the operand
  combination's published rows), with the model's factors (with the model) or the
  `combination_derivation` record's (without it), as the producer forms them, with the
  kernel's own functions: nested pairing (§4, r1 F4), the recomputed magnitudes, a missing
  operand row omitting the row, support groups whole or not at all, and withheld rows.
  Each published row must equal a derived row bit for bit (value, identity, metadata,
  `source_result_refs`) and each derived row must be published; the
  `LOAD_COMBINATION_SOURCE_RESULT_MISSING`, `..._VECTOR_COMPONENT_MISSING`,
  `..._VECTOR_INVALID` and `LOAD_COMBINATION_ROW_WITHHELD` statements naming it must be
  exactly the derived ones (`SOURCE_PREVIEW_PHYSICS_COMBINATION_NESTED_ROW`,
  `..._NESTED_WITHHELD`). A recorded envelope's mode must be a kernel mode. The same
  whole-row derivation reads every admitted recorded absolute sum and SRSS, flat or
  nested, after the unsigned-row checks above (which name a changed value
  `..._UNSIGNED_VALUE`): rows pair by the producer's rules, nested with a combination
  operand and flat otherwise (`combination_source_identity_matches`: kind, unit, entity
  and the whole metadata), and a magnitude states its derivation on both. The codes keep
  their `NESTED` names. Flat mechanics, subtractions and envelopes (load-case operands
  only) keep the reading they had before RT-A4: their gates and magnitude consistency are
  checked, their signed values are not re-derived by the PF readers (a pre-existing
  limit, unchanged here); a nested row is verified against its operand combination's
  published rows as they stand.

### 10.2 Policy texts (old → new)

Agent 0's option (a): the texts state exactly what is built; no numeric output moves.
**Applied at 56b7b0012.** The PF limitation sentence is `preview_physics.rs`
`LIMITATIONS[6]`, published in every PF document's `formulation_basis`, the retained
successors included. Those, and the tables (reviewed inputs #4, #5, #8, #11 and #12 of the
registered retained-memory profile), move only in this order: an independent contract
review of these texts; the texts applied through `semantic_contract_tables.py` (the two
maintained tables, then `derive`, `apply-u4b` and `repin --from-rev HEAD`) and the carriers'
writers; #98's D3 registration; then the retained successors and the shared retained corpus
regenerated through the registered route. While the profile reads Stale, the retained route
cannot regenerate them.

**`policy_id`.** It becomes `PREVIEW-PHYSICS-1-COMBINATION-v2` and `PRESSURE-2-COMBINATION-v2`.
The policy's meaning grows (combination operands, two bases, κ in place of the factor sum,
OQ-1's admission). A consumer keyed on the v1 id must not read a v2 document as primitive-only;
v1 documents keep v1's meaning.

**PF limitation** (`LIMITATIONS[6]`; the preview-physics-2, -timoshenko-1 and -retained-1
tables' copy).
- Old: "Headlines cover all load cases. Mechanics combinations are withheld for nonlinear
  supports, constant effort with sum of factors other than 1, or mixed moduli. Subtraction is
  a labelled difference. No combination maxima and no code compliance."
- New: "Headlines cover all load cases. In a model with nonlinear supports a mechanics
  combination is withheld unless one term is a solved load case with factor exactly 1, at
  least one other term exists, and every other term is a combination whose fixed-load count
  κ is defined and exactly 0; it is then formed from rows, with its operands' standing, as a
  solved state plus derived differences or bounds, not an equilibrium state. With a consuming
  constant effort a mechanics combination is withheld when its κ is undefined or differs from
  1 by more than 64ε, and an absolute sum or SRSS unless every operand's κ is exactly 0.
  Mechanics combinations, absolute sums and SRSS are also withheld for mixed moduli over the
  load cases they reach, and every combination is withheld with a withheld operand
  combination. Absolute sums and SRSS are unsigned. Subtraction is a labelled difference. No
  combination maxima and no code compliance."

**preview-physics-2 `combination_policy`** (and the two derived tables, which publish the same
policy). Members not listed are unchanged.
- `policy_id`: old "PREVIEW-PHYSICS-1-COMBINATION-v1"; new "PREVIEW-PHYSICS-1-COMBINATION-v2".
- `operands` (new member): "load cases or other combinations: a term names a load case or a
  combination by kind; a subtraction's or range envelope's id names the load case when one has
  that id and a combination otherwise; the combination graph is acyclic and is evaluated in
  topological order, the earliest ready combination in the array first; a combination operand
  supplies its own published rows, paired by kind, unit, entity and every metadata field
  except basis and sign_convention, while still comparing any section_basis suffix within
  basis (a displacement magnitude by kind, unit and node); a row an
  operand combination withheld or omitted is missing for every combination that names it".
- `fixed_load_count` (new member): "each operand's fixed-load count κ, an exact dyadic
  rational: a load case 1; mechanics the exact sum of each factor times its operand's κ; a
  subtraction the minuend's minus the subtrahend's; a range envelope the operands' common κ,
  else undefined; an absolute sum or SRSS 0 when every operand's κ is exactly 0, else
  undefined; undefined with an undefined operand (a subtraction or envelope propagates it) or
  beyond 4,096 significand bits (COMBINATION_FIXED_LOAD_COUNT_UNREPRESENTABLE); over load
  cases, κ is the exact sum of the binary64 factors".
- `operand_gate` (new member): `{"code": "LOAD_COMBINATION_OPERAND_WITHHELD", "condition":
  "an operand combination is withheld; the combination is withheld whole, naming each withheld
  operand and its code"}`.
- `mechanics_gates`: old `[NONLINEAR_COMBINATION_REQUIRES_SOLVE: "the model has any nonlinear
  support record, including one inactive in every case"; CONSTANT_EFFORT_COMBINATION_REQUIRES_SOLVE:
  "the model has a consuming constant-effort support and a factor is not finite or |sum(factors)
  - 1| > 64*epsilon, with sum(factors) the exact sum of the binary64 factors, compared exactly
  and not scaled by sum(|factors|)"; COMBINATION_MODULUS_BASIS_MIXED: "the mechanics operands
  have different modulus-basis keys"]`; new:
  - NONLINEAR_COMBINATION_REQUIRES_SOLVE: "the model has any nonlinear support record,
    including one inactive in every case, unless one term is a solved load case with factor
    exactly 1, at least one other term exists, and every other term is a combination whose κ
    is defined and exactly 0 (decision OQ-1), which is formed from rows as a solved state plus
    derived differences or bounds; not an equilibrium state";
  - CONSTANT_EFFORT_COMBINATION_OPERAND_MIXED: "the model has a consuming constant-effort
    support and some operand's κ is undefined";
  - CONSTANT_EFFORT_COMBINATION_REQUIRES_SOLVE: "the model has a consuming constant-effort
    support and the combination's own κ is undefined or |κ - 1| > 64*epsilon, compared exactly
    and not scaled by sum(|factors|); over load cases κ is the exact sum of the binary64
    factors, and a non-finite factor leaves it undefined";
  - COMBINATION_MODULUS_BASIS_MIXED: "the load cases the combination reaches through its
    operands, transitively, have different modulus-basis keys".
- `unsigned_gates` (new member): CONSTANT_EFFORT_UNSIGNED_COMBINATION_OPERAND: "the model has a
  consuming constant-effort support and some operand's κ is undefined or not exactly 0";
  COMBINATION_MODULUS_BASIS_MIXED: as in `mechanics_gates`.
- `gate_precedence`: old "first gate in list order supplies the single withheld reason"; new
  "operand_gate first, then the first gate in list order of the combination's form supplies
  the single withheld reason".
- `gated_forms`: old `["mechanics"]`; new `["mechanics", "absolute_sum", "srss"]`.
- `absolute_sum` (new member): "per row over linear_state_kinds, the exact sum of |factor ×
  source value| rounded once to nearest, ties to even; recomputed magnitudes are the norm of
  the combined components, an upper bound on every signed combination's magnitude; rows are
  unsigned (basis explicit_user_absolute_sum; sign convention \"unsigned value: the absolute sum
  of |factor × source result| over explicit user terms; not an equilibrium state\"); a value,
  or a recomputed magnitude, outside the binary64 range is withheld as load-case algebra
  refused under LOAD_COMBINATION_ROW_WITHHELD (a support group whole); not an equilibrium state".
- `srss` (new member): "per row over linear_state_kinds, the square root of the exact sum of the
  squares of each factor × source value rounded once, the root rounded once to nearest, ties to
  even; recomputed magnitudes are the norm of the combined components, which equals the SRSS of
  the operands' magnitudes in exact arithmetic; rows are unsigned (basis explicit_user_srss; sign
  convention \"unsigned value: the square root of the sum of squares of factor × source result
  over explicit user terms; not an equilibrium state\"); a term or value, or a recomputed
  magnitude, outside the binary64 range is withheld as load-case algebra refused under
  LOAD_COMBINATION_ROW_WITHHELD (a support group whole); not an equilibrium state".
- `canonical_metadata_vocabulary`: `basis` gains "explicit_user_absolute_sum" and
  "explicit_user_srss"; `component` gains "displacement_magnitude".

**pressure-2 `combination_policy`**. Members not listed are unchanged. physics-retained-1 has
no `combination_policy` of its own; its vocabulary and parent digest move by `derive`.
- `policy_id`: old "PRESSURE-2-COMBINATION-v1"; new "PRESSURE-2-COMBINATION-v2".
- `operands`: old "primitive load cases only; a combination is never an operand"; new "load
  cases or other combinations: a term names a load case or a combination by kind; a
  subtraction's or range envelope's id names the load case when one has that id and a
  combination otherwise; the combination graph is acyclic and is evaluated in topological
  order, the earliest ready combination in the array first; a combination operand supplies its
  own published rows under the same structural keys; a declared operating comparison
  (difference_basis) names load cases only".
- `fixed_load_count`, `operand_gate`, `gate_precedence`, `gated_forms`, `absolute_sum`, `srss`:
  as for preview-physics-2 (the `absolute_sum` and `srss` texts add the clause "a combined
  stress row keeps its operand rows' metadata basis" immediately after the parenthetical
  "(basis …; sign convention \"…\")", followed by "; ").
- `mechanics_gates`: old `[COMBINATION_MODULUS_BASIS_MIXED: "the mechanics operands have
  different modulus-basis keys"; COMBINATION_RESOLVED_STIFFNESS_MIXED: "some member's resolved
  stiffness differs between the operands: its selected E/nu/G and material, its
  property-temperature selection, or its section bases, shear model and Timoshenko shear
  area"; COMBINATION_INSTALLED_REFERENCE_REQUIRES_SOLVE: "the model has a connector or tie rod
  whose raw installed reference (q_ref, a null-space reference included, or the rod's reference
  extension) is nonzero, and a factor is not finite or |sum(factors) - 1| > 64*epsilon, with
  sum(factors) the exact sum of the binary64 factors, compared exactly and inclusively and not
  scaled by sum(|factors|)"; CONSTANT_EFFORT_COMBINATION_REQUIRES_SOLVE: "the model has a
  consuming constant-effort support (exactly one translational restraint DOF and a finite
  positive constant load) and a factor is not finite or |sum(factors) - 1| > 64*epsilon, with
  sum(factors) the exact sum of the binary64 factors, compared exactly and inclusively and not
  scaled by sum(|factors|)" (RT-A3's entry, on main before this text applies)]`; new:
  - NONLINEAR_COMBINATION_REQUIRES_SOLVE: the preview-physics-2 text (reached once the route
    admits nonlinear supports);
  - COMBINATION_MODULUS_BASIS_MIXED: "the load cases the combination reaches through its
    operands, transitively, have different modulus-basis keys";
  - COMBINATION_RESOLVED_STIFFNESS_MIXED: "some member's resolved stiffness differs between the
    load cases the combination reaches through its operands, transitively: its selected E/nu/G
    and material, its property-temperature selection, or its section bases, shear model and
    Timoshenko shear area";
  - COMBINATION_INSTALLED_REFERENCE_REQUIRES_SOLVE: "the model has a connector or tie rod whose
    raw installed reference (q_ref, a null-space reference included, or the rod's reference
    extension) is nonzero, and some operand's κ or the combination's own κ is undefined, or
    |κ - 1| > 64*epsilon, compared exactly and inclusively and not scaled by sum(|factors|);
    over load cases κ is the exact sum of the binary64 factors";
  - CONSTANT_EFFORT_COMBINATION_OPERAND_MIXED: "the model has a consuming constant-effort
    support (exactly one translational restraint DOF and a finite positive constant load) and
    some operand's κ is undefined";
  - CONSTANT_EFFORT_COMBINATION_REQUIRES_SOLVE: "the model has a consuming constant-effort
    support, every operand's κ is defined, and the combination's own κ is undefined or
    |κ - 1| > 64*epsilon, compared exactly and inclusively and not scaled by sum(|factors|);
    over load cases κ is the exact sum of the binary64 factors".

  The installed-reference entry precedes both constant-effort entries, so with both sources
  gate 3's code is the single reason (§10, undefined κ on EX, by source).
- `unsigned_gates` (new member): COMBINATION_MODULUS_BASIS_MIXED and
  COMBINATION_RESOLVED_STIFFNESS_MIXED as above; INSTALLED_REFERENCE_UNSIGNED_COMBINATION_OPERAND:
  "the model has such a nonzero installed reference and some operand's κ is undefined or not
  exactly 0"; CONSTANT_EFFORT_UNSIGNED_COMBINATION_OPERAND: "the model has a consuming
  constant-effort support and some operand's κ is undefined or not exactly 0".
- `stress_metadata`: old text, then "; a displacement_magnitude row of an absolute sum, an SRSS
  or a combination with a combination operand carries metadata stating its derivation
  (component displacement_magnitude, global, node, the expression's basis and sign convention);
  a flat combination's carries none".
- `canonical_metadata_vocabulary`: as for preview-physics-2.

**Digests and identities that move when applied** (each by its maintained writer): the five
tables (preview-physics-2 and pressure-2 edited; timoshenko-1, preview-physics-retained-1 and
physics-retained-1 by `derive`); the result, analysis-run and stress-neutral schemas embedding
their digests and the limitation records (`apply-u4b`, `repin`), and
`tools/release/schema_manifest.json`; the pins `repin` lists (`semantic_contract.rs`,
`compatibility.py` twice, `qualification_*.py`, the reader-outcome, persistence and report
fixtures, desktop tests); every PF document carrying `LIMITATIONS[6]` (40 files, among them the
`preview_physics_*` and `product_preview` fixtures, `validation/witness/inputs/del1005_*`, and
the retained successors and `retained_precision_cases.json`, which only the registered route
regenerates); and the retained-memory profile's reviewed inputs #4, #5, #8, #11 and #12 beside
#10, which this branch has already moved, all recorded by #98's D3. No numeric output moves.

### 10.3 Model-free derivation evidence (Agent 0's decision after review r1)

**Why.** Report packaging, derivatives and result export read PF documents without their
model (`semantic_contract::for_source`), and §10.1's P2 repair refused a linear-model
document carrying an absolute sum or SRSS there. Agent 0 decided that PF publishes what the
readers need, as a nonlinear model's `combination_operands` already states its factors, so
every reader verifies such a document bit for bit without the model.

**Shape.** `contract_evidence.combination_derivation`, an optional member of
`results.v0.3`'s `PreviewPhysicsContractEvidence` (and the same definition in
`stress_neutral_export.v0.3`); no version change:

```json
{"nonlinear_supports": false, "consuming_constant_effort": true,
 "combinations": [
   {"combination_id": "d1", "basis": "result_state_subtraction",
    "expression": {"minuend_id": "W1", "subtrahend_id": "W2"}},
   {"combination_id": "srss-d", "basis": "srss",
    "expression": {"terms": [{"combination_id": "d1", "factor": 1.0}]}}]}
```

- `nonlinear_supports` and `consuming_constant_effort` are the two model conditions the gates
  read (PF `Rendered`): the nonlinear gate with decision OQ-1, and the κ gates.
- `combinations` holds, in gate (evaluation) order, one record for every `absolute_sum`, every
  `srss` and every combination with a combination operand, withheld or not, and for every
  combination those reach through their operands, so each record's κ is re-derivable from
  earlier records. The expression is `combination_operands`' shape: a term names its operand by
  kind with its factor; a subtraction's or envelope's id names the load case when the document
  publishes one with that id (`preview_cases`) and a combination otherwise.
- The member is absent when no combination is unsigned or nested. A document with flat
  combinations only (linear, subtraction, envelope) publishes exactly as before (V-8), and a
  flat combination gains a record only as an operand of an unsigned or nested one.
- A nonlinear model keeps its `combination_operands` records. Where both state a combination,
  they must agree.

**Readers** (RE `preview_combination_gates.rs`, Python `preview_combination_gates.py`; the CLI
through `reader_verdict`). Without the model, the readers check the member's closed shape, its
order (each record's gate follows the previous one's), the resolution of every operand (a
published load case, or a combination recorded earlier) and its closure (every record is
unsigned, nested, or an operand of a later record). They also check agreement with
`combination_operands` and that `nonlinear_supports` is true wherever the document shows a
nonlinear support. From the records they re-derive κ and OQ-1. For each recorded combination
they then check its gate up to the modulus gate exactly: withheld operand, nonlinear except
OQ-1, κ. The modulus gate is admitted as stated, as for flat combinations, because the
modulus bases are not part of the evidence. Every admitted absolute sum's and SRSS's rows must
be A or R of the recorded factors and their operands' rows, bit for bit. A κ withholding
without its record, or a row citing an operand combination's row without its record, is
refused as `..._COMBINATION_GATE_UNVERIFIED`, and an unsigned row without its record as
`..._COMBINATION_UNSIGNED_UNVERIFIED`. A malformed or inconsistent member is
`SOURCE_PREVIEW_PHYSICS_COMBINATION_DERIVATION_EVIDENCE`. With the model, the member must
equal the one the model implies, with numbers compared as binary64 (the same code), before
the existing model re-derivation. A blocked envelope carries no member.

**Tests.** `rt_a4_combinations.rs`: every PF document in V-5 to V-10 reads with and without
its model; `pf_model_free_readers_verify_from_the_published_derivation` covers the tampers
(factor, operand, row, model condition, closure, missing member);
`pf_flat_documents_publish_no_derivation` checks that a flat document publishes none and that
one stated on it is refused. `tests/test_rt_a4_pf_reader_tampers.py` runs the same through the
runner CLI and the Python mirror.

### 10.4 Recorded results from before RT-A4 (historical read)

**Why.** §10.2's texts moved the preview-physics tables' digests. A preview-physics statement
carries its table's limitation array (`formulation_basis.limitations`) and no table digest, and
every reader requires that array to equal its table's (`preview_physics_evidence.rs`
`FORMULATION_BASIS`; the retained reader's G7 base read and transport check), so a statement
published before RT-A4 is refused by the current tables. A run recorded on main binds its
table `{id, sha256}`. Agent 0 made reading those runs a merge condition of #98: they read as
main read them, through a scoped historical path like pressure-2's recorded bases
(`RECORDED_RUN_TABLE_BASES`), without widening current admission.

**Registry (closed).** Each variant is the table's bytes as the main line held them, verbatim,
under `fixtures/results/recorded_pre_rt_a4/`, its SHA-256 checked when it is first read
(Rust `recorded_tables::VARIANTS`; Python `compatibility.RECORDED_PRE_RT_A4_TABLES`; desktop
`recordedPreRtA4.ts`, read as raw text and hashed):

| Identity | Recorded digest | Current digest | Origin | Read |
|---|---|---|---|---|
| preview-physics-2 | 1ccc9561… | e7b93f41… | main 6c3cb46d3 | under the variant's table |
| preview-physics-timoshenko-1 | a7b83c37… | ce49ebe0… | main 6c3cb46d3 | under the variant's table |
| preview-physics-retained-1 | b001070e… | 26dce9c2… | main 6c3cb46d3 | under the variant's table, its base under preview-physics-2 1ccc9561… (its inherited digest) |
| physics-retained-1 | cf29fd5f… | ca6bb983… | main 6c3cb46d3 | binding only |
| physics-retained-1 | 045f6f0d… | ca6bb983… | #101 01e05a486 (main once RT-A3 increment 3 lands first) | binding only |
| physics-retained-1 | 040ef9a6… | ca6bb983… | main bbf384dec (RT-A4; history once RT-A3 increment 4 restates pressure-2) | binding only |
| physics-retained-1 | 85c2678a… | 320dc3f1… | main 252bef969 (history once RT-A3 §4, the combination certificate, restates pressure-2) | binding only |

Every variant's rows are the current table's; only limitations, combination-policy text,
vocabulary and inherited digests moved. The exact successor's statements still read under the
current reader (its rows and receipt bindings are unchanged), so its variants change no reader
table: they are recorded bindings, as pressure-2's bases are. pressure-2 itself needs no
entry: its main digest (a2db16a5…) and #101's (be29fabc…) are already
`RECORDED_RUN_TABLE_BASES`, and its statements carry no limitation the reader compares.
RT-A3 increment 4 (DEL-04-07 `pressure-contract.md`, "Nonlinear supports on EX") restates
two pressure-2 limitations, so the RT-A4 main line's pressure-2 digest (10455bf3…) joins
`RECORDED_RUN_TABLE_BASES` and its exact successor table (040ef9a6…) joins this registry as a
binding-only variant (records: main bbf384dec's statement verbatim and a record built by that
revision's builder, `fixtures/persistence/recorded_pre_rt_a4_pf/physics_retained_1/main_bbf384dec/`). RT-A3 §4 (DEL-05-02 Work `combination-certificate.md`) restates pressure-2's combination standing and limitation in the same way: main 252bef969's pressure-2 digest (2c20583d…) joins `RECORDED_RUN_TABLE_BASES` (records: main's verbatim, `fixtures/persistence/recorded_model_0_9_0_pre_cert/`), and its exact successor table (85c2678a…) joins this registry as a seventh, binding-only variant (records: `physics_retained_1/main_252bef969/`).

**Selection rule.** A variant is selected only by a record's binding
`analysis_run.reproducibility.semantic_contract` that is exactly `{id, sha256}` and equal to a
registry member whose identity is the statement's `producer.semantic_contract_id`, and only on
the recorded-run read, compare and re-save paths. There the readers run inside
`recorded_tables::read_recorded`, which selects the variant for that read alone (a scoped
thread-local; see "Scope of the selection" below): the PF evidence reader, its transport check, the header's
table, `binding_of`, and the retained reader's G0 table, inherited-digest check and G7 base read
all take the variant's tables. The current binding on a recorded entry reads as the current
entry does. Any other binding is refused by name: `SOURCE_RECORDED_BINDING_UNKNOWN`, or
`SOURCE_RECORDED_BINDING_IDENTITY` for a variant of another identity (a successor under its
base's variant, an exact successor under a preview successor's, and so on); a refused binding
reads nothing.

**Scope of the selection.** The selection is the calling thread's and lasts only for the
synchronous call of `read_recorded`'s closure. It is restored to what the thread held before
however that call ends: a returned value, an `Err` the closure returns, a refused binding
(nothing is selected), or an unwinding panic (the guard's drop). A current binding nested in a
recorded read clears it for its own read (review r5); a recorded read nested in another restores
the outer one. No caller holds it across an async yield: every caller runs the read to
completion on its own thread (the wasm engine's exports, which are synchronous; the runner's
`read` command; the desktop's native code makes no recorded read). Under `panic = abort` a panic
does not unwind and the guard does not run. Natively that ends the process (and the selection
with it; the native profiles unwind). The wasm32 target aborts by default and a trap leaves the
instance usable with the selection still set, so every outermost entry runs inside
`recorded_tables::entry_scope`, which clears whatever selection it finds and leaves none: every
wasm export of `operation_applier` (a scan test, `wasm_exports_run_as_outermost_entries`, keeps
later exports in it) and the runner's `read` command. A selection leaked that way therefore never
reaches a current read (`recorded_tables::tests::a_leaked_selection_does_not_reach_a_current_read_through_an_outermost_entry`,
which fails without the guard).

**Entries.** Rust `reader_verdict::{dispatch, validate, transport, row_semantics,
row_semantics_each, retained, retained_transport}_recorded`; the runner's `read` admits a
`recorded_binding` member on `dispatch`, `preview-physics`, `preview-physics-transport`,
`retained-precision`, `retained-precision-transport` and `row-semantics` (no other reader
admits it); the wasm engine's `reader_recorded_json(entry, binding, source, model?,
invocation?)`. Python: `validate_analysis_run_v0_3(..., recorded_read=True)` reads a record
binding a variant through those entries and interprets its rows under the variant's table.
Desktop: `validateAnalysisRunV03(..., { recordedRead: true })`, called by
`buildRecordedRunContext` and `buildHistoricalRunContext` (and so by the comparison's
`recordedRunSide`); the route, the preview-physics validation, the successor validation, the
binding and the row interpretation then all come from the recorded read.

**What stays refused.** Every current path reads the current tables only and never selects a
variant: the producer and its output checks, the record builders (Python `build_analysis_run`,
desktop `buildAnalysisRunV03`), the record validator without `recorded_read` (the
stress-neutral package, the result export and the export panels call it so), the header
dispatch, `standing`, `numerical_use_standing` and rule binding. A statement carrying a
variant's limitations is refused on all of them as at a4ff570dc
(`SOURCE_PREVIEW_PHYSICS_FORMULATION_BASIS`; a preview successor's header is refused at the
dispatch). Also refused on the recorded read: an unknown or altered digest, a record binding
the current table over a statement published before RT-A4, a statement restated with the
current limitations under a variant's binding, mixed record and row bindings
(`ANALYSIS_ROW_SOURCE_BINDING_MISMATCH`), a binding with any third member, a variant table whose
bytes differ from its digest (`SOURCE_RECORDED_TABLE_HASH`), and tampered record or statement
bytes (their checksums, as before).

**Standing and display rule.** A recorded or reopened run stays historical. The standing a
recorded run displays, and a recorded comparison side's standing, are those its statement read
with when recorded (Rust's recorded dispatch; a successor's is its registration's, and a saved
successor never registers); they are not replaced by the current standing. They are display
evidence of a historical run with no Current or Solve authority: nothing makes a reopened run
Current, sets the solve stage, drives an overlay or rule check, or readies a report. The current
standing of the same statement stays `unsupported`. Four rules govern the display
(`HistoricalRunContext.tsx`, `runComparisonSides.ts`):

- The findings the historical reader computes under the variant (the recorded standing's
  findings) are kept apart from the reopen's findings, with their scope: the context's
  `historicalReaderFindings` and `recordedTableRead` (`{id, sha256}` of the variant), and the
  comparison side's `recordedTableRead`. They render under "As recorded under <id> (sha256
  <digest>); historical evidence only, not a current standing" (the reopened-run panel, the
  History panel's run check, the comparison's side text), never merged into the ordinary list.
- Every check's finding stays an ordinary finding: corruption and hash mismatches, the record
  validator's refusal with `recordedRead` whatever its code, a retained receipt mismatch or
  successor refusal, a refused recorded read (`SOURCE_RECORDED_BINDING_*`,
  `SOURCE_RECORDED_TABLE_HASH`, a malformed statement) and malformed evidence. The context keeps
  them as `checkFindings` (every ordinary finding but a current standing's).
- A favourable stored or as-recorded status never masks a failed reopen validation: a recorded
  run with any check finding is not compared (`recordedRunBindingFindings` is `checkFindings`;
  before this rule it kept only `HISTORICAL_`/`RECORDED_RUN_`/`ANALYSIS` codes, so a record the
  validator refused with, e.g., `SOURCE_RECOVERY_METADATA_CONTRADICTION` was offered as an
  eligible side on its as-recorded standing; `recordedPreRtA4.test.ts` "a stored record whose
  reopen validation fails" fails without the rule).
- Every display surface a reopened or recorded run reaches classifies it under its recorded
  table, explicitly as recorded, and never with a current-only reader. A current reader refuses
  the statement (its limitation 6 is the old table's), so a surface that read it with one showed a
  false current verdict; focused confirmations r6, r7 and r8 found these one surface at a time,
  and this rule now covers the whole class. The table is threaded explicitly as an optional
  `recordedTableRead` that every current caller omits. There is no ambient or module state, and
  an unknown or refused binding reads as refused (`unsupported`, `integrity_failure`, no
  dimension, no label or notice); it never falls back to the current reader. The threaded
  surfaces are:
  - `HistoricalRunPanel` passes the context's table to `ResultsPanel`.
  - In `ResultsPanel`, the route through `displayRoute` (`resultReader.ts`: the recorded
    dispatch under a table, else `sourceContract`). The route drives the label source and the
    health, and it states the standing line, which is prefixed "As recorded under ...".
  - `standaloneNumericalHealth` (`recordedResultStanding`).
  - The row categories, family groups, filters and readout dimensions (`semanticLabels`).
  - The row labels and the CSV "Row label" column (`resultRowLabel`).
  - `KnownSemanticNotices` (`knownSemanticNotices`).
  - Result Detail's family and endpoint pair (`buildResultInterpretation`).
  - The detail and endpoint-pair readouts and the endpoint-pair CSV dimension
    (`semanticDimension`).
  - The recorded comparison's rows and semantic identity (`flattenRunResult`,
    `resultSemanticIdentity`, from the side's `recordedTableRead`), with the side's standing
    (`recordedResultStanding`).
  - The reopen's and the History run check's own reads (`readStanding`, the successor's
    `validateRetainedPrecisionRecorded`, `validateAnalysisRunV03(..., { recordedRead: true })`).

  The row semantics under a table come from the wasm export `reader_recorded_row_semantics_each_json`
  (Rust `row_semantics_each_recorded`). The other surfaces carry no reader verdict and are not
  applicable:
  - `componentBody`, `hangerPreload`, `PhysicsSourceEvidencePanel` (producer id only), the
    section-basis headlines and the unit policy.
  - The navigator (`resultsNavigator.ts`).
  - The CSV provenance (`csvAnalysisRoute` is the producer id; `csvResultHash` hashes only).
  - `readNumericalHealth` (it takes the threaded standing's findings).
  - `retainedPrecisionStandingText` and `retainedRowClasses` (a session registration, which a
    saved run never has).
  - `RunComparisonView` and its CSV (they read the side's rows and standing).
  - `HistoryPanel`'s run check (it shows the context's scoped findings).

  Some current-only readers are not reached. `LoadReferenceOutputGate`, `loadReferenceDisclosure`,
  `retainedPrecisionDisclosure`, the key-results, summary and export panels, and report
  readiness all receive only the current solved result: the session's result is null while a
  historical run is shown. In the recorded record validator, `readerValidate("pressure")` and
  `validateRetainedRecoverySource` are reached only on the pressure routes, which have no
  registry variant. A historical side's identity is its table's binding, so it compares with
  runs recorded under the same table, and a pair with a current run is refused as
  `semantic_identity_differs`. The retained standing restriction (needs recompute) is unchanged.
  `recordedPreRtA4Display.test.tsx` renders `HistoricalRunPanel` for each registry variant.
  It selects one row of every kind, so Result Detail and each endpoint pair show, and it exports
  every CSV. It requires none of the current readers' refusal texts and a converting readout for
  every displacement and stress row, and it requires the same row cells, Result Detail and
  notices as the statement's current twin in `fixtures/results`. It checks the same for
  recorded comparisons under one table and for the History run check. It fails at 485fb08f7:
  each of the retained milestone's 40 representative Result Details showed `integrity_failure`
  or an unknown-dimension readout, and its maximum-stress row label was missing.

Python has no display surface for these runs: `validate_analysis_run_v0_3(...,
recorded_read=True)` raises its refusal, and no product path turns it into standing.

**Schema contract.** `schemas/analysis_run.v0.3.schema.json` stays the current production and
admission schema; its semantic-contract digests are the current tables' only, and no accepted
family set is widened. Its `description` states that runs recorded under earlier tables are read
through this separate recorded historical-read contract (this registry, and pressure-2's
`RECORDED_RUN_TABLE_BASES`), that the schema makes no claim that such historical records validate
against it, and that a generic schema tool failing a stored record under it is not by itself proof
that the stored bytes are corrupt: the record's checksum and the product's recorded-run read
decide that. The description is an annotation only; the schema's digest moved with it
(3d2776bb… → d30507f1…, `tools/release/schema_manifest.json` regenerated).

**Bytes and re-save.** Nothing is migrated or resealed. Reading changes no byte. The record
path and the store read hashes and bytes, never a table, so a re-saved run keeps its recorded
binding, its received-result checksum and its statement's bytes: the store records main's
transient record into main's witness byte for byte, and Python's and the desktop's recorded
revision of it is main's witness in canonical form.

**Consumers.** Rust result_export (`recorded_tables`, `semantic_contract`, `retained_precision`,
`reader_verdict`): changed. Headless runner `read` (`reader_binding.rs`): changed. Python
`core/analysis_runs/compatibility.py` and `rust_reader.py`: changed (the `_TABLES` lookup is
unchanged; a recorded read returns the variant's table only when the reader's binding is the
record's). `tests/verifiers` mirrors the current reader and is unchanged; the historical test
substitutes the variant's limitations into it as an independent check. Desktop
`resultReader.ts`, `retainedPrecisionReader.ts`, `resultStanding.ts`,
`analysisRunCompatibility.ts`, `HistoricalRunContext.tsx`, `runComparisonSides.ts`,
`runComparison.ts`, `ResultsPanel.tsx`, `resultSemantics.ts`, `resultInterpretation.ts`,
`knownSemanticLimitations.ts`, `KnownSemanticNotices.tsx`,
`loadWasmEngine.ts`, `HistoryPanel.tsx`, `RunComparisonView.tsx` and the wasm export
(`operation_applier`, every export inside `entry_scope`): changed. Native src-tauri
(`analysis_runs.rs`, the store): not applicable; it reads hashes and bytes only, and a test
pins that it stores and reads main's run as main wrote it. The performance lane's C1 native
custody (DEL-04-05 `result-volume-design.md` §C1, branch `claude/rt-perf-c1-custody`) ports the
record validator and the reopen findings natively and must mirror this: in
`analysis_run_v03_validate.rs`, `validate_v03` takes the recorded-read option, which
`saved_run_validation` and `recorded_run_validation` pass and nothing else does; under it a
record binding `recorded_tables::variant_of` reads through `rv::validate_recorded`,
`rv::retained_recorded` and `rv::dispatch_recorded`, binds the variant (in place of
`recorded_run_semantic_binding`'s current binding) and interprets rows under the variant's
`table()`; `standing` for `saved_run_findings` / `recorded_run_findings` and the native
comparison side takes `rv::dispatch_recorded(...)["standing"]` (a successor keeps its
registration standing); `result_detail.rs` may use `rv::row_semantics_each_recorded` (rows are
identical, so its output is the same). Its parity corpus gains `fixtures/persistence/
recorded_pre_rt_a4_pf`. C1 must also mirror the display rule above (the historical reader's
findings kept apart with their table, every check's finding ordinary, and any check finding
refusing the comparison side) and run its reads synchronously on one thread. This is an explicit
dependency of C1 (#59): #98's closure does not imply that #59 already mirrors the historical read
or the display rule, and #59 is not complete for runs recorded before RT-A4 until it does.

**Tests.** Rust `result_export/tests/recorded_pre_rt_a4.rs` and `recorded_tables` unit tests
(restoration on an error, a refusal and an unwind; nested current and nested historical reads;
the leaked selection);
runner `reader_binding` (`a_recorded_binding_runs_the_recorded_entry`, and the unusable
inputs); wasm export `operation_applier/tests/reader_verdict_binding.rs`
(`the_recorded_entry_returns_the_recorded_verdicts`, `wasm_exports_run_as_outermost_entries`); native `analysis_runs_tests.rs`
(`a_run_recorded_before_rt_a4_is_stored_and_read_back_as_main_wrote_it`); Python
`tests/test_recorded_pre_rt_a4_pf.py`; desktop `src/services/recordedPreRtA4.test.ts` (the
display scope and the masking regression), `HistoricalRunContext.test.tsx` and
`HistoryPanel.runs.test.tsx` (the rendered label and lists). Fixtures:
`fixtures/persistence/recorded_pre_rt_a4_pf/` (README there: main's preview-physics-2 records
verbatim; main has no recorded Timoshenko or successor run, so those records were built by
main's own record builder from main's verbatim statements). Repin never edits either
directory.

### 10.5 Deleting and restoring combinations (applier)

The RT-A4 wiring authored unsigned bases and combination operands, but the applier's
`delete_combination` and `delete_combination_term` still accepted only records and terms of
flat `{load_case, factor}` terms. So no subtraction, envelope or nested combination could be
deleted, and neither could a term that names a combination. This section adds those deletes and
their exact inverses. No route, member or basis is admitted, and no schema, version or policy
changes.

- **Before values.**
  - **A record whose `terms` is an array of flat `{load_case, factor}` terms keeps its
    historical display**, byte for byte: `"<id>; <label>; basis=<basis>; terms=<id> x <factor>;
    …"` (or `terms=none`). The predicate does not depend on the basis. Empty `terms` qualifies,
    which covers the subtractions and envelopes the normalizing create makes with `"terms": []`.
    So every record the desktop could delete before still deletes with the display it already
    sends. A `{load_case, factor}` term keeps `"<load_case> x <factor>"`.
  - **Every other record or term** is named by the canonical JSON (RFC 8785) of the stored
    record or term. This covers a record without `terms` (as an authored subtraction or envelope
    may be), a record with a combination operand, and a term naming a combination. The desktop's
    delete producer (#117, DEL-07-03) builds each before value from the stored record or term by
    these two rules. It takes every factor's text from the engine's own `display_number`
    (`display_number_text`, native and wasm, #118) and trims as Rust's `str::trim` does, so the
    two never diverge. JavaScript and Rust choose differently between equally short round-trip
    digit strings.
  - The before value is a stale check, never an inverse.
- **References.**
  - `OP-COMBINATION-DELETE-REFERENCED` refuses deleting a combination that another combination
    still names, and lists every reference:
    - a term's `combination`, of any factor including 0;
    - `minuend_id` or `subtrahend_id`;
    - an `operand_ids` entry.
  - Nothing cascades, and no orphan is left. Combinations made by a template delete top-down.
- **The inverse is the stored record or term itself, at its index.**
  - `create_combination` takes a restore form, `field_path` `combinations[i]`, and
    `create_combination_term` takes `terms.i`. The payload is stored exactly as given, at index
    i, so value types and number forms are kept. Canonical text alone would collapse them: the
    integer 10^18 + 100 and the float 10^18 + 128 render alike.
  - The restore form admits exactly what the normalizing create admits: the same closed basis,
    operand, shape and interim-member checks. The restored id must equal `target.ref` exactly,
    and the index must lie within the collection.
  - So a record no create authors deletes, but its restore is refused by create's own name.
    One example is a termless mechanics record (`OP-CREATE-COMBINATION-PAYLOAD-INVALID`).
    Another is a record carrying `subtrahend_support_state: minuend_final`
    (`LOAD_COMBINATION_SUBTRAHEND_SUPPORT_STATE_UNSUPPORTED`). The desktop's session checkpoint
    still restores such a document exactly.
  - The plain `combinations` and `terms` paths still create a normalized record at the end.
- **The document validator still judges every result**, including cycles and operand rules. The
  existing cardinality rule is unchanged: the model format admits a combination with no terms,
  so a last term deletes as a load-case term always has.
- **Verification.**
  - `operation_applier/tests/combination_delete.rs` covers:
    - deletes and restores on every basis;
    - the reference refusal, including factor 0, subtraction and envelope operands;
    - number forms across the 2^53 collision;
    - typed nested terms at their index;
    - negative controls: wrong before, already deleted, index out of range, id mismatch,
      existing id, a cycle on restore, and a malformed index path.
  - Contract corpus cases 86–90 run natively and in both wasm browser lanes.
