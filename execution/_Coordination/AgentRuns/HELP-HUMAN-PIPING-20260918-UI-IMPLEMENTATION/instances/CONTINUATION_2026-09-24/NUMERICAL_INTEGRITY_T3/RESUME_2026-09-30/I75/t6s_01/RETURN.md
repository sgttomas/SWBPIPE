# I75 RETURN: T6S-3 (the gate split), T6S-5 (stress-neutral) and T6S-4 (result export)

TASK (Type 2), I75, for ROOT. 2026-10-06 UTC.

**Briefs:** `R/BRIEFS/T6S_COMMON.md` (sha256 `c6ac6c65…`) and `R/BRIEFS/I75_T6S_TS.md` (`812f36d3…`). **Basis:** I74's plan (`0350c918…`) and RR "I73's checkpoint 1 and I74's plan ruled; D2 5b.3; the T6 slice dispatched".
**Continuation:** RR "I75's checkpoint: S-1 granted; readings R-1 to R-7; a B8 choice prepared for the owner" (NUM `a1684ffcfe`). It verified `CHECKPOINT_1.md` (`62b00b23…`), granted S-1, ruled R-1 to R-7, and relayed I76's goldens (committed on the branch at `055ee0c0bc`) and inputs (`R/I76/t6s_01/inputs/`, read with I76's `CHECKPOINT_1.md` and `RETURN.md`).

**State.** T6S-3, T6S-4 and T6S-5 are done, with the granted S-1, R-6 and R-7 changes. Twelve desktop files are uncommitted in `WT/t6-outputs`, on top of `055ee0c0bc` (branch `codex/piping-t6-successor-outputs-20261005`); every diff below is against main `c1bfc460fc`, which `055ee0c0bc` does not change for these files. No Git writes (reads with `GIT_OPTIONAL_LOCKS=0`), no cargo, native, solver, DEC-025 or install job, no delegation, nothing in the system temp directory.

## 0. In brief

