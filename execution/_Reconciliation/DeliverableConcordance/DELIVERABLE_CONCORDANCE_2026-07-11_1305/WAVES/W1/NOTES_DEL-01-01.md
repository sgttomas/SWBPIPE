# NOTES — DEL-01-01 Project governance baseline (W1, DELIVERABLE_CONCORDANCE_2026-07-11_1305)

Pilot: fable (owner-assigned for PKG-01 fence-adjacency). Encoding per
`R1_CONVENTIONS.md` (conventions 1–8 + addenda 1–13) with the owner-ruled
deliverable-specific rules for the corpus's only ISSUED deliverable:
LifecycleState=ISSUED copied verbatim; no seeded bootstrap item exists on this
`_STATUS.md` (expected — the flip touched only the 100 IN_PROGRESS
deliverables; not a finding); every change-shaped finding routes
`AuthorityNeeded=SCOPE_CHANGE` with no repair recommendation, because touching
an ISSUED baseline is a scope-change act.

**Path alias (addendum 12):** `DELROOT` =
`execution/PKG-01_Governance, IP Boundary, and Professional Responsibility/1_Working/DEL-01-01_Project governance baseline`
(relative to the frozen tree's `projects/chirality-piping/`).

## 1. Histograms (recounted from the CSV before writing)

Row count: 22.

Disposition histogram:

| Disposition | Count |
|---|---:|
| ALIGNED | 19 |
| STALE_SETUP_SPECIFICATION | 3 |

ClaimType histogram:

| ClaimType | Count |
|---|---:|
| REQUIREMENT | 8 |
| ACCEPTANCE | 5 |
| EXCLUSION | 3 |
| DECLARED_STATE | 6 |

AuthorityNeeded: NO 19; SCOPE_CHANGE 3 (the three stale declared-state rows).
LifecycleState: ISSUED on all 22 rows. SelectableUnderCurrentLoop: NO on all
22 rows (no recorded residual item anywhere; addendum-12 default; ISSUED
lifecycle). REMAINING_WORK rows: 0 — `_STATUS.md` has no `## Remaining`
section and no other surface records a live residual for this deliverable.
IMPLEMENTED_UNMAPPED rows: 0 — the R1 `NONE_FOUND` surface shortlist contains
no PKG-01/governance-orbit surface; governance templates/checklists in the
orbit are already attributed (SURF-001, SURF-142, SURF-169, SURF-217 →
DEL-01-02/-03/-04 etc.).

## 2. Self-flagged rows

- DEL-01-01-ACC-001 — census judgment: AC-01-01-01 substantially restates
  REQ-01-01-01; retained as a row because the Acceptance Criteria section is
  the deliverable's own acceptance surface with distinct evidence pointers,
  and the addendum-12 exclusion targets Verification tables that merely
  restate requirements. Reviewer may prefer merging.
- DEL-01-01-ACC-002 — same census judgment (restates REQ-01-01-03; adds the
  named future-artifact evidence obligation, which I verified now exists).
- DEL-01-01-ACC-003 — same census judgment (restates REQ-01-01-04 + explicit
  TBD-wording condition).
- DEL-01-01-ACC-004 — same census judgment (restates REQ-01-01-05/-07;
  distinct evidence pointers C-01-01-002 / decision surface).
- DEL-01-01-EXC-001 / -002 / -003 — grain judgment: the Specification's single
  scope-exclusion sentence was split into three rows (implementation/solver/
  GUI; legal advice + license selection; certification/approval language +
  release acceptance + professional/code-compliance claims). Conventions do
  not fix exclusion grain; three thematically coherent boundaries seemed the
  auditable grain on this fence-adjacent deliverable.
- DEL-01-01-DECL-001 and DEL-01-01-DECL-004 — judgment call on
  pointer-only drift: Specification/Procedure name revision 0.7 / DAG-006 as
  the *current* authority while the frozen tree's current authority is
  revision 0.8 / DAG-007. Substance is otherwise accurate. I applied the
  widened `STALE_SETUP_SPECIFICATION` (addendum 4 "post-alignment drift";
  consistent with W1 peer encoding of the same 0.7-vs-0.8 lag on DEL-00-01),
  routed `SCOPE_CHANGE` per the ISSUED rule. A reviewer could defensibly read
  issuance-time authority pointers on an ISSUED baseline as historical record
  and encode ALIGNED-with-note; the conventions do not carve out ISSUED
  surfaces from addendum 4.
- DEL-01-01-DECL-006 — MEMORY.md inclusion judgment: every MEMORY entry is
  dated, but the latest (2026-06-03 LICENSE_AND_ISSUED_CLOSEOUT) functions as
  the surface's current-state declaration, so the addendum-1 "when they carry
  current-state declarations" condition was read as satisfied. Historical
  drift inside earlier dated entries (e.g. 2026-05-11 license-TBD language)
  is noted on the row, not dispositioned.

## 3. Evidence-execution log

Re-executed (side-effect-free, read-only — no build, no test, no bytecode):

- Frozen-tree document review at SHA 551f84ef6: full read of the DEL-01-01
  four-document kit, `_STATUS.md`, `MEMORY.md`, `_CONTEXT.md`,
  `_REFERENCES.md`, `_REVIEW.md`, `Review_Findings.csv`, and the pertinent
  `_run_records` (both 2026-06-03 review runs, the 2026-06-03 refresh run,
  the 2026-06-04 authority refresh, the 2026-06-16 dependency-semantic
  refresh); repo-level surfaces `LICENSE.md`, `governance/MAINTAINERS.md`,
  `governance/CONTRIBUTOR_CERTIFICATION_TEMPLATE.md` and
  `CONTRIBUTION_REVIEW_CHECKLIST.md` (presence), `docs/README.md`,
  `docs/DIRECTIVE.md` §6, `docs/AGENTIC_DEVELOPMENT_WORKFLOW.md` (draft
  posture lines), register rows in `docs/_Registers/Deliverables.csv` and
  `ScopeLedger.csv`, `SOFTWARE_DECOMP.md` header (revision: 0.8),
  `execution/_DAG/_LATEST.md` (DAG-007), and
  `execution/_Coordination/_DECISIONS/D-40_RULING_2026-07-11.md` (presence).
- TBD census re-count: `grep -o 'TBD'` over the four-document kit at the
  frozen SHA returns 24, matching the `_REVIEW.md` §10 closeout figure of 24.
- No test suite, build, or script belongs to this DOC_UPDATE deliverable's
  verification basis (its Specification defines verification as document
  review), so nothing else was re-run.

Cited as recorded (not re-executed at frozen SHA 551f84ef6):

- `_REVIEW.md` §9 Gate-5 review pass (snapshot
  `execution/_Reconciliation/Reviews/REV_DEL-01-01_2026-06-03_2327`) and §10
  ISSUED closeout (`REV_DEL-01-01_2026-06-03_2334`), bound to the 2026-06-03
  working state.
- The 2026-06-03 refresh run's validation set (status helper on DAG-005/006,
  `validate_dependencies_schema.py`, `git diff --check`, manual claim scan)
  and the 2026-06-16 dependency refresh validation (schema VALID, 13 rows).
