# MAINTAINER_REVIEWED Case-Page Promotion-Basis Packet

## 1. Identity and conclusion

| Field | Value |
|---|---|
| Run | `HELP-HUMAN-PIPING-20260810-DEL0904-OWNER-GATES` |
| Child | `A2-MAINTAINER-REVIEW` |
| Parent | `WORKING_ITEMS/Agent1/working_items_del0904_owner_gates_prepare` |
| Frozen repository base | `c05fe2d6fbc3bd3d3b690f50075e2c878af0faf3` |
| Scope | `DEL-09-04` case-page `DRAFT_EVIDENCE` → `MAINTAINER_REVIEWED` promotion basis only |
| Candidate count | 64 case pages: 21 mechanics, 15 stress, 28 nonlinear |
| Existing qualifying maintainer-review records | 0 |
| Exact promotion slate supportable now | **None** |
| Required result | A bounded review-work plan, not a promotion act |

**Conclusion:** all 64 generated case pages are candidates for review, but none
is presently eligible for promotion. Passing tests, deterministic generation,
agent checks, deliverable compatibility review, and file existence are not a
maintainer review act. No page-specific owner/maintainer disposition is
recorded. In addition, the required trace from each human-facing hand-calc note
to an authoritative machine-readable witness has not been established by a
review record, and 63 legacy pages carry a now-superseded statement that
`run-benchmark` remains a structured stub. The lawful output is therefore the
review-work plan in §7. Preparing or adopting that plan promotes no page.

## 2. Controlling definitions and authority boundary

All paths below are relative to `projects/chirality-piping/` and all Git blob
IDs are at the frozen base.

1. `docs/validation_manual/index.md` (blob
   `71984004dc6976d9d4f28c647b5b988ca21e210b`, SHA-256
   `acdfbc55afa00e5b644f424df7aa551fea8ecd237f0a0d0f31f52dd17ec86e9d`):
   - §2 defines `DRAFT_EVIDENCE` as evidence for which thresholds,
     provenance, review, or repeatability remain incomplete.
   - §2 defines `MAINTAINER_REVIEWED` only as “Maintainers reviewed the
     evidence for software-quality use.” It is not release, validation for
     reliance, professional acceptance, or public-source legal clearance.
   - §3 says case pages are deterministic renderings; hand-calc notes,
     fixture constructors, suite tests, and governed policy records remain
     sources of record; every current case is `DRAFT_EVIDENCE`.
   - §9 requires the reviewer to check recorded commands/inputs,
     reproducible expected results, machine-readable-witness traceability,
     explicit units/assumptions, truthful warning/non-convergence/missing-data
     states, protected/private-data disposition, human-owned reliance
     language, and visible unresolved gaps.
   - §10 still marks the public-benchmark source acceptance process and
     reviewer roster `TBD`.
2. `docs/VALIDATION_STRATEGY.md` (blob
   `56e4edc1132c854ebd514313f4bc8c4f969cd886`):
   - §§2 and 5 require a machine-readable hand-calc witness to be the source
     of record and human-facing Markdown to be a deterministic rendering.
   - §5 requires each public fixture/witness to record provenance,
     redistribution status, assumptions, units, expected results, tolerance
     basis (or `TBD`), and a review disposition before it is treated as
     accepted public validation evidence.
   - §5 says source/provenance records remain review evidence, not legal
     clearance or final public-benchmark acceptance, while the public-source
     acceptance workflow is `TBD`.
3. `docs/claims_registry.md` (blob
   `d933bce7d6870c2f63981648d24ff0f615d15a40`) §2 defines
   `INTERNALLY_VERIFIED`, `PROVER_CORRELATED`, and `ENGINEER_ACCEPTED`; none is
   a synonym for `MAINTAINER_REVIEWED`. BS-VALID keeps internal benchmarks as
   development verification/screening evidence.
4. `execution/_Decomposition/SOFTWARE_DECOMP.md` (blob
   `a64b02b77248c26d3d17987624131a35a5acbb71`) `DEC-027` records that the
   sole human project authority is the sole developer, maintainer, and release
   authority. Therefore the promotion act belongs to the owner/sole
   maintainer; an agent may prepare and validate the basis but cannot perform
   or fabricate the review act.
5. `execution/_Coordination/_DECISIONS/D-46_r5_external_reproduction_acceptance_protocol.md`
   (blob `e9322ad058e1e126113c4a99e35940986cecde95`) is ruled O-C history and is
   expressly superseded-in-question by `DEC-080`. It supplies no current
   promotion act. `DEC-080` made the PRD §24 R6 reproduction criterion
   actor-neutral; it did not promote case pages or create a page-review record.
6. The R14 gate analysis at
   `execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260719-MECHANICS-CAMPAIGN-R14/instances/W3/RETURN.md`
   (blob `18a2c679949e6dd36240b955b9aaa37e34d16405`) correctly classifies promotion
   as an owner-gated maintainer review act. It is gate analysis, not the act.

### What promotion would and would not mean

A valid promotion would say only that the owner/sole maintainer reviewed the
exact hash-bound evidence for software-quality use and recorded a per-page
disposition. It may leave a tolerance or other gap visibly `TBD` if the review
record expressly dispositions that gap and the page remains truthful. It does
not establish final public-benchmark acceptance, legal clearance, external-
prover correlation, `ENGINEER_ACCEPTED`, professional reliance, release
readiness, lifecycle closure, or PRD exit satisfaction.

## 3. Generator and page-schema behavior

`docs/validation_manual/cases/generate_validation_case_pages.py` (blob
`ae9969be6849770bef4468249e6c065d940ae955`, SHA-256
`baf2e729d91ccfa46dcb406c2e975ee76e0d4b0ce909259adb891d5e0907ff09`)
registers exactly 64 cases and deterministically renders each page. The current
template hard-codes both frontmatter `status: draft_evidence` and the record
row `Evidence state | DRAFT_EVIDENCE`. It has no review-record field and no
governed promotion map. `--check` at the frozen base reports:

```text
checked 64 case page(s) under .../docs/validation_manual/cases
```

The generator does not generate `docs/validation_manual/index.md`; the index is
a separately maintained surface. A deterministic reconciliation found:

| Surface | Total | Mechanics | Stress | Nonlinear |
|---|---:|---:|---:|---:|
| Generator registrations | 64 | 21 | 15 | 28 |
| Generated case-page files | 64 | 21 | 15 | 28 |
| Unique index case links | 64 | 21 | 15 | 28 |

There are no registered-but-missing pages, unregistered case-page files,
missing index targets, unindexed case pages, or duplicate index links.
`docs/validation_manual/headless_runner_reproduction.md` is a separate
reproduction slice, not a `governance.validation_manual_case`, and is not in
this 64-page promotion inventory.

### Common source/evidence blobs

