# I67 return: U7 slice F part 2 (the TypeScript switch)

I67 is a TASK (Type 2) under ROOT. I worked to ROOT's slice F part 2 message, `BRIEFS/U7_SLICE_F_SWITCH.md` Part 2 and I66's part-1 return (`R/I66/u7_slice_f_01/RETURN.md`). The pin inventory is I61's `u7_slice_a_01/RETURN.md` §1.3 to §1.5. I did not delegate.

**Verdict: the TS flag is on and every TS pin carries its 07i or oracle value. Every control passes, with two rulings and one observation for ROOT (§7).**
- **Part 1's 21 failures:** all 21 now pass through pin updates. None was deleted or weakened.
- **Suites:**
  - Vitest gives 3,537 of 3,537, and `tsc` is clean.
  - Against base and against part 1, renamed titles are mapped explicitly, and every outcome change is an oracle change.
- **The TS oracle diff:** over I66's 380 inputs, TS changes on exactly the oracle's set.
- **Python and Rust:**
  - Python's three retained suites give 459 passed.
  - `result_export` gives 169 ok, identical to I66's final run.
- **Mutants:**
  - TS: 22 of 24 are killed, all by assertion. Two survive:
    - C02 is equivalent, as I66 found in Python and Rust;
    - C04 is an extra mutant that no available statement can reach (§6).
  - Python and Rust scope fences: each language's own mutants are killed.

## Basis, host and fence

- **Worktree:** `WT/f2a-u7`, at `12a849a7bd` on `codex/piping-f2a-u7-20261004`. It holds I66's part 1 and my part 2, both uncommitted. ROOT commits both.
- **Git:** no Git writes. Reads used `GIT_OPTIONAL_LOCKS=0`, except for one early `git status --short`, which I ran without it. That command may refresh the index stat cache. It changes no content.
- **When:** 2026-10-04, about 18:15Z to 18:45Z. The memory guard (PID 5387) ran throughout.
- **Runtime** (`_run_records/runtime.txt`):
  - No install. `P/node_modules` is the existing untracked symlink.
  - Nothing was built for WASM. The worktree's ignored prebuilt `public/{wasm-engine,self-weight-engine}` were already present, and the lanes carry byte copies (hashes recorded).
  - `TMPDIR` was set to `WT/scratch/i67_u6d/tmp`.
  - Cargo: the default toolchain, `--locked --offline`, `CARGO_BUILD_JOBS=4`, `RUST_TEST_THREADS=2`, one job at a time. Target dirs were `WT/targets/i67-u7f/{re-cand,re-mut}`.
  - Nothing native, solver-at-scale or DEC-025 was run.
- **Lanes** (`WT/scratch/i67_u6d/lanes`):
  - `base6` is a `git archive` of `12a849a7bd`.
  - `part1` is `base6` plus I66's 12 files.
  - `cand7` is the final bytes of all 18 changed files.
  - `mut6` is `cand7`, used for the TS mutants.
  - `pyrsmut` is `cand7` without `apps/`, used for the Python and Rust scope mutants.
- **I66's part-1 files are untouched except where allowed** (`part1_files_check.txt`). Nine of the 12 match their part-1 sha256. The other three each carry only the change ROOT allowed:
  - the case file's `scope` gains one appended clause;
  - Python's and Rust's scope assertions each gain one line.

**Part-2 files** (`part2_files_sha256.txt`; `part2.diff` is part 1 → final; `candidate_parts_1_and_2.diff` is against `12a849a7bd`):

