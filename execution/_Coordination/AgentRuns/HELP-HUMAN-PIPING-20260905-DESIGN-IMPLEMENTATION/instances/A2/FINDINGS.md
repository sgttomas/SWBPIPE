# A2 architecture witnesses — baseline findings

Bounded evidence execution for HELP_HUMAN, source baseline `7458e9c1eb9399ed259da464207d9a507acdea2e`; no product source changes by A2. This is derivative verification evidence, not accepted decomposition, numerical suitability validation, release or professional acceptance. Native descendant role and nondelegation are instruction+config asserted; runtime model was not exposed.

## Outcome

| Witness | Executed result | Calibration |
|---|---|---|
| MAP-020 native m/mm/in coordinates | Equivalent authored geometry produced unequal displacement; all three runs reported MECHANICS_SOLVED. | **Reproduced native compilation defect** through actual product function and verified upstream command source chain. Transport and packaged app were not exercised. |
| MAP-019 canonical trace/profile seam | Canonical transform and internal adapter deterministic; 7 trace links, source unchanged, no blocking findings; all 24 existing focused Python tests pass. Native rejects canonical analytical wire document directly. | **Verified existing canonical strengths and integration gap**. No nonexistent bridge or cross-engine numerical equivalence is claimed. |
| MAP-029 state/run identity and persistence | Changed geometry changed computed displacement but retained same run/model refs. Unchanged native SQLite helpers preserved latest payload across close/reopen, then overwrote its project row on second save. | **Verified identity reuse and current-snapshot storage behavior**. The storage driver bypassed UI/full command validation and used explicitly labeled pass-through run/hash carriers. |
| MAP-021 nonlinear solution/recovery | Generic and final nonlinear displacement rows differ at the same attached DOF; 758 comparable generic response/force/stress rows remain exactly unchanged when nonlinear supports are removed. | **Reproduced divergent result surfaces** plus source-confirmed recovery using linear vector. No reference engineering solution or threshold acceptance claimed. |

## 1. Coordinate equivalence: repair before trusting non-metre authoring

The invented straight cantilever uses the repository's explicitly invented material/section values and a 350 N transverse nodal load. It has a 2 m span represented as 2 m, 2000 mm or 78.74015748031496 in. Only raw node positions and declared project length unit change; explicitly unit-bearing section/material/load records do not. All three results report mechanics solved:

| Coordinate storage | Maximum displacement returned (mm) |
|---|---:|
| m | 0.406023 |
| mm | 406023368.175308 |
| in | 24777.066116 |

These values are outputs under comparison, not reference answers. The six-decimal output policy limits direct equality precision. The magnitude of this mismatch is independent of that rounding.

The [frozen units packet](snapshots/UNITS_V1/FINDING.md) captures fixtures, actual Rust driver, raw results, toolchain, lock and full relevant baseline sources. App.tsx629 clones the model; its clonePreviewModel2781 is only JSON cloning. previewService.ts94 passes the model unchanged to the native job; native lib.rs1595 invokes solve_preview_mechanics_with_mode1449, which deserializes directly and calls the actual function used by the driver. Product Project117 retains only id, and build_model3013 passes raw positions to FrameNode. normalize_model_units3856 does not normalize these coordinates. operation_applier2973 explicitly treats node positions as project.units.length storage, so an SI-only caller assumption is incompatible with the product authoring route.

Minimal repair belongs to **DEL-02-02 / SOW-025 / OBJ-001, OBJ-012**, coordinated with **DEL-13-04 / SOW-066 / OBJ-014** and native mechanics **DEL-04-01**. Read project units, normalize once at compilation, preserve the authored document, and reject unsupported metadata through the existing diagnostic policy. Legacy missing-unit behavior needs an existing migration/owner basis; do not silently invent one. Re-run equivalent units including translated/non-origin geometry and invalid metadata through the repaired native boundary. This narrow regression does not close PDU-048 numeric suitability or PDU-037 broader unit integration.

## 2. Canonical mapping exists; the product bridge does not yet exist

[Canonical transform output](results/canonical_transform.json) and [adapter output](results/canonical_adapter.json) demonstrate PHYS-1 → ANALYTICAL-DERIVED, stable N-1/N-2 identities and deterministic node indices, E-1 connectivity, property binding, governed local orientation, explicit support targeting, and supplied load-case records. The existing fixture's two nodes remain [0,0,0] and [2,0,0], with coordinate units and source provenance retained. Seven trace links survive deterministic reruns without mutating source input. All 24 existing transform/adapter tests passed after installing the project-pinned development dependencies in an isolated environment; the initial schema-test dependency blockage was resolved, not waived.

Passing the canonical analytical document to the actual native Rust deserializer fails with `invalid type: map, expected a string`. The scratch driver's unwrap reports this as exit 101; the real native wrapper returns a deserialize error, so **the witness does not establish a native app crash**. It establishes incompatible wire contracts. The canonical adapter describes itself as an internal DTO adapter, not a preview runtime or load application engine. Its trace mapping is useful reusable evidence, not proof that every native component/load/nonlinear family is covered by an integrated profile.

**DEL-13-04 owns the bridge contract**; DEL-02-01 owns relevant schema and DEL-14-02 the eventual result/run binding. First define a versioned explicit straight-linear profile, source revision and adapter-settings identity, supported input families, omission/blocking diagnostics, and mapping to the existing native assembly. Then create one equivalent fixture at both boundaries and compare the actual normalized analytical assembly/response through an agreed adapter. Do not silently substitute the canonical adapter for the wider native compiler.

