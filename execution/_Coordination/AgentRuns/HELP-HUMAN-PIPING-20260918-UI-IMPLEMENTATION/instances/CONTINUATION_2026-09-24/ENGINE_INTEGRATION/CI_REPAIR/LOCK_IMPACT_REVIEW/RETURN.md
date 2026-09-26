# Leaf lock repair: independent lock-impact review

**Verdict: CLEAR, with no blocking finding.** The four leaf `Cargo.lock` repairs from `e65001ad50072d3399bf204da02055b0f360353a` to `5b1ccd356c0d9656f7b9f42c6558f7a4c73f9295` are correct and minimal. The native executable's compiled closure does not change. These repairs do not require any native witness to be repeated. Three non-blocking carry-forwards follow at the end. One affects how the next readiness check may word its native-input claim.

Unless shown otherwise, paths are relative to `projects/chirality-piping`. Every command, script and raw output is under [_run_records/](_run_records/). [EXECUTION.json](_run_records/EXECUTION.json) records the identity, environment and write scope. The reviewer is a Type 2 TASK dispatched by ROOT. It did not implement the repair and did not delegate. Every Cargo command used `cargo +1.97.1` (rustc 1.97.1, the CI pin) with at most 2 jobs. Each crate had its own target directory outside the repository. No repository source, lockfile or Git state was modified.

## 1. Every diff only adds lines

[check_additive_and_pins.py](_run_records/check_additive_and_pins.py) parses both revisions with `tomllib` and writes [check1_3.json](_run_records/check1_3.json). The raw diff is [lock-diff-e650-5b1c.patch](_run_records/lock-diff-e650-5b1c.patch) and contains no removed lines.

| Lock | Packages | Added entries | Existing lists that gained entries |
|---|---|---|---|
| `core/reporting/report_package` | 31 → 32 | `open_pipe_stress_units` (path) | `result_export` gains `units` |
| `validation/benchmarks/mechanics` | 33 → 34 | `open_pipe_stress_units` (path) | `result_export` gains `units` |
| `validation/benchmarks/stress` | 30 → 31 | `open_pipe_stress_units` (path) | `result_export` gains `units` |
| `validation/benchmarks/numerical_integrity` | 25 → 37 | `canonical_json` (path) plus 11 registry packages | `product_physics` gains `canonical_json` and `sha2` |

In all four locks:
- No `(name, version, source)` key was removed.
- No checksum or source changed.
- No existing dependency lost an entry.
- The lock format stays at `version = 4`, with no duplicate keys.

## 2. Each lock is exactly what Cargo resolves

**Commands succeed without rewriting the locks.** `cargo metadata --locked --offline --format-version 1` and `cargo tree --locked --offline --target all -e normal,build,dev` exit 0 for all four crates. The SHA-256 of all 39 discovered locks is identical before and after every stage ([meta/RESULTS.json](_run_records/meta/RESULTS.json)).

**Lock equals resolve.** [check_resolve_vs_lock.py](_run_records/check_resolve_vs_lock.py) writes [RESOLVE_VS_LOCK.json](_run_records/resolve/RESOLVE_VS_LOCK.json). For each crate, the lock's package set and dependency edges equal the resolve's nodes and edges: 32/32, 34/34, 37/37 and 31/31. Nothing is extra and nothing is missing. Every added entry has a chain back to the root:
- `units ← result_export ← root`.
- `canonical_json ← product_physics ← root`, and `ryu` through `canonical_json`.
- `sha2 ← product_physics`, and from `sha2`: `digest`, `block-buffer`, `crypto-common`, `generic-array`, `typenum`, `version_check` (a build dependency), `cfg-if`, `cpufeatures` and `libc`.

**Cause.** These manifest edges already existed at 8b982aa7:
- `result_export → units` was added in `1792774a2`.
- `product_physics → canonical_json/sha2` was added in `22452ecd1`.

So the leaf locks were stale from those commits onward. The problem only surfaced when hosted `--locked` fetch reached `report_package`.

**Independent reproduction.** In a scratch export of 5b1c, each lock was reset to its e650 bytes and `cargo metadata` was run without `--locked`. This is Cargo's minimal update. Run offline and again online against the crates.io index, it regenerates all four locks **byte-identical** to 5b1c. The online run noted that `generic-array 0.14.9` is available but kept `0.14.7`, as committed.

## 3. Every added registry pin already exists in the repository

There are 11 added registry packages: `block-buffer 0.10.4`, `cfg-if 1.0.5`, `cpufeatures 0.2.17`, `crypto-common 0.1.7`, `digest 0.10.7`, `generic-array 0.14.7`, `libc 0.2.189`, `ryu 1.0.23`, `sha2 0.10.9`, `typenum 1.20.1` and `version_check 0.9.5`.

