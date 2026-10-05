# I66 return: U6a, the end-to-end carrier slice, with D-U6-1

I66 is a TASK (Type 2) under ROOT, working to `BRIEFS/I66_U6A_SLICE.md` and the plan `R/I66/u6_scoping_01/PLAN.md` (sha256 `8742d105…`), as ruled in RR "U6 plan accepted". It did not delegate.

**Verdict: the slice works end to end, and every control passes.**
- **The slice:** the milestone successor, in both modes, goes through the real Rust canonical derivative (`derive_document`, then `validate_document`) and comes back out.
  - The receipt that comes back out is byte-equal to the source's.
  - It revalidates in all three accepted readers.
- **Standing** stays `needs_recompute`, and every reader's eligibility stays false.
- **Existing identities:** each one is accepted, refused and classified exactly as at base, and `derive_document` emits identical bytes for them.
- **Mutants:** all 62 are killed, none by a compile error.
- **No ruling is needed to continue.** Findings F1–F7 are for ROOT.

## Basis, host and fence

- **Worktree:** `WT/f2a-carriers`, branch `codex/piping-f2a-carriers-20261004`, from NUM `7e4f5a51dd`. NUM has since moved to `7f88ac5f5c`, with records only. I made no Git writes; Git reads used `GIT_OPTIONAL_LOCKS=0`.
- **When:** 2026-10-04, about 07:13Z to 08:30Z, with the memory guard (PID 5387) running throughout.
- **Cargo:** the default toolchain, `--locked --offline`, `CARGO_BUILD_JOBS=4`, `RUST_TEST_THREADS=2`, one job at a time, each under a 1,200 s alarm.
- **Targets:** `WT/targets/i66-u6a/`.
- **Python:** `REPO_ROOT/projects/chirality-piping/.venv`, with the I52 checked-JSON and units CLIs set (`PYTHONDONTWRITEBYTECODE=1`).
- **TypeScript lane:** vitest in a scratch lane, with REPO_ROOT's P `node_modules` linked and READER's prebuilt `public/` WASM copied in.
- **Not run:** nothing native, at scale or DEC-025.
- **Scratch:** `WT/scratch/i66_u6a_slice_01/`. It holds the `base` archive of `7e4f5a51dd` and the `cand`, `mut` and `schema_lane` lanes, all disposable.
- **The fence held: 9 files, listed below.**
  - **The schema delta was not pulled in,** because the slice validates without it (F2).
  - **D-U6-9** (the legacy 0.1.0 wrapper) is Python, so it goes to U6b.
- **Fresh collision recheck:** every name this grant introduces is absent at NUM `7f88ac5f5c`, at `origin/main` `09106477e3` and at the facade head `8abb5274a9`.

## Changed files (`_run_records/changed_files_sha256.txt`; diff: `_run_records/candidate_tracked.diff`)

| File | sha256 | Change |
|---|---|---|
| P/core/reporting/result_export/src/semantic_contract.rs | `1a52afec…` | The successor dispatch, raw and transport, through the accepted reader only. Also: the downgrade guards (F-5), the fresh set (D-U6-6), standing from the verified receipt, the class binding refusal, `classification_summary`, and the pinned retained table. |
| P/core/reporting/result_export/src/derivative.rs | `8694386e…` | The receipt and `contract_evidence` copy (D2 §4.9.7), the class disclosures (D-U6-2 (A)), and validator equality, override and consistency checks. |
| P/core/reporting/result_export/tests/retained_precision_carriers.rs | `20fadd30…` | New: 10 tests. |
| P/core/reporting/result_export/tests/preview_physics_contract.rs | `6dbc9e14…` | The exact fresh-set pin gains the successor (D-U6-6). It is still an exact equality. |
| P/core/analysis_runs/retained_precision.py | `d01abdb0…` | D-U6-1: the public entry runs every gate, and `_IMPLEMENTATION_COMPLETE` gates eligibility only. |
| P/tests/test_retained_precision_contract.py | `eddbddea…` | D-U6-1 tests, described below. |
| P/fixtures/results/retained_precision_milestone_successor_sparse_interactive.json | `ac6986b0…` | D-U6-5: byte-identical to PP's pin. |
| P/fixtures/results/retained_precision_milestone_successor_dense_scrutiny.json | `6cd1d249…` | D-U6-5: byte-identical to PP's pin. |
| P/fixtures/results/retained_precision_carrier_cases.json | `952e39bf…` | The shared 14-case standing-parity file named in plan §6, checkpoint A, at a reserved path (D-U6-4). Rust consumes it now; U6b and U6d consume it later. |

### How the change works

- **Dispatch.**
  - `for_source` sends the successor to `retained_precision::validate(source, None)`, which covers G0–G7.
  - `for_source_metadata` sends it to `validate_transport_metadata`.
  - The reader's G7 calls the unchanged base branch on its own projection, which has no receipt and no token, so nothing re-enters.
  - A G7 failure keeps the base validator's own text.
