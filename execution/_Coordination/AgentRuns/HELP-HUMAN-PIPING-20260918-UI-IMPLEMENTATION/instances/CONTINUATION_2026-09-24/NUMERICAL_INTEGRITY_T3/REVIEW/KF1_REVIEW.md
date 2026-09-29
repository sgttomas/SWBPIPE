# RV20: independent review of slice KF1

- **Reviewer:** RV20 (Type 2 TASK, independent reviewer).
- **PR:** #1056, `codex/piping-kf1-20260929`.
- **Head reviewed:** `1854911d1` (`1854911d16d73a4c66b3e158e26de60febe71dc6`), base main `8cca91701`. The piping diff `ab02ee3a6..8cca91701` is empty, so the base's piping tree is K4's merged tree.
- **Date:** 2026-09-29.
- **Verdict: PASS.** No BLOCKING finding. 1 SHOULD-FIX and 5 NOTEs.

**In short.**
- **Results are unchanged (priority 1).**
  - RETURN §3's invariant proof holds. I checked each step, including the three points ROOT named: the prune, the window's `retain` after a new best, and the ordering of refusals by their place in the stream. I found no gap.
  - **I could not construct a stream where the two trackers differ.** My probe cuts every link between a row's key and its exact value: a test-only override sets each key directly. It then collapses at arbitrary points, as well as at T = 1 to 5.
    - Over 36,000 streams (306,367 rows), `BoundedExtremeTracker::finish` equals K4's verbatim `ExtremeTracker::finish` bit for bit. It also equals an oracle computed from the definitions (L1, L2). That covers 10,595 refusals and 6,539 final tables whose value entries are inverted against their keys.
    - J1, J2 and J3 hold after every row.
- **The seven sites are converted correctly.** The work lands where RETURN §5 says, and every budget check reads a total that includes it.
  - `rule`'s set is keyed by (test, body, kind) and finishes in K4's three-map order. My probe asserts that order.
- **The memory bound does not depend on the data.**
  - Unevaluated rows are capped by constants: T = 512 per tracker, and G = 4,096 of allocated capacity per stop-rule call. `held` equals the sum of the capacities after every offer, and a collapse of every holding tracker returns it to 0.
  - Tables are bounded by the model's row count, not by ties.
  - Two figures need a sharper statement for E_max (RV20-N3).
- **The work claims are right.**
  - One evaluation costs 17,506 LME, each row is evaluated at most once, and the golden pins do not move.
  - I derived the T = 64 pins independently from `rule`'s logged key streams: +768, +768, +128, +192, +640 and +640. The same replay reproduces I18's whole T trade-off table and gives 0 at T = 512.
  - No control moves at T = 512 in my run.
- **The tests are strong, with one gap.** FK's full suite passes 401 of 401 on a clean archive, and the `#[cfg(test)]` hook is absent from the non-test build (a compile probe).
  - Of my 12 mutants, 7 are killed. Three survive only because they are equivalent on every refusal that can be constructed (RV20-N1).
  - Two survive and are not equivalent: RV20-M4, the shared cap's collapse work left uncharged (**RV20-1**), and RV20-M12, the fallback's (RV20-N2, disclosed).
- **Kernel only, and the records hold.** `retained` stays private, and nothing new is `pub`. The site-table row is additive and declared. KF1's `SHA256SUMS` verifies 35 of 35 files, set-equal, with no machine path in the PR's 40 files.

## Findings

