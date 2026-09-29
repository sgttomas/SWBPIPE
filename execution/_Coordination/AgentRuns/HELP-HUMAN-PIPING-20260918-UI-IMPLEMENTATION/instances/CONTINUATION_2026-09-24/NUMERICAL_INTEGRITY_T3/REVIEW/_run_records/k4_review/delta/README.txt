RV19's delta check of K4 at a5fa0eaf7 (REVIEW/K4_REVIEW.md, "Delta check at
a5fa0eaf7"). Placeholders as in ../README.txt. The oracle's solver is
../oracle/rv19_oracle.py.txt, unchanged; the scripts here import it.

checks/
  revisions.txt            PR, heads, write set, visibility, scales_at's callers
  records_check.txt        K4's _run_records (237) and retained_k4 (22) SHA256SUMS
  fk_full.log              FK's full suite from git archive a5fa0eaf7 (392 pass)
  build_warnings.log       FK lib with RUSTFLAGS=-D warnings (passes)
  rustfmt.log              rustfmt --check on K4's files (empty: clean)
  publication_diff.py      rows, classes, bounds and scales, before and after
  publication_diff.txt     its output: only PT-B's and PTF-B's bounds move
  selected_diff.txt        the selected set at 7d8fa9c0e against a5fa0eaf7
  memguard.txt             the memory guard's log (no kill)
probes/
  rv19_probe.rs.txt        RV19's probe module at the delta (adds rv19_dump_large)
  probe_all.log            OVF-ROT, the station combination, coverage, the dump of
                           every selected publication, groups and TINY-S
  probe_large.log          the six RF-LARGE 100-member publications
oracle/
  oracle_all.log           120 publications (114 cases, 6 combinations)
  oracle_large.log         the six 100-member frames
  rv19_xcheck.py.txt       GEN's 128-bit tokens against the 1600-digit solve
  xcheck.log, xcheck_large.log   its output (28,735 keys; 3 marker notes)
  rv19_slack.py.txt        compare_honest's slack delta against each allowance
  slack.log, slack_large.log     its output (largest delta/a 2^-74)
mutations/
  rv19_mut.py.txt          the harness (git archive a5fa0eaf7, fresh target each)
  results.jsonl, run.log   NONE and eight mutants
  logs/                    each run's cargo test output
