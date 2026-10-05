# RV92 confirmation of the post-U6f round (I66 `6383e8e70e`, I67 `b10ee5cf08`)

RV92 is a TASK (Type 2) under ROOT (HELP_HUMAN, Agent 0). It confirms its own U6f findings (`R/REVIEW_RV92/u6f_01/REVIEW.md`) against the post-U6f round, with its earlier context. ROOT is the return path; RV92 did not delegate.

**Candidate:** `b10ee5cf08`, the head of `codex/piping-f2a-carriers-20261004` (I66's `6383e8e70e`, then I67's part). It is compared with `76477534f6`, RV92's U6f candidate. The round changes 6 files:
- product code: `compatibility.py` and `numericalResultQuality.ts`;
- tests: the Rust, Python and TS carrier tests;
- the case file.

No `result_export/src`, reader, schema, PP, runner or src-tauri file changed.

## Verdict: NOT CONFIRMED on one part of item 3; items 1, 2, 4 and 5 CONFIRMED; nothing blocking

| Item | Result |
|---|---|
| 1. S-1, Python's receipt-copy check | **CONFIRMED.** Python refuses the bool/int swaps (and null and string swaps), and accepts `0.0`/`0` and `1.0`/`1`, in agreement with TS: 12 of 12 probes, resealed and unsealed. |
| 2. N-1, TS transport | **CONFIRMED.** All 10 tampered transport probes are refused by TS's new transport route with Rust's codes, and an untampered transport passes, both full and header-only. |
| 3. N-2 to N-5: the five entries, the N-4 sentence, and the rebuilt table | **The entries and the sentence: CONFIRMED.** All 5 entries (20 forms × fixtures) and all 20 shared cases hold in all three languages, and the scope sentence is accurate. **"Every difference is one of the five": NOT CONFIRMED,** on a residue in the transport subject only (N-9): 2 probes where TS now refuses what Rust admits, and 28 where both refuse with different codes. All fail closed; both are inherited Rust/TS transport differences that the scope sentence does not name. |
| 4. RV88's U6a N-3, the seam guard | **CONFIRMED.** The guard scans product text across whole files. 8 of 8 mutants are killed, including RV88's G1, and 3 of 3 controls pass. |
| 5. No other change | **CONFIRMED.** The 546-input sweep is identical to `76477534f6` in all three languages (Rust 514/514, Python 545/545, TS 515/515). The suites match, and no existing test outcome changed. |

**Counts:** 0 BLOCKING, 0 SHOULD-FIX, **1 NOTE (N-9, new).** The fix needs **no code**: one more sentence in the case file's `scope` clears it (see N-9).

## Host

As in u6f_01:
- **Copies:** `git archive` of `b10ee5cf08` into `WT/rv92/cand2`. The mutant lane `WT/rv92/mut2` is an APFS clone of it.
- **Runtime:** `node_modules` is linked to `REPO_ROOT`'s; the WASM assets were copied from `WT/f2a-readers` (same hashes); Python is `REPO_ROOT/P/.venv` with RV92's candidate-built CLIs.
- **Cargo:** `--locked --offline`, `CARGO_BUILD_JOBS=4`, `RUST_TEST_THREADS=2`, **one cargo job at a time** (sequenced by `run_chain2.sh`, then the mutant run). The memory guard (PID 5387) was checked before each job.
- **Not run:** nothing native, at scale or DEC-025; no install; no Git writes. The review harness files were added only to RV92's copies, after the suites ran.
- **When:** 2026-10-04, 14:34Z to about 14:53Z.

## Suites on `b10ee5cf08`

| Suite | Result |
|---|---|
| result_export, every target | **168 passed**; all 167 earlier names present; new: `u6_product_text_drops_only_test_gated_items` |
| Python 24-file sweep + schema + carrier tests | **1,843 passed, 30 skipped, 0 failed**; per-test outcomes identical to `76477534f6` |
| Desktop Vitest / `tsc` | **3,494/3,494**; `tsc` clean. Against `76477534f6`: 0 outcomes changed; 10 declared-entry titles renamed (v2 → v3 forms); 38 new (v3 forms, the scope test, 15 transport tests) |

## Item 1: S-1

compatibility.py:703–705 now refuses unless the record carries `retained_precision` and `_same_canonical` holds: checked canonical JSON bytes equal (:254–262).

RV92's exactness probe (`rv92_py_exact_v2.py`, `zzRV92ExactV2.test.ts`) runs both modes. Each copy is changed in the record, the record's hash is resealed, and the copy is put back on the source for the reader:

| Copy change | Python (unsealed, resealed) | TS (unsealed, resealed) | Copy revalidates |
|---|---|---|---|
| `0 → false`, `1 → true` | `…RECEIPT_MISMATCH`, `…RECEIPT_MISMATCH` | same | no (G1) |
| `0 → null`, `0 → "0"` | `…RECEIPT_MISMATCH` ×2 | same | no (G1) |
| `0 → 0.0`, `1 → 1.0` | ok, ok | ok, ok | yes |

That is 12 of 12 probes, Python and TS identical. The parity table's AnalysisRun mutations (dropped, null, zeroed hash, body +1, integral float) are unchanged, and P = T. The receipt still survives end to end: 18/18 carrier × reader × mode combinations byte-equal, revalidated and `needs_recompute`; the Python record validates in TS; the schemas are valid.

## Item 2: N-1

`sourceContractTransport` (numericalResultQuality.ts:124–138) runs the header route, then, for the successor, `validateRetainedPrecisionTransport`.

| Probe (both modes) | Rust `for_source_metadata` | TS transport route |
|---|---|---|
| zeroed `receipt_sha256` (with and without an invocation), unsealed body edit, header-only with a zeroed hash (8) | `RETAINED_PRECISION_RECEIPT_MISMATCH` | same |
| `{}` receipt (2) | `SOURCE_PRODUCER_CONTRACT_UNSUPPORTED` | same |
| untampered: full, header-only (no `results`), no invocation (6) | ok | ok |

**No product caller exists yet** (git grep). This is as I67 states: every TS consumer of a transported successor is a T6 surface that refuses first. The function is the TS carrier compared here.

## Item 3: the declared differences and the rebuilt parity table

**The case file (v3).** It has:
- the 20 shared cases unchanged;
- 5 fixtures, adding `source_blocks_n05_sparse` (`90baefac…`);
- 5 entries carrying `forms`;
- a top-level `scope` sentence.

**RV92's own columns reproduce every expectation** (`declared_check.txt`): all **20 of 20** declared forms × fixtures and all **20 of 20** shared cases hold in all three languages. In detail:
- I67-F1;
- I67-F2 widened (none: binding and summary; refused: foreign mode per mode, edited model, `{}`);
- F-U6b-2;
- F5;
- RV92-N2-N5 (the token-row header forms on legacy 0.1.0 and preview-physics-1; the source-blocks summary row with a `{}` receipt or a token row).

**The scope sentence** states the inherited G7 text, Rust's header ignoring `carrier_evidence`, and the builders' refusal codes, and that G7 parity compares the reader's gate and code. **It is accurate as far as it goes.**

**The rebuilt table:** 204 probes. They are u6f_01's 184 non-declared probes, byte-identical, plus the 20 v3 declared forms (`rv92_probes_v3.py`; `rv92_compare_v3.py` → `full_summary.txt`).
- **Against u6f_01's columns,** the Rust and Python columns are identical on all 184 common probes.
- **The TS column changed only in transport** (the new route): 10 N-1 refusals, as intended, plus 2 new refusals (N-9a).

| Subject | Probes | Agree | Declared | Scope sentence | Other |
|---|---|---|---|---|---|
| Standing | 204 | R = P 204; R = T 194 | 10 I67-F1 | — | 0 |
| Binding | 204 | R = P 204; R = T 173 | 24 I67-F2 (10 none, 14 refused); 7 N-5 | — | 0 |
| Summary | 204 | R = P 204; R = T 194 | 10 I67-F2 (none:summary) | — | 0 |
| Raw dispatch | 204 | R = P 200 | — | 4 G7 text | 3 harness (TS checks source-block receipts at registration and AnalysisRun, not at the header) |
| AnalysisRun validate and mutations, P/T | 204 | all | — | — | 0 |
| AnalysisRun builders, P/T | 204 | — | — | 106 build and 189 v0.2 refusal codes | 2 harness (Python's router builds 0.2; RV92 called TS's 0.3 builder) |
| **Transport** | 204 | 69 | 72 F-U6b-2; 31 N-2 | 2 `carrier_evidence` | **30 (N-9)** |

## N-9 (NOTE, new): two inherited Rust/TS transport differences the scope sentence does not name

