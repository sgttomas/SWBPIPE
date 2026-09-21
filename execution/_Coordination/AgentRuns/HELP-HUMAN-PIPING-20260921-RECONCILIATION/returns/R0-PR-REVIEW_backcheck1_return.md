VERDICT: FINDINGS

The delta fully closes 16 of the 18 findings. Finding 1 is only partly closed, because the new Scope Detail check reports four false drifts (N2). Finding 6 is only partly closed, because the batch mode that was meant to replace per-hash rows falsely flags conforming ledgers (N1) and still needs the owner's confirmation. I found two new actionable defects and four minor ones.

Delta reviewed: `26e617022..6eba10fdb`, 15 files. I checked the repository's HEAD at `6eba10fdb194409c58001095c84d93be180c4611` and compared against the freeze checkout at `00115c71931bcae79909602d653740d3bb72dfa1`.

## Reruns (item 2)

- **`extract_claims_v2.py`** produces a `CLAIM_KEYS_V2.csv` byte-identical to the committed one (SHA `e1d3f411…`).
  - 12,854 units, 7,257 required, 808 pre-typed.
- **`build_canonical.py`** produces `EVIDENCE_MAP.csv` (`8ec70b7f…`) and `CANONICAL_ASSIGNMENTS.csv` (`02f4c435…`), both byte-identical.
  - 501 parity records and 729 canonical assignments.
- **The latest `BOUND_INPUTS` event** lists 9 files; the hashes I recomputed (all but `CANONICAL_SITUATIONS.md`) match.
- **Only one key changed.** Compared with the old keys, the key set is the same, and every `ParentKey`, `LineStart`, `Required` and `UnitKind` is unchanged. The only unit that changed is `DEL-17-06:SOW#CLM-042` (its `LineEnd`, `PreType` and `TextSHA256`).

## Per-finding closure (item 1)

1. **Partly closed.** Package Reference is now compared on name, scope, assigned items and exclusions. The 16 drifts I reported are now `CS-06-DRIFT`. The new Scope Detail extension, however, adds 4 false drifts (N2).
2. **Closed.**
   - CLM-042 is now bounded at lines 525–532, is not pre-typed, and is required.
   - The section wrapper is computed over every line it owns.
   - The lost-line scan finds 0 substantive lines whose innermost required unit is pre-typed. No v1 text is lost (item 4).
3. **Closed.** `BodySHA256` excludes the heading line. The corpus now has 26 `DuplicateOf` entries (25 of them PDU-054/055 declarations); before the fix there was 1.
4. **Closed.** The evidence map now finds parity records by content signature: 501 records covering 88 of 93 SOW deliverables.
   - Five deliverables have `NONE_FOUND`: DEL-01-01 and DEL-13-01..04.
   - `AnyPassMatchesFrozen=YES` for 9 deliverables. That includes DEL-03-01, 03-02, 03-05 and 03-07, as I had found.
5. **Closed.** CS-07 now covers Context Budget QA only (92 blocks). Context Envelope blocks are judged normally.
6. **Partly closed.**
   - The shared bodies that still have no keyed row are handled by a consistency rationale, batch-mode checks and 100% verifier review.
   - That departs from the adopted `R0_REVIEW.md` §5 item 5 ("one canonical row per hash"). `CONVENTIONS.md` discloses the departure but does not route it to the owner (see owner items).
   - The batch mode itself has a defect (N1).
7. **Closed.** The new precedence clause says the adopted texts govern, and a difference from them is a defect.
8. **Closed.** The INVARIANT subject list, "specification" and the C4 value glosses are restored. The PROTECTED_CHECK sentence is reworded and labelled AGENT.
9. **Closed.**
   - C1 now documents the optional, all-or-none `.rNN` rule and allows a split block to be assessed directly.
   - Synthetic test 04 (a split with the parent assessed directly) now passes.
   - Test 05 (`.sNN` sub-claims used in place of `.rNN` keys) is now rejected.
