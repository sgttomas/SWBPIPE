K2b RETURN addendum 2: the RV11 delta fixes (I10, 2026-09-28; Mac, aarch64-apple-darwin, rustc 1.97.1).
Basis: RV11's delta check (REVIEW/K2B_REVIEW.md, "Delta check at f385a8bc8": PASS, with RV11D-1,
RV11D-2 and three NOTEs) and ROOT's rulings on it (ROOT's message of 2026-09-28).
Earlier run records beside this folder are unchanged.

- targeted/   FK, SD and NI in full, PP s11f_site_test, formation_check_runtime and
              k2a_formation_range_runtime, in <wt>/k2b on the final bytes (f385a8bc8 plus the four
              delta files).
- mutations/  clean archives of f385a8bc8 with the delta files overlaid (overlay.txt); NONE first,
              alone; then at most three at once, -j 4: the new K2B-SPRING-UNCHECKED and
              K2B-PIN-SPRING-EVASION (mutate_k2b.py.txt), RV11's three delta survivors
              (mutate_rv11d.py.txt, RV11's own script copied unchanged), and K2b's three pin mutants
              (the pin list changed). All eight killed (MUTANTS.txt, kill_sites.txt).
- callers.txt the callers scan with the new name force_scaled_spring_action.

Not re-run (no existing entry was touched): the b = 0 probe, T9 and the 39-manifest suites.