| Code | Existing evidence (not a review act) |
|---|---|
| `E63` | Exact page and hand-calc blobs in §5; fixture constructors and named tests in suite source — mechanics `eb65e53075110995a4ddcd93b4181b15392f91d5`, stress `201f5d84d1a666975000d07fb8e21900b88f9807`, nonlinear `37d19abf27a17c1d1333a81732d0b413e4ca2880`; TP-E2 run record `97c07ef9b52fc36e426a503d3c52bfde9a7929a2`, recording 2026-07-10 suite results mechanics 30/30, stress 22/22, nonlinear 19/19 and generator determinism. Each page itself names its exact tests/commands. |
| `E92` | Exact page/note blobs in §5; mechanics source `eb65e53075110995a4ddcd93b4181b15392f91d5`; product-physics source `9116f3ec59108dcf6833ee04dea39b67e91bcede`; implementation run record `b9d9fc9e59a8c23ac8bee131892c565304853715`; committed sweep `fdfba42c4ae3da3b50e0120a7ccf4e8c47437b83`; implementation commit `c394365ca72b8383c7d7203ce5be2cb9ea67d508`. |

The deliverable `_REVIEW.md` (blob
`c7c997d487a5222b9daae789e69ccb8e83ab2bca`) is a 2026-05 compatibility and
lifecycle-readiness review that predates the case pages. `Review_Findings.csv`
(blob `fb92cc1d66426aa512253e68c2fd259552f9623c`) contains only its header.
Neither is page-specific review evidence. TP-E2 and later R13/R14/DEC-092 run
records expressly preserve the promotion gate. Repository-wide searches found
no owner/maintainer page-review disposition. Review status is therefore
`NONE` for every row below.

## 4. Missing-basis codes and corpus-level discrepancies

| Code | Missing or contradictory basis |
|---|---|
| `M1` | No owner/sole-maintainer review act or durable page-specific disposition binds the exact page, note, fixture/test source, tolerance basis, evidence run, checklist results, limitations, and hashes. |
| `M2` | The required machine-readable-witness chain is not established. Current pages point to Markdown hand-calc notes plus fixture constructors. The notes contain useful derivations/provenance, but no review record demonstrates that each is a deterministic rendering of an authoritative machine-readable witness as required by `VALIDATION_STRATEGY.md` §§2/5 and manual §9. Unknown provenance is not inferred. |
| `M3` | Every legacy page except the DEC-092 page repeats the generator's statement that `run-benchmark` still exits with `HEADLESS_RUNNER_OPERATION_STUB_REQUIRES_DOWNSTREAM_PAYLOAD` and that binding is future DEL-10-05 work. DEL-09-04 `_STATUS.md` records those bindings landed via PR #287/Receipt 59 (only `export-results` remains a DEL-10-05 stub), and the R13 headless page records bound behavior. The statement is superseded and must be corrected or explicitly dispositioned before review promotion. This affects 63 pages. |

Two related index statements are also superseded: §5 says benchmark/regression
payload bindings remain stubbed, and §10 repeats that both verbs remain
structured stubs. They do not change the candidate count but must be corrected
in the same review-preparation tranche so the aggregate and page surfaces agree.

## 5. Exhaustive candidate inventory

Each row supplies the exact page path/blob, independent-reference path/blob,
current tier, existing evidence code, review evidence, missing basis, and
eligibility conclusion. Suite identity is explicit in the subsection and page
path. `NO` means “not eligible on current evidence,” not a permanent decline.

### 5.1 Mechanics — 21 candidates