10. **Closed, with one residual (N6).** Every earlier gap now fails correctly: empty evidence columns, `LIFECYCLE_REASSESSMENT_REQUIRED` without a group, an accepted divergence based on `NOT_APPLICABLE` or DEC-101, a Remaining-mismatch disposition on a non-Remaining unit, a `GATE:` token outside `GATE_EVIDENCE`, layers on a quiet row, a UnitKind mismatch, and a bare `CANONICAL_DEPARTURE`. The earlier reverse-answer gaps also fail now: an empty ClaimKey, the Remaining block key, and a duplicate CapabilityID.
11. **Closed.** `RESUME.md` now points at v2, `--batch` and the latest `BOUND_INPUTS` event.
12. **Closed.**
    - CP-04's `AuthorityNeeded` is now OWNER.
    - The hard identifiers are now sourced to `ACTIVATION_2026-09-18.md` §2. I recomputed the stored selection: 1,139 bytes, SHA `96a3d2bb…`, matching the record, and it contains "Keep all four as they are (Recommended)".
13. **Closed.** CS-04 now covers the revision pin only. A `.sNN` row is required when "Resolved Baseline", "Applicable Basis IDs", the `SEMANTIC_READY` statement or a "Still TBD" item diverges.
14. **Closed.** CS-01's basis now states why it picks `STALE_REVIEW_OR_EVIDENCE`.
15. **Closed.**
    - AGENT labels are added.
    - These are restored: the "never mint" rule, MEMORY as a single history unit, the history rule, "by itself", BaselineClass empty on quiet rows, R3 clustering, and the ISSUED change path.
    - Project-root tokens are allowed again.
16. **Closed.** `RUN_BASIS.md` discloses the absolute paths in the gate-evidence copies.
17. **Closed.** The missing fences are recorded in the `RUN_STATE` `REPAIR` event; the sealed files are unpatched.
18. **Closed.** The ScopeLedger data (`sreg`) is now used, by the Scope Detail check.

## Validator reruns (item 3)

- **Sentinel.** The count now sits in `Notes` for forward ledgers and `Reason` for reverse answers, and the new validator enforces this.
- **Single mode.**
  - The conforming DEL-00-05 ledger passes.
  - Cases 01–03, 05–11 and `r_bad` fail as they should.
  - Cases 04, 12, 13, 16 and 17 pass as they should.
  - Case 14 fails; see N5.
  - Case 15 passes but should warn; see N4.
- **Batch mode.** I generated conforming ledgers for DEL-00-05, 01-01, 01-02, 07-01 and 07-02. Each passes on its own, except that my synthetic 07-02 ledger correctly fails for a missing `DUPLICATE_OF`. Together in batch mode they fail with 4 false findings (N1).

## New findings (item 5)

- **N1. ACTIONABLE — batch mode flags ledgers that follow their canonical rows exactly.**
  - Where: `tools/validate_ledger_v2.py:286-287`, `:288-298`.
  - Evidence:
    - Batch mode groups rows by `CanonicalSituation` alone, and the ledger value is `CS-06` without the OK/DRIFT variant.
    - In my test, the DEL-01-01, 01-02, 07-01 and 07-02 Package Reference rows follow their `CS-06-DRIFT` assignment exactly. They are still flagged "differs from the majority without CANONICAL_DEPARTURE" (4 findings).
    - The same will happen for CP patterns that allow more than one outcome: CP-01, CP-08, CP-09 and CP-11.
  - Smallest fix: group CS rows by `Variant` from `CANONICAL_ASSIGNMENTS.csv`, or drop CS grouping, since single mode already checks canonical fields. For CP groups, compare only rows with the same disposition, or report them as warnings.

- **N2. ACTIONABLE — the four Scope Detail drifts are formatting defects, not stale text.**
  - Where: `tools/build_canonical.py:176-181`; `CANONICAL_SITUATIONS.md:35` ("four Scope Detail statements").
  - Evidence:
    - All four blocks (DEL-13-02, 14-03, 14-04, 16-03) have two items run together on one line, as in `…messages.- SOW-067: …`.
    - For DEL-13-02, the SOW-068 statement is identical to ScopeLedger and to the decomposition. The joined second item is never compared.
    - The rows would still inherit `BASIS_POINTER_STALE`, and the table says the statements differ, when they do not.
  - Smallest fix: split on each `- SOW-\d{3}:` occurrence. When every statement matches but items are joined, record a formatting defect (for example `RECORD_DRIFT`), or leave the block unkeyed. Then correct the sentence in the table.

