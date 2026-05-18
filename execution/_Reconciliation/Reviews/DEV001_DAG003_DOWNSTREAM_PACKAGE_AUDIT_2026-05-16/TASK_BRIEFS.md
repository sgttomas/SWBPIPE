# TASK Briefs - DEV-001 DAG-003 Downstream Package Audit

## PKG-03 Package Audit Brief

```markdown
PURPOSE: Package-scoped downstream audit against PKG-02 foundation contracts.
RequestedBy: WORKING_ITEMS
TaskProfile: PACKAGE_AUDIT
PackageID: PKG-03

ScopePath: /Users/ryan/ai-env/projects/chirality-piping/execution/PKG-03_Piping Components, Materials, and Library Data Model/1_Working
AuditDeliverables:
  - DEL-03-01 - Material library schema with provenance
  - DEL-03-02 - Pipe section and component library schema
  - DEL-03-03 - Bend and elbow component model fields
  - DEL-03-04 - Branch connection component model fields
  - DEL-03-05 - Rigid component models for valves, flanges, reducers, and specialty items
  - DEL-03-06 - Expansion joint component model
  - DEL-03-07 - Public/private library import provenance checker
  - DEL-03-08 - Pipe section property and mass-property calculator

Tasks:
  - For each listed deliverable, read `_CONTEXT.md`, `_STATUS.md`, `_REFERENCES.md`, `_DEPENDENCIES.md`, `Dependencies.csv`, `MEMORY.md`, and primary deliverable artifacts.
  - Produce or update deliverable-local `_REVIEW.md`.
  - Produce or update deliverable-local `Review_Findings.csv`.
  - Create one package-level TASK run record under `/Users/ryan/ai-env/projects/chirality-piping/execution/PKG-03_Piping Components, Materials, and Library Data Model/1_Working/_run_records/`.
  - Produce a package audit summary under `/Users/ryan/ai-env/projects/chirality-piping/execution/PKG-03_Piping Components, Materials, and Library Data Model/1_Working/_audit/PKG02_DOWNSTREAM_REVIEW_2026-05-16.md`.
  - Classify each deliverable as `PASS`, `WARNING`, `BLOCKER`, or `NOT_APPLICABLE` for PKG-02 compatibility.

PKG-02 Compatibility Checks:
  - DEL-02-01 canonical model/schema and physical source-of-truth role.
  - DEL-02-02 explicit unit metadata and no silent unit defaults.
  - DEL-02-03 mechanics/rule/human authority separation.
  - DEL-02-04 plugin/adapter no-bypass constraints where applicable.
  - DEL-02-05 persistence/hash/provenance/round-trip assumptions where applicable.

AllowedWriteTargets:
  - /Users/ryan/ai-env/projects/chirality-piping/execution/PKG-03_Piping Components, Materials, and Library Data Model/1_Working/DEL-*/_REVIEW.md
  - /Users/ryan/ai-env/projects/chirality-piping/execution/PKG-03_Piping Components, Materials, and Library Data Model/1_Working/DEL-*/Review_Findings.csv
  - /Users/ryan/ai-env/projects/chirality-piping/execution/PKG-03_Piping Components, Materials, and Library Data Model/1_Working/_run_records/
  - /Users/ryan/ai-env/projects/chirality-piping/execution/PKG-03_Piping Components, Materials, and Library Data Model/1_Working/_audit/

Exclusions:
  - No edits to `_STATUS.md`, `_CONTEXT.md`, `_DEPENDENCIES.md`, `Dependencies.csv`, DAG files, blocker queues, source code, schemas, fixtures, tests, or primary deliverable artifacts.
  - No lifecycle transition.
  - No candidate promotion.
  - No release, professional reliance, certification, sealing, approval, or code-compliance claims.
```

## PKG-04 Package Audit Brief

```markdown
PURPOSE: Package-scoped downstream audit against PKG-02 foundation contracts.
RequestedBy: WORKING_ITEMS
TaskProfile: PACKAGE_AUDIT
PackageID: PKG-04

ScopePath: /Users/ryan/ai-env/projects/chirality-piping/execution/PKG-04_Solver Core and Numerical Methods/1_Working
AuditDeliverables:
  - DEL-04-01 - 3D frame stiffness kernel
  - DEL-04-02 - Straight pipe element
  - DEL-04-03 - Linear support and restraint models
  - DEL-04-04 - Nonlinear support active-set solver
  - DEL-04-05 - Sparse solver performance harness
  - DEL-04-06 - Solver diagnostics and singularity detection

Tasks:
  - For each listed deliverable, read `_CONTEXT.md`, `_STATUS.md`, `_REFERENCES.md`, `_DEPENDENCIES.md`, `Dependencies.csv`, `MEMORY.md`, and primary deliverable artifacts.
  - Produce or update deliverable-local `_REVIEW.md`.
  - Produce or update deliverable-local `Review_Findings.csv`.
  - Create one package-level TASK run record under `/Users/ryan/ai-env/projects/chirality-piping/execution/PKG-04_Solver Core and Numerical Methods/1_Working/_run_records/`.
  - Produce a package audit summary under `/Users/ryan/ai-env/projects/chirality-piping/execution/PKG-04_Solver Core and Numerical Methods/1_Working/_audit/PKG02_DOWNSTREAM_REVIEW_2026-05-16.md`.
  - Classify each deliverable as `PASS`, `WARNING`, `BLOCKER`, or `NOT_APPLICABLE` for PKG-02 compatibility.

PKG-02 Compatibility Checks:
  - DEL-02-01 canonical model/schema and physical source-of-truth role.
  - DEL-02-02 explicit unit metadata and no silent unit defaults.
  - DEL-02-03 mechanics/rule/human authority separation.
  - DEL-02-04 plugin/adapter no-bypass constraints where applicable.
  - DEL-02-05 persistence/hash/provenance/round-trip assumptions where applicable.

AllowedWriteTargets:
  - /Users/ryan/ai-env/projects/chirality-piping/execution/PKG-04_Solver Core and Numerical Methods/1_Working/DEL-*/_REVIEW.md
  - /Users/ryan/ai-env/projects/chirality-piping/execution/PKG-04_Solver Core and Numerical Methods/1_Working/DEL-*/Review_Findings.csv
  - /Users/ryan/ai-env/projects/chirality-piping/execution/PKG-04_Solver Core and Numerical Methods/1_Working/_run_records/
  - /Users/ryan/ai-env/projects/chirality-piping/execution/PKG-04_Solver Core and Numerical Methods/1_Working/_audit/

Exclusions:
  - No edits to `_STATUS.md`, `_CONTEXT.md`, `_DEPENDENCIES.md`, `Dependencies.csv`, DAG files, blocker queues, source code, schemas, fixtures, tests, or primary deliverable artifacts.
  - No lifecycle transition.
  - No candidate promotion.
  - No release, professional reliance, certification, sealing, approval, or code-compliance claims.
```

