# RV78 confirmation: shared corpus, harness formats and joint parity on snapshot 07a

RV78 is a TASK (Type 2), resumed by ROOT (HELP_HUMAN, Agent 0) under `R/BRIEFS/RV78_RV81_CONFIRMATION.md`. ROOT is the return path. RV78 had no descendants and wrote none of the repairs.

- **Candidate:** READER `b36739112a48da40cd22a7a3bdf7691bf2ad4404`, reviewed from RV78's own `git archive` in `WT/rv78/` (deleted afterwards). NUM was at `c57496275b399b9415927ae06b50a850094a4924`.
- **Shared files:**

  | File | sha256 | Change since 06d |
  |---|---|---|
  | corpus | `a6fa398731…` | 15 cases, 236 mutations, 19 must-pass |
  | schema | `07951edacf…` | D9 changes |
  | definition | `3e0779a45a…` | unchanged |
  | semantic table | `c74742ce6a…` | unchanged |

- **Run window:** 2026-10-03, 18:26:12 to about 18:42 local (MDT). That is inside the 90-minute box, and nothing in the brief was left undone. The memory guard (PID 5387) was running.
- **Limits held:** no Git writes or index operations, no install, no new tooling, and no native, solver or DEC-025 job.

## Verdict: PASS

**Counts:** 0 BLOCKING, 2 SHOULD-FIX, 5 NOTE.

- **All 14 original findings are resolved.** Eleven are fixed outright. N7 and N8 are fixed in the readers and schema, with only the C3 clarification text pending under D9d. N6 needed no action. No original finding is open.
- **The joint parity run on 07a is complete:** 270 of 270 entries match their expectations in all three readers, with zero disagreements between readers.
- **All 31 reader_review_01 probes** now give the outcome their decision sets, in all three readers.
- **No check was removed or weakened** in the shared files or harnesses.

The two new SHOULD-FIX findings are a coverage gap and a single-defect first-gate divergence. Under ROOT's rule, both are repaired before acceptance.

## Original findings (reader_review_01) and their disposition

| ID | Disposition | Decision | RV78 evidence on `b36739112a` |
|---|---|---|---|
| B1 | **Fixed** | D4d, D5a, D5b, D5d, D6a, D6b | All six probes are rejected as decided, in all three readers (PROBES_CONFIRM batch C): T1, T2, T3 and T4a/T4e at G5 ATTEMPT, R6a at G5 PRODUCT_ATTEMPT. The shared pins are byte-identical to RV78's probe edits (NEW_ENTRIES_REVIEW). |
| S1 | **Fixed** | D1, D4a, D4b, D3/D17 | All seven divergences converge: R1a, R1b, R2b and R3b at G3; R4 and R5 at G5 PRODUCT_ATTEMPT; T4d at G5 ATTEMPT. The pins are byte-identical to the probes. D17 has its own extra pin. |
| S2 | **Fixed** | D8 (R1′–R4 and kernel scope) | `B_section_accounting_exact_status`, Q1, Q2 and Q4 give G5 WORK in all three readers. The four shared pins are byte-identical. The rebase came first (N2). |
| S3 | **Fixed** | D9a | The schema Refusal union is exactly the five native variants. RV78's two probes (refused terminal `count_range`; group `work_accounting`) and the two corpus pins give G1 everywhere. The 06d schema validates these shapes and the 07a schema rejects them (jsonschema), so the pins discriminate on the schema bytes. |
| S4 | **Fixed**, except that N11 is deferred (D12) and the N13 pin does not isolate its rule (new N2) | D11 | New negative pins: P8 on F′ and on P′, C6, O1, N1, N7, W2, W3 and N13. |
| N1 | **Fixed** | D11 | Four equal-E strict-bracket pins and an equal-E must-pass control, byte-identical to the probes. All three readers enforce the rule. |
| N2 | **Fixed** | Checkpoint A rebase | `prefix_attached_old_input_unbound` now sits on F′ (an emittable context) with the single J edit, and stays at G8 PREPARATION in all three readers. |
| N3 | **Fixed** | D9b | `source_ref` is required on an unavailable case, as `null\|U`, and P′ case 1 now carries `null`. An absent `source_ref` fails G1, and so does `source_decline` alongside a `source_ref` (probe and pins). |
| N4 | **Fixed** | D9c | The schema `$comment` states that the integer and bit constraints are G2. |
| N5 | **Fixed** | D11 | All three harnesses reject any `rehash` other than `"all"`: Python asserts, Rust asserts, TypeScript throws. Array removal now splices in Rust and TypeScript, and Python deletes. Absent receipts or bodies skip the rehash, uniformly. |
| N6 | No action (accepted boundary) | — | Unchanged; the labels are kept. |
| N7 | **Fixed in the readers**; contract text pending | D3, D9d | The class-1 convention is pinned (`native_attempt_defect_after_native_work_defect`). The per-check table is in CHECKPOINT_A. |
| N8 | **Fixed in the schema comment**; C3 clarification pending | D9c/D9d | The `$comment` records the emitted-domain rule. |
| N9 | **Fixed** | D15 | I63's OUTCOMES_07A lists all 236 mutations. Both author files that cover 07a (I63's and I62's) agree with RV78's run wherever they overlap: 0 differences. |

