# CFB-53 — glTF review-geometry profile: GLB or JSON profile, per-family coverage, transform and consumer-risk policy

**Candidate brief (H2). Not executed.** Area: glTF export (DEL-17-08). Run HELP-HUMAN-PIPING-20260921-RECONCILIATION, R3 integration, TASK H2. Execution, if the owner selects it, goes through an owner-steered production brief and the chirality-change PR path, followed by re-verification in a later concordance (ledger rows are not edited in place). Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).

## Scope

Produce GLB or restate the output as the selected JSON glTF profile; record the extras location and a stripping/consumer-risk policy field; add per-family coverage classification with a check; record the centerline simplification as an approximated loss entry; add an impossible-bend diagnostic when bend geometry enters a profile, or amend REQ-043; declare a coordinate transform policy; build the REQ-001..REQ-043 to test matrix.

## Affected claims

10 claim rows on 1 deliverable(s): DEL-17-08.

Classes (portion in this brief / class total): T6-C01 10/180 (Authority NONE).

Reproducing filter: `CODE_FIX_ROWS.csv` where `CFB == "CFB-53"`. Each key below is in `R3/CLASS_ASSIGNMENTS.csv`, `R3/TASKS/T8_ROWS.csv` or `R3/TASKS/T12_UNREACHED.csv` with route CODE_FIX_CANDIDATE, except class T7-C05 and NOT_DIVERGENT rows, which enter through T8 only.

| Key | Class | Authority | BlockedOnPacket | T8/T12 view | Remaining work (effective; OC = OtherCorrections) |
|---|---|---|---|---|---|
| `DEL-17-08:CONTEXT#description` | T6-C01 | NONE | — | — | Produce GLB or restate the description to the selected JSON glTF profile. |
| `DEL-17-08:SOW#CLM-015/DEL-17-08-REQ-021` | T6-C01 | NONE | — | — | Record the extras location and a stripping/consumer-risk policy field in the profile, or state that the authoritative sidecar is the risk policy. |
| `DEL-17-08:SOW#CLM-017/DEL-17-08-REQ-040` | T6-C01 | NONE | — | — | Add per-family coverage classification to the profile. |
| `DEL-17-08:SOW#CLM-017/DEL-17-08-REQ-042` | T6-C01 | NONE | — | — | Record the centerline-for-pipe simplification as an approximated loss entry in the Python fixture/builder path. |
| `DEL-17-08:SOW#CLM-017/DEL-17-08-REQ-043` | T6-C01 | NONE | — | — | Either implement an impossible-bend diagnostic when bend geometry enters a profile, or amend REQ-043 to the selected profile. |
| `DEL-17-08:SOW#CLM-020/A-001` | T6-C01 | NONE | — | — | Build a REQ-to-test matrix for REQ-001 to REQ-043. |
| `DEL-17-08:SOW#CLM-020/X-001` | T6-C01 | NONE | — | — | Add per-family classification and a check over it. |
| `DEL-17-08:SOW#CLM-029` | T6-C01 | NONE | — | — | Complete per-family coverage classification, an origin/transform policy and a consumer-risk policy in the profile. |
| `DEL-17-08:SOW#completion-and-reliance-basis-epistemology/AC-001` | T6-C01 | NONE | — | — | Declare a coordinate transform policy in the Python profile and per-family coverage; record approximation entries for simplified geometry. |
| `DEL-17-08:SOW#purpose-and-objective-traceability/OUT-001` | T6-C01 | NONE | — | — | Either produce the GLB side of the contract or restate the output as the JSON glTF profile. |

## Evidence

Sealed ledgers (reliability: sealed R2 ledger rows with effective values from adopted resolutions; verified where the wave verifier sampled them, otherwise worker reading):

- DEL-17-08: `projects/chirality-piping/execution/_Reconciliation/DeliverableConcordance/RECON_2026-09-21_WHOLE_CORPUS/WAVES/W3/PKG-17/DEL-17-08/DEL-17-08_forward.csv`; ImplementationEvidence cited: `core/handoff/review_geometry/package.py`, `apps/desktop/src/features/review-geometry/ReviewGeometryPanel.tsx`, `schemas/review_geometry_export.schema.json`, `fixtures/review_geometry/invented/source_centerline_payload.json`, `projects/chirality-piping/execution/PKG-17_Export Format Interoperability/1_Working/DEL-17-08_GLB glTF review geometry export/ScopeOfWork.md`.

Freeze line citations in Scope were re-read at the freeze (`00115c719`) by H2 as code reading only; no build or test was run. Classification sources: `R3/TASKS/T4A_CLASSES.md`, `T6_CLASSES.md`, `T7_CLASSES.md`, `T8_CLUSTERS.md`, `T12_UNREACHED.md` (proposals, not accepted results).

## Acceptance checks

- Profile schema test covers per-family classification, transform policy and consumer-risk policy.
- The requirement-to-test matrix exists (record item).
- The affected ledger rows are re-verified in a later concordance; no ALIGNED status is claimed from this brief.

## Protected-content status

No protected subject: no affected row is at INVARIANT tier or carries an IP_DATA, CLAIMS or SECURITY layer. 
This brief quotes no protected, private or third-party content. Execution uses invented or synthetic fixtures only and introduces no standards text, tables or equation sources (DEC-043).

## BlockedOnPacket

None. No affected row's class or T8/T12 reading needs an owner or review decision.

## Notes and open views

- No DEL-17-08 row cites the deleted export plan in its ledger text; the per-family classification is against the selected JSON glTF profile, so an A7 ruling does not change these rows' scope. A7 stays context. glTF fixture provenance is C5 (context).

## Dependencies

A7 (context), C5 (context).

