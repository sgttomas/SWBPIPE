---
doc_id: OPS-VALIDATION-MANUAL-SKELETON
doc_kind: governance.validation_manual
status: draft
created: 2026-05-04
updated: 2026-08-09
refs:
  - rel: governed_by
    to: OPS-CONTRACT
  - rel: strategy
    to: OPS-VALIDATION-STRATEGY
  - rel: implements
    to: DEL-09-04
---

# Validation Manual

## 1. Authority Boundary

This manual organizes verification and validation evidence for OpenPipeStress
software quality. Candidate designs are validated in the user's accepted
professional tools (external-prover correlation, PRD §22.5). Internal
benchmarks and rule checks are development verification and screening
evidence.
Acceptance, professional judgment, and any certification, sealing, or
code-compliance determination remain with the responsible engineer and
project authority (boundary vocabulary: `docs/claims_registry.md`, DEC-081).

Current authority basis: `execution/_Decomposition/SOFTWARE_DECOMP.md`
revision `0.11` and approved `execution/_DAG/DAG-009/` graph coordination
authority. `DAG-009` supplies active dependency context only; it does not
settle lifecycle state, release labels, or legal clearance, and acceptance and
professional judgment remain with the responsible engineer.

Use this manual to answer four separate questions:

| Question | Evidence class | Decision owner |
|---|---|---|
| Does the software match a declared mechanics problem within a declared tolerance? | Mechanics verification | Maintainers, using recorded benchmark evidence |
| Does the workflow preserve assumptions, warnings, reproducibility, and user-facing limitations? | Workflow validation | Maintainers and reviewers |
| Did a user-supplied rule pack run deterministically against its stated inputs? | User rule check | User or project authority |
| Is a real project calculation acceptable for reliance? | Professional reliance context | Competent human reviewer |

Software evidence can support the last question, but it cannot replace the
human review of model scope, inputs, assumptions, rule basis, limitations, and
reporting context.

## 2. Evidence States

Manual entries use these states:

| State | Meaning |
|---|---|
| `PLANNED` | Section exists but no accepted evidence is recorded. |
| `DRAFT_EVIDENCE` | Evidence exists but thresholds, provenance, review, or repeatability remain incomplete. |
| `MAINTAINER_REVIEWED` | Maintainers reviewed the evidence for software-quality use. |
| `BLOCKED` | Required evidence cannot be used until a source, provenance, licensing, or technical issue is resolved. |
| `TBD` | A governed decision or evidence record is still missing. |

Final release thresholds, public benchmark acceptance, release labels, and
professional reliance wording remain `TBD` unless a human governance record says
otherwise. GUI validation evidence requirements and validation evidence-bundle
storage also remain `TBD`.

## 3. Case Index

Each case page assembles the PRD section 16.5 record fields (purpose, input
model, independent reference, expected result, software result, tolerance,
pass/fail, solver version) for one existing validated case, from the
authoritative artifacts already in the tree. Pages are deterministic
renderings produced by
`docs/validation_manual/cases/generate_validation_case_pages.py`; the
hand-calc notes, fixture constructors, suite tests, and governed `DEC-046`
policy records remain the sources of record. All cases are `DRAFT_EVIDENCE`.

### 3.1 Mechanics benchmark cases (`DEL-09-01`)

Suite crate: `validation/benchmarks/mechanics/`. Recorded run 2026-07-10:
`ok. 30 passed; 0 failed` (run record
`execution/PKG-09_Verification, Validation, and Quality Oracles/1_Working/DEL-09-04_Validation manual skeleton/_run_records/WORKING_ITEMS_RUN_2026-07-10_TP-E2-VALMANUAL-001.md`).

Inventory updated 2026-08-09: 64 case pages total (21 mechanics, 15 stress,
28 nonlinear). The DEC-092 row below uses its 2026-08-03 implementation run
and commit-bound evidence; the other mechanics rows retain their historical
2026-07-10 run metadata.

