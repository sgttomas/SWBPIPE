RV22's review of K6b (PR #1058, head 1123d19b9ab0a39b7ad5bdb33fc707846ae58a9c), 2026-09-29.
The review is T3/REVIEW/K6B_REVIEW.md. Everything here was produced from a clean `git archive` of the
head under <wt>/rv22/ with the target <wt>/rv22-target (both deleted afterwards), or read from <wt>/k6b
and I16's <wt>/scratch/i16 (read only). Paths are written <wt>, <scratch>, <home>, <VENV>, <tmp>.

scripts/    RV22's checks and probes (.txt): the RETURN table traces (rv22_tables*.py), the closure
            (rv22_closure.py), the decode-based adapter check against R1 (rv22_adapter_check.py), the
            estimate re-derivation (rv22_estimate.py), my runs against b3 (rv22_compare_runs.py), the
            mutation driver (rv22_mut.py), the probe test used in my copy only (rv22_probe.rs), and the
            suite runner (run_hsuite.sh).
suites/     H's debug suite (--all-targets), the runner suite, the pytest wrapper with the DEC-050/053
            pins, and the release build of k6_observe.
checks/     export_scan, merge_check, records_checks, tables_6_1, tables_6_2_to_7_2, closure_check,
            adapter_check, estimate_rederivation, estimate_m6_effect, plan_rows_check, b3_admission,
            load_ranges, w1a_runs_compare, probe_relaxed_check, toolchain.
w1a_runs/   my four release w1a runs (5 repeats, --w1-prefixes, --dump-published), at 10 and 100 members;
            the rows dumps are not kept (their sha256 are in checks/w1a_runs_compare.out and equal b3's).
mutations/  mut_results.jsonl (one line per run), the driver's output and each run's log.

final/      RV22's confirmation of head 011911e4e (RV22-1 to RV22-3 closed): scripts/ (the move-model
            itemization, the adapted probe, the mutant driver, the admission replay), suites/, checks/
            (counts diff and regeneration sample, the estimate re-derivation, the probe, records checks,
            b3's admission replay, toolchain) and mutations/.