| Case | Page / Git blob | Independent reference / Git blob | Tier | Evidence | Review | Missing | Eligible now |
|---|---|---|---|---|---|---|---|
| `MECH-BRANCH-ASSEMBLY-THREE-MEMBER` | `docs/validation_manual/cases/mechanics/mech-branch-assembly-three-member.md`<br>`e5274d9ef2d351bf78c15d65631efa8c61a12293` | `validation/hand_calcs/mechanics/branch_assembly.md`<br>`fa9d9224e64f8559461ef501a4436722ffdf4fa3` | `DRAFT_EVIDENCE` | `E63` | `NONE` | `M1+M2+M3` | `NO` |
| `MECH-CANTILEVER-TIP-FORCE` | `docs/validation_manual/cases/mechanics/mech-cantilever-tip-force.md`<br>`c33e143804e1ad0de2d66b499e582463c144f776` | `validation/hand_calcs/mechanics/cantilever_tip_force.md`<br>`a3b3d3adae3f94523db00d09af42789691b83d0c` | `DRAFT_EVIDENCE` | `E63` | `NONE` | `M1+M2+M3` | `NO` |
| `MECH-EXPANSION-LOOP-CURVED-BEND-THERMAL` | `docs/validation_manual/cases/mechanics/mech-expansion-loop-curved-bend-thermal.md`<br>`c373e8bb1a52b9c3c5fcb45ebe4b1c149db1cbb3` | `validation/hand_calcs/mechanics/expansion_loop_curved_bend_thermal.md`<br>`31b8c9852214f390acea2fa293d551e288be644a` | `DRAFT_EVIDENCE` | `E63` | `NONE` | `M1+M2+M3` | `NO` |
| `MECH-FIXED-FIXED-THERMAL-AXIAL` | `docs/validation_manual/cases/mechanics/mech-fixed-fixed-thermal-axial.md`<br>`f226ab895a464955b899119b3ffb4d91a60c753e` | `validation/hand_calcs/mechanics/fixed_fixed_thermal_axial.md`<br>`050edf4da8ba0bdbcdd807f835ccc9c4d64a4dda` | `DRAFT_EVIDENCE` | `E63` | `NONE` | `M1+M2+M3` | `NO` |
| `MECH-IMPOSED-DISPLACEMENT-SPRING` | `docs/validation_manual/cases/mechanics/mech-imposed-displacement-spring.md`<br>`0e554355868e3a5040291c1b6284be2608ed8933` | `validation/hand_calcs/mechanics/imposed_displacement_spring.md`<br>`6f93346bee02967461267c56cf90c890d028742f` | `DRAFT_EVIDENCE` | `E63` | `NONE` | `M1+M2+M3` | `NO` |
| `MECH-INCLINED-MEMBER-TRANSFORM` | `docs/validation_manual/cases/mechanics/mech-inclined-member-transform.md`<br>`1437f6708f8085480be42f3bb637d8bc19ca1c04` | `validation/hand_calcs/mechanics/inclined_member_transform.md`<br>`d7ca7c39f4a8b82883b21b4166d9f3e40a5b6f41` | `DRAFT_EVIDENCE` | `E63` | `NONE` | `M1+M2+M3` | `NO` |
| `MECH-PORTAL-SWAY-ORIGINAL` | `docs/validation_manual/cases/mechanics/mech-portal-sway-original.md`<br>`dc5b0c65b507d1fbfb891b475944ed17d3169a3f` | `validation/hand_calcs/mechanics/portal_frame_sway.md`<br>`69704c5e4218aaad0c37ec96981c5ee29b3c60f1` | `DRAFT_EVIDENCE` | `E63` | `NONE` | `M1+M2+M3` | `NO` |
| `MECH-PRIMITIVE-LOAD-PREP` | `docs/validation_manual/cases/mechanics/mech-primitive-load-prep.md`<br>`fbcc708bef4ea958a238d4d2b712d1dccd595eff` | `validation/hand_calcs/mechanics/primitive_load_preparation.md`<br>`7c0d32a1d2a849d32da421c5c1488415e1ea9660` | `DRAFT_EVIDENCE` | `E63` | `NONE` | `M1+M2+M3` | `NO` |
| `MECH-STRAIGHT-PIPE-WEIGHT-RECOVERY` | `docs/validation_manual/cases/mechanics/mech-straight-pipe-weight-recovery.md`<br>`7dfabdce9f98b09c09e19ba7f420cd1624cb613d` | `validation/hand_calcs/mechanics/straight_pipe_weight_recovery.md`<br>`27b960470f06fbf4779e46e4211859e92e5ebd3f` | `DRAFT_EVIDENCE` | `E63` | `NONE` | `M1+M2+M3` | `NO` |
| `MECH-SUPPORT-BOUNDARY-MIXED` | `docs/validation_manual/cases/mechanics/mech-support-boundary-mixed.md`<br>`09eedabb83fa44f5f72d14de1fbcfc8dc4a6a35c` | `validation/hand_calcs/mechanics/support_boundary_mixed.md`<br>`5594c0c200c8789f938b50a0fd60a6b5c956818d` | `DRAFT_EVIDENCE` | `E63` | `NONE` | `M1+M2+M3` | `NO` |
| `MECH-TP-DEC092-TEMPERATURE-INDEXED-SHEAR-MODULUS-TORSION` | `docs/validation_manual/cases/mechanics/mech-tp-dec092-temperature-indexed-shear-modulus-torsion.md`<br>`edcb6e07de0d369a190f04318fd89162772b3f45` | `validation/hand_calcs/mechanics/tp_dec092_temperature_indexed_shear_modulus_torsion.md`<br>`7c14d5160943bea96c802252b11e9b4ccb42fdcd` | `DRAFT_EVIDENCE` | `E92` | `NONE` | `M1+M2` | `NO` |
| `MECH-TP-PHYS-002-LINEAR-STATIC-INTEGRATION` | `docs/validation_manual/cases/mechanics/mech-tp-phys-002-linear-static-integration.md`<br>`776928356ed132049f8f1eabd7a9449938540610` | `validation/hand_calcs/mechanics/tp_phys_002_linear_static_integration.md`<br>`3ba86e77aeb3e33a8db1f5b7d5fb8c562353cd8a` | `DRAFT_EVIDENCE` | `E63` | `NONE` | `M1+M2+M3` | `NO` |
| `MECH-TP-PHYS-004-LOAD-TO-RESULTANT` | `docs/validation_manual/cases/mechanics/mech-tp-phys-004-load-to-resultant.md`<br>`63a4687e97d5cee67e939ad8d9b2075ae23bf198` | `validation/hand_calcs/mechanics/tp_phys_004_load_to_resultant.md`<br>`71a5493bda510dbe4f68cac4f45010ffd4831344` | `DRAFT_EVIDENCE` | `E63` | `NONE` | `M1+M2+M3` | `NO` |
| `MECH-TP-PHYS-005-ORIENTED-LOAD-TO-RESULTANT` | `docs/validation_manual/cases/mechanics/mech-tp-phys-005-oriented-load-to-resultant.md`<br>`27426310c46fa3b1d7a92df2a45704781e6e04ba` | `validation/hand_calcs/mechanics/tp_phys_005_oriented_load_to_resultant.md`<br>`f7bdba8f8fab3bf430547a43e43eb0ea0f863bb9` | `DRAFT_EVIDENCE` | `E63` | `NONE` | `M1+M2+M3` | `NO` |
| `MECH-TP-PHYS-006-PARTIAL-SPAN-LOAD-TO-RESULTANT` | `docs/validation_manual/cases/mechanics/mech-tp-phys-006-partial-span-load-to-resultant.md`<br>`8ae1ea5092a941e4dc2a5aef67b1cf8a938539cf` | `validation/hand_calcs/mechanics/tp_phys_006_partial_span_load_to_resultant.md`<br>`8fc04dea420114074cfb89e48fb5af44b637cb18` | `DRAFT_EVIDENCE` | `E63` | `NONE` | `M1+M2+M3` | `NO` |
| `MECH-TP-PHYS-007-STATION-SWEEP-RESULTANTS` | `docs/validation_manual/cases/mechanics/mech-tp-phys-007-station-sweep-resultants.md`<br>`0f39e87b1a2b53e18bb5a769111c25cdb860a11f` | `validation/hand_calcs/mechanics/tp_phys_007_station_sweep_resultants.md`<br>`1478ddb4c9f2473b0ec0ead16d958c58998cb643` | `DRAFT_EVIDENCE` | `E63` | `NONE` | `M1+M2+M3` | `NO` |
| `MECH-TP-PHYS-008-THERMAL-PRESSURE-AXIAL-EFFECTS` | `docs/validation_manual/cases/mechanics/mech-tp-phys-008-thermal-pressure-axial-effects.md`<br>`64c7b21ba1f4a90733546ba917d85ae0d44373b6` | `validation/hand_calcs/mechanics/tp_phys_008_thermal_pressure_axial_effects.md`<br>`07f3f46bce7582c5ece3553552542f1357be3ecd` | `DRAFT_EVIDENCE` | `E63` | `NONE` | `M1+M2+M3` | `NO` |
| `MECH-TP-PHYS-009-COMBINED-LOAD-AXIAL-EFFECTS` | `docs/validation_manual/cases/mechanics/mech-tp-phys-009-combined-load-axial-effects.md`<br>`8e984b2a74df102c024f2439b06eefd7538000d5` | `validation/hand_calcs/mechanics/tp_phys_009_combined_load_axial_effects.md`<br>`d5abfafc8dbb17f91f5877aa1b3a19438adee658` | `DRAFT_EVIDENCE` | `E63` | `NONE` | `M1+M2+M3` | `NO` |
| `MECH-TP-PHYS-014-CANONICAL-ANALYTICAL-PAYLOAD` | `docs/validation_manual/cases/mechanics/mech-tp-phys-014-canonical-analytical-payload.md`<br>`10ece2d5ae08be0c46ed135d992bddfcb168c909` | `validation/hand_calcs/mechanics/tp_phys_014_canonical_analytical_payload.md`<br>`08e495009c2c81aaaddff1bd1559a1ac17425e07` | `DRAFT_EVIDENCE` | `E63` | `NONE` | `M1+M2+M3` | `NO` |
| `MECH-TP-PHYS-015-CANONICAL-SOLVE-RESULT-ENVELOPE` | `docs/validation_manual/cases/mechanics/mech-tp-phys-015-canonical-solve-result-envelope.md`<br>`dc1dede29871aed7a08309d8c8cee890abd86064` | `validation/hand_calcs/mechanics/tp_phys_015a_canonical_solve_result_envelope.md`<br>`a7ff03c90d4409a4e2f34c4526a6cfad4049495e` | `DRAFT_EVIDENCE` | `E63` | `NONE` | `M1+M2+M3` | `NO` |
| `MECH-TP-PMM-P3-OCCLOADGEN-EQUIVALENT-STATIC` | `docs/validation_manual/cases/mechanics/mech-tp-pmm-p3-occloadgen-equivalent-static.md`<br>`b3509feafd5278323063b5ed61b2b5b69f2b7cc1` | `validation/hand_calcs/mechanics/tp_pmm_p3_occloadgen_equivalent_static.md`<br>`8fa728cd5eeff89120e7b0c62ef4000d3d5421f7` | `DRAFT_EVIDENCE` | `E63` | `NONE` | `M1+M2+M3` | `NO` |

