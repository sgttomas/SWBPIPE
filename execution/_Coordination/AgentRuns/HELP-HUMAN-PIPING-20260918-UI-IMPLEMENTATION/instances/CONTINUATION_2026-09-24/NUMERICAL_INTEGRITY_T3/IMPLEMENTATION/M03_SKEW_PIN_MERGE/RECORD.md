# Skew M03 pin merge record

- **PR:** https://github.com/sgttomas/chirality/pull/1038.
  - ROOT (HELP_HUMAN) merged it on 2026-09-28 as `e8b416e433533fe408ea926edd0de564742ac2d6`: a merge commit with `--match-head-commit 5dd6dfdd8`, under the owner's standing Git authorization.
  - The merge state was clean.
- **Candidate head:** `5dd6dfdd8fe1e49ca271190439b27f61e2d130f5`, on branch `codex/piping-m03-skew-pin-20260928`.
- **Where it ran:** the owner's Mac (`aarch64-apple-darwin`, rustc 1.97.1). ROOT dispatched the implementer and the reviewer directly, as background subagents.
- **Scope:** tests and records only. It pins M03's scope on skew members, from K2a's review (RV7, B1). The work was routed to K1 but not taken before K1 merged (`K1_MERGE/RECORD.md`), and ROOT ruled it a tests-only follow-up before K2b.

## The chain (base main `eb52114e9`)

| Commit | Content |
|---|---|
| `885f065e5` | Tests: new `frame_kernel/tests/m03_skew_scope.rs` (5 tests), and one test appended to `structural_adapter/k1_tests.rs` (I9) |
| `39dfd69c7` | Records: `IMPLEMENTATION/M03_SKEW_PIN/` (I9) |
| `1d105d633` | A merge of main at `6e18505e3`. **Its message says `5a2b9d112` by mistake**; the parent is correct. The commit was pushed before the error was found, so it is disclosed here and in the PR body, not rewritten. Main's changes since `eb52114e9` were records and App docs only |
| `5dd6dfdd8` | Records only: RV10's SHOULD-FIX S1 (stale test-file hashes in two summaries) and S2 (RETURN §3.4 scoped to RV7's configuration) |

## Gates

- **Independent review, RV10** (`REVIEW/M03_SKEW_PIN_REVIEW.md`; numerics `d2df479f3`, then `bea34e7f0`):
  - PASS at `1d105d633`: 0 BLOCKING, 2 SHOULD-FIX, 6 NOTE.
  - PASS on the delta check at `5dd6dfdd8`: S1 and S2 resolved, one optional note (N7).
  - RV10 checked the pre-K2a local-matrix replica against `134eefc24`'s `local_stiffness` bit for bit, re-derived the exact errors, and reproduced all 864 product runs byte for byte.
  - It wrote six mutants of its own. Five were killed; RV10-DEEP-REFUSE-1019 survives (NOTE N2: the guard's last clause overclaims).
- **Hosted CI:**
  - pull_request run **36391476996** on `5dd6dfdd8`: success;
  - pull_request run **36387730033** and the full-SHA dispatch **36387729465** (target_base `6e18505e38520ef3e37fdc46ec434c9aa6ba4f2c`) on `1d105d633`: success.
  - The final PR state: 12 checks passing, 0 failing.
- **T9 (Mac-only):** 112 of 112 byte-identical, base `eb52114e9` against the candidate (I9's RETURN §8).
- **DEC-025**, under the owner's Mac decision (`OWNER_DIRECTION.md`, 2026-09-28), on `1d105d633`; the evidence is in `dec025/`:
  1. `run_evidence_sweep.py --execute --only-capability sandboxed` failed at the cargo surface, on product_physics's platform test `t13`. The tool is fail-fast, so the three later surfaces were recorded as `not_run`. `SWEEP_20260928T064249Z_1d105d63376f.json` is sanitized; the sha256 of the original is `f96a925382cb197e8c9a321a3f3c5e7db7a0bc7d6465f17e28713d411ebcaa1c`.
  2. All 39 cargo manifests were run with `--no-fail-fast` (`suites.log`). Against the Mac baseline of main `eb52114e9`:
     - only frame_kernel (179 → 184) and nonlinear_integration (101 → 102) change, by the 6 added tests;
     - the failing tests are exactly the three Mac platform tests, identical to the baseline (`suites_vs_baseline.txt`).
  3. Surfaces 2, 3 and 5, run with the sweep's own commands, all exit 0 (`surfaces.txt`):
     - pytest: 3023 passed, 32 skipped;
     - vitest: 134 files, 2822 of 2822;
     - the production build.
  - The evidence stands for `5dd6dfdd8`, since that head changes only records (RV10's delta check).
  - Machine paths are sanitized to `<WORKTREE>`, `<VENV>`, `<wt>`, `<home>` and `<tmp>`, and trailing whitespace is stripped.
- **The gate:** not run. There is no product change.

## Findings recorded

These are ROOT rulings on numerics (`9e05200e4`), resting on I9's findings as RV10 reproduced them:
- **K2a's addendum §1.3 question is answered.**
  - Pre-K2a main published no trusted value on the probed skew cases.
  - It did publish some as Sensitive (untrusted), so its refusal downstream was not general.
  - Current main refuses them all at formation, by name.
- **"6EI/L² is the limiting coefficient" holds only in RV7's configuration.** RV10's counterexamples, including the torsion analogue of B1, which K2a's `GJ/L: G*J` covers, are in the scoped RETURN §3.4.
- **The T3-close list gains M03's skew scope in general.** It is a documented limitation, and the guard pins it only in RV7's configuration.

## Process disclosure

Commit `d2df479f3` on numerics also carries RV10's first review file. ROOT's `git add -A` swept it in while RV10 was writing. Its bytes equalled RV10's final bytes at that time. Numerics `bea34e7f0` discloses this, and ROOT now stages explicit paths.
