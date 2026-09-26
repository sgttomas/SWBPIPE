# T1_WP4_HEADLESS commands

These commands are run from the load-state worktree on `codex/piping-load-states-20260925`, with HEAD `6810b67ff` (its ancestor is the spawn base `fe3e2dbbc`). Paths are relative to WORKING_ROOT. `<shared-target>` is the shared cargo target named in the spawn request, `<venv>` is the given venv, and `<scratch>` is the TASK scratch space.

## Baseline (before any WP4 file existed)

```
cd core/runner/headless
CARGO_TARGET_DIR=<shared-target> CARGO_INCREMENTAL=0 cargo +1.97.1 test --locked --offline -j 2
```

Result: 69 passed (lib 44, headless_preview_runner 1, openpipestress-runner 18, physics_source_connected 1, preview_physics_admission 5). See `cargo_headless_baseline.log`.

## Final Rust run, with every artifact lane enabled

```
cd core/runner/headless
HEADLESS_LOAD_REFERENCE_OUTPUT_DIR=<scratch>/art/lr \
HEADLESS_LOAD_REFERENCE_SOURCE_OUTPUT_DIR=<scratch>/art/lrs \
HEADLESS_SOURCE_BLOCK_OUTPUT_DIR=<scratch>/art/sb HEADLESS_PHYSICS_OUTPUT_DIR=<scratch>/art/phys \
HEADLESS_PRECISION_OUTPUT_DIR=<scratch>/art/prec \
CARGO_TARGET_DIR=<shared-target> CARGO_INCREMENTAL=0 cargo +1.97.1 test --locked --offline -j 2
```

Result: 83 passed, 0 failed, 0 ignored (lib 51, headless_preview_runner 1, openpipestress-runner 18, load_reference_cli 7, physics_source_connected 1, preview_physics_admission 5). See `cargo_headless_final.log`.

Artifacts: 28 files for load-reference-1 (4 cases × request, invocation, raw, document, manifest, plus a CLI input and output for each) and 60 for load-reference-source-1 (10 cases × request, invocation, raw, manifest, plus a CLI input and output for each; no document).

## Recorded Python run, with the artifact and parity variables set

```
HEADLESS_LOAD_REFERENCE_OUTPUT_DIR=<scratch>/art/lr HEADLESS_LOAD_REFERENCE_SOURCE_OUTPUT_DIR=<scratch>/art/lrs \
LOAD_REFERENCE_PARITY_OUT=<scratch>/parity/lr LOAD_REFERENCE_SOURCE_PARITY_OUT=<scratch>/parity/lrs \
PYTHONDONTWRITEBYTECODE=1 <venv>/bin/python -m pytest -p no:cacheprovider -q -rs \
  tests/test_load_reference_headless_artifacts.py tests/test_load_reference_readers.py tests/test_load_reference_source_readers.py
```

Result: 407 passed, 0 skipped. The new file contributes 15; the two reader files contribute 392, including both `test_zzz_write_parity_log` lanes. See `pytest_final.log`.

## Mutants (scratch copy, never in place)

```
git archive HEAD projects/chirality-piping/{core,fixtures,schemas,validation} | tar -x -C <scratch>/mut
# then copy the three WP4 Rust files into the copy
python3 _run_records/mutants.py <scratch>/mut <scratch>/mut-target <scratch>/mutants.log
```

- The unmutated scratch copy passes.
- 17 of 17 mutants are killed. See `mutants.log`; the script is `mutants.py`.
- The mutants use a private target under scratch with debuginfo off (`CARGO_PROFILE_{DEV,TEST}_DEBUG=0`). It was deleted afterwards, together with the scratch copy. Mutant builds sit at another source path, so in the shared target they could not be pruned by owner while other TASKs build.