| File (TS = P/apps/desktop/src) | sha256 | Change |
|---|---|---|
| TS/features/results/retainedPrecision.ts | `0819b4b9…` | The flag is `true`; slice A's comment and docstring text. Line-neutral (1,339 lines) |
| TS/features/results/retainedPrecisionStanding.ts | `de1d6560…` | D-U7-6: I61's sentence is in `tail`; the held clause is dropped from the header. Line-neutral (218 lines) |
| TS/features/results/retainedPrecision.test.ts | `25ecbc8b…` | 07i and oracle pins; an independent `c160`; the new G7 blocked-envelope test |
| TS/features/results/retainedPrecisionIntegration.test.tsx | `282b8d90…` | The v4 consumer, D-U7-4, D-U7-6, the G7 clause, post-U7 pins, the `u7.held` knob |
| TS/features/results/retainedPrecisionOutputRefusal.test.tsx | `bc65489c…` | A post-U7 pin that slice A did not list (§7.3) |
| TS/services/retainedPrecisionAnalysisRun.test.ts | `ee235d9d…` | :87 → eligible |
| P/fixtures/results/retained_precision_carrier_cases.json | `c98449f4…` | The G7 blocked-envelope `scope` clause, appended. Nothing else changed (checked by a parsed comparison) |
| P/tests/test_retained_precision_carriers.py | `0f110cf3…` | One line: the scope fence requires the clause and Python's code |
| P/core/reporting/result_export/tests/retained_precision_carriers.rs | `2e553478…` | One line: the scope fence requires the clause and Rust's code |

## 1. The flag and the pins (slice A §1.3)

**`retainedPrecision.ts`:**
- `SUMMARY_COVERAGE_COMPLETE = true`.
- The comment reads "Eligibility is on since U7 (D-U7-5); every gate runs regardless."
- The reader's docstring reads "eligibility as in C1:160".

**`retainedPrecision.test.ts`:**

| Pin | Before | After |
|---|---|---|
| :164 bases | `toEqual(c.expected)` | Code unchanged; 13 of 15 bases now pass as eligible through 07i |
| :195 must-pass | `numerical_eligible toBe(false)` | `{invocation_bound, numerical_eligible, standing}` `toEqual(m.expected_eligibility)`: all three fields, 13 true and 10 false |
| :287 `firstFailure` (44 call sites) | accepted ⇒ `numerical_eligible false` | accepted ⇒ `[numerical_eligible, standing]` equals an independent `c160(source, invocation)`. This is R1 written in the test: an invocation, `MECHANICS_SOLVED`, and every receipt case `selected` or `not_required`. It is not the reader |
| :926 milestone with its invocation | false / `needs_recompute` | true / `eligible` |
| :171, :174 (unbound, transport) | — | Unchanged |

**Elsewhere:**
- `retainedPrecisionAnalysisRun.test.ts:87`: the revalidated original is `[true, "eligible"]`.
- `retainedPrecisionIntegration.test.tsx`: slice A's rows.
  - The header doc is rewritten.
  - Registration through mocked IPC is now true: `{contract, status: "integrity_checked", eligible: true, findings: []}`, the token is `numerically_eligible`, and `retainedPrecisionInvocation` equals the captured invocation.
  - The rule-check gate: per slice A, a registered successor reaches the backend with `sourceBlockInvocation`. The copy and refused-bytes refusals are kept.
  - The summary's `withheld` goes from 97 to 69.
  - The shared parity check reads `eligible toBe(c.expected_standing === "numerically_eligible")`.
  - The D-U7-6 scope regex is added (§3).

**The held reader (test only).** The integration test's reader wrapper gains `u7.held`. It returns the reader's own validation with `numerical_eligible: false` and `standing: "needs_recompute"`, which is exactly the pre-U7 reader's output. It keeps the not-eligible branches exercised:
- the rule-check gate's `NOT_NUMERICALLY_ELIGIBLE` refusal;
- slice T's "with the held reader…" test;
- the notices comparison.

`u7.simulate` is removed from every positive test that used it only to force eligibility, so those tests now run on the real reader. It stays for the not_covered and no-absolute reclassifications, and in three "never eligible" tests (numerical_quality, no invocation, a saved successor), where it is now a no-op. Mutant H01 shows the held tests depend on the knob.

## 2. The v4 consumer and D-U7-4's TS side

**`applyShared` reads v4 explicitly:**
- **Closed field sets,** as Python's and Rust's `FORM_FIELDS`:
  - forms: `capture`, `current_model_edits`, `edits`, `expected`, `fixtures`, `invocation`, `label`, `requested`, `subject`;
  - cases: their seven fields.
  - Any other field fails. This is checked both in `applyShared` and in the structure test.
