---
doc_id: CB-2026-08-02-DEC092-DEL0502-TEMPG-IMPLEMENTATION-001
doc_kind: working_items.candidate_brief
status: candidate_proposal_only
prepared: 2026-08-02
package_id: PKG-05
deliverable_id: DEL-05-02
decision_basis: DEC-092, DEC-077, D-45 O-B
frozen_basis: 97678a841ef58345c73d3470ed8de57c9b1405d2
owner_gate: Gate 1
---

# CANDIDATE Brief — DEL-05-02 DEC-092 Temperature-Indexed Shear-Modulus Implementation

**Epistemic state:** `CANDIDATE / PROPOSAL_ONLY`

**Prepared by:** WORKING_ITEMS for exactly `PKG-05 / DEL-05-02`

**Candidate ID:** `CB-2026-08-02-DEC092-DEL0502-TEMPG-IMPLEMENTATION-001`

**Frozen basis:** `97678a841ef58345c73d3470ed8de57c9b1405d2`

This brief proposes a bounded implementation activation. It does not execute
the work, change deliverable state, or activate itself. The human owner retains
the Gate 1 choice to `ADOPT`, `AMEND`, `DEFER`, or `DECLINE`.

## 1. Objective

Implement the accepted D-45 Option B / `DEC-092` behavior in DEL-05-02:

- each user-authored material temperature point may carry an explicit
  shear-modulus quantity `G` with provenance;
- a load case selecting a material temperature-point ID consumes that point's
  `G` exactly;
- a load case selecting a temperature consumes `G` by linear interpolation
  between the two adjacent bracketing temperature points;
- interpolated evidence identifies both source-point IDs and the interpolation
  method;
- selected temperature-basis paths do not extrapolate and do not fall back to,
  copy, or silently clone the material's base `G`;
- the no-temperature-basis path continues to consume the user-authored base
  `G` as it does now.

The objective is product implementation of already accepted direction, not a
new decision. It is limited to the schema, authoring, operation, solver,
fixture, benchmark, test, and closeout surfaces named in this brief.

## 2. Accepted Authority and Reverified Live Basis

Paths are repository-relative. The later activation must resolve its repository
root with `git rev-parse --show-toplevel` and recheck every authority and live
surface before dispatch.

### 2.1 Accepted authority

| Authority | Role in this candidate | SHA-256 at preparation |
| --- | --- | --- |
| `projects/chirality-piping/execution/_Coordination/_DECISIONS/D-45_RULING_2026-08-01.md` | durable owner ruling: D-45 = O-B | `f24a2e81b742651223b486d9059681d9bf4d9e8d68b72779f867d4a43b4c7eda` |
| `projects/chirality-piping/execution/_Coordination/_DECISIONS/D-45_temperature_indexed_shear_modulus.md` | ruled packet and option semantics | `cfe217eae71d4712e76b861233fb2722c4039cfff4586b87af30420571c78e07` |
| `projects/chirality-piping/execution/_Coordination/_DECISIONS/D-38_temperature_interpolation_policy.md` | DEC-077 interpolation-policy predecessor | `76e4fd46c7b94bd54130b0a5279d085c9ab3b9d7459c3f5cb836962067208e06` |
| `projects/chirality-piping/execution/_Decomposition/SOFTWARE_DECOMP.md` | current `DEC-077` and `DEC-092` decomposition rows | `06419f7e7e399d4732d9e207eb87b17b3a4d4a4ce74a19f9d5fe60f9a342882d` |
| `projects/chirality-piping/execution/_DAG/_LATEST.md` | active dependency pointer | `04f24cd88d16b38d6da00ae2dee32f0387381ee41659de2e352cba07127736e3` |
| `projects/chirality-piping/execution/_DAG/DAG-009/APPROVAL_RECORD.md` | accepted DAG-009 | `f25526c4e0eec239f5d3464ca4d8e0ab8c9638ebd035bc9aa282def33989337b` |
| `projects/chirality-piping/execution/_DAG/DAG-009/DependencyEdges.csv` | active edges | `4293cbe39ff794f74da7031c2f0e2706003fadb666ca4d85f0e7d3ec25baa9cc` |
| `projects/chirality-piping/execution/_DAG/DAG-009/DeliverableNodes.csv` | active node inventory | `6e5050c4e578f6ff9819ee7a11dbb395b3f0a163b4fb0c48e88c3d084d9b0732` |
| `projects/chirality-piping/execution/_DAG/DAG-009/TopologicalWaves.md` | wave assignment | `8108cceaf21bf4eb7e613e2f347f1c925932d8067b397068e2066f23ebb8a3b7` |