- Each one already appears, with the identical checksum, in 4 to 13 maintained e650 lockfiles.
- All 11 are in `core/product_physics/Cargo.lock` (the dependency the repair follows) and `core/reporting/result_export/Cargo.lock`.
- Eight are also in `apps/desktop/src-tauri/Cargo.lock`. That lock pins `cfg-if 1.0.4`, `libc 0.2.186` and `typenum 1.20.0` instead.
- **No third-party version is new to the repository.**
- Across all 106 tracked lockfiles at e650 and 5b1c, no name/version/source key has more than one checksum.

## 4. Locked fetch and tests

**Fetch.** The CI plan is `cargo_plan()` imported from `tools/ci/numerical_ci.py`. Its 39 fetch commands equal this review's commands apart from the `+1.97.1` toolchain argument ([ci-plan-equivalence.json](_run_records/ci-plan-equivalence.json)).
- `cargo +1.97.1 fetch --locked` passes **39/39** with the warm cache ([fetch/](_run_records/fetch/RESULTS.json)).
- It also passes **39/39** from an empty `CARGO_HOME`, which is the analogue of a fresh hosted runner ([fetchcold/](_run_records/fetchcold/RESULTS.json)).
- No lock was rewritten.

**Tests.** `cargo +1.97.1 test --locked --offline -j 2` exits 0 for all four crates ([test/](_run_records/test/RESULTS.json)):

| Crate | Tests passed |
|---|---|
| `report_package` | 19 (7 unit, 12 `tests/container.rs`) |
| `mechanics` | 41 |
| `stress` | 23 |
| **Total** | **83**, 0 failed, 0 ignored |
| `numerical_integrity` | Builds as a `src/main.rs` binary with 0 tests |

This exactly matches the recorded 83 tests and the zero-test `numerical_integrity` build. No numerical benchmark execution is claimed.

## 5. Impact on the native build

**Recorded inputs.** [check_native_inputs.py](_run_records/check_native_inputs.py) writes [check5_inputs.json](_run_records/check5_inputs.json). The 1,092 recorded inputs (`inputs-pre.json` equals `inputs-post.json`) are the tracked files under `apps/desktop`, `core`, `schemas` and `fixtures`, plus `package.json` and `package-lock.json`. They include 35 `Cargo.lock` files. `core/reporting/report_package/Cargo.lock` is the **only** one of the four changed locks among them; `validation/**` is not recorded.

| Revision | Recorded inputs that differ | Missing | New tracked files under the recorded roots |
|---|---|---|---|
| 8b982aa7 | 0 (sanity check) | 0 | 0 |
| e650 | 0 | 0 | 0 |
| 5b1c | **exactly 1**: `core/reporting/report_package/Cargo.lock` | 0 | 0 |

At 5b1c the one differing path has recorded hash `946819d8…` and current hash `75d36eac…`.

**Unchanged at 8b982aa7, e650 and 5b1c:**
- `apps/desktop/src-tauri/Cargo.lock`: `23d43656…`, blob `8b2ae3b5f5`.
- `apps/desktop/src-tauri/Cargo.toml`: `ae8eb2d1…`, blob `91277df5d1`.
- `tauri.conf.json`, `apps/desktop/package.json`, `package-lock.json` and `scripts/build-wasm-engine.mjs`.
- The WASM crates' locks and manifests:
  - `core/model_operations/operation_applier`: lock `40af164d…`, manifest `cfa2c68f…`.
  - `core/loads/self_weight_wasm`: lock `4b56e2f3…`, manifest `9d6527f2…`.
  - Both are separate Cargo roots for `build:wasm`. Both pass `fetch --locked`, and no changed path lies inside their path-package graphs.

**Cargo semantics.** Cargo reads only the root package's or workspace's `Cargo.lock`. A path dependency's own lock is ignored. No Rust source in `core` or `src-tauri` names `Cargo.lock`, and the only build script in the repository's crates is `src-tauri/build.rs` (tauri-build). See [check_include_targets.py](_run_records/check_include_targets.py) and its [output](_run_records/check5_include_targets.json).

**Empirical evidence** ([check_src_tauri_resolve.py](_run_records/check_src_tauri_resolve.py), [SRC_TAURI_RESOLVE.json](_run_records/native/SRC_TAURI_RESOLVE.json), [SRC_TAURI_CLEAN_BUILDS.json](_run_records/native/SRC_TAURI_CLEAN_BUILDS.json)):
- **Resolve is identical in every variant.** After `cargo +1.97.1 fetch --locked` for src-tauri, `cargo metadata --locked --offline` for src-tauri gives an identical 487-node resolve (normalised SHA-256 `70f8f605…`) and an identical `aarch64-apple-darwin` tree (`d572df81…`). The variants were:
  - the real 5b1c worktree;
  - a scratch export with the leaf lock at its 5b1c bytes;
  - the same export with the e650 bytes;
  - the same export with the leaf lock deleted;
  - the same export with the leaf lock replaced by invalid TOML.

  The src-tauri lock was never rewritten.