| ID | Class | Site | Evidence | Fix |
|---|---|---|---|---|
| RV20-1 | SHOULD-FIX | `FK/src/structural/retained/adaptive.rs:812-819`, the shared cap's collapse of every holding tracker. `FK/tests/retained_k4/kf1_tracker_tests.rs:551-629`, `kf1_the_shared_cap_bounds_a_calls_trackers_together`. | **The shared cap's collapse work is charged correctly, but no test checks it.**<br>RV20-M4 runs that collapse on a throwaway 16-limb context, so its evaluations are charged nowhere. It **survives all 7 KF1 tests** (`mutations/logs/RV20-M4.log`).<br>– The shared-cap test checks results, `held` and capacities, not work.<br>– At the model level the cap never acts. My replay finds 0 cap events on the six 100-member frames at every T from 16 to 512, and the model-level differential does not kill the mutant.<br>**Not equivalent.** At production T and G the cap acts whenever many bodies hold rows, which is the case ROOT's decision 2 added it for. Under the mutant the stop rule's `stop_rule_work`, `total`, case budget and invocation meter would all under-count.<br>**By reading, the code is right.** `TrackerSet::offer` receives `rule`'s `ctx16` and passes it to `t.collapse` (`:815`). That work reaches `decision.total` and then the candidate's `stop_rule_work`, `stages.stop_rule`, the budget and the meter (`:3058-3066`). | Add one assertion to the shared-cap test, after each round's finishes: `assert!(lme(&c16s) >= lme(&c16r))`.<br>**Why it is valid:** every row that K4's `finish` evaluates, up to its first refusal, is evaluated exactly once by the bounded tracker, either at a collapse or at `finish`.<br>**Tested:** it passes on the head and kills RV20-M4 in round 0 (1,032,854 < 3,413,670 LME; `mutations/fixcheck_rv20_1.log`).<br>An exact count from a key-only replay (as in `replay`) would be stronger. |
| RV20-N1 | NOTE | `adaptive.rs:737`, the earliest table refusal in `finish`; `:744-746`, table refusals before unevaluated rows; `:1888-1897`, `RuleTest`'s order. | **Three mutants survive, and each is equivalent on every refusal that can be constructed:**<br>– RV20-M1: the latest table refusal is returned;<br>– RV20-M2: unevaluated rows are evaluated before a table refusal;<br>– RV20-M5: `RuleTest` reordered to (d), (a), (b).<br>**Why they are equivalent:** every refusal the tests (and I) can construct is `AttemptStop::Span`, from `product_reaches`. A tracker's row reaches `directed_ratio` only after `approximate_ratio` has already rounded and divided the same operands, at 64 bits. So I found no way to make the 1,024-bit rounding or division refuse instead, though I have not proved there is none. Which Span is returned is therefore unobservable on these inputs, apart from the work spent before it.<br>**The code is right by reading and by §3's proof.** My probe also checks the recorded side of it: J2 matches every table refusal's `seq` and `stop` to its row, and `rv20_rule_test_order_is_k4s_map_order` asserts the set's order equals K4's three maps concatenated. | Optional: add the order test to KF1's tests; it kills RV20-M5. Record M1 and M2 as equivalent. |
| RV20-N2 | NOTE | `adaptive.rs:1521`, the fallback's offer; `:1651-1657` and `:1688`, `fallback_work` and `stages.bounded_gate`. | **RV20-M12 survives:** the fallback's collapse work, run on a throwaway context, is untested. This is the limit RETURN §5 and §11 disclose: on no control does a fallback tracker hold two rows within the window, so no collapse occurs there even at T = 1.<br>**By reading, the work is charged correctly.** `bounded_fallback` gets `solve_case_at`'s `ctx16`, and the fallback's total delta, `ctx16` included, becomes `stages.bounded_gate` and is seen by the guard at `:1656`. | Optional: a unit test that drives `bounded_fallback` on a state whose rows tie at T = 1. Or accept the limit as disclosed. |
| RV20-N3 | NOTE | RETURN §4 and addendum 1's bound table; CHANGE_RECORD, "The per-call bound". These are the figures I16 uses for E_max. | **The bound does not depend on the data, but two figures need a sharper statement.**<br>(a) **The growth transient.** The addendum gives "≤ 4,352 rows (18.7 MB) during one offer". When one tracker grows from 256 to 512 slots, its old and new buffers are both live while `Vec` reallocates. So the peak is G + T = 4,608 rows (19.8 MB). For a standalone tracker it is 1.5T = 768 rows (3.3 MB), and for a solve attempt 2,816 rows (12.1 MB), not 2,560. This holds if the allocator does not grow in place, and it matters if E_max counts the allocator's peak.<br>(b) **The tables.** The per-call figure counts tables only "in practice": one value entry each, which rests on the approximation lemma. ROOT kept that lemma out of the basis. The unconditional bound is ≤ 40 B per row that the call keeps in a window: \|Λ\| + \|Σ\| ≤ \|K_i\|, asserted by the differential. So it is ≤ 40 B × the rows offered to the call, about 0.9% of K4's 4,304 B per row. It is proportional to the model, not to ties. "W + 1 entries per tracker" × the tracker count is looser still. | State both terms in RETURN §4 / the addendum, or pass them to I16 with the E_max notice. |
| RV20-N4 | NOTE | `kf1_tracker_tests.rs:818-831` (`models_differential`, `report`) and `:948`. | **"No control gains work at T = 512" is printed, not asserted.** My run shows 0 "moved" lines at T = 512 and 129 at T = 1 (`suites/kf1_tests.log`). Only the two pinned frames are asserted, at `:948`.<br>**At W1's sizes the extra work returns** once a tracker holds more than 512 rows within the window. W1-T3 measured up to 8,773 kept rows at 1,000 members. The extra is still at most +17,506 LME per row. RETURN addendum 1 says so, and K6b re-measures after the merge. | Optional: assert that nothing moves at the production T in `models_differential`. |
| RV20-N5 | NOTE | RETURN §0 ("At a glance") and §7's test names; CHANGE_RECORD "What changes" and "Results". | **The heads of both records still describe T = 64.** They give G = 512, "at most 2.34 MB per call", "each at T = ∞, 1 and 64", and the test names `…_at_t_1_and_64`. Only addendum 1, at the end, says that T = 512 shipped. A reader of the PR record's opening meets the superseded figures first.<br>Also, "seven sites" counts `residual_rows`'s parameter and the solve loop separately. There are six construction sites in production (`:1243, 1482, 1620, 1983` with `rule`'s three tests). | Add a one-line pointer at the top of each: "superseded by addendum 1: T = 512, G = 4,096". |

