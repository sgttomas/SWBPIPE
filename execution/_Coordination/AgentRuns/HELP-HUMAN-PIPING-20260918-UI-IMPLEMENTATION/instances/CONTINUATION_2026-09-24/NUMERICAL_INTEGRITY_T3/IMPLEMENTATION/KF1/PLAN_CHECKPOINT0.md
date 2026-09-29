# KF1 checkpoint 0: plan (I18)

Status: a plan only. No code has been written and nothing has been built or run. K6b holds a timed slot.

## 0. Basis

- **Brief:** `T3/TASK_BRIEFS/I18_KF1_IMPLEMENTATION.md`. **Rulings:** `T3/ROOT_RULINGS_V1.md`, sections "K6b: A1 accepted; K4's stop-rule memory finding" and "KF1: spawn".
- **Also read:** Root `AGENTS.md`, `agents/AGENT_TASK.md`, `_COMMON.md`, `I8R_K1_RESUME.md:24-50` (the Mac host rules), and K4's `RETURN.md` §14 and §17.
- **Branch:** `codex/piping-kf1-20260929` in `<wt>/kf1`, at `8cca91701`.
  - Per ROOT's correction at spawn, this is not `ab02ee3a6`. I checked that `git diff ab02ee3a6 8cca91701 -- projects/chirality-piping` is empty.
- **Source hashes (sha256):**
  - `K4R/adaptive.rs` `4bdd774d…17a0b`;
  - `K4T/adaptive_tests.rs` `3390a0f2…2bfac`;
  - `K4T/gen_k4_vectors.py` `55c36778…3449b`.
- **Delegation:** I18 runs as a background subagent of ROOT's session and returns to ROOT. I made no Git writes.
- **Code read.** Line numbers below are for `K4R/adaptive.rs` at `8cca91701`.
  - The tracker's pieces: `directed_ratio` (450–510), `approximate_ratio` (514–535), `WINDOW_ULPS` (540) and `ExtremeTracker` (547–596).
  - Every user of the tracker:
    - the pivot margin, 999–1011;
    - `residual_rows`, 1064–1125;
    - the solve loop, 1374–1399;
    - `bounded_fallback`, 1228–1311;
    - `rule`, 1642–1884.
  - Where the stop rule is charged: 2790–2802.
  - The tests: `golden_work_counts` and `A3B_WORK` (`K4T/adaptive_tests.rs:427-640`), and the budget-boundary test (338–380).

## 1. Design

### 1.1 Scope: which trackers are bounded (a decision for ROOT)

`ExtremeTracker` has seven construction sites, not only the three maps in `decide`:
- the pivot margin (`Direction::Down`, the only Down user);
- the residual gate, one tracker per refinement iteration;
- `bounded_fallback`, one tracker per evaluated state, with up to four held until the best state is chosen;
- `rule`'s three maps (for (a), (b) and (d)).

Each of the first three can also keep every row when the rows tie. At n_f free DOFs:
- each holds about n_f × 4.3 KB;
- the fallback holds up to 4 n_f × 4.3 KB, plus its own per-state `rows` list.

At 10,000 members (about 60,000 DOFs) that is about 0.26 GB each, and about 1 GB for the fallback.

- **B (default; this is what the brief authorizes).** Add a bounded tracker and use it only for `rule`'s three maps.
  - `ExtremeTracker` and the other four sites stay byte-identical, so their stage work cannot move (brief item 3).
- **A (for ROOT to rule on).** Bound the other sites as well. This is not a plain swap:
  - `residual_rows` would need `ctx16`.
  - The residual-gate tracker of a failing iteration, and the fallback trackers of states that are not chosen, are dropped today without `finish`. Collapses would add exact work there that is not spent today.
  - So A changes the refinement, `bounded_gate` and condition work, which item 3 forbids. It needs its own ruling, either as a KF1 amendment or as a separate follow-up slice.

### 1.2 The bounded tracker (`BoundedExtremeTracker`, in `K4R/adaptive.rs` next to `ExtremeTracker`)

**State:**
- `direction`;
- `best: Option<u64>`;
- `seq: u64`, the count of rows offered;
- **lazy:** `Vec<(ExactWideSum, ExactWideSum, key: u64, seq: u64)>`, holding at most T rows and never evaluated;
- **table:** `Vec<(key: u64, Outcome)>`, where `Outcome` is `Ratio(f64)` or `Refused { seq, stop: AttemptStop }`.