## PKG-05 Package Audit Brief

```markdown
PURPOSE: Package-scoped downstream audit against PKG-02 foundation contracts.
RequestedBy: WORKING_ITEMS
TaskProfile: PACKAGE_AUDIT
PackageID: PKG-05

ScopePath: /Users/ryan/ai-env/projects/chirality-piping/execution/PKG-05_Loads, Load Cases, and Stress Recovery/1_Working
AuditDeliverables:
  - DEL-05-02 - Load-case algebra engine
  - DEL-05-03 - Fundamental stress recovery module
  - DEL-05-04 - Analysis status semantics
  - DEL-05-05 - Concentrated and distributed user load application

Tasks:
  - For each listed deliverable, read `_CONTEXT.md`, `_STATUS.md`, `_REFERENCES.md`, `_DEPENDENCIES.md`, `Dependencies.csv`, `MEMORY.md`, and primary deliverable artifacts.
  - Produce or update deliverable-local `_REVIEW.md`.
  - Produce or update deliverable-local `Review_Findings.csv`.
  - Create one package-level TASK run record under `/Users/ryan/ai-env/projects/chirality-piping/execution/PKG-05_Loads, Load Cases, and Stress Recovery/1_Working/_run_records/`.
  - Produce a package audit summary under `/Users/ryan/ai-env/projects/chirality-piping/execution/PKG-05_Loads, Load Cases, and Stress Recovery/1_Working/_audit/PKG02_DOWNSTREAM_REVIEW_2026-05-16.md`.
  - Classify each deliverable as `PASS`, `WARNING`, `BLOCKER`, or `NOT_APPLICABLE` for PKG-02 compatibility.

PKG-02 Compatibility Checks:
  - DEL-02-01 canonical model/schema and physical source-of-truth role.
  - DEL-02-02 explicit unit metadata and no silent unit defaults.
  - DEL-02-03 mechanics/rule/human authority separation.
  - DEL-02-04 plugin/adapter no-bypass constraints where applicable.
  - DEL-02-05 persistence/hash/provenance/round-trip assumptions where applicable.

AllowedWriteTargets:
  - /Users/ryan/ai-env/projects/chirality-piping/execution/PKG-05_Loads, Load Cases, and Stress Recovery/1_Working/DEL-*/_REVIEW.md
  - /Users/ryan/ai-env/projects/chirality-piping/execution/PKG-05_Loads, Load Cases, and Stress Recovery/1_Working/DEL-*/Review_Findings.csv
  - /Users/ryan/ai-env/projects/chirality-piping/execution/PKG-05_Loads, Load Cases, and Stress Recovery/1_Working/_run_records/
  - /Users/ryan/ai-env/projects/chirality-piping/execution/PKG-05_Loads, Load Cases, and Stress Recovery/1_Working/_audit/

Exclusions:
  - No edits to `_STATUS.md`, `_CONTEXT.md`, `_DEPENDENCIES.md`, `Dependencies.csv`, DAG files, blocker queues, source code, schemas, fixtures, tests, or primary deliverable artifacts.
  - No lifecycle transition.
  - No candidate promotion.
  - No release, professional reliance, certification, sealing, approval, or code-compliance claims.
```

## PKG-06 Package Audit Brief