| Case | Independent reference |
|---|---|
| [`MECH-BRANCH-ASSEMBLY-THREE-MEMBER`](cases/mechanics/mech-branch-assembly-three-member.md) | `validation/hand_calcs/mechanics/branch_assembly.md` |
| [`MECH-CANTILEVER-TIP-FORCE`](cases/mechanics/mech-cantilever-tip-force.md) | `validation/hand_calcs/mechanics/cantilever_tip_force.md` |
| [`MECH-EXPANSION-LOOP-CURVED-BEND-THERMAL`](cases/mechanics/mech-expansion-loop-curved-bend-thermal.md) | `validation/hand_calcs/mechanics/expansion_loop_curved_bend_thermal.md` |
| [`MECH-FIXED-FIXED-THERMAL-AXIAL`](cases/mechanics/mech-fixed-fixed-thermal-axial.md) | `validation/hand_calcs/mechanics/fixed_fixed_thermal_axial.md` |
| [`MECH-IMPOSED-DISPLACEMENT-SPRING`](cases/mechanics/mech-imposed-displacement-spring.md) | `validation/hand_calcs/mechanics/imposed_displacement_spring.md` |
| [`MECH-INCLINED-MEMBER-TRANSFORM`](cases/mechanics/mech-inclined-member-transform.md) | `validation/hand_calcs/mechanics/inclined_member_transform.md` |
| [`MECH-PORTAL-SWAY-ORIGINAL`](cases/mechanics/mech-portal-sway-original.md) | `validation/hand_calcs/mechanics/portal_frame_sway.md` |
| [`MECH-PRIMITIVE-LOAD-PREP`](cases/mechanics/mech-primitive-load-prep.md) | `validation/hand_calcs/mechanics/primitive_load_preparation.md` |
| [`MECH-STRAIGHT-PIPE-WEIGHT-RECOVERY`](cases/mechanics/mech-straight-pipe-weight-recovery.md) | `validation/hand_calcs/mechanics/straight_pipe_weight_recovery.md` |
| [`MECH-SUPPORT-BOUNDARY-MIXED`](cases/mechanics/mech-support-boundary-mixed.md) | `validation/hand_calcs/mechanics/support_boundary_mixed.md` |
| [`MECH-TP-PHYS-002-LINEAR-STATIC-INTEGRATION`](cases/mechanics/mech-tp-phys-002-linear-static-integration.md) | `validation/hand_calcs/mechanics/tp_phys_002_linear_static_integration.md` |
| [`MECH-TP-PHYS-004-LOAD-TO-RESULTANT`](cases/mechanics/mech-tp-phys-004-load-to-resultant.md) | `validation/hand_calcs/mechanics/tp_phys_004_load_to_resultant.md` |
| [`MECH-TP-PHYS-005-ORIENTED-LOAD-TO-RESULTANT`](cases/mechanics/mech-tp-phys-005-oriented-load-to-resultant.md) | `validation/hand_calcs/mechanics/tp_phys_005_oriented_load_to_resultant.md` |
| [`MECH-TP-PHYS-006-PARTIAL-SPAN-LOAD-TO-RESULTANT`](cases/mechanics/mech-tp-phys-006-partial-span-load-to-resultant.md) | `validation/hand_calcs/mechanics/tp_phys_006_partial_span_load_to_resultant.md` |
| [`MECH-TP-PHYS-007-STATION-SWEEP-RESULTANTS`](cases/mechanics/mech-tp-phys-007-station-sweep-resultants.md) | `validation/hand_calcs/mechanics/tp_phys_007_station_sweep_resultants.md` |
| [`MECH-TP-PHYS-008-THERMAL-PRESSURE-AXIAL-EFFECTS`](cases/mechanics/mech-tp-phys-008-thermal-pressure-axial-effects.md) | `validation/hand_calcs/mechanics/tp_phys_008_thermal_pressure_axial_effects.md` |
| [`MECH-TP-PHYS-009-COMBINED-LOAD-AXIAL-EFFECTS`](cases/mechanics/mech-tp-phys-009-combined-load-axial-effects.md) | `validation/hand_calcs/mechanics/tp_phys_009_combined_load_axial_effects.md` |
| [`MECH-TP-PHYS-014-CANONICAL-ANALYTICAL-PAYLOAD`](cases/mechanics/mech-tp-phys-014-canonical-analytical-payload.md) | `validation/hand_calcs/mechanics/tp_phys_014_canonical_analytical_payload.md` |
| [`MECH-TP-PHYS-015-CANONICAL-SOLVE-RESULT-ENVELOPE`](cases/mechanics/mech-tp-phys-015-canonical-solve-result-envelope.md) | `validation/hand_calcs/mechanics/tp_phys_015a_canonical_solve_result_envelope.md` |
| [`MECH-TP-DEC092-TEMPERATURE-INDEXED-SHEAR-MODULUS-TORSION`](cases/mechanics/mech-tp-dec092-temperature-indexed-shear-modulus-torsion.md) | `validation/hand_calcs/mechanics/tp_dec092_temperature_indexed_shear_modulus_torsion.md` |
| [`MECH-TP-PMM-P3-OCCLOADGEN-EQUIVALENT-STATIC`](cases/mechanics/mech-tp-pmm-p3-occloadgen-equivalent-static.md) | `validation/hand_calcs/mechanics/tp_pmm_p3_occloadgen_equivalent_static.md` |

