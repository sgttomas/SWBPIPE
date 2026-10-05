# RV88 review of U6a (`844448112f`): the Rust carrier slice and D-U6-1

RV88 is the standing independent reviewer for U6: a TASK (Type 2) dispatched directly by ROOT (HELP_HUMAN, Agent 0) under `BRIEFS/RV88_U6_STANDING_REVIEW.md` and `BRIEFS/U6_FANOUT_COMMON.md`. ROOT is the return path, and RV88 did not delegate.

RV88 did not write this code and used none of I66's tests as oracles. Every control below is RV88's own test, script or comparison. I66's suite and mutants were rerun only as items to report.

## Verdict: PASS, with 0 BLOCKING, 2 SHOULD-FIX and 5 NOTE findings

- **Existing identities are unchanged.** RV88 ran its own sweep over 97 existing-identity envelopes, including 7 legacy 0.1.0 envelopes. Each is dispatched, classified, bound and derived exactly as at base, byte for byte. The one exception is the pre-existing nondeterminism that I66 reported as F1, which RV88 reproduced at base.
  - The result_export suite goes from 149 to 159 tests, and the 149 existing outcomes are unchanged.
  - PP's U1 and U3 pin tests pass 35/35, built against the candidate result_export.
  - runner/headless: identical outcomes at base and candidate: 85 passed, with the same 2 Mac load-reference failures.
- **The slice holds by independent derivation, in both modes.**
  - The receipt comes out of `derive_document` and `validate_document` equal to the source's, compared as canonical bytes in both Rust and Python.
  - It revalidates in the Rust and Python readers.
  - All 69 `absolute_verified` rows per mode are disclosed with the receipt's own bound bits.
  - Every other row is identical to what the **base** code derives from the reader's projection.
- **Every downgrade form RV88 could build is refused, always fail-closed,** and with the ruled code wherever a code is ruled. The forms are:
  - 224 relabel forms across eight identities;
  - 9 successor-shape forms and 19 document forms per mode;
  - legacy 0.1.0 forms;
  - the receipt, null-member and token forms injected into the 97 existing envelopes (run 2).
- **D-U6-1 is exactly the ruled change.** Across all 320 entries of 07f:
  - the Python public entry equals the draft;
  - the draft is unchanged from base;
  - all 268 mutations match the corpus's expected gate and code;
  - forcing the flag to True in-process changes eligibility and nothing else.
- **F5 is the right fail-closed choice.** ROOT should make it a parity obligation for U6b and U6d (N-1).
- **F6:** the message is truthful and claims nothing beyond the receipt's bound. It has one wording gap (S-2).
- **Mutants:**
  - I66's 62, rerun: 62/62 killed by I66's suite, none by a compile error.
  - RV88's own 7: RV88's tests kill 6 and I66's suite kills 3. Three survive I66's suite and one survives both; those survivors are S-1.

## Basis and host

- **The candidate:** `844448112f` on `codex/piping-f2a-carriers-20261004`, against base `7e4f5a51dd`.
- **The copies:** both commits were copied with `git archive <commit> -- projects/chirality-piping ':(exclude)projects/chirality-piping/execution'` from `WT/f2a-carriers`. That is a Git read with `GIT_OPTIONAL_LOCKS=0`, and the working tree was not used. The copies went to `WT/rv88/{cand,base}`, with a mutant copy in `WT/rv88/mut`.
  - The records tree was excluded because nothing that is built or tested reads it. Every crate path dependency resolves inside P.
- **The candidate's files match I66's record.** All 9 changed files hash to I66's `changed_files_sha256.txt`, and I66's SHA256SUMS checks 30/30 OK.
- **When:** 2026-10-04, from about 08:36Z to 09:55Z. The memory guard (PID 5387) ran throughout, and every RV88 script checks for it before each cargo job.
- **Cargo:** the default toolchain, with `--locked --offline`, `CARGO_BUILD_JOBS=4` and `RUST_TEST_THREADS=2`, one RV88 cargo job at a time. Targets were under `WT/targets/rv88/`.
  - The checked-JSON CLI (`--features checked-cli`) and the units CLI (`--features cli`) were built from the candidate archive with `--release`.
