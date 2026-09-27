# K-D5 merge record

- **PR:** https://github.com/sgttomas/chirality/pull/1017. ROOT (HELP_HUMAN) merged it on 2026-09-27 as `5ae22926efbe1dc8632d077f552f888fd3efd649` (expectedHeadSha `b0a278d6f`), under the owner's standing Git authorization, and updated the PR body with the final evidence before merging.
- **Candidate head:** `b0a278d6f5a9d4343293101465c0a252a30e1b2b`, on branch `codex/piping-kd5-20260926`.

## The chain

- **K-D5:** `17f3d6e05` (I3), on K3a base `a2e804a75`.
- **Forward merges of main:** `b6156d49d`, `8fd409e78` (onto S11-F) and `3befacff4` (onto the RV3-S1 follow-up).
- **The addendum-4 pass:** `a89fde17b`. PP on the typed entry; per-item dead_code; the nonlinear pins folded into `s11k_tests.rs`.
- **The S11-G merge:** `fbc9661a4` (main `b24b3d536`). It was textually clean, and I3's composition check found it clean.
- **The T20 comment correction:** `2409de83e` (comment only).
- **The RV5 repair:** `67a9558c2` (I3R; tests and records only):
  - the M31b and M31b0 killers (60° planar and 30° skew adapter tests, and the product-level X 5e6 / φ 5° demotion, which M31b flips to CHECKS_PASSED);
  - the 5e5 not-demoted controls;
  - E4 closed (a module-walk source pin excluding test modules by declaration; a multi-iteration unit-force pin);
  - the regenerated callers as a new file;
  - the combined-tree RETURN addendum and CHANGE_RECORD;
  - a clean mutation re-run.
- **Main merge:** `799b0ef46` (ROOT). It brings no piping product change; the piping changes are PR #1015/#1016 execution records.
- **Main merge:** `b0a278d6f` (ROOT, main `974bf7da4`, done with `git merge-tree` so the sweep worktree was untouched). 59 files, all under `projects/pec`, and nothing under `projects/chirality-piping`, `tools/` or `.github/`.

## Gates

- **RV5's independent complete-diff review** of `2409de83e` (`REVIEW/KD5_REVIEW.md`, commit `d88cfc378`): **NOT PASS**, with 1 BLOCKING, 3 SHOULD-FIX and 7 NOTE findings.
  - **RV5-B1:** the M31b chord-only mutant is not equivalent, confirmed in Rust and at product level. ROOT withdrew its earlier acceptance (`ROOT_RULINGS_V1.md`, "K-D5 mutation M31b: equivalence withdrawn", `3547029576`).
  - **S1:** stale callers. **S2:** evasion E4 not caught. **S3:** the combined-tree records not yet committed.
- **RV5's delta check** of `2409de83e..b0a278d6f` (`REVIEW/KD5_REVIEW.md` §12, sha256 `edc6a2d2…`, commit `079226544`): **PASS for `b0a278d6f`**, with no blocking finding remaining. It checks:
  - (a) that `67a9558c2` is tests and records only, the kills are real, and B1 and S1–S3 are closed;
  - (b) that `67a9558c2..799b0ef46` brings no piping product change;
  - (c) that `799b0ef46..b0a278d6f` brings no change under `projects/chirality-piping`, `tools/` or `.github/`.

  RV5's runs from an archive of `67a9558c2`:
  - NI unpatched 87/87 lib and 4/4 doc;
  - M31b and M31b0 killed in NI and PP (FK also kills M31b);
  - E1–E4 killed (E4 by the unit-force pin and the source pin);
  - PP formation_check_runtime 5/5 and the site test 11/11 unpatched.
