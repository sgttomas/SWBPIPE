RV19 run records for REVIEW/K4_REVIEW.md (slice K4, PR #1054, head 7d8fa9c0e).

Placeholders: <wt> is the T3 worktrees root, <scratch> the session scratch
folder, <repo> the repository checkout that holds <wt>, <home> the user's home
and <python> the Python installation. Nothing else was edited; trailing
whitespace was stripped. SHA256SUMS covers every file here (run
`shasum -a 256 -c SHA256SUMS` from inside this folder).

checks/
  revisions.txt          PR state, heads and commits (read-only Git and gh)
  merge_check.txt        the merge 8f8023a20 against main's delta
  kernel_only_scan.txt   visibility, callers, write set, exact_sum.rs diff
  records_check.txt      K4's _run_records and retained_k4 SHA256SUMS; scans
  fk_full.log            FK's full suite from git archive 7d8fa9c0e (389 pass)
  gen_check.log          gen_k4_vectors.py --check (23 of 23 OK)
  build_warnings.log     FK lib built with RUSTFLAGS=-D warnings (passes)
  rustfmt.log            rustfmt --check on K4's files (empty: clean)
  emu7_vs_gen.py.txt     GEN's schedule against R7's emulation (run_controls7)
  emu7_vs_gen.txt        its output: 59 of 59 controls agree
  toolchain.txt          toolchain, host and host rules
  memguard.txt           the memory guard's log (two start lines; no kill)
probes/
  rv19_probe.rs.txt      RV19's probe module (mounted in adaptive.rs of a copy)
  probe_ovf.log          RV19-1: OVF-ROT-928 (false claim) and OVF-ROT-900
  probe_dump.log         every selected control and combination, row by row
  probe_coverage.log     compare_honest's coverage (RV19-2)
  probe_station.log      RV19-3: a combination of different station fractions
  rv19_group_models.txt  RV19-4: support groups with directional springs
  probe_groups.log       their publication
  rv19_tiny_models.txt   RV19-6: TINY-S-995 and TINY-S-900 (S* below and above 2^-988)
  probe_tiny.log         their publication
oracle/
  rv19_oracle.py.txt     RV19's independent honesty oracle (1600-digit solve)
  oracle_all.log         115 selected publications (109 cases, 6 combinations)
  oracle_ovf.log         the OVF-ROT pair
  oracle_groups.log      the GROUP-DIR pair
  oracle_tiny.log        the TINY-S pair (RV19-6)
  ovf_models.txt, rv19_group_models.txt, rv19_tiny_models.txt   the probe models in GEN's format
fix/
  fix_probe_adaptive.diff  the fix probed for RV19-1 (scales_at skips rows the
                           candidate cannot publish), plus the probe mount
  fix_probe_tests.diff     the two test call sites adjusted to the signature
  fixprobe_k4.log          with the fix: 118 K4 tests (all but the N5 streams),
                           45 of K3 and 3 probes pass (166)
  fixprobe_ovf.log         OVF-ROT-928 withheld (Unresolved Ceiling) with it
mutations/
  rv19_mut.py.txt        the harness (clean git archive and fresh target each)
  results.jsonl          NONE and RV19-M1 to M8
  run_a.log, run_b.log   the two campaign logs
  logs/                  each run's cargo test output
