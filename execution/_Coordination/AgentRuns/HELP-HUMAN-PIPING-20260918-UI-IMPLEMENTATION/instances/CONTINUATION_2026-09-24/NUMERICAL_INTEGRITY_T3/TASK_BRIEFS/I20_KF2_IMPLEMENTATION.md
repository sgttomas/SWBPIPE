# I20: implement slice KF2 (K6's N10: bound the dense negative-pair witness with every result unchanged, and diagnose the dense pivot screen)

> This is an implementation TASK. Read Root `AGENTS.md`, `agents/AGENT_TASK.md` and `_COMMON.md` first. The Mac host rules in `I8R_K1_RESUME.md` ("The Mac host" and "Platform calibration", `I8R_K1_RESUME.md:24-50`) override `_COMMON.md`'s host section, and apply to you in full.

## Roles

- ROOT (HELP_HUMAN) dispatches you directly, as a background subagent, and is your return path.
- Make no Git writes and no index operations. ROOT commits.
- Record the delegation mechanism in RETURN.

## Paths and bases

- `P/`, `T3/`, `FK`, `H`, `NI`, `PP` and `SA` are as in `_COMMON.md` and `I15_K6_IMPLEMENTATION.md`. `FKS` is `FK/src/structural.rs`, and `FKP` is `FK/src/structural/sparse.rs`.
- **The base is main `78f55f927`** (K6b merged, after K4, KF1 and V-K). ROOT records the actual base at spawn. T3's records are read in `<wt>/numerics`.
- **Branch** `codex/piping-kf2-20260930`, in `<wt>/kf2`.

## Purpose

K6 found N10 on main's dense path ("K6: B2 accepted; the dense-path finding (N10); B3 granted" in `ROOT_RULINGS_V1.md`; K6's RETURN §8.4 on main):
- **The runs:** on RF-LARGE-CHAIN-n01000-ROT and RF-LARGE-TREE-n01000-AX (6,006 DOFs), in dense mode, the dense Cholesky refuses a pivot at DOF 6001 or 6002 after 65–86 s.
- **Then the witness:** `solve_prepared_dense` (`FKS:1960-1967` on main) runs `negative_pair_witness`, which ran until the 1,800 s kill in every run. This includes F1b's gate part 2, where all 8 dense 1,000-member runs timed out on base and candidate alike.
- **The sparse path** publishes both models as Sensitive, so dense scrutiny disagrees in class and also does not end.
- **In the product,** dense scrutiny runs as a cancellable background job, so a user waits until they cancel.

ROOT's reading (to confirm or refute at checkpoint 0):
- **The witness is O(n⁴).** `negative_pair_witness` (`FKS:2195-2215`) visits all n(n−1)/2 pairs. For each it allocates an n-vector and calls `verify_negative_direction` (`FKS:2152-2193`), which re-validates the prepared system and then scans all n² entries, although the direction has only two nonzeros.
- **K1 already built the O(1) pair evaluation.** `sparse_negative_pair_witness` and `pair_energy` (`FKP:1932-2000` or thereabouts) evaluate the pair e_r + sign·e_c from its four terms in the dense row-major order. They visit only stored pairs, and their doc comment argues this returns the dense search's first witness. The dense witness can use the same evaluation, over the same pairs in the same order, and return **bit for bit** what it returns today: O(n²) in all, with no per-pair allocation.
- **The dense pivot screen** (`screen_pivot`, `FKS:1523-1548`) charges `operation_count = 2j + 2` in the dense Cholesky (`cholesky`, at about `FKS:1863`). The skyline charges `2(i − first_i) + 2`. Late in a banded matrix the dense count is far larger, so the dense screen 64·γ(count)·scale is larger, and it refuses pivots the skyline passes. This is I15's hypothesis; it is not proven.

## Scope

1. **Diagnose first** (checkpoint 0, with no code):
   - **The witness's cost,** term by term (pairs, per-pair allocation, validation, the n² scan). State it as a formula in n, and give the predicted time at 6,006 DOFs.
   - **The equality argument:** for every prepared system, including error returns (`checked_value` and `checked_product` overflow, in the order they occur), the O(n²) witness returns what `negative_pair_witness` returns today: the same `Ok(None)`, the same first `NegativeEnergy` with a bit-equal `direction`, `energy` and `allowance`, or the same `Err`.
     - Show that a pair the dense search visits with a zero coupling can never be its witness, or else keep visiting it.
     - Show that the terms, their order, `magnitude` and `terms` are the ones `verify_negative_direction` uses for that direction.
   - **The screen:** why the dense factor refuses at DOF 6001 or 6002 of 6006 where the skyline passes. Give the two operation counts, the two screens and the pivot at the refusing row. Say whether a profile-based operation count for the dense factor is honest: the terms outside the profile are exact zeros, and adding an exact zero rounds nothing.
