# TASK brief — CP4_READERS (checkpoint-3 review N-2 and N-3)

- **Role:** TASK (Type 2). You do not delegate.
- **Manager:** the T1 WORKING_ITEMS manager (load/reference states) that requested this TASK. Report to it by `SendMessage`, never only as final text.
- **Checkout:** the load-state worktree (location given in the spawn request), branch `codex/piping-load-states-20260925`. Start from the manager's commit `13f752115` or later. Paths below are relative to WORKING_ROOT = `projects/chirality-piping/`. `LSI` = `execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/LOAD_STATE_IMPLEMENTATION`.
- **No Git writes.** Do not stage, commit, push or change branches. The manager integrates and commits.

## Basis (read first)

- `LSI/REVIEW_CHECKPOINT_3/RETURN.md`, findings N-2 and N-3.
- `LSI/REVIEW_CHECKPOINT_3/ROOT_DISPOSITION.md`, items N-2 and N-3 (these are ROOT's selections; implement them exactly).
- `LSI/CP3_WIRE_ADDENDUM.md` §1 (segment vocabulary).
- `LSI/CP3_READERS/RETURN.md` for how the readers and the shared case file were built.

## Assignment

Both readers of `load-reference-1` evidence must change identically:

- Rust: `core/reporting/result_export/src/load_reference.rs`
- Python: `core/analysis_runs/load_reference_evidence.py`

**N-2 (numbers).** The Python reader must accept any finite JSON integer literal as a number, matching Rust (`serde_json` `as_f64`, which falls back to f64 for integers outside i64/u64). Keep the `U64_MAX` bound only for indices (`_index`). The numeric value used downstream must be the same binary64 value Rust obtains (so convert integers with `float(value)` where Rust would use `as_f64`), and later equality checks must compare the same values. An integer whose conversion to binary64 overflows must be refused in Python; state what Rust does with such a literal (it is refused at the text boundary) and record the case as language-specific in the same way non-finite numbers already are, if it cannot be expressed as a shared JSON case.

**N-3 (law segments).** Tighten both readers to `CP3_WIRE_ADDENDUM.md` §1 for every entry of `consumed_law_segments` and `consulted_law_segments`:

- `upper_index == lower_index + 1`;
- `use: integration_interval` requires `start_k < end_k`; `use: interpolation_sample` keeps `start_k == end_k`;
- no duplicate entry within one list (equal on all five keys). Consumed and consulted lists may share entries (addendum §1 "Overlap"); do not refuse that.

Do not add any other tightening (for example the `[T_lower, T_upper]` bound) without asking the manager first. Use the existing error code family; if a new code is needed, use the same code in both languages and name it in your return.

**Shared mutation cases.** Add cases to the shared file `core/reporting/result_export/tests/fixtures/load_reference_mutations.json` so both suites run them:

- N-2: a large positive and a large negative integer literal (for example `100000000000000000000` and `-100000000000000000000`) in a temperature field, accepted by both; the equivalent `1e+20` written form is also accepted by both. Use the fixture that the reviewer named (`pressure-dense`) or another that carries such fields.
- N-3: non-adjacent indices; `integration_interval` with `start_k == end_k`; `integration_interval` with `start_k > end_k`; a duplicate entry within one list; a permitted consumed/consulted overlap (accepted). Each refused case must be refused by both readers, at dispatch and in the validator, with the same code.

If the shared JSON format cannot express integer literals of that size (the file is parsed by both languages), say so and add the smallest per-language test that pins the behaviour, keeping the cases as parallel as possible. The existing accepted fixtures must still be accepted by both readers; if one is now refused, stop and report it to the manager before changing anything else. Never change a producer fixture, raw envelope, schema or semantic table.

## Write boundary (exact)

- `core/reporting/result_export/src/load_reference.rs`
- `core/analysis_runs/load_reference_evidence.py`
- `core/reporting/result_export/tests/fixtures/load_reference_mutations.json`
- `core/reporting/result_export/tests/load_reference_contract.rs` (only if a Rust-only test is needed)
- `tests/test_load_reference_readers.py` (only if a Python-only test is needed, or to update a hash pin of the case file; say so if you do)
- your return folder `LSI/CP4_READERS/` (RETURN.md, and machine logs under `LSI/CP4_READERS/_run_records/`).

Nothing else. Other writers are concurrently editing `core/product_physics/**`; do not touch it.

## Checks to run

Use `CARGO_TARGET_DIR=<scratch>/ls-target-readers` (a directory outside the worktree) (create it; delete it when done) and `-j 2`. Keep runs targeted. The Python venv is `the session DEC-025 venv (location given in the spawn request)`.

1. `cargo +1.97.1 test --locked --offline -j 2` in `core/reporting/result_export` — all pass (baseline 64).
2. `rustfmt +stable --edition 2021 --check` on each Rust file you touch — clean.
3. `python -m pytest -q -p no:cacheprovider tests/test_load_reference_readers.py tests/test_load_reference_schema.py` with `PYTHONDONTWRITEBYTECODE=1` — all pass (baseline 212 passed + 1 skipped, and 527).
4. Mutation evidence on a scratch copy (never in place): revert each new check individually and show at least one maintained test fails in each language. Restore and sha256-verify.
5. `git status --short` shows only files inside your write boundary.

## Return

`LSI/CP4_READERS/RETURN.md`, then a `SendMessage` to the manager with a short summary and that path. Include:

- the files changed with sha256 before and after;
- each rule implemented, with the code emitted;
- the cases added (IDs) and the accept/reject outcome in both languages;
- check commands and counts, and the mutation results;
- anything you could not do, or any design question.

Keep absolute machine paths (home or temporary directories) out of RETURN.md; put them only under `_run_records/`, and prefer relative paths there too.