```markdown
PURPOSE: Package-scoped downstream audit against PKG-02 foundation contracts.
RequestedBy: WORKING_ITEMS
TaskProfile: PACKAGE_AUDIT
PackageID: PKG-06

ScopePath: /Users/ryan/ai-env/projects/chirality-piping/execution/PKG-06_Rule Packs and User-Supplied Code Check Engine/1_Working
AuditDeliverables:
  - DEL-06-01 - Rule-pack schema
  - DEL-06-02 - Sandboxed unit-aware expression evaluator
  - DEL-06-03 - Required-input completeness checker
  - DEL-06-04 - Private rule-pack lifecycle and checksum handling
  - DEL-06-05 - Invented non-code example rule pack

Tasks:
  - For each listed deliverable, read `_CONTEXT.md`, `_STATUS.md`, `_REFERENCES.md`, `_DEPENDENCIES.md`, `Dependencies.csv`, `MEMORY.md`, and primary deliverable artifacts.
  - Produce or update deliverable-local `_REVIEW.md`.
  - Produce or update deliverable-local `Review_Findings.csv`.
  - Create one package-level TASK run record under `/Users/ryan/ai-env/projects/chirality-piping/execution/PKG-06_Rule Packs and User-Supplied Code Check Engine/1_Working/_run_records/`.
  - Produce a package audit summary under `/Users/ryan/ai-env/projects/chirality-piping/execution/PKG-06_Rule Packs and User-Supplied Code Check Engine/1_Working/_audit/PKG02_DOWNSTREAM_REVIEW_2026-05-16.md`.
  - Classify each deliverable as `PASS`, `WARNING`, `BLOCKER`, or `NOT_APPLICABLE` for PKG-02 compatibility.

PKG-02 Compatibility Checks:
  - DEL-02-01 canonical model/schema and physical source-of-truth role.
  - DEL-02-02 explicit unit metadata and no silent unit defaults.
  - DEL-02-03 mechanics/rule/human authority separation.
  - DEL-02-04 plugin/adapter no-bypass constraints where applicable.
  - DEL-02-05 persistence/hash/provenance/round-trip assumptions where applicable.

AllowedWriteTargets:
  - /Users/ryan/ai-env/projects/chirality-piping/execution/PKG-06_Rule Packs and User-Supplied Code Check Engine/1_Working/DEL-*/_REVIEW.md
  - /Users/ryan/ai-env/projects/chirality-piping/execution/PKG-06_Rule Packs and User-Supplied Code Check Engine/1_Working/DEL-*/Review_Findings.csv
  - /Users/ryan/ai-env/projects/chirality-piping/execution/PKG-06_Rule Packs and User-Supplied Code Check Engine/1_Working/_run_records/
  - /Users/ryan/ai-env/projects/chirality-piping/execution/PKG-06_Rule Packs and User-Supplied Code Check Engine/1_Working/_audit/

Exclusions:
  - No edits to `_STATUS.md`, `_CONTEXT.md`, `_DEPENDENCIES.md`, `Dependencies.csv`, DAG files, blocker queues, source code, schemas, fixtures, tests, or primary deliverable artifacts.
  - No lifecycle transition.
  - No candidate promotion.
  - No release, professional reliance, certification, sealing, approval, or code-compliance claims.
```

## PKG-07 Package Audit Brief

```markdown
PURPOSE: Package-scoped downstream audit against PKG-02 foundation contracts.
RequestedBy: WORKING_ITEMS
TaskProfile: PACKAGE_AUDIT
PackageID: PKG-07

ScopePath: /Users/ryan/ai-env/projects/chirality-piping/execution/PKG-07_Graphical User Interface and Engineering Workflow/1_Working
AuditDeliverables:
  - DEL-07-01 - 3D viewport and centerline editor
  - DEL-07-02 - Model tree and property inspector
  - DEL-07-03 - Material, component, and rule-pack editors
  - DEL-07-04 - Missing-data warning and blocking UX
  - DEL-07-05 - Results viewer
  - DEL-07-07 - Solve execution UX: progress, cancellation, and diagnostics
  - DEL-07-08 - Design-authoring state and comparison workspace

Tasks:
  - For each listed deliverable, read `_CONTEXT.md`, `_STATUS.md`, `_REFERENCES.md`, `_DEPENDENCIES.md`, `Dependencies.csv`, `MEMORY.md`, and primary deliverable artifacts.
  - Produce or update deliverable-local `_REVIEW.md`.
  - Produce or update deliverable-local `Review_Findings.csv`.
  - Create one package-level TASK run record under `/Users/ryan/ai-env/projects/chirality-piping/execution/PKG-07_Graphical User Interface and Engineering Workflow/1_Working/_run_records/`.
  - Produce a package audit summary under `/Users/ryan/ai-env/projects/chirality-piping/execution/PKG-07_Graphical User Interface and Engineering Workflow/1_Working/_audit/PKG02_DOWNSTREAM_REVIEW_2026-05-16.md`.
  - Classify each deliverable as `PASS`, `WARNING`, `BLOCKER`, or `NOT_APPLICABLE` for PKG-02 compatibility.

PKG-02 Compatibility Checks:
  - DEL-02-01 canonical model/schema and physical source-of-truth role.
  - DEL-02-02 explicit unit metadata and no silent unit defaults.
  - DEL-02-03 mechanics/rule/human authority separation.
  - DEL-02-04 plugin/adapter no-bypass constraints where applicable.
  - DEL-02-05 persistence/hash/provenance/round-trip assumptions where applicable.

AllowedWriteTargets:
  - /Users/ryan/ai-env/projects/chirality-piping/execution/PKG-07_Graphical User Interface and Engineering Workflow/1_Working/DEL-*/_REVIEW.md
  - /Users/ryan/ai-env/projects/chirality-piping/execution/PKG-07_Graphical User Interface and Engineering Workflow/1_Working/DEL-*/Review_Findings.csv
  - /Users/ryan/ai-env/projects/chirality-piping/execution/PKG-07_Graphical User Interface and Engineering Workflow/1_Working/_run_records/
  - /Users/ryan/ai-env/projects/chirality-piping/execution/PKG-07_Graphical User Interface and Engineering Workflow/1_Working/_audit/

Exclusions:
  - No edits to `_STATUS.md`, `_CONTEXT.md`, `_DEPENDENCIES.md`, `Dependencies.csv`, DAG files, blocker queues, source code, schemas, fixtures, tests, or primary deliverable artifacts.
  - No lifecycle transition.
  - No candidate promotion.
  - No release, professional reliance, certification, sealing, approval, or code-compliance claims.
```

## PKG-08 Package Audit Brief

