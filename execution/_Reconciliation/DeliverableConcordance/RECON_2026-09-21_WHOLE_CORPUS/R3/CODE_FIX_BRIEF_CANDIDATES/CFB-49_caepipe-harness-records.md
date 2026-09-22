# CFB-49 — CAEPIPE harness run-record fields (non-live parts)

**Candidate brief (H2). Not executed.** Area: CAEPIPE external harness (DEL-17-05). Run HELP-HUMAN-PIPING-20260921-RECONCILIATION, R3 integration, TASK H2. Execution, if the owner selects it, goes through an owner-steered production brief and the chirality-change PR path, followed by re-verification in a later concordance (ledger rows are not edited in place). Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).

## Scope

Complete the run-record fields the ledger names: MBF profile ID (or TBD), source model/export IDs, environment context, coverage-register version, missing-section summary and diagnostics, manifest/ID-map/loss-report links, configuration provenance and sidecar references, public-CI nonfailure classification and parser-only execution fields; write harness configuration guidance and a coverage-register note.

## Affected claims

18 claim rows on 1 deliverable(s): DEL-17-05.

Classes (portion in this brief / class total): T6-C01 18/180 (Authority NONE).

Reproducing filter: `CODE_FIX_ROWS.csv` where `CFB == "CFB-49"`. Each key below is in `R3/CLASS_ASSIGNMENTS.csv`, `R3/TASKS/T8_ROWS.csv` or `R3/TASKS/T12_UNREACHED.csv` with route CODE_FIX_CANDIDATE, except class T7-C05 and NOT_DIVERGENT rows, which enter through T8 only.

| Key | Class | Authority | BlockedOnPacket | T8/T12 view | Remaining work (effective; OC = OtherCorrections) |
|---|---|---|---|---|---|
| `DEL-17-05:SOW#CLM-004.r04` | T6-C01 | NONE | — | — | Add an MBF profile field (or TBD) to the run record. |
| `DEL-17-05:SOW#CLM-006.r02` | T6-C01 | NONE | — | — | Complete the named record fields; the live parts wait on the owner gate. |
| `DEL-17-05:SOW#CLM-006.r03` | T6-C01 | NONE | — | — | Complete the named record fields; the live parts wait on the owner gate. |
| `DEL-17-05:SOW#CLM-006.r04` | T6-C01 | NONE | — | — | Complete the named record fields; the live parts wait on the owner gate. |
| `DEL-17-05:SOW#CLM-012/DEL-17-05-REQ-005` | T6-C01 | NONE | — | — | Add the missing record fields or diagnostics named in Notes. |
| `DEL-17-05:SOW#CLM-012/DEL-17-05-REQ-006` | T6-C01 | NONE | — | — | Add the missing record fields or diagnostics named in Notes. |
| `DEL-17-05:SOW#CLM-012/DEL-17-05-REQ-009` | T6-C01 | NONE | — | — | Add the missing record fields or diagnostics named in Notes. |
| `DEL-17-05:SOW#CLM-012/DEL-17-05-REQ-011` | T6-C01 | NONE | — | — | Add the missing record fields or diagnostics named in Notes. |
| `DEL-17-05:SOW#CLM-012/DEL-17-05-REQ-015` | T6-C01 | NONE | — | — | Add the missing record fields or diagnostics named in Notes. |
| `DEL-17-05:SOW#CLM-012/DEL-17-05-REQ-016` | T6-C01 | NONE | — | — | Add the missing record fields or diagnostics named in Notes. |
| `DEL-17-05:SOW#CLM-013/DEL-17-05-PH-002` | T6-C01 | NONE | — | — | Define manifest, ID-map and loss-report links in the run record. |
| `DEL-17-05:SOW#CLM-013/DEL-17-05-PH-004` | T6-C01 | NONE | — | — | Add public-CI nonfailure classification and parser-only execution fields. |
| `DEL-17-05:SOW#CLM-016/VER-004` | T6-C01 | NONE | — | — | As PH-004. |
| `DEL-17-05:SOW#CLM-017.s01` | T6-C01 | NONE | — | — | Write harness configuration guidance and a coverage-register note. |
| `DEL-17-05:SOW#CLM-028` | T6-C01 | NONE | — | — | Add source model/export IDs, MBF profile ID, environment context, coverage-register version and missing-section summary. |
| `DEL-17-05:SOW#CLM-030.s01` | T6-C01 | NONE | — | — | Add configuration provenance and explicit sidecar references. |
| `DEL-17-05:SOW#CLM-035` | T6-C01 | NONE | — | — | Diagnose missing sections and carry manifest and loss-report vocabulary in the run record. |
| `DEL-17-05:SOW#completion-and-reliance-basis-epistemology/AC-001` | T6-C01 | NONE | — | — | Add environment metadata and an MBF profile input; live parts wait on the gate. |

## Evidence

Sealed ledgers (reliability: sealed R2 ledger rows with effective values from adopted resolutions; verified where the wave verifier sampled them, otherwise worker reading):

- DEL-17-05: `projects/chirality-piping/execution/_Reconciliation/DeliverableConcordance/RECON_2026-09-21_WHOLE_CORPUS/WAVES/W3/PKG-17/DEL-17-05/DEL-17-05_forward.csv`; ImplementationEvidence cited: `core/handoff/caepipe_external/run.py`, `schemas/caepipe_external_run.schema.json`, `projects/chirality-piping/docs/developer_guide/index.md#L121`, `projects/chirality-piping/docs/user_guide/index.md#L269`, `fixtures/caepipe_external/invented/caepipe_results.csv` ….

Freeze line citations in Scope were re-read at the freeze (`00115c719`) by H2 as code reading only; no build or test was run. Classification sources: `R3/TASKS/T4A_CLASSES.md`, `T6_CLASSES.md`, `T7_CLASSES.md`, `T8_CLUSTERS.md`, `T12_UNREACHED.md` (proposals, not accepted results).

## Acceptance checks

- Run-record schema tests assert each named field.
- Nothing launches an external process; the live parts stay behind the DEC-080 owner gate (C08, no action).
- The affected ledger rows are re-verified in a later concordance; no ALIGNED status is claimed from this brief.

## Protected-content status

No protected subject: no affected row is at INVARIANT tier or carries an IP_DATA, CLAIMS or SECURITY layer. 
This brief quotes no protected, private or third-party content. Execution uses invented or synthetic fixtures only and introduces no standards text, tables or equation sources (DEC-043).

## BlockedOnPacket

None. No affected row's class or T8 reading needs an owner or review decision.

## Notes and open views

- The DEL-17-05 GUI panel question is B6; the live run is held by DEC-080 (T6-C08).

## Dependencies

B6 (context).

