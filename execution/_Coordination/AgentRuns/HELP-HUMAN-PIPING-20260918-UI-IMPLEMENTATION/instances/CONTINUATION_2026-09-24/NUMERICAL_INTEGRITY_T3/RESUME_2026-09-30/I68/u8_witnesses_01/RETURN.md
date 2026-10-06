# I68 U8-1: PP's witness tests (Part 2 of `BRIEFS/I68_U8_PROBE_AND_WITNESSES.md`)

TASK (Type 2), I68, for ROOT (HELP_HUMAN, Agent 0). 2026-10-06 UTC.

**Authority:** ROOT's ruling "I68's probe verified: L = 0 publishes; W-C1 is Ceiling; F-1 routed to B0 and B1; Part 2 granted", committed on NUM at `7c2ce2637f`, plus the dispatch message. The briefs are unchanged: `U8_COMMON.md` `3146c3e6…`, `I68_U8_PROBE_AND_WITNESSES.md` `5ef6f76a…`. The basis is Part 1's record `R/I68/u8_probe_01/PROBE.md` (`ba5f7df6…`).

**Placeholders:** `WT`, `NUM`, `P`, `R` as in the dispatch; `PP` = `P/core/product_physics`; `CAND` = `WT/f2a-u8` (U8 head `b1e2d7741e` plus the uncommitted candidate); `BASE` = `WT/scratch/i68_u8_witnesses/base` (a `git archive` of `b1e2d7741e`, `P` only); `MUT` = `WT/scratch/i68_u8_witnesses/mut` (BASE plus the candidate's three files).

**Host rules kept:**
- Left uncommitted; no Git writes (reads used `GIT_OPTIONAL_LOCKS=0`).
- Every cargo job ran through `WT/tools/t3_cargo.sh`, with `--locked --offline`, `CARGO_BUILD_JOBS=4`, `RUST_TEST_THREADS=2`, and RUSTFLAGS unset except in the Stale runs. The memory guard was up. The jobs interleaved with I73's under the lock.
- No DEC-025, native-app or solver-at-scale job.
- Scratch was under `WT/scratch/i68_u8_witnesses/`.
- No producer, reader or other production change.
- `retained_memory_witness_tests.rs` was not edited.
- No test pins F-1.

## 1. Changed files (all within the fence)

| File (in CAND) | sha256 | Change |
|---|---|---|
| `PP/src/retained_facade_tests.rs` | `2a1229b5a860489ee3616eb180a8f65cd1e1c6fc1048461b55c22ccf4e3316c1` (base `14d9ab46…`) | **+244 lines, 0 deleted.** A new section "U8 (I68)" at the end of the file, after the last existing test. No existing line is touched |
| `P/fixtures/results/retained_precision_l0_successor_sparse_interactive.json` (new) | `93c6c86548b9d263cba9d9869010043d23ed1c9f2f304dd9f82eb705eb350876` | 226,746 B. Written by the L = 0 test under `I68_U8_OUT` |
| `P/fixtures/results/retained_precision_l0_successor_dense_scrutiny.json` (new) | `dbb3d477364248fb9ae15f7b9cff44410c2bffe45f783dd02d96eca663b0ac88` | 228,096 B. Written the same way |

`git status` in CAND shows exactly these three paths (`_run_records/candidate_status.txt`). The diff is `_run_records/candidate.diff`. The two fixtures are untracked, so they are not in that diff; their hashes are above.

## 2. What the new section contains

**Inputs, derived inline from the milestone request:**
- `u8_first_load_only`: RV93 N-5's Candidate input.
- `u8_tiny_spring`: the Preparation input. Support 1's stiffness is 1e-300, so the ordinary run does not solve, as recorded in the ruling.
- `u8_two_body_case_a`: W-C2's model, kept for B1. It is the milestone plus W6's body moved to x = 5..6, with its material and anchor.
- `u8_two_body_case_b`: **W-C1's input**, case A's model with W6's tip force and torque on body 1 only.
- `u8_l0_isolated_node`: the probe's L = 0 derivation, N2 at (3, 0, 0) and `rigid:N2` with six restraints and no family.

**Tests:**
1. **`u8_real_input_fallbacks_append_one_notice`** covers `first_load_only` → Candidate, `tiny_spring` → Preparation and two-body case B → Native, in both modes.
   - **Registered build:** no hook armed before; admission refusal `None`; the expected cause; no successor; `ONE_RUN_THROUGH_G_C`; **the bytes equal `with_notice(plain, case, None)`** (checked before the count, so a dropped notice fails this byte assertion); `notices == 1`; no hook armed after.
   - **Any other build:** no W1; `ONE_RUN`; the plain bytes; no hook armed after.
   - The kernel reason Ceiling is recorded by the probe and not asserted, per decision 3.
2. **`u8_l0_isolated_node_publishes_pinned_successor`**, in both modes.
   - **Registered build:**
     - B′ (the envelope equals the plain run);
     - admitted;
     - `ONE_RUN_THROUGH_G_C`;
     - the successor is published, and `successor()` agrees;
     - **the pins:** the document sha (id `u8_l0_isolated_node_<mode>`, in U1's form), the receipt sha and the published-bytes sha;
     - the one publication is the successor;
     - the value controls (below);
     - writes the fixture under `I68_U8_OUT`.
   - **Any other build:** B′, no W1, `ONE_RUN`, the plain bytes.
   - **Pins:**
     - sparse: file `93c6c865…`, receipt `c00cbe76954e5188…`, bytes `9b425066029969b9…`;
     - dense: file `dbb3d477…`, receipt `0b4250c8139ba25a…`, bytes `5d84fce64bb0810d…`.
     
     The receipt and byte pins equal the probe's observations. The file pins were read from the test's first runs (`_run_records/bootstrap_pins.log`).
3. **`u8_d_u6_5_l0_fixtures_are_the_live_successors`.** Each fixture is byte-identical to the live successor document, and its file and receipt sha256 equal the pins.
   - In the registered build the document comes from the actual Direct entry.
   - In any other build it comes from the private driver, as the milestone's D-U6-5 test does. The Stale run passed this branch, so the private driver's L = 0 successor equals the Direct entry's.

**The value controls** (`u8_l0_value_controls`, in the registered build, each mode). The Rust reader's `validate` runs with the invocation on both the L = 0 successor and the pinned milestone document.
- **Body 1** (the 15 rows of `N2` and `rigid:N2`):
  - every value is exactly +0;
  - every class is `InputDerived` or an exact zero (`AbsoluteVerified { bound_bits: 0 }` with normalized and scale bits 0), counted as **6 and 9**;
  - the receipt gives body 1 `body_membership {members: [], nodes: [2]}`, `summary_coverage {has_data: false, stop: [F,F,F,F]}` and zero `body_scales`.
- **Body 0:**
  - the row count is the milestone's plus 15;
  - **every milestone row is present and bit-identical** (as observed);
  - it carries **the milestone's class claim** (class and bound bits equal);
  - it agrees within **the unchanged U5 criterion**, with the milestone's U5-verified rows standing as the reference. That criterion is: relative-verified within |v|/1e9, absolute-verified within its published bound, input-derived exactly (`R/I61/u5_reference_01/_run_records/u5_compare.py:104–113`).
  
  See §6, item 1 for this reading.

## 3. Test outcomes

- **Registered build** (identity equal to `REGISTERED_PROFILES`' text; `_run_records/build_identities.txt`): all three U8 tests pass in both modes. See `u8_tests_registered.log` and the full suite below.
- **Stale build** (`RUSTFLAGS=--cfg=i68_u8_stale`, separate target `WT/targets/i68-u8/stale`; identity `rustflags=--cfg%3Di68_u8_stale`): all three pass by their unregistered branches.
- **Controls:**
  - the milestone still publishes its pinned successor: `u3g2_direct_entry_publishes_the_pinned_successor` passes unchanged, registered and Stale;
  - `u3g2_d_u6_5_carrier_fixtures_are_the_live_successors` passes unchanged.
- **Extra check, not required:** the accepted Python reader passes both new fixtures with their invocations, eligible. Sparse is 25/78/9/1, dense 25/78/9/2, with publication hashes equal to the probe's (`py_readers_fixtures.log`). Python ran on BASE's reader with the CLI authorities built in Part 1. Corpus parity across the three readers is U8-2/U8-3's job.

## 4. Mutants (MUT tree; PP `--lib --no-fail-fast`, registered build; `_run_records/mutants/`)

Each mutant is one textual edit. The pristine bytes were restored after every run and checked by sha256. Every mutant compiled and **was killed by an assertion**. The base failure `t13` fails in every run, as it does on base, and is excluded.

| Mutant | Edit | Killed by (the new test's assertion first) |
|---|---|---|
| M1 restore all three loads | `u8_first_load_only` keeps the milestone's loads | `u8_real_input_fallbacks…`: "first_load_only SparseInteractive: **the real input's cause**". Only the new test kills it |
| M2 Candidate without notice | `retained_w1`: the Candidate fallback returns without `notice.publish` | `u8_real_input_fallbacks…`: "first_load_only …: **the ordinary bytes, then the notice**"; also `u3_each_stage_fault…` and `u3g2_direct_entry_w1_fallbacks…` |
| M3 Preparation without notice | the same, for Preparation | the new test, "tiny_spring …: the ordinary bytes, then the notice"; also the two hooked tests |
| M4 Native without notice | the same, for Native | the new test, "w_c1_two_body_case_b …: the ordinary bytes, then the notice"; also the two hooked tests |
| M5 L = 0 body-1 row corrupted | the test's successor gets `result:disp:N2:ux` = 5e-324 before the pin | `u8_l0_isolated_node…`: "sparse_interactive: **the pinned L = 0 successor**". Only the new test kills it |
| M6 fixture byte changed | the sparse fixture's id gets one byte changed | `u8_d_u6_5_l0_fixtures…`: "the L = 0 fixture is the live successor document, byte for byte". Only the new test kills it |

Every new test kills at least one mutant, and M1, M5 and M6 are killed only by new tests.

## 5. Suites, compared test by test with base `b1e2d7741e` (`_run_records/suites/`)

| Suite (registered unless stated) | Base | Candidate | Per-test difference |
|---|---|---|---|
| PP `cargo test --no-fail-fast` (all targets) | 705 ok, 1 failed (`s11g_tests::t13_committed_fallback_uz_is_byte_identical`, the known Mac t13), 10 ignored | 708 ok, the same 1 failed, 10 ignored | **The 3 new U8 tests only (ok).** No other test changed outcome |
| result_export `cargo test --no-fail-fast` | 172 ok | 172 ok | none |
| runner/headless `cargo test --no-fail-fast` | 85 ok, 2 failed (the known `load_reference` pair: `load_reference_route_tests::load_reference_one_actual_solve_mints_bound_evidence_and_canonical_document_both_modes`, `tests/load_reference_cli.rs::cli_load_reference_one_both_modes_is_controlled_and_equals_the_library_route`) | the same 85 and the same 2 | none |
| PP, **Stale** | 705 / 1 (t13) / 10 | 708 / 1 (t13) / 10 | the 3 new tests only. Candidate Stale = candidate registered, and base Stale = base registered, outcome for outcome |

**Compiler warnings:** the same messages and counts on base and candidate for every suite (`*.warnings`). Only the order of cargo's "generated N warnings (duplicates)" summary lines differs.

**Full logs:** PP and result_export are in full. The runner logs (about 975 kB each, mostly the suite's own debug output) are filtered to their Running, test, failure and result lines. The mutant logs are summarized in `mutants.json` and `mutants.out` (failing tests and panic messages).

## 6. For ROOT

1. **My reading of "the unchanged U5 criterion check".** U5's reference is I50's oracle, which lives in records, not product code. So the committed check applies U5's per-class criterion (§2) to body 0 against the pinned, U5-verified milestone rows, and also requires the milestone's class claims. Together with bit identity, this transfers U5's verdict to body 0 unchanged.
   - The check is evaluated in binary64. That is exact at the zero differences observed, but not a rational-arithmetic replay of `u5_compare.py`.
   - If ROOT wants the oracle comparison itself, it is a records-level rerun of U5's script on the L = 0 fixtures. That would need the oracle extended to the third node (N2), and I did not attempt it.
2. **The output variable is `I68_U8_OUT`,** after `I61_U3G2_OUT`. The fixtures are written only when it is set.
3. **`u8_two_body_case_a` is committed** as the builder of B1's W-C2 model, which case B extends. No U8 test asserts case A's outcome, so F-1's dense behaviour is not pinned.
4. **Nothing else to rule on.** No stop fired: no production change, no existing byte changed, nothing weakened, all writes within the fence.

## 7. Commands (cwd as stated; `C` = `env -u RUSTFLAGS -u CARGO_ENCODED_RUSTFLAGS CARGO_BUILD_JOBS=4 RUST_TEST_THREADS=2`)

1. **Bootstrap,** in `CAND/P/core/product_physics`, with `CARGO_TARGET_DIR=WT/targets/i68-u8`:
   - `C WT/tools/t3_cargo.sh test --locked --offline --lib u8_` (the file pins unset; it failed at the sparse pin, which gave the observed value);
   - then `… --lib u8_l0` (the dense pin);
   - then `C I68_U8_OUT=CAND/P/fixtures/results … --lib u8_` (2 passed; this wrote the fixtures);
   - after adding the D-U6-5 test, `… --lib u8_` (3 passed).
2. **Archives:**
   - `GIT_OPTIONAL_LOCKS=0 git -C WT/f2a-u8 archive b1e2d7741e projects/chirality-piping | tar -x -C WT/scratch/i68_u8_witnesses/base`;
   - the same into `…/mut`, then `cp` of the three candidate files (checked with `cmp`).
3. **Suites:** `_run_records/suites/run_suites.sh cand_reg_pp base_reg_pp cand_reg_rx base_reg_rx cand_reg_runner base_reg_runner cand_stale_pp`, then `… base_stale_pp`. Each runs `cargo test --locked --offline --no-fail-fast` through `t3_cargo.sh` under a 7,200 s alarm.
   - Targets: `WT/targets/i68-u8` (candidate), `…/base`, `…/stale` and `…/base-stale`.
   - Stale runs use `RUSTFLAGS=--cfg=i68_u8_stale`.
4. **Mutants:** `VENV/bin/python _run_records/mutants/mutants.py MUT/P <logs> WT/targets/i68-u8/mut WT/tools/t3_cargo.sh` (6 of 6 killed).
5. **Python fixture check:** `OPENPIPESTRESS_UNITS_BIN=… OPENPIPESTRESS_CHECKED_JSON_BIN=… VENV/bin/python _run_records/py_readers.py BASE/P CAND/P/fixtures/results/retained_precision_l0_successor_*.json`.

## 8. Cleanup

- The BASE and MUT archives and all of `WT/scratch/i68_u8_witnesses/` are deleted on return.
- `WT/targets/i68-u8/` is deleted on return, since there is no reason left to keep it. Part 1's CLI authorities in it are deleted with it.
- CAND keeps the three uncommitted files for ROOT.
