# RV78 review: shared retained-precision artefacts, corpus and joint parity (snapshot 06d)

RV78 is a TASK (Type 2), an independent reviewer dispatched by ROOT (HELP_HUMAN, Agent 0) under `R/BRIEFS/RV78_RV81_READER_REVIEW.md`. ROOT is the return path. RV78 had no descendants and wrote none of the code or corpus under review.

- **Candidate:** READER branch `codex/piping-f2a-readers-20261003`, head `6b607fd01f9819a3b6526dd9fde02cd3bc4db586`, reviewed from RV78's own `git archive` in `WT/rv78/` (deleted afterwards). NUM was at `6a5b131b98bca1a1ef8acf0ddf66a6c11722cb15` for instructions, contracts and native code.
- **Shared files reviewed:** corpus `P/fixtures/results/retained_precision_cases.json` (`d02701ed6a…`), schema `P/schemas/retained_precision_mp_v2.schema.json` (`f943ebd351…`) and definition `P/fixtures/results/retained_precision_prepared_ordinary_v1.json` (`3e0779a45a…`). All three match SHARED_SNAPSHOT_06D.
- **Run window:** 2026-10-03, 16:47:26 to about 17:19 local (MDT). That is inside the two-hour box, and nothing in the brief was left undone. The memory guard (PID 5387) was running throughout.
- **Limits held:** RV78 made no Git writes or index operations (Git reads only, with `GIT_OPTIONAL_LOCKS=0`). There was no install, no new tooling, and no native, solver or DEC-025 job. RV78 wrote only to this folder, `WT/rv78/`, `WT/targets/rv78` and `WT/scratch/rv78_reader_review/`.

## Verdict: FAIL

**Counts:** 1 BLOCKING, 4 SHOULD-FIX, 9 NOTE.

On the shared corpus itself, parity is complete. All three readers produce the expected first gate and code on all 178 mutations (G7 per reader), accept all 18 must-pass entries, and validate all 15 cases with the expected classifications. Every expectation RV78 sampled matches the contract.

The FAIL comes from what the corpus does not yet pin. RV78 contract-derived probes show the following:
- five reader-specific over-acceptances of receipts the producer cannot emit, four of them on **selected** cases;
- a sixth over-acceptance whose contract reading ROOT must confirm;
- seven first-gate divergences;
- one class of non-emittable cause that all three readers accept.

No shared entry exercises any of these. Until they are pinned and the readers aligned, the corpus cannot support a three-reader parity claim (C1 §6: "All readers execute G0→G8 in the same order and consume one shared positive/mutation corpus").

## Findings

