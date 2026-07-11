# NOTES — DEL-00-03 Application service command-query-job model (W1)

Run: DELIVERABLE_CONCORDANCE_2026-07-11_1305, R2 wave W1. Frozen source tree
`main` @ `551f84ef6be656f1603ce0acfa5e3935aa9683c7`. Binding encoding rules:
`R1_CONVENTIONS.md` (conventions 1–8 + addenda 1–13). Ledger:
`CLAIM_CONCORDANCE_DEL-00-03.csv` (19 rows, 20 columns, RFC-4180 clean).

## Deliverable character

PKG-00 architecture-runway **documentation** deliverable (Type `API_CONTRACT`),
lifecycle `IN_PROGRESS`. It defines service contracts and flow rules only and
authorizes **no** product implementation code (Datasheet Scope Boundary /
Boundary Confirmation). Requirements `REQ-03-01..05` are architecture-definition
obligations whose evidence bar is "Acceptance review" / "Human review"; the
deliverable is prepared for review, not accepted. No implementation surface maps
to DEL-00-03 (R1 `IMPLEMENTATION_SURFACES.csv` has no DEL-00-03 row); the
anticipated artifact `docs/architecture/application_services.md` does not exist
in the frozen tree.

## Requirement-ID scheme mapping (addendum 12)

Requirement scheme `REQ-03-*` is non-self-identifying (tokens do not embed the
deliverable ID). ClaimID uses the fixed addendum-12 form `DEL-00-03-<TYPE>-NNN`.
Requirement mapping:

| Deliverable requirement ID | Ledger ClaimID |
|---|---|
| REQ-03-01 | DEL-00-03-REQ-001 |
| REQ-03-02 | DEL-00-03-REQ-002 |
| REQ-03-03 | DEL-00-03-REQ-003 |
| REQ-03-04 | DEL-00-03-REQ-004 |
| REQ-03-05 | DEL-00-03-REQ-005 |

## NormativeSource path alias (addendum 12)

`<DELIV>` = `execution/PKG-00_Software Architecture Runway/1_Working/DEL-00-03_Application service command-query-job model/`.
All bare `*.md` surface names in the ledger (`Specification.md`, `Datasheet.md`,
`Guidance.md`, `Procedure.md`, `_STATUS.md`, `MEMORY.md`, `_SEMANTIC.md`,
`_SEMANTIC_LENSING.md`) are relative to `<DELIV>`.

## Disposition histogram (reproduces from the CSV)

- ALIGNED — 16
- STALE_SETUP_SPECIFICATION — 3

Total: 19.

## ClaimType histogram (reproduces from the CSV)

- REQUIREMENT — 5
- ACCEPTANCE — 4
- EXCLUSION — 4
- DECLARED_STATE — 6

Total: 19.

## Row census rationale

- **REQUIREMENT (5):** one per current requirement ID `REQ-03-01..05`. Each is a
  documentation obligation stated at architecture-runway grain; substance
  disposition per convention 1 (requirement rows never take
  `STALE_SETUP_SPECIFICATION`). All ALIGNED: the obligation is articulated in the
  kit and consistent with the deliverable's declared `SEMANTIC_READY`/`IN_PROGRESS`
  posture, with concrete realization deferred to visible TBDs (blessed by the
  acceptance criterion "TBD decisions are visible and routed to human ruling").
- **ACCEPTANCE (4):** the Specification "Acceptance Criteria" bullets that are
  genuine acceptance gates and not requirement restatements — kit-exists+identity,
  scope-limited-to-PKG-00, TBDs-visible-and-routed, semantic-artifacts-exist. The
  two Acceptance-Criteria bullets "no implementation code created" and "no
  protected data introduced" are **not** duplicated as ACCEPTANCE rows; they are
  ledgered as EXCLUSION rows EXC-001 and EXC-003 (they are scope exclusions in
  substance) to avoid double-counting.
- **EXCLUSION (4):** the deliverable's distinct scope exclusions — no product
  implementation code (EXC-001), no PKG-01..12 authorization/advancement
  (EXC-002), no protected standards/proprietary data (EXC-003, OPS-K-IP-1), no
  code-compliance/certification/professional-approval claim (EXC-004,
  OPS-K-AUTH-1). Recording these exclusions asserts no positive certification or
  release claim (F-PIP-1..5 held).
- **DECLARED_STATE (6):** exactly one per four-document kit surface
  (Specification, Datasheet, Guidance, Procedure) plus `_STATUS.md` and
  `MEMORY.md` (both carry state declarations), per addendum 1. No deliverable-owned
  in-tree README exists. `_SEMANTIC.md` / `_SEMANTIC_LENSING.md` / `_CONTEXT.md` /
  `_REFERENCES.md` / `_DEPENDENCIES.md` are outside the addendum-1 census and get
  no DECLARED_STATE row (their content is cross-referenced where relevant).
