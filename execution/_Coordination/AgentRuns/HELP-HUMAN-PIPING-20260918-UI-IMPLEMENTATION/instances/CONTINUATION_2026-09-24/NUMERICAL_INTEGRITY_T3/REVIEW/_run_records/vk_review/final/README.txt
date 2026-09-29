RV21: confirmation of V-K's head 5f0d394262516e4053322730506cb88dfb36a2f0 (ROOT's narrow check; VK_REVIEW.md,
"Confirmation at 5f0d39426"). Placeholders: <wt> is the T3 worktree root.

Copies and targets (deleted afterwards): three git archives of 5f0d39426 (projects/chirality-piping without
execution/, as CI checks out) under <wt>/rv21c/{NONE,RV21-H4,RV21-H6}, each with its own target
<wt>/rv21c-target-<id>; the VK records folder and VR's cases and observations from a fourth archive (<wt>/rv21c/rec).
One cargo job at a time, -j 4, RUST_TEST_THREADS=2, RUSTUP_TOOLCHAIN=1.97.1, --offline --locked,
FK_SEEDED_FAULT unset. A DEC-025 sweep ran alongside.

  diff_check.txt      git diff 3fd1baff3 5f0d39426: stat, no FK or src/ file; RV21-1's line and self-test strings;
                      RV21-2's two tests against RV21's draft
  matrix.txt          NONE 47 of 47; RV21-H4 and RV21-H6 each killed by its own test alone
  logs/               the three cargo test logs
  RV21-H4.diff.txt, RV21-H6.diff.txt   each mutant's single-line edit against the clean archive
  n3_probe.log        N3's set check: not_covered.json's last entry replaced by a duplicate of its first (51 entries,
                      every one in the case lists) fails "a duplicate in not_covered.json" (then restored)
  records_checks.txt  SHA256SUMS at the head (VK 409, cases 15, observations 12/2/1) and the machine-path scans
