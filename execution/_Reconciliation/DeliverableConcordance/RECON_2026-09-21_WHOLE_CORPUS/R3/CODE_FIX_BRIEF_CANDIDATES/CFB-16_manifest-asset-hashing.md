# CFB-16 — Non-JSON asset manifest hashing and asset-entry fields

**Candidate brief (H2). Not executed.** Area: Rule-pack and audit manifests (DEL-06-04, DEL-08-02). Run HELP-HUMAN-PIPING-20260921-RECONCILIATION, R3 integration, TASK H2. Execution, if the owner selects it, goes through an owner-steered production brief and the chirality-change PR path, followed by re-verification in a later concordance (ledger rows are not edited in place). Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).

## Scope

Define the non-JSON/binary manifest-hash partition and add a producer and validator for `non_json_asset_manifest` checksums; add media type and inclusion-policy fields to AssetManifestEntry; add a test that altering an asset entry changes the asset digest while the model hash record is unchanged.

## Affected claims

5 claim rows on 2 deliverable(s): DEL-06-04, DEL-08-02.

Classes (portion in this brief / class total): T6-C01 5/180 (Authority NONE).

Reproducing filter: `CODE_FIX_ROWS.csv` where `CFB == "CFB-16"`. Each key below is in `R3/CLASS_ASSIGNMENTS.csv`, `R3/TASKS/T8_ROWS.csv` or `R3/TASKS/T12_UNREACHED.csv` with route CODE_FIX_CANDIDATE, except class T7-C05 and NOT_DIVERGENT rows, which enter through T8 only.

| Key | Class | Authority | BlockedOnPacket | T8/T12 view | Remaining work (effective; OC = OtherCorrections) |
|---|---|---|---|---|---|
| `DEL-06-04:SOW#CLM-003.r03` | T6-C01 | NONE | — | — | Define the non-JSON/binary manifest-hash partition and add a producer/validator for non_json_asset_manifest checksums. |
| `DEL-06-04:SOW#CLM-010/R-06-04-005` | T6-C01 | NONE | — | — | Select the non-JSON/binary partition and implement manifest-hash production and checking. |
| `DEL-08-02:SOW#CLM-006.r05` | T6-C01 | NONE | — | — | Add media type and inclusion-policy fields to AssetManifestEntry, or cite where they are recorded. |
| `DEL-08-02:SOW#CLM-013/V-4` | T6-C01 | NONE | — | — | Add a test that alters an asset entry and asserts the asset digest changes while the model hash record is unchanged. |
| `DEL-08-02:SOW#CLM-019` | T6-C01 | NONE | — | — | Close the step gaps tracked on CLM-011.r02, r04, r05 and CLM-006.r05. |

## Evidence

Sealed ledgers (reliability: sealed R2 ledger rows with effective values from adopted resolutions; verified where the wave verifier sampled them, otherwise worker reading):

- DEL-06-04: `projects/chirality-piping/execution/_Reconciliation/DeliverableConcordance/RECON_2026-09-21_WHOLE_CORPUS/WAVES/W3/PKG-06/DEL-06-04/DEL-06-04_forward.csv`; ImplementationEvidence cited: `core/rules/rule_pack_document/src/lib.rs`, `core/serialization/canonical_json`, `schemas/rule_pack.schema.yaml`, `core/rules/rule_pack_lifecycle/src/lib.rs`.
- DEL-08-02: `projects/chirality-piping/execution/_Reconciliation/DeliverableConcordance/RECON_2026-09-21_WHOLE_CORPUS/WAVES/W3/PKG-08/DEL-08-02/DEL-08-02_forward.csv`; ImplementationEvidence cited: `core/reporting/audit_manifest/src/lib.rs`, `apps/desktop/src/services/inputManifestService.ts`, `apps/desktop/src/features/report/reportPackageRequest.ts`, `core/reporting/report_package/src/lib.rs`.

Freeze line citations in Scope were re-read at the freeze (`00115c719`) by H2 as code reading only; no build or test was run. Classification sources: `R3/TASKS/T4A_CLASSES.md`, `T6_CLASSES.md`, `T7_CLASSES.md`, `T8_CLUSTERS.md`, `T12_UNREACHED.md` (proposals, not accepted results).

## Acceptance checks

- A non-JSON asset produces and verifies a manifest checksum; tampering is detected.
- The asset-digest isolation test (DEL-08-02 V-4) passes.
- The affected ledger rows are re-verified in a later concordance; no ALIGNED status is claimed from this brief.

## Protected-content status

No protected subject: no affected row is at INVARIANT tier or carries an IP_DATA, CLAIMS or SECURITY layer. 
This brief quotes no protected, private or third-party content. Execution uses invented or synthetic fixtures only and introduces no standards text, tables or equation sources (DEC-043).

## BlockedOnPacket

None. No affected row's class or T8/T12 reading needs an owner or review decision.

## Notes and open views

- The hash basis label follows A2.

## Dependencies

A2 (label only).

