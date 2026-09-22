# CFB-21 — Protected-content linter: product-neutral prohibited-claim phrases

**Candidate brief (H2). Not executed.** Area: Reporting linter (DEL-08-05). Run HELP-HUMAN-PIPING-20260921-RECONCILIATION, R3 integration, TASK H2. Execution, if the owner selects it, goes through an owner-steered production brief and the chirality-change PR path, followed by re-verification in a later concordance (ledger rows are not edited in place). Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).

## Scope

Make the linter's certification/sealing/approval/authentication phrase list match claims naming the current product, or be product-neutral; at the freeze it matches only the former product name (`projects/chirality-piping/core/reporting/protected_content_linter/src/lib.rs:639-642`). Add a fixture case naming the current product.

## Affected claims

3 claim rows on 1 deliverable(s): DEL-08-05.

Classes (portion in this brief / class total): T7-C06 3/42 (Authority REVIEW).

Reproducing filter: `CODE_FIX_ROWS.csv` where `CFB == "CFB-21"`. Each key below is in `R3/CLASS_ASSIGNMENTS.csv`, `R3/TASKS/T8_ROWS.csv` or `R3/TASKS/T12_UNREACHED.csv` with route CODE_FIX_CANDIDATE, except class T7-C05 and NOT_DIVERGENT rows, which enter through T8 only.

| Key | Class | Authority | BlockedOnPacket | T8/T12 view | Remaining work (effective; OC = OtherCorrections) |
|---|---|---|---|---|---|
| `DEL-08-05:SOW#CLM-006.r03` | T7-C06 | REVIEW | H3[T7-C06] | — | Same repair as DEL-08-05-REQ-006 |
| `DEL-08-05:SOW#CLM-011/DEL-08-05-REQ-006` | T7-C06 | REVIEW | H3[T7-C06] | — | Make the engine's certification/sealing/approval/authentication phrases name the current product or be product-neutral, with a test |
| `DEL-08-05:SOW#CLM-013.r04` | T7-C06 | REVIEW | H3[T7-C06] | — | Add a fixture case naming the current product |

## Evidence

Sealed ledgers (reliability: sealed R2 ledger rows with effective values from adopted resolutions; verified where the wave verifier sampled them, otherwise worker reading):

- DEL-08-05: `projects/chirality-piping/execution/_Reconciliation/DeliverableConcordance/RECON_2026-09-21_WHOLE_CORPUS/WAVES/W3/PKG-08/DEL-08-05/DEL-08-05_forward.csv`; ImplementationEvidence cited: `core/reporting/protected_content_linter/src/lib.rs`, `apps/desktop/src/features/report-lint/ReportLintPanel.tsx`, `core/reporting/report_renderer/src/lib.rs`.

Freeze line citations in Scope were re-read at the freeze (`00115c719`) by H2 as code reading only; no build or test was run. Classification sources: `R3/TASKS/T4A_CLASSES.md`, `T6_CLASSES.md`, `T7_CLASSES.md`, `T8_CLUSTERS.md`, `T12_UNREACHED.md` (proposals, not accepted results).

## Acceptance checks

- A fixture claim naming the current product is flagged; a product-neutral phrasing is flagged.
- Existing former-name cases still pass.
- Independent review (H3 item for T7-C06): the check guards the professional boundary.
- The affected ledger rows are re-verified in a later concordance; no ALIGNED status is claimed from this brief.

## Protected-content status

Protected subject: 3 of 3 rows are at INVARIANT tier or carry protected layers (CLAIMS). An independent review of any repair is required before reliance. 
This brief quotes no protected, private or third-party content. Execution uses invented or synthetic fixtures only and introduces no standards text, tables or equation sources (DEC-043).

## BlockedOnPacket

3 of 3 claim rows carry a block: H3[T7-C06] (3). `H3[<class>]` names the H3 register item for that class (review before repair). All rows are blocked.

## Notes and open views

- A4 governs rename residue generally; this brief fixes detection coverage and does not decide identifier renames.

## Dependencies

H3[T7-C06], A4 (context).

