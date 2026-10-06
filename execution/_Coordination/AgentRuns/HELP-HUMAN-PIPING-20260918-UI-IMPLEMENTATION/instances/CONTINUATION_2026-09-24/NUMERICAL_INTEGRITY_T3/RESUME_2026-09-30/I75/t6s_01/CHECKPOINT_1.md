# I75: T6S checkpoint 1 (T6S-3, the gate split; T6S-5, stress-neutral)

TASK (Type 2), I75, for ROOT. 2026-10-06 UTC.

**Briefs:** `R/BRIEFS/T6S_COMMON.md` (sha256 `c6ac6c65…`) and `R/BRIEFS/I75_T6S_TS.md` (`812f36d3…`).
**Basis read:** I74's plan `R/I74/t6_slice_plan_01/PLAN.md` (`0350c918…`, in full); RR (`f7c99c48…`) lines 12394–12458 (the T6 slice ruling), 11898–11966 (decision 12), 9439–9470 (D-U6-2), 9755–9830 (U6c, U6d; F-U6c-2, I67's F4), 10880–10915 (D-U7-4, D-U7-6); D2 (`993f5f3a…`) §4.9 in full; `R/I68/u8_probe_01/PROBE.md` §7–§8 (`ba5f7df6…`); `NUM/AGENTS.md` (`f96feb19…`), `NUM/agents/AGENT_TASK.md` (`1a13a5b0…`), `NUM/P/AGENTS.md` (`d9f2b23a…`). Code: `RE/src/derivative.rs` lines 1–480 and `RE/src/semantic_contract.rs` 147–193, at the base.

**Notation:** WT, NUM, P, DT, RE, R and RR as in the briefs. **Base** = main `c1bfc460fc`. **Successor** = `openpipestress.result_semantics/0.3.0/preview-physics-retained-1` (TS route `retained_preview_physics`). Records use placeholders only.

**Host boundary kept:** no Git writes (reads with `GIT_OPTIONAL_LOCKS=0`); no cargo, native, solver, DEC-025 or install job; no delegation; nothing in the system temp directory. Scratch is `WT/scratch/i75_t6s/`. Work is uncommitted in `WT/t6-outputs` (branch `codex/piping-t6-successor-outputs-20261005`).

## 0. In brief

1. **T6S-3 and T6S-5 are done** in seven fenced files (§1). `tsc --noEmit` is clean.
2. **One fence stop (S-1, §4).** Four existing tests in `DT/features/results/retainedPrecisionIntegration.test.tsx` (outside my fence) pin the explicit N-5 panel refusal of an *eligible* successor, which the slice replaces by ruling. They fail on the candidate. I did not edit the file. A proposed 38-line patch is in `_run_records/proposed_integration_n5.diff`; with it the file passes 182 of 182 in a scratch copy (base: 182 of 182). **ROOT must rule on it.**
3. **Suites (§6.1):** base 3,552/3,552 (138 files); candidate 3,568/3,572 (139 files). The +20 are the new stress-neutral/policy file; the 4 failures are S-1's; 3,548 tests are unchanged. No test is removed.
4. **Unchanged routes (§6.3):** both builders were instrumented identically on base and candidate copies, and the full suite fed them: **182 distinct non-successor builder inputs, 0 byte differences**, no nondeterminism; the only candidate-only inputs are 3 successor packages.
5. **Schema (§6.4):** the successor packages (both pinned modes and `two_case_synthetic`) validate under `stress_neutral_export.v0.3.schema.json` and its dispatcher, matching exactly the successor branch (index 7 of 8), with **no schema change**.
6. **Controls (§6.5)** refuse with their expected codes. **Mutants (§6.6): 34 of 34 killed**, including the seven the brief names.
7. **Readings for ROOT (§5):** the most material is R-2: under the export's existing convention, every successor package with a classed row reads `validation_status: blocked` (`validation_ready: false`), as packages with diagnostic-work rows do today. I followed the convention.

## 1. Changed files (all under DT; uncommitted)

