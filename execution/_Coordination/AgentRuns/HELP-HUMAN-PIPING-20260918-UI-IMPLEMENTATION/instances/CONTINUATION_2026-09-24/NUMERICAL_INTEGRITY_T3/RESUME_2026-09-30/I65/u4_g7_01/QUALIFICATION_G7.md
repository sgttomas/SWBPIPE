# I65 U4 G7: delta re-qualification on the integrated basis (Pass A), an addendum to G6's QUALIFICATION.md

**Agent:** I65, TASK (Type 2) under ROOT. No descendants.

**Brief:** `BRIEFS/I65_U4_G7_DELTA_REQUALIFICATION.md` (NUM `d25d5393dc`, sha256 `92aebe16…`), with the rulings through RR "Registration applied; M = 4,026,531,840 B selected under D-7; U3 grant 2 dispatched".

**The basis.**
- The integrated tree is `ba1faa1c858ce3630a22767677310b1902a14b83`, the clean merge of NUM `f172f86abe` (U6) and memory `0c7827b6ad` (registered).
- ROOT extracted it read-only to WT/scratch/i65_u4_g7_01/basis/. My build copy, `work/`, equals the tree blob for blob: 2,949 of 2,949 files (`tree_check.py`).
- The NUM comparison tree is a `git archive` of `f172f86abe`, equal to it blob for blob (2,943 of 2,943).

**G7 changes no source.** Every source file was read-only. Only Pass A's analysis scripts and rule files changed, and they are records (§4).

**Revised after review** (RV87 and RV89 on G7 Pass A, both PASS): the sentences marked "(review)" below are added. The repaired Pass B script and its run on the final basis are in `R/I65/u4_g7_02/`. That script supersedes §6's.

## Verdict

**The registered entry stays byte-identical.** No `registration.diff` is needed.
- **Registered.** The integrated registered build compiles exactly the registered identity, the 14 reviewed-input hashes and the four reader layouts. `build_status()` is `Ok(0)`, and all 42 law tests pass, including the registered ones (§2).
- **The in-build maximum is unchanged:**

| Mode | E_mov,max + R | Fraction of M | Under 0.9 M |
|---|---|---|---|
| Sparse | 3,575,778,286 B | 0.8881 M | 48,100,370 B |
| Dense | 3,595,488,734 B | 0.8929 M | 28,389,922 B |

- **The U6 delta has one allocating change on the D1 path:** F5's exact `diagnostic_refs` list in the precommit reader. It is priced in T17's V4 stage at +205,960 B, which is 1,131,825,961 B below T17's largest stage (V2_hash). So T17, every phase and the pinned record are unchanged (§3).
- **TEXT is complete.** It matches G6 row for row: all 2,807 G6 rows equal in multiplicity, bytes and requested bytes, in all four runs.
  - The enforced identifier audit passes: no `id-unaudited`, `stale-key` or `stale-audit-entry`.
  - §11 is discharged by the re-run sweep (§4).
- **Witnesses and suites:** all nine witnesses pass, and the challenge passes.
  - PP: 699 passed, 1 failed (t13), 10 ignored. runner/headless: 85 passed, 2 failed.
  - **Both are outcome-identical to the registered baseline on `0c7827b6ad`.** Every difference from NUM `f172f86abe` unregistered is a memory-branch test that NUM does not have yet (§5).

**A finding about the TEXT tool, with its fix** (§4.1):
- The integrated tree's new lexical cycle (`validate` ↔ `for_source`) made the G6 tool **silently** zero the multiplicity of everything reached only through `for_source`. A first run was "complete" (save one argument class) with TAV 112,852,224 B low.
- The G7 tool forms the condensation without `edge_zero` edges, and makes any multi-member cycle among the text sites' ancestors fail the run (`scc`).
- G6 had no such cycle, and the G7 tool reproduces G6's four TEXT outputs exactly.

## 1. The delta inventory (`_run_records/pass_a/delta_inventory.json`, `delta_inventory.py`)

**Scope.** Every file that differs between `0c7827b6ad` and `ba1faa1c`, outside `execution/`. That is 40 files: 3 production Rust files with 24 hunks, and 37 others.
- **Reachability** is the TEXT chain's lexical call graph from `run_linear_static_preview_value_with_retained_direct` ("reached").
- It is also given **after the `edge_zero` rules** ("live"). That is the established G4 method for branches a D1 value cannot take.