| ID | Severity | Where | Evidence | Remedy |
|---|---|---|---|---|
| B1 | BLOCKING | The shared corpus (no entry). Readers: P/core/analysis_runs/retained_precision.py, P/core/reporting/result_export/src/retained_precision.rs, P/apps/desktop/src/features/results/retainedPrecision.ts | **Non-emittable receipts accepted by some readers, with no shared pin** (PROBES.json, batches 1 and 6):<br>• `T1_escalating_failed_verification_pass_entered` (selected): Python and Rust PASS; TypeScript G5 ATTEMPT.<br>• `T2_stop_rule_quantity_other_body` (selected): Python and Rust PASS; TypeScript G5 ATTEMPT.<br>• `T3_candidate_record_with_verification` (selected): Python PASS; Rust and TypeScript G5 ATTEMPT.<br>• `R6a_native_error_with_selected_run`: Rust PASS; Python and TypeScript G5 PRODUCT_ATTEMPT.<br>• `T4a_ordinary_diagnostic_ref_duplicate` (selected): Python PASS; the others G5 ATTEMPT.<br>• `T4e_selected_case_ordinary_checks_passed` (moderate confidence; see Known differences T-4c): Python and Rust PASS; TypeScript G5 ATTEMPT.<br>Native and contract basis: see Known differences R-6, T-1 to T-4. | Add these as shared mutations; PROBES.json holds the exact edits and expectations. Align Python and Rust (and Rust for R-6a). ROOT confirms the T-4c reading. |
| S1 | SHOULD-FIX | The shared corpus (no entry) | **Different first gates, no shared pin** (batches 1 and 2):<br>• `R1a_execution_order_swapped` and `R1b_run_id_not_position`: Python G5 ATTEMPT, the others G3. The contract says G3.<br>• `R2b_unsourced_old_member_noncontiguous`: Rust and TypeScript G8, Python G3. The contract says G3.<br>• `R3b_complete_old_longer_than_source`: Python G5 PRODUCT_ATTEMPT, the others G3. The contract says G3.<br>• `R4_unavailable_source_backref_foreign` and `R5_attempt_and_source_basis_not_ordinary`: Python G8, the others G5 PRODUCT_ATTEMPT. The contract says G5 PRODUCT_ATTEMPT.<br>• `T4d_ordinary_dangling_plus_adapter_fault`: Python G5 WORK, the others G5 ATTEMPT. The contract says ATTEMPT. | Add these as shared mutations with the contract gates (Known differences R-1 to R-5, T-4e). Align the named readers. |
| S2 | SHOULD-FIX | The 06d rules R1–R3 as ruled. Implemented at retained_precision.py:211–232, retained_precision.rs:1523–1550 and retainedPrecision.ts:494–503 | ROOT accepted the ACCOUNTING_CAUSES class facts: product work-status faults are representable only with a non-exact owning status, and lost and adapter faults are never emittable. R3 matches only `{kind:"work_accounting",fault}`, and R1 only `{kind:"accounting",event}`. **All three readers accept** four same-class shapes (batches 3 and 4):<br>• `B_section_accounting_exact_status`: SectionError `{kind:"accounting"}` with an exact PreparationWork status. Native emits it only on a non-exact status or a broken invariant (FK/product_certificate.rs:676–679, 688).<br>• `Q4_old_operational_accounting_not_lost`: OperationalError `{kind:"accounting"}` arises only with `lost` (PP/retained_product.rs:2311–2347), so it is never emittable.<br>• `Q1_nested_stop_work_accounting_exact_status`: the AttemptStop spelling `{space:"stop",tag:"work_accounting",fault}` nested in a NumericError arithmetic cause (FK/adaptive.rs:200–201).<br>• `Q2_view_work_fault_exact_status`: ViewIssue `{kind:"work",fault}` (FK/adaptive.rs:5279, 5468, 5572).<br>No positive case or must-pass entry contains any of these shapes. | ROOT to rule an extension. R3′: every fault-bearing cause, whatever its discriminator spelling, needs its fault in the attempt's emitted statuses. R2′: OperationalError and G5aError-operational accounting is never emittable. R4: a SectionError accounting refusal needs the member's PreparationWork status to be non-exact. All at G5 WORK. Add shared mutations, and rebase `prefix_attached_old_input_unbound` first (N2). |
| S3 | SHOULD-FIX | P/schemas/retained_precision_mp_v2.schema.json:976 (Refusal), variants at :1109 (`work_accounting{fault}`) and :1135 (`count_range{name}`) | Native `Refusal` has exactly five variants (FK/adaptive.rs:2860–2875), and C2:41–45 lists the same five. C1:114 requires the reason enum frozen against the native enums, with no unreachable or catch-all variant. The closed type therefore admits two non-native refusal shapes at G1. No shared entry exercises either one. | Remove both variants from the schema, so they fail G1, or cite the native or contract source that admits them. Add a G1 mutation. |
| S4 | SHOULD-FIX | The shared corpus | **Checklist IDs with no negative shared pin.** The following are exercised only by positive bases, or not at all:<br>• P8: no mutation edits a case `reason`, and R-6a shows a real gap;<br>• C6 (divergence R-1);<br>• N13, including the pre-06d Rust defect (a rejected attempt with a `verified` record) that I63 found by reading, plus T-3;<br>• O1, W3, N1, N7, N11 and W2.<br>Details are in "Checklist coverage". | Add a code/phase-mismatch mutation on F′ and on P′, an execution-order swap, an ordinary-attempt order swap, a preparation conversion-count mismatch, role/outcome mismatches, and a record-count overflow. |
| N1 | NOTE | Corpus `interpolation_target_{below_range,at_lower_point,at_upper_point,above_range}` (corpus line 121984 and neighbours) | The pins do not isolate the strict-bracket rule. The receipt keeps E = 200e9, so an E mismatch fires the same G8 PREPARATION even without the rule. With both point moduli set equal (batch 3), all three readers still fail G8, so they do enforce the rule (native lib.rs:9238–9239 is strict). The control `B_control_equal_point_E_bracketed` passes. | Replace or add the equal-E variants. |
| N2 | NOTE | `prefix_attached_old_input_unbound` (line 121014) and `refused_member_conversion_kind_bits` (line 120733) | Both sit on a non-emittable SectionError-accounting context, which 06d acknowledges. If S2's R4 is adopted, the first of them moves from G8 to G5 WORK. The second stays at G5 PRODUCT_ATTEMPT, which precedes WORK. | Rebase the context onto an emittable refusal when a producer witness exists, or defer the entry, before adding R4. |
| N3 | NOTE | Schema unavailable Case, `source_ref` optional *and* nullable (schema:6834, in Case at :6663); the Case branch `required` omits `source_ref` and `source_decline` | `S2_unavailable_source_ref_null_instead_of_absent` passes in all three readers. "No source" therefore has two encodings with different receipt hashes. C1:92 requires absent and null to be "distinguished as stated", and no rule states which applies. The schema also admits `source_ref` and `source_decline` together. | ROOT/contract: choose one representation (for example required `null\|U`) and make `source_decline` exclusive of `source_ref`. |
| N4 | NOTE | Schema `U`/`Bits` definitions (`x-rp-encoding`) | Run as plain JSON Schema, it fails the G2 mutations `coverage_body_fraction` and `coverage_body_negative`, which I57's G2 row places at G2. The readers correctly split shape (G1) from encoding (G2) through `x-rp-encoding`. | State in the schema's `$comment` that the integer and bit constraints are G2. |
| N5 | NOTE | Harnesses: tests/test_retained_precision_contract.py:97–104, result_export/tests/retained_precision_contract.rs:107, 153, retainedPrecision.test.ts:131 | For a `rehash` other than `"all"`, Python rehashes only the publication and receipt, Rust rehashes nothing, and TypeScript rehashes everything. Rust's `edit` also panics on an array `remove`. This is latent: every entry uses `"all"` and every remove targets an object key. | Fix the format: `rehash ∈ {"all"}` only, or define the other values for all harnesses. |
| N6 | NOTE | All 15 bases | The product work counters are mostly zero attestations. The numeric schedule triggers (Pivot, Condition, stop-rule rejections) on bases K, V, L and S are private synthetic attestations that these cantilever models would not reach natively. The 06a ruling accepts this boundary, and readers cannot detect it, so the corpus proves reader logic, not producer reachability. The storage-only rule (06d) is met: every resource trigger in the bases and must-pass entries is an allocator refusal at a cited reserve site (checked: PP:3456–3560 project_candidate, bridge.rs:135/679, source_residual.rs:34, FC:1464). | None needed; keep the labels. |
| N7 | NOTE | C1:148 and the checklist's B/E tables | C1 permits ATTEMPT or WORK for G5 native checks but does not assign one per check. The corpus and checklist fix the split by convention: for example, `failed_build_reason_mismatch` is ATTEMPT while `failed_slot_not_cached` and `cached_failed_slot_rebuilt` are WORK. The order between native group/build checks and per-run replay is also undetermined (Known differences R-7). | Record the per-check code mapping and the native in-gate order in the C3 clarification (ROOT). |
| N8 | NOTE | Schema Unresolved/Stop `work_accounting{fault}` (:599, :934); ProductError (:1866) | **The WorkAccounting tension, RV78's reading:** it is coherent. C3:262 imports the native AttemptStop/Unresolved mapping including WorkAccounting, so the closed type may keep it. C1:66–68 is an emission prohibition, enforced at G5. `prior` is correctly off the wire (C3:262–263, pinned at G1). The narrowing must be uniform, though (S2), and the non-native Refusal variant is a separate defect (S3). | Clarify in C3 that the type domain may exceed the emitted domain and G5 enforces the latter. |
| N9 | NOTE | R/I63/reader_align_06d/OUTCOMES_06D.json | I63's file tabulates 148 of the 178 mutations; the other 30 are asserted only in `shared_rehashed_first_failure_mutations`. RV78's own run covers all 178. Both authors' outcome files agree with RV78's observations wherever they overlap: 0 differences. | None. |

