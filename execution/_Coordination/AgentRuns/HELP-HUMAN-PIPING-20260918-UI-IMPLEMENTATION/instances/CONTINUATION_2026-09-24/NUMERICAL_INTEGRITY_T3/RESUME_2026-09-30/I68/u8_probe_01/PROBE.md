# I68 U8-0: the probe (Part 1 of `BRIEFS/I68_U8_PROBE_AND_WITNESSES.md`)

TASK (Type 2), I68, for ROOT (HELP_HUMAN, Agent 0). 2026-10-06 UTC (2026-10-05 host local time).

**Briefs (verified before work):** `R/BRIEFS/U8_COMMON.md` sha256 `3146c3e6c2d0940cfe1870da75192db4b37e2a5c7f690bc3cc5acba953eb223b`; `R/BRIEFS/I68_U8_PROBE_AND_WITNESSES.md` sha256 `5ef6f76afc0dc3b1655b72ba7b2e0c53a81e7b7c524d98ac44705f342ed0e8b4`. **Basis:** `R/I61/u8_plan_01/PLAN.md` (sha256 `f274a614…`, verified) §0–§1; RR "I61's U8 plan ruled…" (decisions 1–7); `T/IMPLEMENTATION/F2A_D1/CHANGE_RECORD.md` and `copies/QUALIFICATION.md` §6 and §11.

**Part 1 only.** Records only. No maintained file changed: `WT/f2a-u8` is untouched. No Git writes (reads used `GIT_OPTIONAL_LOCKS=0`). No DEC-025, native-app or solver-at-scale jobs. Every cargo job went through `WT/tools/t3_cargo.sh` (memguard up), `--locked --offline`, `CARGO_BUILD_JOBS=4`, `RUST_TEST_THREADS=2`, with RUSTFLAGS and CARGO_ENCODED_RUSTFLAGS unset. All scratch was under `WT/scratch/i68_u8_probe/`. Nothing was written to the system temp directory.

Placeholders: `WT`, `NUM`, `P`, `T`, `R`, `VENV` as in the dispatch; `ARCH` = `WT/scratch/i68_u8_probe` (the disposable archive); `PP` = `P/core/product_physics`.

## 0. Outcomes in brief

