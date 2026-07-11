# Notes — DEL-01-02 Copyright and protected-data boundary policy (W1 pilot)

Run: DELIVERABLE_CONCORDANCE_2026-07-11_1305, R2 wave W1. Frozen evidence tree
`551f84ef6be656f1603ce0acfa5e3935aa9683c7`. Ledger:
`WAVES/W1/CLAIM_CONCORDANCE_DEL-01-02.csv` (25 rows). Encoded under
`R1_CONVENTIONS.md` (conventions 1–8 + addenda 1–13).

**NormativeSource path alias (addendum 12, declared once for this ledger):**
bare kit filenames (`Specification.md`, `Datasheet.md`, `Guidance.md`,
`Procedure.md`, `MEMORY.md`, `_STATUS.md`, `_CONTEXT.md`, `_REFERENCES.md`,
`Dependencies.csv`, `_DEPENDENCIES.md`, `Review_Findings.csv`,
`_run_records/...`) resolve under
`execution/PKG-01_Governance, IP Boundary, and Professional Responsibility/1_Working/DEL-01-02_Copyright and protected-data boundary policy/`;
all other paths are relative to `projects/chirality-piping/` in the frozen tree.

**Requirement-ID mapping (kit scheme → addendum-12 ClaimID):**
DEL-01-02-R1→REQ-001, R2→REQ-002, R3→REQ-003, R4→REQ-004, R5→REQ-005,
R6→REQ-006, R7→REQ-007, R8→REQ-008, R9→REQ-009, R10→REQ-010, R11→REQ-011.

## 1. Histograms (recounted from the CSV before writing)

Disposition histogram (25 rows):

| Disposition | Count |
|---|---|
| ALIGNED | 23 |
| STALE_SETUP_SPECIFICATION | 1 |
| ACCEPTED_DIVERGENCE | 1 |

ClaimType histogram (25 rows):

| ClaimType | Count |
|---|---|
| REQUIREMENT | 11 |
| ACCEPTANCE | 6 |
| EXCLUSION | 1 |
| DECLARED_STATE | 6 |
| REMAINING_WORK | 1 |

Declared-state census (addendum 1): four kit surfaces + `_STATUS.md` +
`MEMORY.md` (both carry current-state declarations) = 6 rows. No
deliverable-owned in-tree README exists. No `IMPLEMENTED_UNMAPPED` rows: the
deliverable's material surfaces (`docs/IP_AND_DATA_BOUNDARY.md`,
`governance/CONTRIBUTION_REVIEW_CHECKLIST.md`) are mapped to DEL-01-02 via the
Datasheet/`_CONTEXT.md` anticipated-artifact records and the DEV-001 commit
`0d729cf` evidence chain. The seeded `(gated: D-41)` bootstrap item appears
verbatim only in DECL-005's `RecordedRemaining` per addendum 2.

## 2. Self-flagged rows

- **DEL-01-02-REQ-002** — kept ALIGNED at requirement grain (all required
  metadata fields exist) while the certification *mechanism/wording* remains a
  ruled-deferred TBD carried on REM-001. A reviewer could argue the row should
  inherit ACCEPTED_DIVERGENCE; addendum 11 reads to me as ALIGNED because the
  requirement demands fields, not the mechanism.
- **DEL-01-02-REQ-005** — vocabulary drift judgment: kit prose names
  `UNKNOWN_SOURCE` / `PROTECTED_CONTENT_SUSPECTED`; the implemented
  policy/checklist vocabulary is lowercase `unknown` / `protected_suspected`.
  I ledgered blocking substance as ALIGNED (MEDIUM) rather than
  IMPLEMENTED_DIFFERENTLY; reviewer eyes welcome on that threshold.
- **DEL-01-02-ACC-001** — census judgment: Specification Verification bullets
  1–2 (kit exists; default sections present) merged into one acceptance row;
  bullets 3–7 got one row each (ACC-002..006). The bullets are unlabeled, so
  the acceptance grain is my call under addendum 12.
- **DEL-01-02-DECL-002** — the STALE_SETUP_SPECIFICATION call rests solely on
  authority citations (Datasheet References declare decomposition revision
  0.7 as current and `DAG-006` as approved active graph authority; frozen tree
  is revision 0.8 / DAG-007). Substantive boundary declarations are current.
  Whether one overtaken References block makes the surface stale is a
  judgment the widened convention-1/addendum-4 definition leaves open.
- **DEL-01-02-REM-001** — encodes a recorded deferred residual that lives
  outside `_STATUS.md ## Remaining` (Review_Findings RF-001 +
  Dependencies E002/E003 PENDING). The brief's REMAINING_WORK census speaks of
  "real recorded residuals"; I judged these recorded-and-ruled items worth a
  row (ACCEPTED_DIVERGENCE per addendum 5/11) rather than silence. If the
  wave convention is _STATUS-items-only, this row collapses into notes.

## 3. Evidence-execution log

