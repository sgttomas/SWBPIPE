# Operand standing of pressure-free combinations in nonlinear models

This design states what the pressure-free route (PF, preview-physics-2) publishes beside
a `result_state_subtraction` or `range_envelope` combination of a model with nonlinear
supports. It is slice 1a of the operating-case item (Agent 0, 2026-10-27, under the
owner's delegation), and serves RT-A5 of `docs/RELEASE_TARGET_v1.0.md`. It adds no case kind, no model-document member and no solve.
The 0.9.0 member `combinations[].difference_basis` is slice 1b and is not part of this
design.

Paths are relative to `projects/chirality-piping/`. PP is `core/product_physics/src`;
RE is `core/reporting/result_export/src`.

## 1. What exists

- **Gates apply to `mechanics` only.** A mechanics combination of a model with any
  nonlinear support is withheld (`NONLINEAR_COMBINATION_REQUIRES_SOLVE`, PP
  `preview_physics.rs` `gate_reason`). A subtraction and a range envelope are formed
  from the operand cases' published rows (`append_combination_results`).
- **Each operand case is solved independently.** Every case iterates from the model's
  authored initial support states (`built.nonlinear_initial_states`); no state is
  carried between cases. Each case keeps its own reported support state, under
  DEC-067's static, history-free friction model, in which friction may admit several
  states (DEL-04-04 CLM-012, DEL-04-04:2; DEL-04-01 `kd5-nonlinear-active-set.md`
  §2.3).
- **Standing of the operands.** A confirmed case reports
  `NUMERICAL_INTEGRITY_CHECKS_PASSED`; an unconfirmed one is demoted (Sensitive), and
  every gap-bearing case is `NUMERICAL_INTEGRITY_RECOVERY_BASIS_UNQUALIFIED`
  (Unresolved). The document's `numerical_quality.cases` states each case's record.

Before this slice the combination stated neither its operands' standing nor that its
operands ended in different support states.

## 2. What the combination publishes

Only in a model with nonlinear supports (the model condition of
`NONLINEAR_COMBINATION_REQUIRES_SOLVE`), for each admitted combination, which there is
always a subtraction or a range envelope:

### 2.1 `contract_evidence.combination_operands`

A closed array, one record per admitted combination in model order, present only when
there is at least one such record. Every other document keeps its bytes. The record is
W-16b's exact-route record shape (DEL-05-02 `exact-combinations.md` §6) reduced to the
members that apply on PF:
- `combination_id` and `basis`;
- `expression`: `{minuend_id, subtrahend_id}` or `{operand_ids, mode}`, as authored;
- `operands`: one per operand in the expression's evaluation order (minuend then
  subtrahend; a range envelope's sorted operand ids), each `{load_case_id,
  modulus_basis_ref, modulus_basis_temperature, numerical_quality}`. The modulus
  members are the load case's normalized selection (kelvin for a temperature).
  `numerical_quality` is the case's own record in the document's
  `numerical_quality.cases`, copied verbatim after the case assessment.
- `standing`: `derived_from_case_results`.

The rows are unchanged. They are derived inspection results without their own
numerical enclosure: a combination's standing is never "certified" or "established",
and its rows never count towards numerical eligibility, which stays with the primitive
cases. A combination is published with its operands' standing whatever that standing
is; it is never withheld for it (R-5).

### 2.2 Diagnostics

Ids are under `diagnostic:preview-physics:`, with the combination id as the
length-prefixed identity (`exact_source_identity`). Source `core/product_physics`.

**`COMBINATION_OPERAND_SUPPORT_STATES_DIFFER`** (info), id
`combination-operand-states:<identity>`. Present when at least one nonlinear support's
published final state (`nonlinear_support_active_set_state_code`, 0 inactive,
1 active, 2 sticking, 3 sliding) is not the same in every operand case. Supports are
taken in order of first appearance over the operands in evaluation order, each in its
rows' published order. A support an operand publishes no state for is reported as
`unreported` there and counts as differing. The comparison includes the published
state code and, for a sliding friction support, its reported slip sense. Slip sense uses
the relative-displacement row when present and otherwise the zero-ground displacement
row; an unavailable sense is reported explicitly. The state tokens are `inactive`,
`active`, `sticking`, `sliding (positive slip)`, `sliding (negative slip)`,
`sliding (slip sense unavailable)` and `unreported`; an unavailable sense, like an
unreported state, never establishes equal states. Message:

    combination <id>: the reported final state of <n> nonlinear support(s) differs between its operand cases: <named>. Each operand case is solved independently from the model's authored initial support states, under the static, history-free support model, and keeps its own reported support state; the combination's rows are derived from those case results and are not one solved state.

`<named>` lists the first eight differing supports, each `<support> (<case>: <state>,
<case>: <state>)` over the operands in evaluation order, joined by `; `, followed by
`; and <k> more` when more differ. `affected_refs`: the combination id, then every
differing support id. Equal state codes do not make a difference a thermal-only or
single-configuration result; no diagnostic states that.

**`COMBINATION_OPERAND_CHECKS_NOT_PASSED`** (warning), id
`combination-operand-standing:<identity>`. Present when at least one operand's
`numerical_quality.solve_quality` is other than `checks_passed`. Message:

    combination <id> uses operand case(s) whose numerical integrity checks did not pass: <case> (<solve_quality>), ...; its rows are still published, as derived inspection results without their own numerical enclosure; each operand's standing is stated in contract_evidence.combination_operands

The cases are listed in evaluation order, joined by `, `. `affected_refs`: the
combination id, then those case ids.

## 3. Readers

The Rust reader is RE `preview_combination_operands.rs`, called by
`validate_preview_physics_evidence`; the Python mirror is
`tests/verifiers/preview_combination_operands.py`, called by
`tests/verifiers/preview_physics_evidence.py`. Both check, without the model:
- **Presence.** The member is present exactly when the document is solved, publishes
  a nonlinear support state row and admits at least one combination. It has one record
  per admitted combination in gate order; no admitted combination may carry the
  mechanics basis in such a document. A blocked envelope has none.
- **Shape.** Closed records and operands; the basis is a subtraction or a range
  envelope with its own expression shape; the operands follow the expression's
  evaluation order; `standing` is `derived_from_case_results`. A range mode is one of
  `min`, `max`, `min_abs` or `max_abs`.
- **Standing.** Each `numerical_quality` equals the document's own case record bit
  for bit.
- **Rows.** Direct algebra rows carry exactly one matching load-case source row per
  operand, in evaluation order. Recomputed magnitude rows carry the required
  combination-component references. Missing, mixed or incorrect references on these
  rows are refused. Modulus-basis records are excluded from this row-source check; the
  producer emits them without row sources.
- **Diagnostics.** Both codes are re-derived whole (id, code, severity, message,
  source and affected refs) from the published state rows and the records; a missing,
  extra or altered diagnostic is refused, and so is a state row outside 0-3 or
  duplicated for one support and case.

With a supplied model (`reader_verdict.rs`; Python `reader_verdict.py`), each record's
id, basis, expression and operand modulus selections are the model's, one per
subtraction or range envelope in model order when the model has a nonlinear support,
and none otherwise.

The readers re-derive the statement from the published document. They do not re-run
the solve or re-establish any case's standing.

**Schemas and tables.** `results.v0.3.schema.yaml` and
`stress_neutral_export.v0.3.schema.json` give `PreviewPhysicsContractEvidence` the
optional closed member (its operand items are the exact route's
`PressureExactCombination` operands; basis `result_state_subtraction` or
`range_envelope`; standing the constant). The preview-physics-2 semantic table is
unchanged: no row kind, signature, combination algebra or limitation changes, and its
`combination_policy` sentences stay true.

## 4. Verification

PP `tests/op1a_operand_standing.rs`, both solver modes, through the captured value
entry, against independent closed-form expectations (an axial bar along X, anchored at
x = 0, L = 4 m, k = EA/L = 2^20 N/m, α = 2^-16 /K):
- **V-1, a gap closing under thermal growth.** Tip gap on UX closing in +,
  c = 2^-9 m; sustained F = 1536 N; operating F and ΔT = 16 K. Sustained: u = 3·2^-11 m,
  gap inactive, N = 1536 N. Operating: u = c, gap active, stop on the pipe −512 N,
  N = 1024 N. O−S: Δu = 2^-11 m, ΔN = −512 N, Δ(stop) = −512 N. The state diagnostic
  names the gap. Both operands are gap-bearing, so both are Unresolved and the warning
  names both. Twins: c = 2^-8 m (both open, O−S is δ_T = 2^-10 m with ΔN = 0, no state
  diagnostic); F = 2560 N (both active: S N = 2048 N, O N = 1024 N; O−S Δu = 0,
  ΔN = −1024 N; no state diagnostic).
- **V-3, friction.** Tip friction support on UX, explicit normal 4096 N, μ = 0.25;
  sustained F = 2048 N, operating F and ΔT = −8 K. Both slide in +: S u = 2^-10 m,
  O u = 2^-11 m, N = 1024 N in both, friction on the pipe −1024 N in both. O−S:
  Δu = −2^-11 m, ΔN = 0, no state diagnostic. The sequential-history witness
  (sticking, ΔN = +512 N) is DEL-04-04:2's disclosure, not a product result.
- **V-2, a one-way prop lifting off under anchor rise** (stage 2 R-a; the W-16 A-1
  request `validation/references/w16_nonlinear_motion/requests/a1.json`, s* =
  5PL³/(48EI) = 2^-10 m). Sustained P: prop active, R = 5P/16, anchor My = −3PL/16.
  s = 2s*: prop inactive, u_tip = s*, My = −PL/2; O−S ΔR = −5P/16, ΔMy = −5PL/16, with
  the state notice. s = s*/2: R = 5P/32, My = −11PL/32; O−S ΔR = −5P/32,
  ΔMy = −5PL/32, without a notice. Both modes.
- **Opposite slip.** The V-3 bar with sustained +2048 N and operating −2048 N: both
  slide, in opposite senses (u = ±2^-10 m, friction ∓1024 N); O−S Δu = −2^-9 m,
  ΔN = −2048 N. The subtraction and the range envelope both carry the state notice.
- **V-4.** Swapping a subtraction's operands negates every finite nonzero signed row;
  displacement and support-action magnitudes are unchanged, and zero comparisons ignore
  the sign bit. The case-order permutation publishes bit-identical case rows; a PP unit test instruments
  the loop's input seed and checks that every case starts from the authored initial
  states in every case order (a seed carried between cases fails it).
- **Range envelopes** carry the same records and diagnostics.
- **Byte identity.** A nonlinear model with only a mechanics combination, a linear
  model with combinations, and a nonlinear model without combinations publish no
  member and no new diagnostic.

Readers: RE `tests/preview_combination_operands_contract.rs` and Python
`tests/test_preview_combination_operands.py` read the shared corpus
`fixtures/results/preview_physics_nonlinear_operands_corpus.json` (written by the PP
test under `OP1A_WRITE_CORPUS`) and refuse forged standing, forged, missing and extra
diagnostics, forged presence, forged expressions and operand order, and a forged
model binding.

## 5. Later items

- A combination certificate for these rows is not designed; until one exists these
  rows stay inspection evidence.
- Gap certification (R-3), nonlinear supports on EX (R-4) and slice 1b's
  `difference_basis` with G-1/G-2.
- The desktop `operating` kind token and "operating − sustained" template are separable
  and not part of this slice.

## 6. Pricing (D3)

The producer, reader and table-free evidence changes touch priced files (PP `lib.rs`,
`preview_physics.rs`, `exact_combinations.rs`; RE `preview_physics_evidence.rs` and the
new reader). DEL-04-08 Design `retained-pricing-rules.md` §8.31 prices source `0fdda050b0`
against main’s §8.30 registration. All 1,586 merged coefficients are unchanged;
that registration carries. The reached reader scans and its gate-vector backing
remain priced, while D1.6 excludes the nonlinear record and notice bodies.
AC-004 witnesses are not required for this coefficient-identical repricing.
