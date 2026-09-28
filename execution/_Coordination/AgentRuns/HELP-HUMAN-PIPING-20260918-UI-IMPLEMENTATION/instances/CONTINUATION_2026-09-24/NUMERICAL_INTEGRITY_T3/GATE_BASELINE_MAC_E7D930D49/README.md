# Mac gate baseline, part 1, on main `e7d930d49` (G1, 2026-09-28)

This is the Mac base run of T3's both-entry gate, part 1, for F1b (I13, ruling Q13(a)). It is uncommitted scratch, kept and hashed here for F1b's per-run comparison.

- Dispatched by ROOT (HELP_HUMAN) to G1, a Type 2 TASK, as a Claude Code background subagent started through the Agent tool. G1 delegated nothing.
- Part 2 (the 4 dense 1,000-member timeouts: RF-LARGE-CHAIN-n01000-ROT and RF-LARGE-TREE-n01000-AX, dense, both entries) was **not run**. It runs interleaved, base and candidate, at the gate.
- This is a Mac-only record. No comparison is made with any Linux record.

`T3/` = `P/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/NUMERICAL_INTEGRITY_T3/`, read from `<wt>/numerics` at `28bb8dfa8`. `<G>` = `<wt>/scratch/gate_base_e7d930d49`.

## Verdict

