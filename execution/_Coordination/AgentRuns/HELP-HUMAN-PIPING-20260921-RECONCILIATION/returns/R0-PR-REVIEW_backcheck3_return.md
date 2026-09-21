VERDICT: FINDINGS

I reviewed `2b664486a..c0c7a66f0`. Commit `96fd10212` (R1 capability inventory, 598 capabilities) landed while I was reviewing, and I did not review it. It does change `tools/check_inventory.py`, so I also ran that newer version against my synthetic case, and the result is the same (see B3-1).

The findings are one actionable defect in `check_inventory.py` and three minor ones. None of them affects the R0 PR's bound inputs. B3-1 matters for R1 dispatch.

## (a) Backcheck 2 findings

- **B2-1 — closed.** CP-04 and the narrowings in `CONVENTIONS.md` now label the inclusion of the four identifiers "(AGENT reading …, open to the owner's correction)". Both cite ruling addendum item 2.
- **B2-2 — closed.**
  - The hash and result-semantics rows now take `FROZEN_CONTRACT`, `PROJECT_BASELINE` and `RECORD;BASELINE`.
  - The `.opsproj` and store-filename rows take `NONE`, `PROJECT_BASELINE` and `RECORD`, and note the persistence-compatibility obligation.
- **B2-3 — closed.**
  - The ambiguity check now applies to any repository-root path that also exists in the project. `tools/coordination` now fails (case c15e), and `ROOT_DOC:` lets such a token pass.
  - Cases 14b, 15c, 00 and 12 pass as before, and the conforming batch set still returns 0 findings.
- **Bound hashes:** every file in the latest `BOUND_INPUTS` event at `c0c7a66f0` matches, including `check_inventory.py` at that commit (`653163d7…`).

## (b) R1 files

### `build_r1_indexes.py`

- **Byte identity:** a rerun into scratch reproduces `DELIVERABLE_INVENTORY.csv`, `VERIFICATION_INDEX.csv` and `VALIDATION_AND_PROVENANCE_INDEX.csv` exactly.
- **The census reproduces:**
  - 102 deliverables: 100 `IN_PROGRESS`, 1 `ISSUED` (DEL-01-01), 1 `OPEN` (DEL-07-09);
  - 93 Scope of Work, 8 architecture basis, 1 bespoke;
  - 151 Remaining items across 65 deliverables;
  - 7,257 required units of 12,854;
  - 808 pre-typed, 725 canonical.

### `AUTHORITY_AND_SOURCE_RELIABILITY_MAP.md`

- **Adopted-by-reference paths:** all four exist at the freeze (D-66 plan and work graph, D-67 and D-68 approved plans). The ruling ranges D-01–D-72 and DEC-001–DEC-109 are right.
- **Two-scope-pointer paragraph:** accurate.
  - `docs/_ScopeChange` holds only SCA-002 and SCA-003, and its pointer names SCA-003.
  - The execution pointer names SCA-010.
- **Misclassifications and slips:** see B3-3.

### `check_inventory.py`

- The committed DOCS, FEATB and WSUI inventories pass it.
- It lets a non-conforming inventory pass; see B3-1.

## (c) Findings

- **B3-1. ACTIONABLE — the inventory checker's coverage and ownership screens can be passed trivially.**
  - Where: `tools/check_inventory.py:74`, `:77-86` and `:64-73`.
  - Evidence: I wrote a synthetic `INV_FEATB.csv` with one row. It passes with `--coverage` and 0 findings under both the `c0c7a66f0` version and the `96fd10212` version. The row has:
    - `EntryPoints=projects/chirality-piping`;
    - `Area=WRONG`;
    - `del-17-04` in Capability and `Del-07-02` in Notes.
  - Why it passes:
    - Any directory token covers every file beneath it, so coverage proves nothing.
    - The `Area` column is never checked.
    - The ownership regex is case-sensitive.
  - Also, the `96fd10212` version counts `Tests` cells toward coverage. The brief requires every source file to appear in an `EntryPoints` cell.
  - Smallest fix:
    - Reject tokens that equal, or are ancestors of, the area's partition prefixes. Allow directory tokens only in data, fixture and document areas.
    - Check that `Area` equals the area being checked.
    - Make the ownership regex case-insensitive, `(?i)`.
    - Either keep coverage to `EntryPoints` or amend the brief.

- **B3-2. MINOR — `build_r1_indexes.py` mislabels some tests and silently skips validation evidence.**
  - Where: `tools/build_r1_indexes.py:92-99`, `:104` and `:130`.
  - Evidence:
    - Four `apps/desktop/src-tauri` Rust files (102 static cases) are labelled `desktop_vitest`, but cargo runs them.
    - `apps/desktop/scripts/build-wasm-engine.test.mjs` is not detected.
    - `validation/evidence/` is excluded without saying so. That is 759 files, including `benchmarks`, `gates` and `comparison_measurement`. The docstring says "one row per validation or provenance asset", and the authority map sends workers to this index for validation assets.
  - Smallest fix: test the `.rs` suffix before the `apps/desktop/` prefix, add `.mjs`/`.js`, and include or document the evidence subtree, at least its benchmark and gate families.

- **B3-3. MINOR — slips in the authority map.**
  - Where: `AUTHORITY_AND_SOURCE_RELIABILITY_MAP.md`.
  - **Evidence and context overlap.** "Returns" and deliverable `_run_records/**` are listed under Context, while "review returns" and run records are listed under Evidence. A3 lets the same record serve as both; say so.
  - **Stale A3a example.** `ACTIVATION_2026-09-18.md` is still given as the A3a example, although the addendum now treats it as context. DEL-11-01's R18 record is the example R0 used.
  - **Wrong PRD line.** The historical path that keeps the former name is on line 16 of the PRD at the freeze, not line 15.
  - **Wrong fence source.** F-PIP-1..4 are retained in the project's `AGENTS.md` (around line 35), not in `WORKPLAN_2026-09-19`.
  - **Missing governing sources.** The governing list omits the project `AGENTS.md`, `loop/LOOP_INIT.md` and `software-workflow.json`, which `RUN_BASIS` lists as authority.
  - **Reliability rule risk.** "Agent-produced technical evidence … UNVERIFIED" could bring back the F4 split that the R0 review fixed, where some workers marked agent-run tests `UNVERIFIED`. C10 says rows citing tests and run records are `NOT_APPLICABLE`; the map should say that rule wins.

- **B3-4. MINOR — records.**
  - `INV_DOCS.csv` and `INV_FEATB.csv` are committed at `c0c7a66f0`, but that commit's `RUN_STATE` has a `RETURN` event only for WSUI. Commit `96fd10212` may add the missing events; I did not check.
  - The harness REVIEW count is now 7 because of the stored backcheck 2 return. The existing `RUN_BASIS` disclosure covers it.

## Validator outputs (`PYTHONDONTWRITEBYTECODE=1`)

- `validate_claims_language.py`: `VALID claims-language surfaces: 356 files scanned; DEC-081 registry taxonomy satisfied` (exit 0).
- `validate_piping_loop_receipts.py --repo-root .`: VALID, frozen through Receipt-44 (exit 0).
- `harness.py self-check`: exit 0, **0 BLOCK** (INFO 14, NOT_APPLICABLE 1, REVIEW 7, WARN 112). The only findings on this run's files are the disclosed absolute-path REVIEWs on the three stored returns.

Scratch files are in `/private/tmp/claude-501/-Users-ryan-dev-chirality--claude-worktrees-task-management-gen-pass-518da2/efe0b4ff-c1b6-4a97-af70-b04220464651/scratchpad/review/bc3/`.

END-OF-RETURN