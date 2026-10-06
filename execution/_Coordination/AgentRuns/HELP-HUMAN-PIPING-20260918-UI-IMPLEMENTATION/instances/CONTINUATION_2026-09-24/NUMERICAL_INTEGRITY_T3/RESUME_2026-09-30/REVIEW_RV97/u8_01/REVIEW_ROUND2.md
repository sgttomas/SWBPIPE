# RV97: independent review of U8, round 2 (corpus 07l and the reader alignment)

**Reviewer:** RV97, TASK (Type 2), continued by ROOT (HELP_HUMAN, Agent 0) in the same context as round 1 (`REVIEW.md` in this folder). ROOT is the return path. No descendants. I wrote none of the work under review.

**Basis:**
- **ROOT's round-2 message:** items 3, 4 (07l parity, D-U6-5 for the embedded bases, the provenance claim) and 5 on the new commits, and a re-check of I71's wasm provenance reasoning.
- **The rulings:** RR "I76's return verified; RV97 passes U8's round 1; erratum E-6", "I69's corpus 07l committed; I70 and I71 dispatched" and "I70's Rust alignment committed; …". They accept I69's readings (D-U6-5 for an embedded base is value equality with the pinned fixture; `provenance.kind` stays top-level `synthetic_control`; I69's added stop pair) and my round-1 N-4 (my declared `u5_compare.py` variant is the records-level U5 replay, for 07l's producer-solved base too).
- **The accounts, read in full:** `R/I69/u8_corpus_07l_01/RETURN.md`, `R/I70/u8_rust_07l_01/RETURN.md` and `R/I71/u8_ts_07l_01/RETURN.md`.

**Candidate:** `d449097085..bd6b4be2c3` on `codex/piping-f2a-u8-20261005`: three commits, four files, +16,275 / −14.

| Commit | Author's unit | File | sha256 at `bd6b4be2c3` |
|---|---|---|---|
| `69a925bd68` | I69, U8-2 | `P/fixtures/results/retained_precision_cases.json` (07l) | `5ac13296…` |
| | | `P/tests/test_retained_precision_contract.py` | `729405c6…` |
| `de01e43bc4` | I70, U8-3 | `RE/tests/retained_precision_contract.rs` | `7832a602…` |
| `bd6b4be2c3` | I71, U8-3 | `DT/features/results/retainedPrecision.test.ts` | `59531ed1…` |

**Host.**
- **The copy:** a fresh `git archive bd6b4be2c3` (P without `execution/`) in `WT/rv97/cand`.
- **Targets and scratch:** targets in `WT/targets/rv97/{checked-json,units-authority,rx}`; scratch in `WT/scratch/rv97_u8_01/r2/`.
- **Cargo:** three jobs through `WT/tools/t3_cargo.sh` (memory guard PID 5387 up), `--locked --offline`, `CARGO_BUILD_JOBS=4`, `RUST_TEST_THREADS=2`, RUSTFLAGS unset: the two reader CLI authorities and one `result_export` test run.
- **Node:** the existing `node_modules` through an untracked symlink in my copy only, and the eight wasm assets copied from `WT/sweep-skewpin` (the same hashes as I68, I69, I71 and my round 1).
- **Never done:** Git writes (reads with `GIT_OPTIONAL_LOCKS=0`), installs, DEC-025, native or solver jobs, writes to the system temp directory or to `WT/f2a-u8` (clean at the end). The host's write guard did not refuse my writes into NUM.
- **Cleanup:** I deleted the copy (with the symlink and the assets), the targets and the scratch at the end.

## Verdict (round 2): **PASS**

| | Count |
|---|---|
| BLOCKING | 0 |
| SHOULD-FIX | 0 |
| NOTE | 2 |

**The headline:**
- **07l parity holds on identical bytes.** I wrote every 07l item as one document (17 cases with and without the invocation, 286 mutations, 28 must-pass entries: 348 documents) and fed the same bytes to all three readers' public entries. **0 discrepancies:**
  - every mutation is refused at its expected first gate and code in each reader;
  - 284 of the 286 are refused identically in all three; the other 2 differ only where the corpus declares `expected_by_reader`;
  - every case and must-pass entry is admitted with its stated eligibility and its base's classifications, bit for bit;
  - every reader gives the same `publication_sha256`.
- **The committed harnesses pass,** with the counts the accounts give: Python's three retained suites 480 passed; `result_export` 173 passed (including `snapshot_07l_mutation_outcomes`, 8 of 8); TS `retainedPrecision.test.ts` 474 passed; `tsc --noEmit` clean.
- **D-U6-5, as ruled:**
  - each embedded base's `source` and `invocation` equal the pinned fixture's under a strict walk (key order, types, −0.0, float bits);
  - the only text differences are 11 / 12 number spellings, all the same binary64;
  - U5's replay (my accepted variant) on the embedded bases gives STOPS `[]`, identical to round 1's.
- **The provenance claim is truthful,** at the top level and per case.
- **Nothing weakened, only fenced files, no reader `src`.**
- **I71's wasm provenance reasoning holds, and I corroborate it independently:** the assets embed panic-location paths under `WT/sweep-skewpin/P/core/…` and the registered toolchain's commit, and no engine source in that worktree changed after its 08:10 checkout of `e2b83da584`.

## Findings

| # | Sev | Where | Evidence | Remedy |
|---|---|---|---|---|
| R2-N-1 | NOTE | `R/I69/u8_corpus_07l_01/RETURN.md` §4, the TS pre-check bullet ("the reader does not use them") | **E-6's misstatement recurs in I69's record.** E-6 corrected PROBE.md's two sentences. I69's §4 says the same of the wasm assets it copied for its TS pre-check, but the TS reader does use them (G8's unit conversion and canonical hashing; round-1 N-1). I71's §6 records the dependence correctly, and the verdicts stand (§4 below). | Record-level only: extend E-6 to I69's §4 sentence. |
| R2-N-2 | NOTE | `RE/tests/retained_precision_contract.rs:1–2` (`de01e43bc4`) | **One retained sentence now understates.** The module doc reads "Shared synthetic statement controls plus listed producer-solved bases (07l); no native Current evidence. These do not establish execution." The two listed bases were published by an actual execution of the Direct entry in the registered dev/test build; their own `qualification` says so. The sentence makes no over-claim (native Current is disclaimed), so the claim fence holds, but it is now imprecise. | Optional, at the next touch: e.g. "Reading them establishes no execution; no native Current evidence." No action needed for U8. |

There are no BLOCKING or SHOULD-FIX findings.

## 1. Item 4: 07l parity across the three readers

**The method** (`evidence/round2/tools/`):
1. **One set of bytes.** `gen_docs.py` writes each corpus item as one document, using the candidate's own Python entry semantics (`apply_entry`: the edits, the invocation digest, rehash "all", then `after_rehash`). It writes the 17 bases twice (with and without the invocation), and the 286 mutations and 28 must-pass entries once. That is 348 documents (`parity/documents.sha256`).
2. **Three public entries on the same bytes:**
   - Python: `validate_retained_precision`, with CLI authorities built from the candidate copy;
   - Rust: `validate`, from `zz_rv97_parity.rs`, an integration test in my copy only;
   - TS: `validateRetainedPrecision` via vitest, from `zz_rv97_parity.test.ts`.
3. **The comparison** (`compare_parity.py`) checks each reader against the corpus's expectation for that reader (`expected_by_reader`, else `expected`) and against the other two readers. For every admitted document it also compares the bound, eligible and standing flags, the full classification list (result id, class, normalized, scale and bound bits) against the base's `expected_classifications`, and the `publication_sha256`.

**The result** (`parity/parity_comparison.txt`): **0 problems in 348 documents.**

| Set | Python | Rust | TS |
|---|---|---|---|
| Cases with the invocation, 17 | 17 admitted, expected eligibility (15 eligible), classifications exact | the same | the same |
| Cases without the invocation, 17 | 17 admitted, not bound, not eligible | the same | the same |
| Mutations, 286 | 286 at the expected first gate and code | the same | the same |
| Must-pass, 28 | 28 admitted, stated eligibility (18 eligible), base classifications exact | the same | the same |

**Mutation first failures:** 284 of 286 are identical in all three readers. The two that differ are exactly the corpus's two declared per-reader entries:
- `g7_maximum_off_enclosure` (Rust `SOURCE_PREVIEW_PHYSICS_EXTREMA_BOUNDS`);
- `g7_not_required_quality_enum_invalid` (TS `SOURCE_PRODUCER_CONTRACT_UNSUPPORTED`; N-3's codes, aligned later by ruling 11).

The per-reader gate tallies are in `parity_comparison.txt`.

**The 07l additions, all three readers:**

| Entry (each `_sparse_interactive` and `_dense_scrutiny`) | Python, Rust, TS | Python's firing rule (`parity/py_l0_firing_lines.txt`) |
|---|---|---|
| base `u8_l0_isolated_node` | admitted, bound, eligible, 113 / 114 rows; publication `79d42693…` / `ee5a3f29…` in all three (= round 1's) | — |
| the same, without the invocation | admitted, not bound, not eligible | — |
| mutation `isolated_rotation_stop` | G5a `RETAINED_PRECISION_SCALE_MISMATCH` | `_g5a_coverage:1247`, `need(feasible)` |
| mutation `isolated_has_data` | G5a SCALE | `:1273`, `if not free: need(not has_data[…])` |
| mutation `isolated_estimate_coupled` | G5a SCALE | `:1260`, the estimate roster |
| mutation `isolated_translation_rotation_stop` | G5a SCALE | `:1247`, feasibility |
| must-pass `isolated_estimate_uncoupled` | admitted, eligible, base classifications | — |
| must-pass `isolated_translation_stop` | admitted, eligible, base classifications | — |

The firing rules agree with I69's account, and each discriminating pair splits as designed: the coupled estimate refuses while the uncoupled one passes, and the translation-plus-rotation stop refuses while the translation stop passes. For the readers' sensitivity to each rule, I rely on I69's, I70's and I71's reader mutants (RM1–RM4, each killed only by 07l entries in all three readers); I did not re-run them.

**The committed harnesses on my copy** (`evidence/round2/suites/`):
- **Python:** `test_retained_precision_contract.py`, `_schema.py` and `_carriers.py`: **480 passed** (I69: 480).
- **`result_export`, full suite: 173 passed, 0 failed** (I70: 173). `snapshot_07l_mutation_outcomes` prints 8 `I70_OUTCOME_07L` lines, all `match:true` at G5a SCALE, and the 4 new must-pass lines are `true`.
- **TS:** `retainedPrecision.test.ts`: **474 passed** (I71: 474), including the four new 07l tests.
- **`tsc --noEmit -p tsconfig.json`** (TypeScript 5.9.3): rc 0, no output.

## 2. Item 4: the producer-solved bases equal the fixtures (D-U6-5, as ruled)

`corpus/d_u6_5_check.txt`, from `tools/d_u6_5_check.py`:
- **Strict value equality.** A parallel walk of each base's `source` and `invocation` against the fixture file's (key order, Python types, −0.0 and float bits) finds **no difference**, for cases 15 and 16. Each fixture's sha256 equals the pinned `93c6c865…` / `dbb3d477…`, and each fixture's `id` equals the case id.
- **The text.** The fixtures' number tokens (1,277 / 1,278) against the same values in the corpus's format differ in **11 / 12 spellings**, for example `9.6e-9` against `9.6e-09` and `0.00003333333333333333` against `3.333333333333333e-05`. All parse to the same binary64. This is exactly I69's account.
- **Cross-parser confirmation:** from the corpus-spelled bytes, all three readers give the publication hashes that the fixture-derived documents gave in round 1. Rust parses with `serde_json`'s `float_roundtrip` (`RE/Cargo.toml:12`).
- **U5 on the embedded bases** (`u5/`): my accepted variant (`u5_compare_l0_variant.py`, unchanged from round 1) on documents built from cases 15 and 16 gives **STOPS `[]`**. Every summary field except the input file's hash, and every row entry, equals my round-1 report on the fixtures.

## 3. Item 4: the provenance claim is truthful

- **Top level:** `{"kind": "synthetic_control", "claim": "synthetic controls plus listed producer-solved bases; no native Current evidence"}`. This is decision 6's text, and `kind` is as ruled.
  - The corpus holds exactly two producer-solved bases, the listed ones (cases 15–16, case-level `kind: producer_solved`). The other 15 keep the string `synthetic_reader_control_not_producer_execution_or_native_current`.
  - Nothing in the corpus is native Current evidence.
- **Per case, every field checked against its source:**

| Field | Checked against |
|---|---|
| `producer` | equals the base source's `producer` object |
| `entry` | `run_linear_static_preview_value_with_retained_direct`; round 1 established that the fixture was written from that entry's registered-build output |
| `build_identity` | occurs exactly once in `retained_memory.rs`'s `REGISTERED_PROFILES` text at `bd6b4be2c3` |
| `u8_head` | `d449097085`, the commit that added both fixtures (`git log --diff-filter=A`) |
| `fixture`, `fixture_sha256`, `receipt_sha256` | the files and pins |
| `pinned_by` | the two PP tests exist at `bd6b4be2c3` |

- **The case-level `qualification`** (beside `provenance`) is also true:
  - the derivation: N2 at (3, 0, 0), `rigid:N2` with six restraints, coverage `stop [F,F,F,F]`, `has_data false`;
  - "Published by the actual Direct entry in the registered dev/test build";
  - "D-U6-5 copies" (value copies, as ruled);
  - "Not native Current evidence";
  - "the public reader accepts it, eligible with its invocation".

## 4. The wasm assets: I71's provenance reasoning re-checked

**I71's chain** (§6) is: the eight assets' hashes equal every earlier record's; `WT/sweep-skewpin`'s HEAD has been at `e2b83da584` since its checkout at 08:10:02; the assets were written at 08:47; the worktree is clean; and the engine's 17-crate closure is equal at `e2b83da584` and the U8 heads, apart from test-only files.

**What I re-derived independently** (`evidence/round2/wasm/provenance_recheck.txt`):
- **Asset hashes:** the eight assets I used are hash-identical to I71's list.
- **Revision and tree:** the reflog, the HEAD (`e2b83da5841e…`) and a clean worktree.
- **File times:** 08:47:13 (`wasm-engine`) and 08:47:29 (`self-weight-engine`) on 2026-10-05.
- **The closure:** the same 17 crates, from `Cargo.toml` path dependencies at `bd6b4be2c3`.
  - Across `e2b83da584 → bd6b4be2c3`, the closure plus `apps/desktop/scripts` differs only in `PP/src/retained_facade_tests.rs` (compiled only under `#[cfg(test)]`, `lib.rs:128–129`) and `RE/tests/retained_precision_contract.rs` (an integration test).
  - `schemas/` is unchanged.
  - The three changed fixtures are included only by PP's `cfg(test)` facade tests, `RE/src/retained_precision.rs:4349–4353` (inside `#[cfg(test)] mod u6e_reader_round_tests`, which runs to the end of the file) and `RE/tests/`.

**What I add:**
- **Where the assets were built.** Both `.wasm` binaries embed panic-location paths under `WT/sweep-skewpin/P/core/product_physics/src/self_weight.rs` and `…/canonical_json/src/lib.rs`, and 20 / 23 standard-library paths under `rustc/8bab26f4f68e…`, the registered toolchain's commit. So they were built in that worktree, with rustc 1.97.1.
- **No source edits after the checkout.** No file in the closure, in `apps/desktop/scripts` or in `schemas/` in that worktree has an mtime after the 08:10:02 checkout. The newest `src` mtimes there are from 2026-10-04 17:34, files unchanged by the later checkouts.

**Conclusion.** The assets were built at 08:47 in `WT/sweep-skewpin` from the tree checked out at `e2b83da584`, whose engine sources equal the U8 heads' for the non-test wasm build. I71's inference stands. The one residual assumption, also I71's, is that no source was edited and restored with its old mtime between 08:10 and 08:47. A build-from-source witness would not compare byte for byte anyway, because the binaries embed the worktree's absolute paths.

## 5. Item 3: nothing weakened, only fenced files, no reader `src`

`evidence/round2/scope_round2.txt`:
- **Only the four fenced files changed.** No reader `src` (`RE/src`, `core/analysis_runs`, `retainedPrecision.ts`), no schema and no PP file. `WT/f2a-u8` is clean.
- **Every deleted line is a comment, a docstring, or a count pin whose value changed:**
  - Rust `278 → 286` and `24 → 28`;
  - Python `(15, 278, 24) → (17, 286, 28)` and `(13, 14) → (15, 18)`;
  - the corpus's one deleted line is the old claim.
  
  No assertion is removed or loosened. The TS file is pure addition.
- **The corpus, 07k → 07l, structurally** (`corpus/corpus_07k_vs_07l.txt`): every top-level key is kept; `version`, `arithmetic` and `d37` are unchanged; the first 15 cases, 278 mutations and 24 must-pass entries are value-identical; the additions are appended (2, 8 and 4). The text diff is one replaced line (the claim) and three pure insertions.
- **Strengthening only:** Python pins the appended ids, order and bases, and adds the D-U6-5 test. Rust adds the 07l slice. TS adds count pins and literal per-entry expectations (I71 item 3, which pins `provenance.kind` and I68's class counts, is consistent with Python's pins).

## 6. Item 5: scope truth

- **No added line claims a receipt Ceiling row or native Current evidence.** The new docstrings and the claim disclaim native Current. The corpus's added text contains no "Ceiling". The `qualification` text states only what round 1 verified.
- **R2-N-2:** the Rust module doc's retained "These do not establish execution" now understates; it does not over-claim.
- **F-1 is still not pinned:** 07l adds no dense, range-scaled base; the L = 0 dense base carries its one parity row.
- **I69's, I70's and I71's records** contain no Ceiling-row or native-Current claim. R2-N-1 is the one inaccurate sentence (the wasm dependency).

## For ROOT

1. **R2-N-1:** extend erratum E-6 to I69's §4 sentence.
2. **R2-N-2:** no ruling needed (optional rewording at the next touch).
3. **U8-5 is complete** for both rounds: round 1 PASS (0/0/5, ruled) and round 2 PASS (0/0/2). I keep no copies or targets; my tools for both rounds are in `evidence/`.

## Evidence (`evidence/round2/`; placeholder paths only)

- `tools/`: `gen_docs.py`, `py_reader.py`, `zz_rv97_parity.rs`, `zz_rv97_parity.test.ts`, `compare_parity.py`, `d_u6_5_check.py`.
- `parity/`:
  - `parity_comparison.txt`, the full comparison;
  - `{py,rust,ts}_outcomes.jsonl`: per document, the first failure, or the flags with the classification count and sha256;
  - `*_parity_full.sha256`: hashes of the full outputs, which are not kept;
  - `documents.sha256` and `expectations.sha256`;
  - `py_l0_firing_lines.txt`.
- `corpus/`: `d_u6_5_check.txt` and `corpus_07k_vs_07l.txt`.
- `u5/`: the run log on the embedded bases, and the hashes.
- `suites/`: the Python, `result_export` and TS results, the `result_export` 07l lines, and tsc.
- `wasm/`: `assets_used.sha256` and `provenance_recheck.txt`.
- `scope_round2.txt`.