The candidate treats the D-45 durable ruling as authority and the packet as its
decision-support context. `DEC-092` is the implementation row. `DEC-077`
remains the selector contract that DEC-092 extends; it is not reopened.

### 2.2 Deliverable intake

| Surface | Reverified fact | SHA-256 at preparation |
| --- | --- | --- |
| DEL-05-02 `_STATUS.md` | `IN_PROGRESS`; Remaining explicitly calls for D-45 O-B / DEC-092 implementation | `fa6c09a950a08be60a426900e6755b5ad83a1494b5dbfb5d39afb64507c8486f` |
| DEL-05-02 `MEMORY.md` | current implementation history and preserved residual | `35595d35aa561eda2271cd85d1fa80f30f8c838b2ec1bec26b8e58b3f73e5742` |
| DEL-05-02 `_CONTEXT.md` | current context envelope | `0502f2beaf551107358215c881ab14b20fb0fbec8f9eea2fa4fc031f02e1323b` |
| DEL-05-02 `_REFERENCES.md` | accepted references | `e035cdd1ba713fc89ab00d944dadbb17b5a5b0469375cd927d962a42b56b224e` |
| DEL-05-02 `_DEPENDENCIES.md` | local dependency narrative | `f81e720e6245b40de99daf6c296c9fab290836504ed0542e254ad7fe4b5c4e31` |
| DEL-05-02 `Dependencies.csv` | local dependency ledger | `de3567bf415a267ce7c8e9e9c8fdf9c3673c15949f9542b0b45bd3961ec185e7` |
| DEL-05-02 `ScopeOfWork.md` | `SOW_V1` representation | `de2f2cf6249e208bf222515f469d5a0fc4acfeb298657e3de557c2cbf3486750` |
| DEL-05-02 `_SEMANTIC.md` | current semantic contract | `158c609fd916f80672948778f5c3093e58dc0e166921c4abe49a9ddd68eceeac` |
| DEL-05-02 `_SEMANTIC_LENSING.md` | current lensed semantic context | `e10abb304c9e219bfbdbbbc50c3379ef53ea7aeb12cbcf9e321bcff3e67a2cef` |

DEL-05-02 uses `SOW_V1`; separate Datasheet, Specification, Guidance, and
Procedure files are not live representation surfaces and are not invented by
this candidate.

The active DAG places DEL-05-02 in wave 1. Its eight active
`EXECUTION / UPSTREAM` rows are `SATISFIED`:

- `DAG-002-E0135` to DEL-00-01;
- `DAG-002-E0136` to DEL-00-02;
- `DAG-002-E0137` to DEL-00-03;
- `DAG-002-E0138` to DEL-00-06;
- `DAG-002-E0139` to DEL-00-08;
- `DAG-002-E0451` to DEL-05-01;
- `DAG-002-E0452` to DEL-02-02;
- `DAG-002-E0453` to DEL-05-04.

The retired `DAG-002-E0616` row is not an active gate. The ten `ANCHOR`
rows are non-execution anchors. The local `_DEPENDENCIES.md` narrative still
names DAG-007, but the accepted root pointer now selects DAG-009. This brief
records that stale label for later ordinary reconciliation; it does not treat
the label as authority over the active pointer and does not widen this tranche
to edit dependency surfaces.

