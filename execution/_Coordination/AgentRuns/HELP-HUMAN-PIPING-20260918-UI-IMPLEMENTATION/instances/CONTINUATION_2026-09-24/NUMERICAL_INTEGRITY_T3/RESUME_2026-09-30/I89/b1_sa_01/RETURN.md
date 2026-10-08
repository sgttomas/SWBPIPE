# I89 B1-SA: admission at option S3

TASK (Type 2), I89, role I-A, for ROOT (HELP_HUMAN, Agent 0), who is the return path. I made no delegation. I am a fresh instance. 2026-10-07 UTC.

**Briefs (verified before work):**
- `R/BRIEFS/B1_COMMON.md`: sha256 `2d170307516b98c40e8aa2dcf352cf11a13dbdd29aeddd78a4c39f3f15eb2c75`. This matches my computation, the committed blob at NUM `25c745f905`, and the values I85 and I86 recorded. **RR does not record its full hash.** RR names the file at "B1's PLAN_v2 accepted…" without a hash, so the dispatch's prefix `2d170307…` was checked against those three instead.
- `R/BRIEFS/B1_SA.md`: sha256 `594d2a565712418f9899a97e1408b5be20de7e175a0ebb8545cfdf57e0940e27`, as dispatched.
- The specification, `R/I84/b1_plan_01/PLAN_v2.md`, has sha256 `c85786b704805311485b44ba27c8826ca278c1237d92010f9f997dbf3c919be0`, as B1_COMMON states.
- I read NUM's root `AGENTS.md` and `agents/AGENT_TASK.md` first.

**Basis read:**
- PLAN_v2: §1 (the fence), §2.1 (the seam), §2.3 (SA), §3.3, §4 (R8), §7 and §8;
- RR: "B1's PLAN_v2 accepted; RV107's A1 amendments…", "R3: I85's ST verified…" (ruling 2), "Owner decision: SI1c is option D…; RV109 passes ST…" (N-3 → SA), and "RV109 confirms ST's repair; I1 (= I1′) at `262bd687f0`…";
- `R/I85/b1_st_01/RETURN.md`, whose method I followed;
- RV109's REVIEW N-3;
- DESIGN_v2 §1.1–§1.2 (T-3 (e)) and §6;
- I82's STUDY §4.3 (the restated G-B and G-C rows; B-6).

**ROOT's message during the run** (after SP's checkpoint R3′, NUM `a4c2cfa01b`):
- (1) RetainedErrorTextBytes must also read SP's parked case slots. This is answered in §8.
- (2) No record folder may be named `build`, and every write uses an absolute path. Both hold.

**Placeholders:**
- `WT`, `NUM`, `P`, `PP` (= `P/core/product_physics/src`), `PP-tests`, `RE`, `T`, `R`, `RR` and `VENV`, as in the dispatch;
- `S` = my scratch, `WT/scratch/i89_b1_sa`;
- `BASE` = `S/base`, a `git archive` of I1 `262bd687f0` (P without `P/execution`);
- `MUT` = `S/mut`, the same archive of my head;
- `I2P` = `S/i2`, the I2 preview (§8).

**Limits kept:**
- **Cargo.** Every cargo job went through `WT/tools/t3_cargo.sh`, with `--locked --offline`, `CARGO_BUILD_JOBS=4`, `RUST_TEST_THREADS=2`, `TMPDIR` in `S`, and `RUSTFLAGS`/`CARGO_ENCODED_RUSTFLAGS` unset except in the one Stale run. The memory guard was up. There were 40 jobs (12 rc 0; 28 rc 101, each from the known Mac `t13`, a mutant, or a deliberate negative check). They interleaved with I85's, I88's, I90's and RV111's jobs. I killed no job.
- **Waits.** Each wait ended when its job's process was gone. None of my processes or monitors remains.
- **Python and tools.** No vitest or pytest ran. Python (VENV, or the system `python3` for text edits in `S`) was used only for the mutant driver, the log sanitizer and scratch edits.
- **Not run.** No DEC-025, no evidence sweep, no native or solver-at-scale job beyond the inputs below, and no install.
- **Git.** My one commit is on `codex/piping-t3-b1-a-20261007`, and nothing was pushed. Reads used `GIT_OPTIONAL_LOCKS=0`. SP's files were read from `WT/b1` at `56c5579f07` (`git show`, `git archive`) and never edited.
- **Writes.** Nothing went to the system temp directory. The host accepted every write into this records folder.

## 0. Outcomes in brief

