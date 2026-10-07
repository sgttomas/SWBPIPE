# RV109 (RV-P, B1 round 1): independent review of B1's ST slice

TASK (Type 2), RV109, role RV-P for all of B1, for ROOT (HELP_HUMAN, Agent 0), who is the return path. I made no delegation. I am a fresh instance and wrote none of this change. 2026-10-07 UTC (2026-10-06 host local time).

**Brief (verified before work):** `R/BRIEFS/RV109_RVP_ROUND1.md`, sha256 `dffadce77d1b7bbf956266a68e214408b713b722f6e16421657e50d232669419`. I read NUM's root `AGENTS.md` and `agents/AGENT_TASK.md` first.

**Basis read** (sha256 verified where the brief names one):
- PLAN_v2 §0, §1 (the fence and the guards), §2.0–§2.2 (`R/I84/b1_plan_01/PLAN_v2.md`, `c85786b7…`);
- DESIGN_v2 §1.1–§1.4 (T-1 to T-13, the outcome table), decisions 1 and 21 (`R/I78/b0_contract_01/DESIGN_v2.md`, `5933b90b…`); DN §4.3 (`T/DESIGN_NUMERICS/DESIGN.md`, `fb62ef4a…`);
- PROBE §0–§8 and its `I81_*` log lines (`R/I81/b1_probe_01/PROBE.md`, `3e32726d…`); I86's PROBE §0–§7 (`R/I86/b1_w_probe_01/PROBE.md`, `7470a726…`) and its input `b2_k1e3.json` (`89b05619…`);
- RR (read at `4e4c5ae9…`): "B0 selected on DESIGN_v2; …", "I81's B1-0 probe verified; …", "I84's B1 plan returned; …", "R1: …", "B1's PLAN_v2 accepted; RV107's A1 amendments; …", "I86's SW probe accepted; …" and "R3: I85's ST verified at checkpoint R3; RV-P round 1 dispatched as RV109";
- `R/BRIEFS/B1_COMMON.md` and `R/BRIEFS/B1_ST.md`;
- **after forming my own view:** I85's `R/I85/b1_st_01/RETURN.md` (`f4c1cadc…`).

**Placeholders:** `WT`, `NUM`, `P`, `PP` (= `P/core/product_physics/src`), `PP-tests` (= `P/core/product_physics/tests`), `RE`, `T`, `R`, `RR` and `VENV` as in the dispatch. `S` = my scratch, `WT/scratch/rv109_rvp_01/`. `BASE` and `CAND` = my archive copies `WT/rv109/base` (main `47a3bdfcf5`) and `WT/rv109/cand` (head `a8e719f5b4`). `E` = this folder's `evidence/`.

**Limits kept.**
- Every cargo job went through `WT/tools/t3_cargo.sh` (memory guard up), `--locked --offline`, `CARGO_BUILD_JOBS=4`, `RUST_TEST_THREADS=2`, `TMPDIR=S/tmp`, fresh targets `WT/targets/rv109-*`, `RUSTFLAGS`/`CARGO_ENCODED_RUSTFLAGS` unset except `RUSTFLAGS=--cfg=rv109_stale` for the Stale runs. 32 jobs (`E/cargo_jobs_rv109.log`). They interleaved with RV108's B6 jobs; I waited for the lock and killed no job.
- One wait per job: each wait loop ended on its job's result file **or** when the job's process was gone. No wait of mine is running.
- No Git writes (reads used `GIT_OPTIONAL_LOCKS=0`). No DEC-025, no evidence sweep beyond my probe, no installs. Nothing in the system temp directory.
- I85's scratch and targets were not used or touched.
- Probe code went only into my copies, under `#[cfg(test)]`. The host accepted every write into this records folder.

## 0. Verdict

**PASS** at head `a8e719f5b4` (two commits over main `47a3bdfcf5`).

| BLOCKING | SHOULD-FIX | NOTE |
|---|---|---|
| 0 | 1 | 5 |

The product change is right: T-4's classifier, decision 21 and `NoTriggeredCase` match DESIGN_v2 and PLAN_v2 §2.1, sit where the plan puts them, and change no byte outside the three expected inputs. The seam is behaviour-neutral. The one SHOULD-FIX is test-only: no test pins that T-4 runs **before** R-2's reservation (my mutant R10 survives).

## 1. Findings

