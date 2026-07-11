# NOTES — CLAIM_CONCORDANCE_DEL-01-03

Deliverable: **DEL-01-03 — Contributor certification workflow** (PKG-01
Governance, IP Boundary, and Professional Responsibility; DOC_UPDATE;
SOW-028/SOW-048; OBJ-002). Lifecycle at frozen tree: `IN_PROGRESS`. Wave W1.
Frozen source tree: `main` @ `551f84ef6be656f1603ce0acfa5e3935aa9683c7` (all
reads read-only).

F-PIP-2 fence note: "certification" throughout this deliverable and ledger
means the contributor DCO/CLA-style provenance attestation for repository
governance intake. Nothing in this ledger makes, evaluates, or implies any
product certification, sealing, professional-approval, code-compliance, or
release-readiness claim; rows that touch that vocabulary do so only as the
deliverable's own negative-boundary requirements (REQ-005, ACC-002).

## Run-level NormativeSource / path aliases (addendum 12)

Declared once for this ledger; used verbatim in the CSV cells:

- `DELROOT` = `execution/PKG-01_Governance, IP Boundary, and Professional
  Responsibility/1_Working/DEL-01-03_Contributor certification workflow`
- Repo-level artifact paths (`CONTRIBUTING.md`, `governance/*`,
  `.github/ISSUE_TEMPLATE/*`, `docs/*`, `tools/*`) are relative to
  `projects/chirality-piping/` in the frozen tree.

Requirement-ID scheme mapping (self-identifying two-digit scheme normalized
to the addendum-12 three-digit ClaimID form): `DEL-01-03-REQ-01 →
DEL-01-03-REQ-001` … `DEL-01-03-REQ-08 → DEL-01-03-REQ-008`.

## 1. Histograms (recounted from the CSV; each reproduces exactly)

Disposition histogram (22 rows):

| Disposition | Count |
|---|---|
| ALIGNED | 18 |
| STALE_SETUP_SPECIFICATION | 3 |
| ACCEPTED_DIVERGENCE | 1 |
| **Total** | **22** |

ClaimType histogram (22 rows):

| ClaimType | Count |
|---|---|
| REQUIREMENT | 8 |
| ACCEPTANCE | 3 |
| EXCLUSION | 1 |
| DECLARED_STATE | 7 |
| REMAINING_WORK | 3 |
| **Total** | **22** |

Row census rationale:

- **8 REQUIREMENT** rows — one per current requirement ID
  (DEL-01-03-REQ-01..08), each carrying the substance disposition
  (convention 1).
- **3 ACCEPTANCE** rows — Specification "Acceptance Criteria" bullets 1
  (kit completeness), 4 (no protected data / legal conclusions introduced),
  and 6 (human rulings visible). Bullets 2 and 3 restate REQ-08 and bullet 5
  restates REQ-04 and get **no** mirrored ACCEPTANCE rows (addendum-12
  grain).
- **1 EXCLUSION** row — Specification Scope second paragraph (refresh does
  not edit repo-level artifacts, select the legal mechanism, assign
  authority, make legal conclusions, certify rights, or approve engineering
  reliance).
- **7 DECLARED_STATE** rows — one per four-document kit surface
  (Specification, Datasheet, Guidance, Procedure) + `_STATUS.md` +
  `MEMORY.md` (both carry current-state declarations) + one
  deliverable-owned in-tree README:
  `projects/chirality-piping/.github/ISSUE_TEMPLATE/README.md`, which
  carries frontmatter `deliverable_id: DEL-01-03` (addendum 1 census).
- **3 REMAINING_WORK** rows — the three non-bootstrap `## Remaining`
  residuals (D-07b intake mechanism; §17.5 legal review; export wiring).
  The seeded `(gated: D-41)` bootstrap item is recorded verbatim ONLY in
  the `_STATUS.md` surface row's `RecordedRemaining` and excluded from all
  residual/gate/selectability analysis (addendum 2).
  `DELIVERABLE_INVENTORY.csv` confirms `NonBootstrapItems=3`,
  `GateSuffixes = gated: D-07b; gated: owner/counsel; UNGATED` — re-verified
  against the frozen `_STATUS.md`.
- **0 IMPLEMENTED_UNMAPPED** rows — every material surface in DEL-01-03's
  orbit carries deliverable attribution in the frozen tree and R1:
  `.github/ISSUE_TEMPLATE/` (SURF-001 → DEL-01-03; README frontmatter +
  TP-E6 run record homing), `CONTRIBUTING.md` / `governance/*` templates
  (SURF-169 shared PKG-01/DEL-11-05 attribution; CONTRIBUTING.md and the
  certification template carry `deliverable_id: DEL-01-03` frontmatter),
  and `tools/release/export_public_openpipestress.py` (SURF-217, shared
  with DEL-10-04, primarily a PKG-10 surface). Mapped surfaces are not
  UNMAPPED-eligible; the kit's silence on the E6 slice is ledgered as
  declared-state staleness (DECL-001/002), not as an unmapped surface.

## 2. Self-flagged rows