### 2.3 Workflow and prior-run basis

| Surface | Use | SHA-256 at preparation |
| --- | --- | --- |
| `docs/SOFTWARE_WORKFLOW_PROFILE.md` | controlling software work profile | `f97af1d323524f9a2be1dab8b5b33c1350c8f48c38cd3c6f8d0f8a9cd9821ea2` |
| `projects/chirality-piping/software-workflow.json` | registered commands | `3a6fd86bd362eed5e1fbcda05dcde961fca8ad46cb14375ce3dd79c3872e09b7` |
| `projects/chirality-piping/loop/WORKPLAN_2026-07-18b_piping_loop.md` | live standing workplan: branch-first, no self-merge, DEC-025 pre-push five-surface gate, and H4 evidence posture | `82806c43b654a51771f6c4b7eed60ccc620dab6a34cc9f6e5de7949abba9724b` |
| `projects/chirality-piping/execution/_Coordination/_COORDINATION.md` | H4 test posture | `54e071eecfea36658dc8ba99f1ecd81d2f464d2c32be8431339cd2118d10ab5c` |
| `projects/chirality-piping/docs/claims_registry.md` | claim-language posture | `54bc0720265a4445c219469ab6652c9f7005ede83f05b127e70766859db13025` |
| DEL-05-02 `_run_records/WORKING_ITEMS_RUN_2026-07-10_TP-PMM-P3-MODULUSBASIS-001.md` | preceding modulus-basis implementation evidence | `4660b2c5a88ac35254a525e1675625c931e4ddb0d06dc326664f12d28ef379c8` |
| DEL-05-02 `_run_records/WORKING_ITEMS_RUN_2026-07-15_TP-PMM-P3-TEMPINTERP-001.md` | preceding temperature interpolation evidence | `8cbeadf38c7e884631d804b79ee613d1904c32b38df189f1fc0902b73270327b` |

Any adopted activation must re-read these surfaces rather than inheriting this
brief's observations as immutable implementation facts.

## 3. Current-State Finding

The implementation gap was reverified at the frozen basis.

1. `schemas/material.schema.yaml` does not admit `shear_modulus` as a
   temperature-point property kind, and its temperature-reference description
   describes only elastic modulus and thermal-expansion coefficient.
2. `schemas/model.schema.yaml` describes load-case temperature basis in terms
   of elastic modulus and thermal-expansion coefficient, not `G`.
3. `MaterialTemperaturePointInput` in
   `core/product_physics/src/lib.rs` contains temperature, elastic modulus,
   thermal-expansion coefficient, and provenance, but no point-level `G`.
4. Exact-ID resolution currently resolves point `E` and optional point alpha,
   then explicitly preserves `material.shear_modulus` from the base material.
5. Temperature interpolation currently requires and interpolates point `E` and
   alpha only. Its provenance names only those sources.
6. The frame and curved-member paths consume the resolved material's `G`, so a
   correctly resolved point/interpolated value will reach torsional stiffness;
   the present resolved value is the base-material clone.
7. The operation applier has base-material `G` authoring and load-case selector
   operations, but no structured temperature-point `G` field path.
8. The desktop material-library authoring helper does not offer temperature-
   point `G`; the load-case inspector already exposes DEC-077 selectors.
9. Existing exact-ID and interpolation tests exercise `E` and alpha but do not
   distinguish selected point `G` from base `G`.
10. Generic result metadata already carries basis provenance; no structural
    result-schema change has been shown necessary.

These are implementation findings, not a product-direction proposal. If live
state differs when the candidate is adopted, the activation must stop or seek a
fence amendment instead of forcing this plan onto changed code.

## 4. Proposed Acceptance Contract

An adopted implementation is acceptable only if all predicates below are true
on the implementation head.