```markdown
PURPOSE: Package-scoped downstream audit against PKG-02 foundation contracts.
RequestedBy: WORKING_ITEMS
TaskProfile: PACKAGE_AUDIT
PackageID: PKG-08

ScopePath: /Users/ryan/ai-env/projects/chirality-piping/execution/PKG-08_Reporting, Audit, and Reproducibility/1_Working
AuditDeliverables:
  - DEL-08-01 - Calculation report generator
  - DEL-08-02 - Audit manifest and model hash
  - DEL-08-03 - Warnings, assumptions, and provenance report section
  - DEL-08-04 - Result export format
  - DEL-08-05 - Report protected-content linter
  - DEL-08-06 - State, comparison, and handoff report sections

Tasks:
  - For each listed deliverable, read `_CONTEXT.md`, `_STATUS.md`, `_REFERENCES.md`, `_DEPENDENCIES.md`, `Dependencies.csv`, `MEMORY.md`, and primary deliverable artifacts.
  - Produce or update deliverable-local `_REVIEW.md`.
  - Produce or update deliverable-local `Review_Findings.csv`.
  - Create one package-level TASK run record under `/Users/ryan/ai-env/projects/chirality-piping/execution/PKG-08_Reporting, Audit, and Reproducibility/1_Working/_run_records/`.
  - Produce a package audit summary under `/Users/ryan/ai-env/projects/chirality-piping/execution/PKG-08_Reporting, Audit, and Reproducibility/1_Working/_audit/PKG02_DOWNSTREAM_REVIEW_2026-05-16.md`.
  - Classify each deliverable as `PASS`, `WARNING`, `BLOCKER`, or `NOT_APPLICABLE` for PKG-02 compatibility.

PKG-02 Compatibility Checks:
  - DEL-02-01 canonical model/schema and physical source-of-truth role.
  - DEL-02-02 explicit unit metadata and no silent unit defaults.
  - DEL-02-03 mechanics/rule/human authority separation.
  - DEL-02-04 plugin/adapter no-bypass constraints where applicable.
  - DEL-02-05 persistence/hash/provenance/round-trip assumptions where applicable.

AllowedWriteTargets:
  - /Users/ryan/ai-env/projects/chirality-piping/execution/PKG-08_Reporting, Audit, and Reproducibility/1_Working/DEL-*/_REVIEW.md
  - /Users/ryan/ai-env/projects/chirality-piping/execution/PKG-08_Reporting, Audit, and Reproducibility/1_Working/DEL-*/Review_Findings.csv
  - /Users/ryan/ai-env/projects/chirality-piping/execution/PKG-08_Reporting, Audit, and Reproducibility/1_Working/_run_records/
  - /Users/ryan/ai-env/projects/chirality-piping/execution/PKG-08_Reporting, Audit, and Reproducibility/1_Working/_audit/

Exclusions:
  - No edits to `_STATUS.md`, `_CONTEXT.md`, `_DEPENDENCIES.md`, `Dependencies.csv`, DAG files, blocker queues, source code, schemas, fixtures, tests, or primary deliverable artifacts.
  - No lifecycle transition.
  - No candidate promotion.
  - No release, professional reliance, certification, sealing, approval, or code-compliance claims.
```

## PKG-09 Package Audit Brief

```markdown
PURPOSE: Package-scoped downstream audit against PKG-02 foundation contracts.
RequestedBy: WORKING_ITEMS
TaskProfile: PACKAGE_AUDIT
PackageID: PKG-09

ScopePath: /Users/ryan/ai-env/projects/chirality-piping/execution/PKG-09_Verification, Validation, and Quality Oracles/1_Working
AuditDeliverables:
  - DEL-09-01 - Mechanics benchmark suite
  - DEL-09-02 - Stress recovery benchmark suite
  - DEL-09-03 - Nonlinear support regression suite
  - DEL-09-04 - Validation manual skeleton
  - DEL-09-05 - Release quality gate checklist

Tasks:
  - For each listed deliverable, read `_CONTEXT.md`, `_STATUS.md`, `_REFERENCES.md`, `_DEPENDENCIES.md`, `Dependencies.csv`, `MEMORY.md`, and primary deliverable artifacts.
  - Produce or update deliverable-local `_REVIEW.md`.
  - Produce or update deliverable-local `Review_Findings.csv`.
  - Create one package-level TASK run record under `/Users/ryan/ai-env/projects/chirality-piping/execution/PKG-09_Verification, Validation, and Quality Oracles/1_Working/_run_records/`.
  - Produce a package audit summary under `/Users/ryan/ai-env/projects/chirality-piping/execution/PKG-09_Verification, Validation, and Quality Oracles/1_Working/_audit/PKG02_DOWNSTREAM_REVIEW_2026-05-16.md`.
  - Classify each deliverable as `PASS`, `WARNING`, `BLOCKER`, or `NOT_APPLICABLE` for PKG-02 compatibility.

PKG-02 Compatibility Checks:
  - DEL-02-01 canonical model/schema and physical source-of-truth role.
  - DEL-02-02 explicit unit metadata and no silent unit defaults.
  - DEL-02-03 mechanics/rule/human authority separation.
  - DEL-02-04 plugin/adapter no-bypass constraints where applicable.
  - DEL-02-05 persistence/hash/provenance/round-trip assumptions where applicable.

AllowedWriteTargets:
  - /Users/ryan/ai-env/projects/chirality-piping/execution/PKG-09_Verification, Validation, and Quality Oracles/1_Working/DEL-*/_REVIEW.md
  - /Users/ryan/ai-env/projects/chirality-piping/execution/PKG-09_Verification, Validation, and Quality Oracles/1_Working/DEL-*/Review_Findings.csv
  - /Users/ryan/ai-env/projects/chirality-piping/execution/PKG-09_Verification, Validation, and Quality Oracles/1_Working/_run_records/
  - /Users/ryan/ai-env/projects/chirality-piping/execution/PKG-09_Verification, Validation, and Quality Oracles/1_Working/_audit/

Exclusions:
  - No edits to `_STATUS.md`, `_CONTEXT.md`, `_DEPENDENCIES.md`, `Dependencies.csv`, DAG files, blocker queues, source code, schemas, fixtures, tests, or primary deliverable artifacts.
  - No lifecycle transition.
  - No candidate promotion.
  - No release, professional reliance, certification, sealing, approval, or code-compliance claims.
```

