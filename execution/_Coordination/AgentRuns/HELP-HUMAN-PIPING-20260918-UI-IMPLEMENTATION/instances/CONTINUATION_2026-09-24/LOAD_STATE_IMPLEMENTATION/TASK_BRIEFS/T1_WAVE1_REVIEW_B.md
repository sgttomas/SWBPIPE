# TASK brief — T1_WAVE1_REVIEW_B (independent review: operations, harness adapter and VP-STATIC admission)

Read `_T1_COMMON.md` first. Return folder: `LSI/T1_WAVE1_REVIEW_B/`.

You are a **fresh-context, non-author reviewer**. You wrote none of the reviewed bytes, and you delegate nothing.

## Candidate

The branch `codex/piping-load-states-20260925` at `31dc7ce08`. Review:

- `5ced47dec`: WP3 typed operations in `core/model_operations/operation_applier`;
- `31dc7ce08`: the WP5 adapter, `tools/validation/qualification_load_reference{,_helper}.py` and `tests/test_qualification_load_reference.py`;
- `6824b6b6b` and `c1e130818`: the manager's application of the WP6 freeze changes, the generator's `ADMISSION.json` support, and the admission itself;
- the WP6 package (`91ec30630`), only as far as the admission mechanics touch it. Its content was already independently frozen by `LSI/T1_WP6_FREEZE/RETURN.md`; do not redo that freeze.

## Basis

- Returns: `LSI/T1_WP3_OPERATIONS/RETURN.md` (including Addendum 1), `LSI/T1_WP5_HARNESS_ADAPTER/RETURN.md`, `LSI/T1_WP6_STATIC_CASES/RETURN.md`, `LSI/T1_WP6_FREEZE/RETURN.md`.
- Rulings: `LSI/T1_WAVE1_RULINGS.md` §3, §5, §6.
- Design: `validation/qualification/HARNESS_CUT.md` and `GATE_USAGE.md`.

## Review

1. **Operations.**
   - Does each operation refuse exactly what the product's typed boundary refuses, and also the reference, duplicate and inbound-orphan cases?
   - Can any operation change `schema_version`, or apply to a non-0.4.0 model?
   - Is Undo/Redo byte-exact, including an absent key?
   - Are pre-0.4 and 0.3.0 delete outcomes really unchanged? Check the golden control file's provenance: it was generated from the base revision.
   - Are the closed-form solve checks independent of the producer?
2. **Adapter.**
   - Does it keep HARNESS_CUT's rules: the ledger first, exactly-once selectors, missing never zero, refusals, custody, standing reported separately and a failed sensitive case?
   - Can a required assertion silently drop out of the denominator?
   - Is the negative-uniqueness rule (assertion id, plus unique (quantity, wrong value)) sound?
   - Is the reader re-pin in `31dc7ce08` correct, and the start-bytes test honest?
3. **Admission.**
   - Is the generator's `ADMISSION.json` path sound? It verifies the named review's sha256 and changes only `readiness`, `independent_review_ref`, `profile_status` and scope text; no values or rules.
   - Do the admitted files differ from the frozen candidates only in those fields? Diff the reference and criteria files across `6824b6b6b..c1e130818`.
   - Were R1–R3 applied exactly as the freeze specified? Check the hashes in the freeze RETURN §3.
   - Is the manifest re-hash correct?
   - The authoring checker's 28 "pending" invariants now fail by design after admission. Confirm that this is the only effect.

## Checks you may run

- operation_applier `cargo test`: expect 187.
- The adapter suite: `python -m unittest tests.test_qualification_load_reference`, expect 43 OK.
- `test_qualification_gate` and `test_qualification_physics*`.
- The generator `--check`.
- The WP6 `check_package.py` in no-producer mode: expect 4461 passed, and the 28 pending-invariant failures.
- Your own probes and mutants, on a scratch copy.

Use your own `CARGO_TARGET_DIR` and delete it afterwards. Do not run the product comparisons; the manager runs them after this review.

## Return

`LSI/T1_WAVE1_REVIEW_B/RETURN.md`:

- the verdict: CLEAR, FINDINGS or BLOCKING;
- a findings table: severity, location, concrete failure scenario, suggested repair;
- what you verified and how;
- limits.

Then `SendMessage` the manager.
