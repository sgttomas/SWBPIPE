# CFB-24 — Benchmark quantities bound to the DEC-018 project unit basis

**Candidate brief (H2). Not executed.** Area: Benchmarks (DEL-09-01, DEL-09-02). Run HELP-HUMAN-PIPING-20260921-RECONCILIATION, R3 integration, TASK H2. Execution, if the owner selects it, goes through an owner-steered production brief and the chirality-change PR path, followed by re-verification in a later concordance (ledger rows are not edited in place). Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).

## Scope

Bind benchmark inputs, outputs and comparisons to the DEC-018 project unit basis (`core/units`) and check comparisons dimensionally, beyond fixture-local labels; rerun the cases through it.

## Affected claims

6 claim rows on 2 deliverable(s): DEL-09-01, DEL-09-02.

Classes (portion in this brief / class total): T6-C02 6/116 (Authority NONE).

Reproducing filter: `CODE_FIX_ROWS.csv` where `CFB == "CFB-24"`. Each key below is in `R3/CLASS_ASSIGNMENTS.csv`, `R3/TASKS/T8_ROWS.csv` or `R3/TASKS/T12_UNREACHED.csv` with route CODE_FIX_CANDIDATE, except class T7-C05 and NOT_DIVERGENT rows, which enter through T8 only.

| Key | Class | Authority | BlockedOnPacket | T8/T12 view | Remaining work (effective; OC = OtherCorrections) |
|---|---|---|---|---|---|
| `DEL-09-01:SOW#CLM-013/DEL-09-01-RQ-004` | T6-C02 | NONE | — | — | Bind benchmark inputs/outputs to the DEC-018 project unit basis (core/units) and rerun the cases through it. |
| `DEL-09-01:SOW#CLM-015.r04` | T6-C02 | NONE | — | — | As RQ-004. |
| `DEL-09-01:SOW#CLM-024.r04` | T6-C02 | NONE | — | — | As RQ-004. |
| `DEL-09-02:SOW#CLM-004.r07` | T6-C02 | NONE | — | — | Bind benchmark quantities to the DEC-018 project unit basis and check comparisons dimensionally, beyond the fixture-local labels. |
| `DEL-09-02:SOW#CLM-012/DEL-09-02-RQ-005` | T6-C02 | NONE | — | — | As CLM-004.r07. |
| `DEL-09-02:SOW#CLM-014.r03` | T6-C02 | NONE | — | — | As CLM-004.r07. |

## Evidence

Sealed ledgers (reliability: sealed R2 ledger rows with effective values from adopted resolutions; verified where the wave verifier sampled them, otherwise worker reading):

- DEL-09-01: `projects/chirality-piping/execution/_Reconciliation/DeliverableConcordance/RECON_2026-09-21_WHOLE_CORPUS/WAVES/W3/PKG-09/DEL-09-01/DEL-09-01_forward.csv`; ImplementationEvidence cited: `validation/benchmarks/mechanics/src/lib.rs::FIXTURE_UNIT_BASIS`, `core/units/README.md`.
- DEL-09-02: `projects/chirality-piping/execution/_Reconciliation/DeliverableConcordance/RECON_2026-09-21_WHOLE_CORPUS/WAVES/W3/PKG-09/DEL-09-02/DEL-09-02_forward.csv`; ImplementationEvidence cited: `validation/benchmarks/stress/src/lib.rs::STRESS_FIXTURE_UNIT_BASIS`, `validation/benchmarks/stress/src/lib.rs::governed_complete_stress_result_envelope`, `core/units/README.md`.

Freeze line citations in Scope were re-read at the freeze (`00115c719`) by H2 as code reading only; no build or test was run. Classification sources: `R3/TASKS/T4A_CLASSES.md`, `T6_CLASSES.md`, `T7_CLASSES.md`, `T8_CLUSTERS.md`, `T12_UNREACHED.md` (proposals, not accepted results).

## Acceptance checks

- A negative test shows a dimension-mismatched benchmark comparison is rejected.
- Benchmark cases run through the catalog conversion path.
- The affected ledger rows are re-verified in a later concordance; no ALIGNED status is claimed from this brief.

## Protected-content status

No protected subject: no affected row is at INVARIANT tier or carries an IP_DATA, CLAIMS or SECURITY layer. 
This brief quotes no protected, private or third-party content. Execution uses invented or synthetic fixtures only and introduces no standards text, tables or equation sources (DEC-043).

## BlockedOnPacket

None. No affected row's class or T8 reading needs an owner or review decision.

## Notes and open views

- Running the cases is part of execution under a later production brief; this candidate runs nothing.

## Dependencies

CFB-04.

