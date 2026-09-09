# U7/A1 accessibility wording-priority successor

Status: SEALED FOR EXECUTION
Predecessor source SHA-256: `edab67d889d8658cf0cbcde5721b13a5b6b12ec7414ac23a07f3bd1805c8a6e3`
Predecessor return SHA-256: `b3fe988cac18ef3316b6554484fd1b775df26e853d860a5846a2699b5be638af`

Change only the conditional `title` priority on `apply-reviewed-draft` so `draftReviewBusy` is checked first, `!draftReview` second, and `!onApplyDraft` third. Keep all three exact messages, the disabled predicate, click handler, visible label, formatting, and every other byte unchanged. This ensures an overlapping busy/no-review state exposes the busy reason.

Run the unchanged focused dead-controls audit on the final source. Reuse the V1 three-test App evidence because this successor only reorders title selection and does not change enablement, click, state, handler, or Apply behavior. Run `git diff --check` on the source. Write only a compact portable successor return in the existing A1 evidence directory. Do not edit any test or other source and do not run build, native, Rust, WASM, Python, physics, full suite, global validators, or Git mutations. Do not delegate.
