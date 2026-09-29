# Platform calibration on the owner's Mac (ROOT, 2026-09-28)

`HANDOFF_2026-09-28_TO_LOCAL.md` §5 step 2 asks for a platform calibration before any T3 comparison on the Mac:
- build main on the Mac;
- run T9 and a gate sample through both entries;
- compare both with the Linux records;
- report byte-identity, or name each difference.

**Verdict: not byte-identical.**
- **T9:** 100 of 112 outputs match the Linux records byte for byte. The other 12 differ only through the platform libm, in 40 leaves. With correctly rounded libm results replayed, all 112 match.
- **Gate:** on part 1, every gate classification matches Linux: all 764 evaluated rows, and ok/ERR on all 884 runs.
- **Consequence:** for every later comparison, keep main and the candidate on the same machine, as the handoff requires. Never compare a Mac output with a Linux hash record.

Host:
- `aarch64-apple-darwin`, with rustc 1.97.1 (`t9/toolchain.txt`);
- no swap (see §4).

Main is `649162522`. Its piping tree equals `134eefc24`'s: no commit in between touches `projects/chirality-piping/{core,fixtures,validation,schemas}`.

## 1. T9 (the committed-fixture diff)

### Method
- **Harness:** S11-K's `fixdiff_main.rs` (sha256 `ec089c1d…`), unchanged.
- **Build:** against a `git archive` of main, `--release --offline`, with the lock copied from `core/product_physics`.
- **Inputs:** every committed JSON request or model under `P/core`, `P/fixtures` and `P/validation`, in both modes: 112 outputs.
- **Reference:** F1a's Linux candidate hashes (`IMPLEMENTATION/F1A/_run_records/fixture_diff/output_sha256_candidate.txt`, equal to K-D5's).

### Result: 100 of 112 byte-identical (`t9/output_sha256_main_mac_native.txt`)

The 12 differing outputs, 40 leaves in all, each with its Mac and Linux value, are named in `t9/platform_differences.txt`. In summary:
- **10 outputs** differ in 1 or 2 leaves each, by 1 ulp. Each leaf is a `hypot`-derived magnitude:
  - `support_reaction_force_magnitude_v2`;
  - `support_reaction_moment_magnitude_v2`;
  - `displacement_magnitude`.
- **`validation/…/load_reference/coefficient_definition`:** 7 leaves (sparse) and 21 (dense).
  - One `expm1` call (the logarithmic thermal strain at 8.75e-4) is 1 ulp different. It moves `thermal_strain`, `total_eigenstrain` and `resolved_eigenstrain` by 1 ulp.
  - In dense mode, the change propagates into cancellation-level values. Reactions and axial forces read 0 or −0 on the Mac against ±1.16e-10 N on Linux. Axial stresses read 0 or −0 against 1.95e-14 or 1.95e-8 Pa. One M03 residual row in a diagnostic message differs the same way.
  - The sparse mode also differs in one displacement and one magnitude.
- **No other field differs:** no standing, quality, code, id or row.

### Attribution: libm only

- **Logging.** A harness variant defines every libm entry the product can reach, including macOS's fused `__sincos_stret`, as a shim over the platform function (`t9/libm_shim_appended_to_s11k_harness.rs.txt`).
  - With logging on, its outputs are byte-identical to the native build's (the shim is transparent).
  - The 112 runs reach only three libm functions: `hypot` (334 distinct arguments), `exp` (6) and `expm1` (10).
- **Correctly rounded references** (exact rationals, 100-digit Decimal; `t9/cr_table.py.txt`, `t9/cr_table.stdout.txt`):
  - macOS `hypot` is 1 ulp off on 16 of 334 arguments;
  - `expm1` is 1 ulp off on 1 of 10;
  - `exp` agrees on all 6.
- **Replay.** Replaying the correctly rounded results for those calls reproduces **all 112 Linux hashes** (`t9/output_sha256_main_mac_cr_libm_replay.txt`).
  - So the Linux records agree with correctly rounded libm on every call these fixtures make, and all other arithmetic is byte-identical across the two platforms.
  - This is shown for these inputs only. Other inputs may reach other libm functions, for example trigonometry through curved bends, with their own platform differences.

