# CFB-27 — Model tree and viewer: rule-check negative tests, IP-boundary warning tests, deferred categories

**Candidate brief (H2). Not executed.** Area: Desktop GUI (DEL-07-01, DEL-07-02, DEL-07-04). Run HELP-HUMAN-PIPING-20260921-RECONCILIATION, R3 integration, TASK H2. Execution, if the owner selects it, goes through an owner-steered production brief and the chirality-change PR path, followed by re-verification in a later concordance (ledger rows are not edited in place). Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).

## Scope

Add rule-check-required negative UI tests in the model tree (solve-required gaps are covered); add invented-fixture tests that activate IP_BOUNDARY_WARNING on export and report-preview paths; record which viewer categories are deferred and to which slice.

## Affected claims

5 claim rows on 3 deliverable(s): DEL-07-01, DEL-07-02, DEL-07-04.

Classes (portion in this brief / class total): T6-C01 5/180 (Authority NONE).

Reproducing filter: `CODE_FIX_ROWS.csv` where `CFB == "CFB-27"`. Each key below is in `R3/CLASS_ASSIGNMENTS.csv`, `R3/TASKS/T8_ROWS.csv` or `R3/TASKS/T12_UNREACHED.csv` with route CODE_FIX_CANDIDATE, except class T7-C05 and NOT_DIVERGENT rows, which enter through T8 only.

| Key | Class | Authority | BlockedOnPacket | T8/T12 view | Remaining work (effective; OC = OtherCorrections) |
|---|---|---|---|---|---|
| `DEL-07-01:SOW#CLM-011/DEL-07-01-REQ-02` | T6-C01 | NONE | — | — | Record which categories are deferred and to which result/component slice. |
| `DEL-07-02:SOW#CLM-018.r06` | T6-C01 | NONE | — | — | (none recorded; see Notes in ledger) |
| `DEL-07-02:SOW#CLM-025.r04` | T6-C01 | NONE | — | — | (none recorded; see Notes in ledger) |
| `DEL-07-04:SOW#CLM-013` | T6-C01 | NONE | — | — | Add explicit invented-fixture tests that activate IP_BOUNDARY_WARNING on export and report-preview paths (and public contribution if such a flow exists), or narrow the v… |
| `DEL-07-04:SOW#CLM-022` | T6-C01 | NONE | — | — | Add explicit invented-fixture tests that activate IP_BOUNDARY_WARNING on export and report-preview paths (and public contribution if such a flow exists), or narrow the v… |

## Evidence

Sealed ledgers (reliability: sealed R2 ledger rows with effective values from adopted resolutions; verified where the wave verifier sampled them, otherwise worker reading):

- DEL-07-01: `projects/chirality-piping/execution/_Reconciliation/DeliverableConcordance/RECON_2026-09-21_WHOLE_CORPUS/WAVES/W1/PKG-07/DEL-07-01/DEL-07-01_forward.csv`; ImplementationEvidence cited: `apps/desktop/src/features/viewport/PipeViewport.tsx::instancedComponentMeshes`.
- DEL-07-02: `projects/chirality-piping/execution/_Reconciliation/DeliverableConcordance/RECON_2026-09-21_WHOLE_CORPUS/WAVES/W1/PKG-07/DEL-07-02/DEL-07-02_forward.csv`; ImplementationEvidence cited: `apps/desktop/src/features/model-tree/PropertyInspector.tsx::requiredFlagsForSelection`.
- DEL-07-04: `projects/chirality-piping/execution/_Reconciliation/DeliverableConcordance/RECON_2026-09-21_WHOLE_CORPUS/WAVES/W1/PKG-07/DEL-07-04/DEL-07-04_forward.csv`; ImplementationEvidence cited: `core/gui/warnings/engine.py`, `core/gui/warnings/__init__.py`, `core/gui/pkg02_boundary.py`, `apps/desktop/src/features/missing-data/MissingDataBlockingPanel.tsx`, `apps/desktop/src/features/redaction-controls/redactionExportControls.ts` ….

Freeze line citations in Scope were re-read at the freeze (`00115c719`) by H2 as code reading only; no build or test was run. Classification sources: `R3/TASKS/T4A_CLASSES.md`, `T6_CLASSES.md`, `T7_CLASSES.md`, `T8_CLUSTERS.md`, `T12_UNREACHED.md` (proposals, not accepted results).

## Acceptance checks

- A model-tree test fails if a missing rule-check-required value is not shown as a finding.
- An invented-fixture test triggers IP_BOUNDARY_WARNING on export and report preview.
- The affected ledger rows are re-verified in a later concordance; no ALIGNED status is claimed from this brief.

## Protected-content status

No protected subject: no affected row is at INVARIANT tier or carries an IP_DATA, CLAIMS or SECURITY layer. 
This brief quotes no protected, private or third-party content. Execution uses invented or synthetic fixtures only and introduces no standards text, tables or equation sources (DEC-043).

## BlockedOnPacket

None. No affected row's class or T8/T12 reading needs an owner or review decision.

## Dependencies

None.