### 4.1 Schema and units

1. A material temperature point may carry an optional, explicit
   `shear_modulus` stress quantity with value, unit, and provenance.
2. The material schema recognizes the property kind and temperature reference
   without assigning a default value.
3. Model-schema text truthfully describes selected `E`, `G`, and alpha behavior.
4. `G` is positive and finite after normalization. Accepted stress units follow
   the existing quantity normalization path; no second unit system is created.
5. Existing material documents with no temperature-point `G` remain schema-
   valid. Their solve compatibility is governed by predicate 4.4 rather than a
   silent migration.

### 4.2 User authoring and structured operations

1. The desktop material-library surface can select shear modulus, enter an
   explicit value and stress unit, associate it with an existing user-authored
   temperature-point reference, and retain user provenance.
2. The authoring path does not consult a browser catalog, protected table, or
   inferred default.
3. The operation applier supports the exact dynamic field path for a material
   temperature point's `shear_modulus.value` and requires the associated unit
   and dimension contract already used for stress quantities.
4. The applier verifies stale-before state and that the target material and
   temperature point exist before applying an edit.
5. Native and WASM operation-corpus tests prove one accepted edit, one blocked
   missing-point edit, and one blocked invalid-value or invalid-unit edit.
6. The new corpus cases receive fresh case IDs. They do not inherit review or
   acceptance claims from earlier DEC-030 or DEC-032 cases.

### 4.3 Exact-ID resolution

1. When `material_temperature_point_id` is selected, the exact point's `E`,
   `G`, and applicable alpha are resolved from that point.
2. Selected exact-ID resolution requires a valid point `G`; it does not use base
   `G` when the point omits or invalidates it.
3. Result basis metadata identifies the exact material temperature-point ID and
   the point-level `G` source.
4. A test chooses base and point `G` values that differ materially, so the old
   clone behavior cannot satisfy the assertion accidentally.

### 4.4 Temperature interpolation

1. Temperature selection uses the two adjacent points that strictly bracket the
   selected temperature and interpolates `G` linearly with the same normalized
   temperature coordinate used for the accepted DEC-077 basis.
2. Both bracketing points must contain valid `G`; neither endpoint may be
   substituted from base material data.
3. Provenance records both source-point IDs and the interpolation method.
4. Exact endpoint/equality, outside-range selection, nonadjacent selection,
   duplicate temperatures, missing point `G`, and invalid point `G` are covered
   by explicit tests. No extrapolation is permitted.
5. Existing mutual-exclusion behavior between exact-ID and temperature
   selectors remains blocking.

### 4.5 Mechanics consumption and deterministic anti-fallback proof

1. A straight-pipe pure-torsion case demonstrates the classical relationship
   `theta = T L / (G J)` for exact-ID and interpolated `G`.
2. Holding selected point `G` constant while varying base `G` does not change a
   selected-basis torsional result.
3. Holding geometry, loads, and base `G` constant while varying selected point
   `G` changes torsional rotation by the predicted relationship.
4. Removing selected point `G` blocks the selected-basis solve rather than
   reproducing the base-material result.
5. A no-temperature-basis model continues to consume its explicit base `G` and
   preserves existing behavior.
6. Load-combination outputs retain the chosen per-load-case modulus-basis
   provenance.

### 4.6 Evidence and compatibility

1. A dedicated invented product-preview request exercises point `G`, exact-ID,
   and interpolation behavior without introducing proprietary or protected
   values.
2. An independent hand calculation records inputs, units, equations, expected
   rotations, and comparison results.
3. The mechanics benchmark implements the same oracle independently enough to
   detect a solver-side base-`G` clone.
4. Comparison uses the already accepted DEC-024/DEC-026 analytic-class
   tolerance posture. No new tolerance is selected in this tranche.
