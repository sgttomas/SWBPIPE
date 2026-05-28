---
doc_id: DEV-001-BLOCKER-QUEUE
doc_kind: coordination.blocker_queue
status: computed_active_edges_only
created: 2026-04-30
updated: 2026-05-28
source_graph: execution/_DAG/DAG-005/DependencyEdges.csv
implementation_evidence_source: execution/_Coordination/DEV-001_IMPLEMENTATION_EVIDENCE.csv
implementation_threshold: COMMITTED
architecture_basis: satisfied_by_existing_baseline
candidate_edges: excluded
---

# DEV-001 Implementation-Readiness Blocker Queue

This blocker queue is an advisory implementation-readiness view only. It is not a schedule, staffing plan, priority list, lifecycle approval, professional approval, or readiness-for-reliance claim.

## Computation Rule

- Source graph: `execution/_DAG/DAG-005/DependencyEdges.csv`.
- Included edges: `Status=ACTIVE` only.
- Excluded edges: all `Status=CANDIDATE` rows.
- Direction convention: `FromDeliverableID` is the downstream consumer and is blocked by `TargetDeliverableID`, the upstream provider.
- Satisfaction threshold: upstream provider has `COMMITTED` evidence in `execution/_Coordination/DEV-001_IMPLEMENTATION_EVIDENCE.csv`.
- `SEMANTIC_READY` remains decomposition/context readiness evidence; it does not satisfy implementation blockers by itself.
- `PKG-00` provider edges are satisfied by the accepted architecture baseline, not by implementation evidence.
- `UNBLOCKED` means all active upstream implementation dependencies satisfy the threshold or are satisfied architecture-basis edges.
- `BLOCKED` means one or more active upstream providers lack committed implementation evidence.

## Evidence Summary

| Evidence | Count |
|---|---:|
| Packages represented | 18 |
| Deliverable nodes represented | 101 |
| Active edges included | 945 |
| Candidate edges excluded | 11 |
| Implementation evidence records | 93 |
| Committed implementation evidence | 93 |
| Filesystem lifecycle `SEMANTIC_READY` (display only) | 17 |
| PKG-00 architecture-basis edges satisfied | 577 |
| Implementation `UNBLOCKED` deliverables | 101 |
| Implementation `BLOCKED` deliverables | 0 |

## Package Summary

| PackageID | UNBLOCKED | BLOCKED |
|---|---:|---:|
| `PKG-00` | 8 | 0 |
| `PKG-01` | 4 | 0 |
| `PKG-02` | 5 | 0 |
| `PKG-03` | 8 | 0 |
| `PKG-04` | 6 | 0 |
| `PKG-05` | 5 | 0 |
| `PKG-06` | 5 | 0 |
| `PKG-07` | 8 | 0 |
| `PKG-08` | 6 | 0 |
| `PKG-09` | 5 | 0 |
| `PKG-10` | 5 | 0 |
| `PKG-11` | 5 | 0 |
| `PKG-12` | 5 | 0 |
| `PKG-13` | 4 | 0 |
| `PKG-14` | 5 | 0 |
| `PKG-15` | 4 | 0 |
| `PKG-16` | 4 | 0 |
| `PKG-17` | 9 | 0 |

## Unblocked DAG-Ready Items

These deliverables have no active upstream implementation dependency below the `COMMITTED` threshold. Items without their own committed evidence are DAG-ready candidates, not completed work.

