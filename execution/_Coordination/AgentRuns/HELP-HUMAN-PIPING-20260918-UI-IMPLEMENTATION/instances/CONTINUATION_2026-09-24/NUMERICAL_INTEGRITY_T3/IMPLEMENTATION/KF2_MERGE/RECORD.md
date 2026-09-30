# KF2 merge record

- **PR:** https://github.com/sgttomas/chirality/pull/1060.
  - ROOT (HELP_HUMAN) merged it on 2026-09-30 at 06:08:13Z as `7ad3a9adf479a500ae0d133fa08ace84e4ec4bf2`: a merge commit with `--match-head-commit 522167ac6`, under the owner's standing Git authorization.
  - Main was `dd61120ff` (KF3), an ancestor of the head. ROOT checked immediately before the merge that main had not moved.
- **Candidate head:** `522167ac62f27ad999a4416d10b95f922ff8c665`, on branch `codex/piping-kf2-20260930`.
- **Where it ran:** the owner's Mac (`aarch64-apple-darwin`, rustc 1.97.1). ROOT dispatched the implementer (I20) and the reviewer (RV24) directly.
- **Scope:** T3 slice KF2, K6's N10. The dense negative-pair witness goes from O(n⁴) to O(n²), and every result is bit-identical, errors included.
  - **The code** (`FK/src/structural.rs`): `negative_pair_witness` delegates to a private `negative_pair_witness_counted`. For each pair, in the old order, it repeats `verify_negative_direction`'s arithmetic on the pair's four cells, and only a witness verdict calls the unchanged verifier.
  - **The tests:** the new `kf2_witness_tests.rs`, and one declared row in `FK/tests/s11_site_table.rs`.
  - **Product-reaching, with nothing published changed.** The callers: FK's `solve_prepared_dense`; NI's structural adapter (`:2017`), which serves both dense scrutiny and `sparse_interactive`; the nonlinear loop (NI `lib.rs:1990`, `:2013`); and `sparse_direct/src/structural.rs:37`.
  - The account is `IMPLEMENTATION/KF2/RETURN.md` (with addendum 1) and `CHANGE_RECORD.md`, on the merged branch.

## The chain (branch point: main `78f55f927`)

