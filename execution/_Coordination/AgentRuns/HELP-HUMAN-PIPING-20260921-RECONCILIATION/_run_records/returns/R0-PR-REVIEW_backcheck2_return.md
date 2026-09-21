VERDICT: FINDINGS

Backcheck 2 of the R0 PR review, covering `6eba10fdb..2b664486a` (10 files); the last commit is `2b664486a`. N1, N2, N5 and N6 are closed, N4 is partly closed, and N3 is disclosed. There is one actionable finding and two minor ones, all in CP-04's rename wording or path checking; no further owner ruling is needed.

The working tree also has uncommitted Agent 0 files outside this delta: R1 index CSVs, `tools/build_r1_indexes.py`, and an edited `RUN_STATE.jsonl`. I did not review them.

## Closure

- **N1 — closed.**
  - The conforming DEL-00-05, 01-01, 01-02, 07-01 and 07-02 set now passes batch mode with 0 findings. Before the fix it produced 4 false findings.
  - In a modified set, a real inconsistency is flagged: DEL-07-02 R03 against DEL-07-01 R04, which share a body.
  - CP-08 rows with two different allowed outcomes are not flagged.
- **N2 — closed.**
  - `build_canonical.py` rerun into scratch: `CANONICAL_ASSIGNMENTS.csv` (`5f5380a3…`) and `EVIDENCE_MAP.csv` (`8ec70b7f…`) are byte-identical.
  - There are now 725 assignments: 306 CS-06 OK and 16 DRIFT, all 16 in Package Reference.
  - The four run-together Scope Detail blocks (DEL-13-02, 14-03, 14-04, 16-03) now have no canonical row and are judged normally. `CANONICAL_SITUATIONS.md` CS-06 says this correctly.
- **N3 — disclosed** in `RUN_BASIS.md`.
  - The harness now reports REVIEW=6, up from 5 at backcheck 1. The extra one comes from the stored copy of the backcheck 1 return, which has the same scratch paths. The disclosure covers it.
- **N4 — partly closed.**
  - Case 15 (a bare `docs/SPEC.md` where a project copy exists) now fails.
  - The `ROOT_DOC:` note lets such a token pass.
  - A root-only `docs/PRD_ROOT.md` passes.
  - The gap: see B2-3.
- **N5 — closed.** `core/` now passes. My earlier case 14 used a bare `core` with no slash, which correctly still fails.
- **N6 — closed.** A duplicate row whose disposition differs from its `DUPLICATE_OF` target now fails, citing C1.

## Owner items

- **Recording is faithful.**
  - Both verbatim blocks in the `R0_RULING.md` addendum match their stated sizes and SHA-256: 1,209 bytes (`53862dca…`) and 651 bytes (`7b51877b…`).
  - The owner's double space ("past.  If") is preserved.
  - `BOUND_INPUTS` and `WORK_GRAPH` hashes match the files, including `R0_RULING.md` (`a922f21f…`) and the backcheck 1 return (`f8b92cd6…`).
- **Item 1 (optional `.rNN` rows)** is implemented exactly, in C1 ("owner-confirmed") and in the narrowings list.
- **Item 3 (consistency enforcement)** is implemented as described to the owner.
- **Item 2 (rename residue).** Agent 0's reading is reasonable: the four identifiers do name active code, so the owner's condition applies. The addendum correctly labels it "Agent 0's reading … The owner may correct it". Two problems remain:
  - B2-1: CP-04 and the narrowings present the reading more strongly than the addendum does.
  - B2-2: CP-04's FROZEN_CONTRACT rows take tier and layer values that contradict C3 and C5.
- **A3a routing is moot, and I agree.** The four identifiers are no longer exceptions, so nothing rests on the 2026-09-18 record, and it is correctly recorded as context.

## New findings

- **B2-2. ACTIONABLE — CP-04's FROZEN_CONTRACT rows keep the fields of a simple catch-up.**
  - Where: `CANONICAL_SITUATIONS.md` CP-04.
  - Evidence:
    - The rows for the two hash and result-semantics identifiers take `BaselineClass=FROZEN_CONTRACT` and go to R4 as code-change items.
    - They still inherit CP-04's `LOCAL_DESIGN` tier and `RECORD` layer.
    - C3 reserves `LOCAL_DESIGN` for a catch-up that needs no decision. C5's `RECORD` means no protected layer is affected. A frozen-contract rename is neither.
    - CP-04 also does not say how `.opsproj` and the store filename are routed. Both are code identifiers, and DEC-101 imposes a persistence-compatibility obligation on them.
  - Smallest fix:
    - For the FROZEN_CONTRACT rows, set the tier to `PROJECT_BASELINE` (or `INVARIANT`) and add the `BASELINE` layer.
    - State that the `.opsproj` and store-filename rows are also R4 code-change candidates, noting the persistence-compatibility obligation.
- **B2-1. MINOR — the owner is credited with Agent 0's application.**
  - Where: `CANONICAL_SITUATIONS.md` CP-04 ("owner … That includes the four identifiers"); `CONVENTIONS.md` narrowings ("Rename residue (owner, R0 owner items) … including the four").
  - Evidence: including the four identifiers is Agent 0's reading of a conditional answer, which the addendum marks as open to the owner's correction.
  - Smallest fix: mark that clause "(AGENT reading of the owner's conditional; R0 ruling addendum item 2)".
- **B2-3. MINOR — the ambiguity check covers `docs/` only.**
  - Where: `validate_ledger_v2.py:240`; `CONVENTIONS.md` Part D.
  - Evidence: the project also has `tools/`, `.github/` and `execution/`. `tools/coordination` exists in both places and silently resolves to the root copy (test `c15e`).
  - Smallest fix: apply the ambiguity check to every repository-root prefix that also exists under `projects/chirality-piping/`.

## Validator outputs (`PYTHONDONTWRITEBYTECODE=1`)

- `validate_claims_language.py`: `VALID claims-language surfaces: 356 files scanned; DEC-081 registry taxonomy satisfied` (exit 0).
- `validate_piping_loop_receipts.py --repo-root .`: VALID, frozen through Receipt-44 (exit 0).
- `harness.py self-check`: exit 0, **0 BLOCK** (INFO 14, NOT_APPLICABLE 1, REVIEW 6, WARN 112). The only findings on this run's files are the two disclosed absolute-path REVIEWs on the stored returns.

Scratch files are in `/private/tmp/claude-501/-Users-ryan-dev-chirality--claude-worktrees-task-management-gen-pass-518da2/efe0b4ff-c1b6-4a97-af70-b04220464651/scratchpad/review/bc2/`.

END-OF-RETURN