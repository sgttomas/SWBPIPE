# RV89: independent review of U4 G6 and its registration change

**Reviewer:** RV89, TASK (Type 2), dispatched directly by ROOT. No descendants. This continues RV89's part-1 and part-2 reviews (`R/REVIEW_RV89/u4_g5_01/`, `u4_g5_02/`) with the same context.

**Candidate:**
- G6 is committed unregistered as `2bb81ec1ea` on `codex/piping-f2a-memory-20261004`, on top of `cba3e9fda7`.
- `R/I65/u4_g6_01/registration.diff` (sha256 `35c72703…4fd2a`, 287 lines) is **not applied**.
- I applied it with `patch -p1` to my own `git archive` copy of `2bb81ec1ea`, never in WT/f2a-memory. It applies cleanly to the four files it names.

**Basis read:**
- `R/I65/u4_g6_01/` (QUALIFICATION.md, RETURN.md with its Addendum, registration.diff, and `_run_records/`: per_identity, text_g6, id_audit, controls, the mutant harness);
- the rulings "U4 G6 returned; registration rulings; blocked ordinary runs decline W1 at G-C" and "U4 G6 committed unregistered; RV89 and RV87 dispatched";
- my part-2 record;
- STACK_INVENTORY.md and RV83's u4_g2_04 N-3.

**Oracles:** RV89's own.
- My sweep, run unregistered and registered.
- My probes in the registered copy, including **an independent solve-attempt counter that I added to my copy of lib.rs**.
- Two stale builds (another RUSTFLAGS, another opt-level) and the release build.
- My mutants, and my evaluation of the profile.

I65's tests were only run.

## Verdict: **PASS**, with one change to the registration package before it is applied (S-1) and two test additions (S-2, S-3)

| | Count |
|---|---|
| BLOCKING | 0 |
| SHOULD-FIX | 3 |
| NOTE | 4 |

**Is there any reason not to register? None beyond S-1,** which is a test assertion, not product behaviour. S-2 and S-3 are test gaps on behaviour that is correct today; they need not hold registration.
- `registration.diff` as written leaves one runner/headless test red. That test asserts the Headless report's profile is `Missing`, and under registration it reads `Registered`.
- Add that hunk to the package, which is the same kind of change ruling 3 already covers, and the registration is ready. Everything else I checked holds.

**The headline.**
- **The entry binds exactly this build.**
  - The registered identity text and reviewed-input text are byte-identical to what my own builds compile. The 14 input files are unchanged at `2bb81ec1ea`, and the reader layouts are 56/64/96/16.
  - In the registered copy, `build_status()` is `Ok(0)`, `ESTIMATES` is 0, and `threshold_bytes` is 4,026,531,840.
  - **Every other build is `Stale`, refuses at D1.1, and publishes the ordinary bytes:**
    - `RUSTFLAGS="--cfg rv89_stale"` gives `rustflags=--cfg%1Frv89_stale`;
    - `CARGO_PROFILE_DEV_OPT_LEVEL=1` gives `opt_level=1`;
    - the release build gives `profile=release;opt_level=3;debug_assertions=false`.
- **`admit` behaves as specified.**
  - It grants a permit for the milestone in both modes.
  - It refuses Headless (D1.0), the two-case variant, and **21 further D1 violations** of my own, each at its own clause, with the ordinary bytes and no W1 result.
- **The milestone publishes U1's pinned successor through the facade** in both modes. The file sha256 is `ac6986b0…` / `6cd1d249…` and the receipt sha256 is `efc1a39b…` / `3e26499f…`, exactly the committed pins. The ordinary base is unchanged, and `into_publication()` is the successor.
- **Suites, registered:**
  - PP 698/1/11; the one failure is the Mac t13.
  - runner/headless is identical to base **apart from S-1's one test**.
  - The unregistered sweep is byte-identical to base (`e1677d73…`).
  - The registered sweep differs only where registration should change it (§1).
- **G-C's solve-attempt fact is exactly "the ordinary route attempted the case's solve".** I instrumented the one `attempted_linear` statement in my copy and compared it with `ordinary_solve_attempted`: **182 of 182 observed runs agree**, inside and outside D1, including 22 D1 failure variants of my own.
  - Under registration, every one of the 38 in-D1 runs that was not attempted declines with the value route's exact bytes and no notice.
  - Every attempted run proceeds.