### 5.2 Stress — 15 candidates

| Case | Page / Git blob | Independent reference / Git blob | Tier | Evidence | Review | Missing | Eligible now |
|---|---|---|---|---|---|---|---|
| `STRESS-AXIAL-NORMAL-ORIGINAL` | `docs/validation_manual/cases/stress/stress-axial-normal-original.md`<br>`a6aff91f2d35eeb77734f46727b4c7ddfd56eb25` | `validation/hand_calcs/stress/axial_normal.md`<br>`e240d29dda92751fd3f0638dc2d845a72c920c05` | `DRAFT_EVIDENCE` | `E63` | `NONE` | `M1+M2+M3` | `NO` |
| `STRESS-BENDING-NORMAL-ORIGINAL` | `docs/validation_manual/cases/stress/stress-bending-normal-original.md`<br>`124499112ca5dd18a3ce686ce107a032756ba154` | `validation/hand_calcs/stress/bending_normal.md`<br>`05cece7e77a4db4ad887c66bbddd522985d60c34` | `DRAFT_EVIDENCE` | `E63` | `NONE` | `M1+M2+M3` | `NO` |
| `STRESS-INTEGRATED-STRAIGHT-PIPE-ORIGINAL` | `docs/validation_manual/cases/stress/stress-integrated-straight-pipe-original.md`<br>`f57db5fdb07a8590c525ab712bc329701cac1ee2` | `validation/hand_calcs/stress/integrated_straight_pipe_resultants.md`<br>`c44c605421d1a4aeaa14d2274e51c5f4b487af82` | `DRAFT_EVIDENCE` | `E63` | `NONE` | `M1+M2+M3` | `NO` |
| `STRESS-PRESSURE-MEMBRANE-ORIGINAL` | `docs/validation_manual/cases/stress/stress-pressure-membrane-original.md`<br>`aa73f117befd128afeae2ad423b49b9738bea038` | `validation/hand_calcs/stress/pressure_membrane.md`<br>`1b72edb8801e13640afb7cabae6e065787586cba` | `DRAFT_EVIDENCE` | `E63` | `NONE` | `M1+M2+M3` | `NO` |
| `STRESS-RANGE-MECHANICS-ORIGINAL` | `docs/validation_manual/cases/stress/stress-range-mechanics-original.md`<br>`2caea8122bcdebe1ade4993517805ee7eebb4f61` | `validation/hand_calcs/stress/stress_range.md`<br>`3bf7be028f7b94f87af2b6b688bbdc33e280b3c2` | `DRAFT_EVIDENCE` | `E63` | `NONE` | `M1+M2+M3` | `NO` |
| `STRESS-TORSIONAL-SHEAR-ORIGINAL` | `docs/validation_manual/cases/stress/stress-torsional-shear-original.md`<br>`6891a2b83731b4e1aa869e29ae5d6ed839a54b80` | `validation/hand_calcs/stress/torsional_shear.md`<br>`5b9dff099c811ae818a6825de1324e7c6ebe074e` | `DRAFT_EVIDENCE` | `E63` | `NONE` | `M1+M2+M3` | `NO` |
| `STRESS-TP-PHYS-004-LOAD-TO-RESULTANT` | `docs/validation_manual/cases/stress/stress-tp-phys-004-load-to-resultant.md`<br>`62d76f3a3f6084b75e3be3e5ed1f951d3aec182a` | `validation/hand_calcs/stress/tp_phys_004_load_to_resultant_stress.md`<br>`efe2727df78d391a9d4b13869ddf3feecbf638f1` | `DRAFT_EVIDENCE` | `E63` | `NONE` | `M1+M2+M3` | `NO` |
| `STRESS-TP-PHYS-005-ORIENTED-LOAD-TO-STRESS` | `docs/validation_manual/cases/stress/stress-tp-phys-005-oriented-load-to-stress.md`<br>`4b8ca635c58d8833508f528c373a7e6e9caa9e83` | `validation/hand_calcs/stress/tp_phys_005_oriented_load_to_stress.md`<br>`658c229f5751d35c6dd53e9cfa0305170185800f` | `DRAFT_EVIDENCE` | `E63` | `NONE` | `M1+M2+M3` | `NO` |
| `STRESS-TP-PHYS-006-PARTIAL-SPAN-LOAD-TO-STRESS` | `docs/validation_manual/cases/stress/stress-tp-phys-006-partial-span-load-to-stress.md`<br>`67f687f02b18e3bdf09f546b56c2b6a377c7c8df` | `validation/hand_calcs/stress/tp_phys_006_partial_span_load_to_stress.md`<br>`c6fd60333ca2f6eeae2556b0a2f27b27c9bc2aa9` | `DRAFT_EVIDENCE` | `E63` | `NONE` | `M1+M2+M3` | `NO` |
| `STRESS-TP-PHYS-007-STATION-SWEEP-STRESS` | `docs/validation_manual/cases/stress/stress-tp-phys-007-station-sweep-stress.md`<br>`ee2528a693757b58bb1c75dcc03012119e59c65d` | `validation/hand_calcs/stress/tp_phys_007_station_sweep_stress.md`<br>`f1e6e2bbb1632fb4082e05ea8f09ada392598e36` | `DRAFT_EVIDENCE` | `E63` | `NONE` | `M1+M2+M3` | `NO` |
| `STRESS-TP-PHYS-008-THERMAL-AXIAL-EFFECT-TO-STRESS` | `docs/validation_manual/cases/stress/stress-tp-phys-008-thermal-axial-effect-to-stress.md`<br>`a88420c61bb6ff9f8a8058a145ff0c21ab514c0b` | `validation/hand_calcs/stress/tp_phys_008_thermal_axial_effect_to_stress.md`<br>`d36e8e513e49ffd22a46ed2f684b1fccf4972092` | `DRAFT_EVIDENCE` | `E63` | `NONE` | `M1+M2+M3` | `NO` |
| `STRESS-TP-PHYS-009-COMBINED-AXIAL-BENDING-TO-STRESS` | `docs/validation_manual/cases/stress/stress-tp-phys-009-combined-axial-bending-to-stress.md`<br>`772a5388160ef092fe52e674b3e497b82640680c` | `validation/hand_calcs/stress/tp_phys_009_combined_axial_bending_to_stress.md`<br>`2e89a3032fc9b75008fcf1468e0f5035e94e78e5` | `DRAFT_EVIDENCE` | `E63` | `NONE` | `M1+M2+M3` | `NO` |
| `STRESS-TP-PHYS-015-CANONICAL-RESULTANT-STRESS-RECOVERY` | `docs/validation_manual/cases/stress/stress-tp-phys-015-canonical-resultant-stress-recovery.md`<br>`2795de415a78fdedb9b9939147aca6d8c858d610` | `validation/hand_calcs/stress/tp_phys_015_canonical_resultant_stress.md`<br>`9da6ac503119b2efd832254b597a0e05642db7bd` | `DRAFT_EVIDENCE` | `E63` | `NONE` | `M1+M2+M3` | `NO` |
| `STRESS-TP-PMM-P3-MILLTOL-EFFECTIVE-WALL-STRESS` | `docs/validation_manual/cases/stress/stress-tp-pmm-p3-milltol-effective-wall-stress.md`<br>`9b1887c649f8c385234f0600dccf9b732106f57b` | `validation/hand_calcs/stress/tp_pmm_p3_milltol_effective_wall_stress.md`<br>`374759306d65d6a27d0586c5dd05fe7bdf3d5f4c` | `DRAFT_EVIDENCE` | `E63` | `NONE` | `M1+M2+M3` | `NO` |
| `STRESS-TP-PMM-P3-MODULUSBASIS-RANGE-STRESS` | `docs/validation_manual/cases/stress/stress-tp-pmm-p3-modulusbasis-range-stress.md`<br>`908a51e4997f0c596b13c8890fc20ef91e3f1383` | `validation/hand_calcs/stress/tp_pmm_p3_modulusbasis_range_stress.md`<br>`ae2f38d5d7ca765d2171994b4b4db5a7a6126da8` | `DRAFT_EVIDENCE` | `E63` | `NONE` | `M1+M2+M3` | `NO` |