## New findings

| ID | Severity | Where | Evidence | Remedy |
|---|---|---|---|---|
| S1 | SHOULD-FIX | P/core/analysis_runs/retained_precision.py:1592 compares lengths. Rust compares ids at result_export/src/retained_precision.rs:695–701, and TypeScript at retainedPrecision.ts:237. | **D1 sourced coverage, the question ROOT asked RV78.** Probe `N_D1_source_member_map_kernel_id_noncanonical` sets one source map `kernel_member` to 5. A second probe also sets `model_index`. Both are single defects and correct rejections, because C2:98 requires `kernel_member = model_index`. Results: Python **G5a SCALE**, Rust and TypeScript **G3 COVERAGE**. ROOT's note and I64's RETURN_P2 item 4 assume Python's first failure is G8, but Python's G5a canonical-layout rebuild fails first. D1's text reads "old ids equal the full member inventory… For a sourced attempt, that is the source's member map, at G3". | Add a shared pin expecting G3, using the RV78 probe edit. Python compares ids. |
| S2 | SHOULD-FIX | Deferred list, SHARED_SNAPSHOT_07 `deferred[0]` (T4c) | **The claim that the G1 source-identity check is "pinned by existing entries" is incorrect.** No entry in 07a edits any hash field, and `rehash:"all"` recomputes every hash. So none of the C1/C3 G1 hash-integrity rows (receipt, publication, preparation, source identity) has a shared pin. RV78's probes H1, H3, H4 and H5 (stale receipt, preparation, source identity, publication) give G1 RECEIPT in all three readers, so the readers comply. Only the corpus cannot show it. | ROOT admits an explicit post-rehash or post-final edit list. This is orthogonal to D11's `rehash:"all"` rule, which concerns which hashes are recomputed, not edits after them. Add four G1 pins. Alternatively, record the G1 hash rows as reader-local only and correct the deferred text. |
| N1 | NOTE (challenges D10) | D10 "Python only"; ROOT's 07a note calls it unreachable | Probe F1: `work.charged = 17.0`. Python gives G2; Rust and TypeScript accept. The rehashed receipt is hash-valid, because ES6 canonical JSON prints 17. The difference is therefore **reachable on any non-canonical JSON input**, not unreachable. C2:13 says "No JSON float carries a counter". Rust (serde_json) can see the token; TypeScript's `JSON.parse` cannot. | ROOT decides between two options. (a) Define reader input as parsed JSON values; Python's rule then breaks parity and is dropped. (b) Keep the rule, have Rust adopt it, and record TypeScript's limit as a documented, pinned difference. |
| N2 | NOTE | `rejected_attempt_with_verified_record` (on the ordinary base) | It does not isolate N13's "rejected candidate, verified record" rule. With one rejected attempt, the receipt also breaks "accepted last" and the selected terminal, all at G5 ATTEMPT. A plain verification record of a rejected candidate occurs only at the Ceiling (C1:30), so an isolating pin needs the deferred Ceiling base. | Add it to the deferred list as "N13 isolation needs the Ceiling base". |
| N3 | NOTE | `run_ref_null_with_case_run` | It does not isolate D4e. Setting `native=not_entered` before completed proof stages also breaks the P2 stage sequence, which is the same class and code. Inherent to the shape. | None needed; record it. |
| N4 | NOTE | R/I64/review_repair_07/OUTCOMES_P2.json | I64's latest outcome file predates D18: it shows `g5b_zero_section_area` at G5b SCALE. TypeScript's 07a evidence is the vitest run only. RV78's run gives SECTION. | I64 regenerates the outcome file on 07a, or notes it. |
| N5 | NOTE | D16, "at least one dangling reference per G5 class" | Pins exist for class 1 (`dangling_candidate_record`, `dangling_build_ref`) and class 2 (`dangling_ordinary_attempt_ref`, `ordinary_diagnostic_ref_dangling`). Classes 3 and 4 have none, and appear to hold no references of their own. | State this in the C3 clarification, so the parity claim is not read as covering classes 3–4. |

## 1. Repair diffs: shared files and harness formats (`6b607fd01f` → `b36739112a`)

