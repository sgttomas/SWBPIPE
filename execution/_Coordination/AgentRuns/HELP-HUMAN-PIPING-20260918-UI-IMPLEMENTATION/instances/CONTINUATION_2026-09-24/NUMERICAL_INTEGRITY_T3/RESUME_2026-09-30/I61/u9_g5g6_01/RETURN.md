# I61 RETURN: U9 gates G5 (product T9) and G6 (both-entry gate, parts 1 and 2)

**Verdict: G5 PASS. G6 part 1 PASS. G6 part 2 PASS.** No output differs where identity is required, and gate_check passes on every side, so nothing triggered a stop.

**Who and when:** I61 (TASK, Type 2), under ROOT's explicit grant for these two gates (solver at scale for G5 and G6 only; the I20/KF2 precedent), 2026-10-04 from 22:28Z to 23:22Z.

**Revisions:**
- **Base M** = `5fdc5ab6012ddb50ecf286621a291b4da577405e`.
- **Candidate C** = `6b9bb19a5f66143101b6364757ecdfa12da455ef`, the PR head when the grant was given.
- **C2** = `92a5a9da1c24517068031c8ccf4fab80b444a910`, the moved PR head. Its only product change is the reordered `fn integer` in `result_export/src/source_blocks.rs`; its other two files (`tools/ci/e2e_plan.py`, `tests/test_ci_e2e_plan.py`) are outside every gate input.
- **Which candidate each gate used:**
  - T9 ran on C, then again on C2;
  - part 1 ran on C, then again on C2 as ROOT directed;
  - part 2 runs on C2.

**Host:**
- one job of mine at a time;
- my own targets under WT/targets/i61-u9g5/ and i61-u9g6/;
- memory guard PID 5387 running;
- rustc 1.97.1, the default toolchain;
- no Git writes (reads used `GIT_OPTIONAL_LOCKS=0`; `hash-object` ran without `-w`);
- no DEC-025 and nothing native;
- no machine paths in these records.

**Trees:** `git archive` of `projects/chirality-piping`, with `execution/` excluded, for M, C and C2. Every extracted file's `git hash-object` equals its revision's blob: 2,891, 2,950 and 2,950 files, 0 mismatches.

## G5: product T9 (`t9/`)

**Method:** exactly KF2 B's.
- S11-K's `fixdiff_main.rs`, sha256 `ec089c1d…`, unchanged.
- Built `--release --offline`, without `--locked` (U9 decision 10). The lock is copied from each tree's PP lock; cargo adds only the harness package (`lock_diff_*.txt`, 9 added lines each). The trees' locks are unchanged.
- Run over `core`, `fixtures` and `validation`, plus F1b's extra corpus. The corpus inputs equal KF2's hash list.

**Results:**
- **112 of 112 common outputs are byte-identical,** base against C.
- **Exactly 2 outputs are added,** both from the milestone request the PR adds: `fixtures/product_preview__rf_skew_t_cant_off_122_r1e-04.request.json.{sparse_interactive,dense_scrutiny}.out`.
- **Each added output is the ordinary value-route bytes.** It is byte-identical to the pretty serialization of that request's value-route envelope, dumped in G8's Stale build. The compact serialization of the same envelope has the sha256 of G8's Stale sweep `value_mode` row: sparse `9c7ec1a1…`, dense `21ca629c…` (`value_tie_sha256.txt`).
- **Extra corpus: 16 of 16 identical.**
- **C2 reproduces C exactly:** all 114 outputs and all 16 extra outputs (`sha_cand92.txt`, `extra_sha_cand92.txt`).
- **RESULT PASS** (`t9_compare.txt`).

## G6 part 1 (`gate/part1/`, `SUMMARY.txt`)

**Method:** exactly KF2 B's.
- The full-envelope `t3_p1_probe`: `main.rs` `cd1052f7…`, with the calibration's 6 GiB heap cap.
- Built `--release --offline`. Probe lock diffs: the base gains the path packages result_export and t3_p1_probe; C and C2 gain only t3_p1_probe. No registry entry changes.
- `gen.py` produces 223 files, all matching the calibration's hashes.
- 884 runs per side, with G1's `gate_run_base_full.py`, `run.py` and `compare.py` unchanged.
- K-D5's `gate_check.py` with the empty exception lists.
- KF2's `compare_gate_kf2.py`, unchanged.

**Results:**