## 1. Schema

Checks were made against C1 §4, C3 §2–3, F1, S06–S08 and I57 §1. The full audit is in INDEPENDENT_CHECKS.json.

- **Closed shapes:** every object definition has `additionalProperties:false`, confirmed by a walk of the schema. Every property is required except three:
  - the unavailable Case's `source_ref` and `source_decline` (N3);
  - `Body.legacy_source_work`, which is optional as C2:160 allows;
  - the RawRow optional base fields, as S07 specifies.
- **`summary_coverage`:**
  - it is required and `null` or an array of closed `{body:U, stop:[bool;4], has_data:bool}`, as I57 §1 and its G1 row require;
  - there is no `minItems`, so a non-null empty roster reaches G3, as ruled. Body order is not expressible in the schema and is checked at G3.
- **Validation:** jsonschema (Draft 2020-12) validates the `retained_precision` member of all 15 cases and 18 must-pass entries. 15 of the 21 G1 mutations fail the schema itself. The other six all carry a raw-row `recovery_method` shape defect, which the RawRow definition covers (`test_retained_precision_schema.py`) and the readers reject at G1.
- **The WorkAccounting tension:** see N8.
- **Defects:** S3 (non-native Refusal variants). N3 and N4 are notes.
- **Not reviewed:** RV78 did not review the `results.v0.3.schema.yaml` successor branch beyond the existing schema test, which passes.

