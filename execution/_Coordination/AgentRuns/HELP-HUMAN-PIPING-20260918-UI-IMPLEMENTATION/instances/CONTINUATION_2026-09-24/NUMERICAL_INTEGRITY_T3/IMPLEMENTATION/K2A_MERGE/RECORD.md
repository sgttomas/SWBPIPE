# K2a merge record

- **PR:** https://github.com/sgttomas/chirality/pull/1032. ROOT (HELP_HUMAN) merged it on 2026-09-28 as `f12e068761de2135b1500207c0bec90dd6eba05a` (a merge commit) (expectedHeadSha `aad23e82d`), under the owner's standing Git authorization.
- **Candidate head:** `aad23e82dd5d836b0a6da95b4d26b990beca1b63`, on branch `codex/piping-k2a-20260927`.
- **Scope:** K2a, checked formation. `local_stiffness` refuses any zero, subnormal or non-finite intermediate formed from nonzero finite operands, with `FrameKernelError::NumericalRange { name }`. Accepted coefficients keep exactly today's bits.

## The chain

- **K2a:** `80290ce98` (I6), on K-D5 base `5ae22926e`. It changes:
  - FK `src/lib.rs` (+119 −13: the variant, its Display arm, checked `local_stiffness`, and three private helpers);
  - `diagnostics/src/lib.rs` (+49: the mapping and 1 test);
  - new FK tests `k2a_checked_formation.rs` and `k2a/rf_range_models.rs`;
  - new PP test `k2a_formation_range_runtime.rs` (ROOT's four product cases, both entries, both modes);
  - `IMPLEMENTATION/K2A/`.
- **Main merge:** `79c0d320b` (ROOT, main `649162522`, bringing F1a). It is disjoint from K2a's write set.
- **The review repair:** `aad23e82d` (I6; manager-committed). **Tests and records only:**
  - FK `tests/k2a_checked_formation.rs` (+135) gains per-site subnormal rows for the ten previously unpinned sites, an R10 row (2^-1022 accepted bit-identically) and an R5b row (the quotient itself is checked);
  - FK `tests/k2a/rf_range_models.rs` (+19);
  - `K2A/RETURN_ADDENDUM_1.md` and new `_run_records` files. SHA256SUMS gains 10 entries and changes none.
  - `RETURN.md` and `CHANGE_RECORD.md` are unchanged.

## Gates

- **RV7's independent complete-diff review** (`REVIEW/K2A_REVIEW.md`):
  - **At `79c0d320b`: NOT PASS.** One BLOCKING finding, B1: M03's element-entry floor is an axis-aligned argument, confirmed with FK's real `transform_roundoff`. Also three SHOULD-FIX findings (S1: the L-range caveat; S2: main's gap-route evidence is captured-entry only; S3: a cited log that never existed) and five NOTEs (N1–N5). N2 recorded seven surviving per-site mutants: R1–R5, R5b and R10. **The code is correct.**
  - ROOT ruled that B1, S1–S3 and N1–N5 be answered in a records-only addendum. N2 was closed in the PR with per-site test rows (ROOT's revised ruling), not deferred.
  - **At `aad23e82d`: PASS**, with no open BLOCKING or SHOULD-FIX finding. RV7 re-ran R1–R5, R5b and R10 on `a1029d7da`, whose FK tests are identical to those at `aad23e82d`. All seven are killed at named assertions, and the no-mutation control passes 13/13 (`REVIEW/_run_records/k2a/rerun_a1029d7da/`). RV7's delta check is `delta_aad23e82d.txt`.
- **The DEC-025 sandboxed sweep,** on `79c0d320b`: `dec025/SWEEP_20260928T030440Z_79c0d320b880.json` (sha256 `97228e0224d71f86e44d4ebfe88771dd7c7dad1f8d8eb2eeb009db14f1f94c06` for the original). Overall **pass**, `working_tree_dirty: false`, `--only-capability sandboxed`. All four surfaces passed:
  - the cargo crate sweep (39 manifests);
  - pytest: 3023 passed, 32 skipped;
  - desktop vitest: 134 files and 2822/2822 tests;
  - the production build.

  The run window was 03:04:37Z–03:44:56Z (2415 s), exit 0 (`dec025/meta.txt`); the full log is `dec025/sweep.log`. Machine paths are replaced with `<WORKTREE>`, `<wt>`, `<VENV>`, `<home>` and `<scratch>`.
- **Hosted CI:**
  - On `79c0d320b`, all green: the full-SHA **workflow_dispatch** E2E run **36372299519** (target_base `649162522`) and the **pull_request** E2E run **36372300496**, both "Piping Desktop E2E" on PR #1032, plus the numerical cargo suite, harness and pec. The event types were verified through the GitHub API.
  - On `aad23e82d`: the automatic **pull_request** run **36376818888**, all green: the Numerical cargo suite, Source remainder 1–4 and Desktop E2E (source mode). Harness **36376818884** and pec **36376818928** are also green. There was no workflow_dispatch on `aad23e82d` (owner-endorsed).
- **Why the sweep and E2E stand for `aad23e82d`** (ROOT, owner-endorsed): `git diff --stat 79c0d320b aad23e82d` shows 13 files, all additions: the two FK test files (+154) and `IMPLEMENTATION/K2A/` records. There is no change to product source, PP, the desktop, `tools/` or `.github/`, so there is no desktop input. The sweep and the E2E dispatch on `79c0d320b` stand; E2E was not re-dispatched, and the automatic pull_request CI covers the new head.

## Evidence (I6; `IMPLEMENTATION/K2A/`, with RETURN_ADDENDUM_1)

- **Suites:** FK 159, diagnostics 25, PP 519 (1 pre-existing ignore); 24/24 crates. After the repair, FK k2a passes 13/13.
- **Mutations:** 31/31 killed at `80290ce98`, with the no-patch control passing. The seven per-site survivors RV7 found were killed after the repair.
- **T9:** 112/112 byte-identical against `5ae22926e`.
- **The two-part gate,** against the empty lists: 888 runs, 328 trusted, **0 trusted breaches**, and 0 standing changes against main over all 768 frozen-reference runs. Part 2's four dense timeouts timed out at 1800 s, as on main.
- **Product reach:** the authoritative statement is RETURN §5 as scoped by RETURN_ADDENDUM_1 (ROOT correction 3, ruling 4).
  - On axis-aligned members, M03's element-entry floor refuses subnormal-derived coefficients below it.
  - On skew members, M03 accepts subnormal-derived 4EI/L and 2EI/L below the floor. 6EI/L² is then the limiting coefficient.
  - On the probed skew cases, main's product route publishes nothing (UNRESOLVED at the pivot screen). Refusal downstream is not established in general.
  - K2a refuses all of these at formation, by name.

## Findings routed

- **The skew M03 pin** goes to K1's pattern-path M03 tests (K5 fallback), with RV7's confirmed members and figures (work graph). **[K1 did not take it (`K1_MERGE/RECORD.md`, "A routed item K1 did not take"). It is now a tests-only follow-up, I9 (ROOT, 2026-09-28).]**
- **N1:** subnormal *derived* section values (A, I and J from `derive_pipe_section`) can be inexact and still pass K2a when every intermediate is normal. This is added to the input-validation finding routed out of T3 (the original K2a product-reach ruling 3).
- **Other stiffness-forming paths** with 1/L² or 1/L³ terms (curved_bend's closed-form inverse, and K-D5's `Wide<2>` re-formation, which has an extended exponent) stay on the T3-close list (correction 2, ruling 4).

## Not run for this merge

The native macOS witnesses do not apply, because K2a changes no native path.

## Addendum (ROOT, 2026-09-28; RV13-D3)

- Records PR #1042 added a bracketed pointer (RV13-N5) to `IMPLEMENTATION/K2A/RETURN_ADDENDUM_1.md`. That changed the file's sha256 from `b696e806…`, its hash at this merge, to `cdafd957…`.
- Two committed records cite the prior hash: `IMPLEMENTATION/M03_SKEW_PIN/RETURN.md:24` and RV7's `delta_aad23e82d.txt:26`. They refer to the file as it was at `aad23e82d`, which `git show aad23e82d:<path>` reproduces.
