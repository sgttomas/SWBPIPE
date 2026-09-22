# CFB-02 — DEL-01-01 ISSUED residue: MAINTAINERS.md authority pointer and closeout protected-content statement

**Candidate brief (H2). Not executed.** Area: Governance documents (PKG-01, ISSUED deliverable). Run HELP-HUMAN-PIPING-20260921-RECONCILIATION, R3 integration, TASK H2. Execution, if the owner selects it, goes through an owner-steered production brief and the chirality-change PR path, followed by re-verification in a later concordance (ledger rows are not edited in place). Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).

## Scope

Repoint `governance/MAINTAINERS.md` authority lines from revision 0.7/DAG-007 to the current decomposition revision and DAG (the T4A-C07 proposal names 0.12 and DAG-010); add or locate a run-record statement that the 2026-06-03 license/ISSUED closeout edits introduced no protected standards content. DEL-01-01 is ISSUED, so no edit proceeds until A6 settles the ISSUED change path.

## Affected claims

3 claim rows on 1 deliverable(s): DEL-01-01.

Classes (portion in this brief / class total): T4A-C07 1/3 (Authority NONE); T6-C01 2/180 (Authority NONE).

Reproducing filter: `CODE_FIX_ROWS.csv` where `CFB == "CFB-02"`. Each key below is in `R3/CLASS_ASSIGNMENTS.csv`, `R3/TASKS/T8_ROWS.csv` or `R3/TASKS/T12_UNREACHED.csv` with route CODE_FIX_CANDIDATE, except class T7-C05 and NOT_DIVERGENT rows, which enter through T8 only.

| Key | Class | Authority | BlockedOnPacket | T8/T12 view | Remaining work (effective; OC = OtherCorrections) |
|---|---|---|---|---|---|
| `DEL-01-01:SOW#CLM-011/AC-01-01-05` | T6-C01 | NONE | A6 | — | Add, or locate, a run-record statement that the 2026-06-03 license/ISSUED closeout edits reproduced no protected standards/code content. |
| `DEL-01-01:SOW#CLM-019` | T6-C01 | NONE | A6 | — | As AC-01-01-05: a protected-content statement for the closeout run. |
| `DEL-01-01:SOW#CLM-024.s01` | T4A-C07 | NONE | A6 | — | Bounded MAINTAINERS.md edit: point the authority basis at SOFTWARE_DECOMP revision 0.12 and DAG-010. |

## Evidence

Sealed ledgers (reliability: sealed R2 ledger rows with effective values from adopted resolutions; verified where the wave verifier sampled them, otherwise worker reading):

- DEL-01-01: `projects/chirality-piping/execution/_Reconciliation/DeliverableConcordance/RECON_2026-09-21_WHOLE_CORPUS/WAVES/W2/PKG-01/DEL-01-01/DEL-01-01_forward.csv`; ImplementationEvidence cited: `projects/chirality-piping/LICENSE.md`, `projects/chirality-piping/docs/CONTRACT.md#L43`, `projects/chirality-piping/docs/DIRECTIVE.md#L107`, `NOT_APPLICABLE`, `projects/chirality-piping/governance/MAINTAINERS.md#L19` ….

Freeze line citations in Scope were re-read at the freeze (`00115c719`) by H2 as code reading only; no build or test was run. Classification sources: `R3/TASKS/T4A_CLASSES.md`, `T6_CLASSES.md`, `T7_CLASSES.md`, `T8_CLUSTERS.md`, `T12_UNREACHED.md` (proposals, not accepted results).

## Acceptance checks

- MAINTAINERS.md names the current authority revision and DAG (freeze `projects/chirality-piping/governance/MAINTAINERS.md:20-21,31-32` show the stale values).
- A protected-content statement for the closeout run is located or recorded.
- The change follows whichever path A6 selects (reissue, amendment, or record-elsewhere).
- The affected ledger rows are re-verified in a later concordance; no ALIGNED status is claimed from this brief.

## Protected-content status

No protected subject: no affected row is at INVARIANT tier or carries an IP_DATA, CLAIMS or SECURITY layer. 
This brief quotes no protected, private or third-party content. Execution uses invented or synthetic fixtures only and introduces no standards text, tables or equation sources (DEC-043).

## BlockedOnPacket

3 of 3 claim rows carry a block: A6 (3). `H3[<class>]` names the H3 register item for that class (review before repair; mapped in `H3_TOKEN_MAP.csv`). All rows are blocked.

## Notes and open views

- T4A-C07 authority is NONE, but T4A notes OWNER may apply because DEL-01-01 is ISSUED; every row carries A6.

## Dependencies

A6.

