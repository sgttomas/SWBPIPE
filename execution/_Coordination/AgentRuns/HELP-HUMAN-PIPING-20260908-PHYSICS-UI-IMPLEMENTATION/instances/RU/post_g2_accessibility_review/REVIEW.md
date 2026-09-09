# RU independent G2 accessibility review

Verdict: **PASS**

No actionable defect remains in the bounded repair.

The final `PipeViewport.tsx` SHA-256 is `d284969e4d52e6140625fe48c8ad482a406307917845b99af58338e5f9048eda`. The lossless archived diff decodes to SHA-256 `18111a2aa9fc445abc53183650ea757b06549ee75026d7adf8c608f0c98131ce` and changes one control only: `apply-reviewed-draft`. Removing that exact formatted button/title block in memory and restoring the recorded one-line button reconstructs the accepted preimage SHA-256 `ac3789a6b4e8c452fe398d062183b25d884f55f5763af877ac6f1b3d4cb9f2ad` byte-for-byte.

The added `title` reports the existing disabled causes in this order: request busy, missing frozen review, unavailable Apply callback. Each message is truthful for its selected state, including the overlapping busy-plus-no-review state, and the title is absent when enabled. The disabled predicate `!draftReview || draftReviewBusy || !onApplyDraft`, click handler `() => void applyDraftReview()`, visible `Apply` label, App wiring, callbacks, state transitions, and model/physics/route contracts are unchanged.

U7 evidence records the unchanged dead-control audit passing 1/1 on the final source. It records three existing Apply/busy/stale tests passing 3/3 on the V1 annotation cut, with reuse for V2 limited to the later reordering of title selection. This reuse is adequate because the V2 byte delta cannot change enablement, click handling, state, or Apply behavior. RU did not rerun tests under the read-only review brief.

The revised 17-member binding SHA-256 is `58a94da8227c96734a433f8ddd116fee4558527fa01ab5354c35f0dc48ecb7e9`; its aggregate independently recomputes to `d80b48eaa2c1c60532c17fbace51eddd9f049123396af7d01c66a3a2ab20ca6a`, all 17 live files match their declared hashes and byte counts, and comparison with predecessor binding `7eaf4e22e3ed32b00b980938f1da5398907eb4dcbb7c93a2764c2446187ec5e2` identifies only `PipeViewport.tsx` as changed.

The accepted native build and walkthrough remain historical evidence for the unchanged solver/route workflow at the older exact source cut, whose `PipeViewport.tsx` member was `ac3789a6…`. They are not a fresh build or exact bundle witness for the final annotation-repaired cut. Because the final delta only adds an HTML `title` to an existing button and changes no action, state, layout text, IPC, persistence, model, physics, solver, Rust, or WASM behavior, it does not causally require another native walkthrough. The required CHANGE clean build and complete DEC-025 rerun remain the integration gate for the final source.