### 3.2 Stress recovery cases (`DEL-09-02`)

Suite crate: `validation/benchmarks/stress/`. Recorded run 2026-07-10:
`ok. 22 passed; 0 failed` (same run record).

| Case | Independent reference |
|---|---|
| [`STRESS-AXIAL-NORMAL-ORIGINAL`](cases/stress/stress-axial-normal-original.md) | `validation/hand_calcs/stress/axial_normal.md` |
| [`STRESS-BENDING-NORMAL-ORIGINAL`](cases/stress/stress-bending-normal-original.md) | `validation/hand_calcs/stress/bending_normal.md` |
| [`STRESS-INTEGRATED-STRAIGHT-PIPE-ORIGINAL`](cases/stress/stress-integrated-straight-pipe-original.md) | `validation/hand_calcs/stress/integrated_straight_pipe_resultants.md` |
| [`STRESS-PRESSURE-MEMBRANE-ORIGINAL`](cases/stress/stress-pressure-membrane-original.md) | `validation/hand_calcs/stress/pressure_membrane.md` |
| [`STRESS-RANGE-MECHANICS-ORIGINAL`](cases/stress/stress-range-mechanics-original.md) | `validation/hand_calcs/stress/stress_range.md` |
| [`STRESS-TORSIONAL-SHEAR-ORIGINAL`](cases/stress/stress-torsional-shear-original.md) | `validation/hand_calcs/stress/torsional_shear.md` |
| [`STRESS-TP-PHYS-004-LOAD-TO-RESULTANT`](cases/stress/stress-tp-phys-004-load-to-resultant.md) | `validation/hand_calcs/stress/tp_phys_004_load_to_resultant_stress.md` |
| [`STRESS-TP-PHYS-005-ORIENTED-LOAD-TO-STRESS`](cases/stress/stress-tp-phys-005-oriented-load-to-stress.md) | `validation/hand_calcs/stress/tp_phys_005_oriented_load_to_stress.md` |
| [`STRESS-TP-PHYS-006-PARTIAL-SPAN-LOAD-TO-STRESS`](cases/stress/stress-tp-phys-006-partial-span-load-to-stress.md) | `validation/hand_calcs/stress/tp_phys_006_partial_span_load_to_stress.md` |
| [`STRESS-TP-PHYS-007-STATION-SWEEP-STRESS`](cases/stress/stress-tp-phys-007-station-sweep-stress.md) | `validation/hand_calcs/stress/tp_phys_007_station_sweep_stress.md` |
| [`STRESS-TP-PHYS-008-THERMAL-AXIAL-EFFECT-TO-STRESS`](cases/stress/stress-tp-phys-008-thermal-axial-effect-to-stress.md) | `validation/hand_calcs/stress/tp_phys_008_thermal_axial_effect_to_stress.md` |
| [`STRESS-TP-PHYS-009-COMBINED-AXIAL-BENDING-TO-STRESS`](cases/stress/stress-tp-phys-009-combined-axial-bending-to-stress.md) | `validation/hand_calcs/stress/tp_phys_009_combined_axial_bending_to_stress.md` |
| [`STRESS-TP-PHYS-015-CANONICAL-RESULTANT-STRESS-RECOVERY`](cases/stress/stress-tp-phys-015-canonical-resultant-stress-recovery.md) | `validation/hand_calcs/stress/tp_phys_015_canonical_resultant_stress.md` |
| [`STRESS-TP-PMM-P3-MILLTOL-EFFECTIVE-WALL-STRESS`](cases/stress/stress-tp-pmm-p3-milltol-effective-wall-stress.md) | `validation/hand_calcs/stress/tp_pmm_p3_milltol_effective_wall_stress.md` |
| [`STRESS-TP-PMM-P3-MODULUSBASIS-RANGE-STRESS`](cases/stress/stress-tp-pmm-p3-modulusbasis-range-stress.md) | `validation/hand_calcs/stress/tp_pmm_p3_modulusbasis_range_stress.md` |

