# K6b merge record

- **PR:** https://github.com/sgttomas/chirality/pull/1058.
  - ROOT (HELP_HUMAN) merged it on 2026-09-30 at 01:08:38Z as `78f55f927db47c7f44299fd32793d4b64e8b3572`: a merge commit with `--match-head-commit 597c81ba4`, under the owner's standing Git authorization.
  - Main was `f8400d290` (V-K), an ancestor of the head.
- **Candidate head:** `597c81ba4b3dcf9a1154ef438a3c827054a57ad9`, on branch `codex/piping-k6b-20260929`.
- **Where it ran:** the owner's Mac (`aarch64-apple-darwin`, rustc 1.97.1). ROOT dispatched the implementer (I16) and the reviewer (RV22) directly.
- **Scope:** T3 slice K6b: W1 observations on the K6 harness (H, `P/core/solver/performance_harness`). It records limbs, work and memory per precision, and seconds per work unit.
  - **The `w1a` mode:** the K6-model adapter, W1's counts and admission estimate (E_max, E_sel128), and the attempt and prefix records.
  - **The runner's W1 tiers:** these include the by-name deferral of any row the binary's backstop would refuse. There is also the analysis and the W1 packet.
  - **The observations:** `H/observations/k6b/`.
  - **FK's `retained_api` export (A0, `bb89e4f8f`):** V-K carried the same patch and merged it first, so K6b's FK equals main's at the merge. No product crate names the export.
  - **Observation only:** no test or record asserts a time or memory bound.
  - The account is `IMPLEMENTATION/K6B/RETURN.md` (with addendum 2) and `CHANGE_RECORD.md`, on the merged branch.

## The chain (branch point: main `ab02ee3a6`)

| Commit | Content |
|---|---|
| `c0436769f` | Checkpoint 0: the plan |
| `bb89e4f8f` | A0: FK's `retained_api` export (visibility only) |
| `fdbf132d4` | A0: H's test that the export suffices from outside FK |
| `73031b134` | A1: the `w1a` mode, the adapter, W1's counts, the attempt and prefix records, and the W1 estimate |
| `f4d40dd17` | A2: the runner's W1 tiers (K6's four modes frozen), the W1 analysis, and counts-only records for the 33 sealed models |
| `8bbcdfc8b`, `1f5c1a6d0` | C: the by-name deferral of any row the backstop would refuse; each prefix limit pinned to its segment's end (kills M10) |
| `b86081221` | A merge of main `0f5d8c7b4` (KF1), by ROOT |
| `082990c8d` | E_max follows KF1's bounded trackers at every site; counts regenerated |
| `4eeb206c0` | The stage-identity check on stopped builds (against the charged totals, with the unstaged work reported); `Span` outcomes recorded, not treated as stops |
| `126fcb9f3` | D: the W1 packet (`k6b_analysis --packet`) and b3's committed packet |
| `1123d19b9` | D: RETURN, CHANGE_RECORD and run records (W1-T4 pre-KF3) |
| `011911e4e` | RV22's review closed: the stage check tightened on stopped builds; E_max bounds every phase; the committed counts tied to the code (addendum 2) |
| `597c81ba4` | A merge of main `f8400d290` (V-K), by ROOT. Its conflicts were in FK only, and resolved to main's side (A0 is identical on both) |

## Gates

- **Independent review, RV22** (`REVIEW/K6B_REVIEW.md`, with `REVIEW/_run_records/k6b_review/`):
  - **PASS** at `1123d19b9`, with 0 BLOCKING, 3 SHOULD-FIX and 7 NOTEs.
    - **No product byte changes:** A0's patch-id equals both A0 commits.
    - **The `w1a` runs:** RV22's release runs reproduce b3 byte for byte, and the work closure has 0 discrepancies over 330 outcomes.
    - **The RETURN's values:** 934 of them trace to the raw JSONL with 0 mismatches.
    - **The SHOULD-FIX findings:**
      - RV22-1: the stopped-build relaxation also applied to completed builds;
      - RV22-2: E_max omitted terms of the 1024 pass and of the solve-phase fallback;
      - RV22-3: nothing tied the committed counts to the code, and RV22-M6 survived.
  - **PASS** at `011911e4e` (a confirmation): all three are closed. Two new NOTEs:
    - C-N1: no test covers a stop inside the solve;
    - C-N2: summed stages can hide one build's shortfall, which KF3's equality closes.
  - **PASS** at the final head `597c81ba4` (a merge check), with no findings:
    - 0 mismatches path by path over 59,471 paths;
    - FK equals main's byte for byte;
    - H passes 74 tests plus `k6_alloc`, and the runner 47 of 47, on a clean archive.
  - ROOT's rulings: `ROOT_RULINGS_V1.md`, every K6b section.
