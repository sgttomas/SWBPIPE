# I18 return: slice KF1 (K4's extreme trackers in bounded memory, every result unchanged)

## 0. At a glance

**What changed:**
- K4's `ExtremeTracker` is replaced by `BoundedExtremeTracker` at all seven sites.
  - A tracker holds at most T = 64 rows unevaluated.
  - The stop rule's trackers share a cap of G = 512 rows.
- The stop rule's tracker memory falls from a worst case of about 3.2 GB at 10,000 members to at most 2.34 MB per call.

**What did not change:**
- Every published row, class, bound, attempt role and reason, and outcome, under unlimited budgets, on every control.
- `finish` is bit-identical to K4's on every stream, refusals included (§3). The proof does not use the approximation's accuracy.

**What did change:**
- Only work, in whole exact evaluations of 17,506 LME each.
- K4's golden pins do not move. At T = 64, only the six RF-LARGE frames at 100 members move, and only in the stop rule.

**Checks:**
- The differential test, the model-level differential over every control, K4's and FK's full suites and `gen_k4_vectors.py --check` all pass.
- All 10 mutants are killed, and the NONE control passes.

## 1. Brief, basis, delegation and paths

- **Brief:** `T3/TASK_BRIEFS/I18_KF1_IMPLEMENTATION.md`.
- **Rulings:** `T3/ROOT_RULINGS_V1.md`:
  - "K6b: A1 accepted; K4's stop-rule memory finding";
  - "KF1: spawn";
  - "KF1: rulings on I18's checkpoint-0 plan";
  - "KF1: checkpoint A accepted; the S11 site-table row authorized".
- **Plan:** `IMPLEMENTATION/KF1/PLAN_CHECKPOINT0.md` (sha256 `2129c337…`), committed by ROOT at `d0566126e`.
- **Branch:** `codex/piping-kf1-20260929` in `<wt>/kf1`.
  - The base is main `8cca91701`, PR #1055. Per ROOT's correction at spawn, its piping tree equals `ab02ee3a6`'s (K4 merged); the piping diff between them is empty.
  - The candidate is the uncommitted working tree on `d0566126e`. Its files are listed in §10.
- **Delegation:**
  - ROOT (HELP_HUMAN, the chirality-piping session) dispatched I18 directly as a Type 2 TASK. It used the host's native background-subagent mechanism (D-GOV-35), with the brief as the assignment.
  - Rulings came as in-session messages at checkpoints 0 and A.
  - I18 delegated nothing and made no Git write or index operation.
  - **Scopes and enforcement:**
    - The scope was the brief's write set, widened by ROOT's rulings to every tracker site and to the S11 site-table row.
    - The host enforced its session permission system only. I18 kept to the write set and host rules itself, and the memory guard ran throughout (no kill was logged).
- **Paths:**
  - `P/` = `projects/chirality-piping/`;
  - `FK` = `P/core/solver/frame_kernel/`;
  - `K4R` = `FK/src/structural/retained/`;
  - `K4T` = `FK/tests/retained_k4/`;
  - `T3/` = `P/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/NUMERICAL_INTEGRITY_T3/`.
  - Records use `<wt>`, `<scratch>` and `<home>`.
  - Line numbers are for the candidate's `K4R/adaptive.rs`.

## 2. The design (`K4R/adaptive.rs:537-838`)

### 2.1 Keys and the window

`approximate_ratio` and `WINDOW_ULPS` (W = 2^13) are unchanged:
- A row's **key** is the bit pattern of its 64-bit approximate ratio.
- **`in_window(k, b)`** is `k.abs_diff(b) <= W` (`:567`). It is K4's predicate, now shared by every list the tracker keeps.
- **"Nearer"** means a larger key for Up and a smaller key for Down.

### 2.2 `BoundedExtremeTracker` (`:618`)

**State:**
- `direction`;
- `limit`, which is T;
- `best`, the extreme key so far;
- `offered`, each row's place in the stream;
- `lazy: Vec<(num, den, key, place)>`, the unevaluated rows, at most T;
- `table: Vec<(key, Evaluated)>`, where `Evaluated` is `Ratio(f64)` or `Refused { seq, stop }` (`:574`).

**`offer(ctx64, ctx16, num, den)`** (`:645`):
1. The key comes from `approximate_ratio`, called once per row as in K4, so ctx64's work and errors are unchanged.
2. If the key is better than `best`:
   - `best` moves;
   - `lazy` and `table` both drop every key outside the window;
   - an emptied `lazy` releases its memory.
