---
doc_id: DAG-003-MERMAID
doc_kind: coordination.graph_visualization
status: approved_active_edge_set_guarded_followups
created: 2026-05-11
approval_status: approved_2026-05-11
---


# DAG-003 Mermaid

```mermaid
graph TD
  DEL_00_01["DEL-00-01 Architecture decision record baseline"]
  DEL_00_02["DEL-00-02 Repository and module boundary architecture"]
  DEL_00_03["DEL-00-03 Application service command-query-job model"]
  DEL_00_04["DEL-00-04 Persistence and schema versioning architecture"]
  DEL_00_05["DEL-00-05 GUI state and interaction architecture"]
  DEL_00_06["DEL-00-06 Diagnostics, warning, and result-envelope contract"]
  DEL_00_07["DEL-00-07 API boundary and adapter contract map"]
  DEL_00_08["DEL-00-08 Layered software test and acceptance strategy"]
  DEL_01_01["DEL-01-01 Project governance baseline"]
  DEL_01_02["DEL-01-02 Copyright and protected-data boundary policy"]
  DEL_01_03["DEL-01-03 Contributor certification workflow"]
  DEL_01_04["DEL-01-04 Professional responsibility and product-claims policy"]
  DEL_02_01["DEL-02-01 Canonical domain model schema"]
  DEL_02_02["DEL-02-02 Unit system and dimensional-analysis core contract"]
  DEL_02_03["DEL-02-03 Code-neutral analysis boundary model"]
  DEL_02_04["DEL-02-04 Plugin and extension domain contracts"]
  DEL_02_05["DEL-02-05 Project persistence and round-trip serialization"]
  DEL_03_01["DEL-03-01 Material library schema with provenance"]
  DEL_03_02["DEL-03-02 Pipe section and component library schema"]
  DEL_03_03["DEL-03-03 Bend and elbow component model fields"]
  DEL_03_04["DEL-03-04 Branch connection component model fields"]
  DEL_03_05["DEL-03-05 Rigid component models for valves, flanges, reducers, and specialty items"]
  DEL_03_06["DEL-03-06 Expansion joint component model"]
  DEL_03_07["DEL-03-07 Public/private library import provenance checker"]
  DEL_03_08["DEL-03-08 Pipe section property and mass-property calculator"]
  DEL_04_01["DEL-04-01 3D frame stiffness kernel"]
  DEL_04_02["DEL-04-02 Straight pipe element"]
  DEL_04_03["DEL-04-03 Linear support and restraint models"]
  DEL_04_04["DEL-04-04 Nonlinear support active-set solver"]
  DEL_04_05["DEL-04-05 Sparse solver performance harness"]
  DEL_04_06["DEL-04-06 Solver diagnostics and singularity detection"]
  DEL_05_01["DEL-05-01 Primitive load case engine"]
  DEL_05_02["DEL-05-02 Load-case algebra engine"]
  DEL_05_03["DEL-05-03 Fundamental stress recovery module"]
  DEL_05_04["DEL-05-04 Analysis status semantics"]
  DEL_05_05["DEL-05-05 Concentrated and distributed user load application"]
  DEL_06_01["DEL-06-01 Rule-pack schema"]
  DEL_06_02["DEL-06-02 Sandboxed unit-aware expression evaluator"]
  DEL_06_03["DEL-06-03 Required-input completeness checker"]
  DEL_06_04["DEL-06-04 Private rule-pack lifecycle and checksum handling"]
  DEL_06_05["DEL-06-05 Invented non-code example rule pack"]
  DEL_07_01["DEL-07-01 3D viewport and centerline editor"]
  DEL_07_02["DEL-07-02 Model tree and property inspector"]
  DEL_07_03["DEL-07-03 Material, component, and rule-pack editors"]
  DEL_07_04["DEL-07-04 Missing-data warning and blocking UX"]
  DEL_07_05["DEL-07-05 Results viewer"]
  DEL_07_06["DEL-07-06 Accessibility and usability baseline"]
  DEL_07_07["DEL-07-07 Solve execution UX: progress, cancellation, and diagnostics"]
  DEL_08_01["DEL-08-01 Calculation report generator"]
  DEL_08_02["DEL-08-02 Audit manifest and model hash"]
  DEL_08_03["DEL-08-03 Warnings, assumptions, and provenance report section"]
  DEL_08_04["DEL-08-04 Result export format"]
  DEL_08_05["DEL-08-05 Report protected-content linter"]
  DEL_09_01["DEL-09-01 Mechanics benchmark suite"]
  DEL_09_02["DEL-09-02 Stress recovery benchmark suite"]
  DEL_09_03["DEL-09-03 Nonlinear support regression suite"]
  DEL_09_04["DEL-09-04 Validation manual skeleton"]
  DEL_09_05["DEL-09-05 Release quality gate checklist"]
  DEL_10_01["DEL-10-01 Public API and plugin boundary"]
  DEL_10_02["DEL-10-02 Import/export adapter framework"]
  DEL_10_03["DEL-10-03 Local FEA handoff data contract"]
  DEL_10_04["DEL-10-04 Build, packaging, and CI/CD pipeline"]
  DEL_10_05["DEL-10-05 Headless CLI and structured I/O analysis runner"]
  DEL_11_01["DEL-11-01 User guide skeleton"]
  DEL_11_02["DEL-11-02 Developer guide for solver and rule packs"]
  DEL_11_03["DEL-11-03 Theory notes: classical to modern centerline analysis"]
  DEL_11_04["DEL-11-04 Invented educational example models"]
  DEL_11_05["DEL-11-05 Contributor tutorial and onboarding"]
  DEL_12_01["DEL-12-01 Local-first storage and private data paths"]
  DEL_12_02["DEL-12-02 Private data redaction and export controls"]
  DEL_12_03["DEL-12-03 Telemetry off-by-default design"]
  DEL_12_04["DEL-12-04 Secret and private-library handling"]
  DEL_12_05["DEL-12-05 Security threat model"]
  DEL_07_08["DEL-07-08 Design-authoring state and comparison workspace"]
  DEL_08_06["DEL-08-06 State, comparison, and handoff report sections"]
  DEL_13_01["DEL-13-01 Design knowledge schema and provenance model"]
  DEL_13_02["DEL-13-02 Constraint entity and provenance model"]
  DEL_13_03["DEL-13-03 Constraint validation engine"]
  DEL_13_04["DEL-13-04 Physical-to-analytical transformation contract"]
  DEL_14_01["DEL-14-01 Immutable model state records"]
  DEL_14_02["DEL-14-02 Analysis run records"]
  DEL_14_03["DEL-14-03 Model-state comparison engine"]
  DEL_14_04["DEL-14-04 Analysis-run comparison engine"]
  DEL_14_05["DEL-14-05 Comparison mapping, tolerance, and export contracts"]
  DEL_15_01["DEL-15-01 Canonical handoff package schema and manifest"]
  DEL_15_02["DEL-15-02 Target mapping and unsupported-behavior contract"]
  DEL_15_03["DEL-15-03 Downstream modeling export workflow"]
  DEL_15_04["DEL-15-04 External prover boundary metadata"]
  DEL_16_01["DEL-16-01 Structured model operation schema"]
  DEL_16_02["DEL-16-02 Operation validation and diff preview"]
  DEL_16_03["DEL-16-03 User acceptance and operation audit trail"]
  DEL_16_04["DEL-16-04 Agent rationale and professional-boundary controls"]
  DEL_00_01 --> DEL_01_01
  DEL_00_02 --> DEL_01_01
  DEL_00_06 --> DEL_01_01
  DEL_00_08 --> DEL_01_01
  DEL_00_01 --> DEL_01_02
  DEL_00_02 --> DEL_01_02
  DEL_00_06 --> DEL_01_02
  DEL_00_08 --> DEL_01_02
  DEL_01_01 --> DEL_01_02
  DEL_00_01 --> DEL_01_03
  DEL_00_02 --> DEL_01_03
  DEL_00_06 --> DEL_01_03
  DEL_00_08 --> DEL_01_03
  DEL_01_01 --> DEL_01_03
  DEL_01_02 --> DEL_01_03
  DEL_00_01 --> DEL_01_04
  DEL_00_02 --> DEL_01_04
  DEL_00_06 --> DEL_01_04
  DEL_00_08 --> DEL_01_04
  DEL_00_01 --> DEL_02_01
  DEL_00_02 --> DEL_02_01
  DEL_00_03 --> DEL_02_01
  DEL_00_04 --> DEL_02_01
  DEL_00_06 --> DEL_02_01
  DEL_00_07 --> DEL_02_01
  DEL_00_08 --> DEL_02_01
  DEL_00_01 --> DEL_02_02
  DEL_00_02 --> DEL_02_02
  DEL_00_03 --> DEL_02_02
  DEL_00_04 --> DEL_02_02
  DEL_00_06 --> DEL_02_02
  DEL_00_07 --> DEL_02_02
  DEL_00_08 --> DEL_02_02
  DEL_02_01 --> DEL_02_02
  DEL_00_01 --> DEL_02_03
  DEL_00_02 --> DEL_02_03
  DEL_00_03 --> DEL_02_03
  DEL_00_04 --> DEL_02_03
  DEL_00_06 --> DEL_02_03
  DEL_00_07 --> DEL_02_03
  DEL_00_08 --> DEL_02_03
  DEL_02_01 --> DEL_02_03
  DEL_00_01 --> DEL_02_04
  DEL_00_02 --> DEL_02_04
  DEL_00_03 --> DEL_02_04
  DEL_00_04 --> DEL_02_04
  DEL_00_06 --> DEL_02_04
  DEL_00_07 --> DEL_02_04
  DEL_00_08 --> DEL_02_04
  DEL_00_01 --> DEL_02_05
  DEL_00_02 --> DEL_02_05
  DEL_00_03 --> DEL_02_05
  DEL_00_04 --> DEL_02_05
  DEL_00_06 --> DEL_02_05
  DEL_00_07 --> DEL_02_05
  DEL_00_08 --> DEL_02_05
  DEL_02_01 --> DEL_02_05
  DEL_02_02 --> DEL_02_05
  DEL_02_03 --> DEL_02_05
  DEL_00_01 --> DEL_03_01
  DEL_00_02 --> DEL_03_01
  DEL_00_04 --> DEL_03_01
  DEL_00_06 --> DEL_03_01
  DEL_00_07 --> DEL_03_01
  DEL_00_08 --> DEL_03_01
  DEL_01_02 --> DEL_03_01
  DEL_01_03 --> DEL_03_01
  DEL_02_01 --> DEL_03_01
  DEL_02_02 --> DEL_03_01
  DEL_00_01 --> DEL_03_02
  DEL_00_02 --> DEL_03_02
  DEL_00_04 --> DEL_03_02
  DEL_00_06 --> DEL_03_02
  DEL_00_07 --> DEL_03_02
  DEL_00_08 --> DEL_03_02
  DEL_01_02 --> DEL_03_02
  DEL_01_03 --> DEL_03_02
  DEL_02_01 --> DEL_03_02
  DEL_02_02 --> DEL_03_02
  DEL_00_01 --> DEL_03_03
  DEL_00_02 --> DEL_03_03
  DEL_00_04 --> DEL_03_03
  DEL_00_06 --> DEL_03_03
  DEL_00_07 --> DEL_03_03
  DEL_00_08 --> DEL_03_03
  DEL_01_02 --> DEL_03_03
  DEL_02_02 --> DEL_03_03
  DEL_03_02 --> DEL_03_03
  DEL_00_01 --> DEL_03_04
  DEL_00_02 --> DEL_03_04
  DEL_00_04 --> DEL_03_04
  DEL_00_06 --> DEL_03_04
  DEL_00_07 --> DEL_03_04
  DEL_00_08 --> DEL_03_04
  DEL_01_02 --> DEL_03_04
  DEL_02_02 --> DEL_03_04
  DEL_03_02 --> DEL_03_04
  DEL_00_01 --> DEL_03_05
  DEL_00_02 --> DEL_03_05
  DEL_00_04 --> DEL_03_05
  DEL_00_06 --> DEL_03_05
  DEL_00_07 --> DEL_03_05
  DEL_00_08 --> DEL_03_05
  DEL_01_02 --> DEL_03_05
  DEL_02_02 --> DEL_03_05
  DEL_03_02 --> DEL_03_05
  DEL_00_01 --> DEL_03_06
  DEL_00_02 --> DEL_03_06
  DEL_00_04 --> DEL_03_06
  DEL_00_06 --> DEL_03_06
  DEL_00_07 --> DEL_03_06
  DEL_00_08 --> DEL_03_06
  DEL_01_02 --> DEL_03_06
  DEL_02_02 --> DEL_03_06
  DEL_03_02 --> DEL_03_06
  DEL_00_01 --> DEL_03_07
  DEL_00_02 --> DEL_03_07
  DEL_00_04 --> DEL_03_07
  DEL_00_06 --> DEL_03_07
  DEL_00_07 --> DEL_03_07
  DEL_00_08 --> DEL_03_07
  DEL_01_02 --> DEL_03_07
  DEL_01_03 --> DEL_03_07
  DEL_02_04 --> DEL_03_07
  DEL_03_01 --> DEL_03_07
  DEL_03_02 --> DEL_03_07
  DEL_00_01 --> DEL_03_08
  DEL_00_02 --> DEL_03_08
  DEL_00_04 --> DEL_03_08
  DEL_00_06 --> DEL_03_08
  DEL_00_07 --> DEL_03_08
  DEL_00_08 --> DEL_03_08
  DEL_02_02 --> DEL_03_08
  DEL_03_01 --> DEL_03_08
  DEL_03_02 --> DEL_03_08
  DEL_00_01 --> DEL_04_01
  DEL_00_02 --> DEL_04_01
  DEL_00_03 --> DEL_04_01
  DEL_00_06 --> DEL_04_01
  DEL_00_08 --> DEL_04_01
  DEL_02_01 --> DEL_04_01
  DEL_02_02 --> DEL_04_01
  DEL_02_03 --> DEL_04_01
  DEL_00_01 --> DEL_04_02
  DEL_00_02 --> DEL_04_02
  DEL_00_03 --> DEL_04_02
  DEL_00_06 --> DEL_04_02
  DEL_00_08 --> DEL_04_02
  DEL_02_02 --> DEL_04_02
  DEL_03_08 --> DEL_04_02
  DEL_04_01 --> DEL_04_02
  DEL_00_01 --> DEL_04_03
  DEL_00_02 --> DEL_04_03
  DEL_00_03 --> DEL_04_03
  DEL_00_06 --> DEL_04_03
  DEL_00_08 --> DEL_04_03
  DEL_02_01 --> DEL_04_03
  DEL_02_02 --> DEL_04_03
  DEL_04_01 --> DEL_04_03
  DEL_00_01 --> DEL_04_04
  DEL_00_02 --> DEL_04_04
  DEL_00_03 --> DEL_04_04
  DEL_00_06 --> DEL_04_04
  DEL_00_08 --> DEL_04_04
  DEL_02_02 --> DEL_04_04
  DEL_04_01 --> DEL_04_04
  DEL_04_03 --> DEL_04_04
  DEL_04_06 --> DEL_04_04
  DEL_00_01 --> DEL_04_05
  DEL_00_02 --> DEL_04_05
  DEL_00_03 --> DEL_04_05
  DEL_00_06 --> DEL_04_05
  DEL_00_08 --> DEL_04_05
  DEL_04_01 --> DEL_04_05
  DEL_04_06 --> DEL_04_05
  DEL_00_01 --> DEL_04_06
  DEL_00_02 --> DEL_04_06
  DEL_00_03 --> DEL_04_06
  DEL_00_06 --> DEL_04_06
  DEL_00_08 --> DEL_04_06
  DEL_02_02 --> DEL_04_06
  DEL_02_03 --> DEL_04_06
  DEL_04_01 --> DEL_04_06
  DEL_00_01 --> DEL_05_01
  DEL_00_02 --> DEL_05_01
  DEL_00_03 --> DEL_05_01
  DEL_00_06 --> DEL_05_01
  DEL_00_08 --> DEL_05_01
  DEL_00_01 --> DEL_05_02
  DEL_00_02 --> DEL_05_02
  DEL_00_03 --> DEL_05_02
  DEL_00_06 --> DEL_05_02
  DEL_00_08 --> DEL_05_02
  DEL_02_02 --> DEL_05_02
  DEL_05_01 --> DEL_05_02
  DEL_05_04 --> DEL_05_02
  DEL_06_02 --> DEL_05_02
  DEL_00_01 --> DEL_05_03
  DEL_00_02 --> DEL_05_03
  DEL_00_03 --> DEL_05_03
  DEL_00_06 --> DEL_05_03
  DEL_00_08 --> DEL_05_03
  DEL_02_02 --> DEL_05_03
  DEL_03_08 --> DEL_05_03
  DEL_04_02 --> DEL_05_03
  DEL_05_01 --> DEL_05_03
  DEL_05_04 --> DEL_05_03
  DEL_05_05 --> DEL_05_03
  DEL_00_01 --> DEL_05_04
  DEL_00_02 --> DEL_05_04
  DEL_00_03 --> DEL_05_04
  DEL_00_06 --> DEL_05_04
  DEL_00_08 --> DEL_05_04
  DEL_02_03 --> DEL_05_04
  DEL_00_01 --> DEL_05_05
  DEL_00_02 --> DEL_05_05
  DEL_00_03 --> DEL_05_05
  DEL_00_06 --> DEL_05_05
  DEL_00_08 --> DEL_05_05
  DEL_02_02 --> DEL_05_05
  DEL_04_01 --> DEL_05_05
  DEL_05_01 --> DEL_05_05
  DEL_00_01 --> DEL_06_01
  DEL_00_02 --> DEL_06_01
  DEL_00_03 --> DEL_06_01
  DEL_00_04 --> DEL_06_01
  DEL_00_06 --> DEL_06_01
  DEL_00_07 --> DEL_06_01
  DEL_00_08 --> DEL_06_01
  DEL_01_02 --> DEL_06_01
  DEL_01_04 --> DEL_06_01
  DEL_02_01 --> DEL_06_01
  DEL_02_02 --> DEL_06_01
  DEL_02_03 --> DEL_06_01
  DEL_00_01 --> DEL_06_02
  DEL_00_02 --> DEL_06_02
  DEL_00_03 --> DEL_06_02
  DEL_00_04 --> DEL_06_02
  DEL_00_06 --> DEL_06_02
  DEL_00_07 --> DEL_06_02
  DEL_00_08 --> DEL_06_02
  DEL_02_02 --> DEL_06_02
  DEL_06_01 --> DEL_06_02
  DEL_00_01 --> DEL_06_03
  DEL_00_02 --> DEL_06_03
  DEL_00_03 --> DEL_06_03
  DEL_00_04 --> DEL_06_03
  DEL_00_06 --> DEL_06_03
  DEL_00_07 --> DEL_06_03
  DEL_00_08 --> DEL_06_03
  DEL_02_03 --> DEL_06_03
  DEL_05_04 --> DEL_06_03
  DEL_06_01 --> DEL_06_03
  DEL_00_01 --> DEL_06_04
  DEL_00_02 --> DEL_06_04
  DEL_00_03 --> DEL_06_04
  DEL_00_04 --> DEL_06_04
  DEL_00_06 --> DEL_06_04
  DEL_00_07 --> DEL_06_04
  DEL_00_08 --> DEL_06_04
  DEL_02_05 --> DEL_06_04
  DEL_06_01 --> DEL_06_04
  DEL_00_01 --> DEL_06_05
  DEL_00_02 --> DEL_06_05
  DEL_00_03 --> DEL_06_05
  DEL_00_04 --> DEL_06_05
  DEL_00_06 --> DEL_06_05
  DEL_00_07 --> DEL_06_05
  DEL_00_08 --> DEL_06_05
  DEL_06_01 --> DEL_06_05
  DEL_06_02 --> DEL_06_05
  DEL_00_01 --> DEL_07_01
  DEL_00_02 --> DEL_07_01
  DEL_00_03 --> DEL_07_01
  DEL_00_05 --> DEL_07_01
  DEL_00_06 --> DEL_07_01
  DEL_00_07 --> DEL_07_01
  DEL_00_08 --> DEL_07_01
  DEL_02_01 --> DEL_07_01
  DEL_02_02 --> DEL_07_01
  DEL_02_05 --> DEL_07_01
  DEL_03_02 --> DEL_07_01
  DEL_03_03 --> DEL_07_01
  DEL_03_04 --> DEL_07_01
  DEL_03_05 --> DEL_07_01
  DEL_03_06 --> DEL_07_01
  DEL_00_01 --> DEL_07_02
  DEL_00_02 --> DEL_07_02
  DEL_00_03 --> DEL_07_02
  DEL_00_05 --> DEL_07_02
  DEL_00_06 --> DEL_07_02
  DEL_00_07 --> DEL_07_02
  DEL_00_08 --> DEL_07_02
  DEL_02_01 --> DEL_07_02
  DEL_02_02 --> DEL_07_02
  DEL_02_05 --> DEL_07_02
  DEL_03_01 --> DEL_07_02
  DEL_03_02 --> DEL_07_02
  DEL_06_01 --> DEL_07_02
  DEL_06_04 --> DEL_07_02
  DEL_00_01 --> DEL_07_03
  DEL_00_02 --> DEL_07_03
  DEL_00_03 --> DEL_07_03
  DEL_00_05 --> DEL_07_03
  DEL_00_06 --> DEL_07_03
  DEL_00_07 --> DEL_07_03
  DEL_00_08 --> DEL_07_03
  DEL_03_01 --> DEL_07_03
  DEL_03_02 --> DEL_07_03
  DEL_03_07 --> DEL_07_03
  DEL_06_01 --> DEL_07_03
  DEL_06_04 --> DEL_07_03
  DEL_07_02 --> DEL_07_03
  DEL_12_01 --> DEL_07_03
  DEL_00_01 --> DEL_07_04
  DEL_00_02 --> DEL_07_04
  DEL_00_03 --> DEL_07_04
  DEL_00_05 --> DEL_07_04
  DEL_00_06 --> DEL_07_04
  DEL_00_07 --> DEL_07_04
  DEL_00_08 --> DEL_07_04
  DEL_04_06 --> DEL_07_04
  DEL_05_04 --> DEL_07_04
  DEL_06_03 --> DEL_07_04
  DEL_07_02 --> DEL_07_04
  DEL_00_01 --> DEL_07_05
  DEL_00_02 --> DEL_07_05
  DEL_00_03 --> DEL_07_05
  DEL_00_05 --> DEL_07_05
  DEL_00_06 --> DEL_07_05
  DEL_00_07 --> DEL_07_05
  DEL_00_08 --> DEL_07_05
  DEL_04_06 --> DEL_07_05
  DEL_05_03 --> DEL_07_05
  DEL_05_04 --> DEL_07_05
  DEL_08_04 --> DEL_07_05
  DEL_00_01 --> DEL_07_06
  DEL_00_02 --> DEL_07_06
  DEL_00_03 --> DEL_07_06
  DEL_00_05 --> DEL_07_06
  DEL_00_06 --> DEL_07_06
  DEL_00_07 --> DEL_07_06
  DEL_00_08 --> DEL_07_06
  DEL_00_01 --> DEL_07_07
  DEL_00_02 --> DEL_07_07
  DEL_00_03 --> DEL_07_07
  DEL_00_05 --> DEL_07_07
  DEL_00_06 --> DEL_07_07
  DEL_00_07 --> DEL_07_07
  DEL_00_08 --> DEL_07_07
  DEL_04_01 --> DEL_07_07
  DEL_04_06 --> DEL_07_07
  DEL_05_01 --> DEL_07_07
  DEL_05_04 --> DEL_07_07
  DEL_10_05 --> DEL_07_07
  DEL_00_01 --> DEL_07_08
  DEL_00_02 --> DEL_07_08
  DEL_00_03 --> DEL_07_08
  DEL_00_05 --> DEL_07_08
  DEL_00_06 --> DEL_07_08
  DEL_00_07 --> DEL_07_08
  DEL_00_08 --> DEL_07_08
  DEL_07_01 --> DEL_07_08
  DEL_07_02 --> DEL_07_08
  DEL_07_04 --> DEL_07_08
  DEL_07_05 --> DEL_07_08
  DEL_13_01 --> DEL_07_08
  DEL_13_03 --> DEL_07_08
  DEL_13_04 --> DEL_07_08
  DEL_14_01 --> DEL_07_08
  DEL_14_03 --> DEL_07_08
  DEL_14_04 --> DEL_07_08
  DEL_14_05 --> DEL_07_08
  DEL_16_01 --> DEL_07_08
  DEL_16_02 --> DEL_07_08
  DEL_16_03 --> DEL_07_08
  DEL_00_01 --> DEL_08_01
  DEL_00_02 --> DEL_08_01
  DEL_00_03 --> DEL_08_01
  DEL_00_04 --> DEL_08_01
  DEL_00_06 --> DEL_08_01
  DEL_00_07 --> DEL_08_01
  DEL_00_08 --> DEL_08_01
  DEL_01_04 --> DEL_08_01
  DEL_02_05 --> DEL_08_01
  DEL_05_03 --> DEL_08_01
  DEL_05_04 --> DEL_08_01
  DEL_06_04 --> DEL_08_01
  DEL_08_02 --> DEL_08_01
  DEL_08_03 --> DEL_08_01
  DEL_08_04 --> DEL_08_01
  DEL_00_01 --> DEL_08_02
  DEL_00_02 --> DEL_08_02
  DEL_00_03 --> DEL_08_02
  DEL_00_04 --> DEL_08_02
  DEL_00_06 --> DEL_08_02
  DEL_00_07 --> DEL_08_02
  DEL_00_08 --> DEL_08_02
  DEL_02_05 --> DEL_08_02
  DEL_06_04 --> DEL_08_02
  DEL_00_01 --> DEL_08_03
  DEL_00_02 --> DEL_08_03
  DEL_00_03 --> DEL_08_03
  DEL_00_04 --> DEL_08_03
  DEL_00_06 --> DEL_08_03
  DEL_00_07 --> DEL_08_03
  DEL_00_08 --> DEL_08_03
  DEL_01_04 --> DEL_08_03
  DEL_03_07 --> DEL_08_03
  DEL_04_06 --> DEL_08_03
  DEL_05_04 --> DEL_08_03
  DEL_00_01 --> DEL_08_04
  DEL_00_02 --> DEL_08_04
  DEL_00_03 --> DEL_08_04
  DEL_00_04 --> DEL_08_04
  DEL_00_06 --> DEL_08_04
  DEL_00_07 --> DEL_08_04
  DEL_00_08 --> DEL_08_04
  DEL_02_01 --> DEL_08_04
  DEL_02_02 --> DEL_08_04
  DEL_06_04 --> DEL_08_04
  DEL_08_02 --> DEL_08_04
  DEL_00_01 --> DEL_08_05
  DEL_00_02 --> DEL_08_05
  DEL_00_03 --> DEL_08_05
  DEL_00_04 --> DEL_08_05
  DEL_00_06 --> DEL_08_05
  DEL_00_07 --> DEL_08_05
  DEL_00_08 --> DEL_08_05
  DEL_01_02 --> DEL_08_05
  DEL_01_04 --> DEL_08_05
  DEL_08_01 --> DEL_08_05
  DEL_00_01 --> DEL_08_06
  DEL_00_02 --> DEL_08_06
  DEL_00_03 --> DEL_08_06
  DEL_00_04 --> DEL_08_06
  DEL_00_06 --> DEL_08_06
  DEL_00_07 --> DEL_08_06
  DEL_00_08 --> DEL_08_06
  DEL_01_04 --> DEL_08_06
  DEL_08_01 --> DEL_08_06
  DEL_08_02 --> DEL_08_06
  DEL_08_03 --> DEL_08_06
  DEL_08_04 --> DEL_08_06
  DEL_08_05 --> DEL_08_06
  DEL_12_02 --> DEL_08_06
  DEL_14_01 --> DEL_08_06
  DEL_14_02 --> DEL_08_06
  DEL_14_03 --> DEL_08_06
  DEL_14_04 --> DEL_08_06
  DEL_14_05 --> DEL_08_06
  DEL_15_01 --> DEL_08_06
  DEL_15_03 --> DEL_08_06
  DEL_15_04 --> DEL_08_06
  DEL_00_01 --> DEL_09_01
  DEL_00_02 --> DEL_09_01
  DEL_00_06 --> DEL_09_01
  DEL_00_08 --> DEL_09_01
  DEL_01_02 --> DEL_09_01
  DEL_02_02 --> DEL_09_01
  DEL_04_01 --> DEL_09_01
  DEL_04_02 --> DEL_09_01
  DEL_04_03 --> DEL_09_01
  DEL_04_06 --> DEL_09_01
  DEL_05_01 --> DEL_09_01
  DEL_00_01 --> DEL_09_02
  DEL_00_02 --> DEL_09_02
  DEL_00_06 --> DEL_09_02
  DEL_00_08 --> DEL_09_02
  DEL_01_02 --> DEL_09_02
  DEL_03_08 --> DEL_09_02
  DEL_05_03 --> DEL_09_02
  DEL_00_01 --> DEL_09_03
  DEL_00_02 --> DEL_09_03
  DEL_00_06 --> DEL_09_03
  DEL_00_08 --> DEL_09_03
  DEL_04_04 --> DEL_09_03
  DEL_04_06 --> DEL_09_03
  DEL_00_01 --> DEL_09_04
  DEL_00_02 --> DEL_09_04
  DEL_00_06 --> DEL_09_04
  DEL_00_08 --> DEL_09_04
  DEL_01_04 --> DEL_09_04
  DEL_09_01 --> DEL_09_04
  DEL_09_02 --> DEL_09_04
  DEL_09_03 --> DEL_09_04
  DEL_00_01 --> DEL_09_05
  DEL_00_02 --> DEL_09_05
  DEL_00_06 --> DEL_09_05
  DEL_00_08 --> DEL_09_05
  DEL_01_04 --> DEL_09_05
  DEL_08_05 --> DEL_09_05
  DEL_09_01 --> DEL_09_05
  DEL_09_02 --> DEL_09_05
  DEL_09_03 --> DEL_09_05
  DEL_09_04 --> DEL_09_05
  DEL_00_01 --> DEL_10_01
  DEL_00_02 --> DEL_10_01
  DEL_00_03 --> DEL_10_01
  DEL_00_04 --> DEL_10_01
  DEL_00_06 --> DEL_10_01
  DEL_00_07 --> DEL_10_01
  DEL_00_08 --> DEL_10_01
  DEL_02_01 --> DEL_10_01
  DEL_02_02 --> DEL_10_01
  DEL_02_03 --> DEL_10_01
  DEL_02_04 --> DEL_10_01
  DEL_00_03 --> DEL_10_02
  DEL_00_04 --> DEL_10_02
  DEL_00_06 --> DEL_10_02
  DEL_00_07 --> DEL_10_02
  DEL_00_08 --> DEL_10_02
  DEL_02_04 --> DEL_10_02
  DEL_03_07 --> DEL_10_02
  DEL_08_04 --> DEL_10_02
  DEL_12_01 --> DEL_10_02
  DEL_00_01 --> DEL_10_03
  DEL_00_02 --> DEL_10_03
  DEL_00_03 --> DEL_10_03
  DEL_00_04 --> DEL_10_03
  DEL_00_06 --> DEL_10_03
  DEL_00_07 --> DEL_10_03
  DEL_00_08 --> DEL_10_03
  DEL_01_04 --> DEL_10_03
  DEL_04_01 --> DEL_10_03
  DEL_05_03 --> DEL_10_03
  DEL_08_04 --> DEL_10_03
  DEL_10_01 --> DEL_10_03
  DEL_10_02 --> DEL_10_03
  DEL_00_01 --> DEL_10_04
  DEL_00_02 --> DEL_10_04
  DEL_00_03 --> DEL_10_04
  DEL_00_04 --> DEL_10_04
  DEL_00_06 --> DEL_10_04
  DEL_00_07 --> DEL_10_04
  DEL_00_08 --> DEL_10_04
  DEL_09_05 --> DEL_10_04
  DEL_00_03 --> DEL_10_05
  DEL_00_06 --> DEL_10_05
  DEL_02_02 --> DEL_10_05
  DEL_02_05 --> DEL_10_05
  DEL_04_06 --> DEL_10_05
  DEL_08_02 --> DEL_10_05
  DEL_08_04 --> DEL_10_05
  DEL_10_04 --> DEL_10_05
  DEL_00_01 --> DEL_11_01
  DEL_00_02 --> DEL_11_01
  DEL_00_06 --> DEL_11_01
  DEL_00_07 --> DEL_11_01
  DEL_00_08 --> DEL_11_01
  DEL_01_02 --> DEL_11_01
  DEL_01_04 --> DEL_11_01
  DEL_05_04 --> DEL_11_01
  DEL_06_03 --> DEL_11_01
  DEL_07_01 --> DEL_11_01
  DEL_07_03 --> DEL_11_01
  DEL_07_04 --> DEL_11_01
  DEL_07_05 --> DEL_11_01
  DEL_07_07 --> DEL_11_01
  DEL_08_01 --> DEL_11_01
  DEL_08_02 --> DEL_11_01
  DEL_08_03 --> DEL_11_01
  DEL_00_01 --> DEL_11_02
  DEL_00_02 --> DEL_11_02
  DEL_00_06 --> DEL_11_02
  DEL_00_07 --> DEL_11_02
  DEL_00_08 --> DEL_11_02
  DEL_01_02 --> DEL_11_02
  DEL_02_01 --> DEL_11_02
  DEL_02_02 --> DEL_11_02
  DEL_04_01 --> DEL_11_02
  DEL_06_01 --> DEL_11_02
  DEL_10_01 --> DEL_11_02
  DEL_00_01 --> DEL_11_03
  DEL_00_02 --> DEL_11_03
  DEL_00_06 --> DEL_11_03
  DEL_00_07 --> DEL_11_03
  DEL_00_08 --> DEL_11_03
  DEL_01_02 --> DEL_11_03
  DEL_04_01 --> DEL_11_03
  DEL_04_02 --> DEL_11_03
  DEL_09_01 --> DEL_11_03
  DEL_00_01 --> DEL_11_04
  DEL_00_02 --> DEL_11_04
  DEL_00_06 --> DEL_11_04
  DEL_00_07 --> DEL_11_04
  DEL_00_08 --> DEL_11_04
  DEL_01_02 --> DEL_11_04
  DEL_02_01 --> DEL_11_04
  DEL_02_05 --> DEL_11_04
  DEL_06_01 --> DEL_11_04
  DEL_06_05 --> DEL_11_04
  DEL_08_05 --> DEL_11_04
  DEL_09_01 --> DEL_11_04
  DEL_09_02 --> DEL_11_04
  DEL_00_01 --> DEL_11_05
  DEL_00_02 --> DEL_11_05
  DEL_00_06 --> DEL_11_05
  DEL_00_07 --> DEL_11_05
  DEL_00_08 --> DEL_11_05
  DEL_01_01 --> DEL_11_05
  DEL_01_02 --> DEL_11_05
  DEL_01_03 --> DEL_11_05
  DEL_00_01 --> DEL_12_01
  DEL_00_02 --> DEL_12_01
  DEL_00_03 --> DEL_12_01
  DEL_00_04 --> DEL_12_01
  DEL_00_06 --> DEL_12_01
  DEL_00_07 --> DEL_12_01
  DEL_00_08 --> DEL_12_01
  DEL_01_02 --> DEL_12_01
  DEL_02_05 --> DEL_12_01
  DEL_12_05 --> DEL_12_01
  DEL_00_01 --> DEL_12_02
  DEL_00_02 --> DEL_12_02
  DEL_00_03 --> DEL_12_02
  DEL_00_04 --> DEL_12_02
  DEL_00_06 --> DEL_12_02
  DEL_00_07 --> DEL_12_02
  DEL_00_08 --> DEL_12_02
  DEL_03_07 --> DEL_12_02
  DEL_06_04 --> DEL_12_02
  DEL_08_01 --> DEL_12_02
  DEL_08_04 --> DEL_12_02
  DEL_12_01 --> DEL_12_02
  DEL_12_05 --> DEL_12_02
  DEL_00_01 --> DEL_12_03
  DEL_00_02 --> DEL_12_03
  DEL_00_03 --> DEL_12_03
  DEL_00_04 --> DEL_12_03
  DEL_00_06 --> DEL_12_03
  DEL_00_07 --> DEL_12_03
  DEL_00_08 --> DEL_12_03
  DEL_01_02 --> DEL_12_03
  DEL_01_04 --> DEL_12_03
  DEL_05_04 --> DEL_12_03
  DEL_12_05 --> DEL_12_03
  DEL_00_01 --> DEL_12_04
  DEL_00_02 --> DEL_12_04
  DEL_00_03 --> DEL_12_04
  DEL_00_04 --> DEL_12_04
  DEL_00_06 --> DEL_12_04
  DEL_00_07 --> DEL_12_04
  DEL_00_08 --> DEL_12_04
  DEL_03_07 --> DEL_12_04
  DEL_06_04 --> DEL_12_04
  DEL_12_01 --> DEL_12_04
  DEL_12_02 --> DEL_12_04
  DEL_12_03 --> DEL_12_04
  DEL_12_05 --> DEL_12_04
  DEL_00_01 --> DEL_12_05
  DEL_00_02 --> DEL_12_05
  DEL_00_03 --> DEL_12_05
  DEL_00_04 --> DEL_12_05
  DEL_00_06 --> DEL_12_05
  DEL_00_07 --> DEL_12_05
  DEL_00_08 --> DEL_12_05
  DEL_01_02 --> DEL_12_05
  DEL_01_04 --> DEL_12_05
  DEL_02_04 --> DEL_12_05
  DEL_00_01 --> DEL_13_01
  DEL_00_02 --> DEL_13_01
  DEL_00_03 --> DEL_13_01
  DEL_00_04 --> DEL_13_01
  DEL_00_06 --> DEL_13_01
  DEL_00_07 --> DEL_13_01
  DEL_00_08 --> DEL_13_01
  DEL_01_02 --> DEL_13_01
  DEL_01_04 --> DEL_13_01
  DEL_02_01 --> DEL_13_01
  DEL_02_02 --> DEL_13_01
  DEL_00_01 --> DEL_13_02
  DEL_00_02 --> DEL_13_02
  DEL_00_03 --> DEL_13_02
  DEL_00_04 --> DEL_13_02
  DEL_00_06 --> DEL_13_02
  DEL_00_07 --> DEL_13_02
  DEL_00_08 --> DEL_13_02
  DEL_01_04 --> DEL_13_02
  DEL_02_01 --> DEL_13_02
  DEL_02_02 --> DEL_13_02
  DEL_02_05 --> DEL_13_02
  DEL_13_01 --> DEL_13_02
  DEL_00_01 --> DEL_13_03
  DEL_00_02 --> DEL_13_03
  DEL_00_03 --> DEL_13_03
  DEL_00_04 --> DEL_13_03
  DEL_00_06 --> DEL_13_03
  DEL_00_07 --> DEL_13_03
  DEL_00_08 --> DEL_13_03
  DEL_02_02 --> DEL_13_03
  DEL_02_05 --> DEL_13_03
  DEL_04_06 --> DEL_13_03
  DEL_13_01 --> DEL_13_03
  DEL_13_02 --> DEL_13_03
  DEL_00_01 --> DEL_13_04
  DEL_00_02 --> DEL_13_04
  DEL_00_03 --> DEL_13_04
  DEL_00_04 --> DEL_13_04
  DEL_00_06 --> DEL_13_04
  DEL_00_07 --> DEL_13_04
  DEL_00_08 --> DEL_13_04
  DEL_02_01 --> DEL_13_04
  DEL_04_01 --> DEL_13_04
  DEL_04_03 --> DEL_13_04
  DEL_05_01 --> DEL_13_04
  DEL_13_01 --> DEL_13_04
  DEL_13_02 --> DEL_13_04
  DEL_13_03 --> DEL_13_04
  DEL_00_01 --> DEL_14_01
  DEL_00_02 --> DEL_14_01
  DEL_00_03 --> DEL_14_01
  DEL_00_04 --> DEL_14_01
  DEL_00_06 --> DEL_14_01
  DEL_00_07 --> DEL_14_01
  DEL_00_08 --> DEL_14_01
  DEL_02_01 --> DEL_14_01
  DEL_02_05 --> DEL_14_01
  DEL_05_04 --> DEL_14_01
  DEL_08_02 --> DEL_14_01
  DEL_00_01 --> DEL_14_02
  DEL_00_02 --> DEL_14_02
  DEL_00_03 --> DEL_14_02
  DEL_00_04 --> DEL_14_02
  DEL_00_06 --> DEL_14_02
  DEL_00_07 --> DEL_14_02
  DEL_00_08 --> DEL_14_02
  DEL_02_05 --> DEL_14_02
  DEL_05_04 --> DEL_14_02
  DEL_08_02 --> DEL_14_02
  DEL_08_04 --> DEL_14_02
  DEL_14_01 --> DEL_14_02
  DEL_00_01 --> DEL_14_03
  DEL_00_02 --> DEL_14_03
  DEL_00_03 --> DEL_14_03
  DEL_00_04 --> DEL_14_03
  DEL_00_06 --> DEL_14_03
  DEL_00_07 --> DEL_14_03
  DEL_00_08 --> DEL_14_03
  DEL_02_02 --> DEL_14_03
  DEL_14_01 --> DEL_14_03
  DEL_14_05 --> DEL_14_03
  DEL_00_01 --> DEL_14_04
  DEL_00_02 --> DEL_14_04
  DEL_00_03 --> DEL_14_04
  DEL_00_04 --> DEL_14_04
  DEL_00_06 --> DEL_14_04
  DEL_00_07 --> DEL_14_04
  DEL_00_08 --> DEL_14_04
  DEL_02_02 --> DEL_14_04
  DEL_08_04 --> DEL_14_04
  DEL_14_02 --> DEL_14_04
  DEL_14_05 --> DEL_14_04
  DEL_00_01 --> DEL_14_05
  DEL_00_02 --> DEL_14_05
  DEL_00_03 --> DEL_14_05
  DEL_00_04 --> DEL_14_05
  DEL_00_06 --> DEL_14_05
  DEL_00_07 --> DEL_14_05
  DEL_00_08 --> DEL_14_05
  DEL_02_02 --> DEL_14_05
  DEL_08_04 --> DEL_14_05
  DEL_14_01 --> DEL_14_05
  DEL_14_02 --> DEL_14_05
  DEL_00_01 --> DEL_15_01
  DEL_00_02 --> DEL_15_01
  DEL_00_03 --> DEL_15_01
  DEL_00_04 --> DEL_15_01
  DEL_00_06 --> DEL_15_01
  DEL_00_07 --> DEL_15_01
  DEL_00_08 --> DEL_15_01
  DEL_02_01 --> DEL_15_01
  DEL_08_02 --> DEL_15_01
  DEL_08_04 --> DEL_15_01
  DEL_10_03 --> DEL_15_01
  DEL_14_01 --> DEL_15_01
  DEL_14_02 --> DEL_15_01
  DEL_00_01 --> DEL_15_02
  DEL_00_02 --> DEL_15_02
  DEL_00_03 --> DEL_15_02
  DEL_00_04 --> DEL_15_02
  DEL_00_06 --> DEL_15_02
  DEL_00_07 --> DEL_15_02
  DEL_00_08 --> DEL_15_02
  DEL_10_02 --> DEL_15_02
  DEL_10_03 --> DEL_15_02
  DEL_12_02 --> DEL_15_02
  DEL_13_04 --> DEL_15_02
  DEL_14_05 --> DEL_15_02
  DEL_15_01 --> DEL_15_02
  DEL_00_01 --> DEL_15_03
  DEL_00_02 --> DEL_15_03
  DEL_00_03 --> DEL_15_03
  DEL_00_04 --> DEL_15_03
  DEL_00_06 --> DEL_15_03
  DEL_00_07 --> DEL_15_03
  DEL_00_08 --> DEL_15_03
  DEL_10_02 --> DEL_15_03
  DEL_10_03 --> DEL_15_03
  DEL_12_02 --> DEL_15_03
  DEL_13_04 --> DEL_15_03
  DEL_14_05 --> DEL_15_03
  DEL_15_01 --> DEL_15_03
  DEL_15_02 --> DEL_15_03
  DEL_00_01 --> DEL_15_04
  DEL_00_02 --> DEL_15_04
  DEL_00_03 --> DEL_15_04
  DEL_00_04 --> DEL_15_04
  DEL_00_06 --> DEL_15_04
  DEL_00_07 --> DEL_15_04
  DEL_00_08 --> DEL_15_04
  DEL_01_04 --> DEL_15_04
  DEL_14_01 --> DEL_15_04
  DEL_15_01 --> DEL_15_04
  DEL_15_02 --> DEL_15_04
  DEL_15_03 --> DEL_15_04
  DEL_00_01 --> DEL_16_01
  DEL_00_02 --> DEL_16_01
  DEL_00_03 --> DEL_16_01
  DEL_00_04 --> DEL_16_01
  DEL_00_06 --> DEL_16_01
  DEL_00_07 --> DEL_16_01
  DEL_00_08 --> DEL_16_01
  DEL_01_04 --> DEL_16_01
  DEL_02_01 --> DEL_16_01
  DEL_02_05 --> DEL_16_01
  DEL_13_01 --> DEL_16_01
  DEL_00_01 --> DEL_16_02
  DEL_00_02 --> DEL_16_02
  DEL_00_03 --> DEL_16_02
  DEL_00_04 --> DEL_16_02
  DEL_00_06 --> DEL_16_02
  DEL_00_07 --> DEL_16_02
  DEL_00_08 --> DEL_16_02
  DEL_04_06 --> DEL_16_02
  DEL_13_03 --> DEL_16_02
  DEL_14_03 --> DEL_16_02
  DEL_14_05 --> DEL_16_02
  DEL_16_01 --> DEL_16_02
  DEL_00_01 --> DEL_16_03
  DEL_00_02 --> DEL_16_03
  DEL_00_03 --> DEL_16_03
  DEL_00_04 --> DEL_16_03
  DEL_00_06 --> DEL_16_03
  DEL_00_07 --> DEL_16_03
  DEL_00_08 --> DEL_16_03
  DEL_02_05 --> DEL_16_03
  DEL_08_02 --> DEL_16_03
  DEL_14_01 --> DEL_16_03
  DEL_16_01 --> DEL_16_03
  DEL_16_02 --> DEL_16_03
  DEL_00_01 --> DEL_16_04
  DEL_00_02 --> DEL_16_04
  DEL_00_03 --> DEL_16_04
  DEL_00_04 --> DEL_16_04
  DEL_00_06 --> DEL_16_04
  DEL_00_07 --> DEL_16_04
  DEL_00_08 --> DEL_16_04
  DEL_01_04 --> DEL_16_04
  DEL_12_05 --> DEL_16_04
  DEL_16_03 --> DEL_16_04
```