| Check | Result |
|---|---|
| gate_check (base, C, C2) | **PASS**, 764 evaluated, 332 trusted, **0 trusted breach triples** |
| Base vs C, base vs C2 | **884/884 identical** in outcome, ok, exit, summary and full envelope sha256, and error text. The 764 gate rows are identical. RESULT PASS on both |
| `index_cand` vs `index_cand92` | Byte-identical |
| `diff -rq` | `full/` and `envelopes/` 818 identical; `stderr/` 884 identical; `stdout/` differs only in the probe's timing field `run.solve_seconds` |
| Heap-cap aborts | **0** |
| Timeouts | 0 |
| Exit codes | 0 on every run |
| Outcomes, every side | solved 676, refused_blocked 142, refused_capture 64, refused_error 2 |

The `full/` and `envelopes/` list hashes also equal KF2 B's (2026-09-30), so those 818 published envelopes have not changed since KF2.

## G6 part 2 (`gate/part2/`)

**The runs:** the four dense 1,000-member runs (RF-LARGE-CHAIN-n01000-ROT and RF-LARGE-TREE-n01000-AX, `dense_scrutiny`, both entries), on base M and candidate C2.

**Method:**
- I13's `gate_part2.py` (`e43c662b…`, F1b gate 2), unchanged: base then candidate per run, one run at a time, `TIMEOUT_LARGE` 1,800 s, with its load wait.
- The 6 GiB heap cap stays as recorded: `rlimit_as_bytes` 6,442,450,944 in every record.
- **Added outside the driver** (`run_part2.sh.txt`, `foreign_jobs.sh.txt`, `overlap.py`): a wait for a quiet host before starting, and a watcher sampling every 5 s for foreign cargo, rustc, test and probe processes. Each run's interval is then matched against those samples.

**Three executions, as the grant requires ("if a timing run overlaps another job, rerun it and record both"):**

| Execution | Window | Quiet wait | Foreign jobs seen | Overlapping runs |
|---|---|---|---|---|
| `part2_run1` | 22:49:30Z–22:55:53Z | 3 polls | 60 of 76 samples | 7 of 8; only base CHAIN captured was clean |
| `part2_run2` | 23:02:17Z–23:08:29Z | 8 polls | Real cargo jobs at 23:05:36Z–23:06:57Z, plus 1 false positive (the gate's own exiting probe, shown as `(t3_p1_probe)`, excluded thereafter) | 3 (base TREE captured, cand TREE captured, base TREE typed) |
| `part2_run3` | 23:12:50Z–23:18:59Z | 8 polls | **0 of 73 samples** | **None** |

- **Run 1's foreign jobs** were RV95/RV89 cargo test, pytest and vitest, and the runner's own summary misreported them. Its record line said "0 of 76" because the watcher wrote a literal `\t`. The correction is recorded in `part2_run1/` (`outcomes.txt`, `part2_summary.txt`, `overlap_by_run.tsv`), and the watcher was fixed.

**Results:** every execution of every run passes (`part2_combined.tsv`, `part2_combined.py`).
- **Every run ended in 45.4–52.8 s**, against the 1,800 s limit: 24 executions, none timed out, exit 0.
- **Run 3, the clean execution:** all 8 runs ended in 45.8–46.2 s.
- **Outcomes:** every execution, base and candidate, is `refused_blocked`, `MODEL_INCOMPLETE`, no results, one blocking `NUMERICAL_INTEGRITY_UNRESOLVED` (the dense factor's refusal).
- **Full envelopes:** base and candidate are byte-identical in every run and every execution: `68cf1929…`, `cb611210…`, `7a3d32b4…`, `9384c225…`. These equal KF2 B's part-2 envelopes.
- **Every run has at least one clean execution** (0 foreign samples), listed per run in `part2_combined.tsv`.
- **RESULT PASS.**

## Records

- **`t9/`:** sha lists for base, C and C2, the extra-corpus lists, lock diffs, harness manifests and binary hashes, run logs, `run_t9.sh.txt`, `t9_compare.txt`, `value_tie_sha256.txt`.
- **`gate/`:**
  - `scripts_and_inputs_sha256.txt`, probe manifests, lock diffs, build summaries and binary hashes;
  - `gen.log` and `gen_out_sha256_regenerated.txt`;
  - the runner scripts, `foreign_jobs.sh.txt` and `uncommitted_sha256.txt`;
  - `part1/` (SUMMARY, the gate_check results and logs, the comparisons, indexes, driver logs, schedules, host samples) and `part2_run{1,2,3}/`, with `part2_combined.tsv`.
- **Bulk, uncommitted, with hashes and sizes in `gate/uncommitted_sha256.txt`:** WT/scratch/u9_t9/ (the trees and outputs) and WT/scratch/u9_gate/ (`part1_{base,cand,cand92}/`, each `runs.jsonl` about 606 MB, plus `part2_run{1,2,3}/`). Nothing was pruned.
- `SHA256SUMS`.
