# CFB-46 — Native JSON loss category token: TBD vs tbd

**Candidate brief (H2). Not executed.** Area: Native JSON export (DEL-17-02, DEL-17-03). Run HELP-HUMAN-PIPING-20260921-RECONCILIATION, R3 integration, TASK H2. Execution, if the owner selects it, goes through an owner-steered production brief and the chirality-change PR path, followed by re-verification in a later concordance (ledger rows are not edited in place). Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).

## Scope

Align the native JSON builder and schema loss category `TBD` (freeze `projects/chirality-piping/core/handoff/native_json/package.py:26-33`, `LOSS_CATEGORIES`) with the DEL-17-02 contract token `tbd` used by the MBF, PCF, glTF and stress-neutral schemas, or record the case variance in the DEL-17-02 contract. This is the W3 item "the native JSON token is TBD where the contract uses tbd".

## Affected claims

4 claim rows and 1 W3 item(s) on 2 deliverable(s): DEL-17-02, DEL-17-03.

Classes (portion in this brief / class total): T7-C06 4/42 (Authority REVIEW).

Reproducing filter: `CODE_FIX_ROWS.csv` where `CFB == "CFB-46"`. Each key below is in `R3/CLASS_ASSIGNMENTS.csv`, `R3/TASKS/T8_ROWS.csv` or `R3/TASKS/T12_UNREACHED.csv` with route CODE_FIX_CANDIDATE, except class T7-C05 and NOT_DIVERGENT rows, which enter through T8 only.

| Key | Class | Authority | BlockedOnPacket | T8/T12 view | Remaining work (effective; OC = OtherCorrections) |
|---|---|---|---|---|---|
| `DEL-17-02:SOW#CLM-012` | T7-C06 | REVIEW | H3[T7-C06] | — | Align the native JSON loss category token TBD with the contract token tbd (schema and builder), or record the variance in the contract |
| `DEL-17-02:SOW#CLM-023/DEL-17-02-REQ-054` | T7-C06 | REVIEW | H3[T7-C06] | — | Align the native JSON token TBD with tbd |
| `DEL-17-02:SOW#CLM-043` | T7-C06 | REVIEW | H3[T7-C06] | — | Align the native JSON token TBD with tbd |
| `DEL-17-03:SOW#CLM-010/DEL-17-03-REQ-004` | T7-C06 | REVIEW | H3[T7-C06] | — | Align the native category token with the DEL-17-02 contract token (tbd), or record the case variance in DEL-17-02 |
| `W3-PC-06` (ITEM) | W3_ASSESSMENT | REVIEW | H3[T7-C06] | — | Native JSON token TBD where the contract uses tbd |

## Evidence

Sealed ledgers (reliability: sealed R2 ledger rows with effective values from adopted resolutions; verified where the wave verifier sampled them, otherwise worker reading):

- DEL-17-02: `projects/chirality-piping/execution/_Reconciliation/DeliverableConcordance/RECON_2026-09-21_WHOLE_CORPUS/WAVES/W3/PKG-17/DEL-17-02/DEL-17-02_forward.csv`; ImplementationEvidence cited: `core/handoff/native_json/package.py`, `schemas/native_json_export.schema.json`, `core/handoff/caepipe_mbf/package.py`, `core/handoff/pcf_export/package.py`, `core/handoff/review_geometry/package.py` ….
- DEL-17-03: `projects/chirality-piping/execution/_Reconciliation/DeliverableConcordance/RECON_2026-09-21_WHOLE_CORPUS/WAVES/W3/PKG-17/DEL-17-03/DEL-17-03_forward.csv`; ImplementationEvidence cited: `core/handoff/native_json/package.py`, `schemas/native_json_export.schema.json`, `projects/chirality-piping/execution/PKG-17_Export Format Interoperability/1_Working/DEL-17-02_Export package, profile, and stable ID map contracts/ScopeOfWork.md`.

Freeze line citations in Scope were re-read at the freeze (`00115c719`) by H2 as code reading only; no build or test was run. Classification sources: `R3/TASKS/T4A_CLASSES.md`, `T6_CLASSES.md`, `T7_CLASSES.md`, `T8_CLUSTERS.md`, `T12_UNREACHED.md` (proposals, not accepted results).

## Acceptance checks

- If code changes: schema and builder use `tbd`; a cross-exporter test asserts all six category tokens match.
- If the record changes instead: DEL-17-02 records the variance (record repair, H4).
- Independent review (H3 item for T7-C06).
- The affected ledger rows are re-verified in a later concordance; no ALIGNED status is claimed from this brief.

## Protected-content status

No protected subject: no affected row is at INVARIANT tier or carries an IP_DATA, CLAIMS or SECURITY layer. 
This brief quotes no protected, private or third-party content. Execution uses invented or synthetic fixtures only and introduces no standards text, tables or equation sources (DEC-043).

## BlockedOnPacket

4 of 4 claim rows carry a block: H3[T7-C06] (4). `H3[<class>]` names the H3 register item for that class (review before repair). All rows are blocked.

## Notes and open views

- The choice (code or record) is made at brief time (T7-C06 on-ruling mechanism); either side could be the catch-up.

## Dependencies

H3[T7-C06].

