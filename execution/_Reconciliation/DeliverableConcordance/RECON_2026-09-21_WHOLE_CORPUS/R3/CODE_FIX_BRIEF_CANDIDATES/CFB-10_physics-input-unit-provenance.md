# CFB-10 — Component and product-physics inputs: unit-against-dimension and redistribution checks

**Candidate brief (H2). Not executed.** Area: Component library and product physics inputs (DEL-03-04, DEL-03-08); protected subject. Run HELP-HUMAN-PIPING-20260921-RECONCILIATION, R3 integration, TASK H2. Execution, if the owner selects it, goes through an owner-steered production brief and the chirality-change PR path, followed by re-verification in a later concordance (ledger rows are not edited in place). Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).

## Scope

Check each input unit against its declared dimension through the DEC-018 catalog instead of string equality; add branch-specific accepted/rejected/absent unit tests on the product path for run size, header size, connection angle and reinforcement area; require redistribution_status in input provenance and propagate input provenance to derived outputs.

## Affected claims

4 claim rows on 2 deliverable(s): DEL-03-04, DEL-03-08.

Classes (portion in this brief / class total): T6-C03 4/34 (Authority REVIEW).

Reproducing filter: `CODE_FIX_ROWS.csv` where `CFB == "CFB-10"`. Each key below is in `R3/CLASS_ASSIGNMENTS.csv`, `R3/TASKS/T8_ROWS.csv` or `R3/TASKS/T12_UNREACHED.csv` with route CODE_FIX_CANDIDATE, except class T7-C05 and NOT_DIVERGENT rows, which enter through T8 only.

| Key | Class | Authority | BlockedOnPacket | T8/T12 view | Remaining work (effective; OC = OtherCorrections) |
|---|---|---|---|---|---|
| `DEL-03-04:SOW#CLM-011/DEL-03-04-RQ-005` | T6-C03 | REVIEW | H3[T6-C03] | — | Add branch-specific unit tests (accepted, rejected and absent units) for run size, header size, connection angle and reinforcement area on the product path, or cite them… |
| `DEL-03-08:SOW#CLM-011/DEL-03-08-RQ-003` | T6-C03 | REVIEW | H3[T6-C03] | — | Check each input unit against its declared dimension (for example through the DEC-018 catalog in core/units) instead of string equality alone. |
| `DEL-03-08:SOW#CLM-011/DEL-03-08-RQ-004` | T6-C03 | REVIEW | H3[T6-C03] | — | Require redistribution_status in input provenance and propagate input provenance to derived outputs, or narrow the requirement. |
| `DEL-03-08:SOW#CLM-026` | T6-C03 | REVIEW | H3[T6-C03] | — | Add a redistribution-status check on inputs, or narrow the principle. |

## Evidence

Sealed ledgers (reliability: sealed R2 ledger rows with effective values from adopted resolutions; verified where the wave verifier sampled them, otherwise worker reading):

- DEL-03-04: `projects/chirality-piping/execution/_Reconciliation/DeliverableConcordance/RECON_2026-09-21_WHOLE_CORPUS/WAVES/W2/PKG-03/DEL-03-04/DEL-03-04_forward.csv`; ImplementationEvidence cited: `schemas/component.schema.yaml`, `core/product_physics/src/validation.rs`, `core/model_operations/operation_applier/src/lib.rs`.
- DEL-03-08: `projects/chirality-piping/execution/_Reconciliation/DeliverableConcordance/RECON_2026-09-21_WHOLE_CORPUS/WAVES/W2/PKG-03/DEL-03-08/DEL-03-08_forward.csv`; ImplementationEvidence cited: `core/section_properties/calculator.py`, `core/units/README.md`.

Freeze line citations in Scope were re-read at the freeze (`00115c719`) by H2 as code reading only; no build or test was run. Classification sources: `R3/TASKS/T4A_CLASSES.md`, `T6_CLASSES.md`, `T7_CLASSES.md`, `T8_CLUSTERS.md`, `T12_UNREACHED.md` (proposals, not accepted results).

## Acceptance checks

- Negative tests: a wrong-dimension unit and an absent unit are each rejected with a diagnostic.
- A test shows a missing redistribution_status is reported and provenance reaches derived outputs.
- Independent review (H3 item for T6-C03) before reliance.
- The affected ledger rows are re-verified in a later concordance; no ALIGNED status is claimed from this brief.

## Protected-content status

Protected subject: 4 of 4 rows are at INVARIANT tier or carry protected layers (BASELINE, IP_DATA, VALIDATION). An independent review of any repair is required before reliance. 
This brief quotes no protected, private or third-party content. Execution uses invented or synthetic fixtures only and introduces no standards text, tables or equation sources (DEC-043).

## BlockedOnPacket

4 of 4 claim rows carry a block: H3[T6-C03] (4). `H3[<class>]` names the H3 register item for that class (review before repair). All rows are blocked.

## Notes and open views

- Narrowing branches (DEL-03-08 CLM-026 and RQ-004 "or narrow") restate an INVARIANT and are not deliverable-local; they are owner items (UNASSIGNED in the H2 return).
- DEL-03-08 ownership against the product-physics owner is B1 (PHYS-007); the fix lands wherever B1 places it.
- CONTESTED: DEL-03-04 RQ-005 (VALIDATION vs BASELINE layer).

## Dependencies

H3[T6-C03], B1.

