# I74: the T6 successor-output slice plan

TASK (Type 2), I74, for ROOT. Brief: `R/BRIEFS/I74_T6_SUCCESSOR_SLICE_PLAN.md` (sha256 `79602972…`). 2026-10-06 UTC.

**Planning only.** I made no source edits and no Git writes, and ran no cargo, native, solver, test or install job. Git reads used `GIT_OPTIONAL_LOCKS=0`. PR #885 was read with `gh pr view` / `gh pr diff` only. Nothing went to scratch or to the system temp directory.

**Where this record lives.** The host refused my writes into NUM: a PreToolUse hook confines this session to its own isolated worktree.
- **This file and its SHA256SUMS** are therefore written at the same repository-relative path (`…/RESUME_2026-09-30/I74/t6_slice_plan_01/`) in the session's own worktree, untracked. That worktree is a checkout of this repository on branch `claude/t3-numerical-integrity-cont-142312` at main `c1bfc460fc`.
- **ROOT copies both files into NUM byte for byte.** SHA256SUMS verifies after the copy.
- **Before the refusal,** I created an empty `R/I74/t6_slice_plan_01/` folder in NUM with `mkdir`. I removed it again (`rmdir`), so NUM is as I found it.

**Notation.**
- **WT:** the T3 working root. **NUM:** `WT/numerics`, the integration branch `codex/piping-numerical-integrity-20260926`.
- **P:** `NUM/projects/chirality-piping`. **PP:** `P/core/product_physics`. **RE:** `P/core/reporting/result_export`. **DT:** `P/apps/desktop/src`.
- **T:** the T3 records folder (`NUMERICAL_INTEGRITY_T3`). **R:** `T/RESUME_2026-09-30`. **RR:** `T/ROOT_RULINGS_V1.md` (append-only; line numbers are stable).
- **CR:** `T/IMPLEMENTATION/F2A_D1/CHANGE_RECORD.md`. **QUAL:** `T/IMPLEMENTATION/F2A_D1/copies/QUALIFICATION.md`. **D2:** `T/DESIGN_STANDING/DESIGN.md`. **I61:** `R/I61/u8_plan_01/PLAN.md`.
- **Successor:** `openpipestress.result_semantics/0.3.0/preview-physics-retained-1` (TS route `retained_preview_physics`).
- **The two panels:** Result Export (`DT/features/result-export/`) and Stress-Neutral Export (`DT/features/stress-neutral/`).

**Basis.** Code is cited as `P/…:line` at NUM `b1e2d7741e` (maintained source equal to main `c1bfc460fc`). While I worked, NUM moved to `95d3cae8db`, ROOT's dispatch record. I checked that delta: four files, all under `P/execution/` (RR, the work graph, and two `IMPLEMENTATION/SESSION_2026-10-06/` files), so no citation moves.

## 0. Findings in brief

1. **One shared function refuses a successor on 20 desktop surfaces, not 2.** `loadReferenceOutputRefusal` (`DT/features/results/loadReferenceOutputAvailability.ts:41–43`) is:
   - the explicit N-5 gate of the two panels (`DT/features/result-export/ResultExportPanel.tsx:16`; `DT/features/stress-neutral/StressNeutralExportPanel.tsx:88`);
   - the gate of 18 other surfaces through `LoadReferenceOutputGate` (`DT/features/results/LoadReferenceOutputGate.tsx:8–11`), pinned in `DT/features/results/retainedPrecisionOutputRefusal.test.tsx:91–110`. The Rule-check panel is one of them (`DT/features/rule-check/RuleCheckPanel.tsx:54`);
   - a builder-level throw through `refuseLoadReferenceOutput` (`DT/features/result-export/resultExportAdapter.ts:58`, `:118`, `:172`; `StressNeutralExportPanel.tsx:491`, `:622`, `:1061`).
   
   **So the gate must be split, not edited.** The two panels gain a deliberate successor admission. The other 18 surfaces and the report package keep the refusal unchanged.
2. **The desktop result export has no successor form today. Rust's canonical derivative does.**
   - Rust `derive_document` copies `contract_evidence` for the successor (`RE/src/derivative.rs:144–157`) and copies the receipt whole (`:170–175`, D2 §4.9.7). It discloses `absolute_verified` and `not_covered` rows with D-U6-2's codes and messages (`:15–61`), and validates all three (`:407–413`, `:697–710`).
   - The TS derivative `deriveResultDocument` has none of these (`resultExportAdapter.ts:64–68`): no receipt, no `contract_evidence` for this route, and no class disclosures. A successor document it produced would fail the `results.v0.3` successor branch, which requires the receipt.
