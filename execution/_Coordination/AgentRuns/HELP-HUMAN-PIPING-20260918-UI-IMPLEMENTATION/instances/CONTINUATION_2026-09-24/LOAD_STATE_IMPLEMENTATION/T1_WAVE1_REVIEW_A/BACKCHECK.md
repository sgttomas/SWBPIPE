# T1_WAVE1_REVIEW_A backcheck: the WP1 follow-up to notes N-1 to N-3

- **Reviewer.** The same fresh-context, non-author TASK as `RETURN.md`. I wrote none of the reviewed bytes and delegated nothing.
- **Requested by.** The T1 manager (`a56f5deb01ca844b0`), by message.
- **Paths.** WORKING_ROOT-relative. `LSI` is this folder's parent, and `BR` is `LSI/T1_WAVE1_REVIEW_A/_run_records/backcheck`.
- **Writes and Git.** I wrote only in this folder and made no Git writes.

## Scope

- **Diff reviewed.** `6ded3e347..1ccca8b87`, limited to six files:
  - `core/analysis_runs/load_reference_evidence.py`;
  - `core/analysis_runs/load_reference_source.py`;
  - `core/reporting/result_export/src/load_reference.rs`;
  - `core/reporting/result_export/src/load_reference_source.rs`;
  - `core/reporting/result_export/tests/fixtures/load_reference_source_mutations.json`;
  - `tests/test_load_reference_source_readers.py`.
- **Also checked.** The reader re-pin in `ab5919133` (`tools/validation/qualification_load_reference.py` `MODULE_SHA256`, that line only). The rest of `ab5919133` is REVIEW_B's scope.
- **Addendum read.** The end of `LSI/T1_WP1_JOINED_READERS/RETURN.md`.
- **Byte identity.** The six files are byte-identical at `1ccca8b87`, `ab5919133` and the worktree HEAD (`git diff --stat 1ccca8b87 HEAD` over the reviewed paths is empty). Their sha256 values match the addendum's "after" column:
  - `load_reference.rs` `a045664d…c8adfffc`;
  - `load_reference_source.rs` `5fd4902c…a6212e16`;
  - `load_reference_evidence.py` `14e1750e…503b57c7`;
  - `load_reference_source.py` `32cee248…ca022fb4`;
  - the case file `862bcef9…70f29ecd`;
  - the test file `bbdf902d…b4b50e86`.

## Verdict: CLEAR (Python side run; Rust side by code reading only, cargo held)

All three notes are resolved as proposed, and the diff changes nothing else.

**Coverage of the reader bytes.** The Python reader bytes `load_reference_evidence.py` sha256 `14e1750ebb96c8284d71b49e01e55455c9e42fa6197ac979f0118271503b57c7` are covered by this review, taken together with `RETURN.md`:

- They differ from the reviewed `eff1fb3b…` by exactly the 6 lines below, all of them inside the joined-only branch.
- Their load-reference-1 path is byte-for-byte the code that `RETURN.md` reviewed.
- The re-pin in `ab5919133` equals the sha256 of these committed bytes, identical at `1ccca8b87`, `ab5919133` and HEAD.

**Rust not run.** Because of the host hold, I did not run `result_export` cargo. The Rust changes (`load_reference.rs` +20 lines, `load_reference_source.rs` +3 comment lines) are covered by code reading and by comparison with the Python peer. My own Rust run is outstanding. I can run it when the hold is lifted: expected 69 passed, and the shared file's 116 raw cases matching.

## What I verified

**N-1 (declared case).**
- Only `load_reference_source_mutations.json` and the Python declared-set assertion change. No reader code changes for N-1.
- The new case `NUM-unsafe-integer-in-publication` sets `results[0].value = 2^60` on n05-sparse. It carries `dispatch_rust`/`joined_rust` `CHECKED-JSON-UNSAFE-INTEGER: 1152921504606846976`, `dispatch_python`/`joined_python` `CHECKED-JSON-UNSAFE-INTEGER`, and a note.
- These match exactly what my own probe C10 observed in both languages in `RETURN.md`.
- Both harnesses resolve `<kind>_rust` / `<kind>_python` before the shared expectation.

**N-2 (joined S13 refusal).**
- **Code.** In both languages the check is placed after the SELECTED count check, inside the existing `if joined`. It refuses a `SOURCE_BLOCK_RECOVERY_UNAVAILABLE` whose `affected_refs` array contains the string ID of a selected case, with code `SOURCE_LOAD_REFERENCE_JOIN_SELECTED_UNAVAILABLE_DIAGNOSTIC`. The Rust `as_array` / `as_str` and the Python `isinstance(list)` / `isinstance(str)` guards are equivalent.
- **No false refusal.** The producer (`product_physics/src/lib.rs`, the only emitter) sets `affected_refs` to exactly `[load_case.id]` on a failed attempt, and a failed attempt is never selected. So a producer-valid envelope cannot trip the check.
- **Load-reference-1 unchanged:**
  - The new lines sit only inside `if joined`.
  - `tests/test_load_reference_readers.py` passes.
  - My widened pre-existing digest (52 raws, including both SF-1 fallback envelopes, which carry UNAVAILABLE under load-reference-1) is identical between a `git archive` of `6ded3e347` and HEAD. It covers contract, standing, transport, AnalysisRun digest and stress-neutral digest (`BR/py_digest_{6ded,head}.json`).
  - Physics-source-1 is unchanged too (analog probe PS-B04 is still accepted).
