RV14 delta check of K5 at head 28517eaaa (addendum 95c7501a7 on b379e5b27, plus the merge of main 65e2d6c2a), 2026-09-28.
Same host rules as the review: git-archive copy of 28517eaaa under <wt>/scratch/rv14/delta, targets under <wt>/rv14-target
(deleted afterwards), one cargo job at a time, -j 4, RUST_TEST_THREADS=2, rustc 1.97.1, --offline --locked.

Files:
- revisions_delta.txt: the head after a fetch, PR state and hosted checks at that time, the merge checks (the merge's
  diff equals main's delta; the slice's diff is unchanged by it; no piping file; empty remerge diff), the addendum's
  numstat and deleted lines.
- records_checks_delta.txt: K5's SHA256SUMS (225), the FK vectors' SHA256SUMS, gen_k5_vectors.py --check, subnormal.txt
  against RV14's tiny corpus, RETURN 16.6's file table, the reused RV14 scripts' hashes, interface anchors, N1, git diff
  --check, and the path, model and user-name scans of the addendum's 50 files.
- compare_old_new.txt: RV14's unchanged FK probe on its corpora at 28517eaaa against b379e5b27 (exactly the 349
  non-finite witnesses change, W -> U; nothing else), the reasons (probe v2), and RV14's delta cases.
- report_{all,huge,rv14_4,delta}.txt: RV14's oracle on the head's results. results_all.txt is not kept: its sha256
  (681116899a22b2cf...) equals I14's recorded candidate hash.
- cases_delta.txt, cases_delta2.txt, results_v2_delta.txt, results_v2_delta2.txt, rv14_fk_probe_v2.rs.txt.
- pp_runs_head.txt: RV14's PP spot-check at the head (byte-identical to b379e5b27's 40 runs); probes_identity.txt states the SA and PP probes' identity
  ni_build.txt: the non-test NI build (no warning).
- mutations/: rv14_mutate_delta.py.txt, rv14_run_mutant_delta.sh.txt, batch_delta.sh.txt, queue_delta.txt,
  batch_delta.log, MUTANTS_DELTA.txt and every log.
- memguard_delta.txt.
