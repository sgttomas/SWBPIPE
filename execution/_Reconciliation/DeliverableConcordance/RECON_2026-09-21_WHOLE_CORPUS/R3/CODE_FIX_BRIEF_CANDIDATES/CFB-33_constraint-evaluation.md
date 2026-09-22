# CFB-33 — Constraint conflict evaluation and product caller for the validation engine

**Candidate brief (H2). Not executed.** Area: Constraints (DEL-13-03). Run HELP-HUMAN-PIPING-20260921-RECONCILIATION, R3 integration, TASK H2. Execution, if the owner selects it, goes through an owner-steered production brief and the chirality-change PR path, followed by re-verification in a later concordance (ledger rows are not edited in place). Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).

## Scope

Implement conflict evaluation (connectivity, clearance, route/no-go, support zone, slope/drain/vent) over supplied geometry and user criteria; give `validate_constraint_envelope` a product caller or record the library reading (PRODUCT_CALLER: NONE at the freeze).

## Affected claims

2 claim rows on 1 deliverable(s): DEL-13-03.

Classes (portion in this brief / class total): T6-C01 1/180 (Authority NONE); T6-C02 1/116 (Authority NONE).

Reproducing filter: `CODE_FIX_ROWS.csv` where `CFB == "CFB-33"`. Each key below is in `R3/CLASS_ASSIGNMENTS.csv`, `R3/TASKS/T8_ROWS.csv` or `R3/TASKS/T12_UNREACHED.csv` with route CODE_FIX_CANDIDATE, except class T7-C05 and NOT_DIVERGENT rows, which enter through T8 only.

| Key | Class | Authority | BlockedOnPacket | T8/T12 view | Remaining work (effective; OC = OtherCorrections) |
|---|---|---|---|---|---|
| `DEL-13-03:SOW#CLM-005.r04` | T6-C01 | NONE | B9;A1 | T12 T12-C03: OWNER_DECISION | (none recorded; see Notes in ledger) |
| `DEL-13-03:SOW#CLM-013.r01` | T6-C02 | NONE | B9;A1 | — | Implement conflict evaluation (connectivity, clearance, route/no-go, support-zone, slope/drain/vent) over supplied geometry and user criteria, and wire the engine to a p… |

## Evidence

Sealed ledgers (reliability: sealed R2 ledger rows with effective values from adopted resolutions; verified where the wave verifier sampled them, otherwise worker reading):

- DEL-13-03: `projects/chirality-piping/execution/_Reconciliation/DeliverableConcordance/RECON_2026-09-21_WHOLE_CORPUS/WAVES/W3/PKG-13/DEL-13-03/DEL-13-03_forward.csv`; ImplementationEvidence cited: `core/constraints/validation/engine.py`.

Freeze line citations in Scope were re-read at the freeze (`00115c719`) by H2 as code reading only; no build or test was run. Classification sources: `R3/TASKS/T4A_CLASSES.md`, `T6_CLASSES.md`, `T7_CLASSES.md`, `T8_CLUSTERS.md`, `T12_UNREACHED.md` (proposals, not accepted results).

## Acceptance checks

- Invented-fixture tests for each conflict family, positive and negative.
- A product path calls the validation engine, or the record states library-only status.
- The affected ledger rows are re-verified in a later concordance; no ALIGNED status is claimed from this brief.

## Protected-content status

No protected subject: no affected row is at INVARIANT tier or carries an IP_DATA, CLAIMS or SECURITY layer. 
This brief quotes no protected, private or third-party content. Execution uses invented or synthetic fixtures only and introduces no standards text, tables or equation sources (DEC-043).

## BlockedOnPacket

2 of 2 claim rows carry a block: A1 (2); B9 (2). `H3[<class>]` names the H3 register item for that class (review before repair; mapped in `H3_TOKEN_MAP.csv`). All rows are blocked.

## Notes and open views

- CLM-005.r04 is WEAK: the engine reading gives ALIGNED; only the F7/C6(a) path reading gives this remainder.
- The runtime constraint stage is CFB-43.
- Both rows carry B9: the product-caller-or-library choice is B9 option (a) against (b), and the T12-C03 reading of CLM-005.r04 is OWNER_DECISION. Both also carry A1, because the constraint engine is Python (`core/constraints/validation/engine.py`) and B9 option (a) needs A1 if the engines stay Python.

## Dependencies

B9, A1, CFB-43.