- **Standing.**
  - It is `retained_precision::validate(source, invocation)`, then `retained_standing_from`.
  - That requires an invocation-bound validation with eligibility set, the requested refs equal to the receipt's case order, `MECHANICS_SOLVED`, and every case either `selected` or `not_required`. A `not_required` case must also be ordinarily eligible (F-7).
  - `numerical_quality` never contributes.
  - Because the reader flags are held, the result is always `needs_recompute`.
- **Binding.**
  - Each row's refusal follows its validated class: `absolute_verified` gives `RULE_QUANTITY_BELOW_VERIFIED_FLOOR`, and `not_covered` gives `RULE_QUANTITY_NOT_COVERED`.
  - A headline is refused as the row it names.
  - If the reader refuses the statement, every row is refused (F5).
- **The derivative.**
  - The receipt travels whole.
  - Each `absolute_verified` or `not_covered` row becomes `disclosed`, with `retained_precision_absolute_verified` or `retained_precision_not_covered`. Its message names the class and the receipt's published bound b, printed both in decimal and as 16-hex binary64 bits.
  - It claims neither a stop-rule bound nor extrema enclosure (the RV86 limits).
  - `validate_document` enforces receipt equality, the class dispositions and an exact code and message, and it refuses a class code on any other disclosure.
  - A receipt on any other identity is refused, `RETAINED_PRECISION_DOWNGRADE_FORBIDDEN`.
- **Test seams.** `retained_standing_from`, `classification_summary_from`, `class_binding_refusal` and `class_disclosure` are `#[doc(hidden)]` seams (the D14 precedent). They let the post-U7 rules be tested now without touching any flag.

## Controls

**1. Existing identities: unchanged.** (`zz_i66_sweep.rs`; `sweep_*.tsv`; `sweep_compare.txt`)
- **The sweep:** it walked every mechanics envelope in P/fixtures, result_export tests/fixtures, PP tests/fixtures and the runner tests.
- **What it compared:** `for_source`, `for_source_metadata`, `standing_reason`, `numerical_use_standing`, `is_fresh_identity`, each row's binding refusal, and `derive_document` output sha256 followed by `validate_document`. It ran in base and candidate lanes.
- **The result:** all 63 existing-identity envelopes match a base run.
  - 62 are identical to base run 1.
  - The other one is a fixture whose first failure code varies **between base runs themselves** (F1).
- **The successor-identity envelopes:** 15 corpus cases, plus the 2 new fixtures. All 17 now dispatch, derive and validate; at base they were refused.
- **Suites:**

| Suite | Base | Candidate |
|---|---|---|
| result_export | 149 passed | 159 passed: the existing 149 with identical outcomes, plus 10 new |
| runner/headless | — | 85 passed, 2 failed: identical to I61's U3 grant-1 record (the two known Mac load_reference failures) |
| PP pin tests (`retained_wire_tests`, `retained_facade_tests`), built against the candidate result_export | — | 35/35, including `u1_milestone_successor_both_modes` and `u3_permitted_path_publishes_the_pinned_successor` |

**2. The slice, in both modes.**
- **Rust** (`u6a_derivative_carries_the_receipt_and_it_comes_back_out`):
  - `derive_document`, then `validate_document`, passes;
  - the receipt is byte-equal;
  - reattached to the source, it revalidates with the invocation to an identical Validation;
  - the derivative's metadata view passes transport, with the same `publication_sha256`;
  - every row's disposition equals the projected base document's, except that the 69 `absolute_verified` rows are disclosed with their bound.
- **Python** (`lane_py_backout.log`) and **TypeScript** (`lane_ts_backout.results`, TS raw and transport): the carried receipt is equal and revalidates. Classes are 98 and 99, 69 absolute rows are disclosed, and eligibility is false.
- **Standing** is `needs_recompute`, with and without the invocation.
- **The shared cases:** all 14 pass in Rust.

**3. Downgrade and binding refusals.**
- **The downgrade guard:** a successor relabelled as `preview-physics-1` is refused `RETAINED_PRECISION_DOWNGRADE_FORBIDDEN`, at the raw, metadata, standing and derive stages. That holds whether it keeps the receipt or only keeps the token rows; a null member is refused too.
- **Base derivatives:** a base derivative given a receipt, before or after derivation, is refused with the same code.
- **Receipt edits:**
  - a dropped or altered derivative receipt → `RETAINED_PRECISION_RECEIPT_BINDING_MISMATCH`;
  - a class code or message changed, or a class code claimed by another disclosure → `DISCLOSURE_SEMANTICS_MISMATCH`;
  - a mutated source receipt → the reader's own code (`RETAINED_PRECISION_RECEIPT_MISMATCH`);
  - an edited covered row, a foreign mode, or a hash-consistent quality claim of `checks_passed` → standing `unsupported`.

**4. Nothing weakened.** The diff removes no check. Its removed lines are all rewritten lines, plus three deliberate changes:
- **The Python public entry's G0 short-circuit,** removed by the D-U6-1 ruling.
  - The test asserting it was replaced: the public entry now equals the draft, with eligibility false.
  - A new parity test covers all 320 entries of the shared corpus (07f, a superset of 07e: 15 cases with and without the invocation, 268 mutations and 22 must-pass entries). Every outcome is identical to `_validate_draft`, and every pass has eligibility false and standing `needs_recompute`.
  - Both real milestone receipts give classes 25/69/3/1 and 25/69/3/2. A covered-row edit is refused at G1.