- **`capture`** must be `"none"`, with `invocation: "fixture"`. TS delivers the bytes without an IPC capture, then registers them with the fixture invocation and no live predicate.
- **`current_model_edits`:**
  - Each edit is a set-edit with exactly `{path, op, value}`, on an existing key.
  - The edits apply to a copy of the invocation's model, which then becomes the session's current model.
  - The edits must change the model and keep its load-case ids.

**The structure test** is now "exactly the six ruled entries…". It checks:
- the six ids;
- that both v4 fields are exercised;
- **D-U7-4's TS side,** written out literally: both forms read `{standing: "needs_recompute", finding: RETAINED_PRECISION_NATIVE_CAPTURE_REQUIRED}`.

The four D-U7-4 form×fixture tests (`invocation_without_native_capture` and `stale_current_model_same_case_ids`, in both modes) read exactly that through the real carriers.

## 3. D-U7-6

- **The standing text.** `retainedPrecisionStanding.ts`'s `tail` now reads "…against the actual invocation and the requested cases. The reader checks these bytes and their invocation; it does not establish which producer made them. Numerical checks do not establish engineering correctness."
  - The standing-text test asserts this sentence for a registered, a refused and an unregistered successor.
  - Mutant T01 (the sentence removed) is killed.
- **The scope.** TS's scope test requires `/no carrier authenticates producer origin/`. Mutant D01 removes the sentence from the case file, and it is killed.

## 4. The G7 blocked-envelope code (I66's observation 3)

**TS's code is `SOURCE_PREVIEW_PHYSICS_EVIDENCE_INVALID`, the same as Python's.** TS's base validator (`previewPhysicsEvidence.ts`) has only that one code. For a statement that is not solved and still carries evidence, rows or headlines, its detail is "blocked envelope carries evidence, rows or headlines".

**The new TS test** mirrors I66's Python and Rust tests:
- **Inputs:** both milestones, each with status `MODEL_INCOMPLETE`, `MECHANICS_FAILED` and `NOT_RUN`.
- **Resealing:** each statement is resealed with the corpus `rehash`, and the test asserts the receipt hash changed.
- **Expected:** the reader refuses with `{gate: "G7", code: "SOURCE_PREVIEW_PHYSICS_EVIDENCE_INVALID", detail: "blocked envelope carries evidence, rows or headlines"}`. The detail check makes sure the refusal comes from the blocked-envelope check and not some other G7 check.

**The clause appended to the case file's `scope`:**

> A statement whose mechanics status is not MECHANICS_SOLVED (MODEL_INCOMPLETE, MECHANICS_FAILED or NOT_RUN on a milestone successor, made hash-consistent) is a blocked envelope that still carries preview evidence: a blocked envelope is refused at G7 with each language's own base code (Python SOURCE_PREVIEW_PHYSICS_EVIDENCE_INVALID, Rust SOURCE_PREVIEW_PHYSICS_BLOCKED_ENVELOPE, TS SOURCE_PREVIEW_PHYSICS_EVIDENCE_INVALID; the 06b settlement), so for this class G7 parity compares the gate and each language asserts its own code (I66 u7_slice_f_01 observation 3; I67 u7_slice_f_01).

Each code is the one that language's own test asserts:
- Python: `test_a_solved_status_is_required_before_the_eligibility_conjunct`;
- Rust: `u7_a_solved_status_is_required_before_the_eligibility_conjunct`;
- TS: the new test above.

**The three fences** (N-9 precedent; Python and Rust are one line each):

| Language | Requires |
|---|---|
| TS | `/a blocked envelope is refused at G7 with each language's own base code[^.]*TS SOURCE_PREVIEW_PHYSICS_EVIDENCE_INVALID/` |
| Python | "a blocked envelope is refused at G7 with each language's own base code" and "Python SOURCE_PREVIEW_PHYSICS_EVIDENCE_INVALID" |
| Rust | the same phrase and "Rust SOURCE_PREVIEW_PHYSICS_BLOCKED_ENVELOPE" |

## 5. Controls

**Suites** (`suites/`). Titles changed for 42 tests; `compare_renamed.py` maps each of them explicitly, and every one is found.

