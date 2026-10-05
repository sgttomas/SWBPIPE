# RV89: independent review of U4 G7 Pass A, the delta re-qualification on the integrated basis

**Reviewer:** RV89, TASK (Type 2), dispatched directly by ROOT. No descendants. This continues RV89's G5 and G6 reviews (`R/REVIEW_RV89/u4_g5_*`, `u4_g6_01`, `u4_g6_02`) with the same context.

**Candidate.** I65's Pass A (`R/I65/u4_g7_01/`: RETURN.md, QUALIFICATION_G7.md and `_run_records/`), on the clean merge tree `ba1faa1c858ce3630a22767677310b1902a14b83` of NUM `f172f86abe` (U6) and memory `0c7827b6ad` (registered).
- My copy is a `git archive` of that tree, under WT/rv89_g7/.
- The brief is `BRIEFS/I65_U4_G7_DELTA_REQUALIFICATION.md`.

**Oracles:** RV89's own. I65's scripts and records were read, and run only where noted (§4, §5).
- **The build.** In my registered build of the tree I ran the law tests, all nine witnesses, the challenge, PP and runner/headless.
- **My sweep** (71 inputs × 2 modes × 5 routes).
- **My G6 probe module**, with its solve-attempt counter.
- **A new G7 probe** (`evidence/g7/rv89_g7_probe_tests.rs`).
  - Copy-only counters in result_export count `retained_precision::validate`, every `is_retained` call and true result, the new static's initialization, the `forbid_retained_*` refusals and `validate_transport_metadata`.
  - It runs 13 hostile but D1-admissible variants of the milestone, then a positive control.
- **A Stale build** (RUSTFLAGS).
- **The pricing**, evaluated independently from the committed forms and my own record.
- **The proposal**, regenerated with I65's generator.
- **The Pass B gates**, exercised on real and constructed inputs.

## Verdict: **PASS** for Pass A

| | Count |
|---|---|
| BLOCKING | 0 |
| SHOULD-FIX | 1 (Pass B, for the final basis) |
| NOTE | 3 |

**Pass A's conclusions hold. The registered entry stays byte-identical on `ba1faa1c`, and the maxima are unchanged.**

**The proposal is correct and complete,** and nothing else needs regenerating.

**The Pass B script fails closed as claimed:**
- **exit 3** on a Stale build, including a missing or empty law log;
- **exit 4** when a rule's line lies in a changed hunk.

**But it is not, by itself, a re-qualification of grant 2's delta** (S-1). It has no production-delta inventory step, so new non-text allocations in grant 2's `lib.rs` would go unpriced while every check reads "identical". Everything after its two stops only reports. S-1 does not affect Pass A.

**The headline:**
1. **The `is_retained` branches and the new static cannot be reached from `validate` on D1, hostile input included.**
   - **Statically.** On the D1 path, PP calls result_export only at lib.rs:3161 (`retained_precision::validate`). `validate` reaches `semantic_contract` only through `for_source(&projected)` (:4305), and `validate_transport_metadata` only through `for_source_metadata(&projected)` (:4331).
   - `project` **assigns** the literal `…/preview-physics-1` id (retained_precision.rs:4252–4253). It removes the top-level `retained_precision` member, and with `raw` it removes every row's `recovery_method` (:4248–4251, :4255–4263).
   - So on every D1 call, `is_retained` is false, and both `forbid_retained_*` checks return `Ok`. Nothing else in the reached graph calls `is_retained`'s callers or `preview_physics_retained_contract`.
   - **Dynamically.** Under registration, 13 hostile variants all stayed inside D1 and published. They put the successor identity, `retained_precision`, `recovery_method`, the method token or `producer` into ids, provenance, labels and extra request and model members.
     - Each ran the reader: `validate` 1 and `is_retained` 4 per run, across 26 runs.
     - **0** `is_retained` true results, 0 static initializations, 0 downgrade refusals, 0 transport calls.
     - Over the whole probe process the totals were `validate` 30 and `is_retained` 120, with the same zeros. My 71-input sweep gave `validate` 24 and `is_retained` 96, with zeros.
     - **The positive control fires:** `for_source` on the unprojected successor counts 1 true result and 1 static initialization.
   - **If the static were reached** it would cost 286,836 B (reproduced), but that is the smaller consequence (N-2).
