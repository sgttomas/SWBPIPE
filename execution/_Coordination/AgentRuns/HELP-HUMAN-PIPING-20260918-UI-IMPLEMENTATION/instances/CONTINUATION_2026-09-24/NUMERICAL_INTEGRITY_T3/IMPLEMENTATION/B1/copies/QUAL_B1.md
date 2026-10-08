# B1 SQ: the qualification record for D1 at C = 3 (I104)

*Paths `g6/…` are under `_run_records/g6/`; WT, NUM, R, P, PP as in B1_COMMON.*

**Agent:** I104, TASK (Type 2), I-A for SQ, under ROOT. No descendants.

**Grant:** `R/BRIEFS/B1_SQ.md` (`f6eaa6cc…`) with `B1_COMMON.md` (`2d170307…`); PLAN_v2 §3.1–3.7 (`c85786b7…`) with RV107's A1 amendments; RR "R6a: B1's G5 holds on B1's real code; M = 10.5 GiB, provisional" and the coordinator's G6 message.

**Code:** `WT/b1-q`, `codex/piping-t3-b1-q-20261008`, head **`69002bc862`** (G6), over `b075c5c59f` (G5) over `57c92a7b33`. G6's commit touches six files, all inside the brief's writes (`_run_records/g6/g6_commit.diff`):
- `PP/src/retained_memory.rs`: the regenerated GENERATED PROFILE block (N-5), and G-B's byte bound (SF-1) with its doc;
- `PP/src/retained_memory_law_tests.rs`: the re-pins and the new law tests;
- `PP/src/retained_memory_witness_tests.rs`: the witnesses, one entry point per mode, and the controls;
- `PP/tests/retained_memory_challenge.rs`: the challenge;
- `PP/tests/common/b1_sq_inputs.rs` (new): the shared inputs, pinned by I86's input hashes;
- `PP/tests/s11f_site_test.rs`: E-12.

**Registration is not applied.** `threshold_bytes` is still 4,026,531,840 at the head. `registration.diff` (§8) sets it to M. ROOT applies it after R6b.

## 1. What SQ did

| Obligation | Result |
|---|---|
| G5 at C = 3 (R6a) | RETURN.md; regenerated here with N-5 (§2) |
| M (D-7) | **10.5 GiB = 11,274,289,152 B**, ROOT's R6a ruling (provisional) |
| In-build maximum ≤ 0.9 M, both modes | **Yes:** 0.8693 M sparse, 0.8745 M dense (§3) |
| `admission_bound` at M − R − 1, M − R, M − R + 1 | `admission_bound_adds_r_before_comparing_with_m` at the new M: passes |
| A pure `maximum` test, each of the 7 phases in turn the largest | `maximum_takes_every_phase`: passes |
| G-B's SF-1 bound, with N-1, N-2, N-5 | §7 |
| The re-pins (§3.3, A1-N-3, I89) | §9 |
| E-12 | §10 |
| `registration.diff` in a scratch copy, PP's suite | **741 passed, 1 failed (the known Mac t13), 79 ignored** (§8) |
| The S1 witnesses, both builds | **All pass, in both builds;** none overflowed, aborted or panicked (§4) |
| The release record equals the dev/test record | **equal:** the release law record's atoms, phases and profile lines are byte-identical to the registered dev/test record's (`g6/law/record_dev_vs_release.txt`); 47 of 47 law tests pass in each build (the release build is Stale and prints `I65_G6_RECORD_SKIP`) (§5) |
| QUAL §11's carry | **Carried,** with 4 expressions read by type (§11) (§11) |

## 2. N-5 and B1's other adapter reservations, priced

RV112 N-5: `capture_bytes` includes SP's parked-slot reservation, which I82's emulation had no owner for. G6 prices it and B1's two other new adapter reservations (`g6/tools/sq_n5_chain.py`, applied after I82's per-case scaling, so they are not scaled again):

| Row | Owner (retained_product.rs) | Form | In-build at C = 3 |
|---|---|---|---|
| T11.8 | `park_case`: one reservation of c − 1 `CaseSlot`s, at c ≥ 2 | s(CaseSlot)·(c − 1) | 2 × 1,336 = 2,672 B |
| T11.9 | `bind_observations_by_case`, at the finish and at custody: c `[usize; 2]` each, at c ≥ 2 | s([usize;2])·2c | 96 B |
| T13.3 | `native_call`'s batch: \|A\| `PrimitiveSource`s, at \|A\| ≥ 2 (the sources move in; only the vector is new) | s(PrimitiveSource)·a | 3 × 248 = 744 B |

