# V-K change record: the VP-ROBUST kernel lane (`numerical_robustness`)

This is the draft PR record for slice V-K of T3 (numerical integrity). It follows `.agents/skills/chirality-change/SKILL.md`. I17 (TASK) implemented the slice, and ROOT made every commit. The details are in `RETURN.md`.

> **D, with B done.** KF1 (PR #1056) has merged, and V-K is on the new main
> (`485320e95`), with the kill matrix re-run. B has run. **V3's figures
> (10,000 members) are pre-KF3**, re-run after KF3 merges.

- **Branch:** `codex/piping-vk-20260929`, from main `ab02ee3a6` (K4 merged).
- **Commits (made by ROOT):**
  - `40421b7f1`: checkpoint 0, the plan;
  - `3018343c2`: K6b's A0, FK's `retained_api` export, cherry-picked with the same patch-id as `bb89e4f8f` on the K6b branch. Whichever of K6b and V-K merges first carries it to main;
  - `37bff1780`: A1, the crate, the generator and cases, the exact engine, the kernel lane at CI scale, the floor check, parity, invariance and the records;
  - `c1fea8574`: A2, the seeded faults behind FK's `mutation-controls` feature, and the kill matrix;
  - `e24e911e6`: C, the harness mutants, with `tests/engine.rs` closing three paths R1's data cannot discriminate;
  - `24449b5c8`: D, the draft;
  - `485320e95`: main `0f5d8c7b4` (KF1, #1056) merged in, with no conflict. The kill matrix was re-run on it (RETURN §12.4);
  - `f94342a3d`: the post-KF1 records, plus `check_fault_sites.py`'s `--allow-commit` option;
  - `64470c6ba`: B's code (`src/scale.rs`, `tests/scale.rs`, `examples/vk_scale.rs`, `runner/vk_scale_runner.py`) and its small-size check;
  - `a4b8c1957`: B's setup and the KF3 ExactSumSpan exception in the runner, with its tests;
  - `f5379a5d4`: B, the scale runs' records (`_run_records/b/runs/`, `summary.*`);
  - **D:** this record, RETURN, `_run_records/` and SHA256SUMS.
- **Size** (the product tree against main `0f5d8c7b4`, at `a4b8c1957`):
  - 83 files, +42,867 and −260.
  - **FK:** 17 files, +581 and −260:
    - K6b's A0 export, which is shared with K6b: 11 files, +269 and −260;
    - V-K's own change: +186 lines in 11 existing files (`Cargo.toml` and 10 sources) and the new 126-line `retained/seeded.rs`. Every code line is cfg-gated.
  - **VR (new):** 66 files, all insertions:
    - Rust, 5,005 lines;
    - Python, 1,956 lines (the generator and the runners);
    - the manifest and lock, 137 lines;
    - the README, 147 lines;
    - generated data, 35,041 lines (the cases 2,990 and the observations 32,051).
  - The breakdown below is at C (`e24e911e6`), against the export base `3018343c2`: 73 files, +41,086.
  - **VR (new):**
    - Rust, 4,028 lines: `src` 2,820, `tests` 1,067 and `examples` 141;
    - Python, 1,451 lines: the generator 714 and the runners 737;
    - the manifest and lock, 137 lines;
    - the README, 117 lines;
    - generated data, 35,041 lines: the cases 2,990 and the observations 32,051.
  - Records are under `T3/IMPLEMENTATION/VK/`.
- **Basis:**
  - D1 `DESIGN.md` revision 5a.2 (`fb62ef4a…`): §4.10, §4.8's parity items, §4.1.6.1 and §7.3;
  - K4's method, revision 5a.3 with amendment A1;
  - R1's `references.json` (`7b176dbb…`) and `references.py` (`80d473a7…`), which are read-only and pinned;
  - the I17 brief (`06b16b82…`);
  - the plan (`adaf697a…`);
  - `ROOT_RULINGS_V1.md`'s V-K sections, from the spawn to "C accepted", and K6b's A0 sections.
- **Platform:** Mac (`aarch64-apple-darwin`), rustc 1.97.1. **Every observation is Mac-only.**

## What changes

- **New crate `VR` = `P/validation/benchmarks/numerical_robustness/`:**
  - its own `Cargo.lock`, with dependencies FK, SD and `serde_json` (`float_roundtrip`);
  - `cases/gen_vk_cases.py`, standard library only. It reads R1's three pinned inputs and writes the committed case files: 213 cases, of which 201 are CI-scale, the not-covered list (51), the expected-unresolved list (THIN-A and THIN-B), the five absolute-range rows, 2,428 engine vectors and the large models' sha256. `--check` regenerates in memory;
  - `src/`: the adapter's Rust half and R1's key map (`cases`), the exact bignum predicate and floor (`exact`), S\* (`floor`), the verdicts and tallies (`compare`), the lane (`lane`), the per-case records (`records`), RCM (`rcm`), parity (`parity`), invariance (`invariance`) and SHA-256 (`sha256`);
  - `tests/`: `adapter`, `engine`, `feature_guard`, `files`, `invariance`, `lane`, `parity` and `rcm`, 43 tests in all with the lib's units;
  - `examples/vk_records.rs`, which regenerates the committed records in release, by decision only;
  - `observations/`: `kernel_lane/` (the per-case records, parity and invariance), `seeded/` (the kill matrix and the evidence) and `harness/` (the harness-mutant matrix), each with `SHA256SUMS`;
  - `runner/`: `run_seeded_faults.py`, `check_fault_sites.py` and `run_harness_mutants.py`, standard library only, never CI.
- **FK:**
  - `Cargo.toml` gains the `[features] mutation-controls = []` table;
  - `retained/seeded.rs` is new: the fault selector, which reads `FK_SEEDED_FAULT` once and panics on an unknown id;
  - 15 fault sites in `retained/{adaptive,assemble,bound,factor,ledger,recover,source,verify}.rs` and `structural/sparse.rs`, each behind `#[cfg(any(test, feature = "mutation-controls"))]`, and none on KF1's tracker lines;
  - with the feature off and outside `cfg(test)`, FK is its base code (`check_fault_sites.py`: only comment lines remain).
- **Unchanged:** every product crate and path, K4's method and published rows, R1's references, `.github/**`, `tools/**` and `P/tools/**`. No product manifest names `mutation-controls`, and `tests/feature_guard.rs` checks this in CI.

## What differs from the design's letter (ROOT's rulings)

- **THIN-A and THIN-B are W1a's coverage limit.**
  - §4.10 says an unsolved case fails. These two end `Unresolved(Ceiling)`, which GEN confirms.
  - They are on a committed expected-unresolved list, their rows are counted apart and never as passes, and leaving or joining the list fails the gate.
  - The ruling is "V-K: rulings on I17's A1 stop".
- **C1:** twist and extension use k_t and k_a pre-scaled by an exact power of two where fl(G·J) is not normal (RF-RANGE LEF). The result is bit-identical to §4.10's formula wherever that formula is defined, which is tested on 5,922 coefficients.
- **C3:** §7.3 item 7's "recovered" is recorded as inexact. The mechanisms end unresolved, and the kill is on evidence.
- **C2:** §7.3 items 3, 16 and the relabelling half of 10 are not seeded. R1 cannot discriminate them, and each is cited to K4's killing mutant.
- **C4:** parity is §4.8 items 1–3 at up to 100 members. The 103-member RF-MECH-DISC models are recorded in the example only, because the dense witness takes about 300 s (K6's N10 and KF2).
- **C8:** `hypot(My, Mz)` is decided exactly, with no square root formed.
- **C10:** RF-MECH-K0's k = 0 spring is omitted. The literal model's refusal names the spring.
- **The three input-derived rows** (RF-WEAK-W-AX-rho1e-12 `u.N5.*`) are exempt from "`not_covered` implies `absolute_verified`" (rule 2a).
- **C's ruling:** `tests/engine.rs` adds three constructed-input tests, because R1's data never reaches those paths.

## Results

- **The kernel lane (201 CI cases):**
  - 191 selected (all at 128, verified at 256), 8 RF-MECH refused with a witness, and 2 expected unresolved (THIN);
  - 25,704 rows: 25,321 pass, 51 not covered (the committed list), 282 structural zeros, 50 expected unresolved, 0 absolute-range, **0 failures**;
  - all 506 discriminating controls fail the predicate; 171 are non-discriminating, and none fails unexpectedly.
- **RF-RANGE:** LEF-small and LEF-large are admitted by K4's source and solved on all three bases, which answers stale item 10. There is no range refusal.
- **Parity:** K is bitwise equal between the sparse and dense paths, and the class is the same, on RF-LARGE at up to 100 members and on RF-MECH. The DEC-053 basis holds on every both-Passed model, at most 0.038 of the allowance.
- **The adapter:**
  - K4SRC from path A equals path B's bytes on all 201 models;
  - against K4's adapter output (140 cases) there are 0 unexpected differences;
  - RCM equals SD's on 201 models and 404 synthetic graphs.
- **Invariance:** list permutations change no canonical byte and no published bit, which is asserted. Offsets are bit-identical, and relabelled variants differ at most by 7.4e-25 of the allowance, which is recorded.
- **The kill matrix (A2):**
  - NONE passes 40 of 40, and all 15 seeded faults are killed. VK-F05, F06, F07 and R28 are killed on evidence, as ruled. An unknown id panics.
  - FK's suite with the variable unset passes 394.
  - **After KF1's merge:**
    - the same 15 faults are killed, with identical failing tests and kinds, and NONE passes 43 of 43;
    - `check_fault_sites` against main, allowing only A0's patch lines, reports OK;
    - FK's suite passes 402;
    - VR's suite passes 43 of 43, with every committed per-case record unchanged, work included;
    - VK-R28's release selections are unchanged.
- **The harness mutants (C):** all 18 are killed. NONE (43 of 43) and NONE-GEN, whose regeneration is byte-identical, pass.
- **The CI cost:** VR's fresh build takes 3.8 s, and its tests take 37.9 s in debug on the Mac.
- **The scale runs (B):** all 18 runs are ok, with no stop.
  - 100 and 1,000 members: all 12 are selected at 128, and every row passes. At 1,000 members C9's two floor sets are empty.
  - 10,000 members *(pre-KF3)*:
    - CONT-n10000-AX is selected at 128, with 211 passes and 4 absolute-range passes.
    - The other five end `Unresolved(ExactSumSpan)` in the 256 verification's shared build, and their rows are recorded as `unresolved_availability` under the KF3 exception.
  - W1's heap is 81–86 MiB at 1,000 members and 817–835 MiB at 10,000. The time is 0.6–1.5 s and 5–10 s, with the load recorded.

## Limits

- **Mac only.** No Linux or Windows observation was made.
- **V3 is pre-KF3.** Its figures are replaced when V3 re-runs after KF3 merges.
- **No limit or threshold is set.** Work and outcomes are recorded for ROOT's W1 limits, and time is an observation with its load (B).
- **The kernel lane only.** The product lane is V-P's, after F2a.
- **R1-undiscriminated paths** are covered by K4's mutants (§7.3 items 3, 10 relabelling and 16) or by constructed tests (`tests/engine.rs`).
- **Kill kinds** in the seeded matrix come from a heuristic over the test output. The failing tests are exact.

## Gates (ROOT runs the PR)

- **Review:** an independent complete-diff review on the exact head. The reviewer:
  - re-checks the adapter against `references.py`;
  - re-derives the floor check and the not-covered list;
  - re-runs the seeded faults;
  - probes the harness with a wrong answer of their own.
- **Suites:**
  - FK's full suite with the feature off, which includes K4's;
  - VR's suite, with its time;
  - after KF1's merge: FK 402 and VR 43, both passing on `485320e95`.
- **Hosted CI** with the full-SHA dispatch, and **DEC-025**, where the sweep gains VR's manifest and every other suite is unchanged against the Mac baseline.
- **GEN-8** before the records commit that goes to main.
- **T9 and the both-entry gate are not run.** The scans show no product path change: the export is unused by product crates, and the feature is off in every product manifest.

## Downstream notices

- **ROOT's W1 limits:** RETURN §16.1 gives per-family outcomes and work. RETURN §14 gives B's figures: work, heap, RSS and time with load at 100, 1,000 and 10,000 members, the last pre-KF3.
- **KF3:** five of the six 10,000-member models end `Unresolved(ExactSumSpan)`, at the point RETURN §14.2 locates. V3 re-runs after KF3 merges.
- **F2a:** the evidence that the kill-on-evidence faults rely on (O5), and what F2a publishes for a W1-unresolved case (THIN) (RETURN §16.3).
- **V-P:** how VR extends to the product lane, the nine RF-RANGE `AXIS_TOLERANCE` refusals on the ordinary route (C12), and THIN's ordinary-route standing (RETURN §16.4).
- **The owner's PHYS-R4 decision:** THIN's W1 limit, routed by ROOT.
- **D1 and the T3-close list:** W1 beyond a 512-bit candidate, routed by ROOT.
- **K6b:** the export commit is shared. Whichever PR merges first carries it.
