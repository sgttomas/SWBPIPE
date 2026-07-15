<!-- chirality-architecture-basis/v1 -->
# DEL-00-03 — Application service command-query-job model — Architecture Basis

Classification: `ARCHITECTURE_BASIS_REFERENCE`
Consolidates: `Datasheet.md`, `Specification.md`, `Procedure.md`, `Guidance.md` (four-document kit, consolidated 2026-07-15 per piping decision D-43; source bytes hash-bound in the package `CONSOLIDATION_MANIFEST.md` and preserved in git history)
Authority basis: `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.9, architecture-basis row AB-00-03
Dependency direction: one-way — project packages depend on this member; this member depends on no package deliverable (HUMAN-STEER-PKG00-EXCLUSION-001)

## Purpose

Defines the command, query, background job, cancellation, progress, and transaction model connecting user-facing workflows (GUI, headless runner) to the domain core, solver, storage, and reporting layers. This member is architecture-basis context: it constrains service contracts and flow rules across the package universe (PKG-01 through PKG-17); it does not itself implement commands, queues, solvers, storage, CLI handlers, or GUI actions. Scope items SOW-058 and SOW-062; objective OBJ-013; type API_CONTRACT.

Per the D-41 R5 T7 PDU-054 current declaration, the command-query-job contract and application-service seams are implemented and evidenced by their current schemas, modules, and tests; broader application integration remains bounded by the recorded residuals.

## Normative requirements

| ID | Requirement |
|---|---|
| REQ-03-01 | Separate mutating commands from read-only queries and long-running jobs. |
| REQ-03-02 | Require every command and job result to return a diagnostics/result envelope rather than raw success claims. |
| REQ-03-03 | Define transaction boundaries for model edits, solve runs, persistence operations, report generation, and adapter calls. |
| REQ-03-04 | Require cancellation, progress, and reproducibility metadata for background solve/report/export jobs. |
| REQ-03-05 | Preserve the distinction between mechanics solved, user-rule checked, and human-approved states. |

Cross-cutting invariants carried from the kit: `OPS-K-IP-1` (no protected standards/code text, tables, formulas, or proprietary data in public artifacts), `OPS-K-DATA-2` (missing solve/rule-check values remain explicit findings, never silent defaults), `OPS-K-AUTH-1` (no certification, sealing, approval, or code-compliance claims for reliance), `OPS-K-MECH-1` (global analysis stays a 3D centerline/frame model; local FEA is a handoff path), `OPS-K-AGENT-1` (unknown facts become TBD), `OPS-K-AGENT-3` (Type 2 execution stays within sealed scope).

Guardrails: prefer explicit contracts over package-local assumptions; treat diagnostics, provenance, units, and data-boundary checks as cross-cutting obligations; software and agents must not claim code compliance or professional approval.

## Resolved decisions (former TBD and human-ruling queue)

| Former open item | Resolution | Authority |
|---|---|---|
| Command/query/job vocabulary and result-envelope boundaries | Confirmed as dispatch constraints; schema-first command/query/job/result envelopes (JSON Schema 2020-12) selected as the API baseline by SCA-001. | `docs/_Registers/ScopeLedger.csv` rows SOW-058, SOW-062; DEC-010 in `execution/_Decomposition/SOFTWARE_DECOMP.md` §12 |
| Concrete async/job execution mechanism — headless runner CLI and process policy | Ruled by D-33: stable `openpipestress-runner` CLI with verbs `solve`, `validate-input`, `export-results`, `run-benchmark`, `run-regression`; schema-first JSON input (stdin or named file), structured JSON on stdout; single foreground local process, no daemon, network, telemetry, or hidden mutation; exit codes 0/1/2. | `execution/_Coordination/_DECISIONS/D-33_headless_runner_cli_process_policy.md`; DEC-065 in `execution/_Decomposition/SOFTWARE_DECOMP.md` §12 (2026-07-05) |
| Transaction persistence mechanism | Local SQLite project store per SCA-003 with all mutation routed through application services (no direct plugin/adapter SQL); explicit-operation evidence guarantees with no silent destructive rewrites per D-08. | DEC-017, DEC-019 in `execution/_Decomposition/SOFTWARE_DECOMP.md` §12; AB-00-04 |

## Realized artifacts

The anticipated `docs/architecture/application_services.md` was never created. The contract is realized in practice by the implemented application-service layer:

| Artifact | Role | Ownership |
|---|---|---|
| `apps/desktop/src/services/projectService.ts` | Desktop application-service command/query seam | Implementation packages (not DEL-00-03) |
| `core/gui/*/engine.py` | GUI-domain service engines (accessibility, design_workspace, editors, model_tree, results_viewer, solve_execution, warnings, viewport_editor) | Implementation packages (not DEL-00-03) |
| `schemas/operation_outcome.schema.json` | Diagnostics/result-envelope contract (REQ-03-02) | Schema surface owned by schema deliverables |
| `schemas/analysis_run.schema.json` | Solve-run reproducibility/progress metadata contract (REQ-03-04) | Schema surface owned by schema deliverables |

## Open holds and routed questions

- Concrete service interface language details and the cancellation token API shape remain implementation-level detail, delegated per the ScopeLedger SOW-058 note ("concrete interface language details remain implementation-level TBD"); the implemented seams above are the current evidence, with no dedicated architecture ruling on an abstract token API.
- Broader application integration residuals are those recorded in the deliverable-local `_STATUS.md ## Remaining` (currently empty) and the D-41 residual records.

## Currency and provenance

Consolidated 2026-07-15 per piping decision D-43 from the four-document kit (`Datasheet.md`, `Specification.md`, `Procedure.md`, `Guidance.md`) as reconciled by D-41 R5 T7 (PDU-054/PDU-055 currentness declarations, 2026-07-12). Prior wording, including superseded setup-era framing and the revision 0.7 authority pins, is preserved in git history. `MEMORY.md` is retained unchanged as the dated deliverable memory. This document is reference context, not a production contract, and carries no lifecycle, review, release, professional-reliance, or code-compliance claim.
