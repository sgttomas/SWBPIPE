# Mac gate baseline, part 1, on main `e7d930d49`, with the full-envelope probe variant (G1, 2026-09-28)

This is G1's re-run of the base's gate part 1 with the full-envelope probe variant. ROOT asked for it in "F1b: rulings on I13's A2" (`ROOT_RULINGS_V1.md`, numerics `52ead31ed`): P1's `run.envelope` summary omits published bytes, so C3 byte identity is judged on a full-envelope hash from now on.

- **Scope.** Part 1 only. Part 2 (the 4 dense 1,000-member timeouts) runs interleaved at the gate.
- **Status.** Uncommitted scratch, a Mac-only record. No Linux comparison.
- **First run.** G1's first run is `<wt>/scratch/gate_base_e7d930d49/`, with `runs.jsonl` `8525c06d…` and its own README. This run repeats its method with only the probe changed.
- **Placeholders.** `<F>` = `<wt>/scratch/gate_base_e7d930d49_full`. `<G>` = `<wt>/scratch/gate_base_e7d930d49`. `T3/` = `P/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/NUMERICAL_INTEGRITY_T3/`.
- **Delegation.** ROOT (HELP_HUMAN) dispatched G1, a Type 2 TASK, as a Claude Code background subagent started through the Agent tool. G1 delegated nothing.

## Verdict

