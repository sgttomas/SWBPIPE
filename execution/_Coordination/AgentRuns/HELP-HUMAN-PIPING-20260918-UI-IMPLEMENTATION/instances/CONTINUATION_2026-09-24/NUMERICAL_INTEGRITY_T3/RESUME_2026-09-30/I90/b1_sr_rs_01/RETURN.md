# I90 B1-SR-RS: the Rust reader's alignment (R-D38 (4b), F-1 text B per case, G8 per case, G5's `not_required` rule)

TASK (Type 2), I90 (I-RS), for ROOT (HELP_HUMAN, Agent 0), who is the return path. I made no delegation. 2026-10-07 UTC.

**Briefs (verified before work):** `R/BRIEFS/B1_COMMON.md` sha256 `2d170307516b98c40e8aa2dcf352cf11a13dbdd29aeddd78a4c39f3f15eb2c75` (as recorded verified by I85's and I86's returns; unchanged in Git since `3941121e14`, RR "B1's PLAN_v2 accepted …") and `R/BRIEFS/B1_SR_RS.md` sha256 `9c0bb8aca39dfdb58a638a01924dfa1f074a879a68a3c0f60dc6caaf0e4470e2` (as dispatched). The specification, `R/I84/b1_plan_01/PLAN_v2.md`, is `c85786b7…` (verified). I read NUM's `AGENTS.md` and `agents/AGENT_TASK.md`, PLAN_v2 §0–§2.5, §4 and §8, and DESIGN_v2 (`R/I78/b0_contract_01/DESIGN_v2.md`) §0, §2 and §3. No other role's instructions were consulted.

**State.** Done. Three commits on `codex/piping-t3-b1-r-20261007` in `WT/b1-r`, over I1 `262bd687f0`. Head **`cc81e78801`**. Not pushed; ROOT pushes and merges.

**Placeholders:** WT, NUM, P, PP (= `P/core/product_physics`), RE (= `P/core/reporting/result_export`), RS (= `RE/src/retained_precision.rs`), T, R and VENV as in the dispatch. Code is cited by symbol; line numbers are at the head.

## 0. In brief

| Item | Result |
|---|---|
| **The cascade census (R5)** | **Zero changes.** The aligned reader (commit `d7c76de43f`) over 07m (`c21112fd…6807`): all 294 mutations have the same observed first failure as on I1, and all 28 must-pass entries are admitted with their stated classifications and eligibility, on both sides. Re-run at the head: still zero (§2) |
| G8 widened to every case, with P2–P4 | Done (§3.2). P1 for every case; "exactly one parity row iff dense" replaced by P2–P4 |
| G5's three `not_required` conjuncts dropped | Done (§3.3) |
| D38's obligation `[r1: N-6]` | 18 checks listed; one relaxed to (4b), one extended by (4b)'s source equality, the rest shown not to apply (§4) |
| Unit tests on synthetic n-case receipts | 4 tests, derived from the shared two-case bases and must-pass entries (§5) |
| Module doc of `RE/tests/retained_precision_contract.rs` (RV97 R2-N-2) | Reworded (§3.4) |
| RV95 N-5's direct `#[cfg(test)]` test in `RE/src/source_blocks.rs` | Done, with `failure.block_order` (§3.5) |
| c = 1 byte identity through precommit | **Holds.** PP's c = 1 successor pins pass at the head; the six successor documents are byte-identical to I1's and to the committed fixtures (§6.2) |
| Source-text guards that read RE | Pass: RE's carrier test (17 of 17), PP's `reviewed_inputs_bind_the_lock_and_the_reader_statics` and `bindings_need_witnesses_inputs_and_reader_layouts`, and the PP-tests guards (§6.3) |
| Suites against I1 | RE 180 → 186 ok (+6 added tests, nothing else changed); PP 712/1/11 = 712/1/11; headless 85/2 = 85/2; outcome for outcome (§6.1) |
| Mutants | **25 run: 15 killed, 10 survived.** Every mutant the brief names is killed (G8's per-case loop, G5's three conjuncts restored, the relaxed D38 check restored, P2–P4); the 10 survivors are (4b)'s redundant conjuncts, equivalent at `validate` (§7) |
| Stops (§1 fence, R8) | **None fired.** Only fenced files changed; no layout change to `Validation`, `ValidationError`, `RowClassification` or `AccuracyClass`; no FK, schema, base-reader, reviewed-input or D1-visibility change; no c = 1 byte change; no weakened guard |

