# U7 post-G2 accessibility return

Status: PASS; one-file source frozen for root-routed RU review.

Agent 2 `/root/native_authoring/u7_post_g2_accessibility` changed only `PipeViewport.tsx`, from SHA-256 `ac3789a6b4e8c452fe398d062183b25d884f55f5763af877ac6f1b3d4cb9f2ad` to `d284969e4d52e6140625fe48c8ad482a406307917845b99af58338e5f9048eda`. The existing disabled predicate, click handler, visible Apply label, and App handler wiring are unchanged. The button now exposes a title whose priority is busy request, missing reviewed draft, then unavailable Apply handler; enabled state has no title.

The unchanged dead-controls audit passed 1/1 on the final source. Three existing Apply/busy/stale tests passed on V1 and remain applicable because V2 only reorders title selection. The updated 17-member binding rehashed all members and changes only `PipeViewport.tsx`. A lossless base64 diff archive supplies the exact one-file diff without an active raw patch.

Native impact is limited to the rendered button title/tooltip after rebuild. There is no native, IPC, state, model, physics, persistence, solver, or Apply-flow behavior change, so a separate native walkthrough is not causally required by this repair. CHANGE's required clean build and full rerun remain the next integration gate.
