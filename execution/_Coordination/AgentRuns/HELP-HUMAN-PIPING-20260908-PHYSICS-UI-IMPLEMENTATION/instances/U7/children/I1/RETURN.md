# U7-I1 candidate return

Status: `CANDIDATE_COMPLETE_PENDING_PARENT_VALIDATION`

Implemented the frozen native straight-route and inline inspector Apply slice in the exact nine-file fence. Route Add now builds and service-validates a frozen single intent for an existing endpoint or the exact ordered atomic `[create_node, connect_pipe_run]` batch for a new endpoint. Apply accepts only that registered frozen object at its original model revision, epoch, and hash; edits, selection/model changes, create/open, undo/redo, Cancel, stale callbacks, and busy overlap invalidate or reject it. A committed new-end route creates one undo checkpoint; blocked or stale applications publish neither member and create none.

Typed coordinates remain authoritative and zero remains valid. The UI discloses `XZ @ Y=0`, keeps the canvas mounted beside a compact scrolling authoring palette at 1024×768, and shows a bounded diff preview with Add and Apply simultaneously reachable. Node-only blank models save/reopen as `MODEL_INCOMPLETE`. Pipe dimensions remain inline with an existing material reference, and the existing separate section-assignment workflow remains intact.

The property inspector retains Queue and Validate and adds direct Apply for the exact currently selected intent. Its existing App service route preserves units, provenance, rule-input fields, persistence metadata, undo/redo, and solve-result invalidation.

## Changed product paths

- `projects/chirality-piping/apps/desktop/src/App.tsx`
- `projects/chirality-piping/apps/desktop/src/App.test.tsx`
- `projects/chirality-piping/apps/desktop/src/features/viewport/PipeViewport.tsx`
- `projects/chirality-piping/apps/desktop/src/features/viewport/routeDraft.ts` (new)
- `projects/chirality-piping/apps/desktop/src/features/viewport/routeDraft.test.ts` (new)
- `projects/chirality-piping/apps/desktop/src/features/model-tree/PropertyInspector.tsx`
- `projects/chirality-piping/apps/desktop/src/features/model-tree/typedInspector.test.tsx`
- `projects/chirality-piping/apps/desktop/src/styles.css`
- `projects/chirality-piping/apps/desktop/e2e/linear-authoring.spec.ts` (new)

Only these nine product paths were written. Concurrent Rust, decision, Change, and sibling run-record changes visible in the shared worktree were left untouched. `git diff --check` over the tracked members exited 0, and an explicit trailing-whitespace scan over all nine members reported no findings.

## Seven-row consumer-proof map

| Dependency | Current Apply-flow behavior | Exact test evidence |
|---|---|---|
| `DAG-002-E0486` / canonical model schema | Stable node/pipe/load/component IDs and schema field paths are carried through service operations; new-end publication is exactly create-node then connect-pipe, and inspector Apply forwards the exact selected target and field. | `routeDraft.test.ts`: **builds one exact connect intent for an existing endpoint** and **builds the exact atomic create-node then connect-pipe batch and preserves zero coordinates**; `App.test.tsx`: **validates and applies a new endpoint as one ordered batch with one checkpoint**; `typedInspector.test.tsx`: **keeps Queue and Validate and forwards the exact displayed force intent once to inline Apply**. |
| `DAG-002-E0487` / persistence and round trip | Add review is bound to the original revision/epoch/model hash; edits and Cancel invalidate it, delayed callbacks cannot publish, and a model revision clears the frozen review. Accepted node/load state survives save/open while transient review and session history do not masquerade as durable state. | `App.test.tsx`: **discards a delayed Add result after Cancel and publishes no endpoint or checkpoint**, **invalidates a frozen hash-bound route review after another model revision**, and **saves and reopens an incomplete node-only model without inventing prerequisites**; `linear-authoring.spec.ts` saves/reopens both the incomplete node-only model and the final `500 N` load edit. |
| `DAG-002-E0488` / pipe-section and component schema | Straight-route payloads preserve material, OD, wall, and y-reference fields; no section is invented. Existing section assignment and component authoring remain service-routed after direct route Apply. | `routeDraft.test.ts`: exact pipe payload assertion; `App.test.tsx`: **queues and applies explicit straight pipe connectivity through the structured operation seam**, **creates an expansion joint on the exact selected pipe...**, and **clears a queued tee...**; `linear-authoring.spec.ts` applies a separate shared-section assignment after route creation. |
| `DEP-007-02-004` / units | Node/pipe reviews carry explicit length units and inspector Apply carries the exact force value, `N` unit, and `force` dimension. Missing/TBD units fail draft construction rather than being defaulted. | `routeDraft.test.ts`: exact `{value, unit}` section and zero-coordinate payload assertions plus rejection cases; `App.test.tsx`: reviewed node/pipe diffs assert `[m]`; `typedInspector.test.tsx`: exact `350 -> {value:500,unit:"N"}` Apply intent. |
| `DEP-007-02-005` / material provenance | New nodes and pipes require and preserve explicit provenance, and pipe material must resolve to an existing material ID. The applied inspector/tree state exposes the same values. | `routeDraft.test.ts`: exact material and `synthetic_ui_acceptance_input` payload assertions plus missing-material rejection; `App.test.tsx`: straight-pipe integration asserts `material:invented-carbon-steel` and `user_entered_local_preview`; the bounded Playwright journey enters explicit material/node/pipe/load provenance and round-trips the result. |
| `DEP-007-02-006` / rule-pack schema | Inline Apply exposes and forwards the selected user-entered rule field without synthesizing a code value or mutating the source model before service application. Rule-pack reference/missing-input fields remain visible in the existing contract surface. | `typedInspector.test.tsx`: **keeps user rule fields visible and forwards the exact selected rule value to Apply**, asserting `modifiers.sif_user_value.value`, dimensionless/none, and unchanged source model; full `App.test.tsx` test **renders the engineering workspace from invented local fixtures** asserts the rule-pack reference surface and `RULE_PACK_REQUIRED_INPUTS_MISSING`. |
| `DEP-007-02-007` / private rule-pack checksum/source | Direct Apply does not copy or invent private rule-pack payload/checksum data; the existing reference-only contract remains visible and redacted while route/load changes use their own explicit provenance. | Full `App.test.tsx` test **renders the engineering workspace from invented local fixtures** asserts `checksum=TBD_private_rule_pack_not_loaded`, `private_payload_redacted=true`, references/checksums-only policy, and no protected/private payload; the new typed rule-field Apply test proves only the exact public user-entered modifier is forwarded. |

