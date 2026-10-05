# RV88 review of U6d (`9555b6ffc2`): the TypeScript carriers and standing

RV88, the standing independent U6 reviewer, is a TASK (Type 2) under ROOT (HELP_HUMAN, Agent 0), working to ROOT's U6d message, `BRIEFS/U6_FANOUT_COMMON.md` and PLAN §6 U6d. ROOT is the return path, and RV88 did not delegate.

RV88 did not write this code and did not use I67's tests as oracles. Every control below is RV88's own test, script or comparison. I67's suite and mutants were rerun only as items to report.

## Verdict: PASS, with 0 BLOCKING, 1 SHOULD-FIX and 4 NOTE findings

- **Existing behaviour is unchanged.**
  - RV88's own desktop sweep covered 69 existing-identity envelopes, including 7 legacy envelopes with no producer. It recorded 15 carrier, standing, binding and display outcomes, including the rendered ResultsPanel text and the AnalysisRun hash. The results are identical to base.
  - The full Vitest suite is 3,258 → 3,417 tests. Every base test keeps its outcome; the S-1 pin is renamed, and 159 tests are new. `tsc` is clean in both lanes.
- **Registration and standing behave as ruled.**
  - Registration is bound to the exact bytes, including zero signs and key order, and to the captured invocation. Every edit voids it, and a revert restores it.
  - Standing never reads `numerical_quality`, and stays `needs_recompute`.
  - RV88's own TS mapping of the 14 parity cases equals the shared expectations, which Rust U6a asserts.
- **Every downgrade form is refused.** RV88 tried 96 relabel forms per mode, plus 4 injected forms on each existing envelope.
  - AnalysisRun receipt equality refuses 6 mutation forms, and a base record carrying a receipt or a null member.
  - Reopen revalidates, and refuses an altered copy.
  - The rule-check gate refuses before any backend call.
  - The T6 outputs and the report package refuse.
- **The S-1 pin keeps exact equality.**
- **F1 is acceptable as a declared difference, if it is pinned** (N-1). **F2 is fail-closed** (N-2). **F6's texts claim nothing beyond the receipt** (N-3).
- **S-1 (before U7):** the successor's standing value is not bound to the current model or to the capture's validity, as source-block standing is. Every reliance gate checks the native invocation, so nothing opens today. The flag is held, so this is post-U7 only.
- **Mutants:**
  - I67's suite kills 26/26 of a sampled quarter of its 103.
  - RV88's own 9: I67's suite kills 8 and RV88's test kills 8. One survives each suite, but no mutant survives both.

## Basis and host

- **The copies:** `9555b6ffc2` (candidate) and `844448112f` (base) were copied with `git archive … -- projects/chirality-piping ':(exclude)…/execution'`. That is a Git read with `GIT_OPTIONAL_LOCKS=0`. The copies went to `WT/rv88/d_{cand,base}`, with a mutant copy in `WT/rv88/d_mut`.
- **The candidate's files match I67's record.** The 14 changed files equal I67's `changed_files_sha256.txt`; the S-1 test patch is the 15th file. I67's SHA256SUMS checks 42/42 OK.
- **The runtime, disclosed:**
  - `P/node_modules` is a symlink to `REPO_ROOT/projects/chirality-piping/node_modules`.
  - `apps/desktop/public/{wasm-engine,self-weight-engine}` were copied from `WT/f2a-readers`. Their 8 sha256 values equal I67's `runtime.txt` (`wasm_hashes.txt`).
  - Nothing was installed or built.
  - Node v24.18.0 and Vitest 4.1.10, with `TMPDIR` under RV88's scratch.
- **When:** 2026-10-04, about 09:50Z to 10:25Z, with the memory guard (PID 5387) running.
- **Not run:** no native, solver-at-scale or DEC-025 job, and no install. The only cargo builds were the two release CLIs, for U6c's Python lane.

## 1. Existing behaviour

**The suites** (`suite_compare.txt`):

| | Base | Candidate |
|---|---|---|
| Vitest (full) | 3,258/3,258 in 135 files | 3,417/3,417 in 138 files |
| tsc `--noEmit` | 0 errors | 0 errors |
| Per-test outcome changes | — | none; one renamed test (S-1); 159 new |

**RV88's sweep** (`zzRv88Sweep.test.tsx`, both lanes, base APIs only; `rv88_ts_sweep_compare.py` → `sweep_summary.txt`).
- **What it walks:** every JSON file under P (excluding `execution` and `node_modules`), recursing to depth 9 into objects that carry a producer id, plus legacy 0.1.0 raw envelopes.
- **What it records for each envelope:**
  - route, binding, current, fresh;
  - standing with no model and with its own-case model;
  - standing reason, notices, output refusal, report reason, table;
  - every row's binding refusal, label and category;
  - the rendered ResultsPanel's standing line, and the sha256 of its full text;
  - the AnalysisRun v0.3 build and validate hash.