| Acceptance item (PLAN_v2 §2.3; B1_SA) | Outcome |
|---|---|
| The caps and the census | **Done.** `LOAD_CASES` 1 → 3; new `TOTAL_LOADS` = 384. The census reads the per-case load facts as maxima over every case, plus a new Σ l_i (§2.1) |
| D1.4, D1.5 and D1.7 per case | **Done.** D1.4: 1 ≤ c ≤ C. D1.5 and D1.7 read every case. c = 0 to 4 and the per-case facts are pinned (§4) |
| The `cap_rows` | **Done.** `LoadCasesCapacity` ≤ 3; `Loads` and `LoadsCapacity` ≤ 128 over every case; new `TotalLoads` ≤ 384 (`CAP_ROWS` 46 → 47) |
| G-B with `CaseLoadsTotal` | **Done.** `LATE_FACTS` 9 → 10. `CaseLoadsTotal` ≤ 384 reads the seam's `late_loads_total` |
| G-C at C = 3, with the EnvelopeResults bound | **Done.** C·P_final, its capacity and its text; the contract-evidence facts × C; RetainedErrorTextBytes ≤ C·(3m + 1)·Text(err). ROOT's parked-slot note is a ready patch for I2 (§8) |
| T-3 (e) with `requested_cases` | **Done.** `ordinary_solve_attempted(capture, requested)`. Two real three-case runs that block at k < c − 1 are declined; Direct gives exact bytes and no notice |
| The budgets | **Done.** B-6 = `NOTICE_RESERVE_BYTES` × C. B-2 to B-5 keep their profile forms |
| Every law test listed | **Done:** 7 new tests and 12 changed ones (11 in PP, 1 in the runner; §3), including c = 0 to 4 and the D1.4 line of `every_family_clause_refuses_with_its_fact` |
| The out-of-domain oracles at `LOAD_CASES + 1` | **Done:** the law tests' `admit_grants_…` site and `actual_retained_entry_dispatches_ordinary_once`. ST's four facade oracles went to 4 cases on their own, and pass |
| The runner's literal 4, tied by a new PP law test | **Done.** No D1 visibility change |
| The listed mutants killed by assertions | **12 of 12 killed by assertions, and 9 of 9 extras; all compiled** (§6) |
| The registered PP suite differs from I1 only in the listed tests | **Holds.** 712/1/11 → 719/1/11: +7 new tests, all ok; every other outcome is identical. The runner is identical (85/2), the witness lines are identical (28), and the warnings are identical (§5) |
| c = 1 byte identity | **Holds.** All six successor documents are byte-identical on I1 and the head, and equal their fixtures (§5.3) |
| The guards | **Pass.** PP-tests' s11f (11) and admission (5), and RE's carriers (17). The in-fence guards pass in the suite (§5.4) |
| Pricing; nothing leaves the branch before SQ | **Kept.** `cap_priced_maximum`, `admission_bound`, the GENERATED PROFILE block and `REGISTERED_PROFILES` are byte-unchanged. The in-build profile record is unchanged (§2.2) |
| Stops (§1, R8) | **None fired** |

## 1. Head and commits

Branch `codex/piping-t3-b1-a-20261007` in `WT/b1-a`, from I1 `262bd687f0`. **Head `6b626067786523068188ccd29f9ac2db9ed50e7d`**, one commit:

| Commit | Content |
|---|---|
| `6b626067786523068188ccd29f9ac2db9ed50e7d` | SA: caps, census, D1.4/D1.5/D1.7, `cap_rows`, G-B's total, G-C at C, T-3 (e), B-6, the re-based oracles, the runner's literal and the new law tests |

The diff `262bd687f0..6b62606778` (`_run_records/sa.diff`) has 3 files, +569 / −85, all inside SA's fence:

| File | I1 blob | Head blob | sha256 at head | Lines |
|---|---|---|---|---|
| `PP/retained_memory.rs` (outside the GENERATED PROFILE block) | `688cb0da5c` | `391abe42fd` | `c8ad85ec38d22ff2eceec771063913ce95a14f300696b1aca108c04e45f2197a` | +105 / −54 |
| `PP/retained_memory_law_tests.rs` | `95a6dd26e8` | `be0782515c` | `85c9ea9aa0b945ed0615b9108589ef5cab9a68b792de85671994dcfb83741162` | +448 / −25 |
| `P/core/runner/headless/tests/retained_precision_admission.rs` | `143f4e6725` | `c81ee6939b` | `73da3a859e04fd2859d28d808aaf10b3f0db4f66978215a0dc028a149f3afd88` | +16 / −6 |

**Unchanged:**
- `PP/lib.rs` (`84fba5ca…`) and `PP/retained_product.rs` (`5cf8d6d5…`);
- FK, the schema, `Cargo.lock`, the 13 reviewed statics and every reader;
- SP's files, SQ's files (`retained_memory_witness_tests.rs`, the challenge) and the T6S and B6 files;
- in `retained_memory.rs`, the GENERATED PROFILE block (sha256 of the block `a3c62721…`, equal at I1 and at the head) and `REGISTERED_PROFILES` (`9ffb6186…`, equal).

The runner hunk is rustfmt-clean. `rustfmt --check` reports only the import block above it, which is the same at I1.

## 2. The code (`PP/retained_memory.rs`)

### 2.1 Admission

- **`caps`:** `LOAD_CASES` = 3 (C) and a new `TOTAL_LOADS` = 384 (L). Each carries a doc line, and L is stated as C·l, which does not bind (ADDENDUM_01 §2). Every other D1.9 and D1.11 cap is unchanged.
- **Census (`NestedTypedFacts`, `TypedWalk::walk`):**
  - `primitive_loads` is now the largest length and the largest capacity over every case. Before, it was case 0's only, set at `index == 0`.
  - The new `total_loads` is Σ l_i, as a checked `u32`. A length or sum that does not fit is D1.2's `ArithmeticOverflow`.
  - **Why `u32`** (§2.2): it sits in `status`'s padding, so the report's layout, and therefore the profile's in-build values, stay unchanged until SQ.
