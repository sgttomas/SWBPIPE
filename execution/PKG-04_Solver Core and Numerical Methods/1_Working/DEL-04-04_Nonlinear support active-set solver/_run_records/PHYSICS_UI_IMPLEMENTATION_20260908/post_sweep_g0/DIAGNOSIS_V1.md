# DEC-025 G0 product fixture diagnosis

Status: `STALE_TEST_EXPECTATION_CONFIRMED_CANDIDATE_NOT_APPLIED`

## Failure and exact fixture

The clean sweep at tested source `7b73460c5e2d85a9f050344069d211fea4af7b3e` stopped in `core/product_physics` after 137 passes and one failure. `tests::valid_invented_model_exposes_nonlinear_support_loop_evidence` expected the published UZ reaction for `support:NL-130-FRIC` to equal `0.490101 N`; the integrated solver returned `0.489527 N`.

The test loads `fixtures/product_preview/invented_preview_model.json`, removes all positive-stiffness and variable-spring-hanger supports, and replaces distributed loads with the explicitly preserved historical `qL/2` nodal loads. It retains the sliding friction support at node N-130 on global UZ with user-entered coefficient `mu=0.01`, derived normal source `support:S-130` global UY, and initial state `sliding`. It executes both `DenseScrutiny` and `SparseInteractive`. The same test asserts two iterations, zero final state-count residual, converged flag 1, sliding state 3, nonzero negative-Z displacement, positive UZ friction reaction, derived-normal kind/source metadata, and published current normal `48.952652 N`.

## Independent current-normal derivation

The accepted relation for negative-Z sliding is positive tangential reaction

`R_f = mu * abs(R_n(current))`.

The product adapter rounds every result once, at publication, to six decimals. The independently retained normal assertion `48.952652 N` bounds the positive full-precision current normal to `[48.9526515, 48.9526525) N`. Multiplying that interval by the fixture coefficient `0.01` gives `[0.489526515, 0.489526525) N`; every value in that interval publishes as `0.489527 N`. Thus the mathematically supported replacement is exactly `0.489527`, independently of the failing observed value.

The old `0.490101` literal instead corresponds to the prior normal near `49.0101 N`; the original friction run record explicitly records the transition `49.010116 -> 48.952652 N`. Under the former lagged implementation, `round6(0.01 * 49.010116) = 0.490101`. Under the accepted same-iterate coupling, `round6(0.01 * 48.952652) = 0.489527`. This is the precise behavior the F4 repair was intended to change.

## Assertion inventory and conclusion

Only one active assertion contains `0.490101`: the UZ friction reaction literal in `core/product_physics/src/lib.rs` inside this test. The adjacent `48.952652` current-normal assertion, loop/state assertions, sign assertion, metadata assertions, and diagnostics assertions remain valid. Active Python and desktop tests exercise the different retained-spring fixture and already carry `0.411514 N` friction with `41.120255 N` normal; they are unaffected. Historical snapshots and reproduction outputs containing `0.490101` remain immutable evidence of the former behavior.

Conclusion: no product or nonlinear production regression is indicated. The minimum repair is test-only in this one product-physics test: replace `0.490101` with independently derived `0.489527`, retain the current-normal literal, and explicitly assert the six-decimal `mu*N(current)` relation at the public result boundary.

## Required authority and acceptance checks

The candidate is not applied. Because product-physics lies outside F4's expired Step 1 source fence, root needs a new explicit bounded Owner act limited to this test block and successor evidence.

The initial candidate-patch syntax check and its first hunk-count correction both exited 128 without touching source. Correcting the exact old/new hunk spans produced the final candidate, which then passed `git apply --check`.

After authorization: apply the candidate without production edits; run the single affected test (which covers both modes), `cargo fmt --check`, and the full offline locked product-physics crate; freeze the exact test-only diff/source hash; obtain fresh independent review of the replacement derivation and scope; commit through CHANGE; then rerun the full clean DEC-025 five-surface sweep from the corrected clean source. Acceptance requires 138/138 product tests, unchanged production bytes, the exact public relation `0.489527 = round6(0.01 * 48.952652)`, independent review `PASS`, and all five clean sweep surfaces `PASS`.
