# CFB-32 — Constraint and design-knowledge schemas: bind Parameter.value to value_kind quantity; add force_per_length

**Candidate brief (H2). Not executed.** Area: Constraints and design knowledge (DEL-13-01, DEL-13-02). Run HELP-HUMAN-PIPING-20260921-RECONCILIATION, R3 integration, TASK H2. Execution, if the owner selects it, goes through an owner-steered production brief and the chirality-change PR path, followed by re-verification in a later concordance (ledger rows are not edited in place). Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).

## Scope

In the constraint and design-knowledge schemas, require a Quantity object when value_kind is quantity (if/then) and keep text values from carrying bare quantities; add the accepted `force_per_length` to `Quantity.dimension`; make the test compare against the canonical unit vocabulary instead of a local copy.

## Affected claims

10 claim rows on 2 deliverable(s): DEL-13-01, DEL-13-02.

Classes (portion in this brief / class total): T7-C06 10/42 (Authority REVIEW).

Reproducing filter: `CODE_FIX_ROWS.csv` where `CFB == "CFB-32"`. Each key below is in `R3/CLASS_ASSIGNMENTS.csv`, `R3/TASKS/T8_ROWS.csv` or `R3/TASKS/T12_UNREACHED.csv` with route CODE_FIX_CANDIDATE, except class T7-C05 and NOT_DIVERGENT rows, which enter through T8 only.

| Key | Class | Authority | BlockedOnPacket | T8/T12 view | Remaining work (effective; OC = OtherCorrections) |
|---|---|---|---|---|---|
| `DEL-13-01:SOW#CLM-005.r05` | T7-C06 | REVIEW | H3[T7-C06];C7 | T8 UNIT_VOCABULARY: CODE_FIX_CANDIDATE; T12 T12-C03: REVIEW | OC: add force_per_length to Quantity.dimension and to the test set, or derive the test set from units.schema.yaml; verifier: "LOCAL_DESIGN · NO is also defensible if Agent 0… |
| `DEL-13-01:SOW#CLM-009/REQ-13-01-006` | T7-C06 | REVIEW | H3[T7-C06];C7 | T8 UNIT_VOCABULARY: CODE_FIX_CANDIDATE | Tie Parameter.value to value_kind (for example if/then so value_kind 'quantity' requires a Quantity object) and add a negative test. |
| `DEL-13-02:SOW#CLM-003.r13` | T7-C06 | REVIEW | H3[T7-C06];C7 | T8 UNIT_VOCABULARY: CODE_FIX_CANDIDATE | Schema change for the owning loop: bind Parameter.value to the Quantity object when value_kind is quantity (and keep text values from carrying unit-bearing magnitudes),… |
| `DEL-13-02:SOW#CLM-009` | T7-C06 | REVIEW | H3[T7-C06];C7 | T8 UNIT_VOCABULARY: CODE_FIX_CANDIDATE | Schema change for the owning loop: bind Parameter.value to the Quantity object when value_kind is quantity (and keep text values from carrying unit-bearing magnitudes),… |
| `DEL-13-02:SOW#CLM-010/R-13-02-005` | T7-C06 | REVIEW | H3[T7-C06];C7 | T8 UNIT_VOCABULARY: CODE_FIX_CANDIDATE | Schema change for the owning loop: bind Parameter.value to the Quantity object when value_kind is quantity (and keep text values from carrying unit-bearing magnitudes),… |
| `DEL-13-02:SOW#CLM-012/R-13-02-005` | T7-C06 | REVIEW | H3[T7-C06];C7 | T8 UNIT_VOCABULARY: CODE_FIX_CANDIDATE | Schema change for the owning loop: bind Parameter.value to the Quantity object when value_kind is quantity (and keep text values from carrying unit-bearing magnitudes),… |
| `DEL-13-02:SOW#CLM-013` | T7-C06 | REVIEW | H3[T7-C06];C7 | T8 UNIT_VOCABULARY: CODE_FIX_CANDIDATE | Schema change for the owning loop: bind Parameter.value to the Quantity object when value_kind is quantity (and keep text values from carrying unit-bearing magnitudes),… |
| `DEL-13-02:SOW#CLM-018` | T7-C06 | REVIEW | H3[T7-C06];C7 | T8 UNIT_VOCABULARY: CODE_FIX_CANDIDATE | Schema change for the owning loop: bind Parameter.value to the Quantity object when value_kind is quantity (and keep text values from carrying unit-bearing magnitudes),… |
| `DEL-13-02:SOW#CLM-019.r04` | T7-C06 | REVIEW | H3[T7-C06];C7 | T8 UNIT_VOCABULARY: CODE_FIX_CANDIDATE | Schema change for the owning loop: bind Parameter.value to the Quantity object when value_kind is quantity (and keep text values from carrying unit-bearing magnitudes),… |
| `DEL-13-02:SOW#CLM-024` | T7-C06 | REVIEW | H3[T7-C06];C7 | T8 UNIT_VOCABULARY: CODE_FIX_CANDIDATE | Schema change for the owning loop: bind Parameter.value to the Quantity object when value_kind is quantity (and keep text values from carrying unit-bearing magnitudes),… |

