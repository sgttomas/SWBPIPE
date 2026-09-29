RV11 merge confirmation at 33e33c723 (main 57617b0fb, with K3, merged into K2b at 112c1729d), 2026-09-28.
Same host, toolchain and limits as before (../toolchain.txt): opt-level 0, -j 8, RUST_TEST_THREADS=4. The build came
from a git-archive copy under <wt>/scratch/rv11 with target <wt>/rv11-target. No Git writes; no mutants.

Files:
- merge_check.txt: the merge adds exactly main's delta since 98b1723b1, its first-parent-side delta is exactly
  K2b's, the remerge diff is empty, and no file is edited by both K2b and main (in FK or anywhere in core/).
- head_tests/: FK, SD and NI in full, and PP s11f_site_test and formation_check_runtime, on the 33e33c723 archive.
- b0probe/: I10's b = 0 probe rebuilt against 33e33c723 (all 439 outputs), its list and the comparison.
