# R3 T12 — Unreached engines (F7, `PRODUCT_CALLER: NONE`)

T12 covers the 666 rows of `R3/CORPUS_CLAIMS.csv` with `ProductCallerNone = YES`.
641 are effectively `ALIGNED` and 25 are divergent. Each row has code and tests
behind it, but no product path calls the cited code. The rows reach 46 distinct
engines and 41 deliverables, which sit in 13 packages and 6 implementation areas plus `UNMAPPED`.
They fall into eight clusters. The largest pattern, 308 rows in T12-C01 and
T12-C02, is a Python reference engine or schema in PKG-15, PKG-17 and PKG-14,
while the desktop app builds its own TypeScript preview of the same record and
never calls the engine. PKG-13's design-knowledge, constraint and transform
stack (112 rows, T12-C03) has no consumer at all. Three smaller clusters show
different things: the plugin, adapter and API contracts behind the DEC-012 hold
(T12-C04), the DEL-12-04 security helpers waiting on PDU-034 (T12-C05), and
Python or crate duplicates that a product port in another engine has replaced
(T12-C06). The last two clusters, unused API surfaces inside engines the product
does reach (T12-C07) and test harnesses and fixtures (T12-C08), need no action.
Under F7 each engine claim is satisfied by the engine itself. The open question
T12 routes is at the engine level: should the product call the engine, port it,
keep it as a reference or parity oracle, or retire it? The per-row route is in
`T12_UNREACHED.csv`. Routes are proposals. Nothing here re-disposes a row.

`Engine` is the module or file-level unit that the row's `ImplementationEvidence`
cites first. It is a `core/<area>/<unit>` package, crate or file, a schema or
contract file, or a fixture set. `Area` is the `IMPLEMENTATION_SURFACES.csv`
area whose `EntryPoints` contain that engine. Every engine that has an area maps
to exactly one; the only `UNMAPPED` engine is the example fixture set.

## Unreached engines

| Engine | Area | Rows | Deliverables | Cluster |
|---|---|---|---|---|
| core/handoff/target_mapping | COREC | 36 | DEL-15-02 | C01 |
| core/handoff/caepipe_mbf | COREC | 36 | DEL-17-04 | C01 |
| schemas/handoff_package.schema.json | DATA | 32 | DEL-15-01 | C01 |
| core/handoff/external_prover | COREC | 28 | DEL-15-04 | C01 |
| core/handoff/native_json | COREC | 25 | DEL-17-03 | C01 |
| core/handoff/caepipe_external | COREC | 22 | DEL-17-05 | C01 |
| core/handoff/exporter | COREC | 17 | DEL-15-03, DEL-15-02 | C01 |
| schemas/target_mapping.schema.json | DATA | 9 | DEL-15-02 | C01 |
| core/handoff/{stress_neutral, export_adapter_sdk, pcf_export, review_geometry}; schemas/native_json_export.schema.json | COREC / DATA | 2+2+1+1+2 | DEL-17-06, -09, -07, -08, -03 | C01 |
| core/comparison/analysis_run | COREC | 30 | DEL-14-04 | C02 |
| core/comparison/model_state | COREC | 21 | DEL-14-03 | C02 |
| schemas/comparison_mapping.schema.json; comparison_tolerance.schema.json | DATA | 20+2 | DEL-14-05 | C02 |
| schemas/model_state.schema.json | DATA | 14 | DEL-14-01 | C02 |
| core/reporting/state_comparison_handoff_sections | COREC | 8 | DEL-08-06 | C02 |
| schemas/constraint.schema.json | DATA | 52 | DEL-13-02 | C03 |
| schemas/design_knowledge.schema.json | DATA | 25 | DEL-13-01 | C03 |
| core/constraints/validation | COREB | 21 | DEL-13-03 | C03 |
| core/model_transform/physical_to_analytical | PHYS | 14 | DEL-13-04 | C03 |
| schemas/plugin_manifest.schema.yaml | DATA | 21 | DEL-02-04 | C04 |
| core/adapters/framework | COREC | 19 | DEL-02-04, DEL-10-02, DEL-00-07 | C04 |
| api/api_boundary_contract.yaml | DATA | 16 | DEL-10-01 | C04 |
| schemas/adapter_framework.schema.yaml | DATA | 1 | DEL-10-02 | C04 |
| core/security/secret_private_library; local_first_storage | COREC | 26+1 | DEL-12-04 | C05 |
| core/section_properties/calculator.py | SOLVER | 52 | DEL-03-08 | C06 |
| core/loads/user_loads (crate) | SOLVER | 38 | DEL-05-05 | C06 |
| core/library_import/provenance_checker.py | COREB | 3 | DEL-03-07 | C06 |
| core/gui/accessibility (and sibling core/gui Python engines) | COREB | 1 | DEL-00-03 | C06 |
| core/solver/{diagnostics, linear_supports, frame_kernel, straight_pipe} | SOLVER | 9+2+1+1 | DEL-04-06, -03, -01, -02 | C07 |
| core/loads/{primitive_loads, stress_recovery, load_case_algebra} | SOLVER | 6+5+1 | DEL-05-01, -03, -02 | C07 |
| core/reporting/{audit_manifest, report_generator, report_renderer} | COREC | 6+2+1 | DEL-08-02, DEL-12-04, DEL-08-01, DEL-08-03 | C07 |
| core/solver/performance_harness | SOLVER | 11 | DEL-04-05 | C08 |
| examples/models/invented (fixture set) | UNMAPPED | 22 | DEL-11-04 | C08 |
| schemas/model.schema.yaml | DATA | 1 | DEL-11-04 | C08 |

