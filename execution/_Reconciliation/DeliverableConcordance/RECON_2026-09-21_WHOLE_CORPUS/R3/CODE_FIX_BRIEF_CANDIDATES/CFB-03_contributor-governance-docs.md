# CFB-03 — Contributor guide dead link and governance tooling residue

**Candidate brief (H2). Not executed.** Area: Contributor documentation (DEL-11-05) and repository governance tooling (DEL-01-03). Run HELP-HUMAN-PIPING-20260921-RECONCILIATION, R3 integration, TASK H2. Execution, if the owner selects it, goes through an owner-steered production brief and the chirality-change PR path, followed by re-verification in a later concordance (ledger rows are not edited in place). Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).

## Scope

Repoint contributor-guide step 1 from the removed `INIT.md` to an existing bootstrap entry (`AGENTS.md` or `loop/LOOP_INIT.md`, as T4A-C07 proposes); add architecture-basis handling to the guide or narrow REQ-11-05-07 (LOCAL_DESIGN, deliverable-local); add ISSUE_TEMPLATE handling to the sanitized-export tooling when the public export is exercised (DEL-01-03 R03, coordinate with DEL-10-04).

## Affected claims

4 claim rows on 2 deliverable(s): DEL-01-03, DEL-11-05.

Classes (portion in this brief / class total): T4A-C07 2/3 (Authority NONE); T6-C01 2/180 (Authority NONE).

Reproducing filter: `CODE_FIX_ROWS.csv` where `CFB == "CFB-03"`. Each key below is in `R3/CLASS_ASSIGNMENTS.csv`, `R3/TASKS/T8_ROWS.csv` or `R3/TASKS/T12_UNREACHED.csv` with route CODE_FIX_CANDIDATE, except class T7-C05 and NOT_DIVERGENT rows, which enter through T8 only.

| Key | Class | Authority | BlockedOnPacket | T8/T12 view | Remaining work (effective; OC = OtherCorrections) |
|---|---|---|---|---|---|
| `DEL-01-03:STATUS#remaining/R03` | T6-C01 | NONE | — | — | Add ISSUE_TEMPLATE handling to the sanitized-export tooling when the public export is exercised (coordinate with DEL-10-04). |
| `DEL-11-05:SOW#CLM-011/REQ-11-05-01` | T4A-C07 | NONE | — | — | Repoint guide step 1 (and this requirement) from the removed INIT.md to the current bootstrap entry. |
| `DEL-11-05:SOW#CLM-011/REQ-11-05-07` | T6-C01 | NONE | — | — | Add architecture-basis handling to the guide, or narrow the requirement now that D-43 consolidated PKG-00 and redirected its gate. |
| `DEL-11-05:SOW#CLM-028` | T4A-C07 | NONE | — | — | Repoint the first reading step (guide and this block) from the removed INIT.md to the current bootstrap entry. |

## Evidence

Sealed ledgers (reliability: sealed R2 ledger rows with effective values from adopted resolutions; verified where the wave verifier sampled them, otherwise worker reading):

- DEL-01-03: `projects/chirality-piping/execution/_Reconciliation/DeliverableConcordance/RECON_2026-09-21_WHOLE_CORPUS/WAVES/W2/PKG-01/DEL-01-03/DEL-01-03_forward.csv`; ImplementationEvidence cited: `projects/chirality-piping/.github/ISSUE_TEMPLATE/README.md`, `projects/chirality-piping/tools/release/export_public_openpipestress.py`, `projects/chirality-piping/execution/_Decomposition/SOFTWARE_DECOMP.md#L650`.
- DEL-11-05: `projects/chirality-piping/execution/_Reconciliation/DeliverableConcordance/RECON_2026-09-21_WHOLE_CORPUS/WAVES/W3/PKG-11/DEL-11-05/DEL-11-05_forward.csv`; ImplementationEvidence cited: `projects/chirality-piping/docs/contributor_guide/index.md`, `projects/chirality-piping/docs/AGENTIC_DEVELOPMENT_WORKFLOW.md`.

Freeze line citations in Scope were re-read at the freeze (`00115c719`) by H2 as code reading only; no build or test was run. Classification sources: `R3/TASKS/T4A_CLASSES.md`, `T6_CLASSES.md`, `T7_CLASSES.md`, `T8_CLUSTERS.md`, `T12_UNREACHED.md` (proposals, not accepted results).

## Acceptance checks

- `projects/chirality-piping/docs/contributor_guide/index.md:37` no longer links an absent file; a link check over the guide passes.
- REQ-11-05-07 is either met in the guide text or narrowed in the SOW (record repair).
- Sanitized-export tooling either handles ISSUE_TEMPLATE or records the deferral.
- The affected ledger rows are re-verified in a later concordance; no ALIGNED status is claimed from this brief.

## Protected-content status

No protected subject: no affected row is at INVARIANT tier or carries an IP_DATA, CLAIMS or SECURITY layer. 
This brief quotes no protected, private or third-party content. Execution uses invented or synthetic fixtures only and introduces no standards text, tables or equation sources (DEC-043).

## BlockedOnPacket

None. No affected row's class or T8 reading needs an owner or review decision.

## Notes and open views

- The DEL-11-05 SOW rows take the same repoint as record repair (T4A-C05 mechanism, H4).

## Dependencies

H4 (paired SOW record repair).