## 2. Corpus

### Sample

RV78 read every 06a–06d mutation (58), 14 earlier mutations across the 04–05c families, and all 18 must-pass entries: 72 mutations and 18 must-pass entries in all. For each, RV78 derived the expected first gate and code from C1 §6 and C3:294–304 with I57 §4, S06–S08 and the ROOT rulings, then compared that with the corpus.

**Every sampled expectation matches the contract reading.** In detail:
- **Gate-major order across cases:** `cross_case_gate_order_selected` and `cross_case_gate_order_unavailable` follow C3:294 and C1 §6.
- **Within G5, association before work:** `maxima_abandoned_separate_failure`, `certificate_check_wrong_wrapper` and `cert_failed_before_summary_g5a_passed` each carry a later R1/R3 context defect, and PRODUCT_ATTEMPT correctly wins (C3:304).
- **G6 versus earlier gates:** the method-token dual defects follow the S07 matrix, and `method_missing` is G6 as S07 requires.
- **WorkAccounting terminals:** rejected at G5 ATTEMPT (C1:66–68, C1:148).
- **The idle rules:** follow adaptive.rs:4994–5016 and 5055–5075.

### Native fidelity

- **Mutations:** each sampled mutation describes a receipt the producer cannot emit, so rejecting it is correct. Checks against the native code:
  - **N5:** adaptive.rs:4349–4378 (`terminal()`).
  - **N3 and N4:** adaptive.rs:4542–4552, 4576–4589.
  - **P6:** retained_product.rs:3476–3491, where a maxima, aliases or bind-rows failure gives `abandon_values` (merged) and a values failure gives `abandon` (separate_failure).
  - **P9:** retained_product.rs:3502–3508, where observables and G5a run after a certificate refusal only if the verdict copy succeeds.
- **Must-pass entries:** each is either emittable through an allocator refusal at a cited reserve site (F′, P′, the six storage replacements, `cert_failed_*`), or is an I57/F1 attested variant that the contract deliberately leaves publicly undetectable.
- **Cross-checked by RV78 in the native code:**
  - the F′ path: certificate passed, then the verdict-copy reserve was refused, giving Capture(storage) at PP:3522–3524, before Observables at :3530. Stage shape `certificate=completed`, observables and G5a not entered;
  - the P′ path;
  - the strict interpolation bracket (lib.rs:9238–9239);
  - the 301 K interpolation, which equals 200e9 exactly in binary64.
- **Known weaknesses:** N1, N2 and N6 (notes).

### Rehash integrity

- **RV78's own implementations:** RV78 wrote its own edit applier, JCS canonicalization (ES6 number form, UTF-16 key order) and hash payloads, from C1 §3, C2 §3 and C3 §2. They reproduce every stored hash in all 15 cases (receipt, publication, invocation, preparation, source identity) and the definition hash `a7ed7ca0bf…`.
- **Agreement with the authors' harness:** RV78's rehash of all 196 entries is byte-identical to the authors' Python `apply_entry`.
- **K4 digests:** `rehash:"all"` does not recompute the K4SRC/K4STF digests. Two source-map mutations change K4 inputs: `layout_nonzero_prescription` and `maps_member_ends_swapped`. RV78 re-ran both with the digests recomputed and the group stiffness updated (batch K4). All three readers give the same first failure, so no pin depends on a stale native digest.

### The 15 cases under an independent reading

