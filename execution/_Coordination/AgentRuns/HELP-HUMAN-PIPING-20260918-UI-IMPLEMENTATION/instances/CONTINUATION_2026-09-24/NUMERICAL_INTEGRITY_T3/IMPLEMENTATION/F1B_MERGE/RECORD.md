# F1b merge record

- **PR:** https://github.com/sgttomas/chirality/pull/1052.
  - ROOT (HELP_HUMAN) merged it on 2026-09-29 at 05:31:23Z as `59cb200730eef01a805a5b9f4b840a845ff4204b`: a merge commit with `--match-head-commit 6fa422979`, under the owner's standing Git authorization.
  - Main was `1cdeae2c1` (K5), an ancestor of the head.
- **Candidate head:** `6fa42297926e7a218a1dc55187895a6784474aa1`, on branch `codex/piping-f1b-20260928`.
- **Where it ran:** the owner's Mac (`aarch64-apple-darwin`, rustc 1.97.1). ROOT dispatched the implementer (I13) and the reviewer (RV17) directly.
- **Scope:** T3 facade slice F1b, in the product only (`P/core/product_physics`, plus NI's `s11k_tests.rs` pin table):
  - sparse wiring (W3 at the facade);
  - the dense-scrutiny resource guard (96 B per n² entry, provisional 6 GiB);
  - the DEC-050/053 observation-lane guard (24 B per identity-order profile entry, same provisional ceiling);
  - W2 force scaling at formation on linear invocations, after the ordinary attempt and after exact-block.

  The full account is `IMPLEMENTATION/F1B/CHANGE_RECORD.md` and `RETURN.md` (with addenda 1 to 3), on the merged branch.

## The chain (base main `e7d930d49`)

| Commit | Content |
|---|---|
| `94e543a24` | Checkpoint A1: sparse wiring and the dense-scrutiny guard (a pure refactor at b = 0) |
| `e215c6007` | Checkpoint A2: W2 at formation in the product |
| `948e0bb99` | Checkpoint C: two tests that kill the first-round survivors. The first gate ran here and **FAILED** on 4 C2 heap-cap aborts in main's DEC-050/053 lane |
| `130445db2` | The observation-lane guard (ROOT's heap-cap ruling). The gate re-ran here and **PASSED** |
| `9ecf2bdca` | Checkpoint D records, committed on this branch |
| `9aeed9c22` | A merge of main `b37331092` (App v4 files and T3 records) |
| `07bed2638` | The `pressure_thrust_load` product pin (tests only), addendum 1 |
| `c4879c496` | A merge of main `1cdeae2c1` (K5, #1044). F1b merges second |
| `f183e1fa9` | Addendum 2: the re-run after K5's merge |
| `6fa422979` | Tests for RV17's SHOULD-FIX findings, and D10 corrected (addendum 3) |

## Gates

- **Independent review, RV17** (`REVIEW/F1B_REVIEW.md`, with `REVIEW/_run_records/f1b_review/`):
  - **PASS** at `f183e1fa9`: 0 BLOCKING, 4 SHOULD-FIX, 5 NOTEs.
    - **RV17-1:** D10's step 4 was false (exact-block does select range-triggered cases, CX-F and CX-G). ROOT's A2(c) premise was withdrawn; coexistence rests on the arm order.
    - **RV17-2 and RV17-3:** two surviving mutants.
    - **RV17-4:** the C1 disclosure.
  - **PASS** on the delta at `6fa422979`: all four resolved, and RV17 killed F1B-M2, RV17-M1 and RV17-M2 independently from clean archives.
    - The new hash pins equal Mac main's bytes on RV17's own build.
    - The typed-entry deviation (exact-block needs a capture) is confirmed.
  - ROOT's rulings: `ROOT_RULINGS_V1.md`, "F1b: rulings on RV17's review".
- **The both-entry gate,** against G1's full-envelope Mac base of `e7d930d49`:
  - **part 1 at `130445db2`: PASS.**
    - 0 trusted breaches.
    - C3: 832 runs, 0 differences.
    - C1: exactly the ruled 28.
    - C2: 0 heap-cap aborts.
    - The 4 CONT n10000 sparse runs now publish Sensitive / `needs_recompute`, with the lane `not_observed`.
  - **part 1 again at `c4879c496`** (after K5's merge): PASS, with all 884 runs identical to the `130445db2` gate.
  - **part 2 at `130445db2`:** all 8 dense 1,000-member runs time out at 1,800 s on both sides, with no mismatch. It was not re-run after K5, since K5 cannot shorten a dense run.
  - Of the 14 published C1 runs, 8 lie outside R1's exact references. All are Sensitive and equal main's same-family pattern. "Within the criterion" covers trusted publications only (ROOT, RV17-4).
- **T9 (Mac-only):** 112 of 112 at `130445db2` and again at `c4879c496`, plus the extra corpus at 16 of 16.
- **Mutants:** 53 counted, all killed (RETURN addendum 3), including the lane-guard mutant and the NI pin mutants. RV17 re-ran all 18 NI pin mutants and killed them.
- **Hosted CI on `6fa422979`:**
  - pull_request runs: Piping Desktop E2E **36520867670**, Harness Pre-merge **36520867805**, governance-harness **36520867770** and pec-tests **36520867714**, all successful. The Numerical cargo suite passed on Linux, the first cross-platform check of the three full-envelope hash pins.
  - the full-SHA dispatch **36524065976** (target_base `1cdeae2c1de34566d5733ae977dd78e845706c4b`): success.
- **DEC-025,** under the owner's Mac decision (`OWNER_DIRECTION.md`, 2026-09-28), on the final head `6fa422979`. The evidence is in `dec025/`, and the driver is `M03_SKEW_PIN_MERGE/dec025/dec025_mac.sh.txt`, unchanged.
  1. **The sweep invocation** failed at the cargo surface on product_physics's platform test `t13` (fail-fast), as expected on the Mac. The summary JSON is sanitized; its original sha256 is in `sweep_json_original_sha256.txt`.
  2. **All 39 manifests** were run with `--no-fail-fast`, against K5's Mac run at `babcf5e65`, whose piping source equals main `1cdeae2c1`'s (`suites_vs_baseline.txt`).
     - Only product_physics (529 → 569) and nonlinear_integration (133 → 134) change: F1b's 41 added tests.
     - The failing tests are exactly the baseline's three Mac platform tests.
  3. **Surfaces 2, 3 and 5:**
     - pytest: 3023 passed, 32 skipped;
     - the wasm build: exit 0;
     - vitest: 134 files, 2822 of 2822;
     - the production build: exit 0.
  - Machine paths are sanitized to `<WORKTREE>`, `<VENV>`, `<wt>`, `<home>` and `<tmp>`, and ANSI colour codes are stripped.
- **The src-tauri suite** (`apps/desktop/src-tauri`, `cargo test --offline --locked`, from a clean `git archive 6fa422979`): **116 passed**, 0 failed (`src_tauri/`).
  - It runs neither in the sweep nor in CI. The earlier facade merges recorded 114, and tests have been added since.
- **Native witnesses:** not run for this merge. ROOT's ruling: the brief makes §7.5's PHYS-R4 and 1,000-member native witnesses join items. F1b changes the product's solve path, which the both-entry gate covers on both entries and the src-tauri suite covers for the desktop's Rust side. F1b changes no native shell or desktop file.
- **GEN-8** on `6fa422979`: 1 passed (run with `set -o pipefail`).

## Provisional items for the owner (carried to the ceiling decision)

- **The dense-scrutiny and observation-lane ceilings are provisional at 6 GiB,** the gate's heap cap on this Mac. Two K6 observations bear on the final values:
  - **the product-level gap:** CONT n10000 sparse takes about 250 MiB of heap in the kernel, against 4.8–5.2 GiB of RSS at product level. F1b's attribution puts most of that in result-row publication;
  - **macOS resident memory:** it exceeds the estimated heap by about 1.45× for dense at 1,000 members.

  V-P's product-level runs are needed as well (ROOT's K6 rulings at B1 and at the B2 stop).
- **The lane guard's 24 B per entry bounds the identity-order build only.** The lane's peak is about 16P + 24P′ (RV17-N2). K6 measures it.

## Scratch to prune

- `<wt>/scratch/i13` (about 27 GB with `k5m/`);
- `<wt>/gate-cand-full-target` and `<wt>/gate-cand2-full-target`;
- `<wt>/f1b-target`;
- `<wt>/tauri-f1b-target` and `<wt>/scratch/tauri_f1b`.

The records above keep every hash.