## 1. Result equality (priority 1)

### 1.1 The proof (RETURN §3), step by step

I read `BoundedExtremeTracker` (`adaptive.rs:537-838`) against K4's `ExtremeTracker` at `8cca91701` (`adaptive.rs:547-596`). I re-derived each step.

- **L1, K4's kept set.** Keys are u64 bit patterns, and every comparison is an integer comparison.
  - For Up, b_i is the running maximum of the keys, so b_i − a_j is non-decreasing in i. A row that leaves the window never re-enters.
  - A row is pushed only when it is in the window of the updated best.
  - So K4's `kept` after offer i is {j ≤ i : |a_j − b_i| ≤ W}, in offer order.
  - None of this uses the keys' meaning as floats. Keys are never negative or NaN in practice (`approximate_ratio`, `:514-535`, returns |q|, +0 or +∞), but the argument does not need that.
- **The order ⪰ (`Evaluated::beats`, `:584-595`).** It is a strict weak order because `directed_ratio` never returns NaN or −0.0:
  - it starts from |q|, +0 or `f64::MAX`;
  - it steps with `next_up` and `next_down`, and `next_down` returns +0 below the smallest subnormal (`:254-267`);
  - it returns +∞ at the top.
  So `sort_by`'s comparator (`:700-715`) is a total order, and it cannot panic.
- **The prune (`:700-727`).** Entries are sorted nearest the best first, and within a key the most decisive first. An entry is kept only if it beats the last kept entry.
  - The kept entries form a strictly increasing ⪰-chain, so beating the last means beating them all.
  - A dropped entry x has some kept L with a key nearer or equal and L ⪰ x (the order is total), so x is dominated. Domination is transitive, so J3 survives the prune.
  - Within a key only the first entry can be kept, so keys stay distinct (J2).
- **The window after a new best (`:662-669`).** `lazy` and `table` drop exactly the keys that K4's `retain` drops, with the same predicate (`in_window`, `:569`). The emptied `lazy` is released, which changes no row.
  - For J3: a row e that stays has a_e ≤ k ≤ b_new (Up), because its dominator's key k is nearer than a_e and no key exceeds the running maximum. So b_new − k ≤ b_new − a_e ≤ W, and **the dominator outlives every row it stands for.** This is the property that keeps a collapse safe against later `retain`s.
- **Refusals and their place in the stream.** `seq` is the tracker's count of successful offers (`:655-656`), carried in `lazy` and in `Evaluated::Refused`.
  - Refused(s) beats Refused(s′) iff s < s′, and a `Ratio` never dominates a refusal.
  - Let e\* be the earliest refusing row of K_n. If it was collapsed, J3 gives a table refusal with seq ≤ s(e\*), and J2 makes it a refusing row of K_n, hence e\* itself. It is also the minimum over the table's refusals.
  - If e\* is still unevaluated, the table holds no refusal: any table refusal would come from a row of K_n offered before the last collapse, and so before e\*. `finish` then walks `lazy` in order and stops at e\*.
  - K4 walks `kept` in order and stops at the first refusal. The two agree.