- **Probes.** I reran my `RETURN.md` Python probes on HEAD (`BR/probe_outcomes/`). Of 31 common probes, exactly one changed: B04 went from accept to `JOIN_SELECTED_UNAVAILABLE_DIAGNOSTIC` in dispatch and in the validator. Standing is now `unsupported`, and transport still accepts, since it carries no diagnostics. New edge probes (`BR/scripts/make_n2_probes.py`):
  - D02: a warning severity with refs `[1, "case"]` is refused.
  - D03: UNAVAILABLE on the not-joined case only is accepted, as intended.
  - D05: empty refs are accepted.
  - D04: the load-reference-1 fallback control is accepted by dispatch.
  - D01: `affected_refs` authored as the string `"case"` is accepted. See O-1.
- **Shared cases.** The case file gains exactly 5 cases (111 → 116). No existing case, table case or transport case changed, and their order is preserved (parsed comparison against `6ded3e347` `3d368d28…`). The author's recorded Rust and Python outcomes for the 5 new cases agree, apart from the declared N-1 difference (`followup_review_a/parity/`).
- **Mutants.** My own 5 Python mutants on a `git archive` copy of `1ccca8b87` are all killed (`BR/mutants/summary.txt`):
  - check removed;
  - first ref only;
  - all refs instead of any;
  - every case treated as selected (killed by the ordinary-case control);
  - the check also applied to load-reference-1 (killed by `ACCEPT-lr-fallback-sparse`).

**N-3 (comments).** The only change is one 3-line comment at R7 in each reader. The code tokens are unchanged, and the comment is accurate against `composite.rs` (`&[]` for an exact case) and J3's `SOURCE_PRESSURE_INVENTORY`.

**Checks.**
- pytest on HEAD (`tests/test_load_reference_source_readers.py`, `test_load_reference_readers.py`, `test_load_reference_source_schema.py`, `test_load_reference_schema.py`, with `PYTHONDONTWRITEBYTECODE=1 -p no:cacheprovider`): **1078 passed, 2 skipped** (`BR/pytest_readers.log`).
- I did not rerun the full 12-file integration set. Only these four files cover the changed code.

## Observation (not a finding)

**O-1.** The N-2 check matches only when `affected_refs` is an array. An UNAVAILABLE diagnostic whose `affected_refs` is authored as a bare string naming the selected case is accepted after a reseal (probe D01). The readers do not otherwise validate a diagnostic's shape, which is inherited. The producer always emits an array. This is the same class as the other producer-impossible, reseal-only edits already recorded as limits. No action is needed unless the manager wants a diagnostic-shape check.

## Limits

- No Rust run (host hold). The Rust claims rest on code reading and the author's recorded `final_cargo_test.log` (69/69).
- Scratch copies were deleted, and no cargo target was created.

## Addendum: the deferred Rust run (after the host hold was lifted)

- **What ran.** The manager reported the hold lifted. I then ran `cargo +1.97.1 test --locked --offline -j 2` in `core/reporting/result_export`.
- **Where.** On the committed head `277f3d02f`, in the worktree. No reviewed or dependency path differs from `1ccca8b87` there (`core/reporting`, `core/analysis_runs`, `core/serialization`, `core/units`, `fixtures/results`, `fixtures/product_preview`, `schemas`), and those paths are clean in the working tree.
- **Environment, as ROOT constrained it.** The shared `CARGO_TARGET_DIR` (`ls-vp-target`, reused; no new target) with `CARGO_INCREMENTAL=0`. Free disk space was 15 GB before and after.
- **Result: 69 passed, 0 failed** (22 unit, 6 derivative, 5 load-reference, 5 load-reference-source, 10 physics, 7 physics-source, 4 precision, 10 source-blocks). Log: `BR/cargo_result_export_277f3d02f.log`.
- **What that includes.**
  - `shared_adversarial_cases_match_in_rust` runs all 116 raw cases of the joined shared file, including the 4 N-2 cases and the declared N-1 case, and it passes.
  - `table_bytes_and_transport_cases_match_in_rust` passes.
  - The load-reference-1 contract tests (5) pass unchanged.

With this run, the Rust side of the follow-up is verified by execution as well as by reading. The verdict stays **CLEAR** for both languages. The reader bytes `14e1750e…` (Python) and the Rust readers at `a045664d…` (`load_reference.rs`) and `5fd4902c…` (`load_reference_source.rs`) are covered.
