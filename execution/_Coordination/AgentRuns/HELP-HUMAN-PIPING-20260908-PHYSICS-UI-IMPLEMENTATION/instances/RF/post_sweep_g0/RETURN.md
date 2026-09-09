# RF post-sweep G0 return

Status: `CHANGES_REQUIRED`.

The isolated no-spring correction is mathematically supported: `round6(0.01 * 48.952652) = 0.489527`, while `0.490101 = round6(0.01 * 49.010116)` encodes the recorded previous normal. The unapplied patch is exactly test-only and makes no production change.

The candidate package is incomplete. The failed assertion preceded the `48.952652` assertion, so the clean run did not establish that normal. Active retained-spring Python, desktop, and generated-fixture goldens also retain inconsistent previous-normal pairs and were incorrectly classified as unaffected. The correction scope must include the one Rust test block, deterministic regeneration of `invented_mechanics_result.json`, and the narrow Python and desktop fixture assertions/comments, with exact paired values taken from independent diagnostics and generated output.

No production defect was found. A new bounded Owner act is required before editing or regeneration. After a complete candidate exists, the expanded focused checks, full product crate, fresh review, CHANGE integration, and clean five-surface DEC-025 rerun remain required. No source, fixture, build, test, Git, lifecycle, or accepted-evidence state was changed by RF.
