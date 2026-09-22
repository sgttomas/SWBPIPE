# DEL-10-01 Public API and plugin boundary — worker notes (W3 PKG-10 G1)

Forward ledger `DEL-10-01_forward.csv` (94 rows: 85 required keys, the 7
`.rNN` rows of CLM-034, and 2 `.sNN` sub-claims of the Architecture Basis
Injection block). Sealed in `DEL-10-01_SEAL.txt`. Reverse `DEL-10-01_reverse.csv`
(387 capabilities). Shared judgments are in `../_WORKER_DEL-10-01_NOTES.md`
(J1–J18). None of my deliverables was an R0 pilot.

## Path aliases

- Deliverable folder: `projects/chirality-piping/execution/PKG-10_Build, Packaging, API, and Interoperability/1_Working/DEL-10-01_Public API and plugin boundary/`.
- Contract: `api/api_boundary_contract.yaml` (strict JSON in a `.yaml` file;
  project-root token). Prose companion:
  `projects/chirality-piping/docs/architecture/plugin_boundary.md` (explicit
  project form; there is no root copy).
- SOW parity records live at the **repository root**
  (`execution/_Coordination/AgentRuns/SOW-STAGE2-EXEC-20260712-01/...`), not
  under the project, although `EVIDENCE_MAP.csv` prints them as `execution/...`.
- `SOFTWARE_DECOMP.md` is cited as the explicit project path.

## Judgment calls

1. **Contract with no product caller (F7).** The contract is read only by
   `tests/test_api_boundary_contract.py`; there is no API server, plugin loader
   or GUI consumer. Because the SOW says the deliverable "records a contract
   boundary, not an implementation", contract-level requirements are judged
   against the contract and marked `PRODUCT_CALLER: NONE`.
2. **Operation families (REQ-02, CLM-005, CLM-015/REQ-02; FG-DEL-10-01-01).**
   The contract has no model-creation, load-case-definition or rule-pack
   *evaluation* operation (only `ops.rule_pack.attach`). STATUS R01 records
   the same open item. PRD v0.4 no longer has a §19.3 "Public API" section
   (19.3 is now "Required Notice"), so the family list now rests only on
   SOW-030 and local design; `AuthorityNeeded OWNER` so the owner can confirm
   whether those families are still wanted.
3. **REQ-10 invalid-input state.** The six TYPES §4 software statuses and the
   hash-bound human acceptance reference match TYPES §4; there is no explicit
   invalid-input state. Tier `LOCAL_DESIGN` under F8 (the professional
   boundary itself holds).
4. **REQ-09 rule-pack reference.** The prose (`plugin_boundary.md`) states the
   identity/version/checksum/source-notice rule, but the envelope schema has
   no field for version or source notice. `PARTIALLY_IMPLEMENTED`, MEDIUM.
5. **REQ-13 / CLM-015/REQ-13 / CLM-034.r05–r07.** Setup TBDs overtaken by
   rulings (J5): named export targets (SCA-004 via OI-004), rule grammar
   (D-02/DEC-022), CI provider (D-05/DEC-025). Public transport, plugin
   runtime, permission taxonomy and code generation remain TBD (DEC-010/012).
6. **CLM-034.r02 (CP-10, FG-DEL-10-01-02).** The "equivalent schema file
   layout" hold was settled by the DEV-001 implementation of
   `api/api_boundary_contract.yaml`. That dispatch was human-authorised, but
   dispatch briefs are context under A3, not rulings, so the owner confirms.
7. **STATUS R02.** The human disposition for `PKG10-DEL1001-PKG02-W001` is
   still `TBD`. The 2026-06-07 PKG-10 review dispositioned the DEL-10-02, 03
   and 05 findings but not this one. No SOW row carries the review action, so
   under F2 the Remaining row itself takes `PARTIALLY_IMPLEMENTED` (`REVIEW`).
8. **Stale PRD citations inside requirement rows** (REQ-04 "PRD 20", REQ-05
   "13.5", REQ-12 "18.2/18.3") are recorded in Notes only (J7, C6(b)). The
   References block (CLM-008) is the CP-02 row.
9. **CLM-035 hold points** are restrictions ("need X before Y"). They are
   respected because no endpoint, grant or private-data exception exists. I
   judged this `ALIGNED`, not CP-11: the governed behaviour is meant to be
   absent until approved.
10. **REQ-15** is `ALIGNED`, with a `GAP_WORDING_CHECKED` clause. Its present
    obligation (documentation review plus contract tests) is met. The later
    gates depend on a runtime that the requirement itself keeps TBD.
11. **Verification** is cited as the suite-level gate (`pytest -q tests`, B4_4
    sweep); per-test status is not rerun (A6).

## Canonical departures

None. The seven keyed CS rows inherit their fields. The pattern rows follow
CP-01, CP-02, CP-04, CP-05, CP-09 and CP-10 as written. J1 applies F3's
metadata exception to the CP-01 preamble rows, as CP-01 permits when the
text is a later declaration or metadata.

## Convention friction

- CP-01 says "STALE_SETUP_SPECIFICATION (origin text)". F3 keeps metadata as
  `STALE_REVIEW_OR_EVIDENCE` whatever its origin. The YAML preambles are both
  origin text and metadata; I applied F3 (J1) consistently across all three
  deliverables.
- CS-04 says to add `.sNN` rows for divergent parts of the block. Two were
  needed here: the PKG-00 `SEMANTIC_READY` statement and the ruled "Still
  TBD" items.
- There is one cause per row. The SOW SURFACE row carries CP-04, so the
  frontmatter `decomposition_basis` pin (`4d153302`, 2026-07-14) appears only
  in its Notes.

## UNKNOWN rows

None.

## Reverse pass

The answers are: `CLAIMED_BY` RC-10-0190 (contract) and RC-10-0362 (plugin
boundary doc); `COVERS` RC-10-0240 (plugin manifest schema, owned by
DEL-02-04); all others `NOT_MINE`. The overlapping
paths are the contract, the plugin boundary doc, the plugin manifest schema,
the adapter framework schema, PRD, TYPES and the registers. Each overlap got
a capability-specific reason (F5).

The reverse pass did not change my view of any sealed row. One observation:
the SCA-004 export target families (DEL-17-06..09) appear in the contract's
`x_export_interoperability_registry` only as boundary concepts, and I
answered their artifacts `NOT_MINE`. R3 may want to decide whether that
registry is a `COVERS` relation.

## Batch consistency

`--batch` over the three forward ledgers: **PASS, 0 findings**. The shared
bodies with DEL-10-03 (Records: CLM-024 ↔ DEL-10-03 CLM-025; Completion
Condition: CLM-025 ↔ DEL-10-03 CLM-026) carry identical profiles.

## Selectability

`SelectableUnderCurrentLoop` is `NOT_APPLICABLE` on every row (C9). Since
2026-09-19, Piping selects work through owner-steered work graphs, not
`## Remaining`.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
