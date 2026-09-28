RV10 run records for REVIEW/M03_SKEW_PIN_REVIEW.md (the independent review of PR #1038, the skew M03 pin, head
1d105d633). <wt> = the T3 worktree root; <wt>/scratch/rv10 = RV10's scratch; <VENV>, {REPO_ROOT}, <home>, <scratch> as usual.
Host: the owner's Mac, aarch64-apple-darwin, rustc/cargo 1.97.1 (toolchain.txt); RUSTUP_AUTO_INSTALL=0,
CARGO_INCREMENTAL=0, --offline --locked, cargo -j 8 at most (mutants -j 4, at most three at once), RUST_TEST_THREADS=4,
at most two cargo jobs at once; every model has one member. Targets under <wt>/rv10-target, pruned at the end; the
memory guard log (<wt>/guard/memguard.log) shows no kill. Every build is from a git archive of
projects/chirality-piping without execution/ (trees: cand = 1d105d633, base = eb52114e9, pe134 = 134eefc24,
pe_eb5 = eb52114e9) or, for GEN-8, a full archive of 1d105d633. RV10 made no Git write and wrote nothing in
<wt>/skewpin. The probes below were placed only in scratch archive copies.

checks/     merge_and_scope.txt (check 1 and the merge), records_hygiene.txt (check 5), gen8.txt (GEN-8 on the head).
pin/        fk_m03_cand.log, ni_i9_cand.log: the candidate's own tests, --nocapture, from the cand archive (5/5, 1/1).
            rv10_probe.rs.txt (+ replicas_verbatim.rs.inc.txt, the candidate's two replica functions copied verbatim,
            m03_skew_scope.rs lines 206-298 and k1_tests.rs lines 1429-1488) run against 134eefc24's own FK:
            A the replicas against 134eefc24's local_stiffness and global_stiffness bit for bit; B 134eefc24's own
            M03 outcomes; C bits for the Fraction check; D a sweep of (12E)I from 2^-1049 to 2^-1057 in 1/64 binade;
            E Iy != Iz on RV7's members. rv10_probe_f.rs.txt: Iy != Iz on a generic member. rv10_probe_g.rs.txt: a
            subnormal-derived GJ/L with normal bending. rv10_probe_h.rs.txt: the least bound
            of the accepted rows (the 2^-1050 row's margin). rv10_exact.py.txt: the independent Fraction re-derivation.
mutations/  mutate_rv10.py.txt (RV10's six mutants; I9's four re-killed from I9's committed mutate_i9.py.txt, unchanged),
            run_mutant_rv10.sh.txt, logs/ (NONE first, alone), kills.txt.
product/    run_probe_rv10.sh.txt (I9's probe, unchanged, on fresh archives of 134eefc24 and eb52114e9);
            rerun_compare.txt (byte comparison with I9's raw logs; statuses; the committed jsonl's trust check);
            rv10_product_exact*.py.txt and outputs (RV10's own Fraction solve of the 52 published runs).
suites/     the three crates in full on base and cand; summary.txt.
build_records.py.txt builds this folder; SHA256SUMS covers every file here except itself.
