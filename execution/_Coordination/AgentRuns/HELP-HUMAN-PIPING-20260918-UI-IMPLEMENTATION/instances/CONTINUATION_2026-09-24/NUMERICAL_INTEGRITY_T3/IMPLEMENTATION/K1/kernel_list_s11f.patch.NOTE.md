# Note on `kernel_list_s11f.patch` (I8R, checkpoint D)

`kernel_list_s11f.patch` is I8's work-in-progress artefact from the handoff. It is kept unedited.

The KERNEL-list hunk as applied differs from the patch in formatting only. It was committed on its own as `19925122b` ("K1: sparse.rs in the S11-F site test's KERNEL list (tests only)") in `P/core/product_physics/tests/s11f_site_test.rs`.
- **In the patch,** the closing `),` of the existing `FK/structural/formation_check.rs` / `evaluate` row and the opening `(` of the first new `FK/structural/sparse.rs` row share one line: `    ),    (`.
- **As applied,** they are on separate lines, as rustfmt lays out every other row of `FORCE_FUNCTIONS`.

The content is otherwise identical:
- the same `KERNEL` Source, appended after `FK/structural/formation_check.rs`, so `KERNEL[4]` and `KERNEL[6]` are unchanged;
- the same 11 `FORCE_FUNCTIONS` rows, with the same function names and dispositions.

This was checked by comparing the patch's added lines with the applied diff, with whitespace removed. The only differences are the joined `),(` line and the `),` line that the separation adds.