- **Three new InBuild atoms** (247 in all); every other atom's value is unchanged (`g6/law/atoms_g5_vs_g6.diff`).
- `s(CaseSlot)` is **1,336 B** in this build (RV112 measured 1,560 at its basis).
- **c = 1 is unchanged:** the rows are absent at c = 1, and the chain's c = 1 point equals G5's (`g6/n5/n5_points.json`, under the chain's illustrative strides).
- **E_mov,max moves by +3,512 B** in both modes. TEXT, D and D_env are unchanged.
- **The late form's margin** (N-5's check): one case's late form is 38,016 B in-build; RV112 measured one case's real late capture at about 36,805 B on the solvable cap-maximal input (96.8 %).

## 3. The in-build maximum

Phase totals (requested + moving, without R) in the dev/test build, the pinned record (`g6/law/law_tests.registered_dev.log`):

| Phase | Sparse | Dense |
|---|---|---|
| W1 ordinary span | 5,069,321,390 | 5,128,452,734 |
| W2 G-B, G-C, T12–T15, N1 reserve | 5,392,753,352 | 5,451,884,696 |
| **W3 publication (T16) + staged copy** | **9,733,567,302** | **9,792,698,646** |
| W4 precommit (T17) + successor + invocation | 9,518,381,725 | 9,577,513,069 |
| W5 transfer and Direct completion | 5,964,775,312 | 6,023,906,656 |
| X1 ordinary span with T25 | 8,846,487,786 | 8,905,619,130 |
| X2 X completion | 5,023,851,824 | 5,082,983,168 |

| Direct | Sparse | Dense |
|---|---|---|
| E_req,max (W3 requested) | 9,228,111,683 | 9,287,243,027 |
| W3 moving extra | 505,455,619 | 505,455,619 |
| **E_mov,max** (W3) | **9,733,567,302** | **9,792,698,646** |
| E_mov,max + R | 9,800,676,166 = **0.8693 M** | 9,859,807,510 = **0.8745 M** |
| Below ⌊0.9 M⌋ (10,146,860,236) | 346,184,070 | 287,052,726 |
| Text-error budget (margin / TAV_W 4,301,774,658) | 8.05 % | 6.67 % |

- **The 0.9 M rule holds in both modes** (`profile_laws_hold_in_this_build`). `admit` prices its bound only through `admission_bound` (E_mov,max + R against the registered M), so a build whose layouts moved the maximum above M is refused at admission.
- `ESTIMATES` = 0; `profile_transcribes_the_python_chain_exactly` passes on the regenerated block.

## 4. R and the S1 stack evidence (measured evidence for these builds and inputs, not a proof)

R = 64 MiB; the witness stack is R/16 = 4 MiB. Each witness has one entry point per mode and ran as its own process (`<lib test binary> <name>::<mode> --exact --ignored --test-threads=1`), in the registered dev/test build and in the release build (`g6/witnesses/`).

| Witness | Sparse | Dense |
|---|---|---|
| W1 milestone | Successor | Successor |
| W2 cap-maximal D1 (escaped, raw depth 16) | Fallback(Preparation) | Fallback(Preparation) |
| W2-deep (milestone, escaped, raw depth 16) | Successor at 4 MiB and at 1 MiB | Successor at 4 MiB and at 1 MiB |
| W2b (`…_passed_report_no_triggered_case`) | Fallback(NoTriggeredCase), exact ordinary bytes | the same |
| **W2b's replacement, `b2_k1e3`** (I86) | **Fallback(Candidate)**, after the full native run at the cap-maximal counts | **Fallback(Candidate)** |
| W3 n05 exact-selected (T25) | ExactSelected | ExactSelected |
| W4 preparation refusal | Fallback(Preparation) | Fallback(Preparation) |
| W6 force-scaled: W-C2's case C | Fallback(Native) | Fallback(Native) |
| W6's PHYS-R4 input | Fallback(NoTriggeredCase), exact bytes | the same |
| W7 U3 faults (one mode) | Native; Serializer(Encoding); Staging; Precommit G8; Precommit G1 | |
| Headroom: W1 at R/64 = 1 MiB | Successor | Successor |
| **W-C2** (three cases, A and C in A) | **Successor at 4 MiB and at 1 MiB** | **Successor at 4 MiB and at 1 MiB** |
| W-C2's (A, C) (one mode) | Successor | |
| **c1** (I86; the c = 1 publishing cap-maximal input) | **Successor** | **Successor** |
| **The cap-maximal three-case input** (I86; \|A\| = 3; escaped, raw depth 16; `assess`: inside D1, 3 cases, Σ l_i = 384) | **Successor** | **Successor** |

- **The `NoTriggeredCase` pins** (`b1_t4_w2b_and_w6_phys_r4_inputs_are_no_triggered_case_pins`) and **the inputs' pins** (`b1_sq_inputs_are_i86s_and_the_committed_helpers`: I86's input hashes for `b2_k1e3`, `c1`, the three components and the three-case input; equality with `law_tests::cap_maximal`, W2b's input, W2's input and the facade tests' W-C2) pass in both builds.
- **The three-case input publishes at the caps in both modes,** so the whole W1 transaction (W2 to W5) ran at C = 3 with \|A\| = 3 on the witness stack: the furthest any input reaches.

