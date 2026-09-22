# CFB-31 — Private-library guard integration into product export, report and share seams

**Candidate brief (H2). Not executed.** Area: Private libraries (DEL-12-04); partly protected subject. Run HELP-HUMAN-PIPING-20260921-RECONCILIATION, R3 integration, TASK H2. Execution, if the owner selects it, goes through an owner-steered production brief and the chirality-change PR path, followed by re-verification in a later concordance (ledger rows are not edited in place). Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).

## Scope

Route private-library references through the DEL-12-04 guard at export, report and share seams and define the bug-report bundle path; call the helper classification from a product private-library path and map helper diagnostics to the SPEC warning classes; add focused tests for unknown source state and protected_suspected records; add a check over project artifacts or narrow the signal; correct or add the telemetry-exclusion test bullet; apply reference-only credential handling once credentials are used; extend routed control checks to private storage and plugin paths when those exist.

## Affected claims

12 claim rows on 1 deliverable(s): DEL-12-04.

Classes (portion in this brief / class total): T6-C01 6/180 (Authority NONE); T6-C02 3/116 (Authority NONE); T6-C03 3/34 (Authority REVIEW).

Reproducing filter: `CODE_FIX_ROWS.csv` where `CFB == "CFB-31"`. Each key below is in `R3/CLASS_ASSIGNMENTS.csv`, `R3/TASKS/T8_ROWS.csv` or `R3/TASKS/T12_UNREACHED.csv` with route CODE_FIX_CANDIDATE, except class T7-C05 and NOT_DIVERGENT rows, which enter through T8 only.

| Key | Class | Authority | BlockedOnPacket | T8/T12 view | Remaining work (effective; OC = OtherCorrections) |
|---|---|---|---|---|---|
| `DEL-12-04:SOW#CLM-004.r03` | T6-C02 | NONE | — | — | Route private-library references through the DEL-12-04 guard at export and share seams, and define the bug-report bundle path. |
| `DEL-12-04:SOW#CLM-004.r04` | T6-C03 | REVIEW | H3[T6-C03] | — | Call the helper classification from a product private-library path, or record that library-import provenance checks own this condition. |
| `DEL-12-04:SOW#CLM-011.r04` | T6-C01 | NONE | B12 | — | Apply reference-only credential handling in a product artifact path once credentials are used by imports or storage. |
| `DEL-12-04:SOW#CLM-011.r05` | T6-C03 | REVIEW | H3[T6-C03] | — | Integrate private-library reference guarding into export/report seams and define the bug-report bundle path. |
| `DEL-12-04:SOW#CLM-011.r08` | T6-C02 | NONE | B10 | — | Extend routed control checks to private storage and plugin paths when those exist. |
| `DEL-12-04:SOW#CLM-011.r10` | T6-C03 | REVIEW | H3[T6-C03] | — | Map helper diagnostics to the SPEC warning classes and call the classification from a product path; add tests for unknown source state and privacy class. |
| `DEL-12-04:SOW#CLM-013.r03` | T6-C01 | NONE | — | — | Add a check over project artifacts, or narrow the signal to registry records. |
| `DEL-12-04:SOW#CLM-013.r05` | T6-C02 | NONE | — | — | Integrate the DEL-12-04 guard into runtime report/export routes. |
| `DEL-12-04:SOW#CLM-013.r07` | T6-C01 | NONE | — | — | Add focused tests for unknown source state and protected_suspected records. |
| `DEL-12-04:SOW#CLM-018.s01` | T6-C01 | NONE | — | — | Remove telemetry exclusion from the DEL-12-04 test list or add the test. |
| `DEL-12-04:SOW#CLM-023.s01` | T6-C01 | NONE | — | — | Correct the bullet or add the telemetry case. |
| `DEL-12-04:SOW#CLM-027` | T6-C01 | NONE | B10;B12 | — | Product registration flow and plugin permission handling would complete examples 1 and 3. |

## Evidence

Sealed ledgers (reliability: sealed R2 ledger rows with effective values from adopted resolutions; verified where the wave verifier sampled them, otherwise worker reading):

- DEL-12-04: `projects/chirality-piping/execution/_Reconciliation/DeliverableConcordance/RECON_2026-09-21_WHOLE_CORPUS/WAVES/W3/PKG-12/DEL-12-04/DEL-12-04_forward.csv`; ImplementationEvidence cited: `core/security/secret_private_library/controls.py`, `core/security/redaction/route_control.py`, `core/security/redaction/controls.py`, `core/library_import/provenance_checker.py`, `core/library_import/library_import_document/src/lib.rs` ….

Freeze line citations in Scope were re-read at the freeze (`00115c719`) by H2 as code reading only; no build or test was run. Classification sources: `R3/TASKS/T4A_CLASSES.md`, `T6_CLASSES.md`, `T7_CLASSES.md`, `T8_CLUSTERS.md`, `T12_UNREACHED.md` (proposals, not accepted results).

## Acceptance checks

- A product export/report test shows a private-library reference is guarded (blocked or redacted with a diagnostic).
- Tests cover unknown source state and protected_suspected records.
- Independent review for the INVARIANT rows (H3 item for T6-C03).
- The affected ledger rows are re-verified in a later concordance; no ALIGNED status is claimed from this brief.

## Protected-content status

Protected subject: 9 of 12 rows are at INVARIANT tier or carry an IP_DATA, CLAIMS or SECURITY layer (row layers: IP_DATA, SECURITY). An independent review of any repair is required before reliance. 
This brief quotes no protected, private or third-party content. Execution uses invented or synthetic fixtures only and introduces no standards text, tables or equation sources (DEC-043).

## BlockedOnPacket

6 of 12 claim rows carry a block: B10 (2); B12 (2); H3[T6-C03] (3). `H3[<class>]` names the H3 register item for that class (review before repair; mapped in `H3_TOKEN_MAP.csv`). Unblocked rows may proceed separately once selected.

## Notes and open views

- OBSERVED correction on CLM-027: `lib.rs::save_local_library` and `save_local_rule_pack` exist; the remaining gap is registration flow and plugin permission (B10, B12 D4).
- CLM-011.r04 credential handling depends on the secret provider selection (B12 D4).
- CLM-018.s01 and CLM-023.s01 overlap T6-C07 placement of telemetry exclusion tests (H1).

## Dependencies

H3[T6-C03], B10, B12 (D4), H1 (T6-C07 context).