- **The same bits.** `WideContext` holds only a precision and a work counter (`wide/multi.rs:1124-1127`). Every `ctx16` here is built at 1024 (`:1155`, `:1574`, `:1914`, and the test helper). So a row gives the same value, or the same stop, whenever and wherever it is evaluated.
  - Equal values have equal bits (no −0.0), and `max`/`min` are order-independent without NaN.
  - `Ok(None)` happens in both exactly when nothing was offered: every successful offer leaves a lazy row or, after a collapse, a table entry.
- **Everything outside the tracker.** The key is computed first (`:654`), before any state change or collapse. So `approximate_ratio`'s errors and its `ctx64` work are K4's, and a failed offer leaves the tracker and `held` untouched.
  - A collapse records refusals instead of returning them, so no new error path exists.

### 1.2 Trying to break it (`probes/`)

I built a probe copy of the head. Its only additions are test-only: a `#[cfg(test)]` key override read after `approximate_ratio` in `offer`, a log of the stop rule's offers, and the probe module (`probe_adaptive.diff.txt`).

- **`rv20_arbitrary_keys_and_collapse_schedules_match_k4_and_the_oracle`.**
  - **Keys** are set at the running best's window edges: offsets 0–3, W − 2 to W + 2, 2W and 3W + 5, toward or away from the best.
  - **Values** are drawn independently of the keys: exact, inexact, one step off, zero, overflowing, and two refusals.
  - **Schedules:** T from 1 to 5 and ∞, plus explicit `collapse()` calls at random rows (0–70%).
  - **Directions:** both.
  - **After every row, against an oracle built from L1:** K4's kept set equals L1's; J1, the unevaluated rows are a suffix of it, at most T; J2, every table entry is realized by an evaluated row, with the same bits or the same seq and stop, and keys are distinct; J3, every evaluated row is dominated; and |Λ| + |Σ| ≤ |K_i|.
  - **At the end:** bounded = K4 = oracle, bit for bit; and the bounded tracker's work is at most one evaluation per row.
  - **The run:** 36,000 streams, 306,367 rows, 64,707 explicit collapses, 10,595 refusal results and 25,405 value results. 6,539 final tables held more than one value entry, up to 5, which are inversions the real approximation never produces. **All pass.**
- **`rv20_tracker_set_with_arbitrary_keys_tiny_caps`.** 400 rounds of 1 to 12 interleaved trackers, with G from 1 to 9 and T from 1 to 8.
  - After every offer, `held` ≤ G and `held` equals the sum of capacities, with each tracker at most T.
  - There were 2,969 collapses of every holding tracker, and each tracker's `finish` equals its K4 reference.
- **`rv20_rule_test_order_is_k4s_map_order`** checks that `RuleTest`'s derived order and `into_trackers` equal K4's three `BTreeMap`s, concatenated in order.

**No difference found.** I found nothing BLOCKING.

## 2. The seven sites (priority 2)

| Site | Construction and finish | Work lands in | The checks that see it |
|---|---|---|---|
| Pivot margin (`build_shared`) | Down, `:1243`, offers `:1253`, finish `:1255` | `stages.condition`: t5 includes `lme(ctx16)` (`:1256-1261`), then `shared_work` | the case-room test after the build (`:2762`) |
| Residual gate | Up, per evaluation, `:1620`; `residual_rows` now takes `ctx16` (`:1312`) and offers at `:1366`; finish `:1645` | `stages.refinement` (`:1687`) | the guard after `residual_rows` (`:1635`) |
| Fallback | Up, per state, `:1482`, offers `:1521`, finish of the chosen state `:1555` | `stages.bounded_gate` = the fallback's total delta (`:1651-1657`, `:1688`) | the guard after `bounded_fallback` (`:1656`) |
| Stop rule (a), (b), (d) | one `TrackerSet<(RuleTest, u32, Kind)>`, `:1983`; offers `:2020`, `:2057`, `:2103`; finishes `:2034` (no report) and `:2118` | `rule`'s `ctx16`, then `decision.total`, then `stop_rule_work`, `stages.stop_rule`, the budget and the meter (`:3058-3066`) | the checks every 64 rows in (a) and (d) (`:2030`, `:2113`) and the final check (`:2038`, `:2126`) |

