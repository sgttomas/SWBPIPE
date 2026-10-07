# I81 B1-0: the probe before B1's widening (records only; nothing maintained)

TASK (Type 2), I81, for ROOT (HELP_HUMAN, Agent 0), who is the return path. I made no delegation. I am a fresh instance. 2026-10-07 UTC (2026-10-06 host local time).

**Brief (verified before work):** `R/BRIEFS/B1_0_PROBE.md`, sha256 `8502f267d339c2e2d7d9b66d2a2646cc3a626a2b41caeefd9e5eb086ec88d646`. I read NUM's root `AGENTS.md` and `agents/AGENT_TASK.md` first.

**Basis read:**
- `R/I78/b0_contract_01/DESIGN_v2.md` (sha256 `5933b90b…1114`, as RV105's ADDENDUM_01 records): §0, §1.2 (T-3, T-4), §1.3, §1.4, §8 (decisions 1, 2 and 21), §9 and §10;
- `R/REVIEW_RV105/b0_01/`: REVIEW §1–§2 (with the findings table), ADDENDUM_01, and `evidence/probe_quality.log`, `probe_quality.py`, `input_shas.log`;
- U8's probe, `R/I68/u8_probe_01/PROBE.md`, its `_run_records/zz_i68_probe.rs`, `instrumentation.diff` and the `I68_*` lines of `probe_run2.log`. I followed its method;
- `R/BRIEFS/I68_U8_PROBE_AND_WITNESSES.md`, Part 1;
- QUAL §4 and §8a (`T/IMPLEMENTATION/F2A_D1/copies/QUALIFICATION.md`, sha256 `8edbf4b4…2c29`, equal to DESIGN_v2 §10's).

**Placeholders:** `WT`, `NUM`, `P`, `PP` (= `P/core/product_physics/src`), `T`, `R`, `RR` and `VENV` as in the dispatch. `ARCH` = the disposable archive under `WT/scratch/i81_b1_probe/`. DN = `T/DESIGN_NUMERICS/DESIGN.md`.

**Limits kept.** Records only; no maintained file changed. No Git writes (reads used `GIT_OPTIONAL_LOCKS=0`). No DEC-025, no evidence sweep, no installs. Every cargo job went through `WT/tools/t3_cargo.sh` (memory guard up), `--locked --offline`, `CARGO_BUILD_JOBS=4`, `RUST_TEST_THREADS=2`, with `RUSTFLAGS` and `CARGO_ENCODED_RUSTFLAGS` unset. The only inputs run were QUAL §4's committed witness inputs, `attempted_examples()`, U8's committed inputs (named in DESIGN_v2 §1.3 and the brief's item 3), and case C. `TMPDIR` pointed into my scratch; nothing was written to the system temp directory. ROOT's DEC-025 for #1104 held the lock when I started; my first job waited for it (`_run_records/cargo_jobs_i81.log`).

## 0. Outcomes in brief

| Item | Outcome (both modes unless stated) |
|---|---|
| 1. Verdict audit (S-1) | **Two QUAL §4 witnesses change under T-4: W6 and W2b.** W6 is `checks_passed`, W2-published (`initial: structural_failure/range`, `w2: published, b = 536`). **W2b is `checks_passed` with an ordinary Passed report** (`initial: report/checks_passed`, `w2: not_triggered`), so it is `not_required`; its committed assertion `Fallback("Candidate")` would fail. W-C1 (two-body B) changes too, as DESIGN_v2 predicted. **W2 is not attempted** (`not_assessed`, no seed): G-C declines it on the Direct entry today, and its witness reaches W1 only because the private driver skips G-C. Every other witness input is Sensitive (A) or exact-selected (T-3 (c)). **Every `attempted_examples` input stays in A**; none is `checks_passed` and none is excluded. |
| 2. Decision 21's tags | **No audited attempted failure carries `mechanism`, `asymmetric` or `invalid_input`.** `tiny_spring` (the 1e-300 spring) is `structural_failure` / **`numerically_unresolved`**, with `w2: not_triggered`, so it stays in A. Two-body A and B, case C and W6 are `range` with W2 published. K2a's is a `formation_failure`, not a structural one. Decision 21 moves no audited input. |
| 3. W-C2's case C | **Verdict `sensitive`** (seed: `initial: structural_failure/range`; `w2: published, b = 518`; K-D5's D-5 line present). **W1 runs. The native run ends `Unresolved`, `UnresolvedReason::Ceiling`**, with the same ladder as W6 on body 1 (p128 StopRule, p256 StopRule, p512 Charge, p1024 verification solved). **Fallback `Native`; exactly 1 N1 notice;** the bytes equal `with_notice(plain, "case", None)`; admitted; `ONE_RUN_THROUGH_G_C`; no hooks armed. **The stop rule did not fire**, so fallbacks (i) and (ii) were not tried. |
| 4. Re-basing | **Case C alone gives both outcomes.** On the Direct entry it gives W-C1's (Native at Ceiling, one notice). On the W6 witness's own private driver at R/k = 4 MiB, it gives `force_scaled = true` and `Fallback(Native)` after the full native ladder. So W-C1 and the W6 stack witness can both be re-based on case C. **W2b has no probed replacement** (outside this probe's input limits); see §5 and §8. |
| 5. Multi-case | **Untested.** The producer admits one case (D1.4), so every run here is a one-case request. |

**Controls.** Fourteen input-and-mode pairs shared with I68's U8-0 probe give identical plain bytes, published bytes, W1 cause and notice count (`_run_records/compare_i68.log`). The dense two-body A precommit dump is byte-identical to I68's F-1 dump (sha256 `540ee2b8…85f5`). Two probe runs gave identical `I81_*` lines (§7).

## 1. Setup and build identity

- **Archive:** `GIT_OPTIONAL_LOCKS=0 git -C NUM archive d8c88774d0a73bc99fe9c6e906db6296162f8953 | tar -x -C ARCH/`. Before editing, the three production files I instrumented were checked equal to main's blobs: PP `lib.rs` (`a1990c9f58`, sha256 `4c33c250…`), `retained_product.rs` (`b73225304e`, `f536bfe7…`) and `retained_memory.rs` (`843b7ec59f`, `fbc7c9db…`). The first two are byte-equal to I68's `b1e2d7741e` copies.
- **Target:** `WT/targets/i81-b1-probe/`.
- **Registered build, observed:** the probe's `registered()` (`option_env!("OPS_RETAINED_BUILD_IDENTITY")` equal to the facade tests' `REGISTERED_IDENTITY`, rustc 1.97.1 `8bab26f4f68e`, aarch64-apple-darwin, debug) is `true`, and every admission report reads `profile=Registered`. The instrumentation touches none of the identity's inputs (build.rs hashes `Cargo.lock` and the reader's `include_str!` statics).
- **Probe-only instrumentation** (`_run_records/instrumentation.diff`; archive only; every addition is `cfg(test)`; no control-flow change):
  1. lib.rs `permitted_run`: `I81_SEEDS site=permitted_run`, right after the observed ordinary run. It prints the published verdicts (`numerical_quality.cases[].solve_quality`), the mechanics status, whether exact-block selected, and every `OrdinarySeed`: `initial` (kind, outcome or structural tag), `w2` (kind, b, trigger), legacy branch, `recovery_demoted`, load-row finding and D-5 line.
  2. lib.rs `retained_w1`: the same `I81_SEEDS site=retained_w1` at entry (this reaches the committed witnesses, which call `retained_w1` directly); `I81_W1_START` after the notice reservation, where W1 work starts; and I68's `I81_PREPARATION_FAILURE`, `I81_CANDIDATE_REFUSAL` and `I81_PRECOMMIT_ERROR`/`_DUMP`.
  3. retained_product.rs `solve_native`: I68's `I81_NATIVE_OUTCOME`, the kernel outcome with its `UnresolvedReason` and each attempt's (p, role, outcome).
  4. retained_memory.rs: `mod zz_i81_probe` (test only), declared beside `law_tests` and `witness_tests` so it uses their committed inputs (`cap_maximal`, `attempted_examples`, `w6_input`) directly.
- **The probe module** is `_run_records/zz_i81_probe.rs`. Its `probe()` is I68's. Every run goes through the actual Direct entry `run_linear_static_preview_value_with_retained_direct`, counted by grant 2's tally. Each run records: the plain route (status, result count, sha, verdicts, diagnostics); the admission report; the W1 cause; the counts against `ONE_RUN_THROUGH_G_C`; `hooks::armed_names()` before and after; the notice count; byte equality with `with_notice(plain, case, None)` and with plain; and, for a successor, the Rust reader's verdict. Inputs: QUAL §4's witness inputs, built as the witnesses build them (W2's and W2-deep's escaping and depth-16 helpers are copied verbatim); `attempted_examples()` as committed; U8's helpers, copied, whose input sha256s equal I68's and RV105's (`ec6c8e65…` two-body A, `cf688351…` two-body B, `8d1967a7…` the milestone); and **case C** (§4).
- **One parse slip, corrected:** in run 1 the `range_scaling` field of the `I81_ORDINARY` line was cut at the first space. Run 2 fixes the parse (`_run_records/zz_i81_probe.run1_to_final.diff`). The seed's `force_scale_exponent` was always printed correctly.

## 2. Item 1: the verdict audit (S-1)

**The rule applied** (DESIGN_v2 T-4, decision 1, and decision 21):
- `not_required` exactly when the published verdict is `checks_passed`;
- **excluded** when `initial` is `structural_failure` with tag `mechanism`, `asymmetric` or `invalid_input`, and `w2` is not `published`;
- otherwise **A** (including an absent entry or any other verdict).

T-3's invocation gates come first: (c) coexistence and (e) G-C.

Every row has identical facts in both modes, except where a mode is named. "W1 runs" means `I81_W1_START` printed (the notice was reserved and preparation began). The full per-mode table is `_run_records/summary.md`.

### 2.1 QUAL §4's witness inputs

| Witness (input) | Ordinary | Published verdict | Seed `initial` | Seed `w2` | W1 runs today | W1 outcome today | T-4 class | Witness changes? |
|---|---|---|---|---|---|---|---|---|
| **W1, W4, W5, W7, headroom** (the milestone) | solved | `sensitive` | report / sensitive | not_triggered | yes | Direct: successor. Witnesses: W1, W5 and headroom (1 MiB) Successor; W4 Preparation (its observer hook: `annulus preparation refused`, `InvalidGeometry`); W7 Native, Serializer, Staging, Precommit G8, Precommit G1 (its hooks) | A | No |
| **W2** (cap-maximal, escaped, depth 16) | **blocked before the solve:** `MODEL_INCOMPLETE`, 0 results, 32 × `SUPPORT_STIFFNESS_FAMILY_UNSUPPORTED` (blocking) | **`not_assessed`** | **no seed** (`ordinary` is empty: not attempted) | – | **Direct: no.** G-C declines: `CompleteGate(OrdinarySolveNotAttempted)`, exact bytes, 0 notices. **Witness: yes,** because its private driver skips G-C | Direct: plain bytes. Witness: `Fallback("Preparation")`, `Association("prepared case custody/permit")` | Direct: T-3 (e) declines before T-4. Witness path: **A** by T-4's literal rule (`not_assessed` is "anything else"), with no seed for decision 21 to read | **No, if B1's T-4 tolerates a case with no seed on the driver path** (§8 item 2) |
| **W2-deep** (milestone, escaped, depth 16) | solved | `sensitive` | report / sensitive | not_triggered | yes | Successor (Direct; witness at 4 MiB and 1 MiB) | A | No |
| **W2b** (cap-maximal, solvable) | solved; 2,113 / 2,114 results | **`checks_passed`** | **report / checks_passed** | **not_triggered** (legacy `not_required`) | yes | `Fallback("Candidate")`, Direct and witness: native **Selected**, then the proof refuses `Predicate { row: 7 (sparse) / 8 (dense), SharperExact }`; 1 notice | **`not_required`** | **Yes.** `NoTriggeredCase`: exact bytes, no notice, no native run. `assert_eq!(ran, Fallback("Candidate"))` fails, and S1 loses its only full native run at the cap-maximal counts |
| **W3** (n05, one fixture per mode; the two fixtures are byte-identical requests) | solved; exact-block **selected** | `sensitive` | report / sensitive | not_triggered | no | `Coexistence` (Direct: exact bytes, 0 notices; G-C not consulted, counts `{runs: 1, complete_gates: 0}`). Witness: ExactSelected | T-3 (c), before T-4 | No |
| **W6** (PHYS-R4 cantilever) | solved; `range_scaling: force_scale_exponent=536`; `HIGH_DISPLACEMENT_REVIEW` | **`checks_passed`** | **structural_failure / range** | **published, b = 536** | yes | `Fallback("Native")`: `Unresolved`, **Ceiling**; 1 notice | **`not_required`** | **Yes.** `NoTriggeredCase`. The test only prints its outcome, so it keeps passing, but it stops witnessing W1 |

### 2.2 `attempted_examples()` (each run in both modes, as `registered_g_c_declines_only_unattempted_solves` runs them)

| Input | Ordinary | Published verdict | Seed `initial` | Seed `w2` | W1 runs | W1 outcome today | T-4 class |
|---|---|---|---|---|---|---|---|
| The milestone | solved | `sensitive` | report / sensitive | not_triggered | yes | Successor | A |
| "failed attempt" (support 1 at 1e-300; equal to U8's `tiny_spring`, input `1ab95acc…`) | `MODEL_INCOMPLETE`, 0 results | **`unresolved`** | **structural_failure / `numerically_unresolved`** ("positive diagonal contribution absorbed by assembly; stabilization unresolved", `global_dof: Some(3)`; its diagnostic is `diagnostic:numerical-integrity:case`) | not_triggered | yes | `Preparation`: `Association("prepared case custody/permit")`; 1 notice | A (a DN §4.3 trigger; not excluded) |
| K2a's deferred formation | solved; `force_scale_exponent=898` | **`sensitive`** | formation_failure (`NumericalRange { name: "12EIy/L^3: (12*E)*Iy" }`) | published, b = 898 (trigger: formation) | yes | `Candidate`: native Selected, then the proof refuses `Predicate { row: 81, SharperExact }`; 1 notice | A |
| `rejected_stress_range` sparse fixture | `MODEL_INCOMPLETE`, 0 results (legacy exact-block selected, then blocked) | `sensitive` | report / sensitive | not_triggered | yes | `Preparation`: `Association("missing produced mode")`; 1 notice | A |
| `rejected_stress_range` dense fixture (byte-identical request to the sparse one) | as above | `sensitive` | report / sensitive | not_triggered | yes | as above | A |

**So `registered_g_c_declines_only_unattempted_solves` is unchanged.** It still passes, and its doc line ("a solve that ran but failed still reaches W1") still holds: no attempted example is `checks_passed` or excluded. DESIGN_v2 §1.3 left K2a's verdict unrecorded; it is `sensitive`.

**Also recorded:** K2a's seed has `recovery_demoted = true` (R-b′). It falls back at Candidate, before the serializer, so DESIGN_v2's known B1 limit (N-5) is not reached on it today.

### 2.3 U8's committed inputs (context; DESIGN_v2 §1.3)

| Input | Published verdict | Seed | W1 outcome today | T-4 class | Changes? |
|---|---|---|---|---|---|
| `first_load_only` | `sensitive` (D-5 line) | report / sensitive; not_triggered | Candidate (`Predicate { row 80 sparse / 81 dense, SharperExact }`); 1 notice | A | No |
| L = 0 | `sensitive` | report / sensitive; not_triggered | Successor; the Rust reader PASSes | A | No |
| Two-body A | `sensitive` (D-5 line) | structural_failure / range; published, b = 518 | Sparse: successor (PASS). Dense: `Precommit { G8, PREPARATION_MISMATCH }` (F-1, unchanged) | A | No |
| **Two-body B (W-C1)** | **`checks_passed`** | structural_failure / range; **published, b = 518** | Native, Ceiling; 1 notice | **`not_required`** | **Yes** (`u8_real_input_fallbacks_append_one_notice`, variant `w_c1_two_body_case_b`, asserts `Native` and one notice) |

### 2.4 Which witnesses and tests change under T-4 and decision 21

| Witness or test | Change |
|---|---|
| `witness_w6_force_scaled` (QUAL §4 W6) | **Changes:** `NoTriggeredCase` instead of `Fallback("Native")`. There is no assertion, so it passes vacuously. Re-base (§5) |
| `witness_w2b_cap_maximal_solvable` (QUAL §4 W2b) | **Changes and fails:** it asserts `Fallback("Candidate")`. Re-base (§5, §8 item 1) |
| `u8_real_input_fallbacks_append_one_notice`, variant `w_c1_two_body_case_b` | **Changes and fails:** it asserts `Native` plus one notice. Re-base on case C (§5) |
| `witness_w2_cap_maximal` (QUAL §4 W2) | **Unchanged only if** B1's T-4 puts a seedless case in A on the driver path (§8 item 2) |
| W1, W2-deep, W3, W4, W5, W7, headroom; `registered_g_c_declines_only_unattempted_solves` (and `g_c_declines_…`); `first_load_only`; L = 0; two-body A | Unchanged |
| Decision 21 | Moves none of the audited inputs |

## 3. Item 2: decision 21's tags

Every attempted failure among the audited inputs, with its seed (both modes identical):

| Input | `initial` | Tag | `w2` | Decision 21 |
|---|---|---|---|---|
| `tiny_spring` / "failed attempt" | structural_failure | **`numerically_unresolved`** | not_triggered | Not excluded: a DN §4.3 trigger; stays in A |
| Two-body A | structural_failure | `range` | published, b = 518 | Not excluded (W2 published) |
| Two-body B | structural_failure | `range` | published, b = 518 | Not excluded; `not_required` by its verdict |
| Case C | structural_failure | `range` | published, b = 518 | Not excluded |
| W6 | structural_failure | `range` | published, b = 536 | Not excluded; `not_required` by its verdict |
| K2a's deferred formation | formation_failure (`NumericalRange`) | – (not a `StructuralError`) | published, b = 898 | Outside decision 21's condition |

No audited input reaches a `mechanism`, `asymmetric` or `invalid_input` seed. QUAL §8a's `not_attempted_examples` (for example the lone spring, refused as a mechanism before the attempt) have **no seed**, and G-C declines them; decision 21 does not apply to them. **So A1-N3's "second c = 1 change" moves no audited committed test.** I did not audit committed tests outside the brief's inputs.

## 4. Item 3: W-C2's case C (DESIGN_v2 §1.4)

**The input.** U8's `u8_two_body_case_a()` with case B's two loads (`load:tip-y`, `load:tip-torque` on `node:section-b`, magnitude `f64::from_bits(0x0031fa182c40c60d)`) appended to the one load case `case`, after the milestone's three moments. So it is A's loads plus B's loads, as one case. Input sha256 `3649b4dcb96ecb9e8a32ee9ae95bddea65648d3d6aff10860785cb95f73306b3`. Counts as two-body A: 4 nodes, 2 members, 5 supports, 2 materials; 5 loads.

| | sparse_interactive | dense_scrutiny |
|---|---|---|
| Ordinary | `MECHANICS_SOLVED`; 171 results; `range_scaling: force_scale_exponent=518`; diagnostics: `SOURCE_BLOCK_RECOVERY_UNAVAILABLE` (info), `NUMERICAL_INTEGRITY_SENSITIVE`, `HIGH_DISPLACEMENT_REVIEW`; plain `a200ae94fc6a…` (116,424 B) | the same; plain `eafe81082972…` (116,670 B) |
| **Published verdict** | **`sensitive`** | **`sensitive`** |
| **Seed** | `initial: structural_failure/range` ("arithmetic outside normal range"); `w2: published, b = 518`, trigger `Evaluation(Range)`; legacy `unavailable` (source closure); D-5 line present (`d5_diagnostic_ref` set, so K-D5's formation check demoted the report); `recovery_demoted: false`; no load-row finding | identical |
| Admission | Registered; refusal `None`; domain `None` (inside D1); required 3,575,778,286 B; census complete | the same; required 3,595,488,734 B |
| **W1 runs?** | **Yes** (`I81_W1_START case=case`) | **Yes** |
| **Native terminal** | **`Unresolved`, reason `Ceiling`.** Attempts: (128, Candidate, `Rejected(StopRule { Displacement(node 3, Uy), body 1, Translation })`); (256, VerificationThenCandidate, `Rejected(StopRule { EndAction(member 1, I, Ux), body 1, Force })`); (512, VerificationThenCandidate, `Rejected(Charge { EndAction(member 1, I, Ux), body 1, Force })`); (1024, Verification, Solved) | identical |
| **Fallback and notice** | **`Native`**; `ONE_RUN_THROUGH_G_C`; hooks empty before and after; **1** `RETAINED_PRECISION_UNAVAILABLE` notice; published bytes **equal** `with_notice(plain, "case", None)` and differ from plain; published `e28f03dc10dc…` (116,743 B) | **`Native`**; the same; published `8a399b239e04…` (116,989 B) |

**The stop rule did not fire.** C's verdict is not `checks_passed`, and its native run ends at Ceiling. Fallbacks (i) and (ii) were not constructed or run.

**Against DESIGN_v2 §1.4's row for C:** the verdict is not `checks_passed` (Sensitive, by K-D5's line, as expected from body 0), and the native run ends at Ceiling, from body 1's rows. In a three-case W-C2, C would be `unavailable`, with its Run `kernel_terminal` `{kind: unresolved, reason: {space: unresolved, tag: ceiling}}`. The kernel outcome is recorded here. **The wire form is not observed,** because a one-case Native fallback publishes no receipt.

## 5. Item 4: re-basing

| Changed witness | Proposed input for B1 | Established here? |
|---|---|---|
| **W-C1** (`u8_real_input_fallbacks_append_one_notice`, variant `w_c1_two_body_case_b`) | **Case C alone,** as a one-case Direct input in both modes. Expected values, as observed: cause `Native`; 1 notice; bytes = `with_notice(plain, "case", None)`; `ONE_RUN_THROUGH_G_C`; admitted; no hooks. Its kernel reason, `Unresolved(Ceiling)`, is recorded here and not asserted, as U8 did for B | **Yes,** both modes (§4) |
| **W6 stack witness** (`witness_w6_force_scaled`) | **Case C alone,** on the witness's private driver at R/k = 4 MiB. Observed with a verbatim copy of `permitted_work` and the witness's `on_reserved_stack(…, carry_test_hooks(…))` (`zz_i81_item4_case_c_on_the_w6_witness_path`): `force_scaled = true` (the plain route publishes `force_scale_exponent=518`); verdict `sensitive`; W1 runs; the full native ladder to p1024, ending `Unresolved(Ceiling)`; **`Fallback(Native)` in both modes.** So the force-scaled path and the native stack path that W6 witnessed are kept | **Yes,** with instrumentation present (`_run_records/probe_item4_witness_path.log`). The S1 measurement itself is B1's, on the uninstrumented re-qualification build |
| **W2b** (`witness_w2b_cap_maximal_solvable`) | **No probed replacement.** B1 needs an input at W2b's cap-maximal counts (32 nodes, a 32-member ring, 32 supports, 128 loads, 4 + 4 materials with 16 temperature points, a 128-byte id) whose published verdict is **in A and solved** (Sensitive, so that preparation's `MECHANICS_SOLVED` custody holds) and whose W1 work reaches the native stage. Case C cannot serve: it has 4 nodes. Candidate constructions, **none run here:** (a) W2b with every ring section scaled down, so that the reciprocal condition estimate falls below √eps while the solve still succeeds (cond above 6.7e7; too far gives `NumericallyUnresolved`, a blocked envelope and a Preparation fallback); (b) W2b with loads and springs arranged so that K-D5's formation check fires, as it does for the milestone. Both need a probe with a stop rule | **No** (§8 item 1) |

**Optional pins for T-4 itself.** B1 could keep W6's PHYS-R4 input and two-body B as **`NoTriggeredCase` witnesses**: exact bytes, no notice, and no W1, on the one-body and two-body W2-published Passed shapes. That pins T-4's new behaviour on real inputs. W2b's input could likewise pin `NoTriggeredCase` for an ordinary Passed report. This is a suggestion, not something established here.

**For the three-case W-C2:** case B is confirmed as W2-published `checks_passed` (`not_required`), and case C as the Ceiling `unavailable` row, in both modes. Case A's dense half still needs Text B (F-1; the dense refusal reproduced byte for byte).

## 6. Item 5: not tested

**The multi-case invocation is untested.** The producer admits one load case today (D1.4: `w1_case_id`, `family_clauses`, `LOAD_CASES = 1`). Every result here is from a one-case request. Whether A, B and C keep these outcomes **inside one `CaseBatchCall`** (T-8), with one shared group and stiffness, is B1's to establish after the widening. So is whether the receipt's per-case snapshots and work stay inside the caps (§6 of DESIGN_v2).

## 7. Controls and determinism

- **Against I68's U8-0** (`_run_records/compare_i68.py`, `compare_i68.log`): for the 14 input-and-mode pairs with equal input sha256, the plain sha, published sha, W1 cause and notice count are all identical. The pairs are the milestone, `first_load_only`, `tiny_spring`, W6, L = 0, and two-body A and B, each in both modes. The F-1 precommit dump (dense two-body A) is byte-identical to I68's (`540ee2b8…85f5`). So main `d8c88774d0`'s producer behaves as U8-0's `b1e2d7741e` did on those inputs.
- **The committed witnesses** (`_run_records/witness_run.log`, the nine `witness_*` tests run unchanged with `--ignored`) all pass. Their outcomes equal QUAL §4's table. W6's ladder is the one I68 recorded.
- **Determinism:** probe runs 1 and 2 produced identical `I81_*` lines, 261 each, apart from run 1's cut `range_scaling` field.
- **Basis drift during the probe:** NUM moved from `e302ee8c64` to `90f7e3a2f9` (records, and #1104's T6S merge). Between main `d8c88774d0` and NUM `90f7e3a2f9`, nothing under PP `src` or RE `src` changed. Only RE tests, two derivative-golden fixtures and `schemas/results.schema.yaml` changed. So the producer facts here also hold at NUM's head.

## 8. For ROOT to rule on

1. **W2b becomes `not_required`.** It is an ordinary Passed report with no W2. Under T-4, `witness_w2b_cap_maximal_solvable` fails its assertion, and S1 loses its only full native run at the cap-maximal counts. RV105's S-1 anticipated this case. Options:
   - **(a)** authorize a short follow-up probe (B1-0b) of candidate cap-maximal inputs in A, such as §5's (a) and (b), with a stop rule;
   - **(b)** let B1 construct and establish one inside its re-qualification, under the same stop rule;
   - **(c)** keep W2b's input as a `NoTriggeredCase` witness, and rest the cap-count native-stack evidence on QUAL §4's stated limit ("frames do not scale with counts") plus case C's full native run at small counts. This is weaker, because it is argued rather than measured.
   
   I recommend (a) or (b) before T-4 lands, with (c) as the stop-rule outcome.
2. **W2's witness and T-4's seed assumption.** W2's input is never attempted on the ordinary route: it is validation-blocked, `not_assessed`, and has no seed. On the Direct entry, G-C declines it today. Its witness reaches W1 only because the private driver skips G-C (decision 7). Two points need a ruling:
   - **The class of `not_assessed`.** DESIGN_v2's T-4 lists `sensitive`, `unresolved` and `failed` as examples, and `not_assessed` falls under "anything else", so it is in A. Confirm this reading.
   - **The seed lookup.** If B1's T-4 or decision 21 looks up a seed per case, it must not assume one exists on the driver path. A missing seed must count as "not excluded", or W2's witness changes or panics. Otherwise B1 must re-base W2 on an attempted input.
3. **Decision 21 changes no audited input** (§3). If ROOT adopts it, the owner-information line (A1-N3) can say that it moves no committed witness among those audited.
4. **Re-basing:** adopt case C for W-C1 and for the W6 stack witness (§5). Decide whether W6's input, two-body B and W2b's input are kept as `NoTriggeredCase` pins.
5. **Recorded, not rulings:**
   - The two `rejected_stress_range` fixtures are byte-identical requests, and so are the two n05 fixtures (W3). So each attempted example of that pair adds no new input.
   - `rejected_stress_range`'s Preparation cause is `Association("missing produced mode")`. It is not the custody refusal: the envelope is blocked after the legacy exact block selected and finalization failed.
   - K2a's seed is `recovery_demoted` (R-b′).
6. **Host note.** The host accepted writes into NUM at this records folder. It also accepted the file-edit tool's writes into `WT/scratch`. No write guard refused anything.

## 9. Commands run (exact; `cd` into the stated directory first)

1. `mkdir -p WT/scratch/i81_b1_probe/{arch,tmp,out,logs} && GIT_OPTIONAL_LOCKS=0 git -C NUM archive d8c88774d0a73bc99fe9c6e906db6296162f8953 | tar -x -C WT/scratch/i81_b1_probe/arch/`
2. Instrumentation and the probe module written into `ARCH` (`_run_records/instrumentation.diff`, `zz_i81_probe.rs`).
3. In `ARCH/P/core/product_physics`: `env -u RUSTFLAGS -u CARGO_ENCODED_RUSTFLAGS TMPDIR=WT/scratch/i81_b1_probe/tmp CARGO_BUILD_JOBS=4 RUST_TEST_THREADS=2 CARGO_TARGET_DIR=WT/targets/i81-b1-probe WT/tools/t3_cargo.sh test --locked --offline --lib --no-run` (rc 0)
4. Probe run 1, in the same directory and environment, plus `I81_OUT=WT/scratch/i81_b1_probe/out`: `WT/tools/t3_cargo.sh test --locked --offline --lib zz_i81 -- --nocapture --test-threads=1` (rc 0; 4 passed)
5. Probe run 2, after the parse fix: the same command (rc 0; 4 passed)
6. The committed witnesses: `WT/tools/t3_cargo.sh test --locked --offline --lib witness_ -- --ignored --nocapture --test-threads=1` (rc 0; 9 passed)
7. Item 4, after adding the witness-path test: `WT/tools/t3_cargo.sh test --locked --offline --lib zz_i81_item4 -- --nocapture --test-threads=1` (rc 0; 1 passed)
8. Read-only Python (system `python3`): `sanitize.py` (logs into the records), `summarize.py` (`summary.md`), `compare_i68.py` (`compare_i68.log`).

The lock lines for these jobs are in `_run_records/cargo_jobs_i81.log`.

## 10. Records, cleanup and limits

- **`_run_records/`:**
  - the probe sources: `zz_i81_probe.rs`, `zz_i81_probe.run1_to_final.diff` and `instrumentation.diff`;
  - the logs: `build_pp_lib.log`, `probe_run1.log`, `probe_run2.log`, `witness_run.log`, `probe_item4_witness_path.log` and `cargo_jobs_i81.log`;
  - the tools and their outputs: `sanitize.py`, `summarize.py` with `summary.md`, and `compare_i68.py` with `compare_i68.log`;
  - `probe_outputs.sha256`.
  
  Machine paths are replaced by `WT`, `ARCH` and `~`. In the logs, every line over 4,000 bytes that is not one of this probe's `I81_` lines is cut to 1,000 bytes, with its full length and sha256. Those lines are the producer's committed `cfg(test)` prints `I51_DUAL_LANES` and `I51_FROZEN_*`. The unsanitized logs, the F-1 dump and the pre-edit copies stay in `WT/scratch/i81_b1_probe/` (outside NUM), with their sha256 in `probe_outputs.sha256`.
- **Cleanup on return:** the archive `ARCH` and the target `WT/targets/i81-b1-probe/` are deleted. The archive is reproducible from `d8c88774d0` plus `_run_records`.
- **Limits:**
  - **One-case requests only.** The three-case W-C2 is untested (§6).
  - **Case C's wire form is not observed.** The `kernel_terminal` encoding of Ceiling is the serializer's in a multi-case receipt, which does not exist yet.
  - **W2b's replacement is not probed** (§5, §8).
  - **The witness-path run of case C and the committed witnesses ran with probe prints present.** They establish outcomes, not S1 stack margins.
  - **Only the brief's inputs were audited.** Other committed tests that might reach W1 on a Passed input were not.
