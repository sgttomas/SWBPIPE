# F4 post-sweep G0 diagnosis scope

Status: `SEALED_READ_ONLY_SOURCE_DIAGNOSIS`

Objective: diagnose the single clean DEC-025 G0 failure in `tests::valid_invented_model_exposes_nonlinear_support_loop_evidence` and prepare a concrete test-only candidate patch if the accepted current-normal mechanics proves the expected literal stale.

Frozen inputs:

- tested source commit: `7b73460c5e2d85a9f050344069d211fea4af7b3e`;
- product-physics source SHA-256: `e757b8a51e2c4ae68ac4d6c37620663bf4d698ff03b8d6349b40b484bb591903`;
- nonlinear-integration source SHA-256: `6e163a47db60288179f844d473992d28a53599ad2204149093186c760723ad46`;
- invented preview fixture SHA-256: `986c055944776ca8d0d849d552678e1f2bfb6c4559bbb9ff8a1928157a4f871c`;
- DEC-025 failure record SHA-256: `b350be906fe94575a28d35e4f498b0a931c74da36831946c9490133817335ffc`;
- sweep summary SHA-256: `0d6f31872b51d3c1021d574d42d127aabaf0b400fa06690720672dfb5625268e`.

The product fixture, product source, nonlinear source, tests, Git state, and frozen records are read-only. Authorized writes are limited to this diagnosis directory and `DEL-04-04/_run_records/PHYSICS_UI_IMPLEMENTATION_20260908/post_sweep_g0/**`. A candidate patch is proposal evidence only and must not be applied without a new explicit bounded Owner act because the Step 1 exception expired at integration and product-physics is outside F4's former source fence.

No test/build/sweep execution, new threshold, production change, engineering decision, or acceptance claim is authorized. The diagnosis must derive the expected published value from the fixture coefficient, the same returned current normal, and the existing publication rounding rule; it must inventory every active assertion affected and preserve historical records as historical evidence.
