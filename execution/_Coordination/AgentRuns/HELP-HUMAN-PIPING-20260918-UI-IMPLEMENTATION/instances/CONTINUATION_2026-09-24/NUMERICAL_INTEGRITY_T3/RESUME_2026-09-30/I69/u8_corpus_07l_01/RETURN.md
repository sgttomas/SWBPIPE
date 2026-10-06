# I69 U8-2: corpus 07l (Python)

TASK (Type 2), I69, for ROOT (HELP_HUMAN, Agent 0). 2026-10-06 UTC (2026-10-05 host local time).

**Briefs (verified before work):** `R/BRIEFS/U8_COMMON.md` sha256 `3146c3e6c2d0940cfe1870da75192db4b37e2a5c7f690bc3cc5acba953eb223b`; `R/BRIEFS/I69_I70_I71_U8_CORPUS_AND_READERS.md` sha256 `acb04c2596ae1fdd96fff5fb4b13babba0472184cd14a1c6e723c096cf4962e7` (the I69 section only). **Basis read:** `R/I61/u8_plan_01/PLAN.md` (sha256 `f274a614…`, verified) §1.2 and §1.4; RR "I61's U8 plan ruled; …" (decision 6), "I68's probe verified: …" and "U8-1 committed; …"; `R/I68/u8_probe_01/PROBE.md`, `R/I68/u8_witnesses_01/RETURN.md`; `R/I62/coverage_shared_python_01/SNAPSHOT_05_PLAN.md` §1.2.

**Placeholders:** `WT`, `NUM`, `P`, `PP`, `R`, `RR`, `VENV` as in the dispatch; `CAND` = `WT/f2a-u8` (U8 head `d449097085` plus the two uncommitted files below); `ARCH` = `WT/scratch/i69_u8/arch` (a disposable `git archive` of `d449097085`, used for development, reader-mutant probes and the Rust/TS pre-checks).

**Host rules kept:**
- Writes in CAND: exactly the two fenced files, left uncommitted. No Git writes (reads used `GIT_OPTIONAL_LOCKS=0`).
- Cargo only through `WT/tools/t3_cargo.sh` (memguard up), `--locked --offline`, `CARGO_BUILD_JOBS=4`, `RUST_TEST_THREADS=2`, RUSTFLAGS unset, target `WT/targets/i69-u8/`. Three jobs: the two CLI authorities and one records-only `result_export` pre-check (`_run_records/logs/cargo_jobs_i69.log`). No DEC-025, native or solver-at-scale job.
- Python: `VENV/bin/python`, `-p no:cacheprovider`, `PYTHONDONTWRITEBYTECODE=1`, `TMPDIR` and `--basetemp` under `WT/scratch/i69_u8/`. Nothing written to the system temp directory.
- No production, reader `src/`, schema or fixture text changed. F-1 is not pinned by any entry (no dense range-scaled base is added).

## 1. Changed files (CAND; both inside the fence)

| File | sha256 before (`d449097085`) | sha256 after | Change (`git diff --numstat --histogram`) |
|---|---|---|---|
| `P/fixtures/results/retained_precision_cases.json` | `482449bfee58e3193937deaed618ff50245895713be6c4a47772671dce0a6167` (07k) | **`5ac13296c69745ae837e41b93c42e4daa0c4100dc863229f8e6a4cdbe2692ccd`** (07l), 5,517,810 B | +16,139 / −1. The one deleted line is the old top-level claim; everything else is appended |
| `P/tests/test_retained_precision_contract.py` | `55178c932bdc0441ae23a38db06fd1f7cbddbbbe0cbfaf257cbcace8c01e83f8` | **`729405c6352a2f6ac83ef04eddf0bbdcac9c91a10cd4e0b2e9c0aec8036bd794`** | +68 / −7 (`_run_records/candidate_test.diff`) |

`git status --short` in CAND shows exactly these two paths (`_run_records/candidate_status.txt`).

## 2. Corpus 07l

**Counts:** (cases, mutations, must-pass) **(15, 278, 24) → (17, 286, 28)**, so **k = 8, j = 4**. Eligible: bases 13 → 15, must-pass entries 14 → 18.

**Built by** `_run_records/scripts/build_07l.py` (entries from `entries_07l.py`), in the file's own format (`json.dumps(indent=2)` plus a newline, asserted on input). The builder asserts that the input is 07k by sha256 and that every existing case, mutation, must-pass entry, the arithmetic vectors and the D37 table are unchanged. New items are **appended** (cases 15–16, mutations 278–285, must-pass 24–27), so no existing index or slice moves; `cases[0]`, which the Rust `src` `cfg(test)` module reads, is unchanged.

