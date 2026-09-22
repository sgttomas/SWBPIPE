# CFB-40 — Target-mapping contract: remove silent defaults, carry caller provenance, check unit dimensions

**Candidate brief (H2). Not executed.** Area: Handoff target mapping (DEL-15-02). Run HELP-HUMAN-PIPING-20260921-RECONCILIATION, R3 integration, TASK H2. Execution, if the owner selects it, goes through an owner-steered production brief and the chirality-change PR path, followed by re-verification in a later concordance (ledger rows are not edited in place). Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).

## Scope

Stop `build_target_mapping_contract` from defaulting absent mapping_status (freeze `projects/chirality-piping/core/handoff/target_mapping/contract.py:239` defaults to "mapped"), mapping_kind, value_kind, refs and provenance, or emit an explicit diagnostic for each absence; carry caller provenance instead of substituting engine provenance and allow reference-level provenance/review status; add a unit-against-dimension check and a diagnostic for unit-bearing values under an omitted or unrecognised value_kind; add provenance checks and a validation-plan record.

## Affected claims

17 claim rows on 1 deliverable(s): DEL-15-02.

Classes (portion in this brief / class total): T6-C01 1/180 (Authority NONE); T6-C03 3/34 (Authority REVIEW); T7-C06 13/42 (Authority REVIEW).

Reproducing filter: `CODE_FIX_ROWS.csv` where `CFB == "CFB-40"`. Each key below is in `R3/CLASS_ASSIGNMENTS.csv`, `R3/TASKS/T8_ROWS.csv` or `R3/TASKS/T12_UNREACHED.csv` with route CODE_FIX_CANDIDATE, except class T7-C05 and NOT_DIVERGENT rows, which enter through T8 only.

| Key | Class | Authority | BlockedOnPacket | T8/T12 view | Remaining work (effective; OC = OtherCorrections) |
|---|---|---|---|---|---|
| `DEL-15-02:CONTEXT#context-envelope` | T7-C06 | REVIEW | H3[T7-C06];B7 | T12 T12-C01: CODE_FIX_CANDIDATE | As CLM-005.r02. |
| `DEL-15-02:MEMORY` | T7-C06 | REVIEW | H3[T7-C06];B7 | — | As CLM-005.r02; then correct the Boundary Decisions wording. |
| `DEL-15-02:SOW#CLM-005.r01` | T6-C03 | REVIEW | H3[T6-C03];B7;C6 | T8 UNIT_VOCABULARY: CODE_FIX_CANDIDATE; T12 T12-C01: CODE_FIX_CANDIDATE | Add a dimensional-consistency check (unit against declared dimension) and a diagnostic for unit-bearing values whose value_kind is omitted or unrecognised; test both. |
| `DEL-15-02:SOW#CLM-005.r02` | T7-C06 | REVIEW | H3[T7-C06];B7;C6 | T12 T12-C01: CODE_FIX_CANDIDATE | Stop defaulting absent mapping_status, mapping_kind, value_kind, refs and provenance, or emit an explicit diagnostic for each absence; add negative tests for defaulted r… |
| `DEL-15-02:SOW#CLM-005.r03` | T7-C06 | REVIEW | H3[T7-C06];B7;C6 | T12 T12-C01: CODE_FIX_CANDIDATE | Carry caller provenance or emit a diagnostic when it is absent rather than substituting engine provenance; allow provenance/review status on reliance-affecting reference… |
| `DEL-15-02:SOW#CLM-011.r05` | T6-C03 | REVIEW | H3[T6-C03];B7 | T12 T12-C01: CODE_FIX_CANDIDATE | As CLM-005.r01: dimensional-consistency check plus a diagnostic for unit-bearing values under an omitted or unrecognised value_kind. |
| `DEL-15-02:SOW#CLM-011.r06` | T7-C06 | REVIEW | H3[T7-C06];B7 | T12 T12-C01: CODE_FIX_CANDIDATE | As CLM-005.r03. |
| `DEL-15-02:SOW#CLM-011.r07` | T7-C06 | REVIEW | H3[T7-C06];B7 | T12 T12-C01: CODE_FIX_CANDIDATE | As CLM-005.r02. |
| `DEL-15-02:SOW#CLM-013/V-004` | T6-C01 | NONE | B7 | — | Add provenance checks (supplied provenance preserved; absent provenance reported) and a validation-plan record, or narrow V-004. |
| `DEL-15-02:SOW#CLM-020` | T7-C06 | REVIEW | H3[T7-C06];B7 | T12 T12-C01: CODE_FIX_CANDIDATE | (none recorded; see Notes in ledger) |
| `DEL-15-02:SOW#CLM-020.s01` | T7-C06 | REVIEW | H3[T7-C06];B7 | T12 T12-C01: CODE_FIX_CANDIDATE | As CLM-005.r02. |
| `DEL-15-02:SOW#CLM-021.r03` | T7-C06 | REVIEW | H3[T7-C06];B7 | T12 T12-C01: CODE_FIX_CANDIDATE | As CLM-005.r02. |
| `DEL-15-02:SOW#CLM-026` | T7-C06 | REVIEW | H3[T7-C06];B7 | T12 T12-C01: CODE_FIX_CANDIDATE | As CLM-005.r02. |
| `DEL-15-02:SOW#CLM-027.r01` | T7-C06 | REVIEW | H3[T7-C06];B7 | T12 T12-C01: CODE_FIX_CANDIDATE | As CLM-005.r02. |
| `DEL-15-02:SOW#CLM-027.r02` | T6-C03 | REVIEW | H3[T6-C03];B7 | T12 T12-C01: CODE_FIX_CANDIDATE | As CLM-005.r01. |
| `DEL-15-02:SOW#CLM-027.r03` | T7-C06 | REVIEW | H3[T7-C06];B7 | T12 T12-C01: CODE_FIX_CANDIDATE | As CLM-005.r03. |
| `DEL-15-02:SOW#completion-and-reliance-basis-epistemology/AC-001` | T7-C06 | REVIEW | H3[T7-C06];B7 | T12 T12-C01: CODE_FIX_CANDIDATE | Close FG-DEL-15-02-01 (and FG-02, FG-03); refresh the package-container clause to DEC-028. |

