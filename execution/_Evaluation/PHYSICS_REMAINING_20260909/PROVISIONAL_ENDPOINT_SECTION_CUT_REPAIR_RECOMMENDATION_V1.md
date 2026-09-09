# Provisional endpoint section-cut repair recommendation V1

## Status and evidence basis

This is an early, bounded implementation recommendation from EVALUATION P0. It is derivative evaluation evidence for an independently reviewed repair, not a public-contract adoption or proof against an external industry solver.

- Accepted source basis: `533332349a4607eee561d4ef90fb05a62d86519e`.
- `core/product_physics/src/lib.rs`: SHA-256 `f1ae3322a752d92e2125274e7c5ccc0f7ca2c26a00f19d8d5cd3d6c4234a3ed5`.
- `core/loads/stress_recovery/src/lib.rs`: SHA-256 `0c3a0b6278a1cce4a802fc6c2e4c81af1b6f393c5cde4bb28ff55b99d5d956f2`.
- The reproducer is a direct equilibrium/source trace. It does not rely on the broader exact-annulus pressure recommendation and does not claim a fresh runtime execution.

## Reproducible defect

`product_physics` preserves two different and legitimate quantities, but endpoint stress recovery currently mixes them:

- Raw element endpoint actions are retained for endpoint force rows.
- A common, j-side section-cut convention is used for interior station resultants.
- Endpoint stress recovery passes the raw endpoint action directly to `stress_recovery` at both ends.

The relevant current paths are:

- `product_physics/src/lib.rs:6847-6865`: axial fixed-end corrections add the equivalent axial load to the raw i-end UX action and subtract it from the raw j-end UX action.
- `product_physics/src/lib.rs:6325-6353`: the straight-station path negates the i-side action to obtain a common j-side section cut.
- `product_physics/src/lib.rs:7176-7255`: endpoint force rows intentionally remain raw i/j element actions.
- `product_physics/src/lib.rs:7475-7504`: endpoint stress recovery feeds each raw action directly into `ForceResultants`.
- `product_physics/src/lib.rs:7506-7534`: station stress recovery receives an already transformed section cut.
- The fixed-pressure test at `product_physics/src/lib.rs:14543-14581` checks only the i-end stress and therefore codifies one side of the inconsistent pair without testing end/station parity.

Minimal fixed/fixed axial proof: with zero displacement and axial equivalent action magnitude `P`, the corrected raw endpoint pair is `[Fi,Fj]=[+P,-P]`. Current endpoint axial stresses are `[+P/As,-P/As]`. The common j-side station cut is `N=-Fi=-P` and its stress is `-P/As`. The i-end stress therefore disagrees with the limiting interior section, while the two ends report opposite stress for a uniform state. The same error makes restrained uniform thermal expansion tensile at end i and compressive at end j rather than uniformly compressive.

For any self-equilibrated raw axial pair `[-N,+N]` representing member tension, the physical common-basis section resultants are `Ncut_i=-Fi=+N` and `Ncut_j=+Fj=+N`.

## Selected engineering behavior

Keep raw endpoint action rows unchanged. Before stress recovery, explicitly transform each endpoint action into a section cut expressed in one documented element-local basis:

- straight-element end i: negate all six generalized endpoint-action components;
- straight-element end j: retain all six components;
- recover axial, bending, and torsional stress only from those section-cut resultants;
- describe endpoint stress signs as section-cut/stress signs, rather than as raw endpoint-action signs.

This transformation is geometry and equilibrium bookkeeping. It is independent of whether the later pressure implementation uses an exact annulus or a named thin-wall reduction.

For curved macro elements, a chord-frame sign flip is insufficient at both physical ends. Rotate each recovered endpoint action into its local tangent section frame, apply the appropriate cut-face sign, then recover endpoint stress. If this tangent-frame transform cannot be proved in the bounded repair, omit or solve-block curved endpoint stress rows with a deterministic diagnostic; do not publish raw chord actions as physical endpoint stress. Existing curved station stresses may remain on their separately implemented tangent-equilibrium path.

## Exact implementation fence

The minimal straight-element repair should touch only:

- `projects/chirality-piping/core/product_physics/src/lib.rs`
  - add a pure endpoint-action-to-section-cut transform with explicit end side;
  - call it before `recover_endpoint_stress`;
  - leave `append_endpoint_force_result` and its raw-action semantics unchanged;
  - update endpoint stress metadata to name the section-cut convention;
  - replace the one-sided pressure expectation and add focused parity tests.

`core/loads/stress_recovery` need not change if it continues to receive correctly normalized `ForceResultants`. No schema, application, or governed-deliverable change is required for this correction. If curved endpoint stress is included, keep the additional source fence within existing curved-frame helpers called from `product_physics`; do not broaden into a pressure-contract rewrite.

## Required independent refutation

An independent reviewer should attempt to falsify the repair with:

- arbitrary self-equilibrated six-component endpoint-action pairs, checking the expected cut-side transformation component by component;
- axial tension and compression with both member orientations;
- fixed/fixed pressure and uniform-temperature cases, requiring end i, end j, and limiting station stresses to agree in the common local basis;
- free axial load cases and superposed pressure/thermal/mechanical axial actions;
- bending and torsional constant-resultant cases, including orientation reversal;
- unchanged raw endpoint force/action rows and their metadata;
- a mutation that restores raw i-end action recovery, which end/station parity must kill;
- a separate curved-end tangent-frame oracle before curved endpoint stress rows are accepted.

External industry-solver comparison remains a later validation phase.