- **DEL-01-03-DECL-001 (Specification)** — `STALE_SETUP_SPECIFICATION`
  rests on the widened addendum-4 definition: the Specification carries no
  stale authority pointers in-body, but its declared scope/evidence set no
  longer describes the frozen implemented slice, which since 2026-07-10
  includes the E6 public issue templates homed on this deliverable by
  `_STATUS.md` residuals and SURF-001 attribution. A reviewer may prefer
  ALIGNED-with-note if they read the E6 slice as adjacent rather than
  in-scope; I dispositioned staleness because `_STATUS.md` binds the slice's
  residuals to this deliverable while the kit is silent.
- **DEL-01-03-DECL-002 (Datasheet)** — the "License decision: TBD … intends
  to be free/open-source" attribute contradicts the frozen selected-license
  state (`PolyForm-Noncommercial-1.0.0`, a source-available noncommercial
  license). `Review_Findings.csv` DEL-01-03-REV-001 marked the license
  wording RESOLVED at `_CONTEXT.md`/`Specification.md`/`Guidance.md` but its
  location list never included the Datasheet, so this is uncured drift the
  2026-06-04 refresh missed, not a contradiction with the review record.
  Reviewer eyes wanted on that reading.
- **DEL-01-03-DECL-003 (Guidance)** — ALIGNED despite not mentioning the E6
  slice; Guidance's declarations (license selected, C-001 ruling, open
  rulings list) all verify against the frozen tree and it carries no stale
  pointers. Judgment call pairing with the DECL-001 staleness call.
- **DEL-01-03-DECL-006 (MEMORY)** — ALIGNED with a currency-gap note: no
  MEMORY entry exists for the 2026-07-10 TP-E6 tranche although its run
  record and residuals exist. Addendum 1 makes dated-entry drift note-only;
  a *missing* latest entry seemed closest to the same treatment, flagged
  because the conventions do not address log-currency gaps explicitly.
- **DEL-01-03-REM-001** — `ACCEPTED_DIVERGENCE` vs `ALIGNED` call: DEC-027
  is a named owner ruling that *permits* (indeed mandates) the deferred
  state — contributions closed, intake gated at D-07b — so per addendum 11
  the divergence-permitting ruling exists and `ACCEPTED_DIVERGENCE` outranks
  plain `ALIGNED` (convention 8). Flagged because the row is simultaneously
  a correctly-recorded residual (which alone would yield ALIGNED).
- **DEL-01-03-REM-003** — `SourceReliability=UNVERIFIED`: the residual and
  its evidence originate in the agent-authored TP-E6 run record (PR #164
  merged, but I found no named human disposition covering the record's
  content beyond the merge itself, and I did not treat a merge as a recorded
  human disposition per addendum 6). Also a cross-deliverable homing note:
  the export machinery is DEL-10-04's surface; the item is recorded here
  with an explicit `(see also DEL-10-04)` cross-reference, so no
  REMAINING_STATE_MISMATCH (convention 3 homing satisfied by the recorded
  cross-reference). Its `SelectableUnderCurrentLoop=YES` is the mechanical
  convention-6 derivation only.