## PKG-10 Package Audit Brief

```markdown
PURPOSE: Package-scoped downstream audit against PKG-02 foundation contracts.
RequestedBy: WORKING_ITEMS
TaskProfile: PACKAGE_AUDIT
PackageID: PKG-10

ScopePath: /Users/ryan/ai-env/projects/chirality-piping/execution/PKG-10_Build, Packaging, API, and Interoperability/1_Working
AuditDeliverables:
  - DEL-10-01 - Public API and plugin boundary
  - DEL-10-02 - Import/export adapter framework
  - DEL-10-03 - Local FEA handoff data contract
  - DEL-10-05 - Headless CLI and structured I/O analysis runner

Tasks:
  - For each listed deliverable, read `_CONTEXT.md`, `_STATUS.md`, `_REFERENCES.md`, `_DEPENDENCIES.md`, `Dependencies.csv`, `MEMORY.md`, and primary deliverable artifacts.
  - Produce or update deliverable-local `_REVIEW.md`.
  - Produce or update deliverable-local `Review_Findings.csv`.
  - Create one package-level TASK run record under `/Users/ryan/ai-env/projects/chirality-piping/execution/PKG-10_Build, Packaging, API, and Interoperability/1_Working/_run_records/`.
  - Produce a package audit summary under `/Users/ryan/ai-env/projects/chirality-piping/execution/PKG-10_Build, Packaging, API, and Interoperability/1_Working/_audit/PKG02_DOWNSTREAM_REVIEW_2026-05-16.md`.
  - Classify each deliverable as `PASS`, `WARNING`, `BLOCKER`, or `NOT_APPLICABLE` for PKG-02 compatibility.

PKG-02 Compatibility Checks:
  - DEL-02-01 canonical model/schema and physical source-of-truth role.
  - DEL-02-02 explicit unit metadata and no silent unit defaults.
  - DEL-02-03 mechanics/rule/human authority separation.
  - DEL-02-04 plugin/adapter no-bypass constraints where applicable.
  - DEL-02-05 persistence/hash/provenance/round-trip assumptions where applicable.

AllowedWriteTargets:
  - /Users/ryan/ai-env/projects/chirality-piping/execution/PKG-10_Build, Packaging, API, and Interoperability/1_Working/DEL-*/_REVIEW.md
  - /Users/ryan/ai-env/projects/chirality-piping/execution/PKG-10_Build, Packaging, API, and Interoperability/1_Working/DEL-*/Review_Findings.csv
  - /Users/ryan/ai-env/projects/chirality-piping/execution/PKG-10_Build, Packaging, API, and Interoperability/1_Working/_run_records/
  - /Users/ryan/ai-env/projects/chirality-piping/execution/PKG-10_Build, Packaging, API, and Interoperability/1_Working/_audit/

Exclusions:
  - No edits to `_STATUS.md`, `_CONTEXT.md`, `_DEPENDENCIES.md`, `Dependencies.csv`, DAG files, blocker queues, source code, schemas, fixtures, tests, or primary deliverable artifacts.
  - No lifecycle transition.
  - No candidate promotion.
  - No release, professional reliance, certification, sealing, approval, or code-compliance claims.
```

## PKG-11 Package Audit Brief

```markdown
PURPOSE: Package-scoped downstream audit against PKG-02 foundation contracts.
RequestedBy: WORKING_ITEMS
TaskProfile: PACKAGE_AUDIT
PackageID: PKG-11

ScopePath: /Users/ryan/ai-env/projects/chirality-piping/execution/PKG-11_Documentation, Examples, and Education/1_Working
AuditDeliverables:
  - DEL-11-01 - User guide skeleton
  - DEL-11-02 - Developer guide for solver and rule packs
  - DEL-11-03 - Theory notes: classical to modern centerline analysis
  - DEL-11-04 - Invented educational example models

Tasks:
  - For each listed deliverable, read `_CONTEXT.md`, `_STATUS.md`, `_REFERENCES.md`, `_DEPENDENCIES.md`, `Dependencies.csv`, `MEMORY.md`, and primary deliverable artifacts.
  - Produce or update deliverable-local `_REVIEW.md`.
  - Produce or update deliverable-local `Review_Findings.csv`.
  - Create one package-level TASK run record under `/Users/ryan/ai-env/projects/chirality-piping/execution/PKG-11_Documentation, Examples, and Education/1_Working/_run_records/`.
  - Produce a package audit summary under `/Users/ryan/ai-env/projects/chirality-piping/execution/PKG-11_Documentation, Examples, and Education/1_Working/_audit/PKG02_DOWNSTREAM_REVIEW_2026-05-16.md`.
  - Classify each deliverable as `PASS`, `WARNING`, `BLOCKER`, or `NOT_APPLICABLE` for PKG-02 compatibility.

PKG-02 Compatibility Checks:
  - DEL-02-01 canonical model/schema and physical source-of-truth role.
  - DEL-02-02 explicit unit metadata and no silent unit defaults.
  - DEL-02-03 mechanics/rule/human authority separation.
  - DEL-02-04 plugin/adapter no-bypass constraints where applicable.
  - DEL-02-05 persistence/hash/provenance/round-trip assumptions where applicable.

AllowedWriteTargets:
  - /Users/ryan/ai-env/projects/chirality-piping/execution/PKG-11_Documentation, Examples, and Education/1_Working/DEL-*/_REVIEW.md
  - /Users/ryan/ai-env/projects/chirality-piping/execution/PKG-11_Documentation, Examples, and Education/1_Working/DEL-*/Review_Findings.csv
  - /Users/ryan/ai-env/projects/chirality-piping/execution/PKG-11_Documentation, Examples, and Education/1_Working/_run_records/
  - /Users/ryan/ai-env/projects/chirality-piping/execution/PKG-11_Documentation, Examples, and Education/1_Working/_audit/

Exclusions:
  - No edits to `_STATUS.md`, `_CONTEXT.md`, `_DEPENDENCIES.md`, `Dependencies.csv`, DAG files, blocker queues, source code, schemas, fixtures, tests, or primary deliverable artifacts.
  - No lifecycle transition.
  - No candidate promotion.
  - No release, professional reliance, certification, sealing, approval, or code-compliance claims.
```