### 3.3 Nonlinear support cases (`DEL-09-03`)

Suite crate: `validation/benchmarks/nonlinear/`. Recorded run 2026-07-10:
`ok. 19 passed; 0 failed` (same run record). Convergence acceptance rides the
governed `DEC-046` policy records committed beside the crate
(`*.dec046.json`); `TBD` entries in those records stay `TBD`.

| Case | Independent reference |
|---|---|
| [`NL-ACTIVE-ONE-WAY-ORIGINAL`](cases/nonlinear/nl-active-one-way-original.md) | `validation/hand_calcs/nonlinear/active_set_one_way.md` |
| [`NL-ASSEMBLED-FRICTION-BOUNDED-SLIDE-ORIGINAL`](cases/nonlinear/nl-assembled-friction-bounded-slide-original.md) | `validation/hand_calcs/nonlinear/assembled_friction_bounded_sliding.md` |
| [`NL-ASSEMBLED-FRICTION-DERIVED-NORMAL-ORIGINAL`](cases/nonlinear/nl-assembled-friction-derived-normal-original.md) | `validation/hand_calcs/nonlinear/assembled_friction_derived_normal.md` |
| [`NL-ASSEMBLED-FRICTION-SLIDE-ORIGINAL`](cases/nonlinear/nl-assembled-friction-slide-original.md) | `validation/hand_calcs/nonlinear/assembled_friction_sliding.md` |
| [`NL-ASSEMBLED-FRICTION-STICK-ORIGINAL`](cases/nonlinear/nl-assembled-friction-stick-original.md) | `validation/hand_calcs/nonlinear/assembled_friction_sticking.md` |
| [`NL-ASSEMBLED-GAP-CLOSURE-ORIGINAL`](cases/nonlinear/nl-assembled-gap-closure-original.md) | `validation/hand_calcs/nonlinear/assembled_gap_closure.md` |
| [`NL-ASSEMBLED-GAP-LIFT-OFF-ORIGINAL`](cases/nonlinear/nl-assembled-gap-lift-off-original.md) | `validation/hand_calcs/nonlinear/assembled_gap_lift_off.md` |
| [`NL-ASSEMBLED-LIFT-OFF-ORIGINAL`](cases/nonlinear/nl-assembled-lift-off-original.md) | `validation/hand_calcs/nonlinear/assembled_lift_off.md` |
| [`NL-ASSEMBLED-MULTI-DOF-CASCADE-GAP-LIFT-OFF-ACCEPTED-ORIGINAL`](cases/nonlinear/nl-assembled-multi-dof-cascade-gap-lift-off-accepted-original.md) | `validation/hand_calcs/nonlinear/assembled_multi_support_cascade_gap_lift_off_acceptance.md` |
| [`NL-ASSEMBLED-MULTI-DOF-DERIVED-NORMAL-GAP-ACCEPTED-ORIGINAL`](cases/nonlinear/nl-assembled-multi-dof-derived-normal-gap-accepted-original.md) | `validation/hand_calcs/nonlinear/assembled_multi_support_derived_normal_gap_acceptance.md` |
| [`NL-ASSEMBLED-MULTI-DOF-DERIVED-NORMAL-ROTATIONAL-ACCEPTED-ORIGINAL`](cases/nonlinear/nl-assembled-multi-dof-derived-normal-rotational-accepted-original.md) | `validation/hand_calcs/nonlinear/assembled_multi_support_derived_normal_rotational_acceptance.md` |
| [`NL-ASSEMBLED-MULTI-DOF-FOUR-CLASS-ACCEPTED-ORIGINAL`](cases/nonlinear/nl-assembled-multi-dof-four-class-accepted-original.md) | `validation/hand_calcs/nonlinear/assembled_multi_support_four_class_acceptance.md` |
| [`NL-ASSEMBLED-MULTI-DOF-FRICTION-GAP-ACCEPTED-ORIGINAL`](cases/nonlinear/nl-assembled-multi-dof-friction-gap-accepted-original.md) | `validation/hand_calcs/nonlinear/assembled_multi_support_friction_gap_acceptance.md` |
| [`NL-ASSEMBLED-MULTI-DOF-GAP-LIFT-OFF-ACCEPTED-ORIGINAL`](cases/nonlinear/nl-assembled-multi-dof-gap-lift-off-accepted-original.md) | `validation/hand_calcs/nonlinear/assembled_multi_support_gap_lift_off_acceptance.md` |
| [`NL-ASSEMBLED-MULTI-DOF-MULTI-SUPPORT-ACCEPTED-ORIGINAL`](cases/nonlinear/nl-assembled-multi-dof-multi-support-accepted-original.md) | `validation/hand_calcs/nonlinear/assembled_multi_support_multi_dof_acceptance.md` |
| [`NL-ASSEMBLED-MULTI-DOF-MULTI-SUPPORT-OBS-ORIGINAL`](cases/nonlinear/nl-assembled-multi-dof-multi-support-obs-original.md) | `validation/hand_calcs/nonlinear/assembled_multi_support_multi_dof.md` |
| [`NL-ASSEMBLED-MULTI-DOF-NEGATIVE-GAP-ACCEPTED-ORIGINAL`](cases/nonlinear/nl-assembled-multi-dof-negative-gap-accepted-original.md) | `validation/hand_calcs/nonlinear/assembled_multi_support_negative_gap_acceptance.md` |
| [`NL-ASSEMBLED-MULTI-DOF-OPPOSING-GAPS-ACCEPTED-ORIGINAL`](cases/nonlinear/nl-assembled-multi-dof-opposing-gaps-accepted-original.md) | `validation/hand_calcs/nonlinear/assembled_multi_support_opposing_gaps_acceptance.md` |
| [`NL-ASSEMBLED-MULTI-DOF-ROTATIONAL-ACCEPTED-ORIGINAL`](cases/nonlinear/nl-assembled-multi-dof-rotational-accepted-original.md) | `validation/hand_calcs/nonlinear/assembled_multi_support_rotational_acceptance.md` |
| [`NL-ASSEMBLED-MULTI-DOF-THREE-SUPPORT-ACCEPTED-ORIGINAL`](cases/nonlinear/nl-assembled-multi-dof-three-support-accepted-original.md) | `validation/hand_calcs/nonlinear/assembled_multi_support_three_dof_acceptance.md` |
| [`NL-ASSEMBLED-MULTI-DOF-TWO-SPAN-ACCEPTED-ORIGINAL`](cases/nonlinear/nl-assembled-multi-dof-two-span-accepted-original.md) | `validation/hand_calcs/nonlinear/assembled_multi_support_two_span_acceptance.md` |
| [`NL-ASSEMBLED-MULTI-DOF-TWO-SPAN-OPPOSING-GAPS-ACCEPTED-ORIGINAL`](cases/nonlinear/nl-assembled-multi-dof-two-span-opposing-gaps-accepted-original.md) | `validation/hand_calcs/nonlinear/assembled_multi_support_two_span_opposing_gaps_acceptance.md` |
| [`NL-ASSEMBLED-ONE-WAY-DEACTIVATE-ORIGINAL`](cases/nonlinear/nl-assembled-one-way-deactivate-original.md) | `validation/hand_calcs/nonlinear/assembled_one_way_deactivation.md` |
| [`NL-ASSEMBLED-ONE-WAY-REENGAGE-ORIGINAL`](cases/nonlinear/nl-assembled-one-way-reengage-original.md) | `validation/hand_calcs/nonlinear/assembled_one_way_reengagement.md` |
| [`NL-FRICTION-STICK-SLIDE-ORIGINAL`](cases/nonlinear/nl-friction-stick-slide-original.md) | `validation/hand_calcs/nonlinear/friction_transition.md` |
| [`NL-GAP-CLOSURE-ORIGINAL`](cases/nonlinear/nl-gap-closure-original.md) | `validation/hand_calcs/nonlinear/gap_closure.md` |
| [`NL-LIFT-OFF-ORIGINAL`](cases/nonlinear/nl-lift-off-original.md) | `validation/hand_calcs/nonlinear/lift_off.md` |
| [`NL-NONCONVERGENCE-LIMIT-ORIGINAL`](cases/nonlinear/nl-nonconvergence-limit-original.md) | `validation/hand_calcs/nonlinear/unresolved_nonconvergence.md` |