| File | sha256 (candidate) | Base sha256 | Lines |
|---|---|---|---|
| `features/results/outputPolicy.ts` | `712941315278fc92773366d59e34c45bf4d84e06541265402cc5bbd9ffbc9f4b` | new | 148 |
| `features/results/retainedPrecisionDisclosure.ts` | `9ee23d2678426f6c13c2c89032bb2c8ada024c1cd954dd6acb81d04d5976b8b9` | new | 89 |
| `features/results/loadReferenceOutputAvailability.ts` | `e1662f09334a12c2deaeda859693f2cf1d804c4de091b0c90e96abc6da7c30a9` | `910713da…` | 51 |
| `features/result-export/ResultExportPanel.tsx` | `d362eb2bae112a0d9cb412627d53afc7efef193c31cd61d32183e7a6b2fb6da5` | `17f7a686…` | 174 |
| `features/stress-neutral/StressNeutralExportPanel.tsx` | `50bd72355ccbc8a1c1ca9cfa7a1581ba44adfa9982f0b53a3f3350c29aea2e60` | `fae61807…` | 1201 |
| `features/results/retainedPrecisionOutputRefusal.test.tsx` | `e5160b73feba71f889a2ef58d1c2b204efcf646bc4f9fea3b80f2a6ed415155e` | `1b610c31…` | 232 |
| `features/stress-neutral/retainedPrecisionStressNeutral.test.tsx` | `fca85be9f1a845387e5bac1882dd587d5f0bdaff082c02f729e9ca95bd65f8df` | new | 380 |

The complete diff against the base is `_run_records/t6s_cp1.diff`. `resultExportAdapter.ts` is not yet changed (T6S-4).

## 2. T6S-3: the gate split and the output policy