2. **The integrated registered build is Registered.**
   - The compiled identity and the reviewed-input text equal `REGISTERED_PROFILES[0]` byte for byte. My own `sha256` of the 14 inputs in my copy gives 14 of 14 equal, and the layouts are 56/64/96/16, all with alignment 8.
   - My probe gives `build_status() = Ok(0)` and E + R = 3,575,778,286 / 3,595,488,734 B.
   - The pinned successor is unchanged (`ac6986b0…` / `6cd1d249…`), and `admit` still grants the milestone and refuses Headless and the 21 D1 violations.
   - **My own runs:** all nine witnesses pass with G6's outcomes, and the challenge peaks are 3,541,898 / 2,252,863 B.
   - **Suites.** PP gives 699 passed, 1 failed (t13), 10 ignored. runner/headless gives 85 passed, 2 failed. Both are outcome-identical to my G6R registered runs and to I65's Pass A.
   - **My sweep is byte-identical to the G6/G6R registered sweep** (`25cce1e1…`), so U6 changes no published byte on my inputs.
3. **F5's pricing is right.**
   - `exact` grows by push from capacity 4 and doubling, so it holds at most pushcap(D_env) = 16,384 slots.
   - The refs vector is exact-size, at most D_env entries.
   - Both are live only within one case's iteration of `g5_ordinary`, which is T17's V4 stage.
   - That gives 8 × 25,745 = **205,960 B**, with V4 at 16,966,805 → 17,172,765 B, which is **1,131,825,961 B below V2_hash**. The last growth's moving extra is 65,536 B.
   - **Domination at every D1 census is not needed** (§3), because the build takes T17 as the maximum of the evaluated stages.
4. **The proposal (20, 57,880) → (20, 83,625) is right and complete.**
   - The coefficient delta is 25,745 = 16,384 + 9,361.
   - It is byte-identical to `g5_profile.py`'s regeneration from G7's `profile_tree.json`, and G6's tree still regenerates the committed block exactly.
   - The two trees differ only in that coefficient. `profile_tree.json` is a record, not source.
   - No FORMS pin exists. `PINNED_RECORD`, `PYTHON_CHECK` and the challenge pins do not move.
   - With the proposal applied in my copy, PP outcomes, law outcomes, the printed record and the challenge are identical.
5. **Pass B:** see S-1.

## Findings

| ID | Sev | Where | Evidence | Remedy |
|---|---|---|---|---|
| **S-1** | SHOULD-FIX (Pass B, before it is relied on for the final basis) | `R/I65/u4_g7_01/_run_records/g7_pass.sh` | **The two stops work** (`evidence/g7/pricing_proposal_passb.txt` §3):<br>– **Exit 3.** The identity gate continues on my registered log, and stops on my real Stale (RUSTFLAGS) log and on an empty log. That last case is a JSON failure, so any law-build failure is also exit 3.<br>– **Exit 4.** The line map gives exit 0 from `ba1faa1c` to itself and exit 1 from `ba1faa1c` to `0c7827b6ad`, with 4 unmapped `edge_zero` callees in changed hunks.<br>**But the script is not a sufficient re-qualification of grant 2's delta:**<br>– **(a) No production-delta inventory.** Brief items 1 and 3 are not repeated. Grant 2 changes `PP/lib.rs` and the dispatch hook, which are on the D1 path. The TEXT chain prices only text, and the profile's other forms are hand-composed in `g4_caps.py`. So a new `Vec` in grant 2 would pass unpriced while TEXT, the profile block and the outputs all read "identical". `delta_inventory.py` exists but is not called.<br>– **(b) Everything after the two stops only reports, and the script exits 0.** That covers a tree mismatch (:23), added statics (:31–32, though the brief makes a reachable new static a stop), TEXT, output or outcome deltas, controls below 11, and the §11 verdict.<br>– **(c) The entry is compared only with the basis's own entry,** and `threshold_bytes` not at all. With the entry's threshold doubled, `identity_check.py` still reports `registered: true`. The registered law tests are recorded but not gating.<br>– **(d)** `TB_D=14734` is hard-coded for the §11 non-candidate run (:62), so a changed D would make that comparison silently stale.<br>– **(e)** No rule is keyed to the warrant of the dead `is_retained` branches (`project`'s literal assignment, retained_precision.rs:4252–4253, and `validate`'s single call at :4305). An edit there would not stop Pass B (see N-2) | **(a)** Run `delta_inventory.py PASS_A_REV REV`, and stop with a distinct exit (for example 5) on any production hunk that is live on D1. That hunk needs the human inventory and pricing of items 1 and 3.<br>**(b)** End with one verdict line. Exit non-zero (for example 6, "deltas to read") whenever anything differs from Pass A beyond the expected `T17_V4` line.<br>**(c)** Compare the entry block byte for byte with `0c7827b6ad`'s, threshold included, and gate on the registered law tests passing.<br>**(d)** Read D from the run's summary.<br>**(e)** Pin `project`'s and `validate`'s lines, as a keyed rule or a source-pin test, so an edit stops the pass |
| N-1 | NOTE | F5's counting rule (QUALIFICATION_G7 §3; `price_delta.py`) | **The refs bound has a slightly different warrant than stated.** "≤ D_env because the refs are unique and resolve" strictly gives at most \|successor diagnostics\| ≤ D_env + 1, because the successor adds `RETAINED_PRECISION_SELECTED` (retained_wire.rs:767). The bound D_env itself comes from the producer: retained_wire.rs:1426–1429 builds the refs with F5's own filter, so on a passing check refs equal `exact`. `exact` excludes `RETAINED_PRECISION_*` and so has at most D_env entries. The difference is at most 8 B | None needed. Cite the producer as the warrant |
| N-2 | NOTE | `semantic_contract.rs:438–443`, :270–275; `retained_precision.rs:4246–4262` | **The dead-branch warrant is load-bearing far beyond the static.** If `is_retained` were ever true on D1, `for_source` would run a second full `validate(source, None)` inside the first, while the outer V5 working set is live. T17 would grow by at least V5 (180,109,775 B) plus the static (286,836 B). Dense W4 would then reach at least 3,749,527,510 B with R, above 0.9 M (3,623,878,656). Today the warrant is exact, as shown statically and dynamically, but it lives entirely in `project`'s literal assignment | S-1(e). Keep the `edge_zero` rules' `why` citing `project`, and state the magnitude in QUALIFICATION_G7 §1 |
| N-3 | NOTE | The generated block's header, `retained_memory.rs:1051–1055` | Regeneration keeps the header's provenance text ("…NUM 1e323058f3 (G5 part 1 code)"), and with the proposal adopted the F5 term comes from `ba1faa1c`. This is cosmetic: the generator emits this text, and the block equals regeneration | Optionally name the G7 basis when adopting |

