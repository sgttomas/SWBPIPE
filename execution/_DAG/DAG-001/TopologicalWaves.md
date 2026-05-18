---
doc_id: DAG-001-TOPOLOGICAL-WAVES
doc_kind: coordination.topological_waves
status: approved_dependency_order
created: 2026-04-30
---

# DAG-001 Topological Waves

Approval record: `execution/_DAG/DAG-001/APPROVAL_RECORD.md`.

Topological waves are dependency order only. They are not schedule, priority, staffing, readiness, or blocked/unblocked queue status. Candidate edges do not gate wave placement.

Active-edge status: ACYCLIC. Produced 12 dependency waves from 615 active edges.

## Wave 1

- `DEL-00-01` (PKG-00) - Architecture decision record baseline
- `DEL-00-02` (PKG-00) - Repository and module boundary architecture
- `DEL-00-03` (PKG-00) - Application service command-query-job model
- `DEL-00-04` (PKG-00) - Persistence and schema versioning architecture
- `DEL-00-05` (PKG-00) - GUI state and interaction architecture
- `DEL-00-06` (PKG-00) - Diagnostics, warning, and result-envelope contract
- `DEL-00-07` (PKG-00) - API boundary and adapter contract map
- `DEL-00-08` (PKG-00) - Layered software test and acceptance strategy

## Wave 2

- `DEL-01-01` (PKG-01) - Project governance baseline
- `DEL-02-01` (PKG-02) - Canonical domain model schema

## Wave 3

- `DEL-01-02` (PKG-01) - Copyright and protected-data boundary policy
- `DEL-01-04` (PKG-01) - Professional responsibility and product-claims policy
- `DEL-02-02` (PKG-02) - Unit system and dimensional-analysis core contract
- `DEL-02-03` (PKG-02) - Code-neutral analysis boundary model

## Wave 4

- `DEL-01-03` (PKG-01) - Contributor certification workflow
- `DEL-02-04` (PKG-02) - Plugin and extension domain contracts
- `DEL-02-05` (PKG-02) - Project persistence and round-trip serialization
- `DEL-04-01` (PKG-04) - 3D frame stiffness kernel
- `DEL-05-04` (PKG-05) - Analysis status semantics
- `DEL-06-01` (PKG-06) - Rule-pack schema

## Wave 5

- `DEL-03-01` (PKG-03) - Material library schema with provenance
- `DEL-03-02` (PKG-03) - Pipe section and component library schema
- `DEL-04-03` (PKG-04) - Linear support and restraint models
- `DEL-04-06` (PKG-04) - Solver diagnostics and singularity detection
- `DEL-06-02` (PKG-06) - Sandboxed unit-aware expression evaluator
- `DEL-06-03` (PKG-06) - Required-input completeness checker
- `DEL-06-04` (PKG-06) - Private rule-pack lifecycle and checksum handling
- `DEL-10-01` (PKG-10) - Public API and plugin boundary
- `DEL-11-05` (PKG-11) - Contributor tutorial and onboarding
- `DEL-12-05` (PKG-12) - Security threat model

## Wave 6

- `DEL-03-03` (PKG-03) - Bend and elbow component model fields
- `DEL-03-04` (PKG-03) - Branch connection component model fields
- `DEL-03-05` (PKG-03) - Rigid component models for valves, flanges, reducers, and specialty items
- `DEL-03-06` (PKG-03) - Expansion joint component model
- `DEL-03-07` (PKG-03) - Public/private library import provenance checker
- `DEL-03-08` (PKG-03) - Pipe section property and mass-property calculator
- `DEL-04-04` (PKG-04) - Nonlinear support active-set solver
- `DEL-04-05` (PKG-04) - Sparse solver performance harness
- `DEL-05-01` (PKG-05) - Primitive load case engine
- `DEL-06-05` (PKG-06) - Invented non-code example rule pack
- `DEL-07-02` (PKG-07) - Model tree and property inspector
- `DEL-07-04` (PKG-07) - Missing-data warning and blocking UX
- `DEL-08-02` (PKG-08) - Audit manifest and model hash
- `DEL-11-02` (PKG-11) - Developer guide for solver and rule packs
- `DEL-12-01` (PKG-12) - Local-first storage and private data paths
- `DEL-12-03` (PKG-12) - Telemetry off-by-default design