Engine attribution: 637 rows from the first code path in ImplementationEvidence,
2 from Notes (native_json, where evidence cites only fixtures), 2 from a
crate README (performance_harness), 22 from the fixture set, and 3 test-only rows
attributed to the unit under test (DEL-14-03:SOW#CLM-014 to
core/comparison/model_state, DEL-15-01:SOW#CLM-013/V-07 to the handoff package
schema, DEL-15-03:SOW#CLM-006.r03 to core/handoff/exporter), per their Notes.

## Clusters

### T12-C01 — Handoff and export engines shadowed by desktop previews (213 rows, 16 divergent)

- **Description.** PKG-15 and PKG-17 each built a Python builder, and sometimes a
  JSON schema, for a handoff or export record. The desktop app emits its own
  technical-preview record for the same concept and never calls the builder or
  validates against the schema. The engines are target mapping, handoff package,
  exporter, external-prover metadata, native JSON, CAEPIPE MBF, CAEPIPE external
  run, stress-neutral 0.1, PCF, review geometry and the export-adapter SDK.
  Under F7 the builder satisfies each engine claim. Product conformance is
  judged on other rows, for example DEL-15-03 and FG-DEL-15-04-01.
- **Signature.** 197 rows are ALIGNED with no cause. 16 are PARTIALLY_IMPLEMENTED
  at tier INVARIANT or LOCAL_DESIGN: DEL-15-02 has 12 with POSSIBLE_DEFECT and 3
  with PARTIAL_SLICE (FG-DEL-15-02-01/02/03), and DEL-15-04 OUT-001 has 1 with
  PARTIAL_SLICE.
- **Packages and deliverables.** PKG-15 (122): DEL-15-01 32, DEL-15-02 46,
  DEL-15-03 16, DEL-15-04 28. PKG-17 (91): DEL-17-03 27, DEL-17-04 36,
  DEL-17-05 22, DEL-17-06 2, DEL-17-07 1, DEL-17-08 1, DEL-17-09 2.
  Areas are COREC and DATA.
- **Owning authority.** OWNER.
- **Route.** `OWNER_DECISION` for the 197 ALIGNED rows. `CODE_FIX_CANDIDATE`
  for the 16 divergent rows. The target-mapping defects are real in the engine,
  but a fix is worth briefing only if the owner keeps the engine (see the R3
  observations).
- **Decision.** Which implementation is canonical for each handoff record?
  These are the options as they stand in the evidence:
  - (a) Wire the desktop export to call, or validate against, the Python
    builder and schema through a runtime service.
  - (b) Port the contract into the desktop or Rust path and keep the Python
    builder as a parity oracle. This is the precedent set by DEL-03-07, where
    the product calls a Rust port of the same contract.
  - (c) Declare the desktop preview canonical and retire or re-scope the Python
    builder.
  Stress-neutral already splits by version: the Python 0.1 builder is test-only
  and the product emits the 0.2 family (DEL-17-06 Notes).
- **On-ruling mechanism.** An owner ruling at R4 names the canonical path for
  each record family. Under (a) or (b), a CODE_FIX_CANDIDATE brief goes to the
  WORKING_ITEMS change path (chirality-change), and the FG-DEL-15-02-01/02/03
  repairs join it. Under (c), a SCOPE_CHANGE_HANDOFF to the scope-change
  workflow re-scopes the DEL-15-0x and DEL-17-0x SOWs to the desktop record,
  and R5 record repairs follow.
- **Risk if left.** Two record shapes for one handoff concept drift apart. The
  schema-validated guarantees, including provenance, stable IDs, loss reports
  and blocking diagnostics, are then tested only where the product does not
  run. The desktop preview carries no schema conformance check.
- **Representative keys.**
  - DEL-15-02:SOW#CLM-012.r01. `build_target_mapping_contract` is defined at
    core/handoff/target_mapping/contract.py:67 and imported only by tests. The
    desktop builds its own `target_mapping` object at
    apps/desktop/src/features/handoff/HandoffPanel.tsx:236.
  - DEL-17-03:SOW#CLM-010/DEL-17-03-REQ-002. `build_native_json_export_package`
    is at core/handoff/native_json/package.py:78. No app or runtime module
    imports it, and the desktop NativePackagePanel builds its own review record.
  - DEL-15-02:SOW#CLM-005.r02 is divergent (FG-DEL-15-02-01): the builder fills
    absent inputs silently in an engine that has no product caller.
- **Visible exceptions.** DEL-15-02:SOW#CLM-020 is FIRM, and the verifier gives
  two options: re-dispose, or split step 2 as a new `.s04`.
  DEL-15-04:SOW#purpose-and-objective-traceability/OUT-001 is WEAK, with
  AuthorityNeeded NO and two readings surviving.
  DEL-15-04:SOW#completion-and-reliance-basis-epistemology/AC-001 is CONTESTED:
  ALIGNED as a pure restriction, or PARTIALLY_IMPLEMENTED on the CP-11 reading.

### T12-C02 — Comparison and model-state engines shadowed by desktop previews (95 rows, 5 divergent)

- **Description.** PKG-14's analysis-run comparison engine, its model-state
  comparison engine, the comparison and model-state schemas, and DEL-08-06's
  state, comparison and handoff report sections are reached only from tests.
  The desktop Comparison panel is a separate TypeScript preview. The report
  projection is a separate TypeScript module, and the desktop project store
  persists model and run envelopes but no model-state records.
- **Signature.** 90 rows are ALIGNED. 5 are PARTIALLY_IMPLEMENTED ·
  PARTIAL_SLICE: 4 in DEL-14-01 at PROJECT_BASELINE and 1 in DEL-14-03 at
  LOCAL_DESIGN.
- **Packages and deliverables.** PKG-14 (87): DEL-14-01 14, DEL-14-03 21,
  DEL-14-04 30, DEL-14-05 22. PKG-08 (8): DEL-08-06 8. Areas are COREC and DATA.
- **Owning authority.** OWNER.
- **Route.** `OWNER_DECISION` for 94 rows. `CODE_FIX_CANDIDATE` for
  DEL-14-03:SOW#CLM-012/REQ-14-03-001: the comparison tests do not validate
  against the existing state schema.
- **Decision.** Does the product persist named model states and compute
  comparisons through the Python engines? The options as they stand:
  - (a) Wire the engines through a runtime service.
  - (b) Port them to the desktop or Rust path, with the Python engines kept as
    parity oracles.
  - (c) Accept the TypeScript previews as the product and re-scope PKG-14.
  DEL-14-01:SOW#CLM-011.r01 records the product-behaviour gap: no desktop
  command saves a named model state. The hash-basis rows DEL-14-01 CLM-004,
  CLM-005 and CLM-011.r04 belong to the corpus DEC-009 cluster (T8) and are
  routed with it.
- **On-ruling mechanism.** Same as T12-C01. Under (a) or (b), a code brief
  goes through the change path. Under (c), a scope-change handoff re-scopes
  DEL-14-01/03/04/05 and DEL-08-06, and R5 record repairs follow.
- **Risk if left.** Comparison and state guarantees are checked only off the
  product path. The DEC-009 hash-basis choice cannot bind any product record
  while no product record exists.
- **Representative keys.**
  - DEL-14-04, all 30 rows. `compare_analysis_runs` is at
    core/comparison/analysis_run/engine.py:165 and reached only from tests.
    The desktop has its own apps/desktop/src/features/comparison/ComparisonPanel.tsx.
  - DEL-14-03:SOW#CLM-014. `compare_model_states` is at
    core/comparison/model_state/engine.py:64 and called only by its tests.
  - DEL-08-06. The Python sections engine is unreached, and the desktop uses
    apps/desktop/src/features/report/stateComparisonHandoffSections.ts.
- **Visible exceptions.** Three rows are CONTESTED:
  - DEL-08-06:SOW#completion-and-reliance-basis-epistemology/AC-001. PKG-08
    judges AC-001's subject differently across deliverables (contract or
    engine).
  - DEL-14-01:SOW#CLM-005, part of the DEC-009 cluster.
  - DEL-14-01:SOW#CLM-024, which turns on how far F1 reaches on CONTEXT purpose
    rows.