**The corpus.** RV78 compared 07a against 06d independently:
- **version, provenance, arithmetic:** unchanged.
- **Cases:** 15, in the same order. Only P′ changed: case 1 gains `source_ref: null`, with its receipt hash and qualification note.
- **Mutations:** all 178 are carried in order, with no removals. Exactly one changed: `prefix_attached_old_input_unbound`, rebased onto F′. 58 are new.
- **Must-pass entries:** all 18 are carried unchanged, and 1 is new.
- This matches SHARED_SNAPSHOT_07 + 07A exactly. Every entry uses `rehash:"all"`.

**The schema.** Only `Case` and `Refusal` changed, plus the top-level `$comment`:
- `Refusal` keeps exactly the five native variants (D9a);
- the unavailable `Case` splits into a `source_ref: null` branch (with optional `source_decline`) and a `source_ref: U` branch (without `source_decline`), with `source_ref` required in both (D9b);
- every other definition, and the not_required and selected branches, is byte-identical in content;
- every object stays closed.

**The harnesses** (Python, Rust, TypeScript) now share these semantics:
- they reject any rehash value other than `"all"`;
- array removal splices;
- an absent `retained_precision`, or a non-object body, skips the rehash;
- source identity is recomputed for selected cases. Python keys on status and Rust on the presence of the key; these are equivalent under the schema.

RV78's independent rehash equals Python's `apply_entry` on all 255 entries.

**No check was removed or weakened** in the shared files or harnesses. D18's stop condition is not triggered. RV78 also confirmed that 07a's `g5b_zero_section_area` uses SECTION, and that `g5b_zero_section_length` isolates positivity: its source and selection terms are equal.

## 2. The joint parity run on 07a

| Reader | Command result |
|---|---|
| Python | pytest: 328 passed |
| Rust | default toolchain, no `DEVELOPER_DIR`: 35 passed |
| TypeScript | vitest 379/379; tsc exits 0 |

RV78 prepared every entry with its own edit applier, JCS and rehash code, then called each reader's validate entry directly (PARITY_TABLE.json):
- **270 of 270** entries match in all three readers: 15 cases, 236 mutations and 19 must-pass entries. Disagreements: 0. G7 is compared per reader.
- **Hashes:** all 15 base cases' hashes are reproduced, including P′'s.
- **Independent checks** of the 15 cases and 19 must-pass entries found nothing: work partition, chaining, build provenance, I57 coverage, and the 5 p512 floors recomputed with exact rationals.
- **Schema:** jsonschema validates every positive entry. The G1 mutations include the four new D9 shapes.

## 3. RV78's probes on the new head

PROBES_CONFIRM.json holds 52 probes:
- **Batch C (the 31 reader_review_01 probes):** every one gives the outcome its decision sets, in all three readers. These are the B1, S1 and S2 probes, the K4 controls, the strict-bracket probes and the controls.
- **New in batch C:** the two D9a probes and the D9b absent-`source_ref` probe give G1 everywhere. The two D1 probes diverge (S1).
- **Batch H:** the G1 hash-integrity probes (S2).
- **Batch F:** the integral float (N1).
- **Batch X:** eleven D2 G0-scope and related probes. Component name and version, the projection, work and facade policies, a string `receipt_version`, an absent `case_limit`, an unknown `definition_id`, a null body and a non-object receipt all give G0 in all three readers. A `source_ref` null on an unavailable case that has a Run gives G5 ATTEMPT in all three.

## 4. The new entries: contract fidelity and emittability

RV78 reviewed all 58 new mutations, the new must-pass entry and the rebased mutation, 60 entries in all (NEW_ENTRIES_REVIEW.json). The results:
- **Expected gate and code:** RV78's reading agrees on all 60.
- **Probe-derived entries:** 26 are byte-identical to RV78's probes, so their reading carries over from reader_review_01.
- **Emittability:**
  - every new mutation is a correct rejection of a receipt the producer cannot emit, with the native or contract reason recorded per entry;
  - the new must-pass entry is emittable: equal point moduli interpolate to 200e9 exactly;
  - the rebased entry now sits on the emittable base F′;
  - the only base change is P′'s explicit null, which is a producer obligation under D9b.
- **Isolation:**
  - **isolating**, since the 06d reader passed each of them: D4a `source_preparation_null`, D4c `prepared_failure_cause_without_own_attempt`, D4d `preparation_error_with_selected_run`, D5c, D7, and the probe-derived B1 and S2 entries. This rests on RV78's own 06d probes and I62's recorded 06d-Python observations;
  - **not isolating, but the same code:** N2 and N3 above;
  - **discriminating only through the schema bytes:** the D9 G1 pins, as intended.

## 5. The deferred list (SHARED_SNAPSHOT_07 `deferred`)

The stated reasons hold for N11, D4d nonselected, D6c/D6d, the D1 G8 branch, P7 unsourced, the `refused_member_conversion_kind_bits` context and the RV79 survivors: each lacks a faithful or emittable base. RV78 checked them against the bases.

