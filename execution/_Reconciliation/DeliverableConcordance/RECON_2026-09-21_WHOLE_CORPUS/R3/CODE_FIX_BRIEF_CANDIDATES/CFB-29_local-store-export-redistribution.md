# CFB-29 — Local-first store and export: provenance survival and redistribution metadata in export decisions

**Candidate brief (H2). Not executed.** Area: Local-first storage and redaction/export control (DEL-12-01, DEL-12-02); protected subject. Run HELP-HUMAN-PIPING-20260921-RECONCILIATION, R3 integration, TASK H2. Execution, if the owner selects it, goes through an owner-steered production brief and the chirality-change PR path, followed by re-verification in a later concordance (ledger rows are not edited in place). Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).

## Scope

Add a store/adapter-level test that provenance and redistribution status survive save/open and an export route; add runtime private-path resolution and provenance-preservation tests for the governed store; carry schema privacy and redistribution metadata from library and model records into export decisions, with a test; add source and provenance fields to redaction findings or narrow the verification expectation; plugin-route tests once a plugin runtime exists.

## Affected claims

5 claim rows on 2 deliverable(s): DEL-12-01, DEL-12-02.

Classes (portion in this brief / class total): T6-C01 3/180 (Authority NONE); T6-C03 2/34 (Authority REVIEW).

Reproducing filter: `CODE_FIX_ROWS.csv` where `CFB == "CFB-29"`. Each key below is in `R3/CLASS_ASSIGNMENTS.csv`, `R3/TASKS/T8_ROWS.csv` or `R3/TASKS/T12_UNREACHED.csv` with route CODE_FIX_CANDIDATE, except class T7-C05 and NOT_DIVERGENT rows, which enter through T8 only.

| Key | Class | Authority | BlockedOnPacket | T8/T12 view | Remaining work (effective; OC = OtherCorrections) |
|---|---|---|---|---|---|
| `DEL-12-01:SOW#CLM-010/LFSP-REQ-008` | T6-C03 | REVIEW | H3[T6-C03] | — | Add or locate a store/adapter-level test that asserts provenance and redistribution status survive a save/open and an export route. |
| `DEL-12-01:SOW#CLM-010/LFSP-REQ-011` | T6-C01 | NONE | ER-20 | — | Locate or add runtime private-path resolution and provenance-preservation tests for the governed store; decide whether DEL-02-05 store tests count toward LFSP-REQ-011. |
| `DEL-12-02:SOW#CLM-012/REXC-REQ-010` | T6-C03 | REVIEW | H3[T6-C03] | — | Carry schema privacy and redistribution metadata from library and model records into export decisions, with a test. |
| `DEL-12-02:SOW#CLM-012/REXC-REQ-011` | T6-C01 | NONE | — | — | Add source and provenance fields to redaction findings or narrow the verification expectation. |
| `DEL-12-02:SOW#CLM-012/REXC-REQ-014` | T6-C01 | NONE | B10 | — | Plugin-route tests once a plugin runtime exists. |

## Evidence

Sealed ledgers (reliability: sealed R2 ledger rows with effective values from adopted resolutions; verified where the wave verifier sampled them, otherwise worker reading):

- DEL-12-01: `projects/chirality-piping/execution/_Reconciliation/DeliverableConcordance/RECON_2026-09-21_WHOLE_CORPUS/WAVES/W3/PKG-12/DEL-12-01/DEL-12-01_forward.csv`; ImplementationEvidence cited: `core/security/local_first_storage/controls.py`, `core/security/redaction/controls.py`, `apps/desktop/src-tauri/src/lib.rs::validate_library_import`, `core/security/local_first_storage/route_control.py`, `apps/desktop/src-tauri/src/lib.rs::open_project_store`.
- DEL-12-02: `projects/chirality-piping/execution/_Reconciliation/DeliverableConcordance/RECON_2026-09-21_WHOLE_CORPUS/WAVES/W3/PKG-12/DEL-12-02/DEL-12-02_forward.csv`; ImplementationEvidence cited: `core/security/redaction/controls.py`, `core/security/redaction/route_control.py`, `apps/desktop/src/features/redaction-controls/redactionExportControls.ts`, `schemas/redaction_export_controls.schema.yaml`, `apps/desktop/src/features/redaction-controls/RedactionExportControlsPanel.tsx` ….

Freeze line citations in Scope were re-read at the freeze (`00115c719`) by H2 as code reading only; no build or test was run. Classification sources: `R3/TASKS/T4A_CLASSES.md`, `T6_CLASSES.md`, `T7_CLASSES.md`, `T8_CLUSTERS.md`, `T12_UNREACHED.md` (proposals, not accepted results).

## Acceptance checks

- A test exports a record marked non-redistributable and shows the export decision uses the metadata.
- Save/open and export round trips preserve provenance and redistribution status.
- Independent review for the INVARIANT rows (H3 item for T6-C03).
- The affected ledger rows are re-verified in a later concordance; no ALIGNED status is claimed from this brief.

## Protected-content status

Protected subject: 2 of 5 rows are at INVARIANT tier or carry an IP_DATA, CLAIMS or SECURITY layer (row layers: IP_DATA). An independent review of any repair is required before reliance. 
This brief quotes no protected, private or third-party content. Execution uses invented or synthetic fixtures only and introduces no standards text, tables or equation sources (DEC-043).

## BlockedOnPacket

4 of 5 claim rows carry a block: B10 (1); ER-20 (1); H3[T6-C03] (2). `H3[<class>]` names the H3 register item for that class (review before repair; mapped in `H3_TOKEN_MAP.csv`). Unblocked rows may proceed separately once selected.

## Notes and open views

- REXC-REQ-014 waits on B10.
- LFSP-REQ-011 asks whether DEL-02-05 store tests count toward it; H3 ER-20 gives that choice to the reviewer (DEL-12-01 R01). The row is blocked on ER-20.

## Dependencies

H3[T6-C03], B10, ER-20.