### T12-C03 — Design-knowledge, constraint and transform stack with no consumer (112 rows, 2 divergent)

- **Description.** PKG-13 as a whole. The design-knowledge schema and the
  constraint schema are loaded only by their structural tests. The DEL-13-03
  validation engine restates the constraint vocabulary and is imported only by
  its test. The DEL-13-04 physical-to-analytical transform is called only from
  tests, and the handoff exporter names its contract only as a path string.
  Unlike C01 and C02, the evidence records no desktop counterpart.
- **Signature.** 110 rows are ALIGNED. 1 is IMPLEMENTED_DIFFERENTLY ·
  POSSIBLE_DEFECT · PROJECT_BASELINE (DEL-13-01), and 1 is
  PARTIALLY_IMPLEMENTED · PARTIAL_SLICE · LOCAL_DESIGN (DEL-13-03).
- **Packages and deliverables.** PKG-13 (112): DEL-13-01 25, DEL-13-02 52,
  DEL-13-03 21, DEL-13-04 14. Areas are DATA, COREB and PHYS.
- **Owning authority.** OWNER.
- **Route.** `OWNER_DECISION` for 111 rows. `REVIEW` for
  DEL-13-01:SOW#CLM-005.r05, where the FIRM correction sets AuthorityNeeded
  REVIEW: add `force_per_length` to `Quantity.dimension`, or derive the test set
  from the units schema.