- **D1.4:** `load_cases.is_empty() || len > LOAD_CASES` refuses with `(Invocation, LoadCases)`. Combinations and components are unchanged.
- **D1.5:** the five checks run for every case, in request order.
- **D1.7:** the checks run over `load_cases.iter().flat_map(primitive_loads)`. D1.10 already read every case.
- **`cap_rows`** (`CAP_ROWS` 47):
  - `Loads` and `LoadsCapacity` read the census's maxima;
  - the new `TotalLoads` row (`n.total_loads as usize` ≤ `TOTAL_LOADS`) sits right after them;
  - `LoadCasesCapacity` is capped by `LOAD_CASES`, now 3;
  - D1.11's `ControlBytes` stays the last row, as `domain_clauses` requires.

### 2.2 Gates and budgets

- **G-B:**
  - `PhaseFact::CaseLoadsTotal`, right after `CaseLoads`; `LATE_FACTS` 9 → 10.
  - `late_observations` reads `count(f.capture.late_loads_total)`, ST's seam. G-B at case k therefore sees Σ_{i≤k} l_i. `LateFacts` is unchanged.
  - The late caps are `[n, m, m, g, l, L, k, s, 4 + 4, T11 − T11_late_capture]`.
- **G-C, at `c = LOAD_CASES as u64`:**
  - `EnvelopeResults` c·P_final; `EnvelopeResultCapacity` PushCap(c·P_final); `EnvelopeResultTextBytes` 2·c·P_final·Text(row) `[N-13]`;
  - the five contract-evidence facts are c times today's per-case preview facts (I82 STUDY §4.3);
  - RetainedErrorTextBytes is c·(3m + 1)·Text(err), I82's assumption, checked against SP's producer in phase 4;
  - the diagnostic, string and byte bounds keep their expressions over `text_atoms::*` and the profile's forms. SQ regenerates them.
- **T-3 (e):** `ordinary_solve_attempted(capture, requested)` = `requested >= 1 && capture.ordinary.len() == requested && every initial is set`.
  - `complete_observations` passes `f.requested_cases`.
  - `CompleteFacts::requested_cases` loses its `#[allow(dead_code)]`, and its doc now says it is read.
- **B-6:** `notice_reserve_bytes = NOTICE_RESERVE_BYTES × LOAD_CASES`, so at most one reserve per case in A. B-2 to B-5 keep their forms.
- **Pricing is unchanged.** So is the GENERATED PROFILE block.
  - **The census's `u32`.** My first build used a `usize` total. That grew `NestedTypedFacts`, and with it `RetainedPreviewOutput` by 8 B. The in-build atom `s(ThreadPacketOutput)` prices that type, so every phase moved by +8 B.
  - **What failed with the `usize` total:** `profile_in_build_record` (its `PINNED_RECORD`) and `challenge_bounds_are_the_profile` (the challenge's `W1_PHASE_BYTES` literal, which lives in SQ's file). That is `_run_records/early/early_retained_memory.log`.
  - **With the `u32` total** the layout is unchanged. Both tests pass, and the record is untouched for SQ to regenerate.

## 3. The tests

### 3.1 Changed (law tests, in-crate tests, runner)

| Test | Change |
|---|---|
| `admit_grants_a_permit_for_the_milestone_in_the_registered_build` | Its out-of-domain request now has `LOAD_CASES + 1` cases, and it asserts `(Invocation, LoadCases)`. Before, it asserted any `Family(..)` |
| `retained_memory::tests::actual_retained_entry_dispatches_ordinary_once` | `LOAD_CASES + 1` cases (copies `case-2`…`case-4`). Adds an assertion that the domain refusal is D1.4's |
| `nested_typed_census_reads_the_roster` | Adds the total at C = 1. The cap-maximal three cases give (128, 128, 384). On an uneven three-case milestone, the second case is the longest and the third has the largest capacity |
| `every_family_clause_refuses_with_its_fact` | **The D1.4 line:** C + 1 cases and 0 cases refuse with `(Invocation, LoadCases)`. Also: C cases are inside D1; each D1.5 fact and both D1.7 facts on the last case refuse; D1.10 on the last case refuses |
| `every_cap_row_admits_its_cap_and_refuses_cap_plus_one` | Every row, the new one included, at cap and cap + 1. Adds: C = 3, L = 384, L = C·l; the cap-maximal C-case input is inside D1 with `LoadCasesCapacity`, `Loads`, `LoadsCapacity` and `TotalLoads` at their caps; the row order |
| `actual_inputs_map_each_cap_fact` | A later case at l is admitted. At l + 1 it is refused with `Cap { Loads, 129, 128 }`, while case 0 keeps 3 loads |
| `typed_capacity_and_units_rows_read_the_actual_owners` | `LoadCasesCapacity`: one case with room for C more is refused. C cases with C slots are admitted; with one more slot, refused. The last case's load capacity is admitted at 128 and refused at 129 |
| `every_phase_fact_admits_its_cap_and_refuses_cap_plus_one` | The late list gains `CaseLoadsTotal`. The bounds that read the profile are asserted as expressions. The registered atoms' values are kept as value pins (§9 item 5) |
| `structural_budgets_are_u3s` | B-6 = `NOTICE_RESERVE_BYTES` × C. The per-notice bounds (≥ the slot + 196, ≤ 2,048) now apply to `NOTICE_RESERVE_BYTES` itself |
| `late_facts_read_the_actual_owners` | `CaseLoadsTotal` equals the capture's `late_loads_total` (0 for a permitless observer) |
| `g_c_declines_w1_when_the_ordinary_solve_was_not_attempted` | The new signature, `ordinary_solve_attempted(…, 1)` (c = 1). Unchanged otherwise |
| runner `explicit_headless_refusal_preserves_output_and_completion_fields_both_modes` | `const C_PLUS_ONE: usize = 4`, with a comment naming `product_physics::retained_memory::caps::LOAD_CASES` + 1 and the PP tie test. 4 load cases; asserts exact ordinary bytes, no successor and `typed.load_cases.length == 4` |