- Because repo-level governance surfaces changed after issuance (e.g.
  `governance/MAINTAINERS.md` now cites DAG-007), the 2026-06-03 recorded
  review is partially overtaken for the frozen state; per convention 8 the
  ALIGNED dispositions rest on the frozen-SHA re-executed document review
  above, with the recorded passes cited as corroborating history.

Porcelain checks: `git -C FROZEN status --porcelain` run before discovery
reads and again after all outputs were written — empty both times.

## 4. Convention friction notes

- **Addendum-1 census vs product governance surfaces.** The deliverable's
  principal product artifacts (`governance/MAINTAINERS.md`, `docs/README.md`)
  carry their own current-state declarations but are outside the declared-state
  census (not four-document-kit surfaces, not `_STATUS`/`MEMORY`, not
  deliverable-owned READMEs). Observation recorded here, not as rows: at the
  frozen SHA both surfaces declare decomposition "revision 0.7" alongside
  "approved DAG-007", while the frozen `SOFTWARE_DECOMP.md` header is
  revision 0.8 — a mixed authority pointer (DAG pointer current, revision
  pointer lagging) on shared repo-level surfaces. These surfaces remained
  writable after DEL-01-01 issuance (later authorized tranches updated them
  to DAG-007), so drift there is not necessarily an ISSUED-baseline touch;
  ownership of any correction is ambiguous between DEL-01-01 and the later
  refresh tranches. No convention home exists for declared-state drift on
  shared repo-level artifacts; flagged for the wave reviewer/aggregation.
- **Acceptance-row grain.** Addendum 12's exclusion speaks to "verification
  tables that merely restate requirements"; it is silent on Acceptance
  Criteria sections that partially restate requirements. See self-flags.
- **ISSUED vs widened staleness.** Addendum 4 has no carve-out for
  issuance-time authority pointers inside an ISSUED (frozen-by-governance)
  baseline; strictly applied, any later corpus-wide authority bump renders
  ISSUED kit surfaces "stale" in perpetuity. Encoded per the letter of the
  convention with `SCOPE_CHANGE` routing; a run-level clarification may be
  worth adopting before W2–W5 encounter future ISSUED deliverables.
- **Exclusion grain** unspecified (see self-flags).

## 5. Boundary-compliance statement

- Discovery reads were confined to the frozen worktree at
  `551f84ef6be656f1603ce0acfa5e3935aa9683c7`; nothing was written anywhere
  under it (no build artifacts, no bytecode, no caches); porcelain empty
  before and after.
- Writes were confined to exactly two files:
  `WAVES/W1/CLAIM_CONCORDANCE_DEL-01-01.csv` and `WAVES/W1/NOTES_DEL-01-01.md`
  in the working run folder.
- No lifecycle transition, DAG mutation, `_STATUS.md`/register/product edit,
  or cross-project edit was made or proposed. The three change-shaped
  findings are recorded as dispositions routed `AuthorityNeeded=SCOPE_CHANGE`
  only; no repair is recommended anywhere in these outputs.
- F-PIP-1..5 held: no release-readiness, issuance, certification, sealing,
  professional-approval, or code-compliance claim is made in these outputs;
  ISSUED is copied verbatim as a lifecycle/change-control state only, and all
  reproduced exclusion/disclaimer language is marked as quotation.
- All dispositions in the ledger are this discovery agent's judgments, not
  owner or engineering rulings.
