# CFB-11 — Import gate: reject bare numeric values for unit-bearing fields

**Candidate brief (H2). Not executed.** Area: Import gate (DEL-03-07). Run HELP-HUMAN-PIPING-20260921-RECONCILIATION, R3 integration, TASK H2. Execution, if the owner selects it, goes through an owner-steered production brief and the chirality-change PR path, followed by re-verification in a later concordance (ledger rows are not edited in place). Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).

## Scope

If C1 rules that the import gate must reject bare numerics, make the gate reject (or schema-validate) unitless values in material, section and component payloads, with engineering input on which fields are legitimately dimensionless.

## Affected claims

3 claim rows on 1 deliverable(s): DEL-03-07.

Classes (portion in this brief / class total): T7-C05 3/27 (Authority OWNER).

Reproducing filter: `CODE_FIX_ROWS.csv` where `CFB == "CFB-11"`. Each key below is in `R3/CLASS_ASSIGNMENTS.csv`, `R3/TASKS/T8_ROWS.csv` or `R3/TASKS/T12_UNREACHED.csv` with route CODE_FIX_CANDIDATE, except class T7-C05 and NOT_DIVERGENT rows, which enter through T8 only.

| Key | Class | Authority | BlockedOnPacket | T8/T12 view | Remaining work (effective; OC = OtherCorrections) |
|---|---|---|---|---|---|
| `DEL-03-07:SOW#CLM-003.r06` | T7-C05 | OWNER | C1 | T8 UNIT_VOCABULARY: CODE_FIX_CANDIDATE; DISAGREES (T8 CODE_FIX_CANDIDATE / class OWNER_DECISION) | Decide whether the import gate must also reject bare numeric values (or schema-validate material/section/component payloads) so no unitless value is accepted. |
| `DEL-03-07:SOW#CLM-009.r05` | T7-C05 | OWNER | C1 | T8 UNIT_VOCABULARY: CODE_FIX_CANDIDATE; DISAGREES (T8 CODE_FIX_CANDIDATE / class OWNER_DECISION) | As FG-DEL-03-07-01: decide whether bare numeric values must be rejected at the import gate. |
| `DEL-03-07:SOW#CLM-021.s02` | T7-C05 | OWNER | C1 | T8 UNIT_VOCABULARY: CODE_FIX_CANDIDATE; DISAGREES (T8 CODE_FIX_CANDIDATE / class OWNER_DECISION) | As FG-DEL-03-07-01: decide whether bare numeric values must be rejected at the import gate. |

## Evidence

Sealed ledgers (reliability: sealed R2 ledger rows with effective values from adopted resolutions; verified where the wave verifier sampled them, otherwise worker reading):

- DEL-03-07: `projects/chirality-piping/execution/_Reconciliation/DeliverableConcordance/RECON_2026-09-21_WHOLE_CORPUS/WAVES/W2/PKG-03/DEL-03-07/DEL-03-07_forward.csv`; ImplementationEvidence cited: `core/library_import/provenance_checker.py`, `core/library_import/library_import_document/src/lib.rs`.

Freeze line citations in Scope were re-read at the freeze (`00115c719`) by H2 as code reading only; no build or test was run. Classification sources: `R3/TASKS/T4A_CLASSES.md`, `T6_CLASSES.md`, `T7_CLASSES.md`, `T8_CLUSTERS.md`, `T12_UNREACHED.md` (proposals, not accepted results).

## Acceptance checks

- Negative test: a bare numeric for a unit-bearing field is rejected at the import gate.
- The list of legitimately dimensionless fields is recorded with its engineering source.
- The affected ledger rows are re-verified in a later concordance; no ALIGNED status is claimed from this brief.

## Protected-content status

Protected subject: 3 of 3 rows are at INVARIANT tier or carry protected layers (BASELINE). An independent review of any repair is required before reliance. 
This brief quotes no protected, private or third-party content. Execution uses invented or synthetic fixtures only and introduces no standards text, tables or equation sources (DEC-043).

## BlockedOnPacket

3 of 3 claim rows carry a block: C1 (3). `H3[<class>]` names the H3 register item for that class (review before repair). All rows are blocked.

## Notes and open views

- Two views (T8_ROUTE_DISAGREEMENTS): class T7-C05 routes OWNER_DECISION (C1, rule on intent); T8 K3 reads CODE_FIX_CANDIDATE with AuthorityNeeded REVIEW. Nothing proceeds until C1.

## Dependencies

C1, C7 (unit-vocabulary reading).