**Helpers (test-only):**
- `with_cases(raw, c)` copies the first case c times, with ids `case-1…` and each load id suffixed `~i`. The ordinary route's validation requires primitive-load ids to be unique across cases, so a plain copy would be a `DUPLICATE_ID` blocked run.
- `milestone_cases`, `cap_maximal_cases` (`pub(super)`, inside the test module) and `solvable_cap_maximal_cases`. The last is W2b's support shapes on the cap-maximal model: `cap_maximal` itself does not solve; it is W2's `not_assessed` input.

### 3.2 New

| Test | What it pins |
|---|---|
| `b1_sa_d1_4_admits_one_to_c_load_cases` | c = 0, 1, 2, 3, 4 on the milestone, in both modes, through the actual `admit`. c = 0 and 4 refuse `(Invocation, LoadCases)`; 1 to 3 are inside D1. The typed length and capacity equal c. The refusal is D1.1's first, then D1.4's. In the registered build, permits are granted for 1 to 3 only |
| `b1_sa_runner_oracle_literal_is_load_cases_plus_one` | `LOAD_CASES + 1 == 4`, and `admit` refuses `LOAD_CASES + 1` cases at D1.4 with `(Invocation, LoadCases)` in every build, in both modes. Its doc and message name the runner test |
| `b1_sa_total_loads_row_admits_l_and_refuses_l_plus_one` | On the cap-maximal C-case input's actual census: Σ = 384 is admitted. 385 (the census raised by one, which no request can reach) gives `Cap { TotalLoads, 385, 384 }` |
| `b1_sa_g_b_reads_each_case_and_the_running_total` | `late_observations`' fact order. `CaseLoads` reads the case and `CaseLoadsTotal` the capture: (3, 3), (3, 9), (128, 384) and (3, 384) are admitted. (3, 385) → `CaseLoadsTotal`; (129, 129) and (129, 385) → `CaseLoads` (table order). (3, `usize::MAX`) → `CaseLoadsTotal` at `u64::MAX` (§7). **Registered:** an actual permitted run of the milestone holds `late_loads_total == 3` at G-B, with no late refusal, in both modes |
| `b1_sa_envelope_results_bound_is_c_times_p_final` | The three bounds' expressions. C·P_final rows are admitted; one more refuses with `EnvelopeResults`. **Real runs** (W2b-shaped cap-maximal, private route), sparse / dense: one case has 2,113 / 2,114 rows, capacity 4,096, text 687,320 / 688,664. **Three cases** have 6,339 / 6,342 rows (> P_final = 2,115; ≤ 6,345), capacity 8,192 (= its bound), text 2,171,438 / 2,175,514 (≤ 145,605,060) |
| `b1_sa_t3_e_counts_the_requested_cases` | Every term of the predicate on hand-built seed lists: c = 0; no seed; for c = 1–3, one seed short, one seed over, and one unset. **Two real three-case runs** in both modes, each inside D1, blocked, and leaving exactly one attempted seed: case 0's attempt fails (a 1e-300 spring) and blocks; case 1 blocks before its attempt (invalid category). Each is declined by G-C with `OrdinarySolveNotAttempted` (1, 0). The milestone with C cases passes the fact (3 seeds). **Registered Direct:** both blocked inputs are admitted, give `CompleteGate(OrdinarySolveNotAttempted, 1, 0)`, exact value-route bytes and no successor |
| `b1_sa_gate_bounds_at_c_are_the_stated_expressions` | SF-4's back edge. Every G-B and G-C bound by its fact. The profile-dependent ones are expressions: `profile_bytes(F_T11)`, `F_T11_ORDINARY_SEED`, `F_T11 − F_T11_LATE_CAPTURE` and `text_atoms::{ROW, D_ENV, DIAG_ENV, L_PUB, L_DIAGID, ERR}` times their counts. The cap-only ones are values: C·(160, 67, 553, 66,816, 22,120) and P_final. Also the budgets' forms and B-6 = C × `NOTICE_RESERVE_BYTES`. **RV-Q round 1 reviews this test's expressions** |

## 4. Evidence per acceptance item

