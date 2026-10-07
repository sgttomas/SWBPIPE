# RV112 (RV-Q), round 1: B1's SA slice (admission at option S3) and the parked-slot patch

TASK (Type 2), RV112, for ROOT (HELP_HUMAN, Agent 0), who is the return path. I am the independent reviewer holding RV-Q for B1. I made no delegation. I am a fresh instance and wrote none of this change. 2026-10-07 UTC.

**The brief:** `R/BRIEFS/RV112_RVQ_ROUND1.md`, sha256 `ce814b2273d265bd2c90fce9a4f8958dfa5e26c9448476279fec7fb45614c721`, verified before use. I read NUM's root `AGENTS.md` and `agents/AGENT_TASK.md` first.

**ROOT's messages during the run:**
- The follow-up landed (`9812c83ded`; I89's FOLLOWUP_01; the records being re-sanitized). It is reviewed here as b1..b1-a (§6), and the records note is N-8.
- The host rule restated: every heavy job under the lock, built test binaries included. Every direct binary run of mine was already under `/usr/bin/lockf -k WT/guard/cargo_job.lock`.

**Placeholders:** WT, NUM, P, PP (= `P/core/product_physics/src`), T, R, RR and VENV, as in the brief. Also:
- `S`: my scratch, `WT/scratch/rv112_rvq_01`, kept for SQ.
- `E`: this folder's `evidence/`.
- **The copies:** `WT/rv112/{i1,sa,b1,head,mut}`. Each is a `git archive` of P without `P/execution`, at I1 `262bd687f0`, SA `6b62606778`, SP's `56c5579f07` (b1) and b1-a's head `9812c83ded` (head; `mut` is the same tree plus my schemata and probe).
- **Targets:** `WT/targets/rv112-*`.
- `RUSTUP` and `CARGO_HOME`: the toolchain and cargo homes, where a log names them.

**Limits kept.**
- **Cargo.** Every cargo job went through `WT/tools/t3_cargo.sh` with `--locked --offline`, `CARGO_BUILD_JOBS=4`, `RUST_TEST_THREADS=2` and `TMPDIR` in `S`. `RUSTFLAGS`, `CARGO_ENCODED_RUSTFLAGS` and `RUSTC_WRAPPER` were unset, except `RUSTFLAGS=--cfg=rv112_stale` for the Stale builds (`E/cargo_jobs_rv112.log`).
- **Direct binary runs** of already-built test binaries (the mutant schemata, §8; the probes and identities, §7) ran under the same lock, with the memory guard checked first.
- **Waits.** One wait per job, ending when the job's process was gone; none remains.
- **Jobs.** I killed no other job.
  - The lock queue was 8–14 deep, and some holders were long vitest and pytest jobs. So I stopped my own phase scripts three times while their lock waiters had not started, and re-queued the same work in fewer acquisitions (`E/tools/phase_rest.sh`).
  - Nothing ran twice to a different result.
  - The last, optional step (b1's witness lines and record) was stopped while waiting, and did not run.
- **Not done:** no DEC-025, sweep, install or Git write, and nothing in the system temp directory.
- **Git reads** used `GIT_OPTIONAL_LOCKS=0`.

## Verdict: **PASS**, with one SHOULD-FIX for ROOT to rule before SQ's registration

- **Counts:** **0 BLOCKING, 1 SHOULD-FIX, 8 NOTE.**
- **The code does what PLAN_v2 §2.3 says,** item by item: the caps, the census, D1.4/D1.5/D1.7 per case, the `cap_rows`, G-B's `CaseLoadsTotal`, G-C at C = 3, T-3 (e), B-6, the oracles and the runner's tie. The parked-slot patch reads every case's slot.
- **c = 1 is byte-identical,** and the registered profile is unchanged in bytes and in its in-build values.
- **Suites:** the PP suites differ from I1 only in the listed new tests (+7 at SA; +13 at the head, SP's 5 included).
- **Mutants:** 39 of mine; 38 are killed at assertions by the candidate's own tests, and the 39th (G04) by my probe (N-2).
- **SF-1 is in the specification,** which SA follows exactly. G-B's byte bound `T11 − T11_late_capture` omits the earlier cases' late captures, which the fact counts at case k ≥ 1. The fix belongs with SQ's regeneration.

## Findings

| ID | Severity | Where | Finding | Proposed |
|---|---|---|---|---|
| SF-1 | SHOULD-FIX (ROOT rules) | `PP/retained_memory.rs` `phase_caps().late[9]`; its test `b1_sa_gate_bounds_at_c_are_the_stated_expressions`; source: I82 STUDY §4.3 via PLAN_v2 §2.3 | G-B's `LateObservationBytes` bound `T11 − T11_late_capture` is not an upper bound of its own fact at case k ≥ 1. The fact is the adapter's cumulative tally, which at case k already holds cases 0…k−1's late captures. At I82's S3 forms the bound is 326,320 against a priced maximum of 402,352 at case 2: 76,032 short, which is (C − 1) × one case's late capture. Fail-safe (a G-B refusal), and **not reached by the real cap-maximal three-case input** (82,258 B at most), because the pre-late forms carry about 115 kB of slack per case. But the gate's bound is then below the derivation it cross-checks (§2.4) | G-B reads T11 minus **one** case's late capture: a per-case late form from SQ's generator, or `F_T11_LATE_CAPTURE / C` with G5 asserting exact ×C. The expression test changes with it. Equal to today's at c = 1. Rule now; implement with SQ's regeneration; it does not block I2 |
| N-1 | NOTE | runner `explicit_headless_refusal_…`; PP `b1_sa_runner_oracle_literal_is_load_cases_plus_one` | The tie runs one way. PP's test holds its own copy of 4 and does not read the runner file. The runner's workspace is always Stale, so its test passes with any literal, in or out of the domain. A change of C is caught; a change of the runner's literal alone is not | PP's test `include_str!`s the runner test and asserts its `const C_PLUS_ONE: usize = 4;` line |
| N-2 | NOTE | `b1_sa_retained_error_text_reads_every_case_slot` | At c = 2 there is one parked slot, so the test cannot tell "every parked slot" from "the first". My G04 (`parked[..1]` only) survives the candidate's whole lib suite. My C = 3 probe kills it (0 ≠ 101) | A three-case variant with text in the last parked slot |
| N-3 | NOTE (SP's file) | PP `lib.rs` `permitted_run` | The seam's comment "G-C carries it; no G-C fact reads it yet" is stale after SA: `OrdinarySolveNotAttempted` reads `requested_cases` | I-P corrects it at the next touch |
| N-4 | NOTE (SP's file; for RV-P) | `retained_product.rs` `prepared_case_source` | At c ≥ 2 the late hook does not test `late_refusal` on entry. A later case's G-B refusal overwrites an earlier one's (the recorded cause is the last refusing case's, not the first's), and later cases' late captures still run after a refusal. The outcome (LateGate: exact bytes, no notice) is unchanged | Keep the first refusal (`get_or_insert`), and skip later captures once refused, or state why not |
| N-5 | NOTE (for SQ) | G5 pricing of T11 | `capture_bytes` includes SP's parked-slot reservation, (C − 1) × `size_of::<CaseSlot>()` = 2 × 1,560 = 3,120 B at C = 3. I82's emulation has no such owner. The real late capture of the cap-maximal case is about 36,805 B against its priced 38,016 (96.8 %), so that form is nearly exact | SQ's G5 prices the reservation in T11 and checks the late form's margin |
| N-6 | NOTE (informational, interim) | the registered build until SQ (ruling 3) | Under today's c = 1 byte forms, the cap-maximal three-case input passes G-B (at most 82,258 ≤ 117,696) and G-C's byte facts (`ObservationBytes` 119,111 ≤ 155,712; `OrdinarySeedBytes` 1,636 ≤ 11,968), both modes, private route. So SP's post-I2 multi-case runs of that input are not refused by these facts | None. For SP's evidence planning |
| N-7 | NOTE (tracking) | SP's seam at `56c5579f07` | `late_loads_total` still uses `checked_add`, giving `CountRange`. RR "I89's SA verified…" ruling 2's saturating add (I85) is not yet on b1. SA's reading side already handles a saturated total, and it is pinned | I85, as ruled |
| N-8 | NOTE (records) | `R/I89/b1_sa_01/` | FOLLOWUP_01.md's sha256 is now `88eaca2b…`, not the `401d40b4…` in ROOT's message. REDACTION_01.md records the change (§3 reworded). RETURN.md is unchanged (`05c2687d…`). SHA256SUMS, SHA256SUMS.followup_01 and SHA256SUMS.redaction_01 all verify | None |

## 1. The candidate, and what I compared

| Range | Commits | Files |
|---|---|---|
| SA: `262bd687f0..6b62606778` | one commit | `PP/retained_memory.rs` (outside the GENERATED PROFILE block), `PP/retained_memory_law_tests.rs`, `P/core/runner/headless/tests/retained_precision_admission.rs`: +569/−85 |
| The follow-up, `56c5579f07..9812c83ded` (b1..b1-a) | `77f4391a85` (the `--no-ff` merge of b1 at SP's `56c5579f07`) and `9812c83ded` (the parked-slot patch) | exactly SA's three files, +598/−87: SA's commit plus the patch (`retained_memory.rs` +6/−2, the law tests +23) |

- **The merge is the automatic one.** `git merge-tree --write-tree 6b62606778 56c5579f07` gives `dd5c2e86a5…`, the recorded merge's tree. SP's 6 files and SA's 3 are disjoint.
- **Fence.** Every hunk is inside SA's fence (PLAN_v2 §1).
- **No D1 item's visibility changes.** The only changed `pub` line is `LOAD_CASES`' value. `TOTAL_LOADS`, `CapFact::TotalLoads`, `PhaseFact::CaseLoadsTotal` and `NestedTypedFacts::total_loads` take their siblings' visibility.

## 2. Item 1: the expressions (SF-4's back edge)

**Method.** I wrote each bound from the specification alone (DESIGN_v2 §6; I82 STUDY §4.3, §2.2; ADDENDUM_01 §1 and §3; PLAN_v2 §2.3), not from SA's code. I then evaluated it with I82's evaluator `b1_eval.py`, unchanged (sha256 `c404e8db…`), on I82's own trees:
- `d1_c1`: c = 1, byte-identical to the registered tree (STUDY §1);
- `d1_c3`: option S3. ADDENDUM_01's `d1_c3` is the same file (`0a1d5257…`).

The atoms are the registered build's in-build values, from I72's law record (`cbf34c52…`). They equal the 244 atoms the head's own build prints (§7). The script and its output are `evidence/oracle/gate_bounds_s3.{py,json}`.

The numbers are SQ's to fix. They are used here only to test the expressions.

### 2.1 G-B (`late_observations`; `LATE_FACTS` 10)

| Fact | SA's bound | Specification | c = 1 | S3 | Verdict |
|---|---|---|---|---|---|
| BuiltNodes, BuiltMembers, BuiltFrameElements, BuiltSupports | n, m, m, g | unchanged (D1.9) | 32 each | 32 each | holds |
| CaseLoads (from `LateFacts.case`) | l | DESIGN_v2 §6 "max_i l_i ≤ l"; STUDY "≤ l (per case)" | 128 | 128 | holds |
| **CaseLoadsTotal** (new; from `capture.late_loads_total`) | L | DESIGN_v2 §6 "Σ ≤ L"; STUDY "a running total ≤ L" | (L = l at c = 1) | 384 | holds |
| Restrained, Springs, Materials | min(6n, Σr), s, 4 + 4 | unchanged | 192, 192, 8 | same | holds |
| LateObservationBytes | T11 − T11_late_capture | STUDY §4.3, copied into PLAN_v2 §2.3 | 117,696 | 326,320 | **SF-1**: below the priced maximum at the last case (402,352) |

The order of the facts is the same in `late_observations` and in `phase_caps().late`.

### 2.2 G-C (`complete_observations`; `COMPLETE_FACTS` 19)

| Fact | SA's bound | c = 1 | S3 | Verdict |
|---|---|---|---|---|
| EnvelopeResults | C·P_final `[N-13]` | 2,115 | 6,345 | holds (I89's real three-case cap-maximal runs: 6,339 and 6,342) |
| EnvelopeResultCapacity | PushCap(C·P_final) | 4,096 | 8,192 | holds |
| EnvelopeResultTextBytes | 2·C·P_final·Text(row) | 48,535,020 | 145,605,060 | holds |
| EnvelopeDiagnostics; its capacity | D_env; PushCap(D_env) (atoms) | 9,361; 16,384 | 22,911; 32,768 | holds |
| EnvelopeDiagnosticTextBytes | 2·Text(diag_env) | 137,440,472 | 350,819,368 | holds |
| EnvelopeMaxStringBytes; DiagnosticIdMaxBytes | L_PUB; L_DIAGID (atoms) | 2,599,962; 2,330 | SQ's TEXT run (not in I82's trees) | holds as expressions |
| ContractEvidenceStatus | 0 | 0 | 0 | holds |
| ContractEvidence ArrayElements, Objects, Entries, StringBytes, KeyBytes | C × (3m + 2g, 3 + m + g, 9 + 15m + 2g, m(128 + 1024 + 3·120) + (2m + g)·128 + g(128 + 64), (9 + 15m + 2g)·40) | 160, 67, 553, 66,816, 22,120 | 480, 201, 1,659, 200,448, 66,360 | holds (STUDY: "≤ c × today's per-case preview facts") |
| SourceBlockRecovery | 0 | 0 | 0 | holds |
| ObservationBytes | T11 | 155,712 | 440,368 | holds as the expression (N-5: SP's parked-slot reservation is not in I82's emulated T11) |
| OrdinarySeedBytes | T11_ordinary_seed | 11,968 | 35,904 | holds |
| RetainedErrorTextBytes | C·(3m + 1)·Text(err) | 1,589,248 | 4,767,744 | holds. It is I82's assumption; phase 4 checks it against SP's producer |
| OrdinarySolveNotAttempted | 0, over every requested case (T-3 (e)) | 0 | 0 | holds (§4) |

### 2.3 The `cap_rows` and the budgets

**The `cap_rows`** (`CAP_ROWS` 47):
- `LoadCasesCapacity` ≤ C = 3;
- `Loads` and `LoadsCapacity` ≤ l = 128, read from the census's maxima over every case;
- the new `TotalLoads` ≤ L = 384, right after them;
- D1.11's `ControlBytes` stays the last row.

**The budgets:**
- **B-6** = `NOTICE_RESERVE_BYTES` × C = 886 × 3 = 2,658. `NOTICE_RESERVE_BYTES` = 144 + 2·(30 + 128 + 12) + 30 + 4 + 20 + 196 + 24 + 128 = 886. That is exactly I82's c = 1 `NOTICE` form (718 + s(Diagnostic) + s(String)), and 2,658 is exactly I82's S3 `NOTICE` form (2,154 + 3·s(Diagnostic) + 3·s(String)), so B-6 is the S3 form, × |A| ≤ C (STUDY §4.3).
- **B-1** stays one ordinary run (T-13).
- **B-2 to B-5** keep their named forms (`F_STAGED`, `F_SUCC`, `F_INVOC`, `t17`, `F_STATICS`), so they take SQ's regenerated values.
- **The others** are unchanged.

**Pricing** keeps its form, as decision 17 requires: `cap_priced_maximum` and `admission_bound` are byte-unchanged (§7).

**The expression test.** `b1_sa_gate_bounds_at_c_are_the_stated_expressions` asserts every bound above by its fact, in table order. My mutants E01–E09, D03, C01–C03 and H01 each fail it, or a sibling test, at an assertion (§8).

### 2.4 SF-1: G-B's byte bound omits the earlier cases' late captures

**What the fact counts.** G-B's `LateObservationBytes` is `capture_bytes(capture)`: the adapter's cumulative `RustCapacityBytes` tally.
- The tally never decreases.
- SP's parking (`park_case`, `swap_case`) moves a case's slot without an event, so a parked case's bytes stay counted.

So, at case k's G-B, the fact holds:
- the invocation part;
- the pre-late capture of cases 0…k;
- the late old-source capture of cases 0…k−1;
- at c ≥ 2, the parked-slot reservation.

**What the bound must cover.** The maximum over k is at k = C − 1: invocation + C·pre + (C − 1)·late. That is T11 minus **one** case's late capture.

**What SA's bound covers.** I82's S3 tree prices `T11_late_capture` as exactly C × one case's: all 10 of its terms scale by 3. T11 is affine in c: T11(c) = 13,384 + 142,328·c over the c = 1–4 trees, with a per-case late capture of 38,016 and a per-case pre-late part of 104,312. So:

| | B |
|---|---|
| SA's bound at S3: T11 − T11_late_capture = invocation + C·pre | 326,320 |
| Priced maximum at case 0 | 117,696 |
| Priced maximum at case 1 | 260,024 |
| **Priced maximum at case 2** | **402,352**: 76,032 over the bound, which is (C − 1) × 38,016 |
| T11 − one case's late capture | 402,352 |

**The probe confirms the mechanism on SP's real n-case capture at the head:** `zz_rv112_g_b_bytes_per_case`, on the private route, so no gate skips a capture (`E/probe/probes.filtered.log`). On the solvable cap-maximal input (W2b's shape, 128 loads per case), sparse / dense:

| c | The fact at each case's G-B (B) | At G-C (B) |
|---|---|---|
| 1 | 2,442 / 3,044 | 39,247 / 39,849 |
| 2 | 2,442, 41,447 / 3,044, 42,651 | 78,284 / 79,488 |
| 3 | 2,442, 43,007, **80,452** / 3,044, 44,211, **82,258** | 117,305 / 119,111 |

- **Case 0's site does not depend on c.**
- **From case 1 on,** the fact includes the parked-slot reservation (2 × 1,560 B at c = 3) and each earlier case's late capture.
  - At c = 3 each later site adds 37,445 B (sparse; dense 38,047): one late capture plus one case's pre-late part.
  - The late capture is about 36,805 B: the growth after G-B at c = 1, in both modes.
  - So **at case 2, the two earlier cases' late captures are about 73.6 kB of the 80,452 B fact (91 %).**
- **The milestone has the same shape at small sizes:** 780; then 780, 6,137 and 8,374 at c = 3, sparse.

**Against the priced forms,** the late capture is 96.8 % of its form (36,805 of 38,016). The pre-late part, with the invocation part, is about 2 % of its form (2,442 of 117,696). So on this input SA's S3 bound (326,320) is not reached, but only because of that slack.

**Effect.**
- **Fail-safe:** a G-B refusal publishes the exact ordinary bytes, with no notice.
- **The bound is below the derivation it cross-checks.** At I82's S3 forms, the derivation's own maximum for the fact (402,352) exceeds the gate's bound (326,320). G-B "cross-checks the derivation": a fact above its bound is meant to mean the derivation missed something, so this would be a false alarm.
- **Whether a real input is refused is a matter of slack.** On the measured cap-maximal three-case input it is not (82,258 ≪ 326,320): the pre-late forms are about 2 % used. It could become reachable if SQ's real-code G5 tightens the pre-late forms, or for an in-domain input whose pre-late capture approaches its form. The case most at risk is the last case of the cap-maximal shape S3 was selected to admit (ADDENDUM_01 §4: "W-C2 (C = 3) fits exactly").
- **At c = 1 nothing changes.** Every committed input is unaffected.

**Where it comes from.** STUDY §4.3 states the row as "≤ T11 − T11_late_capture, both at the tier's forms. These are per case × c". PLAN_v2 §2.3 carries it, and SA implements the PLAN's text exactly. So this is a specification error that SA inherited, not an implementation slip. Its correction needs ROOT's ruling (§10).

**Proposed correction:** T11 minus one case's late capture.
- Either SQ's generator emits a per-case late form (for example `F_T11_LATE_CAPTURE_CASE`) and G-B reads `F_T11 − F_T11_LATE_CAPTURE_CASE`;
- or G-B reads `F_T11 − F_T11_LATE_CAPTURE / C`, with SQ's G5 asserting that the late form is exactly C × one case's.

Both equal today's bound at c = 1. The expression test changes with the bound.

## 3. Item 2: D1.4, D1.5 and D1.7 per case; the census; the `u32`

**D1.4.** `load_cases.is_empty() || len > LOAD_CASES` refuses with `(Invocation, LoadCases)`. That is 1 ≤ c ≤ 3, in its old place: after D1.3's namespace clauses, before combinations and components. Combinations and components stay 0.

**D1.5.** The five facts are checked for every case, in request order: `pressure_regions`, `equivalent_static`, `modulus_basis_ref`, `modulus_basis_temperature` and `analysis_state`.
- So every case has the default basis, and one stiffness. DESIGN_v2 T-8's one group rests on this.
- A refusal names the first case, and within it the first fact.

**D1.7.** It reads `load_cases.iter().flat_map(primitive_loads)`: every case's loads. D1.10 (`provenance_clause`) already read every case.

**The census.**
- `primitive_loads.length` is the maximum over cases, and so is `primitive_loads.capacity`, taken separately. So `Loads ≤ l` and `LoadsCapacity ≤ l` mean "every case's length and capacity ≤ l".
- `total_loads` = Σ l_i.
- c = 0 leaves both at 0, and D1.4 refuses.
- My mutants A01–A05 (case 0 only, length only, capacity only, the total as a maximum, the total as case 0's) are each killed (§8).

**The `u32`.**
- **The overflow path is checked.** `u32::try_from(len)` and `checked_add` give `CensusStatus::ArithmeticOverflow`. `walk` returns it as the nested census's status, and `domain_clauses` runs `census_complete_part` first, so the request is refused at D1.2, `Census(TypedNested, ArithmeticOverflow)`, before D1.4 or any row.
- **It is unreachable.** It needs more than 4.29 × 10⁹ typed primitive loads in memory, far beyond the raw census's 16,384 values. So no test can reach it, and an unchecked-sum mutant would be equivalent within memory (not run).
- **The layout is unchanged.** The head's `NestedTypedFacts` is 240 B (probe). The in-build profile record, all 244 atoms with `s(ThreadPacketOutput)` = 1,704, both modes' phases and the five budgets, is byte-identical at I1 and SA (§7). I89's `usize` note is borne out.

**The G-B side is a separate count:**
- the seam's running total (`usize`) is converted by `count()` to `u64`, saturating;
- `b1_sa_g_b_…` pins that a saturated total (`usize::MAX`) refuses at G-B with `(CaseLoadsTotal, u64::MAX, 384)`;
- at b1 `56c5579f07` the seam still uses `checked_add` with `CountRange`. RR "I89's SA verified…" ruling 2 assigns the saturating add to I85; it is not yet on b1 (tracking only).

## 4. Item 3: T-3 (e)

**The predicate matches DESIGN_v2 T-3 (e) term for term.** `ordinary_solve_attempted(capture, requested)` is `requested >= 1 && capture.ordinary.len() == requested && every seed's initial.is_some()`.
- `complete_observations` passes `f.requested_cases`.
- `permitted_run` (PP `lib.rs`, ST's seam) sets it from the typed request's `model.load_cases.len()`, before the request moves into the observed run.

**Count equality means one seed per requested case** in this producer:
- seeds are keyed by case id in run order (`ordinary_seed` pushes a new seed only when the id differs from the last one);
- the ordinary route's `validate_ids` rejects duplicate `load-case` ids before any solve.

**c = 1 is unchanged.** One case id gives at most one seed, so `len() == 1` is `!is_empty()`.

**Mutants.** F01–F07 (I1's predicate, `requested` ignored at G-C, ≥, ≤, `requested ≥ 1` dropped, any for all, compared with C) are each killed (§8). I89's real blocked three-case runs (k = 0 and k = 1 < c − 1) are declined at G-C with exact bytes and no notice, and pass in my runs (§9).

## 5. Item 4: the out-of-domain oracles at `LOAD_CASES + 1`, and the runner's literal

**The in-crate oracles:**
- the law tests' `admit_grants_…` site asserts `(Invocation, LoadCases)` at `LOAD_CASES + 1`;
- `retained_memory.rs`'s `actual_retained_entry_dispatches_ordinary_once` builds `LOAD_CASES + 1` cases and now asserts D1.4's refusal;
- ST's four facade oracles take `caps::LOAD_CASES + 1` through their helper, and pass at 4.

**The runner.**
- It builds a literal 4 (`const C_PLUS_ONE`), with a comment naming `caps::LOAD_CASES` + 1 and the tie test.
- It asserts exact ordinary bytes, no successor and `typed.load_cases.length == 4`.
- No D1 item's visibility changed (§1).

**The tie.** `b1_sa_runner_oracle_literal_is_load_cases_plus_one` asserts `LOAD_CASES + 1 == 4` and that `admit` refuses `LOAD_CASES + 1` milestone cases at D1.4, in both modes.

**My probe.** `zz_rv112_runner_input_is_refused_at_d1_4` builds the runner's exact input: the same fixture, with three more copies of its case and the ids unchanged. The producer refuses it at D1.4 with `(Invocation, LoadCases)` in both modes, and grants no permit in any build. The same shape at C cases (identical ids) is inside D1, because D1 has no id-uniqueness clause.

**N-1.** The tie runs one way. It catches a change of C, but not a change of the runner's literal:
- the PP test holds its own copy of 4 and does not read the runner file;
- the runner's workspace is always Stale (its lock is not PP's reviewed one), so the runner test passes with any literal, in or out of the domain, because no build there is granted a permit.

A source-text tie would close it: the PP test `include_str!`s the runner test and asserts its `const C_PLUS_ONE: usize = 4;` line, as other PP law tests read their sources.

## 6. Item 5: the parked-slot patch (`9812c83ded`)

**The code.**
- `retained_error_text` folds every parked slot's `error` and `observable_error` onto the capture's own fields (the last case seen).
- Under SP's T-2 this covers every requested case's slot at G-C. G-C is consulted outside `with_case`, so no slot is out on loan (`permitted_run`).
- `g5a_error` holds only `&'static str` and integer facts, so it owns no text.
- The bound C·(3m + 1)·Text(err) is unchanged; phase 4 checks it.

**The law test** `b1_sa_retained_error_text_reads_every_case_slot` (two cases, one parked slot; 40 + 9 + 7 + 3 = 59):
- fails against the unpatched reader (my G01: 10 ≠ 59), and against the parked-only reader (G02) and the error-only reader (G03);
- **N-2:** with one parked slot, it cannot tell "every parked slot" from "the first parked slot". My G04 (a reader that folds `parked[..1]` only) survives the candidate's whole lib suite (562 passed; only `t13` failed). My probe `zz_rv112_retained_error_text_reads_every_slot_at_c`, at C = 3 with two parked slots, kills it: it puts 101 B in the second parked slot alone. It passes on the candidate. A three-case variant of the law test would pin "every".

**Scope.** The patch commit touches only `retained_memory.rs` (+6/−2) and the law tests (+23), and nothing else is in b1..b1-a beyond SA's commit.

## 7. Item 6: c = 1 byte identity and the registered profile

**Bytes of the profile** (`E/identity/generated_block_and_registered_profiles.txt`). These are byte-identical at I1, SA, b1 and the head:
- the GENERATED PROFILE block (1,250 lines, sha256 `8618e8cb…`);
- `REGISTERED_PROFILES` (`1789d67d…`);
- `text_atoms` (`18cea384…`).

**Its in-build values** (`E/record/`). `profile_in_build_record` and `structural_budgets_are_u3s`, run with `--nocapture`, print 267 `I65_G5_*` lines: the 244 atoms, both modes' profile and phases, and the five budgets.
- **I1 and SA are byte-identical.** The atoms equal I72's law record, the one I82's evaluator used.
- Dense W3 is 3,595,488,734 B, as registered.
- The head's record is byte-identical too.

**The builds are registered** (`E/identity/run_*`). Each copy's own built lib test binary printed its compiled identity:
- I1, SA and the head print exactly `REGISTERED_PROFILES[0].identity`;
- their Stale builds differ only in `rustflags=--cfg%3Drv112_stale`;
- the reviewed-input lines are identical across copies;
- my `mut` build reports `registered=true`.

**c = 1 byte identity.** The c = 1 successor pins compare the live output with committed fixtures, byte for byte. Each passes at I1, SA and the head, registered:
- `u3_permitted_path_publishes_the_pinned_successor`;
- `u3g2_direct_entry_publishes_the_pinned_successor`;
- `u3g2_d_u6_5_carrier_fixtures_are_the_live_successors`;
- `u8_l0_isolated_node_publishes_pinned_successor`;
- `u8_d_u6_5_l0_fixtures_are_the_live_successors`;
- `u3_r2_notice_bytes_are_pinned`;
- `b1_t4_two_body_case_b_is_a_no_triggered_case_pin`.

So every c = 1 successor the candidate publishes equals I1's committed bytes. `challenge_bounds_are_the_profile` passes on both sides too.

## 8. Item 7: mutants

**Method** (`E/mutants/`, `E/tools/schemata.py`, `E/mutants/mut_vs_head.diff`).
- **The schemata.** In `mut` (head plus my edits), each runtime mutant is an exact, unique textual schema switched by the environment variable `RV112_MUT=<id>`.
  - `phase_caps` loses `const` there; it is only called at run time, and no value changes.
  - The pristine control (variable unset) gives 562 passed, 1 failed (the known Mac `t13`) and 11 ignored. That equals I89's follow-up count, and the source-text tests are undisturbed.
- **Each mutant** ran PP's whole lib suite (minus my probes), registered, from one build, under the host lock.
- **H01 (const)** had its own build. The source was then restored and checked by sha256 (`e2e5b84f…`, the schemata state).
- **"Killed"** means a test other than `t13` fails at an assertion. Every mutant compiled.

**Overlap with I89's 21** (RETURN §6):
- mine cover PLAN_v2 §2.3's list (A01, B01, B02, D01, D02, E01, F01, F02, F03) and most of I89's extras, in my own forms;
- new beyond I89's are A02–A05, B03, B05, C01, C03, D03, D04, E02–E08 taken separately, F04–F07 and G01–G04.

| Mutant | Source | Edit | Outcome | Killed by (test: first failing assertion) |
|---|---|---|---|---|
| A01 | PLAN | census: per-case load rows read case 0 only (length and capacity) | KILLED | `actual_inputs_map_each_cap_fact` (retained_memory_law_tests.rs:607:5: assert_eq: a later case at l + 1); `nested_typed_census_reads_the_roster` (retained_memory_law_tests.rs:418:5: assert_eq: the second case is the longest); `typed_capacity_and_units_rows_read_the_actual_owners` (retained_memory_law_tests.rs:718:5: assert_eq: the last case's capacity) |
| A02 | RV112 | census: Loads length from case 0 only (capacity still the maximum) | KILLED | `actual_inputs_map_each_cap_fact` (retained_memory_law_tests.rs:607:5: assert_eq: a later case at l + 1); `nested_typed_census_reads_the_roster` (retained_memory_law_tests.rs:418:5: assert_eq: the second case is the longest) |
| A03 | RV112 | census: LoadsCapacity from case 0 only (length still the maximum) | KILLED | `nested_typed_census_reads_the_roster` (retained_memory_law_tests.rs:419:5: assert_eq: the third case has the largest capacity); `typed_capacity_and_units_rows_read_the_actual_owners` (retained_memory_law_tests.rs:718:5: assert_eq: the last case's capacity) |
| A04 | RV112 | census: Σ l_i is the maximum length, not the sum | KILLED | `actual_inputs_map_each_cap_fact` (retained_memory_law_tests.rs:608:5: assert_eq); `b1_sa_total_loads_row_admits_l_and_refuses_l_plus_one` (retained_memory_law_tests.rs:1588:5: assert_eq); `every_cap_row_admits_its_cap_and_refuses_cap_plus_one` (retained_memory_law_tests.rs:569:9: assert_eq: TotalLoads at its cap); `nested_typed_census_reads_the_roster` (retained_memory_law_tests.rs:411:5: assert_eq) |
| A05 | RV112 | census: Σ l_i counts case 0 only | KILLED | `actual_inputs_map_each_cap_fact` (retained_memory_law_tests.rs:608:5: assert_eq); `b1_sa_total_loads_row_admits_l_and_refuses_l_plus_one` (retained_memory_law_tests.rs:1588:5: assert_eq); `every_cap_row_admits_its_cap_and_refuses_cap_plus_one` (retained_memory_law_tests.rs:569:9: assert_eq: TotalLoads at its cap); `nested_typed_census_reads_the_roster` (retained_memory_law_tests.rs:411:5: assert_eq) |
| B01 | PLAN | D1.4: refuses c ≥ C (admits c < C only) | KILLED | `actual_inputs_map_each_cap_fact` (retained_memory_law_tests.rs:605:5: assert_eq: a later case at l); `b1_sa_d1_4_admits_one_to_c_load_cases` (retained_memory_law_tests.rs:1552:13: assert_eq: c = 3 SparseInteractive); `b1_sa_t3_e_counts_the_requested_cases` (retained_memory_law_tests.rs:1732:13: assert_eq: case 0's attempt fails and blocks: inside D1); `b1_sa_total_loads_row_admits_l_and_refuses_l_plus_one` (retained_memory_law_tests.rs:1596:9: assert_eq: Σ l_i = 384); `every_cap_row_admits_its_cap_and_refuses_cap_plus_one` (retained_memory_law_tests.rs:563:5: assert_eq: the cap-maximal C-case input is inside D1); `every_family_clause_refuses_with_its_fact` (retained_memory_law_tests.rs:500:5: assert_eq: C cases: inside D1); `typed_capacity_and_units_rows_read_the_actual_owners` (retained_memory_law_tests.rs:714:5: assert_eq) |
| B02 | PLAN | D1.4: admits c ≤ C + 1 | KILLED | `u3g2_direct_entry_no_w1_refusals_keep_exact_bytes` (retained_facade_tests.rs:676:13: assert_eq: C + 1 cases SparseInteractive); `admit_grants_a_permit_for_the_milestone_in_the_registered_build` (retained_memory_law_tests.rs:366:9: assert_eq: C + 1 cases); `b1_sa_d1_4_admits_one_to_c_load_cases` (retained_memory_law_tests.rs:1552:13: assert_eq: c = 4 SparseInteractive); `b1_sa_runner_oracle_literal_is_load_cases_plus_one` (retained_memory_law_tests.rs:1574:9: assert_eq: SparseInteractive); `every_family_clause_refuses_with_its_fact` (retained_memory_law_tests.rs:464:5: assert_eq: C + 1 cases); `actual_retained_entry_dispatches_ordinary_once` (retained_memory.rs:3065:13: assert_eq: C + 1 cases: outside D1 at D1.4) |
| B03 | RV112 | D1.4: admits c = 0 (the empty check dropped) | KILLED | `b1_sa_d1_4_admits_one_to_c_load_cases` (retained_memory_law_tests.rs:1552:13: assert_eq: c = 0 SparseInteractive); `every_family_clause_refuses_with_its_fact` (retained_memory_law_tests.rs:465:5: assert_eq: 0 cases) |
| B04 | RV112 | D1.5: reads case 0 only | KILLED | `every_family_clause_refuses_with_its_fact` (retained_memory_law_tests.rs:508:9: assert_eq: D1.5 on case 2: pressure_regions) |
| B05 | RV112 | D1.5: skips case 0 | KILLED | `every_family_clause_refuses_with_its_fact` (retained_memory_law_tests.rs:470:5: assert_eq) |
| B06 | RV112 | D1.7: reads case 0's loads only | KILLED | `every_family_clause_refuses_with_its_fact` (retained_memory_law_tests.rs:514:5: assert_eq: D1.7 on case 2) |
| C01 | RV112 | cap_rows: TotalLoads capped by l (128), not L | KILLED | `actual_inputs_map_each_cap_fact` (retained_memory_law_tests.rs:605:5: assert_eq: a later case at l); `b1_sa_total_loads_row_admits_l_and_refuses_l_plus_one` (retained_memory_law_tests.rs:1596:9: assert_eq: Σ l_i = 384); `every_cap_row_admits_its_cap_and_refuses_cap_plus_one` (retained_memory_law_tests.rs:563:5: assert_eq: the cap-maximal C-case input is inside D1); `typed_capacity_and_units_rows_read_the_actual_owners` (retained_memory_law_tests.rs:714:5: assert_eq) |
| C02 | RV112 | cap_rows: LoadCasesCapacity capped by 1 (D1's), not C | KILLED | `actual_inputs_map_each_cap_fact` (retained_memory_law_tests.rs:605:5: assert_eq: a later case at l); `b1_sa_d1_4_admits_one_to_c_load_cases` (retained_memory_law_tests.rs:1552:13: assert_eq: c = 2 SparseInteractive); `b1_sa_t3_e_counts_the_requested_cases` (retained_memory_law_tests.rs:1732:13: assert_eq: case 0's attempt fails and blocks: inside D1); `b1_sa_total_loads_row_admits_l_and_refuses_l_plus_one` (retained_memory_law_tests.rs:1596:9: assert_eq: Σ l_i = 384); `every_cap_row_admits_its_cap_and_refuses_cap_plus_one` (retained_memory_law_tests.rs:563:5: assert_eq: the cap-maximal C-case input is inside D1); `every_family_clause_refuses_with_its_fact` (retained_memory_law_tests.rs:500:5: assert_eq: C cases: inside D1); `typed_capacity_and_units_rows_read_the_actual_owners` (retained_memory_law_tests.rs:714:5: assert_eq) |
| C03 | RV112 | cap_rows: TotalLoads observes the largest case, not Σ | KILLED | `b1_sa_total_loads_row_admits_l_and_refuses_l_plus_one` (retained_memory_law_tests.rs:1596:9: assert_eq: Σ l_i = 385); `every_cap_row_admits_its_cap_and_refuses_cap_plus_one` (retained_memory_law_tests.rs:569:9: assert_eq: TotalLoads at its cap) |
| D01 | PLAN | G-B: running total dropped (observes 0) | KILLED | `b1_sa_g_b_reads_each_case_and_the_running_total` (retained_memory_law_tests.rs:1631:9: assert_eq) |
| D02 | PLAN | G-B: running total is the current case only | KILLED | `b1_sa_g_b_reads_each_case_and_the_running_total` (retained_memory_law_tests.rs:1631:9: assert_eq); `late_facts_read_the_actual_owners` (retained_memory_law_tests.rs:1076:5: assert_eq) |
| D03 | RV112 | G-B: CaseLoadsTotal bounded by l, not L | KILLED | `b1_sa_g_b_reads_each_case_and_the_running_total` (retained_memory_law_tests.rs:1609:5: assert_eq: CaseLoads ≤ l, CaseLoadsTotal ≤ L); `b1_sa_gate_bounds_at_c_are_the_stated_expressions` (retained_memory_law_tests.rs:1788:5: assert_eq: G-B: n, m, m, g, l, L, min(6n, Σr), s, 4 + 4, T11 − T11_late_capture); `every_phase_fact_admits_its_cap_and_refuses_cap_plus_one` (retained_memory_law_tests.rs:904:5: assert_eq: D1's counts: n, m, m, g, l, L (B1 SA), k = min(6n, r), s, 4 + 4) |
| D04 | RV112 | G-B: CaseLoads and CaseLoadsTotal observations swapped | KILLED | `b1_sa_g_b_reads_each_case_and_the_running_total` (retained_memory_law_tests.rs:1631:9: assert_eq); `late_facts_read_the_actual_owners` (retained_memory_law_tests.rs:1073:5: assert_eq) |
| E01 | PLAN | G-C: EnvelopeResults ≤ P_final, not C·P_final | KILLED | `b1_sa_envelope_results_bound_is_c_times_p_final` (retained_memory_law_tests.rs:1664:5: assert_eq); `b1_sa_gate_bounds_at_c_are_the_stated_expressions` (retained_memory_law_tests.rs:1813:5: assert_eq: G-C at C = 3); `every_phase_fact_admits_its_cap_and_refuses_cap_plus_one` (retained_memory_law_tests.rs:912:5: assert_eq: C·P_final, PushCap(C·P_final)) |
| E02 | RV112 | G-C: EnvelopeResultCapacity ≤ PushCap(P_final) | KILLED | `b1_sa_envelope_results_bound_is_c_times_p_final` (retained_memory_law_tests.rs:1664:5: assert_eq); `b1_sa_gate_bounds_at_c_are_the_stated_expressions` (retained_memory_law_tests.rs:1813:5: assert_eq: G-C at C = 3); `every_phase_fact_admits_its_cap_and_refuses_cap_plus_one` (retained_memory_law_tests.rs:912:5: assert_eq: C·P_final, PushCap(C·P_final)) |
| E03 | RV112 | G-C: EnvelopeResultTextBytes ≤ 2·P_final·Text(row) | KILLED | `b1_sa_envelope_results_bound_is_c_times_p_final` (retained_memory_law_tests.rs:1664:5: assert_eq); `b1_sa_gate_bounds_at_c_are_the_stated_expressions` (retained_memory_law_tests.rs:1813:5: assert_eq: G-C at C = 3); `every_phase_fact_admits_its_cap_and_refuses_cap_plus_one` (retained_memory_law_tests.rs:909:5: assert_eq: 2·C·P_final·Text(row)) |
| E04 | RV112 | G-C: ContractEvidenceArrayElements without ×C | KILLED | `b1_sa_gate_bounds_at_c_are_the_stated_expressions` (retained_memory_law_tests.rs:1813:5: assert_eq: G-C at C = 3) |
| E05 | RV112 | G-C: ContractEvidenceObjects without ×C | KILLED | `b1_sa_gate_bounds_at_c_are_the_stated_expressions` (retained_memory_law_tests.rs:1813:5: assert_eq: G-C at C = 3) |
| E06 | RV112 | G-C: ContractEvidenceEntries without ×C | KILLED | `b1_sa_gate_bounds_at_c_are_the_stated_expressions` (retained_memory_law_tests.rs:1813:5: assert_eq: G-C at C = 3) |
| E07 | RV112 | G-C: ContractEvidenceStringBytes without ×C | KILLED | `b1_sa_gate_bounds_at_c_are_the_stated_expressions` (retained_memory_law_tests.rs:1813:5: assert_eq: G-C at C = 3) |
| E08 | RV112 | G-C: ContractEvidenceKeyBytes without ×C | KILLED | `b1_sa_gate_bounds_at_c_are_the_stated_expressions` (retained_memory_law_tests.rs:1813:5: assert_eq: G-C at C = 3) |
| E09 | RV112 | G-C: RetainedErrorTextBytes ≤ (3m + 1)·Text(err), without ×C | KILLED | `b1_sa_gate_bounds_at_c_are_the_stated_expressions` (retained_memory_law_tests.rs:1813:5: assert_eq: G-C at C = 3); `every_phase_fact_admits_its_cap_and_refuses_cap_plus_one` (retained_memory_law_tests.rs:917:5: assert_eq: C·(3m + 1)·Text(err)) |
| F01 | PLAN | T-3 (e): seeds only (I1's predicate: non-empty and every initial) | KILLED | `b1_sa_t3_e_counts_the_requested_cases` (retained_memory_law_tests.rs:1716:9: 1: more seeds than requested cases) |
| F02 | PLAN | T-3 (e): requested ignored at G-C (the seed count passed as requested) | KILLED | `b1_sa_t3_e_counts_the_requested_cases` (retained_memory_law_tests.rs:1740:13: assert_eq: case 0's attempt fails and blocks SparseInteractive) |
| F03 | PLAN | T-3 (e): ≥ instead of == | KILLED | `b1_sa_t3_e_counts_the_requested_cases` (retained_memory_law_tests.rs:1716:9: 1: more seeds than requested cases) |
| F04 | RV112 | T-3 (e): ≤ instead of == | KILLED | `u3g2_direct_entry_no_w1_refusals_keep_exact_bytes` (retained_facade_tests.rs:699:26: SparseInteractive: Some(Err(Preparation))); `b1_sa_t3_e_counts_the_requested_cases` (retained_memory_law_tests.rs:1712:5: no seed); `g_c_declines_w1_when_the_ordinary_solve_was_not_attempted` (retained_memory_law_tests.rs:1468:17: assert_eq: document kind SparseInteractive); `registered_g_c_declines_only_unattempted_solves` (retained_memory_law_tests.rs:1516:26: document kind SparseInteractive: expected G-C's decline, got Some(Err(Preparation))) |
| F05 | RV112 | T-3 (e): requested ≥ 1 dropped | KILLED | `b1_sa_t3_e_counts_the_requested_cases` (retained_memory_law_tests.rs:1711:5: no requested case) |
| F06 | RV112 | T-3 (e): any seed's initial, not every | KILLED | `b1_sa_t3_e_counts_the_requested_cases` (retained_memory_law_tests.rs:1719:9: 2: a seed without an attempt outcome) |
| F07 | RV112 | T-3 (e): compared with C, not the requested count | KILLED | `b1_t4_two_body_case_b_is_a_no_triggered_case_pin` (retained_facade_tests.rs:1272:13: assert_eq: two-body B SparseInteractive); `u3g2_d_u6_5_carrier_fixtures_are_the_live_successors` (retained_facade_tests.rs:823:53: sparse_interactive: the registered Direct entry did not publish its successor); `u3g2_direct_entry_publishes_the_pinned_successor` (retained_facade_tests.rs:593:22: sparse_interactive: Some(Err(CompleteGate(PhaseRefusal { gate: Complete, fact: OrdinarySol); `u3g2_direct_entry_w1_fallbacks_append_one_notice` (retained_facade_tests.rs:644:13: assert_eq: preparation SparseInteractive); `u8_d_u6_5_l0_fixtures_are_the_live_successors` (retained_facade_tests.rs:1088:53: sparse_interactive: the registered Direct entry did not publish its successor); `u8_l0_isolated_node_publishes_pinned_successor` (retained_facade_tests.rs:1056:22: sparse_interactive: Some(Err(CompleteGate(PhaseRefusal { gate: Complete, fact: OrdinarySol); `u8_real_input_fallbacks_append_one_notice` (retained_facade_tests.rs:945:13: assert_eq: first_load_only SparseInteractive: the real input's cause); `b1_sa_envelope_results_bound_is_c_times_p_final` (retained_memory_law_tests.rs:1670:9: assert_eq: 6345 rows); `b1_sa_t3_e_counts_the_requested_cases` (retained_memory_law_tests.rs:1714:9: 1: one attempted seed per case); `complete_facts_read_the_actual_owners` (retained_memory_law_tests.rs:1008:13: OrdinarySolveNotAttempted: 1 > 0); `g_c_declines_w1_when_the_ordinary_solve_was_not_attempted` (retained_memory_law_tests.rs:1468:17: assert_eq: milestone SparseInteractive); `registered_g_c_declines_only_unattempted_solves` (retained_memory_law_tests.rs:1522:9: SparseInteractive: the milestone publishes); `b1_t4_w2b_and_w6_phys_r4_inputs_are_no_triggered_case_pins` (retained_facade_tests.rs:1272:13: assert_eq: W2b's input SparseInteractive) |
| G01 | RV112 | RetainedErrorTextBytes: the capture's own fields only (the unpatched reader) | KILLED | `b1_sa_retained_error_text_reads_every_case_slot` (retained_memory_law_tests.rs:1844:9: assert_eq: SparseInteractive: both slots) |
| G02 | RV112 | RetainedErrorTextBytes: the parked slots only | KILLED | `b1_sa_retained_error_text_reads_every_case_slot` (retained_memory_law_tests.rs:1844:9: assert_eq: SparseInteractive: both slots); `complete_facts_read_the_actual_owners` (retained_memory_law_tests.rs:998:9: assert_eq) |
| G03 | RV112 | RetainedErrorTextBytes: parked slots' error only (observable_error dropped) | KILLED | `b1_sa_retained_error_text_reads_every_case_slot` (retained_memory_law_tests.rs:1844:9: assert_eq: SparseInteractive: both slots) |
| G04 | RV112 | RetainedErrorTextBytes: the first parked slot only | SURVIVED | — |
| H01 | RV112 | B-6: NOTICE_RESERVE_BYTES without ×C (const; its own build) | KILLED | `b1_sa_gate_bounds_at_c_are_the_stated_expressions` (retained_memory_law_tests.rs:1822:5: assert_eq); `structural_budgets_are_u3s` (retained_memory_law_tests.rs:1033:5: assert_eq: B-6 = NOTICE_RESERVE_BYTES × C) |

**G04** survives the candidate's suite: N-2. My probe `zz_rv112_retained_error_text_reads_every_slot_at_c`, run with `RV112_MUT=G04`, fails at its assertion "the second (last) parked slot's observable_error" (0 ≠ 101). The candidate's own test passes under G04 (`E/probe/probe_G04.filtered.log`).

**Not run:** an unchecked (wrapping) census sum. No input can reach it (§3).

## 9. Item 8: suites against I1, test by test (`E/suites/`, `E/tools/suite_diff.py`)

| Suite | I1 `262bd687f0` | SA `6b62606778` | Head `9812c83ded` | Differences |
|---|---|---|---|---|
| PP registered, all targets | 712 ok, 1 failed (`t13`), 11 ignored | 719 / 1 / 11 | 725 / 1 / 11 | **SA:** +7, all ok: `b1_sa_d1_4_admits_one_to_c_load_cases`, `…_runner_oracle_literal_is_load_cases_plus_one`, `…_total_loads_row_admits_l_and_refuses_l_plus_one`, `…_g_b_reads_each_case_and_the_running_total`, `…_envelope_results_bound_is_c_times_p_final`, `…_t3_e_counts_the_requested_cases`, `…_gate_bounds_at_c_are_the_stated_expressions`. **Head:** +13, all ok: those 7, `b1_sa_retained_error_text_reads_every_case_slot`, and SP's five `b1_sp_*` (R3′'s list). **No other outcome changes**; every re-based test passes on every side |
| PP Stale (`--lib`) | 549 / 1 / 11 | 556 / 1 / 11 | 562 / 1 / 11 | Stale = registered on each side, outcome for outcome. I1 → SA: the same +7 |
| Witnesses (`--lib witness_ -- --ignored`) | 10 passed | 10 passed | 10 passed | the 28 `I65_G5_WITNESS*` lines are identical (I1, SA, head) |
| Runner (`P/core/runner/headless`) | 85 ok, 2 failed (the known `load_reference` pair) | the same | the same | none. `explicit_headless_refusal_…` passes on all three |

**The follow-up as b1..b1-a.** PP registered, all targets, goes from b1 `56c5579f07` at 717 ok, 1 failed (`t13`), 11 ignored (R3′'s 712 → 717) to the head at 725 / 1 / 11.
- The difference is **exactly SA's eight tests,** the parked-slot test included, all ok.
- No other outcome changes, so SP's tests, the multi-case ones included, pass unchanged with SA's `LOAD_CASES` = 3 (`E/suites/`).
- b1's own witness lines and in-build record were not run: I stopped that optional last step while it waited for the lock. The head's equal I1's (§7).

## 10. For ROOT

1. **SF-1: rule the corrected G-B byte bound** (T11 minus one case's late capture) **and its placement.**
   - My recommendation: rule it now; implement it with SQ's regeneration (a per-case late form, or `/ C` with G5's exactness check); add the expression test's change to SQ's brief and re-pin list.
   - It does not block I2: at c = 1 nothing changes, and the measured cap-maximal three-case fact is far below either bound.
   - No interim edit is needed. In today's c = 1 profile, `F_T11_LATE_CAPTURE` *is* one case's late form, so the current expression already equals the corrected bound at those forms. The defect appears only when SQ regenerates the late form at C = 3.
2. **N-1 and N-2** are cheap test strengthenings for I-A: the runner literal's source-text tie, and a three-case parked-slot test. They could ride with SF-1.
3. **N-3 and N-4 go to I-P,** with N-4 for RV-P. N-5 goes to SQ's brief. N-7 is ruling 2's pending edit.
4. **The phase-4 check** of RetainedErrorTextBytes ≤ C·(3m + 1)·Text(err) against SP's producer stays open (PLAN_v2 §2.3). The patch makes the fact read every slot; the bound's assumption is unchanged.

## 11. Records

**This folder:** `REVIEW.md`, `evidence/` and `SHA256SUMS`, with placeholder paths only (`E/tools/sanitize.py` refuses any home, system-temp or home-relative path).

**`evidence/`:**
- `basis/`: input hashes and the candidate's commits;
- `oracle/`: `gate_bounds_s3.py` and its JSON;
- `identity/`: the profile bytes and each build's printed identity;
- `record/`: the in-build records;
- `suites/`: filtered logs, return codes and step times;
- `mutants/`: filtered logs, return codes, the schemata diff and the H01/restore hashes;
- `probe/`: the probe runs;
- `derived/`: the mutant summary JSON and table;
- `tools/`: every script I ran, and the probe source;
- `cargo_jobs_rv112.log`.

**Kept for SQ:** `S` (scratch, with the oracle inputs and tools). The copies `WT/rv112/` and the targets `WT/targets/rv112-*` are deleted.

**Budget:** about 3 h of agent time. Most of the wall time was the shared lock's queue.
