# I83: B6, the reader items (corpus and harnesses; before B1's snapshot)

I83 is a TASK (Type 2) dispatched by ROOT (HELP_HUMAN, Agent 0) for T3, unit B6, under `R/BRIEFS/B6_READER_ITEMS.md` (sha256 `a36341604b70daa958502964d25ed6ed767be6c08ddc2555b38ea721be793777`, verified). ROOT is the return path. I83 did not delegate. I read the repository's `AGENTS.md` and `agents/AGENT_TASK.md` at NUM's root before acting.

**Placeholders:** `WT`, `NUM`, `P`, `T`, `R`, `RR`, `VENV` as in the dispatch; `DT` = `P/apps/desktop/src`; `RE` = `P/core/reporting/result_export`; `NMS` = the shared `node_modules` that `WT/t6-outputs/P/node_modules` links to; `BASE` = main `bfb26596bf`; `HEAD` = the branch head below; `S` = `WT/scratch/i83_b6`.

## 0. Summary

| | |
|---|---|
| **Branch** | `codex/piping-t3-b6-20261007` in `WT/b6`, from BASE. Not pushed. |
| **Head** | **`a7de2a918ff3332133ce1f9d06e205fb87aae3d9`** (`a7de2a918f`) |
| **Commits** | `5e38293530` (items 1, 2 and 4); `a79dbd2e4a` (item 3); `a7de2a918f` (Rust's two new slices also pin their entry ids, after mutant C4; §6.2) |
| **Snapshot** | **07m**: sha256 `c21112fdbfad37dd4832c8dd64d066e1809dd70dce6c89cb920d1d6dd3d46807`, 5,521,277 B; 17 cases, **294** mutations, 28 must-pass entries (07l `5ac13296…`: 17, 286, 28) |
| **Case file** | `fixtures/results/retained_precision_carrier_cases.json`: `cf82deab…` → `98a7213a…` (format v4 unchanged; 6 → 5 declared differences) |
| **Item 1** | Done. Mutation 277 is a one-entry slice (07k) in Rust, Python and TS, each with a literal tally |
| **Item 2** | Done, **and widened** (§2.3; ROOT to confirm): TS's G7 header refusal now carries the base readers' codes for the whole N-3 class and four sibling classes. The N-3 scope sentence is removed; pins updated in three languages |
| **Item 3** | Done. Python's transport validator; F-U6b-2's declared difference removed; RV92's tampered set refused identically in three languages |
| **Item 4** | Done. Python reads `expected_by_reader.python`; R34 is now killed (it survives on BASE) |
| **Item 5** | **Checkpoint, not implemented** (§5). What remains open of RV78-N1 needs a fixture, the schemas, PP and a D1 crate `src`, and it overlaps B1's re-pin |
| **Suites, BASE → HEAD** | Python 1,896 → **1,922** passed (30 skipped both; +26 added, 0 removed, 0 changed). Rust `result_export` 177 → **180** (+3). vitest 3,612 → **3,620** (+13 added; 5 removed, all renamed or the dropped F-U6b-2 forms; 0 changed outcomes). tsc clean on both (§6.1) |
| **Mutants** | 26, plus N0 (the unmutated head): **24 killed**, 2 survive as equivalent on the corpus (T9, T10; §6.2). A per-language corpus field (R34) is killed by the language that reads it, by design. C4 first survived in Rust, which led to commit C; on rerun it is killed in all three languages |

## 1. Item 1: mutation 277 as a one-entry slice

`g7_not_required_quality_enum_invalid` (mutation 277, RV94 N-3's probe, from 07k) was in no slice tally (I70's item 2, RR:12697). Each harness now runs it as its own slice, observes this reader's first failure, compares it with the reader's own expectation, and tallies against a literal:

| Harness | Test | Tally |
|---|---|---|
| Rust `RE/tests/retained_precision_contract.rs` | `snapshot_07k_mutation_outcomes` (`slice_outcomes("I83_OUTCOME_07K", 277..278, …)`) | `G7 SOURCE_NUMERICAL_CASE_INVALID`: 1 |
| Python `P/tests/test_retained_precision_contract.py` | `test_snapshot_07k_g7_probe_slice`, through a new `_slice_outcomes` helper (Python had no slice tally; the helper mirrors Rust's) | the same |
| TS `DT/features/results/retainedPrecision.test.ts` | `07k and 07m slices (B6) … > 07k: mutation 277 …` | the same |

The 07m entries (§2.2) get the same treatment (`snapshot_07m_mutation_outcomes`, `test_snapshot_07m_g7_header_slice`, `07m: …`). Each slice also checks its entries' ids. Python and TS did so from the start; Rust's slice helper tallies codes only, so commit C adds the id assertions to Rust's two new slices after mutant C4 survived there.

Mutants C4–C6 (§6.2) show that only these slices and the count pins see a self-consistent corpus change.

## 2. Item 2: N-3's G7 code alignment (TS aligns to the Python and Rust codes)

### 2.1 The change

`DT/features/results/retainedPrecision.ts`:
- G7 (`:1356`) was `need(sourceContract(base) === 'preview_physics', gate, 'SOURCE_PRODUCER_CONTRACT_UNSUPPORTED')`. It now throws `RetainedPrecisionError(gate, baseHeaderCode(base))` only when the dispatch refuses, so an admitted statement is untouched.
- `baseHeaderCode` (`:1312–1338`, new) returns the base readers' header code in Python's `_source_contract` order: `carrier_evidence` → `SOURCE_PRODUCER_CONTRACT_UNSUPPORTED`; schema version → `SOURCE_SCHEMA_VERSION_UNSUPPORTED`; producer → `SOURCE_PRODUCER_CONTRACT_UNSUPPORTED`; `contract_evidence` not an object → `SOURCE_PREVIEW_PHYSICS_EVIDENCE_REQUIRED`; `source_block_recovery` → `SOURCE_BLOCKS_LEGACY_DOWNGRADE_FORBIDDEN`; the quality level → `SOURCE_NUMERICAL_QUALITY_INVALID`; **any case outside the base case rule → `SOURCE_NUMERICAL_CASE_INVALID`**; the formulation basis → `SOURCE_FORMULATION_BASIS_UNSUPPORTED`; anything else keeps TS's `SOURCE_PRODUCER_CONTRACT_UNSUPPORTED`. The case rule is Python's and Rust's exactly (closed keys, `basis_ref` closed non-empty strings, the four enums, non-empty string `evidence_refs`).
- The Rust base validators, PP and every D1 crate `src` are unchanged (decision 11's reason).

### 2.2 Corpus 07m, and the one in-place value

- **Appended at 286..293** (no existing slice moves): four entries for the N-3 class at its full width and four for the sibling classes (§2.3), each a single hash-consistent edit expected at G7 with the base code, equal in all three readers:

  | # | id | base | edit | expected |
  |---|---|---|---|---|
  | 286 | `g7_selected_quality_enum_invalid` | `ordinary_prepared_synthetic` | case 0 `structural_status` = `mechanism_detected` | `SOURCE_NUMERICAL_CASE_INVALID` |
  | 287 | `g7_unavailable_quality_enum_invalid` | `two_case_preparation_failure_synthetic` | case 1 `model_matrix_fidelity` = `reduced` | `SOURCE_NUMERICAL_CASE_INVALID` |
  | 288 | `g7_quality_case_evidence_ref_empty` | `ordinary_prepared_synthetic` | case 0 `evidence_refs` = `[""]` | `SOURCE_NUMERICAL_CASE_INVALID` |
  | 289 | `g7_quality_case_extra_member` | `ordinary_prepared_synthetic` | case 0 `extra` = 1 | `SOURCE_NUMERICAL_CASE_INVALID` |
  | 290 | `g7_quality_status_invalid` | `ordinary_prepared_synthetic` | `numerical_quality.status` = `estimated` | `SOURCE_NUMERICAL_QUALITY_INVALID` |
  | 291 | `g7_formulation_limitations_empty` | `ordinary_prepared_synthetic` | `formulation_basis.limitations` = `[]` | `SOURCE_FORMULATION_BASIS_UNSUPPORTED` |
  | 292 | `g7_contract_evidence_null` | `ordinary_prepared_synthetic` | `contract_evidence` = null | `SOURCE_PREVIEW_PHYSICS_EVIDENCE_REQUIRED` |
  | 293 | `g7_source_block_recovery_present` | `ordinary_prepared_synthetic` | `source_block_recovery` = null | `SOURCE_BLOCKS_LEGACY_DOWNGRADE_FORBIDDEN` |

- **One value changed in place:** mutation 277's `expected_by_reader.typescript`, `SOURCE_PRODUCER_CONTRACT_UNSUPPORTED` → `SOURCE_NUMERICAL_CASE_INVALID`. Item 2 cannot be done without it (TS reads its own field), and nothing moves. The field stays, now equal in all three languages, so the per-reader entry list (`g7_maximum_off_enclosure`, `g7_not_required_quality_enum_invalid`) and its pins are unchanged. Removing the field instead is the alternative; I kept the smaller edit.
- `_run_records/scripts/build_07m.py` builds 07m from 07l with the file's own serialization (`json.dumps(indent=2)` plus a newline reproduces 07l byte for byte), so the diff is the 277 value plus the appended block.

### 2.3 The widening (ROOT to confirm)

**What I found at BASE** (`_run_records/probes/g7_header_probe.json`; 32 hash-consistent probes through each reader):
- The N-3 class is wider than the clause declared. Python and Rust give `SOURCE_NUMERICAL_CASE_INVALID` and TS `SOURCE_PRODUCER_CONTRACT_UNSUPPORTED` for an invalid quality case of **any** status (selected, unavailable, not_required), and for every part of the case rule (the three enums, an empty evidence ref, an extra member). The clause named not_required and three enums only.
- **Four sibling classes differed the same way and were declared nowhere:** the quality level (`SOURCE_NUMERICAL_QUALITY_INVALID`), the formulation basis (`SOURCE_FORMULATION_BASIS_UNSUPPORTED`), a non-object `contract_evidence` (`SOURCE_PREVIEW_PHYSICS_EVIDENCE_REQUIRED`) and a `source_block_recovery` member (`SOURCE_BLOCKS_LEGACY_DOWNGRADE_FORBIDDEN`). Python and Rust agree on each; TS gave `SOURCE_PRODUCER_CONTRACT_UNSUPPORTED`. The case file calls any undeclared difference a defect.
- One root cause: TS's G7 used a single header code.

**What I did:** aligned TS for all five classes at that one site, in the direction decision 11 ruled for N-3, following ROOT's align-first pattern (RR "RV94 on U7…" S-1, "align, then declare the remainder"; RR:10277, RV92 N-1's remedy). All three readers agree on 30 of the 32 probes at HEAD, against 4 at BASE. Python's outcome is identical on all 32, BASE and HEAD. The two remaining differences are pre-existing Rust base-validator codes, not header classes (§7, item 6).

**If ROOT prefers the literal item:** revert `baseHeaderCode`'s four non-case branches to TS's code, drop entries 290–293 (the end of the corpus, so nothing moves) and the matching slice-tally lines, and declare the four classes in the scope with per-reader pins.

### 2.4 The declaration and the pins

- The case file's N-3 sentence (I67's, RR:11246) is **removed**: no language-specific G7 code remains for this class.
- Pins, all three languages, now assert its absence: Python `test_declared_differences_python`, Rust `u6_declared_differences_rust`, TS `carry the N-4 scope…` (`retainedPrecisionIntegration.test.tsx`).
- TS's `RV94 N-3 …` test (`retainedPrecision.test.ts:961`) keeps its three not_required inputs and now expects `SOURCE_NUMERICAL_CASE_INVALID`; it also checks that all three of 277's per-reader expectations are equal.

## 3. Item 3: F-U6b-2, Python's transport validator for the successor

### 3.1 The change

- `P/core/analysis_runs/retained_precision.py`:
  - `validate_retained_precision_transport(source)` (`:1597`, new) is the twin of Rust's `validate_transport_metadata` and TS's `validateRetainedPrecisionTransport`. It is `_validate_draft(source, None, raw=False)`.
  - `_validate_draft` gains a keyword-only `raw=True` (`:1619`). With `raw=False`, G1 skips the raw rows' shape and the publication digest (`:1645`, `:1648`; Rust's `g1(source, false)`). After G2, transport runs `_transport_g7` (`:1604`, new: the reader's projection, then the base `_source_contract(projected, check_receipt=False)`, whose failure keeps the base's leading code, with its text as detail) and returns not bound, not eligible, `needs_recompute`, the stated publication digest and no classes (`:1664–1666`).
  - The raw path is unchanged in behaviour: three edited lines, each guarded by `raw`, which defaults to True.