| Commit | Content |
|---|---|
| `573bd3835` | Checkpoint 0: the plan (the cost, the equality argument and the screen diagnosis) |
| `1b10121fa` | A: the O(n²) witness, the differential and count tests, and 11 mutants |
| `6caa38e23` | B: T9, the both-entry gate (parts 1 and 2) and the src-tauri suite |
| `f2b8c85a2` | D: RETURN, CHANGE_RECORD and SHA256SUMS |
| `1c7558df5` | RV24's review closed: RV24's cases adopted (tests only) |
| `522167ac6` | A merge of main `dd61120ff` (KF3, and PR #1061), by ROOT: clean, and `structural.rs` is main's plus exactly KF2's delta |

## Gates

- **Independent review, RV24** (`REVIEW/KF2_REVIEW.md`, with `REVIEW/_run_records/kf2_review/`):
  - **PASS** at `f2b8c85a2`, with 0 BLOCKING, 1 SHOULD-FIX and 5 NOTEs.
    - RV24's own differential harness, whose oracle is a verbatim copy generated from the base's bytes, found 0 differences over 24,000 corrupted systems, 2,864 built systems, 134,400 steps around the verdict's crossing, and adversarial cases.
    - B's records hold. RV24 checked the uncommitted `runs.jsonl` hashes, and an instrumented rebuild matched 860 of 860 part-1 runs.
    - **RV24-1 (SHOULD-FIX):** three single-edit regressions of the guard survived the committed tests.
  - **CONFIRMED** at `1c7558df5`: RV24-1 is closed, since the three mutants are killed by the committed tests from a clean archive. I20's correction of RV24-N3's count is right: 4 of the 16 witness runs publish a recovered result, and 12 the refusal.
  - **A merge check at the final head `522167ac6`: clean, with no findings.**
    - The merge adds exactly each side's delta path by path over 60,002 paths, and the remerge diff is empty.
    - FK passes 432 (366 lib, 60 integration, 6 doc-tests), with KF2's 15 debug tests and RV24's harness at 0 differences.
  - ROOT's rulings: `ROOT_RULINGS_V1.md`, from "KF2: spawn" to "KF2 merged".
- **Product gates at B** (`KF2/_run_records/b/`, on this Mac, base `78f55f927` against candidate `1b10121fa`):
  - **T9:** 112 of 112 byte-identical, plus F1b's extra corpus at 16 of 16. RV24-N3: no T9 output reaches the witness, so T9 is an invariance check here.
  - **The both-entry gate, part 1:** 884 of 884 runs identical to a fresh base run, with 0 trusted breach triples and no heap-cap abort. The 16 dense runs that reach the witness are byte-identical.
  - **Part 2:** the four dense N10 runs end in the factor's refusal in 67–68 s each. They were previously killed at 1,800 s.
  - **src-tauri:** 116 passed.
  - The later commits change only tests and records (`1c7558df5`), and main's KF3 (retained, kernel only), so B stands for the final head.
- **Mutants:**
  - I20's 11, all killed;
  - RV24's M1, M4b and M5, killed by the committed tests;
  - RV24-M2 and M9 are equivalent (RV24-N1).
- **Hosted CI on `522167ac6`:**
  - pull_request runs: Piping Desktop E2E **36673665663**, Harness Pre-merge **36673665575**, governance-harness **36673665617** and pec-tests **36673665543**, all successful. 12 checks passed and 4 were skipped as selected;
  - the full-SHA dispatch **36673660523** (target_base `dd61120ff271b02bf5fcb032f264564af5e8bb09`): success;
  - the numerical cargo job took 23.9 min on the pull_request run and 20.9 min on the dispatch.
- **DEC-025,** under the owner's Mac decision (`OWNER_DIRECTION.md`, 2026-09-28), on the final head `522167ac6`, with a fresh sweep target and a clean sweep worktree. The evidence is in `dec025/`, and the driver is `M03_SKEW_PIN_MERGE/dec025/dec025_mac.sh.txt`, unchanged.
  1. **The sweep invocation** failed at the cargo surface on product_physics's platform test `t13` (fail-fast), as expected on the Mac.
  2. **All 40 manifests** were run with `--no-fail-fast`, against KF3's Mac run at `aa83f6796`, whose piping source equals main `dd61120ff`'s. The comparison is keyed by manifest path (`suites_vs_baseline.txt`).
     - frame_kernel grows 417 → 432, with 1 ignored. That is exactly KF2's added tests: `#[test]` +16, of which one, the release cost test, is ignored.
     - The failing tests are exactly the baseline's three Mac platform tests.
  3. **Surfaces 2, 3 and 5:**
     - pytest: 3070 passed, 32 skipped;
     - the wasm build: exit 0;
     - vitest: 134 files, 2822 of 2822;
     - the production build: exit 0.
  - Machine paths are sanitized (`<wt>`, `<VENV>`), and ANSI colour codes are stripped. The unsanitized JSON's sha256 is in `sweep_json_original_sha256.txt`.
- **GEN-8** on `522167ac6`: 1 passed (ROOT, with `set -o pipefail`, in a clean working tree of the head).

## What KF2 established

- **The dense witness costs O(n²)** in all. At 6,006 DOFs it takes 0.875–0.966 s, where before it was about 31 days by prediction and was killed at 1,800 s in K6's B2 and F1b's gate part 2.
- **N10's two models** (RF-LARGE-CHAIN-n01000-ROT and RF-LARGE-TREE-n01000-AX, dense) now end in the dense factor's refusal, with the witness finding no pair. Dense scrutiny on such models ends in minutes (the factor's time).
- **Every result is bit-identical,** errors included, by argument (plan §4), by I20's and RV24's differential tests, and by T9 and the gate.

## Routed

- **The dense pivot screen:** a separate slice, with an owner-facing note ("KF2: rulings on I20's checkpoint-0 plan", Q3). It carries RV24-N4's notes: measure the refusing rows' pivots, enumerate class changes by running, and treat `operation_count` as a contract note.
- **Dense cancellation:** a cancelled dense job keeps its thread and memory until the solve returns. This goes to T6 and T9 as an observation.
- **V-K's 103-member dense parity** is now affordable (CHANGE_RECORD's notice).

## Scratch to prune

`<wt>/kf2-target`, `<wt>/scratch/i20/` (including the uncommitted 606 MB gate `runs.jsonl` files, whose sha256 is recorded), `<wt>/scratch/sweep_kf2`, and `<wt>/kf2` once no follow-up needs it.