**The D1 path** is `admit` → the ordinary run → W1 → the W4 precommit reader `retained_precision::validate` (PP lib.rs:3161) → publication.
- PP calls nothing else in `result_export`.
- `validate`'s G7 calls `semantic_contract::for_source` on `project(source)`. `project` sets `producer.semantic_contract_id` to the literal `…/preview-physics-1` id (retained_precision.rs:4252–4253).
- So **`is_retained` is false on every D1 call** of `for_source` and `for_source_metadata`. This is the same fact G4's `edge_zero` rules 53–58 already use for `for_source`'s other arms.
- **(review, RV89 N-2) The warrant is load-bearing.** If `is_retained` were ever true on D1, `for_source` would run a second full `validate(source, None)` inside the first, while the outer V5 working set is live. Dense W4 would then reach at least 3,749,527,510 B with R, above 0.9 M. The warrant lives in `project`'s literal assignment (retained_precision.rs:4252–4253) and `validate`'s one call at :4305. Pass B makes those lines rule keys and checks their text, and stops (exit 4) on any edit (`u4_g7_02/_run_records/premise_pins.json`).

| # | Hunk (new lines at `ba1faa1c`) | Item | Reached / live | Class |
|---|---|---|---|---|
| 1–13 | `derivative.rs` 4, 6–7, 15–61, 115, 153, 170–175, 243–244, 298–300, 318, 387, 405–413, 560–567, 697–711 | imports; `si_unit`, `not_covered_message`, `class_disclosure` and the new consts; `derive_document`; `validate_document` | no / no | **Unreachable.** The derivative export has no D1 caller. Only `digest`/`guard_json` are reached, unchanged |
| 14 | `retained_precision.rs` 4131 | the D6a comment in `g5_ordinary` | yes / yes | no code |
| 15 | `retained_precision.rs` 4136–4145 | **F5:** `exact: Vec<&Value>` and the list comparison in `g5_ordinary` | yes / yes | **New, priced** in T17 V4 (§3) |
| 16 | `retained_precision.rs` 4345–4402 | `#[cfg(test)] mod u6e_reader_round_tests` (and the blank and doc lines before it) | — | test, excluded |
| 17 | `semantic_contract.rs` 108–193 | consts (`PREVIEW_PHYSICS_RETAINED_*`, `RETAINED_PRECISION_DOWNGRADE_FORBIDDEN`, `RULE_QUANTITY_*`) and new fns | | see below |
| | | `is_retained`, `forbid_retained_member`, `forbid_retained_rows` | yes / yes | **Priced: allocation-free on D1.** Value indexing and `Value == &str` comparisons. The `Err(… .into())` branches are dead on the projection, which has no `retained_precision` member and no `recovery_method` (`project(source, true)`) |
| | | `verify_preview_physics_retained_table`, `preview_physics_retained_contract` (**the new `OnceLock` + `include_bytes!`**), `retained_error` | yes / **no** | **Unreachable on D1.** Called only from the `is_retained` branches (new `edge_zero` rules, §4.1) |
| | | `retained_row_classes` | no / no | unreachable (derivative only) |
| 18 | `semantic_contract.rs` 270–276 | `for_source_metadata`: the `is_retained` branch and `forbid_retained_member(source)?` | yes / yes | branch dead (`edge_zero`); the check is allocation-free |
| 19–20 | `semantic_contract.rs` 438–444, 446 | `for_source`: the `is_retained` branch and `forbid_retained_rows(source)?` | yes / yes | the same |
| 21 | `semantic_contract.rs` 511–513 | a `FRESH_IDENTITIES` entry | — | const data, read only by `is_fresh_identity`, which is not reached |
| 22–24 | `semantic_contract.rs` 538–540, 549–715, 736–739 | `rule_binding_refusal`, the retained standing, binding and classification fns, `numerical_use_standing_with_context` | no / no | unreachable |

**The other 37 files are not compiled into PP's D1 path:**
- the desktop TypeScript (17);
- the Python `analysis_runs` (3);
- the Python tests (7);
- `result_export/tests/*.rs` (3);
- the fixtures (4) and schemas (3).

**None of the 14 reviewed inputs changes.** The only changed data file that any production Rust embeds is `schemas/results.v0.3.schema.yaml`. It is embedded by `physics_evidence::validate_transport_metadata` (unchanged code), which is reached only through `load_reference.rs` paths that G4's `edge_zero` rules already cut: **not live**. The other six changed data files are embedded only by `#[cfg(test)]` code or tests, or not at all.