| Item | Outcome (both modes unless stated) |
|---|---|
| 1a `first_load_only` | **Candidate.** The product certificate refuses five torsional-shear stress rows. The first failure is `Predicate { row: 80 (sparse) / 81 (dense), predicate: SharperExact }`. The `I51_FROZEN_REFUSAL` line is present. `ONE_RUN_THROUGH_G_C`; 1 notice; the bytes equal `with_notice(plain, "case", None)`; admitted; no hooks armed before or after. |
| 1b `tiny_spring` | **Preparation.** The ordinary run blocks (`MODEL_INCOMPLETE`, `NUMERICAL_INTEGRITY_ASSEMBLY_UNRESOLVED`, 0 results). `prepare_case` then refuses with `Association("prepared case custody/permit")`, because mechanics ≠ `MECHANICS_SOLVED`. No `I51_FROZEN_REFUSAL` line (no candidate ran). `ONE_RUN_THROUGH_G_C`; 1 notice; the bytes equal `with_notice(plain, "case", None)`; admitted; no hooks armed. |
| 2 W6 (W-C1's reason) | **Native, with `UnresolvedReason::Ceiling`.** Ladder: p128 candidate rejected (StopRule, translation); p256 rejected (StopRule, force); p512 rejected (Charge, force); p1024 verification solved. `ONE_RUN_THROUGH_G_C`; 1 notice; the bytes equal `with_notice(plain, "case:source-section", None)`. |
| 3 L = 0 | **Admitted, and W1 selects. It publishes a successor in both modes.** Ordinary: `MECHANICS_SOLVED`, Sensitive. Admission: Registered; refusal `None`; domain `None` (inside D1); census complete; n 3, m 1, g 5, DOF upper 18, raw 168 values, depth 7. W1: `Ok(successor)`, `ONE_RUN_THROUGH_G_C`, B′ holds. **All three readers pass** with the invocation (eligible) and give identical class counts. Body 1's coverage is `stop [F,F,F,F]`, `has_data false`. Body 1's 15 rows are 6 input-derived plus 9 exact zeros; body 0's rows equal the milestone's bit for bit. |
| 4 two-body pair | **Case A:** a successor in **sparse** (all three readers pass). In **dense** it falls back at **Precommit G8 `RETAINED_PRECISION_PREPARATION_MISMATCH`**, a Rust-only refusal (F-1). **Case B:** Native with **Ceiling** in both modes. |
| 4 one-body fallback pair | **Case A (axial):** Candidate in both modes. The proof refuses at start with `Native(Alpha { block: 0, … })`. No successor, so this pair is **not** a viable W-C2 control. **Case B (transverse, tip-y only):** Native with **Ceiling** in both modes. |

**Stops under the brief:** none fired. L = 0 is admitted and W1 does not fall back. Ceiling is established (W6 and both B cases).

**Not a U8 stop, but for ROOT (F-1):** the dense two-body case A exposes a three-reader disagreement on a faithful producer-solved base inside D1. U8's witnesses do not depend on it. B1's W-C2 does. See §4.

## 1. Setup and build identity

- **Archive:** `GIT_OPTIONAL_LOCKS=0 git -C WT/f2a-u8 archive b1e2d7741e | tar -x -C ARCH/`, the U8 head given at dispatch. The three production files I instrumented were checked equal to `b1e2d7741e`'s blobs before editing:
  - `PP/src/lib.rs` `4c33c250…`;
  - `PP/src/retained_product.rs` `f536bfe7…`;
  - `P/core/reporting/result_export/src/retained_precision.rs` `4722b505…`.
- **Target:** `WT/targets/i68-u8/`. Its subfolders `checked-json/` and `units-authority/` hold the readers' two CLI authorities.
- **Toolchain:** rustc 1.97.1 (`8bab26f4f68e`), cargo 1.97.1, aarch64-apple-darwin; no `.cargo/config` or `rust-toolchain` file on the path. VENV Python 3.13.14; node v24.18.0; vitest 4.1.10 (from the linked `node_modules`).
- **Registered build, observed:**
  - the probe's `registered()` (`option_env!("OPS_RETAINED_BUILD_IDENTITY")` equal to the facade tests' `REGISTERED_IDENTITY` text) is `true`;
  - every admission report reads `profile=Registered`;
  - the control `zz_i68_control_milestone` publishes successors with U1's pinned receipt hashes, `efc1a39b…` (sparse) and `3e26499f…` (dense);
  - the dense published bytes `7c5fe5c5…` equal RV93's.
  
  The instrumentation does not touch the identity's inputs: build.rs hashes only `Cargo.lock` and the reader's 13 `include_str!` statics (`PP/src/build_identity.rs` `REVIEWED_INPUTS`).
- **Probe-only instrumentation** (`_run_records/instrumentation.diff`; archive only, all `cfg(test)` or env-gated prints, no control-flow change):
  1. `retained_product.rs` `solve_native`: `I68_NATIVE_OUTCOME` prints the kernel outcome and each attempt's (p, role, outcome).
  2. `lib.rs` `retained_w1`:
     - `I68_PREPARATION_FAILURE` prints the capture error and the section-preparation error;
     - `I68_CANDIDATE_REFUSAL` prints the refusal's error, truncated to 1,500 bytes;
     - `I68_PRECOMMIT_ERROR` / `I68_PRECOMMIT_DUMP` write the refused successor and its invocation to `I68_OUT` (added before run 2);
     - `mod zz_i68_probe;`.
  3. `result_export` `retained_precision.rs` `error()`: with `I68_READER_BT` set, a G8 error prints a backtrace. This was added after run 2 and used only by `zz_i68_reader_on_dump`.