## PKG-12 Package Audit Brief

```markdown
PURPOSE: Package-scoped downstream audit against PKG-02 foundation contracts.
RequestedBy: WORKING_ITEMS
TaskProfile: PACKAGE_AUDIT
PackageID: PKG-12

ScopePath: /Users/ryan/ai-env/projects/chirality-piping/execution/PKG-12_Security, Privacy, and Private Data Handling/1_Working
AuditDeliverables:
  - DEL-12-01 - Local-first storage and private data paths
  - DEL-12-02 - Private data redaction and export controls
  - DEL-12-03 - Telemetry off-by-default design
  - DEL-12-04 - Secret and private-library handling
  - DEL-12-05 - Security threat model

Tasks:
  - For each listed deliverable, read `_CONTEXT.md`, `_STATUS.md`, `_REFERENCES.md`, `_DEPENDENCIES.md`, `Dependencies.csv`, `MEMORY.md`, and primary deliverable artifacts.
  - Produce or update deliverable-local `_REVIEW.md`.
  - Produce or update deliverable-local `Review_Findings.csv`.
  - Create one package-level TASK run record under `/Users/ryan/ai-env/projects/chirality-piping/execution/PKG-12_Security, Privacy, and Private Data Handling/1_Working/_run_records/`.
  - Produce a package audit summary under `/Users/ryan/ai-env/projects/chirality-piping/execution/PKG-12_Security, Privacy, and Private Data Handling/1_Working/_audit/PKG02_DOWNSTREAM_REVIEW_2026-05-16.md`.
  - Classify each deliverable as `PASS`, `WARNING`, `BLOCKER`, or `NOT_APPLICABLE` for PKG-02 compatibility.

PKG-02 Compatibility Checks:
  - DEL-02-01 canonical model/schema and physical source-of-truth role.
  - DEL-02-02 explicit unit metadata and no silent unit defaults.
  - DEL-02-03 mechanics/rule/human authority separation.
  - DEL-02-04 plugin/adapter no-bypass constraints where applicable.
  - DEL-02-05 persistence/hash/provenance/round-trip assumptions where applicable.

AllowedWriteTargets:
  - /Users/ryan/ai-env/projects/chirality-piping/execution/PKG-12_Security, Privacy, and Private Data Handling/1_Working/DEL-*/_REVIEW.md
  - /Users/ryan/ai-env/projects/chirality-piping/execution/PKG-12_Security, Privacy, and Private Data Handling/1_Working/DEL-*/Review_Findings.csv
  - /Users/ryan/ai-env/projects/chirality-piping/execution/PKG-12_Security, Privacy, and Private Data Handling/1_Working/_run_records/
  - /Users/ryan/ai-env/projects/chirality-piping/execution/PKG-12_Security, Privacy, and Private Data Handling/1_Working/_audit/

Exclusions:
  - No edits to `_STATUS.md`, `_CONTEXT.md`, `_DEPENDENCIES.md`, `Dependencies.csv`, DAG files, blocker queues, source code, schemas, fixtures, tests, or primary deliverable artifacts.
  - No lifecycle transition.
  - No candidate promotion.
  - No release, professional reliance, certification, sealing, approval, or code-compliance claims.
```

## PKG-13 Package Audit Brief

