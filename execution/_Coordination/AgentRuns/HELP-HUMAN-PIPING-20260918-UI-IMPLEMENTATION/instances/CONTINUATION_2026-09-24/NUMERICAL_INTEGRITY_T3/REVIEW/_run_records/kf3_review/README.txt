RV23's review of KF3 (PR #1059, head b8c55c92e4762f117ba60a608549b1786df4b433, base main 78f55f927), 2026-09-29.
The review is T3/REVIEW/KF3_REVIEW.md. Everything here was produced from clean `git archive` copies of the head:
<wt>/rv23 (pristine: FK, H, VR suites, GEN's --check, vk_scale) with the target <wt>/rv23-target; <wt>/rv23-mut/probe
(the reviewer's probe copy: the patch in scripts/rv23_probe_patch.diff.txt plus the two probe test files) with
<wt>/rv23-mut/probe-target; and one clean FK copy and target per mutant under <wt>/rv23-mut/. All were deleted at the
end. Toolchain: rustc/cargo 1.97.1 (RUSTUP_TOOLCHAIN=1.97.1, --offline --locked, -j 4, RUST_TEST_THREADS=2, one cargo
job at a time), Python 3.13 (standard library). Paths are written <wt>, <scratch>, <home> and <tmp>.

scripts/    rv23_probe.rs (FK probe tests, mounted in the reviewer's copy of adaptive.rs), rv23_probe_patch.diff (the copy-
            only patch: a thread-local forced refusal `rv23_force` at seven sites, ScaledProfile's fields made visible,
            the probe mounted), rv23_h_probe.rs (RV22's six limits and a 2,011-limit sweep on H), rv23_oracle.py (exact
            dyadic tokens, directed rounding, the span rule, R7 7b's passes with A2, exact inverse norms), rv23_syn.py and
            rv23_syn2.py (the synthetic checks, also calling GEN's A2 functions), rv23_forced.py (the forced-refusal
            check), rv23_kf3_closed_form.py (KF3-UC-SPAN's statics and beam-theory solution), rv23_bcmp.py (a vk_scale
            run against the committed B run), rv23_b_records.py (KF3's B records against V-K's), rv23_mut.py (the
            mutation driver). All as .txt.
probes/     syn.out (rv23_synthetic_multiblock), probe2.out (rv23_synthetic_forward_refusal_and_refused_s,
            rv23_kf3_uc_span_published_rows, rv23_b1_estimate_ratio_against_p), forced.out (rv23_forced_refusals),
            sweep.out (rv23_stage_sweep_small, rv23_stage_sweep_kf3, rv23_refusal_evidence_on_a_budget_stop_after_the_refusal),
            h_probe.out (rv23_rv22_paths_and_a_dense_sweep), with their .time files.
checks/     syn_check.out, syn2_check.out, forced_check.out, kf3_closed_form.out (the oracles' verdicts),
            reach_and_scope.txt (git grep and diff stats at the head), b_records.txt (KF3's B records against V-K's),
            kf3_records_integrity.txt (the KF3 folder's SHA256SUMS, coverage and machine-path scan).
suites/     FK's full suite (debug) and its build, H --all-targets and its runner, VR, GEN's --check.
b_runs/     my release vk_scale runs of RF-LARGE-CHAIN-n10000-AX and RF-LARGE-TREE-n10000-AX (JSONL and /usr/bin/time -l).
mutations/  mutants.jsonl (one line per mutant) and each mutant's log. Three logs whose panics printed whole CaseOutcome
            values (42 MB each) are trimmed to their test lines and first panic lines, with the original size and sha256
            in their header.

To re-run a check: copy the scripts to .py files beside rv23_oracle.py and run, for example,
  python3 rv23_forced.py probes/forced.out
  python3 rv23_syn.py probes/syn.out <gen_k4_vectors.py>
  python3 rv23_syn2.py probes/probe2.out <gen_k4_vectors.py>
  python3 rv23_kf3_closed_form.py <K4T/kf3.txt> probes/probe2.out