## 1. Reachability of the `is_retained` branches and the new static (ROOT item 1)

**The D1 call path into result_export** (read in my copy of `ba1faa1c`):
- PP (unchanged since `0c7827b6ad`) uses result_export at lib.rs:3161, `retained_precision::validate(&successor, Some(&invocation))`, and otherwise only for type layouts (retained_memory.rs:921–927).
- No other D1 crate depends on result_export: its only dependents are PP, runner/headless and report_package.
- **Inside the reader,** `validate` calls `semantic_contract` only at :4305, `for_source(&projected)`, where `projected = project(source, true)`. `validate_transport_metadata` calls it only at :4331, `for_source_metadata(&projected)`, with `project(source, false)`. That one is not reached on D1.
- **`project`** (:4246–4265) clones the source, removes `retained_precision`, and **assigns** `projected["producer"]["semantic_contract_id"] = "…/preview-physics-1"`.
  - serde_json's `IndexMut` creates or overwrites the member. It panics, rather than leaving another value, only if `producer` is not an object, and PP's successor always has an object there.
  - With `raw`, it removes `recovery_method` from every row.
- **So on the projection:**
  - `is_retained` (`== CONTRACT_ID`, `…/preview-physics-retained-1`) is false;
  - `forbid_retained_member` finds no `retained_precision`;
  - `forbid_retained_rows` finds no `recovery_method`.
  
  All three are `Value` indexing and `Value == &str` comparisons, and allocate nothing.
- **The callers of the retained pieces in result_export:**
  - `is_retained`'s other callers are `rule_binding_refusal`, `numerical_use_standing_with_context` and `retained_row_classes`. They are called only by `derivative.rs` and by crates outside the D1 path.
  - `preview_physics_retained_contract` is called only at :274 and :443, both inside `is_retained` branches.
  - `verify_preview_physics_retained_table` is called only at :141.
- **The one changed data file that production Rust embeds,** `results.v0.3.schema.yaml`, is parsed per call in `physics_evidence::validate_transport_metadata`. Its only caller is `load_reference.rs:367`, under the `LOAD_REFERENCE_*` arms, which the projection's literal id never selects. **It is not reached.**

**The dynamic probe** (`evidence/g7/reachability_probe.txt`, instrumented copy, registered build):