```markdown
PURPOSE: Package-scoped downstream audit against PKG-02 foundation contracts.
RequestedBy: WORKING_ITEMS
TaskProfile: PACKAGE_AUDIT
PackageID: PKG-13

ScopePath: /Users/ryan/ai-env/projects/chirality-piping/execution/PKG-13_Physical Design Knowledge and Constraint Engine/1_Working
AuditDeliverables:
  - DEL-13-01 - Design knowledge schema and provenance model
  - DEL-13-02 - Constraint entity and provenance model
  - DEL-13-03 - Constraint validation engine
  - DEL-13-04 - Physical-to-analytical transformation contract

Tasks:
  - For each listed deliverable, read `_CONTEXT.md`, `_STATUS.md`, `_REFERENCES.md`, `_DEPENDENCIES.md`, `Dependencies.csv`, `MEMORY.md`, and primary deliverable artifacts.
  - Produce or update deliverable-local `_REVIEW.md`.
  - Produce or update deliverable-local `Review_Findings.csv`.
  - Create one package-level TASK run record under `/Users/ryan/ai-env/projects/chirality-piping/execution/PKG-13_Physical Design Knowledge and Constraint Engine/1_Working/_run_records/`.
  - Produce a package audit summary under `/Users/ryan/ai-env/projects/chirality-piping/execution/PKG-13_Physical Design Knowledge and Constraint Engine/1_Working/_audit/PKG02_DOWNSTREAM_REVIEW_2026-05-16.md`.
  - Classify each deliverable as `PASS`, `WARNING`, `BLOCKER`, or `NOT_APPLICABLE` for PKG-02 compatibility.

PKG-02 Compatibility Checks:
  - DEL-02-01 canonical model/schema and physical source-of-truth role.
  - DEL-02-02 explicit unit metadata and no silent unit defaults.
  - DEL-02-03 mechanics/rule/human authority separation.
  - DEL-02-04 plugin/adapter no-bypass constraints where applicable.
  - DEL-02-05 persistence/hash/provenance/round-trip assumptions where applicable.

AllowedWriteTargets:
  - /Users/ryan/ai-env/projects/chirality-piping/execution/PKG-13_Physical Design Knowledge and Constraint Engine/1_Working/DEL-*/_REVIEW.md
  - /Users/ryan/ai-env/projects/chirality-piping/execution/PKG-13_Physical Design Knowledge and Constraint Engine/1_Working/DEL-*/Review_Findings.csv
  - /Users/ryan/ai-env/projects/chirality-piping/execution/PKG-13_Physical Design Knowledge and Constraint Engine/1_Working/_run_records/
  - /Users/ryan/ai-env/projects/chirality-piping/execution/PKG-13_Physical Design Knowledge and Constraint Engine/1_Working/_audit/

Exclusions:
  - No edits to `_STATUS.md`, `_CONTEXT.md`, `_DEPENDENCIES.md`, `Dependencies.csv`, DAG files, blocker queues, source code, schemas, fixtures, tests, or primary deliverable artifacts.
  - No lifecycle transition.
  - No candidate promotion.
  - No release, professional reliance, certification, sealing, approval, or code-compliance claims.
```

## PKG-14 Package Audit Brief

```markdown
PURPOSE: Package-scoped downstream audit against PKG-02 foundation contracts.
RequestedBy: WORKING_ITEMS
TaskProfile: PACKAGE_AUDIT
PackageID: PKG-14

ScopePath: /Users/ryan/ai-env/projects/chirality-piping/execution/PKG-14_Model States, Analysis Runs, and Comparison/1_Working
AuditDeliverables:
  - DEL-14-01 - Immutable model state records
  - DEL-14-02 - Analysis run records
  - DEL-14-03 - Model-state comparison engine
  - DEL-14-04 - Analysis-run comparison engine
  - DEL-14-05 - Comparison mapping, tolerance, and export contracts

Tasks:
  - For each listed deliverable, read `_CONTEXT.md`, `_STATUS.md`, `_REFERENCES.md`, `_DEPENDENCIES.md`, `Dependencies.csv`, `MEMORY.md`, and primary deliverable artifacts.
  - Produce or update deliverable-local `_REVIEW.md`.
  - Produce or update deliverable-local `Review_Findings.csv`.
  - Create one package-level TASK run record under `/Users/ryan/ai-env/projects/chirality-piping/execution/PKG-14_Model States, Analysis Runs, and Comparison/1_Working/_run_records/`.
  - Produce a package audit summary under `/Users/ryan/ai-env/projects/chirality-piping/execution/PKG-14_Model States, Analysis Runs, and Comparison/1_Working/_audit/PKG02_DOWNSTREAM_REVIEW_2026-05-16.md`.
  - Classify each deliverable as `PASS`, `WARNING`, `BLOCKER`, or `NOT_APPLICABLE` for PKG-02 compatibility.

PKG-02 Compatibility Checks:
  - DEL-02-01 canonical model/schema and physical source-of-truth role.
  - DEL-02-02 explicit unit metadata and no silent unit defaults.
  - DEL-02-03 mechanics/rule/human authority separation.
  - DEL-02-04 plugin/adapter no-bypass constraints where applicable.
  - DEL-02-05 persistence/hash/provenance/round-trip assumptions where applicable.

AllowedWriteTargets:
  - /Users/ryan/ai-env/projects/chirality-piping/execution/PKG-14_Model States, Analysis Runs, and Comparison/1_Working/DEL-*/_REVIEW.md
  - /Users/ryan/ai-env/projects/chirality-piping/execution/PKG-14_Model States, Analysis Runs, and Comparison/1_Working/DEL-*/Review_Findings.csv
  - /Users/ryan/ai-env/projects/chirality-piping/execution/PKG-14_Model States, Analysis Runs, and Comparison/1_Working/_run_records/
  - /Users/ryan/ai-env/projects/chirality-piping/execution/PKG-14_Model States, Analysis Runs, and Comparison/1_Working/_audit/

Exclusions:
  - No edits to `_STATUS.md`, `_CONTEXT.md`, `_DEPENDENCIES.md`, `Dependencies.csv`, DAG files, blocker queues, source code, schemas, fixtures, tests, or primary deliverable artifacts.
  - No lifecycle transition.
  - No candidate promotion.
  - No release, professional reliance, certification, sealing, approval, or code-compliance claims.
```

## PKG-15 Package Audit Brief