### 5.3 Nonlinear — 28 candidates

| Case | Page / Git blob | Independent reference / Git blob | Tier | Evidence | Review | Missing | Eligible now |
|---|---|---|---|---|---|---|---|
| `NL-ACTIVE-ONE-WAY-ORIGINAL` | `docs/validation_manual/cases/nonlinear/nl-active-one-way-original.md`<br>`ca4f11b20d5030b009f1f46ae82799b14ba0637c` | `validation/hand_calcs/nonlinear/active_set_one_way.md`<br>`dadd437ad46010f017479074d5fb01a3948d2da2` | `DRAFT_EVIDENCE` | `E63` | `NONE` | `M1+M2+M3` | `NO` |
| `NL-ASSEMBLED-FRICTION-BOUNDED-SLIDE-ORIGINAL` | `docs/validation_manual/cases/nonlinear/nl-assembled-friction-bounded-slide-original.md`<br>`bca5c3f0d3bdb0cf363d02d4a345446b8e40bff6` | `validation/hand_calcs/nonlinear/assembled_friction_bounded_sliding.md`<br>`3fd29d5d98ce8c410459ccd107d30c03a33c8729` | `DRAFT_EVIDENCE` | `E63` | `NONE` | `M1+M2+M3` | `NO` |
| `NL-ASSEMBLED-FRICTION-DERIVED-NORMAL-ORIGINAL` | `docs/validation_manual/cases/nonlinear/nl-assembled-friction-derived-normal-original.md`<br>`4aa970dcb7738ad2e01bafa3ed2852f80261d1df` | `validation/hand_calcs/nonlinear/assembled_friction_derived_normal.md`<br>`f28d8e543250d72891f93e528732d11095205ffd` | `DRAFT_EVIDENCE` | `E63` | `NONE` | `M1+M2+M3` | `NO` |
| `NL-ASSEMBLED-FRICTION-SLIDE-ORIGINAL` | `docs/validation_manual/cases/nonlinear/nl-assembled-friction-slide-original.md`<br>`5e8a6175ad4da2f7f3fc536e73ad2ce0f46d54a1` | `validation/hand_calcs/nonlinear/assembled_friction_sliding.md`<br>`574734cba3d1147709e4f4969f3ef3cf33f6b70e` | `DRAFT_EVIDENCE` | `E63` | `NONE` | `M1+M2+M3` | `NO` |
| `NL-ASSEMBLED-FRICTION-STICK-ORIGINAL` | `docs/validation_manual/cases/nonlinear/nl-assembled-friction-stick-original.md`<br>`ed1af62cffe304a418537ef1e89bb75b15f3939f` | `validation/hand_calcs/nonlinear/assembled_friction_sticking.md`<br>`5dd71747ede70bb64695c0a5aa187f6276618d67` | `DRAFT_EVIDENCE` | `E63` | `NONE` | `M1+M2+M3` | `NO` |
| `NL-ASSEMBLED-GAP-CLOSURE-ORIGINAL` | `docs/validation_manual/cases/nonlinear/nl-assembled-gap-closure-original.md`<br>`9c1887957e55411510eb119b02b70b60c471c624` | `validation/hand_calcs/nonlinear/assembled_gap_closure.md`<br>`c7f38da2c05f9e1b445777b075e160c949493ea2` | `DRAFT_EVIDENCE` | `E63` | `NONE` | `M1+M2+M3` | `NO` |
| `NL-ASSEMBLED-GAP-LIFT-OFF-ORIGINAL` | `docs/validation_manual/cases/nonlinear/nl-assembled-gap-lift-off-original.md`<br>`cd4d6bc4079db3d916fc0403d656f10ad630bc9f` | `validation/hand_calcs/nonlinear/assembled_gap_lift_off.md`<br>`f5d212d58168bb3879cff15596f1d991278f4e7e` | `DRAFT_EVIDENCE` | `E63` | `NONE` | `M1+M2+M3` | `NO` |
| `NL-ASSEMBLED-LIFT-OFF-ORIGINAL` | `docs/validation_manual/cases/nonlinear/nl-assembled-lift-off-original.md`<br>`e137bbd3c0c0ad43be133e6fb94abe9a61dae08b` | `validation/hand_calcs/nonlinear/assembled_lift_off.md`<br>`1f9e13d3975fc43fb4c9f29f53565609f65b1179` | `DRAFT_EVIDENCE` | `E63` | `NONE` | `M1+M2+M3` | `NO` |
| `NL-ASSEMBLED-MULTI-DOF-CASCADE-GAP-LIFT-OFF-ACCEPTED-ORIGINAL` | `docs/validation_manual/cases/nonlinear/nl-assembled-multi-dof-cascade-gap-lift-off-accepted-original.md`<br>`b976e622200a4da17e6967fc179bce48ce117967` | `validation/hand_calcs/nonlinear/assembled_multi_support_cascade_gap_lift_off_acceptance.md`<br>`770568ef1b20080df1f076c7da10e932d3e8ab5a` | `DRAFT_EVIDENCE` | `E63` | `NONE` | `M1+M2+M3` | `NO` |
| `NL-ASSEMBLED-MULTI-DOF-DERIVED-NORMAL-GAP-ACCEPTED-ORIGINAL` | `docs/validation_manual/cases/nonlinear/nl-assembled-multi-dof-derived-normal-gap-accepted-original.md`<br>`05f24cd29355e71bb760ada1781aae9621fea7b4` | `validation/hand_calcs/nonlinear/assembled_multi_support_derived_normal_gap_acceptance.md`<br>`98da24eca47734d31903733c149c0ccfb3aba8ed` | `DRAFT_EVIDENCE` | `E63` | `NONE` | `M1+M2+M3` | `NO` |
| `NL-ASSEMBLED-MULTI-DOF-DERIVED-NORMAL-ROTATIONAL-ACCEPTED-ORIGINAL` | `docs/validation_manual/cases/nonlinear/nl-assembled-multi-dof-derived-normal-rotational-accepted-original.md`<br>`676b97d8516c4124fec92c2c7253a2e49b88f844` | `validation/hand_calcs/nonlinear/assembled_multi_support_derived_normal_rotational_acceptance.md`<br>`c4afc4cd063f91597dbc7ece99375884a8fffd09` | `DRAFT_EVIDENCE` | `E63` | `NONE` | `M1+M2+M3` | `NO` |
| `NL-ASSEMBLED-MULTI-DOF-FOUR-CLASS-ACCEPTED-ORIGINAL` | `docs/validation_manual/cases/nonlinear/nl-assembled-multi-dof-four-class-accepted-original.md`<br>`13a9ca0b0c579832914c1c65c7b3cac8da2393db` | `validation/hand_calcs/nonlinear/assembled_multi_support_four_class_acceptance.md`<br>`ec270c9bfe42d0085927764ec800981c2604c56b` | `DRAFT_EVIDENCE` | `E63` | `NONE` | `M1+M2+M3` | `NO` |
| `NL-ASSEMBLED-MULTI-DOF-FRICTION-GAP-ACCEPTED-ORIGINAL` | `docs/validation_manual/cases/nonlinear/nl-assembled-multi-dof-friction-gap-accepted-original.md`<br>`7fc742b414828c11024661b61c160fe0410a4638` | `validation/hand_calcs/nonlinear/assembled_multi_support_friction_gap_acceptance.md`<br>`78cb98686cc4356234aed4a8c753a15bddf1d2b4` | `DRAFT_EVIDENCE` | `E63` | `NONE` | `M1+M2+M3` | `NO` |
| `NL-ASSEMBLED-MULTI-DOF-GAP-LIFT-OFF-ACCEPTED-ORIGINAL` | `docs/validation_manual/cases/nonlinear/nl-assembled-multi-dof-gap-lift-off-accepted-original.md`<br>`8a4b4e7d7e7282a8078077debadbcb1c7c7d3554` | `validation/hand_calcs/nonlinear/assembled_multi_support_gap_lift_off_acceptance.md`<br>`9ebfca579ca9b542c874b9fa2b89a8d98b09f6aa` | `DRAFT_EVIDENCE` | `E63` | `NONE` | `M1+M2+M3` | `NO` |
| `NL-ASSEMBLED-MULTI-DOF-MULTI-SUPPORT-ACCEPTED-ORIGINAL` | `docs/validation_manual/cases/nonlinear/nl-assembled-multi-dof-multi-support-accepted-original.md`<br>`46f9c0d6751abeb4ab0883dd4f1d211aeaeb789b` | `validation/hand_calcs/nonlinear/assembled_multi_support_multi_dof_acceptance.md`<br>`f0c1101ae73eb1117728bf14d2d3ed03c8d86120` | `DRAFT_EVIDENCE` | `E63` | `NONE` | `M1+M2+M3` | `NO` |
| `NL-ASSEMBLED-MULTI-DOF-MULTI-SUPPORT-OBS-ORIGINAL` | `docs/validation_manual/cases/nonlinear/nl-assembled-multi-dof-multi-support-obs-original.md`<br>`44e1cb9d583762ba6183571ef1fa228a730edecc` | `validation/hand_calcs/nonlinear/assembled_multi_support_multi_dof.md`<br>`fa9ec17eebeaa496be798b099ddab98ba2263172` | `DRAFT_EVIDENCE` | `E63` | `NONE` | `M1+M2+M3` | `NO` |
| `NL-ASSEMBLED-MULTI-DOF-NEGATIVE-GAP-ACCEPTED-ORIGINAL` | `docs/validation_manual/cases/nonlinear/nl-assembled-multi-dof-negative-gap-accepted-original.md`<br>`223b1942a1cce5a128b960d9d4109671b45795a6` | `validation/hand_calcs/nonlinear/assembled_multi_support_negative_gap_acceptance.md`<br>`acb63535be043f35909ad2fe72838ac2249522b5` | `DRAFT_EVIDENCE` | `E63` | `NONE` | `M1+M2+M3` | `NO` |
| `NL-ASSEMBLED-MULTI-DOF-OPPOSING-GAPS-ACCEPTED-ORIGINAL` | `docs/validation_manual/cases/nonlinear/nl-assembled-multi-dof-opposing-gaps-accepted-original.md`<br>`fae63004e0876b6c8e7b3ee769e63f52f4eaf563` | `validation/hand_calcs/nonlinear/assembled_multi_support_opposing_gaps_acceptance.md`<br>`75b5bc9117a7fe3bbd2aded0098908c7486599ae` | `DRAFT_EVIDENCE` | `E63` | `NONE` | `M1+M2+M3` | `NO` |
| `NL-ASSEMBLED-MULTI-DOF-ROTATIONAL-ACCEPTED-ORIGINAL` | `docs/validation_manual/cases/nonlinear/nl-assembled-multi-dof-rotational-accepted-original.md`<br>`7796ab74d51caa8269c88455499a586ba2f03d0f` | `validation/hand_calcs/nonlinear/assembled_multi_support_rotational_acceptance.md`<br>`3e71b7cdebb4a9eb84ecf30c80c064ad76e5ff01` | `DRAFT_EVIDENCE` | `E63` | `NONE` | `M1+M2+M3` | `NO` |
| `NL-ASSEMBLED-MULTI-DOF-THREE-SUPPORT-ACCEPTED-ORIGINAL` | `docs/validation_manual/cases/nonlinear/nl-assembled-multi-dof-three-support-accepted-original.md`<br>`a5829797a34a92c56461f1820cb30b2439c578af` | `validation/hand_calcs/nonlinear/assembled_multi_support_three_dof_acceptance.md`<br>`3973f8e7edf5e29fd863d44973bfc19c62f73151` | `DRAFT_EVIDENCE` | `E63` | `NONE` | `M1+M2+M3` | `NO` |
| `NL-ASSEMBLED-MULTI-DOF-TWO-SPAN-ACCEPTED-ORIGINAL` | `docs/validation_manual/cases/nonlinear/nl-assembled-multi-dof-two-span-accepted-original.md`<br>`de8af33f8fde3a60723ba133948da97ecbf36c5d` | `validation/hand_calcs/nonlinear/assembled_multi_support_two_span_acceptance.md`<br>`52437931e1c5841f17be93dc17b66338243b6c85` | `DRAFT_EVIDENCE` | `E63` | `NONE` | `M1+M2+M3` | `NO` |
| `NL-ASSEMBLED-MULTI-DOF-TWO-SPAN-OPPOSING-GAPS-ACCEPTED-ORIGINAL` | `docs/validation_manual/cases/nonlinear/nl-assembled-multi-dof-two-span-opposing-gaps-accepted-original.md`<br>`a3ba286eb47bf48ef4bcff13c7702677f53f86ce` | `validation/hand_calcs/nonlinear/assembled_multi_support_two_span_opposing_gaps_acceptance.md`<br>`4372b32d69b465bb366485090182fe45ce4c6820` | `DRAFT_EVIDENCE` | `E63` | `NONE` | `M1+M2+M3` | `NO` |
| `NL-ASSEMBLED-ONE-WAY-DEACTIVATE-ORIGINAL` | `docs/validation_manual/cases/nonlinear/nl-assembled-one-way-deactivate-original.md`<br>`4a4ce8ed0f6285f9c2adbba10eddd9f965ca2aef` | `validation/hand_calcs/nonlinear/assembled_one_way_deactivation.md`<br>`405df9123adfc69dd9e9e197afd74830b5b492b9` | `DRAFT_EVIDENCE` | `E63` | `NONE` | `M1+M2+M3` | `NO` |
| `NL-ASSEMBLED-ONE-WAY-REENGAGE-ORIGINAL` | `docs/validation_manual/cases/nonlinear/nl-assembled-one-way-reengage-original.md`<br>`67e5505762100249d3301cb4e82f5ade7ffc9504` | `validation/hand_calcs/nonlinear/assembled_one_way_reengagement.md`<br>`c53efc6b3008042fca434f0894f3237eced17c23` | `DRAFT_EVIDENCE` | `E63` | `NONE` | `M1+M2+M3` | `NO` |
| `NL-FRICTION-STICK-SLIDE-ORIGINAL` | `docs/validation_manual/cases/nonlinear/nl-friction-stick-slide-original.md`<br>`8484b84f42a51b4761d18574568d9fec5d8ecf20` | `validation/hand_calcs/nonlinear/friction_transition.md`<br>`40ef8b42fb5a825fe8d36092ecfc555d1bd1ae88` | `DRAFT_EVIDENCE` | `E63` | `NONE` | `M1+M2+M3` | `NO` |
| `NL-GAP-CLOSURE-ORIGINAL` | `docs/validation_manual/cases/nonlinear/nl-gap-closure-original.md`<br>`670076969458c0bb6d358045c3d123d4c2a32611` | `validation/hand_calcs/nonlinear/gap_closure.md`<br>`f717aadae69f802ba816dbb609b04e7aa27ec130` | `DRAFT_EVIDENCE` | `E63` | `NONE` | `M1+M2+M3` | `NO` |
| `NL-LIFT-OFF-ORIGINAL` | `docs/validation_manual/cases/nonlinear/nl-lift-off-original.md`<br>`dce02c70c0c991b66a21d3266a8206a18739f684` | `validation/hand_calcs/nonlinear/lift_off.md`<br>`a02403ed0aeea50bb888518dcef10055311ad6af` | `DRAFT_EVIDENCE` | `E63` | `NONE` | `M1+M2+M3` | `NO` |
| `NL-NONCONVERGENCE-LIMIT-ORIGINAL` | `docs/validation_manual/cases/nonlinear/nl-nonconvergence-limit-original.md`<br>`83ee017ee8d76889f79d905014071001d936fbd8` | `validation/hand_calcs/nonlinear/unresolved_nonconvergence.md`<br>`ad6a919fdfedd4d2aded1557129a39446d094d5c` | `DRAFT_EVIDENCE` | `E63` | `NONE` | `M1+M2+M3` | `NO` |