**Top-level provenance:** the claim is now decision 6's text verbatim, `"synthetic controls plus listed producer-solved bases; no native Current evidence"`. `kind` stays `synthetic_control`, because the ruling amends the claim only (see §6, item 5).

### 2.1 The two producer-solved bases

| | `u8_l0_isolated_node_sparse_interactive` (case 15) | `u8_l0_isolated_node_dense_scrutiny` (case 16) |
|---|---|---|
| `source`, `invocation` | the values of `P/fixtures/results/retained_precision_l0_successor_sparse_interactive.json` (sha256 `93c6c865…`) | the values of `…_dense_scrutiny.json` (sha256 `dbb3d477…`) |
| receipt sha256 | `c00cbe76954e5188…` | `0b4250c8139ba25a…` |
| `expected` | eligible with the invocation | eligible with the invocation |
| classes (relative / absolute / input / non-quantity) | 25 / 78 / 9 / 1 | 25 / 78 / 9 / 2 |

**Case-level `provenance`** (an object; the 15 synthetic cases keep their string): `kind: "producer_solved"`; `producer` = the source's own `producer` object (`open_pipe_stress_product_physics` 0.2.0, `openpipestress.result_semantics/0.3.0/preview-physics-retained-1`); `entry: "run_linear_static_preview_value_with_retained_direct"`; `build_identity` = the registered identity text (`v1;rustc.release=1.97.1;rustc.commit=8bab26f4…;…;profile=debug;opt_level=0;debug_assertions=true;rustflags=;pkg=open_pipe_stress_product_physics@0.2.0`, PP `REGISTERED_PROFILES[0]`); `u8_head: "d44909708529c6277fc1fd3b22997218296c8dd3"`; `solver_mode`; `fixture` (repository path relative to P); `fixture_sha256`; `receipt_sha256`; and `pinned_by` (the two PP tests that pin and D-U6-5-check the fixtures). `qualification` states the L = 0 derivation (milestone request plus N2 at (3, 0, 0), `rigid:N2` restraining six DOFs) and "Not native Current evidence". The full objects are in `_run_records/appended_07l.json`.