| DeliverableID | PackageID | Implementation evidence | Active upstream | Name |
|---|---|---|---:|---|
| `DEL-00-01` | `PKG-00` | `ARCHITECTURE_BASELINE` | 0 | Architecture decision record baseline |
| `DEL-00-02` | `PKG-00` | `ARCHITECTURE_BASELINE` | 0 | Repository and module boundary architecture |
| `DEL-00-03` | `PKG-00` | `ARCHITECTURE_BASELINE` | 0 | Application service command-query-job model |
| `DEL-00-04` | `PKG-00` | `ARCHITECTURE_BASELINE` | 0 | Persistence and schema versioning architecture |
| `DEL-00-05` | `PKG-00` | `ARCHITECTURE_BASELINE` | 0 | GUI state and interaction architecture |
| `DEL-00-06` | `PKG-00` | `ARCHITECTURE_BASELINE` | 0 | Diagnostics, warning, and result-envelope contract |
| `DEL-00-07` | `PKG-00` | `ARCHITECTURE_BASELINE` | 0 | API boundary and adapter contract map |
| `DEL-00-08` | `PKG-00` | `ARCHITECTURE_BASELINE` | 0 | Layered software test and acceptance strategy |
| `DEL-01-01` | `PKG-01` | `COMMITTED` `7650cf6` | 4 | Project governance baseline |
| `DEL-01-02` | `PKG-01` | `COMMITTED` `0d729cf` | 5 | Copyright and protected-data boundary policy |
| `DEL-01-03` | `PKG-01` | `COMMITTED` `df461f8` | 6 | Contributor certification workflow |
| `DEL-01-04` | `PKG-01` | `COMMITTED` `65f3119` | 4 | Professional responsibility and product-claims policy |
| `DEL-02-01` | `PKG-02` | `COMMITTED` `7b256f3` | 7 | Canonical domain model schema |
| `DEL-02-02` | `PKG-02` | `COMMITTED` `a458cba` | 8 | Unit system and dimensional-analysis core contract |
| `DEL-02-03` | `PKG-02` | `COMMITTED` `abc1306` | 8 | Code-neutral analysis boundary model |
| `DEL-02-04` | `PKG-02` | `COMMITTED` `ef44f4c` | 7 | Plugin and extension domain contracts |
| `DEL-02-05` | `PKG-02` | `COMMITTED` `742016e` | 10 | Project persistence and round-trip serialization |
| `DEL-03-01` | `PKG-03` | `COMMITTED` `3793e87` | 10 | Material library schema with provenance |
| `DEL-03-02` | `PKG-03` | `COMMITTED` `f0fdeac` | 10 | Pipe section and component library schema |
| `DEL-03-03` | `PKG-03` | `COMMITTED` `7a84472` | 9 | Bend and elbow component model fields |
| `DEL-03-04` | `PKG-03` | `COMMITTED` `ae693b6` | 9 | Branch connection component model fields |
| `DEL-03-05` | `PKG-03` | `COMMITTED` `d8ee0db` | 9 | Rigid component models for valves, flanges, reducers, and specialty items |
| `DEL-03-06` | `PKG-03` | `COMMITTED` `f15cbc6` | 9 | Expansion joint component model |
| `DEL-03-07` | `PKG-03` | `COMMITTED` `4d880b3` | 11 | Public/private library import provenance checker |
| `DEL-03-08` | `PKG-03` | `COMMITTED` `9712e98` | 9 | Pipe section property and mass-property calculator |
| `DEL-04-01` | `PKG-04` | `COMMITTED` `1506cc0` | 8 | 3D frame stiffness kernel |
| `DEL-04-02` | `PKG-04` | `COMMITTED` `b0516e5` | 8 | Straight pipe element |
| `DEL-04-03` | `PKG-04` | `COMMITTED` `d227a27` | 8 | Linear support and restraint models |
| `DEL-04-04` | `PKG-04` | `COMMITTED` `d3c3533` | 9 | Nonlinear support active-set solver |
| `DEL-04-05` | `PKG-04` | `COMMITTED` `75f6688` | 7 | Sparse solver performance harness |
| `DEL-04-06` | `PKG-04` | `COMMITTED` `fdb0252` | 8 | Solver diagnostics and singularity detection |
| `DEL-05-01` | `PKG-05` | `COMMITTED` `e3c9695` | 5 | Primitive load case engine |
| `DEL-05-02` | `PKG-05` | `COMMITTED` `0f9189c` | 9 | Load-case algebra engine |
| `DEL-05-03` | `PKG-05` | `COMMITTED` `26dc805` | 10 | Fundamental stress recovery module |
| `DEL-05-04` | `PKG-05` | `COMMITTED` `dbaf21e` | 6 | Analysis status semantics |
| `DEL-05-05` | `PKG-05` | `COMMITTED` `3cfcfd2` | 9 | Concentrated and distributed user load application |
| `DEL-06-01` | `PKG-06` | `COMMITTED` `20241f9` | 12 | Rule-pack schema |
| `DEL-06-02` | `PKG-06` | `COMMITTED` `7490f67` | 9 | Sandboxed unit-aware expression evaluator |
| `DEL-06-03` | `PKG-06` | `COMMITTED` `c075522` | 10 | Required-input completeness checker |
| `DEL-06-04` | `PKG-06` | `COMMITTED` `ad270f6` | 9 | Private rule-pack lifecycle and checksum handling |
| `DEL-06-05` | `PKG-06` | `COMMITTED` `73506b7` | 9 | Invented non-code example rule pack |
| `DEL-07-01` | `PKG-07` | `COMMITTED` `4785806` | 15 | 3D viewport and centerline editor |
| `DEL-07-02` | `PKG-07` | `COMMITTED` `6e0b8f4` | 16 | Model tree and property inspector |
| `DEL-07-03` | `PKG-07` | `COMMITTED` `6e0b8f4` | 13 | Material, component, and rule-pack editors |
| `DEL-07-04` | `PKG-07` | `COMMITTED` `6e0b8f4` | 10 | Missing-data warning and blocking UX |
| `DEL-07-05` | `PKG-07` | `COMMITTED` `6e0b8f4` | 10 | Results viewer |
| `DEL-07-06` | `PKG-07` | `COMMITTED` `bfb3931` | 7 | Accessibility and usability baseline |
| `DEL-07-07` | `PKG-07` | `COMMITTED` `6e0b8f4` | 12 | Solve execution UX: progress, cancellation, and diagnostics |
| `DEL-08-01` | `PKG-08` | `COMMITTED` `9e21716` | 14 | Calculation report generator |
| `DEL-08-02` | `PKG-08` | `COMMITTED` `061f1af` | 9 | Audit manifest and model hash |
| `DEL-08-03` | `PKG-08` | `COMMITTED` `50f947a` | 11 | Warnings, assumptions, and provenance report section |
| `DEL-08-04` | `PKG-08` | `COMMITTED` `3e33ea4` | 15 | Result export format |
| `DEL-08-05` | `PKG-08` | `COMMITTED` `69adffa` | 10 | Report protected-content linter |
| `DEL-09-01` | `PKG-09` | `COMMITTED` `b34ecd6` | 11 | Mechanics benchmark suite |
| `DEL-09-02` | `PKG-09` | `COMMITTED` `bf1dc20` | 7 | Stress recovery benchmark suite |
| `DEL-09-03` | `PKG-09` | `COMMITTED` `abdecbd` | 6 | Nonlinear support regression suite |
| `DEL-09-04` | `PKG-09` | `COMMITTED` `03344e6` | 9 | Validation manual skeleton |
| `DEL-09-05` | `PKG-09` | `COMMITTED` `03344e6` | 10 | Release quality gate checklist |
| `DEL-10-01` | `PKG-10` | `COMMITTED` `53cc3d6` | 11 | Public API and plugin boundary |
| `DEL-10-02` | `PKG-10` | `COMMITTED` `be29df7` | 9 | Import/export adapter framework |
| `DEL-10-03` | `PKG-10` | `COMMITTED` `abdecbd` | 12 | Local FEA handoff data contract |
| `DEL-10-04` | `PKG-10` | `COMMITTED` `daaff87` | 7 | Build, packaging, and CI/CD pipeline |
| `DEL-10-05` | `PKG-10` | `COMMITTED` `9de5e9b` | 7 | Headless CLI and structured I/O analysis runner |
| `DEL-11-01` | `PKG-11` | `COMMITTED` `bfb3931` | 17 | User guide skeleton |
| `DEL-11-02` | `PKG-11` | `COMMITTED` `abdecbd` | 11 | Developer guide for solver and rule packs |
| `DEL-11-03` | `PKG-11` | `COMMITTED` `abdecbd` | 9 | Theory notes: classical to modern centerline analysis |
| `DEL-11-04` | `PKG-11` | `COMMITTED` `abdecbd` | 13 | Invented educational example models |
| `DEL-11-05` | `PKG-11` | `COMMITTED` `bfb3931` | 8 | Contributor tutorial and onboarding |
| `DEL-12-01` | `PKG-12` | `COMMITTED` `84e0a73` | 12 | Local-first storage and private data paths |
| `DEL-12-02` | `PKG-12` | `COMMITTED` `abdecbd` | 11 | Private data redaction and export controls |
| `DEL-12-03` | `PKG-12` | `COMMITTED` `7834b97` | 11 | Telemetry off-by-default design |
| `DEL-12-04` | `PKG-12` | `COMMITTED` `bfb3931` | 12 | Secret and private-library handling |
| `DEL-12-05` | `PKG-12` | `COMMITTED` `b97121d` | 11 | Security threat model |
| `DEL-07-08` | `PKG-07` | `COMMITTED` `bfb3931` | 17 | Design-authoring state and comparison workspace |
| `DEL-08-06` | `PKG-08` | `COMMITTED` `cf6ffb9` | 20 | State, comparison, and handoff report sections |
| `DEL-13-01` | `PKG-13` | `COMMITTED` `dcdc1ac` | 11 | Design knowledge schema and provenance model |
| `DEL-13-02` | `PKG-13` | `COMMITTED` `002263b` | 12 | Constraint entity and provenance model |
| `DEL-13-03` | `PKG-13` | `COMMITTED` `05878bf` | 12 | Constraint validation engine |
| `DEL-13-04` | `PKG-13` | `COMMITTED` `24b5717` | 15 | Physical-to-analytical transformation contract |
| `DEL-14-01` | `PKG-14` | `COMMITTED` `dcdc1ac` | 11 | Immutable model state records |
| `DEL-14-02` | `PKG-14` | `COMMITTED` `002263b` | 12 | Analysis run records |
| `DEL-14-03` | `PKG-14` | `COMMITTED` `24b5717` | 13 | Model-state comparison engine |
| `DEL-14-04` | `PKG-14` | `COMMITTED` `24b5717` | 13 | Analysis-run comparison engine |
| `DEL-14-05` | `PKG-14` | `COMMITTED` `05878bf` | 11 | Comparison mapping, tolerance, and export contracts |
| `DEL-15-01` | `PKG-15` | `COMMITTED` `05878bf` | 13 | Canonical handoff package schema and manifest |
| `DEL-15-02` | `PKG-15` | `COMMITTED` `c08b0a2` | 15 | Target mapping and unsupported-behavior contract |
| `DEL-15-03` | `PKG-15` | `COMMITTED` `4601724` | 13 | Downstream modeling export workflow |
| `DEL-15-04` | `PKG-15` | `COMMITTED` `68d863b` | 11 | External prover boundary metadata |
| `DEL-16-01` | `PKG-16` | `COMMITTED` `002263b` | 13 | Structured model operation schema |
| `DEL-16-02` | `PKG-16` | `COMMITTED` `c08b0a2` | 10 | Operation validation and diff preview |
| `DEL-16-03` | `PKG-16` | `COMMITTED` `4601724` | 12 | User acceptance and operation audit trail |
| `DEL-16-04` | `PKG-16` | `COMMITTED` `68d863b` | 10 | Agent rationale and professional-boundary controls |
| `DEL-17-01` | `PKG-17` | `COMMITTED` `6f1e3427` | 7 | CAEPIPE and export-format source basis |
| `DEL-17-02` | `PKG-17` | `COMMITTED` `d0e1aa80` | 8 | Export package, profile, and stable ID map contracts |
| `DEL-17-03` | `PKG-17` | `COMMITTED` `019ef027` | 8 | Native open JSON export package |
| `DEL-17-04` | `PKG-17` | `COMMITTED` `8ebf8b33` | 8 | CAEPIPE MBF export profile and deterministic writer |
| `DEL-17-05` | `PKG-17` | `COMMITTED` `5961995f` | 8 | CAEPIPE external run harness and CSV parser |
| `DEL-17-06` | `PKG-17` | `COMMITTED` `5961995f` | 11 | Stress-neutral CSV/JSON package |
| `DEL-17-07` | `PKG-17` | `COMMITTED` `cd710e02` | 11 | Conservative PCF subset exporter |
| `DEL-17-08` | `PKG-17` | `COMMITTED` `3e174ff2` | 12 | GLB/glTF review geometry export |
| `DEL-17-09` | `PKG-17` | `COMMITTED` `3f04b6c0` | 11 | Export adapter SDK and additional targets |

## Blocked Items Grouped By Missing Upstream

No blocked items were found under the implementation-readiness threshold.

## Per-Deliverable Blocked Items

No per-deliverable implementation blockers were found.

## Candidate Edges Excluded

Candidate edges remain non-gating pending later `RECONCILIATION` and `CHANGE`; they were not used in the blocker state calculation.

## Machine-Readable Queue

Full per-deliverable queue rows are recorded in `execution/_Coordination/DEV-001_BLOCKER_QUEUE.csv`.
