# CFB-51 — PCF export silent fallbacks: zero coordinates, millimetre assumption, missing disclosure member

**Candidate brief (H2). Not executed.** Area: PCF export (DEL-17-07). Run HELP-HUMAN-PIPING-20260921-RECONCILIATION, R3 integration, TASK H2. Execution, if the owner selects it, goes through an owner-steered production brief and the chirality-change PR path, followed by re-verification in a later concordance (ledger rows are not edited in place). Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).

## Scope

Three W3 items in one exporter: (1) the Python renderer writes 0 for an absent node coordinate (freeze `projects/chirality-piping/core/handoff/pcf_export/package.py:724-725`, `_coord` uses a default of 0) -> add a blocking diagnostic; (2) the desktop packet treats any section quantity unit other than m as millimetres (freeze `apps/desktop/src/features/pcf-export/PcfExportPanel.tsx:701`, factor 1 for non-m) -> replace with a blocking diagnostic for units other than m or mm; (3) `write_pcf_export_package` (`package.py:463`, members written at `:480-488`) omits `unit_system_disclosure.json`, which the manifest lists (`:623`) -> write it when materialization is permitted.

## Affected claims

4 claim rows and 3 W3 item(s) on 1 deliverable(s): DEL-17-07.

Classes (portion in this brief / class total): T7-C06 4/42 (Authority REVIEW).

Reproducing filter: `CODE_FIX_ROWS.csv` where `CFB == "CFB-51"`. Each key below is in `R3/CLASS_ASSIGNMENTS.csv`, `R3/TASKS/T8_ROWS.csv` or `R3/TASKS/T12_UNREACHED.csv` with route CODE_FIX_CANDIDATE, except class T7-C05 and NOT_DIVERGENT rows, which enter through T8 only.

| Key | Class | Authority | BlockedOnPacket | T8/T12 view | Remaining work (effective; OC = OtherCorrections) |
|---|---|---|---|---|---|
| `DEL-17-07:SOW#CLM-006` | T7-C06 | REVIEW | H3[T7-C06] | — | Add blocking diagnostics for absent node coordinates in the Python renderer and for section quantity units other than m or mm in the desktop packet. |
| `DEL-17-07:SOW#CLM-015/DEL-17-07-REQ-021` | T7-C06 | REVIEW | H3[T7-C06] | — | Add diagnostics for absent node coordinates (Python) and unmapped section quantity units (desktop); add negative tests for missing units and coordinates. |
| `DEL-17-07:SOW#CLM-030` | T7-C06 | REVIEW | H3[T7-C06] | — | Add the absent-coordinate diagnostic and write unit_system_disclosure.json in write_pcf_export_package when materialization is permitted. |
| `DEL-17-07:SOW#CLM-043` | T7-C06 | REVIEW | H3[T7-C06] | — | Replace the desktop factor-1 fallback for non-m section units with a blocking diagnostic. |
| `W3-PC-02` (ITEM) | W3_ASSESSMENT | REVIEW | H3[T7-C06] | — | PCF writes 0 for a missing coordinate |
| `W3-PC-03` (ITEM) | W3_ASSESSMENT | REVIEW | H3[T7-C06] | — | Desktop panel treats any section unit other than m as millimetres (worker-raised; not verifier-confirmed) |
| `W3-PC-04` (ITEM) | W3_ASSESSMENT | REVIEW | H3[T7-C06] | — | unit_system_disclosure.json missing from PCF packages |

## Evidence

Sealed ledgers (reliability: sealed R2 ledger rows with effective values from adopted resolutions; verified where the wave verifier sampled them, otherwise worker reading):

- DEL-17-07: `projects/chirality-piping/execution/_Reconciliation/DeliverableConcordance/RECON_2026-09-21_WHOLE_CORPUS/WAVES/W3/PKG-17/DEL-17-07/DEL-17-07_forward.csv`; ImplementationEvidence cited: `core/handoff/pcf_export/package.py`, `apps/desktop/src/features/pcf-export/PcfExportPanel.tsx`, `core/security/redaction/route_control.py`.

Freeze line citations in Scope were re-read at the freeze (`00115c719`) by H2 as code reading only; no build or test was run. Classification sources: `R3/TASKS/T4A_CLASSES.md`, `T6_CLASSES.md`, `T7_CLASSES.md`, `T8_CLUSTERS.md`, `T12_UNREACHED.md` (proposals, not accepted results).

## Acceptance checks

- Negative tests: a node without a coordinate, and a section unit other than m/mm, each yield a blocking diagnostic.
- A materialized package contains every manifest-listed member, including unit_system_disclosure.json.
- Independent review (H3 item for T7-C06).
- The affected ledger rows are re-verified in a later concordance; no ALIGNED status is claimed from this brief.

## Protected-content status

No protected subject: no affected row is at INVARIANT tier or carries an IP_DATA, CLAIMS or SECURITY layer. 
This brief quotes no protected, private or third-party content. Execution uses invented or synthetic fixtures only and introduces no standards text, tables or equation sources (DEC-043).

## BlockedOnPacket

4 of 4 claim rows carry a block: H3[T7-C06] (4). `H3[<class>]` names the H3 register item for that class (review before repair). All rows are blocked.

## Notes and open views

- The millimetre item was worker-raised and not confirmed by a verifier (W3 assessment); the ledger (DEL-17-07 CLM-043) and a code reading of the freeze line agree, but no test has been run.
- The disclosure-file gap is latent: the redaction route withholds materialization for the fixture (tested).
- PCF fixture provenance and the published-table match are C5.

## Dependencies

H3[T7-C06], C5 (context).

