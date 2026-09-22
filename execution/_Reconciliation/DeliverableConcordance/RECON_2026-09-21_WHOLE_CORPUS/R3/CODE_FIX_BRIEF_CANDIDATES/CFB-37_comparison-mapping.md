# CFB-37 — Comparison mapping: non-element object refs and catalog-based normalization

**Candidate brief (H2). Not executed.** Area: Comparison (DEL-14-04). Run HELP-HUMAN-PIPING-20260921-RECONCILIATION, R3 integration, TASK H2. Execution, if the owner selects it, goes through an owner-steered production brief and the chirality-change PR path, followed by re-verification in a later concordance (ledger rows are not edited in place). Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).

## Scope

Add focused comparison tests whose mapped results carry node, support and terminal object refs (not only PipeElement), or narrow the claim; bind normalization to the DEC-018 unit catalog instead of caller-supplied factors only, or record the caller-reviewed posture; document the delta output structure, ordering basis and limitations.

## Affected claims

4 claim rows on 1 deliverable(s): DEL-14-04.

Classes (portion in this brief / class total): T6-C01 4/180 (Authority NONE).

Reproducing filter: `CODE_FIX_ROWS.csv` where `CFB == "CFB-37"`. Each key below is in `R3/CLASS_ASSIGNMENTS.csv`, `R3/TASKS/T8_ROWS.csv` or `R3/TASKS/T12_UNREACHED.csv` with route CODE_FIX_CANDIDATE, except class T7-C05 and NOT_DIVERGENT rows, which enter through T8 only.

| Key | Class | Authority | BlockedOnPacket | T8/T12 view | Remaining work (effective; OC = OtherCorrections) |
|---|---|---|---|---|---|
| `DEL-14-04:SOW#CLM-004.r05` | T6-C01 | NONE | — | — | Add focused comparison tests whose mapped results carry node, support and terminal object refs (not only PipeElement), or narrow the claim to result families. |
| `DEL-14-04:SOW#CLM-006.r03` | T6-C01 | NONE | — | — | Bind normalization to the accepted DEC-018 unit catalog / units contract instead of caller-supplied factors only, or record that caller-reviewed factors are the chosen d… |
| `DEL-14-04:SOW#CLM-013/R-14-04-004` | T6-C01 | NONE | — | — | Add focused comparison tests whose mapped results carry node, support and terminal object refs (not only PipeElement), or narrow the claim to result families. |
| `DEL-14-04:SOW#CLM-016` | T6-C01 | NONE | — | — | Write deliverable documentation of the delta output structure, ordering basis and limitations (or cite an accepted schema once PDU-011 closes). |

## Evidence

Sealed ledgers (reliability: sealed R2 ledger rows with effective values from adopted resolutions; verified where the wave verifier sampled them, otherwise worker reading):

- DEL-14-04: `projects/chirality-piping/execution/_Reconciliation/DeliverableConcordance/RECON_2026-09-21_WHOLE_CORPUS/WAVES/W3/PKG-14/DEL-14-04/DEL-14-04_forward.csv`; ImplementationEvidence cited: `core/comparison/analysis_run/engine.py`, `projects/chirality-piping/execution/PKG-14_Model States, Analysis Runs, and Comparison/1_Working/DEL-14-04_Analysis-run comparison engine/MEMORY.md`.

Freeze line citations in Scope were re-read at the freeze (`00115c719`) by H2 as code reading only; no build or test was run. Classification sources: `R3/TASKS/T4A_CLASSES.md`, `T6_CLASSES.md`, `T7_CLASSES.md`, `T8_CLUSTERS.md`, `T12_UNREACHED.md` (proposals, not accepted results).

## Acceptance checks

- Tests map node, support and terminal results.
- A normalization test uses the catalog conversion path.
- The affected ledger rows are re-verified in a later concordance; no ALIGNED status is claimed from this brief.

## Protected-content status

No protected subject: no affected row is at INVARIANT tier or carries an IP_DATA, CLAIMS or SECURITY layer. 
This brief quotes no protected, private or third-party content. Execution uses invented or synthetic fixtures only and introduces no standards text, tables or equation sources (DEC-043).

## BlockedOnPacket

None. No affected row's class or T8 reading needs an owner or review decision.

## Notes and open views

- Tolerance suitability is an engineering item (T7-C07, H3), not here.

## Dependencies

CFB-04.