## 1. Commits and files

| Commit | Content |
|---|---|
| `d7c76de43f` | RS: R-D38 (4b), F-1 text B per case, G5 `not_required` (the census ran on exactly this reader) |
| `a4eab1dd01` | `RE/tests/retained_precision_contract.rs`: four B1 tests; the module doc reworded |
| `cc81e78801` | `RE/src/source_blocks.rs`: RV95 N-5's direct `#[cfg(test)]` module, appended |

`git diff --stat 262bd687f0..cc81e78801`: 3 files, +483/−26. Each file is in SR-RS's fence (PLAN_v2 §1):

| File | sha256 at I1 | sha256 at the head |
|---|---|---|
| RS | `4722b505…3f93b69` | `c6965da0…780d0f5` |
| `RE/tests/retained_precision_contract.rs` | `ef8b67ff…6489bd8` | `2476f61f…7130a9178` |
| `RE/src/source_blocks.rs` | `a9ee998a…3c96e534` | `e388416b…0a76a13e` |

The `source_blocks.rs` hunk is one appended module, wholly inside `#[cfg(test)]` (Pass B's class `test`). The full diff is `_run_records/sr_rs.diff`.

## 2. The cascade census (R5)

**Method** (`_run_records/census/`). RE's contract test already prints, for each shared mutation, the reader's observed first failure (`slice_outcomes`: one JSON line per mutation, with `observed`, over slices that cover 0..294), and, for each must-pass entry, whether it is admitted with the base's classifications and the stated eligibility (`I63_MUST_PASS <id> <same>`). I ran RE's whole suite with `--nocapture`:
- **base:** I1 (an archive of `262bd687f0`, `P` only), before any edit;
- **aligned:** the aligned reader, commit `d7c76de43f` (RS only; no test change), the 07m corpus `c21112fd…6807` unchanged.

`census.py` compares the two logs entry by entry against 07m's id lists.

**Result: `CENSUS mutations 294 (base lines 294, aligned lines 294); must_pass 28 (base 28 true, aligned 28 true); changes 0`.** Every mutation's observed (gate, code) is the same on both sides, and every one still equals its expectation; every must-pass entry is admitted as stated. The 17 bases also validate on both sides with their shared eligibility and classifications (`complete_synthetic_controls_carry_their_shared_eligibility`). RE's test list is identical on both sides (180 ok).

**At the head** (`cc81e78801`, with the B1 tests): the same comparison again reports 0 changes (`census_head.out`).

This agrees with I84's static reading (`R/I84/b1_plan_01/_run_records/cascade_static_07m.out.txt`): no 07m entry edits a non-selected case's mode or parity row, or reaches R-D38's (4b) branch. 07m's `not_required` cases (07j's must-pass entry and mutation 277, built on it) are Passed reports with W2 untriggered, which the old and the new rule both admit.

## 3. The changes

### 3.1 R-D38 (DESIGN_v2 §2), in `g5_products`

Before: `if st.native != not_entered { pf(!run_ref.is_null() && preparation == completed) }`, R-D38's rule 4 without (4b).

After: when the native stage **failed and `run_ref` is null**, the attempt must satisfy `d38_capture_before_run` (G5 `PRODUCT_ATTEMPT_MISMATCH`); otherwise the old rule (4a) applies unchanged. `d38_capture_before_run` checks (4b)'s local conjuncts:
- `result` is unavailable with `error.kind == "capture"`;
- the stage record is done(1, [failed]): preparation completed, every stage after native `not_entered`;
- the case is unavailable, its cause `prepared_product_failure` names this attempt, and its reason is (`source_unavailable`, `preparation`);
- `source_ref` is non-null and **equals `case.source_ref`** `[r01: N-6]` (new in RS; TS already had it).

The rest of (4b) already holds at the call site or across the receipt, and the function's doc says where:
- `run_ref` null is the branch; the case's Run is then null (`if run_ref.is_null() { pf(case.run.is_null()) }`), and `proof` is null (a non-null proof with no `run_ref` is refused just above);
- a non-null `source_ref` resolves to a CaseSource whose `preparation.attempt_ref` is this attempt (checked above for every attempt);
- "no Run, Call or Group `source_refs` entry, Build or `execution_order` entry names this case or that source" is enforced for every receipt by G3 and G5's native class (§4, items 8 and 9).

`(native == completed) == (Run selected)` still follows; with no Run it holds for a failed native stage.

### 3.2 F-1 text B in G8 (DESIGN_v2 §3.2–§3.3)

In `g8`'s per-case loop (request order), after the requested-mode and material-basis check:
- **P1** (exactly one `linear_solver_mode_basis` row, valued 1 in `sparse_interactive` and 2 in `dense_scrutiny`) now runs **for every case**, not only for selected ones;
- "exactly one parity row iff dense" (selected only) is replaced, **for every case**, by **P2** (at most one parity row), **P3** (none in `sparse_interactive`) and **P4** (none when the case's ordinary `w2.kind == "published"`). A dense b = 0 case without a parity row is admitted (the disclosed limit).

`o` is `ordinary_attempts[i]`; G3 already binds `cases[i].ordinary.attempt_ref == i`, so it is DESIGN's `ordinary_attempts[case.ordinary.attempt_ref]`. All four use `RETAINED_PRECISION_PREPARATION_MISMATCH`. P5 is not implemented (deferred, decision 10).

**Order inside G8.** RS's per-case loop computes the case's material selector, then checks the requested mode and material basis, then P1, then P2–P4; its `material_bases` loop runs after the per-case loop. DESIGN lists the material checks before the loop. Every one of these is G8 `PREPARATION_MISMATCH`, so the first failure's (gate, code) is the same in either order; I did not reorder RS's existing checks.

### 3.3 G5's `not_required` rule (DESIGN_v2 §3.3, decision 9), in `g5_ordinary`

The rule is now `product_attempt_ref` null and verdict `checks_passed`, with the existing `initial.kind != "not_attempted"` check just above it. The three conjuncts `initial.kind == "report"`, `initial.outcome == "checks_passed"` and `w2.kind == "not_triggered"` are dropped. RS keeps its report-outcome equality with its `w2 == not_triggered` guard (equivalent under D6c, as DESIGN §3.3 says).

### 3.4 The module doc (RV97 R2-N-2)

It now reads: "Shared statement controls: synthetic bases, plus the two listed producer-solved L = 0 bases (07l), which the Direct entry published in the registered dev/test build (their own `provenance` and `qualification` say so), and B1's reader-local n-case receipts derived from the synthetic bases. Reading them establishes no execution; no native Current evidence." The old "These do not establish execution" understated, as RV97 noted.

### 3.5 RV95 N-5's direct test (I74 decision 9; RV101 NT-1 and A2-N3)

`mod rv95_n5_integer_tests`, appended to `RE/src/source_blocks.rs` under `#[cfg(test)]`:
- `integer_admits_two_to_53_minus_1_and_refuses_two_to_53`: `integer` admits 0 and 2^53−1, and refuses 2^53, 2^53+1 and `u64::MAX` with `SOURCE_BLOCKS_INTEGER`. This kills RV95's S1 (§7).
- `every_field_read_through_integer_refuses_two_to_53`:
  - a census of the production text's `integer(&…)` calls: their arguments are exactly the expected 15 expressions. These read the **13 receipt fields** (I76's twelve and **`failure.block_order`**) and the four `summary` counts (`summary[key]`). A new call site, or a lost one, fails the test;
  - each of the 13 receipt fields, placed at its receipt path at 2^53 and read through `integer`, is refused; at 2^53−1 it is admitted;
  - where a private check reaches `integer` before any other layer, it is exercised there: `source_plan`'s four counts (2^53 → `INTEGER`; 2^53−1 → `SOURCE_COUNTS` or `FUNCTIONAL_COUNT`), and `summary`'s four counts (2^53 → `INTEGER`; 2^53−1 → `SUMMARY_MODEL_COUNTS`).

The public-API masking test (`RE/tests/source_blocks.rs`) is unchanged.

## 4. D38's list `[r1: N-6]`: every RS check that assumes a prepared source has a Call or a Run

The (4b) shape: an unavailable case whose attempt prepared a registered CaseSource and whose native stage failed before any Run; no Run, Call or Group entry, Build or `execution_order` entry names it.

| # | Check (function) | What it assumes | Disposition |
|---|---|---|---|
| 1 | `g5_products`: an entered native stage requires `run_ref` non-null and preparation completed | an entered native stage has a Run | **Relaxed to (4b)** (§3.1). Restoring it is mutant D38-1 |
| 2 | `g5_products`: `a.source_ref == case.source_ref` was checked only on the Run branch (`run_ref` non-null) | the source binding goes through a Run | **Extended:** (4b) requires the equality (m8). Dropping it is mutant D38-2. The null-Run branch is otherwise unchanged: preparation failures and captured prefixes have no source |
| 3 | `g5_products`: a non-null `proof` requires `run_ref` and a selected Run | a proof follows a selected Run | Does not apply: (4b) has no proof. Unchanged |
| 4 | `g5_products`: native completed ⇒ Run selected; `(native == completed) == (Run selected)` | — | Does not apply / holds: (4b) is native failed, and an absent Run reads as not selected |
| 5 | `g5_coverage`: a complete coverage vector requires the attempt's own selected Run | a vector follows a Run | Does not apply: with no proof it returns first |
| 6 | `reason_table` (D4d): `native` requires the case's non-selected Run; `capture` with no Run gives (`source_unavailable`, `preparation`) | — | Already admits (4b) by its `capture`-without-Run arm. Unchanged |
| 7 | `error_stages` (D37): `capture` ⇔ native failed, or a later capture point | — | Already admits (4b) (first failed stage = native). Unchanged |
| 8 | `g5_native` (class 1): every Run in one Call position, in order; each position's source and owner are its Run's own case's; Group sources ⊆ its Call's sources, and C5 group formation over Runs; every Build built by a record; `charged` = the running meter | constraints on the Runs, Calls, Groups and Builds that exist | Does not apply: no check requires a source to appear in a Call or Group. These checks are what enforce (4b)'s "nothing names it" clause (m7 fails here, G5 `ATTEMPT_MISMATCH`). Not relaxed |
| 9 | `g3`: Run ids = execution positions; `execution_order` = the Runs' owners | keyed on `case.run` | Does not apply; it enforces (4b)'s `execution_order` clause (m4 fails here, G3 `COVERAGE_MISMATCH`) |
| 10 | `g3`: an attempt's member ids against its resolved source's member map; a coverage vector against the source's bodies (only with a vector) | — | No Run or Call assumption; holds for (4b) |
| 11 | `g3` (D29): every CaseSource has a non-empty body inventory | — | No Run assumption; holds |
| 12 | `g1`: the preparation digest of each source whose attempt's members are all prepared; `source_identity_sha256` (selected cases only, by the schema) | — | No Run assumption; holds |
| 13 | `g5a` (I57 s4): an unavailable case's attempt with a complete vector is checked through its Run's last attempt and verification record (`coverage_g5a` reads `case.run`) | a vector follows a Run | Does not apply: (4b) has no vector (item 5) |
| 14 | `numeric_cases`, `g5a_selected`, `g5b`, `g5c`, G6 row methods | selected cases | Does not apply |
| 15 | `g8`'s source loop: binds every CaseSource to its owner case from the invocation and recomputes `kernel_source_sha256` and `stiffness_sha256` from the binding (`verify_native_source_hashes`) | — | No Call needed (DESIGN §2's S-2 reading: C2 §3's digests are raw sha256 of the source's own encodings); holds |
| 16 | `g8`'s attempt loop: binds attached members' sections to the attempt's source section terms when `source_ref` is non-null | — | No Run assumption; holds |
| 17 | `g5_ordinary` O5: a `source_decline` case has no source and no Run | — | Does not apply: (4b) has a source and no decline |
| 18 | `validate`'s eligibility: every case selected or not_required | — | (4b)'s case is unavailable, so the statement is not eligible; standing `needs_recompute` (tested) |

**Redundant (4b) conjuncts.** Of `d38_capture_before_run`'s eleven conjuncts, only the source equality is enforced nowhere else. The other ten repeat checks that later G5 `PRODUCT_ATTEMPT` checks in `g5_products` also make, each with the same gate and code. I kept them so that (4b) reads as one predicate, as DESIGN writes it, and does not silently widen if a later check changes. Their mutants are equivalent at `validate` (§7), and the table there names the check that catches each.

## 5. The unit tests (`RE/tests/retained_precision_contract.rs`)

They are reader-local, on receipts derived from the shared bases by edits and a full reseal (the file's own `apply_entry` and `rehash`). They are not shared corpus entries; SC's 07n pins those.

1. **`b1_d38_capture_before_any_run_beside_a_selected_case`.** On `two_case_facade_after_certificate_synthetic`, case 1 is rewritten into (4b), as SC's `d38_beside_selected` will rewrite W-C2's case C:
   - its Run, `execution_order` entry, and Call and Group entries are removed;
   - the call's after-value and `charged` are recomputed (case 1's Run built nothing, which the test asserts);
   - the cause is a typed `CaptureError::Origin`, and every hash is resealed.

   It validates with G0–G8 passing, not eligible, and standing `needs_recompute`. Sixteen variants are refused, each at this reader's first failure:

   | Variant | RS first failure |
   |---|---|
   | m1: `error` set to `{kind: native, run_ref: 1}` | G5 `PRODUCT_ATTEMPT` |
   | m2: native `completed` | G5 `PRODUCT_ATTEMPT` |
   | m3: `run_ref` 1 while the case has no Run | G5 `PRODUCT_ATTEMPT` |
   | m4: `execution_order` still lists the case | G3 `COVERAGE` |
   | m5: `proof_start` completed | G5 `PRODUCT_ATTEMPT` |
   | m6: `source_ref` null, preparation completed | G5 `PRODUCT_ATTEMPT` |
   | m7: the case's source in the call's `source_refs` | G5 `ATTEMPT` |
   | m7 (group): in the group's `source_refs` | G5 `ATTEMPT` |
   | m8: `case.source_ref` = 0 ≠ the attempt's 1 | G5 `PRODUCT_ATTEMPT` |
   | result `ready`; preparation `failed`; observables and G5a entered; reason code `kernel_unresolved`; reason phase `kernel`; cause naming the other attempt; cause `receipt_failure` | G5 `PRODUCT_ATTEMPT` each |

2. **`b1_g5_not_required_admits_a_w2_published_case`.** On 07j's must-pass entry (selected, then not_required), case 1 is made W2-published with the verdict `checks_passed`, by an evaluation trigger (`structural_failure`/`range`) and by a formation trigger (`formation_failure`/`numerical_range`). Each is admitted and **eligible**, `reader_logic::ordinary` returns `Ok`, and the standing is `numerically_eligible`. Still refused:
   - W2-published with verdict `sensitive`: G5 `ATTEMPT`;
   - `initial` `not_attempted`: G5 `ATTEMPT`;
   - a report whose outcome differs from the verdict: G5 `ATTEMPT` (the kept equality);
   - `product_attempt_ref` non-null: **G3 `COVERAGE`** (it names case 0's attempt, which G3 refuses first).
3. **`b1_g8_mode_row_and_requested_mode_for_every_case`.** Each is refused at G8 `PREPARATION`:
   - on F_BASE's unavailable case 1: mode code 2 in sparse; mode code 3; two mode rows; the requested mode flipped;
   - on 07j's not_required case: mode code 2 in sparse.
4. **`b1_g8_parity_rows_p2_to_p4_for_every_case`.** 07j's two-case statement is made dense (the invocation's mode, both requested modes and both mode rows; it has no parity row).
   - **Admitted:** the dense selected case at b = 0 **without** a parity row (before B1 RS needed exactly one); one parity row on the not_required case at b = 0; a W2-published not_required case without a parity row.
   - **Refused at G8 `PREPARATION`:**
     - P2: two parity rows on the not_required case, and on the dense base's selected case;
     - P4: a parity row on the W2-published not_required case, and on a W2-published selected case (`ordinary_prepared_dense_synthetic`);
     - P3: a parity row on F_BASE's sparse unavailable case.

## 6. Evidence per item

### 6.1 Suites against I1 (`_run_records/suites/`)

Each ran `cargo test --locked --offline --no-fail-fast` through `WT/tools/t3_cargo.sh`, with `RUSTFLAGS` unset (registered), `CARGO_BUILD_JOBS=4` and `RUST_TEST_THREADS=2`. Base used an archive of I1 (`P` only, plus src-tauri's `src` for PP-tests' source guard) in `WT/targets/i90-b1-sr-rs/base`. The candidate was `WT/b1-r` at the head, in `WT/targets/i90-b1-sr-rs`.

| Suite | I1 | Head | Per-test difference |
|---|---|---|---|
| RE (`result_export`, all targets) | 180 ok | 186 ok | **+6 new (ok):** the four B1 tests (§5) and the two N-5 tests (§3.5). Nothing else |
| PP (all targets) | 712 ok, 1 failed (`s11g_tests::t13_committed_fallback_uz_is_byte_identical`, the known Mac `t13`), 11 ignored | the same | none (724 outcome lines identical) |
| runner/headless | 85 ok, 2 failed (the known `load_reference` pair) | the same | none |
| PP pins (§6.2) | 3 ok | 3 ok | none |

Compiler warnings: the same lines and counts on both sides (RE 2, PP 18, headless 15; RE's is the existing `derived` never used).

### 6.2 c = 1 byte identity through precommit

RS is compiled into PP for precommit. At the head, PP's c = 1 successor pins pass:
- `u1_milestone_successor_both_modes`;
- `u3_permitted_path_publishes_the_pinned_successor`;
- `u3g2_direct_entry_publishes_the_pinned_successor`;
- `u3g2_d_u6_5_carrier_fixtures_are_the_live_successors`;
- `u3_r1_carrier_names_exactly_one_publication`;
- `u8_l0_isolated_node_publishes_pinned_successor`;
- `u8_d_u6_5_l0_fixtures_are_the_live_successors`.

**A direct comparison** (after I85's `run_pins.sh`; `_run_records/pins/`). The three publishing pin tests' own output variables (`I61_U3_OUT`, `I61_U3G2_OUT`, `I68_U8_OUT`) wrote the successor documents on I1 and on the head:

| Document | sha256 (I1 = head) |
|---|---|
| milestone, sparse (U3 private driver and U3G2 Direct) | `ac6986b0680e0df9d88c33a5bf4635372fc3b83cbb9080da44e6d32dbdca59dc` |
| milestone, dense | `6cd1d249e5352aaffbd2b7d7349c74a1d0e0572df35500f66be49c3cad95c9b5` |
| L = 0, sparse | `93c6c86548b9d263cba9d9869010043d23ed1c9f2f304dd9f82eb705eb350876` |
| L = 0, dense | `dbb3d477364248fb9ae15f7b9cff44410c2bffe45f783dd02d96eca663b0ac88` |

- All 6 files are byte-identical between I1 and the head.
- Each equals its committed fixture (`P/fixtures/results/retained_precision_{milestone,l0}_successor_*.json`), and the hashes are I85's (`R/I85/b1_st_01/RETURN.md` §4).

### 6.3 Source-text guards that read RE (PLAN_v2 §1)

- **RE's carrier test** `RE/tests/retained_precision_carriers.rs`: 17 of 17 ok at the head.
- **PP's law tests that read RS and `source_blocks.rs`:** `retained_memory::law_tests::reviewed_inputs_bind_the_lock_and_the_reader_statics` and `bindings_need_witnesses_inputs_and_reader_layouts` are ok. The reviewed inputs and `READER_LAYOUTS` are unchanged.
- **PP-tests' guards** (`s11f_site_test.rs`, `retained_precision_admission.rs`): all ok, unchanged.
- FK's `k2a_checked_formation.rs` is untouched.

### 6.4 G8 and G5 per DESIGN §3.3 on synthetic receipts

See §5, tests 2–4, and the mutants in §7.

## 7. Mutants (`_run_records/mutants/`)

Each mutant was applied in a scratch archive of the head, `WT/scratch/i90_b1_sr_rs/mut`. Each ran one cargo job: `test --locked --offline --no-fail-fast --test retained_precision_contract --lib`, which covers RE's contract test and RE's lib tests. The file's bytes were restored after each, and the programme checks the restoration (`mutants.py`, `results.jsonl`, one log per mutant).

The scope is 95 tests per run.

| Mutant | Result | Killed by |
|---|---|---|
| **G8-1** P1–P4 for case 0 only (the per-case loop narrowed) | killed | `b1_g8_mode_row_and_requested_mode_for_every_case`, `b1_g8_parity_rows_p2_to_p4_for_every_case` |
| **G8-2** P1–P4 for selected cases only (B1's widening undone) | killed | the same two |
| **G8-3** P1 dropped | killed | `b1_g8_mode_row_…` |
| **G8-4** P1's "exactly one" dropped (the first row's value only) | killed | `b1_g8_mode_row_…` |
| **G8-5** P2 dropped | killed | `b1_g8_parity_rows_…` |
| **G8-6** P3 dropped | killed | `b1_g8_parity_rows_…` |
| **G8-7** P4 dropped | killed | `b1_g8_parity_rows_…` |
| **G8-8** the old "exactly one parity row iff dense" restored | killed | `b1_g8_parity_rows_…` |
| **G5-1** `initial.kind == "report"` restored | killed | `b1_g5_not_required_…`, `b1_g8_parity_rows_…` |
| **G5-2** `initial.outcome == "checks_passed"` restored | killed | the same two |
| **G5-3** `w2.kind == "not_triggered"` restored | killed | the same two |
| **D38-1** the relaxed check restored (`run_ref` required whenever native was entered) | killed | `b1_d38_…` |
| **D38-2** (4b)'s source equality dropped | killed | `b1_d38_…` (m8) |
| **S1** `integer`'s 2^53−1 bound removed (RV95's S1) | killed | both N-5 tests |
| **S1b** the bound off by one (`<`) | killed | both N-5 tests |
| D38-3 (4b) `source_ref` non-null dropped | survived, equivalent | `g5_stages`: preparation completed ⇔ a source (`PRODUCT_ATTEMPT`) |
| D38-4 result unavailable dropped | survived, equivalent | a Ready result needs every stage completed and a proof (`PRODUCT_ATTEMPT`) |
| D38-5 `error.kind == "capture"` dropped | survived, equivalent | `reason_table` (`native` needs a Run; `preparation` a failed preparation) and `error_stages` (`PRODUCT_ATTEMPT`) |
| D38-6 preparation completed dropped | survived, equivalent | `g5_stages`' stage order: an incomplete preparation leaves native `not_entered` (`PRODUCT_ATTEMPT`) |
| D38-7 the later stages `not_entered` dropped | survived, equivalent | the proof-null rule just above (`PRODUCT_ATTEMPT`) |
| D38-8 the case unavailable dropped | survived, equivalent | D19: an unavailable result's case is unavailable with a cause naming it (`PRODUCT_ATTEMPT`) |
| D38-9 the cause's kind dropped | survived, equivalent | D19 (`PRODUCT_ATTEMPT`) |
| D38-10 the cause's attempt dropped | survived, equivalent | D19 and D4c (`PRODUCT_ATTEMPT`) |
| D38-11 reason code dropped | survived, equivalent | `reason_table`'s `capture`-without-Run arm (`PRODUCT_ATTEMPT`) |
| D38-12 reason phase dropped | survived, equivalent | the same (`PRODUCT_ATTEMPT`) |

**The survivors.** For each, the D38 test's matching variant (§5, test 1) is still refused at G5 `PRODUCT_ATTEMPT_MISMATCH`, by the later check named in the table. That is why the mutant survives.

**The corpus.** None of these mutants changes a 07m outcome. The census shows that 07m does not exercise B1's rules; SC's 07n will.

## 8. Host and limits

- Every cargo ran through `WT/tools/t3_cargo.sh`, with `--locked --offline`. My jobs are in `_run_records/cargo_jobs_i90.log` (extracted from `WT/guard/cargo_jobs.log`).
  - One early wait of mine (`pid=88006`, a base run in `WT/b1-r` itself) was stopped by me while it was still waiting, before it took the lock. I moved the base to an archive so that I could edit while it ran. That `WAIT` line has no `START`.
  - I killed no other job.
- **Targets:** `WT/targets/i90-b1-sr-rs` (head), `…/base` (I1 archive) and `…/mut` (mutants; I deleted it, and the mutant archive, after the programme; their logs are recorded). The head and base targets (7.7 GB) are kept for a repair round. **Scratch:** `WT/scratch/i90_b1_sr_rs` (the base and mutant archives, logs, pins). Nothing went to the system temp directory (`TMPDIR` was in scratch).
- No DEC-025, no evidence sweep, no native or solver-at-scale job beyond PP's own suite and pins, and no installs.
- **Git:** commits only on my branch. Reads used `GIT_OPTIONAL_LOCKS=0`. The worktree is clean at the head.
- **A temporary probe.** Before writing §5's tests, I ran one uncommitted exploration test in `WT/b1-r`'s contract test file to see RS's verdicts on candidate edits. I removed it, and restored the file from Git before writing the committed tests. The census and every suite ran without it.
- **Records:** no record folder is named `build`, and every write used an absolute path (ROOT's host rules of today).
- **Not run:** PY's and TS's suites (their alignment is SR-PY's and SR-TS's), the src-tauri suite and the full 40-manifest suite (ROOT's, §3.9), and PP's ignored `witness_` set.

## 9. For ROOT

1. **The census is clean (R5 does not fire).** SR-RS can proceed to RV-R.
2. **For SC's 07n expectations: RS's first failures on my synthetic analogues** (the shared entries' expectations come from the three readers' agreement):
   - **m4** fails at G3 `COVERAGE` (`execution_order` ≠ the Runs' owners);
   - **m7** fails at G5 `ATTEMPT` (class 1: the call's position triples, or a group source outside its call), not `PRODUCT_ATTEMPT`;
   - **m1** must replace the whole error value. Setting only `error.kind` to `native` leaves `cause`, which the schema's `native` variant does not have, so RS fails at **G1** `RECEIPT_MISMATCH`. With `{kind: native, run_ref: …}`, RS fails at G5 `PRODUCT_ATTEMPT`;
   - **`not_required`'s "`product_attempt_ref` set non-null"** fails at **G3** `COVERAGE` in RS when it names another case's attempt (G3's product-attempt ownership check);
   - a parity row copied onto a **non-selected** case must drop `recovery_method`, or RS fails first at G6 `ROW_METHOD`. W-C2's dense case A is selected, so F-1's P4 entry is unaffected.
3. **For SQ's TEXT loop rules (PLAN_v2 §3.1, N-16), RS's new or changed loops on the D1 call graph:**
   - G8's per-case loop now counts parity rows (a filter over the case's rows) for **every** case. It was for selected cases only; the mode-row filter already ran for every case;
   - `d38_capture_before_run` scans 8 constant stage names, only for an attempt whose native stage failed with no Run;
   - `g5_ordinary`'s `not_required` check lost three comparisons; no loop changed.
4. **(4b)'s redundant conjuncts** are kept deliberately (§4). Their mutants are equivalent; RV-R may prefer the minimal form (the source equality alone), which is behaviour-identical at `validate`.
5. **Re-qualification:** RS's production code changed (commit `d7c76de43f`). Per DESIGN §2 and §3.3, one re-qualification covers D38, F-1 and `not_required` (SQ's G5 at I5). The `source_blocks.rs` hunk is test-only.
