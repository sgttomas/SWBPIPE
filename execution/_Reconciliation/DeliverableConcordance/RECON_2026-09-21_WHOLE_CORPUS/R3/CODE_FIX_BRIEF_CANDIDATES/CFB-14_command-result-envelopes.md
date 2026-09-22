# CFB-14 — Storage, rule-pack and library commands: diagnostics envelope conformance

**Candidate brief (H2). Not executed.** Area: Desktop command layer (DEL-00-03, DEL-00-06, DEL-06-04). Run HELP-HUMAN-PIPING-20260921-RECONCILIATION, R3 integration, TASK H2. Execution, if the owner selects it, goes through an owner-steered production brief and the chirality-change PR path, followed by re-verification in a later concordance (ledger rows are not edited in place). Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).

## Scope

Conform storage, rule-pack and library commands to the schema-first result/diagnostics envelope (they return typed receipts or a bare error string at the freeze), per AB-00-06 REQ-06-03 and DEL-00-03 REQ-03-02.

## Affected claims

4 claim rows on 3 deliverable(s): DEL-00-03, DEL-00-06, DEL-06-04.

Classes (portion in this brief / class total): T6-C02 4/116 (Authority NONE).

Reproducing filter: `CODE_FIX_ROWS.csv` where `CFB == "CFB-14"`. Each key below is in `R3/CLASS_ASSIGNMENTS.csv`, `R3/TASKS/T8_ROWS.csv` or `R3/TASKS/T12_UNREACHED.csv` with route CODE_FIX_CANDIDATE, except class T7-C05 and NOT_DIVERGENT rows, which enter through T8 only.

| Key | Class | Authority | BlockedOnPacket | T8/T12 view | Remaining work (effective; OC = OtherCorrections) |
|---|---|---|---|---|---|
| `DEL-00-03:AB#normative-requirements/REQ-03-02` | T6-C02 | NONE | — | — | Decide whether storage, rule-pack and library commands must return the diagnostics envelope; they now return typed receipts or a bare error string. |
| `DEL-00-06:AB#normative-requirements/REQ-06-03` | T6-C02 | NONE | — | — | OC: decide whether storage, rule-pack and library commands must return the diagnostics envelope, or narrow REQ-06-03 |
| `DEL-06-04:SOW#CLM-004.r05` | T6-C02 | NONE | — | — | Decide whether rule-pack commands must return the schema-first result envelope, then conform them or narrow the claim. |
| `DEL-06-04:SOW#CLM-010/R-06-04-012` | T6-C02 | NONE | B10 | — | Decide whether rule-pack commands must return the schema-first envelope; bind any future rule-pack plugin execution to a governed dispatch path. |

## Evidence

Sealed ledgers (reliability: sealed R2 ledger rows with effective values from adopted resolutions; verified where the wave verifier sampled them, otherwise worker reading):

- DEL-00-03: `projects/chirality-piping/execution/_Reconciliation/DeliverableConcordance/RECON_2026-09-21_WHOLE_CORPUS/WAVES/W2/PKG-00/DEL-00-03/DEL-00-03_forward.csv`; ImplementationEvidence cited: `schemas/operation_outcome.schema.json`, `core/model_operations/operation_applier/src/lib.rs`, `schemas/headless_runner.schema.yaml`, `apps/desktop/src-tauri/src/lib.rs`.
- DEL-00-06: `projects/chirality-piping/execution/_Reconciliation/DeliverableConcordance/RECON_2026-09-21_WHOLE_CORPUS/WAVES/W2/PKG-00/DEL-00-06/DEL-00-06_forward.csv`; ImplementationEvidence cited: `projects/chirality-piping/schemas/operation_outcome.schema.json`, `projects/chirality-piping/schemas/results.v0.2.schema.yaml`, `projects/chirality-piping/schemas/rule_check_run_result.schema.json`, `projects/chirality-piping/schemas/headless_runner.schema.yaml`, `projects/chirality-piping/schemas/report_generator.schema.yaml` ….
- DEL-06-04: `projects/chirality-piping/execution/_Reconciliation/DeliverableConcordance/RECON_2026-09-21_WHOLE_CORPUS/WAVES/W3/PKG-06/DEL-06-04/DEL-06-04_forward.csv`; ImplementationEvidence cited: `schemas/rule_pack.schema.yaml`, `core/rules/rule_pack_document/src/lib.rs`, `apps/desktop/src-tauri/src/lib.rs`.

Freeze line citations in Scope were re-read at the freeze (`00115c719`) by H2 as code reading only; no build or test was run. Classification sources: `R3/TASKS/T4A_CLASSES.md`, `T6_CLASSES.md`, `T7_CLASSES.md`, `T8_CLUSTERS.md`, `T12_UNREACHED.md` (proposals, not accepted results).

## Acceptance checks

- Each storage, rule-pack and library command returns the envelope, with a contract test per command family.
- Error paths return envelope diagnostics, not bare strings.
- The affected ledger rows are re-verified in a later concordance; no ALIGNED status is claimed from this brief.

## Protected-content status

No protected subject: no affected row is at INVARIANT tier or carries an IP_DATA, CLAIMS or SECURITY layer. 
This brief quotes no protected, private or third-party content. Execution uses invented or synthetic fixtures only and introduces no standards text, tables or equation sources (DEC-043).

## BlockedOnPacket

1 of 4 claim rows carry a block: B10 (1). `H3[<class>]` names the H3 register item for that class (review before repair). Unblocked rows may proceed separately once selected.

## Notes and open views

- Owner alternative (FIRM OtherCorrections on DEL-00-06 REQ-06-03): exempt the storage commands from the envelope, which makes AuthorityNeeded OWNER. No packet carries this alternative; it is reported UNASSIGNED.
- DEL-06-04 R-06-04-012 also names binding future rule-pack plugin execution to a governed dispatch path (B10).

## Dependencies

B10 (R-06-04-012 part).