**D-U6-5, as applied to an embedded base.** The fixture files themselves are the byte-identical copies (I68's PP test). In the corpus each base's `source` and `invocation` are the fixture's JSON **values exactly**: the new Python test compares `json.dumps` of both sides, which is strict on key order, int against float, signed zero and float bits, and pins the fixture file's sha256. The corpus text is not the fixture's text: Python's serializer writes 11 (sparse) / 12 (dense) floats in a different exponent form (e.g. `9.6e-09` against Rust's `9.6e-9`); the binary64 values are identical (Rust parses with `float_roundtrip`; TS `JSON.parse` is correctly rounded). See §6, item 1.

**`expected_classifications`** are the accepted Python reader's output on the unedited base, admitted only after these independent checks (all in `build_07l.py`):
- (a) **the producer's own claims:** the absolute-verified roster with its bounds equals the receipt's `selection.absolute_verified`; `not_covered` is empty; the input-derived rows are exactly the displacement rows of `selection.input_derived_dofs`;
- (b) **I68's three-reader observation:** 25/78/9/1 (sparse) and 25/78/9/2 (dense) in Rust, Python and TS (PROBE.md §3);
- (c) **the pinned milestone successor:** every one of its 98 (sparse) / 99 (dense) rows is present with an identical classification (normalized, scale, class and bound bits); the 15 extra rows are body 1's, 6 input-derived plus 9 absolute-verified exact zeros (normalized, scale and bound all `0`).

### 2.2 The appended entries

All are on the two producer-solved bases, `rehash: "all"`, with only explicit synthetic attestations edited. Paths: `COV1` = `retained_precision.body.product_attempts[0].proof.summary_coverage[1]` (body 1), `SEL` = `…cases[0].selection`, `VER` = `…cases[0].run.records[1].verification`. Body 1 is the memberless node N2 (extent L = 0): its six displacement rows are input-derived, its translation has one non-input row (`displacement_magnitude`), its rotation has none, and its force and moment come from six reaction rows and the two support magnitudes.

| Id (each `_sparse_interactive` and `_dense_scrutiny`) | Edits | Expected first gate and code | Rule that fires (Python reader, `core/analysis_runs/retained_precision.py` at `e58938be…`) |
|---|---|---|---|
| **mutation** `isolated_rotation_stop` (SNAPSHOT_05_PLAN §1.2) | `COV1.stop` = [F,T,F,F]; `SEL.stop_rule` + (1, rotation, 0) | **G5a `RETAINED_PRECISION_SCALE_MISMATCH`** | `_g5a_coverage` feasibility, `need(feasible)` (:1247): no permitted A, since rotation has no non-input row (:1240) |
| **mutation** `isolated_has_data` (§1.2) | `COV1.has_data` = true; `SEL.certified_bound` + (1, 1.0); `VER.bound[1]` = 1.0; `VER.data_blocks` = 2 (so the B list, record bound, theta and data-block rules all hold) | **G5a `RETAINED_PRECISION_SCALE_MISMATCH`** | the no-free-DOF rule, `if not free: need(not has_data)` (:1273) |
| **mutation** `isolated_estimate_coupled` (§1.2) | body 1 E = (force 1.0, moment 0) in `SEL.resolution_scale` and `VER.resolution`; `SEL.verification_estimate` and `SEL.verification_charge` + (1, force), (1, moment) | **G5a `RETAINED_PRECISION_SCALE_MISMATCH`** | the estimate roster (:1260): at L = 0 the hats stay uncoupled (:1250), so only `force` is required |
| **must-pass** `isolated_estimate_uncoupled` (§1.2) | as above, with only (1, force) added to estimate and charge | **pass**, eligible; the base's classifications | — |
| **mutation** `isolated_translation_rotation_stop` (I69) | `COV1.stop` = [T,T,F,F]; `SEL.stop_rule` + (1, translation, 0), (1, rotation, 0) | **G5a `RETAINED_PRECISION_SCALE_MISMATCH`** | feasibility (:1247): at L = 0, translation does not imply rotation (:1241) |
| **must-pass** `isolated_translation_stop` (I69) | `COV1.stop` = [T,F,F,F]; `SEL.stop_rule` + (1, translation, 0) | **pass**, eligible; the base's classifications | — |

The probe of every entry with its firing line is `_run_records/logs/probe_entries.log`. **I69's pair** (the last two rows) is not in SNAPSHOT_05_PLAN's list. It is added because PLAN §1.2 says the L = 0 base proves "the feasibility rule's L = 0 branch", and the plan's entries do not discriminate that branch: [F,T,F,F] is infeasible under both the L = 0 and the coupled reading. A reader that coupled translation and rotation at L = 0 would refuse `isolated_translation_stop` and admit `isolated_translation_rotation_stop` (§3.2, RM2). See §6, item 2.

## 3. Python results

### 3.1 The suites

| Run | Where | Result |
|---|---|---|
| The three retained suites (contract, schema, carriers), base | CAND before installing (= `d449097085`) | **463 passed** (console only; the baseline sweep below re-ran the same three suites, all passing) |
| The same, candidate | ARCH with the two candidate files (byte-identical to CAND's) | **480 passed** (`_run_records/logs/arch_retained_1.log`) |
| The 24-file sweep (I66's list, which includes the contract suite) plus the schema and carrier suites, base | CAND before installing | **1,856 passed, 30 skipped** (`logs/sweep_baseline.log`) |
| The same, candidate | CAND after installing | **1,873 passed, 30 skipped** (`logs/sweep_candidate.log`) |

**Collected test ids** (`logs/collect_*`): the retained suites go from 463 to 480 and the sweep from 1,886 to 1,903, with **17 additions and no removal or rename**: the 8 new mutation and 4 new must-pass parametrizations, the new `test_producer_solved_bases_are_the_pinned_live_successors_d_u6_5`, and 4 parametrizations that `tests/test_retained_precision_schema.py` (outside my fence, unchanged) derives from the corpus cases (`test_rv78_n2_y0_every_successor_statement_has_a_valid_derivative_shape` and `test_rv78_n2_receipt_and_branch_probes` on each new base). The baseline had 0 failures (1,856 passed, 30 skipped); the candidate has 0 failures, the same 30 skipped and 17 more passed, matching the 17 added ids. (The logs carry totals, not per-test skip lines, so the skip sets are compared by count only.)

**The test file's changes** (`candidate_test.diff`): the module docstring takes decision 6's wording; two comments take the new eligibility numbers; `test_snapshot_07_counts_and_entry_format` pins 07l (`(17, 286, 28)`, eligible `(15, 18)`) plus the ids, order and bases of the appended slices; the new test checks the D-U6-5 identity, the provenance fields, the top-level provenance, that exactly the two listed bases are producer-solved, eligibility with and without the invocation, the classifications and I68's class counts. No assertion is deleted or loosened; the two count pins change only their expected values.

### 3.2 Discrimination evidence (records only; no file edited)

**Reader mutants** (`scripts/reader_mutants.py`, `logs/reader_mutants.jsonl`): each patches the Python reader's `_g5a_coverage` in memory and runs every 07l case, mutation and must-pass entry.

| Mutant | Killed by | Killed by any 07k entry? |
|---|---|---|
| none (control) | nothing | — |
| RM1: the A-exclusion removed (:1240) | `isolated_rotation_stop` ×2, `isolated_translation_rotation_stop` ×2 | **no** |
| RM2: feasibility coupled at L = 0 (:1241) | `isolated_translation_rotation_stop` ×2, `isolated_translation_stop` ×2 (I69's pair only) | **no** |
| RM3: hats coupled at L = 0 (:1250) | `isolated_estimate_coupled` ×2, `isolated_estimate_uncoupled` ×2 | **no** |
| RM4: the no-free-DOF rule removed (:1273) | `isolated_has_data` ×2 | **no** |

**Test mutants** (`scripts/test_mutants.py`, `logs/test_mutants.jsonl`; ARCH corpus edited, then restored and sha-checked): a one-ulp change in a base row, an integral float `1.0` for the receipt version (accepted by the reader under D25/D32, so only the strict D-U6-5 comparison catches it), a wrong `u8_head`, the old claim, a dropped must-pass entry and two swapped appended mutations. **6 of 6 are killed**: the first four by the new D-U6-5 test, the last two by the counts test.

## 4. Pre-checks for I70 and I71 (records only, in ARCH; not their runs)

- **Rust** (`logs/rx_precheck.log`): the full `result_export` suite on ARCH, with the 07l corpus and the two count literals changed **in the scratch copy only** (`logs/rx_precheck_scratch_counts.diff`): **172 passed, 0 failed** (the contract test 62, the `src` lib tests 24). So the Rust reader accepts both bases with their eligibility, classifications and bounds, refuses all 8 new mutations at G5a SCALE and admits the 4 must-pass entries. **The Rust test has two count pins:** `:281` (`278` → `286`) and **`:835` (`entries.len()` `24` → `28`), which lies outside the brief's ":276–281"**. A comment at `:206` ("13 bases are eligible") also goes stale.
- **TS** (`logs/ts_precheck_verbose.log`): vitest on `retainedPrecision.test.ts`, unchanged, with the 07l corpus: **470 passed** (452 before, by count: 2 bases × 3 tests plus 8 plus 4). The TS test has **no count pin**, so I71 may have nothing to change. Setup followed I68's: an untracked `node_modules` symlink in ARCH and the wasm assets copied from `WT/sweep-skewpin` (hashes in `logs/ts_wasm_assets.sha256`; the reader does not use them). Vite's transient config passed through the shared, pre-existing `node_modules/.vite-temp/` (created 2026-10-03), which is empty again. tsc was not run.

## 5. Stops

None fired. Python's reader accepts both faithful bases (and so do Rust's and TS's, in the pre-checks). No production change, no existing byte changed outside the claim line, nothing weakened, all writes inside the fence.

## 6. For ROOT

1. **D-U6-5's reading for an embedded base.** I read "byte-identical copy" as: the fixture files are byte-identical to the live successors (I68), and each corpus base's `source` and `invocation` are those files' JSON values exactly (strict serialization equality, file sha256 pinned). Text identity inside the corpus is not possible in its format (§2.1). If ROOT wants text identity, the corpus would have to reference the file, a format change in all three harnesses beyond counts and pins.
2. **I69's discriminating pair** (2 mutations, 2 must-pass) goes beyond SNAPSHOT_05_PLAN's list. It is the only part of the corpus that kills RM2, the feasibility rule's L = 0 branch that PLAN §1.2 names. Dropping it gives (17, 284, 26).
3. **Every plan entry is applied on both modes.** The rules involved are mode-independent; the dense copies show the dense producer-solved base works with the same edits. Sparse only would give (17, 282, 25) without I69's pair.
4. **I70's fence must include `:833–835`** of `result_export/tests/retained_precision_contract.rs` (the must-pass count `24` → `28`), as well as `:281` (`278` → `286`). I71's TS test has no count pin.
5. **Top-level `provenance.kind` stays `"synthetic_control"`.** Decision 6 amends the claim only. The claim is written verbatim (lowercase, no final period). Changing `kind` (e.g. to `mixed`) is a one-line follow-on if ROOT rules it. No reader test reads either field; Python now pins both.
6. **Case-level `provenance` is an object** for the producer-solved bases, while synthetic bases keep their string. No Rust or TS test reads case provenance (grep), and Python's new test uses the string to list exactly the producer-solved bases.
7. **For I72 (Pass B):** the corpus is also `include_str!`'d by `result_export/src/retained_precision.rs`'s `cfg(test)` module (`u6e_reader_round_tests`, which reads only `d37` and `cases[0]`, both unchanged). It is not among PP's `REVIEWED_INPUTS`, so the registered identity does not change, but the lib test build of `result_export` sees a changed input.

## 7. Commands (cwd as stated; `E` = `OPENPIPESTRESS_UNITS_BIN=WT/targets/i69-u8/units-authority/release/openpipestress_units OPENPIPESTRESS_CHECKED_JSON_BIN=WT/targets/i69-u8/checked-json/release/openpipestress_jcs_ijson PYTHONDONTWRITEBYTECODE=1`)

1. CLI authorities, in `CAND/P/core/serialization/canonical_json` and `CAND/P/core/units`: `env -u RUSTFLAGS -u CARGO_ENCODED_RUSTFLAGS CARGO_BUILD_JOBS=4 WT/tools/t3_cargo.sh build --locked --offline --release --features checked-cli --bin openpipestress_jcs_ijson --target-dir WT/targets/i69-u8/checked-json`, and the same with `--features cli --bin openpipestress_units --target-dir WT/targets/i69-u8/units-authority` (rc 0, 0).
2. `GIT_OPTIONAL_LOCKS=0 git -C WT/f2a-u8 archive d449097085 projects/chirality-piping | tar -x -C WT/scratch/i69_u8/arch/`.
3. Baselines in CAND (clean): `E VENV/bin/python -m pytest -p no:cacheprovider -q <the three retained suites>` (463 passed); `scripts/run_py.sh CAND/P sweep_baseline sweep`.
4. Build: `E VENV/bin/python scripts/build_07l.py ARCH/P <staged corpus>`; the test edits made in ARCH; then `scripts/probe_entries.py ARCH/P`, `scripts/run_py.sh ARCH/P arch_retained_1 retained`, `scripts/reader_mutants.py ARCH/P <none|RM1…RM4>`, `scripts/test_mutants.py ARCH/P VENV/bin/python -m pytest -p no:cacheprovider -q -rf tests/test_retained_precision_contract.py`.
5. Pre-checks: in `ARCH/P/core/reporting/result_export`, `env -u RUSTFLAGS -u CARGO_ENCODED_RUSTFLAGS CARGO_BUILD_JOBS=4 RUST_TEST_THREADS=2 CARGO_TARGET_DIR=WT/targets/i69-u8/rx WT/tools/t3_cargo.sh test --locked --offline --no-fail-fast` (rc 0); in `ARCH/P/apps/desktop`, `../../node_modules/.bin/vitest run --reporter=verbose src/features/results/retainedPrecision.test.ts` (rc 0).
6. Install: `cp` of the two files into CAND (sha256 checked); `--collect-only` on both trees; `scripts/run_py.sh CAND/P sweep_candidate sweep`.

## 8. Cleanup and limits

- **Deleted on return:** `WT/scratch/i69_u8/` (ARCH, the node_modules symlink and copied wasm assets inside it, logs, temp) and `WT/targets/i69-u8/` (the two CLI authorities and the `result_export` pre-check build). Neither I70 (Rust) nor I71 (TS) uses the Python CLI authorities; they rebuild in about 8 s by step 1.
- CAND keeps the two uncommitted files for ROOT.
- **Limits:** the base `expected_classifications` come from the Python reader under test, checked against the producer's claims, I68's three-reader counts and the milestone's rows (§2.1), not an independent oracle replay; the Rust and TS pre-checks ran on ARCH with scratch-only edits and are not I70's or I71's results; tsc was not run.