| Item | Evidence (registered build, both modes where a run is involved) |
|---|---|
| Caps and census | `every_cap_row_…`; `nested_typed_census_reads_the_roster`; `b1_sa_total_loads_row_…` |
| D1.4 per case | `b1_sa_d1_4_…` (c = 0–4); `every_family_clause_…`'s D1.4 line |
| D1.5, D1.7 per case | `every_family_clause_…`: 5 D1.5 facts and 2 D1.7 facts on the last case; D1.10 on the last case |
| `cap_rows` | `every_cap_row_…`, `actual_inputs_map_each_cap_fact`, `typed_capacity_…`, `b1_sa_total_loads_row_…` |
| G-B, `LATE_FACTS` 10 | `b1_sa_g_b_…` (including the actual seam at c = 1), `every_phase_fact_…`, `late_facts_read_the_actual_owners` |
| G-C at C, EnvelopeResults | `b1_sa_envelope_results_…`, `b1_sa_gate_bounds_…`, `every_phase_fact_…`, `complete_facts_read_the_actual_owners` |
| T-3 (e) | `b1_sa_t3_e_…`; `g_c_declines_…` and `registered_g_c_declines_only_unattempted_solves` (c = 1) |
| Budgets | `structural_budgets_are_u3s`, `b1_sa_gate_bounds_…` |
| Out-of-domain oracles | `admit_grants_…`, `actual_retained_entry_…`, and ST's four facade oracles (all ok at C + 1 = 4) |
| Runner literal and tie | runner test (ok, `_run_records/suites/cand_reg_runner.*`); `b1_sa_runner_oracle_…` |
| Mutants | §6 |
| Suite, bytes, guards | §5 |

The new tests' outcomes are also in `_run_records/suites/cand_sa_nocapture.log`, which carries the row counts printed as `I89_B1_SA_ENVELOPE_RESULTS`.

## 5. Suites against I1 `262bd687f0` (`_run_records/suites/`)

Each suite ran `cargo test --locked --offline --no-fail-fast` in separate targets for base and candidate. **Both builds are registered.** `identity_carries_every_key_in_order` printed one identity text on both, and it is byte-equal to `REGISTERED_PROFILES[0].identity` (`_run_records/identity/`). So the registered-only branches ran.

### 5.1 The suites

| Suite | I1 | Head | Difference |
|---|---|---|---|
| PP, registered (all targets) | 712 ok, 1 failed (`s11g_tests::t13_committed_fallback_uz_is_byte_identical`, the known Mac `t13`), 11 ignored | 719 ok, the same 1 failed, 11 ignored | **+7 new, all ok:** the seven `b1_sa_*` tests. **No other outcome changed.** Every re-based test in §3.1 passes on both sides (`suites_diff.txt`) |
| runner/headless, registered | 85 ok, 2 failed (the known `load_reference` pair) | the same 85 and 2 | none. The re-based oracle passes on both |
| PP witnesses, `--lib witness_ -- --ignored` | 10 passed | 10 passed | The 28 `I65_G5_WITNESS*` lines are identical |
| Compiler warnings, PP and runner | — | — | identical (sorted and counted) |
| Stale (`--cfg=i89_b1_sa_stale`), head, `--lib retained_memory` | — | 50 ok, 10 ignored | Every new test passes in a Stale build. Their registered-only halves skip |

### 5.2 Early runs

`_run_records/early/` holds two runs of `--lib retained_memory`.
- **The first** found the `usize` layout effect (§2.2) and two test-input facts:
  - `cap_maximal` does not solve;
  - load ids must be unique across cases.
- **The second** passed 50/50 before the commit.

### 5.3 c = 1 byte identity

`S/scripts/run_pins.sh` follows I85's method: the pin tests' own output variables wrote the documents on I1 and on the head.

| Document | sha256 (I1 = head) |
|---|---|
| milestone, sparse (U3 private driver and U3G2 Direct) | `ac6986b0680e0df9d88c33a5bf4635372fc3b83cbb9080da44e6d32dbdca59dc` |
| milestone, dense | `6cd1d249e5352aaffbd2b7d7349c74a1d0e0572df35500f66be49c3cad95c9b5` |
| L = 0, sparse | `93c6c86548b9d263cba9d9869010043d23ed1c9f2f304dd9f82eb705eb350876` |
| L = 0, dense | `dbb3d477364248fb9ae15f7b9cff44410c2bffe45f783dd02d96eca663b0ac88` |

- All 6 files are identical between I1 and the head (`_run_records/pins/`).
- Each equals its committed fixture and I85's recorded hash.
- The rest of the c = 1 pin tests (U1, the carrier fixtures, W2-deep) pass in the suite.

### 5.4 Guards (§1)

