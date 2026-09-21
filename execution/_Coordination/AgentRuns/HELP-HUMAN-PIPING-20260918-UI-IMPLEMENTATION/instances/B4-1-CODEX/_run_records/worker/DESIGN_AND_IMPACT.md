# B4.1 worker design and impact checkpoint

Actor `/root/b3_manager/b4_1_table`, TASK Type2, fresh native delegated instance; no descendants. Base fd195cf4287e84572a12183169478a6f7ddf6a92. Input hashes verified in INPUT_VERIFICATION.json. Retained preparation is historical: actual existing `draftKey` is generation/type/id/field, not before-bound; Queue captures current before in the reviewed intent. This correction does not revise prior evidence.

Core owns captured edit state only, never a canonical shadow model. Rows/cells are immutable props keyed by existing entityKey. At edit start capture row/field/before/entered unit/project identity+generation and unique editor token. Validate decimal/exponent finite number syntax locally; suffix/feet-inch grammar remains deferred. Before apply recheck target/current before/current unit/generation. Controller still owns all hash/revision/busy gates, engine execution, model/history/selection/result publication. An optional readonly observer inside handleApplyIntent receives only the owned engine outcome after its existing guards; it does not imply React publication has completed. Its boolean public return and existing consumers remain.

Extract the existing grid operation builder and types; both legacy queue and direct cell use it. Direct-cell source note/rationale distinguishes direct Apply from bulk review; author remains user. Offline equivalence preserves agent attribution and requires the existing explicit batch review/apply path.

Pointer first click focuses, second/double click edits. Apply/Cancel footer visible. Enter edits then applies down file order among visible rows, Shift Enter up; Tab wraps across XYZ in view order; arrows navigate; editing left/right remain caret keys, up/down apply; Escape cancels; printable characters replace the cell to start. Native text undo is left to the input; no new global keyboard listener. Synchronous pending ref deduplicates blur/pointer requests. Completion owns its token/generation and may restore DOM focus only when its original active element still owns focus.

Sorting is a view; filtering retains an active draft row (including invalid input) visibly with a note. VirtualList stable key/pin/reveal APIs are reused with the same component across the threshold. Engine rejection reverts to canonical value and shows actual diagnostic; local invalid text is retained. Proposed integration retains EntityGrid across Tree transitions and core across family transitions. Existing bulk drafts stay lifted.

## Compatibility disposition submitted to manager

Default node XYZ interaction plus a native details disclosure, “Review multiple changes”, carrying existing node grid/label/provenance/multi-draft queue. All detail children remain mounted across collapse. Other entity-family grids remain immediately available. This is pending ROOT disposition before dependent source edits; pure core/state/adapter/tests proceed independently.

Exact consumer paths proposed for setup-only explicit disclosure entry:

- src/App.test.tsx — queued-review, apply/history, result integrity, persisted batch context, stale responses and large-model cases; retain every numeric/history/actor/basis/result assertion.
- src/App.projectHandlers.test.tsx — B3B landed save under newer edit, stale Blank and history/save/open; preserve reachability during projectBusy and exact oracles.
- src/features/model-tree/ModelTree.test.tsx — multi-draft retention across family/Tree/filter/session and filtered Queue count.
- src/features/toolkit/DisplayIntegration.test.tsx — existing source-unit grid draft/display conversion invariance.
- e2e/b3a-session-status.spec.ts — persisted snapshot Edited/save/Undo/Redo/Open.
- e2e/r2-smoke.spec.ts — two queued coordinate review intents and subsequent workflow.
- e2e/ui-foundation.spec.ts — keyboard-driven multi-draft filtered queue and virtualization retention.

`e2e/ui-foundation-dist.spec.ts` only inspects pipe tab/queue controls in the matching surface; no change expected. Existing shared workspaceTestControls/workspace-driver contain no grid-entry helper. No broader tests or production paths are requested. New default-table checks are independent of these retained review tests.

## Execution boundary

Focused one-worker Vitest and TypeScript only. No build/browser/native/full-suite/Git mutation. Two configured browser projects should each run one new B4 connected journey after slot grant. Native witness plan: actual WebKit first/second-click editing, pointer Apply/Cancel, Enter/Tab/up/down and caret arrows, type replacement, text Cmd-Z/Shift-Cmd-Z while editing, model Undo outside input, sort/filter/scroll retention; no native menu writes without separate scope. Contrast/fine appearance and full B4 grammar/other-family migration/copy/paste/row actions/Checked remain deferred.

## Applied disposition and final inventory

The earlier pending compatibility proposal was authorized by ROOT's `CONTINUATION_2026-09-19_CODEX/B4_1_COMPATIBILITY_DISPOSITION.md` in the ROOT checkout, SHA256 f317cddf86edc5faf672629f70c0575d26ff77e2b3b494d7e167687e42865827, read and verified before dependent integration. The final implementation uses an optional internal owned-outcome holder rather than an executable observer callback, preserving the boolean API and avoiding observer code in the commit path.

All seven permitted legacy test files gained only explicit `openNodeGridReview` setup calls and a local idempotent helper. No existing assertions, values, timeouts, tolerances, actor/basis/history/Current-Historical/result-identity checks were changed. `ui-foundation.spec.ts` opens its disclosure through its existing keyboard activation helper. `ui-foundation-dist.spec.ts`, shared drivers/helpers, fixture bytes, VirtualList and all excluded production surfaces are unchanged. Newly added tests are independently identified in SOURCE_FREEZE_01.json.

Final lifecycle: lazy first Grid entry, then retained mounted EntityGrid; primary node editor retained under hidden/inert family wrapper; native details retains bulk children and shows retained-draft count while collapsed. Existing source row searchText supplies table filtering. Direct missing declared length unit is shown and blocks Apply; legacy grid unit fallback stays untouched. Removed target is explicitly marked as a removed edit, cannot be selected/applied as canonical, and Cancel moves focus to a live row or the table root. Engine rejection retains its captured correction context with text reverted to captured before and exact diagnostic; local-invalid text remains unchanged.

Retirement task for later B4: migrate remaining node label/provenance and multiple-change review into common table/review grammar; remove duplicate grid only after equivalent behavioral coverage and independent review. No final B4/MVP anatomy claim is made by this temporary coexistence. Fine appearance/contrast, joined grammar, other-family migration, unit-suffix/feet-inch grammar, row/paste/copy actions and Checked remain deferred.
