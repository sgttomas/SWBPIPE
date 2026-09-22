# CFB-45 — Export profiles: align to the common field list and ID-map carrier values

**Candidate brief (H2). Not executed.** Area: Export package contracts (DEL-17-02, DEL-17-03). Run HELP-HUMAN-PIPING-20260921-RECONCILIATION, R3 integration, TASK H2. Execution, if the owner selects it, goes through an owner-steered production brief and the chirality-change PR path, followed by re-verification in a later concordance (ledger rows are not edited in place). Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).

## Scope

Align exporter profiles to the common field list (entity_coverage, target_version_basis, coordinate_policy, stable_id_policy; `identity_policy` is used in three profiles); declare target version basis uniformly; add source_basis_refs to PCF and stress-neutral manifests; add a manifest-level field-status summary or state the loss report carries it; converge ID-map carrier fields; add a timestamp policy declaration; define the solver-ready condition that makes tbd/unsupported entries blocking; add manifest/profile metadata to harness run records or record the exemption; declare canonical ID families, member path policy and timestamp policy in the native profile. Each item may instead narrow the claim (LOCAL_DESIGN).

## Affected claims

12 claim rows on 2 deliverable(s): DEL-17-02, DEL-17-03.

Classes (portion in this brief / class total): T6-C01 12/180 (Authority NONE).

Reproducing filter: `CODE_FIX_ROWS.csv` where `CFB == "CFB-45"`. Each key below is in `R3/CLASS_ASSIGNMENTS.csv`, `R3/TASKS/T8_ROWS.csv` or `R3/TASKS/T12_UNREACHED.csv` with route CODE_FIX_CANDIDATE, except class T7-C05 and NOT_DIVERGENT rows, which enter through T8 only.

| Key | Class | Authority | BlockedOnPacket | T8/T12 view | Remaining work (effective; OC = OtherCorrections) |
|---|---|---|---|---|---|
| `DEL-17-02:SOW#CLM-009` | T6-C01 | NONE | — | — | Align exporter profiles to the common field list (entity_coverage, target_version_basis, coordinate_policy, stable_id_policy), or narrow the contract list |
| `DEL-17-02:SOW#CLM-010` | T6-C01 | NONE | — | — | Extend ID-map coverage to the listed families as exporters grow, or narrow the 'at least' list |
| `DEL-17-02:SOW#CLM-019/DEL-17-02-REQ-014` | T6-C01 | NONE | C7 | T8 TIER_IN_SCOPE_REQ: NO_ACTION; DISAGREES (T8 NO_ACTION / class CODE_FIX_CANDIDATE) | Add an explicit timestamp policy declaration to the profile or manifest contract fields |
| `DEL-17-02:SOW#CLM-020/DEL-17-02-REQ-020` | T6-C01 | NONE | — | — | Declare target version basis uniformly (a const or TBD for project-owned targets) |
| `DEL-17-02:SOW#CLM-020/DEL-17-02-REQ-021` | T6-C01 | NONE | — | — | Add coordinate policy to MBF and stress-neutral profiles and converge the stable-ID policy field |
| `DEL-17-02:SOW#CLM-020/DEL-17-02-REQ-022` | T6-C01 | NONE | — | — | Add profile-level entity coverage classification or relax REQ-022 to loss-report classification |
| `DEL-17-02:SOW#CLM-021/DEL-17-02-REQ-035` | T6-C01 | NONE | — | — | Converge exporter ID-map carrier fields on the four contract values |
| `DEL-17-02:SOW#CLM-022/DEL-17-02-REQ-040` | T6-C01 | NONE | — | — | Add source_basis_refs to PCF and stress-neutral manifests |
| `DEL-17-02:SOW#CLM-022/DEL-17-02-REQ-042` | T6-C01 | NONE | — | — | Add a manifest-level field-status summary, or state that the loss report carries it |
| `DEL-17-02:SOW#CLM-023/DEL-17-02-REQ-053` | T6-C01 | NONE | C7 | T8 TIER_IN_SCOPE_REQ: NO_ACTION; DISAGREES (T8 NO_ACTION / class CODE_FIX_CANDIDATE) | Define the solver-ready or compatibility-sensitive condition and make matching tbd/unsupported entries blocking |
| `DEL-17-02:SOW#CLM-024/DEL-17-05` | T6-C01 | NONE | — | — | Add manifest and profile metadata to harness run records, or record the harness as outside the manifest contract |
| `DEL-17-03:SOW#CLM-006` | T6-C01 | NONE | — | — | Declare canonical ID families, member path policy and timestamp policy in the native profile, or narrow CLM-006 |

## Evidence

Sealed ledgers (reliability: sealed R2 ledger rows with effective values from adopted resolutions; verified where the wave verifier sampled them, otherwise worker reading):

- DEL-17-02: `projects/chirality-piping/execution/_Reconciliation/DeliverableConcordance/RECON_2026-09-21_WHOLE_CORPUS/WAVES/W3/PKG-17/DEL-17-02/DEL-17-02_forward.csv`; ImplementationEvidence cited: `core/handoff/native_json/package.py`, `core/handoff/caepipe_mbf/package.py`, `core/handoff/pcf_export/package.py`, `core/handoff/review_geometry/package.py`, `core/handoff/stress_neutral/package.py` ….
- DEL-17-03: `projects/chirality-piping/execution/_Reconciliation/DeliverableConcordance/RECON_2026-09-21_WHOLE_CORPUS/WAVES/W3/PKG-17/DEL-17-03/DEL-17-03_forward.csv`; ImplementationEvidence cited: `core/handoff/native_json/package.py`, `fixtures/native_json/invented/native_json_export_package.json`, `schemas/native_json_export.schema.json`.

Freeze line citations in Scope were re-read at the freeze (`00115c719`) by H2 as code reading only; no build or test was run. Classification sources: `R3/TASKS/T4A_CLASSES.md`, `T6_CLASSES.md`, `T7_CLASSES.md`, `T8_CLUSTERS.md`, `T12_UNREACHED.md` (proposals, not accepted results).

## Acceptance checks

- A cross-profile schema test asserts every profile carries the common fields under the contract names.
- Each narrowing choice is recorded as an SOW repair (H4) rather than silently dropped.
- The affected ledger rows are re-verified in a later concordance; no ALIGNED status is claimed from this brief.

## Protected-content status

No protected subject: no affected row is at INVARIANT tier or carries an IP_DATA, CLAIMS or SECURITY layer. 
This brief quotes no protected, private or third-party content. Execution uses invented or synthetic fixtures only and introduces no standards text, tables or equation sources (DEC-043).

## BlockedOnPacket

2 of 12 claim rows carry a block: C7 (2). `H3[<class>]` names the H3 register item for that class (review before repair; mapped in `H3_TOKEN_MAP.csv`). Unblocked rows may proceed separately once selected.

## Notes and open views

- REQ-014 and REQ-053 are CP-11 rows where T8 reads NO_ACTION and the class reads CODE_FIX_CANDIDATE; C7 (scope-item test) confirms which applies.
- The export timestamp/generator policy is a held owner selection (PDU-031; B12 D15), so REQ-014 declares the policy slot, not a new policy.

## Dependencies

C7, B12 (D15), A7 (export plan context).