Current paired status/memory reads preserve **PDU-023** (runtime result-envelope producer/home) and **PDU-047** (broader 3D mechanics suitability). Executing these witnesses appoints no producer, changes no home and closes neither hold. Those owner decisions precede the integrated bridge slice.

## 3. Latest save/reopen works; immutable lineage needs a product contract

Changing the invented native span from 2 m to 3 m changes maximum displacement from 0.406023 mm to 1.370329 mm, while both outputs retain `run:preview-linear-static-001` and `project:invented-loop-01`. This is a direct actual-function witness of run-ID reuse across distinct models. The source's buildAnalysisRunPreview240–243 additionally binds model-state identity to `state:{project}:preview`; that TypeScript builder was read, not executed here.

The native persistence driver extracts **verbatim baseline helper sections** from lib.rs556 through the migration helpers and from now_unix_seconds through load_project; [extraction hashes](results/persistence_extraction.json) bind exact ranges. It executes migration v0→v11, save, connection close/reopen, verifies the first model/result/run/hash carriers exactly, saves changed content for the same project, closes/reopens and verifies the second payload exactly. There is one local_projects row after two saves; load_project returns the second payload and this schema exposes no separate native state/run-history tables. This does not disprove preservation of caller-supplied canonical history inside a JSON payload; that separate canonical producer route was not exercised. The relevant [raw result](results/persistence.json) also lists all tables.

This is a **current-envelope persistence witness**, not a full valid-project save test: synthetic run and hash carriers are intentionally labeled pass-through values, and full command validation, model migration and UI session undo were bypassed. It does not show invalid input is accepted by the actual save command. No user database was opened.

**DEL-14-01/SOW-071 and DEL-14-02/SOW-072/OBJ-016** need immutable state/run producer and identity semantics; **DEL-02-05/SOW-050, SOW-041/OBJ-001, OBJ-012** needs the corresponding persistence/migration integration. Choose the relationship between content identity and execution identity: deterministic model hashes may deduplicate states, while repeated runs need an explicit distinct-execution policy; distinct submitted models must never alias merely through project name. Preserve current input-manifest hashes and compatibility. Decide whether a first implementation persists immutable state/run members in canonical project history or dedicated indexed storage, then specify stale/current run behavior and save/reopen history tests. This evidence does not reopen or settle `.opsproj` compatibility/container residuals, the stage-gated Phase G program, PDU-033 or external-reference privacy screening.

## 4. Nonlinear outputs must not silently imply one coherent selected solution

With the existing invented full preview fixture, the first load case produces:

| Attached DOF | Generic node result (mm) | Nonlinear final-support result (mm) |
|---|---:|---:|
| N-140 / NL-140 / UY | 0.332485 | 0 |
| N-130 / NL-130-FRIC / UZ | -3.530824 | -5.469167 |

A second run removes only the nonlinear supports. All **758 comparable generic rows** across displacement, rotation, restraint reactions, local forces/moments, local stresses, pressure hoop and stress summaries remain exactly equal. [Comparison evidence](results/identity_and_nonlinear_summary.json) names the compared families; complete results remain available beside it.

Source confirms solve_load_case1225 fills the response vector from the linear solve; generic results/reactions use it; append_nonlinear_support_loop_results1329 appends separate rows without returning a replacement vector; straight/bend force recovery1353/1374 still consumes that original vector, and endpoint stress recovery1467 follows those recovered forces. The separate nonlinear helper emits its own solve.displacements1795.

The reproduced fact is **two response surfaces and linear-vector recovery**, not an asserted validated physical answer. Current preview naming may intentionally expose both, but the redesigned product must choose and label the result basis explicitly. **DEL-04-04/SOW-012/OBJ-003** should define the authoritative converged selected solution returned to **DEL-05-03/SOW-015/OBJ-003** recovery and **DEL-07-05** inspection. A narrow next repair/witness can make the selected displacement/reaction vector an explicit return value, propagate it to forces/stresses, and compare a simple supplied active/released support case with an independently reviewed mechanics basis. Until then the first connected slice should remain explicit linear. Existing PDU-035, friction path-history D-XX and DEC-046/052/054 threshold-promotion holds remain intact.

## Handoff

Accepted upstream authority remains SOFTWARE_DECOMP revision0.12, accepted SCA-009 and approved DAG-010, with report/map used as derivative planning context. [Source-read fingerprints](_run_records/SOURCE_READS.json) include governing contract, fixtures/tests and paired deliverable status/memory read records. Applicable constraints include OPS-K-UNIT-1, DATA-2/3, SOLVER-1/2, AUTH-1/2, MECH-1/2, REPORT-1 and AGENT-2/3/4. No standards equation extraction, private data, live provider, app UI interaction, scope amendment or lifecycle change occurred.

Evidence execution is complete for parent review. Root has received the frozen units defect early for owner-routed repair. Other findings require package-scoped briefs and the identified contract choices before production changes. Re-run source-dependent witnesses after repairs; retain this baseline packet rather than revising old observations. Full native walkthrough, bridge numerical equivalence, immutable history implementation and selected nonlinear solution recovery are **remaining implementation/validation work**, not completed by these narrow witnesses.