- **Python:** `REPO_ROOT/projects/chirality-piping/.venv` with `PYTHONDONTWRITEBYTECODE=1`, and the two CLIs above set through `OPENPIPESTRESS_CHECKED_JSON_BIN` and `OPENPIPESTRESS_UNITS_BIN`.
- **Not run:**
  - nothing native, at scale or DEC-025;
  - no TypeScript lane. U6a has no TS change, and the TS carriers are U6d's. So I66's TS back-out control is not independently reproduced here.
- **Scratch:** `WT/scratch/rv88_u6/`, deleted at the end with `WT/rv88/` and `WT/targets/rv88/`. Nothing was written to the system temp directory, and no Git writes were made.

## 1. Existing identities (RV88's sweep and the suites)

**The sweep** is `_run_records/zz_rv88_sweep.rs`, compared by `rv88_sweep_compare.py`; the output is `sweep_summary.txt`. One identical test source runs in both lanes and uses only APIs present at base.
- **What it walks:** every JSON file under P (excluding `execution`, `node_modules`, `target` and `.venv`), recursing into every nested object that has a `producer.semantic_contract_id`. Run 2 adds legacy 0.1.0 raw envelopes.
- **What it records for each envelope:**
  - `for_source` and `for_source_metadata`, as the table identity and version, or the error text;
  - `standing_reason` and `is_fresh_identity`;
  - `numerical_use_standing` with three requested lists (the `numerical_quality` case refs, empty, and the `source_block_recovery` refs);
  - the same standing with the sibling invocation, where there is one;
  - every row's `rule_binding_refusal`;
  - the sha256 of `derive_document`'s output;
  - `validate_document` on that output.
- **What it injects** into each existing envelope:
  - a receipt member;
  - a null member;
  - the W1 token on the first row and on the last row;
  - a different token value;
  - the U3 R-2 unavailable notice (run 2).

| Result | Run 1 | Run 2 |
|---|---|---|
| Existing-identity envelopes (legacy 0.1.0 among them) | 90 (0) | 97 (7) |
| Differing from base in any recorded field | 2: F1's `rejected_stress_range` pair, `derive` code only | 2: the same F1 pair (`for_source` code on one, `derive` code on the other) |
| Accepted by `for_source` / derived | 59 / 57 | 66 / 64 |
| `!receipt` and `!null`: refused `RETAINED_PRECISION_DOWNGRADE_FORBIDDEN` at raw, metadata and derive, with standing `unsupported` everywhere | 90/90 each (at base, 23 were accepted and 21 derived) | 97/97 each (at base, 30 were accepted and 28 derived) |
| `!token0` and `!tokenlast`: refused at raw and derive, with standing `unsupported` (metadata cannot see rows) | 60/60 each (at base, 21 accepted) | 67/67 each (at base, 28 accepted) |
| `!othertoken`: identical to base | 60/60 | 67/67 |
| `!r2notice` (the R-2 noticed ordinary envelope): identical to base | n/a | 97/97 |
| Base run 1 against base run 2 (the same code) | | the same F1 fixture differs **at base** (`for_source` code); the candidate's two runs also differ on it |

The adversarial probe `validate_document_seeded_receipt` puts a receipt on an existing identity's derivative. It changes as intended: at base it was refused only incidentally, with `DERIVATIVE_HASH_MISMATCH`; the candidate refuses it with `RETAINED_PRECISION_DOWNGRADE_FORBIDDEN` (57/57).

**The suites:**

| Suite | Base | Candidate |
|---|---|---|
| result_export (`cargo test`) | 149 passed | 159 passed: the same 149 with identical outcomes, plus I66's 10 |
| PP pin tests (`--lib -- retained_wire_tests retained_facade_tests`), built against the candidate archive's result_export | — | 35/35, including `u1_milestone_successor_both_modes` and `u3_permitted_path_publishes_the_pinned_successor` |
| runner/headless (`cargo test --no-fail-fast`) | 85 passed, 2 failed (`load_reference_one_actual_solve_mints_bound_evidence_and_canonical_document_both_modes`, `cli_load_reference_one_both_modes_is_controlled_and_equals_the_library_route`) | identical: the same 87 outcomes, with the same 2 failures at the same panic lines |
| Python `tests/test_retained_precision_contract.py` | 359 passed | 362 passed: the same tests plus I66's 3 |

