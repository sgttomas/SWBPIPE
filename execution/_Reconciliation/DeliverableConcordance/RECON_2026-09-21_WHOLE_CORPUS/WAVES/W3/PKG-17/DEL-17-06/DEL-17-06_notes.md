# DEL-17-06 — W3 notes (PKG-17, worker G2)

Forward ledger sealed at SHA-256
`e5e7a68099060461034c730e9649170e2a1cf8cfad2b9ade4c9c1fda53324850`
(123 rows). The rows are:
- the 94 required keys;
- the CLM-005, CLM-008 and CLM-014 `.rNN` splits;
- the sub-claims `SOW.s01`, `SOW.s02`, `CLM-013.s01` to `.s03`, `CLM-027.s01`
  and `CONTEXT.s01`.

Dispositions are agent judgments, not owner rulings.

## Path aliases

- As DEL-17-04. The product path is `StressNeutralExportPanel.tsx`, mounted
  at `App.tsx` L953 and covered by `e2e/result-compatibility.spec.ts`, which
  appears as passing in the B4.4 sweep log. Not rerun.

## Judgment calls

- **Hash text not scoped after D-67 (FG-DEL-17-06-03,
  SCOPE_REDIRECTED_BY_RULING, AdoptedByReference YES).** CLM-042 keeps CLM-018
  and CLM-041 governing the 0.1 family. The 0.2 family uses
  `openpipestress_jcs_ijson_v1`. Several texts still state the sorted-compact,
  no-JCS basis for all JSON without that scoping:
  - CLM-009;
  - CLM-014.r02;
  - VER-008;
  - CLM-027.s01;
  - CLM-028;
  - AC-001.

  These rows are STALE_REVIEW_OR_EVIDENCE with tier LOCAL_DESIGN, because a
  catch-up with no decision repairs them. CLM-018 and CLM-041 are ALIGNED
  because they are self-scoped. The 0.1 Python builder has no product caller, so
  those rows carry `PRODUCT_CALLER: NONE`.
- **Rename residue.** The default CP-04 variant is on the SOW SURFACE row: SOW
  prose, ENGINE_PROVENANCE, the schema `$id` and title, and the desktop
  `document_kind` and filenames. The frozen-contract variant for
  `openpipestress_jcs_ijson_v1` is on `SOW.s02` (CLM-042) and on `CONTEXT.s01`
  (the D-67 block). Both are PROJECT_BASELINE · FROZEN_CONTRACT ·
  RECORD;BASELINE. They are code-change candidates for R4, not deliverable
  edits.
- **Phase A and TBD residue (FG-DEL-17-06-04).** Much of the SOW is origin text
  (7bee9ae41) that declares fields, layouts or schemas TBD or not produced.
  Those are now fixed by the 0.1 and 0.2 schemas and builders:
  - STALE_SETUP_SPECIFICATION · DOC_BEHIND_CODE where the code advanced (for
    example CLM-005 r01-r06, CLM-007, CLM-012, CLM-014 r01 and r03, CLM-037 to
    CLM-039, VER-005);
  - CP-01 · REPRESENTATION_MIGRATED where the text names the four-document kit.
- **Requirements with a stale TBD clause** (REQ-003, REQ-004, REQ-006) are
  ALIGNED on their substance under C6(b). The overtaken "exact names TBD" clause
  is recorded in Notes with `GAP_WORDING_CHECKED`.
- **Requirements-table source column.** The column cites:
  - the absent export plan (`CLM-013.s01`, CP-08);
  - DEL-17-01 and DEL-17-02 `Specification.md` (`CLM-013.s02`, CP-01);
  - DAG-006 (`CLM-013.s03`, CP-02).

  The requirement items themselves are judged on substance. CLM-004 was assessed
  unsplit as CP-08, because its values hold and only its source pointers are
  stale.
- **CLM-006 candidate members.** The package ships one result-row table keyed by
  `canonical_ref` with nine 0.2 members. There are no per-entity node,
  element, component, restraint, equipment, material, section or load-case
  tables. Rated IMPLEMENTED_DIFFERENTLY · DOC_BEHIND_CODE at MEDIUM. Whether
  per-entity tables are still wanted is a catch-up question.
- **VER-006.** No requirement-to-evidence traceability table was found by the
  A3 discovery in the deliverable run records, so the row is
  DOCUMENTED_UNIMPLEMENTED.
- **VER-011 and REQ-010** are ALIGNED: the stress-neutral profile deliberately
  carries no target-specific support flag. I read this as the design the check
  protects, not as CP-11.
- **Remaining.** Both items are accurate, and no governing SOW row carries their
  work (F2), so each carries its own gap:
  - R01 (two withheld diagnostic-work witnesses; the test asserts 830 rows,
    828 witnesses and 2 withheld): AuthorityNeeded ENGINEERING;
  - R02 (semantic and lensing regeneration; `_DEPENDENCIES.md` names DAG-007).
- **Architecture Basis Injection** is a deliverable-specific body (T2A-edited).
  It is ALIGNED because the D-67 block that follows scopes it to 0.1.

## Canonical departures

None. CS rows inherit their assigned values.

## Convention friction

- Table source columns carry pointer residue that is independent of each row's
  substance. The `.sNN` sub-claims on CLM-013 keep that residue visible without
  hiding the requirement assessments. A block with `.rNN` keys cannot do this.
- Some text is origin text but was later edited by D-41 T2A. For such text the
  F3 origin test was applied to the divergent clause (the T2A additions), not to
  the block's first words.

## UNKNOWN rows

None.

## Reverse pass

Six of 320 capabilities are CLAIMED_BY: RC-17-0041, -0128, -0148, -0171, -0259
and -0298. F5 reasons are specific for RC-17-0108, -0193, -0231, -0250 and
-0314 (governance and register paths cited only as references). RC-17-0144
(semantic contract fixture) and RC-17-0021 (native save) are NOT_MINE: DEL-17-06
consumes them. The reverse pass did not change my view of any sealed row.

## Batch consistency

`--batch` over the three G2 forward ledgers: PASS, 0 findings.

## Selectability

`SelectableUnderCurrentLoop` is `NOT_APPLICABLE` on every row (C9).

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