## 6. Existing evidence versus review evidence

The inventory does not reject the substantial verification evidence. Each
page records purpose, inputs, an independent reference, expected result,
software result, tolerance or `TBD`, pass/fail, solver version, provenance,
and exact reproduction commands. The 2026-07-10 producing run records passing
suite executions and deterministic generation; DEC-092 has newer
commit-bound evidence. Those are strong reviewer inputs.

They are not the missing act. The following do **not** qualify as a page
promotion basis:

- TP-E2 generator/suite execution, because its run record explicitly leaves
  `MAINTAINER_REVIEWED` promotion open;
- D-48 claims-boilerplate alignment, because it changed boundary language and
  did not record page-specific evidence review;
- R13/R14 agent checks, because they expressly preserve the promotion gate;
- the deliverable `_REVIEW.md`, because it predates the pages and reviews
  PKG-02 compatibility/checking readiness, not page evidence;
- the DEC-092 implementation/sweep, because its derivative page expressly
  remains `DRAFT_EVIDENCE` and the run record disclaims promotion;
- `INTERNALLY_VERIFIED` reproduction, because that label attaches to an
  internal reproduction bundle and is not a maintainer-review disposition.

## 7. Bounded review-work plan (`MR-W1`)

### Phase A — make the review candidate truthful (agent-executable only after authorization)

