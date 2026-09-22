# CFB-52 — PCF profile: per-family classification and component mappings beyond straight pipe

**Candidate brief (H2). Not executed.** Area: PCF export (DEL-17-07). Run HELP-HUMAN-PIPING-20260921-RECONCILIATION, R3 integration, TASK H2. Execution, if the owner selects it, goes through an owner-steered production brief and the chirality-change PR path, followed by re-verification in a later concordance (ledger rows are not edited in place). Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).

## Scope

Add per-family classification to the PCF profile for every plan-listed candidate family (or record the loss report as the carrier); implement component mappings (bends, tees, reducers, flanges, valves) with approximation classification, or record their exclusion; add ambiguous/missing-unit and coordinate-policy tests; document fixture provenance and a per-class support table; extend the sidecar to omitted entities and loss rows or record the linkage.

## Affected claims

11 claim rows on 1 deliverable(s): DEL-17-07.

Classes (portion in this brief / class total): T6-C01 11/180 (Authority NONE).

Reproducing filter: `CODE_FIX_ROWS.csv` where `CFB == "CFB-52"`. Each key below is in `R3/CLASS_ASSIGNMENTS.csv`, `R3/TASKS/T8_ROWS.csv` or `R3/TASKS/T12_UNREACHED.csv` with route CODE_FIX_CANDIDATE, except class T7-C05 and NOT_DIVERGENT rows, which enter through T8 only.

| Key | Class | Authority | BlockedOnPacket | T8/T12 view | Remaining work (effective; OC = OtherCorrections) |
|---|---|---|---|---|---|
| `DEL-17-07:CONTEXT#description` | T6-C01 | NONE | A7 | — | Extend the exporter beyond the straight-pipe slice per the profile classification. |
| `DEL-17-07:SOW#CLM-005` | T6-C01 | NONE | — | — | Add a per-family classification record to the PCF profile (or record that the loss report is the classification carrier) and reconcile the ordering clause with the lande… |
| `DEL-17-07:SOW#CLM-014/DEL-17-07-REQ-013` | T6-C01 | NONE | A7 | — | Add per-family classification to the profile record for every plan-listed candidate family. |
| `DEL-17-07:SOW#CLM-016/DEL-17-07-REQ-034` | T6-C01 | NONE | C7 | T8 TIER_IN_SCOPE_REQ: NO_ACTION; DISAGREES (T8 NO_ACTION / class CODE_FIX_CANDIDATE) | Implement component mappings (bends, tees, reducers, flanges, valves) with approximation classification, or record their exclusion in the profile. |
| `DEL-17-07:SOW#CLM-019` | T6-C01 | NONE | — | — | Add ambiguous/missing-unit negative tests and a coordinate-policy test once the profile is selected. |
| `DEL-17-07:SOW#CLM-020` | T6-C01 | NONE | — | — | Convert the remaining coverage classes (units, coordinates and vertical axis, material provenance, component approximations) into tests. |
| `DEL-17-07:SOW#CLM-021` | T6-C01 | NONE | C5 | — | Document fixture provenance and a per-class support table for the PCF profile. |
| `DEL-17-07:SOW#CLM-028` | T6-C01 | NONE | — | — | Add per-family classification to the profile. |
| `DEL-17-07:SOW#CLM-029` | T6-C01 | NONE | A7 | — | Represent each plan-listed family separately in the profile. |
| `DEL-17-07:SOW#CLM-040` | T6-C01 | NONE | — | — | Extend the sidecar to omitted entities and loss rows, or record that loss affected_refs and diagnostic affected_object are the link. |
| `DEL-17-07:SOW#purpose-and-objective-traceability/OUT-001` | T6-C01 | NONE | A7 | — | Extend mapping boundaries to the plan-listed families beyond straight pipe. |

## Evidence

Sealed ledgers (reliability: sealed R2 ledger rows with effective values from adopted resolutions; verified where the wave verifier sampled them, otherwise worker reading):

- DEL-17-07: `projects/chirality-piping/execution/_Reconciliation/DeliverableConcordance/RECON_2026-09-21_WHOLE_CORPUS/WAVES/W3/PKG-17/DEL-17-07/DEL-17-07_forward.csv`; ImplementationEvidence cited: `core/handoff/pcf_export/package.py`, `apps/desktop/src/features/pcf-export/PcfExportPanel.tsx`, `schemas/pcf_export.schema.json`, `fixtures/pcf_export/invented/source_pcf_payload.json`, `projects/chirality-piping/execution/PKG-17_Export Format Interoperability/1_Working/DEL-17-07_Conservative PCF subset exporter/ScopeOfWork.md` ….

Freeze line citations in Scope were re-read at the freeze (`00115c719`) by H2 as code reading only; no build or test was run. Classification sources: `R3/TASKS/T4A_CLASSES.md`, `T6_CLASSES.md`, `T7_CLASSES.md`, `T8_CLUSTERS.md`, `T12_UNREACHED.md` (proposals, not accepted results).

## Acceptance checks

- A profile test enumerates every plan-listed family with a classification.
- Each implemented component mapping has an approximation-classification test.
- The affected ledger rows are re-verified in a later concordance; no ALIGNED status is claimed from this brief.

## Protected-content status

No protected subject: no affected row is at INVARIANT tier or carries an IP_DATA, CLAIMS or SECURITY layer. 
This brief quotes no protected, private or third-party content. Execution uses invented or synthetic fixtures only and introduces no standards text, tables or equation sources (DEC-043).

## BlockedOnPacket

6 of 11 claim rows carry a block: A7 (4); C5 (1); C7 (1). `H3[<class>]` names the H3 register item for that class (review before repair; mapped in `H3_TOKEN_MAP.csv`). Unblocked rows may proceed separately once selected.

## Notes and open views

- "Plan-listed" refers to the deleted export plan; A7 decides whether it is restored, re-pointed or retired. The four rows scoped to plan-listed families (REQ-013, CLM-029, OUT-001, CONTEXT#description) carry A7: without a plan they have no target set.
- CLM-021 (document fixture provenance) carries C5, as C5 §5 and §9 ask.
- REQ-034 is CP-11: T8 reads NO_ACTION, class reads CODE_FIX_CANDIDATE (C7).

## Dependencies

A7, C7, C5.

