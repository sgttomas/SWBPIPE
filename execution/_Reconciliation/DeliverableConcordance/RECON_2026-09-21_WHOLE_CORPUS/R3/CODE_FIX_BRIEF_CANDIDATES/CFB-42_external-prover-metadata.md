# CFB-42 — External-prover metadata from the product surface; comparison link kind

**Candidate brief (H2). Not executed.** Area: External prover boundary (DEL-15-04). Run HELP-HUMAN-PIPING-20260921-RECONCILIATION, R3 integration, TASK H2. Execution, if the owner selects it, goes through an owner-steered production brief and the chirality-change PR path, followed by re-verification in a later concordance (ledger rows are not edited in place). Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).

## Scope

Make the product surface author and emit schema-valid external-prover metadata (the desktop panel builds its own fixed preview packet); add a comparison link kind or state that comparison reports use generic external references.

## Affected claims

5 claim rows on 1 deliverable(s): DEL-15-04.

Classes (portion in this brief / class total): T6-C01 4/180 (Authority NONE); T6-C02 1/116 (Authority NONE).

Reproducing filter: `CODE_FIX_ROWS.csv` where `CFB == "CFB-42"`. Each key below is in `R3/CLASS_ASSIGNMENTS.csv`, `R3/TASKS/T8_ROWS.csv` or `R3/TASKS/T12_UNREACHED.csv` with route CODE_FIX_CANDIDATE, except class T7-C05 and NOT_DIVERGENT rows, which enter through T8 only.

| Key | Class | Authority | BlockedOnPacket | T8/T12 view | Remaining work (effective; OC = OtherCorrections) |
|---|---|---|---|---|---|
| `DEL-15-04:SOW#CLM-004.r01` | T6-C02 | NONE | B7 | — | Make the product surface author and emit schema-valid external-prover metadata. |
| `DEL-15-04:SOW#CLM-004.r02` | T6-C01 | NONE | — | — | Add a comparison link kind or state that comparison reports use generic external references. |
| `DEL-15-04:SOW#CLM-006` | T6-C01 | NONE | — | — | Add a comparison link kind or state that comparison reports use generic external references. |
| `DEL-15-04:SOW#CLM-012.r01` | T6-C01 | NONE | — | — | Add a comparison link kind or state that comparison reports use generic external references. |
| `DEL-15-04:SOW#purpose-and-objective-traceability/OUT-001` | T6-C01 | NONE | B7 | T12 T12-C01: CODE_FIX_CANDIDATE | (none recorded; see Notes in ledger) |

## Evidence

Sealed ledgers (reliability: sealed R2 ledger rows with effective values from adopted resolutions; verified where the wave verifier sampled them, otherwise worker reading):

- DEL-15-04: `projects/chirality-piping/execution/_Reconciliation/DeliverableConcordance/RECON_2026-09-21_WHOLE_CORPUS/WAVES/W3/PKG-15/DEL-15-04/DEL-15-04_forward.csv`; ImplementationEvidence cited: `core/handoff/external_prover/metadata.py`, `schemas/external_prover_metadata.schema.json`, `apps/desktop/src/features/external-prover/ExternalProverBoundaryPanel.tsx`, `core/handoff/external_prover/authority_boundary.py`, `tests/test_external_prover_boundary_metadata.py`.

Freeze line citations in Scope were re-read at the freeze (`00115c719`) by H2 as code reading only; no build or test was run. Classification sources: `R3/TASKS/T4A_CLASSES.md`, `T6_CLASSES.md`, `T7_CLASSES.md`, `T8_CLUSTERS.md`, `T12_UNREACHED.md` (proposals, not accepted results).

## Acceptance checks

- A product-path test emits metadata that validates against `schemas/external_prover_metadata.schema.json`.
- Comparison links are either a distinct kind with a test or documented as generic.
- The affected ledger rows are re-verified in a later concordance; no ALIGNED status is claimed from this brief.

## Protected-content status

No protected subject: no affected row is at INVARIANT tier or carries an IP_DATA, CLAIMS or SECURITY layer. 
This brief quotes no protected, private or third-party content. Execution uses invented or synthetic fixtures only and introduces no standards text, tables or equation sources (DEC-043).

## BlockedOnPacket

2 of 5 claim rows carry a block: B7 (2). `H3[<class>]` names the H3 register item for that class (review before repair). Unblocked rows may proceed separately once selected.

## Notes and open views

- CLM-004.r01 and OUT-001 wait on B7 (prover packet field; T12-C01 names OUT-001). OUT-001 is WEAK: two readings survive. External-prover activation is C3.

## Dependencies

B7, C3 (context).