3. If the key is in the window:
   - if `lazy` already holds T rows, the tracker **collapses** first;
   - then the row is pushed.

**`collapse(ctx16)`** (`:679`):
- It takes `lazy` and releases its memory.
- Each row is evaluated with `directed_ratio` on the caller's `ctx16`.
  - `Ok(v)` becomes `Ratio(v)`.
  - `Err(stop)` becomes `Refused { seq, stop }`. The refusal is recorded, not returned.
- Then it **prunes** (`:698`):
  1. Sort nearest key first; within a key, the most decisive outcome first (§3.2).
  2. Keep an entry only if it beats every entry kept before it.
  3. Shrink the table to fit.

**`finish(ctx16)`** (`:728`):
1. If a `Refused` entry survives, return the stop with the smallest `seq`.
2. Otherwise evaluate `lazy` in order, returning the first `Err`, as K4 did.
3. Otherwise return the directed extreme of the table's values and the lazy values. `None` means nothing was offered.

### 2.3 `TrackerSet<K>` (`:766`)

It holds a call's trackers by key and caps their unevaluated rows together at G:
- The rows are counted by `lazy`'s **allocated capacity**, the actual memory, as a running `held`.
- `holding` is the set of trackers with nonzero capacity.
- When an offer takes `held` above G, every tracker in `holding` collapses and `held` returns to 0. Finding them costs O(G), not O(number of trackers).
- `into_trackers` yields the trackers in key order.

G = 8T = 512, so the eight trackers the stop rule keeps for one body can never reach it.

### 2.4 The seven sites

| Site | Line | Tracker | Its work lands in |
|---|---|---|---|
| Pivot margin (`build_shared`) | 1241 | one, Down | the shared condition stage |
| Residual gate (`residual_rows`, called by `solve_case_at`) | 1618, 1310 | one per refinement evaluation, Up | refinement |
| Fallback (`bounded_fallback`) | 1480 | one per evaluated state, at most 4, Up | the bounded gate |
| Stop rule (`rule`): (a), (b), (d) | 1981–2125 | one `TrackerSet` keyed by (`RuleTest`, body, kind) | the stop rule |

- **`residual_rows`** gains a `ctx16` parameter, which its caller already owned.
- **`RuleTest` (`:1888`)** is an enum ordered (a), (b), (d). In key order the set finishes (a)'s trackers, then (b)'s, then (d)'s, each by (body, kind), exactly as K4 finished its three maps. The first `Err` and each summary's order are therefore K4's.
- **The old `ExtremeTracker` is removed.** No caller remains.

### 2.5 The test hook

- `tracker_hook` (`:3418`) is a thread-local override of T, and it exists only under `#[cfg(test)]`. G follows as 8T.
- The non-test build has its own `tracker_rows()` (`:552`), which returns the constant `TRACKER_ROWS`. This is ROOT's decision 4.

## 3. The invariant proof: `finish` is bit-identical on every stream

### 3.1 Notation, K4's kept set and K4's result

**Notation:**
- The stream is e_1…e_n, and a_i is e_i's key. Both trackers compute keys identically.
- b_i is the best key after offer i. It moves only toward the extreme.
- For Up, every a_j ≤ b_i for j ≤ i. Down is the mirror image.

