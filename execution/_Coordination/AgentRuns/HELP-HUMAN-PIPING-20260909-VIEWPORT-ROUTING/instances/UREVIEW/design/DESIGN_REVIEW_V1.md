# Independent viewport-routing design review V1

Status: `FROZEN`
Verdict: `PASS`
Basis HEAD: `533332349a4607eee561d4ef90fb05a62d86519e`
Reviewed candidate: U7 `ASSESSMENT_AND_FENCE_V1.md` SHA-256 `0dd7f2433dea46cbc6ba92d3aa5f8c10e63ff6fb68344e1067ae4fed275b1252`
Consumed amendment: U7 `DESIGN_AMENDMENT_V2.md` SHA-256 `66974cc4c4773c2940c0a1abf5b903b220da99db8f7842cc0b86d1203a375719`
Checked implementation brief: U7/I1 `SEALED_IMPLEMENTATION_BRIEF_V1.md` SHA-256 `986ac8fe0e5c317e445805ba65a29c6c1259eab28459ef968b93259046418d55`

This is a pre-effect, read-only design refutation. The verdict applies to the U7 design with the refinements below, which root selected as ordinary implementation judgment during review. It grants no source, dependency, engineering, lifecycle, or acceptance authority and does not replace the separately fresh review of 100% of the frozen final diff.

## Actionable findings

No blocking finding remains after the selected refinements. The original candidate left four behaviors under-specified; treating these refinements as part of the implementation brief resolves them without widening the six-path fence.

### R1 — Use ordinary click slop and isolate it from OrbitControls damping

Replace literal `same-position` / `any movement cancels` with a 4 CSS-pixel radial click slop from primary `pointerdown` to `pointerup`, bound to the same `pointerId`. Capture the pointer on the original event target so `pointerup`, `pointercancel`, and lost capture are observable without starving the canvas controls. Commit selection or authoring only on qualifying `pointerup`; movement beyond 4 CSS pixels, pointer cancellation, or lost capture cancels the candidate.

A sub-slop OrbitControls `change` is normal click jitter and does not veto the click. Camera changes caused by pre-existing damping or scene setup also do not veto it. Movement beyond the slop is the pointer-drag signal and cancels authoring while leaving OrbitControls free to orbit. Four CSS pixels is an interaction affordance, not an engineering tolerance and never enters model data.

Tests must cover exactly 4 pixels accepted, greater than 4 rejected, pointer-ID mismatch, cancellation/lost capture, and an orbit drag that changes no draft coordinate, model, receipt, or checkpoint.

### R2 — Make ghost provenance explicit and never show stale geometry

For a resolved new-end From node, a pointer currently over the visible WebGL canvas drives the live constrained hover ghost in model coordinates. After a successful converted capture, the ghost uses the exact captured model point. A manual X, Y, Z, or coordinate-unit edit clears that pointer-derived ghost; it must not leave the prior hover/capture geometry visible as though it represented the authoritative typed draft. A later valid hover may provide a new transient pointer preview without rewriting the typed fields until a qualifying click.

For existing-end mode, the ghost is derived only from the exact two node IDs after both resolve in the current model. Missing, equal, reserved-only, or stale IDs render no route ghost. Mode change, tool exit, cancel, selection invalidation, model replacement, undo/redo, and stale conversion clear hover/capture state. No-WebGL state supplies no projector and no pointer ghost; manual entry remains usable.

### R3 — Close plane/constraint state transitions and explain disabled controls

The construction-plane anchor is the current model node resolved by `pipeDraft.from`, never the selected entity or a cached prior node. With no resolved From node, plane and axis controls are inactive, grid/ghost are absent, and adjacent text explains that a From node is required. Plane equations are exact: `XY => z=anchor.z`, `XZ => y=anchor.y`, and `YZ => x=anchor.x`. Axis projection then preserves only the selected in-plane coordinate and fixes the other two coordinates to the anchor.

Changing plane resets an incompatible selected axis atomically to `Free`; enabled axes are `X/Y` for XY, `X/Z` for XZ, and `Y/Z` for YZ. Plane or constraint change invalidates a frozen Add review but does not rewrite typed coordinates. A ray with no finite intersection publishes nothing and shows a local placement message. Disabled plane/axis/Add controls expose their reason in visible text and/or `title`/`aria-describedby`; the route Add reason is sourced from the existing `routeBuild.errors` or busy state.