5. Existing no-temperature-basis fixtures remain passing.
6. Legacy documents that select a temperature basis but omit point `G` newly
   block instead of silently falling back to base `G`. This is the intentional
   DEC-092 compatibility boundary; the implementation must report it clearly
   and must not auto-populate or destructively migrate user data.
7. Generic result metadata is tested for truthful `G` provenance. No change to
   `schemas/results.schema.yaml` is planned. A discovered structural need is a
   stop-and-amend condition.

### 4.7 Bounded deliverable state update

Only after every required implementation and validation predicate passes:

1. `ScopeOfWork.md` makes the active CLM-007 target-contract declaration
   current by replacing its stale "Shear modulus remains ... pending D-45"
   sentence with the validated DEC-092 end-state and evidence binding. This is
   target-contract currentness, not generic evidence accumulation; unrelated
   claims remain unchanged.
2. `MEMORY.md` receives one concise entry binding the decision, implementation
   head, validation evidence, compatibility behavior, and remaining truth.
3. `_STATUS.md` remains `IN_PROGRESS`; the exact DEC-092 Remaining item is
   removed only after its full proof is recorded. Any genuine residual remains
   explicit.
4. One implementation run record captures dispatched briefs, returns, changed
   paths, tests, evidence-sweep artifact, risks, and derivative disposition.
5. WORKING_ITEMS prepares exact content for the next sequential minimal loop
   receipt after re-reading the live cursor. WORKING_ITEMS does not write the
   loop-owned receipt surface. A separate parent-validated CHANGE integration
   step applies that exact content to `loop/LOOP_RECEIPTS.md`, binding the
   validated implementation head and evidence without claiming owner
   integration before that act occurs.

The validation-manual pages are a derivative package owned by DEL-09-04. Their
regeneration is explicitly deferred and routed as a later follow-up; this
candidate performs no foreign deliverable write.

## 5. Proposed Exact Write Fence

Adoption may authorize only the paths below. Existing paths are listed first;
proposed-new paths are marked. Any additional path requires an owner amendment
before it is written.

### 5.1 Existing paths

1. `projects/chirality-piping/schemas/material.schema.yaml`
2. `projects/chirality-piping/schemas/model.schema.yaml`
3. `projects/chirality-piping/tests/test_material_schema.py`
4. `projects/chirality-piping/tests/test_model_schema.py`
5. `projects/chirality-piping/fixtures/material/invented_material_library_valid.json`
6. `projects/chirality-piping/core/model_operations/operation_applier/src/lib.rs`
7. `projects/chirality-piping/fixtures/model_operations/contract_corpus/README.md`
8. `projects/chirality-piping/core/product_physics/src/lib.rs`
9. `projects/chirality-piping/apps/desktop/src/features/library/LibraryManagerPanel.tsx`
10. `projects/chirality-piping/apps/desktop/src/features/library/LibraryManagerPanel.test.tsx`
11. `projects/chirality-piping/apps/desktop/e2e/r2-smoke.spec.ts`
12. `projects/chirality-piping/apps/desktop/SMOKE.md`
13. `projects/chirality-piping/validation/benchmarks/mechanics/src/lib.rs`
14. `projects/chirality-piping/validation/benchmarks/mechanics/README.md`
15. `projects/chirality-piping/validation/hand_calcs/mechanics/README.md`
16. `projects/chirality-piping/execution/PKG-05_Loads, Load Cases, and Stress Recovery/1_Working/DEL-05-02_Load-case algebra engine/ScopeOfWork.md`
17. `projects/chirality-piping/execution/PKG-05_Loads, Load Cases, and Stress Recovery/1_Working/DEL-05-02_Load-case algebra engine/MEMORY.md`
18. `projects/chirality-piping/execution/PKG-05_Loads, Load Cases, and Stress Recovery/1_Working/DEL-05-02_Load-case algebra engine/_STATUS.md`
19. `projects/chirality-piping/loop/LOOP_RECEIPTS.md` — reserved for the
    separate parent-validated CHANGE integration step; not a WORKING_ITEMS
    package write