## Evidence

Sealed ledgers (reliability: sealed R2 ledger rows with effective values from adopted resolutions; verified where the wave verifier sampled them, otherwise worker reading):

- DEL-15-02: `projects/chirality-piping/execution/_Reconciliation/DeliverableConcordance/RECON_2026-09-21_WHOLE_CORPUS/WAVES/W3/PKG-15/DEL-15-02/DEL-15-02_forward.csv`; ImplementationEvidence cited: `projects/chirality-piping/docs/_Registers/Deliverables.csv`, `core/handoff/target_mapping/contract.py#L231`, `projects/chirality-piping/execution/PKG-15_Handoff and External Prover Workflow/1_Working/DEL-15-02_Target mapping and unsupported-behavior contract/MEMORY.md`, `core/handoff/target_mapping/contract.py#L274`, `core/handoff/target_mapping/contract.py#L44` ….

Freeze line citations in Scope were re-read at the freeze (`00115c719`) by H2 as code reading only; no build or test was run. Classification sources: `R3/TASKS/T4A_CLASSES.md`, `T6_CLASSES.md`, `T7_CLASSES.md`, `T8_CLUSTERS.md`, `T12_UNREACHED.md` (proposals, not accepted results).

## Acceptance checks

- Negative tests: each absent input yields a diagnostic, not a default.
- A test shows supplied caller provenance is preserved and absent provenance is reported.
- A wrong-dimension unit is rejected.
- Independent review (H3 items for T7-C06 and T6-C03).
- The affected ledger rows are re-verified in a later concordance; no ALIGNED status is claimed from this brief.

## Protected-content status

Protected subject: 16 of 17 rows are at INVARIANT tier or carry protected layers (IP_DATA, VALIDATION). An independent review of any repair is required before reliance. 
This brief quotes no protected, private or third-party content. Execution uses invented or synthetic fixtures only and introduces no standards text, tables or equation sources (DEC-043).

## BlockedOnPacket

17 of 17 claim rows carry a block: B7 (17); C6 (3); H3[T6-C03] (3); H3[T7-C06] (13). `H3[<class>]` names the H3 register item for that class (review before repair). All rows are blocked.

## Notes and open views

- The topic file places the DEL-15-02 defaults behind B7 (canonical handoff path): all rows are PRODUCT_CALLER: NONE; the function is imported only by tests (T12-C01).
- The AuthorityTier of the CLM-005 Units, Missing-values and Provenance rows is contested evidence (W3 departure 5, the rerun-launch hint); C6 carries it.
- FIRM on CLM-020: re-dispose the block row, or split step 2 as a new sub-claim (record choice for H4).

## Dependencies

B7, C6, H3[T7-C06], H3[T6-C03].

