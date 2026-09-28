I9 (the skew M03 pin) run records. <wt> = the worktree root; <wt>/scratch/i9 = I9's scratch directory; <home>, <VENV> as usual.
Host: the owner's Mac, aarch64-apple-darwin, rustc/cargo 1.97.1 (toolchain.txt); RUSTUP_AUTO_INSTALL=0, CARGO_INCREMENTAL=0,
--offline --locked, -j 8 at most (mutants -j 4, at most three at once), RUST_TEST_THREADS=4, at most two cargo jobs at once.
The memory guard log (<wt>/guard/memguard.log) shows no kill during I9's runs. Built from git archives (trees.txt).

Kernel pin (the candidate's own tests):
  fk_m03_skew_scope.log       FK --test m03_skew_scope --nocapture on the candidate tree: 5/5 pass, with every figure printed.
  ni_i9_adapter.log           NI --lib i9_ --nocapture on the candidate tree: 1/1 pass (both evidences, both modes).
  exact_reference.py.txt      the independent exact (Fraction) reference for the coefficient errors; its stdout; the E bits
                              equal the Rust test's printed bits.
  development/                runs in <wt>/skewpin: run 1 failed (the counterfactual as first drafted, with a factor of two,
                              and the (1,2,2) 2^-1055 product underflow it exposed); run 2 passed; the first NI run (these
                              three on earlier bytes: before rustfmt and later comment-only edits); the final worktree runs
                              (the committed bytes); and a standard-library replica (counterfactual_diagnosis) that reports
                              where each bound refuses (development evidence only; the Rust tests are the evidence).

Product evidence (product/): the probe i9_skew_product_probe.rs.txt, placed only in scratch archive copies of main
  134eefc24 (pe134, pre-K2a) and eb52114e9 (pe_eb5), run with run_probe.sh.txt: 432 runs each (4 (12E)I cases x 9 G
  values x 3 members x 2 modes x 2 entries). probe_*.log are the raw outputs (data; lines not cut); product_reference.py.txt
  computes the exact references and the standing table (product_reference.stdout.txt, product_table.txt) and writes
  product_runs.jsonl (one record per run; messages cut at 240 characters). The stop rule was not triggered.

Mutations (mutations/): mutate_i9.py.txt (the patches; every anchor matches once), run_mutant.sh.txt, the 8 logs (NONE
  first, alone; then three at a time), kill_sites.py.txt and kill_sites.txt.

Suites (suites/): run_suites_i9.sh.txt on base (main eb52114e9) and cand, frame_kernel, sparse_direct and
  nonlinear_integration in full, --no-fail-fast; compare_suites_i9.py.txt -> suites_compare.txt.

T9 (t9/, Mac-only): run_t9.sh.txt, the harness manifests, build and run logs, output hashes, t9_summary.txt.