| ID | Severity | Path | Evidence | Remedy |
|---|---|---|---|---|
| **SF-1** | SHOULD-FIX | PP `retained_facade_tests.rs`, `b1_t4_two_body_case_b_is_a_no_triggered_case_pin` (and the witness pins' `no_triggered_case_witness`) | **No assertion pins "no reservation" for `NoTriggeredCase`.** Mutant R10 moves T-4 after `ReservedNotice::reserve` (the slot is reserved, then dropped unpublished). It **survives the whole lib suite with the ignored witnesses** (`E/mutants/summary.md`), because every pin compares bytes only and the reserved slot is not a byte. Its two observable effects, both shown by my discriminator (`E/mutants/r10_head.filtered.log`, `r10_mutant.filtered.log`): (a) the ordinary owner returned with `NoTriggeredCase` carries a spare diagnostics slot (`spare_slot: true` on two-body B and W6-PHYS-R4, both modes); (b) when the base already carries `diagnostic:retained-precision:<case>:unavailable`, the cause becomes `NoticeReservation` instead of `NoTriggeredCase`. The head gives no spare slot and `NoTriggeredCase` in all 8 rows. PLAN_v2 §2.1 puts T-4 "before the notice reservation", T-5 reserves "for each case in A", and the brief's item 2 asks for "no reservation". SP rewrites exactly this code (T-5 per case) after I1 | Test-only, in ST's fence. In the two-body B pin's private-driver half (`observed()` already shrinks the diagnostics): after `NoTriggeredCase`, `assert_eq!(envelope.diagnostics.capacity(), envelope.diagnostics.len())`; and add the collision variant (two-body B with the notice id pre-pushed, as `u3_each_stage_fault_falls_back_to_the_ordinary_bytes` does for the milestone) asserting `NoTriggeredCase` and exact bytes. Each kills R10 (shown). ROOT may instead carry it to SP's "reservation count" mutant (PLAN_v2 §2.2), provided that pin includes W-C2's `not_required` case and counts slots, not bytes |
| N-1 | NOTE | PP `lib.rs` `case_triggers`; the classifier test | Mutant R16 (`not_required` only when a seed also exists) survives. It is equivalent on every reachable state: a Passed verdict needs an attempted solve, and an installed observer seeds every attempted case (Direct and the private driver). T-4 makes `not_required` depend on the verdict alone, and the unit test has no "Passed, no seed" row | Optional, with SF-1's repair: `assert_eq!(one(Some(V::ChecksPassed), None), NotRequired)` |
| N-2 | NOTE | PP `lib.rs` `retained_w1` (`.any(…Attempted)`) | Mutant R17 (`.all` for `.any`) survives; it is equivalent at c = 1, since there is one case. It differs only when A is a proper subset of the cases | For SP: W-C2 (A in A, B `not_required`, C in A) distinguishes it; add it to SP's mutant list |
| N-3 | NOTE | PP `retained_product.rs` `prepared_case_source` (the seam) | **For SA.** (a) R3 ruling 2's overflow path: `CountRange("late loads total")` sets `self.error`, skips G-B and the late capture, and later surfaces as a Preparation fallback **with a notice**, not as a typed G-B refusal. Unreachable (a `usize` sum of `Vec` lengths). (b) Neither seam field is pinned in ST, by design (nothing reads them). My probe, with a real `admit` permit and `permitted_probe`, read `late_loads_total` equal to the case's load count on every admitted run whose capture reached G-B (milestone 3, two-body B 2, case C 5, W6 2, W2b and `b2_k1e3` and `c1` 128; 36 rows), and 0 where the late hook returned before G-B (exact-block selection, blocked or unattempted runs; 38 rows) (`E/probe/compare.txt`, "SEAM") | SA decides the overflow's disposition and pins Σ l_i at G-B on a multi-case input, and `requested_cases` in G-C's fact (e) |
| N-4 | NOTE | PP `retained_memory_witness_tests.rs`: `witness_w2b_cap_maximal_passed_report_no_triggered_case`, `witness_w6_phys_r4_input_no_triggered_case` | Ruling 4's three `NoTriggeredCase` pins: only two-body B's runs in the default suite (and so in hosted CI's Stale branch). W2b's (an ordinary Passed report at the cap counts) and W6-PHYS-R4's (one-body W2-published Passed) are `#[ignore]` witnesses, run only explicitly. `NoTriggeredCase` does no W1 work, so they carry no stack risk; W2b's ordinary solve takes well under a second | Optional: a default-suite pin of those two inputs (private driver and Direct), keeping the witnesses for G6 |
| N-5 | NOTE | QUAL §4's S1 evidence (records; SQ) | **Ruled, tracked.** From this head until SQ pins `b2_k1e3`, no committed test runs the full native ladder at the cap-maximal counts (W2b's former role). My probe confirms `b2_k1e3` at the head: Sensitive by the report, in A, native reached, `Candidate` with one notice, both modes, on Direct and on the 4 MiB driver (§2 item 3) | SQ pins it (RR "I86's SW probe accepted…", ruling 1; R3 ruling 4) |

## 2. The review, item by item

### Item 1: T-4's classifier is right

**Checked against DESIGN_v2 T-4, decisions 1 and 21, DN §4.3 and PLAN_v2 §2.1** (PP `lib.rs` `CaseTrigger`, `case_triggers`, `only_one`, `dn_trigger_excluded`):
- **The verdict is looked up by `basis_ref.ref_id`, never by position.** `only_one(quality.cases.iter().filter(|e| e.basis_ref.ref_id == case_id))`. This is `ordinary_value`'s binding (PP `retained_wire.rs`: exactly one match on `basis_ref.ref_id`). Mutant P2 (by position) is killed.
- **The seed is looked up by case id** (`seed.case == case_id`), also unique. Mutant R6 (seed by position) is killed.
- **`not_required` exactly when the verdict is `checks_passed`,** tested first. The seed's `initial` plays no part. Mutants P1 (keyed on the seed's report code) and R11 (`not_assessed` as `not_required`) are killed.
- **The exclusion is exactly decision 21's.** `initial` is `StructuralFailure` with `Mechanism`, `Asymmetric` or `InvalidInput`, and `w2` is not `Published` (`NotTriggered` and `Failed` both count as not published). These three Rust variants are exactly the wire tags `mechanism`, `asymmetric` and `invalid_input` (PP `retained_wire.rs`'s structural tag map), and DN §4.3's "Never triggered by `Mechanism`, `Asymmetric` or `InvalidInput`". `NegativeEnergy` and `NumericallyUnresolved` stay in A, as DN §4.3 requires. Mutants P3, R8, R9, R12 and R13 are killed.
- **Everything else is in A:** `sensitive`, `unresolved`, `failed`, `not_assessed`, an absent entry, a seed with `initial` unset, and a case with no seed (ruling 2). Mutant P4 (seedless excluded) is killed, including by W2's committed witness.