### 5.2 Proposed-new paths

20. `projects/chirality-piping/fixtures/product_preview/invented_dec092_temperature_g_request.json`
21. `projects/chirality-piping/fixtures/model_operations/contract_corpus/case_76_accept_set_field_material_temperature_point_shear_modulus.json`
22. `projects/chirality-piping/fixtures/model_operations/contract_corpus/case_77_block_set_field_material_temperature_point_shear_modulus_missing_point.json`
23. `projects/chirality-piping/fixtures/model_operations/contract_corpus/case_78_block_set_field_material_temperature_point_shear_modulus_invalid.json`
24. `projects/chirality-piping/validation/hand_calcs/mechanics/tp_dec092_temperature_indexed_shear_modulus_torsion.md`
25. `projects/chirality-piping/execution/PKG-05_Loads, Load Cases, and Stress Recovery/1_Working/DEL-05-02_Load-case algebra engine/_run_records/WORKING_ITEMS_RUN_2026-08-02_DEC092_TEMPERATURE_G_IMPLEMENTATION.md`
26. `projects/chirality-piping/execution/PKG-05_Loads, Load Cases, and Stress Recovery/1_Working/DEL-05-02_Load-case algebra engine/_run_records/DEC092_TEMPERATURE_G_IMPLEMENTATION_RUNTIME_EVENTS.jsonl`
27. `projects/chirality-piping/execution/PKG-05_Loads, Load Cases, and Stress Recovery/1_Working/DEL-05-02_Load-case algebra engine/_run_records/DEC092_TEMPERATURE_G_IMPLEMENTATION_RUNTIME_SUMMARY.json`

### 5.3 Tool-generated evidence path

After a clean implementation commit, the registered evidence sweep may create
exactly one new file matching its deterministic output convention:

`projects/chirality-piping/validation/evidence/sweeps/SWEEP_<UTC>_<implementation-commit12>.json`

The integration owner records the actual resolved filename in the run record
and receipt. This allowance is for the registered tool output only, not an
open directory fence.

The fence intentionally excludes `schemas/results.schema.yaml`, dependency
ledgers, DAG snapshots and pointers, decision records, generated validation-
manual pages, lifecycle surfaces outside DEL-05-02, and all foreign-loop paths.

## 6. Proposed Work Graph

Execution mode is `MIXED`: bounded specialist implementation with a single
serialized integration owner. No node below is dispatched by this candidate.

| Node | Bounded objective | Depends on | Sole write ownership |
| --- | --- | --- | --- |
| N1 — contract | Add the optional temperature-point `G` contract, model descriptions, schema tests, and invented material fixture | adopted activation and fresh-basis check | schema files, schema tests, material fixture |
| N2 — operation | Add the structured point-`G` edit path and cases 76–78 | N1 | operation applier and operation corpus |
| N3 — solver | Add normalization, exact-ID resolution, interpolation, provenance, blocks, and anti-fallback tests | N1 | product physics and product-preview fixture |
| N4 — oracle | Add independent torsion hand calculation and mechanics benchmark | N1 contract; may proceed parallel to N2/N3 | mechanics benchmark and hand-calc paths |
| N5 — authoring | Add user authoring and UI coverage | N1, then N2 contract | library panel, unit test, Playwright smoke, SMOKE.md |
| N6 — integration | Reconcile N1–N5, repair only within fence, run focused/full checks, and prove containment | all producer nodes | exclusive integration control over all implementation paths; no concurrent writes |
| N7 — package closeout | Write truthful DEL-05-02 state, run record, and runtime summary; prepare exact next-receipt content without applying it | N6 passes | the four DEL-05-02 package closeout paths only |
| N8 — parent-validated CHANGE integration | Create the implementation commit, run the commit-bound sweep, apply the exact prepared loop receipt, create the closeout commit, push the candidate branch, and present the owner integration gate | N7 | Git state, generated sweep output, and `loop/LOOP_RECEIPTS.md` only |

