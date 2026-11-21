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

Run from any directory. `--scratch` (and `--work`, if given) names a directory that must not exist yet: the tool creates it, and its parent must resolve to an existing directory outside every git work tree. `--table` is a file name, created in the scratch area:

```bash
python3 tools/byte_compare/byte_compare.py compare --a <before> --b <after> \
  --scratch <new dir> [--work <new dir>] [--table <file name>] \
  [--cargo "<cargo command>"] [--jobs 4] [--run-wrapper "<prefix>"] [--timeout 1800] \
  [--manifest tools/byte_compare/manifest.json] [--keep-worktrees]
```

On the shared T3 host, build through the cargo wrapper and run everything else under the slot wrapper:

```bash
--cargo /Users/ryan/dev/chirality-t3/tools/t3_cargo.sh --run-wrapper /Users/ryan/dev/chirality-t3/tools/t3_slot.sh
```

What runs where:

- `--cargo` runs each side's build. Cargo starts the compiler and the build scripts itself, inside that command.
- `--run-wrapper` starts every other run of a built binary or of the build's compiler, each as its own wrapped command: each side's `rustc -vV` and `rustc --print sysroot`, each binary's `--build-identity` check, and the batch of fixture runs, which executes inside one wrapped command.
- The outer `python3` itself, and its `git` calls, run directly. They start no binary and no build.