1. **Golden parity:** for both pinned successors, the TypeScript derivative's canonical JSON equals I76's Rust golden **byte for byte** (`958df02e…`, `3f9905ad…`), from the Current builder's own base and origin functions fed I76's stand-ins; their canonical bytes equal I76's four recorded inputs. The TS validator accepts both Rust goldens.
2. **The Current builder and the panel** export an eligible successor (mocked IPC, test-built hash-consistent manifest evidence, CQ-11), and refuse a copied, moved or unregistered one with the standing's code.
3. **Suites:** base 3,552/3,552 (138 files); final candidate **3,590/3,590** (141 files). Test by test: 44 added, 6 removed by renaming (S-1's four and two re-expected refusal tests), 0 status changes, 3,546 unchanged. `tsc --noEmit` clean. No lint script exists.
4. **Unchanged routes:** **186 distinct non-successor builder inputs, 0 byte differences** against the base (182 from the full suite, 4 from a probe of the new test's preview exports).
5. **Mutants:** **53 of 54 killed**; R20 (the Result Export panel's policy term removed) survives as an equivalent mutant, because the panel's own standing and capture checks and the builder's policy gate still refuse (§6). The run was interrupted once by the host and resumed (§6).
6. **Schema:** the TS successor result documents (both Current documents and the two-case base) are valid under `results.v0.3.schema.yaml` and the branch's dispatcher; the successor stress-neutral packages were valid at the checkpoint.
7. **For ROOT (§9):** the Current builder's base and origin are now two exported functions (byte-identical output); the derivative's validator now refuses a receipt on any other identity; the integration test's two `t6` seams stay declared but unused outside the granted block.

## 1. Changed files (all under DT)

| File | sha256 | Base sha256 | Lines | What |
|---|---|---|---|---|
| `features/results/outputPolicy.ts` | `712941315278fc92773366d59e34c45bf4d84e06541265402cc5bbd9ffbc9f4b` | new | 148 | the per-route, per-surface output policy (T6S-3) |
| `features/results/retainedPrecisionDisclosure.ts` | `9ee23d2678426f6c13c2c89032bb2c8ada024c1cd954dd6acb81d04d5976b8b9` | new | 89 | D-U6-2 codes and messages, `{:e}`, classes from the reader, the summary line |
| `features/results/loadReferenceOutputAvailability.ts` | `e1662f09334a12c2deaeda859693f2cf1d804c4de091b0c90e96abc6da7c30a9` | `910713da…` | 51 | the shared refusal, read from the policy; meaning unchanged |
| `features/results/knownSemanticLimitations.ts` | `07decdc36f974b02253476f88a1b01e126c5c2db82f7f25e14b1308b314eab4d` | `b0af6b5b…` | 196 | R-6: one comment sentence, comment-only (+2 −1 lines inside one `/** */`) |
| `features/result-export/ResultExportPanel.tsx` | `d362eb2bae112a0d9cb412627d53afc7efef193c31cd61d32183e7a6b2fb6da5` | `17f7a686…` | 174 | the policy gate; the summary line |
| `features/result-export/resultExportAdapter.ts` | `b2ba958adc6465b4d3adc740a5548fc7d7a90c00429d1dd16c00a4310c5cba13` | `2473cc21…` | 259 | T6S-4: the successor derivative and validator; the policy gates; base and origin extracted |
| `features/stress-neutral/StressNeutralExportPanel.tsx` | `50bd72355ccbc8a1c1ca9cfa7a1581ba44adfa9982f0b53a3f3350c29aea2e60` | `fae61807…` | 1201 | T6S-5 (unchanged since the checkpoint) |
| `features/results/retainedPrecisionOutputRefusal.test.tsx` | `4c3abb0bb46353f25572be9dbe85b41e5f76d5ebd3cac8b888a7b9b73c76df71` | `1b610c31…` | 242 | the two panels' expectations |
| `features/results/retainedPrecisionIntegration.test.tsx` | `d1bf8a5df4937b9b2ddf7454343150b23fa64c177d3b210d1fa8bd6d08125776` | `1fac2453…` | 1114 | S-1: exactly `_run_records/proposed_integration_n5.diff` |
| `features/results/outputPolicy.test.ts` | `ee51a26d2542145d6ccaa64fe93256d89c95cce93ce204e08702b8232973edcb` | new | 122 | R-7: the policy tests, moved |
| `features/stress-neutral/retainedPrecisionStressNeutral.test.tsx` | `8e9766048c1d6a0da4a28917b95ff8f6cd6711e91eb425901718d4a145f87a66` | new | 317 | T6S-5 tests, the disclosure texts, the panels' summary line |
| `features/result-export/retainedPrecisionResultExport.test.tsx` | `a171d71c65a53fd79c72df1e8413a807c2fba468d1543e0b07f59ab7634be6a1` | new | 316 | T6S-4 tests |

The complete diff is `_run_records/final/t6s_final.diff` (12 files). Nothing else was written in the worktree; the copied wasm assets are removed (§10).

## 2. T6S-4: the result JSON of a successor

**`deriveResultDocument`** gains Rust `derive_document`'s successor form:
- the route gate is the policy's route-level entry for `result-export` (a pure projection reads no standing, as in Rust); load/reference-state results keep T1's refusal;
- classes come from the accepted reader run on the bytes **without an invocation** (`retainedRowClassesFromReader`, Rust `retained_row_classes`); a statement the reader refuses throws the reader's code;
- `contract_evidence` copied, and `retained_precision` **copied whole** (D2 §4.9.7);
- each `absolute_verified` or `not_covered` row is **disclosed, not valued**: disposition `disclosed`, reason code `retained_precision_absolute_verified` or `retained_precision_not_covered`, and Rust `class_disclosure`'s exact message, with b printed as Rust's `{:e}`. No value, review entry or unit witness is emitted for it. Every other row is as before.

**`validateResultDocument`** mirrors Rust `validate_document`:
- receipt equality (`RETAINED_PRECISION_RECEIPT_BINDING_MISMATCH`), and `RETAINED_PRECISION_DOWNGRADE_FORBIDDEN` for a receipt on any other identity's document;
- `contract_evidence` equality for the successor;
- a classed row's disposition is `disclosed`; its disclosure states exactly the class code and message, and no other disclosure of a successor claims a class code (`DISCLOSURE_SEMANTICS`).

**`buildCurrentResultExport`** gates through the policy with its model, so it refuses an ineligible successor with the standing's code (`RETAINED_PRECISION_VALIDATION_REQUIRED: …`, `…NATIVE_CAPTURE_REQUIRED: …`, `…NOT_NUMERICALLY_ELIGIBLE: …`) before any other check. Its other checks are unchanged. **The document makes no standing or producer-origin claim:** no standing token, no `request` or `solver_mode` member, `request_hash: null`, no raw producer hash, and the desktop origin keeps `authentic_producer_available: false` with its existing limit text (CQ-4, CQ-5, D-U7-6).

**The base and origin, extracted.** The builder's inline base and origin literals moved, verbatim, into two exported functions, `currentResultDocumentBase(model, result, run, manifestRef, solverBasis)` and `currentReceivedOrigin(model, result, manifestRef, enriched, originLimit)`, with `CURRENT_ORIGIN_LIMIT` the builder's text. The builder calls them with exactly its former values. This lets the golden test build I76's desktop-shaped base and origin with the product's own code rather than a test copy. The output is byte-identical (§7).

## 3. Golden parity (`retainedPrecisionResultExport.test.tsx`)

For each mode:
- the pinned successor is checked by the carrier case file's sha256, and the golden file by I76's sha256;
- `base` = `currentResultDocumentBase` with I76's stand-ins (`run_id` from the result, `load_basis_refs` = `modelLoadBasisRefs(model)`, `hashes: []`, the sorted analysis status, the analysis builder's professional boundary, manifest ref `test:t6s-golden-reference-only-manifest`, solver build ref `test:t6s-golden-pinned-producer-test-bytes-not-native-attestation`); `origin` = `currentReceivedOrigin(…, enriched false, I76's test-labelled limit)`;
- `canonicalSha256Hex(base)` and `(origin)` equal I76's inputs: `5a48e00d…`/`1a0f642c…` (sparse) and `3f054be3…`/`04ee7eb9…` (dense);
- `canonicalJsonString(deriveResultDocument(base, model, source, origin))` **equals the golden file's text**, and its digest equals `958df02e…` / `3f9905ad…`; the inputs are not mutated;
- the parsed Rust golden passes `validateResultDocument`.

The parity facts hold: 98/99 rows; classes 25/69/3/1 and 25/69/3/2, no `not_covered`; 69 absolute disclosures plus the non-quantity rows' ordinary reason; 28 values; each message equals `class_disclosure`'s, with no `e+`.

## 4. The rulings applied

- **S-1:** `retainedPrecisionIntegration.test.tsx` now equals the proposal exactly (checked: the file was unchanged since the proposal, and its diff reproduces `proposed_integration_n5.diff` byte for byte). No other edit in the file. Its four tests are renamed by the patch, and pass.
- **R-2:** unchanged; every successor package still reads `blocked` through the existing aggregate. No readiness rule was added.
- **R-5:** the corpus stand-in stays labelled TEST STAND-IN in both test files.
- **R-6:** one comment sentence in `knownSemanticLimitations.ts`; the diff is two `+` and one `-` comment line inside the same `/** */` block, and no code.
- **R-7:** the six policy tests are in the new `outputPolicy.test.ts`. The panels' summary-line test stays with the panels (it renders both panels, which a `.ts` file cannot), and the stress-neutral load/reference-state builder check stays in the stress-neutral file.

## 5. Controls (T6S-4; T6S-5's are in `CHECKPOINT_1.md` §6.5)

| Control | Refusal (observed) |
|---|---|
| copied successor | `RETAINED_PRECISION_VALIDATION_REQUIRED: …` (Current builder and panel) |
| moved model | `RETAINED_PRECISION_NATIVE_CAPTURE_REQUIRED: …` (Current builder and panel) |
| unregistered build's bytes (the pinned file) | `RETAINED_PRECISION_VALIDATION_REQUIRED: …` |
| load-reference route | T1's `LOAD-REFERENCE-OUTPUT-NOT-YET-AVAILABLE` (Current builder, derivative, validator, policy) |
| relabelled statement | `SOURCE_SEMANTIC_CONTRACT_UNSUPPORTED` (derivative and validator) |
| receipt on another identity (a preview-physics-1 Current document carrying one; a base carrying one) | `RETAINED_PRECISION_DOWNGRADE_FORBIDDEN` (validator, and the derivative's own final validation) |
| receipt unbound / removed | `RETAINED_PRECISION_RECEIPT_BINDING_MISMATCH` |
| `contract_evidence` removed | `SOURCE_PHYSICAL_METADATA_MISMATCH` |
| forged class code; a non-class row claiming one | `DISCLOSURE_SEMANTICS` |
| edited disclosure message; `e-24` → `e+24` | `DISCLOSURE_SEMANTICS` |
| a classed row claimed as a value | `SOURCE_ACCOUNTING_IDENTITY` |
| a statement the reader refuses | the reader's `RETAINED_PRECISION_RECEIPT_MISMATCH` (derivative and validator) |
| an unresealed edit | `SOURCE_TARGET_VALUE` |

Edited documents are resealed (derivative hash recomputed) so that only the relational check under test can refuse.

**Multi-case** (corpus, read by id, test stand-in): `two_case_synthetic` is eligible and admitted; its derivative validates, carries the receipt, and discloses all 68 absolute rows across both cases. `two_case_facade_after_certificate_synthetic` is refused by the policy and the Current builder with `RETAINED_PRECISION_NOT_NUMERICALLY_ELIGIBLE: …`.

**The positive panel witness (CQ-11):** the panel shows `available; rows=28` (both modes) and the summary line for an eligible successor with test-built manifest evidence, and refuses a moved model with the standing's code and no download. The Current document equals the golden in its disclosures and values; only the stand-in fields differ.

## 6. Mutants (`_run_records/final/mutants/`, `tools/mutants_final.py`)

All 54 mutants (the checkpoint's M01–M34, re-run against the final test layout, and R01–R20 for T6S-4) ran in a scratch copy (`S/mut`: the base plus the twelve files and I76's two goldens), each as one exact, single-occurrence replacement, restored after its run, against ten test files (430 tests): `retainedPrecisionStressNeutral`, `outputPolicy`, `retainedPrecisionOutputRefusal`, `loadReferenceOutputRefusal`, `StressNeutralExportPanel`, `ResultExportPanel`, `retainedPrecisionResultExport`, `resultExportAdapter`, `physicsResultExport` and `retainedPrecisionIntegration`. The unmutated control passes 430/430 in both parts.

**The run was interrupted once.** The host's Claude process ended while M24 was running, leaving M24's edit in the scratch copy. On resuming I restored the copy from the worktree (checked file by file with `cmp`), kept part 1's results (control, M01–M23), and re-ran the control and M24–R20 as part 2. `mutants/mutants_final_summary.json` combines both parts; the two part logs are beside it.

| ID | Mutation (file) | Failing tests of 430 | Brief's list |
|---|---|---|---|
| M01 | gate admits a non-panel surface (rule-check) (policy) | 3 | gate admits a non-panel surface |
| M02 | admission without the live capture (standing not read) (policy) | 18 | admission without the live capture |
| M03 | a route without an entry fails open (policy) | 1 |  |
| M04 | a surface the entry does not name is admitted (policy) | 1 |  |
| M05 | load_reference route ungated (policy) | 24 |  |
| M06 | the shared function stops refusing the successor on the eighteen surfaces (shared refusal) | 39 |  |
| M07 | stress-neutral panel shows no policy refusal (stress-neutral) | 8 |  |
| M08 | summary line swaps the two counts (disclosure) | 5 |  |
| M09 | e+ exponent left unnormalized (disclosure) | 2 | e+ left unnormalized |
| M10 | negative-zero sign dropped (disclosure) | 1 |  |
| M11 | SI unit table maps mm to mm (disclosure) | 9 |  |
| M12 | an unnormalized unit still names a bound (disclosure) | 1 |  |
| M13 | a dropped disclosure (absolute class not disclosed) (disclosure) | 17 | dropped disclosure |
| M14 | classes never read from the reader (disclosure) | 16 | dropped disclosure |
| M15 | UTF-8 CSV policy omits the successor (stress-neutral) | 4 |  |
| M16 | successor semantic table path wrong (stress-neutral) | 2 |  |
| M17 | contract_evidence not copied for the successor (stress-neutral) | 7 |  |
| M18 | receipt not copied into the package (stress-neutral) | 7 |  |
| M19 | missing receipt in the transport header (stress-neutral) | 10 | missing receipt in the transport header |
| M20 | transport validation skipped for the successor (stress-neutral) | 2 |  |
| M21 | withheld-witness code emitted with the wrong severity (stress-neutral) | 7 | wrong severity |
| M22 | validator accepts the wrong severity for the new codes (stress-neutral) | 7 | wrong severity |
| M23 | receipt equality not checked against the source (stress-neutral) | 2 |  |
| M24 | a receipt on another identity not refused (with a source) (stress-neutral) | 1 |  |
| M25 | loss-report class count not checked (stress-neutral) | 2 |  |
| M26 | class findings not counted in the loss report (stress-neutral) | 7 |  |
| M27 | builder ignores classes (no class findings) (stress-neutral) | 7 | dropped disclosure |
| M28 | a valued absolute row (witness kept alongside the finding) (stress-neutral) | 7 | valued absolute row |
| M29 | class finding message replaced by a generic text (stress-neutral) | 4 |  |
| M30 | builder admits without eligibility (route level only) (stress-neutral) | 5 | admission without the live capture |
| M31 | validator keeps the old shared refusal (stress-neutral) | 9 |  |
| M32 | legacy 0.2.0 packet may carry a receipt (stress-neutral) | 1 |  |
| M33 | a blocking disposition loses precedence to a class (fail-safe order) (stress-neutral) | 1 |  |
| M34 | diagnostic work takes precedence over a class (stress-neutral) | 1 |  |
| R01 | derivative: a dropped disclosure (classes ignored per row) (result export) | 13 | dropped disclosure |
| R02 | derivative: a valued absolute_verified row (disposition ignores the class) (result export) | 13 | valued absolute row |
| R03 | derivative: the class row keeps the ordinary reason code (result export) | 13 |  |
| R04 | derivative: the class row keeps the ordinary message (result export) | 13 |  |
| R05 | derivative: the receipt not copied (result export) | 13 |  |
| R06 | derivative: contract_evidence not copied for the successor (result export) | 13 |  |
| R07 | derivative: classes never read from the reader (result export) | 13 | dropped disclosure |
| R08 | validator: receipt equality unchecked (result export) | 4 |  |
| R09 | validator: a receipt on another identity unrefused (result export) | 2 |  |
| R10 | validator: class code and message unchecked (result export) | 2 |  |
| R11 | validator: a non-class disclosure may claim a class code (result export) | 2 |  |
| R12 | validator: the class disposition ignored (result export) | 13 |  |
| R13 | validator: classes never read from the reader (result export) | 13 | dropped disclosure |
| R14 | Current builder: admission at route level only (no live capture) (result export) | 7 | admission without the live capture |
| R15 | Current builder: the policy gate removed (result export) | 10 |  |
| R16 | origin: the limit argument ignored (result export) | 4 |  |
| R17 | derivative: the successor still refused (the old shared refusal) (result export) | 13 |  |
| R18 | derivative: the route gate removed (load/reference admitted) (result export) | 3 |  |
| R19 | validator: the route gate removed (result export) | 3 |  |
| R20 | result-export panel: the policy gate removed (result-export panel) | **0: survives (equivalent; see below)** |  |

**53 of 54 are killed by assertions.** **R20 survives, as expected: it is equivalent.** It removes the policy term from the Result Export panel's own live binding; that binding still requires numerically eligible standing (the same condition as the policy's admission for the successor) and the live native capture, and the Current builder's policy gate (R15, killed) still refuses a load/reference-state result before any packet exists. The panel term is defence in depth, kept so that the panel reads its own policy entry explicitly (RR decision 2).

## 7. Unchanged routes (`_run_records/final/unchanged_routes/`)

- The checkpoint's identical env-gated instrumentation of `buildStressNeutralExportPacket`, `deriveResultDocument` and `buildCurrentResultExport`, run with the full desktop suite on the base copy and on a candidate copy holding the final twelve files and I76's two goldens (the desktop suite reads no other `055ee0c0bc` file).
- **182 distinct non-successor inputs in both, 0 differ**, none nondeterministic (`compare_final.txt`).
- The candidate's other 16 inputs: 12 successor, and 4 preview-physics-1 Current exports built by the new downgrade control with a new solver build ref. A probe running exactly those 4 on both copies finds **0 differences** (`compare_preview_probe.txt`).

## 8. Suites (`_run_records/final/suites/`)

Base: the `git archive c1bfc460fc` copy. Candidate: `WT/t6-outputs` (with `055ee0c0bc`). vitest 4.1.10 (`--maxWorkers=6`), Node v24.18.0, TypeScript 5.9.3; identical copied wasm assets (hashes in `_run_records/ts_wasm_assets.sha256`, equal to I68's).

| | Files | Tests | Passed | Failed |
|---|---|---|---|---|
| Base | 138 | 3,552 | 3,552 | 0 |
| Final candidate | 141 | 3,590 | 3,590 | 0 |

Test by test (`compare_base_final.txt`, `final_tests.tsv`, and the checkpoint's `base_tests.tsv`):
- **added 44:** `retainedPrecisionResultExport` 17, `retainedPrecisionStressNeutral` 15, `outputPolicy` 6, and 6 renamed: S-1's four (`…gate a successor by an explicit policy entry (U7 slice T; T6S-3)…`) and the two re-expected result-export refusal tests (`result export: the Current build and the panel refuse; the pure projection and its validator read no standing`);
- **removed 6:** the same six under their old names;
- **0 status changes; 3,546 unchanged.**
- The re-expected result-export test: the Current build and the panel now refuse the unregistered successor with the standing's code, and the pure projection admits it (as Rust does) with the receipt copied; the validator refuses a document without the receipt (`RETAINED_PRECISION_RECEIPT_BINDING_MISMATCH`).

`tsc --noEmit -p apps/desktop`: exit 0 (`final_tsc.log`). Lint: none defined (checkpoint R-8).

## 9. For ROOT

- **a. The extracted base and origin (§2).** A structural change to `buildCurrentResultExport`: the literals moved verbatim into two exported functions; the output is byte-identical (§7: the suite's 24 Current exports and the probe's 2). Exported so that the goldens pin the product's own base and origin. RV101 may prefer them unexported, in which case the golden test would carry a copy of the literals instead.
- **b. A stricter derivative validator.** `validateResultDocument` now refuses a `retained_precision` member on any non-successor document (`RETAINED_PRECISION_DOWNGRADE_FORBIDDEN`), as Rust's does. It only refuses more; no committed document carries one (the 182 non-successor exports are unchanged).
- **c. The pure projection reads no standing,** as in Rust: `deriveResultDocument` and `validateResultDocument` admit a reader-valid successor whatever its standing. The only product caller is `buildCurrentResultExport`, which requires eligible standing with the live capture through the policy.
- **d. The integration test's `t6.throwless` and `t6.noRefusal`** stay declared, with their mock, outside the granted block, and are now unused. A later cleanup may remove them.
- **e. Unchanged from the checkpoint:** R-2's owner choice for B8; Python keeps refusing successor packages (decision 8); the native panel witness and the product-flow export of a desktop-shaped successor are B8's (decision 10, CQ-11).

## 10. Host state and cleanup

- **Removed at return:** the copied wasm assets in `WT/t6-outputs/P/apps/desktop/public/` (the folder, which I created, is gone; it was Git-ignored), and `WT/scratch/i75_t6s/` (the base archive, the instrumented and mutation copies, the dumps and logs). Everything needed to check the claims is in `_run_records/`.
- **Left for ROOT:** the `node_modules` symlink in `WT/t6-outputs/P/` (untouched).
- **I76's commit `055ee0c0bc`** is on the branch; I did not touch its files.

## 11. Commands (from `P/apps/desktop` unless stated; `S` = `WT/scratch/i75_t6s`)

1. Suites: `../../node_modules/.bin/vitest run --maxWorkers=6 --reporter=default --reporter=json --outputFile.json=S/runs/<base|final>_vitest.json`; `../../node_modules/.bin/tsc --noEmit -p .`.
2. Unchanged routes: `python3 tools/instrument.py <copy>/P`; `I75_DUMP=… vitest run --maxWorkers=6 --reporter=dot`; `python3 tools/compare_dumps.py`; the preview probe `tools/zz_i75_preview_rc_probe.test.ts` placed in each copy and removed.
3. Schema: `tools/zz_i75_rd_probe.test.ts` in the candidate copy writes the documents; `VENV/bin/python tools/validate_result_documents.py WT/t6-outputs/P S/out3`.
4. Mutants: `python3 tools/mutants_final.py S/mut/P S/runs/mutants_final` (the base copy plus the twelve files and I76's goldens); after the interruption, part 2 with the id list `M24,…,M34,R01,…,R20` as the third argument, into `S/runs/mutants_final_part2`.

## 12. Records (`R/I75/t6s_01/`)

- `CHECKPOINT_1.md` and its `_run_records/` (unchanged).
- `RETURN.md` (this file).
- `_run_records/final/`: `t6s_final.diff`; `suites/`; `unchanged_routes/`; `schema/`; `mutants/`; `tools/`.
- `SHA256SUMS` covers every file except itself. Paths are placeholders; a grep for home-directory and host-temp prefixes finds none.