- **`outputPolicy.ts`** holds `OUTPUT_POLICY: Readonly<Record<SourceContract, RouteOutputPolicy>>`, one deliberate entry per route, and `OUTPUT_SURFACES`: the 18 surfaces of T1's group a (by their test-id prefixes), `report-package`, `result-export` and `stress-neutral`.
  - Ungated (`{gate: "none"}`): legacy, precision, physics, source_blocks, physics_source, preview_physics and unsupported. The policy adds no refusal; each surface's own checks apply, including its refusal of an unsupported (relabelled or downgraded) statement, as today.
  - `load_reference`, `load_reference_source`: every surface refused with T1's text.
  - `retained_preview_physics`: every surface named; `result-export` and `stress-neutral` are `admitted_when_eligible`; the other 19 (including `rule-check` and `report-package`) are `refused` with the reworded text.
  - **Fail closed:** a route value with no entry refuses (`OUTPUT-ROUTE-NOT-REGISTERED`), and a surface an entry does not name refuses.
  - **`tsc` exhaustiveness, checked (§6.2):** removing a route entry, adding a route to `SourceContract` (B3's `physics-retained-1`), or leaving a surface unnamed in the successor entry each fails to compile.
- **Admission** (`surfaceOutputRefusal(source, model, surface)`): for an admitted surface, `numericalResultStanding(source, model).eligible`, which for the successor already requires the live native capture (D-U7-4). Otherwise the reason is the standing's own first finding, e.g. `RETAINED_PRECISION_VALIDATION_REQUIRED: …` or `RETAINED_PRECISION_NATIVE_CAPTURE_REQUIRED: …`. The route-level form (`surfaceRouteRefusal`) is for validators and pure projections, which take no model.
- **`loadReferenceOutputRefusal` keeps its meaning:** it now returns the policy's route refusal, which is identical on every route (pinned by the existing tests and the new ones). The 18 surfaces keep calling it through `LoadReferenceOutputGate`; the report package keeps T0R's fresh-result refusal. The refusal constants moved to `outputPolicy.ts` and are re-exported unchanged, to avoid an import cycle.
- **The two panels** read their own entries: `ResultExportPanel`'s `liveResultBinding` and `StressNeutralExportPanel`'s `liveStressBinding` and refusal line. The stress-neutral panel keeps its existing test id for the refusal line.
- **The summary line:** each panel shows one text-only line from `classificationSummary(result, model)` (`retainedPrecisionSummaryLine`), e.g. `Retained precision, per case: case: 69 verified only to an absolute bound; 0 uncovered.` It appears only for a successor with a validated registration and changes nothing.
- **Decision 12's rewording** (the 18 surfaces and the report): `RETAINED-PRECISION-OUTPUT-NOT-YET-AVAILABLE: This output of retained-precision results (preview-physics-retained-1) is not yet available on the desktop; only the result JSON and stress-neutral exports admit a numerically eligible result. It is routed to T6. The result remains readable here; this is not a finding about the result.` The existing assertions use the constant, so none changed text. The load-reference text is unchanged.
- **Closed until activation:** no product caller delivers a successor with a live capture before B8 (RR decision 3). Every positive test here registers through mocked IPC.
- **Result export at this checkpoint:** the panel's gate admits an eligible successor, but `resultExportAdapter.ts`'s builders still refuse it with the shared text until T6S-4.

## 3. T6S-5: stress-neutral export of the successor

`buildStressNeutralExportPacket` and `validateStressNeutralExportPacket` gain the successor:
- **UTF-8 CSV** (`usesUtf8Csv`), and the successor's table path (`semantic_contract_v0_3_preview_physics_retained_1.json`). The table-path map is now a `Record` over every route, so a new route must name its table (or null).
- **`contract_evidence`** copied; **`retained_precision` copied whole** (D2 §4.9.7); **`source_annotations`** as for every non-precision-1 route.
- **The transport header carries the receipt** (I67's F4 closed), so a successor package dispatches to its own route, and a receipt on any other identity reads unsupported by the existing downgrade guard. **Transport validation** runs the accepted reader's `validateRetainedPrecisionTransport` (G0–G2 and the base transport metadata); a refusal carries the reader's code.
- **Admission:** the builder calls the policy with its model, so it refuses an ineligible successor with the standing's reason. The existing precision branch (analysis-run binding, received-result hash) is unchanged.
- **Classified rows (S-d):** classes come from the accepted reader run on the bytes **without an invocation** (`retainedRowClassesFromReader`, Rust `retained_row_classes`). For an `absolute_verified` or `not_covered` row the CSV row and value stay, and the unit-preservation witness is withheld with `SN-UNIT-WITNESS-WITHHELD-RETAINED-PRECISION-ABSOLUTE-VERIFIED` or `…-NOT-COVERED`, severity `info`, class `unit_preservation_witness`, whose **message is Rust `class_disclosure`'s text exactly**, e.g.
  `displacement_magnitude: retained_precision_absolute_verified; verified only to the receipt's absolute bound b = 5.4215527659630466e-24 m (binary64 3b1a378ea78c5ce9), below the relative accuracy floor; source value/unit and annotation retained; withheld from rule binding and reliance`.
  The texts live once, in `retainedPrecisionDisclosure.ts` (codes, SI-unit table, `{:e}` formatter, messages), for T6S-4 to reuse (PLAN §4.3).
- **Counted in the loss report:** the exported entry's reason gains, for the successor only, `Of these, A rows are verified only to the retained-precision receipt's absolute bound and U rows are uncovered; both are withheld from rule binding and reliance.` A successor boundary note states that the receipt travels whole, that classed rows are withheld, and that no invocation or producer-origin claim travels (CQ-4, CQ-5).
- **Validation with a source** additionally checks: receipt equality (`SN-RETAINED-PRECISION-RECEIPT-MISMATCH`); a receipt on another identity (`RETAINED_PRECISION_DOWNGRADE_FORBIDDEN`); the class findings, codes, severities and messages, recomputed from the reader (`SN-PRECISION-WITNESS-BINDING-MISMATCH`); and the loss-report count (`SN-RETAINED-PRECISION-LOSS-COUNT-MISMATCH`). Header-only validation checks the new codes' `info` severity in the existing accounting.
- **Bound formatting (CQ-1):** `rustLowerExp` prints shortest round-trip digits (`toExponential()`), normalizes `e+` to `e`, and keeps a negative zero's sign as Rust does. Pinned edges: `1e0`, `1.5e0`, `1.2345e3`, `1e21`, `9.007199254740992e15`, `5e-324`, the largest subnormal, the smallest normal, the largest finite, `0e0`, `-0e0`, and the pinned successors' five distinct bounds (expected strings computed independently with Python's shortest `repr`).

## 4. Stop S-1: a needed change outside the fence

- **What:** `DT/features/results/retainedPrecisionIntegration.test.tsx`, the block "`%s: the T6 panels refuse a successor by an explicit gate (U7 slice T)`" (lines 892–931 at the base; 4 tests, 2 modes). It registers an **eligible** successor through mocked IPC and asserts that both panels withhold it: the RV91 N-5 refusal. The slice replaces exactly this by ruling (decision 2; "replacing the explicit N-5 panel refusal deliberately").
- **Why it is a stop:** the fence admits only `retainedPrecisionOutputRefusal.test.tsx`'s panel expectations, and other tests "only where the reworded refusal text changes an expected string". These four need a changed expectation, not a changed string. PLAN §2.1 did not list them.
- **The proposal** (`_run_records/proposed_integration_n5.diff`, 38 changed lines; not applied in the worktree): keep the block's purpose, an explicit gate rather than a builder throw, by inverting it:
  - result export: with the builder stubbed to return a document (`t6.builderDoc`), no packet is offered for a moved model (not the live capture), and the packet is offered for the captured model;
  - stress-neutral: the live binding is closed for a moved model, whose panel shows `RETAINED_PRECISION_NATIVE_CAPTURE_REQUIRED: …`, and open for the captured model.
  - It swaps one import (`RETAINED_PRECISION_OUTPUT_REFUSAL` → `N_OUTPUT_NOT_NUMERICALLY_ELIGIBLE`). The hoisted `t6.throwless` and `t6.noRefusal` flags and their mock become unused; I left them, for ROOT to keep or remove.
  - Verified in a scratch candidate copy: the file passes **182/182** with the patch (`_run_records/suites/`).
- **Effect until ruled:** the candidate suite carries these 4 failures; nothing else depends on them.

## 5. Readings for ROOT

- **R-1. Class precedence in stress-neutral.** A row can, in principle, have both a validated class and another withholding disposition. Rust's derivative gives the class precedence over every reason, but there every disclosure is non-blocking. In the package, the class finding is `info`, so it would hide a **blocking** disposition (unknown semantic, missing semantic, contradiction). I kept blocking dispositions first (fail safe), then the class, then diagnostic work. No reader-valid input in the fixtures has such a pair, so the order is pinned by a unit test on `strictWitnessDisposition` (exported for that test only): an unknown-kind row stays a blocking `unknown_semantic` with a class, and a diagnostic-work row takes its class with Rust's message (mutants M33 and M34).
- **R-2. Successor packages read `blocked`.** The existing builder adds the blocking aggregate `SN-DECLARED-DIMENSION-WITNESS-UNAVAILABLE` whenever **any** witness is withheld, including the `info` diagnostic-work rows today (the legacy fixture's package reads `validation_ready: false` for 2 such rows). S-d uses that convention, so each pinned successor's package reads `validation_status: blocked` with 69 class findings (preview-physics-1 replay packages read `passed`). I followed the convention, which is fail safe. The alternative, excluding info-only class withholdings from the aggregate for the successor, would make a package with only classed rows read `passed`; it is a new rule for one route, so I did not take it. D2 §4.9.9 says envelope standing is not demoted by these rows; this is package validation, not standing, but ROOT may judge whether the effect is intended.
- **R-3. A stricter legacy check.** The 0.2.0 package validator's forbidden-member list gains `retained_precision` (`SN-LEGACY-PRECISION-METADATA-FORBIDDEN`). It only refuses more; no byte changes.
- **R-4. Header-only validation scope.** Without a source, the validator checks the transport (G0–G2), the new codes' severities and the existing accounting. Class membership and messages bind only with a source, because output code never parses the receipt (PLAN §4.3). The panel's decoded-payload validation passes the source.
- **R-5. The corpus two-case bases** are registered with their own invocation and a **test stand-in for the live capture**, labelled in the test. Their invocation has no `materials` member, so it cannot pass the product's IPC capture (`{model, materials: []}`), whose digest the reader binds.
- **R-6. A comment goes stale in a never-touched file.** `knownSemanticLimitations.ts` says TS "emits only this display label, never the derivative's disclosure message". The stress-neutral finding now emits that message, and T6S-4's derivative will too. For a later owner of that file.
- **R-7. Placement of the policy tests.** The fence names no test file for `outputPolicy.ts`, so the policy tests are in the new `retainedPrecisionStressNeutral.test.tsx`, under their own `describe`.
- **R-8. "The desktop lint the project's scripts define":** none exists. `P/package.json` and `P/apps/desktop/package.json` define no lint script, `software-workflow.json` registers none, and no ESLint is installed; `tsc --noEmit` is the static check (the `build` script is `tsc -b && vite build`, not run).

## 6. Checks

### 6.1 Suites (`_run_records/suites/`)

Both runs: the project's vitest (4.1.10, `vitest run --maxWorkers=6`, config `vite.config.ts`), Node v24.18.0, TypeScript 5.9.3, identical copied wasm assets (§7).

| | Files | Tests | Passed | Failed |
|---|---|---|---|---|
| Base (`git archive c1bfc460fc`) | 138 | 3,552 | 3,552 | 0 |
| Candidate (`WT/t6-outputs`) | 139 | 3,572 | 3,568 | 4 |

Test by test (`compare_base_cand3.txt`; per-test lists `base_tests.tsv`, `cand3_tests.tsv`): **20 added** (all in `retainedPrecisionStressNeutral.test.tsx`, all passing), **0 removed**, **4 status changes** (S-1's, passed → failed), **3,548 unchanged**. The two changed expectations in `retainedPrecisionOutputRefusal.test.tsx` (the stress-neutral build and panel now read the unregistered successor's standing reason; the header-only packet now dispatches to the successor and reaches the CSV profile and annotation checks) keep their tests' names and pass.

### 6.2 `tsc` (`suites/cand3_tsc.log`, `tsc_controls/`)

- Candidate `tsc --noEmit -p apps/desktop`: exit 0, no errors (`suites/cand3_tsc.log`). An intermediate run (cand2) found two typing errors in the new precedence test's row literal; I fixed them, and cand3 is the run on the final files.
- Exhaustiveness controls, in a scratch copy: T1 remove `unsupported` from `OUTPUT_POLICY` → TS2741; T2 add `"physics_retained"` to `SourceContract` → 3 errors (`OUTPUT_POLICY`, the table-path map, the test's route list); T3 drop `rule-check` from the successor entry → TS2322. T0 unmodified → 0.

### 6.3 Unchanged routes (`unchanged_routes/`)

- **Method:** identical env-gated instrumentation (`tools/instrument.py`) on a base copy and a candidate copy hashes the inputs and the JSON output of every `buildStressNeutralExportPacket`, `deriveResultDocument` and `buildCurrentResultExport` return. The full desktop suite then fed both builders every committed fixture the tests use. Instrumentation does not change results (base 3,552/3,552; candidate 3,568/3,572, as §6.1). The candidate copy holds the final seven files.
- **Result:** 182 distinct non-successor inputs in both (stress-neutral: legacy 6, precision-1 5, physics 1, preview-physics 3, source-blocks 6, physics-source 16; result JSON: 145 across the same routes); **0 differ**; 0 inputs with two outputs. Candidate-only: 3 successor packages.

### 6.4 Schema (`schema/`)

- `tools/zz_i75_sn_probe.test.tsx` (scratch only) wrote the packages of both pinned successors and `two_case_synthetic`; `tools/validate_packages.py` validated them with the repository's registry helper (`P/tests/schema_validation.py`; Python 3.13.14, jsonschema 4.26.0): **valid** under the v0.3 file and the dispatcher, matching exactly branch 7 of 8 (the successor's). The packages were dumped again from the final files: byte-identical to the first dump (`dumped_packages.sha256`).
- Negative controls (`schema_negatives.txt`): removing the receipt, the annotations, `contract_evidence` or `csv_encoding`, or adding `source_block_recovery`, makes each package invalid.

### 6.5 Controls (in `retainedPrecisionStressNeutral.test.tsx`, both modes)

| Control | Expected refusal (observed) |
|---|---|
| copied successor | `RETAINED_PRECISION_VALIDATION_REQUIRED: …` (build and panels) |
| moved model | `RETAINED_PRECISION_NATIVE_CAPTURE_REQUIRED: …` (build and panels) |
| unregistered build's bytes (the pinned file) | `RETAINED_PRECISION_VALIDATION_REQUIRED: …` |
| load-reference route | T1's `LOAD-REFERENCE-OUTPUT-NOT-YET-AVAILABLE` on every surface, build and validator |
| relabelled statement | build `SN-SOURCE-CONTRACT-UNSUPPORTED`; relabelled package header `SN-PRECISION-CONTRACT-MISMATCH` |
| receipt on another identity (a preview-physics-1 package carrying one) | with source `RETAINED_PRECISION_DOWNGRADE_FORBIDDEN`; header-only `SN-PRECISION-CONTRACT-MISMATCH`; as a 0.2.0 package `SN-LEGACY-PRECISION-METADATA-FORBIDDEN` |
| forged class code | `SN-PRECISION-WITNESS-BINDING-MISMATCH` |
| edited disclosure message | `SN-PRECISION-WITNESS-BINDING-MISMATCH` |
| wrong severity | header-only `SN-WITNESS-CATEGORY-ACCOUNTING-MISMATCH`; with source `SN-PRECISION-WITNESS-BINDING-MISMATCH` |
| unbound receipt (hash) / no receipt | with source `SN-RETAINED-PRECISION-RECEIPT-MISMATCH`; header-only the reader's `RETAINED_PRECISION_RECEIPT_MISMATCH` / `SN-PRECISION-CONTRACT-MISMATCH` |
| loss count edited | `SN-RETAINED-PRECISION-LOSS-COUNT-MISMATCH` |
| annotation edited | `SN-SOURCE-ANNOTATION-BINDING-MISMATCH` |
| non-panel surface, eligible successor | the reworded shared refusal, on all 19 |

**Multi-case** (corpus, read by id): `two_case_synthetic` is eligible and admitted (68 class findings, its per-case summary line, valid package); `two_case_facade_after_certificate_synthetic` refuses with `RETAINED_PRECISION_NOT_NUMERICALLY_ELIGIBLE: …`.

### 6.6 Mutants (`_run_records/mutants_cp1/`, `tools/mutants_cp1.py`)

Each mutant is one exact, single-occurrence replacement in a scratch copy (`S/mut`: the base plus the seven files), run against five test files (`retainedPrecisionStressNeutral`, `retainedPrecisionOutputRefusal`, `loadReferenceOutputRefusal`, `StressNeutralExportPanel`, `ResultExportPanel`; 173 tests). The unmutated control passes 173/173. The first 32 were run on the files before the precedence test was added (172 tests; a superset of assertions cannot revive them); M33 and M34 on the final files.

| ID | Mutation | Failing tests | Brief's list |
|---|---|---|---|
| M01 | the policy admits a non-panel surface (`rule-check`) | 3 | gate admits a non-panel surface |
| M02 | admission without reading standing (no live capture) | 9 | admission without the live capture |
| M03 | a route without an entry fails open | 1 | |
| M04 | a surface an entry does not name is admitted | 1 | |
| M05 | `load_reference` ungated | 22 | |
| M06 | the shared function stops refusing the successor on the 18 surfaces | 41 | |
| M07 | the stress-neutral panel shows no policy refusal | 6 | |
| M08 | the summary line swaps its two counts | 3 | |
| M09 | `e+` left unnormalized | 2 | `e+` exponent left unnormalized |
| M10 | a negative zero's sign dropped | 1 | |
| M11 | SI table maps `mm` to `mm` | 3 | |
| M12 | an unnormalized unit still claims a bound | 1 | |
| M13 | the absolute class not disclosed | 6 | a dropped disclosure |
| M14 | classes never read from the reader | 5 | a dropped disclosure |
| M15 | UTF-8 CSV policy omits the successor | 4 | |
| M16 | wrong semantic table path | 2 | |
| M17 | `contract_evidence` not copied | 7 | |
| M18 | the receipt not copied | 7 | |
| M19 | no receipt in the transport header | 10 | missing receipt in the transport header |
| M20 | transport validation skipped | 2 | |
| M21 | class finding emitted as `blocking` | 7 | wrong severity |
| M22 | validator accepts the wrong severity for the new codes | 7 | wrong severity |
| M23 | receipt equality unchecked | 2 | |
| M24 | a receipt on another identity unrefused (with a source) | 1 | |
| M25 | loss-report count unchecked | 2 | |
| M26 | class findings not counted in the loss report | 7 | |
| M27 | the builder ignores classes | 7 | a dropped disclosure |
| M28 | a valued absolute row (witness kept beside the finding) | 7 | a valued `absolute_verified` row |
| M29 | the class message replaced by a generic text | 4 | |
| M30 | the builder admits at route level only (no eligibility) | 5 | admission without the live capture |
| M31 | the validator keeps the old shared refusal | 9 | |
| M32 | a 0.2.0 package may carry a receipt | 1 | |
| M33 | a blocking disposition loses precedence to a class | 1 | |
| M34 | diagnostic work takes precedence over a class | 1 | |

All 34 are killed by assertions (`mutants_cp1/`). For result export, the brief's "valued `absolute_verified` row" and "dropped disclosure" also apply to the derivative; those mutants come with T6S-4.

## 7. Host state and cleanup

- **Wasm assets:** copied (not built) from `WT/sweep-skewpin/P/apps/desktop/public/{wasm-engine,self-weight-engine}` into the base archive and into `WT/t6-outputs/P/apps/desktop/public/` (git-ignored there). The eight files' sha256 equal I68's `ts_wasm_assets.sha256`; aggregate `95b6899f…` in both places. They stay until the final return, for T6S-4.
- **`node_modules`:** ROOT's symlink in `WT/t6-outputs/P/` is untouched. Each scratch copy has its own symlink.
- **Scratch:** `WT/scratch/i75_t6s/` holds the base archive (`base/`, without `P/execution/_Coordination/AgentRuns`, which no desktop test reads), two instrumented copies, a mutation copy and the runs. Kept for T6S-4; removed at the final return.
- **I76's files:** during my work ROOT committed them on the branch as `055ee0c0bc` (T6S-1, T6S-2: seven files, none under DT; desktop code names `results.schema.yaml` only as a string). My seven files stay uncommitted on top; their diff is against `c1bfc460fc`, which `055ee0c0bc` does not touch for them. I did not read or touch I76's files.

## 8. Commands (from `WT/t6-outputs/P/apps/desktop` unless stated; `S` = `WT/scratch/i75_t6s`)

1. Base copy: `GIT_OPTIONAL_LOCKS=0 git -C WT/t6-outputs archive c1bfc460fc -- projects/chirality-piping ':(exclude)projects/chirality-piping/execution/_Coordination/AgentRuns' | tar -x -C S/base/`; symlink `node_modules`; copy the wasm assets.
2. Suites: `../../node_modules/.bin/vitest run --maxWorkers=6 --reporter=default --reporter=json --outputFile.json=S/runs/<base|cand3>_vitest.json` (in each tree; cand1 and cand2 were earlier candidate runs, superseded by cand3 after the precedence test and a typing fix); `../../node_modules/.bin/tsc --noEmit -p .`.
3. Comparisons: `python3 tools/compare_vitest.py`, `python3 tools/compare_dumps.py`.
4. Unchanged routes: `cp -cR S/base S/cmp_base` and `S/cmp_cand` (plus the seven files); `python3 tools/instrument.py <copy>/P`; `I75_DUMP=S/dumps/<copy>.jsonl vitest run --maxWorkers=6 --reporter=dot`.
5. Schema: `I75_OUT=S/out vitest run src/features/stress-neutral/zz_i75_sn_probe.test.tsx` (in `S/cmp_cand`); `VENV/bin/python tools/validate_packages.py S/cmp_cand/P S/out`; `tools/schema_negatives.py`.
6. Proposal check: the patched integration test in `S/cmp_cand`; `vitest run src/features/results/retainedPrecisionIntegration.test.tsx`.
7. Mutants and `tsc` controls: in `S/mut` (base plus the seven files), `python3 tools/mutants_cp1.py S/mut/P S/runs/mutants_cp1 [ids]`; the `tsc` controls by one-line edits in `S/mut` with `../../node_modules/.bin/tsc --noEmit -p .`, each restored.

## 9. Next: T6S-4

On ROOT's continuation with I76's golden hashes: `deriveResultDocument` and `validateResultDocument` gain Rust `derive_document`'s successor form, reusing `retainedPrecisionDisclosure.ts`; golden byte parity for both pinned successors; the result-export controls and mutants; the result-export expectations in `retainedPrecisionOutputRefusal.test.tsx`; `buildCurrentResultExport` through the policy; and the final suites. S-1's ruling decides how the four N-5 tests end.