## 2. The gate sample (part 1, both entries, both modes)

### Method
- **P1's tools:** probe `main.rs` (sha256 `8dc727f4…`), `gen.py`, `run.py` (`run_one`, imported unchanged), `compare.py` and K-D5's `gate_check.py`.
- **Inputs:** the frozen references (`references.json` `7b176dbb…`) and the empty `GATE/S11_EXCEPTIONS.json` and `GATE/FORMATION_EXCEPTIONS.json`. The generated `gen_out` is hash-listed in `gate/gen_out_sha256.txt` (222 requests).
- **Part 1** means all 884 runs except the 4 known dense timeouts, which are part 2. Part 2 was **not run**.

### Two deviations from the Linux method, both forced by macOS

1. **Memory cap.** `RLIMIT_AS` cannot be set on macOS.
   - In its place, the probe caps its own heap at 6 GiB with a counting global allocator (`gate/heap_cap_appended_to_p1_probe.rs.txt`). A refused allocation aborts as on Linux ("memory allocation of N bytes failed").
   - **Why the cap classifies runs as `RLIMIT_AS` did:** on Linux, the largest successful run peaked at 3.53 GiB. Every refused run sat at the 6 GiB limit: the 24 runs at 10,000 members.
2. **Parallel driver.** `gate/gate_run_mac_parallel.py.txt` runs with 6 workers. It is a calibration driver, not a gate of record, and timing was not compared.

### Results
- **Gate check:** 764 runs evaluated, 328 trusted, 0 trusted breach triples, **PASS** (`gate/result_part1_mac.json`).
- **Against Linux K-D5 part 1** (`IMPLEMENTATION/KD5/_run_records/combined/gate/part1_result.json`, candidate `2409de83e`):
  - **all 764 rows are identical** in outcome, quality, standing, trusted and breaches;
  - the files differ only in row order, which comes from parallel completion.
  - F1a, the only product change since then, changes message text only.
- **All 884 runs,** including the 120 outside the frozen-reference set, agree on ok/ERR with the Linux log (`gate_part1.log`).
- **Scope of the claim:** no gate classification is platform-sensitive on this corpus. Published bytes on the gate corpus were not compared, because the Linux runs' `runs.jsonl` were never committed.

## 3. Suites on Mac main

See `suites/SUMMARY.md`.

## 4. The crash, and the host rules it produced

- **What happened.** The first attempt at §2 crashed the Mac: a kernel watchdog panic from memory exhaustion, with the compressor at 100% of its limit and no swap. At the time it was running:
  - 12 probe workers with **no** memory cap (the `RLIMIT_AS` call replaced by a no-op), largest cases first;
  - a full cargo suite.

  The 10,000-member cases each allocate dense n×n arrays of about 29 GB at 60,000 DOFs. Linux had stopped each one at 6 GiB.
- **The re-run above used:**
  - the heap cap;
  - 6 workers;
  - no concurrent cargo suite;
  - a host memory guard. The guard SIGKILLs T3 processes if available memory falls below 35%. It never fired.
- **Available memory** stayed at 96% throughout.
- `TASK_BRIEFS/I8R_K1_RESUME.md` carries the resulting host rules.

## Addendum 1 (ROOT, 2026-09-28): RV9's notes N4 and N5 (records PR #1035)

- **N4(a).** §2's "all 884 runs … agree on ok/ERR" rests on `runs_part1_mac.jsonl`, which is not committed. Its sha256 is in `gate/uncommitted_sha256.txt`, and RV9 matched it and re-derived 0 mismatches. ROOT keeps the file in `<scratch>/calib/gate/` for as long as this record cites it.
- **N4(b).** §4's "available memory stayed at 96% throughout" should read **95–96%**. The per-run `memorystatus_level_end` values in that file are 95 and 96.
- **N5.** §1's summary of the `coefficient_definition` differences is incomplete. The sparse output also differs in one M03 residual-row diagnostic message: the denominator `2090073.3883936002` against `2090073.3883936`. `t9/platform_differences.txt` lists every leaf and is the complete statement. This addendum does not change the §1 counts (12 outputs, 40 leaves).