N1–N5 must receive sealed briefs naming their exact reads, writes, outputs,
checks, and return schema. A `TASK + software-bounded-implementation` form is
preferred for bounded implementation; `software-test-planning` or
`software-code-review` may be used where the live skill contract fits. An
ephemeral generalist is permitted only when TASK cannot express the bounded
objective and must receive the same sealed contract.

No child may delegate. Shared-file work is serialized by the table above. N6
is the only node permitted to reconcile cross-node changes.

## 7. Dispatch, Runtime, and Recovery Contract

An adopted activation must:

1. reverify the branch, frozen basis ancestry, authority hashes, active DAG
   pointer, Remaining text, write-fence existence/nonexistence, and absence of
   overlapping uncommitted writes;
2. obtain the ordinary branch-first CHANGE gate before any implementation
   dispatch;
3. materialize an adopted WORKING_ITEMS activation record that transcribes the
   owner's exact Gate 1 ruling and any amendments;
4. freeze each node brief before launch and record its hash, reads, writes,
   parentage, acceptance checks, attempt count, and return location;
5. append runtime events for launch, progress, return, validation, retry,
   quarantine, and closure; summarize final node states in the runtime summary;
6. quarantine an incomplete or out-of-fence return rather than integrating it;
7. retry only with a new attempt ID and an amended sealed brief that records the
   prior failure; and
8. stop for owner direction if authority, fence, compatibility, or result-
   schema assumptions change materially.

Recovery is commit-based and non-destructive. Before owner integration, the
candidate branch can be abandoned or the scoped tranche reverted by CHANGE.
No automatic model migration is permitted. Accepted source authority and
historical evidence remain unchanged.

## 8. Validation Plan

The integration owner must record exact command, exit code, and relevant
artifact/hash for every executed check.

### 8.1 Focused checks

- Rust format and tests for `core/model_operations/operation_applier`.
- Rust format and tests for `core/product_physics`.
- Rust format and tests for `validation/benchmarks/mechanics`.
- Targeted schema tests for material and model schemas.
- `tests/test_library_import_provenance.py` as a no-write compatibility check.
- LibraryManagerPanel unit tests.
- Native and WASM operation-corpus parity for cases 76–78.
- Dedicated exact-ID, interpolation, edge, missing-`G`, duplicate-temperature,
  mutual-selector, provenance, combination, and base-behavior solver tests.
- Hand-calculation versus benchmark and solver comparison at the existing
  analytic-class tier.

### 8.2 Registered and standing checks

From `projects/chirality-piping`, using the commands registered in
`software-workflow.json` where applicable:

1. `npm run test:desktop`
2. `npm run build:desktop`
3. `npm run test:e2e:desktop` and `npm run test:e2e:dist:desktop` because H4
   requires Playwright evidence for this user-visible UI behavior change;
4. the project pytest suite;
5. `python3 tools/release/run_evidence_sweep.py --execute` after the clean
   implementation commit;
6. from the repository root, practitioner-harness self-check and full pytest;
7. from the repository root,
   `python3 tools/validation/validate_claims_language.py --repo-root .`;
8. from the repository root,
   `python3 tools/validation/validate_piping_loop_receipts.py --repo-root .`;
9. from the repository root, `git diff --check` and exact write-fence
   containment; and
10. all configured pull-request checks, with actual pass/fail results recorded.

The standing workplan requires the DEC-025 five-surface sweep as the pre-push
merge gate for every code-touching branch; it does not itself prescribe a
multi-commit sequence. This candidate proposes the commit-bound pattern already
used by the two cited DEL-05-02 runs: implementation commit, clean-head sweep,
then a separate closeout commit for the generated evidence and parent-validated
receipt application. The sweep must cover the standing Rust, Python,
WASM/Vitest, Playwright, and desktop-build surfaces. A missing configured
hosted check is recorded as absent, not converted into a passing claim.