## 2. Identity and statics (`pass_a/identity_check.out.json`, `profile_record.test.txt`, `statics_a.json`)

**The integrated registered build** (`cargo test --lib retained_memory`, test build, rustc 1.97.1) compiles each of the following equal to `REGISTERED_PROFILES[0]`:
- `OPS_RETAINED_BUILD_IDENTITY`;
- the reviewed-input text: 14 inputs, none `unavailable`;
- the reader layouts: Validation 56/8, ValidationError 64/8, RowClassification 96/8, AccuracyClass 16/8.

The `LAYOUT_WITNESSES` compile, and the threshold is 4,026,531,840. These tests all pass:
- `the_registered_profile_is_the_only_permit_source`, which asserts `build_status() == Ok(0)`;
- `admit_grants_a_permit_for_the_milestone_in_the_registered_build`;
- `registered_g_c_declines_only_unattempted_solves`;
- `reviewed_inputs_bind_the_lock_and_the_reader_statics`;
- `profile_in_build_record`;
- `challenge_bounds_are_the_profile`.

**The build is Registered on the merged basis. No stop.**

**Statics** (`statics_list.py`: every production `include_str!`/`include_bytes!`/`OnceLock`/`LazyLock`/`thread_local!` in the D1 crates, against the same list at `0c7827b6ad`):
- **Exactly one is added:** `semantic_contract::preview_physics_retained_contract`'s `static CONTRACT: OnceLock<Value>`, initialized from `include_bytes!(…/semantic_contract_v0_3_preview_physics_retained_1.json)`.
- **It is not reachable from `validate` on D1.** Its only callers are the `is_retained` branches (§1), cut by `edge_zero`. So it is never initialized there, and the in-build record holds.
- For disclosure: if it were ever initialized, its parsed tree bound (G4's static rule) is 286,836 B, under 1 % of the 28.4 MB margin.
- The reader's own static for the same table, already among T17.0's 13 statics, is unchanged.

## 3. The delta, priced (`pass_a/price_delta.out.json`, `price_delta.py`; `chain/g4_caps.py` `F5_EXACT`)

**The counting rule for F5** (`g5_ordinary`, retained_precision.rs:4136–4145, once per receipt case):
- `exact` collects a `filter` over the envelope diagnostics, so its capacity follows the push law: ≤ `pushcap(D_env)` = 16,384 slots of `s(&Value)`.
- `list(&o["diagnostic_refs"]).iter().collect::<Vec<_>>()` is exact-size, with ≤ D_env slots.
  - **(review, RV89 N-1) The warrant is the producer, not uniqueness.** The bound D_env rests on the producer's construction: `retained_wire.rs:1426–1429` builds the refs with F5's own filter, which excludes `RETAINED_PRECISION_*`. So on a passing check the refs equal `exact`, which has at most D_env entries.
  - Uniqueness and resolution alone would allow D_env + 1, because the successor adds `RETAINED_PRECISION_SELECTED`. That is at most 8 B more.
- Both are live together for the comparison and drop at the end of the case's iteration. So they are counted in full in T17's V4 stage (G3–G6 working sets): **+s(&Value) × (16,384 + 9,361) = +205,960 B** at the in-build stride of 8.
- The moving extra of `exact`'s last growth is 8 × 8,192 = 65,536 B, below W4's moving maximum (189,303,281 B).

**In-build** (atoms from the integrated build's own record):

| T17 stage | Bytes |
|---|---|
| V1 | 321,428,243 |
| V2_clone | 172,256,082 |
| **V2_hash** | **1,148,998,726** |
| V3 | 32,598,012 |
| V4 | 16,966,805, and **17,172,765 with F5** |
| V5 | 180,109,775 |
| V6 | 200,409,127 |

- **T17 = max(stages) + output is unchanged.** V4 with F5 stays 1,131,825,961 B under V2_hash.
- So W4 and every other phase are unchanged in both modes, and so are E_mov,max and the pinned record.
- **No other phase is touched.** The dispatch checks are allocation-free, and the rest of the delta is unreachable (§1). TEXT is unchanged (§4).

**The profile tree with F5** differs from G6's only in `T17_V4`'s `s(&Value)` coefficient, 57,880 → 83,625. Phases and checks are identical.
- The committed generated block in `retained_memory.rs` transcribes G6's tree. Without F5, the integrated chain regenerates it byte for byte.
- **Proposal, not needed for the bound** (`pass_a/proposal/t17_v4_f5.diff`, one line): regenerating with F5 changes only that `Form` line. No maximum, record, identity or test moves.
- It is a source change, so G7 does not make it. ROOT may take it with Pass B or later.

## 4. TEXT and the identifier audit (`pass_a/text_g7/`, `pass_a/controls/`)

**The run.** The TEXT chain runs on the integrated tree with G6's final rules, line-mapped from the text basis `1e323058f3` by `g7_linemap.py`.
- **The line map** covers 7 changed production files: 27 keys moved, and none fell inside a changed hunk.
- **The template inventory** is regenerated: 93 files, now including `retained_resource.rs`, and 1,133 rows. The 3 new rows are the 2 `derivative.rs` messages and `semantic_contract.rs`'s `{:x}`.

**Rules added:**
- 6 `edge_zero` rules for the dead `is_retained` branches of `for_source` and `for_source_metadata`, with G4's reason (§1);
- 1 argument rule: `si` = `si_unit`'s `&'static str`, ≤ 3 B. It is unreachable, but every placeholder must be classed.

**Result** (complete; D 14,734, D_env 9,361):

| | G6 | Integrated |
|---|---|---|
| TAV | 2,150,800,830 | 2,150,800,830 |
| TAV_W | 1,570,041,862 | 1,570,041,862 |
| TAV_X | 1,440,401,002 | 1,440,401,002 |
| Largest site | 2,599,962 | 2,599,962 |

- **Row for row** (`text_row_diff.py`): 2,807 of 2,807 G6 rows have equal multiplicity, bytes and requested bytes in all four runs. The 7 new rows all have multiplicity 0. The reached functions go from 2,752 to 2,770: the dead branches' callees.
- **Every other output is byte-identical to G6's:** the composite, ordinary, producer, T07, T08 and T25 outputs, and the summary. The only exceptions are the profile tree and `g4_caps`'s output, which add F5 (§3).
- **The identifier audit is enforced in the run.** It finds no `id-unaudited`, `stale-key` or `stale-audit-entry`.
- **Controls** (`audit_controls_g7.out.json`): the unmodified copy is complete, and 10 single changes each fail with exactly their own finding:
  - G6's eight;
  - **c9**: one cut edge restored → `scc`, TAV 2,037,948,606;
  - **c10**: the `si` rule removed → unclassified.

### 4.1 The cycle finding and the tool change

- **The problem.** `text_budget.py` propagates multiplicity only along edges between condensation components. A multi-member cycle leaves its non-entry members at 0.
- **Why it now matters.** The new lexical edges `for_source → validate` and `for_source_metadata → validate_transport_metadata → for_source_metadata` close two cycles (`controls/scc_probe_g7.out.json`). Everything reached only through `for_source` then got multiplicity 0.
- **What the G6 tool did.** The run reported only the `si` argument: TAV −112,852,224 B (`preview_physics_evidence.rs` −110,776,988; `semantic_contract.rs` −2,073,808; …), with no other signal.
- **G7's tool changes:**
  - (a) the condensation is formed without `edge_zero` edges, which carry no call;
  - (b) a multi-member cycle among the text sites' ancestors makes the run incomplete (`scc`);
  - (c) it outputs the self-recursive ancestors (11, as at G6: the tree walkers, all single-function recursions, whose text G4 and RV87 priced);
  - (d) it dumps the non-candidates (§4.2).
- **No cycle at G6.** G6's reached graph has no multi-member cycle (`scc_probe_g6.out.json`).
- **(review, RV87 N-2) The check is only as complete as the call graph.** A cycle closed through dispatch the lexical graph does not see (a `dyn` call, an `Fn` value, a fn pointer) cannot be detected. The U6 delta adds none (RV87 G7).
- **(review, RV87 N-1) Self-recursion is now a stop too.** Pass B's chain makes a self-recursive text ancestor outside the reviewed list of 11 (loop_bounds `recursion_reviewed`, each with the reason its text is priced) fail the run (`self-recursion`, control c11).
- **Regression control** (`controls/text_regression.txt`): the G7 tool on G6's final inputs reproduces all four of G6's TEXT outputs exactly.

### 4.2 The §11 discharge: the by-type sweep, re-run

**Chosen: re-run the sweep, mechanically.**
- The G7 tool dumps every positive-multiplicity priced expression that the identifier predicate does not treat as identifier-bearing (`TB_NONCAND_OUT`, RV87's own instrumentation).
- On the integrated tree that is **410 rows, and they are exactly the 410 RV87 read at G6.** They match on (file, fn, expression, kind/spec, class, multiplicity), and no new non-candidate exists (`noncand_compare.out.json`). So RV87's reading of each one carries unchanged.
- That follows from §4: the delta adds no positive-multiplicity text expression.

**Why not the explicit-row rule now.**
- It would invert the default for the k5 shape by giving each bare-local non-candidate its own table row. That is 240 of the 410, each to be classified by its binding.
- It is the durable closure, and remains the recommendation for a grant of its own.
- At this basis, the sweep closes the residual exactly at no reading cost. Pass B repeats the comparison mechanically.
- **(review, RV87 SF-1)** Pass A's script only reported that comparison. The repaired Pass B (`u4_g7_02`) exits non-zero (6) when the set differs from the reviewed 410.

## 5. Witnesses and behaviour (`pass_a/witnesses_challenge.txt`, `pass_a/outcomes/`)

**The integrated registered build** (test build, one process per witness, R/k = 4 MiB unless stated). Every witness passes, with the same outcomes as G6:

| Witness | Outcome |
|---|---|
| W1 | Successor ×2 |
| W2 | Fallback(Preparation) |
| W2-deep | Successor at 4 and 1 MiB, ×2 |
| W2b | Fallback(Candidate) ×2 |
| W3 | ExactSelected |
| W4 | Fallback(Preparation) |
| W6 | Fallback(Native) ×2 |
| W7 | Serializer, Staging, Precommit G8 and G1 |
| W1 at 1 MiB | Successor ×2 |

**The challenge** passes: the milestone is permitted, peaking at 3,541,898 B (sparse) and 2,252,863 B (dense), against 3,508,669,422 and 3,528,379,870. The cap-maximal input is not permitted (13.2 MB). These equal the registered G6 copy's peaks.

**Suites** (`--locked --offline --no-fail-fast`):

| Suite | Integrated, registered | `0c7827b6ad`, registered (baseline) | NUM `f172f86abe`, unregistered |
|---|---|---|---|
| PP, all targets | 699 passed, 1 failed (t13), 10 ignored | **identical outcome list** | 660 passed, 1 failed (t13), 1 ignored |
| runner/headless | 85 passed, 2 failed (`load_reference` ×2) | **identical** | **identical** |

**Every difference from NUM unregistered** is one of 48 tests that NUM does not have yet, all from the memory branch (`num_vs_a_pp.diff`). No test present in NUM changes outcome:
- 46 `retained_memory::` law and witness tests (9 of them ignored);
- `tests/retained_memory_challenge.rs`;
- `retained_facade_tests::u3_unfired_hooks_come_back_across_the_hop`, from U3 grant 1d (`8abb5274a9`).

## 6. Pass B (`_run_records/g7_pass.sh`; superseded by `R/I65/u4_g7_02/_run_records/g7_pass.sh`, review)

**Usage:** `I65_T=<WT> g7_pass.sh <basis dir> <basis rev> <tag>`. It repeats items 2–5 mechanically on a fresh copy:
1. checks the tree blob for blob;
2. runs the law tests (identity, inputs, layouts), and **stops (exit 3) if the build is not Registered**;
3. diffs the statics against Pass A;
4. line-maps Pass A's rules from `ba1faa1c` to the new revision, and **stops (exit 4) if a rule's line was edited**;
5. regenerates the inventory;
6. runs the TEXT chain and compares it with Pass A: row for row, each output's bytes, and the profile tree;
7. checks the committed profile block against the transcription;
8. prices the delta;
9. runs the §11 sweep comparison and the 11 controls;
10. runs the witnesses, the challenge, PP and runner/headless, and diffs their outcomes against Pass A.

**Self-test** on Pass A's own basis (`pass_selftest.txt`):
- Registered;
- no statics added;
- 0 rows changed;
- every output identical;
- the profile block differs from the transcription only by the proposed T17_V4 line;
- the sweep carried;
- controls and suites as recorded.

## 7. Non-claims (unchanged from G6 §6, D-7)

- Not RSS, allocator overhead or fragmentation.
- No concurrency.
- No supported-machine claim.
- No stack claim beyond the measured witnesses.
- The release build was not re-run: its identity is Stale and not registered.
