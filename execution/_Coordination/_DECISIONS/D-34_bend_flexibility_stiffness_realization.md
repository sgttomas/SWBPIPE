# D-34 - Bend/Branch Flexibility Realization In The Global Stiffness

**Status:** AWAITING_RULING
**Date prepared:** 2026-07-09
**Decision ID:** D-34
**Prepared by:** agent (piping work loop iteration 2026-07-09), at owner direction
**Basis:** owner scope adoption of 2026-07-09 (CAND-2 of
`plans/ASSESSMENT_2026-07-09_physical_model_evaluation.md`, approved verbatim
in `loop/LOOP_RECEIPTS.md` Receipt 3; codified as `DEC-066` in
`execution/_Decomposition/SOFTWARE_DECOMP.md` §12), which partially supersedes
the `DEC-045` multiplier-only realization for the solve path. Per the
2026-07-03 residual-work convention, `DEC-045` stays unedited immutable
history; this row carries the superseding method fork.

## 1. Decision Statement And Scope

The owner has adopted the scope: **user-entered bend/branch flexibility
factors shall enter the global stiffness so the solve is
flexibility-correct** ("I want to introduce flexibility factors and bend
flexibility together and now properly solve those elements", 2026-07-09,
Receipt 3), explicitly choosing implementation over disclosure (CAND-1
superseded, not executed). What remains human-gated here is the **realization
method** — how user flexibility factors reach the assembled stiffness — plus
the mandatory exit-evidence bar.

Unchanged under any option: the code-neutral boundary (all factors are
user-entered; no k or SIF equation, catalog value, or default ships —
F-PIP-1, PRD §6.1/§6.2), the SIF stress-review multiplier path, and all
F-PIP-2/3 claim fences.

## 2. Verified Facts (Checked 2026-07-09)

- Bends/branches are assembled as straight prismatic chords; the user
  flexibility factor is consumed only as a post-solve stress multiplier
  whose provenance string reads "base frame stiffness unchanged"
  (`core/product_physics/src/lib.rs`, `append_component_stress_multiplier_result`
  region, ~4630–4654).
- `flexibility_factor_user_value` modifies no stiffness term anywhere in
  `core/` (grep-verified at the PR #135 merge state of `main`).
- PRD §11.3.2 requires user-entered in-plane/out-of-plane flexibility
  factors on a "Bend Element / Bend Macro-Element" — unmet as written.
- The schema already carries the user slots: `schemas/component.schema.yaml`
  bend/branch families with `flexibility_factor_user_value` and full
  provenance (`DEC-045` evidence rows).
- No benchmark exercises bend flexibility against a known-flexibility
  reference; thermal validation evidence is axial-only
  (`validation/hand_calcs/mechanics/`).

## 3. Why This Is Human-Gated

The realization method partially supersedes a ruled outcome (`DEC-045`
Option C `mechanics_geometry_only` for bend/branch), changes the numerical
results of every model containing a bend, and sets the fidelity/effort
frontier for the largest-value mechanics tranche of the adopted program
(K-AUTH-1; D-GOV-04).

## 4. Options

### 4.1 O-A — User-k-scaled bending stiffness on bend-span elements (recommended)

Elements associated with a bend (or branch) component are assembled with
their local transverse bending stiffness divided by the user-entered
flexibility factor: `EI_effective = EI / k_user` about the relevant local
axes. In-plane and out-of-plane factors, where both are supplied, map to the
element's local axes via the bend-plane orientation recorded on the
component; a single supplied factor applies to both transverse axes with the
mapping recorded in provenance. Axial and torsional terms are unchanged.
Elements not associated with a component are untouched. Missing factor on a
bend-flagged span = blocking diagnostic (no silent default, PRD §6.2), with
an explicit user-visible "rigid-realization" opt-out recorded in provenance.

- Boundaries: no curved element, no arc auto-meshing, no pressure
  stiffening (each a recorded follow-on trigger, §4.2); geometry stays the
  user's chords; the SIF multiplier path is unchanged.
- Why recommended: smallest correct step to a flexibility-correct solve;
  purely user-entered data (code-neutral fence intact); deterministic,
  easily hand-verifiable; reuses the existing element and both solvers
  unchanged (only assembled `EI` values change, dense/sparse parity oracle
  applies as-is).

### 4.2 O-B — Curved-bend macro-element

A dedicated arc-consistent bend macro-element (curved-beam stiffness with
the user factor applied), assembled like the `DEC-045` expansion-joint
macro-element. Higher geometric fidelity (captures arc geometry without
manual intermediate nodes); a substantially larger tranche (new element
formulation, new transform handling, new recovery path). Recorded here as
the named follow-on: prepare a new `D-XX` if O-A benchmark discrepancy
against a curved-element/known-flexibility reference exceeds the governed
thresholds adopted under §6, or on owner demand.

### 4.3 O-C — Defer

Retain multiplier-only realization. Listed for completeness; contradicts
the owner's 2026-07-09 scope adoption and is not recommended.

## 5. Recommended Disposition (PROPOSAL)

**O-A**, as a bounded Phase-P1 tranche of
`plans/PLAN_2026-07-09_physical_model_mechanics.md`, with this mandatory
exit-evidence bar:

1. An **expansion-loop (L-bend) thermal-bending benchmark** with a
   hand-calculated known-flexibility reference using invented user factors —
   the missing representative case — landed as a
   `validation/hand_calcs/mechanics/` witness plus benchmark-crate fixture.
2. k-sweep sanity evidence: monotonic anchor-moment reduction with
   increasing user k on the benchmark model.
3. Dense/sparse parity and the `DEC-025` five-surface sweep on a clean head.
4. Report/native-package provenance rows showing the factor now enters the
   solve (superseding the "base frame stiffness unchanged" string), with the
   `DEC-045` provenance conventions preserved.
5. Tolerances per the `DEC-024`/`DEC-026` class-tiered policy; new threshold
   values enter as governed fixture records, tighten-only.

## 6. On-Ruling Mechanism

On an owner ruling selecting an option: record the ruling verbatim in
`loop/LOOP_RECEIPTS.md`; update this row to RULED in `_REGISTER.md`; append
the codifying `DEC-*` entry to `execution/_Decomposition/SOFTWARE_DECOMP.md`
§12 citing this packet; execution proceeds branch-first as a P1 tranche
under the plan's evidence conventions. No lifecycle, release-readiness,
professional, certification, or code-compliance claim is created by the
ruling or the tranche.

## 7. Human Ruling And Disposition

*(blank until the owner rules)*