| Result | Count |
|---|---|
| Existing-identity envelopes, identical to base in all 15 fields | **69/69**: load-reference-1 ×6, load-reference-source-1 ×10, physics-1 ×6, physics-source-1 ×14, precision-1 ×4, preview-physics-1 ×7, source-blocks-1 ×15, legacy ×7 |
| `!receipt` and `!null`: route unsupported, standing finding `RETAINED_PRECISION_DOWNGRADE_FORBIDDEN`, not fresh, AnalysisRun refused | 69/69 each. At base, 68 routed as supported and 23 built an AnalysisRun, which confirms I66's F-5 in TS. |
| `!token0` and `!tokenlast`: the same refusal | 67/69 each. The other 2 envelopes have no rows. |
| `!othertoken` (another method string): identical to base | 69/69 |
| The 17 successor envelopes | base: unsupported. Candidate: the retained route, fresh, the AnalysisRun builds after the reader passes, standing `VALIDATION_REQUIRED` (unregistered). |

## 2. Registration and standing (`zzRv88U6d.test.tsx`; facts in `u6d_facts.tsv`)

All assertions pass, in both modes, through mocked direct IPC with PP's pinned bytes.
- **What registers:** only a delivery with a captured model. A copy, or a model-less delivery, reads `VALIDATION_REQUIRED`.
- **What voids it:** six edits void the registration, and every row then binds-refuses `RULE_QUANTITY_NOT_COVERED`. Each revert restores it. The six are:
  - a row value moved by one ulp;
  - an unknown top-level key;
  - a receipt field;
  - `numerical_quality.status`;
  - 0 changed to -0;
  - the token removed from one row.

  Reordering a key also voids it, because the fingerprint is the exact checked JSON text: fail-closed.
- **Refused and foreign deliveries:** a refused delivery records the reader's code (`unsupported`) and never registers natively. A foreign-mode delivery reads `unsupported` (`RETAINED_PRECISION_INVOCATION_MISMATCH`).
- **The standing seam never reads `numerical_quality`.**
  - With `{}`, `null`, a `checks_passed` claim or `failed`, eligibility stays exactly the validation's.
  - A two-case receipt with its refs reversed is `needs_recompute`.
- **The 14 shared cases, by RV88's own TS mapping:** all 14 equal the file's expected standing and dispatch. The mapping is:
  - the fixture invocation is captured through mocked IPC; `null` means a model-less delivery;
  - the requested refs come from the model's load cases;
  - dispatch is the route, then the reader without an invocation.

**Post-U7, simulated by a test-only reader wrapper; no flag was touched:**

| Scenario | `retainedPrecisionStanding` | `hasNativeMechanicsInvocation` | Rule-check gate |
|---|---|---|---|
| Same model | `numerically_eligible` | true | — |
| Model edited after the solve, with the same load-case ids | **`numerically_eligible`** (`numericalResultStanding` → `integrity_checked`) | false | refused, `RULE_NATIVE_INVOCATION_REQUIRED` |
| The caller's model object changed while the reader validated (the native capture is not registered) | **`numerically_eligible`** | false | refused, `RULE_NATIVE_INVOCATION_REQUIRED` |

These two rows are S-1.

## 3. The downgrade guards and refusal codes

- **Relabel forms (96 per mode):** 8 identities × 2 profiles × 6 forms. The forms are a receipt, null, `{}`, tokens only, one token on the last row, and a legacy 0.1.0 source with a receipt. Every form is:
  - routed `unsupported` and not fresh;
  - given the standing finding `RETAINED_PRECISION_DOWNGRADE_FORBIDDEN` (96/96);
  - refused by `buildAnalysisRunV03`;
  - refused by a real `deriveResultDocument` attempt, `SOURCE_SEMANTIC_CONTRACT_UNSUPPORTED`.
- **AnalysisRun:**
  - the product build copies the receipt byte-equal and validates;
  - a dropped, null, `receipt_sha256`-edited, body-edited, extra-key or other-mode copy is refused `ANALYSIS_RETAINED_PRECISION_RECEIPT_MISMATCH`;
  - a preview-physics-1 record carrying a receipt, or a null member, is refused `ANALYSIS_RETAINED_PRECISION_DOWNGRADE_FORBIDDEN`.
