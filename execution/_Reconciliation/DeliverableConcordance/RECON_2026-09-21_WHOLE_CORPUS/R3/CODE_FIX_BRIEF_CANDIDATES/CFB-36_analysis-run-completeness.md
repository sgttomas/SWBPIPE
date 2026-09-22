# CFB-36 — Analysis-run records: producer dimensions, diagnostic breadth, versioned result contract

**Candidate brief (H2). Not executed.** Area: Analysis runs and stress results (DEL-14-02, DEL-05-03). Run HELP-HUMAN-PIPING-20260921-RECONCILIATION, R3 integration, TASK H2. Execution, if the owner selects it, goes through an owner-steered production brief and the chirality-change PR path, followed by re-verification in a later concordance (ledger rows are not edited in place). Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).

## Scope

Obtain producer-supplied physical dimensions through the governed result-envelope change and extend negative unit tests to every unit-bearing referenced value; complete producer-side solve-required and rule-check-required diagnostic breadth and test it through the record builder; versioned public result-contract (raw public result DTO compatibility) work.

## Affected claims

5 claim rows on 2 deliverable(s): DEL-05-03, DEL-14-02.

Classes (portion in this brief / class total): NOT_DIVERGENT 1/— (Authority NONE); T6-C02 4/116 (Authority NONE).

Reproducing filter: `CODE_FIX_ROWS.csv` where `CFB == "CFB-36"`. Each key below is in `R3/CLASS_ASSIGNMENTS.csv`, `R3/TASKS/T8_ROWS.csv` or `R3/TASKS/T12_UNREACHED.csv` with route CODE_FIX_CANDIDATE, except class T7-C05 and NOT_DIVERGENT rows, which enter through T8 only.

| Key | Class | Authority | BlockedOnPacket | T8/T12 view | Remaining work (effective; OC = OtherCorrections) |
|---|---|---|---|---|---|
| `DEL-05-03:STATUS#remaining/R02` | T6-C02 | NONE | — | — | Versioned public result-contract (raw public result DTO compatibility) work. |
| `DEL-14-02:SOW#CLM-011.r05` | T6-C02 | NONE | — | — | Obtain producer-supplied physical dimensions through the governed result-envelope change and extend negative unit tests to every unit-bearing referenced value |
| `DEL-14-02:SOW#CLM-011.r06` | T6-C02 | NONE | — | — | Complete producer-side solve-required and rule-check-required diagnostic breadth and test it through the record builder |
| `DEL-14-02:SOW#CLM-019` | T6-C02 | NONE | — | — | Obtain producer-supplied physical dimensions and complete unit diagnostics for every unit-bearing referenced value |
| `DEL-14-02:SOW#CLM-024` | NOT_DIVERGENT | NONE | C7 | T8 F1_ON_CONTEXT: CODE_FIX_CANDIDATE; DISAGREES (T8 CODE_FIX_CANDIDATE / class NOT_DIVERGENT) | (none recorded; see Notes in ledger) |

## Evidence

Sealed ledgers (reliability: sealed R2 ledger rows with effective values from adopted resolutions; verified where the wave verifier sampled them, otherwise worker reading):

- DEL-05-03: `projects/chirality-piping/execution/_Reconciliation/DeliverableConcordance/RECON_2026-09-21_WHOLE_CORPUS/WAVES/W3/PKG-05/DEL-05-03/DEL-05-03_forward.csv`; ImplementationEvidence cited: `core/product_physics/src/lib.rs`.
- DEL-14-02: `projects/chirality-piping/execution/_Reconciliation/DeliverableConcordance/RECON_2026-09-21_WHOLE_CORPUS/WAVES/W3/PKG-14/DEL-14-02/DEL-14-02_forward.csv`; ImplementationEvidence cited: `schemas/analysis_run.v0.2.schema.json`, `apps/desktop/src/services/analysisRunCompatibility.ts`, `core/analysis_runs/compatibility.py`, `core/analysis_runs/records.py`, `apps/desktop/src/services/previewService.ts#L165`.

Freeze line citations in Scope were re-read at the freeze (`00115c719`) by H2 as code reading only; no build or test was run. Classification sources: `R3/TASKS/T4A_CLASSES.md`, `T6_CLASSES.md`, `T7_CLASSES.md`, `T8_CLUSTERS.md`, `T12_UNREACHED.md` (proposals, not accepted results).

## Acceptance checks

- Negative unit tests cover every unit-bearing referenced value in the record.
- Record-builder tests cover each solve-required and rule-check-required diagnostic.
- A versioned result-DTO compatibility test exists.
- The affected ledger rows are re-verified in a later concordance; no ALIGNED status is claimed from this brief.

## Protected-content status

No protected subject: no affected row is at INVARIANT tier or carries an IP_DATA, CLAIMS or SECURITY layer. 
This brief quotes no protected, private or third-party content. Execution uses invented or synthetic fixtures only and introduces no standards text, tables or equation sources (DEC-043).

## BlockedOnPacket

1 of 5 claim rows carry a block: C7 (1). `H3[<class>]` names the H3 register item for that class (review before repair). Unblocked rows may proceed separately once selected.

## Notes and open views

- DEL-14-02 CLM-024 is not divergent in the effective values; T8 K4 reads it as PARTIAL_SLICE joining FG-DEL-14-02-02 (C7 confirmation).
- The result-envelope home is a held selection (B12 D7, context).

## Dependencies

C7, B12 (D7, context), CFB-17.