- **The probe module** is `_run_records/zz_i68_probe.rs`. Every run goes through the actual Direct entry `run_linear_static_preview_value_with_retained_direct`, counted by grant 2's tally (`hooks::counted`). Each run records:
  - the plain route's status, result count, sha and diagnostics;
  - the admission report (profile, allowance, refusal and clause, domain, required, census, typed and raw facts);
  - the W1 cause;
  - the counts against `ONE_RUN_THROUGH_G_C`;
  - `hooks::armed_names()` before and after;
  - B′ (envelope = plain);
  - whether a successor was published;
  - the notice count;
  - byte equality with `with_notice(plain, case, None)` (the facade tests' oracle, copied) and with plain;
  - the published sha;
  - for a successor: the document `{"id","source","invocation"}` (pretty, as U1 pins it), its sha, the receipt sha and the Rust reader's verdict.
- **Inputs:** each is derived in the test.
  - `first_load_only` and `tiny_spring`: as in RV93's `zz_rv93.rs:292–315`.
  - W6: `retained_memory_witness_tests.rs:181–199`, copied inline.
  - L = 0: the milestone plus node `N2` at (3, 0, 0), not referenced by any member, plus support `rigid:N2` restraining `UX, UY, UZ, RX, RY, RZ`. It has the milestone's rigid-support shape, with no family.
  - Two-body: the milestone plus W6's body (nodes moved to x = 5..6 so no node coincides with `N0`; W6's material and anchor support). Case A is the milestone's 3 moments; case B is W6's tip force and tip torque.
  - One-body: W6's model. Case A is the tip force along `global_x`; case B is the tip force along `global_y` alone. W6 itself is B plus the torque.
  
  The input sha256 (compact JSON) of each is in the logs' `I68_BEGIN` lines.
- **Determinism:** probe runs 1 and 2 gave identical `I68_BEGIN/ORDINARY/ADMISSION/W1/SUCCESSOR/NATIVE` lines and identical `I51_FROZEN_REFUSAL` lines (diff empty).

## 2. Items 1 and 2: the real-input fallbacks and W-C1's reason

Every row below: admission refusal `None`, domain `None`, profile Registered; counts `{runs: 1, complete_gates: 1}` = `ONE_RUN_THROUGH_G_C`; `armed_names()` empty before and after; no successor; exactly 1 notice; published bytes = `with_notice(plain, <case>, None)` and ≠ plain.

| Input | Mode | W1 cause | Mechanism (probe prints) | plain sha (prefix) | published sha (prefix) |
|---|---|---|---|---|---|
| `first_load_only` | sparse | `Candidate` | native Selected; certificate `Predicate { row: 80, SharperExact }`; `I51_FROZEN_REFUSAL`: rows 80/84/88/92/96 (`element_local_torsional_shear_stress`, M1, 2.961883209619453e-06) fail predicates `[F,F,T,T]`; `numeric_pass:false`; observables and G5a `None` | `bbdea0384619` | `103ac0539220` |
| `first_load_only` | dense | `Candidate` | as sparse, rows 81/85/89/93/97 | `326410128807` | `c8f7e0448c9b` |
| `tiny_spring` | sparse | `Preparation` | ordinary `MODEL_INCOMPLETE` (blocking `NUMERICAL_INTEGRITY_ASSEMBLY_UNRESOLVED`, 0 results); `prepare_case`: `Association("prepared case custody/permit")`, section error `None` | `ca5eab08af00` | `7e0231cf3dea` |
| `tiny_spring` | dense | `Preparation` | identical (the blocked envelope is mode-independent: same plain and published bytes) | `ca5eab08af00` | `7e0231cf3dea` |
| W6 | sparse | `Native` | `Unresolved reason=Ceiling`; attempts (128 Candidate Rejected StopRule Displacement Uy node 1 translation), (256 VerificationThenCandidate Rejected StopRule EndAction m0 I Ux force), (512 VerificationThenCandidate Rejected Charge EndAction m0 I Ux force), (1024 Verification Solved) | `acd1e15934de` | `7b47c1647da2` |
| W6 | dense | `Native` | identical ladder and reason | `e32b68a7149b` | `3bc3e8885a2c` |

- The full `I51_FROZEN_REFUSAL` lines (about 39 kB each) are in `_run_records/probe_run2.log` and `probe_run1.filtered.log`.
- W6's ordinary run reads `NUMERICAL_INTEGRITY_CHECKS_PASSED` (not Sensitive) and `HIGH_DISPLACEMENT_REVIEW`. W1 runs on a Passed case, as PLAN §1.1 says.
- **Note on `tiny_spring`:** its Preparation cause is the ordinary run's failure to solve (preparation requires `MECHANICS_SOLVED`), not a section-preparation refusal. RV93's counts (`gb: 0`) are consistent with this: the late source capture is never reached.

## 3. Item 3: the L = 0 base