- **REMAINING_WORK (0):** the only recorded `## Remaining` item is the seeded
  `(gated: D-41)` concordance bootstrap item, recorded verbatim only in the
  `_STATUS.md` surface row's `RecordedRemaining` (DECL-005) and excluded from all
  residual/gate/selectability analysis (addendum 2). R1
  `DELIVERABLE_INVENTORY.csv` records `NonBootstrapItems=NONE`. No real residual.
- **IMPLEMENTED_UNMAPPED (0):** no material implementation surface (crate, binary,
  schema, app panel) is in DEL-00-03's orbit; R1 `IMPLEMENTATION_SURFACES.csv` has
  no DEL-00-03 row; `application_services.md` is unbuilt. Nothing to map.

## The three STALE_SETUP_SPECIFICATION calls (rev-0.7 authority-citation drift)

DECL-001 (Specification), DECL-002 (Datasheet), and DECL-004 (Procedure) each
cite `SOFTWARE_DECOMP.md` **revision 0.7** as the current/upstream basis
(Specification line 32; Datasheet line 33; Procedure line 8). The frozen tree's
`SOFTWARE_DECOMP.md` is **revision 0.8** (header `revision: 0.8`; records SCA-005
/ D-21; RUN_BASIS confirms v0.8 with DEC-072 carried through). Under the widened
`STALE_SETUP_SPECIFICATION` definition (addendum 4 — "post-alignment drift"),
these declared-state surfaces carry a stale authority-revision citation. Context
recorded for the reviewer:

- The lag is **sanctioned**: `SOFTWARE_DECOMP.md` (~line 655) states downstream
  production documents "may be stale relative to revision 0.8 until refreshed by
  their owning workflows." The deliverable's last authority refresh
  (`TASK_RUN_2026-06-04_authority-refresh-0.7-dag006`) refreshed to 0.7/DAG-006; a
  later refresh advanced DAG-006→DAG-007 on most surfaces but no 0.8 refresh has
  run. This is a pending owning-workflow refresh, not an authority question:
  `AuthorityNeeded=NO`.
- The drift is **immaterial to substance**: rev 0.8 (SCA-005 D-21 propagation +
  PKG-17 export interoperability) does not alter the command-query-job model
  scope. The disposition flags the citation, not a substance defect.
- `Guidance.md` (DECL-003) carries no revision citation → ALIGNED. `_STATUS.md`
  (DECL-005) is current (2026-07-11) → ALIGNED. `MEMORY.md` (DECL-006) cites 0.7
  and DAG-006 only inside a **dated 2026-06-04 log entry**; per addendum 1 that is
  a historical record — the drift is noted on the surface row, not dispositioned
  as staleness → ALIGNED.

## Self-flagged rows