```markdown
PURPOSE: Package-scoped downstream audit against PKG-02 foundation contracts.
RequestedBy: WORKING_ITEMS
TaskProfile: PACKAGE_AUDIT
PackageID: PKG-15

ScopePath: /Users/ryan/ai-env/projects/chirality-piping/execution/PKG-15_Handoff and External Prover Workflow/1_Working
AuditDeliverables:
  - DEL-15-01 - Canonical handoff package schema and manifest
  - DEL-15-02 - Target mapping and unsupported-behavior contract
  - DEL-15-03 - Downstream modeling export workflow
  - DEL-15-04 - External prover boundary metadata

Tasks:
  - For each listed deliverable, read `_CONTEXT.md`, `_STATUS.md`, `_REFERENCES.md`, `_DEPENDENCIES.md`, `Dependencies.csv`, `MEMORY.md`, and primary deliverable artifacts.
  - Produce or update deliverable-local `_REVIEW.md`.
  - Produce or update deliverable-local `Review_Findings.csv`.
  - Create one package-level TASK run record under `/Users/ryan/ai-env/projects/chirality-piping/execution/PKG-15_Handoff and External Prover Workflow/1_Working/_run_records/`.
  - Produce a package audit summary under `/Users/ryan/ai-env/projects/chirality-piping/execution/PKG-15_Handoff and External Prover Workflow/1_Working/_audit/PKG02_DOWNSTREAM_REVIEW_2026-05-16.md`.
  - Classify each deliverable as `PASS`, `WARNING`, `BLOCKER`, or `NOT_APPLICABLE` for PKG-02 compatibility.

PKG-02 Compatibility Checks:
  - DEL-02-01 canonical model/schema and physical source-of-truth role.
  - DEL-02-02 explicit unit metadata and no silent unit defaults.
  - DEL-02-03 mechanics/rule/human authority separation.
  - DEL-02-04 plugin/adapter no-bypass constraints where applicable.
  - DEL-02-05 persistence/hash/provenance/round-trip assumptions where applicable.

AllowedWriteTargets:
  - /Users/ryan/ai-env/projects/chirality-piping/execution/PKG-15_Handoff and External Prover Workflow/1_Working/DEL-*/_REVIEW.md
  - /Users/ryan/ai-env/projects/chirality-piping/execution/PKG-15_Handoff and External Prover Workflow/1_Working/DEL-*/Review_Findings.csv
  - /Users/ryan/ai-env/projects/chirality-piping/execution/PKG-15_Handoff and External Prover Workflow/1_Working/_run_records/
  - /Users/ryan/ai-env/projects/chirality-piping/execution/PKG-15_Handoff and External Prover Workflow/1_Working/_audit/

Exclusions:
  - No edits to `_STATUS.md`, `_CONTEXT.md`, `_DEPENDENCIES.md`, `Dependencies.csv`, DAG files, blocker queues, source code, schemas, fixtures, tests, or primary deliverable artifacts.
  - No lifecycle transition.
  - No candidate promotion.
  - No release, professional reliance, certification, sealing, approval, or code-compliance claims.
```

## PKG-16 Package Audit Brief

```markdown
PURPOSE: Package-scoped downstream audit against PKG-02 foundation contracts.
RequestedBy: WORKING_ITEMS
TaskProfile: PACKAGE_AUDIT
PackageID: PKG-16

ScopePath: /Users/ryan/ai-env/projects/chirality-piping/execution/PKG-16_Model Operation and Agent Proposal Framework/1_Working
AuditDeliverables:
  - DEL-16-01 - Structured model operation schema
  - DEL-16-02 - Operation validation and diff preview
  - DEL-16-03 - User acceptance and operation audit trail
  - DEL-16-04 - Agent rationale and professional-boundary controls

Tasks:
  - For each listed deliverable, read `_CONTEXT.md`, `_STATUS.md`, `_REFERENCES.md`, `_DEPENDENCIES.md`, `Dependencies.csv`, `MEMORY.md`, and primary deliverable artifacts.
  - Produce or update deliverable-local `_REVIEW.md`.
  - Produce or update deliverable-local `Review_Findings.csv`.
  - Create one package-level TASK run record under `/Users/ryan/ai-env/projects/chirality-piping/execution/PKG-16_Model Operation and Agent Proposal Framework/1_Working/_run_records/`.
  - Produce a package audit summary under `/Users/ryan/ai-env/projects/chirality-piping/execution/PKG-16_Model Operation and Agent Proposal Framework/1_Working/_audit/PKG02_DOWNSTREAM_REVIEW_2026-05-16.md`.
  - Classify each deliverable as `PASS`, `WARNING`, `BLOCKER`, or `NOT_APPLICABLE` for PKG-02 compatibility.

PKG-02 Compatibility Checks:
  - DEL-02-01 canonical model/schema and physical source-of-truth role.
  - DEL-02-02 explicit unit metadata and no silent unit defaults.
  - DEL-02-03 mechanics/rule/human authority separation.
  - DEL-02-04 plugin/adapter no-bypass constraints where applicable.
  - DEL-02-05 persistence/hash/provenance/round-trip assumptions where applicable.

AllowedWriteTargets:
  - /Users/ryan/ai-env/projects/chirality-piping/execution/PKG-16_Model Operation and Agent Proposal Framework/1_Working/DEL-*/_REVIEW.md
  - /Users/ryan/ai-env/projects/chirality-piping/execution/PKG-16_Model Operation and Agent Proposal Framework/1_Working/DEL-*/Review_Findings.csv
  - /Users/ryan/ai-env/projects/chirality-piping/execution/PKG-16_Model Operation and Agent Proposal Framework/1_Working/_run_records/
  - /Users/ryan/ai-env/projects/chirality-piping/execution/PKG-16_Model Operation and Agent Proposal Framework/1_Working/_audit/

Exclusions:
  - No edits to `_STATUS.md`, `_CONTEXT.md`, `_DEPENDENCIES.md`, `Dependencies.csv`, DAG files, blocker queues, source code, schemas, fixtures, tests, or primary deliverable artifacts.
  - No lifecycle transition.
  - No candidate promotion.
  - No release, professional reliance, certification, sealing, approval, or code-compliance claims.
```
