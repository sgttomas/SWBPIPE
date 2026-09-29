# K5 merge record

- **PR:** https://github.com/sgttomas/chirality/pull/1044.
  - ROOT (HELP_HUMAN) merged it on 2026-09-29 at 03:18:22Z as `1cdeae2c1de34566d5733ae977dd78e845706c4b`: a merge commit with `--match-head-commit babcf5e65`, under the owner's standing Git authorization.
  - Main was `b37331092`, an ancestor of the head, and the merge state was clean.
- **Candidate head:** `babcf5e6587081bfb3697013d27a53cf960b6e02`, on branch `codex/piping-k5-20260928`.
- **Where it ran:** the owner's Mac (`aarch64-apple-darwin`, rustc 1.97.1). ROOT dispatched the implementer (I14) and the reviewer (RV14) directly.
- **Scope:** W4, the constrained-body witness (`assess_constrained_bodies` in FK) and the curved rule, wired into the four selected formation-checked branches of SA (DESIGN.md §4.9). The full account is `IMPLEMENTATION/K5/RETURN.md` and `CHANGE_RECORD.md`, on the merged branch.

## The chain (base main `24dea2dae`)

| Commit | Content |
|---|---|
| `0c732061b` | Checkpoint A1: `assess_constrained_bodies` in FK, with tests |
| `6bf64f5a9` | Checkpoint A2: SA wiring; curved slots qualified by source |
| `416b0d456` | Checkpoint B records: 39-manifest suites, T9, gate part 1 |
| `f89662e1f` | Checkpoint C: the mutation table, and two added tests |
| `b379e5b27` | Checkpoint D: CHANGE_RECORD and RETURN |
| `95c7501a7` | RV14's four SHOULD-FIX findings resolved |
| `28517eaaa` | A merge of main. **Erratum (RV16-N11):** its message says main `df6d59e3c`, but its second parent is `65e2d6c2a`. RV14's delta check states the parents correctly |
| `a4378835c` | RV14-D1 closed (tests only); RV14-D2 recorded as an accepted limitation |
| `babcf5e65` | A merge of main `b37331092` (App v4 files and records PR #1049's T3 records; no piping source) |

## Gates

- **Independent review, RV14** (`REVIEW/K5_REVIEW.md`, with `REVIEW/_run_records/k5_review/`):
  - **PASS** at `b379e5b27`: 0 BLOCKING, 4 SHOULD-FIX, 5 NOTEs. No false witness and no missed mechanism in RV14's corpora.
  - **PASS** on the delta at `28517eaaa`. The four SHOULD-FIX findings and N1, N2 and N5 are resolved. RV14-M1 to M4 are each killed by a new test. It raised RV14-D1 (SHOULD-FIX, a test gap) and RV14-D2 (NOTE).
  - **PASS** on the delta at `babcf5e65`, with 0 new findings. RV14-M5 is killed at `k5_constrained_bodies.rs:269` from a clean archive, and the merge adds exactly main's delta.
  - ROOT's rulings: `ROOT_RULINGS_V1.md`, "K5: rulings on RV14's review (recorded late, RV16-S3)" and "K5: rulings on RV14's delta check at 28517eaaa".
- **Hosted CI on `babcf5e65`:**
  - pull_request runs: Piping Desktop E2E **36511682533**, Harness Pre-merge **36511682393**, governance-harness **36511682439** and pec-tests **36511682318**, all successful;
  - the full-SHA dispatch **36511678387** (target_base `b37331092a778a8087df57cb55630ee735425ece`): success;
  - the final PR state: 12 checks passing, 4 skipped as selected, 0 failing.
- **T9 (Mac-only):** 112 of 112 at checkpoint B, on candidate `6bf64f5a9`, with its records at `416b0d456` (RETURN §5). C (`f89662e1f`) added tests only.
  - The later product-reaching change, `95c7501a7`'s FK `publish` exactness check, cannot reach the T9 or gate part 1 corpora: W4 runs in none of them, since they contain no curved bend and no user element (RETURN §16).
  - RV14's first delta confirmed its SA probes, SA mutant probe and 40-run PP spot check are byte-identical to `b379e5b27`.
  - K5 merged before F1b. By the rule in RETURN, F1b, merging second, merges main and re-runs its suites, T9 and its affected tables.
- **Gate part 1:** run at checkpoint B on `6bf64f5a9` (RETURN §5). K5 is kernel and SA only; the both-entry gate runs in full at F1b.
- **DEC-025,** under the owner's Mac decision (`OWNER_DIRECTION.md`, 2026-09-28), on the final head `babcf5e65`. The evidence is in `dec025/`; the driver is `M03_SKEW_PIN_MERGE/dec025/dec025_mac.sh.txt`, unchanged.
  1. **The sweep invocation** failed at the cargo surface on product_physics's platform test `t13` (fail-fast), as expected on the Mac. The summary is `SWEEP_20260929T024330Z_babcf5e65870.json`, sanitized; the sha256 of the original is in `sweep_json_original_sha256.txt`.
  2. **All 39 manifests** were run with `--no-fail-fast` (`suites_vs_baseline.txt`). The baseline is the Mac run of K2b's final head `33e33c723`, whose piping tree equals main's (`e7d930d49` and `b37331092`).
     - Only three manifests change, each by K5's added tests: frame_kernel 249 → 267, nonlinear_integration 120 → 133 and product_physics 525 → 529.
     - The failing tests are exactly the baseline's three Mac platform tests: product_physics `t13` and runner_headless's two load-reference tests.
  3. **Surfaces 2, 3 and 5:**
     - pytest: 3023 passed, 32 skipped;
     - the wasm build and the production build: exit 0;
     - **vitest: 1 failure in the sweep run.** `App.test.tsx`'s "renders the engineering workspace from invented local fixtures" timed out at 30 s, the same test that timed out under load at K2b (`K2B_MERGE/dec025/earlier_087b3a088/`). K5 changes no TypeScript or desktop file.
     - The vitest surface was re-run at once on the same head (`dec025/vitest_rerun/`): **134 files, 2822 of 2822 passed.** An unrelated system `codesign` process held the host's load average at about 10 to 14 during the re-run; the timeout was not raised.
  - Machine paths are sanitized to `<WORKTREE>`, `<VENV>`, `<wt>`, `<home>` and `<tmp>`, and ANSI colour codes are stripped.
- **GEN-8** on `babcf5e65`: `pytest tools/practitioner_harness/test_live_baseline.py -k gen8` → 1 passed (run with `set -o pipefail`).

## Accepted limitations and routes

- **RV14-D2:** a refused witness ends the candidate search with `NumericallyUnresolved`, even when a later candidate would publish. This is conservative and reachable through the FK API only, and it is on the T3-close list as a candidate refinement.
- **RV14-4's refusal:** 349 witnesses in RV14's FK corpus change from W to U (subnormal spans; FK API only). Every other result is byte-identical.
- **For F1b (merging second):** merge main, then re-run its suites, T9 and its affected tables.
- **For K6:** after this merge, K6 merges main and re-runs its test E (K6 plan N6; the SA entry now calls `constrained_geometry`).

## Scratch to prune

`<wt>/k5-target`, `<wt>/k5-gate-target` and `<wt>/scratch/i14`. I14's gate envelopes were kept until merge and may now go. The records above keep every hash.
