# Mac gate baseline of main `e7d930d49`, part 1

- **Purpose:** the baseline for F1b's both-entry gate on this Mac (ROOT's F1b ruling Q13). Mac outputs are compared only with Mac outputs, never with Linux records.
- **Run by G1,** a Type 2 TASK that ROOT dispatched directly as a background subagent, on 2026-09-28 from 13:23:00Z to 13:28:02Z. The host is `aarch64-apple-darwin` with rustc 1.97.1.
- **Method:** the platform calibration's (`PLATFORM_CALIBRATION_MAC/RECORD.md` §2):
  - P1's probe `8dc727f4…` with the 6 GiB heap cap appended. The built source is `404e1ff0…`, identical to the calibration's, and the binary is `7e5150a6…`;
  - P1's `gen.py`, which produced 223 of 223 files matching `gen_out_sha256.txt`;
  - `run.py`'s `run_one`, unchanged, and K-D5's `gate_check.py` `8ad89fca…`.
- **Scheduling:**
  - The 24 runs at 10,000 members ran first, two at a time and alone. Each aborted at the heap cap in under 1 s.
  - The other 860 runs used 4 workers.
  - Part 2, the four dense 1,000-member timeouts, is **not** here. It runs interleaved, base and candidate, at F1b's gate.
- **Result:** PASS.
  - 764 runs evaluated, 328 trusted, 0 trusted breach triples, 0 violations (`result_part1_base_e7d930d49.json`).
  - Against the calibration's part 1 on `649162522`, there are 0 differences in ok/ERR, exit code, outcome, and the gate rows' outcome, quality, standing, trusted flag and breach count (`comparison_vs_calibration_649162522.json`).
  - **Six runs differ in message text only:** RF-RANGE-{CHAIN,CONT,SKEW}-LEF-large, typed, both modes. Each is a `SOLVER_SYSTEM_BLOCKED` message, "computed local stiffness must be finite, got inf" on `649162522` against K2a's named range refusal on `e7d930d49`. That is K2a's intended change.
- **For F1b's C1 list:** RF-RANGE on the Mac base has 32 cases and 128 runs. Of those, 62 runs over 20 cases are refused_blocked:
  - `NUMERICAL_INTEGRITY_UNRESOLVED`: 22;
  - `PIPE_ELEMENT_INPUT_INVALID`: 30;
  - `SOLVER_SYSTEM_BLOCKED`: 10, typed only.

  The rest are 38 refused_capture and 28 solved.
- **Scope limit, for F1b's gate:** each envelope file holds the exact bytes of P1's `run.envelope`, which is **P1's summary of the product envelope, not the full `MechanicsEnvelope`**. Raw stdout also carries `solve_seconds`. A byte comparison on these files covers what P1 summarizes. ROOT rules at F1b's gate whether a full-envelope hash is also needed.
- **Files:**
  - `README.md` (G1's method, commands, times, load and guard excerpt);
  - the result and the comparison;
  - `envelope_sha256.tsv` (794 envelopes);
  - `schedule.tsv` and `host_samples.tsv`;
  - `scripts/*.py.txt`;
  - `uncommitted_sha256.txt`, the digests of the large files kept in ROOT's scratch.
- **The memory guard** logged no event. Load peaked at 9.28, and available memory stayed at 95–96%.
