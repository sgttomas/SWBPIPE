RV12's delta check of K3 at 2511f5a3c (PR #1041; follow-ups e62837f7e, tests only, and 2511f5a3c,
records), on top of the reviewed head b7e93650e. The section "Delta check at 2511f5a3c" in
T3/REVIEW/K3_REVIEW.md reports it. Built only from `git archive` copies of 2511f5a3c in
<wt>/scratch/rv12/delta, at opt-level 0, with no profile; targets under <wt>/rv12-target (deleted).
No Git writes; <wt>/k3 read only (plus one read-only GEN-8 run).

checks/      the untouched sources and manifest (blobs), the product and records paths changed,
             the vector prefix identity, the vector SHA256SUMS, both generators' --check,
             K3's records SHA256SUMS, hygiene greps, rustfmt, GEN-8, hosted CI status, and the
             taildecides vectors against RV12's own oracle (check_taildecides.py.txt, which
             imports the review's rv12_oracle.py, recorded in ../oracle/).
head_tests/  FK's full suite at opt-level 0 on the archive of 2511f5a3c (229 passed) and the
             non-test build (no warnings).
mutations/   RV1, RV2, RV4 and RV5 (the review's own patches, verbatim) on clean archives of
             2511f5a3c's core/ tree, with a NONE control first; test filters aimed at the new
             tests (RV5: only the L = 4, 8 and 16 targeted vectors, so the L = 2 cross-check
             cannot be the kill).
SHA256SUMS covers every file here except itself; the review folder's SHA256SUMS also lists them.
