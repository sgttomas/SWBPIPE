# CFB-06 — Plugin manifest fixture conformance and manifest hashing tests

**Candidate brief (H2). Not executed.** Area: Plugin manifest contract (DEL-02-04). Run HELP-HUMAN-PIPING-20260921-RECONCILIATION, R3 integration, TASK H2. Execution, if the owner selects it, goes through an owner-steered production brief and the chirality-change PR path, followed by re-verification in a later concordance (ledger rows are not edited in place). Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).

## Scope

Bring the published invented fixture `fixtures/plugin_manifest/invented_manifest_no_bypass.json` into conformance with `schemas/plugin_manifest.schema.yaml` (manifest_kind const, checksums shape, id patterns) or retire it, and add a full-schema validation assertion; add manifest canonicalization/hash tests; record the gate state; add behavioural sandbox/security tests only once a rule-pack-facing hook exists.

## Affected claims

5 claim rows on 1 deliverable(s): DEL-02-04.

Classes (portion in this brief / class total): T6-C01 2/180 (Authority NONE); T6-C03 2/34 (Authority REVIEW); T7-C06 1/42 (Authority REVIEW).

Reproducing filter: `CODE_FIX_ROWS.csv` where `CFB == "CFB-06"`. Each key below is in `R3/CLASS_ASSIGNMENTS.csv`, `R3/TASKS/T8_ROWS.csv` or `R3/TASKS/T12_UNREACHED.csv` with route CODE_FIX_CANDIDATE, except class T7-C05 and NOT_DIVERGENT rows, which enter through T8 only.

| Key | Class | Authority | BlockedOnPacket | T8/T12 view | Remaining work (effective; OC = OtherCorrections) |
|---|---|---|---|---|---|
| `DEL-02-04:SOW#CLM-012/DEL-02-04-REQ-11` | T7-C06 | REVIEW | H3[T7-C06] | — | Bring the public invented fixture into conformance with the canonical manifest schema, or retire it, and add a full-schema validation assertion for it. |
| `DEL-02-04:SOW#CLM-014.r04` | T6-C01 | NONE | — | — | Add manifest canonicalization/hash tests (JCS) and validate the public fixture against the canonical schema. |
| `DEL-02-04:SOW#CLM-014/REQ-13` | T6-C03 | REVIEW | H3[T6-C03];B10 | — | When a rule-pack-facing hook exists, add behavioural security tests; until then record that only declaration-level checks exist. |
| `DEL-02-04:SOW#CLM-015.r01` | T6-C01 | NONE | — | — | Record the gate state: schema validation evidence exists; manifest hash evidence and a layout/field approval record do not. |
| `DEL-02-04:SOW#CLM-015.r03` | T6-C03 | REVIEW | H3[T6-C03];B10 | — | Behavioural sandbox tests when a rule-pack-facing hook exists. |

## Evidence

Sealed ledgers (reliability: sealed R2 ledger rows with effective values from adopted resolutions; verified where the wave verifier sampled them, otherwise worker reading):

- DEL-02-04: `projects/chirality-piping/execution/_Reconciliation/DeliverableConcordance/RECON_2026-09-21_WHOLE_CORPUS/WAVES/W2/PKG-02/DEL-02-04/DEL-02-04_forward.csv`; ImplementationEvidence cited: `schemas/plugin_manifest.schema.yaml`, `core/adapters/framework/plugin_verification.py`, `fixtures/plugin_manifest/invented_manifest_no_bypass.json`.

Freeze line citations in Scope were re-read at the freeze (`00115c719`) by H2 as code reading only; no build or test was run. Classification sources: `R3/TASKS/T4A_CLASSES.md`, `T6_CLASSES.md`, `T7_CLASSES.md`, `T8_CLUSTERS.md`, `T12_UNREACHED.md` (proposals, not accepted results).

## Acceptance checks

- A test validates every published plugin-manifest fixture against the canonical schema (fails on the frozen fixture).
- Manifest canonicalization and hash tests exist.
- Behavioural hook tests are added only when B10 selects a runtime; until then the record states declaration-level checks only.
- The affected ledger rows are re-verified in a later concordance; no ALIGNED status is claimed from this brief.

## Protected-content status

Protected subject: 2 of 5 rows are at INVARIANT tier or carry an IP_DATA, CLAIMS or SECURITY layer (row layers: SECURITY). An independent review of any repair is required before reliance. 
This brief quotes no protected, private or third-party content. Execution uses invented or synthetic fixtures only and introduces no standards text, tables or equation sources (DEC-043).

## BlockedOnPacket

3 of 5 claim rows carry a block: B10 (2); H3[T6-C03] (2); H3[T7-C06] (1). `H3[<class>]` names the H3 register item for that class (review before repair; mapped in `H3_TOKEN_MAP.csv`). Unblocked rows may proceed separately once selected.

## Notes and open views

- REQ-13 and CLM-015.r03 wait on B10 (plugin runtime and grant model).

## Dependencies

B10, A2 (hash basis label).