- **Decision.** Is the PKG-13 stack in product scope for the current baseline?
  The options as they stand:
  - (a) Wire constraint validation and the transform into the product's
    authoring or solve path.
  - (b) Hold it as a staged foundation, with no product obligation now, and
    record it as such.
  - (c) Retire or merge it through scope change.
- **On-ruling mechanism.** Under (a), a code brief goes through the change path.
  Under (b), R5 record repairs mark the deliverables as foundation-only, and the
  lifecycle is checked by T9. Under (c), a SCOPE_CHANGE_HANDOFF goes to the
  scope-change workflow.
- **Risk if left.** Constraint and provenance validation exists but guards no
  product input. Records may be read as though product data is constrained when
  it is not.
- **Representative keys.**
  - DEL-13-03, 20 ALIGNED rows. `validate_constraint_envelope` is at
    core/constraints/validation/engine.py:157, and only
    tests/test_constraint_validation.py imports it.
  - DEL-13-04. `transform_physical_to_analytical` is at
    core/model_transform/physical_to_analytical/contract.py:113 and called
    only from tests.
  - DEL-13-02, 52 rows. The constraint schema is consumed only by its test.
- **Visible exceptions.**
  - DEL-13-01:SOW#CLM-005.r05 is FIRM (REVIEW).
  - DEL-13-02:SOW#CLM-003.r15 and SOW#CLM-023 are CONTESTED. There are two
    readings of the stale INIT.md pointer: ALIGNED, or STALE_REVIEW_OR_EVIDENCE ·
    BASIS_POINTER_STALE.
  - DEL-13-02:SOW#CLM-008 and SOW#CLM-010/R-13-02-006 are OBSERVED, with the
    same pattern.
  - DEL-13-03:SOW#CLM-005.r04 is WEAK, with AuthorityNeeded NO. Its values hold
    only on a paths reading of the claim (see the R3 observations).