3. **The desktop stress-neutral builder has five successor gaps.** The schema's successor branch already admits a package (`P/schemas/stress_neutral_export.v0.3.schema.json:3762–3860`; it requires `contract_evidence`, `source_annotations` and `retained_precision`, with a UTF-8 CSV). The TS builder lacks:
   - the successor in `usesUtf8Csv` (`:403–405`);
   - its semantic table in `semanticTablePath` (`:1057–1063`);
   - its `contract_evidence` copy (`:588`);
   - the receipt in the validator's header (`:666`; I67's F4, RR:9810);
   - a transport branch in `validateNeutralTransportEvidence` (`:1064–1074`). TS already has `validateRetainedPrecisionTransport` (`DT/features/results/retainedPrecision.ts:1332`).
4. **Python cannot admit a successor package yet.** Its packager refuses the method (`P/core/handoff/stress_neutral/package_v0_3.py:32`, `:406–407`), and the refusal is pinned in `P/tests/test_retained_precision_carriers.py`. Its reader has no transport validator (F-U6b-2, RR:9922), while Rust (`RE/src/retained_precision.rs:4324`) and TS do. No Python caller of a successor exists (Headless is refused at D1.0), so checklist item 4 does not need Python.
5. **The dispatcher's v0.3 branch is an inline copy that drifted.** `P/schemas/results.schema.yaml` is a "self-contained" dispatcher (`:4–6`). Its `oneOf[2]` has 32 `$defs` against the version file's 47, and it names precision-1 only (F-U6c-2, RR:9785). The current version file `$ref`s four other schema files. The one Python test that validates through the dispatcher builds its validator with no registry (`P/tests/test_result_export_v0_2.py:8`).
6. **The slice must not edit `results.v0.3.schema.yaml`.** D1 production code embeds that file (`RE/src/physics_evidence.rs:1005`, an `include_str!` in a D1 crate), so a change re-opens re-qualification. The derivative's format is therefore frozen to what U6c admitted. The same applies to the 14 reviewed inputs (QUAL §5) and every other file D1 code embeds.
7. **RV95 N-5's bound cannot be reached through the public API.** It also cannot be tested without touching a D1-closure file.
   - `source_blocks::integer` (`RE/src/source_blocks.rs:54–59`) runs only after two earlier checks:
     - the receipt shape check (`:728–731`): every receipt integer it reads has the schema maximum 2^53−1 (`P/schemas/source_block_recovery.schema.json`, `$defs` source, work, ordinary, failure and receipt_body), compared in f64 (`:166–172`);
     - the receipt and publication hashes under the checked profile (`:732–745`), which refuses any integer above 2^53−1 (`P/core/serialization/canonical_json/src/lib.rs:98–107`).
   - The `summary` counts it reads (`:1549–1566`) lie inside the publication hash.
   - So any 2^53 fixture fails at `RECEIPT_SHAPE` or at the hash, never at `SOURCE_BLOCKS_INTEGER`. RV95's mutant S1 is **equivalent at the public API**.
   - A test that kills S1 must be an in-module `#[cfg(test)]` test in `RE/src/source_blocks.rs`, as `norm_tests` (`:1614`) and `stress_range_tests` (`:1684`) are. That file is in PP's dependency closure (Pass B `crate_dirs`), and PP's law test embeds it (`PP/src/retained_memory_law_tests.rs:253–262`). **This is the one fence flag** (§2.3).