| Guard | Head |
|---|---|
| `PP-tests/s11f_site_test.rs`: rules 1–6 and 8, the scanner self-checks, `t8_*`, `t10b_*` | 11 passed |
| `PP-tests/retained_precision_admission.rs`, including `legacy_native_and_typed_call_graphs_cannot_acquire_retained_admission` | 5 passed |
| `RE/tests/retained_precision_carriers.rs` | 17 passed |
| In-fence: `the_registered_profile_is_the_only_permit_source`, `admission_bound_adds_r_before_comparing_with_m` (they read `admit`'s source), `challenge_bounds_are_the_profile`, `profile_in_build_record`, and U3's source guards | pass, in the PP suite |

- No guard's expected text changed, and no assertion was weakened.
- I touched neither `lib.rs` nor `retained_product.rs`, so there is no new rule-8 site.

## 6. Mutants (`_run_records/mutants/`)

**Method (I85's):**
- one textual edit each in `MUT` (an archive of the head), then PP's whole `--lib --no-fail-fast` suite, registered, in `WT/targets/i89-b1-sa/mut`;
- the pristine bytes were restored and checked by sha256 after each: `c8ad85ec…` for `retained_memory.rs`, `5cf8d6d5…` for `retained_product.rs`;
- `t13` fails in every run, as on I1, and is excluded.

**The first failing assertion of each killing test is listed in `mutants.json`.**

| Mutant | Edit | Killed by (examples) |
|---|---|---|
| **MA1** D1.4 `< C` | `len() >= LOAD_CASES` | `b1_sa_d1_4_…` "c = 3"; `every_family_clause_…` "C cases: inside D1"; and 5 more |
| **MA2** D1.4 `≤ C + 1` | `len() > LOAD_CASES + 1` | `b1_sa_d1_4_…` "c = 4"; `b1_sa_runner_oracle_…`; `admit_grants_…` "C + 1 cases"; `actual_retained_entry_…`; facade `u3g2_direct_entry_no_w1_refusals_keep_exact_bytes`; `every_family_clause_…` |
| **MA3** the per-case load rows read case 0 only | the census's pre-B1 `index == 0` shape | `actual_inputs_map_each_cap_fact` "a later case at l + 1"; `nested_typed_census_…` "the second case is the longest"; `typed_capacity_…` "the last case's capacity" |
| **MA4** G-B's running total dropped | `CaseLoadsTotal` observes 0 | `b1_sa_g_b_…` |
| **MA5** G-B's total reads the current case only | observes `f.case.primitive_loads.len()` | `b1_sa_g_b_…`; `late_facts_read_the_actual_owners` |
| **MA6** T-3 (e) seeds only | the pre-B1 predicate | `b1_sa_t3_e_…` "1: more seeds than requested cases" |
| **MA7a** T-3 (e) `requested` ignored at G-C | `complete_observations` passes 1 | `b1_sa_t3_e_…` "case 0's attempt fails and blocks" |
| **MA7b** T-3 (e) `requested` ignored in the count | no seed-count comparison | `b1_sa_t3_e_…` "no seed"; `g_c_declines_…`; `registered_g_c_declines_only_unattempted_solves`; facade `u3g2_direct_entry_…` |
| **MA8** T-3 (e) `≥` for `==` | `len() >= requested` | `b1_sa_t3_e_…` "1: more seeds than requested cases" |
| **MA9** EnvelopeResults P_final | `P_FINAL` for `c * P_FINAL` | `b1_sa_envelope_results_…`; `b1_sa_gate_bounds_…`; `every_phase_fact_…` |
| **MA9b** its capacity at P_final | `push_capacity(P_FINAL)` | the same three |
| **MA9c** its text at P_final | `2 * P_FINAL * ROW` | the same three ("2·C·P_final·Text(row)") |
| *MX1* D1.5 case 0 only | `.take(1)` | `every_family_clause_…` "D1.5 on case 2" |
| *MX2* D1.7 case 0 only | `.take(1)` | `every_family_clause_…` "D1.7 on case 2" |
| *MX3* `TotalLoads` observes 0 | — | `b1_sa_total_loads_row_…` "Σ l_i = 385"; `every_cap_row_…` |
| *MX4* the census total keeps the last case only | — | `nested_typed_census_…`, `actual_inputs_…`, `every_cap_row_…`, `b1_sa_total_loads_row_…` |
| *MX5* the seam's add dropped (`retained_product.rs`, in `MUT` only) | — | `b1_sa_g_b_…` "Σ l_i = l_0 at G-B" (the registered actual run) |
| *MX6* B-6 one notice | — | `structural_budgets_are_u3s`; `b1_sa_gate_bounds_…` |
| *MX7* a contract-evidence fact not × C | — | `b1_sa_gate_bounds_…` |
| *MX8* RetainedErrorTextBytes not × C | — | `b1_sa_gate_bounds_…`; `every_phase_fact_…` |
| *MX9* `LoadCasesCapacity` capped at 1 | — | `b1_sa_d1_4_…` "c = 2"; and 6 more |

**21 of 21 killed by assertions; every mutant compiled.** The parked-slot patch has its own negative check (§8).

## 7. The seam's overflow path: proposal (R3 ruling 2; RV109 N-3)

**Today** (ST's seam in `prepared_case_source`): `checked_add`, and on overflow `error = CountRange("late loads total")` and return.
- G-B and the late capture are skipped, and `late_refusal` stays `None`.
- G-C then runs. In `retained_w1`, a case in A gets its notice reserved, and `prepare_case` fails on the capture error.
- **The result is a Preparation fallback with a notice.** With A empty, it is `NoTriggeredCase`.

**Proposal: a typed G-B refusal.** I-P replaces the checked add's two arms with one saturating add:

```rust
self.late_loads_total = self.late_loads_total.saturating_add(case.primitive_loads.len());
```

`check_late` then refuses with `PhaseRefusal { Late, CaseLoadsTotal, observed: u64::MAX, cap: 384 }`. The outcome is `W1Fallback::LateGate`: the exact ordinary bytes, no notice, and the late capture skipped. This is T-3 (d), the same class as every G-B refusal.

**Reasons:**
1. **It is the gates' own overflow rule.** `Bytes` saturates at `u64::MAX` "which exceeds every bound", and `PhaseRefusal` documents its observed value as saturated on overflow. A gate fact's overflow then refuses at the gate that owns the fact.
2. **The cause is typed and recorded** as the fact itself. It is no longer a generic capture-error string that surfaces as `Preparation`.
3. **No notice for a pre-W1 refusal.** The ordinary output is not given a notice for what is an invocation-level gate overflow.
4. **The reading side is done and pinned.** `CaseLoadsTotal` reads `count(late_loads_total)`, and `b1_sa_g_b_…` asserts that a saturated total (`usize::MAX`) is refused at G-B with `(CaseLoadsTotal, u64::MAX, 384)`.
5. **No rule-8 site.** `retained_product.rs` is not in s11f's `RULE8_FILES`. The self-assignment detector also does not count `x = x.saturating_add(…)`, because the right-hand side continues with `.`.
6. **It is unreachable at D1's caps,** since Σ ≤ 384. No committed byte changes either way.

**The change is in SP's file,** so it is I-P's or a ROOT ruling. A pin for it, in the registered build:
- `admit` the milestone, then `permitted_probe(permit)`;
- preset `observer.late_loads_total = usize::MAX` before the observed run;
- assert `late_refusal() == Some(&PhaseRefusal { Late, CaseLoadsTotal, u64::MAX, 384 })` and `error.is_none()`.

Today's code gives `error = CountRange(…)` and no late refusal, so this pin tells the two dispositions apart.

**Keeping today's path is also acceptable,** because it is unreachable. My preference is the typed refusal, for reasons 1–3.

## 8. ROOT's note after R3′: RetainedErrorTextBytes over every case's slots

**SP's head `56c5579f07`** (read only) keeps one `CaseSlot` per requested case. The case being captured is in the capture's own fields, and earlier ones are parked.
- `parked_cases() -> &[CaseSlot]` is `pub(super)`.
- `CaseSlot`'s `error` and `observable_error` are `pub`.
- **So no accessor is missing.**

**These names do not exist at I1,** so the change cannot compile on `b1-a`. I prepared it as **`_run_records/i2_preview/parked_slots.diff`** (53 lines; `retained_memory.rs` and the law tests only).
- **`retained_error_text`** folds the parked slots' `error` and `observable_error` onto the capture's own fields.
- **The new law test `b1_sa_retained_error_text_reads_every_case_slot`:**
  - a two-case milestone run on `prepared_probe` parks case 0 (`cases_seen() == 2`, `parked_cases().len() == 1`);
  - texts of 40 and 9 B are set in the parked slot (`with_case(0, …)`), and 7 and 3 B in the capture's own fields;
  - G-C's `RetainedErrorTextBytes` must read 59.
- **The bound** C·(3m + 1)·Text(err) is unchanged. Its numeric check against SP's producer stays in phase 4.

**The I2 preview** (`S/scripts/i2_preview.sh`; `_run_records/i2_preview/`):
- **The tree** `I2P` = `git archive 56c5579f07` plus my three files from `6b62606778`, plus the patch. SP's diff from I1 and mine are disjoint by file (SP: 6 files, none of mine), so `I2P` without the patch is exactly the merge's tree.
- **With the patch:** PP's whole `--lib` suite gave 562 passed, 1 failed (`t13` only), 11 ignored. The SA tests pass on SP's n-case capture, including T-3 (e)'s blocked three-case runs and their Direct decline, and the new parked-slot test.
- **Negative check:** with `retained_error_text` as at my head, the new test fails at its assertion, `left: 10, right: 59`. The patch was then restored and checked by sha256 (`16cc41b9…`).
- This build used the same toolchain, profile, flags and reviewed inputs as the candidate. Its identity line was not printed in this run.

**Proposed application:** at I2, in the integration step, either ROOT applies `parked_slots.diff` as a commit on `b1` (at `retained_memory.rs`, SA's file), or I-A commits it there right after the merge. It must not land on `b1-a`, where it does not compile.

## 9. For ROOT

1. **No stop fired.** There was no FK, schema, base-reader, reviewed-input or D1-visibility change, no c = 1 byte change, no unexplained row, and no integration conflict: the I2 preview merges by file with no overlap.
2. **The parked-slot patch for I2 (§8)** needs a ruling on who commits it. It is ready and verified on the merge's tree.
3. **The overflow proposal (§7)** needs a ruling. The reading side is done either way.
4. **Interim behaviour on `b1` after I2** (decision 17: nothing leaves the branch).
   - The registered build now admits 2–3-case Direct requests, priced on the c = 1 profile until SQ.
   - At I1's producer, such a request either reaches `retained_w1`'s `Domain` refusal (exact bytes, no notice) or is declined by G-C. That was A1-S-2's expectation. SP's head changes this.
   - I pinned none of the interim W1 outcomes, so SP owns them.
5. **Values for SQ's re-pin list.** I kept the registered atoms' value pins unchanged:
   - `every_phase_fact_…`: `TEXT_TEXT_DIAG_ENV == 68_720_236`, and `(L_PUB, L_DIAGID, ERR, PushCap(D_env)) == (2_599_962, 2_330, 16_384, 16_384)`;
   - `profile_laws_hold_in_this_build`: `D_env == 9_361` and `ROW == 11_474`.
   
   Every bound SA defines is asserted as an expression, so SQ re-pins only these atom values. That is beside A1-N-3's three and PLAN_v2 §3.3's list.
6. **Real-input margins at C = 3,** informational for SQ and RV-Q:
   - W2b-shaped cap-maximal runs have 2,113 / 2,114 rows per case against P_final 2,115, and 6,339 / 6,342 against 6,345 at three cases;
   - the three-case row capacity is exactly its bound, 8,192 = PushCap(6,345).
   
   These hold (≤) with small margins, and the bound is exact by construction.
7. **Test inputs, for SP and SQ.**
   - The ordinary route requires primitive-load ids to be unique across cases (`DUPLICATE_ID`). ST's `beyond_load_cases` copies keep the first case's load ids, and in two oracles its case id too, so those C + 1 runs are blocked ordinary runs. They remain valid D1.4 oracles, being refused at admission, and they pass.
   - Also, `ordinary_seed` merges consecutive same-id cases, so duplicate case ids would give fewer seeds than requested and a G-C decline. That is fail-safe, and validation blocks such requests anyway.
   - `law_tests::cap_maximal` does not solve: it is W2's `not_assessed` input. My multi-case real-run test uses W2b's support shapes.
8. **The census total is a `u32`** to keep the priced layout until SQ (§2.2). SQ may widen it with the profile's regeneration if it prefers. That choice is SQ's.
9. **Phase 4 carries** RetainedErrorTextBytes ≤ 3·(3m + 1)·Text(err) against SP's producer, as briefed. Not done now.
10. **Budget:** about 1.5 h of agent time (13:58–15:30 UTC), much of it waiting for the lock, against 5–8 h.

## 10. Commands

`S/scripts/` is copied, sanitized, to `_run_records/scripts/`.

1. **The base:** `git -C WT/b1-a archive 262bd687f0 projects/chirality-piping ':(exclude)projects/chirality-piping/execution' | tar -x -C S/base`.
2. **The first build and the early runs:** `S/scripts/cargo_cand.sh build_norun product_physics test --locked --offline --no-run`; then `… early_retained_memory{,_2} product_physics test --locked --offline --lib retained_memory -- --nocapture --test-threads=2`.
3. **Commit `6b62606778`, then:**
   - `S/scripts/run_suites.sh base_identity base_reg_pp base_reg_runner base_reg_witness`;
   - `… cand_identity cand_reg_pp cand_reg_runner cand_guards_pp cand_guards_re cand_reg_witness cand_sa_nocapture`;
   - `… cand_stale_retained_memory`.
   
   Each job runs through `t3_cargo.sh` under a 7,200 s alarm. The targets are `WT/targets/i89-b1-sa{,/base,/stale}`.
4. **The pins:** `S/scripts/run_pins.sh`.
5. **The mutants:** `VENV/bin/python S/scripts/mutants.py MUT/P S/logs/mutants WT/targets/i89-b1-sa/mut WT/tools/t3_cargo.sh S/tmp`.
6. **The I2 preview:** `S/scripts/i2_preview.sh`, in the target `WT/targets/i89-b1-sa/i2`.
7. **The records:** `S/scripts/write_records.sh`, which runs `sanitize.py` (I85's, copied).

## 11. Records, cleanup and limits

**`_run_records/`:**
- `sa.diff` and `commits.txt`;
- `scripts/`;
- `suites/` (the logs, `*.outcomes`, `*.warnings`, `suites_diff.txt`, the `run_suites_*.out`; the runner logs filtered);
- `guards/` and `pins/`;
- `mutants/` (filtered logs, `mutants.json`, `mutants.out`);
- `identity/` (the identity runs and the first build log);
- `early/`;
- `i2_preview/` (`parked_slots.diff`, both logs, `i2_preview.out`);
- `cargo_jobs_i89.log`.

**Handling:**
- Machine paths become `WT`, `VENV` and `~`.
- A log line over 4,000 bytes is cut to 1,000 bytes, followed by its length and sha256. Those lines are the producer's committed `cfg(test)` prints.
- No folder is named `build`, and none of these paths is git-ignored (checked with `git check-ignore`).

**SHA256SUMS** covers RETURN.md and every file under `_run_records/`.

**Kept for ROOT, RV-Q and the I2 step:**
- `S`, including `BASE`, `MUT`, `I2P` and the full logs;
- the targets `WT/targets/i89-b1-sa{,/base,/mut,/stale,/i2}` (about 10 GB in all).

ROOT may delete them.

**Limits:**
- **No SP multi-case W1 outcome is pinned here.** The I2 preview ran PP's `--lib` suite only, not its integration tests or the runner.
- **The `TotalLoads` cap + 1** is shown on the census facts, because no request inside the other caps reaches it.
- **RetainedErrorTextBytes' bound** remains I82's assumption until phase 4.