- **Mutants:**
  - C and the parity fix: 24 mutants, with 23 killed and M3e equivalent (derived: Iy = Iz in every section used).
  - Addendum 2: M21, M22, M23, RV22-M6, E1, E2, RV22-M5L and RV22-M7 killed.
  - RV22-M5B survives as recorded: the binary only calls the tested predicate.
- **Hosted CI on `597c81ba4`:**
  - pull_request runs: Piping Desktop E2E **36650521264**, Harness Pre-merge **36650521165**, governance-harness **36650521163** and pec-tests **36650521260**, all successful. 12 checks passed and 4 were skipped as selected;
  - the full-SHA dispatch **36650532005** (target_base `f8400d29059bfa742cffe14028fcd733e89f7a96`): success;
  - the numerical cargo job took 22.3 min on the pull_request run and 18.9 min on the dispatch.
- **DEC-025,** under the owner's Mac decision (`OWNER_DIRECTION.md`, 2026-09-28), on the final head `597c81ba4`. The shared sweep target was fresh (deleted before and after), and the sweep worktree was clean. The evidence is in `dec025/`, and the driver is `M03_SKEW_PIN_MERGE/dec025/dec025_mac.sh.txt`, unchanged.
  1. **The sweep invocation** failed at the cargo surface on product_physics's platform test `t13` (fail-fast), as expected on the Mac.
  2. **All 40 manifests** were run with `--no-fail-fast`, against V-K's Mac run at `5f0d39426`, whose piping source equals main `f8400d290`'s. The comparison is keyed by manifest path (`suites_vs_baseline.txt`).
     - The only change is performance_harness, 54 → 74 (K6b's tests).
     - frame_kernel is unchanged at 402.
     - The failing tests are exactly the baseline's three Mac platform tests.
  3. **Surfaces 2, 3 and 5:**
     - pytest: 3070 passed, 32 skipped. V-K's run had 3062; the difference is K6b's runner tests (`test_k6_runner.py`, 39 → 47);
     - the wasm build: exit 0;
     - vitest: 134 files, 2822 of 2822;
     - the production build: exit 0.
  - Machine paths are sanitized (`<wt>`, `<VENV>`), and ANSI colour codes are stripped. The unsanitized sweep JSON's sha256 is in `sweep_json_original_sha256.txt`.
- **GEN-8** on `597c81ba4`: 1 passed (ROOT, with `set -o pipefail`, in a clean working tree of the head).
- **T9 and the both-entry gate:** not run. No product path changes: K6b's FK equals main's, and H is the harness.

## What K6b established

W1-T4, at 10,000 members, is pre-KF3 throughout.
- **W1's growth and memory:**
  - W1's heap grows with slope 0.92–0.98 per family, from 10 to 10,000 members.
  - The measured heap is 4–31% of E_max, and ρ_fp is at most 0.43, at every size.
  - After RV22-2, E_max at 10,000 members is 2,708–2,873 MiB, and E_sel128 is 1,065–1,102 MiB.
- **CONT-AX at 10,000 members** (the one selected model):
  - it charges 8,234,772,320 LME;
  - its heap peaks at 814.0 MiB, and a median call takes 6.09 s (0.74 ns/LME, 7.6 × the binary64 sparse `entry_checked`);
  - the stop rule is 31.5% of the call;
  - it publishes 265,013 rows, and R1's 215 values at 10,000 members pass.
- **The findings K6b routed:**
  - K4's data-dependent stop-rule memory, fixed by KF1;
  - the backstop and runner disagreement, fixed at C;
  - W1's `Span` stop at 10,000 members on five of six models, and the unstaged partial work on stopped builds, both routed to KF3.
- **Observations for ROOT's W1 limits:** time, work and memory, in RETURN §8. ROOT sets the limits from the post-KF3 figures.

## Routed

- **W1-T4 (10,000 members)** re-runs after KF3 merges, as a records addendum (RETURN addendum 1), with `counts.jsonl` regenerated if needed.
- **KF3 merges second,** so it restores `stages_equal_totals` to equality on every attempt and updates K6b's `uc == 0` test ("KF3: main merged; K6b's parity restored in KF3; B's slot granted"). RV22's C-N2 closes there.
- **VR's cited copy of E_max** is deduplicated against K6b's, with RV22-2's terms.
- **RV22's C-N1** (no test of a stop inside the solve) stays open as a NOTE. Its two ready-made paths (RV22-N7) go to KF3.

## Scratch to prune

`<wt>/k6b-target`, `<wt>/k6b-mut/`, `<wt>/scratch/i16` and `<wt>/scratch/sweep_k6b`. The records above keep every hash.
