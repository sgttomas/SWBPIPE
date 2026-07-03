# Decision Log

## Human Ruling

Owner ruling of record (Ryan Tufts, bridge Loop 2 session, 2026-07-02), on the
DEL-10-03 exemplar analysis and the STATUS_HISTORY_MISMATCH class it stands
for, quoted verbatim:

> Yes this is the exemplar and all shall be IN_PROGRESS.

## Exemplar Case

`execution/PKG-10_Build, Packaging, API, and Interoperability/1_Working/DEL-10-03_Local FEA handoff data contract/_STATUS.md`
(DEL-10-03 Local FEA handoff data contract):

- Commit `daab547a2` (2026-06-07, "review: advance PKG-10 deliverables to
  checking") advanced the deliverable to CHECKING in both the header and the
  history; sibling commits advanced the other package cohorts the same way.
- Commit `28219696d` (2026-06-16, "deps: refresh dependency registers and
  lifecycle status") reverted ONLY the header `**Current State:**` line from
  CHECKING back to IN_PROGRESS — verified against the commit diff: 92 header
  lines flipped CHECKING to IN_PROGRESS, zero history lines added — leaving
  every CHECKING history tail with no offsetting entry.
- The resulting class signature is the recorded drift baseline of
  `tools/practitioner_harness/README.md` §Drift baseline: "history records an
  approved advance to CHECKING while the frontmatter `Current State:` line
  still reads IN_PROGRESS" — measured 92 of 154 `_STATUS.md` files, all 92 in
  chirality-piping (101 files), surfaced by the harness `drift` command as
  STATUS_HISTORY_MISMATCH under K-CONFLICT-1 (which surface is right is a
  human call; the harness never resolves it).

## Correction Policy

- The header `**Current State:** IN_PROGRESS` is authoritative for the entire
  STATUS_HISTORY_MISMATCH class (all 92 files listed below).
- No deliverable state changes: every affected deliverable remains
  IN_PROGRESS. No promotion to CHECKING, ISSUED, or any release/professional
  acceptance state is made or authorized.
- Prior history is preserved; exactly one new history entry is appended to
  each affected `_STATUS.md`, dated 2026-07-02, recording the previously
  unlogged 2026-06-16 header reversal from CHECKING and citing this Decision
  Log. Each file's `**Last Updated:**` line is bumped to 2026-07-02. No
  existing history line is edited.

## Rationale

Commit `28219696d` corrected the headers without logging the reversal, so the
status truth (header) and its own audit trail (history) disagreed on 92 files.
Per K-CONFLICT-1 the conflict is a human call; the owner ruled the header
IN_PROGRESS authoritative class-wide. Authoring the missing reversal entries
restores history/header agreement without rewriting any prior assertion,
mirroring the append-only precedent of
`execution/_Reconciliation/LifecycleCorrection/LIFECYCLE_CORRECTION_2026-05-11_2052/Decision_Log.md`.

## Affected Files (92)

- `execution/PKG-01_Governance, IP Boundary, and Professional Responsibility/1_Working/DEL-01-02_Copyright and protected-data boundary policy/_STATUS.md`
- `execution/PKG-01_Governance, IP Boundary, and Professional Responsibility/1_Working/DEL-01-03_Contributor certification workflow/_STATUS.md`
- `execution/PKG-01_Governance, IP Boundary, and Professional Responsibility/1_Working/DEL-01-04_Professional responsibility and product-claims policy/_STATUS.md`
- `execution/PKG-02_Domain Model, Units, and Core Schemas/1_Working/DEL-02-01_Canonical domain model schema/_STATUS.md`
- `execution/PKG-02_Domain Model, Units, and Core Schemas/1_Working/DEL-02-02_Unit system and dimensional-analysis core contract/_STATUS.md`
- `execution/PKG-02_Domain Model, Units, and Core Schemas/1_Working/DEL-02-03_Code-neutral analysis boundary model/_STATUS.md`
- `execution/PKG-02_Domain Model, Units, and Core Schemas/1_Working/DEL-02-04_Plugin and extension domain contracts/_STATUS.md`
- `execution/PKG-02_Domain Model, Units, and Core Schemas/1_Working/DEL-02-05_Project persistence and round-trip serialization/_STATUS.md`
- `execution/PKG-03_Piping Components, Materials, and Library Data Model/1_Working/DEL-03-01_Material library schema with provenance/_STATUS.md`
- `execution/PKG-03_Piping Components, Materials, and Library Data Model/1_Working/DEL-03-02_Pipe section and component library schema/_STATUS.md`
- `execution/PKG-03_Piping Components, Materials, and Library Data Model/1_Working/DEL-03-03_Bend and elbow component model fields/_STATUS.md`
- `execution/PKG-03_Piping Components, Materials, and Library Data Model/1_Working/DEL-03-04_Branch connection component model fields/_STATUS.md`
- `execution/PKG-03_Piping Components, Materials, and Library Data Model/1_Working/DEL-03-05_Rigid component models for valves, flanges, reducers, and specialty items/_STATUS.md`
- `execution/PKG-03_Piping Components, Materials, and Library Data Model/1_Working/DEL-03-06_Expansion joint component model/_STATUS.md`
- `execution/PKG-03_Piping Components, Materials, and Library Data Model/1_Working/DEL-03-07_Public-private library import provenance checker/_STATUS.md`
- `execution/PKG-03_Piping Components, Materials, and Library Data Model/1_Working/DEL-03-08_Pipe section property and mass-property calculator/_STATUS.md`
- `execution/PKG-04_Solver Core and Numerical Methods/1_Working/DEL-04-01_3D frame stiffness kernel/_STATUS.md`
- `execution/PKG-04_Solver Core and Numerical Methods/1_Working/DEL-04-02_Straight pipe element/_STATUS.md`
- `execution/PKG-04_Solver Core and Numerical Methods/1_Working/DEL-04-03_Linear support and restraint models/_STATUS.md`
- `execution/PKG-04_Solver Core and Numerical Methods/1_Working/DEL-04-04_Nonlinear support active-set solver/_STATUS.md`
- `execution/PKG-04_Solver Core and Numerical Methods/1_Working/DEL-04-05_Sparse solver performance harness/_STATUS.md`
- `execution/PKG-04_Solver Core and Numerical Methods/1_Working/DEL-04-06_Solver diagnostics and singularity detection/_STATUS.md`
- `execution/PKG-05_Loads, Load Cases, and Stress Recovery/1_Working/DEL-05-01_Primitive load case engine/_STATUS.md`
- `execution/PKG-05_Loads, Load Cases, and Stress Recovery/1_Working/DEL-05-02_Load-case algebra engine/_STATUS.md`
- `execution/PKG-05_Loads, Load Cases, and Stress Recovery/1_Working/DEL-05-03_Fundamental stress recovery module/_STATUS.md`
- `execution/PKG-05_Loads, Load Cases, and Stress Recovery/1_Working/DEL-05-04_Analysis status semantics/_STATUS.md`
- `execution/PKG-05_Loads, Load Cases, and Stress Recovery/1_Working/DEL-05-05_Concentrated and distributed user load application/_STATUS.md`
- `execution/PKG-06_Rule Packs and User-Supplied Code Check Engine/1_Working/DEL-06-01_Rule-pack schema/_STATUS.md`
- `execution/PKG-06_Rule Packs and User-Supplied Code Check Engine/1_Working/DEL-06-02_Sandboxed unit-aware expression evaluator/_STATUS.md`
- `execution/PKG-06_Rule Packs and User-Supplied Code Check Engine/1_Working/DEL-06-03_Required-input completeness checker/_STATUS.md`
- `execution/PKG-06_Rule Packs and User-Supplied Code Check Engine/1_Working/DEL-06-04_Private rule-pack lifecycle and checksum handling/_STATUS.md`
- `execution/PKG-06_Rule Packs and User-Supplied Code Check Engine/1_Working/DEL-06-05_Invented non-code example rule pack/_STATUS.md`
- `execution/PKG-07_Graphical User Interface and Engineering Workflow/1_Working/DEL-07-01_3D viewport and centerline editor/_STATUS.md`
- `execution/PKG-07_Graphical User Interface and Engineering Workflow/1_Working/DEL-07-02_Model tree and property inspector/_STATUS.md`
- `execution/PKG-07_Graphical User Interface and Engineering Workflow/1_Working/DEL-07-03_Material, component, and rule-pack editors/_STATUS.md`
- `execution/PKG-07_Graphical User Interface and Engineering Workflow/1_Working/DEL-07-04_Missing-data warning and blocking UX/_STATUS.md`
- `execution/PKG-07_Graphical User Interface and Engineering Workflow/1_Working/DEL-07-05_Results viewer/_STATUS.md`
- `execution/PKG-07_Graphical User Interface and Engineering Workflow/1_Working/DEL-07-06_Accessibility and usability baseline/_STATUS.md`
- `execution/PKG-07_Graphical User Interface and Engineering Workflow/1_Working/DEL-07-07_Solve execution UX- progress, cancellation, and diagnostics/_STATUS.md`
- `execution/PKG-07_Graphical User Interface and Engineering Workflow/1_Working/DEL-07-08_Design-authoring state and comparison workspace/_STATUS.md`
- `execution/PKG-08_Reporting, Audit, and Reproducibility/1_Working/DEL-08-01_Calculation report generator/_STATUS.md`
- `execution/PKG-08_Reporting, Audit, and Reproducibility/1_Working/DEL-08-02_Audit manifest and model hash/_STATUS.md`
- `execution/PKG-08_Reporting, Audit, and Reproducibility/1_Working/DEL-08-03_Warnings, assumptions, and provenance report section/_STATUS.md`
- `execution/PKG-08_Reporting, Audit, and Reproducibility/1_Working/DEL-08-04_Result export format/_STATUS.md`
- `execution/PKG-08_Reporting, Audit, and Reproducibility/1_Working/DEL-08-05_Report protected-content linter/_STATUS.md`
- `execution/PKG-08_Reporting, Audit, and Reproducibility/1_Working/DEL-08-06_State, comparison, and handoff report sections/_STATUS.md`
- `execution/PKG-09_Verification, Validation, and Quality Oracles/1_Working/DEL-09-01_Mechanics benchmark suite/_STATUS.md`
- `execution/PKG-09_Verification, Validation, and Quality Oracles/1_Working/DEL-09-02_Stress recovery benchmark suite/_STATUS.md`
- `execution/PKG-09_Verification, Validation, and Quality Oracles/1_Working/DEL-09-03_Nonlinear support regression suite/_STATUS.md`
- `execution/PKG-09_Verification, Validation, and Quality Oracles/1_Working/DEL-09-04_Validation manual skeleton/_STATUS.md`
- `execution/PKG-09_Verification, Validation, and Quality Oracles/1_Working/DEL-09-05_Release quality gate checklist/_STATUS.md`
- `execution/PKG-10_Build, Packaging, API, and Interoperability/1_Working/DEL-10-01_Public API and plugin boundary/_STATUS.md`
- `execution/PKG-10_Build, Packaging, API, and Interoperability/1_Working/DEL-10-02_Import-export adapter framework/_STATUS.md`
- `execution/PKG-10_Build, Packaging, API, and Interoperability/1_Working/DEL-10-03_Local FEA handoff data contract/_STATUS.md`
- `execution/PKG-10_Build, Packaging, API, and Interoperability/1_Working/DEL-10-04_Build, packaging, and CI-CD pipeline/_STATUS.md`
- `execution/PKG-10_Build, Packaging, API, and Interoperability/1_Working/DEL-10-05_Headless CLI and structured I-O analysis runner/_STATUS.md`
- `execution/PKG-11_Documentation, Examples, and Education/1_Working/DEL-11-01_User guide skeleton/_STATUS.md`
- `execution/PKG-11_Documentation, Examples, and Education/1_Working/DEL-11-02_Developer guide for solver and rule packs/_STATUS.md`
- `execution/PKG-11_Documentation, Examples, and Education/1_Working/DEL-11-03_Theory notes- classical to modern centerline analysis/_STATUS.md`
- `execution/PKG-11_Documentation, Examples, and Education/1_Working/DEL-11-04_Invented educational example models/_STATUS.md`
- `execution/PKG-11_Documentation, Examples, and Education/1_Working/DEL-11-05_Contributor tutorial and onboarding/_STATUS.md`
- `execution/PKG-12_Security, Privacy, and Private Data Handling/1_Working/DEL-12-01_Local-first storage and private data paths/_STATUS.md`
- `execution/PKG-12_Security, Privacy, and Private Data Handling/1_Working/DEL-12-02_Private data redaction and export controls/_STATUS.md`
- `execution/PKG-12_Security, Privacy, and Private Data Handling/1_Working/DEL-12-03_Telemetry off-by-default design/_STATUS.md`
- `execution/PKG-12_Security, Privacy, and Private Data Handling/1_Working/DEL-12-04_Secret and private-library handling/_STATUS.md`
- `execution/PKG-12_Security, Privacy, and Private Data Handling/1_Working/DEL-12-05_Security threat model/_STATUS.md`
- `execution/PKG-13_Physical Design Knowledge and Constraint Engine/1_Working/DEL-13-01_Design knowledge schema and provenance model/_STATUS.md`
- `execution/PKG-13_Physical Design Knowledge and Constraint Engine/1_Working/DEL-13-02_Constraint entity and provenance model/_STATUS.md`
- `execution/PKG-13_Physical Design Knowledge and Constraint Engine/1_Working/DEL-13-03_Constraint validation engine/_STATUS.md`
- `execution/PKG-13_Physical Design Knowledge and Constraint Engine/1_Working/DEL-13-04_Physical-to-analytical transformation contract/_STATUS.md`
- `execution/PKG-14_Model States, Analysis Runs, and Comparison/1_Working/DEL-14-01_Immutable model state records/_STATUS.md`
- `execution/PKG-14_Model States, Analysis Runs, and Comparison/1_Working/DEL-14-02_Analysis run records/_STATUS.md`
- `execution/PKG-14_Model States, Analysis Runs, and Comparison/1_Working/DEL-14-03_Model-state comparison engine/_STATUS.md`
- `execution/PKG-14_Model States, Analysis Runs, and Comparison/1_Working/DEL-14-04_Analysis-run comparison engine/_STATUS.md`
- `execution/PKG-14_Model States, Analysis Runs, and Comparison/1_Working/DEL-14-05_Comparison mapping, tolerance, and export contracts/_STATUS.md`
- `execution/PKG-15_Handoff and External Prover Workflow/1_Working/DEL-15-01_Canonical handoff package schema and manifest/_STATUS.md`
- `execution/PKG-15_Handoff and External Prover Workflow/1_Working/DEL-15-02_Target mapping and unsupported-behavior contract/_STATUS.md`
- `execution/PKG-15_Handoff and External Prover Workflow/1_Working/DEL-15-03_Downstream modeling export workflow/_STATUS.md`
- `execution/PKG-15_Handoff and External Prover Workflow/1_Working/DEL-15-04_External prover boundary metadata/_STATUS.md`
- `execution/PKG-16_Model Operation and Agent Proposal Framework/1_Working/DEL-16-01_Structured model operation schema/_STATUS.md`
- `execution/PKG-16_Model Operation and Agent Proposal Framework/1_Working/DEL-16-02_Operation validation and diff preview/_STATUS.md`
- `execution/PKG-16_Model Operation and Agent Proposal Framework/1_Working/DEL-16-03_User acceptance and operation audit trail/_STATUS.md`
- `execution/PKG-16_Model Operation and Agent Proposal Framework/1_Working/DEL-16-04_Agent rationale and professional-boundary controls/_STATUS.md`
- `execution/PKG-17_Export Format Interoperability/1_Working/DEL-17-01_CAEPIPE and export-format source basis/_STATUS.md`
- `execution/PKG-17_Export Format Interoperability/1_Working/DEL-17-02_Export package, profile, and stable ID map contracts/_STATUS.md`
- `execution/PKG-17_Export Format Interoperability/1_Working/DEL-17-03_Native open JSON export package/_STATUS.md`
- `execution/PKG-17_Export Format Interoperability/1_Working/DEL-17-04_CAEPIPE MBF export profile and deterministic writer/_STATUS.md`
- `execution/PKG-17_Export Format Interoperability/1_Working/DEL-17-05_CAEPIPE external run harness and CSV parser/_STATUS.md`
- `execution/PKG-17_Export Format Interoperability/1_Working/DEL-17-06_Stress-neutral CSV JSON package/_STATUS.md`
- `execution/PKG-17_Export Format Interoperability/1_Working/DEL-17-07_Conservative PCF subset exporter/_STATUS.md`
- `execution/PKG-17_Export Format Interoperability/1_Working/DEL-17-08_GLB glTF review geometry export/_STATUS.md`
- `execution/PKG-17_Export Format Interoperability/1_Working/DEL-17-09_Export adapter SDK and additional targets/_STATUS.md`