- **The DEC-025 sandboxed sweep,** on `799b0ef46`: `dec025/SWEEP_20260927T201431Z_799b0ef4609e.json` (sha256 `9458fbcb…` for the original), overall **pass**, `working_tree_dirty: false`, `--only-capability sandboxed`. All four surfaces passed:
  - the cargo crate sweep;
  - pytest: 3023 passed, 32 skipped;
  - desktop vitest: 134 files and 2822/2822 tests;
  - the production build.

  The run window was 20:14:28Z–20:55:48Z, exit 0 (`dec025/meta.txt`); the full log is `dec025/sweep.log`. Machine paths are replaced with `<WORKTREE>`, `<wt>`, `<VENV>` and `<scratch>`. **It stands for `b0a278d6f`,** because the piping, `tools/` and `.github/` trees are identical (RV5 check (c)).
- **Hosted CI on `b0a278d6f`:** all green.
  - The pull_request E2E run **36347475982**: Select source coverage, the Numerical cargo suite, Source remainder 1–4/4, and Desktop E2E (source mode).
  - The full-SHA dispatch run **36347475874** (target_base `974bf7da4`): the same jobs, green.
  - harness, Harness pre-merge and pec are green; the App jobs were skipped by selection.
- **Superseded E2E runs, disclosed:**
  - dispatch **36347348097**, which carried a malformed target_base and ended at once;
  - the dispatch on `799b0ef46`;
  - the pull_request run **36347349219**, which failed at plan validation ("event target base … not integrated into head") because main had moved to `974bf7da4`. The merge to `b0a278d6f` resolved it.

## Evidence on the combined tree (K-D5 on top of S11-G)

- **Suites:** 24/24 crates on `fbc9661a4`. The repair tests on `67a9558c2`: NI 89/89, FK kd5 5/5, PP 516 (1 pre-existing ignore), site test 11/11.
- **T9** against main `b24b3d536`: 112/112 byte-identical. S11-G moved no fixture output.
- **The gate:** the full 888 against main's empty lists, in two parts (ROOT ruling, `8fcf14d7a`). The **union is a PASS:**
  - part 1 (884 runs): 0 trusted breaches;
  - part 2: the 4 known dense timeouts, on a quiet host with no other cargo, timed out at 1800 s as on main, with nothing published.

  RV5 recounted all 888.
- **The attribution:** the P1 probe built on main `b24b3d536` over the 836 non-large runs. **Only RF-SKEW-T-CANT-OFF-122-r1e-04 differs** (checks_passed on main, sensitive on the candidate, both entries and both modes). Main alone would fail the empty-list gate on 122's 8 trusted triples, and K-D5 removes exactly those. The 34 checks_passed→sensitive moves since the pre-S11-G gate are all S11-G's forecast demotions.
- **Timing:** K-D5 adds no measurable cost.
  - On CHAIN-n01000-AX and TREE-n01000-ROT dense, interleaved, the candidate is 3.0% and 4.8% faster than main (noise).
  - On CONT-n1000-AX dense, where the check runs, RV5 measured 205 s against 220 s.
  - I3R's per-run K-D5 wall cost over 382 runs: median −0.0004 s, max +0.081 s.
  - **An S11-G performance finding** (not K-D5's): roughly +15–20% on dense 1000-member solves, from non-interleaved runs. It is on the T3-close decision list, to be settled by an interleaved `72d5ff864` against `b24b3d536` run.

## Findings routed

- **The gate corpus lacks realized curved bends.** Curved-bend formation integrity is evidenced by unit and product tests only. This is routed to the gate-corpus owner, and is on the T3-close list.
- **The product's curved element carries formation error above the criterion at coordinates ≳ 2e6 m** (UTM northing scale), from PP's binary64 centre. K-D5 correctly demotes such elbows. This is routed to T4/W1c, with its priority raised. The earlier ~5e5 m figure is superseded as a repr-input artefact.

## Not run for this merge

The native macOS witnesses do not apply, because K-D5 changes no native path. I3 ran the src-tauri suite (114/114) on the combined tree; it runs neither in the sweep nor in CI.
