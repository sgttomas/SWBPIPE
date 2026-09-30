RV23's confirmation of KF3 head aa83f67969c2f618034b856ba6e1fc13ae10762e (the fix of RV23-1 and RV23-N1's test), 2026-09-29.
The section is "Confirmation at aa83f6796" in T3/REVIEW/KF3_REVIEW.md. Built from clean `git archive` copies of the head under
<wt>/rv23c/ (head: suites and GEN; probe: the copy-only patch and probe tests; one FK copy per mutant), targets <wt>/rv23c-target,
<wt>/rv23c/probe-target and one per mutant, all deleted. Toolchain as in ../README.txt.

scripts/    rv23_probe.rs (the probe tests at this head: the earlier ones with the new signatures, plus
            rv23c_a_cached_failed_shared_build_carries_its_refusals_to_a_later_case and
            rv23c_s_refusals_survive_a_stop_inside_the_schedule), rv23c_probe_patch.diff (the copy-only patch: rv23_force with a
            forced Span and a forced Arithmetic stop, ScaledProfile's fields visible, the probe mounted), rv23c_mut.py (the driver).
probes/     confirm.out (the two new probes), sweep.out (the stage sweeps and the LOST probe), cache_probe_under_M1.out (the cache
            probe with RV23C-M1 applied to the probe copy).
checks/     invariance.txt (the probes' RV23 lines at aa83f6796 against b8c55c92e's committed outputs), reach_and_scope.txt.
suites/     FK's full suite (debug) and GEN's --check.
mutations/  mutants.jsonl and each mutant's log.