The hash/revision/cancel protections are also exercised together by the first two rows: the route review renders its `sha256:` basis, editing disables it, a concurrent model commit removes it, and delayed validation after Cancel produces no entities or checkpoint. Repeated Apply is covered by the ordered-batch App test, which double-clicks Apply and observes one publication and one checkpoint.

## Verification

- `npx vitest run src/features/viewport/routeDraft.test.ts` — 8 passed.
- `npx vitest run src/features/model-tree/typedInspector.test.tsx` — 9 passed.
- `npx vitest run src/App.test.tsx -t "native straight-route Add and Apply"` — initial focused slice 5 passed; the expanded slice is included in the final full-file result.
- `npx vitest run src/App.test.tsx -t "queues and applies explicit node deletion|queues and applies explicit viewport node geometry|captures viewport pointer geometry|queues and applies explicit straight pipe connectivity|picks straight pipe endpoints|creates an expansion joint|clears a queued tee"` — 7 passed after converting legacy queue/dock setup expectations to direct reviewed Apply.
- `npx vitest run src/App.test.tsx` — 132 passed, 0 failed, 212.64 s.
- `PLAYWRIGHT_WORKERS=1 npx playwright test e2e/linear-authoring.spec.ts --project=chromium-desktop` — 1 passed, 0 failed, final 12.9 s after the final compact-preview CSS change; the preceding green run was 13.0 s.
- `git diff --check -- <tracked fence paths>` — exit 0; `rg -n '[[:blank:]]+$' <all nine paths>` — no findings.

The first full App run had 124 passes and seven legacy viewport tests still expecting queue/dock application; those tests were updated to the direct reviewed seam without weakening state, receipt, checkpoint, or undo/redo assertions. Playwright remediation corrected an ambiguous radio locator, the post-open session-history expectation, navigation to the Review panel before Undo, and the exact dotted operation ID. No product/API/backend defect or fence expansion was needed.

## Visual witness and handoff

`COMPACT_ROUTE_REVIEW_1024x768.png` (SHA-256 `eeebc72db4f1899c216328aefef9bc75d175ebff6a3840b5d5f472e4d0ecc107`) is the bounded browser witness. It shows the 1024×768 canvas, compact palette, Add route control, frozen atomic-batch hash/diff card, and reachable Apply button together. It is visual fan-in only and is not a native GUI or functional substitute.

Exact handoff state: candidate implementation complete; targeted JS/TS and the specifically authorized bounded browser journey are green; no unresolved product/API/fence blocker. Native packaged-GUI witness and broader root-coordinated validation remain separate parent/root work. This return makes no acceptance, release, engineering-compliance, or professional-reliance claim. No commit, push, or delegation occurred.
