# I85 B1-ST: T-4 at c = 1, decision 21, `NoTriggeredCase`, the re-basings and the SA–SP seam (checkpoint R3)

TASK (Type 2), I85, role I-P, for ROOT (HELP_HUMAN, Agent 0), who is the return path. I made no delegation. I am a fresh instance. 2026-10-07 UTC. This is checkpoint R3: ST is complete, and no review has run.

**Briefs (verified before work):** `R/BRIEFS/B1_COMMON.md` sha256 `2d170307516b98c40e8aa2dcf352cf11a13dbdd29aeddd78a4c39f3f15eb2c75` and `R/BRIEFS/B1_ST.md` sha256 `25df720319da96c2a0367a0c345faac4edb303e52c5ab4a38af2652de069350d`. The specification, `R/I84/b1_plan_01/PLAN_v2.md`, has sha256 `c85786b704805311485b44ba27c8826ca278c1237d92010f9f997dbf3c919be0`, as B1_COMMON states. I read NUM's root `AGENTS.md` and `agents/AGENT_TASK.md` first.

**Basis read:**
- PLAN_v2: §0, §1 (the fence and the source-text guards), §2.1 (ST), §2.2–§2.3 (for the seam), §4, §7 and §8;
- RR "I81's B1-0 probe verified…" (rulings 1–4), "R1: B1's plan ruled…" and "B1's PLAN_v2 accepted; RV107's A1 amendments; phase 1 dispatched";
- `R/I81/b1_probe_01/PROBE.md` (PROBE) §0–§8, its `_run_records/zz_i81_probe.rs` (case C's builder and the input shas) and `witness_run.log`;
- `R/I78/b0_contract_01/DESIGN_v2.md` §1.1–§1.4 (T-1 to T-13; the outcome table) and decisions 1 and 21;
- I68's `R/I68/u8_witnesses_01/RETURN.md` and its `run_suites.sh` and `mutants.py`, whose method I followed;
- I77's `R/I77/u8_package_01/RETURN.md` items 1 and 4 (the two citation renames).

**Placeholders:** `WT`, `NUM`, `P`, `PP` (= `P/core/product_physics/src`), `PP-tests` (= `P/core/product_physics/tests`), `RE`, `T`, `R`, `RR` and `VENV` as in the dispatch. `S` = my scratch, `WT/scratch/i85_b1_st`. `BASE` = `S/base`, a `git archive` of main `47a3bdfcf5` (`P` without `P/execution`). `MUT` = `S/mut`, the same archive of the head.

**Limits kept.**
- Every cargo job went through `WT/tools/t3_cargo.sh`, with `--locked --offline`, `CARGO_BUILD_JOBS=4`, `RUST_TEST_THREADS=2`, `TMPDIR` in my scratch, and `RUSTFLAGS`/`CARGO_ENCODED_RUSTFLAGS` unset except in the Stale runs. The memory guard was up. There were 29 jobs; they waited behind ROOT's DEC-025 `SI1b_b4f22e6ce7` and interleaved with I83's and I86's (`_run_records/cargo_jobs_i85.log`). I killed no job.
- No vitest or pytest ran. Python was used only for the mutant driver (VENV) and the log sanitizer.
- No DEC-025, no evidence sweep, no native or solver-at-scale job, no install. The only witness inputs run were QUAL §4's committed ones, case C, and two-body B, as the plan names them.
- The Git writes were my two commits on `codex/piping-t3-b1-20261007`; nothing was pushed. Reads used `GIT_OPTIONAL_LOCKS=0`.
- Nothing was written to the system temp directory. The host accepted every write into this records folder; no write guard refused anything.

## 0. Outcomes in brief

| Acceptance item (PLAN_v2 §2.1) | Outcome |
|---|---|
| Byte identity of every committed c = 1 successor pin, both modes | **Holds.** The milestone (U1 and U3's private-driver and Direct pins), U6's carrier fixtures and L = 0 (document, receipt and published-bytes pins, plus its fixtures) pass unchanged. The bytes that the pin tests write are identical on base and head, file for file (§4) |
| The registered PP suite differs from base only in the listed tests | **Holds.** Base: 708 ok, 1 failed (the known Mac `t13`), 10 ignored. Head: 711 / 1 (`t13`) / 11. The only differences are 3 new tests (ok), 1 renamed ignored witness and 1 new ignored witness. Every other outcome is identical, including the re-based oracles and W-C1's variant (§5) |
| The source-text guards pass | **Yes,** early (on the uncommitted change) and again at the head. s11f's 11 tests, PP-tests' admission guard (5), RE's carrier test (16), and every in-fence guard pass. No expected text changed and no `TABLE` row was needed (§6) |
| The classifier's unit tests | **Pass,** covering every case on PLAN_v2's list (§3.2) |
| The five mutants, each killed by an assertion | **5 of 5 killed by assertions.** Every mutant compiled (§7) |
| Stale: every output is the plain bytes | **Holds.** The Stale PP suite equals the registered suite outcome for outcome, on base and on the head. In the Stale build, the head's `NoTriggeredCase` outputs, Direct and private driver, are the plain bytes (§8) |
| Decision 21's branch | Pinned only by unit tests (and one private-driver test with hand-set seeds), as planned. No audited committed input reaches it (PROBE §3) |
| W2 (`not_assessed`, no seed) | **Unchanged:** `Fallback("Preparation")` in both modes. A seedless case stays in A (ruling 2) |
| Stops (§1, R8) | **None fired.** There was no FK, schema, base-reader, reviewed-input or D1-visibility change, no c = 1 byte change, and no weakened guard |

## 1. Head and commits

Branch `codex/piping-t3-b1-20261007` in `WT/b1`, from main `47a3bdfcf5`. **Head `a8e719f5b4452fffec62ea29305e9a3bec8b067f`.**

| Commit | Content |
|---|---|
| `4a51783e655610c4e2a4b808993c76d8e77768c0` | ST: the production change, the seam, and the tests and re-basings |
| `a8e719f5b4452fffec62ea29305e9a3bec8b067f` | Test-only. In the classifier test, the request-order block now comes before the not-unique block, so that the position mutant is first refused by the out-of-order assertion that PLAN_v2 names (§7). No assertion was added, removed or changed |

The diff `47a3bdfcf5..a8e719f5b4` (`_run_records/st.diff`): 6 files, +402 / −53. All are PP `src`, inside ST's fence:

| File | Base blob | Head blob | sha256 at head | Lines |
|---|---|---|---|---|
| `PP/lib.rs` | `a1990c9f58` | `f26924d239` | `84fba5ca57905d69ea979ce815e584f26eb95ac45d32a54e30918d7580edd586` | +69 / −2 |
| `PP/retained_product.rs` | `b73225304e` | `d43529ff2f` | `5cf8d6d56a59ba018cb7f2f8c8a2a60707da5539f040f6b1c717fca2dbf452c4` | +10 / −0 |
| `PP/retained_memory.rs` (only the `CompleteFacts` field) | `843b7ec59f` | `688cb0da5c` | `cc67222a0fdcf87144445309fa25c4ac9bbd987a493e03976967cf757399558b` | +5 / −0 |
| `PP/retained_memory_law_tests.rs` (only the seven `CompleteFacts` literals) | `d914ad3738` | `95a6dd26e8` | `6c34f9deecdfc0e73c28af331307ca7f082c489156d6456b4ae1ef5a2e4d2164` | +7 / −7 |
| `PP/retained_facade_tests.rs` | `a7c4cf92f4` | `148d5e4673` | `454affff5ccd521ad7ae444b544ea1c2b58440a9ff4afc72c87ed8469f544c24` | +256 / −34 |
| `PP/retained_memory_witness_tests.rs` | `9745e3fe38` | `aeabe8e48a` | `c415c3e1c9893d49cf2fad0be2420e14d2c52cbdd0b9c026975cfc3dd3865b8a` | +55 / −10 |

No other file changed: not FK, the schema, `Cargo.lock`, the 13 reviewed statics, RE or any reader, `grant2.rs`, PP-tests, or the T6S or B6 files.

## 2. The code

### 2.1 T-4 and decision 21 (`PP/lib.rs`)

- **`W1Fallback::NoTriggeredCase`.** It carries no payload, so it allocates nothing (B-10). Its doc names it as distinct from `LegacySeed::NotRequired` and from the `not_required` disposition.
- **`CaseTrigger { NotRequired, Excluded, Attempted }`** and **`case_triggers(quality, seeds, case_ids)`.** The classifier is written for n cases: it yields one class per requested case id, in request order. For each case:
  - **The verdict** is the `solve_quality` of the one `numerical_quality.cases[]` entry whose `basis_ref.ref_id` is the case id. It is never found by position. The binding is `ordinary_value`'s, which also requires exactly one match.
  - **The seed** is the one `OrdinarySeed` whose `case` is the case id.
  - **`NotRequired`** exactly when the verdict is `checks_passed`. This is checked first, so a Passed verdict is `not_required` whatever its seed says.
  - **`Excluded`** (`dn_trigger_excluded`) when the seed's `initial` is `StructuralFailure` with `Mechanism`, `Asymmetric` or `InvalidInput`, and its `w2` is not `Published`. `NotTriggered` and `Failed` both count as not published.
  - **`Attempted`** (in A) otherwise. That includes `sensitive`, `not_assessed`, `unresolved` and `failed` verdicts, an absent entry, a seed whose `initial` is unset, and a case with no seed (ruling 2).
  - **An entry or a seed that is not unique counts as absent** (helper `only_one`). That is my reading, mirroring `ordinary_value`; see §9 item 2.
- **`retained_w1`:** after coexistence, G-B's outcome and `w1_case_id`'s D1.4 scope (`Domain`), and before the notice reservation:

  ```rust
  if !case_triggers(&ordinary.numerical_quality, &observer.ordinary, &[case_id]).any(|trigger| trigger == CaseTrigger::Attempted) {
      return (ordinary, Err(W1Fallback::NoTriggeredCase));
  }
  ```

  With A empty, the ordinary owner is returned untouched: no notice, no reservation and no W1 stage. The temporary `[case_id]` allocates nothing. **`w1_case_id` is unchanged**, as PLAN_v2 says; SP replaces it and passes every request id.
- **Guards on this code.** It adds no `borrowed_raw()` read and no `parse(` site, and the call counts that `u3_n9_single_parse_custody` pins are unchanged. It adds no rule-8 shape (no compound assignment, `.sum(`, `fold(` or self-assignment fold).

### 2.2 The SA–SP seam (behaviour-neutral at c = 1; decision 22)

- **`ProductCapture::late_loads_total: usize`** (`PP/retained_product.rs`; `Default` 0).
  - In `prepared_case_source`, inside `if let Some(permit)=self.permit.as_ref() {` and after the test-only `before_late_gate` hook, immediately before `LateFacts` and `check_late`: `checked_add(case.primitive_loads.len())`. On overflow it sets `self.error = Some(CaptureError::CountRange("late loads total"))` and returns.
  - So G-B at case k will see Σ_{i≤k} l_i.
  - **It records no adapter event** (RV107 A1-N-2): the update is a plain field write, outside the adapter's counters.
  - It is a plain assignment, so neither rule 3 (`+=` on a "load" binding) nor rule 8 counts it.
- **`CompleteFacts::requested_cases: usize`** (`PP/retained_memory.rs`). It is set in `permitted_run` from `request.model.load_cases.len()`, read after D1.3's defensive `Domain` return and before the request moves into the observed run. It allocates nothing. It carries `#[allow(dead_code)]` with a comment, as `LateFacts.model` does, so that the build gains no warning.
- **The law tests' seven `CompleteFacts { … }` literals** gain `requested_cases: 1`. No other edit was made to SA's files.
- **Neither field is read in ST.** `ordinary_solve_attempted`, G-B's observation list and `LateFacts` are unchanged.

## 3. The tests

### 3.1 Changed (`PP/retained_facade_tests.rs`, `PP/retained_memory_witness_tests.rs`)

| Test | Change |
|---|---|
| `u8_real_input_fallbacks_append_one_notice` | **W-C1's variant is re-based on case C alone** (`w_c1_case_c`; it was `w_c1_two_body_case_b`). It runs through Direct in both modes. The loop already asserts admitted, `Native`, no successor, `ONE_RUN_THROUGH_G_C`, bytes = `with_notice(plain, "case", None)`, one notice, and no hooks before or after. The test first asserts the input's sha256 equals PROBE §4's `3649b4dc…`. The kernel reason, Unresolved(Ceiling), is PROBE's record and is not asserted |
| `u3_each_stage_fault_falls_back_to_the_ordinary_bytes` | Its D1.4 oracle takes `caps::LOAD_CASES + 1` cases, labelled "C + 1 cases", through the new helper `beyond_load_cases(raw, renamed)`. The combination variant is kept |
| `u3_no_permit_entries_are_the_ordinary_route` | The same (copies keep the first case's id, as before) |
| `u3g2_direct_entry_no_w1_refusals_keep_exact_bytes` | The same (ids `case-2` …, as before); the combination and namespace variants are kept |
| `u3g2_no_permit_path_runs_once_without_a_copy` | The same (copies keep the first id, as before) |
| (doc comments) | **I77's two renames:** `zz_rv93.rs:292–315` becomes `zz_rv93_input_fallbacks`, and `retained_memory_witness_tests.rs :181–199` becomes `w6_input()`. The U8 section's header and two-body B's doc now state its new role |
| `witness_w6_force_scaled` (ignored) | **Re-based on case C,** on its private driver at R/k = 4 MiB. It asserts the input sha (PROBE §4), `force_scaled` (newly asserted; it was only printed), and `Fallback("Native")` in both modes |
| `witness_w2b_cap_maximal_solvable` → **`witness_w2b_cap_maximal_passed_report_no_triggered_case`** (ignored) | **Renamed** to what it now pins: a `NoTriggeredCase` pin with exact bytes, on the same input (sha `d74d01ce…`, asserted). Its doc says its cap-maximal native role moves to SQ (W2b's replacement, from SW) |
| W2, W2-deep, W1, W3, W4, W7, headroom | Unchanged |

**`beyond_load_cases`** builds the request by copying the first case `LOAD_CASES` times. While C = 1 it is exactly the old two-case request: the id `case-2` where the test renamed its copy, and a duplicate of `case` where it did not. After SA it gives C + 1 = 4 cases with no further edit.

### 3.2 New

| Test | What it pins |
|---|---|
| `b1_t4_classifier_reads_the_published_verdict_by_case_id` | The unit tests, on hand-built quality entries and seeds:<br>• **each verdict:** Passed is `NotRequired`; Sensitive, NotAssessed, Unresolved and Failed are `Attempted`;<br>• **W2-published Passed** (structural/range, W2 published) is `NotRequired`;<br>• **report Passed** is `NotRequired`;<br>• **a report-Passed seed under a Sensitive verdict** (R-b′'s shape) is `Attempted`: the verdict decides, not `initial`;<br>• **each excluded tag** (Mechanism, Asymmetric, InvalidInput), with W2 not triggered and with W2 failed, is `Excluded` under each of Failed, Unresolved, NotAssessed and an absent entry. With W2 published it is `Attempted`, and with a Passed verdict it is `NotRequired`;<br>• **the other failures** (Range, NumericallyUnresolved as in `tiny_spring`, NegativeEnergy, K2a's FormationFailure, and an unset `initial`) are `Attempted`;<br>• **no seed** is `Attempted` (W2's shape), including with no entry;<br>• **out of request order:** quality entries `[c, a, b, d]`, seeds `[d, b, c, a]`, requests `a, b, c, d` and `d, c, b, a` read correctly by id; no requested case gives no class;<br>• **not unique:** two entries or two seeds for one case count as absent |
| `b1_t4_two_body_case_b_is_a_no_triggered_case_pin` | **Two-body B** (input sha `cf688351…`, asserted), both modes.<br>• **Private driver, every build:** the observed run equals plain; the published verdict is `[("case", checks_passed)]`; the seed is structural failure with W2 published; the cause is `NoTriggeredCase`; the bytes equal plain exactly; and an armed native-stage fault is still armed afterwards, so no W1 stage ran.<br>• **Direct, registered:** admitted, `NoTriggeredCase`, no successor, `ONE_RUN_THROUGH_G_C`, 0 notices, exact plain bytes, and the native fault handed back unfired.<br>• **Direct, Stale:** no W1, `ONE_RUN`, plain bytes |
| `b1_t4_retained_w1_applies_decision_21_and_keeps_a_seedless_case` | Through `retained_w1` on the milestone's actual observer, with one hand-set seed:<br>• a Mechanism failure without W2 gives `NoTriggeredCase` and exact bytes;<br>• with W2 published, and with no seed, the case is in A and W1 runs (a successor with the untouched owner, or the plain notice) |
| `witness_w6_phys_r4_input_no_triggered_case` (ignored) | **W6's PHYS-R4 input** (sha `19a424c5…`, asserted) is a `NoTriggeredCase` pin on the witness stack in both modes, with exact bytes |

**The witness pins' helper, `no_triggered_case_witness`.** It runs the same private-driver work as `permitted_work` on `on_reserved_stack(WITNESS_STACK, carry_test_hooks(…))`, and also hands back the ordinary owner, whose bytes it compares with the plain route's. `permitted_work` itself is unchanged, so the other witnesses run exactly the driver they ran before.

**Case C's builder.** `w_c2_case_c()` is `u8_two_body_case_a()` with case B's two tip loads appended to the one case, which is PROBE §4's construction. It and `W_C2_CASE_C_INPUT_SHA256` are `pub(super)` inside the `#[cfg(test)]` facade test module, so that the witness module reuses them. This is test-only; no D1 `src` item's visibility changed (§9 item 6).

## 4. Byte identity of the committed c = 1 successor pins

**The pin tests pass on the head**, registered and Stale, in both modes:
- `u1_milestone_successor_both_modes`;
- `u3_permitted_path_publishes_the_pinned_successor`;
- `u3g2_direct_entry_publishes_the_pinned_successor`;
- `u3g2_d_u6_5_carrier_fixtures_are_the_live_successors`;
- `u3_r1_carrier_names_exactly_one_publication`;
- `u8_l0_isolated_node_publishes_pinned_successor`, which pins the document, receipt and published-bytes sha256;
- `u8_d_u6_5_l0_fixtures_are_the_live_successors`;
- W2-deep's successors on the witness stack (R/16 and R/64).

**A direct comparison** (`_run_records/scripts/run_pins.sh`; `_run_records/pins/`). The pin tests' own output variables (`I61_U3_OUT`, `I61_U3G2_OUT`, `I68_U8_OUT`) wrote the successor documents on base and on `4a51783e65`. The head differs from that commit only in the classifier test's statement order:

| Document | sha256 (base = head) |
|---|---|
| milestone, sparse (U3 private driver and U3G2 Direct) | `ac6986b0680e0df9d88c33a5bf4635372fc3b83cbb9080da44e6d32dbdca59dc` |
| milestone, dense | `6cd1d249e5352aaffbd2b7d7349c74a1d0e0572df35500f66be49c3cad95c9b5` |
| L = 0, sparse | `93c6c86548b9d263cba9d9869010043d23ed1c9f2f304dd9f82eb705eb350876` |
| L = 0, dense | `dbb3d477364248fb9ae15f7b9cff44410c2bffe45f783dd02d96eca663b0ac88` |

- All 6 files are **byte-identical between base and the candidate**.
- Each equals its committed fixture (`P/fixtures/results/retained_precision_{milestone,l0}_successor_*.json`) and U1's or U8's pinned hash.

## 5. Suites against base `47a3bdfcf5` (`_run_records/suites/`)

Each suite ran `cargo test --locked --offline --no-fail-fast`, registered (`RUSTFLAGS` unset) or Stale (`--cfg=i85_b1_st_stale`), in separate targets. The build identities are in `_run_records/build/build_identities.txt`: the registered ones equal `REGISTERED_IDENTITY`, and the Stale ones read `rustflags=--cfg%3Di85_b1_st_stale`.

| Suite | Base | Head | Per-test difference |
|---|---|---|---|
| PP, registered (all targets) | 708 ok, 1 failed (`s11g_tests::t13_committed_fallback_uz_is_byte_identical`, the known Mac `t13`), 10 ignored | 711 ok, the same 1 failed, 11 ignored | **+3 new (ok):** `b1_t4_classifier_reads_the_published_verdict_by_case_id`, `b1_t4_retained_w1_applies_decision_21_and_keeps_a_seedless_case`, `b1_t4_two_body_case_b_is_a_no_triggered_case_pin`. **Ignored:** `witness_w2b_cap_maximal_solvable` renamed to `witness_w2b_cap_maximal_passed_report_no_triggered_case`; `witness_w6_phys_r4_input_no_triggered_case` new. Nothing else |
| PP, Stale | 708 / 1 (`t13`) / 10 | 711 / 1 (`t13`) / 11 | The same differences. Stale = registered, outcome for outcome, on both sides |
| runner/headless, registered | 85 ok, 2 failed (the known `load_reference` pair) | the same 85 and 2 | none |
| PP witnesses, `--lib witness_ -- --ignored` (registered; head also Stale) | 9 passed | 10 passed (registered and Stale) | See below |

**The witness lines** (`I65_G5_WITNESS`; `_run_records/witness/`):

| Witness | Base | Head |
|---|---|---|
| W1, W1@1MiB, W2-deep (R/16, R/64), W3, W4, W7 | as QUAL §4 | identical |
| W2 (cap-maximal, `not_assessed`, no seed) | `Fallback("Preparation")` ×2 | **identical** (ruling 2 holds) |
| W2b | `Fallback("Candidate")` ×2 | `Fallback("NoTriggeredCase")` ×2, exact bytes |
| W6 | `Fallback("Native")` ×2, on the PHYS-R4 input | `Fallback("Native")` ×2, **on case C**, `force_scaled=true` |
| W6-PHYS-R4 (new) | — | `Fallback("NoTriggeredCase")` ×2, exact bytes |

**Run time** (informational): the witness set took 49 s on base and 9 s on the head. The difference is W2b's full cap-maximal native run, which no longer happens. That is ruling 1's known loss, carried to SW and SQ.

**Compiler warnings:** the same messages and counts on base and head, for PP (registered and Stale) and the runner (`*.warnings`). Only the order of cargo's two summary lines differs, as in I68's record.

**Which commit each run used.**
- The runner suite, the witness runs (registered and Stale) and the pins ran on `4a51783e65`.
- The PP suites, registered and Stale, ran first on `4a51783e65` (`suites/round1/`) and again on the head, after the test-only reorder of `a8e719f5b4`. The two runs' outcomes are equal line for line.
- Between the commits, only `retained_facade_tests.rs` changed, and only in one test's statement order. So the runner, witness and pin results carry to the head.

## 6. The source-text guards (§1)

| Guard | Early (uncommitted change, = `4a51783e65`) | At the head |
|---|---|---|
| `PP-tests/s11f_site_test.rs`: rules 1–6 and 8, the scanner self-checks, `t8_*` and `t10b_*` (11 tests) | 11 passed | 11 passed |
| `PP-tests/retained_precision_admission.rs`, including `legacy_native_and_typed_call_graphs_cannot_acquire_retained_admission` (5 tests) | 5 passed | 5 passed |
| `RE/tests/retained_precision_carriers.rs`, including `u6_doc_hidden_seams_have_no_product_callers`, the P-tree product-text walk (16 tests) | 16 passed | 16 passed |
| **In-fence guards in PP's suite:**<br>• `u3_n9_single_parse_custody`, `u3_permitted_outputs_keep_the_report_and_gate_order`, `u3_capture_permit_is_linear` and `u3g2_no_permit_path_runs_once_without_a_copy`;<br>• `u1_serializer_reads_no_legacy_work_field`;<br>• the law tests that read `admit`'s source (`the_registered_profile_is_the_only_permit_source`, `admit_grants_a_permit_for_the_milestone_in_the_registered_build`);<br>• `challenge_bounds_are_the_profile` | pass | pass |

- **No guard's expected text changed.** No assertion was weakened.
- **No new rule-8 site,** so s11f's `TABLE` is unchanged. The seam's add is a checked add followed by a plain assignment. The classifier only filters and matches.
- **FK's `k2a_checked_formation.rs`** cites `lib.rs:8471-8475` at commit `5ae22926e` in a comment. It names its own historical commit, so it is unaffected; it is not a test assertion.

## 7. Mutants (`_run_records/mutants/`)

**Method.** I68's method: one textual edit each in `MUT/P/core/product_physics/src/lib.rs`, then PP's whole `--lib --no-fail-fast` suite, registered, in the target `WT/targets/i85-b1-st/mut`. The pristine bytes were restored and checked by sha256 (`84fba5ca…` each time). `t13` fails in every run, as it does on base, and is excluded.

| Mutant (PLAN_v2 §2.1) | Edit | Compiled | Killed by (first failing assertion of each test) |
|---|---|---|---|
| M1 keying on `initial` | `NotRequired` when the seed's `initial` is a report with code `NUMERICAL_INTEGRITY_CHECKS_PASSED`, not when the verdict is `checks_passed` | yes | the classifier test, "**W2-published Passed**"; the two-body B pin, the private driver's cause (`NoTriggeredCase`) |
| M2 verdict by position | `case_ids.iter().enumerate()…`, with the verdict `quality.cases.get(position)` | yes | the classifier test, "**request order, looked up by id**" |
| M3 dropping decision 21 | `seed.is_some_and(\|_\| false)` | yes | the classifier test, "Mechanism … Some(Failed), no W2"; the decision-21 test, "excluded: A is empty" |
| M4 a seedless case excluded | `seed.is_none_or(dn_trigger_excluded)` | yes | the classifier test, "**no seed**"; the decision-21 test, "no seed: in A, so W1 runs" |
| M5 `NoTriggeredCase` appending a notice | reserve the notice, then `notice.publish(ordinary, NoTriggeredCase)` | yes | the two-body B pin, "the exact ordinary bytes, no notice"; the decision-21 test, "excluded: the exact ordinary bytes" |

**5 of 5 killed by assertions, none by compilation.**

**Round 1** (`mutants/round1/mutants.json`, on `4a51783e65`) gave the same verdicts. M2 was then first refused by the not-unique assertion ("two entries for one case"), because it came first in the test. Commit `a8e719f5b4` moved the request-order block ahead of it, and M2 is now refused by the assertion the plan names for it.

## 8. Stale

- **The Stale PP suite** (`RUSTFLAGS=--cfg=i85_b1_st_stale`) equals the registered one, outcome for outcome, on base and on the head.
- **On the head in Stale:**
  - every Direct output is the plain bytes from one run (`ONE_RUN`), including two-body B's, the re-based W-C1 variant's and the C + 1 oracles';
  - the private driver's `NoTriggeredCase` outputs are the plain bytes: two-body B in the suite, and W6-PHYS-R4 and W2b in the Stale witness run (10 passed, the same lines as registered).

## 9. For ROOT

1. **No stop fired.** Nothing in §1's never-touched list or R8 was reached.
2. **The uniqueness rule is my reading.** "The published verdict" and "the seed" are taken to exist only when exactly one entry or seed matches the case id; otherwise the case is treated as having none, which puts it in A unless Passed. `ordinary_value` binds the verdict the same way. The classifier test pins the rule. RV-P may rule otherwise.
3. **Where the seam's add sits.** It is inside the permit branch, so a run without a permit (the private driver) keeps `late_loads_total` at 0. Only G-B, which runs only with a permit, will read it.
   - **On overflow** the capture records `CaptureError::CountRange("late loads total")` and skips G-B and the late capture. The later preparation then surfaces it, as a Preparation fallback with its notice. It is not a G-B refusal.
   - The overflow needs a `usize` sum of `Vec` lengths to wrap, so it is unreachable on a real input, and no test exercises it. SA or RV-P may prefer a typed G-B refusal there.
4. **Additions beyond PLAN_v2's explicit list,** all as "new pins":
   - the decision-21 test through `retained_w1`;
   - `witness_w6_phys_r4_input_no_triggered_case`, which makes "W6's PHYS-R4 input stays a `NoTriggeredCase` pin" an executable witness;
   - the private-driver half of the two-body B pin, so that T-4 is pinned in the Stale build too;
   - the native-fault sentinel for "no W1 work";
   - `force_scaled` asserted in W6;
   - the PROBE input sha256s asserted.
5. **Renamed or re-based witnesses for SQ's records.**
   - QUAL §4's W2b row names `witness_w2b_cap_maximal_solvable`, now `witness_w2b_cap_maximal_passed_report_no_triggered_case`.
   - `witness_w6_force_scaled` keeps its name but now runs case C.
   - G6's witness logs, Pass B's lists and QUAL_B1 should use the new names and inputs.
6. **Test-only visibility.** `w_c2_case_c()` and `W_C2_CASE_C_INPUT_SHA256` are `pub(super)` in `retained_facade_tests` (a `#[cfg(test)]` module), so that `retained_memory::witness_tests` can share case C's builder. No production (`src`) item's visibility changed. If RV-P prefers, the builder can be duplicated in the witness module instead.
7. **For SP.** `case_triggers` takes the request's case ids and is ready for n cases. SP's replacement of `w1_case_id` passes every id and keeps the `Domain` refusal of c = 0, c > C and combinations ahead of T-4. `late_loads_total` and `requested_cases` are as decision 22 states.
8. **For SA.**
   - The four facade oracles move to 4 cases automatically when `LOAD_CASES` becomes 3.
   - Two of them (`u3_no_permit_entries_are_the_ordinary_route`, `u3g2_no_permit_path_runs_once_without_a_copy`) duplicate the case id `case`, as they did before B1. They assert only "refused" or "the ordinary route", so a refusal at any clause satisfies them.
   - `u3g2_direct_entry_no_w1_refusals_keep_exact_bytes` asserts the refusal clause `D1.4` (Invocation) on distinct ids `case-2…case-4`.
9. **Budget:** about 1.6 h of agent time (02:40–04:15 UTC), much of it waiting for the lock, against 5–7 h.

## 10. Commands (cwd as stated; `C` = `env -u RUSTFLAGS -u CARGO_ENCODED_RUSTFLAGS TMPDIR=S/tmp CARGO_BUILD_JOBS=4 RUST_TEST_THREADS=2`)

1. **The base archive:** `GIT_OPTIONAL_LOCKS=0 git -C WT/b1 archive 47a3bdfcf5 projects/chirality-piping ':(exclude)projects/chirality-piping/execution' | tar -x -C S/base`.
2. **The candidate,** through `S/cargo_cand.sh` (`_run_records/scripts/`; target `WT/targets/i85-b1-st`), in `WT/b1/P/core/<crate>`:
   - `test --locked --offline --no-run` (rc 0);
   - the guards, `test --locked --offline --no-fail-fast --test s11f_site_test --test retained_precision_admission` (rc 0) and, in `reporting/result_export`, `… --test retained_precision_carriers` (rc 0);
   - `test --locked --offline --no-fail-fast --lib` (rc 101: `t13` only).
3. **Commit `4a51783e65`,** then the suites: `S/run_suites.sh cand_reg_witness base_reg_witness cand_reg_pp base_reg_pp cand_stale_pp base_stale_pp cand_stale_witness cand_reg_runner base_reg_runner`. Each job runs through `t3_cargo.sh` under a 7,200 s alarm. The targets are `…/i85-b1-st`, `…/base`, `…/stale` and `…/base-stale`.
4. **The pins:** `S/run_pins.sh`.
5. **The mutants,** round 1: `VENV/bin/python S/mutants.py MUT/P S/logs/mutants WT/targets/i85-b1-st/mut WT/tools/t3_cargo.sh S/tmp`, with `MUT` archived from `4a51783e65`.
6. **Commit `a8e719f5b4`.** `MUT` was re-archived from the head; then `S/run_suites.sh cand_reg_pp cand_stale_pp` and the mutants again. The guards ran again at the head.
7. **The records:** `S/write_records.sh`, which runs `sanitize.py`.

## 11. Records, cleanup and limits

- **`_run_records/`:**
  - `st.diff` and `commits.txt`;
  - `scripts/` (sanitized: `WT` and `VENV` replace the machine paths, so they are not runnable verbatim);
  - `guards/`;
  - `suites/` (the logs, `*.outcomes`, `*.warnings`, `round1/`; the runner logs filtered to cargo's Running, test, failure and result lines);
  - `witness/`, `pins/` and `mutants/` (with `round1/`; the mutant logs filtered);
  - `build/` (the first build log, the first lib run and the build identities);
  - `cargo_jobs_i85.log`.
- **Path and length handling.** Machine paths are replaced by `WT`, `VENV` and `~`. A log line over 4,000 bytes is cut to 1,000 bytes, followed by its length and sha256. Those lines are the producer's committed `cfg(test)` prints.
- **SHA256SUMS** covers RETURN.md and every file under `_run_records/`.
- **Kept for ROOT and RV-P:**
  - the full logs, `BASE` and `MUT` in `S`;
  - the targets `WT/targets/i85-b1-st{,/base,/stale,/base-stale,/mut}`, for RV-P's probe and SP.

  ROOT may delete them; I delete nothing outside my scratch.
- **Limits:**
  - **c = 1 only.** Multi-case behaviour is SP's (with A1-S-2: `Domain` until I2).
  - **Decision 21 is pinned on hand-built seeds only.** No audited committed input reaches it.
  - **The S1 witnesses** ran in the dev/test build as outcome checks. Their stack margins are SQ's measurement on the uninstrumented re-qualification build.
  - **The seam's overflow branch is untested** (§9 item 3).
