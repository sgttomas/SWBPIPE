# DEL-09-04 Validation manual skeleton — worker notes (W3, PKG-09)

Forward ledger sealed at SHA-256
`72d1c2c2e26a79a87634c4f97c19a08c8221981838d24ce950b451373829f5b3`
(`DEL-09-04_SEAL.txt`). Frozen state `00115c719`. All evidence was read from
the frozen checkout. No build, test or suite was run.

## Path aliases

- `VM` means `projects/chirality-piping/docs/validation_manual/index.md`. It is
  the deliverable's main artifact; its frontmatter says `implements: DEL-09-04`.
  The line anchors cited are §1 L18, §2 L44, §3 L61, §4 L175, §5 L190, §8 L241
  and §10 L270.
- `VALIDATION_STRATEGY.md` is the project copy. `tools/validation/check_four_documents.sh`
  is the root file, and there is no project copy of it.
- The parity records are cited at the root path
  `execution/_Coordination/AgentRuns/SOW-STAGE2-EXEC-20260712-01/instances/CHANGE-P2/checks/DEL-09-04/`.
  These are the paths `EVIDENCE_MAP.csv` lists. All five DEL-09-04 records land
  in commit `3ea904a6e` (2026-07-14).

## Judgment calls

- **Setup-origin text (F3).** `git log -S` shows that the SOW text blocks first
  appear at `7bee9ae41`. The SOW migration `fcbe4d994` (2026-07-14) carried them
  over. Stale setup text is therefore `STALE_SETUP_SPECIFICATION`. The
  exceptions are the D-41 declarations (CP-03, a later declaration, so
  `STALE_REVIEW_OR_EVIDENCE`) and the PKG-00 state sub-claim (a review state).
- **Stale source pointers (FG-DEL-09-04-02).** `INIT.md` was removed on
  2026-07-04 (`9c4caf8fd`). `SPEC section 6` is Loads and stress recovery; the
  rule-pack evaluator is §7, and this numbering was already in place at
  `7bee9ae41`. Rows CLM-005, CLM-007 and VAL-REQ-004 are affected. The substance
  of each row holds in the manual. The SPEC "sections 7-9" in VAL-REQ-009 and
  CLM-005 is read as rule-pack, GUI and reporting, which is plausible, and was
  not flagged. Confidence MEDIUM.
- **Four-document residue (CP-01, FG-DEL-09-04-03).** Rows VAL-REQ-001 and
  VAL-REQ-005 (acceptance hooks), CLM-013, CLM-014, CLM-019, CLM-020 and CLM-021
  are affected. The two VAL-REQ rows meet their substance; only the hook names
  Procedure.md or the Datasheet. Under F1 they are not `ALIGNED`.
- **Write scope (FG-DEL-09-04-01).** CLM-010 says the deliverable "does not
  edit VALIDATION_STRATEGY.md". CLM-018 limits writes to the deliverable folder.
  MEMORY (Tranche B, sealed-brief override) and commit `0000ce609` (2026-08-11)
  show edits to VALIDATION_STRATEGY.md. The manual itself lives in
  `docs/validation_manual/`. Both rows are disposed `SCOPE_GREW_BY_DIRECTION`,
  with PR #154 and #287 as context.
- **STATUS R01** is `REMAINING_STATE_MISMATCH` with cause `DOC_BEHIND_CODE`.
  Export-results has been bound since 2026-07-23 (`f82bb28e2`, DEL-10-05 R17),
  and four `del1005_export_results_*` witnesses exist, so the clause "the only
  structured runner stub" is stale. The MAINTAINER_REVIEWED promotion is still
  open: all 64 pages are DRAFT_EVIDENCE.
- **STATUS R02** is `DOCUMENTED_UNIMPLEMENTED` with cause `NOT_STARTED`, tier
  `INVARIANT` and layer `VALIDATION` (F2 and F8). The text is accurate, and no
  governing SOW row carries the selection of public comparison values.
  Confidence MEDIUM on the tier; `PROJECT_BASELINE` or `LOCAL_DESIGN` are
  defensible alternatives.
- **CONTEXT surface** is `STALE_REVIEW_OR_EVIDENCE` (CP-02). It is not
  `ALIGNED`, because the file pins revision 0.7 in two blocks.
  `architecture-basis-injection.s01` records that all eight PKG-00 deliverables
  are IN_PROGRESS (D-40/DEC-072), not SEMANTIC_READY.
- **MEMORY** is `HISTORY` and `ALIGNED`. The undated sections belong to the
  2026-05-04 Tranche B record. No undated D-41 current declaration exists in
  this MEMORY, unlike DEL-09-05.
- `.rNN` blocks were not split; each block was assessed directly.

## Canonical departures

- `CANONICAL_DEPARTURE` on CLM-005, CLM-007 and VAL-REQ-004: CP-02 names
  `STALE_REVIEW_OR_EVIDENCE`. F3's origin test gives `STALE_SETUP_SPECIFICATION`
  for setup-origin section and file references, because F3's exception covers
  only pins, review states and metadata. The verifier should confirm this
  reading.
- The keyed CS rows follow `CANONICAL_ASSIGNMENTS.csv` exactly.

## Convention friction

- CP-02 lists "section reference" under `STALE_REVIEW_OR_EVIDENCE`, while F3
  sends setup-origin text to `STALE_SETUP_SPECIFICATION` and does not list
  section references in its exception. This ledger applies F3.
- The implementation surfaces carry drift that no DEL-09-04 key owns. This is
  recorded here for R3, not as rows:
  - Manual §1 pins revision `0.11` and `DAG-009`.
  - Manual §5 and §10 still call export-results a stub.
  - Manual §5 and VALIDATION_STRATEGY §4 and §6 still list evidence-bundle
    storage as TBD, although DEC-080 set the location
    `validation/evidence/reproduction/`.
  - The runner is named `openpipestress-runner`, an active code identifier with
    the former name. It is not named by the SOW, so it is not on this
    deliverable's CP-04 row.
- VAL-REQ-007 (make open items visible) is `ALIGNED`. Stale TBDs over-report
  open items; they do not hide them.

## UNKNOWN rows

None.

## Reverse pass

The reverse pass (368 capabilities: 6 CLAIMED_BY, 1 PARTIAL, 12 COVERS,
349 NOT_MINE) did not change my view of any sealed row.
- The GUI workflow validation spec (`apps/desktop/e2e/gui-workflow-validation.spec.ts`)
  is not in the PKG-09 routing file, and neither is any capability entry point
  for it. The only exception is its invented fixtures (RC-09-0160, COVERS).
  R3 may find it routed to another package, although DEL-09-04's 2026-08-20
  run produced it.
- RC-09-0136, the validation evidence panel, carries a hardcoded manual map
  that can drift. It was answered COVERS, not owned.
- Every NOT_MINE on a capability whose entry points hit a path this ledger
  cites has its own specific reason (F5). The capabilities are RC-09-0025,
  0073, 0183, 0237, 0245, 0236 and the governing documents.

## Batch consistency

`validate_ledger_v2.py --batch` over the DEL-09-04 and DEL-09-05 forward
ledgers: PASS, 0 findings.

## Selectability

`SelectableUnderCurrentLoop` is `NOT_APPLICABLE` on every row (C9). Since
2026-09-19, Piping has selected work through owner-steered work graphs, not
through `## Remaining`.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
