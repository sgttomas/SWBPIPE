# I76: the T6 slice's schema, Python and Rust test work (T6S-1, T6S-2)

TASK (Type 2). Read `BRIEFS/T6S_COMMON.md` first; it binds you. Your plan sections are I74's PLAN §1.4, §1.5, §3 (CQ-7, CQ-9) and §4 (T6S-1, T6S-2), as ruled (decisions 7 and 9).

## Order

1. **T6S-2 first:** the Rust goldens. I75's T6S-4 waits for their hashes.
2. **Return a short checkpoint** at `R/I76/t6s_01/CHECKPOINT_1.md`, with SHA256SUMS, as soon as the goldens and their hashes exist. End your turn there; ROOT relays the hashes to I75 and continues you.
3. **T6S-1** and the N-5 test.
4. The final RETURN.

## T6S-2: the Rust goldens

- **A new test, `RE/tests/retained_precision_derivative_golden.rs`.** It derives the successor result document from both pinned successors (`P/fixtures/results/retained_precision_milestone_successor_{sparse_interactive,dense_scrutiny}.json`) through Rust `derive_document`, with one fixed desktop-shaped base and origin.
- **Record that base and origin** exactly in the test and in RETURN, so I75 can reproduce them in TypeScript.
- **The test writes the goldens** under an output variable (as `I61_U3G2_OUT` does in PP) and otherwise compares the live derivative with the committed files by sha256:
  - `P/fixtures/results/retained_precision_successor_derivative_sparse_interactive.json`;
  - `P/fixtures/results/retained_precision_successor_derivative_dense_scrutiny.json`.
- **Record the regeneration command.**
- **Controls:** the golden contains the receipt whole, `contract_evidence`, and one disclosure per `absolute_verified` or `not_covered` row with D-U6-2's code and message. A mutated pinned successor changes the golden's hash.

## T6S-1: the dispatcher (CQ-7 B)

- **In `P/schemas/results.schema.yaml`,** replace the inline v0.3 branch (`oneOf[2]`) with a `$ref` to `results.v0.3.schema.yaml`, as the stress-neutral and AnalysisRun dispatchers already do. Update the description. **Leave `$defs`** (the v0.1 projection) unchanged.
- **Move dispatcher validation to the registry helper** (`P/tests/schema_validation.py`). In `P/tests/test_result_export_v0_2.py`, change validator construction only.
- **A new `P/tests/test_results_dispatcher_v0_3.py`:**
  - **Equivalence:** over every committed v0.3 document (the U6c 24-file sweep's documents, RR:9775) and T6S-2's goldens, the dispatcher and the version file give the same verdict.
  - **Coverage:** the dispatcher admits preview-physics-1, physics-1 and the successor at 0.3.0.
  - **Refusals:** the existing mixed-version refusals still hold.
  - **Mutants:** the `$ref` pointed at the wrong file; a dropped `oneOf` arm. Each is killed by an assertion.

## RV95 N-5: the public-API masking test (decision 9)

**In `RE/tests/source_blocks.rs`:**
- a receipt integer of 2^53 fails at `RECEIPT_SHAPE`;
- a `summary` count of 2^53 fails at the checked-profile hash refusal;
- neither reaches `SOURCE_BLOCKS_INTEGER`.

In RETURN, record that RV95's mutant S1 is equivalent at the public API. The direct `#[cfg(test)]` unit test in `RE/src/source_blocks.rs` is **not** yours: it goes with PR-B1.

## Suites

Run each on base `c1bfc460fc` and the candidate, and compare them test by test:
- `result_export` (`cargo test`);
- the Python sweep (`VENV/bin/python -m pytest` over `P/tests`, as the project runs it).

Explain every count change by an added test.

## Records

`NUM/R/I76/t6s_01/`: CHECKPOINT_1.md, then RETURN.md (changed files and hashes, the goldens' hashes, the base and origin, controls, mutants, suites, anything to rule on) and SHA256SUMS.

## Budget

6–11 h. Return at the checkpoint and at the end.