Without `--run-wrapper` those runs start directly. A wrapper that is not an executable file refuses (exit 2) before anything is built. A wrapper that refuses a run with exit 97 or 98 (the slot wrapper's host conditions) stops the comparison with exit 2, as does `--cargo` refusing the build with one of those exits.

The tool:

1. resolves A and B to full commits in the repository containing `--repo` (by default, this checkout's);
2. reserves the scratch area (and the work area, by default `<scratch>/build`), and in it every directory and fixed file it will write (below, "Every sink"), before anything is built;
3. makes a detached worktree of each commit in its reserved directory and requires it to be clean;
4. checks the manifest's coverage in each worktree (below);
5. builds the runner in each worktree with `cargo build -vv --message-format=json --release --locked --bin release_timing` into its own fresh, empty target directory `<work>/target/<side>-<sha>`; it clears `RUSTFLAGS`, `CARGO_PROFILE_*`, `CARGO_BUILD_*`, `CARGO_TARGET_*` and every `GIT_*` variable from the build, as a first barrier;
6. attests each build from its own output (below) and refuses, before anything runs, unless both sides used an identical `rustc -vV`;
7. copies each binary to `<work>/bin/<side>-<sha>/`, requires its SHA-256 to be the attested binary's and checks its embedded identity (below);
8. applies the lift to a temporary copy of each fixture under `<scratch>/inputs/<side>/`;
9. runs each copy through its side's binary in both modes, `sparse` and `dense`, one run at a time;
10. checks again that the binaries, the fixtures and the worktrees are unchanged;
11. classifies every run, judges every pair, writes the results and the table and removes the worktrees with `git worktree remove`.

The target directories and binaries stay in the work area. Every comparison needs a new scratch area (and work area, if given separately). Delete them when finished.

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

The profile's name says nothing about the settings a build actually used. Those are established by the build attestation.

## Build attestation

The tool attests each build from what that build itself recorded: cargo's JSON messages (stdout), its `-vv` text (the compiler invocations on stderr, and the build scripts' output on stdout) and the target directory. It never edits the tree, so a historical revision is attested exactly as committed. A revision whose driver lacks `--build-identity` is still refused.

Wherever cargo's JSON messages carry a fact, they are the source. The `-vv` text is used only for what they do not give.

| From the JSON messages | From the `-vv` text |
|---|---|
| each unit's package id (source, name, version), target kind, crate types and source path | the compiler program of every unit |
| each unit's `opt_level`, `debuginfo`, `debug_assertions` and `overflow_checks` | every `-C` option: `lto`, `codegen-units`, `panic`, `strip`, `embed-bitcode` and any other |
| resolved features | the `--cfg` flags, checked against the features and the build scripts' declared cfgs |
| `fresh`, the files each unit produced, the uplifted executable | the `--extern` link graph, `-L` paths, lint levels, and any other argument |
| each build script's cfgs, environment, link libraries and paths, and output directory | the environment cargo gave each compiler invocation and each build-script run |
| the build's success | the build scripts' directives (`cargo:` lines), checked against cargo's record of them |

The attestation refuses (exit 2, before any run) unless all of the following hold:

- **Complete and fresh.** Every unit has exactly one compiler invocation and exactly one artifact message, and none is reported fresh. Every line of both streams is accounted for: an unparseable line, an unknown message, an unknown stderr line, or a build that did not finish once successfully, refuses.
- **Runtime profile checks.** The runner binary, and every unit it links (followed through `--extern`, past nothing but proc macros), has exactly the manifest's `build_settings.runtime`. That means its JSON profile and every `-C` option except `metadata` and `extra-filename`. The attested set is the link graph, not a flag filter: in the real build, linked units get `-C opt-level=3` and build-time units get none, so filtering by flags could not separate them.
- **Build-time dependency checks.** Build scripts, proc macros and the units they link never run in the binary, but they can change what is compiled into it: a build script built under another setting can generate different source for a unit whose own flags are exactly right. Each such host unit has exactly the manifest's `build_settings.host`, which is cargo's documented default for build-time dependencies. A proc macro additionally carries `-C prefer-dynamic`. No unit may be both runtime and host, and every unit is one or the other.
- **Every flag accounted for.** Accepted arguments are cargo's own (`--crate-name`, `--crate-type`, `--edition`, `--emit` of dep-info, metadata and link, `--error-format`, `--json`, `--out-dir`, `--check-cfg`, `--cap-lints`), resolved features, a build script's declared cfgs, lint levels, `-L dependency=` inside the target directory, and `--extern` to a file this build produced (or `proc_macro`, for a proc macro). Anything else refuses: `-Z`, `--target`, `-l`, `-L native=`, a linker or other `-C` option outside the block, a stray `--cfg`. A compiler invocation's environment holds only what cargo sets and the variables its package's build script declared. A build-script run's environment holds only what cargo sets for one, with no rustflags, the build's own rustc and the settings of the units it serves. Cargo can report the profile's trim-paths setting for a build-script run. Either variable may be absent. When present, `CARGO_TRIM_PATHS_SCOPE` must be `none` and `CARGO_TRIM_PATHS_REMAP` must be empty. A nonempty remap can expose build-specific paths to generated source.
- **One rustc.** Every unit is compiled by the same program. That program is `rustc` itself (not a wrapper) and is its own sysroot's `bin/rustc`. Its `rustc -vV` must be identical on both sides.
- **Provenance.** Each unit's source is in the generated worktree (a path source), or is the exact Cargo.lock registry package. For a registry package that means the same name, version and source; the cached archive's SHA-256 equal to the lock's checksum; and the extracted source cargo compiled holding exactly the archive's files and bytes, plus cargo's own `.cargo-ok`. Any other source (git, another registry, source replacement, a `[patch]` outside the worktree) refuses. Each side verifies every registry package itself, after its own build; nothing is remembered from the other side. The archive's digest and a digest over the extracted files are bound into that side's attestation (`registry` in `results.json`).
- **Source closure.** For each side, every file rustc reports a unit read (the unit's own dep-info, which `--emit=dep-info` writes beside its output), the path packages' manifests and the driver's `Cargo.lock` must each be one of: a committed regular file of that side's worktree (`git ls-files -s`), a file of a registry package that side verified, or a generated file the manifest allows. Anything else refuses. A link anywhere in the closure refuses: every path is first walked exactly as written, one component at a time with no link followed (each prefix examined with `lstat`, `..` stepping back to the parent of the prefix walked so far), and a component that is a link refuses even where a later `..` cancels it. The same operations apply to every reported path. A committed link (mode 120000) is never a committed file either. This is a list comparison over what rustc reports. It stays one: if it ever needs rules about which paths matter, the limit is stated here instead.
- **Build scripts within the supported closure.** A build script may emit only `rerun-if-changed`, `rerun-if-env-changed`, `rustc-check-cfg`, `rustc-cfg`, `rustc-env` and `warning`, and these must match cargo's record. Link libraries or paths, any other directive and any other output line refuse. Each build script's output directory may hold only the files `build_closure.generated_sources` lists for its package (serde's and serde_core's `private.rs`); their SHA-256 is recorded.
- **Binding.** The file rustc wrote for the runner, the binary cargo uplifted, and the copy that runs have equal SHA-256. The `-vv` log (`build-<side>.log`) and the JSON messages (`build-<side>.jsonl`) stay in the scratch directory, and their SHA-256 is recorded beside the binary's, from the same build.

**Trusted, and named as such.** The prebuilt standard library in the compiler's sysroot is part of the toolchain. Cargo compiles it nowhere in the build, so no compiler invocation records it. What identifies it is the `rustc -vV` output and the sysroot path (`rustc --print sysroot`), which the tool records beside the identity.

The manifest states the settings once, in `build_settings`, with their ground: cargo's documented default release profile, and its default for build-time dependencies, at the toolchain the identity records. `tests/test_byte_compare_tool.py` builds a synthetic repository at cargo's default release profile under controlled configuration (the tool's clean environment, an empty Cargo home, no cargo configuration in any ancestor) and fails if that build and the block disagree. The block cannot drift from what "default release" means.

This is bounded to this runner's build closure. Anything outside it refuses rather than being interpreted. The tool is not a general Cargo provenance system.

One limit of the source closure: a build script's own reads of files (as opposed to the sources rustc compiles) appear in no dep-info. They are seen only through what the build script prints and what it generates, both checked above; the files it reads are not compared.

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
- `entries`: each with a `path`, its `source_version` and an `expect`;
- `build_settings`: the settings every attested unit must have, stated once with their ground. `runtime` and `host` each give a JSON `profile` and the nine `codegen` options (`null`: cargo passes none). `host` also gives `proc_macro_codegen`;
- `build_closure`: the one registry, and the files each package's build script may generate.

Each entry's scratch input and run evidence are named by an ID derived from its path (`/` becomes `__`). Two entries whose IDs collide are refused when the manifest is loaded.

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

The table goes to stdout, or with `--table <name>` to that new file in the scratch area. The name must be a plain file name (no `/`, not `.` or `..`) and none of the tool's own names below; it is reserved with the other fixed files before anything is built. The table has:

- each side's revision, commit, binary SHA-256, identity and target version;
- each side's build attestation: unit counts, the binding digest, the log digests, the compiler and its trusted sysroot, and the `rustc -vV` both sides share;
- one row per pair;
- the summary, which leads with the pairs **compared with numerical work on both sides** beside the total;
- the coverage lists;
- every entry not expected to reach numerical work, with what each side actually did;
- the failures.

The scratch area keeps the raw outputs:

- `inputs/<side>/`: the lifted copies;
- `runs/<side>/<fixture>.<mode>.{stdout,stderr,status.json,envelope.json}`;
- `build-<side>.log`: the `-vv` text (stderr), and `build-<side>.jsonl`: the JSON messages and build-script output (stdout);
- `plan.json`;
- `results.json`: every run's class, detail and envelope SHA-256.

| Exit | Meaning |
|---|---|
| 0 | every pair as the manifest expects and every compared pair byte-identical |
| 1 | a failing pair, a byte difference among compared pairs, a coverage failure, or a binary, fixture or worktree that changed during the runs |
| 2 | nothing compared: bad arguments or manifest (including colliding entry IDs), a scratch or work area that exists, resolves inside a git work tree or cannot be reached without a link, a refused `--table` name, a sink that already exists or a reserved directory replaced, a failed build, a build the attestation refuses, or a binary not built as stated |

## Threat model

**Limit:** the tool never takes an executed run as good from output alone; it trusts the built driver's own report of numerical work.

What the tool defends:

- its own writes never land outside a fresh reserved area;
- each side's build is attested from that side's own verified inputs;
- no input is silently skipped or aliased;
- a run counts only on its own exit and report.

What it does not defend:

- a compromised toolchain or sysroot (rustc, cargo and the sysroot's prebuilt standard library are trusted: recorded, and required identical on both sides, but their own build is not verified);
- the driver's own report of numerical work;
- a hostile host: a process that races the tool, changing a path between the tool's check and the moment a subprocess uses it. A stale or pre-existing area, a planted link, a substituted directory present when the tool checks (a link back to the same directory included), and a reserved file replaced before the tool writes or reads it are all refused. A link planted at a run's envelope path after the batch's absence check and before the driver's own write is in that excluded interval, between a check and a subprocess's use.

The tool is verification tooling, reviewed against this model (AGENTS.md "Proportionate review"). `tests/test_byte_compare_tool.py` covers it. Most tests build nothing: they use a stub cargo and a stub driver in a two-commit repository. The build attestation is also tested on `captured_build/`, a maintained capture of the real driver's `-vv` JSON build at this toolchain, with each unit's dep-info, the committed mode of each worktree file it read and each registry package's verified files. Its paths are cleaned (`/WT`, `/TARGET`, `/CARGO_HOME`, `/SYSROOT`, `/HOME`) and its rendered compiler warnings dropped, and it pins the formats the parser reads. The tests marked `runs_cargo` build a synthetic offline two-crate repository with real cargo: the default-release control, one test for each inherited configuration below, and a committed link in the source closure. Every test that checks a diagnostic's wording does so through one helper, which the mutation harness switches off (`BYTE_COMPARE_BEHAVIOUR_ONLY=1`), so that a mutation counts as caught only where what the tool did changed.

How the four original threats map onto it:

- **(a) Agreement where the intended numerical work did not happen** (a run counts only on its own exit and report). Only `numerical_work` on both sides is compared. Each of the following is a named class, never agreement: refusals (including schema and contract refusals), model-incomplete results, empty result sets, crashes and timeouts. An observed class that differs from the manifest fails, so two refusals of a request expected to solve fail.
- **(b) Binaries not built from the stated revisions, or not at the default release settings** (each side's build attested from its own verified inputs). The tool:
  - uses fresh detached worktrees, required clean before and after the build;
  - builds each side into a fresh, empty target directory;
  - clears the flag, profile and target environment, as a first barrier;
  - attests every unit's settings, sources and source closure from the build's own output, as above. That closes every route inherited configuration takes: an ancestor or Cargo-home `.cargo/config.toml`, a `[profile.release.package.<dep>]` or `build-override` setting, rustflags, and a rustc wrapper. Real-cargo tests refuse each of these, the ancestor configuration and the build override on side A and on side B;
  - requires an identical `rustc -vV` on both sides;
  - runs only binaries whose embedded identity names the commit, its tree, a clean source and the release profile, and whose SHA-256 is the file rustc wrote;
  - re-hashes each binary after the runs.

  It never runs a binary from `PATH`, another worktree or an argument.
- **(c) A frozen request modified on disk** (writes never outside a fresh reserved area). Every write is one of the sinks below. After the runs, every fixture is re-hashed and every worktree must still be clean.
- **(d) Silent loss of coverage** (no input silently skipped or aliased). In each worktree the tool lists every file under a stated root that the manifest does not name, and fails on any. It also fails on any manifest path that does not exist, or that is not a committed regular file reached without a link. Colliding entry IDs are refused, and every per-entry file is created exclusively, so no request can stand in for another. For every entry not expected to reach numerical work, it reports what each side did, and it fails when an expected refusal solves. `tests/test_byte_compare_tool.py` also checks the maintained manifest against the current tree, so a fixture added under a root without an entry fails CI.

Also outside the model:

- The files cargo leaves that the tool reads (the binary cargo uplifted, the file rustc wrote, each unit's dep-info and the build scripts' `OUT_DIR` files) are read by name with no held identity (the binary and dep-info content reads follow no link; generated-output hashing uses ordinary pathname opens after checking the `OUT_DIR` leaf and walked entries for links). This is a deliberate trust assumption, not a technical limit: the tool could hold a file's identity once it has opened it, but it cannot establish which file cargo originally created, so it takes the files cargo leaves as cargo's and makes no replacement promise for them. A replacement of one between cargo's exit and the tool's read is outside the model. The binary that runs is the tool's own copy, whose SHA-256 must equal that of the file rustc wrote, and which is held from the copy onward.
- Only the stated roots are closed. The five single-file roots close existence only.
- What a fixture tests is the manifest's business, not the tool's.

## Every sink

Every path the tool writes, how it is created, and the test that covers it. The table is derived from `byte_compare.py`, not from memory: every call that can create or change a path was listed with

```bash
grep -nE "os\.mkdir|os\.open\(|\.open_new\(|\.write\(|new_file\(|\.directory\(|Area\.reserve\(|write_text|write_bytes|shutil\.|os\.rename|os\.link|os\.symlink|os\.unlink|rmdir|subprocess\.(run|Popen)|stdout=|--target-dir|worktree.*add|worktree.*remove|--write" tools/byte_compare/byte_compare.py
```

and each hit traced to the path it writes. That covers every `open`, write, copy, `mkdir`, rename and link, every subprocess, and each path or output stream given to one. No `write_text`, `write_bytes`, `shutil` copy, rename, link or unlink remains in the tool.

How the tool creates what it writes:

- **Reserve:** `Area.reserve`. The parent resolves strictly (a missing component, with or without a following `..`, refuses), must be outside every git work tree, and is opened one component at a time from `/` with no link followed. The area is then created with `mkdir` relative to the parent's descriptor; it must not exist in any form, a link included.
- **Directory:** `Area.directory`, `mkdir` relative to the parent area's held descriptor, then opened with no link followed and held. All of these are made before anything is built. Checking one (`Area.verify`, or the batch's own check) reopens its absolute path one component at a time from `/` with no link followed and requires the held directory's device and inode, so a link at the directory or at any ancestor refuses, even a link back to the held directory.
- **Exclusive file:** created with `O_CREAT | O_EXCL | O_NOFOLLOW` relative to a held directory descriptor, so a link or an existing file at the name refuses.
- **Held file:** an exclusive file the tool writes or reads later. Before each later write or read, its name is reopened with `O_NOFOLLOW` relative to the held directory and must be a regular file with the held file's device and inode and a single link; otherwise the tool refuses and nothing is written to it.
- **Checked path:** a path handed to a subprocess. Where the checks are, exactly:
  - `git worktree add`: every reserved directory is verified before and after.
  - each side's cargo build: verified before and after, and the two build logs are checked as held files before they are read.
  - the `rustc -vV` and `--print sysroot` probes: not verified around them.
  - each `--build-identity` check: verified before it, not after.
  - the run batch: verified before it starts. With a run wrapper, the plan is checked as a held file before the batch reads it by name. Inside the batch, before the first run every run's outputs are checked as held files and every envelope is checked absent. Before each driver run, the scratch area and that run's directory are checked as reserved directories, and that run's input and binary must be the regular, singly linked files the tool wrote (same device and inode).
  - after the batch: verified once, for the whole reservation; each run's status and stdout are then read as held files.
  - before `results.json` and the table are written: verified, and each is written as a held file.
  - cleanup (`git worktree remove`): not verified.

Paths are relative to the scratch area; `<work>` is the work area (`build/` by default); `<side>` is `A` or `B`.

| Path | Written by | How it is created | Covered by |
|---|---|---|---|
| the scratch area itself | the tool | reserve | `test_a_link_at_any_sink_of_an_existing_area_is_refused_before_anything_is_built` (sink `""`), `test_an_existing_empty_area_is_refused_before_anything_is_built`, `test_an_area_inside_the_callers_work_tree_is_refused_before_anything_is_built`, `test_an_area_inside_a_worktree_kept_by_an_earlier_run_is_refused`, `test_an_area_reached_through_an_alias_is_refused_before_anything_is_built`, `test_an_area_through_a_linked_directory_outside_every_work_tree_is_made_where_it_resolves` |
| a separate `--work` area | the tool | reserve, joined to the scratch reservation | `test_a_link_at_a_separate_work_area_is_refused_before_anything_is_built` |
| `build/`, `inputs/`, `inputs/<side>/`, `runs/`, `runs/<side>/`, `<work>/target/`, `<work>/bin/`, `<work>/bin/<side>-<sha>/` | the tool | directory | the sink test (`inputs`, `build`), `test_a_reserved_directory_swapped_for_a_link_during_the_build_is_refused_and_nothing_runs`, `test_an_area_never_takes_a_name_it_did_not_make`, `test_a_reserved_directory_swapped_for_a_link_is_refused_and_writes_stay_in_it`, `test_a_reserved_directory_swapped_for_a_link_back_to_itself_is_refused`, `test_a_deeper_reserved_directory_reached_through_a_substituted_parent_is_refused` |
| `plan.json`, `results.json`, the `--table` file | the tool | exclusive file, reserved before the build; written as a held file | the sink test, `test_a_fixed_sink_is_reserved_before_the_build_so_a_later_link_cannot_take_it`, `test_a_table_name_that_is_not_a_new_plain_name_in_the_area_is_refused`, `test_a_reserved_file_replaced_before_it_is_written_or_read_is_refused` |
| `build-<side>.jsonl`, `build-<side>.log` | the tool opens them; cargo writes its stdout and stderr into them | exclusive file; checked as held files after the build, before they are read | the sink test (`build-A.log`, `build-A.jsonl`) |
| `inputs/<side>/<id>.json` | the tool | exclusive file; checked by identity before the run that reads it | the sink test, `test_a_link_planted_at_a_sink_during_the_build_is_refused_and_nothing_runs` |
| `<work>/bin/<side>-<sha>/release_timing` | the tool (the copy that runs) | exclusive file, mode 0755, bytes checked against the attested binary; checked by identity before each run | the sink test, the planted-link test (`bin/B`) |
| `runs/<side>/<id>.<mode>.{stdout,stderr,status.json}` | the run batch; the driver's stdout and stderr go into the first two | the tool creates every output of every run as an exclusive file before the batch, recording each file's device and inode in the plan. The batch reopens the scratch area from `/` with no link followed, requires it to be the reserved one, and writes each output as a held file. The tool reads status and stdout back as held files. | the sink test, the planted-link test, `test_a_run_output_replaced_after_its_reservation_is_refused`, `test_a_reserved_file_replaced_before_it_is_written_or_read_is_refused` |
| `runs/<side>/<id>.<mode>.envelope.json` | the driver (`--write <path>`), by path | must not exist (checked without following a link) before any run starts; the area and run directory are checked to be the held ones before each run | the sink test, the planted-link test (`envelope`) |
| `<work>/target/<side>-<sha>/**` | cargo, rustc and build scripts, by path (`--target-dir`) | directory, reserved empty; checked path | the sink test (`build/target/A-…`), the swap test (`build`) |
| `<work>/wt-<side>-<sha12>/**` | `git worktree add`, by path, into the reserved empty directory | directory; checked path | the sink test (`build/wt-A-…`), the swap test |
| `<git common directory>/worktrees/<Git-allocated worktree id>/**` (outside the area) | git itself, for `worktree add` | git's own administrative files for each generated worktree, in the git common directory of `--repo`'s repository (`--repo` may itself be a linked worktree, whose `.git` is a file; git may suffix the ID to avoid a collision). They are removed only when cleanup with `git worktree remove` succeeds, and not with `--keep-worktrees`. | `test_identical_revisions_compare_only_the_numerical_pairs` (no worktree left registered) |
| stdout (the table without `--table`) and stderr (diagnostics) | the tool | streams, not paths | — |

Nothing else is written. Cargo may extract or fetch into its own registry cache under `$CARGO_HOME`; the tool only reads and verifies it. The host's wrappers keep their own logs.