- `P/core/analysis_runs/compatibility.py`: `_retained_contract(check_receipt=False)` runs the new `_retained_transport` (`:279`), which maps a reader failure to its text (`detail or code`, as `_retained_validation` and Rust's `retained_error` do), instead of refusing every transported successor.

### 3.2 RV92's tampered probes as the shared refusal set

- **Pinned in all three languages, with the same codes:** RV92's ten tampered transported successors (five forms, both modes; `R/REVIEW_RV92/u6f_01`):

  | Form | Code |
  |---|---|
  | `receipt_sha_zero`, `receipt_sha_zero_no_invocation` | `RETAINED_PRECISION_RECEIPT_MISMATCH` |
  | `receipt_body_edit` (unsealed) | `RETAINED_PRECISION_RECEIPT_MISMATCH` |
  | `receipt_empty` | `SOURCE_PRODUCER_CONTRACT_UNSUPPORTED` |
  | `transport_no_results_receipt_zero` | `RETAINED_PRECISION_RECEIPT_MISMATCH` |

  - **Python** (new, `P/tests/test_retained_precision_carriers.py`): `test_transport_refuses_rv92_tampered_successors` (10), `test_transport_admits_the_untampered_successor_with_or_without_rows_and_never_eligible` (2), `test_transport_refuses_a_receipt_consistent_base_inconsistent_statement_at_g7` (2) and `test_transport_applies_g2_negative_zero_d34` (2; the twin of Rust's transport D34 test, RV80-N1).
  - **Rust** (new, `RE/tests/retained_precision_carriers.rs`): `b6_transport_refuses_rv92_tampered_successors`, with the same five forms and the untampered statement with and without rows.
  - **TS:** I67's existing `the carrier transport route runs the reader's transport checks` block, unchanged.
- **The whole RV92 probe set, three languages** (`_run_records/probes/transport_parity_base_head.json`). RV92's generator (`rv92_probes.py`, sha256 equal to RV92's copy) was patched in one place to read case format v4's per-form fixtures, giving 212 probes. Each language's transport dispatch at HEAD (Python `_source_contract(check_receipt=False)`, Rust `for_source_metadata`, TS `sourceContractTransport`):

  | Outcome | Probes | Status |
  |---|---|---|
  | All three accept | 96 | — |
  | All three refuse, same code | 53 | — |
  | All refuse; TS's header code differs | 28 | Declared: "parity there compares only accept against refuse". Python's code equals Rust's on all 28 |
  | Accept/refuse differ: TS refuses token rows at its header | 31 | Declared, RV92-N2-N5 |
  | Accept/refuse differ: Rust accepts `carrier_evidence` | 2 | Declared inherited |
  | Accept/refuse differ: Rust accepts a key added to `contract_evidence` | 2 | Declared scope sentence ("TS's transport route, like Python's transport check, refuses…") |

  - Python changes on 86 probes, BASE → HEAD, all from `SOURCE_PRODUCER_CONTRACT_UNSUPPORTED` (the old blanket refusal).
  - Rust's and TS's columns are the same at BASE and HEAD. Rust's `src` is unchanged, and TS's transport uses `validateRetainedPrecisionTransport`'s own G7 line, which B6 does not touch.
  - Python's HEAD code equals Rust's on 208 of 212. The 4 others are the two Rust-accepts classes above.

### 3.3 Which declarations change

- **Removed:** `F-U6b-2:python_refuses_transport` (case file: 6 → 5 entries). Python now admits the unedited successor's transport, as Rust and TS do, so it is no longer a difference. The `DECLARED` sets in Python, Rust and TS drop it; TS's test title says "five".
- **Reworded:** the transport scope sentence's code list, from "(TS SOURCE_NUMERICAL_CONTRACT_UNSUPPORTED; Rust the reader's G0 code or its base header code; Python F-U6b-2's code)" to "(TS SOURCE_NUMERICAL_CONTRACT_UNSUPPORTED; Rust and Python the reader's G0 code or their base header code, Python's transport dispatch running the reader's transport validator since B6, F-U6b-2)". It is pinned positively in all three languages, and the old phrase negatively.
- **Unchanged and still true:**
  - **T6S decision 8, "Python keeps refusing successor packages"** (`T/IMPLEMENTATION/T6S/CHANGE_RECORD.md:134`). The packager's `SUPPORTED_METHODS` excludes the successor (`SN-SOURCE-METHOD-UNSUPPORTED` on build), and its validator's metadata view never carries `retained_precision`, so the new transport refuses it at G0 with the same `SOURCE_PRODUCER_CONTRACT_UNSUPPORTED` as before. `test_stress_neutral_packager_still_refuses_the_successor` is unchanged and passes.
  - I67-F1, I67-F2, F5, RV92-N2-N5 and D-U7-4.
- **The Python pin that flips:** `test_dispatch_admits_the_successor_through_the_accepted_reader_only` expected `SOURCE_PRODUCER_CONTRACT_UNSUPPORTED` for the transported milestone. It now expects the successor's contract, and the reader's code for the zeroed-hash statement.

## 4. Item 4: RV94 N-5, Python reads its own expectation

- `P/tests/test_retained_precision_contract.py`:
  - `_expected(mutation)` (`:235`) returns `expected_by_reader.python` when present, else `expected` (the 06b format). `test_shared_draft_first_failure_controls` and the new slices use it.
  - The format test also pins the two per-reader entries' whole expectation blocks literally. Under the 06b format no reader now reads a per-reader entry's shared `expected`, and this pin keeps a change there (R35) killed, as it was before.
- **The mutation the corpus kills** (`_run_records/mutants/`): R34, a wrong `expected_by_reader.python`, on 277 (C1) and on 139 (C2):
  - **BASE harness on 07l:** both survive (`3 passed`, `base_harness_C1.log`, `base_harness_C2.log`);
  - **HEAD on 07m:** both killed (§6.2).

## 5. Item 5: RV78-N1, checkpoint (not implemented)

### 5.1 What "RV78-N1" names

RV78 wrote seven reviews, and each has its own N1:

| Source | N1 | Status today |
|---|---|---|
| `reader_review_01` (RR:8069, D11) | equal-E variants for the strict-bracket pins | Fixed in 07; confirmed by `reader_confirm_02` |
| `reader_confirm_02` (RR:8302, D25) | Python's integral-float rule | Fixed (D25); confirmed by `reader_confirm_03` |
| `reader_confirm_03` (RR:8512) | D19's Ready direction as shared negatives | Fixed in 07d; confirmed by `reader_confirm_04` |
| `reader_confirm_04` (RR:8619, :8637) | **the rehash indexing rule** | Fixed in 07e: Python `_rehash_ref` and `test_rehash_index_rule_07e`, Rust `index()` and `rehash_index_rule_07e`, TS `rehashRef` and `RV78-N1: the harness rehash…`, all present at BASE. Confirmed by `reader_confirm_05` §3 ("Fixed. All three harnesses follow the stated format_rule") |
| `carried_artefacts_01` (RR "The F2a readers accepted…", routing; RR:9174; **RR:9461**, D-U6-7) | **the retained semantic table does not bind** the projection policy `RP-LOGICAL-ATTEMPTS-v1`, the work policy `W1-LME-20B-60B-v1` and its 20B/60B limits, the method token, or the canonicalization profile. The readers enforce all of them as G0 constants (D2), so behaviour is unaffected. Remedy: "optionally add them to the table at the next table revision, which changes its pinned hash in all three readers" | **Open**, D36-tracked, non-gating |

- **RR:9461 is the carried-artefacts N1.** I66's plan (`R/I66/u6_scoping_01/PLAN.md:196`, `:308`) gives it that content, and the "re-pin cascade" that deferred it: "Table fixture; all readers' pinned hash; PP's bound table hash and pinned successor bytes; U5 evidence".
- **The label in PLAN §2.1's B6 row and DESIGN_v2 §7's B6 row, "RV78-N1's rehash-index rule (RR:9461)", conflates two findings.** The rehash rule is `reader_confirm_04`'s N1, and it is closed.

### 5.2 What the open item would touch

The table's sha256 `c74742ce…` is pinned outside B6's fence:
- **the fixture itself:** `fixtures/results/semantic_contract_v0_3_preview_physics_retained_1.json`;
- **PP:** `PP/src/retained_memory.rs:952` (`reviewed_inputs`, so the registered build identity changes), `retained_memory_law_tests.rs`, `retained_wire_tests.rs` and `build_identity.rs:153`;
- **D1 crate `src`:** `RE/src/retained_precision.rs` (`TABLE_HASH`, `include_bytes!`) and `RE/src/semantic_contract.rs`;
- **the schemas:** `schemas/analysis_run.v0.3.schema.json` and `schemas/stress_neutral_export.v0.3.schema.json`;
- **the readers' and carriers' constants:** Python `retained_precision.py` and `compatibility.py`, TS `retainedPrecision.ts` and `numericalResultQuality.ts`, plus their tests.

It does **not** occur in the corpus, the case file, the milestone successors or the derivative goldens (grep: 0), so existing corpus entries are not re-pinned directly. But PP's reviewed-input change re-identifies the registered build, which **overlaps B1's re-pin and its one re-qualification** (DESIGN_v2 §3.3, and §7's B1 rows).

### 5.3 Options for ROOT

- **(a) Fold it into B1.** It rides B1's re-pin and its single re-qualification. B1's fence widens to the table, the two v0.3 schemas, PP's `reviewed_inputs`, RE's `TABLE_HASH` and the readers' constants, with T6S's goldens regenerated only if the successor bytes change. I66 estimated 3–5 h with the cascade.
- **(b) Defer to the next table revision,** as D-U6-7 ruled, still D36-tracked. Cheaply, B3's new `physics-retained-1` table can bind these policies from its first version, since it is a new hash anyway.
- **(c) Close it as accepted.** The four bindings are enforced as G0 constants in all three readers; record that the table binds less than C1:88 suggests.

**My recommendation: (b) with B3's table binding them from the start,** unless ROOT wants the preview table aligned before PR-B1, in which case (a). Nothing in B6 depends on the choice.

## 6. Suites and mutants

### 6.1 The suites, BASE against HEAD

- **Where:** both ran in `WT/b6`. BASE ran on the clean tree before any edit. HEAD ran on the tree whose 11 changed files are byte-identical to `a79dbd2e4a` (checked file by file with `cmp`, then committed as they were).
- **Commit C** changes only RE's contract test, adding two id assertions. On its tree, Rust `result_export` was run again in full: 180 passed, 0 failed, the same 180 tests as at `a79dbd2e4a`. Python and TS read nothing it changes.
- **The records:** `_run_records/suites/suites_summary.txt`, and `suites_compare.json` (test by test: Python's junit, vitest's JSON, cargo's log).

| Suite | BASE | HEAD | Test by test |
|---|---|---|---|
| Python: the 24-file set earlier T3 TASKs ran (I69's `run_py.sh`), plus the retained schema, carrier and contract tests and `test_results_dispatcher_v0_3.py` | 1,896 passed, 30 skipped | **1,922** passed, 30 skipped | +26: 16 transport tests, 8 new corpus entries, 2 slices. 0 removed, 0 changed |
| Rust `result_export`, `cargo test --no-fail-fast`, all targets | 177 | **180** | +3: `b6_transport_refuses_rv92_tampered_successors`, `snapshot_07k_mutation_outcomes`, `snapshot_07m_mutation_outcomes` |
| vitest, the whole desktop suite (141 files) | 3,612 | **3,620** | +13 / −5. The −5 are 3 renamed tests (§7, item 5) and the 2 forms of the dropped F-U6b-2 entry (one per fixture). The +13 are those 3 renamed tests, the 8 entries and the 2 slices. 0 changed outcomes |
| tsc `--noEmit` | rc 0, no output | rc 0, no output | — |

**Commit A on its own** (`_run_records/suites/stageA_summary.txt`) was checked before it was committed:
- the three Python retained files: 490 passed;
- `retainedPrecision.test.ts` and `retainedPrecisionIntegration.test.tsx`: 666 passed;
- Rust `retained_precision_carriers` and `retained_precision_contract`: 16 and 65 passed.

**Weakening:** I read every removed line in the non-fixture diff (`_run_records/diff/b6.diff`). Each is:
- a count that grows by the appended entries;
- a slice bound narrowed from `[278:]` to `[278:286]`, so the 07l pin keeps exactly its 8 ids;
- an expectation value changed for the stated reason (N-3 aligned; F-U6b-2 removed);
- the N-3 positive scope pins, replaced by negative ones;
- a renamed test title;
- the replaced G7 line;
- the replaced transport refusal.

No assertion is dropped.

### 6.2 Mutants

`_run_records/scripts/mutants.py`; `_run_records/mutants/mutants.jsonl`. Each mutant is one edit to a copy of HEAD (`S/mut`), run in its lanes, then restored. The lanes:
- `py`: the Python contract test, `-k "snapshot_07 or (first_failure_controls and g7)"`;
- `pyc`: the Python carrier tests;
- `ts`: `retainedPrecision.test.ts`;
- `rs`: `--test retained_precision_contract`.

`N0` (no mutation) passes in every lane.

| Id | One edit | Python | TS | Rust | Killed by |
|---|---|---|---|---|---|
| N0 | none | passes (all lanes) | passes | passes | — |
| C1 | 277's `expected_by_reader.python` wrong (R34) | **killed** | **killed** | (reads its own) | Py: the per-entry test, the 07k slice, the literal per-reader pin. TS: its N-3 test checks the three fields are equal. On BASE's harness and 07l it **survives** (§4) |
| C2 | 139's `expected_by_reader.python` wrong (R34) | **killed** | survives (reads its own) | (reads its own) | Py: the per-entry test, the literal pin. On BASE it **survives** (§4) |
| C3 | 277's shared `expected` wrong (R35) | **killed** | survives | (reads its own) | Py: the literal per-reader pin. No reader reads it under the 06b format |
| C4 | mutations 277 and 286 swapped, each self-consistent | **killed** | **killed** | **killed** at HEAD (survived before commit C) | Py and TS: the slices' id checks and Python's 07m id pin. Rust: the slices' id checks added in commit C. No per-entry test sees it |
| C5 | entry 290 re-pointed self-consistently to the formulation class | **killed** | **killed** | **killed** | the 07m slices' literal tallies only |
| C6 | the last 07m entry dropped | **killed** | **killed** | **killed** | the count pins, the 07m slices |
| T1 | TS's G7 header code back to the constant | — | **killed** | — | 12 tests: entry 277, entries 286–293, both slices, the N-3 test |
| T2 | the case loop removed | — | **killed** | — | 8 |
| T3 | the case rule's exact-keys part removed | — | **killed** | — | entry 289, the 07m slice |
| T3b | the `structural_status` part removed | — | **killed** | — | entry 286, the 07m slice, the N-3 test |
| T3c | the `model_matrix_fidelity` part removed | — | **killed** | — | entry 287, the 07m slice, the N-3 test |
| T4 | the `evidence_refs` part removed | — | **killed** | — | entry 288, the 07m slice |
| T5 | the quality-level check removed | — | **killed** | — | entry 290, the 07m slice |
| T6 | the formulation check removed | — | **killed** | — | entry 291, the 07m slice |
| T7 | the evidence-required check removed | — | **killed** | — | entry 292, the 07m slice |
| T8 | the `source_block_recovery` check removed | — | **killed** | — | entry 293, the 07m slice |
| T9 | `contract_evidence` and `source_block_recovery` checked in Rust's order | — | survives | — | **Equivalent on single-defect inputs.** The two orders differ only when both defects are present, where Python and Rust themselves disagree (inherited), so no shared pin is possible |
| T10 | the `carrier_evidence` branch removed | — | survives | — | **Equivalent:** the fallback code is the same `SOURCE_PRODUCER_CONTRACT_UNSUPPORTED` |
| X1 | transport dispatch refuses again (old F-U6b-2) | **killed** | — | — | 16, among them the flipped dispatch pin and every transport test |
| X2 | transport skips the receipt digest | **killed** | — | — | the RV92 tampered set (zeroed and unsealed forms), the dispatch pin |
| X3 | transport skips the base transport metadata (G7) | **killed** | — | — | the base-inconsistent test |
| X4 | transport reads eligible | **killed** | — | — | the untampered test |
| X5 | transport requires raw rows | **killed** | — | — | the untampered header-only test |
| X6 | transport checks the publication digest | **killed** | — | — | the untampered header-only and base-inconsistent tests |
| X7 | the carrier keeps the code, not the base text | **killed** | — | — | the base-inconsistent test (text through `_source_contract`) |
| X8 | transport skips G2 | **killed** | — | — | the −0 transport test |

Lanes: Python is `py` for C mutants and `pyc` for X mutants; TS is `ts`; Rust is `rs`. "—" means the lane was not run, because the mutant cannot reach it (a TS reader edit has no Python or Rust lane). The full records, with every failing test id, are in `mutants.jsonl`, with C4's rerun as its last record.

## 7. For ROOT

1. **Item 5:** choose (a), (b) or (c) (§5.3). Also correct the RV78-N1 label in PLAN §2.1 and DESIGN_v2 §7.
2. **Item 2's widening** (§2.3): confirm it, or have it narrowed to the case class with the four sibling classes declared.
3. **"The fixtures":** I edited two files under `fixtures/results/`, the corpus and the carrier case file, because items 1–3 require them (the corpus is B6's to write, and the scope clause lives in the case file). No other fixture changed, and nor did PP, any D1 crate `src`, the schemas or the T6S files.
   - `retainedPrecisionIntegration.test.tsx` was also touched by T6S, but only in its T6-panel block; my edits are in the case-file pins.
4. **One in-place corpus value:** mutation 277's TS expectation (§2.2). No entry moved, and every count change is an appended entry or an added test.
5. **Renamed test ids:**
   - TS `RV94 N-3: …` → `RV94 N-3 (B6, PLAN decision 11): …`;
   - `07l: 17 cases, 286 mutations …` → `… (07m: 294 mutations) …`;
   - `are exactly the six ruled entries…` → `five`.
6. **Pre-existing, not changed:** at BASE and HEAD, a hash-consistent `formulation_basis.limitations` set to another non-empty list is refused at G7 by Python and TS with `SOURCE_PREVIEW_PHYSICS_EVIDENCE_INVALID`, and by Rust with `SOURCE_PREVIEW_PHYSICS_FORMULATION_BASIS` (each base evidence validator's own code). This is the 06b kind, like `g7_maximum_off_enclosure`, but **no per-reader entry pins it**. A per-reader corpus entry would declare it, in B1's snapshot or later. (Rust's `SOURCE_PREVIEW_PHYSICS_FOREIGN_METHOD_EVIDENCE` for `carrier_evidence` is the declared inherited difference.)
7. **The transport gate label:** a base transport-metadata failure is labelled G7 in Python (as in TS) and G2 in Rust (`validate_transport_metadata`). No carrier compares the gate; noted only.
8. **Pre-existing, optional:** Rust's `slice_outcomes` tallies codes only, so a reorder of same-code entries inside the 03–07l slices is invisible to Rust (Python's per-entry ids and TS's 07l id checks cover part of it). Commit C adds id checks to the two new slices only. Moving the id check into the helper would touch every existing slice test, so I left that to ROOT (B1's touch of the file is a natural place).

## 8. Host rules, disclosures and cleanup

### Cargo

Every cargo job went through `WT/tools/t3_cargo.sh` with `--locked --offline`, `CARGO_BUILD_JOBS=4`, `RUST_TEST_THREADS=2` and no RUSTFLAGS:
- the two CLI authorities, release, into `WT/targets/i83-b6/{checked-json,units-authority}`;
- `result_export`'s BASE, stage-A, HEAD and commit-C tests, in `…/rx`;
- the scratch probe test, in `…/rx-dev`;
- the mutant lane, in `…/rx-mut`.

One of my own queued probe jobs was stopped while it waited, before it started, and re-queued with its environment.

### The shared lock

Under `lockf -k WT/guard/cargo_job.lock`:
- the Python suites;
- the full vitest and tsc runs;
- the stage-A checks;
- every mutant lane;
- the item-4 BASE-harness runs.

Light single-file runs ran without the lock:
- the TS probes, about 1 s each;
- the targeted dev checks: 20 carrier tests in 35 s, 21 contract tests in 2 s, and two vitest files in 9 s;
- the Python probe scripts.

I waited for other jobs, including ROOT's DEC-025 `SI1b_b4f22e6ce7`, which started during my mutant runs, and killed none. I ran no DEC-025, no PP evidence sweep, no native or solver job and no install.

### Node and the wasm assets

- The untracked `WT/b6/P/node_modules` → NMS link was created for the runs and removed.
- The eight wasm assets were copied (not built) from `WT/sweep-skewpin/P/apps/desktop/public/` into `WT/b6/P/apps/desktop/public/` and removed. Their sha256 values equal I71's eight (`_run_records/suites/wasm_assets_worktree.sha256`).
- Vitest's cache in `WT/b6/P/apps/desktop/node_modules/.vite` was removed after each run.
- NMS's pre-existing `.vite-temp/` is empty again; only its mtime moved.
- At return, `git status --short --ignored` shows nothing under P's `apps`, `core`, `tests` or `fixtures`.

### Scratch and temp

- Scratch was `S` only, with TMPDIR, pytest's basetemp and cargo's TMPDIR all pointing into it.
- **Disclosed:** the agent host writes its own background-task output files under the system temp directory. That is the harness, not my tools; nothing I ran wrote there.

### Git

- Three commits on `codex/piping-t3-b6-20261007`; no push; reads used `GIT_OPTIONAL_LOCKS=0` and `git archive`.
- **Disclosed:** I once ran `git init` by mistake inside the scratch copy `S/dev` (not a repository). I removed its `.git` immediately, with no object or commit written and no effect on any repository.

### Copies and probes

- **Copies:**
  - `S/base` and `S/mutbase` (`git archive` of BASE, P without `execution/`);
  - `S/dev` (where I wrote the changes first);
  - `S/stagecheck` (proves that stage A then stage B reproduces `S/dev` byte for byte);
  - `S/mut` (the mutants).
- **The probe files** `zz_i83_probe.rs`, `zzI83Probe.test.ts` and `zzI83Transport.test.ts` existed only in `S/dev` and are recorded under `_run_records/scripts/`.

### Records and cleanup

- **Records:** this folder, with placeholder paths only (`collect_records.py` substitutes them and refuses any machine path). The host guard allowed the writes into NUM.
- **Cleanup at return:** `S` (the copies, probes and logs, after the records were copied) and `WT/targets/i83-b6/` are deleted.

## 9. Records in this folder

- **`RETURN.md`**, and **`SHA256SUMS`** over every other file.
- **`_run_records/diff/`:**
  - `b6.diff` (BASE..HEAD, 11 files);
  - `changed_files.tsv` (each file's BASE and HEAD sha256 and line counts);
  - `commits.txt`.
- **`_run_records/suites/`:**
  - `suites_summary.txt` and `suites_compare.json`;
  - `stageA_summary.txt`;
  - the run windows;
  - the wasm assets' sha256 values;
  - the two tsc logs.
- **`_run_records/probes/`:**
  - `g7_header_probe.json`: 32 probes, with Python, TS and Rust at BASE and HEAD;
  - `transport_parity_base_head.json`: 212 RV92 probes, with Python at BASE and HEAD, Rust and TS.
- **`_run_records/mutants/`:** `mutants.jsonl`, and the item-4 BASE-harness logs.
- **`_run_records/scripts/`:**
  - `build_07m.py` and `build_case_file.py` (the two fixture edits, reproducible from BASE's bytes);
  - `stage_patch.py` (stages A and B);
  - `suites.sh`, `env.sh`, `compare_suites.py`;
  - the probe sources;
  - `rv92_probes.py` with `rv92_probes.patch.txt`;
  - `mutants.py` and `item4_base.sh`;
  - `collect_records.py`.

