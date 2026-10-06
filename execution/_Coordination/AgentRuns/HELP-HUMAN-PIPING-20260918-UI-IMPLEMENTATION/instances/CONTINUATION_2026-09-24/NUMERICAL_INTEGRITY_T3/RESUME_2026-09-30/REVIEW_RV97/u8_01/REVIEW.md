# RV97: independent review of U8, round 1 (the probe and U8-1)

**Reviewer:** RV97, TASK (Type 2), dispatched directly by ROOT (HELP_HUMAN, Agent 0). ROOT is the return path. No descendants. I wrote none of the work under review.

**Briefs:** `R/BRIEFS/U8_COMMON.md` (`3146c3e6…`) and `R/BRIEFS/RV97_U8_REVIEW.md` (`80c8f6ac…`, verified), read in full with the repository's root `AGENTS.md`, `agents/AGENT_TASK.md` and `P/AGENTS.md`. I read `R/REVIEW_RV93/u3_grant2_01/` (REVIEW.md, ADDENDUM_01.md and its evidence list) first, as the brief says.

**Round 1 scope (ROOT's dispatch):** review items 1, 2, 3, 4 (except 07l parity, which is not committed yet) and 5, on the probe and U8-1. F-1 is routed to B0/B1; I only confirm that no U8 test or record pins its current behaviour.

**Candidate:** `b1e2d7741e..d449097085` on `codex/piping-f2a-u8-20261005`: one commit, three files, +12,751 / −0:
- `PP/src/retained_facade_tests.rs`: +244 / −0, a new "U8 (I68)" section after the last existing test;
- `P/fixtures/results/retained_precision_l0_successor_{sparse_interactive,dense_scrutiny}.json`: new.

**Implementer accounts read:** `R/I68/u8_probe_01/PROBE.md` (`ba5f7df6…`) and `R/I68/u8_witnesses_01/RETURN.md` (`23340db9…`), with their `_run_records` lists. **Basis read:** `R/I61/u8_plan_01/PLAN.md` §0–§1 (`f274a614…`); RR "Owner decision: F2a's delivery steps move to the end of F2a; erratum E-1", "I61's U8 plan ruled; …", "I68's probe verified: L = 0 publishes; W-C1 is Ceiling; F-1 routed to B0 and B1; Part 2 granted" and "U8-1 committed; I68's readings accepted; I69 and RV97 dispatched"; U5's `R/I61/u5_reference_01/_run_records/` and RV86's extract records.

**Copies and host.**
- `git archive` copies (P without `execution/`) of `d449097085` in `WT/rv97/cand` and of `b1e2d7741e` in `WT/rv97/base`, plus two disposable derivatives of the candidate copy: `WT/rv97/probe` (my probe) and `WT/rv97/mut` (my mutants).
- Targets in `WT/targets/rv97/{probe,cand,cand-stale,mut,checked-json,units-authority}`; logs and scratch in `WT/scratch/rv97_u8_01/`.
- Every cargo job through `WT/tools/t3_cargo.sh` (memory guard PID 5387 up), `--locked --offline`, `CARGO_BUILD_JOBS=4`, `RUST_TEST_THREADS=2`, RUSTFLAGS and CARGO_ENCODED_RUSTFLAGS unset for the registered build (`RUSTFLAGS=--cfg=rv97_stale` for the one Stale run), each under a perl alarm. rustc 1.97.1 `8bab26f4f68e`, aarch64-apple-darwin; VENV Python; node v24 with vitest 4.1.10 from the existing `node_modules` through an untracked symlink in my probe copy only.
- No Git writes (reads with `GIT_OPTIONAL_LOCKS=0`), no installs, no DEC-025, native or solver-at-scale job, nothing in the system temp directory. I did not touch `WT/f2a-u8` or any other TASK's files. I deleted my copies, the symlink and my targets at the end (§ "Cleanup").

**My oracles (I68's tests and tables were only run, never relied on):**
- **My own inputs.** I built the five U8 inputs my own way (truncate, find-by-id, W6 retyped and shifted by +5 m) and compared each with the candidate's builder as a JSON value.
- **My own counters,** compiled into my probe copy only: process-global counts of ordinary runs entered and of G-C consultations, independent of grant 2's tally, read beside it.
- **My own byte oracle,** independent of the facade tests' `with_notice`: the published bytes must be the plain bytes with exactly one insertion, at the end of the diagnostics array, consisting of `,` plus one JSON object equal (as a value) to the N1 notice, and the parsed diagnostics must be the plain ones plus that notice, last.
- **The kernel outcome read directly:** a stage-by-stage private-driver replay (`observed` → `prepare_case` → `solve_native` → `freeze_candidate` → `staged_envelope` → `serialize_frozen` → the Rust reader) that reads `ProductCapture::native`'s `ExecutionOutcome` without any production print, beside a probe-only record on the Direct path.
- **All three readers on my bytes:** Rust (`validate` with the invocation), Python (`validate_retained_precision`, with CLI authorities I built from the candidate copy) and TS (`validateRetainedPrecision` via vitest), with negative controls.
- **U5's pinned `u5_compare.py`** with RV86's extract, plus a declared three-change L = 0 variant of it.
- **29 mutants and 1 survival control** of my own (§2).

## Verdict (round 1): **PASS**

| | Count |
|---|---|
| BLOCKING | 0 |
| SHOULD-FIX | 0 |
| NOTE | 5 |

**The headline:**
- **Every probe fact I re-derived agrees with PROBE.md,** in both modes, on the actual Direct entry and on the private driver: the causes (Candidate, Preparation, Native), the notice counts and bytes, W-C1's `Unresolved(Ceiling)` ladder, the L = 0 successor, and every W-C2 per-case outcome including F-1. Two probe runs are identical.
- **The L = 0 successor is what U8-1 pins.** My live documents are `cmp`-identical to the two committed fixtures (D-U6-5), and the receipt and published-byte hashes equal the test's pins. All three readers PASS them with the invocation (eligible; 25/78/9/1 and 25/78/9/2).
- **Body 0 passes U5's oracle under the unchanged criterion:** STOPS `[]`, 97 class claims per mode, tallies and per-row entries identical to the milestone's. Body 1's 15 rows are 6 input-derived plus 9 exact zeros, with coverage `has_data false, stop [F,F,F,F]`, as PLAN §1.2 states.
- **The witness tests are not vacuous:** **29 of 29 mutants are killed by an assertion of a U8 test, none by a compile error** (17 only by U8 tests), and my survival control survives. Every class the brief names is covered: wrong cause, two notices, plain bytes without the notice, a corrupted body-1 row, and a hook left armed.
- **Nothing weakened, nothing outside the fence:** three fenced files, 0 deleted lines, no production, reader or schema text. The milestone still publishes `ac6986b0…` / `6cd1d249…`. PP registered 708 passed / 1 failed (the Mac t13) / 10 ignored, as I68 recorded; the Stale build passes the three U8 tests through their unregistered branches.
- **Scope truth holds,** and no U8 test or record pins F-1's current behaviour.

## Findings

| # | Sev | Where | Evidence | Remedy |
|---|---|---|---|---|
| N-1 | NOTE | `R/I68/u8_probe_01/PROBE.md` §7 step 9 and §8 ("the reader does not use them", "The reader does not depend on them") | **The TS reader does use the copied wasm assets.** `apps/desktop/src/features/results/retainedPrecision.ts:1120` loads the wasm engine inside G8's invocation binding and converts the request's quantities (node coordinates, section dimensions, moduli, temperatures) through `engine.convertDisplayQuantitiesJson`; `services/hashService.ts:96–101` routes canonical JSON and hashing through the same wasm build. So PROBE.md's TS verdicts (and mine) rest on the assets copied from `WT/sweep-skewpin`. **Mitigation, measured:** that worktree is at `e2b83da584`, and `git diff e2b83da584 b1e2d7741e -- P/core P/apps/desktop/scripts` is empty, so the engine's sources (operation_applier, PP, canonical_json, units) equal the U8 base's. The assets' build revision itself is not recorded anywhere I found. My TS results match Rust and Python exactly, and the TS negative controls refuse (§1.3). | Record-level only: an erratum line for PROBE.md's two sentences. For round 2's TS lane (I71), record the wasm asset hashes and the revision they were built from (or build them), since the 07l TS parity verdict will rest on them too. |
| N-2 | NOTE | `retained_facade_tests.rs` (candidate) :910–915 and :1019–1024, the `!registered()` branches | **Hosted CI (Stale) sees only the plain bytes.** In any build but the registered one, `u8_real_input_fallbacks_append_one_notice` checks only `ONE_RUN` and the plain bytes, and the L = 0 value controls do not run. The real-input causes, the notice bytes and body 0/1's controls are exercised only on the registered host. This is as PLAN §1.2 specified, and it mirrors RV93 N-4. **But** the private driver reaches every one of these facts in any build: my stage replay gets the same stage and the same successor bytes (§1.2), and U8's own D-U6-5 test already uses that driver in its Stale branch. | Optional: a Stale-branch private-driver arm (`observed` → `retained_w1`) asserting the cause and the noticed bytes, and running `u8_l0_value_controls` on the private driver's successor, would give CI coverage at no production cost. Keep the registered-host run as the acceptance evidence. |
| N-3 | NOTE | `R/I61/u5_reference_01/_run_records/u5_compare.py` (pinned, `df4684d3…`) with the current Python reader | **U5's pinned script no longer reproduces U5's report byte for byte.** On the milestone documents my run differs from U5's committed `u5_report.json` in exactly two fields per mode: the reader now reports `numerical_eligible: true, standing: eligible` (U7 set `_IMPLEMENTATION_COMPLETE = True`), where U5 recorded `false` / `needs_recompute` (`evidence/u5/milestone_pinned_vs_U5_report.diff`). With those two fields normalised the reports are equal. No criterion, class, bound or tally moved. | None now. Later U5 replays (07l, B1) should expect this difference; RV93's "byte-identical to U5's" was measured before U7. |
| N-4 | NOTE | Item 4's independent reference | **The pinned U5 script cannot take the L = 0 documents unchanged:** it asserts the invocation is I50's request (`u5_compare.py:64`), and the oracle's `truth` gives every node other than N0 N1's values, and has no `rigid:N2` (its spring lookup raises a `ValueError`, which the script does not catch). I therefore ran a **declared variant** with exactly three changes (`evidence/u5/u5_compare_l0_variant.diff`): (a) the request must equal I50's once the L = 0 node and support are removed, and the removed items must be exactly that node and that support; (b) body 1's rows bypass the oracle and are checked against the exact statics of an unloaded node held only by a rigid support (value +0; class input-derived, or absolute with normalized and bound 0); (c) the observables read the invocation's own model, and the negative controls pick body-0 rows. **Control:** on the milestone documents the variant's report and log are byte-identical to the pinned script's. | **For ROOT:** accept this variant as the records-level U5 replay of body 0 (it extends U5 by statics, not by the oracle), or ask for the oracle to be extended to N2. The variant is ready for reuse on 07l's producer-solved base. |
| N-5 | NOTE | PLAN §1.2, the Native Ceiling column's control; `retained_facade_tests.rs` (candidate) :850–869 | **W-C1's planned control is met by the probe, not by committed code.** PLAN §1.2 lists "pair case A, alone, publishes a successor", which shows the Ceiling comes from case B's loads and not from the model. U8-1 commits `u8_two_body_case_a` only as case B's builder and asserts nothing about case A, deliberately, so that F-1's dense fallback is not pinned (RETURN.md §6.3). The control holds today in **sparse** mode (my probe: successor, receipt `b6fbf65d…`, all three readers PASS). | Optional: a sparse-only committed control would not pin F-1. Otherwise B1's W-C2 commits both cases of the pair, which covers it. |

There are no BLOCKING or SHOULD-FIX findings, and no mutant survived.

## 1. Item 1: the probe's facts, re-derived

### 1.1 The build and the inputs

- **The registered build.** My probe asserts `registered()` (the build script's `OPS_RETAINED_BUILD_IDENTITY` equals the facade tests' `REGISTERED_IDENTITY`, which I checked is the text of `REGISTERED_PROFILES[0]`), and every one of the 18 Direct admission reports reads `Registered`. My instrumentation (`evidence/probe/probe_instrumentation.diff`) is `cfg(test)` only and touches none of the identity's inputs.
- **The inputs.** My five constructions equal the candidate's builders exactly (`first_load_only` `4f0ad6b7…`, `tiny_spring` `1ab95acc…`, two-body case A `ec6c8e65…`, case B `cf688351…`, L = 0 `5e562856…`; sha256 of compact JSON). W6 and the one-body pair are my retyping of `w6_input()` (`retained_memory_witness_tests.rs:181–199`).
- **Determinism.** Two probe runs give identical records, identical `I51_FROZEN_REFUSAL` lines and byte-identical saved documents (`evidence/probe/determinism.txt`).

### 1.2 Outcomes (both modes; every row admitted, refusal and domain `None`; my counters = grant 2's tally = 1 run / 1 G-C; every seam at rest before and after)

"At rest" means `hooks::armed_names()` empty, `hooks::armed() == Armed::default()`, and both `cfg(test)` overrides (the dense ceiling and the reserved stack) `None`.

| Input | Mode | Direct W1 result | Private-driver stage | Kernel outcome (read directly) | Published | My byte oracle |
|---|---|---|---|---|---|---|
| `first_load_only` | sparse | `Candidate` | Candidate: `Proof(… Predicate { row: 80, predicate: SharperExact })` | Selected | plain `bbdea038…` → `103ac053…`, 1 notice | plain + one N1 notice |
| | dense | `Candidate` | the same, row 81 | Selected | `32641012…` → `c8f7e044…`, 1 | plain + one N1 notice |
| `tiny_spring` | both | `Preparation` | Preparation: `capture.error = Association("prepared case custody/permit")`, `preparation_error = None` | not reached | `ca5eab08…` → `7e0231cf…` (mode-independent), 1 | plain + one N1 notice |
| W6 | sparse / dense | `Native` | Native: `NativeUnavailable` | **`Unresolved { reason: Ceiling }`**: p128 Candidate Rejected StopRule (Uy, node 1, translation); p256 VerificationThenCandidate Rejected StopRule (end action m0 I Ux, force); p512 VerificationThenCandidate Rejected Charge (same quantity); p1024 Verification Solved | `acd1e159…` → `7b47c164…`; `e32b68a7…` → `3bc3e888…`; 1 | plain + one N1 notice |
| two-body B (**W-C1**) | sparse / dense | `Native` | Native | **`Unresolved { reason: Ceiling }`**, the same ladder on body 1 (node 3, member 1) | `18033fae…` → `1945e56f…`; `ea729aea…` → `13be90b6…`; 1 | plain + one N1 notice |
| L = 0 | sparse | **successor** | Successor (equals the Direct successor) | Selected | `9b425066…` (126,034 B), receipt `c00cbe76…`, 0 notices; B′ holds | — |
| | dense | **successor** | Successor (equal) | Selected | `5d84fce6…` (127,196 B), receipt `0b4250c8…`; B′ holds | — |
| two-body A | sparse | successor | Successor (equal) | Selected | `6d028521…`, receipt `b6fbf65d…`, 0 notices; B′ holds | — |
| | dense | `Precommit { G8, RETAINED_PRECISION_PREPARATION_MISMATCH }` (**F-1**) | Precommit G8 PREPARATION_MISMATCH (Rust reader on the refused successor) | Selected | `ad094f4f…` → `0586ca95…`, 1 | plain + one N1 notice |
| one-body A (axial) | both | `Candidate` | Candidate: `Proof(… cause: Native(Alpha { block: 0, … }))`, no `I51_FROZEN_REFUSAL` line | Selected | `0bce79f3…` → `6a46dfbf…`; `c415c274…` → `7ca40698…`; 1 | plain + one N1 notice |
| one-body B (transverse) | both | `Native` | Native | `Unresolved { reason: Ceiling }`, W6's ladder | `b498ccd4…` → `e4def0d5…`; `f8acf0b5…` → `bc0186ae…`; 1 | plain + one N1 notice |
| milestone (control) | both | successor | Successor (equal) | Selected | `1d9ba709…` / `7c5fe5c5…` (= RV93's); documents `ac6986b0…` / `6cd1d249…` | — |

**Mechanisms behind the causes:**
- **`first_load_only`:** the product certificate refuses the five torsional-shear rows (`result:stress:M1:{end-i,end-j,quarter-1,midspan,quarter-3}:torsional-shear`, value 2.961883209619453e-06) with predicates `[F,F,T,T]`; `numeric_pass false` (`evidence/probe/probe_frozen_refusals_and_census.txt`).
- **`tiny_spring`:** the ordinary status is `MODEL_INCOMPLETE` with 0 results and `NUMERICAL_INTEGRITY_ASSEMBLY_UNRESOLVED`, so `prepare_owned_case`'s custody check (`retained_product.rs:3325–3328`, `status.mechanics != "MECHANICS_SOLVED"`) refuses. This is RR's recorded reading (an unsolved ordinary run, not a section-preparation refusal).
- **W6 and W-C1:** the ordinary runs read `NUMERICAL_INTEGRITY_CHECKS_PASSED` and `HIGH_DISPLACEMENT_REVIEW`.
- **L = 0:** ordinary `MECHANICS_SOLVED`, Sensitive; 113 / 114 results; census nodes 3, members 1, supports 5, DOF upper 18; required 3,575,778,286 / 3,595,488,734 B (= the milestone's).
- **F-1:** the two-body model is range-scaled (`force_scale_exponent: 518` in both modes) and its dense base has no `sparse_live_path_dense_parity_relative_delta` row; L = 0 and the milestone are not range-scaled, and their dense bases carry exactly one.

### 1.3 The readers on my bytes

| Document | Rust (`validate`, bound) | Python (bound / unbound) | TS (bound / unbound) |
|---|---|---|---|
| L = 0 sparse | PASS, eligible; 25 rel / 78 abs / 9 input / 1 non-qty; publication `79d42693…` | PASS eligible, `eligible` / PASS `needs_recompute`; same counts and publication | the same |
| L = 0 dense | PASS, eligible; 25/78/9/2; `ee5a3f29…` | the same pattern | the same |
| two-body A sparse | PASS, eligible; 25/136/9/1; `6329d114…` | PASS eligible | PASS eligible |
| two-body A dense (the refused successor, receipt `eea87407…`) | **refused G8 `RETAINED_PRECISION_PREPARATION_MISMATCH`** | **PASS eligible**, 25/136/9/1, `c1acd36f…` | **PASS eligible**, the same |
| milestone sparse / dense | PASS, 25/69/3/1 and 25/69/3/2 | PASS eligible (the reader's eligibility is on since U7; N-3) | PASS eligible |

**Negative controls:** one receipt nibble flipped (L = 0 sparse) is refused at G1 `RECEIPT_MISMATCH` by Python and TS; the dense L = 0 document rebound to `sparse_interactive` is refused at G8 `INVOCATION_MISMATCH` by both (`evidence/readers/*negative_controls.log`). The TS lane used the wasm assets copied from `WT/sweep-skewpin` (hashes in `ts_wasm_assets.sha256`; N-1).

**Agreement with PROBE.md:** every outcome, cause, mechanism, hash prefix, count and reader verdict in PROBE.md §0, §2, §3 and §4 that I re-derived agrees with mine. The one disagreement is with PROBE.md's description of the TS lane's dependencies (N-1).

## 2. Item 2: the witness tests are not vacuous

**Real inputs, no hook.** The U8 section arms nothing: its only seam calls are `hooks::armed_names()` before and after each run (`evidence/scope/scope.txt`). The inputs are the candidate's builders, which equal my independent constructions (§1.1), and my probe saw every seam at rest before and after every run (§1.2).

**The harness** (`evidence/tools/rv97_mutants.py`): a `git archive` copy of the candidate, one set of exact-text edits per mutant (each anchor checked unique), PP `--lib --no-fail-fast retained_facade_tests` in the registered build, the pristine files restored and checked by sha256 after every run. A compile error would not count as a kill; there were none. "Past the pin" mutants change only the data the value controls read, after the pin assertion, to show that each value control is live on its own.

| Id | Brief's class | Mutant | First failing U8 assertion | Also killed by |
|---|---|---|---|---|
| F1 | wrong cause | production: the Candidate fallback reports `Native` | first_load_only: "the real input's cause" | `u3_each_stage…`, `u3g2_…w1_fallbacks…` |
| F2 | wrong cause | production: the Preparation fallback reports `Candidate` | tiny_spring: "the real input's cause" | three hooked u3/u3g2 tests |
| F3 | wrong cause | production: the Native fallback reports `Preparation` | w_c1_two_body_case_b: "the real input's cause" | four hooked u3/u3g2 tests |
| F4 | wrong cause | input: tiny_spring's stiffness back to 144 | tiny_spring: "the real input's cause" | **only U8** |
| F5 | wrong cause | input: W-C1's entry runs two-body case A | w_c1_two_body_case_b: "the real input's cause" | **only U8** |
| F6 | (admission) | input: first_load_only gains a combination (D1.4) | first_load_only: "admitted (inside D1)" | **only U8** |
| F7 | two notices | production, no allocation: the last ordinary diagnostic overwritten by the notice, then the notice appended | first_load_only: "the ordinary bytes, then the notice" | three u3/u3g2 tests |
| F8 | two notices | isolated: production reserves and appends two, **and** the test's `with_notice` oracle appends two, so only the count can see it | first_load_only: "exactly one N1 notice" | `u3g2_…w1_fallbacks…` |
| F9 | plain bytes, no notice | production: the Candidate fallback drops its notice | first_load_only: "the ordinary bytes, then the notice" | two hooked tests |
| F10 | plain bytes, no notice | production: the Preparation fallback drops its notice | tiny_spring: "the ordinary bytes, then the notice" | two hooked tests |
| F11 | plain bytes, no notice | production: the Native fallback drops its notice | w_c1_two_body_case_b: "the ordinary bytes, then the notice" | two hooked tests |
| F12 | (counts) | production: a second ordinary run on the Candidate path | first_load_only: "one ordinary run, then G-C once" | `u3_n9_…`, `u3g2_…w1_fallbacks…` |
| F13 | hook left armed | test: a staging fault armed before the loop | first_load_only: "no fault armed before" | **only U8** |
| F14 | hook left armed | test: a staging fault armed after the before-check; Candidate falls back before staging, so it is handed back unfired | first_load_only: "no fault armed after" | **only U8** |
| F15 | hook left armed | production: the Candidate path arms staging on the worker; it is handed back | first_load_only: "no fault armed after" | `u3g2_…w1_fallbacks…` |
| L1 | corrupted body-1 row | production: `result:disp:N2:ux` = 5e-324 in the published successor | "the pinned L = 0 successor"; D-U6-5: "byte for byte" | **only U8** |
| L2 | (B′) | production: the ordinary envelope beside a successor gains a diagnostic | "B′, the ordinary envelope is the plain run" | three milestone tests |
| L3 | (counts) | production: a second ordinary run on the success path | "one ordinary run, then G-C once"; D-U6-5 counts | three u3/u3g2 tests |
| L4 | (admission) | input: the L = 0 request gains a combination | "admitted (inside D1)"; D-U6-5 counts | **only U8** |
| L5 | (W1 falls back) | input: N2 left unsupported | "B′…" (W1 fell back after work began, so the envelope carries a notice); D-U6-5: "did not publish its successor" | **only U8** |
| V1 | corrupted body-1 row | past the pin: N2:ux = 5e-324 in the rows the controls read | "body 1's row is exactly +0" | **only U8** |
| V2 | (body-1 class) | past the pin: N2:ux classified relative-verified | "neither input-derived nor an exact zero" | **only U8** |
| V3 | (body-1 tally) | past the pin: one input-derived row reclassified as an exact-zero absolute row | "body 1's six displacements, its magnitude and its nine reaction rows" (5, 10) ≠ (6, 9) | **only U8** |
| V4 | (body-1 coverage) | past the pin: `has_data` true | "body 1's coverage (extent 0, no free DOF)" | **only U8** |
| V5 | (body 0) | past the pin: `result:disp:N1` moved by one ulp | "bit-identical to the milestone" | **only U8** |
| V6 | (body 0, U5) | past the pin: N1's normalized value moved by 3e-9 relative | "outside the U5 criterion" | **only U8** |
| V7 | (body 0, class) | past the pin: N1 classified input-derived | "the milestone's class claim" | **only U8** |
| D1 | (D-U6-5) | the dense fixture gains a trailing newline | D-U6-5: "byte for byte" | **only U8** |
| D2 | (D-U6-5) | the two fixtures swapped in the test | D-U6-5: "byte for byte" | **only U8** |
| V6c | **control** | past the pin: N1's normalized value moved by 5e-10 relative, inside the criterion | **survives, as expected** (the criterion is not stricter than stated) | — |

**What cannot be killed separately, and why that is acceptable:**
- the fallback test's "no successor" and the L = 0 test's `successor()` agreement follow from the type (`retained()` is `Err` / `Ok`);
- the L = 0 test's "the one publication is the successor" and the D-U6-5 test's hash pins are subsumed by the published-bytes pin and the byte-for-byte comparison that precede them.

**A design observation, not a finding.** In the committed form the value controls run only after the pins, so any change to the successor fails a pin first. They describe the pinned bytes and would guard a future re-pin; V1–V7 show each of them fires on its own. The unregistered branches are exercised only by the Stale run (§3; N-2).

## 3. Item 3: nothing weakened, nothing outside the fence

- **The whole-repository diff** `b1e2d7741e..d449097085` has three paths: `M` `PP/src/retained_facade_tests.rs` (+244 / −0) and two `A` fixtures. The diff contains 0 deleted lines. My archive copies (P without `execution/`) differ in exactly those three paths (`evidence/scope/scope.txt`).
- **No production text changes** in PP, the readers (`result_export`, `core/analysis_runs`, `apps/desktop`) or `schemas/`. No existing test, assertion or pin is touched: the U8 section starts after the last line of `u3g2_d_u6_5_carrier_fixtures_are_the_live_successors`.
- **New fixture names reach no other consumer.** No test or tool enumerates `fixtures/results/`; the only directory walk (`result_export/tests/retained_precision_carriers.rs:1067`) skips `fixtures/` and reads `.rs` files only.
- **The milestone still publishes its pinned successors.** My Direct-entry documents are `cmp`-identical to the U6 carriers (`ac6986b0…`, `6cd1d249…`; receipts `efc1a39b…`, `3e26499f…`), and `u3_permitted_path…`, `u3g2_direct_entry_publishes…` and `u3g2_d_u6_5_carrier…` pass unchanged.
- **Suites on my candidate copy:**
  - registered, PP all targets: **708 passed, 1 failed (`s11g_tests::t13_committed_fallback_uz_is_byte_identical`, the known Mac t13), 10 ignored**, equal to I68's record;
  - Stale (`RUSTFLAGS=--cfg=rv97_stale`), PP `--lib`: 545 / 1 (t13) / 10. The three U8 tests pass through their unregistered branches, where `direct()` asserts the `Stale` status.

## 4. Item 4: the L = 0 base (07l parity deferred to round 2)

- **D-U6-5.** My L = 0 documents (U1's form, ids `u8_l0_isolated_node_<mode>`) are `cmp`-identical to `retained_precision_l0_successor_{sparse_interactive,dense_scrutiny}.json` (`93c6c865…`, `dbb3d477…`). Each fixture's `source` is the published successor and its `invocation.request` is my independently built L = 0 request. The private driver's successor equals the Direct entry's in both modes.
- **Body 0 against the milestone's independent reference** (`evidence/u5/`):
  - **inputs:** the pinned oracle `named_oracle.py` (`b1b58639…`), RV86's extract (`a66a8a49…`, 7,240 B), the candidate copy's Python reader, and my own CLI authorities;
  - **control:** the pinned `u5_compare.py` on my milestone documents reproduces U5's report except the two eligibility fields per mode (N-3);
  - **L = 0:** the declared variant (N-4) on the two L = 0 fixtures-equal documents: **STOPS `[]`**. Per mode, 97 class claims are checked (25 relative within 1e-9 on both readouts, 69 absolute within bound, 3 input-derived), the observable checks pass, both negative controls move to fail, and the 7 `represented:info:StopRuleSharper:fail` rows per mode are information only, exactly as U5's. **Every body-0 per-row entry and every tally is identical to the milestone's** (98 of 98, 99 of 99; `l0_vs_milestone.txt`).
  - **Bit identity,** independently in Python: all 98 / 99 milestone rows are present in the L = 0 successor, in the same order, bit-identical, with identical class, bound, normalized and scale bits.
- **Body 1, against PLAN §1.2:**
  - `body_membership` body 1 = members `[]`, nodes `[2]`;
  - `summary_coverage` body 1 = `has_data false, stop [F,F,F,F]` (body 0: `true`, `[T,T,T,T]`);
  - `body_scales` body 1 all `0000000000000000`; the stop rule lists body 0 only;
  - the 15 rows: the six `result:disp:N2:{ux,…,rz}` are input-derived; `result:disp:N2` (the one non-input translation row), the six `rigid:N2` reaction components and its two magnitudes are absolute-verified with normalized and bound 0. Every value is +0. There is no rotation magnitude row.
- **07l parity:** not reviewed in this round (not committed).

## 5. Item 5: scope truth, and F-1

- **No receipt Ceiling row and no native Current evidence is claimed.** The candidate's added lines mention Ceiling only as W-C1's kernel reason, "recorded by the U8-0 probe and not asserted here" (:833) and in the case-B builder's comment (:871); W-C2 appears only as the name of B1's model (:850). PROBE.md and RETURN.md contain no "native Current", "Current evidence" or receipt-Ceiling-row claim; PROBE.md §4 is headed "B1's record; nothing committed".
- **The Ceiling comments are true today** (§1.2, both modes, both paths), and by decision 3 they are not pinned in code; W-C2 in B1 brings the wire-level assertion.
- **F-1 is not pinned anywhere in U8:**
  - `u8_two_body_case_a` is used only as the builder of case B; no U8 test runs case A or asserts any Precommit, G8, `PREPARATION_MISMATCH` or parity-row outcome (`evidence/scope/scope.txt`);
  - the L = 0 dense fixture is not range-scaled and carries its one parity row, which the current Rust rule and an OQ5-aligned rule both accept;
  - PROBE.md records F-1 as an observation with its dump, and RETURN.md §6.3 states it is not pinned.

## For ROOT

1. **N-4:** accept my declared U5 variant as the records-level U5 replay of body 0 (and for 07l's producer-solved base in round 2), or ask for the oracle to be extended to N2.
2. **N-1:** an erratum line for PROBE.md's two TS-lane sentences, and wasm asset provenance in I71's records for round 2.
3. **N-2, N-3 and N-5** need no ruling.
4. **Round 2:** I am ready to continue onto 07l (I69) and the Rust/TS alignment (I70, I71). I keep no copies or targets: round 2 needs a fresh archive of the new head, and my probe, harness and U5 variant are in `evidence/` for re-use.

## Cleanup

I deleted `WT/rv97/` (all four copies, with the `node_modules` symlink and the copied wasm assets), `WT/targets/rv97/` and `WT/scratch/rv97_u8_01/` after copying the evidence below. Nothing else on the host was touched.

## Evidence (`evidence/`; placeholder paths only)

- `probe/`: `zz_rv97_probe.rs` (my probe module), `probe_instrumentation.diff` (counters, the native record and the module line), `probe_records_run2.txt` (every RV97 record), `probe_table.txt`, `probe_frozen_refusals_and_census.txt`, `probe_documents.sha256` (both runs) and `determinism.txt`.
- `readers/`: `py_check.log` (Python verdicts, body membership, coverage, scales, body-1 rows, bit identity, parity rows), `zz_rv97_probe.test.ts` and `ts_reader.log`, the negative controls, `ts_vitest.out`, `ts_wasm_assets.sha256`, the CLI authorities' hashes and build logs.
- `u5/`: `u5_compare_l0_variant.py` and its diff against the pinned script, the L = 0 report and log, the milestone logs (pinned and variant), `milestone_pinned_vs_U5_report.diff`, `l0_vs_milestone.txt` and `u5_inputs_outputs.sha256`.
- `suites/`: registered PP all-targets and Stale PP `--lib` outcomes and result lines.
- `mutants/`: `mutants.json`, `mutants.out` and `summary.txt` (the harness is `tools/rv97_mutants.py`).
- `scope/scope.txt`: the diff, deleted-line count, hook and F-1 scan, and claims scan.
- `tools/`: `tabulate.py` (probe tables), `py_check.py` (the Python reader and body checks), `rv97_mutants.py` (the mutant harness) and `mutant_summary.py`.