- **DEL-00-03-DECL-001 / DECL-002 / DECL-004** — judgment call the conventions
  leave partly open: whether a one-revision-behind, decomp-sanctioned, substance-
  immaterial authority citation warrants `STALE_SETUP_SPECIFICATION` vs `ALIGNED`.
  I chose the honest drift-flag (convention 1 puts staleness on declared-state
  rows; addendum 4 covers post-alignment drift). Reviewer may prefer ALIGNED-with-
  note given the sanction; disposition is at surface grain while only the revision
  pointer (not the surface's substance) is stale.
- **DEL-00-03-REQ-003 / REQ-004** — MEDIUM confidence. Judgment that "define
  transaction boundaries" (REQ-03-03) and "require cancellation/progress/
  reproducibility metadata" (REQ-03-04) are ALIGNED at architecture-runway grain
  even though concrete mechanisms (transaction persistence; cancellation token API
  shape; job runner) are named TBDs. Read as ALIGNED because the requirement is a
  documentation obligation to *define/require*, met by the kit, with concrete
  realization appropriately deferred and visible — not PARTIALLY_IMPLEMENTED
  (there is no partial *implementation*; there is no implementation at all in this
  documentation deliverable).
- **DEL-00-03-EXC-003** — MEDIUM confidence. "No protected standards/proprietary
  data" rests on a non-exhaustive agent content scan of generic architecture
  prose; no protected payload observed, but exhaustive certification is out of
  scope and not claimed (F-PIP boundary).
- **General (requirement rows):** ALIGNED for REQ rows rests on runway-grain
  articulation plus a pending (not failed) human review; `SourceReliability=
  UNVERIFIED` per addendum 6 (agent-generated architecture prose, `_SEMANTIC.md`
  Audit PASS, human disposition pending). The prior PKG00 lock review
  (2026-05-11) was administratively set aside by the D-40 rebaseline (DEC-072) and
  preserved as historical evidence available for future `CHECKING` re-entry — it
  is **not** cited as current acceptance evidence on any row.

## Evidence-execution log

Read-only discovery only; **no re-executable tests exist** for this documentation
deliverable, so nothing was re-executed. Checks run (all read-only, side-effect-
free, inside/over the frozen tree):

- `find <DELIV> -type f ! -name "*.md"` → 0 (backs EXC-001 / ACC / "no code").
- Identity grep `DEL-00-03` per kit doc → Specification 2, Datasheet 3, Guidance
  1, Procedure 1 (backs ACC-001).
- `grep -n` rev-0.7 citations → Specification:32, Datasheet:33, Procedure:8
  (backs DECL-001/002/004); confirmed frozen `SOFTWARE_DECOMP.md` header
  `revision: 0.8`.
- `grep -niE certif|seal|code.compliance|professional approval|authenticate`
  over kit → only disclaimers/boundary-preservation statements (backs EXC-004).
- R1 index cross-checks (re-verified against the frozen tree): DEL-00-03 present
  in `DELIVERABLE_INVENTORY.csv` (`IN_PROGRESS`; `RequirementIDs=REQ-03-01..05`;
  `NonBootstrapItems=NONE`; `SelectableUnderCurrentLoop=NO`); **absent** from
  `IMPLEMENTATION_SURFACES.csv`, `VERIFICATION_INDEX.csv`,
  `VALIDATION_AND_PROVENANCE_INDEX.csv`, `DECISIONS_INDEX.csv`.
- `git -C FROZEN status --porcelain` run before and after all reads and after CSV
  generation → **empty every time** (0 lines). CSV generated with
  `PYTHONDONTWRITEBYTECODE=1`; all build/temp artifacts confined to the scratch
  dir; nothing written under the frozen tree.

Recorded-pass citations: none applicable (no test suites; nothing to cite with
the `not re-executed at frozen SHA 551f84ef6` marker beyond the human-review
evidence-bar notes carried inline on REQ/ACC rows).

## Convention friction notes

1. **Documentation-deliverable evidence columns.** Conventions/§6 are written for
   code-bearing deliverables (Implementation/Verification/Validation evidence).
   For a pure architecture-documentation deliverable the honest values are the
   surface locations of the articulated obligation (Implementation), a
   `NONE_FOUND`-with-reason for Verification (acceptance is human review, not a
   test), and `NOT_APPLICABLE`-with-reason for Validation. Flagged so aggregation
   does not read `NONE_FOUND` verification as a defect.
2. **Internal kit tension (not dispositioned).** Datasheet "Write boundary:
   Deliverable-local document kit and semantic artifacts only" vs Datasheet/
   _CONTEXT "Anticipated Artifacts / Outputs Expected: docs/architecture/
   application_services.md; service interface contracts." The concrete artifact is
   outside the deliverable-local write boundary yet listed as an expected output.
   I treated `application_services.md` as a forward/anticipated artifact (correct
   for a SEMANTIC_READY runway; not in `## Remaining`), so it is not a residual
   row — but the wording tension is a candidate for a future documentation
   clean-up by the owning workflow. Noted, not encoded as a claim row.
3. **Acceptance vs exclusion overlap.** The Specification lists "no code" and "no
   protected data" as both Acceptance Criteria and (via Datasheet/Invariants)
   scope exclusions. Resolved by ledgering each once, as EXCLUSION rows, to keep
   the ClaimType histogram meaningful (addendum 12: acceptance rows never merged
   with requirement rows; here the analogous care avoids acceptance/exclusion
   double-counting).

## Boundary-compliance statement

- Discovery was strictly READ-ONLY outside the two W1 output files
  (`CLAIM_CONCORDANCE_DEL-00-03.csv`, `NOTES_DEL-00-03.md`). No `_STATUS.md`,
  register, product, or DAG file was edited; no lifecycle transition applied
  (`LIFECYCLE_REASSESSMENT_REQUIRED` was not needed and not applied); no
  cross-project edits.
- F-PIP-1..5 held: no release-readiness, issuance, certification, sealing,
  professional-approval, or code-compliance claim appears anywhere in the ledger
  or these notes (exclusion rows assert the *absence* of such claims, consistent
  with the fences).
- All dispositions are agent judgments routed via `AuthorityNeeded`
  (all `NO` here); none phrased as owner or engineering rulings. No
  `DEFERRED_AGENT_WORKFLOW` items arose.
- Frozen tree porcelain clean before, during, and after (0 lines each check).
  Writes confined to the two output files under `WAVES/W1/`.
