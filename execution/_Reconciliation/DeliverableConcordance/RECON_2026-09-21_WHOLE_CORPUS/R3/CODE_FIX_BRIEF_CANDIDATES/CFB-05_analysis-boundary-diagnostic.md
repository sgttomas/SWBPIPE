# CFB-05 — Analysis-boundary Diagnostic code and acceptance-reference staleness

**Candidate brief (H2). Not executed.** Area: Analysis boundary contract (DEL-02-03). Run HELP-HUMAN-PIPING-20260921-RECONCILIATION, R3 integration, TASK H2. Execution, if the owner selects it, goes through an owner-steered production brief and the chirality-change PR path, followed by re-verification in a later concordance (ledger rows are not edited in place). Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).

## Scope

Add a machine-readable diagnostic code to the boundary Diagnostic (AB-00-06); specify how a human acceptance reference is invalidated when its bound hashes change; add evidence for V11 (no unvalidated status write path) and reconcile V05/V06 with the layout settled in code.

## Affected claims

3 claim rows on 1 deliverable(s): DEL-02-03.

Classes (portion in this brief / class total): T6-C01 1/180 (Authority NONE); T6-C02 1/116 (Authority NONE); T6-C03 1/34 (Authority REVIEW).

Reproducing filter: `CODE_FIX_ROWS.csv` where `CFB == "CFB-05"`. Each key below is in `R3/CLASS_ASSIGNMENTS.csv`, `R3/TASKS/T8_ROWS.csv` or `R3/TASKS/T12_UNREACHED.csv` with route CODE_FIX_CANDIDATE, except class T7-C05 and NOT_DIVERGENT rows, which enter through T8 only.

| Key | Class | Authority | BlockedOnPacket | T8/T12 view | Remaining work (effective; OC = OtherCorrections) |
|---|---|---|---|---|---|
| `DEL-02-03:SOW#CLM-003` | T6-C03 | REVIEW | H3[T6-C03];B12 | — | Add a machine-readable diagnostic code to the boundary Diagnostic, and specify how a human acceptance reference is invalidated when its bound hashes change. |
| `DEL-02-03:SOW#CLM-011` | T6-C02 | NONE | B12 | — | Add a diagnostic code to the boundary Diagnostic (AB-00-06), and decide the stale-state representation for human acceptance references. |
| `DEL-02-03:SOW#CLM-013` | T6-C01 | NONE | A3 | — | Add evidence for V11 (no unvalidated status write path) and reconcile V05/V06 with the settled layout after the owner decides FG-DEL-02-03-01. |

## Evidence

Sealed ledgers (reliability: sealed R2 ledger rows with effective values from adopted resolutions; verified where the wave verifier sampled them, otherwise worker reading):

- DEL-02-03: `projects/chirality-piping/execution/_Reconciliation/DeliverableConcordance/RECON_2026-09-21_WHOLE_CORPUS/WAVES/W2/PKG-02/DEL-02-03/DEL-02-03_forward.csv`; ImplementationEvidence cited: `schemas/analysis_boundary.schema.yaml`, `schemas/project_persistence.schema.yaml`, `apps/desktop/src/services/ruleCheckService.ts`, `core/solver/diagnostics/src/lib.rs`, `fixtures/analysis_boundary/invented_mechanics_solved_rule_inputs_incomplete.json` ….

Freeze line citations in Scope were re-read at the freeze (`00115c719`) by H2 as code reading only; no build or test was run. Classification sources: `R3/TASKS/T4A_CLASSES.md`, `T6_CLASSES.md`, `T7_CLASSES.md`, `T8_CLUSTERS.md`, `T12_UNREACHED.md` (proposals, not accepted results).

## Acceptance checks

- The boundary schema carries a diagnostic code field, with a schema test.
- A test demonstrates V11 (no status write bypasses validation).
- The stale-acceptance representation is specified only after the acceptance workflow is selected.
- The affected ledger rows are re-verified in a later concordance; no ALIGNED status is claimed from this brief.

## Protected-content status

Protected subject: 1 of 3 rows are at INVARIANT tier or carry an IP_DATA, CLAIMS or SECURITY layer (row layers: BASELINE, CLAIMS). An independent review of any repair is required before reliance. 
This brief quotes no protected, private or third-party content. Execution uses invented or synthetic fixtures only and introduces no standards text, tables or equation sources (DEC-043).

## BlockedOnPacket

3 of 3 claim rows carry a block: A3 (1); B12 (2); H3[T6-C03] (1). `H3[<class>]` names the H3 register item for that class (review before repair; mapped in `H3_TOKEN_MAP.csv`). All rows are blocked.

## Notes and open views

- The acceptance-invalidation part depends on OI-007 selection (B12); V05/V06 depend on the CP-10 layout hold (A3).

## Dependencies

B12 (OI-007 selection), A3 (FG-DEL-02-03-01 layout).