- **Reopen,** with the simulated post-U7 reader:
  - a saved successor gives `[HISTORICAL_INPUT_MANIFEST_MISSING, VALIDATION_REQUIRED]`, and is never eligible;
  - a mutated saved row adds the reader's `RETAINED_PRECISION_RECEIPT_MISMATCH`;
  - an altered copy gives `ANALYSIS_RETAINED_PRECISION_RECEIPT_MISMATCH`.
  - The copy is compared by checked canonical hash, so TS avoids the `0.0`-against-`0` exposure found in U6a's N-5.
- **The rule-check gate:** a registered successor is refused `RETAINED_PRECISION_NOT_NUMERICALLY_ELIGIBLE` with **no** `run_rule_checks` call.
- **T6 and the report package:**
  - the shared refusal returns `RETAINED-PRECISION-OUTPUT-NOT-YET-AVAILABLE: …`;
  - `deriveResultDocument` and `validateResultDocument` throw it;
  - the report package gives `REPORT-PACKAGE-FRESH-RESULT-UNAVAILABLE`.
  - I67's 18-panel render test is not repeated here.

## 4. The S-1 pin patch

`knownSemanticLimitations.test.ts:46–51` still uses `toEqual` on the sorted set. It gains exactly the ruled id, and the title names it. Nothing else changed in the file. **It keeps exact equality.**

## 5. F1, F2 and F6

- **F1, judged acceptable as a declared difference** (D2 §4.7: "declared language-specific strings, zero undeclared differences, and zero cases accepted by one language and refused by another"). Three reasons:
  - **Neither language accepts the input.** An unregistered invalid successor is never eligible in either.
  - **TS has no `unsupported` standing status for any route.** An unsupported route is `needs_recompute` with a finding; that is plan F-2's existing convention.
  - **The ruled plan defines TS's case.** PLAN §3 rule 1 sets "nothing is registered (TS): `needs_recompute`". The synchronous standing cannot run the asynchronous reader.

  **But the difference is not pinned anywhere.** No shared case has an invalid statement without an invocation (N-1).
- **F2** (`ruleBindingPrecheck`): **fail-closed and display-only.**
  - Registered: only the 69 absolute rows are refused (`RULE_QUANTITY_BELOW_VERIFIED_FLOOR`).
  - Unregistered: all 98 or 99 rows are refused, with `N_RP_UNVALIDATED`.
  - The real gate needs a native invocation and eligible standing, which an unregistered successor never has.
  - This is a second declared difference from Rust's helper, which classes a valid statement without an invocation (N-1).
- **F6, the new text.** Each claims nothing beyond the receipt:
  - **`N_RP_ABSOLUTE` and `N_RP_NOT_COVERED`** are D2 §4.9.9's texts, verbatim (asserted).
  - **b is printed upward to 3 significant digits.** Checked against an exact BigInt rational oracle on 20,014 doubles, including subnormals, the maximum, carries (9.995e-5) and the milestone's bounds:
    - never below b;
    - 20,012 are the least 3-digit decimal at or above b;
    - 2, where b is exactly a 3-digit decimal, are one step above, which is conservative;
    - 0 are wider.
  - **Every absolute row's label names the receipt's bound in the reader's SI unit.** That is 69 per mode: mm→m ×2, MPa→Pa ×15, N, N*m and Pa unchanged. The units are checked against the receipt's own list, and the reader normalizes the same 4 units. No label mentions a stop rule, enclosure or interval.
  - **`N_RP_UNVALIDATED`, the per-case notices** ("case: 69 quantities verified only to an absolute bound, below the relative accuracy floor…"), **the output refusal and the registered and unregistered standing texts** are all truthful (N-3 has one wording point).
  - **TS already names the unit,** which is what U6a's S-2 asks of Rust's derivative message.

## 6. Mutants (`rv88_ts_mutants.py` → `mutants_i67.json`, `mutants_own.json`)

- **The setup:** each mutant runs I67's 11 files (the 3 new and 8 related) together with RV88's `zzRv88U6d.test.tsx`, in `WT/rv88/d_mut`. Kills are attributed by file. The control passes 311/311.
- **I67's sample** (every 4th of 103: N01, N05 … H03; 26 in all):
  - **26/26 are killed by I67's suite.**
  - RV88's test also kills 11. It is a refusal-focused oracle, so this is informational.
- **RV88's own 9:**