- **The fresh-set pin,** widened only by the ruled member and still exact.
- **The derive-side receipt downgrade check,** removed in my own revision. It was redundant with `validate_document`, which ends `derive_document` and refuses the same input with the same code; leaving it would have been an equivalent mutant.

**No reader file changed, apart from D-U6-1's Python public entry, and no eligibility flag changed.** Python went from 371 to 374 passed; the 3 added tests all pass.

**5. Mutants: all 62 are killed, none by a compile error** (`mutants.py`, `mutants_final.json`, `mutants_run.log`). There are 42 in semantic_contract.rs, 17 in derivative.rs and 3 in the Python entry.
- **The first round left survivors,** each closed before the final run:
  - **S09:** strengthened with a detail-bearing G7 failure.
  - **S29:** the case-status rule was rewritten as an explicit match; unavailable and unknown statuses are now refused.
  - **S34:** a fabricated `not_covered` class in the post-U7 summary test.
  - **S40:** a same-JSON, other-bytes table.
  - **D10:** a leading-zero bound.
  - **D04:** the redundant code was removed.
- **The final run on the frozen candidate** kills every mutant (`mutants_run_part1/2.log` are the earlier rounds).

## Findings

- **F1 (pre-existing, outside the fence).** The source-blocks validator's first failure is not deterministic.
  - On `fixtures/product_preview/source_blocks/rejected_stress_range/{dense_scrutiny,sparse_interactive}.raw.json`, `for_source` reports `SOURCE_BLOCKS_STRESS_OUTPUT_RANGE` in one run and `SOURCE_BLOCKS_SUMMARY_INPUT_RANGE` in another. This happens at base itself (3 base runs differ), apparently from hash-map iteration order.
  - It refuses either way. But it undermines the first-failure parity rule and any byte-for-byte sweep.
  - **Proposed:** route it to the source-blocks reader's owner (T3 or T6).
- **F2 (D-U6-2's schema half).** The committed `results.v0.3` schema refuses the slice's derivatives **only** because of the two new reason codes (`lane_schema.log`).
  - A lane copy that adds them to `RowDisclosure.reason_code` accepts both derivatives.
  - The same lane copy refuses a derivative whose receipt was dropped (the successor branch).
  - **Not pulled into this grant:** the slice validates through `validate_document`, and no maintained path schema-validates a successor derivative today. Headless export is gated on eligible standing, and desktop export is refused.
  - **U6c must land before any schema-validated successor export.**
- **F3. No validated statement can reach a `not_covered` row today.** The milestone and all 15 corpus cases have none.
  - Those paths are tested through the class seams, a claim test and a fabricated summary class.
  - The `not_covered` disposition in `derive_document` is therefore unexercised end to end.
- **F4. New names for D-U6-4's recheck.** All are absent at the three trees.
  - **Two table-verification codes:** `SOURCE_PREVIEW_PHYSICS_RETAINED_TABLE_HASH` and `_IDENTITY`.
    - They follow the load-reference table pattern.
    - With the committed bytes they are unreachable: the pinned-table init would panic on `expect`, like the other pinned tables.
    - They are not in D-U6-4's list. **Please reserve them, or rule another spelling.**
  - **Other new Rust names:** the constants `PREVIEW_PHYSICS_RETAINED_ID/PROFILE/SHA256`, `RETAINED_ABSOLUTE_VERIFIED` and `RETAINED_NOT_COVERED`, and the hidden seams.
  - **A test-only output hook:** `I66_U6A_OUT`.
- **F5 (a reading for ROOT).** When the reader refuses a successor, `rule_binding_refusal` refuses every row with `RULE_QUANTITY_NOT_COVERED`.
  - This fails closed, and such a statement is already `unsupported` at standing.
  - The alternative is `None`, leaving the refusal to standing only.
- **F6 (wording for review).** The derivative's class-disclosure message is new product text.
  - It is not D2's UI notice text `N_RP_*`. It names only the receipt's published absolute bound.
- **F7 (cost, for U6f).** A successor is validated about twice per carrier call, and about four times for `derive_document` plus `validate_document`.
  - The tests are quick: 10 tests in about 12 s in debug.
  - Caching by receipt sha256 is possible later. It is not needed now.

**Not yet covered, and by design:**
- **Python and TS dispatch guards and carriers.** These are U6b and U6d. Until they land, the Python and TS base dispatch still read a relabelled successor as `preview-physics-1`, with ordinary standing. That is `needs_recompute` for a selected case.

## Records and next

- **Records:** `_run_records/` holds the scripts, lane tests, sweeps, outcomes, mutants, logs and the changed-file hashes, all with placeholder paths. SHA256SUMS covers this folder.
- **Next steps:**
  - ROOT verifies and commits on `codex/piping-f2a-carriers-20261004`.
  - Then U6b–U6e fan out as planned. U6c should take F2's `RowDisclosure` delta, scoped to the successor branch.