| Run | Vitest | `tsc` |
|---|---|---|
| Base `12a849a7bd` (`r6_base`) | 3,531 / 3,531 | clean |
| Part 1 alone (`r6_part1`) | 3,535: 3,514 passed, 21 failed | clean |
| Final (`r7_final`) | 3,537 / 3,537 | clean |

- **Final against base** (`compare_base_vs_final.txt`): no outcome changes. Six tests exist only in the final run:
  - the four D-U7-4 forms (from part 1's data);
  - the G7 test;
  - "the shared file's eligible cases are exactly these".
- **Final against part 1** (`compare_part1_vs_final.txt`): exactly the 21 part-1 failures change, from failed to passed:
  - 13 corpus bases;
  - 2 parity `…:invocation` cases;
  - 4 D-U7-4 forms;
  - the format test (now v4);
  - the declared-entries structure test.

  Two tests are new.
- **Python, three retained suites at the final bytes:** 459 passed. This equals I66's final count; my line adds no test.
- **`result_export`:** 169 ok, 0 failed. The outcomes are identical to I66's `cand_re_final.outcomes` (`result_export_compare.txt`).

**The TS oracle diff** (`oracle/`):
- **Inputs:** I66's 380, regenerated with I66's `gen_inputs.py` from the candidate. The sha256 equals I66's `52026a92…`.
- **Dumper:** `zzI67U7fOracle.test.ts` runs in the base and candidate lanes. For each input it records:
  - the reader (outcome, classes, `invocation_bound`, eligibility, standing) and its transport check;
  - the carriers' transport route;
  - **`live`:** the input's own invocation, registered with a stand-in live capture, with standing for its requested refs. This is TS's standing rule, comparable with Python and Rust;
  - **`ipc`:** TS's real delivery through mocked IPC, for carrier, declared and milestone inputs, as `applyShared` does it;
  - the binding refusals.
- **Results** (`ts_oracle_diff.txt`: **PASS**):
  - 40 inputs change. No field outside the eligible path changes: reader outcome, classes, `invocation_bound`, transports and bindings are all identical.
  - The reader's eligibility and standing change on exactly the oracle's 40.
  - The `live` token changes on exactly the oracle's 34: 13 corpus bases, 13 must-pass, 2 carrier `…:invocation`, 2 milestones with their invocation, and 4 D-U7-4 inputs.
  - The `ipc` token changes on exactly 4: the 2 carrier `…:invocation` cases and the 2 milestones with their invocation (the live successor).
  - On the four D-U7-4 inputs, TS's `ipc` token stays `needs_recompute`, as ruled. Only the finding changes, from `NOT_NUMERICALLY_ELIGIBLE` to `NATIVE_CAPTURE_REQUIRED`.
  - `withheld` follows I66's rule (40 of 40).
  - Every base value equals `oracle_pre_u7.json`, and every candidate value equals `oracle_post_u7.json`, on every key the oracle covers.

**TS mutants** (`mutants/mutants_f.*`, lane `mut6`):
- 24 mutants. Each is one textual or data mutation, run against the 4 retained TS test files and 9 related ones, then restored.
- 22 are killed, all by assertion; none by a compile or load error.

| Id | Mutation | Result |
|---|---|---|
| F01 | The TS flag reverted | killed (68) |
| C01 | Invocation conjunct dropped | killed (13) |
| C02 | `MECHANICS_SOLVED` conjunct dropped | **survives: equivalent.** G7 refuses every unsolved statement first; the new G7 test pins this, as I66 found in Python and Rust |
| C03 | Case-status conjunct dropped | killed (14) |
| C04 | Case-status conjunct narrowed to `selected` (extra) | **survives** (§7.2) |
| C05 | The reader's standing label decoupled from eligibility | killed (32) |
| T01 | D-U7-6 sentence removed from the standing text | killed |
| G01 | TS's blocked-envelope check removed | killed, including by the new G7 test |
| D01 | D-U7-6 scope sentence removed | killed |
| D02 | D-U7-4 entry removed | killed |
| D03 | D-U7-4's TS side set to `numerically_eligible` | killed |
| D04 | D-U7-4's TS finding changed | killed |
| D05 | `capture` removed | killed |
| D06 | `capture` set to another value | killed |
| D07 | `current_model_edits` removed | killed |
| D08 | Unknown form field | killed |
| D09 | Unknown case field | killed |
| D10 | Edit path names an absent key | killed |
| D11 | Edit changes a load-case id | killed |
| D12 | G7 clause removed | killed |
| D13 | G7 clause misstates TS's code | killed |
| A01 | The consumer ignores `capture` | killed |
| A02 | The consumer ignores `current_model_edits` | killed |
| H01 | The held knob ignored | killed (6) |

**Python and Rust scope-fence mutants** (`mutants/pyrs_mutants.*`, lane `pyrsmut`; Python's declared-differences test, Rust's `u6_declared_differences_rust` rebuilt each time because the file is `include_str!`):

| Mutant | Python | Rust |
|---|---|---|
| control | passes | passes |
| G7 clause removed | killed | killed |
| Python's code misstated | killed | passes, as intended: each language asserts its own code |
| Rust's code misstated | passes, as intended | killed |
| D-U7-6 sentence removed | killed | killed |

## 6. Inherited and unchanged

No product file changed outside the flag, the comments and the standing text:
- `numericalResultQuality.ts`, `previewService.ts` and the panels are slice T's bytes;
- the 07i corpus is I66's;
- no published byte changed.

## 7. For ROOT

1. **Ruling: the G7 clause text** (§4). I wrote it as an appended `scope` sentence that names each language's code. If ROOT prefers I66's wording ("gate, and each language's base code") inside the N-4 sentence, the three fences key on the phrase "a blocked envelope is refused at G7 with each language's own base code" and on the language-and-code pairs. A rewording would only need those substrings kept.

2. **Ruling or later work: C04, reader-level `not_required` eligibility is untested in every language.**
   - No 07i statement has a `not_required` case: every base is `selected`, or `selected` and `unavailable`. No Python or Rust contract test has one either.
   - So narrowing the conjunct to `selected` survives in TS, and would survive in Python and Rust.
   - The carrier-level `not_required` rule (F-7, `retainedStandingFrom` with two cases) is tested.
   - Dropping the whole conjunct (C03, the brief's mutant) is killed.
   - Closing C04 needs a hash-consistent statement with a `not_required` case that passes G1 to G8: a 07j corpus entry, if the producer can emit one. This is not in my fence.

3. **Observation: slice A §1.3 and §1.4 missed one TS pin and one consumer.**
   - **The pin:** `retainedPrecisionOutputRefusal.test.tsx:200` (U6d) said "a registered successor is fresh but never numerically eligible, so it is never Current". It fails with the flag on (2 tests).
   - **The update:** it now reads eligible for the captured model only. A copy reads `VALIDATION_REQUIRED`; a moved node reads `NATIVE_CAPTURE_REQUIRED`.
   - **The consumer:** session Current (`resultsSessionState.ts`) now admits a live registered successor whose other conjuncts hold, including `hasNativeMechanicsInvocation`. D-U7-4's description already names session Current. In product nothing changes: no native capture of a successor exists (F-1).

4. **Observation: TS's `classificationSummary(source, model).withheld` is not bound to the live capture.**
   - `withheld` follows D2 4.9.4 (`retainedStandingFrom`), as Rust's `classification_summary_from` does, without TS's live-capture conjunct.
   - So after U7, both D-U7-4 forms read `withheld` 69, as if Current, while TS's standing is `needs_recompute` with `NATIVE_CAPTURE_REQUIRED` (`ts_oracle_diff.txt`).
   - No product caller passes a model: `knownSemanticLimitations.ts:168` calls it without one, and the notices do not show `withheld`. So nothing displays this today.
   - **Fix, if ROOT wants it** (fails closed, one line, not applied): in `classificationSummary`, pass `requestedRefs(model)` only when `retainedPrecisionStanding(source, model).eligible`, and `[]` otherwise. Alternatively, name the summary in D-U7-4's scope.

5. **Disclosure:** the one Git read without `GIT_OPTIONAL_LOCKS=0` (see Basis).