### 8.3 Required negative and mutation proofs

- selected exact point without `G` blocks;
- either interpolation endpoint without `G` blocks;
- invalid, zero, nonfinite, or dimensionally wrong `G` blocks;
- outside-range temperature blocks without extrapolation;
- both DEC-077 selectors together block;
- changing base `G` cannot change a selected-temperature-basis torsion result;
- changing selected point `G` changes the result by the hand-calculated amount;
- removing the selector restores base-`G` behavior; and
- no authoring or operation path supplies a hidden value.

## 9. Risks and Controls

| Risk | Control |
| --- | --- |
| Material-library and product-preview temperature-point shapes drift | N1 maps the contract explicitly; schema and bridge tests must fail on mismatch |
| An axial or thermal-only test never observes `G` | require a pure-torsion oracle and mutation proof |
| Existing base-`G` clone survives behind a helper | vary base and selected `G` independently; require missing-point `G` to block |
| Pa/MPa handling creates an apparent interpolation result | normalize units before interpolation and test mixed accepted stress units |
| Legacy temperature-selected models silently change | block with a specific diagnostic; no default or auto-migration |
| Operation corpus case numbers collide or imply inherited review | freeze fresh IDs 76–78 and review their entries independently |
| UI work expands into model-tree redesign | limit authoring to the existing material-library surface; amend before any additional UI path |
| Generic result metadata proves structurally insufficient | stop and request a fence amendment; do not edit results schema implicitly |
| Parallel nodes overlap on shared Rust/schema surfaces | sole-owner table plus serialized N6 integration |
| Generated manual becomes stale | record explicit DEL-09-04 derivative deferral and later routed follow-up |
| Closeout overstates acceptance | distinguish validated candidate head, owner integration, and later lifecycle acts |

## 10. Derivative and Handoff Disposition

- Authoritative implementation truth, if adopted and completed, is the tested
  source plus its accepted repository commit.
- Benchmark, hand-calculation, runtime summary, and evidence sweep are
  derivative evidence bound to that commit.
- Validation-manual regeneration is `DEFERRED` to DEL-09-04. The adopted run
  may prepare a local draft handoff, but dispatch or foreign-path writing
  requires the receiving loop/deliverable's ordinary cadence and authority.
- No DAG change is proposed. No pointer update is proposed.
- No dependency-state reinterpretation is proposed.

## 11. Gate 1 Decision Interface

The owner may rule:

- **ADOPT** — authorize materialization of a separate, adopted WORKING_ITEMS
  activation from this brief, followed by branch-first bounded implementation
  inside the exact fence and the validation plan above.
- **AMEND** — return exact changed predicates, fence paths, work-graph nodes, or
  evidence requirements for a revised candidate.
- **DEFER** — preserve the candidate without activation and name the trigger for
  reconsideration.
- **DECLINE** — close the candidate without implementation.

**Nonbinding WORKING_ITEMS recommendation:** `ADOPT`. The durable owner ruling,
DEC-092 row, active DAG posture, and DEL-05-02 Remaining item align, and the
implementation gap is live and bounded. This recommendation does not replace
the owner's act.

If adopted, the exact next act is to reverify the live basis, record the Gate 1
ruling in a separate activation record, obtain the CHANGE branch gate, and only
then instantiate the sealed N1–N5 briefs. Adoption of this candidate alone:

- does not change lifecycle state;
- does not authorize release or publication;
- does not authorize professional reliance;
- does not accept a DAG or update a pointer; and
- does not authorize merge.

Owner integration remains a separate later gate after validated closeout.

## 12. Candidate Closure Statement

This file is the sole durable output of the candidate-preparation run. It
records a proposal and current-state evidence at the frozen basis. It contains
no implementation result, no dispatched return, no validation pass, and no
claim that DEC-092 is closed.