### 3.4 Reproduction slices

| Slice | Evidence state | Purpose |
|---|---|---|
| [Headless runner reproduction](headless_runner_reproduction.md) | `DRAFT_EVIDENCE` | Records the current external-reproducibility path for the invented `openpipestress-runner` examples produced by `TP-RUNNER-015` under `DEC-065`. |

## 4. Manual Section Map

| Section | Evidence class | Current source slots | Required content |
|---|---|---|---|
| Product scope and limitations | Professional reliance context | `docs/PROFESSIONAL_BOUNDARY.md`, `docs/IP_AND_DATA_BOUNDARY.md` | Intended support role, exclusions, private-data boundary, reliance warning, known unsupported cases |
| Solver theory summary | Verification context | `docs/theory/centerline_analysis.md`, solver docs | Stated mechanics assumptions, element families, load handling, nonlinear support limits, result-envelope policy |
| Unit and schema verification | Mechanics verification | schema tests, `core/units/`, `docs/TYPES.md` | Unit dimensions, schema validation, invalid-input behavior, serialization boundaries |
| Element verification | Mechanics verification | [section 3.1 cases](#31-mechanics-benchmark-cases-del-09-01) | Frame, straight-pipe, support, thermal, imposed displacement, curved-bend, and transform benchmarks |
| Load and stress recovery verification | Mechanics verification | [section 3.2 cases](#32-stress-recovery-cases-del-09-02) | Axial, bending, torsion, pressure membrane, stress range, load-case algebra, documented tolerances |
| Nonlinear support verification | Mechanics verification | [section 3.3 cases](#33-nonlinear-support-cases-del-09-03) | Active-set behavior, gap/lift-off/friction cases, convergence and non-convergence diagnostics |
| Rule-pack evaluator verification | User rule check | rule schemas, evaluator tests, invented rule packs | Required inputs, unit awareness, sandboxing, deterministic pass/fail status, checksum/provenance |
| GUI workflow validation | Workflow validation | GUI workflow tests and screenshots when available | Missing-data behavior, warning visibility, assumptions, solve status, result state transitions |
| Report reproducibility validation | Workflow validation | report generator, audit manifest, protected-content linter | Stable output, checksums, warning inclusion, provenance disclosure, professional-boundary notice |
| Known limitations and open issues | Cross-cutting | issue records, release notes, `TBD` queue | Missing evidence, unapproved thresholds, source restrictions, model limitations, accepted risks |

## 5. Current Evidence Inventory

| Evidence area | Current repository surface | Manual status |
|---|---|---|
| Mechanics benchmarks | [Section 3.1 case pages](#31-mechanics-benchmark-cases-del-09-01); `validation/benchmarks/mechanics/`, `validation/hand_calcs/mechanics/` | `DRAFT_EVIDENCE`; final public benchmark acceptance and release tolerances are `TBD`. |
| Stress recovery benchmarks | [Section 3.2 case pages](#32-stress-recovery-cases-del-09-02); `validation/benchmarks/stress/`, `validation/hand_calcs/stress/` | `DRAFT_EVIDENCE`; fatigue, allowable, and release-threshold decisions are `TBD`. |
| Nonlinear support regression | [Section 3.3 case pages](#33-nonlinear-support-cases-del-09-03); `validation/benchmarks/nonlinear/` with governed `DEC-046` policy records | `DRAFT_EVIDENCE`; production release thresholds and external validation claims are `TBD`. |
| Headless runner reproduction | `docs/validation_manual/headless_runner_reproduction.md`; `validation/witness/inputs/`; `validation/witness/generated/` | `DRAFT_EVIDENCE`; the current DEC-065 local CLI examples and the committed DEL-10-05 `run-benchmark`/`run-regression` payload families are reproducible from invented fixtures. `export-results` remains the only structured runner stub, and public benchmark thresholds remain `TBD`. |
| GUI workflow validation | GUI workflow tests, screenshots, and interaction evidence when available | `PLANNED`; required evidence type, coverage, and review criteria are `TBD`. |
| Report protected-content lint | `core/reporting/protected_content_linter/` | Draft review evidence for report/public-artifact checks; development review evidence, not legal clearance, and acceptance stays with the responsible engineer. |
| Release quality evidence | Future `DEL-09-05` release quality gate checklist | `TBD`; this manual may organize evidence inputs but does not settle release labels or release readiness. |
| Validation evidence bundles | Future evidence package location/format | `TBD`; long-term storage, retention, and release attachment policy are unsettled. |
| Professional boundary | `docs/PROFESSIONAL_BOUNDARY.md` | Draft policy surface for reliance wording and release-claim limits. |

This inventory is the starting index for future evidence review and
release-quality decisions. Validation occurs in the user's accepted
professional tools; this package is screening and handoff evidence.

## 6. Section Requirements

Each manual section must include:

- purpose and evidence class;
- source artifacts and commands used to generate evidence;
- expected behavior and comparison basis;
- tolerance or threshold source, or `TBD` if not governed;
- provenance and redistribution status for public examples;
- known limitations and excluded interpretations;
- review disposition and open risks.

## 7. Hand-Calc Witness Evidence

A formal hand-calc witness is a machine-readable validation evidence artifact.
It records the problem statement, assumptions, units, symbolic relations,
numeric inputs, intermediate quantities, expected results, tolerances or `TBD`
tolerance markers, provenance, redistribution status, and review disposition.
The machine-readable artifact is authoritative for automated comparison,
hashing, reproducibility, and review traceability.

Markdown, MathML, PDF, or other human-facing hand-calc pages are deterministic
renderings generated from the witness artifact. They support inspection and
review, but they are not the source of record when they disagree with the
machine-readable witness.

Hand-calc witnesses must remain limited to open mechanics, invented or cleared
public evidence, and user-supplied private data where applicable. They must not
embed protected standards text, protected tables, proprietary benchmark files,
private rule-pack content, code-specific acceptance criteria, or any statement
that the software certifies, seals, approves, authenticates, or declares code
compliance for project reliance.

## 8. Provenance Rules

Public validation examples must be original, public-domain, or permissively
licensed. Public artifacts must not copy protected standards text, protected
tables, protected examples, proprietary commercial benchmark files, private
owner data, private user models, or vendor data without documented rights.

Private rule packs, private material libraries, owner requirements, and project
models belong in user-controlled project records. They may be referenced in a
private validation package, but they must not be copied into public
OpenPipeStress artifacts.

## 9. Manual Review Checklist

Before a manual section is treated as maintainer-reviewed software evidence,
reviewers check that:

- evidence commands and inputs are recorded;
- expected results are reproducible from repository artifacts or documented
  external sources with redistribution rights;
- generated hand-calc renderings trace back to their authoritative
  machine-readable witness artifacts;
- units and assumptions are explicit;
- warnings, non-convergence, and missing-data behavior are not collapsed into a
  generic success state;
- protected-content and private-data scans are clean or risk-dispositioned;
- professional reliance wording stays human-owned;
- unresolved gaps remain visible as `TBD` or open issues.

## 10. Open Decisions

- TBD: final benchmark tolerance policy for mechanics, stress recovery, and
  nonlinear support evidence (the governed `DEC-046` convergence records for
  the current assembled validation seed and accepted multi-support set exist;
  the `DEC-024`/`DEC-026` per-quantity-kind verification tolerance record
  remains unfilled).
- TBD: public benchmark source acceptance process and reviewer roster.
- TBD: release-label policy beyond the minimum validation strategy gate.
- TBD: required GUI validation evidence once the GUI tranche matures.
- TBD: long-term storage format for reviewed validation evidence bundles.
- TBD: additional `openpipestress-runner` benchmark/regression payload coverage
  beyond the exact committed DEL-10-05 witness families, `export-results`
  payload binding (the only remaining structured runner stub), a clean
  environment demonstration record, and the human-gated release-quality and
  professional-boundary reviews required at the R5 exit.