**`offer(ctx64, ctx16, num, den)`:**
1. `key = approximate_ratio(ctx64, …)?.to_bits()`. This is the same call as today, once per row, so ctx64's work and errors are unchanged.
2. If the key is better than `best`: set `best`, then drop every lazy row and every table entry whose key fails `in_window(k, best) := k.abs_diff(best) <= WINDOW_ULPS`. This is one shared predicate for both lists, identical to today's.
3. If `in_window(key, best)`:
   - if lazy already holds T rows, **collapse** first;
   - then push the new row lazily.

**Collapse:** drain lazy, in order:
- Evaluate each row with `directed_ratio(ctx16, num, den, direction)`, using `rule`'s own `ctx16`.
  - `Ok(v)` becomes `(key, Ratio(v))`.
  - `Err(stop)` becomes `(key, Refused { seq, stop })`. The error is recorded, not returned.
- Then **prune** the table to its *staircase*:
  1. Order the entries by key, nearest the best first: descending for Up, ascending for Down. Within a key, the most decisive outcome comes first.
  2. Keep an entry only if its outcome is strictly more decisive than every entry already kept (§2.2's order).
- In practice this leaves one `Ratio` entry, `(top key, directed extreme)`, which is ROOT's prior "collapse to the single extreme entry". Section 2.5 explains why.

**`finish(ctx16)`:**
1. If the table holds a `Refused` entry, return the stop of the one with the smallest `seq`.
2. Otherwise evaluate the lazy rows in order. The first `Err` is returned, as today.
3. Otherwise return the extreme of the table's values and the lazy values. `None` only when nothing was offered.

**Wiring in `rule`:** the three `or_insert_with(|| ExtremeTracker::new(..))` become `BoundedExtremeTracker::new(Direction::Up)`, and the three `offer` calls also pass `&mut ctx16`. Nothing else in `rule` changes: every check, early return, sum and finish loop stays as it is.

**Test hook (cfg(test) only):**
- a constructor with an explicit limit, for the stream tests;
- a thread-local override of T, read at construction, for the model-level differential (§5, test 3);
- `held() -> (lazy, table)`.

The production build reads the constant T.

### 1.3 T and the memory bound

- **T = 64**, the example in the ruling. The limit on lazy rows is fixed and does not depend on data.
- **Entry sizes:**
  - `ExactWideSum` is 2 × 128 limbs plus a header, about 2,144 B, so a lazy row is about 4.3 KB (confirmed with `size_of` at A);
  - a table entry is about 40 B.
- **Hard bound per tracker:**
  - T × 4.3 KB ≈ 275 KB of lazy rows;
  - at most W + 1 = 8,193 staircase entries (distinct keys within the window) ≈ 330 KB;
  - so about 0.6 MB in total, and under 1 MB with `Vec` slack.
- **In practice:** 275 KB plus a few entries.
- **Never more than today, at any moment:** the lazy rows are a subset of today's kept rows, and each table entry stands for a distinct kept row.
- **`rule`:** at most 8 trackers per body (4 kinds in (a); force and moment in (b) and (d)). With one body that is under 5 MB hard (about 2.2 MB in practice), against today's worst of about 3.2 GB (6.4 GB with slack) at 10,000 members.
- **Caveat (§7 R2):** the bound is per tracker, so `rule`'s total scales with the body count B.

## 2. Equality: `finish` is bit-identical on every stream

**Notation:**
- a stream is e_1…e_n;
- a_i is e_i's key, computed identically in both trackers;
- b_i is the best key after offer i, which moves only toward the extreme;
- "nearer" means a larger key for Up and a smaller key for Down.

**L1 (today's kept set):** after offer i, today's `kept` is exactly K_i = [e_j : j ≤ i, in_window(a_j, b_i)], in offer order.

Proof, by induction on i:
- For Up, every a_j ≤ b_i, and b_i − a_j only grows with i, so a row that leaves the window never re-enters.
- `retain` runs exactly when b changes and removes exactly the rows that left.
- The push test uses the updated b.
- Down is the mirror image.

**Why today's drops are safe.** This is K4's rationale, and KF1 does not rely on it. The approximation is within about 1 ulp of the exact ratio: 64-bit roundings, then a binary64 rounding. A row more than W = 2^13 ulps below the best approximation therefore has a smaller exact ratio than the best row, so it cannot be the extreme. KF1 reproduces today's K key for key, so it returns whatever today returns, whether or not that rationale holds on a given stream.

**L2 (today's result):** `finish_today` = F(K_n), where:
- if some row of K_n refuses, F = `Err` of the earliest such row in offer order (`finish` walks `kept` in order and stops at the first error);
- otherwise F = `Ok(ext dir(e))`;
- F is `None` for an empty set, which happens only when nothing was offered.

### 2.2 An order on outcomes

Define o(e) = `Ratio(dir(e))` or `Refused(seq(e))`, and a total preorder ⪰ on outcomes:
- any `Refused` ≻ any `Ratio`;
- `Refused(s)` ≻ `Refused(s′)` iff s < s′;
- `Ratio(x)` ⪰ `Ratio(y)` iff x ≥ y for Up (x ≤ y for Down).

Then F(S) is the ⪰-maximum of o over S. Two consequences:
- the maximum over a set equals the maximum over any partition of it;
- if every element of S is ⪯ some element of S′ ⊆ S, then F(S) = F(S′).

`directed_ratio` is monotone in the exact ratio, so `dir` of the extreme equals the extreme of `dir`.

### 2.3 The invariant, and why a collapse keeps today's `retain` behaviour

After every offer i, with Λ the lazy rows and Σ the table:
- **(J1)** Λ is the rows of K_i offered since the last collapse, in order, and |Λ| ≤ T.
- **(J2)** Every table entry (k, o) equals (a_e, o(e)) for some e ∈ K_i \ Λ.
- **(J3)** Every e ∈ K_i \ Λ is *dominated*: some (k, o) ∈ Σ has k nearer than or equal to a_e, and o ⪰ o(e).

Each step preserves the invariant:
- **`retain`** removes rows from Λ and Σ by key, using today's predicate, so J1 and J2 hold.
  - For J3: if e is still in the window, its dominator's key lies between a_e and b, so the dominator is in the window too.
  - Survival is therefore monotone in "nearness". **This is the property that makes a collapse safe against later `retain`s.** A dominator always outlives the rows it replaced. So no row that today's code would still keep can depend on an entry that `retain` has already removed.
- **push:** the new row goes to Λ (J1).
- **collapse and prune:** the evaluated rows move from Λ to Σ, which gives J2, and each row dominates itself, which gives J3.
  - The prune removes an entry only if a kept entry with a nearer-or-equal key and a ⪰ outcome exists.
  - Domination is transitive, so J3 survives the prune.
- **A collapse at any moment preserves the invariant.** Correctness does not depend on T or on when collapses happen. This is what the differential test exercises with T = 1, 2, 3, 5 and 64.

### 2.4 `finish`

By J1–J3, max⪰(Σ ∪ o(Λ)) = F(K_n):
- by J2, every element on the left is realized in K_n, so the left side is ⪯ F(K_n);
- by J3, every row of K_n is ⪯ some element on the left, so the left side is ⪰ F(K_n).

`finish` computes this maximum:
- A surviving table refusal has a smaller `seq` than every lazy row, because lazy rows come after the last collapse. If the table has none, the first lazy refusal in order has the smallest `seq`.

The two results are the same bits:
- `directed_ratio` never returns NaN or −0.0: it returns +0.0 for a zero numerator, and otherwise the absolute value, the next value up or down, or +∞. So equal `Ratio` values have equal bits, and the max/min fold does not depend on order.
- Equal `seq` means the same row. `directed_ratio` is deterministic on its operands, and the context affects only the work counters, so the same row gives the same stop.
- `None` occurs in both exactly when nothing was offered.

### 2.5 The rest of `rule`, and the staircase in practice

**Nothing else in `rule` changes:**
- `offer` still returns only `approximate_ratio`'s errors, in the same places;
- every rejection, `first_failure`, summary, floor and `?` path is unchanged;
- `sum` is untouched, so `k4_work` is unchanged;
- only ctx16's counts grow.

**Why the staircase has one `Ratio` entry on real streams (an observation, not used in the proof):**
- Two keys can order two rows opposite to their exact ratios only when the ratios straddle a binary64 rounding midpoint within about 2^-63 relative.
- A directed rounding gives both rows the same value there, since no binary64 value lies between them.
- So the entry with the nearer key always has a value at least as extreme, and the prune leaves one `Ratio` entry, plus any refusals.

## 3. Work

**Cost per evaluation.** One `directed_ratio` on a nonzero numerator charges ctx16:
- 2 roundings at 2·16 = 32 LME each;
- 1 division at 17 · (64·16 + 2) = 17,442 LME;
- **17,506 LME in total.**

A zero numerator costs 0, because of the early return. For comparison, the approximation at 64 bits costs 1,306 LME.

**What changes:**
- Each row is evaluated at most once, either at a collapse or at `finish`.
- The stop rule's work rises by exactly 17,506 × (nonzero rows evaluated at a collapse that today's `finish` does not evaluate). Those are:
  - rows later dropped by `retain`;
  - rows in trackers that are never finished because (a), (b), (c) or (d) returns early (today those trackers spend 0);
  - rows after today's first refusal.
- **Worst case: +17,506 LME per offered row** with a nonzero numerator, about 14 times the approximation's cost. Rows that today's `finish` evaluates cost the same in both.

**Where it is charged:** as today (R7 item 7), with no other stage or attempt moving:
- to `rule`'s ctx16, and so to `StopDecision.work` and `total`;
- to the candidate attempt's `stop_rule_work` and `stages.stop_rule`;
- to the case budget and the invocation meter (2796–2802).

In `AttemptWork` only the width-16 counts move, by +2n rounds and +n divisions.

**Budget checks:**
- The existing checks every 64 rows in (a) and (d), and the final check, read `spent`, which includes ctx16. They now see the collapse work when it is spent.
- In (a) and (d), the extra overrun between two checks is at most (T + 64) × 17,506 ≈ 2.2 M LME.
- (b) has no periodic check today. Its collapse work is caught by (d)'s first check, by the final check or, on a rejection, by the next attempt's guards.
  - **Option for ROOT:** add the same 64-row check to (b). My default is no, so that every check point stays where it is today.
- A limit that lies between today's work and the new work now ends on budget. `a_case_limit_is_exhausted_exactly_where_its_work_ends` derives its limit from the run, so it holds unchanged.

**Golden pins:**
- I expect **no change** to `golden_work_counts` or `A3B_WORK`. The largest golden model, SKEW6-K1E-12, has 7 nodes, 6 members and no stations, so every tracker holds at most about 45 rows, below T.
  - This differs from the brief's expectation. It will be confirmed at A by test 3's division counts.
  - If any golden attempt does move, I re-pin it with Δ = 17,506 × Δ(width-16 divisions).
- **New pin:** the stop-rule work of one control where collapses do occur at T = 64 (chosen at A; HH-FOOL-m100 or RF-LARGE at 10 members), with its derivation from the T = ∞ run.

## 4. GEN

**GEN does not model the tracker.**
- `decide_em` (`K4T/gen_k4_vectors.py:4369`) decides (a) to (d) exactly with Fractions and returns only the rejection. It emits no summary ratio, window or work.
- The 64-bit approximate ratio in `emulate` (about lines 2714–2780) is the residual gate's early stop, not the tracker.

So GEN and the vectors are unchanged, and `gen_k4_vectors.py --check` must pass as it is.

## 5. Tests (in `K4T/`: new `kf1_tracker_tests.rs`, declared under cfg(test) in `adaptive.rs` like its siblings)

1. **Differential test on streams.**
   - **Reference:** `ReferenceTracker`, a verbatim test-only copy of today's `ExtremeTracker` (adaptive.rs:547–596 at `8cca91701`).
   - **Setup:** both trackers are fed the same pairs, for T ∈ {1, 2, 3, 5, 64} and for Up and Down.
   - **Asserted after every offer:**
     - lazy count ≤ T (the memory bound, asserted inside the test);
     - table size ≤ W + 1;
     - every table key is in the window.
   - **Asserted at the end:** the `finish` results are equal as `Result<Option<u64 bits>, AttemptStop>`.
   - **Streams:** seeded, with splitmix64 as in K4's stream tests.
     - random ratios over a wide exponent range;
     - rows clustered within ±2^k ulps of a centre, for k ∈ {0, 3, 12, 13, 14, 20};
     - all-equal ratios of length 1, T−1, T, T+1 and 3T+2;
     - rows with the same key but different directed values, straddling a binary64 value by 2^-100;
     - window-edge rows exactly W and W+1 from the best, then a best that moves the edge by one;
     - many ties (at least 3T, so several collapses), then a new extreme exactly W, W+1, or far above;
     - zero numerators: all zero, and mixed with subnormal ratios whose keys lie within W of 0;
     - overflow keys (+∞ bits) and ratios near `f64::MAX`;
     - **refusals.** A pair can pass `approximate_ratio` yet make `directed_ratio` refuse with `Span`. Example: den = 1 + 2^-8100 and num = 1 + 2^-52 gives c·den a span of about 8,153 bits, over the 8,128-bit limit.
       - The cases: a refusal collapsed and then dropped (both `Ok`); one that survives (both `Err`); two refusals; a refusal at exactly the window edge.
       - On real ratios, a row W ulps from the best never carries the extreme, so window and key faults do not show in values. Refusals dominate every value, which makes those faults visible.
   - **Coverage assertions:** collapses occurred; a collapsed row was later dropped; each refusal path ran. The largest staircase depth is printed.
2. **Work on streams.**
   - ctx16 LME (bounded) = ctx16 LME (reference) + 17,506 × n_extra.
   - n_extra comes from an independent replay that uses keys only: it simulates the lazy list and the collapse schedule, counts the nonzero rows evaluated at collapses, and subtracts those in K_n.
   - Also asserted: bounded LME ≤ 17,506 × (nonzero rows offered).
3. **Model-level differential.**
   - Each control is solved under T = ∞, where the bounded tracker behaves exactly like today's, and under T = 1 and T = 64, using the cfg(test) override.
   - **Controls:** the golden four, SKEW-K1E-28, REACTIONS-ONLY (rejected in (a)), controls rejected by (b) and by (d), the 5a.2 `stop_rule` entry, and RF-LARGE at 10 members.
   - **Asserted:** every published row, class, bound, outcome and piece of evidence is identical, and every attempt record is identical except `stop_rule_work`, `stages.stop_rule` and ctx16's round and division counts. Those differ by (17,506 n, 2n, n), where n is the difference in width-16 divisions and n ≥ 0.
4. **K4's full suite and FK's full suite,** with the controls token-equal to GEN, and `gen_k4_vectors.py --check`.

## 6. Mutants

Each mutant gets its own `git archive` copy and target under `<wt>/kf1-mut/`, serialized at `-j 4`, with each target deleted afterwards. NONE runs first and must pass.

| Mutant | Edit | Expected kill |
|---|---|---|
| KF1-M1 (brief) | the prune ranks `Ratio` values in the wrong direction | test 1 (same-key and tie groups) and test 3 at T = 1 |
| KF1-M2 (brief) | the collapse drops the evaluated entries | test 1 (extreme in a collapsed batch; `None` when lazy is empty) |
| KF1-M3 | the collapse evaluates with the opposite rounding | test 1 (inexact ratios) |
| KF1-M4 | `retain` skips the table | test 1 (refusal collapsed, then dropped) |
| KF1-M5 | dominance by the farther key | test 1 (refusal at the window edge) |
| KF1-M6 | a collapse refusal is returned at once | test 1 (refusal collapsed, then dropped) |
| KF1-M7 | no collapse (T = ∞) | the memory-bound assertion |

A surviving mutant is a stop.

## 7. Risks and questions for ROOT

- **R1 (scope, §1.1).** Under B, the pivot-margin, residual-gate and fallback trackers keep memory that depends on the data. The worst case at 10,000 members is about 0.26 GB, 0.26 GB and 1 GB.
  - Should they be bounded by A, as a KF1 amendment, or routed as a separate slice?
- **R2 (bodies).** The bound is per tracker, and `rule` holds up to 8 trackers per body.
  - A model with many small bodies, each with fewer than T rows per kind, never collapses, so its memory stays at today's.
  - **Option:** a cap G on lazy rows across all of `rule`'s trackers, which collapses every tracker that holds lazy rows when the cap is reached.
    - It is safe because §2.3 holds for any collapse schedule.
    - It costs about 20 lines in `rule`.
    - It is not in by default. Should it be?
- **R3 (budget boundary).** More work, charged earlier, can turn a case whose limit sits between the old and new work into a budget outcome. A check in (a) or (d) can also fire before a later rejection. This is inherent to the work change the brief accepts.
- **R4 (golden).** I expect the golden pins not to move (§3). A pinned control where collapses do occur is added.
- **R5 (test hook).** A cfg(test) thread-local override of T lives in `adaptive.rs` for test 3. The alternative is stream tests only.
- **Kernel only:** on this base, `retained` is private (`mod retained;` in `FK/src/structural.rs`), and there is no `retained_api`. No product crate can reach the change.

## 8. At checkpoint A (after K6b's slot, on ROOT's word)

**Host:**
- one cargo job, `-j 4`, `RUST_TEST_THREADS=2`;
- target `<wt>/kf1-target`;
- `RUSTUP_TOOLCHAIN=1.97.1 RUSTUP_AUTO_INSTALL=0 CARGO_INCREMENTAL=0 --offline --locked`;
- the memory guard running.

**Work:** the change, tests 1–3, the new pin (and any re-pin with its derivation), K4's and FK's suites, and `--check`.

**Write set:** `K4R/adaptive.rs` (the new type, `rule`'s three maps and offers, and the cfg(test) hooks), `K4T/` and `T3/IMPLEMENTATION/KF1/`.