8. **The manifest-bound result-export path cannot be exercised with the pinned D1 successors.**
   - The product's manifest builder requires desktop load-case fields (`DT/services/inputManifestService.ts:193–207`). PP's pinned request model lacks them (I67's F3, RR:9809).
   - Manifest verification checks only hash consistency (`:131–160`).
   - A desktop-shaped successor needs a new PP run and a PP file, so it belongs to B8's native witness, not this slice (§3, CQ-11).
9. **G10 will go stale.** Its definition includes "the result-export and stress-neutral panels refusing a successor" (RR:11697–11699). After this slice those panels admit an eligible successor. Native-app witnesses are owner-held, so this one question goes to the owner (decision 10).
10. **No src-tauri change is needed.** The native save is content-agnostic (`P/apps/desktop/src-tauri/src/lib.rs:4705–4718` → `native_result_download.rs`). The slice therefore stays off `NUMERICAL_APP_INPUTS` (`P/tools/ci/e2e_plan.py:118–120`), off checklist item 1's file, and off PR #885's files.
11. **Closed until activation, by construction.**
    - TS standing is `numerically_eligible` only with the live native capture (`DT/features/results/retainedPrecisionStanding.ts:27–32`; D-U7-4).
    - The desktop calls only the ordinary wrapper, and no product caller of the Direct entry exists (CR §4).
    - So the opened panels are reachable only by tests until B8 wires a caller. No extra switch is needed (decision 3).

## 1. Scope: the minimal T6 work for checklist item 4

The scope is the owner's decision 12 (RR:11932–11939): result export and stress-neutral export of a successor, replacing the explicit N-5 refusal deliberately; the v0.3 dispatcher; and RV95 N-5. It covers the **desktop (TS)** outputs only, because that is where the N-5 refusal and activation's product caller live.

### 1.1 The two panels and the shared gate

| | |
|---|---|
| **Must do** | Replace the N-5 refusal in the two panels by an explicit successor admission. Keep `loadReferenceOutputRefusal` unchanged for every other surface. Add a per-route output policy, `Record<SourceContract, …>` checked by `tsc` for exhaustiveness, that names each surface. A new route (B3's `physics-retained-1`) then fails to compile until it is given a deliberate entry, and a route left unadmitted refuses. |
| **Standing shown** | The export appears only at `numerically_eligible` with the live native capture (`numericalResultStanding(result, model).eligible`, already in both bindings). The existing `KnownSemanticNotices` class notices stay. Each panel gains one text-only summary line from the carriers' `classificationSummary(result, model)`: per case, "n verified only to an absolute bound; n uncovered" (D2 §4.9.9's UI summary). |
| **Refuses** | Load-reference routes, with today's text. A successor that is not eligible, with a specific reason: unregistered or copied bytes, a changed model, or no live capture (`RETAINED_PRECISION_VALIDATION_REQUIRED` or `…NATIVE_CAPTURE_REQUIRED`). A relabelled or downgraded statement reads unsupported, as today. |
| **Closed until activation** | The 18 other surfaces, the report package (T0R's fresh-result refusal) and the Rule-check panel (checklist item 1's) keep refusing. Their refusal text is reworded so it does not say every output is unavailable (decision 12). |

### 1.2 Result export: the local result JSON

| | |
|---|---|
| **Must do** | `deriveResultDocument` and `validateResultDocument` gain the successor form of Rust `derive_document`: `contract_evidence` copied; `retained_precision` copied whole; each `absolute_verified` or `not_covered` row disclosed, not valued, with D-U6-2's code and the exact Rust message (`RE/src/derivative.rs:38–61`); every other row as the table says. Classes come from the accepted reader run without an invocation (`validateRetainedPrecision(source)`), mirroring Rust `retained_row_classes` (`RE/src/semantic_contract.rs:176–193`). Validation mirrors Rust: receipt equality, `RETAINED_PRECISION_DOWNGRADE_FORBIDDEN` for a receipt on any other identity, and exact class-code consistency. Byte parity is proved against a Rust-generated golden for both pinned successors (§4, T6S-2). `buildCurrentResultExport` needs no new logic beyond the gate. |
| **Standing shown** | In the document: the receipt, the base `numerical_quality` statement as received, and the class disclosures. The document makes no standing token or producer-origin claim. The desktop origin stays `authentic_producer_available: false` with its existing `origin_limit` (D-U7-6). |
| **Refuses** | Everything it refuses today, plus a successor without validated classes (the reader refuses it). Values outside the checked profile keep refusing (`UNSAFE_JSON_NUMBER`), as for every identity (DD-8 stays in T6's later slot). |
| **Closed until activation** | The manifest-bound panel export of a desktop-shaped successor through the product's manifest builder, natively. That is B8's witness (CQ-11). |

### 1.3 Stress-neutral export

| | |
|---|---|
| **Must do** | `buildStressNeutralExportPacket` and `validateStressNeutralExportPacket` gain the successor. They need: the UTF-8 CSV policy; the successor's semantic table path; `contract_evidence`; `retained_precision` whole; `source_annotations` (already produced for every non-precision route); the receipt in the transport header; and transport validation through `validateRetainedPrecisionTransport`. Classified rows are labelled by CQ-2's recommendation (withheld unit witness, two new codes). The package must validate under the existing schema successor branch, with **no schema change**. |
| **Standing shown** | The package carries the receipt, the base statements and the classified-row diagnostics. The panel shows what §1.1 shows. |
| **Refuses** | As today. Also an ineligible successor, and a package whose receipt, annotations or class findings do not bind the supplied source. |
| **Closed until activation** | Python packaging and validation of successor packages, a declared difference (CQ-8). Native packaging is B8's. |

### 1.4 The v0.3 dispatcher

| | |
|---|---|
| **Must do** | Make `results.schema.yaml` admit exactly what `results.v0.3.schema.yaml` admits at 0.3.0, including preview-physics-1, physics-1 and the successor (CQ-7). Move its validating test to the registry helper (`P/tests/schema_validation.py`). Add a dispatcher-versus-version-file equivalence test over every committed v0.3 document, and keep the mixed-version refusals. |
| **Standing shown** | None. It is a shape contract only. |
| **Refuses** | Everything the v0.3 version file refuses, and mixed versions, as now. |
| **Closed until activation** | Nothing; it has no runtime caller (only tests and the declared `result_binding` reference strings). |

### 1.5 RV95 N-5

| | |
|---|---|
| **Must do** | Test the 2^53−1 bound of `source_blocks::integer`. In the slice: a public-API test in `RE/tests/source_blocks.rs` pinning the two masking layers, `RECEIPT_SHAPE` for a receipt integer of 2^53 and the checked-profile hash refusal for a `summary` count of 2^53, plus the record of S1's equivalence at the public API. A direct unit test that kills S1 rides with PR-B1 (decision 9). |
| **Standing shown, refuses, closed** | Not applicable. It is a test only, and `result_export`'s behaviour does not change. |

### 1.6 Out of scope: the rest of T6

T6's row (work graph; `P/execution/_Coordination/WorkGraphs/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/WORK_GRAPH.md:38`) keeps its slot for everything else:
- the report-package rebuild;
- the restored T0R coverage and the reader hardening;
- load-reference outputs;
- solver-mode custody;
- `unit_round_trip_summary`;
- case-scoped standing (F-P2) and the S11 guard's envelope-level Sensitive;
- the carrier migration to binary64 (DD-8, F-P7);
- the KF2 dense-cancellation observation;
- the precision-1 fixture pair;
- I67's F5 (ComparisonPanel's unknown dimension).

## 2. The fence

### 2.1 Write set (all disjoint from U8, S-I1, checklist item 1 and #885)

| Area | Files |
|---|---|
| **The gate and the policy** | `DT/features/results/loadReferenceOutputAvailability.ts`, which is T6's under D-U6-8's notice (RR:9463). Optionally a new `DT/features/results/outputPolicy.ts` |
| **Result export** | `DT/features/result-export/ResultExportPanel.tsx`, `DT/features/result-export/resultExportAdapter.ts`, and a new `DT/features/results/retainedPrecisionDisclosure.ts` (the D-U6-2 texts and the bound formatter, mirroring Rust) |
| **Stress-neutral** | `DT/features/stress-neutral/StressNeutralExportPanel.tsx` |
| **TS tests** | `DT/features/results/retainedPrecisionOutputRefusal.test.tsx` (the two panels' expectations only; the 18-surface table and the report assertion unchanged); new `DT/features/result-export/retainedPrecisionResultExport.test.tsx` and `DT/features/stress-neutral/retainedPrecisionStressNeutral.test.tsx` |
| **The dispatcher** | `P/schemas/results.schema.yaml` |
| **Python tests** | `P/tests/test_result_export_v0_2.py` (validator construction only); a new `P/tests/test_results_dispatcher_v0_3.py` |
| **Rust tests (not D1 crate sources)** | A new `RE/tests/retained_precision_derivative_golden.rs`; `RE/tests/source_blocks.rs` (N-5's public-API test) |
| **New fixtures** | `P/fixtures/results/retained_precision_successor_derivative_sparse_interactive.json` and `…_dense_scrutiny.json` (the Rust goldens) |

### 2.2 Never touched

- **PP:** every file under `PP/`, including `Cargo.lock` and `examples/`.
- **The D1 crate sources:** every `src/` in Pass B's `crate_dirs` (`R/I65/u4_g7_06/_run_records/chain/crate_dirs.txt`), including RE's reader, `derivative.rs` and `semantic_contract.rs`.
- **Files D1 production code embeds,** including:
  - the 14 reviewed inputs (QUAL §5);
  - `P/schemas/results.v0.3.schema.yaml` (`RE/src/physics_evidence.rs:1005`);
  - `P/schemas/load_reference_state.schema.json`;
  - `P/schemas/units.schema.yaml`;
  - every `semantic_contract_*.json`.
- **The readers and carriers:** `DT/features/results/retainedPrecision.ts`, `retainedPrecisionStanding.ts`, `knownSemanticLimitations.ts`, `P/core/analysis_runs/*.py`, and `P/core/handoff/stress_neutral/*.py`.
- **Checklist item 1's and #885's files:** `P/apps/desktop/src-tauri/**` and `P/tools/ci/e2e_plan.py`.
- **U8's files:** `PP/src/retained_facade_tests.rs`, `P/fixtures/results/retained_precision_cases.json`, `P/tests/test_retained_precision_contract.py`, `RE/tests/retained_precision_contract.rs`, `DT/features/results/retainedPrecision.test.ts`, and U8's L = 0 fixtures.
- **S-I1's rules crates.**
- **The carrier case file** `P/fixtures/results/retained_precision_carrier_cases.json` (v4). The slice reads it and does not edit it.
- **The stress-neutral and AnalysisRun schemas.** CQ-2's recommendation needs no schema change, because diagnostic codes are free strings (`stress_neutral_export.v0.3.schema.json`, `$defs/Diagnostic`).

### 2.3 Conclusion

**With decision 9 as recommended, the slice touches no PP file and no file in the D1 call graph or PP's compiled inputs.** It therefore needs no re-qualification, and no Pass B (gate set item 7 does not apply).

**Flagged (decision 9):** a direct test of RV95 N-5's bound needs `RE/src/source_blocks.rs` (a test-only hunk in a D1-closure file). If ROOT keeps it in the slice, the slice gains:
- a no-build Pass B, in which the hunk classes `test` ("wholly inside `#[cfg(test)]` items", the tool's existing class);
- that Pass B's independent confirmation;
- RR "T3's gate set…" item 7, applied.

Item 7 adds no T9 or both-entry gates here, because no product behaviour changes.

## 3. Contract questions

Each has my recommendation and its decider. Only CQ-10 is owner-facing.

**CQ-1. The result JSON's form for a successor.** Settled by D2 §4.9.7, §4.9.9 and D-U6-2. One open point is the bound's text.
- Rust prints b with `{:e}` (`RE/src/derivative.rs:52`): shortest round-trip digits, with no `+` in the exponent. JS `toExponential()` prints `e+` for exponents ≥ 0.
- **Recommendation:** TS formats b as Rust does (shortest digits, with `e+` normalized to `e`), pinned by the golden and by edge values (b ≥ 1; subnormal b). No new meaning. **ROOT.**

**CQ-2. How classified rows appear in a stress-neutral package,** which has no `row_disclosures` and a fixed CSV column set (`export_profile.csv_columns`).
- **Options:**
  - **(S-a)** info diagnostics only. Classified rows then carry no row-level label in any member.
  - **(S-b)** a successor-only CSV column. That changes the public CSV format and both validators.
  - **(S-d)** use the export's existing convention for rows retained without a physical claim: withhold the row's unit-preservation witness with two new withholding dispositions, `SN-UNIT-WITNESS-WITHHELD-RETAINED-PRECISION-ABSOLUTE-VERIFIED` and `…-NOT-COVERED`. These are info severity, carry the derivative's D-U6-2 message text, and are counted in the loss report's reason. The CSV row and its value stay unchanged, and the receipt travels whole.
- **Recommendation: S-d.** It applies D2 §4.9.9 ("at every binding and export site"; "never appears unlabelled") through a mechanism downstream consumers already honour, as they do for diagnostic-work rows. It needs no schema change. **ROOT.** It would become owner-facing only if ROOT preferred S-a (a departure from D2 §4.9.9) or S-b (a change to the public CSV format).

**CQ-3. Which standing gates an export.** **Recommendation:** whole-envelope `numerically_eligible` only (D2 §4.9.4).
- With one exception: from B1 on, a successor with any `unavailable` case reads `needs_recompute`, and none of it exports.
- That is today's rule. Case-scoped standing (F-P2) stays in T6's later slot. The ordinary envelope for such a request would not be eligible either, so this is no regression.
- **ROOT.**

**CQ-4. Producer origin.** **Recommendation:** no export claims producer origin or W1 attestation (D-U7-6). The desktop origin keeps `authentic_producer_available: false`, and its existing `origin_limit` text already says the bytes are not producer-attested. **ROOT** (confirmation).

**CQ-5. The invocation in exports.** Neither export carries `{request, solver_mode}`, so a downstream reader cannot re-establish eligibility from the export alone, as for physics-source-1 today. Carrying it would change the `results.v0.3` derivative format, a D1-embedded static, which means re-qualification.
- **Recommendation:** no invocation in exports. Record this as a limit; any later need is a breadth or T6 unit with its own re-qualification.
- **Disclose also:** the exported `numerical_quality` is the base statement. For a selected case it records the ordinary route's quality, which is fail-safe: a consumer reading it alone understates standing.
- **ROOT.**

**CQ-6. Values outside the checked profile.** **Recommendation:** keep refusing (`UNSAFE_JSON_NUMBER` / the checked-profile errors) for successors as for every identity. DD-8's binary64 carriers stay in T6's slot. **ROOT.**

**CQ-7. The dispatcher's form.**
- **Options:**
  - **(A)** regenerate the inline v0.3 branch from the version file, localizing its refs, with a drift pin;
  - **(B)** make `oneOf[2]` a `$ref` to `results.v0.3.schema.yaml`, as the stress-neutral and AnalysisRun dispatchers already do, and update the description.
- In either case the external refs mean dispatcher validation needs the registry helper.
- **Recommendation: (B).** It removes the drift that caused F-U6c-2 by construction. The equivalence test pins it. `$defs` (the v0.1 projection, `test_version_sources_and_projected_legacy_definitions`) stays unchanged.
- **ROOT.** This is not a change of meaning: it admits exactly the documents the version file already defines.

**CQ-8. Python.** **Recommendation:**
- Python keeps refusing successor packages (pinned). This is a declared TS↔Python difference, recorded in the slice's PR record. It is not added to the carrier case file, whose scope is standing parity.
- F-U6b-2's Python transport validator joins B6 (reader items) in wider F2a.
- Python admission follows in T6's later slot.

**ROOT.**

**CQ-9. RV95 N-5's placement.** See decision 9. **ROOT** (the owner is informed, since decision 12 listed N-5 in the slice).

**CQ-10 (owner-facing). G10's panel half.** After the slice, "the result-export and stress-neutral panels refusing a successor" is no longer the intended behaviour.
- **Recommendation:**
  - G10 keeps its ordinary-route half.
  - The successor-panel witness moves to B8's native Current witness: the two panels export an eligible successor natively, and the other surfaces refuse it.
  - Until B8, vitest and the hosted browser shards cover the gates, as the G10 ruling allowed.
- **Owner** (native-app witnesses are owner-held).

**CQ-11. The positive result-export witness.** **Recommendation:**
- In the slice: builder-level tests with Rust-golden parity, plus panel tests whose input-manifest evidence is hash-consistent but built by the test, not by the product's builder. Each test is labelled as such, because the pinned D1 model is not desktop-complete (RR:9809).
- The product flow through the manifest builder, with a desktop-shaped successor (which needs a PP run and a PP file), is B8's.

**ROOT.**

## 4. Slices, owners, reviewers, order and estimates

**IDs:** the next unused are I75 and RV101 (RR:12342). I propose two implementers on disjoint files and one fresh reviewer.

| Slice | Owner | What | Estimate (agent) |
|---|---|---|---|
| **T6S-1** | I76 | The dispatcher (CQ-7 B), `test_result_export_v0_2.py`'s registry, and the equivalence test over every committed v0.3 document (the U6c sweep's documents, RR:9775's 24-file sweep). The equivalence also covers T6S-2's goldens | 3–5 h |
| **T6S-2** | I76 | Rust goldens: the successor derivative of both pinned successors, with a fixed desktop-shaped base and origin, pinned by sha256, with the regeneration command recorded. N-5's public-API masking test. One cargo job, through `WT/tools/t3_cargo.sh` | 3–6 h |
| **T6S-3** | I75 | The gate split and the exhaustive output policy; the refusal-text rewording; the two panels' expectation changes | 3–5 h |
| **T6S-4** | I75 | Result export (§1.2): derivative and validator, disclosure texts, golden parity, panel tests. **Negative controls:** copy, moved model, unregistered, load-reference, relabelled, receipt on another identity, forged class code, edited message. **Multi-case controls:** the corpus's synthetic two-case bases, read by id, `two_case_synthetic` eligible and `two_case_facade_after_certificate_synthetic` refused | 7–10 h |
| **T6S-5** | I75 | Stress-neutral (§1.3, CQ-2 S-d): builder and validator, transport header, withheld-witness dispositions, panel test, the same negative and multi-case controls. Mutant sets per new branch | 7–10 h |
| **Review** | RV101 | Fresh complete-diff review, then same-reviewer confirmation of repairs | 6–9 h + 1–2 h |
| **Gates and merge** | ROOT | RR "T3's gate set…" (RR:12262), items 1–6 (§4.2) | ROOT 4–6 h; machine 2–3 h |

### 4.1 Order

1. ROOT rules on §6, and brings decision 10 to the owner.
2. I76 (T6S-1, T6S-2) and I75 (T6S-3) start in parallel.
3. I75 runs T6S-4 once T6S-2's goldens exist, then T6S-5. T6S-5 can overlap T6S-4, since it touches different files.
4. RV101 reviews; repairs; confirmation.
5. The gates, then the merge.

**It runs alongside breadth,** off its critical path. Its only hard deadline is to merge before B8. Its cargo use is one short golden job and the RE suites, under the host lock.

### 4.2 How it reaches main

**Its own compact product PR.**
- **The branch:** `codex/piping-t6-successor-outputs-<date>`, cut from main in its own worktree `WT/t6-outputs`, as S-I1 was. Records live on NUM.
- **The gates:**
  1. maintained-source equality with NUM after ROOT integrates the slice there;
  2. RV101's review;
  3. the full 40-manifest suite before the freeze;
  4. hosted CI with the full-SHA dispatch;
  5. GEN-8 by E-4's method;
  6. the exact-head Mac DEC-025 through `run_dec025.sh` against a fresh main baseline.
- **Merged with** `--merge --match-head-commit`, after checking that main has not moved.
- **Merge order:** U8 and S-I1 share no file with it, so whichever merges second absorbs main without conflict.
- **If decision 9 keeps N-5's unit test in-slice,** add item 7: a no-build Pass B and its confirmation.

### 4.3 Staying consistent as B1–B3 widen the receipt

- **Output code never parses the receipt.** It copies the receipt whole. It takes classes only from the reader and standing only from the carriers. It assumes no case count, and its tests include two-case bases.
- **B1** (multi-case; RV78-N1's re-pin cascade): regenerate T6S-2's goldens if the pinned successors change, and rerun the T6S suites. A changed disclosure meaning returns to ROOT. Add this to B1's brief.
- **B2** (combinations): stress-neutral `load_case_ref` and AnalysisRun basis refs for combination cases must be checked on a successor. Add it to B2's brief, together with the T6S suites.
- **B3** (`physics-retained-1`): the exhaustive policy will not compile until B3 adds the route's entry (semantic table, `contract_evidence` kind, transport validator), and until then the route refuses. B3 already carries the schema branch and re-qualification (I61 §2.1).
- **S-I2:** D-U6-2's text "withheld from rule binding and reliance" becomes inaccurate for `absolute_verified` once intervals bind. The text lives in one constant per language (Rust `class_disclosure`, the new TS module and the stress-neutral finding), so S-I2's plan changes all three together. Add a note to S-I2's planning.

## 5. Interactions

- **S-I2.** It runs after B8, so it does not interact with this slice's schedule. Its consequences:
  - the shared disclosure texts (§4.3);
  - the per-case summary line, which reads `classificationSummary` and so follows S-I2's `interval_bindable` counts automatically;
  - the derivative's disclosure itself stays (D2 §4.9.9: "adds one row_disclosures entry" regardless of S-I).
  
  The T6S files do not overlap S-I2's planned files (`RE/src/semantic_contract.rs`, `compatibility.py`, `knownSemanticLimitations.ts`).
- **S-I1** (rules crates): disjoint.
- **Checklist item 1** (`qualify_rule_mechanics_with_context`, `P/apps/desktop/src-tauri/src/lib.rs:2794`): disjoint files, with two coordination points.
  - **(a)** The Rule-check panel's successor refusal is the shared gate (`RuleCheckPanel.tsx:54`). Item 1 opens it deliberately, by one entry in T6S-3's policy, under item 1's own review.
  - **(b)** Item 1's `lib.rs` edit selects the numerical crate suite through `NUMERICAL_APP_INPUTS`. T6S does not.
- **PR #885** (draft, last updated 2026-09-25):
  - It touches `src-tauri/Cargo.toml`, `src-tauri/src/lib.rs` (module declarations and `run()`), three new src-tauri modules, `workspaceSession.ts` and `BatchReviewPanel.tsx`. None of these is in T6S's write set, and none is an output surface.
  - It does not interact with this slice.
  - **For B8 and item 1:** if revived, it edits PP's caller-separation guard input (`lib.rs`), so it should be serialized with item 1 through main. It adds no Direct caller, from my reading of its `lib.rs` hunk.
- **U8:** disjoint files. If L = 0 publishes, its new successor fixtures are extra positive inputs T6S may read; nothing depends on them. The corpus is read by case id, not pinned by file hash, so 07l does not break T6S.
- **B8:** after the slice, activation's checklist item 4 is satisfied, except the native panel witness (decision 10) and the product-flow manifest export (CQ-11), which join B8's native witness.

## 6. Decisions

Each decision has my recommendation and its decider. Only decision 10 is owner-facing; none of the others is on the owner-held list in the work graph's T3 section.

1. **The scope** is §1: desktop (TS) result export and stress-neutral export, the dispatcher, and RV95 N-5. The rest of T6 keeps its slot (§1.6). **ROOT.**
2. **The gate:** split the shared refusal, with an exhaustive, surface-named output policy. Only the two panels admit `retained_preview_physics`, and only at eligible standing with the live capture. The other 18 surfaces and the report keep refusing, and unknown routes fail closed. **ROOT.**
3. **No activation switch.** The panels admit eligible successors on merge. They stay dormant in the product until B8 because no product caller exists and TS standing needs the live native capture. The alternative is a closed switch that B8 flips. **ROOT.**
4. **Result JSON** = Rust `derive_document`'s successor form (D2 §4.9.7, D-U6-2). TS classes come from the reader without an invocation; byte parity is proved by Rust goldens; b is formatted as Rust formats it (CQ-1). **ROOT.**
5. **Stress-neutral classified rows:** S-d (CQ-2), withheld unit witnesses with two new codes, D-U6-2's text and the receipt whole, with no schema change. **ROOT.** It becomes owner-facing only if ROOT chooses S-a or S-b.
6. **Standing and claims:** whole-envelope standing only (CQ-3); no producer-origin claim (CQ-4); no invocation in exports (CQ-5); out-of-profile values keep refusing (CQ-6). **ROOT.**
7. **The dispatcher:** `$ref` to the v0.3 version file (CQ-7 B), with registry-based tests and an equivalence test. **ROOT.**
8. **Python:** keeps refusing successor packages, as a declared difference. F-U6b-2's transport validator goes into B6; Python admission comes later in T6 (CQ-8). **ROOT.**
9. **RV95 N-5 placement.**
   - In the slice: the public-API masking test, with S1 recorded as equivalent at the public API.
   - The direct unit test of `integer` (an appended `#[cfg(test)]` module in `RE/src/source_blocks.rs`) rides with **PR-B1**. PR-B1 edits `result_export` sources and runs Pass B anyway. U8's binding rule forbids reader `src/` text, so U8 is not a home without amending that rule.
   - **The alternative:** keep the unit test in the slice and add a no-build Pass B and its confirmation (+3–4.5 h; gate item 7).
   - **ROOT.** Inform the owner, because decision 12 listed N-5 in the slice. The obligation still closes before B8.
10. **(Owner-facing) G10:** keep its ordinary-route half, and move "the panels with a successor" into B8's native Current witness (CQ-10). **Owner.**
11. **The positive witness:** test-built, hash-consistent manifest evidence for the pinned model, labelled as such, plus golden parity. The product-flow manifest export of a desktop-shaped successor is B8's (CQ-11). **ROOT.**
12. **The refusal text** for the remaining surfaces is reworded, for example "This output of retained-precision results is not yet available on the desktop; result JSON and stress-neutral export are. It is routed to T6. …". It stays display-only, and the existing assertions change only in the expected text. **ROOT.**
13. **Delivery:** I75 (TS) and I76 (schema, Python and Rust tests), fresh RV101; its own branch from main and its own product PR with gate items 1–6 (§4.2). It runs alongside breadth and merges before B8. **ROOT.**
14. **Consistency notes** go into the B1, B2, B3 and S-I2 briefs (§4.3), and the Rule-check panel's entry into item 1's brief (§5). **ROOT.**

## 7. Estimate summary

| | Estimate |
|---|---|
| **I75** (T6S-3, -4, -5) | 17–25 h |
| **I76** (T6S-1, -2) | 6–11 h |
| **Agent total** | **23–36 h** |
| **RV101** | **7–11 h** (review 6–9 h, confirmation 1–2 h) |
| **ROOT** | 4–6 h (rulings, verification, gates, merge), plus 2–3 h machine (two Mac DEC-025 runs, CI, dispatch) |
| **Decision 9's alternative** | +3–4.5 h |
| **Elapsed** | About 3–5 working days with I75 and I76 in parallel, off breadth's critical path |

These are reading estimates of the same kind as I61's. The largest uncertainty is T6S-4's golden parity: the TS derivative's field order and text against Rust.

## 8. What I read, and limits

**Read** (sha256 prefixes where hashed):
- `NUM/AGENTS.md`, `NUM/agents/AGENT_TASK.md` and `P/AGENTS.md`;
- the brief (`79602972…`);
- RR: 9439–9470, 9750–9830, 9895–9935, 10000–10035, 11200–11300, 11580–11620, 11690–11705, 11840–12000 and 12000–12342;
- CR §1, §3 (U6, U7, after the cut), §4 and §5 (`f4207994…`);
- QUAL §1, §5–§11 (`8edbf4b4…`);
- the work graph's T6 row and the "T3 current route" section (`6fa30869…`, at NUM `95d3cae8db`);
- I61 §0 and §2 (`f274a614…`);
- D2 §4.6.2 and §4.9.4–§4.9.10 (`edc78f9c…`);
- RV95's N-5 row (`R/REVIEW_RV95/u9_01/REVIEW.md:52`, `:169`);
- I65's no-build Pass B RETURN (`R/I65/u9_refreeze_01/RETURN.md`), and `u4_g7_06`'s `crate_dirs.txt`, `delta_inventory.json` and `statics.json`;
- `BRIEFS/U8_COMMON.md`, `I68_U8_PROBE_AND_WITNESSES.md`, `I69_I70_I71_U8_CORPUS_AND_READERS.md` and `I72_U8_PASS_B.md` (fences only);
- the code and schemas cited above. `P/schemas/retained_precision_mp_v2.schema.json` (`07951eda…`) equals its reviewed-input digest (QUAL §5);
- the carrier case file v4 (`cf82deab…`), structure only;
- the corpus, base shapes only;
- PR #885's metadata and its `lib.rs` hunk.

**Limits:**
- **Nothing was run.** Every behavioural claim is from reading source.
  - Those that matter most are §0 items 7 and 8. Item 7 says the bound is masked at the public API; item 8 says manifest verification is hash-only.
  - Each can be checked cheaply. For item 7: a 2^53 receipt integer fails with `RECEIPT_SHAPE`, and a resealed 2^53 summary count fails at the publication hash.
- **I did not read** RE's full reader or `derivative.rs` beyond the cited regions, or the TS validators' full bodies.
- **The golden-parity estimate** assumes the TS and Rust derivatives agree for existing identities on identical inputs. That is not established by a shared test today.
- **#885** was read at its head on origin. I did not assess its merge state against current main.