Re-executed (side-effect-free, addendum 9; `git -C FROZEN status --porcelain`
empty **before and after** every execution):

1. `PYTHONDONTWRITEBYTECODE=1 python3 -B tools/validation/validate_dependencies_schema.py "<FolderPath>/Dependencies.csv"`
   → `VALID`, 29 required columns + 0 extension, 13 data rows (matches the
   recorded 2026-06-04 formal-review result). Cited on ACC-006, DECL-004.
2. Read-only grep prohibited-claim scan over `docs/IP_AND_DATA_BOUNDARY.md`,
   `governance/CONTRIBUTION_REVIEW_CHECKLIST.md`, and `Specification.md`
   (terms: certif/seal/endors/legal advice/code compliance/approval). Every
   hit is negative boundary language, a quoted disclaimer, or a checklist
   field name; no affirmative claim found. Cited on REQ-008, ACC-004, EXC-001.
3. Direct read-only inspections at the frozen SHA: kit files, repo-level
   policy/checklist (field-by-field against R2/R11), decomposition revision
   header (0.8) and AB-00-06/AB-00-08 rows, `_DAG/_LATEST.md` (DAG-007),
   SOW-003/SOW-028/OBJ-002/DEC-003/DEC-027 rows, both LifecycleCorrection
   `Decision_Log.md` records, `TP-PKG01-CHECKING-TRANSITION-DEL-01-02_2026-06-04.md`.

Cited as recorded (not re-executed):

- `_run_records/TASK_RUN_2026-06-04_DEL-01-02_formal-review.md`
  (PASS_WITH_WARNINGS; dependency-schema, `git diff --check`,
  prohibited-claim, coverage checks) — cited with the exact marker
  `not re-executed at frozen SHA 551f84ef6`; its two mechanical checks were
  independently reproduced at the frozen SHA (items 1–2 above), so no
  content-identical qualifier was needed.
- DEV-001 committed evidence `0d729cf` via the MEMORY.md TP-RECON-01
  reconciliation record (file-set of the bounded implementation).

No addendum-10 `content-identical` qualifier used anywhere (no ancestor-commit
diff was run; alignment rests on direct frozen-SHA inspection instead).

## 4. Convention friction notes

- **Non-census kit-adjacent drift home:** `_CONTEXT.md` ("Accepted Revision:
  0.7", "Status: current_basis") and `_REFERENCES.md` (DAG-006 as approved
  authority) carry the same overtaken authority citations as the Datasheet,
  but addendum 1's census gives them no DECLARED_STATE row. Recorded here and
  on DECL-002's RemainingWork; a census extension for `_CONTEXT.md` would have
  given this drift a first-class home.
- **REMAINING_WORK census boundary:** the binding set defines residual rows
  around `_STATUS.md ## Remaining`, but this deliverable's only real residual
  is recorded in `Review_Findings.csv` + `Dependencies.csv` (ruled-deferred).
  Addendum 5 handles the disposition cleanly; *where* such items sit in the
  row census is left open (see self-flag on REM-001).
- **R1 index gap (orientation only):** `IMPLEMENTATION_SURFACES.csv` has no
  rows for `docs/IP_AND_DATA_BOUNDARY.md` or
  `governance/CONTRIBUTION_REVIEW_CHECKLIST.md` — the two material surfaces of
  this deliverable — though it indexes sibling governance templates
  (SURF-142, SURF-169). Not a concordance defect (both surfaces are mapped,
  so no UNMAP row arises), but wave fan-in should not treat that index as a
  complete governance-surface census.
- **Acceptance grain for unlabeled Verification bullets:** addendum 12 fixes
  the ClaimID form but not how to segment an unlabeled Specification
  Verification section; I used one row per distinct check, merging the two
  presence bullets.

## 5. Boundary-compliance statement

- All discovery reads were against the frozen worktree at
  `551f84ef6be656f1603ce0acfa5e3935aa9683c7` (HEAD verified). `git status
  --porcelain` on the frozen worktree was empty before and after every
  re-execution and at close-out.
- Writes were confined to exactly two files:
  `WAVES/W1/CLAIM_CONCORDANCE_DEL-01-02.csv` and `WAVES/W1/NOTES_DEL-01-02.md`.
  No lifecycle transition, DAG mutation, register edit, `_STATUS.md` edit,
  cross-project edit, or product-file edit was made or proposed.
- F-PIP-1..5 held: nothing in the ledger or these notes is a
  release-readiness, issuance, certification, sealing, professional-approval,
  or code-compliance claim. Where policy disclaimers are reproduced they are
  quoted text of the frozen artifacts; ALIGNED cells assert only that the
  frozen artifacts' text satisfies the deliverable's own requirement wording.
  All dispositions are agent judgments routed through `AuthorityNeeded`;
  none is an owner or engineering ruling.
- No agent-workflow implication arose; no `DEFERRED_AGENT_WORKFLOW`
  disposition was needed.