**ROOT's uniqueness reading (R3 ruling 1): ACCEPTED.**
- It mirrors `ordinary_value`, which refuses a non-unique quality binding (`Association`). A non-unique case is therefore attempted and, at T-11, abandons the successor with T-12's notices. That is the fail-safe direction: no case is silently skipped, and no wrong successor can be published.
- It is unreachable on today's producer: `numerical_quality.cases[]` carries one `load_case` entry per requested case, validation's `detect_duplicate_ids` covers load-case ids, and the observer pushes one seed per attempted case. The classifier test pins it (mutant R7, first match, is killed by "two entries for one case").

### Item 2: where it runs

`retained_w1` (PP `lib.rs`): coexistence → G-B's outcome → `w1_case_id`'s D1.4 scope (`Domain`) → **T-4** → R-2's reservation → preparation and the rest. G-C runs before `retained_w1`, in `permitted_run`. With A empty it returns `(ordinary, Err(NoTriggeredCase))` with the owner untouched.
- **No other path reaches W1:** `retained_w1` has one production caller (`permitted_run`, after G-C; `u3_n9_single_parse_custody` pins the call count), and `ReservedNotice::reserve` one caller (inside `retained_w1`, after T-4). The Headless entry never gets a permit in D1.
- **Observed, not only read** (my probe, `E/probe/compare.txt`, "NO-TRIGGER CHECK"): for all six `NoTriggeredCase` rows (two-body B, W2b's input, W6-PHYS-R4; both modes):
  - Direct publishes the plain bytes exactly, with 0 notices, `ONE_RUN_THROUGH_G_C` and no hooks;
  - on the 4 MiB private driver the owner's bytes are the plain bytes, **no diagnostics slot is reserved** (capacity = length after `shrink_to_fit`), and **an armed native-stage fault stays armed** (no W1 stage ran).
- **No case in A is skipped:** every input whose verdict is not `checks_passed` and that reaches T-4 still runs W1 at the head with base's outcome and bytes (item 4's differential). Decision 21 moves no committed input; no seed in my 112 rows carries an excluded tag (seeds seen: report, `Range` with W2 published, `NumericallyUnresolved`, K2a's formation failure).
- **The gap:** the ordering of T-4 before the reservation is right in code but pinned by no test (SF-1).

### Item 3: my probe on the head, both modes (`E/probe/`)

**Method.** `zz_rv109_probe.rs`, a child of `witness_tests` in my copies (it reuses the committed witness inputs and helpers; U8's builders are verbatim copies; case C is built from DESIGN_v2 §1.4's words and matches PROBE's sha). For each input and mode it records: the plain route; one counted Direct run (admission, cause, counts, hooks, notices, published sha, equality with the plain bytes and with an independently built plain-plus-N1-notice text); the private driver at R/16 = 4 MiB (cause, the owner's bytes, a reserved-slot check, seeds and verdicts); a second driver run with a native-stage sentinel; and the seam (`admit` + `permitted_probe`). Phase follows each cause (PLAN_v2 §3.5's stages). Two-run determinism is not claimed; the control is I81's records (below).

| Input (input sha) | Mode | Verdict; seed | Direct (head) | Phase reached | Notices | Driver (4 MiB) | Against PROBE |
|---|---|---|---|---|---|---|---|
| **Case C** (`3649b4dc…`) | sparse | `sensitive`; structural `Range`, W2 published b = 518, D-5 line | `Native`; admitted; `ONE_RUN_THROUGH_G_C`; bytes = plain + one notice; plain `a200ae94…`, published `e28f03dc…` (116,743 B) | native | 1 | `Native`, native reached | = PROBE §4 |
| | dense | the same | `Native`; plain `eafe8108…`, published `8a399b23…` (116,989 B) | native | 1 | `Native` | = PROBE §4 |
| **Two-body B** (`cf688351…`) | both | `checks_passed`; `Range`, W2 published b = 518 | `NoTriggeredCase`; admitted; `ONE_RUN_THROUGH_G_C`; exact plain bytes | T-4 (no W1) | 0 | `NoTriggeredCase`; no slot; sentinel unfired | PROBE §2.3's class (`not_required`) |
| **W6's PHYS-R4 input** (`19a424c5…`) | both | `checks_passed`; `Range`, W2 published b = 536 | `NoTriggeredCase`; exact plain bytes | T-4 | 0 | the same | PROBE §2.1's class |
| **W2b's input** (`d74d01ce…`) | both | `checks_passed`; report Passed, W2 not triggered | `NoTriggeredCase`; exact plain bytes | T-4 | 0 | the same | PROBE §2.1's class |
| **`b2_k1e3`** (`1ea4a168…`, file `89b05619…`) | both | `sensitive` (the report); report Sensitive, W2 not triggered | `Candidate`; admitted; `ONE_RUN_THROUGH_G_C`; bytes = plain + one notice | proof, after native Selected (sentinel consumed) | 1 | `Candidate` | = I86 §3.3: **stays Sensitive, in A, reaches native** |

`c1` (I86's publishing cap-maximal input) publishes its successor at the head with I86's bytes (`52573b15…`, `246f844e…`).

**Control.** My base run reproduces I81's `probe_run2.log` on all 28 shared (input, mode) pairs: input, plain and published sha, notices and cause.

### Item 4: c = 1 byte identity

**The committed successor pins** pass on both sides: `u1_milestone_successor_both_modes`, `u3_permitted_path_publishes_the_pinned_successor`, `u3g2_direct_entry_publishes_the_pinned_successor`, `u3g2_d_u6_5_carrier_fixtures_are_the_live_successors`, `u3_r1_carrier_names_exactly_one_publication`, `u8_l0_isolated_node_publishes_pinned_successor`, `u8_d_u6_5_l0_fixtures_are_the_live_successors`, and W2-deep at R/16 and R/64. Independently, my probe's published bytes equal the pins: the milestone's receipts `efc1a39b…` / `3e26499f…`, and L = 0's published bytes `9b425066…` / `5d84fce6…` with receipts `c00cbe76…` / `0b4250c8…`.

**My own differential, base against head** (`E/probe/out_base.jsonl`, `out_cand.jsonl`, `compare.txt`): 56 inputs × 2 modes = 112 rows.
- **Inputs:** every QUAL §4 witness input; U8's inputs and case C; `attempted_examples()` and `not_attempted_examples()`; all 29 `P/fixtures/product_preview/**/*.request.json` plus `invented_dec092_temperature_g_request.json`; the invocations inside the four committed milestone and L = 0 successor fixtures; and I86's `b2_k1e3` and `c1`.
- **Plain bytes:** 112 of 112 identical.
- **Direct's published bytes:** 106 of 112 identical. **The 6 that differ are exactly the triggered set:** two-body B, W2b's input and W6-PHYS-R4, both modes. Base published plain + one notice (`Native`, `Candidate`, `Native`); the head publishes the plain bytes.
- **The private driver's owner bytes:** the same 106 identical, the same 6 differing.
- **Every successor is identical**, and so is every driver successor. There are 21 Direct successor rows: the milestone (also as its fixture request, as `attempted_examples`' first entry and as the two milestone fixtures' invocations), L = 0 (also as its two fixtures' invocations), two-body A (sparse; dense is F-1's known `Precommit` G8 refusal on both sides), W2-deep and `c1`.
- The other `checks_passed` inputs (the load-reference and dec092 requests, the mixed-units source fixtures) stop at `Domain` or `Coexistence` before T-4 on both sides.

### Item 5: the seam is behaviour-neutral

- **`late_loads_total`** (PP `retained_product.rs`): `Default` 0; written only in `prepared_case_source`, inside `if let Some(permit)`, after the test hook and immediately before `LateFacts` and `check_late`, by `checked_add`; read nowhere (`grep`). It does not touch `self.adapter`, so **no adapter event is recorded** (RV107 A1-N-2). It is not in any `size_of`-priced or layout-recorded type.
- **`requested_cases`** (PP `retained_memory.rs`'s `CompleteFacts`): set in `permitted_run` from `request.model.load_cases.len()` before the request moves; read nowhere; `#[allow(dead_code)]` keeps the warning set equal (base and head: the same warnings, line numbers apart).
- **No receipt byte changes:** every successor and receipt is identical (item 4), including `c1`'s cap-maximal receipt.
- **The overflow path** (R3 ruling 2) is noted for SA (N-3).

### Item 6: the tests and the re-basings

**Every removed line was read** (53; `E/diffs/head_vs_base_47a3bdfcf5..a8e719f5b4.diff`). None weakens an assertion:
- the four oracles' two-case constructions become `beyond_load_cases(raw, renamed)`, which builds the identical two-case request while `LOAD_CASES` = 1 (renamed `case-2` where the test renamed, a duplicate `case` where it did not); the assertions are unchanged except their labels ("C + 1 cases");
- the seven `CompleteFacts { … }` literals gain `requested_cases: 1` and nothing else;
- W-C1's variant `w_c1_two_body_case_b` becomes `w_c1_case_c`; the loop's assertions are unchanged;
- W2b's `assert_eq!(ran, Fallback("Candidate"))` becomes the `NoTriggeredCase` pin. This is ruling 4's re-basing, forced by T-4; the native role goes to `b2_k1e3` at SQ (N-5);
- W6's input line becomes case C, with two **added** assertions (`force_scaled`, `Fallback("Native")`);
- doc lines: the two I77 renames and the new roles.

**Pinned as named:**
- **W-C1 is on case C** (`u8_real_input_fallbacks_append_one_notice`, sha asserted): admitted, `Native`, no successor, `ONE_RUN_THROUGH_G_C`, bytes = `with_notice(plain, "case", None)`, one notice, no hooks before or after; the kernel reason is not asserted.
- **W6 is on case C** (`witness_w6_force_scaled`, sha asserted): `force_scaled` and `Fallback("Native")` in both modes, at 4 MiB.
- **The `NoTriggeredCase` pins:** two-body B (default suite: private driver with a native sentinel; Direct; Stale branch), W2b's input and W6-PHYS-R4 (witnesses; cause and exact bytes). Input shas are asserted and equal I81's.
- **The four facade oracles** take `caps::LOAD_CASES + 1` cases, and each keeps its combination variant.
- **I77's renames** are applied (`zz_rv93_input_fallbacks`; `w6_input()`). No stale citation remains outside `P/execution`.
- **The classifier test** covers PLAN_v2 §2.1's whole list; each assertion's label names what it pins. The decision-21 test runs `retained_w1` on the milestone's real observer, in sparse mode only.
- **The test-only commit `a8e719f5b4`** moves the not-unique block after the request-order block and reorders the doc sentence; no assertion is added, removed or changed (`E/diffs/testonly_4a51783e65..a8e719f5b4.diff`).
- **Test-only visibility:** `w_c2_case_c()` and its sha are `pub(super)` inside the `#[cfg(test)]` facade module (R3 ruling 3). No `src` item's visibility changed.

### Item 7: mutants (`E/mutants/`)

Each is one exact textual edit of `CAND`'s `PP/lib.rs`, then PP's whole `--lib --no-fail-fast -- --include-ignored` (the witnesses included), registered, target `rv109-mut`. The pristine control in the same target and flags: 559 passed, 1 failed (`t13`, known), 0 ignored. `PP/lib.rs` was restored and checked (`84fba5ca…`) after the set. `t13` is excluded below.

| Mutant | Edit | Killed by (first failing assertion of each failing test) |
|---|---|---|
| **P1** keyed on `initial` (PLAN) | `not_required` when the seed's report code is `…CHECKS_PASSED` | classifier test "W2-published Passed"; two-body B pin (cause); W6-PHYS-R4 witness (cause) |
| **P2** verdict by position (PLAN) | `quality.cases.get(position)` | classifier test "request order, looked up by id" |
| **P3** dropping decision 21 (PLAN) | the exclusion branch disabled | classifier test "Mechanism … Some(Failed), no W2"; decision-21 test "excluded: A is empty" |
| **P4** seedless excluded (PLAN) | `seed.map_or(true, dn_trigger_excluded)` | classifier test "no seed"; decision-21 test "no seed: in A, so W1 runs"; `witness_w2_cap_maximal` |
| **P5** `NoTriggeredCase` appends a notice (PLAN) | reserve, then `notice.publish(ordinary, NoTriggeredCase)` | decision-21 test "excluded: the exact ordinary bytes"; two-body B pin "the exact ordinary bytes, no notice"; W2b and W6-PHYS-R4 witnesses (exact bytes) |
| R6 seed by position | `seeds.get(position)` | classifier test "request order, looked up by id" |
| R7 first match, not unique | `find` for `only_one` | classifier test "two entries for one case" |
| R8 exclusion ignores W2 | the `Published` clause disabled | classifier test "… W2 published"; decision-21 test "W2 published: in A, so W1 runs" |
| R9 failed W2 counts as published | exclusion only for `NotTriggered` | classifier test "… W2 failed" |
| **R10** T-4 after the reservation | reserve first, then T-4 (slot dropped) | **SURVIVED** (SF-1). My discriminator kills it: spare slot, and `NoticeReservation` on a pre-marked base |
| R11 `not_assessed` as `not_required` | | classifier test "NotAssessed"; `witness_w2_cap_maximal` |
| R12 `NegativeEnergy` excluded | | classifier test (the other-failures loop) |
| R13 `Mechanism` not excluded | | classifier test "Mechanism … no W2"; decision-21 test "excluded: A is empty" |
| R14 seeds not wired into `retained_w1` | `observer.ordinary[..0]` | decision-21 test "excluded: A is empty" |
| R15 quality not wired into `retained_w1` | empty `cases` | two-body B pin (cause); W2b and W6-PHYS-R4 witnesses |
| R16 `not_required` needs a seed | `&& seed.is_some()` | **Survived;** equivalent on reachable states (N-1) |
| R17 `.all` for `.any` | | **Survived;** equivalent at c = 1 (N-2) |

**PLAN_v2's five are each killed by an assertion; every mutant compiled.** Of my twelve, ten are killed; R16 and R17 are equivalent at c = 1, and R10 is SF-1.

### Item 8: the suites, base against head, test by test (`E/suites/`)

| Suite | Base `47a3bdfcf5` | Head `a8e719f5b4` | Per-test difference |
|---|---|---|---|
| PP registered, all targets | 708 ok, 1 failed (`t13`), 10 ignored | 711 ok, 1 failed (`t13`), 11 ignored | **+3 new (ok):** `b1_t4_classifier_reads_the_published_verdict_by_case_id`, `b1_t4_retained_w1_applies_decision_21_and_keeps_a_seedless_case`, `b1_t4_two_body_case_b_is_a_no_triggered_case_pin`. **Ignored:** `witness_w2b_cap_maximal_solvable` renamed `witness_w2b_cap_maximal_passed_report_no_triggered_case`; `witness_w6_phys_r4_input_no_triggered_case` new. **Nothing else** (`diff_pp_reg_base__pp_reg_cand.txt`: 6 differences) |
| PP Stale (`--lib`; identity `rustflags=--cfg=rv109_stale`) | 545 / 1 (`t13`) / 10 | 548 / 1 (`t13`) / 11 | the same 6. **Stale = registered, outcome for outcome,** on both sides (556 and 560 lib tests) |
| Runner (`P/core/runner/headless`) | 85 ok, 2 failed (the known `load_reference` golden pair) | the same | none |
| Witnesses (`--lib witness_ -- --ignored`), registered | 9 passed | 10 passed | W2b: `Candidate` → `NoTriggeredCase` ×2; W6: `Native` ×2 on PHYS-R4 → `Native` ×2 on case C (`force_scaled=true`); W6-PHYS-R4 (new): `NoTriggeredCase` ×2. W1, W1@1MiB, W2, W2-deep (R/16, R/64), W3, W4, W7 identical |
| Witnesses, Stale (head) | – | 10 passed | the same lines as registered: the `NoTriggeredCase` outputs are the plain bytes |
| RE `retained_precision_carriers` (head) | – | 16 passed | – |

The listed differences are PROBE §2.4's table (W2b, W6, W-C1), the new pins and the renames. The `t13` and `load_reference` failures are the same tests with the same assertions on both sides.

### Item 9: the fence and the guards

- **The fence:** 6 files, all PP `src` (`lib.rs`, `retained_product.rs`, `retained_memory.rs`, `retained_memory_law_tests.rs`, `retained_facade_tests.rs`, `retained_memory_witness_tests.rs`), +402 / −53. Nothing in FK, the schema, any `Cargo.lock` or `Cargo.toml`, the 13 reviewed statics (build.rs's `REVIEWED_INPUTS`), RE or any reader, `grant2.rs`, PP-tests, or the T6S or B6 files.
- **In `retained_memory.rs`, only `CompleteFacts`** (outside the GENERATED PROFILE block; `LOAD_CASES` unchanged); **in the law tests, only the seven literals.**
- **The registered identity is unchanged:** the head's build reads `Registered`.
- **The guards pass with their expected text unchanged:**
  - `PP-tests/s11f_site_test.rs` 11/11 and `PP-tests/retained_precision_admission.rs` 5/5, on both sides; RE's carrier test 16/16;
  - the in-fence guards (`u3_n9_single_parse_custody`, `u3_permitted_outputs_keep_the_report_and_gate_order`, `u3_capture_permit_is_linear`, `u3g2_no_permit_path_runs_once_without_a_copy`, `u1_serializer_reads_no_legacy_work_field`, the law tests that read `admit`'s source, `challenge_bounds_are_the_profile`) pass, and no hunk touches their expected text.
- **Rule 8:** the seam's add is a `checked_add` followed by a plain assignment, a shape rule 8 does not count. It is an integer count, not a force accumulation, so no `TABLE` row is owed.

## 3. The PR-head ledger (start)

**Head `a8e719f5b4`** = `4a51783e65` (ST) + `a8e719f5b4` (test-only), over main `47a3bdfcf5`. **Every hunk of `47a3bdfcf5..a8e719f5b4` is reviewed here (RV109, round 1).**

| # | File | Hunk (base → head) | Content | Commit | Reviewed |
|---|---|---|---|---|---|
| 1 | PP `lib.rs` | `-2221,6 +2221,11` | `W1Fallback::NoTriggeredCase` | `4a51783e65` | RV109 r1 |
| 2 | | `-2988,6 +2993,9` | `permitted_run`: `requested_cases` read | `4a51783e65` | RV109 r1 |
| 3 | | `-3003,7 +3011,7` | `CompleteFacts { …, requested_cases }` at G-C | `4a51783e65` | RV109 r1 |
| 4 | | `-3093,8 +3101,62` | `CaseTrigger`, `case_triggers`, `only_one`, `dn_trigger_excluded`; `retained_w1`'s doc | `4a51783e65` | RV109 r1 |
| 5 | | `-3114,6 +3176,11` | `retained_w1`: T-4 before the reservation | `4a51783e65` | RV109 r1 |
| 6 | PP `retained_facade_tests.rs` | `-51,6 +51,23` | `beyond_load_cases` | `4a51783e65` | RV109 r1 |
| 7 | | `-168,16 +185,13` | `u3_each_stage_fault_…`: C + 1 oracle | `4a51783e65` | RV109 r1 |
| 8 | | `-207,14 +221,12` | `u3_no_permit_entries_…`: C + 1 oracle | `4a51783e65` | RV109 r1 |
| 9 | | `-650,16 +662,13` | `u3g2_direct_entry_no_w1_refusals_…`: C + 1 oracle | `4a51783e65` | RV109 r1 |
| 10 | | `-773,13 +782,12` | `u3g2_no_permit_path_runs_once_…`: C + 1 oracle | `4a51783e65` | RV109 r1 |
| 11 | | `-829,11 +837,13` | U8 header; `zz_rv93_input_fallbacks` rename | `4a51783e65` | RV109 r1 |
| 12 | | `-848,8 +858,8` | `w6_input()` rename | `4a51783e65` | RV109 r1 |
| 13 | | `-867,8 +877,9` | two-body B's doc (its new role) | `4a51783e65` | RV109 r1 |
| 14 | | `-879,6 +890,19` | `w_c2_case_c`, the two input shas | `4a51783e65` | RV109 r1 |
| 15 | | `-893,13 +917,16` | W-C1 on case C | `4a51783e65` | RV109 r1 |
| 16 | | `-1069,3 +1096,198` | the B1 ST tests: helpers, classifier test, two-body B pin, decision-21 test | `4a51783e65`, then `a8e719f5b4` (3 sub-hunks: the doc order and the not-unique block moved after the request-order block) | RV109 r1 |
| 17 | PP `retained_memory.rs` | `-2366,6 +2366,11` | `CompleteFacts::requested_cases` | `4a51783e65` | RV109 r1 |
| 18–23 | PP `retained_memory_law_tests.rs` | `-807`, `-817`, `-852`, `-861` (two literals), `-1203`, `-1340` | `requested_cases: 1` in the seven literals | `4a51783e65` | RV109 r1 |
| 24 | PP `retained_memory_witness_tests.rs` | `-76,6 +76,28` | `no_triggered_case_witness` | `4a51783e65` | RV109 r1 |
| 25 | | `-126,11 +148,14` | W2b's doc and rename | `4a51783e65` | RV109 r1 |
| 26 | | `-144,8 +169,7` | W2b's `NoTriggeredCase` pin | `4a51783e65` | RV109 r1 |
| 27 | | `-174,10 +198,11` | `w6_input`'s doc | `4a51783e65` | RV109 r1 |
| 28 | | `-199,17 +224,37` | W6 on case C; the W6-PHYS-R4 pin | `4a51783e65` | RV109 r1 |
| 29 | PP `retained_product.rs` | `-144,6 +144,11` | `ProductCapture::late_loads_total` | `4a51783e65` | RV109 r1 |
| 30 | | `-3242,6 +3247,11` | the seam's checked add before G-B | `4a51783e65` | RV109 r1 |

**Carried open:** SF-1's repair (test-only), to be confirmed by RV-P before I1. Later rounds extend this ledger with SF-1's repair commit, SP's commits, and every integration merge.

## 4. For ROOT

1. **SF-1's severity is a choice for ROOT.** I rate it SHOULD-FIX: a test-only repair in ST's fence, before I1, which I would confirm. The alternative is carrying it to SP's reservation-count pin, under the condition stated in the table.
2. **R3 ruling 1 (uniqueness) is accepted,** for the reasons in item 1.
3. **R3 ruling 2 (the seam's overflow)** is noted for SA (N-3).
4. **No stop fired.** Nothing in PLAN_v2 §1's never-touched list or R8 was reached. Decision 1's owner-information line holds over every committed fixture request too: among 56 inputs, only the three ruled inputs change.

## 5. Host, commands and cleanup

- **Copies:** `git archive 47a3bdfcf5` and `git archive a8e719f5b4` (from `WT/b1`, `GIT_OPTIONAL_LOCKS=0`) into `BASE` and `CAND`.
- **Jobs** (`E/tools/`; each through `runjob.sh` → `WT/tools/t3_cargo.sh`):
  - `phase1_suites.sh`: PP registered all targets, base and head; the witnesses, base and head; PP Stale `--lib`, base and head; the runner, base and head; RE's carrier test, head.
  - `phase2_probe.sh`: the head's witnesses in Stale; then the probe installed in both copies and `test --lib zz_rv109_probe_all -- --ignored --nocapture --test-threads=1`, base and head (`RV109_FIX_FILES` in `E/probe/fixture_files.txt`; `RV109_FILES` = I86's `b2_k1e3.json`, `c1.json`).
  - `phase3_mutants.sh`: the probe removed, the pristine control, then each mutant (`mutants.py`).
  - `phase4_r10.sh`: R10's discriminator (`zz_rv109_r10_discriminator`, appended to the probe file after phase 2) on the head and on R10; then `CAND` restored (`PP/lib.rs` `84fba5ca…`, `retained_memory_witness_tests.rs` `c415c3e1…`).
- **Analysis:** `suite_diff.py`, `probe_compare.py`, `mutant_summary.py` (system `python3`, read-only); `sanitize.py` for the records.
- **Cleanup:** `BASE`, `CAND` and the eight targets `WT/targets/rv109-*` were deleted at 06:01 UTC, after the last job ended. My scratch `S` (full logs, probe outputs, mutant logs, scripts) is kept for my later rounds. No process of mine is running.
- **Time:** about 2 h of agent time (04:15–06:05 UTC), much of it waiting for the lock, against 4–6 h.

## 6. Limits

- **c = 1 only.** The classifier is written for n cases, but its multi-case behaviour (R17's distinction, the reservation count) is SP's to pin.
- **Decision 21 is reached only through hand-set seeds.** No committed input reaches it, as PROBE §3 found.
- **The probe ran in the registered dev/test build with its own prints.** It establishes outcomes and bytes, not S1 stack margins, which are SQ's.
- **Stale:** PP `--lib` and the witnesses only. I85 ran Stale on all targets.

## 7. Records

- `REVIEW.md` (this file) and `SHA256SUMS`.
- `evidence/suites/`: filtered logs of every suite run, and the test-by-test differences (`diff_*.txt`).
- `evidence/witness/`: the `I65_G5_WITNESS` lines (base, head, head Stale).
- `evidence/probe/`: the probe source and its two shims, both outputs (`out_base.jsonl`, `out_cand.jsonl`), `compare.txt`, `fixture_files.txt` and the filtered run logs.
- `evidence/mutants/`: `summary.md`, each mutant's filtered log, its apply record and its `lib.rs` sha256, the pristine control, and the R10 discriminator's two runs.
- `evidence/diffs/`: the head's diff and the test-only commit's diff.
- `evidence/tools/`: every script I ran.
- `evidence/cargo_jobs_rv109.log`: my lines of the lock log.

Machine paths are replaced by `WT`, `NUM`, `T`, `R`, `VENV` and `~`. A log line over 4,000 bytes is cut to 1,000 bytes, followed by its length and sha256.
