# V-K merge record

- **PR:** https://github.com/sgttomas/chirality/pull/1057.
  - ROOT (HELP_HUMAN) merged it on 2026-09-30 at 00:26:51Z as `f8400d29059bfa742cffe14028fcd733e89f7a96`: a merge commit with `--match-head-commit 5f0d39426`, under the owner's standing Git authorization.
  - Main was `0f5d8c7b4` (KF1), an ancestor of the head.
- **Candidate head:** `5f0d394262516e4053322730506cb88dfb36a2f0`, on branch `codex/piping-vk-20260929`.
- **Where it ran:** the owner's Mac (`aarch64-apple-darwin`, rustc 1.97.1). ROOT dispatched the implementer (I17) and the reviewer (RV21) directly.
- **Scope:** T3 slice V-K, the VP-ROBUST kernel lane (D1 §4.10), in the new crate `P/validation/benchmarks/numerical_robustness` (VR). It also changes FK:
  - **the `retained_api` export:** commit `3018343c2`, the same patch as K6b's A0 `bb89e4f8f`. It is visibility only, and no product crate names it.
  - **the `mutation-controls` feature:** its fault sites are `cfg(any(test, feature))`, insertions only, plus `K4R/seeded.rs`. With the feature off, FK is unchanged in effect.
  - The account is `IMPLEMENTATION/VK/RETURN.md` (with addendum 1) and `CHANGE_RECORD.md`, on the merged branch.

## The chain (branch point: main `ab02ee3a6`)

| Commit | Content |
|---|---|
| `40421b7f1` | Checkpoint 0: the plan |
| `3018343c2` | K6b's A0: FK's `retained_api` export, cherry-picked |
| `37bff1780` | A1: the crate, the adapters, the kernel lane at CI scale, the floor check and THIN's expected-unresolved list |
| `c1fea8574` | A2: the seeded faults behind FK's `mutation-controls` feature, and the kill matrix |
| `e24e911e6` | C: the harness mutants, and `tests/engine.rs` |
| `24449b5c8` | D, a draft before B |
| `485320e95` | A merge of main `0f5d8c7b4` (KF1), by ROOT, with no conflict |
| `f94342a3d` | The post-KF1 re-run: all 15 faults killed, identical to A2 |
| `64470c6ba`, `a4b8c1957` | B's preparation: `vk_scale`, its runner, and the KF3 exception |
| `f5379a5d4` | B's scale records |
| `3fd1baff3` | D, filled from B |
| `5f0d39426` | RV21's review closed: the feature guard, and the out-of-range and `Overflow` tests (tests and records only) |

## Gates

- **Independent review, RV21** (`REVIEW/VK_REVIEW.md`, with `REVIEW/_run_records/vk_review/`):
  - **PASS** at `3fd1baff3`, with 0 BLOCKING, 2 SHOULD-FIX and 5 NOTEs.
    - No way was found for the harness to pass a wrong answer on a covered row:
      - 225,405 engine vectors against RV21's own `Fraction` oracle;
      - 129,968 wrong answers through the harness's own path, every one failed;
      - nine whole-case probes.
    - The adapter, the floor lists (46, 3 and 2), FK with the feature off, A0 and B's exception were confirmed independently.
    - **The SHOULD-FIX findings:**
      - RV21-1: the feature guard missed a manifest that enables VR's `seeded-faults`;
      - RV21-2: two promised tests were missing (a wrong out-of-range observation, and an `Overflow` row).
  - **PASS** at the final head `5f0d39426` (a confirmation), with no new finding. RV21-H4 and H6 are each killed by exactly their own test.
  - ROOT's rulings: `ROOT_RULINGS_V1.md`, every V-K section.
- **Tests at the final head:**
  - VR: 47 tests in 9 files, about 40 s;
  - FK's suite, with the fault variable unset: 402.
- **Mutants:**
  - 15 seeded faults, all killed, re-run after KF1's merge;
  - 20 harness mutants, all killed (plus NONE and NONE-GEN).
- **Hosted CI on `5f0d39426`:**
  - pull_request runs: Piping Desktop E2E **36646865501**, Harness Pre-merge **36646865424**, governance-harness **36646865511** and pec-tests **36646865507**, all successful. 12 checks passed and 4 were skipped as selected;
  - the full-SHA dispatch **36646861753** (target_base `0f5d8c7b46570dbb8e7efabe542f99d04d63aad9`): success;
  - the numerical cargo job took 18.4 min on the pull_request run and 20.4 min on the dispatch, including VR.
- **DEC-025,** under the owner's Mac decision (`OWNER_DIRECTION.md`, 2026-09-28), on the final head `5f0d39426`, with a fresh sweep target and a clean sweep worktree. The evidence is in `dec025/`, and the driver is `M03_SKEW_PIN_MERGE/dec025/dec025_mac.sh.txt`, unchanged.
  1. **The sweep invocation** failed at the cargo surface on product_physics's platform test `t13` (fail-fast), as expected on the Mac.
  2. **All 40 manifests** were run with `--no-fail-fast`, against KF1's Mac run at `66adfede4`, whose piping source equals main `0f5d8c7b4`'s. The comparison is keyed by manifest path, because V-K adds a manifest (`suites_vs_baseline.txt`).
     - The only change is the new `numerical_robustness` (47 tests).
     - frame_kernel is unchanged at 402: the fault sites are inert with the feature off.
     - The failing tests are exactly the baseline's three Mac platform tests.
  3. **Surfaces 2, 3 and 5:**
     - pytest: 3062 passed, 32 skipped;
     - the wasm build: exit 0;
     - vitest: 134 files, 2822 of 2822;
     - the production build: exit 0.
  - Machine paths are sanitized, and ANSI colour codes are stripped.
- **GEN-8** on `5f0d39426`: 1 passed (ROOT, with `set -o pipefail`).
- **T9 and the both-entry gate:** not run. No product path changes: the export is unused by product crates, and the feature is off in every product manifest, which a CI-run test checks.

## What V-K established

- **R1's kernel lane passes through W1a at CI scale:** 201 cases, 25,704 covered rows, 0 failures.
  - The `not_covered` set equals the committed list.
  - All 506 discriminating controls fail.
  - RF-MECH is refused.
  - RF-RANGE admits LEF-small.
- **THIN-A and THIN-B are W1a's design limit:** EA/(12EI/L³) ≈ 2^507, confirmed by GEN. They are on a committed expected-unresolved list.
- **The scale runs** (examples; B):
  - 100 and 1,000 members: all 12 models selected at 128, and every row passes.
  - **10,000 members, pre-KF3:** CONT-AX is selected, with 4 absolute-range passes. The other five end `Unresolved(ExactSumSpan)`, which KF3 fixes; V3 re-runs after KF3.
- **Per-case work records** for ROOT's W1 limits (RETURN §16).

## Routed

- **V3 (10,000 members)** re-runs after KF3 merges, as a records addendum.
- **VR's admission estimate** is a cited copy of K6b's E_max. It is deduplicated once K6b merges, and updated for RV22-2's added terms.
- RV21's NOTEs N2 to N5 are in RETURN addendum 1.

## Scratch to prune

`<wt>/vk-target`, `<wt>/vk-mut/`, `<wt>/scratch/i17` and `<wt>/scratch/sweep_vk`. The records above keep every hash.
