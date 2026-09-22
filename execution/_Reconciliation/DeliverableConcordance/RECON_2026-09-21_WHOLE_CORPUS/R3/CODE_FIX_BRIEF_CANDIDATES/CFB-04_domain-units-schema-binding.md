# CFB-04 — Load and unit schema binding to the DEC-018 units catalog

**Candidate brief (H2). Not executed.** Area: Domain schemas and units (PKG-02: DEL-02-01, DEL-02-02). Run HELP-HUMAN-PIPING-20260921-RECONCILIATION, R3 integration, TASK H2. Execution, if the owner selects it, goes through an owner-steered production brief and the chirality-change PR path, followed by re-verification in a later concordance (ledger rows are not edited in place). Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).

## Scope

Bind schema-level unit strings to the `core/units` catalog (or validate unit/dimension consistency at the schema boundary); remove the remaining literal DimensionId sets named in the DEL-02-02 CLM-020 FIRM correction; add a provenance manifest for load fixtures and refresh stored fixtures to the typed LoadRecord shape; update SPEC section 4 so it no longer gates conversion constants that DEC-018 accepted, and add schema-location and diagnostic-code decision records once those are decided.

## Affected claims

4 claim rows on 2 deliverable(s): DEL-02-01, DEL-02-02.

Classes (portion in this brief / class total): T6-C01 3/180 (Authority NONE); T6-C02 1/116 (Authority NONE).

Reproducing filter: `CODE_FIX_ROWS.csv` where `CFB == "CFB-04"`. Each key below is in `R3/CLASS_ASSIGNMENTS.csv`, `R3/TASKS/T8_ROWS.csv` or `R3/TASKS/T12_UNREACHED.csv` with route CODE_FIX_CANDIDATE, except class T7-C05 and NOT_DIVERGENT rows, which enter through T8 only.

| Key | Class | Authority | BlockedOnPacket | T8/T12 view | Remaining work (effective; OC = OtherCorrections) |
|---|---|---|---|---|---|
| `DEL-02-01:SOW#CLM-019` | T6-C01 | NONE | — | — | Add a provenance manifest for the fixtures/domain examples and refresh the stored fixtures to the typed LoadRecord shape so they validate as committed; reword 'eventual… |
| `DEL-02-02:SOW#CLM-014/U-001` | T6-C02 | NONE | — | — | Bind schema-level unit strings to the core/units catalog (or validate unit/dimension consistency at the schema boundary) and trace the remaining import/export paths. |
| `DEL-02-02:SOW#CLM-018` | T6-C01 | NONE | B12 | — | Add the schema-location and diagnostic-code decision records when decided, and update SPEC section 4 so it no longer gates conversion constants that DEC-018 accepted. |
| `DEL-02-02:SOW#CLM-020` | T6-C01 | NONE | — | — | OC: literal DimensionId sets remain in `core/gui/pkg02_boundary.py:10`, `core/constraints/validation/engine.py:64`, `core/model_transform/physical_to_analytical/contract.py:34`; derive them from the units schema vocabulary. |

## Evidence

Sealed ledgers (reliability: sealed R2 ledger rows with effective values from adopted resolutions; verified where the wave verifier sampled them, otherwise worker reading):

- DEL-02-01: `projects/chirality-piping/execution/_Reconciliation/DeliverableConcordance/RECON_2026-09-21_WHOLE_CORPUS/WAVES/W2/PKG-02/DEL-02-01/DEL-02-01_forward.csv`; ImplementationEvidence cited: `schemas/model.schema.yaml`, `fixtures/domain/invented_minimal_project_model.json`, `fixtures/domain/invented_physical_source_of_truth_model.json`.
- DEL-02-02: `projects/chirality-piping/execution/_Reconciliation/DeliverableConcordance/RECON_2026-09-21_WHOLE_CORPUS/WAVES/W2/PKG-02/DEL-02-02/DEL-02-02_forward.csv`; ImplementationEvidence cited: `core/units/src/lib.rs`, `core/product_physics/src/lib.rs`, `core/model_operations/operation_applier/src/lib.rs`, `core/rules/rule_check_runner/src/lib.rs`, `schemas/model.schema.yaml` ….

Freeze line citations in Scope were re-read at the freeze (`00115c719`) by H2 as code reading only; no build or test was run. Classification sources: `R3/TASKS/T4A_CLASSES.md`, `T6_CLASSES.md`, `T7_CLASSES.md`, `T8_CLUSTERS.md`, `T12_UNREACHED.md` (proposals, not accepted results).

## Acceptance checks

- A negative test shows a unit string outside the catalog, or inconsistent with its dimension, is rejected at the schema boundary.
- No literal DimensionId set remains outside the units vocabulary module (the FIRM correction names three files).
- Committed load fixtures validate against the typed LoadRecord schema and carry a provenance manifest.
- The affected ledger rows are re-verified in a later concordance; no ALIGNED status is claimed from this brief.

## Protected-content status

No protected subject: no affected row is at INVARIANT tier or carries an IP_DATA, CLAIMS or SECURITY layer. 
This brief quotes no protected, private or third-party content. Execution uses invented or synthetic fixtures only and introduces no standards text, tables or equation sources (DEC-043).

## BlockedOnPacket

1 of 4 claim rows carry a block: B12 (1). `H3[<class>]` names the H3 register item for that class (review before repair; mapped in `H3_TOKEN_MAP.csv`). Unblocked rows may proceed separately once selected.

## Notes and open views

- DEL-02-02:SOW#CLM-018 waits on the unit/diagnostic-code namespace selection held in B12 (T6-C04 D6).

## Dependencies

B12 (D6), CFB-32 (same catalog).

