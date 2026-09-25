# Stable consumer review findings

Reviewer: `/root/physics_resume/joined_consumer_review`, actual parent
`/root/physics_resume`, delegated-harness-native TASK Type2. Product read-only;
no delegation, build, native, Git write, or external operation. Original review
basis is `MANIFEST.json` / `CANDIDATE.patch`, plus the subsequently supplied
`STABLE_TS.json` / `STABLE_TS.patch`. Findings below concern those frozen bytes.
Repair is in progress elsewhere; this file does not close any finding.

## CON-R1 — Python admits internally contradictory physics-1 evidence (P2)

At `core/analysis_runs/physics_evidence.py`, the case/extrema loop only binds
listed extrema to rows, without the reverse binding or duplicate physical slot
checks. Adding a unique-ID stress maximum for the existing unpressurized
case/member and pointing the headline to its arbitrary `100000000.25 Pa` value
therefore passes. Other missing direct-contract checks permit an understated
certified gap, an unknown extrema coefficient method, contradictory wall-cap
transfer, duplicate terminals, pressure moment, duplicate source-factor group,
unbound source-factor coefficient, and a cyclic operand on a non-v2 row.

`PROBES_FROZEN_PYTHON_CURRENT_CANONICAL.json` demonstrates all nine contradictions
through `_source_contract`, `numerical_use_standing`, public
`build_analysis_run`, `validate_analysis_run_v0_3`, and checksum verification.
All were admitted and marked numerically eligible with a matching record hash.
`probe_python_admission.py` preserves exact mutations. The unchanged control
also passes. This is analysis construction/admission evidence, not a claim that
an attacker can mint opaque native/headless producer provenance.

Remediation: enforce the direct physics-1 relationships consistently with the
Rust reader: extrema bijection/slot uniqueness, exact method strings and
`global_upper_bound - lower <= certified_gap`, direct-only operands on every
row, closed terminal relationships, and source-factor coefficient/group/moment
rules. Keep supported blocked empty evidence and finite unquantized values.

## CON-R2 — TypeScript refuses genuine pressure-plus-thermal output (P2)

`apps/desktop/src/features/results/physicsResultEvidence.ts:153` requires each
pressure applied-load ledger's `thermal_included` to equal the material's
`thermal_consumed`. The producer deliberately emits `thermal_included:false`
for this pressure-only eigen/cap ledger (`pressure_runtime.rs:605`) while
material evidence records actual thermal consumption separately (lines 549–603).

The parent supplied actual new sparse/dense producer captures with a thermal
primitive. Both have `MECHANICS_SOLVED` / `checks_passed`, material
`thermal_consumed:true`, and pressure ledger `thermal_included:false`. The frozen
TS reader rejects both with `PHYSICS_EVIDENCE_APPLIED_LOADS`, preventing valid
source interpretation/qualified use. `ACTUAL_THERMAL_TS_FROZEN.json` binds the
source hashes, capture record and actual rejections. These are genuine core
invocations and a direct reader probe, not native UI execution.

Remediation: bind the ledger to its actual pressure-only meaning. Do not alter
the producer observation or require thermal to disappear from the model.

## CON-R3 — TypeScript omits direct pressure/geometry guards (P2)

The frozen `physicsResultEvidence.ts` `section()` does not bind rounded radii to
authored OD/wall, and `validateRhs()` omits zero pressure moments, unique group
identity, and term coefficient magnitude binding to cap/Poisson kind. The
physical-row loop also skips unknown `_v2` kinds rather than rejecting the
unsupported reserved family. `PROBES_FROZEN_TS.json` confirms admission of all
five corruptions using the direct Node24 validator, including a contradictory
duplicate `ro_m=0.1` with unchanged OD/wall, term coefficient 999, pressure
moment 999 and a repeated group. Rust rejects these contradictions.

Remediation: apply the existing direct physics-1 guard relationships and the
closed supported v2 vocabulary; retain disclosed ordinary unknown signatures
only where the contract allows them. This result is direct validator evidence,
not a new authentic Current/export witness.

## CON-R4 — Rust temperature selection is detached from case basis (P2)

