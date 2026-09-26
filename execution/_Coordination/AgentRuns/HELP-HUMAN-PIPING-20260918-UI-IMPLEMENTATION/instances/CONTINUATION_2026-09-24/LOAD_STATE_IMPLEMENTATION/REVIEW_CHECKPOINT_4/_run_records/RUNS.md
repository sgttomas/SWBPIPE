# CP4 review — run index

Placeholders: `<checkout>` the load-state worktree, `<scratch>` a `git archive 14a74b793` copy of
`core/`, `fixtures/`, `schemas/`, `tests/`, `examples/`, `validation/`, `tools/` and
`requirements-dev.txt` (one copy, deleted at the end), `<target>` the cargo target (deleted at the
end), `<venv>` the session DEC-025 venv. Toolchain `cargo +1.97.1 --locked --offline -j 2`;
rustfmt 1.8.0-stable. The checkout was never mutated; all probes and mutants ran in `<scratch>`.

| Record | Command (cwd) | Result |
|---|---|---|
| `start_state.txt`, `reviewed_files.txt`, `prehash_worktree.txt` | `git rev-parse HEAD`; sha256 of the 80 files in `a68326039..14a74b793` (`<checkout>`) | HEAD `14a74b793` |
| `posthash_worktree.txt` | same, at the end | equal to prehash |
| `product_physics_all_tests.log` | `cargo test` (`<scratch>/core/product_physics`) | 415 passed, 0 failed, 1 ignored (lib 324) |
| `result_export_tests.log` | `cargo test` (`<scratch>/core/reporting/result_export`, target `<target>/re`) | 64/64 |
| `pytest_named.log` | `<venv>/bin/python -m pytest -p no:cacheprovider -q tests/test_load_reference_readers.py tests/test_load_reference_schema.py tests/test_stress_neutral_export_package.py tests/test_stress_neutral_physics_source.py tests/test_stress_neutral_precision.py`, `PYTHONDONTWRITEBYTECODE=1` (`<scratch>` WORKING_ROOT) | 957 passed, 1 skipped |
| `regen_compare.log` | `cargo build --examples`, then the committed `_run_records/session3/cp4_regen_compare.py <target>/debug/examples` (`<scratch>` WORKING_ROOT) | 34/34, identical to the committed log |
| `pre04_differential.py`, `pre04_differential_{base_a68326039,candidate_14a74b793}.json` | the example `physics_source_connected` over every committed request in `fixtures/product_preview/{physics_source,source_blocks,load_reference,load_reference_source}` plus invented pre-0.4 stress variants (1–3 extra 1e-6 tip loads; P12), both modes; once with the candidate producer, once with `a68326039`'s `core/product_physics/src` (touched to force a rebuild; the base binary lacks the string "invocation join withheld") | 120 runs, 106 pre-0.4; 0 differ (stdout sha256, exit and stderr) |
| `review4_probes.rs.txt`, `probes_public.log`, `probes_replay_measure.log` | scratch-only module `source_receipt::review4_probes` declared in scratch `source_receipt.rs`; `cargo test --lib review4_probes -- --nocapture` | see RETURN §2 |
| `probes_support_norm_injection.log` | as above, with a scratch fault injection at the top of `composite_support_norms` (`if is_load_state && id == "stop" { return Err(..) }`), then restored and sha256-verified | falls back to ordinary publication |
| `mutations.py`, `mutations.log` | `python3 mutations.py <REVIEW_CHECKPOINT_3 review3_mutations.py> <scratch>/core/product_physics <target>` | K8, K9 and SF1-M1..M6 killed; R1..R7 as tabulated |
| `reader_boundary_cases.py`, `reader_boundary_cases.log` | 8 reviewer cases appended to the SCRATCH shared case file; Rust `cargo test --test load_reference_contract`, Python `pytest tests/test_load_reference_readers.py -k "R4 or nonfinite"`; a control with one deliberately wrong expectation fails in both; the scratch file was restored and compared byte-equal to `14a74b793` | pass in both; control fails in both |
| `rustfmt_check.log` | `rustfmt +stable --edition 2021 --check <file>` on the checkout (read-only) | 0 hunks on every new/SF-1 file; `lib.rs` own hunks 47 → 48 |
