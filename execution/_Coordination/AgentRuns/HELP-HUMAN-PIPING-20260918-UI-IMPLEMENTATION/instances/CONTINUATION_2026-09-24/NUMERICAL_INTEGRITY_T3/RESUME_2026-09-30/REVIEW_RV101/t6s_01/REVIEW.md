# RV101: independent review of the T6 successor-output slice (T6S)

**Reviewer:** RV101, TASK (Type 2), dispatched by ROOT (HELP_HUMAN, Agent 0). ROOT is the return path. No descendants were launched. I wrote none of the work under review, and I am not an F2a or reader reviewer of record. 2026-10-06 UTC.

**Placeholders:** `WT` is the T3 host root; `NUM = WT/numerics`; `P = projects/chirality-piping`; `RE = P/core/reporting/result_export`; `DT = P/apps/desktop/src`; `PP = P/core/product_physics`; `T` is the T3 records folder; `R = T/RESUME_2026-09-30`; `RR = T/ROOT_RULINGS_V1.md`; D2 is `T/DESIGN_STANDING/DESIGN.md`; `VENV` is the piping virtual environment; `NMS` is the shared `node_modules`. No machine paths are recorded here or in `evidence/`.

**Brief and basis read** (sha256 prefixes):
- `NUM/AGENTS.md` (`f96feb19…`), `NUM/agents/AGENT_TASK.md` (`1a13a5b0…`), `NUM/P/AGENTS.md` (`d9f2b23a…`);
- `R/BRIEFS/T6S_COMMON.md` (`c6ac6c65…`) and `R/BRIEFS/RV101_T6S_REVIEW.md` (`7aaf14cf…`), with ROOT's dispatch text and ROOT's two later messages (the lock rule for heavy test jobs; the resumption after a network stop);
- I74's `R/I74/t6_slice_plan_01/PLAN.md` (`0350c918…`, in full);
- RR (read at `f747e37e…`): "I73's checkpoint 1 and I74's plan ruled; …" (decisions 1–14), "I76's checkpoint: …", "Owner decision: G10 is redefined; …", "I76's return verified; …", "I75's checkpoint: S-1 granted; readings R-1 to R-7; …" and "T6S complete; RV101 dispatched";
- D2 (`993f5f3a…`) §4.9.4, §4.9.7 and §4.9.9;
- the accounts: `R/I75/t6s_01/CHECKPOINT_1.md` (`62b00b23…`), `RETURN.md` (`2ccafccb…`) and `_run_records/proposed_integration_n5.diff` (`8b773e10…`); `R/I76/t6s_01/CHECKPOINT_1.md` (`a260fc5c…`), `RETURN.md` (`d1d73597…`) and `inputs/` (four files, hashes equal to I76's);
- code: every changed file in full; `RE/src/derivative.rs` (whole), `RE/src/source_blocks.rs` (`integer`, every call site, `validate`'s order), `RE/src/semantic_contract.rs` (`retained_row_classes`); `DT/services/previewService.ts` (capture and registration); `DT/features/results/retainedPrecisionStanding.ts`; `numericalResultQuality.ts` (`sourceContract`, standing); `reportPackageRequest.ts`; `LoadReferenceOutputGate.tsx` and its 18 call sites; `PP/src/lib.rs` (the ordinary wrapper); `P/apps/desktop/src-tauri/src/lib.rs` (the native solve commands).

**Candidate:** `c1bfc460fc..2033260c57` on `codex/piping-t6-successor-outputs-20261005`: `055ee0c0bc` (I76: 7 files) and `2033260c57` (I75: 12 files). 19 files, +2,017 / −1,903. Read with `GIT_OPTIONAL_LOCKS=0`; nothing written to `WT/t6-outputs`.

**Independence.** I built my own oracles; the implementers' tests appear here only as subjects of mutation:
- **A Rust oracle** (`evidence/tools/zz_rv101_oracle.rs`, placed only in my archive copy): Rust `derive_document` (request none) and `validate_document` over inputs written by my TypeScript driver; Rust `{:e}` over word lists; Rust `class_disclosure` over a combination list.
- **A TypeScript successor oracle** (`zz_rv101_successor_oracle.test.ts`): the golden inputs; the whole corpus (15 bases, 24 must-pass and 278 refusal entries, rebuilt with the corpus's edit-and-rehash rule) through the product's base and origin functions; 60,000 binary64 words plus 21,332 targeted ties; 990 disclosure combinations; closure probes through the product's own IPC capture; stress-neutral packages.
- **A fixture-driven differential** (`zz_rv101_differential.test.ts`), the same file in base and candidate copies, over every committed MechanicsResult-shaped JSON in the tree (71; 69 non-successor), with my own base and origin literal.
- **A dispatcher oracle** (`dispatcher_oracle.py`) with its own local registry, over every committed result document anywhere in the candidate tree (read from Git), my own derived documents, and 693 mutations of my own.
- **A committed-package validation differential**, a stress-neutral schema check and negative probes, `tsc` exhaustiveness controls, and 13 mutant edits of my own beside both implementers' sets.

## Verdict: **PASS**

| | Count |
|---|---|
| BLOCKING | 0 |
| SHOULD-FIX | 1 |
| NOTE | 10 |

**The headline:**
- **Closure holds.** In every probe a successor is admitted only by the two panels, and only at `numerically_eligible` standing with the live native capture. Copies, JSON round trips, a moved or null model, a registration without the live capture, and bytes delivered under the other solver mode are all refused with the standing's own code. All 19 other policy surfaces and the shared function return the reworded refusal, and the report package returns `REPORT-PACKAGE-FRESH-RESULT-UNAVAILABLE`, even for an eligible successor. No product caller can deliver a successor before B8: the native commands call PP's ordinary wrapper, which never selects a retained profile.
- **Contract faithfulness holds, except for the bound's text on exact ties (SF-1).** My own Rust harness re-derives both goldens byte for byte from I76's inputs. The product's own base and origin functions reproduce those inputs byte for byte, and TypeScript derives the same bytes. Over 63 reader-valid successor statements (2 pinned successors, 15 corpus bases, 24 must-pass entries, 22 invocation-dependent refusal entries), the TypeScript and Rust derivatives are byte-identical, and Rust's validator accepts every TypeScript document. The disclosures, the receipt, `contract_evidence` and S-d's findings all check. No export carries a standing token, an invocation or a producer-origin claim.
- **SF-1: `rustLowerExp` differs from Rust's `{:e}` on exact 17-digit decimal ties.** 9 of 60,000 random words differ, and 5,262 of 21,332 targeted ties. V8 rounds the tie to even; Rust rounds it up. No bound below 2^-25 can tie, so no real receipt is affected. But decision 4's parity claim is not exact, and the committed edge oracle (Python `repr`) shares JavaScript's tie rule. The suggested one-line repair matches Rust on all 81,332 words.
- **Nothing weakened, nothing outside the fence.** The 19 files are exactly the fence plus the granted S-1, R-6 and R-7. My differential over the 69 committed non-successor fixtures gives identical outcomes in all 891 steps (267 byte outputs, among them 100 derivatives, 30 Current result exports and 82 stress-neutral packages). The 64 committed stress-neutral packages anywhere in the tree get identical verdicts from both validators, and none carries a receipt (R-3).
- **The dispatcher is correct.** On all 24 committed 0.3.0 documents in the whole tree (8 of them outside I76's sweep roots) it agrees with the version file. All 651 of my mutations that the version file refuses, the dispatcher refuses too. It also agrees with the base dispatcher on all 354 committed 0.1.0 and 0.2.0 documents.
- **N-5 is argued correctly.** Every `integer` read follows the receipt-shape check and both checked-profile hashes. S1 survives the whole suite (176/176), as I76 found. My mutant removing the publication-hash check is killed by I76's test. `RE/src/` is unchanged.
- **Mutants.** I76's 14 reproduce exactly: 12 killed; S1 survives both runs, as designed. I75's 54 reproduce exactly: 53 killed with I75's failing-test counts; R20 survives and is equivalent. Of my 13 edits, 10 are killed and 3 survive: RV-T2 (NT-7), RV-T6 (NT-10), and RV-R1 against the golden test only, since the carriers test kills the same edit.

## Findings

| # | Sev | Where | Evidence | Remedy |
|---|---|---|---|---|
| SF-1 | SHOULD-FIX | `DT/features/results/retainedPrecisionDisclosure.ts` `rustLowerExp`; its edge tests in `retainedPrecisionStressNeutral.test.tsx` | **Exact ties at 17 significant digits print differently from Rust `{:e}`.** Over 60,000 binary64 words (every binade, 28,844 subnormals, every corpus bound), 59,991 print identically and 9 differ. Each difference is an exact decimal tie (`evidence/oracle/exp_differences.txt`). Example: `43180467b3a7ed6d` is exactly 1690060720831323.25; Rust prints `1.6900607208313233e15`, `rustLowerExp` prints `1.6900607208313232e15`. Over 21,332 targeted tie words, 5,262 differ (every tie whose lower candidate digit is even; `exp_ties.txt`). V8's `toExponential()` and Python `repr` round the tie to even; Rust rounds it up. The doc comment says `toExponential()` "gives the same digits". The edge expectations were computed with Python `repr`, which shares V8's rule, so no committed test can see this. **Reach:** a tie needs an exact decimal expansion of 18 significant digits, so no b below 2^-25 (about 2.98e-8) can tie. A successor bound that large needs a scale S* of about 5.5e11 in SI units, and no committed or corpus bound comes near. But a Rust-produced document with such a b would fail the TS validator (`DISCLOSURE_SEMANTICS`), and the converse; that breaks decision 4's byte parity. | When the shortest form has 17 significant digits, print `Math.abs(v).toExponential(16)` instead. On a tie, ECMAScript's `toExponential(f)` picks the larger candidate, which is Rust's rule, and ties occur only at 17 digits. This repair matches Rust on all 81,332 words (`exp_repair_check.json`, `exp_ties.txt`). Add tie vectors with Rust-computed expectations, and correct the comment. Or, if ROOT prefers, bound the domain: refuse b ≥ 2^-25 with a named code and document it. |
| NT-1 | NOTE | `RE/tests/source_blocks.rs`, N-5's test | **The test pins 12 of the 13 receipt fields that `integer` reads, and only the non-composite path.** `failure.block_order` (`source_blocks.rs:969`, schema maximum 2^53−1) is not in its lists, and the composite physics-source receipt is not exercised. The public-API argument covers both all the same: both checked-profile hashes run before any `integer` call. My RV-R3 (the publication-hash check removed) is killed, so the hash layer is pinned. | For PR-B1's direct unit test: include `block_order`, beside I76's item (b) field list. |
| NT-2 | NOTE | `P/tests/test_results_dispatcher_v0_3.py` `committed_documents` | **"Every committed v0.3 document" covers only five roots.** My Git-wide sweep finds 24 committed 0.3.0 documents. 8 of them are physics-1 and precision-1 headless captures under `P/execution/`, outside the test's roots. All 24 get the same verdict (admitted) from the dispatcher and the version file. | None needed. Optionally, name the roots in the test's docstring. |
| NT-3 | NOTE | `P/fixtures/product_preview/{preview_physics,precision}_fixture_generation.json` | **I76 (c), confirmed.** Both records list `schemas/results.schema.yaml` at the old `9f2adf6a…` in their source-input inventories. They are historical generation records. At base they already list 6 schema hashes that differ from the tree (the v0.3 results, AnalysisRun and stress-neutral files). The only test that reads them checks their `outputs`, not these entries. | None. GEN-8 should be aware of them. |
| NT-4 | NOTE | `DT/features/results/retainedPrecisionIntegration.test.tsx` lines 46–61 | **I75 (d), confirmed.** The hoisted `t6.throwless` and `t6.noRefusal` flags and their mock of the shared function remain, now never set. With both flags false, the mock delegates to the real function. | T6's later cleanup, as ruled. |
| NT-5 | NOTE | `resultExportAdapter.ts` `currentResultDocumentBase`, `currentReceivedOrigin`, `CURRENT_ORIGIN_LIMIT` | **I75 (a): acceptable.** The extraction is output-neutral: all 30 Current exports in my differential are byte-identical to base. The two functions reproduce I76's inputs byte for byte, so the goldens pin product code. They add no capability: `deriveResultDocument` was already exported and accepts any origin, and only the Current builder qualifies. | None. |
| NT-6 | NOTE | `StressNeutralExportPanel.tsx` | **R-2 verified.** Every successor package reads `validation_status: blocked`, through the existing blocking aggregate `SN-DECLARED-DIMENSION-WITNESS-UNAVAILABLE`. The diff does not touch the aggregate, `blockingCount` or the readiness fields, so no route-specific readiness rule was added. | Carried: the owner's choice at B8, as ruled. |
| NT-7 | NOTE | The `not_covered` paths (derivative text; the package's withholding) | **No validated statement has a `not_covered` row, so the goldens and the package tests cannot exercise this class.** My RV-R1 (Rust's `not_covered` text shortened) survives I76's golden test. Its literal is pinned on both sides all the same: `retained_precision_carriers.rs:633` kills the same edit (RV-R1b), and I75's unit test kills RV-T4. My RV-T2 survives everything: the header-only validator expects `blocking` for a `not_covered` withholding, so it would refuse a correct package. That is fail-safe. My TS↔Rust `class_disclosure` comparison over 990 combinations found no difference. | Optional: one package-level test with a classed `not_covered` row (synthetic class map), and a shared vector read by both languages. |
| NT-8 | NOTE | `P/tests/test_result_export_v0_2.py` `validator()` | **I76 (a): acceptable.** The 8 mirrored lines build the same registry as `tests/schema_validation.py`: all resources local, no network retrieval. Without them, the dispatcher's external `$ref` would fall back to network retrieval. | Optionally, a later helper export, as I76 proposes. |
| NT-9 | NOTE | `RE/src/derivative.rs` `class_disclosure` (unchanged) and its TS mirror | **The disclosure's decimal b is a round-trip decimal, not an upward bound.** b itself is `fl↑(2^-64·S*)`. Shortest round-trip digits can lie below that binary64 value in the last place. The message also carries the exact word (`binary64 …`), so nothing is lost. D2's UI notice uses an upward three-digit form (`upwardBoundText`). This is D-U6-2's text, mirrored by ruling, not a slice defect. | For S-I2's rewording of the shared texts (PLAN §4.3): consider saying that the hex word is the bound. |
| NT-10 | NOTE | `outputPolicy.ts` `surfaceOutputRefusal` | **The fail-closed `catch` around standing is unpinned.** My RV-T6 (a throwing standing reads eligible) survives every test. On reachable inputs it is equivalent: for the only admitted route, standing reads a registration and fingerprints, and neither throws on JSON-parsed IPC data. | Optional: one unit test with a standing that throws. |

**SF-1 is cheap, TS-only and testable.** I recommend repairing it before the PR, and I will confirm the repair.

## 1. Item 1: closure

### 1.1 Where a successor can leave the desktop

- **The 18 surfaces.** The same 18 panels call `LoadReferenceOutputGate` at base and candidate: pcf-export, caepipe-mbf, caepipe-external, export-adapter-sdk, adapter-framework, external-prover, missing-data, design-workspace, rule-check (`RuleCheckPanel.tsx:54`), report-lint, solve-job, headless-runner, local-fea, native-package, handoff, export-review, report and rendered-report. None of their files changed.
  - The gate calls `loadReferenceOutputRefusal`, which now returns `routeOutputRefusal`.
  - `sourceContract` returns only members of `SourceContract`, and every member has an entry. So the fail-closed branch (`OUTPUT-ROUTE-NOT-REGISTERED`) is unreachable at runtime, and the function's meaning is unchanged apart from decision 12's text.
  - My differential confirms this: the shared refusal is identical for all 69 committed non-successor fixtures and differs only by the reworded text for the two successors.
- **The report package** refuses through `reportPackageUnavailableReason` (an unchanged file). `isFreshSemanticResult` includes the successor id, so it returns the fresh-result refusal in every probe, the eligible one included.
- **The Rule-check panel** is one of the 18 and refuses with the reworded text. Its policy entry is `refused`; checklist item 1 will open it by its own entry.
- **Other downloads.** 36 desktop files build downloads. 14 panels outside the policy have download controls, but only `ReviewGeometryPanel` receives a `MechanicsResult`, and its export carries only the run reference, not values. These files are unchanged by the slice.
- **Callers of the new admission.**
  - `deriveResultDocument` is called only by `buildCurrentResultExport`.
  - `buildStressNeutralExportPacket` is called only by the stress-neutral panel.
  - `validateResultDocument` is called only within the adapter.

### 1.2 The policy is exhaustive (`evidence/tsc_controls/`)

These are my own `tsc --noEmit` controls in a scratch copy. Unmodified, it reports 0 errors. Each of these fails to compile:
- a new route in `SourceContract` (3 errors: `OUTPUT_POLICY`, the stress-neutral table map and the policy test);
- a successor entry that leaves `rule-check` unnamed;
- a new member of `OUTPUT_SURFACES`;
- a removed route entry;
- a surface decision outside the two values.

### 1.3 Closure probes through the product's own capture (`evidence/oracle/facts.jsonl`, kind `closure`)

Both pinned successors were delivered through mocked IPC with their pinned request models, so that `previewService.validateCapturedSource` registers them with its live-capture callback. Each row holds for both modes.

| Probe | Standing finding | Result export and stress-neutral | 19 other surfaces and the shared function | Report package |
|---|---|---|---|---|
| file bytes, unregistered | `VALIDATION_REQUIRED` | refused, with that code | reworded refusal | fresh-result refusal |
| registered through IPC, captured model | eligible | **admitted** | reworded refusal | fresh-result refusal |
| `structuredClone` of the registered result | `VALIDATION_REQUIRED` | refused | reworded refusal | fresh-result refusal |
| JSON round trip of it | `VALIDATION_REQUIRED` | refused | reworded refusal | fresh-result refusal |
| registered, moved model | `NATIVE_CAPTURE_REQUIRED` | refused | reworded refusal | fresh-result refusal |
| registered, null model | `NOT_NUMERICALLY_ELIGIBLE` | refused | reworded refusal | fresh-result refusal |
| registered without the live capture (`live = null`) | `NATIVE_CAPTURE_REQUIRED` | refused | reworded refusal | fresh-result refusal |
| bytes delivered under the other solver mode | the reader's `INVOCATION_MISMATCH` | refused | reworded refusal | fresh-result refusal |

The corpus's `two_case_synthetic`, registered with a labelled test stand-in for the live capture, is admitted (68 class findings; the package validates). `two_case_facade_after_certificate_synthetic` is refused with `NOT_NUMERICALLY_ELIGIBLE`.

### 1.4 Dormant until B8 (decision 3)

- **Registration.** `registerRetainedPrecision` has one product caller, `validateCapturedSource`. It is reached only after actual IPC (`__TAURI_INTERNALS__` present) or a completed known job. It keys a `WeakMap` by object identity and a checked-JSON fingerprint, so saved, loaded, copied or edited bytes never register (the probes above).
- **The native commands.** `run_preview_mechanics_with_solver_mode` and the job command call `PP` `run_linear_static_preview_value_with_mode`. That function passes `retained_entry: None`, so `ordinary_dispatch` runs and no `RetainedPublication::Successor` can be produced.
- **So, without a product caller,** the opened panels are reachable only by tests, as ruled.

### 1.5 The panel's own policy term (I75's R20)

R20 is equivalent, as I75 says. `ResultExportPanel`'s binding also requires `numericalResultStanding(result, model).eligible` and `hasNativeMechanicsInvocation(…, solver_mode)`. The Current builder's policy gate refuses a load/reference result before any document exists.

## 2. Item 2: contract faithfulness

### 2.1 The goldens, re-derived (`evidence/oracle/`)

- **Inputs.** The product's exported `currentResultDocumentBase` and `currentReceivedOrigin`, fed I76's stand-ins, reproduce I76's four recorded input files byte for byte (canonical JSON, both modes).
- **Rust.** My own integration test calls `derivative::derive_document(base, model, source, origin, None)` with the pinned request model. Its canonical JSON equals both committed goldens byte for byte (`958df02e…`, `3f9905ad…`).
- **TypeScript.** `canonicalJsonString(deriveResultDocument(…))` on the same inputs equals both files, with no trailing newline.

### 2.2 Byte parity beyond the goldens (`evidence/oracle/parity_summary.txt`)

Every corpus statement was derived in both languages through the product's base and origin functions (test-labelled stand-ins):

| Inputs | Outcome |
|---|---|
| 2 pinned successors, 15 corpus bases, 24 must-pass entries, and 22 refusal entries that are valid without an invocation | **63 of 63 byte-identical.** Rust `validate_document` accepts every TypeScript document, and TypeScript `validateResultDocument` accepts each one |
| the other 256 refusal entries | refused by both. Codes are equal in 253. Of the other 3: one is the corpus's declared reader difference (`g7_maximum_off_enclosure`); in two, the TypeScript derivative's route check runs before the reader (`SOURCE_SEMANTIC_CONTRACT_UNSUPPORTED` against Rust's reader code). Both orders predate the slice |

My TypeScript input writer drops `-0` (JSON text), which made one entry look one-sided at first. With `-0` restored, Rust refuses all four affected entries with TypeScript's own codes (`negative_zero_inputs.txt`).

### 2.3 Disclosure, claims and the bound's text

- **Every `absolute_verified` and `not_covered` row is disclosed, never valued or unlabelled.** Each golden has 69 `retained_precision_absolute_verified` disclosures (units mm, N, N*m, MPa and Pa). None appears among the values or has a witness. Each golden has 28 values and 28 witnesses, and the non-quantity rows keep their ordinary reasons. No validated statement has a `not_covered` row. So I compared TS `retainedClassDisclosure` with Rust `class_disclosure` directly over 990 combinations:
  - 6 kinds, including Unicode, quotes and an empty kind;
  - 15 units, including unnormalized ones;
  - every class, and 6 bound words.

  They are identical, the `not_covered` text and the unnormalized-unit fallback included.
- **No producer-origin claim, standing token or invocation.** This holds for both goldens and for the stress-neutral packages:
  - no `request` or `solver_mode` member at any depth;
  - no `numerically_eligible` or `needs_recompute` text;
  - `authentic_producer_available: false`, `original_producer_checksum: null`, `request_hash: null` and `raw_source_hashes: []`.

  The receipt and `contract_evidence` equal the source's.
- **b as Rust's `{:e}`:** see SF-1. All 60,000 words except 9 exact ties agree, subnormals and `-0` included. No disclosure contains `e+`.

### 2.4 Stress-neutral S-d (`evidence/stress_neutral/`)

Both pinned successors were registered through the product's capture:
- **Findings.** Each package has 69 `SN-UNIT-WITNESS-WITHHELD-RETAINED-PRECISION-ABSOLUTE-VERIFIED` findings, severity `info`, class `unit_preservation_witness`. Their rows are exactly the golden's class disclosures, and **all 69 messages equal Rust's derivative messages for the same rows**. No class row has a witness, and every CSV value and unit equals the source's.
- **The receipt** travels whole and, with `contract_evidence`, equals the source's. The loss report counts "69 … 0 … uncovered".
- **The transport header** carries the receipt. Header-only and with-source validation both pass.
- **Schema validity, with no schema change.** Both packages, and the two-case package, are valid under `stress_neutral_export.v0.3.schema.json` and its dispatcher, and match only `oneOf` branch 7 of 8. Removing the receipt, the annotations, `contract_evidence` or `csv_encoding`, or adding a member to the receipt, makes each invalid under both schemas. The stress-neutral and AnalysisRun schemas are unchanged.
- **Negative probes.** Each of these edits is refused, with and without the source:
  - an edited receipt body, resealed or not;
  - a removed or null receipt;
  - a finding's severity, code or message edited;
  - a finding removed;
  - `contract_evidence` removed;
  - the identity relabelled.

  Header-only refusals come from the member checksums or the contract dispatch (R-4's scope).
- **R-2:** see NT-6.

## 3. Item 3: nothing weakened, nothing outside the fence

### 3.1 The fence (`evidence/fence/`)

The 19 changed files and their candidate hashes equal I75's and I76's returns:
- **I76's seven:** the dispatcher; `test_result_export_v0_2.py` (validator construction only); the new dispatcher test; the new golden test; `RE/tests/source_blocks.rs` (one appended test); the two goldens.
- **I75's nine fenced files, and the three granted:** S-1 `retainedPrecisionIntegration.test.tsx`, R-6 `knownSemanticLimitations.ts` and R-7 `outputPolicy.test.ts`.
- **S-1:** the file's hunks equal `proposed_integration_n5.diff` exactly; only Git's hunk-header context text differs. They are one import line and the U7 slice T block.
- **R-6:** +2 / −1 lines, all inside one `/** */` comment; no code.
- **Untouched:** everything under `RE/src/` and `PP/`; every D1 crate `src/`; every embedded static (the version file `results.v0.3.schema.yaml` is byte-identical); the readers and carriers; `src-tauri`; `e2e_plan.py`; U8's and S-I1's files; the carrier case file; every lock file.

### 3.2 No refusal assertion loosened

- `retainedPrecisionOutputRefusal.test.tsx`'s hunks lie in its header comment, its imports and the one `describe` for the two panels. The 18-surface table and the report-package test are unchanged.
- The two re-expected tests now assert the standing's reason (`VALIDATION_REQUIRED`) for unregistered bytes, and keep their no-download checks.
- The integration block keeps a negative (a moved model).
- No other desktop test changed.

### 3.3 Suites (`evidence/suites/`)

| Suite | Base `c1bfc460fc` | Candidate `2033260c57` |
|---|---|---|
| desktop vitest, whole | 3,552 / 3,552, 138 files | 3,590 / 3,590, 141 files: 44 added, 6 removed by renaming (S-1's four and the two re-expected tests), 0 status changes |
| `tsc --noEmit` | clean | clean |
| `result_export` (cargo, whole) | not rerun (I76: 172) | 176 / 176, plus my oracle test |
| Python, 14 schema and retained-precision modules, with my own CLI authorities | 1,315 passed, 11 skipped | 1,338 passed, 11 skipped: +23, the dispatcher test; 0 status changes |

### 3.4 My differential over the committed non-successor fixtures (`evidence/differential/`)

- **The inputs:** 69 committed non-successor MechanicsResult JSONs, each with its paired model: legacy 0.1.0 ×7, precision-1 ×4, physics-1 ×6, preview-physics-1 ×7, source-blocks-1 ×15, physics-source-1 ×14, load-reference-1 ×6 and load-reference-source-1 ×10.
- **The 13 steps**, run in base and candidate copies:
  - the pure projection (both origin scopes) and its validator;
  - the Current path through mocked IPC: the product's registration, manifest and AnalysisRun builders, then `buildCurrentResultExport`;
  - stress-neutral packets with the registered result and with a clone, against preview and v0.2 AnalysisRuns, and both validator modes.
- **All 891 step outcomes are identical:** 267 byte outputs (100 derivatives, 30 Current result exports, 82 stress-neutral packages and 55 AnalysisRuns), 153 identical errors and 471 identical values. The only differences are the two successors' rows, which are deliberate.
- **Independent of I75's method:** the inputs come from the committed fixtures rather than the test suite, and the driver is my own.

### 3.5 Committed packages (R-3)

- 64 stress-neutral packages are committed in the tree: 8 at 0.1.0, 40 at 0.2.0 and 16 at 0.3.0, most under `P/execution/`.
- None carries `retained_precision`.
- Base and candidate validators give identical header-only verdicts on all 64.
- Apart from the two goldens, no committed result document carries a receipt either. So I75's stricter validator (item b) refuses nothing committed.

## 4. Item 4: the dispatcher (`evidence/dispatcher/`)

- **The `$ref` is correct.** `oneOf[2]` is `{"$ref": "results.v0.3.schema.yaml"}`, which resolves against the dispatcher's `$id` to the version file's `$id`.
  - `oneOf[0]`, `oneOf[1]`, `$defs` and every top-level keyword are unchanged in value.
  - The dispatcher's top-level guard has the same seven properties and the same `required` list as the version file's top level, and the same `ExportFormatStatus`.
  - The only `#` pointers into the dispatcher anywhere in the tree are `#/$defs/QuantityResult`, which is unchanged.
  - No production code validates through the dispatcher; production code holds only reference strings.
- **Every committed document.** I read 378 result documents from 372 committed files, the whole tree included (`P/execution/` too):
  - **at 0.3.0, 24 documents:** all admitted by both the version file and the candidate dispatcher. The base dispatcher refuses 18 of them (F-U6c-2).
  - **at 0.1.0 and 0.2.0, 354 documents:** the candidate and base dispatchers agree on every one.
- **My own documents.** 145 distinct documents of my own:
  - 62 successor derivatives through the product base;
  - 30 Current exports;
  - 53 projections built on my stand-in base, which the version file refuses because the stand-in's `professional_boundary` is incomplete.

  The dispatcher and the version file agree on all 145.
- **My own refusals.** 693 mutations of one representative per identity (successor, precision-1, physics-1, preview-physics-1, source-blocks-1, physics-source-1, load-reference-1 and load-reference-source-1):
  - dropped and added top-level and envelope members;
  - wrong versions, objectives, deliverable and export-format values;
  - a receipt grafted onto other identities, and relabelled identities;
  - edits to the receipt's shape and to disclosures;
  - random retypes.

  **The version file refuses 651, and the dispatcher refuses every one of them.** Both admit 34, and all three schemas refuse the 8 without `schema_version`. There is no disagreement at 0.3.0.
- **Mutants:** I76's D1–D3 are killed. My D4 (an arm that admits any 0.3.0 document) and D5 (the `$ref` plus a ban on the receipt) are killed.

## 5. Item 5: N-5

- **The argument is correct.** In `source_blocks::validate`, three checks run before any `integer` call:
  - the receipt-shape check, in which every receipt field that `integer` reads has a schema maximum of 2^53−1 or less;
  - the receipt hash, under the checked profile, which refuses integers above 2^53−1;
  - the publication hash, under the same profile.

  The invocation is hashed under the same profile before its model is read. So no value above 2^53−1 reaches `integer` through the public API, and S1 is equivalent there.
- **Reproduced:** S1 survives `--test source_blocks` (15/15) and the whole suite (176/176). N1 and N2 are killed. My RV-R3 (the publication-hash check removed) is killed by I76's test, which therefore pins the hash layer for the summary counts.
- **A coverage detail:** see NT-1.
- **`RE/src/` is unchanged** in the candidate. The direct unit test that kills S1 goes with PR-B1 (decision 9).

## 6. Item 6: mutants (`evidence/mutants/`)

**The implementers' mutants, rerun on my copies:**
- **I76's 14:** G1–G7 killed; S1 survives `--test source_blocks` and the whole suite, as designed; N1 and N2 killed; D1–D3 killed.
- **I75's 54:** M01–M34 and R01–R19 killed, each with exactly I75's failing-test count. R20 survives (equivalent; §1.5). The controls passed 430 of 430 in both parts.

**My own** (13 edits in 14 runs; each edit is exact and restored, checked by sha256):

| ID | Mutation | Check | Result |
|---|---|---|---|
| RV-T1 | an absolute row in a unit the reader does not normalize is valued (no disclosure) | I75's 10 test files | killed (1) |
| RV-T2 | the header-only validator treats a `not_covered` withholding as blocking | same | **survives** (NT-7) |
| RV-T3 | the loss report's class sentence swaps its two counts (builder and validator agree) | same | killed (4) |
| RV-T4 | the `not_covered` text loses its withholding clause | same | killed (1) |
| RV-T5 | b is printed in the row's source unit instead of the SI unit | same | killed (9) |
| RV-T6 | the policy fails open when standing throws | same | **survives** (NT-10) |
| RV-T7 | the derivative copies the receipt without `receipt_sha256` | same | killed (13) |
| RV-R1 | Rust's `not_covered` text shortened | `--test retained_precision_derivative_golden` | **survives** (NT-7) |
| RV-R1b | the same edit | `--test retained_precision_carriers` | killed |
| RV-R2 | Rust's SI table keeps `mm` | golden test | killed |
| RV-R3 | the publication-hash check removed from `source_blocks::validate` | `--test source_blocks` | killed |
| RV-R4 | `relative_verified` rows disclosed | golden test | killed |
| RV-D4 | the dispatcher's 0.3.0 arm admits any 0.3.0 document | dispatcher and v0.2 tests | killed |
| RV-D5 | the 0.3.0 arm `$ref`s the version file but bans the receipt | same | killed |

## 7. I75's final items and I76's items

- **(a) The exported base and origin: acceptable** (NT-5).
- **(b) The stricter result validator** mirrors Rust `validate_document`: a receipt on a non-successor document gives `RETAINED_PRECISION_DOWNGRADE_FORBIDDEN`. It only refuses more. No committed non-successor document carries a receipt, and my 100 non-successor derivatives validate identically on both sides.
- **(c) The pure projection reads no standing, and its only product caller enforces eligibility.** Confirmed (§1.1, §1.3): unregistered bytes derive, as in Rust, while `buildCurrentResultExport` refuses them with the standing's code before any other check.
- **(d) The two unused seams:** see NT-4.
- **(e) Carried items.**
  - R-2's owner choice for B8 (NT-6).
  - Python's declared refusal of successor packages (decision 8).
  - The native panel witness and the product-flow export, both at B8 (G10's redefinition; CQ-11). My own Current export through the product's manifest builder with the pinned model fails with `INPUT-MANIFEST-LOAD-BASIS-INCOMPLETE`, which confirms CQ-11's reason.
- **I76 (a):** see NT-8. **I76 (c):** see NT-3.
- **I76 (d):** the synthetic 0.2.0 instance is reasonable. My sweep also found 334 committed 0.2.0 documents under `P/execution/`, and both dispatchers give each the same verdict.

## 8. Host, method and limits

- **Copies.** I made a `git archive` of `2033260c57` and of `c1bfc460fc` into `WT/rv101/{cand,base}`: all of `P/` except `P/execution/_Coordination/AgentRuns`, which my sweeps read from Git instead. I also made three APFS clones: `mut` for TypeScript mutants, `mutr` for Rust and Python mutants, and one for the `tsc` controls. Targets went to `WT/targets/rv101/`, scratch to `WT/scratch/rv101_t6s_01/`, and `TMPDIR` pointed there. All of these are deleted at return.
- **Wasm assets** were copied from `WT/sweep-skewpin/P/apps/desktop/public/`. The eight hashes equal I71's list (`evidence/host/wasm_assets.sha256`).
- **Tools.** Each copy has an untracked `node_modules` symlink to `NMS`; nothing was installed. Versions: Node v24.18.0, vitest 4.1.10, TypeScript 5.9.3; rustc and cargo 1.97.1; Python 3.13.14 with jsonschema 4.26.0.
- **Cargo.** Every build and test went through `WT/tools/t3_cargo.sh`, with `--locked --offline`, `CARGO_BUILD_JOBS=4` and `RUST_TEST_THREADS=2`. I built the checked-JSON and units CLI authorities through the lock from the candidate copy; their sources equal base's. One direct call, to disclose: `cargo --version`, run once to record the toolchain, with no build.
- **Python** ran with my authorities and with a `cargo` shim on `PATH` that refuses any call; no test invoked it. No pytest run started cargo.
- **ROOT's lock rule for heavy test jobs** arrived while my work was running.
  - Three heavy jobs were still running unlocked when ROOT's first DEC-025 sweep took the lock at 14:02:06Z. My two-copy differential ended about 10 s into the sweep (14:02:14Z), and my dispatcher oracle about 3 min in (14:04:59Z). My rerun of I75's mutant loop (M01–M18 done) ran until about 14:05Z. My unlocked pytest runs had already ended, at 14:00:02Z.
  - On the message, I stopped the mutant loop at about 14:05Z (M19 mid-run). I restored its file from the candidate copy and checked it by sha256. I re-queued M19–R20 and my own TypeScript mutants as one `lockf` job.
  - Everything after that ran under the lock. I added `WAIT`, `START` and `END` marker lines for my `lockf` job to `WT/guard/cargo_jobs.log`.
  - After the network stop, my five waiting jobs ran in turn: the mutant loop (14:58–15:27Z), the dispatcher re-run, RV-R1b, the `-0` oracle and the tie oracle. I cancelled one queued job before it started: a Rust re-run of the parity oracle on inputs that differed only in stand-in values. Its first run is the evidence. I logged the cancellation in `cargo_jobs.log`.
  - ROOT may judge whether the first sweep's first three minutes need a rerun. That sweep ended with rc 0.
- **My own harness errors, found and corrected:**
  - My first differential runs left stray workers writing into the same output files, so I discarded them. The reported run (`r4`) uses run-tagged outputs, and its entries are in strict manifest order in both copies.
  - My TypeScript input writer drops `-0`. I rebuilt the affected inputs with `-0` restored.
  - My first base and origin stand-in had the wrong professional-boundary keys. The corrected run is the one reported in §2.1.
- **Not done:** no Git writes, installs, DEC-025, native or solver-at-scale jobs, and no native witness (that is B8's).

## 9. For ROOT

1. **SF-1:** rule the repair: the one-line `toExponential(16)` repair with Rust-computed tie vectors, or a bounded domain. I will confirm the repair.
2. **The lock-rule overlap** (§8): whether the first DEC-025 sweep (SI1, `START` 14:02:06Z) needs a rerun because of my unlocked jobs in its first three minutes.
3. **Carry, optionally:** NT-1 (`block_order`) into PR-B1's brief; NT-9 into S-I2's text planning; the test gaps in NT-7 and NT-10 into T6's later slot.

## 10. Records here (`R/REVIEW_RV101/t6s_01/`)

- `REVIEW.md` (this file) and `SHA256SUMS`, which covers every file except itself.
- `evidence/`:
  - `tools/`: my oracles, drivers and run scripts, with paths replaced by placeholders, and the sha256 of I75's driver, which I reran unchanged;
  - `oracle/`: `facts.jsonl`, `parity_summary.txt`, `exp_differences.txt`, `exp_ties.txt`, `exp_repair_check.json` and `negative_zero_inputs.txt`;
  - `differential/`: the manifest, both outcome files and `compare.txt`;
  - `dispatcher/`, `packages/`, `stress_neutral/`, `fence/`, `suites/`, `tsc_controls/`, `mutants/` and `host/`.
