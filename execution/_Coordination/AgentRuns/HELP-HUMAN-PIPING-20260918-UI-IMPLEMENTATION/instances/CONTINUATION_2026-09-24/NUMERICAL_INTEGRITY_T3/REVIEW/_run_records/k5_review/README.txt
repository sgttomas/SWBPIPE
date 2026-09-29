RV14: run records of the independent full-diff review of slice K5 (PR #1044, head b379e5b27, base main 24dea2dae), 2026-09-28.
Mac, aarch64-apple-darwin, rustc and cargo 1.97.1 (toolchain.txt). RUSTUP_TOOLCHAIN=1.97.1 RUSTUP_AUTO_INSTALL=0
CARGO_INCREMENTAL=0 --offline --locked, -j 4, RUST_TEST_THREADS=2, one cargo job at a time (F1b shared the host).
Default dev profile throughout. Everything built came from git-archive copies under <wt>/scratch/rv14 (base and head),
with targets under <wt>/rv14-target; each mutant had its own clean archive and target, deleted after its run.
Nothing was built or written in <wt>/k5; GEN-8 ran there read-only (gen8.txt). No dense matrix at 10,000 or more
members was formed (the largest SA model has 6 nodes; the FK corpus has at most 400 nodes, all at FK level).
The review was paused once on ROOT's order (no process was running) and resumed on the unchanged head.

Placeholders: <wt> is the T3 worktrees root, <VENV> the repository venv, <repo> the outer worktree, <home> the home
directory, <tmp> a temporary directory. build_records.py.txt replaced machine paths with these when it assembled this
folder, and checks that none remains.

Files:
- revisions.txt: the head after a fetch, the PR state, the commits, the code numstat, the slice's only deleted lines,
  rigid_body.rs lines 1-248 hashed on base and head, what changed after A2, and git diff --check.
- records_checks.txt: K5's SHA256SUMS (187 files, set-equal) and the FK vectors' SHA256SUMS, the path, model-identifier
  and user-name scans of the 199 slice files, the Mac-only statements, RETURN section 11's line counts and section 9's
  interface anchors at the head.
- gen8.txt: GEN-8 on the head in <wt>/k5 (1 passed; the working tree clean before and after).
- libm_scan.txt: K5's non-test FK block, comments and strings removed: no call of a function of unspecified precision.
- product_runs_reclass.txt: I14's 152 raw product-run records reclassified without I14's classifier.
- memguard.txt, toolchain.txt.
- oracle/: rv14_oracle.py.txt (my exact-rational oracle of the UNREDUCED stacked map, written without K5's generator;
  gen/gen_tiny/check), rv14_permute.py.txt, rv14_scale.py.txt, rv14_huge.py.txt; the corpora cases_all.txt (seeds
  1402 and 1403, 4,226 cases), cases_huge.txt (seed 1405, 700 cases) and cases_rv14_4.txt (6 constructed cases); the FK
  probe's results and the oracle's reports for each; comparisons.txt (hashes, order-permutation and 2^+-40 results,
  the prefilter mutant's identity, M0438's singular values).
- probes/: the FK probe (rv14_fk_probe.rs.txt, and the variant printing singular values), the SA probe on base and head
  (frame-only byte identity; X2 is the one K5-C1 difference), the SA mutant probe (P1, P2), the PP spot-check harness
  (built on I14's harness functions) with both trees' runs, and probe_comparisons.txt.
- mutations/: my patch script (rv14_mutate.py.txt), K1's mutate_k1.py.txt (byte-identical to K1's record, sha256
  c2b54737...; used for K1-PIN-LOOP-B and KD5-M32a), the runner, queue, batch log, MUTANTS.txt and every log. Large probe
  outputs are kept as excerpts with the full file's sha256.
- build_records.py.txt: the assembler of this folder.
