# DEL-10-03 Local FEA handoff data contract — worker notes (W3 PKG-10 G1)

The forward ledger `DEL-10-03_forward.csv` has 88 rows:
- 79 required keys;
- the 7 `.rNN` rows of CLM-035;
- 2 `.sNN` sub-claims of the Architecture Basis Injection block.

It is sealed in `DEL-10-03_SEAL.txt`. The reverse file `DEL-10-03_reverse.csv`
answers 387 capabilities. Judgments shared across my deliverables are in
`../_WORKER_DEL-10-01_NOTES.md`.

## Path aliases

- Deliverable folder: `projects/chirality-piping/execution/PKG-10_Build, Packaging, API, and Interoperability/1_Working/DEL-10-03_Local FEA handoff data contract/`.
- Contract files:
  - `schemas/local_fea_handoff.schema.yaml` (strict JSON);
  - `projects/chirality-piping/docs/local_analysis/local_fea_handoff_guidance.md`.

  `_CONTEXT.md` names the guidance location "docs/local-analysis notes" (hyphen). The folder on disk is `local_analysis` (underscore).
- Tests: `tests/test_local_fea_handoff_contract.py` (Python) and `apps/desktop/src/features/local-fea-handoff/LocalFeaHandoffPanel.test.tsx` (desktop).
- Product caller: `LocalFeaHandoffPanel.tsx`. It builds packets that follow the schema, so no DEL-10-03 row carries `PRODUCT_CALLER: NONE` (J9).

## Judgment calls

1. **CP-10 group (FG-DEL-10-03-01).** The Human-Ruling Queue held three items open:
   - schema filename and location (CLM-035.r01);
   - field names and JSON Schema layout (r04);
   - the advisory label vocabulary, marked PROPOSAL "requires later human/API review" (r05).

   All three were settled in code by the DEV-001 rev 0.5 sealed brief. That brief is context under A3, not a ruling.
   - The implemented vocabulary uses different labels from the SOW proposal. The implemented labels are lowercase, for example `global_centerline_expected_sufficient_for_screening` and `local_shell_solid_handoff_consider`.
   - The 2026-06-07 PKG-10 review decided two things only: it accepted the PKG-02 dependency finding, and it deferred the dependency TBD rows. It did not review the vocabulary.
   - REQ-09's schema-placement element is the same finding.

   The owner confirms these at R4.
2. **OUT-001 (purpose).** Two elements are unmet:
   - no explicit property (material/section) slots;
   - no result re-association structure.

   The distinct FR-025 local FEA export is scheduled post-beta (Phase H) by D-12/DEC-078. That makes the row `PARTIALLY_IMPLEMENTED · DEFERRED_BY_RULING`.
3. **REQ-08 and CLM-015/REQ-08.** The handoff schema references a result envelope, but it carries no no-bypass constraint set, and no export adapter exists. `PARTIALLY_IMPLEMENTED`, confidence MEDIUM (CP-11).
4. **STATUS R01 is CP-07.** D-12 was ruled (DEC-078: FR-025 "implement post-beta", with DEL-10-03 as contract authority). The item's either/or wording and its D-12 gate are therefore overtaken. The implementation action itself remains open for Phase H.
5. **CLM-014 Standards.** It says "does not create a repository-level schema file". That is setup text overtaken by the schema (J3). The row also pins revision 0.7. One cause per row, and the substantive gap wins over the pin, so the row is `DOC_BEHIND_CODE`. (DEL-10-01 CLM-014 has only the pin, so it takes CP-02. The two rows differ because their text differs, not by inconsistency.)
6. **CLM-005 Attributes** is `ALIGNED`, with a `GAP_WORDING_CHECKED` clause. Its only defect is a citation of `INIT.md`, which is absent from the freeze. A citation is not a claim element (C6(b), J7).
7. **CLM-032 Considerations** is `STALE_SETUP_SPECIFICATION`. Its "advisory label examples for future schema review" table was overtaken when the schema adopted a different vocabulary.
8. **MEMORY.** The 2026-07-02 bridge entry published `operation_outcome.schema.json` and `rule_check_run_result.schema.json` under this deliverable's lane. The history is accurate, so the row is `ALIGNED`. In the reverse pass both schemas are `COVERS` (a record relation), not owned.

## Canonical departures

None. All seven keyed CS rows inherit their fields.

## Convention friction

- CLM-015 (Verification) has keyed children only for REQ-07, REQ-08 and REQ-12. Its combined rows (REQ-01/06/10; REQ-02/03/04/11; REQ-05/09; the future implementation gate) have no keys, so they are assessed on the block row itself.

## UNKNOWN rows

None.

## Reverse pass

- `CLAIMED_BY`: RC-10-0116 (schema) and RC-10-0338 (guidance).
- `PARTIAL`: RC-10-0384 (desktop handoff panel). It builds this contract's packet, but the region-selection heuristic and download UX have no keys, and FR-025 export is post-beta.
- `COVERS`: RC-10-0106 and RC-10-0213 (the bridge-published schemas recorded in MEMORY).
- `NOT_MINE` with specific reasons:
  - the API contract (RC-10-0190);
  - the registers;
  - the FR-HAND handoff package schema and panel (RC-10-0009, RC-10-0191), which DEC-078 keeps distinct from FR-025.

The reverse pass did not change my view of any sealed row.

## Batch consistency

`--batch` over the three forward ledgers: **PASS, 0 findings**. The shared bodies with DEL-10-01 have identical profiles:
- Records: CLM-025 ↔ DEL-10-01 CLM-024;
- Completion Condition: CLM-026 ↔ DEL-10-01 CLM-025.

## Selectability

`SelectableUnderCurrentLoop` is `NOT_APPLICABLE` on every row (C9). Since 2026-09-19, Piping selects work through owner-steered work graphs, not `## Remaining`.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
