# Datasheet: DEL-07-07 Solve execution UX: progress, cancellation, and diagnostics

## Identification

| Field | Value |
|---|---|
| Deliverable ID | DEL-07-07 |
| Deliverable name | Solve execution UX: progress, cancellation, and diagnostics |
| Package ID | PKG-07 |
| Package name | Graphical User Interface and Engineering Workflow |
| Type | UX_UI_SLICE |
| Scope item | SOW-055 |
| Objectives | OBJ-006, OBJ-007 |
| Context envelope | M |
| Setup state | SEMANTIC_READY candidate after setup gates pass |

## Attributes

| Attribute | Setup value |
|---|---|
| UX surface | Solve run panel, progress/cancel controls, diagnostic presentation |
| Execution mode | Background solve execution through application-service command/job boundaries |
| Progress basis | Report only progress states, phases, or measures supplied by the job contract; exact progress semantics remain TBD |
| Cancellation basis | User cancellation is routed through the application-service job cancellation contract; direct GUI bypass of solver state is out of scope |
| Diagnostics basis | Diagnostic/result-envelope contract from PKG-00, including code, class, severity, source, affected object, message, remediation, and provenance where available |
| Result review basis | Mechanics solve status, missing data, assumptions, diagnostics, and reproducibility signals remain visible for professional review |
| Implementation status | No GUI source code, tests, schemas, job code, or solver code is implemented by this setup run |

## Conditions

| Condition | Treatment |
|---|---|
| Missing solve-required inputs | Presented as `SOLVE_BLOCKING` diagnostics or equivalent envelope entries; never silently defaulted |
| Missing rule-check inputs | Presented separately from solve readiness as `RULE_CHECK_BLOCKING` or equivalent diagnostics |
| Nonlinear or numerical uncertainty | Presented as solver diagnostics and warning classes rather than hidden state |
| Protected standards/code data | Not introduced; any code-specific or proprietary data remains user-supplied/private |
| Professional reliance | Human review remains required; the GUI shall not claim certification, approval, sealing, or code compliance |
| Reproducibility | Run records should preserve model/version/hash/checksum inputs when provided by upstream contracts; exact fields remain TBD |

## Construction

This setup constructs only deliverable-local planning artifacts:

- `Datasheet.md`
- `Specification.md`
- `Guidance.md`
- `Procedure.md`
- `_SEMANTIC.md`
- `_SEMANTIC_LENSING.md`
- `Dependencies.csv`
- `_DEPENDENCIES.md`
- `_run_records/*`
- `_STATUS.md`

Future implementation work must remain bounded by the GUI/application-service seam defined by PKG-00. It should consume command/job/progress/cancellation and diagnostics/result-envelope contracts rather than inventing GUI-owned solver semantics.

## References

| Reference | Use |
|---|---|
| `INIT.md` | Bootstrap boundaries: protected data, mechanics vs rule check, human authority |
| `AGENTS.md` | Type 2 sealed deliverable execution and write-scope discipline |
| `docs/CONTRACT.md` | Invariants for data, units, IP, privacy, authority, and agents |
| `docs/DIRECTIVE.md` | Product stop rules and no-silent-default principles |
| `docs/SPEC.md` | GUI warnings, result/report expectations, and layer boundaries |
| `docs/TYPES.md` | Analysis-status and professional-boundary vocabulary |
| `docs/_Decomposition/SOFTWARE_DECOMP.md` | DEL-07-07 package, scope, objectives, and architecture-basis rows |
| `docs/_Registers/Deliverables.csv` | Deliverable row for DEL-07-07 |
| `docs/_Registers/ScopeLedger.csv` | SOW-055 row |
| `execution/PKG-00_Software Architecture Runway/1_Working/DEL-00-03_Application service command-query-job model/Specification.md` | Command, job, cancellation, progress, and result-envelope boundary |
| `execution/PKG-00_Software Architecture Runway/1_Working/DEL-00-05_GUI state and interaction architecture/Specification.md` | Durable/transient GUI state and job-progress state separation |
| `execution/PKG-00_Software Architecture Runway/1_Working/DEL-00-06_Diagnostics, warning, and result-envelope contract/Specification.md` | Diagnostics fields, warning classes, and no-compliance-claim boundary |