### T12-C04 — Plugin, adapter and API boundary contracts with no runtime (57 rows, 0 divergent)

- **Description.** DEL-02-04's plugin manifest schema and verifier, DEL-10-01's
  API boundary contract, and DEL-10-02's adapter framework (with the DEL-00-07
  REQ-07-02 row that cites it) are consumed only by their contract tests. No
  plugin loader, API server or adapter runtime exists at the freeze. Three
  things explain this: the contested rows record that there is no plugin
  runtime because DEC-012 is held; DEL-10-01 CLM-007 declares that the
  deliverable "records a contract boundary, not an implementation"; and
  api/api_boundary_contract.yaml:27 carries `plugin_runtime` as TBD. For
  DEL-00-07, the product's library-import path validates provenance through
  the Rust library-import crate instead.
- **Signature.** All 57 rows are ALIGNED.
- **Packages and deliverables.** PKG-02 (33): DEL-02-04 33. PKG-10 (23):
  DEL-10-01 16, DEL-10-02 7. PKG-00 (1): DEL-00-07 1. Areas are DATA and COREC.
- **Owning authority.** NONE for DEL-02-04, DEL-10-01 and DEL-00-07, which are
  contract-by-scope with the runtime held. OWNER for DEL-10-02.
- **Route.** `NO_ACTION` for 50 rows: the contract is the deliverable and the
  runtime is held by ruling (DEC-012). `OWNER_DECISION` for the 7 DEL-10-02
  rows. They show the C01 pattern: `validate_adapter_declaration`
  (core/adapters/framework/adapter_framework.py:539) and
  `gate_adapter_runtime_dispatch` are called only by tests, while
  apps/desktop/src/features/adapter-framework/AdapterFrameworkPanel.tsx builds
  its own preview packet.
- **On-ruling mechanism.** None while DEC-012 holds. Lifting DEC-012 is the
  reopening trigger for DEL-02-04 and DEL-10-01, through the owner's ruling and
  then a code brief. DEL-10-02 follows the C01 decision options.
- **Risk if left.** Low while the hold stands. If a plugin or adapter runtime is
  built later without these contracts, the tested guards (privacy defaults,
  quarantine, required guards) could be bypassed.
- **Representative keys.**
  - DEL-02-04:SOW#CLM-012/DEL-02-04-REQ-18. The PrivacyDeclaration defaults
    are fixed and the verifier rejects other values. The verifier is imported
    only by tests.
  - DEL-10-01:SOW#CLM-013/DEL-10-01-REQ-05. The contract's `required_guards`
    are read only by contract tests.
  - DEL-10-02, 6 rows citing core/adapters/framework. The desktop panel builds
    its own preview.
- **Visible exceptions.** Four rows are CONTESTED:
  DEL-02-04:SOW#CLM-012/DEL-02-04-REQ-12 (the hash basis coincides with JCS
  only for this content), DEL-02-04:SOW#CLM-012/DEL-02-04-REQ-18 (holds by
  construction because no runtime exists), DEL-10-01:SOW#CLM-006 and
  DEL-10-01:SOW#CLM-013/DEL-10-01-REQ-05 (contract reading; the verifier would
  not re-dispose).

### T12-C05 — Security and private-library helpers awaiting policy (27 rows, 1 divergent)

- **Description.** DEL-12-04's secret and private-library controls
  (core/security/secret_private_library/controls.py) are imported only by their
  tests. The one local-first storage row cites the same unreached helper. The
  helper keeps records metadata-only and secrets reference-only, never promotes
  an unknown status to public, and blocks by default. Delivery at product
  boundaries is assessed on the CLM-011 requirement rows.