RV78 ran its own checks over the 15 cases and the 18 must-pass entries (INDEPENDENT_CHECKS.json). There were **zero findings**:
- **schema:** per §1;
- **hashes:** per "Rehash integrity";
- **work:** the C1 §1 partition per record, fragment and attempt; the native-versus-logical totals; invocation chaining across runs and calls; `work.charged`; and the execution order;
- **build provenance:** the built flag against the build origin, and build work against stages and the shared amounts;
- **I57 §2/§4 coverage:**
  - the roster runs 0..n−1;
  - the feasibility rule is enumerated over all sixteen A vectors;
  - the estimate and charge derivation at p128/p256 against p512;
  - the stop/estimate/charge/B rosters;
  - the verification `bound` against `has_data`, and the bounds 2^-64, 1/4, 1 and 1/2;
  - the data facts;
- **p512 floors:** every Φ is recomputed as the least binary64 ≥ 2^-438·ê, with ê from verify.rs:20–30 using exact rationals. All 5 floor entries match.

As a sanity check, the same checker flags 76 of the 178 mutations, including 35 of the 49 G5a and 5 of the 12 G5 WORK. It is a partial reading, not a fourth reader.

## 3. The joint parity run

RV78 ran all three readers' test commands from its archive of `6b607fd01f`:

| Reader | Command result |
|---|---|
| Python | pytest: 221 passed |
| Rust | 23 passed, 0 failed (`DEVELOPER_DIR=/Library/Developer/CommandLineTools`, disclosed) |
| TypeScript | vitest 271/271; `tsc --noEmit` exits 0 |

For per-entry outcomes, RV78 prepared every entry independently (§2). It then called each reader's own validate entry directly:
- Python: `rp._validate_draft`;
- Rust: `rp::validate`, through a probe test placed only in the review copy;
- TypeScript: `validateRetainedPrecision`, through a probe vitest file placed only in the review copy.

Results (PARITY_TABLE.json):
- **211 of 211 entries match the expectation in all three readers.** That is 15 cases, 178 mutations and 18 must-pass entries.
- **Disagreements between readers on corpus entries: 0.** G7 is compared per reader.
- **Entries where all three agree but RV78 believes the expectation is wrong: none in the corpus.** Outside the corpus, the S2 probes are cases where all three agree with each other but are wrong against the accepted class facts.

## 4. Checklist coverage

The checklist (READER_AUDIT_PLAN Part 1) holds 43 IDs (17+6+5+11+4), not the 42 quoted. The status below is from RV78's reading of the 06d entries. "Pinned" means a negative shared entry fails if the rule is absent.

- **Pinned by shared entries (26):**
  - **N2:** `schedule_fresh_first_p256`.
  - **N3:** `skip_after_failed_candidate_reused`, `_wrong_slot`.
  - **N4:** `failed_verification_reused_as_candidate`, `_one_slot`. Solve versus pass is not pinned: T-1.
  - **N5:** `terminal_stop_wrong_translation`, `terminal_refusal_wrong_kind`, `escalating_end_not_a_terminal_translation`.
  - **N9:** `ceiling_before_last_slot`, `work_accounting_after_escalating_stop`, `work_accounting_at_last_slot`.
  - **N10:** the four `idle_*` entries.
  - **N14:** `corrections_above_three`, `physical_residual_basis`.
  - **N15:** `certified_bound_unbound_drop_existing_g5`.
  - **N16:** the three `physical_*`/stage-projection entries.
  - **C1:** `failed_build_reason_mismatch`, `cached_failed_slot_rebuilt`.
  - **C3:** `failed_slot_not_cached`.
  - **C5:** `group_sources_out_of_order`, `distinct_stiffness_merged_group`.
  - **O2:** `formation_d5_dangling_diagnostic`, `report_reference_unrelated_diagnostic`. List-level duplicates are not pinned: T-4a.
  - **O3:** `w2_published_without_initial_failure`, a dual defect.
  - **O4:** `legacy_source_dangling_diagnostic`.
  - **P1:** `rebind_source_run_only`, `attempt_owner_only`. The back-reference and material basis are not pinned: R-4 and R-5.
  - **P2:** `stage_entered_after_failure`, `certificate_stage_check_disagree`.
  - **P3:** `lane_k_failed_with_coverage`, plus must-pass `lane_k_failed` and `lane_source_failed`.
  - **P4:** the four `row_index_*` entries.
  - **P5:** the five `conversion_*` entries and `refused_member_conversion_kind_bits`.
  - **P6:** `maxima_abandoned_separate_failure`, plus four must-pass entries.
  - **P7:** `prefix_attached_old_input_unbound`, `member_prefix_coverage`, `prefix_captured_with_members`, `old_inputs_must_bind_old_J`. The index readings are not pinned: R-2 and R-3.
  - **P9:** `certificate_check_wrong_wrapper`.
  - **P10:** five coverage entries and two must-pass entries.
  - **P11:** `native_stage_disagrees_with_run`.
  - **W1:** `product_work_only`, `coverage_null_and_product_work`.