### R4 — Add an explicit Playwright check to the registered affected checks

`select_affected_checks.py` selects `desktop-test`, `desktop-build`, and the always-on `harness-self-check` when the six paths are supplied relative to `projects/chirality-piping`, as required by `software-workflow.json`. The profile does not register Playwright. Because `linear-authoring.spec.ts` changes and browser pointer/OrbitControls behavior is central, add this explicit check after the frozen cut:

```text
cd projects/chirality-piping/apps/desktop && ../../node_modules/.bin/playwright test e2e/linear-authoring.spec.ts --project=chromium-desktop --project=chromium-compact
```

That command runs both configured projects (`chromium-desktop`, `chromium-compact`); the spec itself sets the required 1024x768 witness where applicable. U7/I1 correctly conditions it on root confirming the prepared Wasm/browser slot. The implementation return must separately report desktop test/build, this Playwright run, and any root-owned broader checks. No tests were run in this design review.

## Integration and identity trace

- `PipeViewport.tsx:606-660` already separates Add validation from Apply and gates stale review responses. `App.tsx:988-1109` freezes the submission, binds basis revision/epoch/hash, validates the exact operation or ordered batch, and commits the applied model with `review.reviewId`.
- `PipeViewport.tsx:304-319` continues only when `modelCommitToken` equals the pending own review ID. The implementation can preserve this identity path without editing `App.tsx`.
- Successful continuation moves From to the accepted end; retains selected plane, any still-applicable axis, endpoint unit, material, OD/wall, y-reference, pipe provenance, and continue choice; clears pipe ID/label/to plus new-node ID/label/X/Y/Z/provenance; and leaves new-end mode ready for the next capture. The retained plane is re-anchored through the new From node. External model events cannot satisfy the review-ID equality and clear continuation/ghost through the existing invalidation path.
- Pointer capture and ghost updates remain transient. They do not call Add, Apply, queue operations, create checkpoints, write persistence, or infer topology. Existing-end Add remains one `connect_pipe_run`; new-end Add remains the ordered atomic `[create_node, connect_pipe_run]` batch produced by `routeDraft.ts:109-134`.
- The three-axis model-to-draft conversion may import the existing `convertDisplayQuantities` service directly into `PipeViewport.tsx`; no public type, service, Rust, operation, schema, or persistence edit is needed. Use a conversion-specific generation/token, not the review gate. Accept exactly one finite `converted` result for each expected axis ID, no duplicates/extras, each in the requested unit. Any throw, unavailable item, ID/unit mismatch, non-finite value, or invalidated generation publishes no coordinate or generated identity/label and shows a local error.

## Fence and dependency result

The exact six-path fence is sufficient:

1. `apps/desktop/src/features/viewport/PipeViewport.tsx`
2. `apps/desktop/src/features/viewport/viewportRouting.ts` (new private helper)
3. `apps/desktop/src/features/viewport/viewportRouting.test.ts` (new focused tests)
4. `apps/desktop/src/App.test.tsx`
5. `apps/desktop/src/styles.css`
6. `apps/desktop/e2e/linear-authoring.spec.ts`

Current product-source inventory is unchanged at the basis HEAD; the two helper paths are absent. The planned-path whitelist check passed with no violation. A whole-worktree source-only validator call reported the already-present untracked coordination/change/evaluation evidence as violations; that is expected context outside this product-source fence and is not a product-source modification.

E0478 canonical model, E0479 units, E0480 applied-only persistence, and E0481 straight-pipe section/material fields are directly consumed as U7 records. The design introduces no bend, branch, rigid/specialty component, or expansion-joint use, so E0482-E0485 remain unconsumed and their formal `ACTIVE/TBD` state is unchanged. No dependency status is accepted or modified here.

`DESIGN_AMENDMENT_V2.md` faithfully records R1-R4 and leaves the fence unchanged. The sealed U7/I1 brief binds both frozen design documents by exact hash, carries the same six product/test paths, requires source to remain held until this terminal review and root release, preserves service/operation identity, and routes later implementation validation without adding a source or contract surface. No contradiction requiring another design iteration was found.

## Residual risk

The remaining risk is implementation-specific: native pointer capture and OrbitControls event ordering, transient Three object disposal/rebuild behavior, conversion race handling, compact layout, and actual Add/review/Apply payload identity must be established by focused, App, Playwright, build, and later native evidence. Final manager fan-in still requires a separately fresh review over the complete frozen code diff.
