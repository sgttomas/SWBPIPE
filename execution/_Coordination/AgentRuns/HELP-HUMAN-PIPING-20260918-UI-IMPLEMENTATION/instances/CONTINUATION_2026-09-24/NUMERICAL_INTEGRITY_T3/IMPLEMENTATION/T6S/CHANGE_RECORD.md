# T6S: change record

**This PR brings the T6 successor-output slice (T6S) to main.** It is the narrow T6 slice that the owner pulled forward (RR "I61's U8 plan ruled…", decision 12). It closes public activation's checklist item 4 before B8.
- **The output policy.** The shared refusal is split into a per-route, per-surface desktop output policy, which `tsc` checks for exhaustiveness. Only the Result Export and Stress-Neutral Export panels admit a retained-precision successor, and only at numerically eligible standing with the live native capture.
- **Result export.** The desktop result JSON of a successor takes Rust `derive_document`'s form, byte for byte against Rust goldens.
- **Stress-neutral export.** A successor's package carries its receipt whole and labels each classified row (S-d), with no schema change.
- **The v0.3 dispatcher** references the v0.3 version file instead of a drifted inline copy.
- **RV95 N-5.** A public-API test pins the two layers that mask the 2^53−1 integer bound.

T6S is a product PR, after S-I1 (#1100) and U8 (#1102) in the merge order (RR "U8's full suite passes; I77's package accepted; NUM sequencing for the product PRs"). The reviews are agent reviews, not personal review by the owner.

**Status of this file.** I80 drafted it, records only, after RV101 confirmed the repair. The PR-head gates run before the merge, and are recorded in the merge record on the integration branch.
- **The T6S head** is `fdcdb5e024` on `codex/piping-t6-successor-outputs-20261005`, cut from main `c1bfc460fc`.
- **NUM carries it.** ROOT merged it into NUM with `--no-ff` as `8eaa4403a6`, for the PR's source equality (RR "RV101 confirms SF-1's repair; T6S merged into NUM"). At drafting, NUM is `2b1f244faf`, which adds only records and main's #1103.
- **The PR branch** `codex/piping-t3-t6s-pr-<date>` is cut by ROOT from main, with the 19 files from `fdcdb5e024` and this package.
- **Main has moved since the base.** At drafting, main is `d8c88774d0`. Its first-parent commits after `c1bfc460fc` are:
  - #1098 and #1099 (App v4 only);
  - #1100 (S-I1);
  - #1101 and #1103 (records only);
  - #1102 (U8).

  They touch none of the 19 files, so the PR's diff against main is the slice's diff.

**Notation:**
- **P** = `projects/chirality-piping`; **PP** = `P/core/product_physics`; **RE** = `P/core/reporting/result_export`; **DT** = `P/apps/desktop/src`.
- **RR** = T3's `ROOT_RULINGS_V1.md`, which is append-only; an RR "title" names a ruling heading.
- **R/…** paths are T3's `RESUME_2026-09-30` records on NUM.
- **PLAN** = I74's `R/I74/t6_slice_plan_01/PLAN.md`; **D2** = T3's `DESIGN_STANDING/DESIGN.md`.
- **"Decision n"** is I74's decision n, as ruled in RR "I73's checkpoint 1 and I74's plan ruled; D2 5b.3; the T6 slice dispatched".
- **Citations in the 19 files resolve through `citations.json`,** pinned at NUM `2b1f244faf` (§6).

## 1. What the PR contains

**Maintained source: 19 files under P.** 9 are added and 10 modified, +2,062 / −1,903 lines, 1,281,065 B at the head. Main's commits after `c1bfc460fc` through `d8c88774d0` touch none of the 19.

| File | +/− | Change | sha256 at `fdcdb5e024` |
|---|---|---|---|
| `DT/features/result-export/ResultExportPanel.tsx` | +8 / −4 | The panel reads its policy entry; the D2 §4.9.9 summary line | `d362eb2bae112a0d9cb412627d53afc7efef193c31cd61d32183e7a6b2fb6da5` |
| `DT/features/result-export/resultExportAdapter.ts` | +51 / −13 | T6S-4: the successor form of `deriveResultDocument` and `validateResultDocument`; the policy gates; the Current base and origin moved verbatim into two exported functions | `b2ba958adc6465b4d3adc740a5548fc7d7a90c00429d1dd16c00a4310c5cba13` |
| `DT/features/result-export/retainedPrecisionResultExport.test.tsx` | new, 316 lines | T6S-4's tests: golden parity, the controls and the test-built positive witness | `a171d71c65a53fd79c72df1e8413a807c2fba468d1543e0b07f59ab7634be6a1` |
| `DT/features/results/knownSemanticLimitations.ts` | +2 / −1 | One comment sentence, no code (R-6) | `07decdc36f974b02253476f88a1b01e126c5c2db82f7f25e14b1308b314eab4d` |
| `DT/features/results/loadReferenceOutputAvailability.ts` | +13 / −10 | The shared refusal now reads the policy. Its meaning is unchanged, apart from decision 12's reworded text | `e1662f09334a12c2deaeda859693f2cf1d804c4de091b0c90e96abc6da7c30a9` |
| `DT/features/results/outputPolicy.test.ts` | new, 122 lines | The policy tests (R-7) | `ee51a26d2542145d6ccaa64fe93256d89c95cce93ce204e08702b8232973edcb` |
| `DT/features/results/outputPolicy.ts` | new, 148 lines | T6S-3: `OUTPUT_POLICY`, one entry per `SourceContract` over 21 surfaces | `712941315278fc92773366d59e34c45bf4d84e06541265402cc5bbd9ffbc9f4b` |
| `DT/features/results/retainedPrecisionDisclosure.ts` | new, 99 lines | D-U6-2's codes and messages; b printed as Rust's `{:e}`, repaired for 16- and 17-digit ties; classes from the reader; the summary line | `cb7c7e225c0575b742957a4c0e6f8f4548e5d682fc745867834d3c4dae361fc1` |
| `DT/features/results/retainedPrecisionIntegration.test.tsx` | +19 / −19 | S-1: the U7 slice T block now asserts the policy's admission and keeps a moved-model negative | `d1bf8a5df4937b9b2ddf7454343150b23fa64c177d3b210d1fa8bd6d08125776` |
| `DT/features/results/retainedPrecisionOutputRefusal.test.tsx` | +45 / −21 | The two panels' expectations and the reworded text. The 18-surface table and the report assertion are unchanged | `4c3abb0bb46353f25572be9dbe85b41e5f76d5ebd3cac8b888a7b9b73c76df71` |
| `DT/features/stress-neutral/StressNeutralExportPanel.tsx` | +115 / −26 | T6S-5: successor packaging (S-d), the receipt in the transport header, transport validation, the policy gate | `50bd72355ccbc8a1c1ca9cfa7a1581ba44adfa9982f0b53a3f3350c29aea2e60` |
| `DT/features/stress-neutral/retainedPrecisionStressNeutral.test.tsx` | new, 352 lines | T6S-5's tests, the disclosure texts, and the `{:e}` test's 69 Rust-computed vectors | `dd65f2b61f3176a671dbdbb088958141c5ff0b41abcccd006d41d6c614d7aa47` |
| `RE/tests/retained_precision_derivative_golden.rs` | new, 422 lines | T6S-2: three tests that regenerate and pin the Rust goldens | `2706d06b561756b530a234e4c488cd341061093f822b45bfccf06e75c99fc798` |
| `RE/tests/source_blocks.rs` | +88 / −0 | RV95 N-5's public-API masking test, appended | `00b322c51a35da25b9f552bd9965eb6a30a853dffea4e8a4cc3e4ca1513f73ee` |
| `P/fixtures/results/retained_precision_successor_derivative_dense_scrutiny.json` | new, 384,426 B | The Rust golden, dense | `3f9905ad4c4bba688687714751abe965d18cea834701c32c379d81f573dcf682` |
| `P/fixtures/results/retained_precision_successor_derivative_sparse_interactive.json` | new, 381,178 B | The Rust golden, sparse | `958df02e276538a96c4732302a46cb36a53c2298ca7e1f4deb88a3748c67c661` |
| `P/schemas/results.schema.yaml` | +2 / −1,808 | `oneOf[2]` becomes `{"$ref": "results.v0.3.schema.yaml"}` and the description is updated. `oneOf[0]`, `oneOf[1]`, `$defs` and every other top-level keyword are value-identical | `d6b5bace42ef5208ae8686dbeee88f9e942aa51b24c504ba2652bacb4dff1d13` |
| `P/tests/test_result_export_v0_2.py` | +14 / −1 | `validator()` gets the local schema registry; nothing else | `9605b438d88862d442e99d94db80e48504f132c160b66ab4e36d4bc210a16c48` |
| `P/tests/test_results_dispatcher_v0_3.py` | new, 244 lines | 23 tests: dispatcher against version file, refusals and in-file mutants | `08007e4ddbbb3a2ecd2ea750503376e54f73e4e6a6eef7df0959b3ace7462988` |

**Every hash** equals the implementers' returns and REPAIR_01. I computed them from the Git objects at `fdcdb5e024`.

**The commits,** on the T6 branch:
- `055ee0c0bc`: I76's T6S-1 and T6S-2 (7 files);
- `2033260c57`: I75's T6S-3 to T6S-5 (12 files);
- `fdcdb5e024`: I75's REPAIR_01 for RV101's SF-1 (2 files), committed by ROOT.

**This package** has 4 files in `…/NUMERICAL_INTEGRITY_T3/IMPLEMENTATION/T6S/`:
- this record;
- `PR_BODY.md`;
- `citations.json`;
- `SHA256SUMS`, which covers the other three.

The package reuses main's `IMPLEMENTATION/F2A_D1/source_equality.py` and `check_citations.py` (#1082) unchanged, without copies. The draft's run records, in `T6S/_draft_run_records/`, stay on NUM and are not in the PR.

## 2. What it does

**The output policy (T6S-3; decisions 2, 3 and 12).**
- `OUTPUT_POLICY` is a `Record<SourceContract, …>` with one deliberate entry per route. `tsc` refuses a route without an entry, so B3's `physics-retained-1` will not compile until it is given one.
- **For `retained_preview_physics`,** the entry names 21 surfaces. `result-export` and `stress-neutral` are `admitted_when_eligible`. The other 19 refuse:
  - the 18 panels behind `LoadReferenceOutputGate`, including Rule-check;
  - the report package, which refuses at its own point (`REPORT-PACKAGE-FRESH-RESULT-UNAVAILABLE`).
- **Admission** requires numerically eligible standing with the live native capture of these exact bytes for the current model (D2 §4.9.4; D-U7-4).
- **Fail-closed rules:** a surface the entry does not name refuses, and a route value without an entry refuses with `OUTPUT-ROUTE-NOT-REGISTERED`. Load/reference-state routes refuse on every surface, as before.
- **The refusal text** on the remaining surfaces is reworded so that it no longer says every output is unavailable. It is display only.

**Result export (T6S-4; decisions 4, 6 and 11; D2 §4.9.7, §4.9.9; D-U6-2).**
- **`deriveResultDocument`** takes Rust `derive_document`'s successor form:
  - `contract_evidence` is copied, and `retained_precision` is copied whole;
  - each `absolute_verified` or `not_covered` row is disclosed, not valued, with D-U6-2's reason code and Rust `class_disclosure`'s exact message, with b printed as Rust's `{:e}`;
  - classes come from the accepted reader, run without an invocation, as Rust `retained_row_classes` does.
- **`validateResultDocument`** mirrors Rust `validate_document`:
  - the receipt must equal the source's;
  - `RETAINED_PRECISION_DOWNGRADE_FORBIDDEN` refuses a receipt on any other identity;
  - class codes and messages must be consistent.
- **`buildCurrentResultExport`** refuses an ineligible successor with the standing's own code, before any other check.
- **The base and origin.** The Current builder's base and origin literals moved verbatim into the exported `currentResultDocumentBase` and `currentReceivedOrigin`. The output is byte-identical, and the goldens pin the product's own code (I75 item a; RV101 NT-5).
- **Byte parity.** For both pinned successors, the TypeScript document equals I76's Rust goldens byte for byte (`958df02e…`, `3f9905ad…`).
- **The bound's text.** `rustLowerExp` prints Rust's `{:e}` exactly, exact decimal ties included (§4).

**Stress-neutral export (T6S-5; decision 5, S-d).**
- **The package** gains for a successor:
  - the UTF-8 CSV policy and the successor's semantic table;
  - `contract_evidence` and the receipt, whole;
  - the receipt in the transport header, which closes I67's F4;
  - transport validation through the reader's `validateRetainedPrecisionTransport`.
- **Each classified row** withholds its unit-preservation witness under one of two new info codes, `SN-UNIT-WITNESS-WITHHELD-RETAINED-PRECISION-ABSOLUTE-VERIFIED` and `…-NOT-COVERED`. Each carries D-U6-2's message and is counted in the loss report. The CSV row and its value are unchanged.
- **No schema change.** The packages validate under the existing successor branch of `stress_neutral_export.v0.3.schema.json`.
- **Readiness.** Every successor package reads `validation_status: blocked`, through the existing aggregate (R-2, the fail-safe convention until the owner's choice at B8).

**The dispatcher (T6S-1; decision 7, CQ-7 B).**
- `results.schema.yaml`'s 0.3.0 arm is a `$ref` to the version file, so it admits exactly what the version file admits. That removes F-U6c-2's drift by construction.
- The one Python test that validates through the dispatcher now builds the local registry, with no network retrieval.
- An equivalence test covers every committed v0.3 document under the test's roots, and mixed and unknown versions are still refused.

**RV95 N-5 (decision 9).**
- The test sets the 12 receipt fields that `source_blocks::integer` reads, and four summary counts, to 2^53.
- It pins the two layers that refuse them before `integer` runs: the receipt shape and the checked-profile hashes.
- RV95's mutant S1 is equivalent at the public API, so the test that kills it is a direct unit test in `RE/src/source_blocks.rs`. That test goes with PR-B1.

## 3. What it does not do

- **No product caller of the successor yet.**
  - The native commands call PP's ordinary wrapper, which passes `retained_entry: None`.
  - TS standing needs the live native capture.
  - So the admitted panels are reachable only by tests until B8 (decision 3; RV101 §1.4).
- **Activation and native Current are B8's.** That includes the product-flow export of a desktop-shaped successor through the manifest builder (CQ-11).
- **The owner-held items are untouched:**
  - **R-2,** whether a successor's stress-neutral package may read ready while its classed rows carry withheld witnesses, is prepared for the owner at B8. Until then, every successor package reads `blocked`.
  - **G10 keeps its ordinary-route half,** still outstanding by the owner's decision.
  - **Native-app witnesses** stay owner-held.
- **G10's moved half is B8's.** By the owner's decision (RR "Owner decision: G10 is redefined…"), B8's native Current witness includes the two panels exporting an eligible successor while every other surface refuses it. Until B8, this slice's vitest suites and the hosted browser shards cover these gates. They are not native evidence.
- **No PP, D1-crate `src`, D1-embedded static, build-identity, lock or dependency change.**
  - `results.v0.3.schema.yaml` and the 14 reviewed inputs are unchanged.
  - The readers and carriers are unchanged, apart from R-6's comment.
  - So are `src-tauri`, `e2e_plan.py`, and the stress-neutral and AnalysisRun schemas.
- **Python keeps refusing successor packages,** a declared difference (decision 8). F-U6b-2's transport validator goes to B6.
- **No standing token, invocation or producer-origin claim** in either export (decision 6; D-U7-6). Values outside the checked profile keep refusing.
- **The rest of T6 keeps its slot** (PLAN §1.6).

## 4. Review

- **RV101,** fresh and independent, on `c1bfc460fc..2033260c57`: **PASS,** 0 BLOCKING, 1 SHOULD-FIX, 10 NOTE (`R/REVIEW_RV101/t6s_01/REVIEW.md`, `510fbdb5…`).
  - **Closure holds.** In every probe, only the two panels admit a successor, at eligible standing with the live capture.
  - **Faithfulness.** RV101's own Rust harness re-derives both goldens. Over 63 reader-valid statements, the TypeScript and Rust derivatives are byte-identical.
  - **Nothing outside the fence.** The 19 files are the fence plus the granted S-1, R-6 and R-7.
  - **Unchanged routes.** 891 step outcomes over 69 committed non-successor fixtures are identical.
  - **The dispatcher** refuses all 651 of RV101's mutations that the version file refuses.
  - **Mutants.** The implementers' sets reproduce. 10 of RV101's 13 edits are killed; the 3 that survive are NT-7, NT-10 and RV-R1 against the golden test only.
- **SF-1, the bound's text on exact decimal ties.** `rustLowerExp` printed V8's round-half-even choice where Rust's `{:e}` rounds a tie up in magnitude. No b below 2^-25 can tie, so no real receipt was affected, but decision 4's byte-parity claim was not exact.
- **The repair** (I75's REPAIR_01, committed as `fdcdb5e024`) is broader than RV101's 17-digit rule, which misses 16-digit ties: for example, `4308628432e3716a` = 857964921253421.25.
  - **The general form:** with n the shortest form's digit count, print `toExponential(n−1)` when it round-trips, and the shortest form otherwise.
  - **I75's results:** 0 of 123,607 Rust-computed words differ (6,395 before), the test now has 69 Rust-computed vectors, and 9 of 9 mutants are killed.
- **RV101's ADDENDUM_01** (`461507ab…`; its own sum file 17/17 OK): **CONFIRMED,** with no new findings.
  - RV101's own Rust oracle matches the product function on all 185,401 words. Among them are 21,997 16-digit and 10,788 17-digit ties.
  - The round-trip guard is necessary: without it, 90 power-of-two words misprint.
  - The ten T6S test files pass 430/430, and `tsc` is clean, at `fdcdb5e024`.
  - **E-1** corrects RV101's remedy text, which said ties occur only at 17 digits. The diagnosis and the 2^-25 reach bound stand.
- **The review and the addendum together cover the complete diff** `c1bfc460fc..fdcdb5e024`.
- **RV101's confirmation** of the PR head's package and equality comes before the merge (the merge record).

## 5. Gates (RR "T3's gate set and Git rules, consolidated…"; RR "RV101 confirms SF-1's repair; T6S merged into NUM")

| Gate | Result | Record |
|---|---|---|
| **Complete-diff review** | RV101 PASS (0/1/10); SF-1 repaired and CONFIRMED (§4) | `R/REVIEW_RV101/t6s_01/` |
| **The full 40-manifest suite before the freeze** (gate set item 3) | **Ruled:** satisfied by the exact-head DEC-025's 40 manifests against a fresh main baseline, in fresh targets, as for S-I1. The slice moves no registered identity, so there is no separate D1 freeze (RR "I80's package accepted; …"). See the merge record | — |
| **`source_equality.py`** (the PR head against NUM with T6S merged) | See the merge record. **Pre-check at NUM `2b1f244faf` against main `d8c88774d0`,** by Git reads only: \|S\| = 19, exactly the slice's paths; every blob and mode equals `fdcdb5e024`'s; main changed none of them since `c1bfc460fc`, so check 3 has no path. Check 4 needs the PR head with this package | `T6S/_draft_run_records/outputs/source_equality_precheck.txt` |
| **`check_citations.py`** | **PASS** in three dry runs: 2 resolved, 0 ambiguous, 0 unresolved (§6). Rerun on the PR head: see the merge record | `T6S/_draft_run_records/outputs/` |
| **GEN-8** on the exact head (E-4's method) | See the merge record. GEN-8 should know RV101's NT-3 (§7, routed note 4) | — |
| **Hosted CI** on the PR, and the full-SHA dispatch (`piping-desktop-e2e.yml`, `target_base` = main) | See the merge record | — |
| **The exact-head Mac DEC-025** against a fresh baseline of current main, in fresh targets (`run_dec025.sh`, `compare_suites.py`, counted at `ALL-DONE`) | See the merge record | — |
| **Pass B** (gate set item 7) | **Not applicable (ruled),** on §5.1's evidence; RV101 re-reads the scope at the PR head | §5.1 |
| **Native witness** | Not applicable. The successor-panel witness is B8's (G10's moved half) | RR "Owner decision: G10 is redefined…" |

### 5.1 Pass B: does the slice touch the F2a D1 milestone's call graph?

**From the diff, it does not.** The slice changes only desktop TypeScript, RE's integration tests, the dispatcher schema, two new fixtures and two Python tests. I checked this with a read-only scan, `passb_scope_scan.py`, over `c1bfc460fc..fdcdb5e024` and Pass B's own `crate_dirs.txt` (`R/I65/u4_g7_06/_run_records/chain/crate_dirs.txt`).

1. **No D1 crate `src`.** None of the 19 paths lies in any of the 15 D1 crate `src` directories. All 15 `src` trees are identical at base and head.
   - **Of the 15 crate roots,** only RE's tree differs, by its two `tests/` files. RE's `Cargo.toml` and `Cargo.lock` are unchanged, and RE has no build script.
   - **PP's whole tree is identical,** including `Cargo.lock`, `build.rs` and `examples/`.
2. **No embedded static.** I resolved all 137 `include_str!`, `include_bytes!` and `include!` literals in the D1 crate sources and PP's `build.rs` (68 distinct files). None is a slice path, and every one has the same blob at base and head.
3. **No reviewed input.** PP's 14 `REVIEWED_INPUTS` (`build_identity.rs`) are unchanged, so the registered build identity cannot move.
4. **The dispatcher is not read by D1 code.**
   - Three D1 crates (`load_case_algebra`, `primitive_loads`, `stress_recovery`) name it only as the reference string `schemas/results.schema.yaml#/$defs/QuantityResult`.
   - Its `$defs` is value-identical at base and head: I parsed both, and I76 and RV101 found the same.
   - The file D1 code embeds is `results.v0.3.schema.yaml`, which is unchanged.
5. **No reader source.**
   - These are unchanged: RE's `retained_precision.rs`, `derivative.rs`, `semantic_contract.rs` and `source_blocks.rs`; every `P/core/analysis_runs/*.py` and `P/core/handoff/stress_neutral/*.py`; and TS `retainedPrecision.ts` and `retainedPrecisionStanding.ts`.
   - The one carrier touched, `knownSemanticLimitations.ts`, changes inside one `/** */` comment only (R-6). It is desktop TypeScript, which no D1 build compiles.
6. **No manifest, lock or build script** changes anywhere in the slice.

**The tool's classes, by analogy.** I72's U8 run classed RE `tests/`, `P/tests/`, DT and un-embedded fixture rows as `not-d1`, and a full run here would be expected to class all 19 rows the same way. That is a prediction, not a run.

**RV101's evidence supports this independently.** Its §3.1 found everything under `RE/src/` and `PP/`, every D1 crate `src/`, every embedded static and every lock file untouched at `2033260c57`. ADDENDUM_01 confirms the repair changes only the two DT files.

**Recommendation: gate item 7 does not apply, so the PR runs no Pass B.**
- This is PLAN §2.3's conclusion under decision 9 as ruled: the direct unit test that would touch `RE/src/source_blocks.rs` goes with PR-B1.
- RV101's PR-head confirmation can re-read the scan against the PR head.
- If ROOT wants a mechanical confirmation anyway, a no-build Pass B with I65's tool is the proportionate form.

**Acceptance runs on the slice,** by the implementers and RV101. The suite differences are only added tests.

| Suite | Result | Source |
|---|---|---|
| Desktop vitest | 3,552 → **3,590** (138 → 141 files): 44 added and 6 renamed (S-1's four, two re-expected), 0 status changes. Rerun at the repair: 3,590/3,590. RV101: 3,590/3,590 at `2033260c57`, and the ten T6S files 430/430 at `fdcdb5e024` | I75 RETURN and REPAIR_01; RV101 |
| `tsc --noEmit` | Clean at `2033260c57` and `fdcdb5e024` | I75; RV101 |
| `result_export` | 172 → **176**: the 3 golden tests and N-5's | I76; RV101 176/176 |
| Python | The sweep: 3,424 → **3,447** passed (+23, the dispatcher test). The 116 failures and errors are the same set on both sides: archive copies without Git, and the cargo shim. RV101's 14 schema and retained-precision modules: 1,315 → 1,338 | I76; RV101 |
| Unchanged routes | I75: 186 non-successor builder inputs, 0 byte differences. I76: 116 Rust-captured derivatives byte-identical. RV101: 891 step outcomes identical over 69 fixtures | I75, I76, RV101 |
| Mutants | I76: 12 of 14 killed; S1 survives both runs, by design. I75: 53 of 54; R20 is equivalent. REPAIR_01: 9 of 9. RV101: 10 of its 13 edits killed, and both implementers' sets reproduce | I75, I76, RV101 |

## 6. Citations

**`citations.json`** follows #1082's index format, pinned at NUM `2b1f244faf` (NUM's head when written; pushed).
- **The documents table** carries #1082's 23 names unchanged as detection vocabulary, as S-I1's and U8's do.
- **The slice's added lines cite no design document in the tool's document class.** Its D2 and PLAN references are written without "§" or ":" (for example "D2 4.9.7", "I74 PLAN 1.2"), so the tool finds none.

**Checked by main's `check_citations.py`:** 2 record citations, both resolving at the pin.
- **`R/I76/t6s_01/inputs`,** in the result-export test's header. The folder is the same tree on main.
- **`REVIEW_RV101/t6s_01/evidence/oracle/exp_differences.txt`,** in the `{:e}` test's comment. It is on NUM, and on main since #1103.

**The dry runs** (`T6S/_draft_run_records/outputs/`): each gives 2 resolved, 0 ambiguous, 0 unresolved, PASS.

| Base → head | Paths in the tool's diff | Why |
|---|---|---|
| `f8ed4f0551` → `fdcdb5e024` | 243 | The brief's form, with the main named at dispatch. The tool takes a two-dot diff, so this pair also scans main's later changes in reverse: 209 App v4 paths, and S-I1's and U8's 15 piping files. They carry no citation |
| `c1bfc460fc` → `fdcdb5e024` | 19 | The slice's own diff, equal to the PR's diff against main |
| main `d8c88774d0` → NUM `2b1f244faf` | 19 | NUM with T6S merged, whose maintained diff from current main is the PR head's |

**Negative controls fail as they should,** each with exit 1:
- a missing entry gives UNRESOLVED;
- a pin at `f8ed4f0551`, which lacks RV101's evidence, gives FAILED;
- a wrong record path gives FAILED.

**Outside the tool's classes,** and listed under `named_references` (18 entries, 135 sites):
- RR decisions 2–6, 11 and 12;
- PLAN §1.1–§1.3 and §4.3; D2 §4.9.4, §4.9.7 and §4.9.9;
- D-U6-2, D-U6-5, D-U7-4 and D-U7-6;
- CQ-1, -4, -5, -7, -10 and -11;
- RV101 SF-1 and REPAIR_01;
- I67's F4, F-U6c-2, and readings R-1 and R-5;
- I76's CHECKPOINT_1 §2;
- RV91 N-5, carried from main's text, and RV95 N-5;
- the slice labels and the implementers;
- the goldens' commit `055ee0c0bc`.

`verify_named_t6s.py`, adapted from I77's `verify_named.py`, checks every site and target at the pin: 179 checks, 0 failed.

**Commands, from a checkout that holds NUM's objects.** `--index` must be given, or the tool reads #1082's index.
```
python3 …/IMPLEMENTATION/F2A_D1/source_equality.py --repo . --pr <PR head> --int <NUM head with T6S> --main <main> --work <scratch dir> --package …/IMPLEMENTATION/T6S
python3 …/IMPLEMENTATION/F2A_D1/check_citations.py --repo . --base <main> --head <PR head> --index …/IMPLEMENTATION/T6S/citations.json
```

## 7. Routed notes, and points for ROOT

**Routed (RR "RV101 passes the T6 slice; SF-1 repaired by I75; no DEC-025 rerun", and the earlier T6S rulings):**
1. **NT-1 → PR-B1.** N-5's test omits `failure.block_order` and the composite receipt. It joins PR-B1's direct unit test, with I76's item b: four receipt integers have tighter maxima than 2^53−1.
2. **NT-9 → S-I2.** The disclosure prints b as a round-trip decimal, not an upward bound. This joins S-I2's planning of D-U6-2's shared text, which lives in one constant per language (PLAN §4.3).
3. **NT-7, NT-10 and I75's item d → T6's later slot.**
   - NT-7: no committed `not_covered` package-level test or shared vector.
   - NT-10: the fail-closed `catch` around standing is unpinned.
   - I75's item d: the two unused `t6` seams in `retainedPrecisionIntegration.test.tsx`.
4. **I76's item c (RV101 NT-3).** The historical generation records `P/fixtures/product_preview/{preview_physics,precision}_fixture_generation.json` still name the old dispatcher blob `9f2adf6a…`.
   - They are true of their generation, and they stay unchanged.
   - No test reads those entries.
   - GEN-8 should know of them.
5. **R-2: owner-held, B8.** Whether a successor's stress-neutral package may read ready.
6. **Also carried:**
   - decision 8's Python difference (F-U6b-2 → B6);
   - CQ-11's product-flow export and the native panel witness (B8);
   - decision 14's consistency notes for the B1, B2, B3 and S-I2 briefs (PLAN §4.3). For example, B1 regenerates T6S-2's goldens if the pinned successors change, and B3 gives its route a policy entry;
   - I76's item a / NT-8, an optional registry helper in `tests/schema_validation.py`.

**For ROOT's ruling:**
1. **The full 40-manifest suite before the freeze.** RR "T3's gate set…" item 3 lists it for product PRs, and U8 ran it. RR "RV101 confirms SF-1's repair; T6S merged into NUM" does not list it for T6S. Either DEC-025's suites part, on the exact head in fresh targets, stands for it, or it runs before the freeze. The records do not say which.
2. **Pass B** (§5.1): recommended not applicable.
3. **The brief's citation form.** `--base f8ed4f0551 --head fdcdb5e024` scans 243 paths, not the slice's 19, because the tool's diff is two-dot. The result is the same here. The PR-head run (`--base <main> --head <PR head>`) scans exactly the 19.
   - Main has also moved since dispatch, to `d8c88774d0` (#1103, records only). The pin and the pre-check use the current heads.
4. **Citation style,** for information:
   - D2 and PLAN sections are cited without "§", so the tool cannot check them. `named_references` and its script do. They could read "D2 §4.9.7" at a later touch.
   - "RR decision n" names no heading, and resolves by context to I74's plan ruling.
   - The goldens' commit `055ee0c0bc` stays reachable from the pushed T6 branch and from NUM, not from main. This is the same as U8's `u8_head`, which RR "U8's full suite passes; …" accepted as point 5.
