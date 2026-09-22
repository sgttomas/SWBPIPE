# CFB-35 — Model-state external references: privacy and protected-content screening

**Candidate brief (H2). Not executed.** Area: Model-state records (DEL-14-01); protected subject. Run HELP-HUMAN-PIPING-20260921-RECONCILIATION, R3 integration, TASK H2. Execution, if the owner selects it, goes through an owner-steered production brief and the chirality-change PR path, followed by re-verification in a later concordance (ledger rows are not edited in place). Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).

## Scope

Bind model-state external-reference ingestion and consumption to the governed privacy and protected-content runtime screening.

## Affected claims

2 claim rows on 1 deliverable(s): DEL-14-01.

Classes (portion in this brief / class total): T6-C03 2/34 (Authority REVIEW).

Reproducing filter: `CODE_FIX_ROWS.csv` where `CFB == "CFB-35"`. Each key below is in `R3/CLASS_ASSIGNMENTS.csv`, `R3/TASKS/T8_ROWS.csv` or `R3/TASKS/T12_UNREACHED.csv` with route CODE_FIX_CANDIDATE, except class T7-C05 and NOT_DIVERGENT rows, which enter through T8 only.

| Key | Class | Authority | BlockedOnPacket | T8/T12 view | Remaining work (effective; OC = OtherCorrections) |
|---|---|---|---|---|---|
| `DEL-14-01:SOW#CLM-011.r09` | T6-C03 | REVIEW | H3[T6-C03] | — | Bind model-state external-reference ingestion and consumption to the governed privacy and protected-content runtime screening |
| `DEL-14-01:SOW#CLM-027` | T6-C03 | REVIEW | H3[T6-C03] | — | Bind model-state external-reference ingestion and consumption to governed privacy and protected-content runtime screening |

## Evidence

Sealed ledgers (reliability: sealed R2 ledger rows with effective values from adopted resolutions; verified where the wave verifier sampled them, otherwise worker reading):

- DEL-14-01: `projects/chirality-piping/execution/_Reconciliation/DeliverableConcordance/RECON_2026-09-21_WHOLE_CORPUS/WAVES/W3/PKG-14/DEL-14-01/DEL-14-01_forward.csv`; ImplementationEvidence cited: `schemas/model_state.schema.json`.

Freeze line citations in Scope were re-read at the freeze (`00115c719`) by H2 as code reading only; no build or test was run. Classification sources: `R3/TASKS/T4A_CLASSES.md`, `T6_CLASSES.md`, `T7_CLASSES.md`, `T8_CLUSTERS.md`, `T12_UNREACHED.md` (proposals, not accepted results).

## Acceptance checks

- A negative test: an external reference marked private or protected-suspected is screened at ingestion.
- Independent review (H3 item for T6-C03).
- The affected ledger rows are re-verified in a later concordance; no ALIGNED status is claimed from this brief.

## Protected-content status

Protected subject: 2 of 2 rows are at INVARIANT tier or carry an IP_DATA, CLAIMS or SECURITY layer (row layers: IP_DATA, SECURITY). An independent review of any repair is required before reliance. 
This brief quotes no protected, private or third-party content. Execution uses invented or synthetic fixtures only and introduces no standards text, tables or equation sources (DEC-043).

## BlockedOnPacket

2 of 2 claim rows carry a block: H3[T6-C03] (2). `H3[<class>]` names the H3 register item for that class (review before repair; mapped in `H3_TOKEN_MAP.csv`). All rows are blocked.

## Notes and open views

- Useful only if B8 brings model-state into the product; no block beyond the review is recorded on these rows.

## Dependencies

H3[T6-C03], B8 (context).