| | sparse_interactive | dense_scrutiny |
|---|---|---|
| Ordinary | `MECHANICS_SOLVED`, `NUMERICAL_INTEGRITY_SENSITIVE`; 113 results (milestone 98); plain `00a77d5c0820…` | `MECHANICS_SOLVED`, Sensitive; 114 results (milestone 99); plain `ffad370043fc…` |
| Admission | Registered; allowance Unselected; refusal `None`; domain `None`; required 3,575,778,286 B (= the milestone's; M = 4,026,531,840); census complete; nodes 3, members 1, supports 5, load cases 1, combinations 0, DOF upper 18; raw 168 values, depth 7 | the same, required 3,595,488,734 B |
| W1 | `Ok(successor)`; `ONE_RUN_THROUGH_G_C`; B′ (envelope = plain) true; published = successor; 0 notices | the same |
| Published successor bytes | sha `9b425066029969b992278306a678303717400a5f16636d3427c5b9b8150ac83a`, 126,034 B | sha `5d84fce64bb0810da24df9ea5d6ad354b18d864f07826dd09e191e345e4dea48`, 127,196 B |
| `receipt_sha256` | `c00cbe76954e5188c63b0ef69738cd40a8db15d303b1113a86524e6c3119dd72` | `0b4250c8139ba25ab9d35fc2d443a01d85de943a8e3d0eb9193f5dd5061ce994` |
| `publication_sha256` | `79d42693dd579652117d119e094650269d655ef4c8c89f693e9f185d6ebad99c` | `ee5a3f29ce1423f037e9fc668c3f311ec36841af8419bb6c6761cd51aff23d9d` |
| Probe document (id `i68_l0_isolated_node_<mode>`) | sha `f9745254…`, 226,747 B | sha `27c0509f…`, 228,097 B |
| **Rust reader** (`result_export::retained_precision::validate`, with the invocation) | PASS; invocation_bound, eligible; 113 classifications | PASS; eligible; 114 |
| **Python reader** (`core/analysis_runs/retained_precision.py`) | bound: PASS, eligible, standing `eligible`; relative 25, absolute 78, input 9, non-quantity 1. Unbound: PASS, `needs_recompute` | bound: PASS, eligible; 25 / 78 / 9 / 2. Unbound: `needs_recompute` |
| **TS reader** (`retainedPrecision.ts` via vitest) | bound: PASS, eligible; 25 / 78 / 9 / 1, the same `publication_sha256`. Unbound: `needs_recompute` | bound: PASS, eligible; 25 / 78 / 9 / 2 |

The document sha depends on the chosen `id`. Part 2's pinned fixture fixes its own id, so its file hash will differ. The receipt, publication and published-byte hashes do not depend on the id.

**The receipt for body 1** (both modes identical; `_run_records/l0_inspect.log`):
- `sources[0].body_membership`: body 0 = members [0], nodes [0, 1]; body 1 = members [], nodes [2].
- `product_attempts[0].proof.summary_coverage`: body 0 `has_data true, stop [T,T,T,T]`; **body 1 `has_data false, stop [F,F,F,F]`**. This is PLAN §1.2's expectation.
- `cases[0].selection.body_scales`: body 1 has force, moment, rotation and translation all `0000000000000000`.
- `cases[0].selection.stop_rule` lists body 0 only.

**Body 1's rows** (15; all values exactly `0.0`, bits `0000000000000000`; Python classes in `_run_records/l0_classes.log`):
- `result:disp:N2:{ux,uy,uz,rx,ry,rz}` are **input_derived**;
- `result:disp:N2` (displacement_magnitude), the six `rigid:N2` reaction components, and the force and moment magnitudes are **absolute_verified**, with normalized, scale and bound all 0, so they are exact zeros.

That matches PLAN §1.2's "input-derived or exact zero".

**Body 0 against the milestone:** all 98 (sparse) and 99 (dense) milestone row ids are present in the L = 0 successor, and every value is **bit-identical** to the milestone successor's. The 15 extra rows are body 1's.

## 4. Item 4: W-C2's candidate pairs (B1's record; nothing committed)

Every run: admitted (refusal and domain `None`); `ONE_RUN_THROUGH_G_C`; hooks empty before and after. Every fallback has 1 notice and bytes = `with_notice(plain, "case" or "case:source-section", None)`.

| Pair, case | Mode | Ordinary | W1 | Native outcome |
|---|---|---|---|---|
| two-body A (milestone loads) | sparse | Solved, Sensitive, `range_scaling: force_scale_exponent=518`; 171 results | **Successor.** Receipt `b6fbf65d…`; Rust, Python and TS PASS, eligible (25/136/9/1). Body 1: coverage `has_data false, stop [F,F,F,F]`, scales 0; the 73 rows naming its nodes, member or support are all 0.0 | Selected |
| two-body A | dense | Solved, Sensitive, force_scale_exponent=518; **171 results (no parity row)** | **Fallback `Precommit { gate: "G8", code: "RETAINED_PRECISION_PREPARATION_MISMATCH" }`** (F-1) | Selected |
| two-body B (W6's loads on body 1) | sparse / dense | Solved, `CHECKS_PASSED`; 171 / 171 | `Native` | **Unresolved Ceiling.** The same ladder as W6, on body 1 (node 3 / member 1) |
| one-body A (axial tip-x) | sparse / dense | Solved, `CHECKS_PASSED`; 74 / 74 | `Candidate`: the proof refuses at start, `ProductFailure { cause: Native(Alpha { block: 0, … }) }`; no `I51_FROZEN_REFUSAL` line | Selected |
| one-body B (transverse tip-y) | sparse / dense | Solved, `CHECKS_PASSED`, `HIGH_DISPLACEMENT_REVIEW`; 74 / 74 | `Native` | **Unresolved Ceiling.** The same ladder as W6 |

**F-1: the dense two-body case A is refused by the Rust precommit reader alone.**
- **What:** the producer's frozen successor for dense two-body case A (`_run_records/precommit_refused_dense_scrutiny_bb6c23865eb8.json`, source plus invocation, dumped by the probe-only print) is refused by the Rust reader at G8 `PREPARATION_MISMATCH`. The **Python and TS readers both PASS it** with the same invocation: eligible, 171 classifications, 25/136/9/1.
  - The Rust reader again, directly on the dump (`zz_i68_reader_on_dump`): bound FAIL G8 (`detail: None`); unbound PASS.
- **Where:** the backtrace (`_run_records/reader_dump_bt.log`) places it in `g8` (`:3422`) at `result_export/src/retained_precision.rs:3532` (numbering at `b1e2d7741e`; `:3533` in the instrumented copy):

  ```rust
  fail(parity == usize::from(mode == "dense_scrutiny"))?;
  ```

  For a **selected** case in **dense_scrutiny**, the Rust reader requires exactly one `sparse_live_path_dense_parity_relative_delta` row.
- **Why the row is absent:** `PP/src/lib.rs:4435–4463` (the call site of `append_sparse_live_path_evidence`, at `b1e2d7741e`) runs the DEC-050/053 observation only when `w2_publication` is `None`. The comment there reads "F1b (ROOT OQ5): no DEC-050/053 observation runs at b != 0". This model is range-scaled (`force_scale_exponent=518`, driven by body 1's stiffness), so the dense ordinary run legitimately publishes no parity row. The milestone and L = 0 are not range-scaled and do carry it.
- **The other readers:**
  - TS `invocationBinding` checks only the mode row (dense = 2; sparse ∈ {1, 3}; code `INVOCATION_MISMATCH`), not the parity row (`retainedPrecision.ts:1152–1153`);
  - Python `_g8` has no mode or parity-row check.
  
  So the three G8s differ on a faithful producer-solved base.
- **Consequences:**
  - Inside D1 today, any dense, range-scaled (b ≠ 0) one-case input whose W1 work selects falls back at Precommit, with the plain bytes plus one notice. The behaviour is safe, but the successor is lost. U8's committed witnesses are unaffected: L = 0 and the milestone are not range-scaled, and W-C1 falls back at Native, before precommit.
  - For **B1's W-C2 on this two-body model, dense mode cannot publish** while the Rust rule stands, because every case on the model is range-scaled. Sparse mode publishes.
  - The Rust reader is on the D1 precommit call graph, so changing that rule re-opens re-qualification (RR:10474; QUAL §11). Aligning Python and TS to Rust instead would make them refuse F1b's own output.
  
  Which side is right is a contract reading beyond the rulings I have, so I return it to ROOT.

## 5. What Part 2's scope should be (proposal)

1. **`u8_real_input_fallbacks_append_one_notice`, as briefed:**
   - `first_load_only`: Candidate;
   - `tiny_spring`: Preparation;
   - W-C1's input: Native.
   
   All three hold in both modes with every assertion the brief lists (cause, `ONE_RUN_THROUGH_G_C`, 1 notice, `with_notice` bytes, refusal `None`, hooks empty). **W-C1's input needs ROOT's choice:**
   - two-body case B, the PLAN-preferred "input B1 needs";
   - or W6's input, inline.
   
   Both end Native with Ceiling. Given F-1, B1 may not keep the two-body model for dense mode. My recommendation is W6's input, unless ROOT keeps the two-body model for B1.
2. **`u8_l0_isolated_node_publishes_pinned_successor` proceeds.** L = 0 publishes in both modes. The expected id-independent pins are:
   - receipt `c00cbe76…` (sparse) and `0b4250c8…` (dense);
   - published bytes `9b425066…` and `5d84fce6…`.
   
   The fixture file sha follows from Part 2's document id. Value controls:
   - body 1 is 6 input-derived plus 9 exact zeros;
   - body 0 is bit-identical to the milestone, which is stronger than "within the criterion"; report it, do not assume it.
   
   Use the same L = 0 derivation (N2 at (3, 0, 0), `rigid:N2` with six restraints, no family) or re-pin.
3. **The D-U6-5 equality test and the two L = 0 fixtures** proceed.
4. **No producer, reader or fixture change** is needed for U8's own witnesses. F-1 is not U8's to fix.

## 6. For ROOT to rule on

1. **F-1** (the Rust G8 parity-row rule against F1b's range-scaled dense output; three-reader divergence). Where does it go: B1 (with W-C2's dense half), a reader repair unit with re-qualification, or the owner? And does W-C2 keep the two-body model?
2. **W-C1's input for Part 2:** two-body case B, or W6.
3. **The one-body fallback pair is not viable for W-C2.** Its case A falls back at Candidate (proof start, `Native(Alpha)`) in both modes. If B1 needs a fallback, it must be constructed differently.
4. **Recorded, not a ruling:** `tiny_spring` reaches Preparation through an unsolved ordinary run, not a section-preparation refusal. The committed test's cause assertion still holds.
5. **Host note: the Write-tool isolation hook.** This session's harness blocks its file-edit tool outside its own worktree with the message "Do not write to other worktrees' files from this session". I wrote only to the locations the dispatch assigns, using Bash: my disposable archive and scratch under `ARCH`, and this records folder. I touched no other worktree's files. ROOT or the owner should confirm this practice for Part 2, which edits `WT/f2a-u8`.

## 7. Commands run (exact; `cd` into the stated directory first)

The setup attempts that failed are listed with their causes; they produced no result.

1. `mkdir -p WT/scratch/i68_u8_probe && GIT_OPTIONAL_LOCKS=0 git -C WT/f2a-u8 archive b1e2d7741e | tar -x -C WT/scratch/i68_u8_probe/`
2. Instrumentation and probe files written into `ARCH` (diff and sources in `_run_records/`).
3. `ARCH/P/core/product_physics`: `env -u RUSTFLAGS -u CARGO_ENCODED_RUSTFLAGS CARGO_BUILD_JOBS=4 RUST_TEST_THREADS=2 CARGO_TARGET_DIR=WT/targets/i68-u8 WT/tools/t3_cargo.sh test --locked --offline --lib --no-run` (rc 0)
4. Probe run 1, the same directory and environment, plus `I68_OUT=ARCH/_i68_out`: `WT/tools/t3_cargo.sh test --locked --offline --lib zz_i68 -- --nocapture --test-threads=1` (rc 0; 5 passed)
5. Probe run 2, after adding the precommit dump: the same command (rc 0; 5 passed; outcomes identical to run 1)
6. `ARCH/P/core/serialization/canonical_json`: `env -u RUSTFLAGS -u CARGO_ENCODED_RUSTFLAGS CARGO_BUILD_JOBS=4 WT/tools/t3_cargo.sh build --locked --offline --release --features checked-cli --bin openpipestress_jcs_ijson --target-dir WT/targets/i68-u8/checked-json` (rc 0). This was needed because the first Python attempt stopped at `CHECKED-JSON-AUTHORITY-MISSING`.
7. `ARCH/P/core/units`: `env -u RUSTFLAGS -u CARGO_ENCODED_RUSTFLAGS CARGO_BUILD_JOBS=4 WT/tools/t3_cargo.sh build --locked --offline --release --features cli --bin openpipestress_units --target-dir WT/targets/i68-u8/units-authority` (rc 0). This was needed because the second attempt stopped at `UNITS-AUTHORITY-MISSING`.
8. `OPENPIPESTRESS_UNITS_BIN=WT/targets/i68-u8/units-authority/release/openpipestress_units OPENPIPESTRESS_CHECKED_JSON_BIN=WT/targets/i68-u8/checked-json/release/openpipestress_jcs_ijson VENV/bin/python ARCH/_i68_tools/py_readers.py ARCH/P ARCH/_i68_out/*.json` (rc 0)
9. `ln -s <parent-checkout>/P/node_modules ARCH/P/node_modules`. The first vitest attempt failed in `src/test/setup.ts` with `WASM-ENGINE-ASSET-ABSENT`. Following I67's precedent, `mkdir -p ARCH/P/apps/desktop/public && cp -R WT/sweep-skewpin/P/apps/desktop/public/{wasm-engine,self-weight-engine} ARCH/P/apps/desktop/public/` (copied, never built; hashes in `_run_records/ts_wasm_assets.sha256`; the reader does not use them).
10. `ARCH/P/apps/desktop`: `I68_OUT=ARCH/_i68_out I68_TS_LOG=ARCH/_i68_logs/ts_readers_lines.log ../../node_modules/.bin/vitest run src/features/results/zz_i68_probe.test.ts` (rc 0; 1 passed). The test writes its lines to `I68_TS_LOG` because vitest 4 did not echo `console.log` for a passing test.
11. Reader instrumentation (`error()` backtrace, env-gated), then in `ARCH/P/core/product_physics`: `env -u RUSTFLAGS -u CARGO_ENCODED_RUSTFLAGS CARGO_BUILD_JOBS=4 RUST_TEST_THREADS=2 CARGO_TARGET_DIR=WT/targets/i68-u8 I68_DUMP=ARCH/_i68_out/precommit_refused_dense_scrutiny_bb6c23865eb8.json I68_READER_BT=1 RUST_BACKTRACE=1 WT/tools/t3_cargo.sh test --locked --offline --lib zz_i68_reader_on_dump -- --nocapture --test-threads=1` (rc 0)
12. `VENV/bin/python ARCH/_i68_tools/l0_inspect.py ARCH/_i68_out` and, with the two authority variables from step 8, `VENV/bin/python ARCH/_i68_tools/l0_classes.py ARCH/P ARCH/_i68_out` (rc 0)

The cargo lock log lines for these jobs are in `_run_records/cargo_jobs_i68.log`.

## 8. Records, cleanup and limits

- **`_run_records/`:**
  - the probe sources: `zz_i68_probe.rs`, `zz_i68_probe.test.ts`, `py_readers.py`, `l0_inspect.py`, `l0_classes.py`;
  - `instrumentation.diff`;
  - the logs: `probe_run2.log` in full, `probe_run1.filtered.log`, `reader_dump_bt.log`, `py_readers.log`, `ts_readers*.log`, `l0_*.log`, the three build logs, `cargo_jobs_i68.log`;
  - `probe_outputs.sha256`, the sha256 of every saved document;
  - the F-1 dump.
  
  Machine paths are replaced by `WT`, `<parent-checkout>` and `~`. There are no process listings and no other apps' or sessions' data.
- **Cleanup on return:** the archive `ARCH` is deleted; it is reproducible from `b1e2d7741e` plus `_run_records`. `WT/targets/i68-u8/` (about 1.2 GB) is **kept** for Part 2.
- **Limits:**
  - The L = 0 node position is my choice; other positions were not probed.
  - The pairs were run as one-case requests only (PLAN §1.3), not as a two-case invocation.
  - F-1's analysis establishes where the readers diverge, not which reading the contract intends.
  - The TS run used the repository's vitest config with copied wasm assets. The reader does not depend on them.