## Wave 7

- `DEL-04-02` (PKG-04) - Straight pipe element
- `DEL-05-02` (PKG-05) - Load-case algebra engine
- `DEL-05-05` (PKG-05) - Concentrated and distributed user load application
- `DEL-07-01` (PKG-07) - 3D viewport and centerline editor
- `DEL-07-03` (PKG-07) - Material, component, and rule-pack editors
- `DEL-07-07` (PKG-07) - Solve execution UX: progress, cancellation, and diagnostics
- `DEL-08-03` (PKG-08) - Warnings, assumptions, and provenance report section
- `DEL-09-03` (PKG-09) - Nonlinear support regression suite
- `DEL-10-02` (PKG-10) - Import/export adapter framework
- `DEL-12-04` (PKG-12) - Secret and private-library handling

## Wave 8

- `DEL-05-03` (PKG-05) - Fundamental stress recovery module
- `DEL-09-01` (PKG-09) - Mechanics benchmark suite

## Wave 9

- `DEL-07-05` (PKG-07) - Results viewer
- `DEL-08-01` (PKG-08) - Calculation report generator
- `DEL-08-04` (PKG-08) - Result export format
- `DEL-09-02` (PKG-09) - Stress recovery benchmark suite
- `DEL-10-03` (PKG-10) - Local FEA handoff data contract
- `DEL-11-03` (PKG-11) - Theory notes: classical to modern centerline analysis

## Wave 10

- `DEL-07-06` (PKG-07) - Accessibility and usability baseline
- `DEL-08-05` (PKG-08) - Report protected-content linter
- `DEL-09-04` (PKG-09) - Validation manual skeleton
- `DEL-10-05` (PKG-10) - Headless CLI and structured I/O analysis runner
- `DEL-11-01` (PKG-11) - User guide skeleton
- `DEL-12-02` (PKG-12) - Private data redaction and export controls

## Wave 11

- `DEL-09-05` (PKG-09) - Release quality gate checklist
- `DEL-11-04` (PKG-11) - Invented educational example models

## Wave 12

- `DEL-10-04` (PKG-10) - Build, packaging, and CI/CD pipeline

## Candidate Edge Caveats

- `DAG-001-E0616`: `DEL-05-02` -> `DEL-06-02` (LOAD_STRESS_PREDECESSOR, LOW) - load-case algebra may reuse the same sandboxed expression machinery as rule packs; current decomposition leaves expression grammar/library TBD
- `DAG-001-E0617`: `DEL-07-05` -> `DEL-08-04` (GUI_PREDECESSOR, MEDIUM) - results viewer may share the structured result export schema, but GUI rendering could also consume internal result models directly
- `DAG-001-E0618`: `DEL-10-03` -> `DEL-08-04` (INTEROP_PREDECESSOR, MEDIUM) - local FEA handoff may reuse result export envelopes, but the handoff contract may define a separate package
- `DAG-001-E0619`: `DEL-12-05` -> `DEL-10-02` (SECURITY_PREDECESSOR, LOW) - threat model may require adapter framework details; active graph currently places threat model before adapter framework to avoid bypass risk
- `DAG-001-E0620`: `DEL-09-05` -> `DEL-10-04` (VALIDATION_PREDECESSOR, LOW) - CI implementation may refine release quality gate details; active graph currently treats gates as predecessor to CI/CD implementation
- `DAG-001-E0621`: `DEL-08-05` -> `DEL-11-04` (GOVERNANCE_PREDECESSOR, LOW) - protected-content linter may need educational example fixtures; active graph currently treats examples as consumers of the linter
- `DAG-001-E0622`: `DEL-04-06` -> `DEL-04-04` (DIAGNOSTICS_CONTRACT, LOW) - diagnostics may need nonlinear support cases to finalize all warning classes; active graph currently makes nonlinear solver consume diagnostics
- `DAG-001-E0623`: `DEL-06-02` -> `DEL-12-05` (SECURITY_PREDECESSOR, MEDIUM) - sandboxed evaluator may require threat-model review before implementation freeze
- `DAG-001-E0624`: `DEL-07-07` -> `DEL-10-05` (GUI_PREDECESSOR, LOW) - solve execution UX and headless runner may share job orchestration implementation; current evidence only proves shared architecture basis
