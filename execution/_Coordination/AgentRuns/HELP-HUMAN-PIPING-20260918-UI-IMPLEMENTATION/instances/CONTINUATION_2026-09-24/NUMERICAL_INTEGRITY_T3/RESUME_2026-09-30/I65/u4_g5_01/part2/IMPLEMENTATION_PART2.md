# I65 U4 G5 part 2: the cap-priced profile in the build, the witnesses and the challenge

**Agent:** I65, TASK (Type 2) under ROOT. No descendants.
**Grant:** RR "U4 G5 part 1 verified and committed; I65's four decisions; part 2 granted" (NUM `20bb4a0373`), with RR "RV89 on U4 G5 part 1: PASS; routing into part 2". Brief: `BRIEFS/I65_U4_G5_IMPLEMENTATION.md` (NUM `b4ab0159c3`).
**Code basis:** WT/f2a-memory, branch `codex/piping-f2a-memory-20261004`, on top of `1e323058f3` (part 1). Part 2 is uncommitted.
**Records:** this folder, `R/I65/u4_g5_01/part2/`. Part 1's sealed packet (`R/I65/u4_g5_01/`, 39 files) is unchanged; its corrections are made here (§9).

## 1. What part 2 changed (7 files, all inside the fence)

| File | Change |
|---|---|
| `PP/src/retained_memory.rs` | The generated profile module (§2); `cap_priced_maximum` reads it and stays `Unpriced` while any Estimate atom remains; the gate caps and U3's byte budgets now come from the profile; `checked_or_zero`; RV89 N-4's guard in `bindings_hold`; the witness module mount |
| `PP/src/retained_memory_law_tests.rs` | The profile tests, the pinned in-build record, the RV89 S-1/N-2/N-4 tests (§6), the valid cap-maximal input |
| `PP/src/retained_memory_witness_tests.rs` (new) | The S1 witnesses W1–W7 and the headroom witness (§4) |
| `PP/tests/retained_memory_challenge.rs` (new) | The allocation challenge, in its own test binary (§5) |
| `FK/src/structural/retained_resource.rs` (new) | 98 `size_of` exports of kernel types, plus `MAX_ALIGN` and `FORMATION_ALIGN`; compile-time constants only |
| `FK/src/structural.rs` | `pub mod retained_resource;` (its mod line) |
| `SR/src/elastic_extrema.rs` | `pub const NODE_STRIDE: usize = size_of::<Node>()` (T14's stride; the type stays private) |

No published byte changed (§7). No reader, schema, fixture or existing check changed. `lib.rs` and `retained_product.rs` are untouched in part 2. `REGISTERED_PROFILES` is still `&[]`: no permit is constructible.

Size: +1,983 / −44 lines against `1e323058f3`, the three new files included (`_run_records/candidate_part2.diff`; hashes in `changed_files_sha256_part2.txt`). Of these, 1,250 lines are the generated profile block, so about 730 are written by hand: tests, witnesses, the challenge and the FK module.

## 2. The profile in the build

`g5_profile.py` (here, `_run_records/`) reads `profile_tree.json` (`_run_records/text_p2/`), which the G4 chain (`g4_caps.py`, with the corrections of §3) exports. It rewrites the block between the `GENERATED PROFILE` markers of `retained_memory.rs`.

- **Every term is a linear form over layout atoms.** There are 47 forms over 244 atoms, with the counts already substituted at the D1 caps (l ≤ 128).
- **Every maximum is taken in the build:** T16 and T17's stage maxima, T25's stages, each phase's moving candidates, and the phase maximum. Python never chooses a branch.
- **Each atom has exactly one binding:**

| Binding | Atoms | Meaning |
|---|---|---|
| InBuild | 190 | `size_of`/`align_of` of the actual type in this build. This is direct where PP can name the type, through `structural::retained_resource` for kernel types, and through SR's `NODE_STRIDE`. BTreeMap nodes use BUILD.md §4's node law at in-build key and value layouts |
| SourceUpper | 6 | serde's `(Content, Content)` ≤ 64. Four private PP record types (`Projection`, `RowTreatment`, `Derived`, and the `(String, RowTreatment)` node) and the kernel's private `Option<FormationRecord>` use field-sum upper bounds: Σ up(size_i, A) at the in-build field layouts. G6 witnesses each against the actual `size_of` |
| Text | 6 | The T08 closure's byte counts at `1e323058f3` (layout-free) |
| Estimate | 42 | G3 design-record strides with no single in-build type identified (§8). **While any remains, `cap_priced_maximum` returns `Unpriced`** |

- **Transcription check.** `profile_transcribes_the_python_chain_exactly` evaluates the generated forms at the chain's own (illustrative) strides and reproduces the Python chain's maximum exactly in both modes.

### 2.1 The in-build record (for G6)

Debug test build. rustc 1.97.1, aarch64-apple-darwin. Layouts do not depend on the opt level. Bytes are requested + moving, without R. R = 67,108,864. M = 4,026,531,840. 0.9 M = 3,623,878,656.

| Phase | Sparse | Dense |
|---|---|---|
| W1 ordinary span | 1,850,979,440 | 1,870,689,888 |
| W2 G-B, G-C, T12–T15, N1 reserve | 1,949,018,238 | 1,968,728,686 |
| **W3 publication (T16) + staged copy** | **3,493,720,906** | **3,513,431,354** |
| W4 precommit (T17) + successor + invocation | 3,467,363,071 | 3,487,073,519 |
| W5 transfer and Direct completion | 2,121,274,320 | 2,140,984,768 |
| X1 ordinary span with T25 | 3,222,167,488 | 3,241,877,936 |
| X2 X completion | 1,742,905,402 | 1,762,615,850 |
| **E_mov,max + R** | **3,560,829,770 = 0.8843 M** | **3,580,540,218 = 0.8892 M** |
| Margin below 0.9 M | 63,048,886 | 43,338,438 |
| Estimate atoms' weight in the maximum | 6,053,120 | 6,053,120 |

**The 0.9 M rule holds at in-build strides in both modes, so nothing was adjusted.**
- `profile_in_build_record` prints this table (`I65_G5_PROFILE` / `I65_G5_PHASE` lines; the atom table as `I65_G5_ATOM`) and asserts it. A changed atom, form, combination or phase fails the test until the record is regenerated.
- `profile_laws_hold_in_this_build` asserts ≤ 0.9 M in both modes.
- The printed output is `_run_records/profile_record_p2.txt`.

**For comparison:**
- At the chain's illustrative strides with every correction, the maximum is 0.8692 / 0.8741 M.
- At G4's l ≤ 128 record it was 0.8510 / 0.8559 M.
- The largest in-build shift is SR's `Node`, 184 B against an assumed 64 B.

### 2.2 What the profile now drives

- **`cap_priced_maximum`:** `Unpriced` while `ESTIMATES != 0` (42 today). Otherwise it returns the mode's in-build maximum, which `bound_admits` adds to R.
- **The gate caps (`phase_caps`):**
  - G-B's late-observation cap = T11 − T11's late capture;
  - G-C's observation cap = T11; the seed cap = T11's ordinary seed;
  - the P_final and D_env slot caps use RawVec doubling (`push_capacity`).
  
  A profile value whose checked arithmetic overflowed is a 0 cap (`checked_or_zero`), so it is fail-closed.
- **U3's byte budgets:** staged copy = T18.1; successor = SUCC; invocation = T17.1; reader = T17; statics = STATICS.
- **The registry stays empty.** Nothing reaches a gate or a budget on a permitted path, because no permit exists.

## 3. Routed items folded into the constants and expressions

| Item | Where | Effect |
|---|---|---|
| RV84 C-N1(a) | `g4_caps.py` L_PUB; gate `text_atoms::L_PUB` (part 1) | The final integrity message is priced at the largest reached site, 2,599,962 B |
| **RV84 C-N1(b)** | `t25_g4.py` L_BODY = max(L_ROW, 2,330) | The receipt body carries diagnostic ids ≤ L_DIAGID |
| **RV84 C-N2** | `t25_g4.py` factor terms | Each factor term carries its `dof` number (≤ 16,384 numbers) |
| **RV84 C-N3(a)** | `g4_caps.py` EVERY | T19's spawn heap is counted in every phase, including X1 and W1–W4 |
| **RV84 C-N3(b)** (ruled) | `g4_caps.py` EVERY | The reader's process-lifetime statics are counted in every phase |
| **RV84 C-N3(c)** | `g4_caps.py` SUCC_B | The unused RECEIPT_X is removed |
| **RV87 S-1** | `g4_caps.py` THIRD | T16 P2 carries the third `run_v`/`selection_v` copy (`json!` deep-copies both while the originals live) |
| **RV87 S-2** | `text_args.g4.json` result-id rule `^(row\|result)\.id$` | **16 sites, against RV87's six.** The 10 extra are all genuine `ResultItem` ids: retained_product.rs:2239 and :2434 (`format`); source_receipt/rows.rs:294, :522, :608, :610, :631 and :633; preview_physics.rs:517 and :690. S-2 adds 71,882,496 B to TAV over the rebased run, against RV87's 9.75 MB. With carry 3's 16,080 B the total is +71,898,576, of which +53,136,336 falls in TAV_W. Per-site deltas: `text_p2/text_budget.caps.out.json` against `text_budget.caps.rebase.out.json` |
| **RV87 N-1(a)** | `g4_caps.py` ENV_GROWTH | `to_value` sizes the diagnostics array exactly, so the selected push doubles it: D_env more slots |
| **RV87 N-1(b)** | `g4_caps.py` STAGED | `ResultItem.source_result_refs` Vec backings in the staged copy |
| **RV87 N-1(c)** | `g4_caps.py` LOCALS16 | `row_ids` from `collect::<Option<Vec<_>>>` is doubling-priced |
| **RV87 N-1(d)** | `g4_caps.py` PREPARATION / OPERATIONAL | RV87's corrected grammar: numeric() and the work's `e.count` objects; 12 entries per operational record |
| **RV87 N-1(e)** | `g4_caps.py` NOTICE; `NOTICE_RESERVE_BYTES` | Grant 1b's exact reservation (`ReservedNotice::reserve`) |
| **RV87 N-2** | `g4_caps.py` SETS, OBJ_WALK | `RowClassification.basis_ref` clones (one Value object per row) and the reader's per-object walkers |
| **Carry 3** (RV83) | `text_lexicon.py` `collect_string` | `.collect::<String>()` at lib.rs:1764 (1e32), class range_named: +16,080 B |
| Carry 6 | `READER_LAYOUTS` (part 1) | RV89 N-3 is accepted: the compiler identity pins the std entry-tuple layouts. Nothing is recorded beyond that |
| **Carry 8** | Witness `witness_headroom_w1_at_one_mebibyte` | §4 |
| **Carry 11** | §2.1 | Every G4 expression is re-evaluated at the in-build strides |
| RV83 R-4 | `callgraph_g5.py` | The text run uses the R-4 call graph (part 1's R4_CALLGRAPH.md) |

**The text run at `1e323058f3`** (`_run_records/text_p2/`, `run_text_part2.sh`; G4's rules line-mapped by `linemap.py`, plus the rules above):
- complete;
- D 14,734; D_env 9,361; Text(diag_env) 68,720,236; Text(row) 11,474; L_PUB 2,599,962;
- TAV 2,116,606,292; TAV_X 1,406,259,436; TAV_W 1,564,864,714.

**The rebase alone** (`sens_rebase`, rules mapped and nothing else) gives TAV 2,044,707,716: +545,776 B over G4's 2,044,161,940, from the code between `b1f80234dc` and `1e323058f3` (+40 in D and +1 in D_env). Two rules were added for the rebase:
- `iter:pending.take().map`, bound 1;
- `retained_wire::UNAVAILABLE_CODE`, a 30-byte literal.

## 4. The S1 witnesses (W1–W7)

`retained_memory_witness_tests.rs`. Each witness runs the permitted path's work on one thread with R/k = 4 MiB of reserved stack (`on_reserved_stack`, `carry_test_hooks`):
- the observed ordinary run with capture installed;
- then `retained_w1`, through the private driver, because no permit exists.

The witnesses are `#[ignore]`d, since an overflow aborts the process, and run one process each (`_run_records/run_witness.sh`). They are measured evidence for this build and these inputs, not a proof (D-3 = S1).

All witnesses ran in the debug test build. Each passed, with no overflow, abort or panic (`_run_records/witnesses_p2.txt`):

| Witness | Input | Sparse | Dense |
|---|---|---|---|
| W1 | The milestone | Successor | Successor |
| W2 | The cap-maximal D1 input (law_tests::cap_maximal), plus a quote-and-backslash provenance and a raw depth-16 value | Fallback(Preparation): the support build refuses the shape | Fallback(Preparation) |
| W2b | W2 made solvable (milestone support shapes: one rigid support, 31 springs) | Fallback(Candidate), after the full native run at the largest counts | Fallback(Candidate) (about 46 s for both) |
| W3 | n05 source_blocks requests: legacy exact recovery selects | ExactSelected (retained_w1 refuses with Coexistence) | ExactSelected |
| W4 | The milestone, with the captured member's diameter zeroed before W1: the preparation refusal and its fallback chain | Fallback(Preparation) | Fallback(Preparation) |
| W5 | Dense mode, through the dense halves of W1, W2, W2b, W4 and W6 | — | as above |
| W6 | PHYS-R4's cantilever without its pressure contract: the input is force-scaled (`force_scaled=true`) | Fallback(Native) | Fallback(Native) |
| W7 | U3's faults, sparse: withdraw the native source, serializer Encoding, staging, precommit rebinding, precommit corruption | Native; Serializer(Encoding); Staging; Precommit G8 INVOCATION_MISMATCH; Precommit G1 RECEIPT_MISMATCH | — |
| **Headroom** (carry 8) | W1 at R/64 = 1 MiB | Successor | Successor |

**Carry 8.**
- W1's successor is validated by the precommit reader, so passing at 1 MiB bounds the reader's schema walk on W1's input at a quarter of the witness stack.
- Whether that walk reaches the deepest 36-level `$ref` chain is not observable without instrumenting the reader, which is outside the fence. That is the stated residual for G6.

**Release-profile witnesses** are left to G6, which records witnesses per qualified build identity (§10).

## 5. The allocation challenge

`PP/tests/retained_memory_challenge.rs` is a separate test binary with its own counting global allocator. It runs the public retained Direct entry. Because no permit exists, the ordinary span runs: G-A, the parse and the ordinary run. Each measured peak must stay at or below the profile's in-build W1 phase.

`challenge_bounds_are_the_profile` (a lib test) reads the binary's `W1_PHASE_BYTES` and checks that it equals the in-build evaluation.

| Input | Mode | Rows | Peak (B) | W1 phase (B) | Ratio |
|---|---|---|---|---|---|
| Milestone | Sparse | 98 | 533,234 | 1,850,979,440 | 0.000288 |
| Milestone | Dense | 99 | 540,831 | 1,870,689,888 | 0.000289 |
| Large D1 input (32 nodes, ring of 32, 1 anchor + 31 springs, 128 loads, 4 + 4 materials × 16 points, a 128-byte id) | Sparse | 2,113 | 13,191,619 | 1,850,979,440 | 0.007127 |
| The same | Dense | 2,114 | 13,199,237 | 1,870,689,888 | 0.007056 |

Every run is `MECHANICS_SOLVED` with no blocking diagnostic. The output is `_run_records/challenge_p2.txt`.

The challenge is not a proof and not a production guard.

## 6. RV89's part-1 findings routed into part 2

| Item | Change | Evidence |
|---|---|---|
| **S-1** | Tests only:<br>– `d1_3_refuses_one_authored_law_among_several_materials`: model and request lists, the law on either material;<br>– `restraint_capacity_total_is_the_sum_over_supports`: one support `reserve_exact(100)`, so every single capacity ≤ 192 and the total > 192, refusing with the exact `Cap`;<br>– `typed_walk_reads_request_materials_and_temperature_point_ids`: length and capacity, for a request material id and for request and model temperature-point ids | V03, V06, V07 and V08 re-run (§7.1) |
| **N-2** | `gate_sums_saturate_and_the_longest_string_reads_every_diagnostic_field`:<br>– `Bytes` saturates at u64::MAX on add, times and plus;<br>– the longest string reads `source` and the first and last `affected_refs`, directly and through G-C's `EnvelopeMaxStringBytes` | V17 and V24 re-run (§7.1). V19 needs no test (RV89) |
| **N-4** | Code guard: `bindings_hold` refuses a compiled reviewed-input text with any `<path>=unavailable` field. It splits at `;` and needs no allocation, and an encoded path holds no raw `=`. An unreadable reviewed input is therefore Stale by construction, as `identity_match` treats `v1;unavailable`.<br>Test: `an_unreadable_reviewed_input_never_binds` covers each of the 14 inputs unreadable, all unreadable, and this build reading all 14 | Two new mutants (§7.1) |
| **N-1** | Part 1's tally, restated: of 84 mutants, **81 behavioural kills, 2 text-pin kills** ("D1.8 deletion", "admission ignores the refusal") **and 1 equivalent survivor** ("build_status ignores bindings", by decision 7) | Part 1's `_run_records/mutants.out.jsonl` |
| **N-6** | R4_CALLGRAPH.md §2's sentence is corrected in §9 below. Part 1's sealed bytes are left unchanged | §9 |
| N-3 | Accepted: the compiler identity pins the std entry-tuple layouts | — |
| N-5 | Noted. No later consumer may publish the report's `Debug` text | — |

## 7. Controls

`_run_records/run_p2_controls.sh` and `run_p2_pp.sh`; the outcomes and summary are in `_run_records/controls/`.
- **The candidate copy** is `1e323058f3`'s git archive with part 2's seven files overlaid; `diff -rq` confirms it equals the worktree's core tree.
- **The base** is `8abb5274a9`, part 1's base.
- **Host:** the default toolchain (rustc 1.97.1), `--locked --offline`, `CARGO_BUILD_JOBS=4`, `RUST_TEST_THREADS=2`, one cargo job at a time, with memguard (PID 5387) checked before each job.

| Control | Result |
|---|---|
| 1. No published byte changes | **The fixture sweep is byte-identical to base**, sha256 `0690bc64…41e1` for both (part 1's sweep: every route and mode, including every public admission-report field). The bound stays `Unpriced` and the registry stays empty, so no admission outcome changes |
| 2. Nothing weakened: PP | 694 passed, 1 failed, 10 ignored, against base's 661 / 1 / 2. The differences:<br>– part 1's 23 law tests and part 2's 9;<br>– the challenge binary's test;<br>– the 8 `#[ignore]`d witnesses.<br>The failure is the known Mac `t13_committed_fallback_uz_is_byte_identical`, as at base. The lib warnings are 8, as at base |
| runner/headless | 85 passed, 2 failed: outcomes identical to base |
| FK (`open_pipe_stress_frame_kernel`) | 546 passed, 1 ignored at both base and candidate: outcomes identical |
| SR (`open_pipe_stress_stress_recovery`) | 48 passed at both: outcomes identical |
| No permit | `REGISTERED_PROFILES` is `&[]`. `no_profile_or_permit_is_constructible` passes. There is no test permit in maintained code, and the witnesses use the private driver |
| Readers, schemas, fixtures | Unchanged. Part 2 touches no file outside the seven |

**A note on the PP control.** The first PP run (`run_p2_controls.sh`) failed to compile `retained_precision_admission.rs`, because the 1e32 archive had no `apps/`, which that test `include_str!`s. `apps/` is identical at `8abb5274a9` and `1e323058f3`, so `run_p2_pp.sh` added base's copy and re-ran the PP suite. That run is the result above.

### 7.1 Mutants

`_run_records/mutants_g5_part2.py` runs three sets, each as one exact replacement in a scratch copy of the candidate core tree, followed by `cargo test --lib retained_memory` (one cargo job):
- **P1:** part 1's 84, re-run, with the three anchors part 2 rewrote restated;
- **RV:** RV89's 24, unchanged apart from V12's anchor, which N-4's guard moved;
- **P2:** part 2's own 40.

The output is `_run_records/mutants_p2.out.jsonl`, from 10:07 to 10:28 UTC.

| Set | Run | Killed by a test | Survived | Compile-only |
|---|---|---|---|---|
| P1 (part 1) | 84 | 83 (81 behavioural, 2 text pins: "D1.8 deletion", "admission ignores the refusal") | 1: "build_status ignores bindings", equivalent by decision 7 | 0 |
| RV (RV89) | 24 | 23, **including V03, V06, V07, V08, V17 and V24**, each by its new test (§6) | 1: **V19**. build.rs accepts a duplicated `rustc -vV` field, which cannot be observed in-crate. RV89's standalone run D shows the candidate emits `v1;unavailable`, and RV89 ruled that no test is needed | 0 |
| P2 (part 2) | 40 | 40 | 0 | 0 |
| **All** | **148** | **146** | **2, both recorded above** | **0** |

**The first P2 run** (`_run_records/mutants_p2_run1.out.jsonl`) found five gaps. Each was closed before the final run, and none by weakening anything:
- "modes swapped", "add saturates", "node: alignment floor 8 dropped" and "FK export: Formation align 1" survived:
  - `priced_maximum(estimates, mode)` became a pure function and is tested at 0 and at 1 estimates in both modes;
  - the profile's `add` and `max` are now tested at overflow;
  - the node law is tested at a 4-byte alignment;
  - the kernel exports are cross-checked against the kernel types PP can name.
- "form drops its constant" did not compile (`0` was ambiguous). It was restated as `0 * f.constant`, and a test now kills it.

**The P2 mutants and the tests that kill them:**

| Mutant | Result | Killed by |
|---|---|---|
| profile: Estimate gate removed | KILLED | `law_order_reports_the_first_failing_clause`, `profile_laws_hold_in_this_build` |
| profile: Estimate gate inverted | KILLED | `law_order_reports_the_first_failing_clause`, `profile_laws_hold_in_this_build` |
| profile: Estimate count ignored | KILLED | `law_order_reports_the_first_failing_clause`, `profile_laws_hold_in_this_build` |
| profile: estimates not counted | KILLED | `law_order_reports_the_first_failing_clause`, `profile_laws_hold_in_this_build` |
| profile: modes swapped | KILLED | `profile_laws_hold_in_this_build` |
| profile: maximum keeps the smallest | KILLED | `profile_in_build_record`, `profile_transcribes_the_python_chain_exactly` |
| profile: maximum ignores moving | KILLED | `profile_in_build_record`, `profile_transcribes_the_python_chain_exactly` |
| profile: max takes the smaller | KILLED | `challenge_bounds_are_the_profile`, `profile_in_build_record`, `profile_laws_hold_in_this_build`, `profile_transcribes_the_python_chain_exactly` |
| profile: add saturates | KILLED | `profile_laws_hold_in_this_build` |
| profile: form drops its constant | KILLED | `challenge_bounds_are_the_profile`, `complete_facts_read_the_actual_owners`, `profile_in_build_record`, `profile_transcribes_the_python_chain_exactly` |
| profile: form drops its last term | KILLED | `challenge_bounds_are_the_profile`, `profile_in_build_record`, `profile_transcribes_the_python_chain_exactly` |
| profile: form coefficient ignored | KILLED | `challenge_bounds_are_the_profile`, `profile_in_build_record`, `profile_transcribes_the_python_chain_exactly`, `structural_budgets_are_u3s` |
| profile: T16 is a sum | KILLED | `profile_in_build_record`, `profile_laws_hold_in_this_build`, `profile_transcribes_the_python_chain_exactly` |
| profile: W3 drops T16 (sparse) | KILLED | `profile_in_build_record`, `profile_transcribes_the_python_chain_exactly` |
| profile: X2 drops the reserve (dense) | KILLED | `profile_in_build_record` |
| node: drops the internal node | KILLED | `challenge_bounds_are_the_profile`, `profile_in_build_record`, `profile_laws_hold_in_this_build` |
| node: alignment floor 8 dropped | KILLED | `profile_laws_hold_in_this_build` |
| node: 11 keys read as 10 | KILLED | `challenge_bounds_are_the_profile`, `profile_in_build_record`, `profile_laws_hold_in_this_build` |
| binding: ResultItem read as Diagnostic | KILLED | `challenge_bounds_are_the_profile`, `profile_in_build_record` |
| binding: SR node stride dropped | KILLED | `challenge_bounds_are_the_profile`, `profile_in_build_record` |
| binding: Content pair halved | KILLED | `profile_in_build_record` |
| binding: Projection upper loses a String | KILLED | `profile_in_build_record` |
| FK export: Wide<16> read as Wide<8> | KILLED | `profile_in_build_record` |
| FK export: ProductFinalRow read as ProductRecipe | KILLED | `profile_in_build_record`, `profile_laws_hold_in_this_build` |
| FK export: Formation align 1 | KILLED | `profile_laws_hold_in_this_build` |
| gate: late cap keeps the late capture | KILLED | `every_phase_fact_admits_its_cap_and_refuses_cap_plus_one` |
| gate: seed cap is T11 | KILLED | `every_phase_fact_admits_its_cap_and_refuses_cap_plus_one` |
| gate: observation cap is the seed's | KILLED | `every_phase_fact_admits_its_cap_and_refuses_cap_plus_one` |
| gate: D_env slots unpushed | KILLED | `every_phase_fact_admits_its_cap_and_refuses_cap_plus_one` |
| gate: P_final slots unpushed | KILLED | `every_phase_fact_admits_its_cap_and_refuses_cap_plus_one` |
| gate: overflowed form reads u64::MAX | KILLED | `profile_laws_hold_in_this_build` |
| budget: reader unchecked | KILLED | `structural_budgets_are_u3s` |
| push: doubles from 8 | KILLED | `profile_laws_hold_in_this_build` |
| push: off by one | KILLED | `profile_laws_hold_in_this_build` |
| push: zero is four | KILLED | `profile_laws_hold_in_this_build` |
| text: D_env is D | KILLED | `profile_laws_hold_in_this_build` |
| budget: staged and successor swapped | KILLED | `structural_budgets_are_u3s` |
| budget: reader is the successor's | KILLED | `structural_budgets_are_u3s` |
| N-4: unavailable inputs bind | KILLED | `an_unreadable_reviewed_input_never_binds` |
| N-4: only the first input checked | KILLED | `an_unreadable_reviewed_input_never_binds` |

## 8. The Estimate atoms G6 closes (42)

Each is a G3 design-record stride (from `producer_caps.py` / `ordinary_caps.py`) with no single in-build type identified in G5. Their weight in the maximum is 6,053,120 B in each mode. **While any remains, the bound is `Unpriced`.** The value in brackets is the design-record stride carried today:

- **Maps and nodes:**
  - `Node(TrackerKey,())` [512], `Node(TrackerKey,Tracker)` [4,096]: the kernel stop rule's tracker map and key set;
  - `AliasE` [32], `BodyMapE` [56], `ConstraintMapE` [40], `MemberMapE` [64], `NodalMapE` [48], `NodeMapE` [40], `SectionMapE` [64], `SpringMapE` [48], `StationMapE` [32], `SupportMapE` [64], `TableE` [48], `RegistryE` [128], `TrackerE` [64], `LazyE` [2,232], `EnclosureE` [304], `IntervalE` [32], `MaximumE` [128], `LawE` [64], `ContributionE` [40], `ConversionE` [48].
- **Records:**
  - `AdapterSnapshot` [512], `ArcPrepared` [512], `BodyCoverage` [32], `ConversionEvent` [64], `CoverageFact` [32], `Expansion6` [144], `FinalRowConversion` [64], `LaneTerminal` [256], `MaterialDescriptor` [96], `Pair` [16], `PreparedMemberEvent` [256];
  - `ProductRow` [96], `ProductValue` [32], `ProjectionOutcome` [64], `Row6` [48], `RunRow` [32], `SupportCoverage` [64], `SupportVector` [80], `Tag` [8], `(usize,VecPair)` [32].

**Closed in part 2** (from 48):
- `Projection` [80 → 192, SourceUpper];
- `RowTreatment` [104 → 104];
- `Derived` [360 → 336];
- `Node(String,RowTreatment)` [1,520 → 1,528];
- `Option<FormationRecord>` [64 → 56, SourceUpper over the exported `Formation`];
- `RowBinding` [64 → 64, InBuild: `bind_rows`' element is the kernel's `ProductFinalRow`, exported].

## 9. Corrections to part 1's records (not edited in place)

**R4_CALLGRAPH.md §2 (RV89 N-6).**

The sentence "load_ledger.rs:131 `push` loses false edges and carries no text" is wrong. Corrected:
- `push` (load_ledger.rs:131) carries one text site: the `into_text` at load_ledger.rs:133, 128 B.
- R-4 lowers that site's multiplicity from 915,466,156 to 915,000,428.
- Its requested bytes are 163,840 B both before and after, so TEXT (2,044,161,940 B) and the G4 maximum are unchanged.

**The `function_multiplicity` table also gains 10 entries** under R-4. None carries a text site, and `sites_with_positive_multiplicity` is 1,426 in both runs. The 10 entries are:
- **Eight PP functions already reachable at G4:**
  - six `typed_trace`: retained_product.rs:3803, :3860, :3874 and :3881; retained_wire.rs:1488 and :1495;
  - retained_receipt.rs:105 `summary_coverage`;
  - retained_receipt.rs:148 `project`.
- **Two newly reachable final_case functions:** final_case.rs:283 `coverage_facts` and :315 `check_summary_coverage`.

## 10. Not done, and why

- **RV85 U1** (optional): binding the permit to its invocation.
  - A lifetime or digest binding changes `CapturePermit`'s construction and `permitted_run` in lib.rs, which is outside the D-5 exception.
  - The dependency-free compile-time "not `Copy`/`Clone`" assertion would make its own mutant killable only by a compile error, against control 3.
  - Both are left to G6 or a later grant. The text check and N7's call-site counts stand.
- **Carry 8's deepest chain.** Whether W1's successor drives the reader's 36-level `$ref` chain cannot be observed without instrumenting the reader, which is outside the fence. The 1 MiB headroom witness bounds the reader's walk on W1's input at a quarter of the witness stack.
- **Release-profile witnesses:** left to G6. G6 records witnesses per qualified build identity.