2. **Bound the witness, results unchanged.** Change `negative_pair_witness` so that its cost is O(n²) in all, with O(1) work and no allocation per pair (the returned direction is allocated once), and so that it returns bit for bit what it returns today for every prepared system.
   - Its signature and its callers are unchanged: `solve_prepared_dense`, NI's `structural_adapter.rs` (about `:2017`) and H's `k6/staged.rs`.
   - Share `pair_energy`'s evaluation with the sparse witness if that is clean. Otherwise keep a dense twin, and give the reason.
   - Say whether a budget or cancellation check inside the witness is still needed once it is O(n²), and where the product's dense route observes cancellation, with file:line. **Do not add a product-visible budget in KF2.** If you think one is needed, propose it.
3. **The screen: diagnose and propose; change nothing in KF2.** A change to the dense screen changes published dense classes (dense scrutiny's standing on models like the two above), so it is a ROOT ruling, and possibly an owner-facing note, not part of this slice.
   - At checkpoint 0, propose the change if it is honest, with its proof and the list of committed and gate requests whose dense class would change. ROOT then rules it into KF2, splits it out, or declines it.
4. **Nothing else changes.** Every committed output, every T9 hash, and every part-1 gate envelope is byte-identical. Only time changes.

## Write set

- `FKS`: `negative_pair_witness`, and one private helper for the pair evaluation if needed.
- `FKP`: only to share `pair_energy`, if you choose to, with the sparse witness's results unchanged.
- **FK's tests:** a new test file, or `FK/src/structural/sparse/tests.rs` for the shared helper. Include a test-only reference copy of today's `negative_pair_witness` and `verify_negative_direction`.
- `FK/tests/s11_site_table.rs`: **one declared, additive row, only if** the table requires it for the new checked-arithmetic site. Name it at checkpoint 0.
- `T3/IMPLEMENTATION/KF2/`.

Anything else is a stop, including NI, PP, SA, H and the screen.

## Required tests

- **Equality:** a differential test in which the new witness and the reference copy return bit-identical results, direction, energy and allowance included, on randomized and adversarial prepared systems. The systems must cover:
  - a witness at the first pair, at the last pair, and none;
  - zero couplings next to a witness;
  - ties at the allowance's edge (energy = −allowance, and one ulp either side);
  - both coupling signs;
  - an overflow `Err` inside a pair's terms, before and after a would-be witness;
  - n from 2 up to at least 64.
- **The cost:** a test that counts pairs and term evaluations and asserts O(1) terms per pair. A 1,000-member-class prepared system (at least 6,006 DOFs), built without the factor, finishes the witness under a bound you state, in release. A debug-suite test may use a smaller n.
- **N10's two models:** in release, as an example or an ignored test (not in CI). RF-LARGE-CHAIN-n01000-ROT and RF-LARGE-TREE-n01000-AX, dense, end with a published outcome: the factor's refusal or a `NegativeEnergy` witness. Record the outcome, the factor's time and the witness's time.
- FK's full suite, and the suites of NI, PP, SA and H, which call the witness. `gen_k4_vectors.py --check` is unaffected.
- **Mutants:**
  - the witness skipping the first pair;
  - the witness evaluating the pair's terms in another order;
  - `terms` off by one in the allowance;
  - the witness returning the last witness instead of the first;
  - the zero-coupling skip applied to a pair it must visit (if the argument allows a skip).
  - All must be killed. The NONE control passes.

## Gates (ROOT runs the PR)

KF2 is **product-reaching**: dense scrutiny and NI's structural adapter call the witness.
- **An independent reviewer,** directed to the equality argument (errors included), the differential test's coverage, and the screen diagnosis.
- **T9:** 112 of 112 byte-identical, Mac-only, from `git archive` copies of base and candidate.
- **The both-entry gate,** as F1b's (`I13_F1B_IMPLEMENTATION.md`, "Gates"), on this Mac:
  - **Part 1** (884 runs): byte-identical to a Mac run of the base.
  - **Part 2** (the four dense N10 runs, both entries): each candidate run ends within 1,800 s, with its outcome recorded. The base's timeouts are already recorded (F1b's gate and K6's B2), so the base is not re-run unless ROOT asks.
  - Because no oracle can finish the old search at 6,006 DOFs, the equality at that size rests on the argument and the differential test.
- **Hosted CI** with the full-SHA dispatch.
- **DEC-025** with a fresh sweep target.
- **GEN-8.**
- **The src-tauri suite,** as F1b ran it.

## Checkpoints

- **0: the diagnosis and plan,** with no code: the cost formula, the equality argument, the screen diagnosis and any proposal, the design, the tests and mutants, and whether the site table needs a row.
- **A:** the change, the tests, the suites, and N10's two models in release.
- **B:** T9 and the both-entry gate, in a slot ROOT grants.
- **D:** RETURN, CHANGE_RECORD, `_run_records/` and SHA256SUMS.

**Stop and report** on any committed-byte or T9 change, any part-1 envelope change, a result difference in the differential test, an edit outside the write set, or a surviving mutant.

## Host

- One cargo job at `-j 4`, `RUST_TEST_THREADS=2`.
- Your own target, `<wt>/kf2-target`.
- The memory guard running.
- **No building during a timed slot ROOT grants to another slice.** KF3's slot B is running at spawn: checkpoint 0 is reading only, and your first build waits for ROOT's word.