- **Partly pinned (6):**
  - **N6:** reuse without a prior verification is pinned; non-immediate reuse is not.
  - **N8:** `work_accounting_at_last_slot` is pinned; the positive Ceiling is deferred.
  - **N12:** the `prior` entry at G1 is pinned; the reason-to-layout link (T-2) and the non-native Refusal variants (S3) are not.
  - **N17:** sums are pinned (`native_work`); overshoot and the final guard are deferred (≥20B/60B).
  - **C2:** slot naming is pinned; build work = Σstages and origin ordering are not isolated.
  - **C4:** run charge is pinned; there is no call-chaining or run_refs negative.
- **Positive bases only, or no shared control (9):** N1, N7, N11, N13, C6, O1, P8, W2, W3. These are S4.
- **Deferred; reader-local only (1):** O5.
- **Not publicly checkable (1):** W4.
- **R1–R3 (06d):** pinned by `adapter_fault_present`, `accounting_cause_without_fault`, `scalar_trace_lost_unavailable` and `work_accounting_cause_exact_status`. The class is incomplete (S2).

**The important gaps with no shared pin** are P8, C6, N13, the T-1/T-2 schedule rules, the P1/P7 association readings and the within-G5 ordinary-before-work order. RV78 shows a reader divergence on each (B1, S1, S4).

## 5. Known differences (contract readings)

RV78 built each probe from a corpus base with its own harness, rehashed it fully, and ran it on all three readers. The observed column reads Python / Rust / TypeScript. "Pinnable" means a faithful shared mutation exists: an emittable receipt, or the correct rejection of one that cannot be emitted.