- **N3. MINOR — the stored review return contains absolute scratch paths.**
  - Where: `returns/R0-PR-REVIEW_return.md:172` and its final paragraph.
  - Evidence: the harness now raises `REVIEW ABS_PATH_IN_UNCLASSIFIED_SURFACE` on this file. It is the one new REVIEW (5 instead of 4).
  - Smallest fix: disclose it as a verbatim copy, as was done for the gate evidence, or redact it with a note.

- **N4. MINOR — project documents cited from the project root silently resolve to the root copy.**
  - Where: `validate_ledger_v2.py:73`, `:116-119`; `CONVENTIONS.md` Part D.
  - Evidence: `docs/…` resolves at the repository root. Root and project each have `SPEC.md`, `CONTRACT.md`, `TYPES.md` and `DIRECTIVE.md`. A worker citing the project's `docs/SPEC.md` passes (case 15), but the path is checked against the wrong file.
  - Smallest fix: state in Part D that project documents must be cited as `projects/chirality-piping/docs/…`. Optionally warn when a root `docs/` token's basename also exists in the project.

- **N5. MINOR — a bare top-level project directory is rejected.**
  - Where: `validate_ledger_v2.py:110`.
  - Evidence: the token is stripped of its trailing `/` before the prefix check, so `core/` becomes `core`, which the project-root prefix no longer matches (case 14).
  - Smallest fix: match the prefix before stripping.

- **N6. MINOR (residual of 10) — duplicate units are not checked for the same disposition.**
  - Where: `validate_ledger_v2.py:211-212`.
  - Evidence: C1 says a `DUPLICATE_OF` row "takes the same disposition", but only the Notes marker is checked. Batch mode groups only bodies shared across deliverables. There are now 26 in-deliverable duplicates.
  - Smallest fix: in single mode, compare the consistency fields of each row against its `DuplicateOf` target.

## Owner items (item 6)

The three items are stated correctly, and one more needs to go to the owner.

- **The `.rNN` departure.** Correctly stated. It is disclosed in `RUN_STATE.jsonl`, `CONVENTIONS.md` C1 and the disclosed-narrowings section as pending owner confirmation.
- **CP-04.** Correctly stated: `AuthorityNeeded` is OWNER, and the hard identifiers rest on the A3a record whose hash I verified.
  - One point to put to the owner with it: A3a says reliance on such a record goes to R4 for confirmation. CP-04 uses the record to exclude the four identifiers from residue but does not route that reliance to R4.
- **The D-73 addendum reading.** Correctly stated. It is implicitly supported by the R0 ruling's adoption of the `GATE:` token.
- **Add a fourth item: the departure from §5 item 5.** Consistency enforcement replaces one canonical row per shared hash. It departs from adopted text. Under the new precedence clause that is a defect unless the owner confirms it.

## Validator outputs (`PYTHONDONTWRITEBYTECODE=1`)

- `validate_claims_language.py`: `VALID claims-language surfaces: 356 files scanned; DEC-081 registry taxonomy satisfied` (exit 0).
- `validate_piping_loop_receipts.py --repo-root .`: `VALID …/projects/chirality-piping/loop/LOOP_RECEIPTS.md: frozen through Receipt-44; versioned receipt contract satisfied` (exit 0).
- `harness.py self-check`: exit 0, **0 BLOCK** (INFO 14, NOT_APPLICABLE 1, REVIEW 5, WARN 112). The only finding on this run's files is the REVIEW in N3.

Scratch files for the backcheck are under `/private/tmp/claude-501/-Users-ryan-dev-chirality--claude-worktrees-task-management-gen-pass-518da2/efe0b4ff-c1b6-4a97-af70-b04220464651/scratchpad/review/bc/` and `…/review/val2/`, with the batch ledgers in `val2/batch/`.

END-OF-RETURN