| Run | Inside D1 | `validate` | `is_retained` calls / true | static init | `forbid` refusals | transport |
|---|---|---|---|---|---|---|
| 13 hostile variants × 2 modes, each | all 26 | 1 | 4 / **0** | **0** | **0** | 0 |
| All probes in the process (G6 probe + hostile) | | 30 | 120 / **0** | **0** | **0** | 0 |
| My sweep (71 inputs × 2 modes × 5 routes; 24 reader runs) | | 24 | 96 / **0** | **0** | **0** | 0 |
| **Positive control:** `for_source(unprojected successor)` | (off D1) | 1 | 5 / **1** | **1** | 0 | 0 |

**The hostile variants:**
- the project id, the case id and the material id set to the successor identity;
- every provenance set to the method token, or to the identity;
- the case label `retained_precision`;
- N1 renamed `recovery_method`, or the method token;
- the segment renamed `retained_precision`;
- load ids prefixed `producer`;
- `model.producer = {semantic_contract_id: <identity>}`;
- a top-level `retained_precision` member, and a `model.retained_precision` member.

All 26 runs stayed inside D1 and published a successor. D1 bounds raw members by its census caps; it is not a schema closure. The instrumented sweep is byte-identical to the uninstrumented one.

**If it were reached,** the static's parsed tree is 286,836 B by G4's static rule at in-build strides: s(Value) 32, Node(String,Value) 736, 199 array slots, 95 objects, 973 entries and 35,924 text bytes. As a process-lifetime static it would count in every phase from its first use. N-2 gives the larger consequence.

**I65's inventory, spot-checked:**
- The delta touches exactly 3 production `.rs` files (derivative.rs, retained_precision.rs and semantic_contract.rs). PP is unchanged.
- **Statics:** the only production static added is the one above. The other `include_str!` added is in `#[cfg(test)] mod u6e_reader_round_tests`.
- **retained_precision.rs** has three hunks: the comment, F5 (:4136–4145) and the test module.
- **`derivative::`** has no caller in PP or in the rest of result_export.

## 2. The integrated registered build (ROOT item 2)

**The law tests** (`--lib retained_memory`, my target): 42 passed, 0 failed, 9 ignored, including:
- `the_registered_profile_is_the_only_permit_source`;
- `admit_grants_…`;
- `registered_g_c_…`;
- `reviewed_inputs_bind_the_lock_and_the_reader_statics`;
- `admission_bound_adds_r_…`;
- `profile_in_build_record`;
- `challenge_bounds_are_the_profile`.

**My comparisons** (`evidence/g7/identity_witnesses_challenge.txt`):
- `I65_G5_IDENTITY` equals the entry's `identity`.
- `I65_G5_REVIEWED_INPUTS` equals the entry's `reviewed_inputs`.
- My own sha256 of each of the 14 named inputs in my copy equals the entry: 14 of 14.
- The layouts compile as Validation 56/8, ValidationError 64/8, RowClassification 96/8 and AccuracyClass 16/8.
- My G6 probe asserts `COMPILED_REVIEWED_INPUTS == entry`, `READER_LAYOUTS == entry` and `LAYOUT_WITNESSES`, gives `build_status() == Ok(0)`, and shows `ESTIMATES = 0`.

**The witnesses,** one process each, `--ignored --exact --test-threads=1`, all passing:

| Witness | Outcome |
|---|---|
| W1 | Successor ×2 |
| W2 | Fallback(Preparation) ×2 |
| W2-deep | Successor at 4 MiB and 1 MiB, ×2 |
| W2b | Fallback(Candidate) ×2 |
| W3 | ExactSelected ×2 |
| W4 | Fallback(Preparation) ×2 |
| W6 | Fallback(Native) ×2 |
| W7 | Native; Serializer(Encoding); Staging; Precommit G8; Precommit G1 |
| Headroom W1 at 1 MiB | Successor ×2 |

**The challenge:** the milestone is permitted, peaking at 3,541,898 B (sparse) and 2,252,863 B (dense). The cap-maximal input is not permitted, at 13,221,606 / 13,229,826 B.