- **Signature.** 26 rows are ALIGNED. 1 is PARTIALLY_IMPLEMENTED · PARTIAL_SLICE ·
  INVARIANT (DEL-12-04:SOW#CLM-024).
- **Packages and deliverables.** PKG-12 (27): DEL-12-04 27. The area is COREC.
- **Owning authority.** OWNER. The FIRM correction on CLM-024 sets
  AuthorityNeeded to OWNER, because the PDU-034 quarantine policy and the plugin
  grant model are held.
- **Route.** `OWNER_DECISION` for all 27 rows.
- **Decision.** Where, if anywhere, should the product enforce private-library
  and secret handling? The options as they stand:
  - (a) Settle PDU-034 and wire the helper at the import, report and export
    boundaries.
  - (b) Keep the helper as a design reference until the plugin grant model is
    decided.
- **On-ruling mechanism.** An owner ruling on PDU-034 authorises a code brief
  through the change path. Without it, R5 records the helper as reference-only.
- **Risk if left.** The security boundary is tested in isolation and does not
  guard any product path. It could be mistaken for enforced behaviour.
- **Representative keys.**
  - DEL-12-04:SOW#CLM-024 (FIRM).
  - The 12 DEL-12-04 rows whose Notes read "imported only by its tests".
- **Visible exceptions.** DEL-12-04:SOW#CLM-024 (FIRM).

### T12-C06 — Engines replaced by a product port elsewhere (94 rows, 1 divergent)

- **Description.** A tested Python module or Rust crate whose job the product
  does through a different implementation:
  - The DEL-03-08 Python section calculator
    (`calculate_pipe_section_properties`, core/section_properties/calculator.py:63).
    The product derives sections in Rust at `derive_pipe_section`,
    core/product_physics/src/lib.rs:6321 (DEL-03-08_notes.md, lines 22–32).
  - The DEL-05-05 `open_pipe_stress_user_loads` crate. Its only non-test
    caller is the validation benchmark
    (validation/benchmarks/mechanics/Cargo.toml:19). The product authors and
    solves the three load kinds through primitive_loads and straight_pipe.
  - The DEL-03-07 Python provenance checker. The product calls a Rust port of
    the same contract.
  - The core/gui Python engines named by DEL-00-03. They have no desktop caller.
- **Signature.** 93 rows are ALIGNED. 1 is STALE_REVIEW_OR_EVIDENCE ·
  RECORD_DRIFT · LOCAL_DESIGN (DEL-00-03:AB#realized-artifacts.r02).
- **Packages and deliverables.** PKG-03 (55): DEL-03-08 52, DEL-03-07 3.
  PKG-05 (38): DEL-05-05 38. PKG-00 (1): DEL-00-03 1. Areas are SOLVER and COREB.
- **Owning authority.** ENGINEERING.
- **Route.** `ENGINEERING_AUTHORITY` for 93 rows. `R5_RECORD_REPAIR` for the
  DEL-00-03 row: its realized-artifact list names an engine.py for
  core/gui/viewport_editor, which is a Rust crate.
- **Decision.** For each duplicate, engineering decides between two options:
  - (a) Keep it as a named parity or validation oracle, with a parity check
    against the product implementation.
  - (b) Retire it. Retirement or merger goes to scope change.
  DEL-05-05 has an extra open question: its CONTESTED rows ask whether a crate
  with only a benchmark caller should be read PARTIALLY_IMPLEMENTED at
  PROJECT_BASELINE, as DEL-05-02 REQ-05-02-008 and DEL-05-03 RQ-005 are read for
  the same shape.
- **On-ruling mechanism.**
  - An engineering ruling under the validation path.
  - A parity check becomes a CODE_FIX_CANDIDATE test brief.
  - Retirement becomes a SCOPE_CHANGE_HANDOFF.
  - R5 records the chosen status in each SOW.
- **Risk if left.** Two implementations of the same section, load or provenance
  rule can diverge silently. Evidence gathered on the Python or crate twin may
  be read as evidence about the product.
- **Representative keys.**
  - DEL-03-08:SOW#CLM-013.r04 (the calculator module; F7).
  - DEL-05-05:SOW#CLM-010.r02 (user_loads; benchmark-only caller).
  - DEL-03-07, 3 rows (Python module tests-only; Rust port used by the product).
- **Visible exceptions.**
  - Four DEL-05-05 rows are CONTESTED: SOW#CLM-010.r06, SOW#CLM-010.r08,
    SOW#CLM-010.r10 and SOW#completion-and-reliance-basis-epistemology/AC-001.
    The alternatives are PARTIALLY_IMPLEMENTED · PROJECT_BASELINE, or
    VERIFIED_NOT_VALIDATED · VALIDATION_GAP, because the TP-PHYS witnesses were
    produced by agents.
  - DEL-00-03:AB#realized-artifacts.r02 is divergent (RECORD_DRIFT).

### T12-C07 — Unused surfaces inside engines the product reaches (34 rows, 0 divergent)

- **Description.** The product calls the engine, but not the specific surface
  the claim cites:
  - diagnostics helpers for class, remediation, canonical ref and condition
    ratio;
  - `apply_linear_supports`;
  - `FrameKernelUnitBasis`;
  - `StraightPipeElement::weight_hook`;
  - the primitive-load records, lumping and axial-effect helpers
    (product_physics calls `prepare_loads` only);
  - the stress-recovery conventions (the product calls `recover_stresses` only);
  - `to_result_boundary_record`;
  - the reporting rule-pack and asset-entry branches (the product sends no
    rule-pack references and `assets []`).
  Each claim is about the engine surface itself (F7).
- **Signature.** All 34 rows are ALIGNED.
- **Packages and deliverables.** PKG-04 (13): DEL-04-06 9, DEL-04-03 2,
  DEL-04-01 1, DEL-04-02 1. PKG-05 (12): DEL-05-01 6, DEL-05-03 5, DEL-05-02 1.
  PKG-08 (8): DEL-08-02 5, DEL-08-01 2, DEL-08-03 1. PKG-12 (1): DEL-12-04 1.
  Areas are SOLVER and COREC.
- **Owning authority.** NONE.
- **Route.** `NO_ACTION`. These are library API surfaces of reached engines,
  and the claims are satisfied. The reporting rule-pack branch is tied to the
  rule-pack inclusion requirement on FG-DEL-08-01-01. That requirement is
  routed with its own divergent rows, so it needs no T12 action.
- **On-ruling mechanism.** None.
- **Risk if left.** Low. A surface without callers can bit-rot, but its crate
  tests still run. The rule-pack branch is exercised only once the product
  sends rule-pack references.
- **Representative keys.**
  - DEL-04-06:SOW#CLM-013/REQ-04-06-008 (`classify_condition_ratio`; the
    performance harness is its only caller).
  - DEL-05-01:SOW#CLM-014/REQ-05-01-006.
  - DEL-08-02:SOW#CLM-004.r05 (rule-pack path).
- **Visible exceptions.** None.

### T12-C08 — Test harnesses, fixtures and examples (34 rows, 0 divergent)

- **Description.** The DEL-04-05 performance harness and the DEL-11-04 invented
  example models, including the one model-schema row. These are test-suite and
  educational artifacts by scope, so a product caller is not expected.
- **Signature.** All 34 rows are ALIGNED.
- **Packages and deliverables.** PKG-04 (11): DEL-04-05 11. PKG-11 (23):
  DEL-11-04 23. The areas are SOLVER, DATA and UNMAPPED (the fixture set is in
  no implementation area).
- **Owning authority.** NONE.
- **Route.** `NO_ACTION`. The claim is about the harness or fixture itself.
- **On-ruling mechanism.** None.
- **Risk if left.** None identified.
- **Representative keys.**
  - DEL-04-05:SOW#CLM-029 (Notes: "test-suite harness; the claim is about the
    harness itself").
  - DEL-11-04:CONTEXT#description.
  - DEL-11-04:SOW#CLM-028.
- **Visible exceptions.** None.

## Route summary

| Route | Rows |
|---|---|
| OWNER_DECISION | 436 (C01 197, C02 94, C03 111, C04 7, C05 27) |
| ENGINEERING_AUTHORITY | 93 (C06) |
| NO_ACTION | 118 (C04 50, C07 34, C08 34) |
| CODE_FIX_CANDIDATE | 17 (C01 16, C02 1) |
| REVIEW | 1 (C03) |
| R5_RECORD_REPAIR | 1 (C06) |

Owner-decision candidates for R4, one decision per family:

1. C01. Pick the canonical handoff and export path in PKG-15 and PKG-17: the
   Python builder and schema, a port, or the desktop preview.
2. C02. Decide whether PKG-14 model-state persistence and comparison enter the
   product. This is coupled with the DEC-009 cluster (T8).
3. C03. Decide the product status of the PKG-13 stack: wire it, hold it as a
   foundation, or retire it.
4. C05. PDU-034 and the plugin grant model, for DEL-12-04.
5. C04 (DEL-10-02 only). Handled with the C01 decision pattern.

## Coverage

- **Population.** `R3/CORPUS_CLAIMS.csv` rows with `ProductCallerNone = YES`:
  666 rows, of which 641 are effectively not divergent and 25 are divergent.
- **Check.** A script read `T12_UNREACHED.csv` and confirmed:
  - 666 body rows, matching `#END`;
  - 666 unique ClaimKeys;
  - key set equal to the population, with 0 missing and 0 extra;
  - every DeliverableID matching the corpus;
  - every row carrying one ClusterID from T12-C01..C08 and one Route from the
    route vocabulary.
- **Cluster counts.** C01 213, C02 95, C03 112, C04 57, C05 27, C06 94,
  C07 34, C08 34. The total is 666. The divergent counts, 16+5+2+0+1+1+0+0 = 25,
  match the corpus.
- **Area counts.** COREC 284, DATA 195, SOLVER 126, COREB 25, PHYS 14,
  UNMAPPED 22.
- **Sources read.** The sealed forward and reverse ledgers for evidence and
  Notes (`superseded_*` ignored), the `RESOLUTIONS_DRAFT*.csv` files ignored
  per backcheck 1, `IMPLEMENTATION_SURFACES.csv`, `CONVENTIONS.md` F7,
  `DEL-03-08_notes.md` and the frozen source (file:line cited above).

## R3 observations

- **R3_OBSERVATION 1: repair sequencing (C01).** The 15 DEL-15-02 divergent rows
  (FG-DEL-15-02-01/02/03: silent fill of absent inputs, engine provenance
  substituted for absent provenance, presence-only unit checks) are defects in
  an engine that no product path calls. Whether to fix them, or to carry the
  same guards into the desktop record, depends on the C01 owner decision. The
  code brief should follow that ruling, not precede it. This changes no value.
- **R3_OBSERVATION 2: F7 subject reading is inconsistent across rows.**
  - DEL-13-03:SOW#CLM-005.r04 is PARTIALLY_IMPLEMENTED on a paths reading
    (WEAK resolution, AuthorityNeeded NO), while 20 sibling rows on the same
    engine are ALIGNED on the engine reading.
  - DEL-05-05's four CONTESTED rows are ALIGNED, while DEL-05-02 and DEL-05-03
    dispose the same shape as PARTIALLY_IMPLEMENTED.
  - DEL-08-06 AC-001 is CONTESTED because PKG-08 judges AC-001's subject
    differently from deliverable to deliverable.
  These are candidates for one corpus-level F7 ruling at R4, not for per-row
  correction here.
- **R3_OBSERVATION 3: the marker population is a lower bound.** `PRODUCT_CALLER:
  NONE` is required only on ALIGNED rows whose implementation evidence has no
  product caller. A read-only scan found ALIGNED rows that cite these engines
  and carry no marker: 29 citing core/constraints/validation, 10 citing
  core/handoff/target_mapping and 7 citing core/comparison/analysis_run. The
  sampled ones (target_mapping, analysis_run) are absence or restriction claims,
  or rows that also cite a register or desktop path; the constraint-validation
  rows were not sampled. Another 52 rows cite core/gui Python engines alongside desktop files,
  mostly DEL-07-03, DEL-07-04 and DEL-07-06. Rows relying on unreached engines
  may therefore exceed 666. The scan did not reassess those rows.
- **R3_OBSERVATION 4: F7 markers on fixtures.** The 22 DEL-11-04 rows carry
  `PRODUCT_CALLER: NONE` on example fixtures rather than engines, so the marker
  has no routing content there. They are routed NO_ACTION (C08).

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081). This file
describes records and evidence only, and makes no claim of engineering adequacy,
certification or code compliance.
