# DEC-025 for PR #1082 (F2a D1 milestone), on the merged head F′ = 5488136a19

- `Fprime/`: the DEC-025 run on F′ by the recorded Mac driver (`dec025_mac.sh.txt`, unchanged; fresh target). Its suites, sweep, pytest, desktop vitest and builds.
- `baseline_M/`: the fresh Mac baseline on main M = `5fdc5ab601` (`run_suites_nff.sh`, clean worktree, fresh target). Main did not move before the merge, so it is the baseline for F′.
- `first_freeze_F/`: the run on the first freeze F = `20dd3d929d` that found the defect: `suites.log`, the failing nonlinear_integration log, `meta.txt`, its sweep summary and pytest log.
- `suites_vs_baseline.txt`: `compare_suites.py baseline_M Fprime`, per manifest and per test name.
- `F_vs_Fprime.txt`: per-manifest counts, F against F′.
- `run_all.sh.txt` / `run_fp.sh.txt`: the quiet-host wrappers (6 quiet samples 20 s apart, memguard checked); `quiet_*.log`: their logs.
- `SUMMARY.json`: the result.

**Result:**
- **34 of 40 manifests are identical to main.** The other six differ only by added tests: PP, result_export, runner, frame_kernel, performance_harness and numerical_robustness.
- **The failing set is main's:** PP `t13` and runner's two `load_reference` tests, all Mac-only.
- **PP's 9 added non-ok tests** are U8's ignored `witness_*` tests. Three frame_kernel doc-tests are renamed by a 22-line shift.
- **Other suites:** pytest, vitest and both desktop builds pass.
- **F against F′:** only nonlinear_integration changes, from 132/2 to 134/0.