- **The deepest call chain** from the Direct root on B1's call graph is **40** (callgraph_g5.py's `root_depths`, at c = 1, 2 and 3; `g6/deepest_chain_b1_c3.json` lists one such chain). Unchanged from RV83 N-3: a loop over A adds no recursion. R and k are unchanged.
- **The stated limits** of QUALIFICATION.md §4 stand: carry 8 (the reader's 36-level `$ref` walk is not observed), the panic-hook path, the lexical call graph, the grammar-fixed envelope depth.

## 5. The build identity

**Registered (unchanged identity):** the dev/test build on this host (QUALIFICATION.md §5's identity string, `profile=debug;opt_level=0`), with the same reviewed inputs and reader layouts. Only `threshold_bytes` changes.

**Release:** **equal:** the release law record's atoms, phases and profile lines are byte-identical to the registered dev/test record's (`g6/law/record_dev_vs_release.txt`); 47 of 47 law tests pass in each build (the release build is Stale and prints `I65_G6_RECORD_SKIP`)

## 6. D1 and M

D1 is as amended through B1 SA (C = 3, L = 384; D1.4 admits 1 to C cases). **M = 11,274,289,152 B** (10.5 GiB), ROOT's R6a ruling, provisional until R6b. In this build E_mov,max + R = 0.8745 M (dense). The non-claims of QUALIFICATION.md §6 stand: M is requested and moving heap per invocation, not RSS; no concurrency or supported-machine claim; no stack claim beyond §4.

## 7. G-B's byte bound (RV112 SF-1), N-1, N-2

**SF-1.** G-B's `LateObservationBytes` reads the capture's cumulative tally, which at case k already holds cases 0…k−1's late captures. Its bound is now **T11 − `F_T11_LATE_CAPTURE` / C** (`phase_caps().late[9]`).

| In-build, C = 3 | B |
|---|---|
| T11 | 443,136 |
| T11_late_capture = 3 × 38,016 | 114,048 |
| **G-B's bound: T11 − one case's late capture** | **405,120** |
| The defective bound, T11 − T11_late_capture | 329,088 |
| RV112's measured fact at case 2, solvable cap-maximal three-case input (sparse / dense) | 80,452 / 82,258 |

- **Exactness.** The chain asserts the c = 3 late form is exactly 3 × the c = 1 form, term by term (`g6/sf1_late_form_exact.json`; the ordinary-seed form likewise). The law test `b1_sq_late_capture_form_is_c_times_one_case` checks, in-build, that every coefficient and the constant of `F_T11_LATE_CAPTURE` are multiples of C, that its value is C × the divided form's, that G-B's bound equals T11 less that one case, and that T11 holds the late form. At C = 1 the bound is the old one.
- **The expression test** `b1_sa_gate_bounds_at_c_are_the_stated_expressions` asserts the new expression.
- **N-1.** `b1_sa_runner_oracle_literal_is_load_cases_plus_one` now `include_str!`s the runner's test and holds its one `const C_PLUS_ONE: usize = …;` line to `LOAD_CASES + 1`, so a change of C or of the runner's literal alone fails in PP.
- **N-2.** `b1_sq_retained_error_text_reads_every_parked_slot_at_c`: three milestone cases (two parked slots); 101 B of text in the last parked slot alone must be counted (G04, which folds `parked[..1]`, reads 0); then every slot with the capture's own. **RV112's G04 dies:** on a mutant copy (`parked_cases().iter().take(1)`), SA's two-case test passes and the new test fails, 0 ≠ 101 (`g6/mutant_g04.log`).

## 8. The registration change (prepared, not applied): `registration.diff`

**One hunk:** `threshold_bytes: 4_026_531_840` → `11_274_289_152`, with its comment. The generated block and §9's re-pins are already committed (A1-N-11: the diff is `threshold_bytes`, the block and the re-pins; the coordinator's message keeps the threshold a diff).

**Tested in a scratch copy** (`g6/registration/`): `git archive` of `69002bc862` (`projects/chirality-piping` without `execution/`) with `registration.diff` applied (`g6/tools/mk_regcopy.sh`), built fresh in `WT/targets/i104-sq-reg`:
- **PP's whole suite: 741 passed, 1 failed, 79 ignored.** The failure is the known Mac `s11g_tests::t13_committed_fallback_uz_is_byte_identical`.
- **The law tests all pass** in the registered build (47 of 47), including the 8 that fail at the unregistered head (`g6/law/g6_law_tests.unregistered.log`, `g6/law/law_tests.registered_dev.log`): `admit` grants the milestone a permit in both modes with `required` = E_mov,max + R ≤ ⌊0.9 M⌋.
- **The challenge's default test** (the milestone and W-C2 through the registered Direct entry, both modes) passes in the suite; its per-input peaks are RSS_TIME.md's.
- **No other file needs a change with the registration:** the suite above ran with the one hunk applied and nothing else.

## 9. The re-pins

Every law-test literal derived from M, the caps or the profile (A1-N-3's rule) is re-pinned, as a value here or as SA's expression:

| Test | Re-pin |
|---|---|
| `const M`; new `const MARGIN` = ⌊0.9 M⌋ | 11,274,289,152; 10,146,860,236 (pinned in `profile_laws_hold_in_this_build`) |
| `the_registered_profile_is_the_only_permit_source` | `threshold_bytes` = 11,274,289,152 |
| `admit_grants_…`, `admission_bound_adds_r_…`, `profile_laws_hold_…` (the three 0.9 M margins) | `≤ MARGIN` |
| `profile_in_build_record` | the ratio's denominator M; `PINNED_RECORD` (§3) |
| `challenge_bounds_are_the_profile` | the challenge's W1 and E_mov,max constants; `CAP_BYTES` = 16 GiB above M; the challenge's N1 text is the notice's |
| `every_phase_fact_admits_its_cap_and_refuses_cap_plus_one` (I89) | `TEXT_TEXT_DIAG_ENV` 175,409,684; (L_PUB, L_DIAGID, Text(err), D_env, PushCap(D_env)) = (2,599,962, 2,330, 16,384, 22,911, 32,768); C·(3m + 1)·Text(err) = 4,767,744; P_final 2,115 unchanged |
| `profile_laws_hold_in_this_build` (A1-N-3) | `caps.complete[3]` (D_env) 22,911; Text(row) 11,474 unchanged; the binding counts (16 SourceUpper, 6 Text, 0 Estimate) unchanged |
| `structural_budgets_are_u3s` (A1-N-3) | SA's B-6 expression (`NOTICE_RESERVE_BYTES × C`, per case ≤ 2,048): passes unchanged |
| `b1_sa_gate_bounds_at_c_are_the_stated_expressions` | G-B: T11 − T11_late_capture / C |

## 10. E-12

RR "RV109 passes SP in RV-P round 2; …", ruling 3: `PP/retained_product.rs` is added to s11f's `RULE8_FILES` with its site table. Rule 8's scan finds two accumulations there, both integer:

| Function | Count | Disposition |
|---|---|---|
| `check_support_maps` | 1 | integer: one support's spring-map count |
| `selected_attempts` | 1 | integer: the selected attempts' bit set (T-12's detail) |

s11f passes, 11 of 11 (`g6/s11f.log`). A new accumulation in `retained_product.rs` now fails rule 8 until it is listed.

## 11. Carries and routed items

- **QUAL §11 (RV87 G6r N-1): the by-type non-candidate sweep, carried.** `text_budget.py` with `TB_NONCAND_OUT` on the c = 3 point (`g6/noncand/`). I65's `noncand_compare.py` keys on the multiplicity, which at C = 3 is c times RV87's for every per-case site, so it reports 288 rows as new (exit 1). Matched without the multiplicity (`g6/tools/noncand_compare_nomult.py`): **408 of the run's 412 rows are RV87's**, with only their multiplicities changed; 2 of RV87's 410 are gone (`freeze_candidate`'s and `one_case`'s, which B1 replaced). The 4 new ones, read by type and binding:

  | Site (fn) | Expression, class | Type and binding | Identifier? |
  |---|---|---|---|
  | `retained_product.rs:4362` (`freeze_case`) | `failure.failure()`, error_display | `k::ProductFailure` (static texts, numeric and bridge errors, `Predicate { row, predicate }`); the old `freeze_candidate` row, moved | no |
  | `retained_product.rs:3692` (`native`) | `error`, error_display | `CaptureError`, the batch call's one error cloned per prepared case; priced as error text | no |
  | `retained_wire.rs:1989` (`serialize_attempt`) | `terminal["kind"].as_str().unwrap_or_default()`, to_owned static | the kernel terminal's kind token (a fixed vocabulary); the old `one_case` row, moved | no |
  | `retained_wire.rs:1998` (`serialize_attempt`) | `other`, error_debug | that same token, in `format!("kernel_{other}")` | no |

  **No identifier alias.** The residual stays accepted, as at RV87's G6r; RV-Q reviews the four by type.
- **DEF-O's availability (item 9; RR "RV115 confirms …", NC-1):** report only, in RSS_TIME.md §6.
- **Item 10 (optional), a Direct-entry variant with a fault armed:** not taken.
- **RV83 N-3:** 40, unchanged (§4).