**Two corrections:**
1. **T4c.** Its reason is right, because the format admits no post-rehash edits. But its claim that existing entries pin the G1 integrity check is not (S2).
2. **A missing item.** The list should include "N13 isolation needs the Ceiling base" (N2).

The earlier producer-solved deferrals still stand: the Ceiling, L = 0, source-construction failure, old-Err/new-Ready, positive subnormal rows, and budget overshoot or exhaustion.

## 6. TypeScript's D1 sourced comparison: does it need a pin?

**Yes; see S1.** The difference appears only on a source map whose `kernel_member` values are not `0..n−1`. Such a receipt is not emittable (C2:98), but it is a single defect, and the readers disagree on its first gate:
- Python: G5a;
- Rust and TypeScript: G3.

D1's wording, "the source's member map", supports comparing ids, at G3. RV78's probe edit is a ready pin.

## 7. Method, environment and evidence

**Commands** (from `WT/rv78/P`):
- **Python:** `OPENPIPESTRESS_CHECKED_JSON_BIN=WT/targets/i52-readers/canonical_json/release/openpipestress_jcs_ijson OPENPIPESTRESS_UNITS_BIN=WT/targets/i52-readers/units/release/openpipestress_units VENV/bin/python -m pytest -q -p no:cacheprovider tests/test_retained_precision_contract.py tests/test_retained_precision_schema.py`
- **Rust:** `CARGO_BUILD_JOBS=4 RUST_TEST_THREADS=2 CARGO_NET_OFFLINE=true CARGO_TARGET_DIR=WT/targets/rv78 cargo test --locked --offline --manifest-path WT/rv78/P/core/reporting/result_export/Cargo.toml --test retained_precision_contract -- --test-threads=2 --nocapture`, on the default toolchain with `DEVELOPER_DIR` unset.
- **TypeScript:** `npm test -- src/features/results/retainedPrecision.test.ts --maxWorkers=2`, then `../../node_modules/.bin/tsc --noEmit --pretty false -p tsconfig.json`.

**Disclosed:**
- **`node_modules`:** `WT/rv78/P/node_modules` was linked to READER's link target.
- **WASM assets:** READER's prebuilt `apps/desktop/public/wasm-engine` and `public/self-weight-engine` were **copied**, not built, into the archive copy (D15). sha256:
  - wasm-engine `.js` `5682432840e2…`;
  - wasm-engine `_bg.wasm` `7843297271c6…`;
  - self-weight-engine `.js` `ea2b3fd611a6…`;
  - self-weight-engine `_bg.wasm` `3bc83f88bfef…`.
- **Probe test files:** RV78's two probe test files existed only in `WT/rv78/`, which is deleted.

**Evidence in this folder:**
- **PARITY_TABLE.json:** 270 rows.
- **PROBES_CONFIRM.json:** 52 probes with their three-reader outcomes.
- **NEW_ENTRIES_REVIEW.json:** 60 entries.
- **INDEPENDENT_CHECKS.json.**
- **SHA256SUMS.**

**Scripts and logs** are in `WT/scratch/rv78_reader_confirm/` (sha256 prefixes):

| File | sha256 |
|---|---|
| `rv78_prepare.py` | `dd43014fc32bf6df` |
| `rv78_tabulate.py` | `bad02f862105efa3` |
| `rv78_case_checks.py` | `54ef3264e90eeb3d` |
| `rv78_floor_check.py` | `e2b5058101fdbdfc` |
| `rv78_schema_checks.py` | `0045d1f018717398` |
| `rv78_probe_corpus_confirm.py` | `882e79ba00976b4e` |
| `rv78_new_entries_review.py` | `25c8d7f3d812d54f` |
| `rv78_python_outcomes.py` | `4ca4aa4f55e70b0e` |
| `rv78_python_probe.py` | `83e59c69fd2c112b` |
| `rv78_rust_outcomes.rs` | `4e6574f6027456ab` |
| `rv78_ts_outcomes.test.ts` | `6a858811b31948cf` |

**Limits:**
- Isolation against the 06d readers rests on RV78's own 06d probe runs and I62's recorded 06d-Python observations. RV78 did not rebuild the 06d Rust and TypeScript readers.
- G5c class and bound bits were not independently rederived; that is RV79–RV81's scope.

## 8. What ROOT must rule on

1. **S1:** confirm the D1 reading (compare ids at G3) and the shared pin. Python then aligns.
2. **S2:** admit a post-rehash or post-final edit list in the shared format and add the four G1 hash pins. Otherwise, record the G1 hash rows as reader-local only and correct the T4c deferred text.
3. **N1:** reachability of the integral-float difference, and option (a) or (b).
4. **N2:** add "N13 isolation needs the Ceiling base" to the deferred list.