- **Gate check (K-D5's `gate_check.py`, unchanged): PASS.** 764 runs evaluated, 328 trusted, 0 trusted breach triples (captured 0, typed 0), 0 violations, 0 pinned triples (both exception lists are empty). See `result_part1_base_e7d930d49.json` and `gate_check.log`.
- **Against the Mac calibration's part 1 on `649162522`** (`<wt>/scratch/calib/gate/runs_part1_mac.jsonl`, sha256 `ef8993f4…`, matching `T3/PLATFORM_CALIBRATION_MAC/gate/uncommitted_sha256.txt`; `result_part1_mac.json`, sha256 `5947a348…`):
  - all 884 runs are present in both;
  - **0 differences** in ok/ERR (794 ok / 90 not ok in both), exit code, and `compare.classify` outcome;
    - both runs have 658 solved, 136 refused_blocked, 64 refused_capture, 2 refused_error and 24 memory_refused;
  - **0 differences** in the 764 gate rows' outcome, quality, standing, trusted and breach count;
  - **0 differences** in the 24 heap-cap aborts' stderr tails.
- **Informational: 6 runs differ in probe payload content,** compared as parsed JSON values, because the calibration kept no raw bytes. All 6 are in the RF-RANGE family, and all are LEF-large typed runs:

  | Case | Modes | Outcome (both trees) | What differs |
  |---|---|---|---|
  | RF-RANGE-CHAIN-LEF-large | sparse, dense | refused_blocked, `MODEL_INCOMPLETE` | text of the one blocking diagnostic |
  | RF-RANGE-CONT-LEF-large | sparse, dense | refused_blocked, `MODEL_INCOMPLETE` | text of the one blocking diagnostic |
  | RF-RANGE-SKEW-LEF-large | sparse, dense | refused_blocked, `MODEL_INCOMPLETE` | text of the one blocking diagnostic |

  - The code (`SOLVER_SYSTEM_BLOCKED`), severity (blocking), id, status and every other field are equal.
  - The message on `649162522` reads "computed local stiffness must be finite, got inf".
  - The message on `e7d930d49` reads "range: stiffness formation outside the binary64 normal range at GJ/L: G*J (zero, subnormal or non-finite from nonzero finite operands)".
  - The other 878 runs' parsed envelopes (or error texts) are equal to the calibration's.
  - These differences are listed here, not explained. The code changed between the two trees (K2a, K1, K2b, the skew pin and K3).
- **RF-RANGE on this base (for F1b's C1 list; Mac data):**
  - 32 cases, 128 runs;
  - 62 `refused_blocked` runs over 20 cases (22 captured, 40 typed), all with mechanics status `MODEL_INCOMPLETE`. Each has exactly one distinct blocking code:
    - `NUMERICAL_INTEGRITY_UNRESOLVED`: 22 runs (10 captured, 12 typed);
    - `PIPE_ELEMENT_INPUT_INVALID`: 30 runs (12 captured, 18 typed);
    - `SOLVER_SYSTEM_BLOCKED`: 10 runs (typed only);
  - 38 `refused_capture` and 28 `solved` (4 captured, 24 typed).

  The per-run list is in `runs.jsonl` and `envelope_sha256.tsv`.
- **Memory guard: no event.** `<wt>/guard/memguard.log` was unchanged by this run (see below).

## Inputs and tool hashes (sha256)

| Item | Source | sha256 |
|---|---|---|
| probe `main.rs` as built | `T3/DETECTION/probe/main.rs.txt` + `"\n"` + `T3/PLATFORM_CALIBRATION_MAC/gate/heap_cap_appended_to_p1_probe.rs.txt` | `404e1ff08237274475820d1a53f643444d589fdb9802d656e8d151ee89f398b7` (equal to the calibration's `<wt>/scratch/calib/gate/probe/src/main.rs`) |
| P1 probe source | `T3/DETECTION/probe/main.rs.txt` | `8dc727f476ed4ade25b4d4b0310c9dcaa61649a5b1e56c94921c4091f093fcac` |
| heap cap (6 GiB) | `T3/PLATFORM_CALIBRATION_MAC/gate/heap_cap_appended_to_p1_probe.rs.txt` | `96c8a1295a78cfbb8d8f7c14bdd04c4566e1a2f5fe6ea19a3042aa1e0d6c42ae` |
| probe `Cargo.toml` | the calibration's, with the two path dependencies pointed at `<G>/tree/...` (identical otherwise, including `[workspace]`) | `d6b17f4d7ce95a9535a8ac70654285e5fc00555f43254a50d462c0c56b8600e6` (it contains machine paths; build input only) |
| probe `Cargo.lock` after the build | copied from `core/product_physics/Cargo.lock` at `e7d930d49`; cargo added the two path packages | `6ccdc17f0bd1818bd10c10b50671f58fdfc7947a55f4ba3d4ca657ffd1ff0424` (byte-identical to the calibration's lock) |
| **probe binary** | `<wt>/gate-base-target/release/t3_p1_probe`, copied to `build/t3_p1_probe` | **`7e5150a6401ed1630c072e9dbee8724caf794c2ff2f906adf1febdb382cc01b0`** |
| `gen.py` | `T3/DETECTION/scripts/gen.py.txt` | `c515ec63f8828addb88e82aa92bc4c5c0060764046bf9cbd6a82dfcd06909b5b` |
| `run.py` (`run_one`) | `T3/DETECTION/scripts/run.py.txt` | `8d9a32a34bc0602f5ba24b72c999068ca8e6ee2c770a1cbdbc217629983816f6` |
| `compare.py` | `T3/DETECTION/scripts/compare.py.txt` | `1c91b6c687fa1009dbbc9b28b9ee241fcfbab220d020203d003513f686c4318c` |
| `gate_check.py` | `T3/IMPLEMENTATION/KD5/_run_records/gate/gate_check.py.txt` | `8ad89fca7641d3e1d947772c2b9e24e2bf43f40a6b7e4ab3130a40f12c6ee96b` |
| calibration driver | `T3/PLATFORM_CALIBRATION_MAC/gate/gate_run_mac_parallel.py.txt` | `5a2029a63eb116e56fd2b3df7bff07317ede44c4ad546860e0f78373f38da8a2` |
| this run's driver | `scripts/gate_run_base_envelopes.py` (derived; see Method) | in `SHA256SUMS` |
| references | `T3/REFERENCES/references.json` / `references.py` | `7b176dbbf2296be02d8bca19c698ee56d0d5753751150a9175e0d3ad4cf89cc9` / `80d473a7351e92a2a903d233b9e633aac794aeff07f3ac41e40d25a0d94fc8a8` |
| fixtures | `validation/benchmarks/numerical_integrity/fixtures.json` at `e7d930d49` | `c061d73481d2ad137f7cef988475721681131789ec222e3e93c5ed3836b2910e` |
| exception lists (empty) | `T3/GATE/S11_EXCEPTIONS.json`, `T3/GATE/FORMATION_EXCEPTIONS.json` | `138515b3…`, `0e110b4b…` |

The copies used are in `scripts/` and `inputs/`, byte-identical to the sources above (the `.txt` suffix dropped). The calibration's working copies in `<wt>/scratch/calib/gate/` have the same hashes. They were read, not modified.

## Method

### 1. Tree and build
- **Platform:** `aarch64-apple-darwin`, rustc 1.97.1 (8bab26f4f 2026-07-14), cargo 1.97.1. The host has 18 CPUs, 128 GiB and no swap.
- **Tree:** `git archive --format=tar e7d930d49 projects/chirality-piping | tar -x -C <G>/tree`. The commit's tree is `2b44cdf06…` and the piping subtree `d3893532c4701223392383151d2323e890ee9a43`. The calibration's `main_tree` was the same kind of archive of `649162522` (piping subtree `f8286d0f…`), without `execution/`. That folder is not a build input. `tree/` is not in `SHA256SUMS`; it is reproducible from the commit.
- **Build** (13:19:30Z–13:19:52Z, 21.65 s; no cargo job of anyone else was running):
  ```
  cd <G>/probe
  RUSTUP_TOOLCHAIN=1.97.1 RUSTUP_AUTO_INSTALL=0 CARGO_INCREMENTAL=0 \
    CARGO_TARGET_DIR=<wt>/gate-base-target cargo build --release --offline -j 8
  ```
  It finished with warnings only: unused functions in `product_physics`, and one warning in `result_export`.
  - `--locked` is not used, as in P1 and the calibration. The probe root adds path packages to the copied lock, and registry versions are unchanged.

### 2. Requests
- **Command** (13:20:12Z–13:21:03Z): `<VENV>/bin/python -B scripts/gen.py inputs/references.py inputs/references.json inputs/fixtures.json gen_out`.
  - `<VENV>` is Python 3.13.14, standard library only.
  - It printed `cases 252 authorable 222 not_authorable 30`.
- **Check:** all 223 files (`cases.json` and 222 requests) match `T3/PLATFORM_CALIBRATION_MAC/gate/gen_out_sha256.txt` (`shasum -a 256 -c`: 223 OK). The sorted re-listing `gen_out_sha256_regenerated.txt` is identical to it.

### 3. Part 1 (884 runs)
- **Command:** `<VENV>/bin/python -B scripts/gate_run_base_envelopes.py <wt>/gate-base-target/release/t3_p1_probe gen_out . 4`, with output in `gate_part1.log`.
- **Taken unchanged from the calibration driver:**
  - the case set (222 authorable), the two modes and both entries;
  - the 4 part-2 dense timeouts excluded, which leaves 884 runs;
  - `run.py`'s `run_one`, imported unchanged, with `resource.setrlimit` a no-op (the probe caps its own heap at 6 GiB);
  - the timeouts (600 s, or 1800 s for 1,000 or more members);
  - the ordering by descending members;
  - the added record fields (`rlimit_as_enforced: false`, `heap_cap_bytes`, `memorystatus_level_end`).
  - Records are appended in completion order.
- **Worker counts (ROOT's rule):**
  - **Phase A:** the 24 runs at 10,000 members (the 6 RF-LARGE-*-n10000-* cases, 12 sparse and 12 dense), 2 workers, nothing else running. Each such run therefore had at most one other run alongside.
  - **Phase B:** the other 860 runs, 4 workers.
  - Phase B starts only after phase A's pool has closed.
- **Times (UTC, 2026-09-28):**
  - start 13:23:00Z; phase A 13:23:00Z–13:23:08Z; phase B 13:23:08Z–13:28:02Z; end 13:28:02Z;
  - **wall time 302 s.**
  - A 12-run smoke test of the driver (3 small cases, 13:22:41Z, 1 s) wrote into `<G>/smoke/`. It was deleted before the real run and is not part of this record.
- **The 10,000-member runs:** all 24 aborted at the heap cap in under 1 s each, as on main.
  - Each exited −6, with stderr "memory allocation of 480048 bytes failed". The tails are identical to the calibration's.
  - `compare.classify` gives `memory_refused`.
- **Host load** (`host_samples.tsv`, every 5 s, 60 samples):
  - 1-minute load: 4.43 at start (other host activity, not T3 cargo); **peak 9.28 at 13:25:18Z**; mean 7.29;
  - `kern.memorystatus_level` 95–96% throughout;
  - at most 2 concurrent probes in phase A and 4 in phase B;
  - 0 `cargo` and 0 `rustc` processes in every sample.

  Per-run `memorystatus_level_end` in `runs.jsonl` is also 95–96. Per-run start and end times are in `schedule.tsv`. No timing is compared with anything.

### 4. Envelope capture (how, and why it does not change `run_one`'s classification)
- **Capture.** `run_one` parses only the probe's last stdout line into `rec['probe']` and keeps the last 1500 characters of stderr. It keeps no raw bytes.
  - The driver rebinds the module-level name `subprocess` inside `run.py` only, to a copy of the module whose `Popen` wraps the child's stdout and stderr pipes in a pass-through reader.
  - Each `read()` returns the pipe's bytes unchanged, and also keeps a copy. `run_one`'s code is untouched and receives the same bytes, so its record is unchanged. The global `subprocess` module, which `os.popen` uses for the sysctl read, is not patched.
- **Per-run checks by the driver:** `json.loads` of the kept stdout's last line equals `rec['probe']` for every run with a probe line (860). The 24 aborts printed nothing.
- **Per (case, mode, entry)** (`<key>` = `<case>__<mode>__<entry>`):
  - `stdout/<key>.out`: the probe's complete raw stdout (884 files; the 24 aborts are empty);
  - `stderr/<key>.err`: the complete raw stderr (884 files). Every one is at most 1500 characters, so each equals its record's `stderr_tail`;
  - `envelopes/<key>.json`: the exact byte span of `run.envelope` within the probe's line (**794 files**, one per run with `ok: true`). The line is serde_json's compact, key-sorted output. The span starts after `{"mode":…,"run":{"entry":…,"envelope":`, and its end is found by `json.JSONDecoder.raw_decode`. Each span is checked to parse to the recorded envelope.
  - The 66 not-ok runs with a probe line have no envelope. Their error text is in `stdout/<key>.out` and in `runs.jsonl`.
- **Index:** `envelope_sha256.tsv` has one row per run, sorted: case, mode, entry, outcome, ok, exit code, envelope sha256, error-text sha256 and stdout sha256. It was recomputed from the files on disk, and cross-checked against `schedule.tsv` and `runs.jsonl` with 0 inconsistencies.
- **The envelope is P1's probe summary,** not the product's complete `MechanicsEnvelope`. It keeps:
  - producer, numerical_quality, status, standing and the source-block receipt summary;
  - all diagnostics, with non-`NUMERICAL_INTEGRITY*` or `SOURCE_BLOCK*` messages cut at 600 bytes;
  - the result count, and the results of the 13 kept kinds.

  Raw stdout also carries `solve_seconds`, so compare envelope spans, not stdout bytes.

### 5. Check and comparison
- `gate_check.py gen_out runs.jsonl inputs/references.json result_part1_base_e7d930d49.json inputs/S11_EXCEPTIONS.json inputs/FORMATION_EXCEPTIONS.json` gives the result above.
- `scripts/compare_with_calibration.py` (this TASK's) writes `comparison_vs_calibration_649162522.json`, with each difference given as [calibration, base].

## Memory guard
- The guard `<wt>/guard/memguard.sh` (floor 35%, pid 5387) ran throughout.
- `<wt>/guard/memguard.log` before and after the run:
  ```
  2026-09-27 19:36:07 memguard start floor=35% pid=5266
  2026-09-27 19:36:20 memguard start floor=35% pid=5387
  ```
  It has 2 lines, sha256 `79e2ce8eb2bd8f4a6432beb5b919cb46f37c22046c08a057b21a36183801cc41`, and no `KILLED` line. It was watched live during the run and checked afterwards.

## Files
- `runs.jsonl`: 884 records, sha256 **`8525c06d5c6fb07bf0c9ad9e698aaf691756e4b166d32297eba7e1f90e7e525c`**.
- `envelopes/` (794 files), `stdout/` (884), `stderr/` (884), `envelope_sha256.tsv`, `schedule.tsv`, `host_samples.tsv`.
- `result_part1_base_e7d930d49.json`, `gate_check.log`, `comparison_vs_calibration_649162522.json`, `compare.log`.
- `gate_part1.log`, `gen.log`, `gen_out/`, `gen_out_sha256_regenerated.txt`.
- `probe/` (the source and manifest as built), `build/t3_p1_probe` (a copy of the binary), `scripts/`, `inputs/`.
- `SHA256SUMS`: every file under `<G>` except `tree/` and itself.
- `<wt>/gate-base-target` (103 MB) is kept, because part 2's base runs need this binary. ROOT may prune it once part 2 is done.

## Not done
- Part 2.
- Any timing comparison.
- Any Linux comparison.
- Any Git write other than the read-only `git archive`.
- Any write outside `<G>` and `<wt>/gate-base-target`.