| Mutant | I67's suite | RV88's test |
|---|---|---|
| V01: the guard reads only the first row's token | **survives** | killed |
| V02: the guard lets a null member through (`!= null`) | killed | **survives** (RV88's null form keeps tokens) |
| V03: the registration ignores zero signs | killed | killed |
| V04: requested refs are order-insensitive | killed | killed |
| V05: b is printed to the nearest value, not upward | killed | killed |
| V06: an unregistered successor's rows bind | killed | killed |
| V07: b is labelled in the row's unit, not SI | killed | killed |
| V08: the successor's output is not refused | killed | killed |
| V09: a reader refusal reads `needs_recompute` | killed | killed |

## Findings

| ID | Severity | Where | Evidence | Remedy |
|---|---|---|---|---|
| **S-1** | SHOULD-FIX (before U7) | P/apps/desktop/src/features/results/retainedPrecisionStanding.ts:148–154; previewService.ts:129–133 | `retainedPrecisionStanding` binds the bytes and the captured invocation, but **not the current model** and **not the capture's validity**. C1:162 asks for a TS validation "bound to immutable actual request/mode/result fingerprints". Source-block standing (`knownSourceStanding`, sourceBlockRecovery.ts:303–310) requires the current model to equal the invocation's model or the caller snapshot. Two consequences, post-U7 (simulated; §2): (a) after the model is edited with the same load-case ids, standing is still `numerically_eligible` (`integrity_checked`); (b) the registration is recorded before `validateCapturedSource`'s final invalidation check, so a raced caller edit leaves an eligible retained registration with no native registration. **Nothing relies on it today:** the rule-check gate (`hasNativeMechanicsInvocation`, ruleCheckService.ts:122) and session Current both check the native invocation, and every output refuses the successor. The flag is held. But the standing value, and its `integrity_checked` status, would disagree with the native binding. | Bind the model inside `retainedPrecisionStanding`, as `knownSourceStanding` does, or require `hasNativeMechanicsInvocation(source, model)` there. Record the retained registration only after `validateCapturedSource`'s final fingerprint and invalidation check, or void it when that check fails. Add two U6d tests (model edit; raced caller edit), each `needs_recompute` under `u7.simulate`. |
| **N-1** | NOTE (F1, F2: parity) | P/fixtures/results/retained_precision_carrier_cases.json; knownSemanticLimitations.ts:111–114 | Two TS-specific differences are declared only in I67's RETURN: (1) an unregistered invalid statement is `needs_recompute` (`VALIDATION_REQUIRED`), where Rust and Python say `unsupported`; (2) an unregistered valid statement refuses every row's binding, where Rust classes it. Both are fail-closed, and no reliance differs. Neither is in the shared case file, so U6b and U6f cannot see them. | U6f, or U6b, which edits the file next: add per-language expectations for (1) an edited row with no invocation, and (2) binding on a valid statement with no invocation. Record both as declared differences in the U6f parity summary, alongside U6a's N-1 (a refused statement refuses every row). |
| **N-2** | NOTE | retainedPrecisionStanding.ts:58–60, 94–99 | Registration fingerprints the exact checked JSON text, so reordering one member's keys in place voids it (`u6d_facts.tsv`). This is fail-closed and consistent with "exact bytes". Only an in-place mutation can do it, because registration is keyed by object identity. | None needed; state it in the TS doc comment. |
| **N-3** | NOTE (F6 wording) | retainedPrecisionStanding.ts:196 | For a delivery the reader refused **in this session**, the standing text says "…unsupported, **historical values only**". The phrase is reused from the generic unsupported text, but the values are not historical. | Prefer "values shown for inspection only". It is text only. |
| **N-4** | NOTE (scope) | — | Every check here is a mocked-IPC unit replay. No native or session witness is possible (I67 F3; plan F-1; D-U6-3). RV88 did not repeat I67's 18-panel render test; it checked the shared refusal function and the adapter and report surfaces directly. | As ruled: the native witness belongs to native activation. |

## For ROOT to rule

1. **S-1:** whether U6d is repaired now, or before U7. RV88 recommends before U6f closes, since it is a 2-test, few-line change in U6d's fence.
2. **N-1:** have the shared case file carry the declared TS differences with per-language expectations, so that U6f's parity summary can count them.

## Records

Everything is in `_run_records/`, with placeholder paths only. **The tests and scripts:**
- `zzRv88Sweep.test.tsx` and `zzRv88U6d.test.tsx`;
- `rv88_ts_sweep_compare.py` and `rv88_ts_mutants.py`.

**The outputs:**
- the sweeps `sweep_d_{base,cand}.tsv.gz` and `sweep_summary.txt`;
- `u6d_facts.tsv`;
- `suite_compare.txt` and `suites.txt`;
- `mutants_i67.json` and `mutants_own.json`;
- `wasm_hashes.txt` and `runtime.txt`.

SHA256SUMS covers this folder.
