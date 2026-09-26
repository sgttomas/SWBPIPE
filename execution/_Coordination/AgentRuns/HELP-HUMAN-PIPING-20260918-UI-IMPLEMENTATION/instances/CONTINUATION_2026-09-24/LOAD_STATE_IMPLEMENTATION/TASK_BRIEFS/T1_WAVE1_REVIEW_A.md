# TASK brief — T1_WAVE1_REVIEW_A (independent review: joined readers, schemas and packager)

Read `_T1_COMMON.md` first. Return folder: `LSI/T1_WAVE1_REVIEW_A/`.

You are a **fresh-context, non-author reviewer**. You wrote none of the reviewed bytes, and you delegate nothing. This review is the D1 gate: `load-reference-source-1` is activated in the T1 PR only once its joined readers pass independent review.

## Candidate

The candidate is the branch `codex/piping-load-states-20260925` at `31dc7ce08`. Review commit `bfef71b19` (its parent is `c1e130818`), together with the reader re-pin in `31dc7ce08` (`tools/validation/qualification_load_reference.py` `MODULE_SHA256` only).

## Basis

- The joined wire: `LSI/CP2_WIRE_ADDENDUM_2.md` §5 and `CP4_WIRE_ADDENDUM.md` (with `CP4_REVIEW_DISPOSITION.md`).
- The rulings: `LSI/T1_WAVE1_RULINGS.md` §1, §2, §4, §6, §7, §8.
- The returns: `LSI/T1_WP1_JOINED_READERS/RETURN.md` and `LSI/T1_WP1_JOINED_SCHEMAS/RETURN.md`.
- ROOT's constraints:
  - no change to any Current, rule or export standing function except the one declared early `needs_recompute`;
  - no change to the source-blocks readers;
  - pre-existing outcomes unchanged, apart from the one ruled case expectation.

## Review

1. **Joined readers** (Rust `core/reporting/result_export/src/load_reference_source.rs`, Python `core/analysis_runs/load_reference_source.py`, and the dispatch/derivative/compatibility changes).
   - Can either reader accept an envelope the producer could not have emitted under the joined contract, or accept one whose receipt, physical-evidence hash or per-case records do not bind?
   - Do the two languages agree in order and code?
   - Verify the physical-evidence hash recomputation independently against the producer (`core/product_physics/src/source_receipt/composite.rs`), on at least two committed raws.
   - Probe relabels in all directions, and probe tampering after a `reseal`.
2. **The standing edit.** Check that exactly one early return was added in each language, placed after validation. Check that no other standing, Current, rule or export behaviour changed: diff the functions, and rerun the pre-existing output comparisons.
3. **The pre-existing expectation change** (`TABLE-id-reserved-source-successor`). Is it exactly the D1 consequence, and nothing else changed?
4. **Schemas.**
   - Old documents keep their branches, and the only non-append change is the ruled `anyOf` wrap.
   - The joined branches are tight: pins, the required receipt and exact keys.
   - The cross-carrier `$ref` resolves.
   - The two minimal test-pin edits weaken nothing.
5. **Packager edit and packages.** `core/handoff/stress_neutral/package_v0_3.py` must add the joined ID to exactly `SUPPORTED`, `PHYSICAL` and `RECEIPT_METHODS`. The ten `fixtures/results/load_reference_source_*.stress_neutral.json` packages must reproduce with `LSI/_run_records/session4/t1_joined_stress_neutral_outputs.py <WORKING_ROOT> --check`.
6. **Carriers.** The 20 documents and AnalysisRun records reproduce, and they validate on the joined branch.

## Checks you may run

- result_export `cargo test`: expect 69.
- The Python suites in `LSI/_run_records/session4/t1_wp1_integration_python.log`: expect 1521 passed, 16 skipped.
- Your own probes and mutants, on a scratch copy.

Use your own `CARGO_TARGET_DIR` and delete it afterwards. Set `OPENPIPESTRESS_CHECKED_JSON_BIN` and `OPENPIPESTRESS_UNITS_BIN` to the helpers already built under `core/serialization/canonical_json/target` and `core/units/target`.

## Return

`LSI/T1_WAVE1_REVIEW_A/RETURN.md`:

- the verdict: CLEAR, FINDINGS or BLOCKING;
- a findings table: severity, location, concrete failure scenario, suggested repair;
- what you verified and how;
- limits.

Then `SendMessage` the manager.