1. Freeze the intended review base and inventory the same 64 IDs.
2. Correct only the superseded runner-binding statements in the generator,
   63 affected generated pages, and the two aggregate index statements. Use
   current committed DEL-10-05/R13 evidence; do not describe unsupported
   per-case runner coverage. The DEC-092 page retains its explicit
   locked/offline-crate-test-only statement unless new evidence exists.
3. For every case, map the Markdown derivation to an actual authoritative
   machine-readable witness artifact. If the fixture constructor/expected
   slots are proposed as that artifact, record and verify the mapping
   explicitly; if they do not meet the strategy's witness contract, retain
   the page as `DRAFT_EVIDENCE` and record `WITNESS_CHAIN_MISSING`. Do not
   manufacture structured witnesses in the review tranche.
4. Regenerate into a temporary directory first, compare, then generate the
   authorized repository outputs. No hand edits to generated pages.

### Phase B — reviewer inputs

For each page, present the owner/sole maintainer with:

- final page path and Git blob/SHA-256;
- fixture ID, suite/deliverable, generator blob and page-schema version;
- independent-reference path/hash and the machine-readable witness path/hash
  (or an explicit missing-chain finding);
- fixture/test source paths/hashes and exact named test commands;
- producing run/sweep paths/hashes, date, toolchain, commit, and observed
  result;
