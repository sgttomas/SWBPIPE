K2b RETURN addendum 1: the RV11 fixes (I10, 2026-09-28; Mac, aarch64-apple-darwin, rustc 1.97.1).
Basis: RV11's review (REVIEW/K2B_REVIEW.md, FAIL at 087b3a088) and ROOT's rulings on it
(ROOT_RULINGS_V1.md, "K2b: rulings on RV11's review (ROOT)", numerics 4ec82a9b3).
The checkpoint-D run records beside this folder are unchanged.

The candidate: a git archive of 087b3a088 (without execution/) with the five fix files overlaid
(overlay.txt, overlay_hashes.txt); equal to <wt>/k2b's working tree (diff -r, build output excluded).
The base for the probe and T9: a git archive of main 98b1723b1.

- targeted/   FK, SD and NI in full, PP s11f_site_test, formation_check_runtime and
              k2a_formation_range_runtime, in <wt>/k2b on the final bytes.
- suites/     all 39 cargo manifests --no-fail-fast on the candidate tree (run_suites_nff.sh.txt,
              the calibration script), compared test by test with ROOT's baseline for current main,
              <wt>/scratch/sweep_skewpin/suites/ (the skew pin's candidate 1d105d633, whose core/ and
              validation/ equal main 98b1723b1's): suites_compare.txt/.json, failure_blocks_sha256.txt.
- probe/      checkpoint B's b = 0 Debug probe source, unchanged, built in release from the base and
              candidate trees; the two output hash lists.
- t9/         S11-K's committed-fixture harness (Mac-only), release, base and candidate.
- mutations/  the full table re-run from clean archives (NONE first; at most three at once, -j 4):
              mutate_k2b.py.txt (I10's, with the fix's four new mutants), mutate_rv11.py.txt (RV11's
              own script, copied unchanged from REVIEW/_run_records/k2b_review/mutations/),
              mutate_k1.py.txt (K1's, unchanged), MUTANTS.txt, kill_sites.txt, and
              kill_test_comparison.txt (killing tests by name against checkpoint C, batch 3 and
              RV11's run; the fixes moved lines).
- callers.txt the callers scan with the two new names (force_scaled_end_actions,
              row_product_stays_normal).

A first suites run, and first probe and T9 runs, were made on a candidate tree whose lib.rs differed
from the final bytes only in rustfmt whitespace inside the new function; they were discarded and
re-run on the final bytes. Every file here ends with one newline (RV11-N6).