**Addendum (after RR "U6c returned with a stop", which widened ROOT's acceptance runs).** RV88 later ran I66's 24-file schema sweep, plus `test_retained_precision_schema.py`, at U6a's base and candidate (archive lanes; `_run_records/suite24_u6a_compare.txt`):
- **`7e4f5a51dd`:** 1,735 passed, 22 failed.
- **`844448112f`:** 1,738 passed, 22 failed.

No test changes outcome. The only additions are I66's 3 D-U6-1 tests, which pass. The 22 failures are identical in both lanes:
- 19 handoff tests that read execution-record fixtures the archive excludes;
- the 3 pin tests that have been broken since the reader fan-in (RR "U6c returned with a stop").

U6a therefore changes nothing in that sweep.

**D-U6-5 is independently confirmed.** Both successor fixtures hash to the values pinned in PP's `retained_facade_tests.rs:14–15` and `retained_wire_tests.rs:27–28`. PP's `u3_permitted_path_publishes_the_pinned_successor` regenerates those bytes from committed code, and it passed in RV88's build.

## 2. The slice, by independent derivation

`zz_rv88_slice.rs` runs in both lanes and writes the candidate's successor derivative together with **each lane's** derivative of the reader projection. That projection drops the receipt and tokens and relabels the identity and profile. The comparison is `rv88_slice_compare.py`, with output in `slice_compare.tsv`.
- **Its oracles** are the receipt's own selection lists (`absolute_verified` with bound bits, `not_covered` and `input_derived_dofs`) and the **base** lane's projection derivative.
- **It uses no candidate carrier code.**

| Check | Sparse | Dense |
|---|---|---|
| The projection's derivative is byte-identical in the base and candidate lanes | yes | yes |
| `for_source` and `for_source_metadata` give the retained table and "0.3.0" (base: `SOURCE_PRODUCER_CONTRACT_UNSUPPORTED`) | yes | yes |
| Standing with no invocation, with the invocation, and with the invocation but empty refs | `needs_recompute` ×3 | `needs_recompute` ×3 |
| `derive_document`, then `validate_document` | Ok | Ok |
| Carried receipt equal to the source's, both as Rust canonical bytes and serde bytes and as Python sorted canonical text | yes | yes |
| Reattached to the raw source and revalidated with the invocation, in Rust: Validation identical, eligible false | yes | yes |
| The same in Python (`validate_retained_precision`): result identical, eligible false | yes | yes |
| `absolute_verified` rows disclosed with `retained_precision_absolute_verified`; source value and unit retained; message carries the receipt's bits and a decimal that parses back to the same bits; no "stop", "enclos" or "interval" | 69/69 | 69/69 |
| The same rows exported as quantities in the base projection's derivative | 69/69 | 69/69 |
| Every other row's accounting and target identical to the base projection's derivative | 29/29 | 30/30 |
| `not_covered` rows (F3: none exists) | 0 | 0 |
| Row checksums bind the successor's raw rows, token included, by sha256 of the checked-JSON CLI's JCS output | 98/98 | 99/99 |
| Quantity values / disclosures, successor against base | 28/70 against 97/1 | 28/71 against 97/2 |

**The retained table's 73 rows equal preview-physics-1's,** and its `inherited_semantic_contract_sha256` is `ae55503d…`, which is `PREVIEW_PHYSICS_SHA256`. So "the rest keep their table disposition" holds by construction as well as by output.

## 3. The downgrade guards and every refusal code

The checks are in `zz_rv88_refusals.rs` (candidate), which asserts fail-closed on every form; the output is `refusals.tsv`.

- **Relabel forms (224 per run):**
  - **The scope:** both modes; eight identities (the six existing ones, precision-1 and an unknown id); profile kept or set to ordinary; and seven member forms (the receipt; the receipt rehashed; null; `{}`; a string; tokens only; one token on the last row of the projection).
  - **Raw dispatch never admits.**
    - 178 are refused `RETAINED_PRECISION_DOWNGRADE_FORBIDDEN`.
    - 46 are refused earlier by the identity's own metadata checks: 38 `SOURCE_FORMULATION_BASIS_UNSUPPORTED` and 8 `SOURCE_PRODUCER_CONTRACT_UNSUPPORTED`. All 46 are token-only or profile forms; every form carrying a member gets the guard's code at raw and at metadata.
  - **Standing** is `unsupported` with and without the invocation.
  - **Derive** refuses, and the summary is empty.
- **Successor shapes:**

| Form | Raw / standing / derive |
|---|---|
| Receipt dropped | `SOURCE_PRODUCER_CONTRACT_UNSUPPORTED` |
| Receipt null | `SOURCE_PRODUCER_CONTRACT_UNSUPPORTED` |
| `schema_version` 0.3.0 or 0.4.0 | `SOURCE_PRODUCER_CONTRACT_UNSUPPORTED` |
| Ordinary profile, rehashed | `SOURCE_PRODUCER_CONTRACT_UNSUPPORTED` |
| Tokens dropped | `RETAINED_PRECISION_RECEIPT_MISMATCH` |
| Tokens dropped and rehashed | `RETAINED_PRECISION_ROW_METHOD_MISMATCH` |
| The other mode's receipt | `RETAINED_PRECISION_RECEIPT_MISMATCH` |
| Producer id removed, receipt kept | `RETAINED_PRECISION_DOWNGRADE_FORBIDDEN` |

  Transport passes three of these by design: tokens dropped (with or without rehash), and another publication's receipt. It cannot see raw rows or authenticate the publication digest (semantic_contract.rs:267–268).
- **Document forms (each refused):**
  - **`RETAINED_PRECISION_RECEIPT_BINDING_MISMATCH`:**
    - the receipt dropped, null, swapped for the other mode's, given a `receipt_sha256` edit, given a body edit, or given an extra key;
    - the projection's derivative validated against the successor source.
  - **`DISCLOSURE_SEMANTICS_MISMATCH`:**
    - one bit of the message's hex bound flipped;
    - the message truncated;
    - an absolute code changed to `not_covered`;
    - a plain disclosure claiming the class code.
  - **`RETAINED_PRECISION_DOWNGRADE_FORBIDDEN`:**
    - the successor's derivative validated against its own projection;
    - the projection's derivative carrying a receipt, or a null member;
    - a base derive seeded with a receipt, `{}` or null.
  - **Other codes:**
    - a plain disclosure carrying the class message: `DERIVATIVE_HASH_MISMATCH`;
    - a class row's `source_value` edited: `SOURCE_TARGET_VALUE_OR_HASH_MISMATCH`;
    - `semantic_contract_ref` relabelled: `SEMANTIC_CONTRACT_BINDING_MISMATCH`.
- **Legacy 0.1.0** (`fixtures/product_preview/invented_mechanics_result.json`, accepted plain):
  - with a receipt or a null member, it is refused at raw, metadata and derive;
  - with a token, it is refused at raw and derive.
- **Binding:**
  - a refused statement refuses every row with `RULE_QUANTITY_NOT_COVERED` (F5);
  - a row with no id, or a row not in the envelope, gives `None`;
  - a relabelled successor gives `None` for every row, as any invalid envelope does at base. Dispatch and standing refuse it.

## 4. D-U6-1 (`rv88_py_lane.py`, `rv88_py_summary.py` → `py_lane_summary.txt`)

**The lane's oracles:**
- each mutation's own `expected` gate and code in the 07f corpus;
- the **base** lane's draft and public entry;
- an in-process flip of `_IMPLEMENTATION_COMPLETE`, which never writes a file.

`apply_entry` is borrowed from the test module only to apply corpus edits.

| Check (320 entries: 15 cases with the invocation, 15 without, 268 mutations, 22 must-pass) | Result |
|---|---|
| Candidate public entry equals candidate `_validate_draft` | 320/320 |
| Candidate draft equals base draft (the reader body is unchanged) | 320/320 |
| Base public entry refuses at G0 `SOURCE_PRODUCER_CONTRACT_UNSUPPORTED` (the removed short-circuit) | 320/320 |
| Mutations matching the corpus's expected gate and code | 268/268 |
| Must-pass entries and cases passing | 22/22 and 30/30 |
| Passes with `numerical_eligible` false and standing `needs_recompute` | 52/52 |
| Flag forced True: refusals unchanged | 268/268 |
| Flag forced True: passes unchanged except eligibility and standing | 52/52 |
| Flag forced True: passes that become eligible | 25, all invocation-bound; none without an invocation |
| Milestone receipts (public entry) | pass, not eligible, classes 25/69/3/1 and 25/69/3/2; base refuses at G0 |
| No production caller of `validate_retained_precision` (grep over P) | none, so the change alters no existing caller |

The flag stays `False` (retained_precision.py:30) and gates only line 1702.

## 5. F5 and F6

**F5: a refused statement refuses every row with `RULE_QUANTITY_NOT_COVERED` (semantic_contract.rs:553–556). RV88's answer: yes, keep it fail-closed.**
- `rule_binding_refusal` is the only per-row gate inside `run_rule_checks_core`; the core (src-tauri lib.rs:2843) binds with no standing check of its own; its own tests bind a producer-less envelope (lib.rs:8261–8285).
- In the desktop command, `qualify_rule_mechanics_with_context` (lib.rs:2759) refuses an `unsupported` source before the core runs (lib.rs:2779), so F5 is defence in depth there. Any other library caller relies on the helper alone.
- `None` would let a tampered successor's rows bind wherever standing is not consulted first.
- The code stretches D2's meaning a little. D2 §4.9.9 gives `RULE_QUANTITY_NOT_COVERED` for the `not_covered` class ("no accuracy claim for this kind"), but a refused statement has no verified class for any row. That is defensible, since no row has verified accuracy, and a new code would need a reservation. See N-1 for the parity consequence.

**F6: the class-disclosure message (derivative.rs:26–41) is truthful and claims nothing beyond the receipt's bound.**
- **The bound is the reader's G5c-recomputed bound.** G5c requires it to equal the receipt's listed bits. RV88 checked all 138 messages against the receipt's lists, independently of the reader.
- **The decimal is the shortest round-trip form,** and parses back to the same bits.
- **The text adds nothing beyond D2.** "Verified only to … below the relative accuracy floor" and "withheld from rule binding and reliance" are D2 §4.9.9's own terms (option A, before S-I).
- **It keeps RV86's limits.** It claims neither the stop-rule bound nor extrema enclosure.
- **Its one gap is S-2.**

## 6. Mutants (`rv88_mutants.py` → `mutants_i66.json`, `mutants_own.json`)

Each mutant edits one site in `WT/rv88/mut` and runs:
- I66's three test targets (`retained_precision_carriers`, `preview_physics_contract`, `derivative_contract`), or I66's pytest selection for the Python entry;
- RV88's `zz_rv88_refusals`, plus, for RV88's own mutants, `zz_rv88_slice` and the Python comparator.

**Kills are attributed by test name.** A compile error never counts as a kill. The unmutated copy passes everything (the control).

**I66's 62:** all 62 are killed by I66's suite (42 in semantic_contract.rs, 17 in derivative.rs, 3 in the Python entry), and none by a compile error. That matches I66's `mutants_final.json`. RV88's refusal-only test also kills 23 of the 59 Rust mutants, which is informational; it is not built to pin positive behaviour.

**RV88's own 7:**

| Mutant | Site | I66's suite | RV88's tests |
|---|---|---|---|
| R01: the row guard reads only the first row | semantic_contract.rs:163–173 | **survives** | killed (`!tokenlast`, one-token form) |
| R02: `validate_document` ignores a null member on a base document | derivative.rs:391 | **survives** | killed |
| R03: receipt equality by `receipt_sha256` only | derivative.rs:388 | killed | killed |
| R04: the decimal bound rounded to 4 digits | derivative.rs:31 | killed | killed (comparator, 138 rows) |
| R05: requested refs compared as a set, not in case order | semantic_contract.rs:597 | **survives** | **survives** (no multi-case statement) |
| R06: the member guard exempts legacy 0.1.0 sources | semantic_contract.rs:157 | **survives** | killed |
| R08: the message drops the binary64 bits | derivative.rs:31 | killed | killed |

## Also verified

Verified rather than defective. The headless runner's derivative-metadata check (runner lib.rs:617–621: `result_envelope` with `schema_version` 0.2.0, through `for_source_metadata`, plus the `semantic_contract_ref` binding) accepts the successor's derivative as the retained table at "0.3.0", with the reference bound. The headline rule also holds: `max_open_formula_stress` names an `absolute_verified` row and is refused `RULE_QUANTITY_BELOW_VERIFIED_FLOOR`, while `max_displacement` names a relative row and binds. `classification_summary` gives 25/69/0/0/3/1 with 97 withheld (sparse) and 25/69/0/0/3/2 with 97 withheld (dense), with or without the invocation. That equals the Python reader's independent class counts and D2 §4.9.9's withheld definition (not Current, so every quantity row).

## Findings

| ID | Severity | Where | Evidence | Remedy |
|---|---|---|---|---|
| **S-1** | SHOULD-FIX | P/core/reporting/result_export/tests/retained_precision_carriers.rs:392–424 (downgrade test) and :221–276 (seam test) | Four guard and standing forms are not pinned by I66's suite. The code is correct, but mutants R01 (row guard on the first row only), R02 (null member on a base derivative), R06 (legacy 0.1.0 source carrying a receipt) and R05 (requested refs order-insensitive) survive it. RV88's tests kill R01, R02 and R06; R05 survives both, because no statement has two cases. I66's "62/62 killed" is true of its own list, but the list lacks these forms. | Add to `retained_precision_carriers.rs`: a token on a non-first row only; a null member, `{}` and a string member on a base source and on a base derivative; a legacy 0.1.0 source with a receipt, a null member and a token; and a seam test of `retained_standing_from` with a two-case receipt and its refs reversed (`needs_recompute`). Copy the guard forms into `retained_precision_carrier_cases.json` so U6b and U6d inherit them. This can go in a small U6a repair, or before U6f. |
| **S-2** | SHOULD-FIX (F6 wording) | P/core/reporting/result_export/src/derivative.rs:31 | The message says the bound is "in the SI unit of this quantity" but does not name the unit, while the same disclosure's `source_unit` is the row's own unit. Per mode, 17 of the 69 absolute rows are in mm or MPa. Read in the row's unit, b would overstate the bound 10^6× for MPa rows but **understate it 10^3× for the 2 mm rows**. D2 §4.9.9's notice for the same bound names the unit ("±{b} {unit}"). The text is truthful as written, so this is not a correctness failure. | Print the canonical SI symbol of the row's dimension (m, rad, N, N·m, Pa), for example "b = 5.4215527659630466e-24 m (binary64 3b1a378ea78c5ce9)". `validate_document` recomputes the same text, so the derive and validate pair stays exact. Settle it before U6f; U6d's own notice texts are reviewed separately. |
| **N-1** | NOTE (F5, parity) | semantic_contract.rs:553–556 | F5 is correct fail-closed (§5). The Python and TS helpers do not exist yet, and `retained_precision_carrier_cases.json` has no binding scenarios, so nothing yet enforces the three languages' agreement on a refused statement. | **ROOT to rule:** U6b's `compatibility.rule_binding_refusal` and U6d's `ruleBindingRefusal` return `RULE_QUANTITY_NOT_COVERED` for every row of a refused successor statement, and the shared cases gain binding scenarios: a valid statement's absolute row, a relative row, a refused statement, and a relabelled envelope. |
| **N-2** | NOTE (F7, measured) | semantic_contract.rs:553; derivative.rs:94–95, 366–367 | Measured in a debug build: one full reader validation of the milestone takes 39 ms, and `derive_document` 292 ms (four validations). Binding every row through `rule_binding_refusal` takes 3.8 s for 98 rows, because each row revalidates the whole statement. Release is faster, but the cost is per row. | U6f: validate once per call, or cache by `receipt_sha256` and the source bytes, before any interactive binding path uses it. |
| **N-3** | NOTE | semantic_contract.rs:565–676; derivative.rs:25–26 | The `#[doc(hidden)]` seams (`class_binding_refusal`, `retained_standing_from`, `classification_summary_from`, `class_disclosure`) are `pub`. `retained_standing_from` returns `numerically_eligible` for a caller-built `Validation` with the flag-held field set. This follows the D14 precedent, but nothing stops a product caller from using it. | U6f: add a source-text guard test, like PP's `tests/retained_precision_admission.rs:217–222`, that no non-test product file calls the seams. |
| **N-4** | NOTE (F1 reproduced) | P/core/reporting/result_export/src/source_blocks.rs (outside the fence) | I66's F1 is reproduced independently. On `fixtures/product_preview/source_blocks/rejected_stress_range/dense_scrutiny.raw.json`, base run 1 and base run 2 of RV88's sweep, on identical code, report different first codes from `for_source` (`SOURCE_BLOCKS_STRESS_OUTPUT_RANGE` against `SOURCE_BLOCKS_SUMMARY_INPUT_RANGE`). Both runs refuse, and no other field of any of the 97 envelopes varies. | As ROOT ruled: a separate task for the source-blocks reader's owner. |
| **N-5** | NOTE | P/core/reporting/result_export/src/derivative.rs:388 and the pre-existing equality checks of the same function (`contract_evidence`, :396–403) | `validate_document` compares the copied members as `serde_json::Value`s, which distinguishes `0.0` from `0`. **This is pre-existing:** at base, a canonical round trip (`canonical_json`, then reparse) of any preview-physics-1 or physics-1 derivative fails `validate_document` against its own raw source with `SOURCE_CONTRACT_EVIDENCE_BINDING_MISMATCH`, because `contract_evidence` carries integral floats (`local_fraction: 0.0`). That covers 3/3 in both lanes, and the milestone too. The receipt check adds the same exposure: a hash-consistent receipt with `receipt_version: 1.0` passes the reader (D25/D32), derives and validates in memory, and after a canonical round trip is refused `RETAINED_PRECISION_RECEIPT_BINDING_MISMATCH`. PP emits integers, so the milestone's receipt is unaffected. This is fail-closed. | Pre-existing; not U6a's to fix. **For U6d and later:** any path that revalidates a *re-read* canonical document against its raw source (reopen, U7/U9) will be refused for preview-physics-1 and the successor alike. Compare normalized values (the reader's `integral_receipt` rule) or canonical bytes there, or state the limit. Route to T3 (the derivative owner) as a separate item. |

## For ROOT to rule

1. **S-1:** where the four test additions land (a small U6a repair by I66, or U6f), and that the guard forms go into the shared carrier-cases file for U6b and U6d.
2. **S-2:** whether the disclosure message names the SI unit. This is new product text.
3. **N-1:** make F5's refused-statement behaviour a parity obligation for U6b and U6d, and add binding scenarios to the shared cases.

## Records

Everything is in `_run_records/`, with placeholder paths only (WT, REPO_ROOT, P). **The RV88 tests and scripts:**
- `zz_rv88_sweep.rs`, `zz_rv88_slice.rs`, `zz_rv88_refusals.rs`, `zz_rv88_extra.rs` and `zz_rv88_rt.rs`;
- `rv88_sweep_compare.py`, `rv88_slice_compare.py`, `rv88_py_lane.py`, `rv88_py_summary.py` and `rv88_mutants.py`.

**The outputs:**
- the sweeps (`sweep_{base,cand}_{1,2}.tsv`) and `sweep_summary.txt`;
- `slice_{base,cand}.tsv` and `slice_compare.tsv`;
- `refusals.tsv` and `extra.tsv`;
- `py_lane_summary.txt`;
- `mutants_i66.json` and `mutants_own.json`;
- `suites.txt`, which holds the outcome lists and counts and the canonical round-trip probe (both lanes);
- `commands.txt`;
- `changed_files_sha256.txt`.

SHA256SUMS covers this folder.