| # | Difference | Contract reading (first gate and code; basis) | Pinnable? | Reader(s) wrong | RV78 probe: Py / Rust / TS |
|---|---|---|---|---|---|
| R-1 | Run-id contiguity and `work.execution_order` | **G3 COVERAGE.** C1:146 puts the "execution-order bijection" at G3; C2:117 says "each nested run.id equals its execution-order position"; see also C2:131. | Yes: R1a and R1b on `two_case_synthetic` | Python | G5 ATT / G3 / G3 (both probes) |
| R-2 | Member indices at G3 | **G3 COVERAGE.** F1 R3: helper and new arrays are "entered prefixes of the same member order"; complete old ids equal the inventory; "G3 checks the declared complete/prefix inventory and ordered helper/new overlap". C3:126–127 and :302; C2:98 has kernel_member = model_index, so the ids are 0..k−1. | Yes: R2b (on P′, old member 1); R2 (swap) gives G3 everywhere | Rust, TypeScript | G3 / G8 / G8 |
| R-3 | Old coverage against the source | **G3 COVERAGE** for sourced attempts (F1: "For old_coverage=complete, old ids exactly equal the full member inventory", at G3). For an unsourced complete old list, M is not bound by the attempt itself: **ROOT to rule** (another CaseSource of the same model, or G8). | Yes for sourced attempts: R3b | Python | G5 PA / G3 / G3 |
| R-4 | Source preparation back-reference | **G5 PRODUCT_ATTEMPT** for every attempt with a source, Ready or not (C3:146–149; the C3:304 association class) | Yes: R4 on F′ | Python | G8 / G5 PA / G5 PA |
| R-5 | Material basis against the ordinary attempt | **G5 PRODUCT_ATTEMPT** (C3:165 "agrees with owner, ordinary attempt and material basis"; C2:149) | Yes: R5 on `two_case_two_groups` | Python | G8 / G5 PA / G5 PA |
| R-6 | P8 reason-table edges | **(a)** A `native` error with a selected Run is invalid; G5 PA. S06 §1 row: "native selected is invalid for this error". **(b)** A native error's `run_ref` must be the same Run (S06: "Non-null same Run"); Rust is right. **(c)** A preparation error needs `run=null` and preparation failed (S06 row preparation); Rust is right. | (a) yes: R6a on F′. (b) needs a base with a nonselected native Run (none; deferred). (c) not probed; probably masked by the P1/P2 rules. | (a) Rust; (b) and (c) Python, per I63 (TypeScript not compared by I63; not probed by RV78) | (a) G5 PA / **PASS** / G5 PA |
| R-7 | Order of checks inside native G5 (dual ATTEMPT+WORK defects) | **Undetermined by the contract.** C1:148 lists both codes without an order. C3:294–304 orders the classes, and indexes only attempts, members, lanes and rows; native group and build checks have no such index. | Not until ROOT fixes an order | Neither, by the contract | — |
| R-8 | Rust-only group checks (call exists; sources unique and in the call) | **G5 ATTEMPT** (C2:119, :135) | Yes for single defects: R8 | None for single defects; dual defects fall under R-7 | G5 ATT / G5 ATT / G5 ATT |
| T-1 | Failed verification: solve versus pass (N4/N5) | **G5 ATTEMPT.** An escalating stop on a failed verification must be a solve failure. A pass failure goes to `finish_terminal`, and `terminal()` is unreachable for escalating stops (adaptive.rs:4593–4610, 4377). A solve failure has `verification_lme`=0 and no verification build: `solve_precision` initialises them (:4076–4079), and only `verify_precision` sets them (:4286–4333). C1:31, :148. | Yes: T1 on V (selected) | Python, Rust | **PASS / PASS** / G5 ATT |
| T-2 | Stop-rule reason locator | **G5 ATTEMPT.** C2:22 and C2:54: "Every quantity reference is resolved against that run's exact layout". Native StopRule takes `quantity`, `body` and `kind` from the same layout row. | Yes: T2 on L (selected) | Python, Rust | **PASS / PASS** / G5 ATT |
| T-3 | Candidate record shape | **G5 ATTEMPT.** A candidate-role record has `verification` null, no verification build and `verification_lme` 0 (adaptive.rs:4076–4079; set only by `verify_precision` on the verification record, :4333). C1:105. | Yes: T3 on the ordinary base (selected) | Python | **PASS** / G5 ATT / G5 ATT |
| T-4a | `diagnostic_refs` list: resolve, unique, names the case | **G5 ATTEMPT.** C1:100 ("exact existing diagnostics"), C1:148 ("ordinary refs resolve"), C2:11 (DiagnosticRef unique in the envelope) and C2:166. | Yes: T4a (duplicate). T4b (dangling) gives G5 ATT in all three. | Python (duplicate) | **PASS** / G5 ATT / G5 ATT |
| T-4b | W2 trigger match; published b ≠ 0 | **G5 ATTEMPT** (C2:158: "preserving the initial trigger"; "nonzero b") | Needs a faithful initial-failure + W2 base (deferred, O3) | Python, Rust per I64 (TypeScript right) | not probed |
| T-4c | A selected case needs ordinary `solve_quality` ∉ {checks_passed} | **G5 ATTEMPT**, moderate confidence. C1:101: `not_required` "means ordinary pass"; C2:164. **ROOT to confirm** against the I30 routing contract. | Yes: T4e | Python, Rust | **PASS / PASS** / G5 ATT |
| T-4d | `source_identity_sha256` at G1 (TypeScript) versus a G5 re-check (Python) | **G1 RECEIPT** (C3:300) | Only with a receipt-only rehash, which is not in the format (N5) | None observed: T4c gives G1 in all three; Python's G5 re-check is redundant | G1 / G1 / G1 |
| T-4e | Ordinary checks before the C3 work list | **Ordinary first** (C3:304: references before typed checks before work; ROOT's 06a record of the intended order). The dual defect is G5 ATTEMPT. | Yes: T4d on F′ | Python | **G5 WORK** / G5 ATT / G5 ATT |
| T-5 | Structural only: G1/G2 guards, −0 at G5a, raise locus | No outcome difference by construction: G2 rejects −0 first, and the gate and code are the same | Nothing to pin | None | — |

## 6. Method, environment and evidence

**Commands** (from `WT/rv78/P`, the archive of `6b607fd01f`):
- **Python:** `OPENPIPESTRESS_CHECKED_JSON_BIN=WT/targets/i52-readers/canonical_json/release/openpipestress_jcs_ijson OPENPIPESTRESS_UNITS_BIN=WT/targets/i52-readers/units/release/openpipestress_units VENV/bin/python -m pytest -q -p no:cacheprovider tests/test_retained_precision_contract.py tests/test_retained_precision_schema.py`
- **Rust:** `DEVELOPER_DIR=/Library/Developer/CommandLineTools CARGO_BUILD_JOBS=4 RUST_TEST_THREADS=2 CARGO_NET_OFFLINE=true CARGO_TARGET_DIR=WT/targets/rv78 cargo test --locked --offline --manifest-path WT/rv78/P/core/reporting/result_export/Cargo.toml --test retained_precision_contract -- --test-threads=2 --nocapture`
- **TypeScript:** `npm test -- src/features/results/retainedPrecision.test.ts --maxWorkers=2`, then `../../node_modules/.bin/tsc --noEmit --pretty false -p tsconfig.json` (both from `apps/desktop`).

**Versions:** Python 3.13.14, pytest 9.1.1, jsonschema 4.26.0, cargo and rustc 1.97.1, node v24.18.0, vitest 4.1.10, tsc 5.9.3.

**Disclosed host deviations:**
1. **`DEVELOPER_DIR` for Rust.** This follows the interim ruling. It is local evidence only, not gate evidence.
2. **Prebuilt wasm assets for TypeScript.** `src/test/setup.ts` loads the wasm engine from `apps/desktop/public/`, which is an untracked build output and absent from a `git archive`. The first vitest run failed with WASM-ENGINE-ASSET-ABSENT, and a symlink was refused by the module loader. RV78 therefore **copied** READER's existing prebuilt `public/wasm-engine` and `public/self-weight-engine` into the review copy. Nothing was built or installed. The wasm-engine sha256s:
   - `.js` `5682432840e2…`;
   - `_bg.wasm` `7843297271c6…`;
   - `.d.ts` `af2b07abf12d…`;
   - `_bg.wasm.d.ts` `37a68fc929d2…`.
   
   These copies, `node_modules` (linked to READER's link target) and RV78's two probe test files existed only in `WT/rv78/`, which is deleted.

**Evidence in this folder:**
- **PARITY_TABLE.json:** 211 per-entry outcomes from each reader.
- **PROBES.json:** 31 probe entries in seven batches, with edits, RV78's expectations and the three readers' outcomes.
- **INDEPENDENT_CHECKS.json:** hashes, harness comparison, the invariant checks, floors and schema.
- **SHA256SUMS.**

**Scripts and bulk logs** stay in `WT/scratch/rv78_reader_review/` (sha256):

| File | sha256 |
|---|---|
| `rv78_prepare.py` | `8ab0a2f8877d5a06…` |
| `rv78_tabulate.py` | `bad02f862105efa3…` |
| `rv78_case_checks.py` | `54ef3264e90eeb3d…` |
| `rv78_floor_check.py` | `e2b5058101fdbdfc…` |
| `rv78_schema_checks.py` | `654fa9b45994e484…` |
| `rv78_python_outcomes.py` | `4ca4aa4f55e70b0e…` |
| `rv78_python_probe.py` | `83e59c69fd2c112b…` |
| `rv78_rust_outcomes.rs` | `4e6574f6027456ab…` |
| `rv78_ts_outcomes.test.ts` | `6a858811b31948cf…` |
| `rv78_probe_corpus*.py`, `rv78_k4_probe.py` | see the scratch listing |

**Limits:**
- RV78 did not independently re-derive the G5c class or bound bits. It checked only that all three readers produce identical classifications equal to the expected ones on all 33 positive entries; arithmetic is RV79–RV81's scope.
- The native reachability of the synthetic numeric triggers is not established (N6).
- The I62 Python outcome records for 06d are in WT scratch, not under `R/I62`; RV78 used the snapshot's summary only.

## 7. What ROOT must rule on

1. **The S2 rule extension** (R3′, R2′, R4) and its codes. Before adding R4, the rebase of `prefix_attached_old_input_unbound` (N2).
2. **The T-4c reading:** whether a selected case can have ordinary `checks_passed`. Check it against the I30 routing contract.
3. **The R-3 inventory for unsourced, complete old lists:** which member inventory G3 compares against.
4. **R-7 and N7:** the in-gate order of native group and build checks against the per-run replay, and the per-check ATTEMPT/WORK mapping, for the C3 clarification.
5. **S3:** remove the two non-native Refusal variants, or cite their source.
6. **N3:** the single representation of "no source" on an unavailable case.
7. **The corpus additions for B1, S1 and S4.** These come before the readers align and before any acceptance.
