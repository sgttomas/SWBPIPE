# DEL-04-03 — Linear support and restraint models: worker notes (W2, PKG-04)

Run `HELP-HUMAN-PIPING-20260921-RECONCILIATION`, gate wave W2, worker G1
(TASK). Frozen state `00115c71931bcae79909602d653740d3bb72dfa1`. Forward
ledger sealed before the routing file was read (B1). Agent judgments only;
nothing here is an owner ruling. Shared treatments are in
`../_WORKER_DEL-04-01_NOTES.md`.

## Path aliases

- Deliverable folder paths contain spaces. The 2026-06-05 review-readiness
  record (`_run_records/TASK_RUN_2026-06-05_2221_TP-DEL-04-03-04-06_REVIEW-READINESS_A.md`)
  is named in Notes, not tokenized.
- Parity records are cited in their repository-root form.
- `open_pipe_stress_frame_kernel` (named in the SOW) is the package name of
  `core/solver/frame_kernel`.

## Judgment calls

- **SURFACE row:** CP-04 rename residue, recorded once with the default
  fields and AuthorityNeeded OWNER. The SOW names the active identifier
  `open_pipe_stress_frame_kernel` four times: CLM-003, CLM-012, CLM-018 and
  CLM-024. The frontmatter pin moves to the `SOW.s01` sub-claim under CP-02.
- **CLM-010 split:** all twelve `.rNN` keys are present and the block is a
  CONTAINER.
- **R12 (`CLM-010.r12`):** IMPLEMENTED_DIFFERENTLY · OWNERSHIP_ELSEWHERE ·
  LOCAL_DESIGN. Product support application does not call
  `apply_linear_supports`. The product adds spring stiffness itself and
  reduces through frame-kernel `reduce_system` or its sparse reduced-entry
  assembly. `apply_linear_supports` is called only by crate tests, one
  product unit test and the mechanics benchmarks. The descriptive rows
  CLM-003 and CLM-025 stay ALIGNED with `PRODUCT_CALLER: NONE` (F7).
- **R10 and R11:** ALIGNED. The conditional rule "remain TBD until
  resolved" was followed; DEC-050 and DEC-053 settled sparse integration.
  The crate itself does no sparse integration, so R11 still holds.
- **Test counts (FG-DEL-04-03-02: CLM-005, CLM-018):**
  STALE_REVIEW_OR_EVIDENCE · DOC_BEHIND_CODE. The crate has 15 tests at the
  freeze; the texts, dated 2026-06-05, present 14 as current. The dated
  CLM-012 record "June 5 evidence records 14" is accurate as a record.
- **Protected-content review (FG-DEL-04-03-03: CLM-012):** the block is not
  split, so its worst gap governs. The review-readiness check dated
  2026-06-05 was followed by the 2026-09-05 spring validity repair, and no
  DEC-058 scan record exists. Fields: STALE_REVIEW_OR_EVIDENCE ·
  EVIDENCE_OVERTAKEN · INVARIANT · IP_DATA;RECORD · REVIEW.
- **AC-001:** STALE_REVIEW_OR_EVIDENCE. Constant-effort-hanger work landed
  2026-07-19 (R14-W1-T2).
- **Four-document residue (CP-01):** CLM-019 and CLM-020 are
  STALE_SETUP_SPECIFICATION (first present at 7bee9ae41). CLM-013 is
  STALE_REVIEW_OR_EVIDENCE (first present 2026-06-05, 2c93dea83).
- **D-41 declarations:** CLM-007 and its duplicates CLM-014, CLM-021 and
  CLM-029 are accurate. They pin no revision, so they take CP-03 · ALIGNED.

## Canonical departures

None.

## Convention friction

- CP-04 and the frontmatter pin compete for the single SURFACE row, so the
  pin was moved to `SOW.s01`.
- CLM-012 was left unsplit (its `.rNN` keys are optional). As a result the
  IP-review gap governs the whole verification block. Splitting it would
  isolate that gap to r05 (R06).

## UNKNOWN rows

None.

## Reverse pass

- 3 CLAIMED_BY, 4 PARTIAL, 3 COVERS, 0 UNKEYED, 330 NOT_MINE.
- DEC-049 hanger handling (RC-04-0178) and constant-effort consumption
  (RC-04-0026) are PARTIAL. The code sits in product_physics, but CLM-007
  declares DEC-049 hanger user data part of this slice, and R14-W1-T2 was a
  DEL-04-03 tranche.
- The hanger library schema and GUI capabilities (RC-04-0113, 0145, 0192,
  0238) are answered NOT_MINE as library and GUI surfaces. R3 may weigh
  them against CLM-007.
- Did the reverse pass change my view of anything sealed? No. It confirms
  R12's departure: RC-04-0190 exists with no product caller, and RC-04-0136
  is the product's own application path.

## Batch consistency

`--batch` over the three sealed forward ledgers: PASS, 0 findings.

## Selectability

`SelectableUnderCurrentLoop` is `NOT_APPLICABLE` on every row (C9).

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
