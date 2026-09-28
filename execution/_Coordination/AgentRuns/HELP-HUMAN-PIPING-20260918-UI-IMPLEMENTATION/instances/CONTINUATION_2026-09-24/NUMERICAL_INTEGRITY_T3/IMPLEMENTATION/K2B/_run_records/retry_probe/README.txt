The "third attempt" question (ROOT, after checkpoint C; I10, checkpoint D, 2026-09-28).

Scratch probe, not committed: two tests appended to a git-archive copy of 8e6698282's
k2b_tests.rs, under <wt>/scratch/i10/d/tree, and run there with
  cargo test --offline --locked -j 8 --lib <name> -- --nocapture
(RUSTUP_TOOLCHAIN=1.97.1, CARGO_INCREMENTAL=0, RUST_TEST_THREADS=4, target <wt>/k2b-target/d,
pruned). The test sources are retry_probe_test.rs.txt and retry_probe_test2.rs.txt; the logs
are retry_probe.log and retry_probe2.log. Each test only prints; it asserts nothing about
the outcome.

- Candidate 1 (EA/L 2^400 N/m and 1 N/m in series): refused at the chosen b = 326. It is also
  refused at every forced b >= 356, by a scale-free failure: the 1 N/m bar is absorbed into
  N1 UX's 2^400 diagonal (ratio invariance). It does not distinguish the mutant.
- Candidate 2 (EA/L 2^440 N/m for both bars; UX loads 2^-1010 N at N1 and 1 N at N2): refused
  at the chosen b = 312 in all four mode and representation pairs. At forced b = 500 and
  b = 572 it is solved Passed in all four, with u within 1e-9 of the exact reference. At
  b = 416 it is Passed in SparseInteractive and refused in DenseScrutiny ("division overflow
  or underflow"). It distinguishes a third-attempt mutant from K2b. See RETURN §13.3.

After ROOT's ruling (ROOT_RULINGS_V1, "K2b: the b-rule's window misses the solve's range;
'no third attempt' is not equivalent (ROOT)", 97000ab9f), candidate 2 is pinned in NI
k2b_tests.rs by k2b_the_rules_b_refuses_a_case_another_b_in_its_window_solves (tests only,
added after checkpoint C). pin_targeted.log is its targeted run: the NI lib tests matching "k2b",
15 passed. The third-attempt mutants and their NONE control are in ../mutations/batch3/.