In `core/reporting/result_export/src/physics_evidence.rs`, `temperature()`
validates only shape/unit, and the region material loop removes
`temperature_basis` before comparing duplicate material records. No check binds
the selection to `exact_case.material_basis` and the material ID. Changing a
base-material region's selection to `exact_point` with a missing point ID
therefore passes the Rust code path while claiming contradictory E/nu source
provenance. Python and TS already perform the base/exact-point pairing.

This finding is established by complete static trace, not an executed Rust
mutation. Producer `pressure_material.rs` emits the paired case basis and
`pressure_runtime.rs::temperature_basis` emits the matching selection. The
canonical schema only checks shape and does not close the relationship.

Remediation: bind the region selection to the case/material basis without
inventing a new material authority, and add contradiction controls.

## CON-R5 — Rust remote closure reaction has no sign binding (P2)

The `separately_supported_or_compensated` branch in Rust
`physics_evidence.rs` checks the remote reaction only as a length-three finite
vector. It does not require it to be the negative of
`closure_pressure_load_global_n`. The producer emits exactly that negative and
TS checks it. Thus a coherent remote-closure carrier with one remote reaction
component changed to 999 retains admission with inconsistent reaction evidence.

This finding is static trace only. Remediation: preserve the zero transfer into
the pipe solve and bind the excluded remote reaction to the opposite closure
load; add a focused positive/negative remote-closure pair.

## Checks and limits

`POSITIVE_SCOPE_CHECKS.json` independently verifies the original four actual
raw fixtures, additive table counts/signature uniqueness, unchanged first 60
precision rows and precision byte hash, actual retained headless derivative
schema/source checksums, and the two actual blocked empty carriers' inspectable
analysis records with `needs_recompute`. The minimal core input and full UI
input have the same project ID but different exact bytes and load-case shapes;
neither is silently relabeled as the other.

The first Python probe lacked the checked executable at its default path;
`PROBES_FROZEN_PYTHON.json` preserves that unavailable attempt. The successful
rerun used the parent-supplied current NUM executable at
`/private/tmp/piping-numerical-corrections-canonical-resume-target/debug/openpipestress_jcs_ijson`,
SHA256 `bc1343ef45704c0ec65c1c7ac244c50e349af49de3a9b850dc11578bb2fc6c54`.
No fallback serializer or build was used. A first ad hoc positive-check command
missed the tests import path; the retained corrected script uses that actual
path and completed successfully.

Raw immutability changes were traced through headless production, opaque
same-Value evidence binding, derivative construction and the revised tests.
Rule status is now derived state while the original producer carrier and hash
remain unchanged. Public nonzero legacy-pressure refusals are separate from
locally unpressurized source-proof tests; no original physical oracle was
changed in this review. PHYS-R4 direct publication repair does not establish
joined public solve-range completion. Source-blocks-1 remains unjoined, and
precision-1 history remains immutable.

Final browser provenance/Current changes and later source/test/fixture deltas
still need separately frozen review coverage. No final-candidate, native,
engineering-acceptance, CI, merge or release conclusion is made here.

## Separate shared-conversion applicability note

The parent requested a bounded applicability check after NUM reported a Pa/MPa
representability defect under the separate unjoined source-blocks method.
Rust canonical derivatives and analysis records preserve source value/unit;
their mapping performs no division by a display scale. That NUM defect does
not by itself establish a physics-1/p1 canonical derivative defect.

Display preferences do reach a shared conversion path:
`ResultsPanel.tsx::SemanticResultQuantity` → `QuantityReadout` /
`useDisplayQuantity` → `displayQuantityService.ts` →
`operation_applier/src/display_units.rs` → `units::convert_for_dimension`.
The inspected cross-unit path checks finite input/output but does not contain
subnormal or underflowed outputs according to a relative representability
criterion. This is a generic display range risk; no claim is made that a
current physics/p1 solve reaches NUM's failing source state. The same-unit
exact-preservation branch remains distinct. Parent/NUM will supply the
concrete shared guard API and any separately authorized delta for review.

Phase-one hash recheck is `PHASE1_HASH_RECHECK.json`. It preserves every frozen
hash and records any concurrent repair changes as unreviewed deltas. Finding
closure requires the repair bytes and actual focused checks, not this initial
report or a passing pre-repair suite.
