# KF2 change record: the dense negative-pair witness in O(n²), every result unchanged

This is the draft PR record for slice KF2 of T3 (numerical integrity; K6's finding N10), following `.agents/skills/chirality-change/SKILL.md`. It was implemented by I20, a TASK. The details are in `RETURN.md`, and the full equality argument is in `PLAN_CHECKPOINT0.md` §4.

- **Branch:** `codex/piping-kf2-20260930`, from main `78f55f927` (K6b merged).
- **Commits (made by ROOT):**
  - `573bd3835`: checkpoint 0, the plan;
  - `1b10121fa`: A, meaning the code, the tests, the site-table row and `_run_records/a/`;
  - `6caa38e23`: B, meaning T9, the both-entry gate and src-tauri (`_run_records/b/`);
  - the next commit: D, meaning `RETURN.md`, this record and `SHA256SUMS`.
- **Size (against main `78f55f927`):**
  - `P/core/solver/frame_kernel/src/structural.rs`: +57 −6;
  - `P/core/solver/frame_kernel/src/structural/kf2_witness_tests.rs`: new, 969 lines;
  - `P/core/solver/frame_kernel/tests/s11_site_table.rs`: +2;
  - records under `T3/IMPLEMENTATION/KF2/`.
- **Basis:**
  - the I20 brief (`TASK_BRIEFS/I20_KF2_IMPLEMENTATION.md`);
  - the checkpoint-0 plan (`76337c43…`);
  - `ROOT_RULINGS_V1.md`'s sections "KF2: spawn", "KF2: rulings on I20's checkpoint-0 plan", "KF2: checkpoint A accepted; B granted" and "KF2: checkpoint B accepted; D now".
- **Platform:** Mac (`aarch64-apple-darwin`, 128 GiB), rustc 1.97.1.

## What changes

- **`negative_pair_witness` is O(n²).** It keeps its signature, doc comment and callers, and now delegates to the new private `negative_pair_witness_counted`, which holds the loop, the counts and the pair evaluation inline.
  - Each pair (i, j < i) is visited in today's order. Its only four cells, (j,j), (j,i), (i,j) and (i,i), are evaluated in O(1) with exactly `verify_negative_direction`'s operations and order.
  - Only a pair judged a witness is built and passed to the unchanged `verify_negative_direction`, which publishes the witness.
  - There is no allocation per pair, and zero-coupling pairs are still visited.
- **The S11 site table** gains one declared, additive row, authorized by ROOT (Q5): `negative_pair_witness_counted`, with 6 matches (four integer counts, and the energy and magnitude sums). No other row changes.
- **Test mount:** one `#[cfg(test)] mod kf2_witness_tests;` line in `structural.rs` (Q1).
- **Unchanged:**
  - `verify_negative_direction`, and every caller: FKS `solve_prepared_dense`, SA `:2017`, SD `:37`, H `k6/staged.rs:474`;
  - the sparse witness;
  - the dense pivot screen, which is declined and split out;
  - PP's site test;
  - every file outside FK.

## Results

- **Equality:**
  - The new witness returns today's result bit for bit, errors included, for every `PreparedSystem` (the plan's §4, accepted by ROOT).
  - The differential tests against verbatim reference copies (n = 2 to 100) cover:
    - the witness first, last, in the middle, absent, and several at once;
    - ±0 couplings, and both coupling signs;
    - 240 random systems (142 witnesses, 98 without);
    - all 9 exact allowance ties found, with b ± 1 ulp on each side;
    - overflow and underflow errors before and after a witness;
    - corrupted systems.
  - The count tests show O(1) cells per pair and at most one verification.
- **Cost:**
  - **Before:** about 3·c_r·n⁴, calibrated at 0.68 ns per read. That is about 31 days at 6,006 DOFs, as a lower bound.
  - **After:** 0.875 s at n = 6,006 in release (18,033,015 pairs).
  - **N10's two models** now end with the dense factor's refusal at DOF 6001 and 6002. The witness takes 0.94 s and 0.91 s and finds no pair. Before, they were killed at 1,800 s.
- **Suites:**
  - FK: 415 passed, 1 ignored (the release-only cost test);
  - the site table: 3 of 3;
  - SD: 30; NI, with SA: 134; H: 74, plus `k6_alloc`;
  - PP: everything passes except the known Mac platform test `t13`, which fails identically on the base;
  - src-tauri: 116.
- **T9 and the both-entry gate:**
  - T9 is 112 of 112 byte-identical, and F1b's extra corpus 16 of 16.
  - Gate part 1: all 884 runs are identical to a fresh Mac base run, full envelopes included, and `gate_check` passes on both sides with 0 trusted breaches.
  - Gate part 2: the four dense N10 runs end in 67–68 s, each with the factor's `NUMERICAL_INTEGRITY_UNRESOLVED`.
- **Mutations:** NONE passes, and 11 of 11 are killed, among them the brief's five:
  - skipping the first pair;
  - the cells in another order;
  - `terms` off by one;
  - returning the last witness;
  - the zero-coupling skip.

## Limits

- **Mac only.** No timing claims: every time is an observation with its load, and other agents' builds overlapped the runs.
- **The dense witness returns `Some`** in the tests only. No gate or T9 run publishes a `NegativeEnergy`; 16 gate runs reach the witness, and it finds no pair.
- **The screen diagnosis's pivot figures** are predictions from the model's structure. No probe measured them.
- **The part-1 `runs.jsonl` files** (606 MB each) are uncommitted. Their hashes are recorded.

## Gates (ROOT runs the PR)

- An independent reviewer, directed to the equality argument (errors included), the differential test's coverage and the screen diagnosis.
- Hosted CI, with the full-SHA dispatch.
- DEC-025, with a fresh sweep target.
- GEN-8 before the records commit that goes to main.
- T9, the both-entry gate and the src-tauri suite: done at B (`_run_records/b/`).

## Downstream notices

- **The dense-screen slice (split out, not yet scheduled):**
  - the profile-based operation count, with its proof (plan §7.3);
  - the expected dense class changes: the N10 pair certainly; RF-CHAIN-A and RF-CHAIN-T at n10-r1e-12 likely; RF-SKEW-T-PIN-OFF-122 and RF-WEAK-W-L at r1e-12 as candidates;
  - every dense report's published pivot evidence changes bytes, and an owner-facing note is needed.
- **T6 and T9 (routed by ROOT):** the desktop's dense route observes cancellation only before the solve and before publication (APP:1688-1692, :1711-1717). After KF2 the dense factor is the long step, 65–188 s at 1,000 members up to the ceiling.
- **V-K:** the 103-member RF-MECH-DISC dense parity, which stayed out of CI because of the witness's cost (about 300 s each), could now be reconsidered by its owners.
- **The reviewer:** I20's scratch (`<wt>/scratch/i20/`, `<wt>/kf2-target`) is kept until KF2 merges.