- **(a) Accept against refuse: 2 probes, created by N-1's repair.** `contract_evidence_edit_resealed`, both modes: the successor's `contract_evidence` gains a key, hash-consistent.
  - Rust's transport admits it (`ok`). Its header dispatch checks only that preview `contract_evidence` is an object (semantic_contract.rs, the preview branch of `for_source_metadata`).
  - TS's new route refuses it at the reader's G7 (`SOURCE_PREVIEW_PHYSICS_EVIDENCE_INVALID`, transport evidence shape), as Python's transport check would.
  - At `76477534f6`, TS's header route admitted it, like Rust.
  - **Inherited at base:** on the projected preview-physics-1 envelope, Rust's `for_source_metadata` admits the edit, while TS's `validatePreviewPhysicsTransportMetadata` and Python's transport check refuse it ("transport evidence shape"; `base_ts_preview_transport.json`, u6f_01 `extra_*`).
  - This is the same Rust laxity as the named `carrier_evidence` case. I67's own new test (a broken `combination_gates`) is this class, but Rust admits it, so it is not a G7 text difference: it is accept against refuse.
- **(b) Code only: 28 probes, both refuse.** These are a successor-id envelope whose header TS's dispatch refuses: wrong profile, component version or schema version; a null, array, string or absent receipt; the successor id written into the other identities; a missing `contract_evidence`; or `source_block_recovery`.
  - TS rejects with its header-route code, `SOURCE_NUMERICAL_CONTRACT_UNSUPPORTED`.
  - Rust gives the reader's G0 code (24) or its base header code (4).
  - **RV92's u6f_01 table listed these as "agree (TS finding vocabulary)" and omitted them from N-4's list; that was RV92's error.** With a TS transport route that "rejects with the first refusal code", they are code differences on the transport subject.
  - **The reader's own TS transport validator returns Rust's code on 24 of the 28** (`ts_reader_transport.json`). The other 4 give the reader's G7 code, against Rust's base code.
- **Impact:** none on reliance. Transport is never eligible; TS is stricter in (a); both refuse in (b).
- **Remedy (ROOT to rule):**
  - **Records only, preferred:** add to the case file's `scope`: "Rust's header dispatch checks only that preview `contract_evidence` is an object, so the transport route of TS (and Python's transport check) refuses evidence content that Rust admits. A transport that a header check refuses before the reader runs carries each language's own code (TS `SOURCE_NUMERICAL_CONTRACT_UNSUPPORTED`; Rust the reader's G0 code or its base code)." Parity then compares accept/refuse there.
  - **Optionally, I67:** `sourceContractTransport` could run `validateRetainedPrecisionTransport` for any successor-id source before the header route. That aligns 24 of the 28 codes; the other 4 fall under the G7 rule.

## Item 4: RV88's U6a N-3, the seam guard

`u6_doc_hidden_seams_have_no_product_callers` (retained_precision_carriers.rs:955–1010):
- it scans each product `.rs` file's text minus only its `#[cfg(test)]`-gated items (`product_text`, :905);
- comments and literals are skipped while matching;
- an unclosed gated item fails the test;
- it counts the bare seam name (`bare_count`, :927);
- it pins 4 deep product functions in the scanned text.

RV92 ran its own mutants in a clone lane against the two guard tests only (`rv92_guard_mutants.py`; `guard_mutants.json`). Each was killed at :1005, the `bare_count` assertion.

| Mutant | Result |
|---|---|
| G1: a seam named inside src-tauri `solver_result_row_value` (RV88's site) | killed |
| G2: an `as`-aliased seam `use` at the end of runner lib.rs | killed |
| G3: a seam in a product fn at the end of PP lib.rs | killed |
| G4: a 4th `class_disclosure` mention in derivative.rs | killed |
| G5: a raw string `"#[cfg(test)] mod x {"`, then a seam | killed |
| G6: a line comment `// #[cfg(test)] {`, then a seam | killed |
| G7: a `#[cfg(test)]` statement, then a seam on the next statement | killed |
| G8: a seam in a product doc comment | killed |
| Controls C1 (a seam in src-tauri's `mod tests`), C2 (a gated `use`), C3 (a gated fn with braces in strings and chars) | pass, all 3 |

The baseline passed before and after.

## Item 5: no other change

- **The 546-input existing-behaviour sweep** (u6f_01's inputs, from the base tree) on `b10ee5cf08` against `76477534f6` is identical in all three languages:
  - Rust: 514 of 514;
  - Python: 545 of 545, AnalysisRun build, validate and schema included;
  - TS: 515 of 515.
- **The removed lines:**
  - one Python comparison line, replaced by a stricter one;
  - one TS import line, extended;
  - the tests' v2 consumers and declared-entry pins, rewritten for v3.
- **Unchanged:** the flags are false and no reader, schema or T6 file changed.

## SHA256SUMS

`SHA256SUMS` covers this report and every file in `_run_records/`. Paths are placeholders only.