- **DEL-01-03-ACC-003** — grain call: Acceptance bullet 6 ("Human rulings
  needed are visible") could be read as restating REQ-04's TBD-preservation;
  I kept it as a distinct visibility obligation (it is testable against
  Guidance's Human Rulings Needed section independently of TBD values).
- **DEL-01-03-REQ-007** — MEDIUM confidence: boundary *visibility* for
  future adapters/imports/rule packs is doc-level at the frozen SHA;
  enforcement lives downstream in PKG-10 export machinery. ALIGNED on the
  claim as worded (visibility, not enforcement).

## 3. Evidence-execution log

Re-executed (side-effect-free per addendum 9; all runs inside the frozen
tree used `PYTHONDONTWRITEBYTECODE=1`, wrote nothing, and `git -C FROZEN
status --porcelain` was empty BEFORE and AFTER every batch):

1. `yaml.safe_load` structural checks on all four
   `.github/ISSUE_TEMPLATE/*.yml` files — PASS (required keys, body element
   types within the GitHub issue-form set, checkbox blocks present;
   `config.yml` `blank_issues_enabled: false`). Reproduces the TP-E6
   recorded structural validation at the frozen SHA.
2. `python3 tools/validation/validate_dependencies_schema.py` on
   `DELROOT/Dependencies.csv` — VALID, 29 columns, 15 data rows (matches the
   2026-06-04 `_REVIEW.md` and 2026-06-16 refresh records).
3. Prohibited-claim grep (REQ-05 verification approach) over
   `CONTRIBUTING.md`, both governance templates, the checklist, and the
   issue forms — every hit is negative boundary language, a
   certification-field name, or non-affiliation wording; no reliance claim.
4. `git merge-base --is-ancestor 1581b8c0de72 551f84ef6` → ancestor
   confirmed; `git diff 1581b8c0de72..551f84ef6 --
   projects/chirality-piping/.github/ISSUE_TEMPLATE` → empty. This grounds
   the addendum-10 qualifier used on DECL-007/REM-003: `content-identical
   at frozen SHA 551f84ef6be656f1603ce0acfa5e3935aa9683c7 (diff empty over
   projects/chirality-piping/.github/ISSUE_TEMPLATE)`.
5. Frozen-register re-verification: D-07b `AWAITING_RULING` and DEC-027
   substance in `execution/_Coordination/_DECISIONS/_REGISTER.md`;
   `execution/_DAG/_LATEST.md` → DAG-007; `SOFTWARE_DECOMP.md` frontmatter
   `revision: 0.8`; sweep JSON
   `validation/evidence/sweeps/SWEEP_20260711T031432Z_1581b8c0de72.json`
   present, `overall_status: pass`, `commit_hash 1581b8c0de72…`,
   `working_tree_dirty: false`.

Cited as recorded (not re-executed):

- `DELROOT/_REVIEW.md` 2026-06-04 formal review (PASS_WITH_WARNINGS;
  validation table rows cited per claim) — no commit binding (pre-DEC-025
  review-binding caveat per the R1 authority map); marked `not re-executed
  at frozen SHA 551f84ef6`. Its human disposition is the approved
  lifecycle update to CHECKING recorded in `_STATUS.md` history 2026-06-04.
- TP-E6-ISSUETEMPLATES-001 recorded validations (harness self-check exit 0;
  DEC-025 five-surface sweep at commit `1581b8c0de72`, overall pass) —
  marked `not re-executed at frozen SHA 551f84ef6`, with the addendum-10
  content-identical qualifier over the template paths (diff actually run,
  empty).

## 4. Convention friction notes

- **Log-currency gaps on MEMORY:** addendum 1 covers drift *inside* dated
  MEMORY entries but not a missing entry for a landed tranche. I treated the
  gap as a note on the MEMORY surface row (DECL-006 ALIGNED), consistent
  with addendum 1's spirit; the binding set could state this explicitly.
- **Residual rows that are also ruled deferrals:** a recorded residual whose
  deferred state is itself mandated by a named ruling (REM-001 / DEC-027)
  sits between addendum 11's "recorded residuals alone yield ALIGNED" and
  convention 8's `ACCEPTED_DIVERGENCE > ALIGNED` precedence. I applied the
  precedence rule because the permitting ruling exists and is named; the
  addenda could clarify which reading wins for REMAINING_WORK-type rows.
- **README census scope:** addendum 1 says "deliverable-owned in-tree
  README" without defining ownership; I used frontmatter `deliverable_id`
  as the ownership test (`.github/ISSUE_TEMPLATE/README.md` qualifies).
  Note `CONTRIBUTING.md` also carries `deliverable_id: DEL-01-03`
  frontmatter but is a repo-level governance artifact, not a README, and
  the R1 authority map places the `governance/*`+`CONTRIBUTING.md` family
  under the DEL-01-01 ISSUED change-managed regime — a mild attribution
  tension between the authority map (F-row: DEL-01-01 baseline) and R1
  IMPLEMENTATION_SURFACES/frontmatter (DEL-01-03 et al.). Not a live
  AUTHORITY_CONFLICT (the map row describes the change-control regime, not
  claim ownership), but worth a reviewer glance at wave fan-in.
- **`AuthorityNeeded` with a named decision ID vs OWNER:** REM-001 uses the
  named gate `D-07b` (§6 permits named decision IDs); REM-002 uses `OWNER`
  because "owner/counsel" is not a register decision ID.
- **Merge ≠ human disposition:** addendum 6 leaves open whether an owner
  merge of a PR containing an agent-authored record counts as a "recorded
  human disposition covering the cited record". I answered no (REM-003
  UNVERIFIED). A run-level ruling either way would remove per-pilot
  variance on every post-DEC-025 tranche record.

## 5. Boundary-compliance statement

- Discovery reads were confined to the frozen worktree
  (`.claude-worktrees/piping-frozen-551f84ef6`, HEAD verified
  `551f84ef6be656f1603ce0acfa5e3935aa9683c7`) and the run folder's R1
  artifacts. Writes were confined to exactly two files:
  `WAVES/W1/CLAIM_CONCORDANCE_DEL-01-03.csv` and this
  `WAVES/W1/NOTES_DEL-01-03.md` (plus a generator script in the session
  scratchpad outside both trees).
- `git status --porcelain` in the frozen worktree was empty before and
  after all evidence work; all in-tree executions used
  `PYTHONDONTWRITEBYTECODE=1` and wrote no artifacts.
- No lifecycle transition, DAG mutation, register edit, `_STATUS.md` edit,
  or product-file edit was made or proposed as applied.
- F-PIP-1..5 held: no release-readiness, issuance, certification, sealing,
  professional-approval, or code-compliance claim is made anywhere in the
  two output files; contributor "certification" is used strictly in its
  DCO-style repository-governance sense.
- All dispositions are agent judgments for reviewer/owner consumption;
  authority routing is via `AuthorityNeeded` only. No
  `DEFERRED_AGENT_WORKFLOW` matters arose beyond the friction notes above.