- **Every result is the same at each site.** Each site calls `finish` exactly where K4 did, and a site's result is its tracker's `finish` result, so §1 carries over.
  - The set's key order, derived `Ord` on `RuleTest` (Disagreement < Estimate < Charge), then (body, kind), reproduces K4's order: (a)'s map, then (b)'s, then (d)'s.
  - The three summary vectors are filled per test, so their contents and order are K4's.
  - The first `Err` among the finishes is also K4's, and the no-report path finishes (a)'s trackers only, as K4 did.
- **The old tracker is removed.** No `ExtremeTracker` token is left (`records_checks.txt`). The test helper `gate_ratios` passes its own `ctx16` at precision 1024 and does not read the tracker.
- **The model-level differential compares everything except work.** I read `same_attempts`, `same_solve` and `same_outcome` (`kf1_tracker_tests.rs:677-782`).
  - It compares: the selection, the publication, the states (`Debug`), the evidence apart from its attempts, and each attempt's full `Debug` with its five work fields zeroed.
  - `k4_work`, the verification work fields, and the width-4 and width-8 counts must be equal.
  - The width-16 counts may differ only by 2n rounds and n divisions, and the LME difference by exactly 17,506 × n, split among the stop rule, refinement and bounded gate. The condition stage's delta must equal the shared delta.
  - Refused outcomes compare their full `Debug`.
  - The one field not compared is `RetainedSolve.cache`, the shared stages. The results it holds (pivot margin, rcond) appear in the attempt records, and the combinations are solved from it.
- **The kills confirm that work reaches three sites.** RV20-M3 (the residual gate's charge dropped) and RV20-M11 (the pivot margin's) are killed by "work fell" at T = 1, and M11 also by the pin's condition stage at T = 64.
  - The shared cap and the fallback are untested (RV20-1, RV20-N2).

## 3. The memory bound (priority 3)

- **Unevaluated rows, per tracker.**
  - `offer` collapses before a push once `lazy` holds T rows (`:671-674`). So `lazy.len()` ≤ T after every offer, whatever the data.
  - For a 4,304 B element, `Vec`'s capacity grows 1, 2, 4, …, so it never exceeds T when T is a power of two, as 512 is.
  - My probe and I18's differential assert len ≤ T after every row. RV20-M6 (collapse only above T) is killed.
- **The shared cap.** `held` is updated as held − before + after around each tracker's offer (`:803-806`), and `holding` tracks exactly the trackers with nonzero capacity.
  - A collapse leaves capacity 0 (`mem::take`), so collapsing every holding tracker makes `held` = 0 ≤ G.
  - The accounting is exact: the tests assert `held` = Σ capacity after every offer, and RV20-M7 (counting rows) and RV20-M8 (collapsing only the offered tracker) are killed.
  - One body's eight trackers (4 kinds in (a), force and moment in (b) and (d)) hold at most 8T = G. So the cap acts only across bodies, as intended. A body is a connected component (`source.rs:544-566`), so the RF-LARGE frames are one body each.
- **Independent of the data.** The only data-dependent term left is the tables. They are bounded by the rows the call keeps in its windows, which the model fixes, not the ties. The two refinements for E_max are RV20-N3.
- **Sizes.** My run prints ExactWideSum 2,144 B, a row 4,304 B, an entry 40 B and a tracker 88 B. Both halves of the row are inline arrays (`wide_sum.rs:108-124`), and `AttemptStop` holds no heap data (`adaptive.rs:126-161`), so `size_of` is the whole cost.
- **Per call, at T = 512.**
  - The stop rule holds ≤ 4,096 rows (17.6 MB) after every offer, with a transient of ≤ 4,608 rows (RV20-N3).
  - The pivot margin holds ≤ 512 rows, one residual-gate evaluation ≤ 512, and the fallback ≤ 2,048 (4 states, since `corrections` ≤ 3).
  - The fallback's per-state row list is proportional to the model, as ROOT ruled.

## 4. Work (priority 4)

- **17,506 LME per evaluation.** `limb_multiply_cost` (`wide/multi.rs:987-998`) gives 2·16 = 32 per round and (16 + 1)(64·16 + 2) = 17,442 for the division. `directed_ratio` charges two rounds and one division before its correction loops, which use `ExactWideSum` and do not charge the context.
  - A zero numerator returns before any charge.
  - A refusing row is charged in full before `Span`.
  - `kf1_one_evaluation_costs_17506` passes.
- **At most one extra evaluation per row.** A collapse moves rows out of `lazy` for good, and `finish` evaluates only `lazy`, so each row is evaluated at most once. K4 evaluates a subset of those rows.
  - My probe asserts at most one evaluation per row on all 36,000 streams, and I18's stream test asserts it on 882.