- **No recompilation.** `report_package` was compiled inside the src-tauri resolve. After swapping its leaf lock from the 5b1c bytes to the e650 bytes, a verbose rebuild reported **all 31 units Fresh, 0 recompiled**.
- **Byte-identical outputs.** Clean builds in the same target path with `CARGO_INCREMENTAL=0` produced **60/60 byte-identical** `.rlib`/`.rmeta` files for the 5b1c and e650 leaf locks. A same-lock control was also identical. An earlier two-path incremental comparison in `SRC_TAURI_RESOLVE.json` (`C_compile`) was not controlled and is superseded by this one.
- **The leaf lock's versions are not used.** The leaf lock pins `serde_json 1.0.150`, `typenum 1.20.1` and `memchr 2.8.3`. Inside the src-tauri graph, `report_package` compiles against the src-tauri pins: `1.0.149`, `1.20.0` and `2.8.0`.
- **The native record agrees.** The recorded 8b982aa7 build ran with this same stale leaf lock (identical blob `b4e26cded5`). It did not pass `--locked`, succeeded, and left the lock untouched.

**Classification of every changed path** ([classify_changed_paths.py](_run_records/classify_changed_paths.py), [classification](_run_records/check5_changed_path_classification.json)). 820 paths changed from 8b982aa7 to 5b1c:

| Group | Paths | Can it enter the compiled native closure or the bundled dist? |
|---|---|---|
| Execution-evidence records | 784 | No |
| Recorded native input `core/reporting/report_package/Cargo.lock` | 1 | No, per the Cargo semantics and evidence above |
| Python `tests/` and `tools/validation/` | 12 | No |
| `validation/qualification/**` | 20 | No |
| Benchmark locks under `validation/benchmarks/` | 3 | No |

None of these paths lies inside the 31 src-tauri path packages, the WASM graphs or `apps/desktop`, and no native or frontend source references them. The frontend imports only `fixtures/` and `schemas/`, which are recorded roots.

Rust `include!` targets reach three files outside the recorded roots: one under `validation/benchmarks/`, one under `validation/witness/` and one under `validation/evidence/`. All three are test-only (`#[cfg(test)]` or `tests/`), and none changed.

## 6. Conclusions

**(a) Correct and minimal.** Each lock is additive and equals Cargo's resolve exactly. Every pin already exists in the repository with the same checksum. The repair reproduces byte-for-byte from e650. CI-equivalent fetch passes, including from a cold cache, and the four crates' tests pass.

**(b) Compiled closure unchanged.** The src-tauri root lock and manifest are unchanged. The resolve is invariant to the leaf lock, and the compiled `report_package` outputs are byte-identical. The dist inputs, `package-lock.json` and the WASM roots are unchanged. The only differing recorded input cannot enter the closure.

**(c) What the native manifest can still claim.** It remains a correct record of 8b982aa7 and the executable `f68dc4de906eb5e8c96bb72c8736b7a5a9afec09ba533ada7ac676223fd36dcc`. It also matches e650 in all 1,092 paths.

For 5b1c or any later head containing this repair, it may state only:

> 1,091 of 1,092 recorded inputs are byte-identical; `core/reporting/report_package/Cargo.lock` differs (`946819d8…` → `75d36eac…`) and is outside the compiled native closure (this review).

It must **not** be claimed to match all candidate bytes, and it must not be re-recorded as if built from 5b1c. A macOS rebuild was not possible on this Linux host, so this review does not claim a byte-identical executable from a 5b1c rebuild.

**(d) No native witness must be repeated because of these repairs.** Nothing that is compiled, bundled or embedded changed. This is the owning evidence's own criterion: build inputs unchanged in the compiled closure. The witness stays bound to its actual 8b982aa7 executable.

## Carry-forwards (none blocking)

1. **Next readiness wording and script.** Earlier records say "all 1,092 native inputs match". Those statements are true for e650, the candidate they checked:
   - [readiness precheck](../_run_records/readiness_precheck/RETURN.md);
   - `ROOT_VALIDATION_UNION`;
   - `RECORD_PORTABILITY_REVIEW`.

   They are false for 5b1c. The precheck's `reconcile_coverage.py` asserts all 1,092 recorded hashes equal the candidate's blobs, so run against any head with this repair it **will fail** on this one path. The final readiness review should account for this path explicitly, citing this review. It must not edit the recorded native manifest or drop the assertion wholesale. The four lock paths also need to be added to its maintained-path coverage, and this review is that coverage.
2. **Concurrent browser repair.** The worktree currently has uncommitted changes to `apps/desktop/src/styles.css` and to e2e files. `styles.css` is bundled into `dist`, which Tauri embeds in the executable. This review covers 5b1c only. If `styles.css` or any other `apps/desktop/src`, `core` or WASM input changes in the next frozen revision, the compiled native closure changes. The native-reuse question must then be reassessed at that revision; this review does not clear it.
3. **Tests not run here.** Hosted numerical CI never reached its test step for this candidate. The other 35 crates' tests at the next head remain for hosted CI; running them was outside this brief.

The historical `provenance/build-artifacts/*Cargo.lock` copies already differed at e650. They are not CI inputs and need no action.

Standard claim fence applies.
