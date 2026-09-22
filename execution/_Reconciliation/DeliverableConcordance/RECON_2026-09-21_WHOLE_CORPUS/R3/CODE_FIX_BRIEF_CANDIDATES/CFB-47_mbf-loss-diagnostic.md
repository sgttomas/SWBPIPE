# CFB-47 — MBF export: make MBF-LOSS-REPORT-MISSING reachable; sidecar identity coverage

**Candidate brief (H2). Not executed.** Area: CAEPIPE MBF export (DEL-17-04). Run HELP-HUMAN-PIPING-20260921-RECONCILIATION, R3 integration, TASK H2. Execution, if the owner selects it, goes through an owner-steered production brief and the chirality-change PR path, followed by re-verification in a later concordance (ledger rows are not edited in place). Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).

## Scope

Stop `_loss_report` (freeze `projects/chirality-piping/core/handoff/caepipe_mbf/package.py:632-640`) from substituting a default tbd entry for an empty loss report so `MBF-LOSS-REPORT-MISSING` (`:335`) can fire; add a check that every canonical identity appears in the sidecar map or a loss entry. This is the W3 item "the MBF loss-report diagnostic can never fire".

## Affected claims

3 claim rows and 1 W3 item(s) on 1 deliverable(s): DEL-17-04.

Classes (portion in this brief / class total): T7-C06 3/42 (Authority REVIEW).

Reproducing filter: `CODE_FIX_ROWS.csv` where `CFB == "CFB-47"`. Each key below is in `R3/CLASS_ASSIGNMENTS.csv`, `R3/TASKS/T8_ROWS.csv` or `R3/TASKS/T12_UNREACHED.csv` with route CODE_FIX_CANDIDATE, except class T7-C05 and NOT_DIVERGENT rows, which enter through T8 only.

| Key | Class | Authority | BlockedOnPacket | T8/T12 view | Remaining work (effective; OC = OtherCorrections) |
|---|---|---|---|---|---|
| `DEL-17-04:SOW#CLM-009/DEL-17-04-REQ-009` | T7-C06 | REVIEW | H3[T7-C06] | — | Smallest check: build with nodes, elements and sidecar map present and loss_report=[]; confirm whether validation_status is blocked and MBF-LOSS-REPORT-MISSING appears. |
| `DEL-17-04:SOW#CLM-011.r10` | T7-C06 | REVIEW | H3[T7-C06] | — | Same smallest check as REQ-009 (missing loss report alone). |
| `DEL-17-04:SOW#CLM-023` | T7-C06 | REVIEW | H3[T7-C06] | — | Same smallest check as REQ-009. Also check whether an ID absent from both sidecar map and loss report is detected. |
| `W3-PC-05` (ITEM) | W3_ASSESSMENT | REVIEW | H3[T7-C06] | — | The MBF loss-report diagnostic can never fire |

## Evidence

Sealed ledgers (reliability: sealed R2 ledger rows with effective values from adopted resolutions; verified where the wave verifier sampled them, otherwise worker reading):

- DEL-17-04: `projects/chirality-piping/execution/_Reconciliation/DeliverableConcordance/RECON_2026-09-21_WHOLE_CORPUS/WAVES/W3/PKG-17/DEL-17-04/DEL-17-04_forward.csv`; ImplementationEvidence cited: `core/handoff/caepipe_mbf/package.py#L299`, `core/handoff/caepipe_mbf/package.py#L332`, `core/handoff/caepipe_mbf/package.py#L633`, `core/handoff/caepipe_mbf/package.py`, `schemas/caepipe_mbf_export.schema.json` ….

Freeze line citations in Scope were re-read at the freeze (`00115c719`) by H2 as code reading only; no build or test was run. Classification sources: `R3/TASKS/T4A_CLASSES.md`, `T6_CLASSES.md`, `T7_CLASSES.md`, `T8_CLUSTERS.md`, `T12_UNREACHED.md` (proposals, not accepted results).

## Acceptance checks

- Smallest check named in the ledger: build with nodes, elements and sidecar map present and `loss_report=[]`; validation_status is blocked by MBF-LOSS-REPORT-MISSING.
- An ID absent from both sidecar map and loss report is detected.
- Independent review (H3 item for T7-C06).
- The affected ledger rows are re-verified in a later concordance; no ALIGNED status is claimed from this brief.

## Protected-content status

No protected subject: no affected row is at INVARIANT tier or carries an IP_DATA, CLAIMS or SECURITY layer. 
This brief quotes no protected, private or third-party content. Execution uses invented or synthetic fixtures only and introduces no standards text, tables or equation sources (DEC-043).

## BlockedOnPacket

3 of 3 claim rows carry a block: H3[T7-C06] (3). `H3[<class>]` names the H3 register item for that class (review before repair). All rows are blocked.

## Notes and open views

- The DEL-17-04 MBF panel question is B6, not here.

## Dependencies

H3[T7-C06], B6 (context).

