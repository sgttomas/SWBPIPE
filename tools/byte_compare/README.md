---
name: swbpipe-byte-compare
description: Show that a numerical change leaves the results it did not mean to move byte-identical.
metadata:
  kind: instruction
  surface: tool
---
# Byte comparison

This tool shows whether two revisions give byte-identical results for the maintained solve requests in its manifest. A numerical change uses it to show that the results it did not mean to move did not move.

A pair is one fixture in one solver mode. It is **COMPARED** only where both revisions reached numerical work on it. Every other pair is reported by its class and never counted as agreement. A pair whose observed class differs from the manifest fails, even where both sides behave alike: two refusals of a request expected to solve are a failure, not a match.

Standard library only. It needs `git`, and `cargo` or a host wrapper around it.

## Use

Run from any directory. The scratch and work directories, and the `--table` file, must be outside every git work tree; the table file must not exist yet:

```bash
python3 tools/byte_compare/byte_compare.py compare --a <before> --b <after> \
  --scratch <new or empty dir> [--work <dir>] [--table <file>] \
  [--cargo "<cargo command>"] [--jobs 4] [--run-wrapper "<prefix>"] [--timeout 1800] \
  [--manifest tools/byte_compare/manifest.json] [--keep-worktrees]
```

On the shared T3 host, build through the cargo wrapper and run the batch inside one slot:

```bash
--cargo /Users/ryan/dev/chirality-t3/tools/t3_cargo.sh --run-wrapper /Users/ryan/dev/chirality-t3/tools/t3_slot.sh
```

The tool:

1. resolves A and B to full commits in the repository containing `--repo` (by default, this checkout's);
2. makes a detached worktree of each commit under `--work` (by default `<scratch>/build`) and requires it to be clean;
3. checks the manifest's coverage in each worktree (below);
4. builds the runner in each worktree with `cargo build --release --locked --bin release_timing` into its own target directory `<work>/target/<side>-<sha>`, removing any earlier binary first; it clears `RUSTFLAGS`, `CARGO_PROFILE_*`, `CARGO_BUILD_*`, `CARGO_TARGET_*` and every `GIT_*` variable from the build;
5. copies each binary to `<work>/bin/<side>-<sha>/`, records its SHA-256 and checks its embedded identity (below);
6. applies the lift to a temporary copy of each fixture under `<scratch>/inputs/<side>/`;
7. runs each copy through its side's binary in both modes, `sparse` and `dense`, one run at a time;
8. checks again that the binaries, the fixtures and the worktrees are unchanged;
9. classifies every run, judges every pair, writes the table and removes the worktrees with `git worktree remove`.

The target directories and binaries stay under `--work` for reuse. Delete them when finished.

## The runner

The runner is the release timing driver, `core/solver/release_timing` (DEL-04-05). It performs the desktop solve worker's ordinary solve:

- It parses the document and calls `run_linear_static_preview_value_with_mode` on `{"model": model, "materials": []}`, as the worker's `native_request` does.
- `--write` keeps `serde_json::to_string(&envelope)`, the worker's exact result bytes.
- Its build mirrors the desktop's. Its `serde_json` features are the desktop's. It sets no release profile, because the desktop sets none. Its `Cargo.lock` pins the desktop lock's versions.

A byte claim made with this tool is therefore a claim about the bytes the desktop publishes at cargo's default release profile.

The headless runner (`core/runner/headless`, `openpipestress-runner solve --solver-mode`) reaches the same entry, but it is not used for two reasons:

- It prints the envelope inside its own runner document, not as the worker's result bytes.
- Its `serde_json` features and lock are its own, not the desktop's. `tools/runner/build_runner.py` builds it for its `read` verb.

## Identity

`release_timing --build-identity` prints what the driver's `build.rs` embedded at build time. It runs on every build and reads the tree the binary was linked from. The tool requires:

- `source_commit` to be the stated commit;
- `source_tree` to be that commit's tree;
- `source_dirty` to be exactly `false`;
- `build_profile` to be `release`.

Otherwise it stops with exit 2 before anything runs. It also requires the binary to have been linked by this build. No option accepts a binary from elsewhere.

## The lift

The lift is the maintained tests' own re-authoring (`core/product_physics/src/kd5_nonlinear_tests.rs` `current_reference_request`), applied in memory:

- A bare model document is wrapped as `{"model": doc}`, as `w16_nonlinear_motion_tests.rs` does.
- `model.schema_version` must equal the entry's `source_version`. Otherwise the pair is `lift_refused` and fails.
- The version is then set to `MODEL_SCHEMA_VERSION` as `core/product_physics/src/model_format.rs` states it in the revision under test. It is read from each side's worktree and never hard-coded.
- A document already at that version is not changed.

Nothing else changes, and the tool checks this for every document. No field is migrated: the product's retired-version message says "Nothing is migrated." A document the lift cannot make current runs as it is and shows up as a refusal.

The lifted request is written as JSON text. Python writes each number as the shortest text that round-trips its binary64 value, and the driver parses with `float_roundtrip`, so every number reaches the entry unchanged.

Some requests cannot be passed to the runner unchanged. A request with top-level keys other than `model` and `materials`, or with materials the runner would drop, is classed `outside_runner_input` and not run.

## The manifest

`manifest.json` (schema `swbpipe-byte-compare-manifest/v1`) has:

- `roots`: glob patterns relative to the repository, each with its `why`;
- `entries`: each with a `path`, its `source_version` and an `expect`.

`expect` is either `{"disposition", "numerical"}` for both sides, or `{"A": {...}, "B": {...}}` for different expectations on each side. `numerical` must be `true` exactly when the disposition is `numerical_work`. An entry may give a `why`. Unknown keys are refused.

Dispositions:

| Class | Meaning |
|---|---|
| `numerical_work` | an envelope with `MECHANICS_SOLVED`, at least one result row and no blocking diagnostic |
| `entry_refusal` | the entry refused the document: the driver's entry error (exit 3), or an envelope blocked by a document-format code (`MODEL_SCHEMA_VERSION_RETIRED`, `PREVIEW_SCHEMA_VERSION_UNSUPPORTED`, `PRESSURE_MODEL_REAUTHOR_REQUIRED`, `PRESSURE_CONTRACT_UNSUPPORTED`) |
| `model_incomplete` | an envelope with `MODEL_INCOMPLETE` blocked by anything else |
| `outside_runner_input` | not run: the runner cannot take the request unchanged |

These classes are observed but never expected, so they always fail:

- `crash`: an exit other than 0 or 3, a timeout, stdout that is not one driver line, a missing envelope, an envelope that is not the bytes the driver reported, or materials the driver ignored;
- `empty_result`;
- `solved_with_blocking`;
- `status:<other>`;
- `lift_refused`;
- `missing`.

The current entries are:

- every `validation/references/kd5_nonlinear/*_request.json` (0.1.0);
- every `validation/references/w16_nonlinear_motion/requests/*.json` (0.8.0);
- the four preview and reference-system models (0.9.0);
- `fixtures/results/modal_state_friction_sliding.request.json`.

`a1` is expected `model_incomplete`, and the modal request `outside_runner_input`, for the reasons their entries give.

## Outputs and exit codes

The table goes to stdout, or to `--table`, a new file the tool creates exclusively. It refuses a `--table` path that already exists (so neither a file hard-linked to a request nor an earlier table is overwritten), that is inside a git work tree after resolving symlinks, that is under `--work`, or that is one of its own scratch paths below. It checks before building and again at the write. The table has:

- each side's revision, commit, binary SHA-256, identity and target version;
- one row per pair;
- the summary, which leads with the pairs **compared with numerical work on both sides** beside the total;
- the coverage lists;
- every entry not expected to reach numerical work, with what each side actually did;
- the failures.

`--scratch` keeps the raw outputs:

- `inputs/<side>/`: the lifted copies;
- `runs/<side>/<fixture>.<mode>.{stdout,stderr,status.json,envelope.json}`;
- `build-<side>.log`;
- `plan.json`;
- `results.json`: every run's class, detail and envelope SHA-256.

| Exit | Meaning |
|---|---|
| 0 | every pair as the manifest expects and every compared pair byte-identical |
| 1 | a failing pair, a byte difference among compared pairs, a coverage failure, or a binary, fixture or worktree that changed during the runs |
| 2 | nothing compared: bad arguments or manifest, a scratch or work directory inside a git work tree, a refused `--table` destination, a failed build, or a binary not built as stated; also when the `--table` destination came to exist or entered a work tree during the comparison, in which case nothing is written there and `results.json` holds the comparison |

## Threat model

The tool is verification tooling, reviewed against these four threats (AGENTS.md "Proportionate review"). `tests/test_byte_compare_tool.py` covers each without building cargo: it uses a stub cargo and a stub driver in a two-commit repository.

- **(a) Agreement where the intended numerical work did not happen.** Only `numerical_work` on both sides is compared. Each of the following is a named class, never agreement: refusals (including schema and contract refusals), model-incomplete results, empty result sets, crashes and timeouts. An observed class that differs from the manifest fails, so two refusals of a request expected to solve fail.
- **(b) Binaries not built from the stated revisions.** The tool:
  - uses fresh detached worktrees, required clean before and after the build;
  - keeps a target directory per side and commit, and removes the binary before each build;
  - clears the flag, profile and target environment;
  - runs only binaries whose embedded identity names the commit, its tree, a clean source and the release profile;
  - re-hashes each binary after the runs.

  It never runs a binary from `PATH`, another worktree or an argument.
- **(c) A frozen request modified on disk.** The lift touches only copies under the scratch directory. The scratch directory must be outside every git work tree. After the runs, every fixture is re-hashed and every worktree must still be clean. The `--table` file, written after those checks, must be new and is created exclusively, outside every work tree, under neither `--work` nor the tool's scratch evidence paths, as above.
- **(d) Silent loss of coverage.** In each worktree the tool lists every file under a stated root that the manifest does not name, and fails on any. It also fails on any manifest path that does not exist. For every entry not expected to reach numerical work, it reports what each side did, and it fails when an expected refusal solves. `tests/test_byte_compare_tool.py` also checks the maintained manifest against the current tree, so a fixture added under a root without an entry fails CI.

Outside the model:

- Toolchain differences between hosts are not controlled; the identity records `rustc -Vv`.
- Only the stated roots are closed. The five single-file roots close existence only.
- What a fixture tests is the manifest's business, not the tool's.