**Suites:**
- PP, all targets: 699 passed, 1 failed (t13), 10 ignored.
- runner/headless: 85 passed, 2 failed (`load_reference` ×2).
- Both are identical outcome for outcome to my G6R registered runs (with my sweep's ignored test removed) and to I65's `pass_a/a_pp.outcomes`.

**Unchanged behaviour:**
- The milestone's pinned successor: `ac6986b0…` / `6cd1d249…`, receipts `efc1a39b…` / `3e26499f…`.
- `admit` grants the milestone and refuses Headless and 21 D1 violations.
- My K2a and G-C probes agree, with 44 attempt checks and 0 mismatches.
- **My sweep is byte-identical to the G6/G6R registered sweep** (sha256 `25cce1e14090048263ef285cb1a571c41ccc34955e57de0fb388df598940ccbb`).

**Stale for contrast** (`RUSTFLAGS="--cfg rv89_stale"`): the identity differs only at `rustflags=`. The law tests still pass, taking their Stale branches, and the record prints `I65_G6_RECORD_SKIP` (`evidence/g7/stale_law.txt`).

## 3. F5's pricing (ROOT item 3)

**The code** (retained_precision.rs:4136–4145), per receipt case:
- `exact: Vec<&Value>` is collected from `ds.iter().filter(..).map(..)`. Filter's lower size hint is 0, so `Vec` starts at capacity 4 and doubles on each push past capacity. With n kept items its capacity is pushcap(n).
- `list(&o["diagnostic_refs"]).iter().collect::<Vec<_>>()` comes from a slice iterator, so it is allocated at exactly its length.
- Neither the filter nor the comparison allocates. `list`, `text` and `contains` borrow.
- Both vectors drop at the end of the case's iteration. They are inside `g5_ordinary`, which is in V4 ("G3–G6 working sets").

**Counts:**
- **`exact`:** the successor's diagnostics are the ordinary envelope's (at most D_env = 9,361), less an omitted legacy disclosure, plus `RETAINED_PRECISION_SELECTED` (retained_wire.rs:749–768). `exact` excludes `RETAINED_PRECISION_*`, so it has at most D_env entries and capacity at most pushcap(9,361) = 16,384.
- **refs:** at most D_env, by the producer's construction (retained_wire.rs:1426–1429; N-1).
- **The total:** s(&Value) × (16,384 + 9,361) = 8 × 25,745 = **205,960 B**.

**At in-build atoms** (`evidence/g7/pricing_proposal_passb.txt` §1; the committed forms, my record):

| Stage | Bytes |
|---|---|
| V1 | 321,428,243 |
| V2_clone | 172,256,082 |
| **V2_hash** | **1,148,998,726** |
| V3 | 32,598,012 |
| **V4** | 16,966,805, and **17,172,765 with F5** |
| V5 | 180,109,775 |
| V6 | 200,409,127 |

- **Headroom to V2_hash:** 1,131,825,961 B.
- **Moving extra** (F5's last growth): 65,536 B, against T17's moving publication of 189,303,281 B.

**On "domination at every D1 census", and V2_hash having no term 20:**
- **Correct:** V1 to V3 have no s(&Value) term. V4, V5 and V6 each carry (20, 57,880).
- **Why it does not matter.** The bound never relies on V2_hash dominating:
  - the build computes `t17 = max(V1 … V6) + output` (retained_memory.rs:2227–2229);
  - every stage is a monotone form evaluated once, at the D1 caps and the in-build strides.
  - So for any D1 census c, the run's V4(c) + F5(c) ≤ V4(caps) + F5(caps) ≤ the T17 bound, whichever stage is the maximum.
- **Domination at each census is neither needed nor claimed.** "T17 unchanged" is a statement at the caps, the only point the profile is evaluated.
- **The stride is fixed.** s(&Value) is `size_of::<&'static Value>()`, InBuild, which is 8 in the registered identity's 64-bit target. A different stride is a different identity, which is Stale.
- **F5 alone can never overtake V2_hash.** At any census with d case diagnostics, F5 adds at most 8 × (pushcap(d) + d), which is under 24 B per diagnostic beyond the first four. ENVP, in V2_hash, carries each of those diagnostics as a parsed object of at least 736 B at the in-build node stride.

## 4. The proposal `pass_a/proposal/t17_v4_f5.diff` (ROOT item 4)

- **The coefficient is right:** 83,625 − 57,880 = 25,745 = pushcap(D_env) + D_env.
- **It is exactly the regeneration,** using I65's `chain/g5_profile.py`, read and run in my scratch:
  - from `pass_a/text_g7/profile_tree.json` (with F5) it gives the committed block with only the `T17_V4` line changed, **byte-identical to the proposal applied**;
  - from G6's `text_g6r/profile_tree.json` it gives the committed block byte for byte.
  - The two trees differ in exactly one leaf, `/forms/T17_V4/s(&Value)`.
- **Nothing else needs regenerating:**
  - `profile_tree.json` is a record (G7's already has F5), not source.
  - No test pins the forms. The pins are `PINNED_RECORD` (phase totals), `PYTHON_CHECK` (the maxima at chain-assumed atoms, still W3), the challenge's `W1_PHASE_BYTES` and `MAX_PHASE_BYTES`, and `admission_bound_adds_r_…`'s 0.9 M check. None reads V4.
  - No reviewed input, identity or layout is involved.
- **Nothing moves** (applied in my copy after restoring it from the tree, `phase3.sh`):
  - PP all-targets outcomes, the law-test outcomes, the printed record (identity, inputs, layouts, 244 atoms, all phases, both maxima) and the challenge lines are identical to the pristine runs (`outcomes/g7p_*`, `record_with_proposal.txt`);
  - V4 becomes 17,172,765 B, and T17, W4 and E_mov,max are unchanged.
- **Ordering with grant 2 is free.** Pass B's profile-block check reports 0 changed lines if the proposal is adopted first, and the `T17_V4` line if it is not. See N-3 for the header text.

## 5. The Pass B script (ROOT item 5)

**Read in full,** with `identity_check.py`, `tree_check.py`, `g7_linemap.py` and `delta_inventory.py`. It is parameterized by basis, revision and tag, its paths stay inside I65's fence, and it is Git-read-only and memguard-checked.

**The stops I exercised:** see S-1 and `evidence/g7/pricing_proposal_passb.txt` §3. I did not run the whole script, because it writes only to I65's scratch and targets, which are outside my fence.

**The line map's handling of bare `lib.rs:N` keys,** which matters because grant 2 changes PP's `lib.rs`:
- I attributed all 110 bare keys in the rules.
- The operative keys either carry a function name, which the map checks at the old line, or are PP-only lines past 13,900.
- The ambiguous no-fn keys are all in descriptive fields, which the map leaves alone.
- So I found no silent mis-mapping at this basis. A rule keyed in a hunk that grant 2 edits stops the pass (exit 4), as intended.

**Is it fit to rerun mechanically on the final basis?**
- **As a regression check, yes:** identity, statics, TEXT row for row, the outputs, the §11 sweep, the witnesses, the challenge and the suites against Pass A.
- **As the re-qualification of grant 2's D1-path changes, not by itself** (S-1(a)). Its exit 0 also needs reading (S-1(b)).

## Execution record

- **Who.** RV89, TASK (Type 2) under ROOT. No descendants.
- **When.** 2026-10-04, about 09:42–10:10 MDT, within the 2.5-hour box.
- **Memory guard.** `memguard.sh` PID 5387 was running, and every cargo job checked it.
- **Cargo.** The default toolchain (rustc 1.97.1), `--locked --offline`, `CARGO_BUILD_JOBS=4`, `RUST_TEST_THREADS=2` (1 for the witnesses, the challenge and the probes), and `TMPDIR` in scratch. One cargo job at a time, with targets in WT/targets/rv89_g7/.
- **Copies.** WT/rv89_g7/base, a `git archive` of `ba1faa1c`.
  - **Phase 1:** pristine, plus my sweep test, added after the suites.
  - **Phase 2:** copy-only counters in result_export, plus my G6 and G7 probe modules and the counting sweep.
  - **Phase 3:** the four instrumented files restored from the tree, each checked byte-equal to the tree with `git show`, and the probes removed; then I65's proposal applied.
- **Not run.** No Git writes or index operations (Git reads, `git archive` and `git show` used `GIT_OPTIONAL_LOCKS=0`), no installs, no new tooling, and no native, solver-at-scale or DEC-025 jobs. Nothing was written to the system temp directory. WT/f2a-memory and I61's and I65's files were not touched.
- **Writes.** Only R/REVIEW_RV89/u4_g7_01/, WT/rv89_g7/, WT/targets/rv89_g7/ and WT/scratch/rv89_u4_g7_01/. Machine paths in the evidence are replaced by `WT` and `R`.
- **Evidence** (`evidence/`):
  - `g7/`: the probe module and the instrumentation script; the counting sweep; the phase and run scripts; `checks.py`; the reachability, identity, witness and challenge output; the Stale law output; the pricing, proposal and Pass B checks; the records with and without the proposal.
  - `outcomes/`: the registered PP, runner/headless and law outcomes, pristine and with the proposal.