- **Gate check (K-D5's `gate_check.py` `8ad89fca…`, unchanged): PASS.** 764 runs evaluated, 328 trusted, 0 trusted breach triples (captured 0, typed 0), 0 violations. This is the same verdict as the first run. The two `gate_check` results are equal in every field, and in all 764 rows compared as sets.
- **Against the first run: 0 differences in 884 of 884 runs** (`comparison_vs_first_run_8525c06d.json`). Compared per (case, mode, entry):
  - the `run.envelope` summary bytes: every `envelopes/<key>.json` is byte-identical, 794 of 794, and present for the same runs;
  - `compare.classify` outcome and reason;
  - ok (794 in both), exit code, `timed_out` and `stderr_tail`;
  - the error text of the 66 refused runs;
  - the whole parsed probe line with `solve_seconds` and `envelope_sha256` removed;
  - the gate rows' outcome, quality, standing, trusted flag and breaches.
- **Full-envelope hashes: 794,** one `run.envelope_sha256` for each ok run and none for the 90 others. 535 of the hashes are distinct (captured and typed, or sparse and dense, sometimes publish identical envelopes).
  - All 794 full envelopes were written to `full/<key>.json` (1.0 GB).
  - Each file's sha256 equals the probe's reported `run.envelope_sha256`. The driver checked this at run time, and `compare_with_first_run.py` checked it again from disk.
- **Memory guard: no event.** Details are under "Memory guard" below.

## The variant (ROOT: diff it first)

- **Received:** `<wt>/scratch/i13/a2/probe_cand/`, copied unchanged to `variant_as_received/`:
  - `src/main.rs`: `cd1052f7a4ac4cb8f3a13b38c899081cb2cd39f099991944157226be6939f232`;
  - `Cargo.toml`: `b90405c5…`;
  - `Cargo.lock`: `d7bdd546848a0c97f73373b71820abccacb5f283bdab1c6953609ef9a20bf744`.
- **Diffed against G1's `404e1ff0…` source** (P1's `8dc727f4…` plus a newline plus the calibration's heap cap). The full diff is in `variant_diff.txt`, and the only changes are:
  1. **A new function `full_envelope(&MechanicsEnvelope) -> String`**, inserted before `main`.
     - It computes `serde_json::to_vec(e)`. If `T3_FULL_ENVELOPE_PATH` is set, it writes those bytes to that path.
     - It returns the lowercase hex sha256 of the bytes (`sha2::Sha256`).
  2. **In the two `ok: true` branches** (typed and captured), `, "envelope_sha256": full_envelope(&e)` is appended to the `run` object. The error branches are unchanged.
  3. **`Cargo.toml`** adds `sha2 = "0.10"`, and the path dependencies point at the variant author's tree. **`Cargo.lock`** adds `"sha2"` to `t3_p1_probe`'s dependencies. The `sha2 0.10.9` registry package was already in the lock through `result_export`, and no version changes.
- **Checked mechanically:**
  - removing the new function and the two appended keys gives back the `404e1ff0` source exactly;
  - `summarize()`, and so `run.envelope`, is unchanged;
  - `run.envelope_sha256` sorts after `run.envelope` in serde_json's key-sorted output, so the summary span is found as before;
  - `run.py`'s `run_one` and `compare.classify` read neither `envelope_sha256` nor the file.
- **The full envelope** is the product's complete `MechanicsEnvelope` as `serde_json::to_vec` serializes it: compact, in the struct's field order. The summary is P1's filtered view of it.

## Build
- **Tree:** a fresh `git archive --format=tar e7d930d49 projects/chirality-piping | tar -x -C <F>/tree`. It is identical to the first run's tree (`diff -r`). The piping subtree is `d3893532…`; `tree/` is not hashed.
- **Probe crate** (`<F>/probe/`):
  - `src/main.rs` is the variant's (`cd1052f7…`);
  - `Cargo.toml` is the variant's, with the two path dependencies pointed at `<F>/tree/...` and otherwise identical (sha256 `e9da21dc…`; it contains machine paths and is a build input only);
  - `Cargo.lock` is the variant's `d7bdd546…`, unchanged by the build.
- **Command** (16:05:58Z–16:06:39Z, 40.98 s; two other implementers' cargo processes were running at the start):
  ```
  cd <F>/probe
  RUSTUP_TOOLCHAIN=1.97.1 RUSTUP_AUTO_INSTALL=0 CARGO_INCREMENTAL=0 \
    CARGO_TARGET_DIR=<wt>/gate-base-full-target cargo build --release --offline -j 8
  ```
  The target is a sibling of the first run's `<wt>/gate-base-target`, which is left intact.
- **Binary:** `<wt>/gate-base-full-target/release/t3_p1_probe`, copied to `build/t3_p1_probe`, sha256 **`577b10d498e599c6ef8763d4697dfbdd85a83ccfa8f151c906b9fe046569be6a`**.
- **Platform:** `aarch64-apple-darwin`, rustc 1.97.1 (8bab26f4f 2026-07-14).

## Requests and tools
- **Requests:** re-generated with `<VENV>/bin/python -B scripts/gen.py inputs/references.py inputs/references.json inputs/fixtures.json gen_out` (16:06:59Z–16:07:50Z; Python 3.13.14). All 223 files match `T3/PLATFORM_CALIBRATION_MAC/gate/gen_out_sha256.txt`.
- **Tools and inputs** in `scripts/` and `inputs/` are byte-identical to the first run's, checked with `cmp`:
  - `gen.py` `c515ec63…`, `run.py` `8d9a32a3…`, `compare.py` `1c91b6c6…`, `gate_check.py` `8ad89fca…`;
  - the calibration driver `5a2029a6…`, and the first run's driver;
  - `references.json` `7b176dbb…`, `references.py` `80d473a7…`, `fixtures.json` `c061d734…`;
  - the empty `S11_EXCEPTIONS.json` `138515b3…` and `FORMATION_EXCEPTIONS.json` `0e110b4b…`.

## Run
- **Driver:** `scripts/gate_run_base_full.py`, a copy of the first run's `gate_run_base_envelopes.py` with two additions. Everything else is unchanged, including the case set, exclusions, timeouts, ordering, record fields, the pass-through capture and the phases. The additions:
  1. The same pass-through `Popen`, used for `run.py` only, gives each child the environment variable `T3_FULL_ENVELOPE_PATH=<F>/full/<key>.json`. The rest of the environment is inherited unchanged, and `run_one` passes no `env` of its own.
  2. After each run, the driver hashes that file and checks it against `run.envelope_sha256`. The results are the `full_*` columns of `schedule.tsv`.
- **Command:** `<VENV>/bin/python -B scripts/gate_run_base_full.py <wt>/gate-base-full-target/release/t3_p1_probe gen_out . 4`, with output in `gate_part1.log`.
- **Worker counts:**
  - **Phase A:** the 24 runs at 10,000 members, 2 workers, nothing else of the gate running. All 24 aborted at the heap cap (exit −6, `memory_refused`), as in the first run.
  - **Phase B:** the other 860 runs, 4 workers.
- **Times (UTC, 2026-09-28):** start 16:09:02Z; phase A 16:09:02Z–16:09:13Z; phase B 16:09:13Z–16:14:46Z; end 16:14:46Z. **Wall time 344 s.**
  - A 16-run smoke test (4 small cases, 16:08:44Z) wrote into `<F>/smoke/`. It was deleted before the run and is not part of this record.
- **Host load** (`host_samples.tsv`, every 5 s, 68 samples):
  - 1-minute load 12.06 at start; **peak 17.38 at 16:14:14Z**; mean 12.59. Two other implementers were running suites.
  - `kern.memorystatus_level` 94–96%.
  - At most 4 concurrent probes.
  - Other implementers' `cargo` and `rustc` were present in 21 of 68 samples (at most 2 of each).
  - No timing is compared.

## Memory guard
- The guard `<wt>/guard/memguard.sh` (floor 35%, pid 5387) ran throughout. It was watched live, together with the run log, and checked afterwards.
- `<wt>/guard/memguard.log` is unchanged (2 lines, sha256 `79e2ce8eb2bd8f4a6432beb5b919cb46f37c22046c08a057b21a36183801cc41`, no `KILLED` line):
  ```
  2026-09-27 19:36:07 memguard start floor=35% pid=5266
  2026-09-27 19:36:20 memguard start floor=35% pid=5387
  ```

## Files
- `runs.jsonl`: 884 records, sha256 **`9139140c80a6b5233ef67c01b8eaec0df85e7c4ed44df41249cbb73ff902a38d`**. It differs from the first run's only by `solve_seconds`, `envelope_sha256`, `wall_seconds`, `peak_rss_kib`, `memorystatus_level_end` and the completion order.
- `full/` (794 files), `envelopes/` (794 summary spans), `stdout/` (884), `stderr/` (884).
- `envelope_sha256.tsv`: one sorted row per run with case, mode, entry, outcome, ok, exit code, summary sha256, **full sha256 and size**, error-text sha256 and stdout sha256. It was cross-checked with 0 inconsistencies.
- `schedule.tsv`, `host_samples.tsv`, `gate_part1.log`, `gen.log`, `gen_out/`.
- `result_part1_base_e7d930d49_full.json`, `gate_check.log`, `comparison_vs_first_run_8525c06d.json`, `compare.log`.
- `probe/`, `variant_as_received/`, `variant_diff.txt`, `build/t3_p1_probe`, `build/build.log`, `scripts/` (including `compare_with_first_run.py`), `inputs/`.
- `SHA256SUMS`: every file under `<F>` except `tree/` and itself.
- `<wt>/gate-base-full-target` is kept for part 2. ROOT may prune it, and `<wt>/gate-base-target`, afterwards.

## Not done
- Part 2.
- Any timing comparison.
- Any Linux comparison.
- Any Git write other than the read-only `git archive`.
- Any write outside `<F>` and `<wt>/gate-base-full-target`.
- `<wt>/scratch/i13/` was read only.