**L1 (K4's kept set).** After offer i, K4's `kept` is exactly K_i = [e_j : j ≤ i, in_window(a_j, b_i)], in offer order.

Proof, by induction on i:
- For Up, b_i − a_j only grows with i, so a row that leaves the window never re-enters.
- K4's `retain` runs exactly when b changes, and removes exactly the rows that left.
- The push test uses the updated b.

**L2 (K4's result).** `finish_K4` = F(K_n):
- if some row of K_n refuses, F is `Err` of the earliest such row in offer order, because K4's `finish` walks `kept` in order and stops at the first error;
- otherwise F is `Ok(ext dir(e))`;
- F is `None` for an empty set, which happens only when nothing was offered.

**What KF1 relies on, and what it does not.** K4's own reason for its drops is the approximation's accuracy: a row more than W ulps below the best approximation has the smaller exact ratio. KF1 does not rely on this. It reproduces K4's K_n key for key, so it returns what K4 returns on any stream.

### 3.2 An order on outcomes

Define o(e) = `Ratio(dir(e))` or `Refused(seq(e))`, and let ⪰ be the total preorder implemented by `Evaluated::beats`:
- any `Refused` beats any `Ratio`;
- `Refused(s)` beats `Refused(s′)` iff s < s′;
- `Ratio(x)` beats `Ratio(y)` iff x > y for Up (x < y for Down).

Then F(S) is the ⪰-maximum of o over S. Consequently:
- the maximum over a set equals the maximum over any partition of it;
- if every element of S is ⪯ some element of S′ ⊆ S, then F(S) = F(S′).

### 3.3 The invariant

After every offer i, with Λ the unevaluated rows and Σ the table:
- **(J1)** Λ is the rows of K_i offered since the last collapse, in order, and |Λ| ≤ T.
- **(J2)** Every table entry (k, o) equals (a_e, o(e)) for some e ∈ K_i \ Λ. Entries have distinct keys.
- **(J3)** Every e ∈ K_i \ Λ is *dominated*: some (k, o) ∈ Σ has k nearer than or equal to a_e, and o ⪰ o(e).

Each step preserves the invariant:
- **The window.** It removes from Λ and Σ exactly the keys K4 removes, using the same predicate, so J1 and J2 hold.
  - For J3: if e is still in the window, its dominator's key lies between a_e and b (for Up, a_e ≤ k ≤ b), so the dominator is in the window too.
  - **A dominator therefore outlasts every row it stands for.** No row that K4 still keeps can depend on an entry the window has already removed. This is why a collapse is safe against later `retain`s.
- **A push** adds to Λ (J1).
- **A collapse and its prune.**
  - The evaluated rows move from Λ to Σ, which gives J2, and each row dominates itself, which gives J3.
  - The prune removes an entry only if a kept entry with a nearer-or-equal key and a ⪰ outcome exists.
  - Domination is transitive, so J3 survives the prune.
  - Within a key the most decisive entry comes first, so no two kept entries share a key (J2).
- **Collapse schedule.** A collapse may happen at any time, whether because T was reached, because G was exceeded, or because of the test hook. Correctness does not depend on T, G or when collapses happen. The tests use T = 1, 2, 3, 5 and 64, and G from 4 to 512.
- **Released memory.** Releasing an emptied Vec changes no row.

### 3.4 `finish`

By J1–J3, max⪰(Σ ∪ o(Λ)) = F(K_n):
- by J2, every element on the left is realized in K_n, so the left side is ⪯ F(K_n);
- by J3, every row of K_n is ⪯ some element on the left, so the left side is ⪰ F(K_n).

`finish` computes this maximum:
- A surviving table refusal has a smaller `seq` than every lazy row, because lazy rows come after the last collapse. If the table has none, the first lazy refusal in order has the smallest `seq`.

The two results are the same bits:
- `directed_ratio` never returns NaN or −0.0: it returns +0.0 for a zero numerator, and otherwise the absolute value, the next value up or down, or +∞. So equal `Ratio` values have equal bits, and the max/min fold does not depend on order.
- Equal `seq` means the same row. `directed_ratio` is deterministic on its operands, and the context affects only the work counters.
- `None` occurs in both exactly when nothing was offered.

### 3.5 Everything outside the tracker

- `offer` still returns only `approximate_ratio`'s errors, at the same places.
- Every early return, rejection, `first_failure`, summary, floor and `?` path at the seven sites is unchanged.
- No site touches its `sum` at a collapse, so the K4 sum work (`k4_work`) is unchanged. Only ctx16's counts grow.

§5's model-level differential confirms all of this on every control.

## 4. The memory bound, per site and per call

**Sizes, measured by `kf1_the_bound_in_bytes`:**
- `ExactWideSum` 2,144 B;
- an unevaluated row 4,304 B;
- a table entry 40 B;
- a tracker without its rows 88 B.

**Per site:**

| Site | Unevaluated rows (allocated) | Tables |
|---|---|---|
| Stop rule (`rule`, one call) | ≤ G = 512 rows (2.20 MB) after every offer; ≤ 544 rows (2.34 MB) during one offer, before the cap acts | one per tracker, at most 8 trackers per body (4 kinds in (a), force and moment in (b) and (d)) |
| Pivot margin (one per factor) | ≤ T = 64 rows (275 KB) | one |
| Residual gate (one per refinement evaluation, dropped after it) | ≤ 64 rows (275 KB) | one |
| Fallback (up to 4 states held until the best is chosen) | ≤ 256 rows (1.10 MB) | up to four |

- The transient in the stop rule: within one offer, a tracker's capacity may double from 32 to 64 before the cap check, which adds at most 32 rows.
- During the fallback of a refinement iteration, that iteration's residual tracker is also alive, so a solve attempt peaks at about 1.38 MB.

**A table, unconditionally:** at most W + 1 = 8,193 entries (327,720 B). Its keys are distinct and lie in the window.

**A table in practice** (a practical bound only; it is not used in §3's proof):
- It holds at most **one value entry**, plus one entry per refusing row in its window.
- **The lemma behind this:**
  - The approximation is the binary64 rounding of a 64-bit quotient of the 64-bit roundings of num and den. Its relative error is at most about 3·2^-64, well below 2^-54.
  - So two keys can order two rows opposite to their exact ratios only when those ratios straddle a binary64 rounding midpoint within that error.
  - A directed rounding gives both rows the same value there. So a nearer key never carries a less extreme directed value, and the prune leaves one value entry.
- The differential test asserts at most one value entry after every row of every stream. No refusal arises on any control.

**Per call, stop rule:** ≤ 2.34 MB of unevaluated rows, plus about 0.2 KB per tracker (8 per body) in practice. Today's worst case is about 3.2 GB (6.4 GB with `Vec` slack) at 10,000 members.

**Never more than K4, at any moment.** The unevaluated rows are a subset of K4's kept rows, and each table entry stands for a distinct kept row, at 40 B instead of 4,304 B. The test asserts |Λ| + |Σ| ≤ |K_i|.

**For K6b's E_max: the fallback's row list (not a tracker).**
- `bounded_fallback` keeps, for the state it is evaluating, every nonzero row's (num, den, approx) in its own `rows` list: 4,304 B per row.
- That is n_f × 4.3 KB, about 0.26 GB at 60,000 free DOFs, while the fallback runs. One state is held at a time.
- It is proportional to the model, not to the data. It is outside KF1's scope. ROOT's ruling at A: I16 includes it.

## 5. Work: the change and its derivation

**Cost of one exact evaluation.** One `directed_ratio` on a nonzero numerator charges ctx16:
- 2 roundings at 2·16 = 32 LME each;
- 1 division at (16 + 1)·(64·16 + 2) = 17,442 LME;
- **17,506 LME in total** (`kf1_one_evaluation_costs_17506`).

A zero numerator costs 0, because of the early return. Pruning evaluates nothing.

**The change.**
- Each row is evaluated at most once, either at a collapse or at `finish`. So the change is exactly 17,506 LME × (nonzero rows evaluated at a collapse that K4's `finish` does not evaluate). Those rows are:
  - rows the window later drops;
  - rows in trackers that K4 drops unfinished: the stop rule's early rejections in (a) to (d), a refinement iteration whose gate does not pass, and a fallback state that is not chosen or does not pass;
  - rows after K4's first refusal in a tracker.
- **Worst case: +17,506 LME per offered row** with a nonzero numerator, whatever T is. Rows that K4's `finish` evaluates cost the same in both.

**Where it is charged:** at each site, to the context and stage that already carried that site's `finish` (§2.4):
- the stop rule's ctx16, through `StopDecision.work` and `total`, to the candidate's `stop_rule_work` and `stages.stop_rule`, the case budget and the invocation meter;
- `solve_case_at`'s ctx16, to `stages.refinement` and `stages.bounded_gate`;
- `build_shared`'s ctx16, to `shared_stages.condition` and `shared_work`.

In `AttemptWork`, only the width-16 counts move: +2n roundings and +n divisions.

**Budget checks.** Every existing check reads a total that includes ctx16, so it sees the collapse work when it is spent:
- the stop rule's check every 64 rows in (a) and (d), and its final check;
- the guard after `residual_rows` and after `bounded_fallback`;
- the case-room test after the shared build.

(b) has no periodic check, as in K4. No check point was added or moved.

**The budget boundary** (accepted by ROOT at checkpoint 0, decision 3). The work grows and is charged earlier, so:
- a case whose limit lies between K4's work and KF1's now ends on budget;
- a check in (a) or (d) can fire before a later rejection.

`a_case_limit_is_exhausted_exactly_where_its_work_ends` derives its limit from the run, so it holds unchanged.

**Measured.**
- **Golden pins:** `golden_work_counts` and `A3B_WORK` do not move. No tracker of the four golden models reaches 64 rows; the largest, SKEW6-K1E-12, has at most about 45 per tracker.
- **At T = 64** (the model-level differential), only the six RF-LARGE frames at 100 members move, and only in the stop rule:

| Frame | + evaluations | + LME | Stop rule | Case total |
|---|---|---|---|---|
| CHAIN-n00100-AX | 768 | 13,444,608 | 8,625,285 → 22,069,893 (+156%) | 89,040,372 (+15.1%) |
| CHAIN-n00100-ROT | 768 | 13,444,608 | 8,450,524 (+159%) | 110,522,408 (+12.2%) |
| TREE-n00100-AX | 128 | 2,240,768 | 10,723,845 (+21%) | 91,059,023 (+2.5%) |
| TREE-n00100-ROT | 192 | 3,361,152 | 8,424,526 (+40%) | 107,353,917 (+3.1%) |
| CONT-n00100-AX | 640 | 11,203,840 | 13,830,960 → 25,034,800 (+81%) | 69,294,406 (+16.2%) |
| CONT-n00100-ROT | 640 | 11,203,840 | 14,727,834 (+76%) | 92,115,467 (+12.2%) |

- "Case total" is every attempt's own, K4-sum, shared and verification-shared work, which is the case limit's charge.
- **A correction to ROOT's A ruling:** it records this as "at most about +15% of the stop rule's work (about 2% of the call)". The measured figures are +21% to +159% of the stop rule's work, and +2.5% to +16.2% of the case's work.
- **The cause:** rows that tie within the window are collapsed, and then a best more than W away drops them.
- **The T trade-off** (a temporary probe, not committed; `_run_records/a/t_tradeoff_probe.txt`):
  - at 100 members the extra is 768/768/640/640/192/128 evaluations at T = 64, and 0 at T = 512;
  - the 10-member frames and HH-FOOL-m100 add nothing at T ≥ 64.
  - ROOT kept T = 64 and declined putting the approximation lemma into the correctness basis, which would be needed to cut this further.
- **The new pin** (`kf1_golden_stop_rule_work_where_collapses_occur`), per attempt:
  - (p, stop rule, refinement, bounded gate, condition) for CHAIN-n00100-AX and CONT-n00100-AX, at T = ∞ and at T = 64;
  - the T = ∞ figures equal K4's, measured on main `8cca91701`'s own code (`_run_records/a/base_probe.txt`);
  - the T = 64 figures add 768 × 17,506 and 640 × 17,506 to the stop rule only.
- **At T = 1** (test only), the refinement and condition stages also move on some controls, so those sites' collapse work is exercised.
  - The bounded-gate stage never moved: on no control did a fallback tracker hold two rows within the window.
  - That call site is covered by result equality and by the unit-level tests.

## 6. GEN

- **GEN does not model the tracker.**
  - `decide_em` (`K4T/gen_k4_vectors.py:4369`) decides (a) to (d) exactly with Fractions and returns only the rejection. It emits no summary ratio, window or work.
  - The 64-bit ratio in `emulate` is the residual gate's early stop, not the tracker.
- GEN and its vectors are unchanged. `gen_k4_vectors.py --check` gives 23 of 23 OK, SHA256SUMS included (15 min 18 s; `_run_records/a/gen_check.log`).

## 7. Tests (`K4T/kf1_tracker_tests.rs`, new; mounted in `adaptive.rs`)

- **`kf1_one_evaluation_costs_17506`**
  - One evaluation costs 17,506 LME.
  - The constructions behave as intended:
    - a refusing row (den = 1 + 2^-8100, c with an odd significand) gets c's key and refuses with `Span` in both directions;
    - a nudged row, v·(1 ± 2^-100), keeps v's key and moves its directed value by one step;
    - an overflowing row gets +∞'s key.
- **`kf1_bounded_tracker_equals_k4s_on_every_stream`: the differential test.**
  - **Reference:** `ReferenceTracker`, a verbatim copy of K4's tracker (`adaptive.rs:542-596` at `8cca91701`).
  - **Runs:** 828, over T = 1, 2, 3, 5 and 64, Up and Down, and nine families of seeded streams:
    - random ratios over a wide range, with zeros and refusals;
    - rows clustered within ±2^k ulps of a centre, for k ∈ {0, 3, 12, 13, 14, 20};
    - all ties, of length 1, T−1, T, T+1 and 3T+2;
    - the window edge: rows exactly W and W ± 1 from the best, refusals among them, and the best moving by 1 or 2;
    - many ties, then a new extreme exactly W, W + 1 or far beyond;
    - zeros, all zero or among subnormal ratios near 0;
    - the top of the range: overflow, `f64::MAX` and below;
    - refusals: early and then dropped, surviving, or two of them;
    - a monotone run toward the extreme.
  - **Asserted after every row:**
    - at most T unevaluated rows (the memory bound);
    - at most W + 1 entries;
    - every key in the window;
    - the unevaluated rows equal to the tail of K4's kept rows (J1);
    - |Λ| + |Σ| ≤ |K_i|;
    - at most one value entry.
  - **Asserted at the end:**
    - `finish` gives an identical `Result<Option<bits>, AttemptStop>`;
    - for streams without refusals, ctx16's work equals 17,506 × the evaluations counted by an independent replay that uses keys only, and is at most 17,506 × the nonzero rows.
  - **Coverage** (asserted as minimums): 2,229 collapses; 153 runs where collapsed rows were later dropped; 121 runs where both trackers refused; 81 runs where a refusal was collapsed and then dropped; 38 all-tie streams longer than T. The largest table held 6 entries; at most one of them can be a value entry (asserted), so at least five were refusal records.
- **`kf1_the_shared_cap_bounds_a_calls_trackers_together`**
  - 60 rounds, with 1 to 60 trackers, T at 1, 2, 3, 5 and 64, G at 4, 8, 24 and 512, and interleaved rows from all nine families.
  - After every row: `held` ≤ G, `held` equals the sum of capacities, the rows held are at most G, and each tracker holds at most T.
  - Each tracker's `finish` equals its own K4 reference. There were 4,095 collapses of every holding tracker.
- **`kf1_every_control_is_unchanged_but_its_work_at_t_1_and_64`** and **`kf1_rf_large_at_100_members_is_unchanged_but_its_work_at_t_1_and_64`**
  - Coverage: 131 controls (every model outside the 100-member lane), the 6 frames at 100 members, and GEN's 4 combinations with their operands.
  - Each is solved at T = ∞ (the bounded tracker never collapses, which is K4's behaviour), at T = 1 and at T = 64, under unlimited budgets.
  - **Identical:** the selection, the publication, the states, the evidence (summaries, margins, residual worst and the rest) and every attempt record apart from its work.
  - **The work,** checked exactly:
    - `k4_work` and the width-4 and width-8 counts are identical;
    - the width-16 counts differ only in roundings and divisions, by 2n and n;
    - the own LME differs by 17,506·n, split among the stop rule, refinement and bounded gate;
    - the shared delta equals the condition stage's;
    - every other stage is identical, and every delta is a whole number of evaluations.
- **`kf1_golden_stop_rule_work_where_collapses_occur`:** the new pin (§5).
- **`kf1_the_bound_in_bytes`:** the sizes in §4.
- **The adapted helper:** `K4T/adaptive_tests.rs`'s `gate_ratios` now uses the bounded tracker and passes `ctx16` to `residual_rows`.
- **Suites.** Both runs used rustc 1.97.1, offline and locked, `-j 4`, `RUST_TEST_THREADS=2`, and my own target.
  - **FK's full suite at D:** 401 passed, 0 failed (`_run_records/d/fk_full.log`).
    - The lib has 335 tests, 7 of them new; it ran in 691 s of debug time at 2 threads, with load around 6–7.
    - It includes K4's suite, `golden_work_counts` and the controls token-equal to GEN.
  - **At checkpoint A:** 400 passed and 1 failed, the S11 site table, before the authorized row was added (`_run_records/a/fk_full.log`).
  - **The S11 site table** (`FK/tests/s11_site_table.rs`): ROOT authorized one declared, additive row at A, for `adaptive.rs` `offer` with 2 integer accumulations (the row's place in the stream and the set's held capacity). No other row changes. 3 of 3 pass (`_run_records/d/s11_site_table.log`).
- **rustfmt, clippy and warnings:**
  - the changed files are rustfmt-clean, and the non-test build has no warnings;
  - clippy reports nothing in the new code;
  - two `unnecessary_map_or` findings in it were fixed with `is_none_or`;
  - clippy's other findings in the crate predate KF1.

## 8. Mutations

**Method:**
- One clean copy of the candidate's FK tree and one fresh target per mutant, deleted afterwards.
- Serialized: one cargo job at `-j 4`, `RUST_TEST_THREADS=2`.
- Exact string edits with their match counts checked. NONE ran first.
- Kill means a failing test of `cargo test --lib kf1_`.
- The runner is `_run_records/a/mutants.py`, and the results are in `mutants.jsonl`.

| Mutant | Edit | Result | Killed by |
|---|---|---|---|
| NONE | none | passes | — |
| KF1-M1 (brief) | the prune ranks ratios in the wrong direction (keeps the least extreme) | killed | differential; model-level; shared cap |
| KF1-M2 (brief) | the collapse drops the evaluated entries | killed | differential; model-level; shared cap |
| KF1-M3 | the collapse evaluates with the opposite rounding | killed | differential; model-level; shared cap |
| KF1-M4 | a new best does not prune the table | killed | differential; shared cap |
| KF1-M5 | dominance by the farther key | killed | differential; shared cap |
| KF1-M6 | a refusal at a collapse is swallowed (recorded as +∞) | killed | differential; shared cap |
| KF1-M6b | a refusal at a collapse survives the window | killed | differential; shared cap |
| KF1-M7 | no collapse | killed | differential (the memory bound); shared cap; the new pin |
| KF1-M8 | the shared cap is ignored | killed | shared cap |
| KF1-M9 | the table's window uses `<` instead of `<=` | killed | differential; shared cap |

- M4, M5 and M9 change nothing on real ratios: a row W ulps from the best never carries the extreme. The refusal streams kill them, because a refusal outranks every value.
- The model-level differential kills M1–M3 at T = 1, so the controls themselves exercise collapses.

## 9. Results at D

**Candidate:** the tree in §10, with the authorized site-table row added.
- **FK's full suite:** 401 passed and 0 failed. The non-test build has 0 warnings (`_run_records/d/fk_full.log`).
- **The site table:** 3 of 3 (`d/s11_site_table.log`).
- **The KF1 tests with their output:** 7 of 7 in 52 s (`d/kf1_tests.log`). The first run at A, before the pin and the size test were added, is `a/kf1_tests_first.log`.
- **Toolchain and host:** `_run_records/toolchain.txt`.
  - The coverage line and the sizes line are the ones given in §4 and §7.
  - 135 "moved" lines were printed: 129 at T = 1 and 6 at T = 64, the six frames in §5.
- **`gen_k4_vectors.py --check`** (run at A; GEN and its vectors are unchanged since): 23 of 23 OK.
- **Mutations** (run at A on the same code, without the site-table row): NONE passes and 10 of 10 are killed (§8).

## 10. Files

The sha256 of each file is in the checkpoint's status message, and ROOT's commit binds the bytes.

**Changed:**
- `K4R/adaptive.rs`: +347 −62. The bounded tracker, the tracker set, the seven sites, `RuleTest` and the cfg(test) hook.
- `K4T/adaptive_tests.rs`: +3 −1. The `gate_ratios` helper.
- `FK/tests/s11_site_table.rs`: +2. The authorized row.

**New:** `K4T/kf1_tracker_tests.rs` (964 lines).

**Records:** `T3/IMPLEMENTATION/KF1/`: `RETURN.md`, `CHANGE_RECORD.md`, `_run_records/` and `SHA256SUMS`. `PLAN_CHECKPOINT0.md` was committed at `d0566126e`.

**Kernel only.**
- On this base `retained` is private (`mod retained;` in `FK/src/structural.rs`), and there is no `retained_api`.
- The change adds nothing `pub` outside `pub(crate)`, and no file outside `K4R`, `K4T` and the site table changes.
- So T9 and the both-entry gate are not run, and the reviewer re-runs the scan.

## 11. What was not done, and limits

- **Mac only.** No Linux or Windows run.
- **No timing claims.** Suite times are context only. Performance is K6b's.
- **The approximation lemma** (§4) is a practical bound only. It is argued, not machine-checked, and the proof of equality does not use it.
- **The bounded-gate stage** never moved at the model level (§5). Its site is covered by result equality.
- **Not measured:** memory, K6b's or I16's E_max, and W1-T4. I16 recomputes E_max from this code, and K6b re-measures after merge.
