# I66 return: U7 slice F, part 1 (the Python and Rust switch, the shared fixtures, PP's pin)

I66 is a TASK (Type 2) under ROOT. This part follows `BRIEFS/U7_SLICE_F_SWITCH.md` (`8f2674570b`), ROOT's dispatch message, and these sources:
- PLAN `R/I61/u7_scoping_01/PLAN.md`;
- the rulings "…U7 planned and ruled" (D-U7-1 to D-U7-6), "U7 slice A returned…" and "The memory branch merged into NUM; U7 slices T and P committed" (D-U7-4's v4 format);
- slice A's records.

I66 did not delegate.

**Verdict: part 1 is done, with no stop.**
- The Python and Rust flags are on, and the Rust edit is line-neutral.
- 07i is applied, every Python, Rust and PP pin is moved to its oracle value, and the stale texts are rewritten.
- D-U7-6 is in the scope and is asserted. D-U7-4 is added, and the case file is at **v4**.
- **Every control passes:**
  - the oracle diff is exact in both languages;
  - no gate, classification, derivative, binding, transport, AnalysisRun, packager or D-U6-9 outcome changes;
  - PP's registered and Stale sweeps are byte-identical to `u3_grant2_02`;
  - every suite matches the base apart from the renamed and new tests;
  - every mutant is killed, except one equivalent mutant per language, explained in §6.
- **TS is untouched, so it is I67's part 2.** The TS failures this leaves are listed in §7.

## Basis, host and fence

- **Worktree:** `WT/f2a-u7`, branch `codex/piping-f2a-u7-20261004`, at `12a849a7bd`, uncommitted. No Git writes; Git reads used `GIT_OPTIONAL_LOCKS=0`. I67's untracked `P/node_modules` link is untouched.
- **When:** 2026-10-04, about 17:15Z to 18:15Z. The memory guard (PID 5387) was checked before every run.
- **Cargo:** the default toolchain, `--locked --offline`, `CARGO_BUILD_JOBS=4`, `RUST_TEST_THREADS=2`, one job at a time, in my own target dirs under `WT/targets/i66-u7f/`, never WT/f2a-memory's.
  - **Stale** is `RUSTFLAGS=--cfg=i61_u3g2_stale`, as I61 builds it.
  - **Not run:** native, solver-at-scale or DEC-025 jobs, and no install.
- **Python:** `REPO_ROOT/P/.venv` with the I52 CLIs. `TMPDIR` was `WT/scratch/i66_u7f/tmp`.
- **Vitest** was run only for information (§7), through the existing node_modules link and the WASM copies already in `apps/desktop/public`.
- **Lanes,** under `WT/scratch/i66_u7f/`:
  - `base` is a `git archive` of `12a849a7bd`;
  - `cand` is the worktree plus the sweep and dump harnesses;
  - `mut` (Python) and `mut_rs` (Rust) are the mutant lanes.
- **The fence: 12 files.** All are named by the brief: the readers, their stale text, the pins, the shared fixtures and PP's pin. One is a PP comment (§4).

| File | sha256 | Change |
|---|---|---|
| P/core/analysis_runs/retained_precision.py | `cd98ff8115a57696fdc0f048f88d0a0d5a5492343b8630ecb2535c4e3a3e7411` | `_IMPLEMENTATION_COMPLETE = True` (:30). Comment :27–29 and the docstrings :1591–1593 and :1598 use slice A's text, plus the producer-origin sentence |
| P/core/analysis_runs/compatibility.py | `76b0dc2ef8c6b88881b34fe5f8033919e2650f226bdf36eb134d8637f4907e2e` | The "while held" sentence is dropped from `_retained_standing_from`'s docstring |
| P/core/reporting/result_export/src/retained_precision.rs | `efaf16d16a5a33d6c90f0c321f1b41308900ad2bd8177972e8377a68e2786f96` | :4267–4269: two comment lines and `const IMPLEMENTATION_COMPLETE: bool = true;`. **Line-neutral** (§3) |
| P/core/reporting/result_export/src/semantic_contract.rs | `dcd9681826ad23e5e3f97182e0abef407d60713ec34cd46f9f79df864d860075` | :581–582: the doc lines. Line-neutral |
| P/core/product_physics/src/lib.rs | `bd74e080934be520a92fe1e85107af5448686792fe247992f3db8bfedb20ebe3` | :3156: one comment line. Line-neutral (§4) |
| P/core/product_physics/src/retained_wire_tests.rs | `ef85791efc2257b88aa298967058f2dfcefd79d6ff5b976396848742cdc2aa78` | The :122 pin and the :109 doc |
| P/core/reporting/result_export/tests/retained_precision_contract.rs | `737693847a8416f7492d93fc0ee05917727c69ac49976e3fc60149031369ad7c` | Pins (§2) |
| P/core/reporting/result_export/tests/retained_precision_carriers.rs | `ac6b152519a5670b6d3b9e302a56bbac24a2765993289128ba679af5fe0c2423` | Pins, the v4 consumer and one new test (§2, §5, §6) |
| P/tests/test_retained_precision_contract.py | `faf5d40999177cab34a3920ade8c2af4f3989e6ef4558d98d936d6fc56bd8f9e` | Pins (§2) |
| P/tests/test_retained_precision_carriers.py | `12af6980763d6b9f3df4b1f85edc8874add1cc2c86d9c2cf25219b0cbd2bee0d` | Pins, the v4 consumer and one new test |
| P/fixtures/results/retained_precision_cases.json | `07f6f95cb564c661d31fd1b397fb9f1bce4e0a2a5dda53912911437d528bf86e` | 07i (the patch gives the staged `1e53ea9c…` exactly), then 9 `qualification` strings corrected (§4) |
| P/fixtures/results/retained_precision_carrier_cases.json | `4f8c3dad97b080dc9a0782ef862b3f72687fd6913fecb9e554f5381f9953afa2` | 07i's case-file part (the staged `f20a7db0…` exactly), then v4 with D-U7-4 (§5) |

## 1. The flags (brief items 1–3)

- **07i applied:** `patch -p1` from P. Both staged hashes matched before I edited anything: 07i `1e53ea9c…` and the case file `f20a7db0…`.
- **Python:** `_IMPLEMENTATION_COMPLETE = True`.
- **Rust:** `const IMPLEMENTATION_COMPLETE: bool = true;`.
- **TS** (`retainedPrecision.ts:97`) is untouched; it is I67's.
- **The stale texts use slice A's §1.2 wording.** The Python reader docstring also gains D-U7-6's sentence ("Hashes bind the supplied statements; they do not establish producer origin").

## 2. The pins (brief item 4): each moves to its 07i or oracle value; none is deleted or weakened

**Python, `test_retained_precision_contract.py`:**
- The bases with their invocation now assert `numerical_eligible is fixture["expected"]["numerical_eligible"]` and `standing == fixture["expected"]["standing"]` (13 true, 2 false). Without an invocation they stay False.
- `_corpus_entries` yields each entry's expectation:
  - a base's `expected`;
  - a must-pass entry's `expected_eligibility`;
  - not eligible without an invocation.
  
  The public-entry test compares all three fields on every passing entry.
- **The milestone:** with its invocation True and `"eligible"`; without it, False and `"needs_recompute"`.
- **The refused-coefficient control:** True, and the base's expectation is asserted to be True.
- **Must-pass:** the three fields equal `expected_eligibility`.
- **Snapshot 07i:** the allowed keys gain `expected_eligibility`; it is present exactly on the must-pass entries; the eligible counts are pinned at 13 and 13.

**Python, `test_retained_precision_carriers.py`:**
- The milestone with its invocation is `numerically_eligible`, and its reader validation is eligible. Without the invocation it stays `needs_recompute`. The test is renamed `test_standing_is_eligible_only_with_the_invocation_and_comes_only_from_the_receipt`.
- `classification_summary(source, invocation)` withholds the absolute rows only (69). Without the invocation, 97.
- The scope assertion requires "no carrier authenticates producer origin" (D-U7-6).
- The module docstring and the seam comments are reworded.

**Rust, `retained_precision_contract.rs`:**
- `complete_synthetic_controls_keep_eligibility_held` → `…_carry_their_shared_eligibility`, which asserts equality with each base's `expected`.
- `shared_must_pass_entries_validate` asserts equality with `expected_eligibility`.
- `publicly_consistent_coverage_attestations_are_not_rejected` asserts eligible, and that its bases expect eligible. Its doc is reworded.

**Rust, `retained_precision_carriers.rs`:**
- The milestone with its invocation is `numerically_eligible`, under the renamed fn `u6a_standing_is_eligible_only_with_the_invocation_…`.
- `validation.numerical_eligible` is true, and so is the derivative round trip's `after.numerical_eligible`.
- The summary with the invocation is Current: 69 withheld. Without it, 97.
- The scope requires D-U7-6. The docs are reworded.

**PP, `retained_wire_tests.rs:122`:** `invocation_bound && numerical_eligible`, with the message "eligible with the actual invocation (U7)". The :109 doc is reworded.

## 3. The Rust flag hunk: line-neutral, no allocation, no text (evidence for the Pass B entry)

**Line-neutrality.** The RS edits keep every line number:
- `retained_precision.rs` stays at 4,402 lines. The hunk is `@@ -4267,3 +4267,3 @@`: two comment lines plus the const line.
- `semantic_contract.rs` stays at 855 lines (`@@ -581,2 +581,2 @@`, doc lines only).
- `product_physics/src/lib.rs` stays at 24,333 lines (`@@ -3156 +3156 @@`, a comment).
- **The G7 Pass B pins** `retained_precision.rs:4252`, `:4253` and `:4305` (premise_pins.json) are untouched, byte for byte.
- **The four rule files** (callgraph_rules, loop_bounds, text_args, sens) have no key on any edited line. Their only keys near the hunk are `:4272:validate` and the premise pins.

**The changed code: one token.** `false` became `true` on :4269. Everything else in the hunk is comment.

**Where it is used.** `IMPLEMENTATION_COMPLETE` is read in exactly one place, :4309 (`grep`). That is `let eligible = IMPLEMENTATION_COMPLETE && actual_invocation.is_some() && source["status"]["mechanics"] == "MECHANICS_SOLVED" && list(&body["cases"]).iter().all(|c| matches!(text(&c["status"]), "selected" | "not_required"));`.
- **With the flag false,** the rest of the conjunction is dead.
- **With the flag true,** it is evaluated, and every operation in it borrows:
  - `Option::is_some`;
  - serde_json's `Index<&str>` for `Value`, which returns `&Value`, a static `Null` when absent;
  - `PartialEq<&str>` for `Value`;
  - `list` (:224), which returns `&[Value]` (`as_array().map(Vec::as_slice)`);
  - `slice::iter().all`;
  - `text` (:227), which returns `&str`;
  - `matches!` on string literals.
- **None of these allocates or builds text.** No `format!`, `String`, `to_string`, `clone`, `collect` or `Vec` is involved. The three literals were already in the source.
- **`Validation`'s layout is unchanged:** the same `bool` field takes the new value.

**Measured** (`oracle/zz_i66_u7f_alloc.rs`, a counting `#[global_allocator]`; output `alloc_{base,cand}.tsv`):
- **Setup:** `validate` on both milestones, with and without the invocation, three runs each, in the base lane (flag false) and the candidate lane (flag true).
- **Result: the allocation counts and bytes are identical in all 12 base/candidate pairs.**
  - With the invocation, sparse: run 0 gives 88,504 allocations and 8,343,370 bytes in both lanes. Runs 1–2 give 75,446 and 6,643,320 in both.
  - With the invocation, dense: 75,658 and 6,663,353 in both.
  - The candidate reports `numerical_eligible=true`, the base `false`.
- **So evaluating the newly live conjunct allocates nothing.**

The **D1 caller,** PP's precommit (`lib.rs:3161`), uses only `validate`'s `Ok`/`Err` (§4). Its published bytes are unchanged (§6, control 3).

## 4. Text the switch makes false (brief item 6), each corrected

1. **Python reader:** the comment at :27–29 and the docstrings at :1591–1593 and :1598 (slice A's text).
2. **Python carriers:** `compatibility.py:322`, the "while the reader's eligibility is held" sentence, removed.
3. **Rust reader:** the comment at `retained_precision.rs:4267–4268` (slice A's text, line-neutral).
4. **Rust carriers:** `semantic_contract.rs:581–582` (slice A's text, line-neutral).
5. **PP comment:** `product_physics/src/lib.rs:3156`, "(eligibility stays off)", becomes "(only Ok/Err is used here; eligibility is not read)". It is line-neutral.
   - It is a second D1-file hunk, comment only.
   - Its line is not a key in any Pass B rule file or pin.
   - Pass B will list it with `lib.rs`, which slice P already changed, so **ROOT needs a reviewed "comment only, no code" entry** for it, beside the flag hunk.
6. **PP test doc:** `retained_wire_tests.rs:109`.
7. **The corpus bases' `qualification` strings** (07i), as RR "U7 slice A returned" rules.
   - The phrase "public API intentionally rejects." becomes "the public reader accepts it (D-U6-1); its eligibility is its expected value (U7)."
   - It occurs **9 times**: in `ordinary_prepared_synthetic`, `ordinary_prepared_dense_synthetic`, `ordinary_prepared_no_data_synthetic`, `ordinary_prepared_cancelled_loads_synthetic`, `two_case_facade_after_certificate_synthetic`, `two_case_preparation_failure_synthetic`, `two_body_synthetic`, `p512_ladder_synthetic` and `two_case_synthetic`.
   - Only those strings changed, asserted against 07i. **So the corpus is `07f6f95c…`, not the staged `1e53ea9c…`.** No reader or test pins its hash or reads the string.
8. **The case file's note:** the v3 phrase "the held withheld count" (the summary vocabulary) becomes "the not-Current withheld count (these forms carry no invocation)".
9. **Test docs and names** that said "held" or "U7 owns the switch", in all four Python and Rust test files.

**After the edits, a search of P's core, tests, fixtures, schemas and tools** for "eligibility stays/remains/is held/off", "until U7", "U7 owns", "while the reader's eligibility" and "eligibility is held" finds nothing outside TS.

## 5. D-U7-6 and D-U7-4: the case file at v4 (brief item 5; ROOT's D-U7-4 ruling)

- **D-U7-6.** The scope sentence and the note's U7 sentence come from 07i's patch. The Python and Rust scope assertions require "no carrier authenticates producer origin".
- **D-U7-4: I did both the entry and the v4 bump** (`_run_records/make_carrier_cases_v4.py`, from `f20a7db0…`). Only TS's assertions are left to I67.
  - The format is now `I66-U6-CARRIER-CASES-v4`.
  - **The entry is I67's draft, adopted verbatim:**
    - id `D-U7-4:ts_requires_live_native_capture`, kind `language`;
    - two forms on both milestones, `invocation_without_native_capture` (`"capture": "none"`) and `stale_current_model_same_case_ids` (`current_model_edits` sets `nodes[0].position.x`);
    - Rust and Python `numerically_eligible`; TS `needs_recompute` with `RETAINED_PRECISION_NATIVE_CAPTURE_REQUIRED`.
  - **The note** documents both new fields and says that any other unknown field is a defect.
  - The 20 shared cases and the other five entries are unchanged (asserted).
- **The Python and Rust consumers read v4 explicitly, with no silent ignore:**
  - **A closed field set.** Every form's keys must be within the 9 known fields; anything else fails.
  - **`capture`.** It must be `"none"`, and only on an `invocation: "fixture"` form. The consumer then reads the invocation argument, as for `"fixture"`.
  - **`current_model_edits`.** Each edit must be `{path, op: "set", value}`. Applied to a copy of the fixture invocation's model, the path must exist and the model must change, while the consumer's own inputs (source, invocation, refs) stay exactly the fixture's.
  - **Coverage.** The run must exercise both fields and **D-U7-4's side,** a `numerically_eligible` standing with the actual invocation. The id set is now 6.

## 6. Controls (brief §Controls; each passes)

**1. The oracle diff** (`oracle/`).
- **Inputs:** `gen_inputs.py` builds every input once (380, `inputs_sha256.txt`):
  - the 15 bases, each with and without its invocation;
  - the 23 must-pass entries;
  - the 277 mutations;
  - the 20 carrier cases;
  - every declared-difference form (24, D-U7-4's included);
  - the live milestone with its invocation, without it, and with other requested refs.
- **Dumps:**
  - `dump_py.py` records the reader outcome, classifications, invocation_bound, `numerical_eligible`, the label, the carrier token, the summary, the transport, the binding of every row (the bases, the milestone and the binding forms), the AnalysisRun, the packager and the 0.1.0 wrapper;
  - `zz_i66_u7f_dump.rs` records the reader, classifications, eligibility, the token, the summary, the transport, the binding, and the derivative with its validation.
- **Each dump is run in the base lane (flags off) and the candidate lane (flags on).** `compare_oracle.py` checks them against slice A's `oracle_pre_u7.json` and `oracle_post_u7.json`. **Result: PASS in both languages** (`oracle_diff.txt`, `oracle_diff_fields.txt`):
  - **Only the eligible-path fields change** (`numerical_eligible`, the label, the token, `withheld`). **No other field changes on any of the 380 inputs.** That covers every gate outcome and code, the classifications, the transports, the binding, the derivative bytes and their validation (Rust), the AnalysisRun record, the packager's refusal and D-U6-9 (Python).
  - **The changed set equals the oracle's exactly.**
    - **The token** changes on 34 inputs: 13 bases, 13 must-pass entries, the 2 carrier `…:invocation` cases and the live milestone with its invocation (2). The other 4 are D-U7-4's forms, which are byte-identical to the milestone with its invocation.
    - **The reader's eligibility and the summary** change on those 34 plus 6. These are the milestone with other requested refs and the carrier `no_requested` and `other_requested` cases (2 modes each). There the reader is eligible, but the requested refs differ, so the token stays `needs_recompute`, exactly as the oracle has it.
  - **Every value equals the oracle's** on every key it covers, both before and after the switch.

**2. Gates and classifications unchanged; the derivative, binding, transports, packager and D-U6-9 unchanged.** These are part of control 1; see also the suites.

**3. No published byte changes.** The 324-output sweep (I61's harness, unchanged) is byte-identical to `R/I61/u3_grant2_02`'s, in all four runs (`sweep/vs_u3_grant2_02.txt`):

| Run | Result |
|---|---|
| base registered | `9a74ff16…` |
| base Stale | `0e2db8b8…` |
| **candidate registered** | **`9a74ff16…`** |
| **candidate Stale** | **`0e2db8b8…`** |

**4. Mutants** (`mutants.py`; `mutants_py_phase.json`, `mutants_rs_phase.json`). None is killed by a compile or syntax error. The mutant lanes pass their baselines first: Python 403/403, Rust 75/75.

| Mutant | Python | Rust |
|---|---|---|
| The flag reverted alone (F1, F2) | killed | killed |
| No invocation conjunct (C1, C4) | killed | killed |
| No case-status conjunct (C3, C6) | killed | killed |
| **No `MECHANICS_SOLVED` conjunct (C2, C5)** | **survives: equivalent** | **survives: equivalent** |
| Carriers ignore the reader's eligibility (C7, C8) | killed | killed |
| D-U7-6 sentence removed (D1) | killed | killed |
| D-U7-4 entry removed (D2) | killed | killed |
| `capture` removed, or another value (D3, D4) | killed | killed |
| A `current_model_edits` path absent (D5) | killed | killed |
| An unknown form field (D6) | killed | killed |
| D-U7-4's Rust and Python side set to `needs_recompute` (D7) | killed | killed |

**Why C2 and C5 are equivalent.** Any statement whose mechanics status is not `MECHANICS_SOLVED` is refused at **G7** by the base preview validator, because a blocked envelope carries preview evidence. A successor always carries `contract_evidence`, so such a statement never reaches the conjunct.
- The corpus has only `MECHANICS_SOLVED` bases.
- Hash-consistent `MODEL_INCOMPLETE`, `MECHANICS_FAILED` and `NOT_RUN` statements are refused in both languages.
- New tests pin this:
  - `test_a_solved_status_is_required_before_the_eligibility_conjunct` (Python);
  - `u7_a_solved_status_is_required_before_the_eligibility_conjunct` (Rust).
- **If G7 ever let such a statement through, these tests fail first.** The conjunct stays as defense in depth.

**5. Suites, compared with the U7 base head `12a849a7bd`** (`suite_totals.txt`, `suites/`):

| Suite | Base | Candidate | Change |
|---|---|---|---|
| Python 24-file sweep, plus the schema and carrier suites (the 24 files include the contract suite) | 1,843 passed, 30 skipped | 1,843 passed, 30 skipped | Only `test_standing_is_needs_recompute…` (×2) is renamed (`sweep.compare.txt`) |
| The three retained suites at the final bytes | — | 459 passed | +2: the new equivalence pin |
| result_export | 168 | 169 | 2 renamed (§2), 1 new (§6); every other outcome identical |
| PP registered | 705 ok, 1 failed (t13), 9 ignored | identical | `retained_wire_tests::u1_milestone_successor_both_modes` passes with its new pin |
| PP Stale | identical to registered | identical | |
| runner/headless | 85 ok, 2 failed (the known `load_reference` pair) | identical | |

The outcome tooling counts 9 ignored where ROOT's count is 10. Base and candidate are counted alike.

## 7. TypeScript: left for I67, as directed

- **No TS file is changed.** `tsc` is therefore unchanged and was not rerun.
- **Vitest on the candidate** (`vitest_summary.txt`): 3,535 tests, 3,514 passed, **21 failed**, all in `retainedPrecision.test.ts` and `retainedPrecisionIntegration.test.tsx`. Every one follows from data, with the TS flag still off:
  - 13 base expectations (07i);
  - the case-file version assertion (`v3` against `v4`);
  - the declared-difference id set (6 against 5);
  - 4 D-U7-4 form runs: TS reads `…NOT_NUMERICALLY_ELIGIBLE` where the entry expects `…NATIVE_CAPTURE_REQUIRED`;
  - the 2 milestone `…:invocation` shared cases.
- **TS already iterates the new entry's forms.** Its consumer must still read `capture` and `current_model_edits` explicitly, as ruled.
- These are I67's part 2: the flag, the pins and the TS v4 assertions.

## 8. For ROOT

1. **Pass B (slice Q) needs two reviewed entries:**
   - the RS flag hunk, D1-live, with the evidence in §3;
   - the PP `lib.rs:3156` comment-only hunk (§4).

   The `semantic_contract.rs:581–582` doc hunk is in a region already classed unreachable (G7 rows 22–24).
2. **The corpus hash moved from the staged `1e53ea9c…` to `07f6f95c…`.** The cause is the 9 `qualification` corrections that RR "U7 slice A returned" asked for (§4). Nothing pins either hash.
3. **A G7-code observation, outside U7.** For a blocked envelope, Python's reader reports `(G7, SOURCE_PREVIEW_PHYSICS_EVIDENCE_INVALID)` and Rust's reports `(G7, SOURCE_PREVIEW_PHYSICS_BLOCKED_ENVELOPE)`. Each language reports its own base code (the 06b settlement, retained_precision.rs:4302–4304). The new Rust test asserts Rust's own code. The case file's N-4 scope sentence says "G7 parity compares the reader's (gate, code)". That is right for the probes RV92 ran, but not for this class. ROOT may want the sentence to say "gate, and each language's base code". This does not gate U7.
4. **A design note for RV94, consistent with the oracle and pre-existing since U6.** `classification_summary(source, invocation)` takes its cases from the invocation's own model, not from the caller's requested refs. So a statement read with other requested refs stands `needs_recompute` while its summary reports the Current withheld count (69). This holds in Python and Rust.

## Records

`_run_records/` holds:
- the runners and chains;
- `make_carrier_cases_v4.py` and `mutants.py`;
- `oracle/`: the generator, both dumpers, the comparer, the four dumps, the alloc harness and both alloc outputs, and the inputs' sha256;
- `sweep/`: the four TSVs and the comparison with grant 2;
- `suites/`: base and candidate outcomes;
- the Python sweep comparison, the Vitest summary and the mutant results;
- the candidate diff against `12a849a7bd` and the changed-file hashes.

All use placeholder paths. SHA256SUMS covers this folder.
