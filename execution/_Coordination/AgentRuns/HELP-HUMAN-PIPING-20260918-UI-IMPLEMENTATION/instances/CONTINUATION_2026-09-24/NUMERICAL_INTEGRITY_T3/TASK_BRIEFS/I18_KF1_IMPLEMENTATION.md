# I18: implement slice KF1 (bound the memory of K4's stop-rule extreme trackers, with every result unchanged)

> This is an implementation TASK. Read Root `AGENTS.md`, `agents/AGENT_TASK.md` and `_COMMON.md` first. The Mac host rules in `I8R_K1_RESUME.md` ("The Mac host" and "Platform calibration", `I8R_K1_RESUME.md:24-50`) override `_COMMON.md`'s host section, and apply to you in full.

## Roles

- ROOT (HELP_HUMAN) dispatches you directly, as a background subagent, and is your return path.
- Make no Git writes and no index operations. ROOT commits.
- Record the delegation mechanism in RETURN.

## Paths and bases

- `P/`, `T3/` and `FK` are as in `_COMMON.md` and `I15_K6_IMPLEMENTATION.md`. `K4R` is `FK/structural/retained/`, and `K4T` is `P/core/solver/frame_kernel/tests/retained_k4/`.
- **The base is main after K4's merge** (`ab02ee3a6` or later). ROOT records it at spawn. T3's records are read in `<wt>/numerics`.

## Purpose

K6b's A1 found that K4's stop rule keeps memory that depends on the data (`ROOT_RULINGS_V1.md`, "K6b: A1 accepted; K4's stop-rule memory finding"):
- `ExtremeTracker` (`K4R/adaptive.rs:533-590`) keeps every row whose 64-bit ratio approximation lies within `WINDOW_ULPS` = 2^13 of the running extreme. Each is two `ExactWideSum` values, about 4.3 KB, kept until `finish`.
- `decide` holds three trackers until it returns.
- At 100 members the trackers keep 67 to 1,145 rows. The worst case at 10,000 members is about 3.2 GB (6.4 GB with `Vec` slack).

ROOT does not set W1's per-case memory limit until this is bounded, or until the limit accounts for the worst case (same ruling). KF1 bounds it **without changing any result.**

## Scope

1. **Bound the kept set.** Change `ExtremeTracker` so its memory is O(1) in the number of rows offered, independent of the data, and `finish` returns bit for bit what it returns today, for every stream.
   - **ROOT's prior:** when the kept count reaches a threshold T (for example 64), evaluate the kept entries exactly with `directed_ratio` and collapse them to the single entry whose exact ratio is the directed extreme, keeping its approximation. Or keep a running exact extreme from the start.
   - **Show** that the result equals today's `finish` on every stream. The extreme of a set is the extreme of the extremes of any partition, and `directed_ratio` is monotone in the exact ratio. **Also show** that no entry the current code keeps can be lost to a later `retain` in a way that changes the result.
   - State the bound, and T.
2. **Work.** The exact evaluations move earlier and may grow, because entries that `retain` would later drop are now evaluated exactly.
   - K4's `golden_work_counts` pins change. Re-pin them with the derivation of each change.
   - The work stays charged to the candidate's stop rule, as today (R7 item 7), with budget checks.
   - Report the worst-case work increase per row.
3. **Nothing else in K4's method changes:** every published row, class, bound, outcome, attempt record other than the stop rule's work, and every control's GEN comparison.
   - GEN (`gen_k4_vectors.py`) mirrors the tracker only if it models it. Say whether it does.

## ROOT rulings for this slice

- **Kernel only.** `K4R` is reachable outside FK only through `retained_api`, whose only consumer is K6b's harness, and V-K's once it lands. No product crate names it, so T9 and the both-entry gate are not run. The reviewer re-runs the scan.
- **K6's N10** (the dense witness) is **not** in KF1. It is product-reaching and gets its own slice (KF2).

## Write set

- `K4R/adaptive.rs`: `ExtremeTracker` and its callers in `decide`, only as needed.
- `K4T/`: tests, the re-pinned golden work, and GEN if it models the tracker.
- `T3/IMPLEMENTATION/KF1/`.

Anything else is a stop.

## Required tests

- **Equality:** a differential test in which the bounded tracker and the current tracker, kept as a test-only reference copy, give bit-identical `finish` results on randomized and adversarial streams. These include all ties, ties at the window edge, a new extreme arriving after many ties, zeros, and both directions.
- **The memory bound:** the kept count never exceeds T, asserted inside the test.
- K4's full suite and FK's full suite, with the controls token-equal to GEN.
- `gen_k4_vectors.py --check`.
- **A mutant** that collapses with the wrong direction, and one that drops the collapsed entry: both are killed. The NONE control passes.

## Gates (ROOT runs the PR)

- An independent reviewer: the equality argument, the differential test's coverage, and the re-pinned work.
- Hosted CI with the full-SHA dispatch.
- DEC-025 with a fresh sweep target.
- GEN-8.

## Checkpoints

- **0: a short plan,** with no code: the design, the bound, the argument and the tests.
- **A:** the change, the tests, the re-pinned work, and the suites.
- **D:** RETURN, CHANGE_RECORD, `_run_records/` and SHA256SUMS.

**Stop and report** on any change to a published result or outcome, an edit outside the write set, or a surviving mutant.

## Host

- One cargo job at `-j 4`, `RUST_TEST_THREADS=2`.
- Your own target, `<wt>/kf1-target`.
- The memory guard running.
- No building during a timed slot ROOT has granted to K6b.