## Evidence

Sealed ledgers (reliability: sealed R2 ledger rows with effective values from adopted resolutions; verified where the wave verifier sampled them, otherwise worker reading):

- DEL-13-01: `projects/chirality-piping/execution/_Reconciliation/DeliverableConcordance/RECON_2026-09-21_WHOLE_CORPUS/WAVES/W3/PKG-13/DEL-13-01/DEL-13-01_forward.csv`; ImplementationEvidence cited: `schemas/design_knowledge.schema.json`, `schemas/units.schema.yaml`.
- DEL-13-02: `projects/chirality-piping/execution/_Reconciliation/DeliverableConcordance/RECON_2026-09-21_WHOLE_CORPUS/WAVES/W3/PKG-13/DEL-13-02/DEL-13-02_forward.csv`; ImplementationEvidence cited: `schemas/constraint.schema.json#L354`, `schemas/constraint.schema.json#L495`, `schemas/units.schema.yaml#L247`, `projects/chirality-piping/docs/SPEC.md#L172`, `schemas/constraint.schema.json` ….

Freeze line citations in Scope were re-read at the freeze (`00115c719`) by H2 as code reading only; no build or test was run. Classification sources: `R3/TASKS/T4A_CLASSES.md`, `T6_CLASSES.md`, `T7_CLASSES.md`, `T8_CLUSTERS.md`, `T12_UNREACHED.md` (proposals, not accepted results).

## Acceptance checks

- Negative tests: value_kind quantity with a bare string or number fails validation.
- A force_per_length quantity validates.
- The vocabulary test imports the canonical vocabulary.
- Independent review (H3 item for T7-C06).
- The affected ledger rows are re-verified in a later concordance; no ALIGNED status is claimed from this brief.

## Protected-content status

Protected subject: 8 of 10 rows are at INVARIANT tier or carry protected layers (BASELINE). An independent review of any repair is required before reliance. 
This brief quotes no protected, private or third-party content. Execution uses invented or synthetic fixtures only and introduces no standards text, tables or equation sources (DEC-043).

## BlockedOnPacket

10 of 10 claim rows carry a block: C7 (10); H3[T7-C06] (10). `H3[<class>]` names the H3 register item for that class (review before repair). All rows are blocked.

## Notes and open views

- CONTESTED (8 DEL-13-02 rows): tier/disposition/AuthorityNeeded are settled corpus-wide in the unit-vocabulary cluster; T8 K3 reads CODE_FIX with REVIEW. C7 carries "unit-vocabulary tier follows the gap" for owner confirmation; if settled as an owner item these rows move to an owner route.
- DEL-13-01 CLM-005.r05 FIRM sets REVIEW; LOCAL_DESIGN/NO was also judged defensible. DEL-13-01 CLM-009 carries a T8 tier observation (INVARIANT vs effective PROJECT_BASELINE).
- The pending human disposition of PKG13-DEL-13-02-PKG02-001 is part of the DEL-13-02 RemainingWork (a review act).

## Dependencies

C7, H3[T7-C06], CFB-04.