- **Golden work is unchanged.** `golden_work_counts` and `A3B_WORK` pass in the full suite, and the only edit to `adaptive_tests.rs` is `gate_ratios`'s `ctx16`.
- **The T = 64 pin, derived independently.** My probe logs each `rule` call's offers: tracker, key and nonzero numerator, at T = ∞.
  - `replay_call` then simulates the window, T, and G's capacity accounting from the keys alone, and counts the rows evaluated at a collapse that K4's `finish` would not evaluate.
  - At T = 64 it gives +768, +768, +128, +192, +640 and +640 for the six frames (CHAIN, TREE, CONT; AX and ROT). Each equals the stop-rule delta measured by solving at T = 64 and T = ∞, and the pin's 768 and 640.
  - Across T = 16 to 512 it reproduces I18's trade-off table exactly, and it gives **0 at T = 512**, with 0 cap events.
  - **The cause, as RETURN says:** rows that tie in the window are collapsed, and a later best more than W away drops them.
  - The pin's T = ∞ figures equal K4's own figures on main (I18's `base_probe.txt`). The pin also asserts that T = 512 leaves both frames at those figures.
- **No control gains work at T = 512.** In my run of the model-level differential, 0 "moved" lines are at T = 512 and 129 at T = 1.
  - At T = 1: 129 lines move the stop rule, 7 refinement and 7 condition, and none the bounded gate.
  - The claim holds for the suite's controls (at most 103 members). RV20-N4 covers W1's sizes.
- **Budget boundary.** As accepted at checkpoint 0. `a_case_limit_is_exhausted_exactly_where_its_work_ends` passes.

## 5. The tests (priority 5)

- **The stream differential's coverage.** Its nine families cover:
  - all ties at 1, T − 1, T, T + 1 and 3T + 2;
  - the window edge at W and W ± 1, with refusals there and the best moving by 1 or 2;
  - a new extreme at W, W + 1 or far;
  - zeros and subnormals, overflow, refusals (early and dropped, surviving, or two), and monotone runs;
  - both directions, at T = 1, 2, 3, 5, 64 and 512.
  The coverage minimums are asserted. My run: 882 runs, 2,447 collapses, 152 runs where collapsed rows were later dropped, 131 where both refused, and 95 where a refusal was collapsed and then dropped.
  - **Adequate, with one limit.** Real ratios never invert keys against values, so the prune's value ordering is exercised only through ties, one-step nudges and refusals. My probe closes that with arbitrary keys (§1.2).
  - The `ratios <= 1` assertion (`:435`) checks the practical lemma, not the proof. That is sound as a check, and it would fail, without any result being wrong, only if the lemma failed.
- **The hook is absent from the non-test build.** A non-test function naming `tracker_hook::get()` fails to compile with E0433; the same function naming the non-test `tracker_rows()` compiles (`suites/hook_check.log`). `held()` and `trackers()` are `#[cfg(test)]` as well.
- **Mutants** (`mutations/mutation_table.txt`). NONE passes. RV20-M3, M6, M7, M8, M9, M10 and M11 are killed. M1, M2 and M5 are equivalent (RV20-N1). M4 (RV20-1) and M12 (RV20-N2) survive.
  - I18's ten mutants, re-read, are sound edits, and their logs show the kills.

## 6. Kernel only (priority 6)

- **Four code files change:** `adaptive.rs`, `adaptive_tests.rs`, the new `kf1_tracker_tests.rs` and `s11_site_table.rs`. Everything else is KF1's records (`revisions.txt`).
- **Visibility.** `mod retained;` stays private (`FK/src/structural.rs:5`), and there is no `retained_api`. Every added item is `pub(crate)` or private.
- **Outside FK.** No `.rs` or `.toml` file outside FK names `structural::retained`, `retained_api`, the tracker, the set or `TRACKER_ROWS`. So T9 and the both-entry gate are rightly not run.
- **The site-table row is additive and declared** (`s11_site_table.rs:286-287`). It counts `self.offered += 1` and the self-assignment fold `self.held = self.held - before + after`, both integer and both in functions named `offer`. No other row changes, and the site test passes 3 of 3.
- **GEN is unchanged.** GEN and its vectors are byte-identical to main, and GEN does not model the tracker: `decide_em` has no window or summary. I did not re-run `--check`.

## 7. Records (priority 7)

- **KF1's `SHA256SUMS`:** 35 of 35 files match the committed bytes at the head, and the set equals the folder (`records_checks.txt`).
- **Paths.** GEN-8's `MACHINE_ABS_PATH_RE` and a user-name scan find 0 hits in the PR's 40 files.
- **The reference copy** in `kf1_tracker_tests.rs` is K4's `ExtremeTracker` (`adaptive.rs:547-596` at `8cca91701`) verbatim, apart from its name and `pub(crate)`.
- **Figures.** The RETURN and CHANGE_RECORD numbers I re-ran match: 401 passed; the coverage, sizes and shared-cap lines; the work tables; the pin. RV20-N3 and RV20-N5 are the only corrections.
- **Hosted CI on the head passes,** including the numerical cargo suite (read-only `gh pr checks`, `revisions.txt`).

## Reviewer, brief and delegation

- **Reviewer.** RV20 is a Type 2 TASK, dispatched directly by ROOT (HELP_HUMAN, the SWBPIPE chirality-piping session) as a background subagent of ROOT's session. ROOT is the only return path.
  - I did not write KF1, and I delegated nothing.
- **Read:**
  - Root `AGENTS.md`, `agents/AGENT_TASK.md`, `T3/TASK_BRIEFS/_COMMON.md` and `I8R_K1_RESUME.md:24-50`;
  - `TASK_BRIEFS/I18_KF1_IMPLEMENTATION.md`, in `<wt>/numerics`;
  - `ROOT_RULINGS_V1.md` in `<wt>/numerics`: "K6b: A1 accepted; K4's stop-rule memory finding", "KF1: spawn", "KF1: rulings on I18's checkpoint-0 plan", "KF1: checkpoint A accepted; the S11 site-table row authorized" (with its correction), "KF1: D received; T reopened and set to 512", and the K6b and V-K sections between them;
  - on the KF1 branch: `PLAN_CHECKPOINT0.md`, `RETURN.md` with addendum 1, `CHANGE_RECORD.md`, and the `_run_records/` I cite (`base_probe.txt`, `t_tradeoff_probe.txt`, `t512/work_table_probe.txt`, `t512/kf1_tests.log`, both `mutants.py`, both `mutants.jsonl`);
  - the complete code diff, the whole of `kf1_tracker_tests.rs`, and the code it relies on: `directed_ratio`, `approximate_ratio`, `next_up`/`next_down`, `WideContext`, `limb_multiply_cost`, `ExactWideSum`'s layout, `AttemptStop`, `build_shared`, `residual_rows`, `bounded_fallback`, `solve_case_at`, `rule`, the stop rule's charging, the S11 scanner, and GEN's `decide_em`.
- **Ran** (records in `T3/REVIEW/_run_records/kf1_review/`; the sha256 of each file is in its `SHA256SUMS`):
  - FK's full suite on a clean `git archive` of the head, and KF1's tests with their output;
  - the hook compile probe;
  - four probe tests on a probe copy;
  - NONE and 12 mutants, and the RV20-1 fix check with NONE and M4.
  - All with rustc 1.97.1, `--offline --locked`, `-j 4`, `RUST_TEST_THREADS=2`, one cargo job at a time, and targets under `<wt>/rv20-target`. The memory guard ran throughout, and its log records no kill.
- **Git use.** Read-only only: `rev-parse`, `log`, `show`, `diff`, `merge-base`, `ls-tree` and `archive` in `<wt>/kf1`, and `status` in `<wt>/kf1` and `<wt>/numerics`. `status` may refresh the index stat cache, as earlier reviewers disclosed. I made no Git write or index operation and ran no `fetch`.
  - GitHub: read-only `gh pr view` and `gh pr checks`. No GitHub write.
  - My writes are this file and `T3/REVIEW/_run_records/kf1_review/**`, left uncommitted, plus scratch copies under `<wt>/rv20`, deleted at the end.
- **Not done:**
  - no GEN `--check` (GEN and its vectors are unchanged), no DEC-025 sweep, no GEN-8 run (my own scan instead), no T9 or both-entry gate (kernel only), and no Linux run;
  - no timing or memory measurement (K6b's).
- **Abbreviations.** `P/` = `projects/chirality-piping/`; `T3/` = the T3 folder; `FK` = `P/core/solver/frame_kernel`. Line numbers are for the head's `FK/src/structural/retained/adaptive.rs` unless a file is named.