- **The in-build maximum reproduces:** 0.887840 / 0.892735 M, 48,961,532 / 29,251,084 B under 0.9 M. The text-error budget is 1.86% dense and 3.12% sparse.
  - Between part 2 and G6, exactly the 42 Estimate atoms changed (+9,771,352 B), plus the audit's TAV constants (+4,316,002 B in TAV_W). Nothing else moved.
- **The witnesses:** all 9, plus the deep-input witness, pass in my debug build and in my release build, one process each. W2 and W2b now assert their outcomes.
- **Mutants.**
  - I re-ran 83 of I65's 167 (all 19 of G6's, including the 5 on the G-C fact and Q10, plus RV89's 24 and part 2's 40). They equal I65's record mutant for mutant: 80 killed, 3 recorded survivors.
  - Of my 9 on the registration and the G-C fact, 6 are killed and 3 survive. R9 is equivalent inside D1. **R8 and R6 are test gaps (S-2, S-3).**
  - None was killed only by a compile error.

## Findings

| ID | Sev | Where | Evidence | Remedy |
|---|---|---|---|---|
| **S-1** | SHOULD-FIX (registration package) | `P/core/runner/headless/tests/retained_precision_admission.rs:82`, which `registration.diff` does not touch | **Registration turns one runner/headless test red.** `explicit_headless_refusal_preserves_output_and_completion_fields_both_modes` asserts `report.profile == ProfileStatus::Missing`.<br>– With the entry applied, the runner's workspace builds PP with the registered identity, and the same reviewed-input record (it hashes PP's own lock).<br>– So the Headless report's profile is `Registered` (`left: Registered, right: Missing`, `evidence/g6/runner_registered_failure.txt`).<br>– Behaviour is right: Headless is still refused at D1.0, and the runner's output bytes equal the ordinary run's. Only the stale assertion fails.<br>– I65's registered run covered PP only. The unregistered runner run, which I65 and I both did, cannot show it | Add the hunk to `registration.diff`: the expected profile is `Registered` in the registered build and `Stale` otherwise, like PP's own admission test. Apply it with the entry under ruling 3, and re-run runner/headless registered |
| **S-2** | SHOULD-FIX (test-only) | `PP/src/retained_memory_law_tests.rs:1157–1168` (`attempted_examples`); `retained_memory.rs:2545–2547` (line numbers at `2bb81ec1ea`) | **The deferred-formation arm of the attempt fact is reachable in D1 but untested.**<br>– `ordinary_solve_attempted` counts a seed whose `initial` is `FormationFailure`, which is F1b's `RangeDeferred` arm (lib.rs:4143). That is correct: my counter counts that arm as an attempt, and the ruling admits solves "of any quality".<br>– No committed example reaches it. I65's attempted examples are the milestone, a 1e-300 spring and `rejected_stress_range`, and W6's seed is `StructuralFailure(Range)`, not `FormationFailure`.<br>– So my mutant R8, which stops counting `FormationFailure` as attempted, **survives**.<br>– **The arm is live inside D1.** K2a's four product-reach shapes (tests/k2a_formation_range_runtime.rs, rebuilt by RV89) are all inside D1. Each seeds `FormationFailure`, agrees with my counter, and under registration reaches W1: `spring carried` falls back at `Preparation`, and `partial underflow`, `exact zero` and `least subnormal` (W2 published, `MECHANICS_SOLVED`) fall back at `Candidate`. Each gains the N1 notice | Add one K2a shape (for example `partial underflow`) to `attempted_examples()`, so that both `g_c_declines_…` and `registered_g_c_…` pin the arm. Re-run R8 |
| **S-3** | SHOULD-FIX (test-only) | `retained_memory.rs:2906` at `2bb81ec1ea`, :2921 once registered (`admit`'s bound closure) | **No test pins that admission adds R.** My mutant R6 passes `0` in place of `RESERVED_STACK_BYTES` to `bound_admits` inside `admit`, and it **survives**.<br>– Today E_mov,max (3.53 GB) and E_mov,max + R (3.59 GB) are both ≤ M, so the verdict cannot tell them apart.<br>– `bound_admits` is tested only as a pure function, and `admit_grants_…` recomputes `required` itself rather than reading admit's value.<br>– A regenerated profile within 64 MiB of M would be admitted wrongly | Record the bound's `required` in the private law record and assert it equals `cap_priced_maximum(mode) + R` in `admit_grants_…`, or make the closure a named pure function and test it at M − R − 1, M − R and M − R + 1 |
| N-1 | NOTE | BUILD.md §2.3 residual; the lock record in `build.rs` | **Registration makes the lock residual live for the first time.**<br>– The reviewed-input record hashes PP's own `Cargo.lock` (serde_json 1.0.149).<br>– A workspace that builds PP under another lock still reports `Registered`: runner/headless and self_weight_wasm resolve serde_json 1.0.151, and operation_applier 1.0.150.<br>– Today no other workspace calls the Direct entry. Runner/headless asserts it never does (`tests/retained_precision_admission.rs:137`), the desktop app is source-tested to call only the ordinary wrapper, and Headless is refused at D1.0. The serde_json layout witnesses still guard `Value`, `Number` and `Map` | None now. Any future Direct caller in another workspace needs its own reading. This is the accepted residual |
| N-2 | NOTE | The registered sweep and RV89's G-C probe | **The published bytes that change under registration, observed:**<br>– **successors** for the milestone and five in-D1 milestone variants (schema 0.2.0, 128-B text, 128-B two-byte text, raw depth 16, escaped raw text), in both modes;<br>– **the N1 notice appended** where the ordinary solve was attempted and W1 then fell back: a 1e-300 spring stiffness (`Preparation`); the W6 force-scaled cantilever (W2 published → `Native`); both `rejected_stress_range` fixtures (Sensitive solve, then finalization blocked → `Preparation`); and K2a's four range shapes (S-2: `Preparation` or `Candidate`).<br>– Everything else in D1 keeps exact bytes: not attempted (38 runs: 19 kinds × 2 modes) or exact-selected (`Coexistence`, 18 runs).<br>– Every report's `profile` reads `Registered`, including Headless reports, which are still refused | For U7's activation record. Under ROUTING:98 all of these are W1 work that actually ran |
| N-3 | NOTE | Mutants | **Equivalent survivors, as recorded:**<br>– I65's "Estimate count ignored" and "estimates not counted" now that ESTIMATES = 0;<br>– V19 (build.rs);<br>– my R9 (the attempt fact reads only the first seed), equivalent because D1.4 admits one case and so one seed | None |
| N-4 | NOTE | QUALIFICATION §3; ROOT's heads-up on RV87 SF-1 to SF-3 | **The dense margin to 0.9 M is now 29.3 MB, and the text-error budget is 1.86%,** which is within the ruled standard. ROOT has ruled RV87's SF-1 to SF-3 repaired before registration, moving the maximum by about +0.6 MB (to about 0.8929 M dense), with an updated `registration.diff`. **This review covers `2bb81ec1ea` and the current `registration.diff`;** that delta is to come as a follow-on | Review the delta's constants, pinned record and updated `registration.diff`, and fold S-1 into it |

## 1. registration.diff in a registered copy

**What it changes** (read in full):
- **One `RegisteredProfile` entry:**
  - the identity text;
  - the reviewed-input text;
  - `reader_layouts` [56/8, 64/8, 96/8, 16/8];
  - `threshold_bytes: 4_026_531_840`.
- **Law tests:**
  - `the_registered_profile_is_the_only_permit_source`: two `RegisteredProfile {` literals, the definition and the entry, and one `CapturePermit { _profile` construction;
  - `admit_grants_a_permit_for_the_milestone_in_the_registered_build`;
  - `registered_g_c_declines_only_unattempted_solves`;
  - two expectations flipped to `build_status()`.
- **The `tests` module:** two flips, `actual_retained_entry_dispatches_ordinary_once` on the two-case milestone and the forged law refusal.
- **Two out-of-fence test hunks:**
  - `retained_facade_tests.rs`: the two-case milestone;
  - `tests/retained_precision_admission.rs`: the expected profile, and the invalid document now gives exact bytes.

**Are the out-of-fence hunks only the flips that registration implies? Yes.** Each changes an expectation that registration makes false: `Missing` becomes `Registered`/`Stale`, and an admitted milestone stands in for a refused one, using the out-of-D1 two-case variant. None weakens a check. The facade control still compares the plain route with both retained entries, now on an input no build admits. **But one implied flip is missing** (S-1).

**The identity, inputs and layouts match what this build compiles** (my own checks):
- the identity text equals my part-1 compiled `OPS_RETAINED_BUILD_IDENTITY`;
- the reviewed-input text equals my compiled `OPS_RETAINED_REVIEWED_INPUTS`;
- my `shasum` of the 14 inputs at `2bb81ec1ea` equals both and G4's ORIGINS;
- in the registered copy, `COMPILED_REVIEWED_INPUTS == entry`, `READER_LAYOUTS == entry`, `LAYOUT_WITNESSES` holds, and `build_status() == Ok(0)`.

**Stale elsewhere** (`rv89g6_identity_and_bound`, run in each build):

| Build | Compiled identity differs at | build_status | admit (milestone) | Direct bytes |
|---|---|---|---|---|
| RUSTFLAGS=`--cfg rv89_stale` | `rustflags=--cfg%1Frv89_stale` | Stale | `Profile(Stale)` | ordinary, no successor |
| opt-level 1 | `opt_level=1` | Stale | `Profile(Stale)` | ordinary, no successor |
| `--release` | `profile=release;opt_level=3;debug_assertions=false` | Stale | `Profile(Stale)` | ordinary, no successor |

In each, I65's `the_registered_profile_is_the_only_permit_source` and `admit_grants_…` also pass, taking their Stale branches; `registered_g_c_…` was run too in the RUSTFLAGS and opt-level builds.

**`admit` in the registered build** (`rv89g6_admit_matrix`, both modes):
- **The milestone gets a permit:** refusal None, domain None, `Registered`.
- **Headless is refused** at `Caller(Headless)`.
- **21 violations each refuse** with `refusal == domain` at its own clause, and the Direct route then publishes exactly the value route's bytes with no W1 result: schema 0.3.0, a pressure contract, reference configurations, a material expansion law, sections, `section_ref`, two cases, a combination, a component, pressure regions, equivalent static, a modulus basis, a hanger, family `" anchor"`, an element target, a pressure dimension, an object provenance, a control byte, a DEL key, 129-B text, and raw depth 17.

**The bound at admission:** E_mov + R = 3,574,917,124 B (sparse) and 3,594,627,572 B (dense), both ≤ 0.9 M ≤ M.

**The successor bytes** (`rv89g6_milestone_publishes_the_pinned_successor`).
- Through the public Direct entry, the successor in U1's pinned framing (`{"id":"u1_milestone_<mode>","source":…,"invocation":…}`, pretty-printed) hashes to **`ac6986b0…59dc` (sparse) and `6cd1d249…c9b5` (dense)**, with `receipt_sha256` **`efc1a39b…7494` and `3e26499f…ac4a`**: the constants pinned in `retained_wire_tests.rs`.
- The ordinary base equals the value route's bytes, and `into_publication()` returns that successor.

**Suites, registered** (`--locked --offline`):

| Suite | Result |
|---|---|
| PP | 698 passed, 1 failed (t13), 11 ignored. The only outcome changes from part 2 are the added and renamed U4 tests |
| runner/headless | identical to base except `explicit_headless_refusal_…` (S-1) |
| My sweep (71 inputs × 2 modes × 5 routes) | Differs from base only by: `profile` Missing → Registered (276 report lines); `successor()` and `into_publication()` for the 6 publishing inputs (both modes). **No envelope line changed.** Unregistered, the sweep is byte-identical to base |

## 2. G-C's solve-attempt fact (ruling 2(a))

**The predicate and the cited exits** (read): `ordinary_solve_attempted` is `!ordinary.is_empty() && every seed's initial is Some`.
- `initial` is set only by `ordinary_initial_failure`, called at the `Err` of the one `attempted_linear` match (lib.rs:4125–4148), and by `ordinary_report` (:4537–4543).
- `linear_solve` is `Some` for every `Ok` attempt (:4281–4284), and `append_integrity_report` always pushes the record whose `last()` `ordinary_report` reads.
- The post-attempt `return`s at :4351, :4371 and :4382 are in `Err` arms. :4510 is behind `nonlinear_supports`, which D1.6 excludes.
- Pre-attempt exits leave no seed or an unset one.
- **That matches I65's §8a.**

**The oracle (independent).**
- In my registered copy, I added a test-only thread-local counter after lib.rs's single `let attempted_linear = match stiffness {…};` statement. It counts both arms: a formed solve, and F1b's deferred-formation attempt.
- I then ran the observed ordinary route on 91 inputs in both modes (182 runs): my 71 sweep inputs, both `rejected_stress_range` fixtures, the W6 cantilever, and 20 milestone variants of my own (`evidence/g6/probe_output.txt`).
- **Result: `attempts > 0` ⇔ `ordinary_solve_attempted` in all 182 runs, with 0 mismatches.** This holds inside and outside D1, and for multi-case requests, where the counter reaches 2 and every seed is set.

**The predicate holds for every outcome I could drive inside D1.** Under registration, through the public Direct entry:

| Outcome (inside D1) | Examples | Attempted | Registered Direct |
|---|---|---|---|
| Validation blocks | invalid document kind; unknown restraint token; coincident nodes; wall thickness > radius; zero modulus; tiny OD; missing material; unknown pipe endpoint; duplicate node ids; bad units; a unit mismatch; no supports; RV89's cap-maximal shape (`PROVENANCE_INPUT_MISSING`) | no | `CompleteGate(OrdinarySolveNotAttempted)`; **exact value-route bytes, no notice** |
| Load input blocks (:3918) | invalid category; bad direction; a load on an unknown node; zero loads | no | same |
| Mechanism blocked before the attempt | lone spring; negative springs (`SOLVER_SYSTEM_BLOCKED`) | no | same |
| Solved | the milestone and 5 variants | yes | successor |
| Exact-selected | n05/n06 and their ui twins; the sensitive torsion model | yes | `Coexistence`, exact bytes |
| Attempt fails, then blocked | 1e-300 springs (`NUMERICAL_INTEGRITY_ASSEMBLY_UNRESOLVED`) | yes | W1 `Preparation`, N1 notice |
| Solved, then finalization blocks | `rejected_stress_range`, both fixtures | yes | W1 `Preparation`, N1 notice |
| Formed solve fails Range, W2 publishes | W6 (`StructuralFailure(Range)`) | yes | W1 `Native`, N1 notice |
| **Deferred formation** (F1b `RangeDeferred`, `FormationFailure` seed) | K2a's spring-carried, partial-underflow, exact-zero and least-subnormal shapes | yes | W1 `Preparation` / `Candidate`, N1 notice (S-2: untested by the committed tests) |

**`rejected_stress_range`, judged.** Its ordinary solve ran: my counter is 1 and its seed is a Sensitive report. Only afterwards did the legacy source-block finalization block the envelope (`SOURCE_BLOCK_RECOVERY_FINALIZATION_FAILED`). W1 then ran its preparation and fell back.
- Under ROUTING:98 the notice is for W1 work that actually ran, and here it did. So keeping U3's N1 notice is consistent with the ruling, and with C1:64, which bars only W1 that should be declined before execution.
- Its pre-existing non-determinism (U6a F1: the validator's first error code varies between runs) carries into the registered Direct output unchanged.

## 3. The closed Estimates (QUALIFICATION §2)

**Exactly the 42 Estimate atoms changed between part 2 and G6** (my diff of the two printed records). No other atom moved. The forms changed only in TAV_W (+4,316,002) and TAV_X (+33,295,754), from the audit. The W3 delta from part 2 is +14,087,354 B dense = 9,771,352 + 4,316,002, exact.

**The large ones, bound to the right owners** (read against FK source):
- **`LazyE`** = `fkr::LAZY_ROW` = `size_of::<(ExactWideSum, ExactWideSum, u64, u64)>()`. This is exactly `BoundedExtremeTracker.lazy`'s element (adaptive.rs:687): 4,304 B = 2 × 2,144 + 16.
- **`TrackerE`** = LAZY_ROW + 3 × the table entry.
  - The table entry is `(u64, Evaluated)` (adaptive.rs:689). `Evaluated` is `Ratio(f64) | Refused { seq: u64, stop: AttemptStop }` (:637–640), bound by field sum (u64 + AttemptStop + tag), which is sound.
  - The three table-sized entries are the table, the kept table and the stable-sort scratch. That is per offer, which is P3's Tracker(O) law, and conservative.
- **`Node(TrackerKey,Tracker)`:** the node law at key (RuleTest ≤ 1 B, u32, Kind) by field sum at max(4, align Kind), and value `BoundedExtremeTracker` in-build. Sound: the law is monotone in both sizes.
- **`LaneTerminal`** = `size_of::<ProductCertificateSpent>()`, the whole record per lane (4,168 B), counting more than the two `Option<ResidualWork>` it stands for. Conservative.
- **`RegistryE`:** one entry of each of OriginStore's six registries plus `SelectedOrigin`'s fields, by field sum (1,608 B).
- **`PreparedMemberEvent`** = PreparationEntry + PreparedAssociation + PreparedAnnulus + SectionPreparationWork, per member (1,304 B).

All are sound uppers. The spot-checked small ones match their owners too: Enclosure is 2 × Wide<16> = 288, and the row_scales bool vectors are 1 and 6.

## 4. The in-build maximum

**Reproduced** with my composition (COMPOSITION_G4 §1, plus C-N3) from the record's 244 atoms and the generated forms. All 47 forms and 244 illustrative atoms transcribe G6's `profile_tree.json`, and all 7 phases equal the record in both modes.

| Mode | E_mov + R | Fraction of M | Under 0.9 M |
|---|---|---|---|
| Sparse | 3,574,917,124 B | **0.887840 M** | 48,961,532 B |
| Dense | 3,594,627,572 B | **0.892735 M** | 29,251,084 B |

- **The text-error budget:** 29,251,084 / 1,569,180,716 = **1.864%** dense; 3.120% sparse.
- **The release record** equals the dev/test record apart from the identity line and the identity-gated skips.
- **The registered copy's own admission bound** (`cap_priced_maximum` + R) gives the same two numbers.

**The D-7 non-claims** in QUALIFICATION §6 are stated, as the rulings require: not RSS, allocator overhead or fragmentation; no concurrency; no supported-machine claim; no stack claim beyond the measured evidence; the statics counted in every phase. The cfg(test) argument also holds. QUALIFICATION names `ProductCapture`, `FrozenCandidate` (which holds a `ProductCapture`) and `ReservedNotice`, and my part-2 scan found test-only fields only in `ProductCapture`. None is an atom or contained in one. **The margin is thin (29 MB) but within the ruled standard** (RR "RV89 on U4 G5 part 2", margin standard): ≤ 0.9 M after the audit, with the budget stated.

## 5. The S1 evidence

**All nine witnesses plus the deep-input witness, run one process each** (`--ignored --exact --test-threads=1`) in both builds:

| Witness | Debug (registered copy) | Release |
|---|---|---|
| W1 milestone | Successor ×2 | Successor ×2 |
| W2 cap-maximal (every provenance escaped, raw depth 16) | Fallback(Preparation) ×2, asserted | same |
| W2-deep | Successor at 4 MiB and 1 MiB, both modes | same |
| W2b cap-maximal, solvable | Fallback(Candidate) ×2, asserted | same |
| W3 n05, exact-selected | ExactSelected ×2 | same |
| W4 preparation refusal | Fallback(Preparation) ×2 | same |
| W5 | the dense halves above | same |
| W6 force-scaled | Fallback(Native) ×2 | same |
| W7 U3 faults | Native; Serializer(Encoding); Staging; Precommit G8; Precommit G1 | same |
| Headroom W1 at 1 MiB | Successor ×2 | same |

Every run passes, with no overflow, abort or panic, matching I65's `per_identity/witnesses.{test,release}.txt`.

- **The standard is stated as measured, not proved.** QUALIFICATION §4's heading reads "measured evidence for these builds and inputs, not a proof". Carry 8's limit, the panic-hook residual and the lexical call graph are listed. The 40-frame chain matches RV83's u4_g2_04 N-3 (39 → 40).
- **W2** now escapes every provenance and asserts `Fallback(Preparation)`. **W2b** asserts `Fallback(Candidate)`. **W2-deep** asserts it is inside D1, at raw depth 16, and publishes at R/16 and at R/64. That is RV89's part-2 S-2, closed.

## 6. Part-2 S-3

`maximum_takes_every_phase` (law tests :943–964) makes each of the 7 phases in turn the largest, by total and by moving part alone. It also checks that a tie keeps the earliest phase, and that an overflow in any requested, moving or summed part gives `None`. **Q10 (`while i < 5`) is killed by it** in my re-run of I65's G6 set, and by no other test.

## 7. Mutants

All mutants were run with `cargo test --lib retained_memory`, one job at a time.
- **I65's sample** ran in WT/rv89/mutU, restored from my unregistered `2bb81ec1ea` copy before each mutant.
- **Mine** ran in WT/rv89/mutR, restored from a registered pristine copy.

| Set | Run | Killed | Survived | Compile-only |
|---|---|---|---|---|
| I65 G6 (19: 9 FK exports, 4 profile bindings, Q10, 5 G-C fact) | 19 | 19 | 0 | 0 |
| I65 RV (RV89's 24) | 24 | 23 | V19, as recorded | 0 |
| I65 P2 (part 2's 40) | 40 | 38 | the two Estimate-count mutants, equivalent at ESTIMATES = 0 | 0 |
| **RV89 R1–R9** (`evidence/g6/mutants_rv89_g6.py`) | 9 | 6 | R6 (S-3), R8 (S-2), R9 (equivalent in D1) | 0 |

I65's 83 re-run mutants equal its record (`controls/mutants_g6.out.jsonl`) one for one.

**Mine, on the registration and the G-C fact:**

| Mutant | Result | Killed by |
|---|---|---|
| R1: the registered identity names rustc 1.97.2 | KILLED | `the_registered_profile_is_the_only_permit_source` |
| R2: one digit of the lock hash in the entry | KILLED | `admit_grants_…`, `registered_g_c_…`, `the_registered_profile_…` |
| R3: the entry's Validation layout 64 | KILLED | the same three |
| R4: threshold 3,500,000,000 (below E + R) | KILLED | `admit_grants_…`, `milestone_and_cap_maximal_inputs_are_inside_d1`, `registered_g_c_…` |
| R5: threshold `u64::MAX` | KILLED | `the_registered_profile_…` |
| R6: admit drops R from the bound | **SURVIVED** | S-3 |
| R7: the attempt fact inverted | KILLED | `complete_facts_read_the_actual_owners`, `g_c_declines_…`, `registered_g_c_…` |
| R8: a deferred-formation attempt not counted | **SURVIVED** | S-2 |
| R9: the attempt fact reads only the first seed | survived | equivalent in D1 (one case) |

## 8. The identifier audit (spot check only; RV87 owns it)

- My part-2 S-1's five sites are in `id_audit_table.json` as `RES`, at 1,024 B each: `source_receipt.rs:1016`, `rows.rs:533`, `:590`, `:592`, and `lib.rs:5592`.
- The enforcement control (`controls/audit_enforcement.txt`) reports `id-unaudited lib.rs:5592 matched[0].id` when that entry is removed.

## Execution record

- **Who.** RV89, TASK (Type 2) under ROOT. No descendants.
- **When.** 2026-10-04, about 07:20–07:50 MDT (about 30 minutes of the 3.5-hour box).
- **Memory guard.** `memguard.sh` PID 5387 was running, and every cargo job and mutant checked it.
- **Cargo.** The default toolchain (rustc 1.97.1), `--locked --offline`, `CARGO_BUILD_JOBS=4`, `RUST_TEST_THREADS=2` (`--test-threads=1` for the probes and witnesses), and `TMPDIR` in scratch. One cargo job at a time.
  - **The stale builds** used `RUSTFLAGS="--cfg rv89_stale"`, `CARGO_PROFILE_DEV_OPT_LEVEL=1` and `--release`, each in its own target.
- **Copies.**
  - WT/rv89/cand3 (unregistered) and reg3 (registered, `patch -p1`), as full archives of `2bb81ec1ea`.
  - mutU, pristR and mutR, as subset archives (pristR and mutR patched).
  - RV89's attempt counter and probe module exist only in reg3.
- **Not run.** No Git writes or index operations (Git reads used `GIT_OPTIONAL_LOCKS=0`; the diff was applied with `patch`, not `git apply`), no installs, no new tooling, and no native, solver-at-scale or DEC-025 jobs. Nothing was written to the system temp directory.
- **Writes.** Only R/REVIEW_RV89/u4_g6_01/, WT/rv89/, WT/targets/rv89/ and WT/scratch/rv89_u4_g5/. The evidence here has machine paths replaced by `WT`.
- **Evidence** (`evidence/`):
  - `g6/`: my probe module, its output (`probe_output.txt`), the stale-build output, the witness output in both builds, both sweeps and their diff against base, my profile check, the mutant scripts and outputs, and the run scripts;
  - `outcomes/`: the registered PP and runner/headless outcomes, and base's runner outcomes.
