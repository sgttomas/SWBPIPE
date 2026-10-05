# Errata to the F2a D1 merge record

**E-1 (ROOT, 2026-10-05 UTC): the 9 ignored PP tests are U4's stack witnesses, not U8's.**
- **The tests:** `retained_memory::witness_tests::witness_*` in `core/product_physics/src/retained_memory_witness_tests.rs`.
- **What they are:** U4 G5 part 2's S1 stack witnesses W1–W7 (D-3 = S1). A stack overflow aborts the test process, so they are `#[ignore]` and run explicitly, one invocation each, per qualified build identity. They are Pass B's "witnesses 9/9" (`R/I65/u4_g7_06/`).
- **The mislabel:** `dec025/README.md`, `dec025/SUMMARY.json` and RR "DEC-025 on F finds…" call them "U8's" witnesses. RV95's ADDENDUM_02 and ADDENDUM_03 repeat the label from ROOT's reading.
- **What does not change:** the counts and every conclusion. They are tests the PR adds, ignored by design, and the failing set is main's.
- **U8's deferred witnesses** are different work: the native Ceiling row, the L = 0 base, and RV93 N-5's real-input Candidate test. None of them exists yet.
- **Sealed files are not edited.** This file and an appended SHA256SUMS line record the correction.