- tolerance/policy pointer or visible `TBD`, including any separately ruled
  DEC-046 release values without implying that review selected them;
- provenance/redistribution fields and protected/private-data scan result;
- known limitations, blocked behavior, warning/non-convergence semantics,
  stale/currency caveats, and current evidence tier.

### Phase C — per-page checklist and disposition

The owner/sole maintainer records `PASS`, `FAIL`, or `NOT_APPLICABLE` plus a
reason for each manual §9 item:

1. evidence commands and inputs recorded;
2. expected results reproducible from repository artifacts or lawful external
   sources;
3. human rendering traces to the authoritative machine-readable witness;
4. units and assumptions explicit;
5. warnings, non-convergence, and missing-data behavior truthfully preserved;
6. protected-content/private-data scan clean or risk-dispositioned;
7. professional reliance remains human-owned; and
8. unresolved gaps remain visible.

The per-page conclusion is one of:

- `PROMOTE_TO_MAINTAINER_REVIEWED` — reviewed for software-quality use;
- `RETAIN_DRAFT_EVIDENCE` — review completed but one or more findings prevent
  promotion or the owner elects not to promote; or
- `BLOCKED` — the evidence cannot be used until a source, provenance,
  licensing, or technical issue is resolved.

### Phase D — durable review-record schema

Use one immutable batch record with one row per page (or one immutable record
per page), containing at least:

```yaml
schema: chirality-validation-case-maintainer-review/v1
review_id: <stable-id>
reviewer_name: Ryan Tufts
reviewer_role: owner-and-sole-maintainer-per-DEC-027
reviewed_at: <timestamp>
repository_commit: <exact-commit>
fixture_id: <exact-id>
suite: <mechanics|stress|nonlinear>
page_path: <path>
page_git_blob: <40-hex>
page_sha256: <64-hex>
generator_path: docs/validation_manual/cases/generate_validation_case_pages.py
generator_git_blob: <40-hex>
independent_reference_path: <path>
independent_reference_git_blob: <40-hex>
machine_readable_witness_path: <path-or-NONE>
machine_readable_witness_git_blob: <40-hex-or-NONE>
fixture_and_test_source_blobs: [{path: <path>, git_blob: <40-hex>}]
evidence_records: [{path: <path>, git_blob: <40-hex>}]
tolerance_basis: <pointer-or-TBD>
provenance_disposition: <recorded-result>
checklist:
  commands_inputs: <PASS|FAIL|NOT_APPLICABLE + reason>
  expected_result_reproducibility: <...>
  witness_traceability: <...>
  units_assumptions: <...>
  diagnostic_fidelity: <...>
  protected_private_data: <...>
  professional_boundary: <...>
  visible_gaps: <...>
findings: [<stable-finding-ids>]
disposition: <PROMOTE_TO_MAINTAINER_REVIEWED|RETAIN_DRAFT_EVIDENCE|BLOCKED>
scope_statement: software-quality-review-only
owner_ruling_verbatim: <exact-owner-text>
```

The record must state that `MAINTAINER_REVIEWED` is not public-source legal
clearance, final benchmark acceptance, external-prover correlation,
professional acceptance, release readiness, lifecycle closure, or a reliance
claim.

### Phase E — deterministic application and backcheck

1. Add a governed promotion map to the generator keyed by exact fixture ID and
   review-record pointer; do not hand-edit generated pages.
2. Render `status: maintainer_reviewed`, the visible evidence-state row, and
   the review-record pointer only for IDs whose exact owner disposition is
   `PROMOTE_TO_MAINTAINER_REVIEWED`. Retained/blocked IDs remain truthful.
3. Update the index so every row's state and review pointer agree with its
   page; remove the blanket “All cases are DRAFT_EVIDENCE” sentence only if it
   is no longer true, replacing it with exact counts.
4. Run generator `--check`; reconcile registrations/files/index links and
   tier counts; verify all pointers and hashes; search for stale runner text;
   run claims-language lint and the relevant non-writing repository checks.
5. Produce a durable fan-in showing promoted, retained-draft, and blocked IDs
   separately. Promotion applies only after the owner has supplied the review
   dispositions and separately authorized the application scope.

### Rerun/invalidation criteria

A later change to a reviewed page, its generator/template, independent
reference, machine-readable witness, fixture/test source, cited tolerance
policy, cited evidence run/sweep, or claims boundary makes the hash-bound
record non-current for that page. The page must be regenerated and re-reviewed
or explicitly retained at its prior tier with a truthful currency note. A
mere rerun with unchanged inputs may supplement evidence but does not silently
renew the owner review.

## 8. Owner options and exact ruling interface

| Option | Owner act | Effect now |
|---|---|---|
| `MR-A` — adopt `MR-W1` (**recommended**) | Authorize preparation/correction and the bounded review workflow; later review dispositions remain separate owner acts. | No page promoted. Produces truthful final candidates and a hash-bound owner-review instrument. |
| `MR-B` — amend `MR-W1` | Supply exact amendments to scope, checklist, evidence record, or review sequencing. | No page promoted. Amended plan must be executed and returned before promotion. |
| `MR-C` — defer/decline review work | Leave all 64 pages at `DRAFT_EVIDENCE`. | No page, status, register, or Remaining change. |

Exact response form:

```text
MAINTAINER_REVIEWED BASIS RULING, 2026-08-10: MR-A
```

or:

```text
MAINTAINER_REVIEWED BASIS RULING, 2026-08-10: MR-B
Amendments: <exact text>
```

or:

```text
MAINTAINER_REVIEWED BASIS RULING, 2026-08-10: MR-C
Reason (optional): <text>
```

This interface intentionally contains no direct promotion option because the
required review basis does not yet exist. After `MR-W1` returns, promotion
requires an exact page list (or a complete all-pages selector), exact reviewed
hashes, and the owner's recorded review dispositions.

## 9. Conditional on-ruling mechanism

- `MR-A`: prepare an exact bounded implementation brief for Phase A, then a
  separate owner-review packet for Phases B–D. Do not apply a tier change
  until that review returns exact dispositions and the owner authorizes the
  Phase E application.
- `MR-B`: amend this plan only as ruled, revalidate containment and authority,
  and return the amended plan before work proceeds.
- `MR-C`: record deferral verbatim in the parent run handoff; leave all 64
  page bytes, generator/index, deliverable state, and Remaining text
  unchanged.

## 10. Non-effects

This packet performs no promotion, review act, source acceptance, case edit,
generator/index edit, GUI evidence work, DEC-046 value selection, reproduction
acceptance, external-prover correlation, release act, reliance-posture change,
lifecycle transition, register disposition, DEL-09-04 status/Remaining edit,
receipt append, Git action, or external action. It does not fabricate review
provenance. Standard claim fence applies (F-PIP-2; claims taxonomy per
DEC-081).
