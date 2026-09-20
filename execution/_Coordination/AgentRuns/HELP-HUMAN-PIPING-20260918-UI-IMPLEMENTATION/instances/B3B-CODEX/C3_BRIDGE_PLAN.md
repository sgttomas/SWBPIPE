# C3 shell bridge integration plan

Manager /root/b3_manager (WORKING_ITEMS, gpt-6-astra/high) is the sole shell integrator. ROOT supplied and confirmed this interface in the active coordination thread. Implementation waits for a clean B3B controller checkpoint and worker release. This is a separately identifiable C3 effect, not part of the B3B implementation TASK.

Basis: ROOT continuation C3_SHARED_INTERFACE_DRAFT.md SHA256 34815f05fe750db1ae36af3dff30a117d3e74d3ffde6522d32f64fb60671cea9; canvas pure helper checkpoint 3a5cf5a78a1da0a0c30d312ed0625d583e7c439d. Helper source SHA256 1e97cf7ad3467e06b45fe39085eb29f5a46bd3f74bf4849dcb618921d2ad46a3; test SHA256 6df2e9a108cee22338d704db1dc20986d37f324aad8fd46c251dac3989b04dc0. Consume this checkpoint only after B3B freezes. ROOT owns final combined review/sweep/CI/merge.

Shell-owned changes: apps/desktop/src/features/workspace/selectionSessionState.ts replaces isolateHiddenEntityKeys with isolationSelectionKeys: ReadonlySet<EntityKey> | null; workspaceSession.ts memoizes deriveViewportVisibility and owns snapshot/Show All callbacks, pruning and project replacement reset; App.tsx wires one projection; styles.css receives only the approved rules below; workspaceSession.shell.test.tsx and any exact affected shell tests assert the changed interface and semantics. No canvas file writes by shell manager; canvas owns helper, PipeViewport and renderer/resource tests. CI-owned existing E2E files remain off limits until ROOT hands them back. Any new connected C3 shell spec is distinct from B3B and requires resource reservation.

Exact final PipeViewport contract: hiddenKeys, explicitHiddenKeys, dimmedKeys, hiddenCount, isolationActive, isolationSelectionKeys are read-only. onIsolationSelectionChange(keys: ReadonlySet<EntityKey> | null) accepts the canonical snapshot; Isolate supplies non-null. Existing onHiddenKeysChange manages explicit Hide. onClearVisibility clears Hide and isolation through the session. Final bridge removes legacy isolate props/state. Null is inactive; empty set remains active even after all captured entities are deleted. New geometry outside the snapshot dims. Selection/stage/view changes retain isolation; successful project replacement clears it. Show All remains operable for explicit Hide nonempty or isolationActive, including hiddenCount zero. Picking/selection ordering and protected picking bytes remain unchanged.

ROOT approved exact CSS:
```css
.viewport-select-target[data-dimmed="true"] > svg,
.viewport-select-target[data-dimmed="true"] > span { opacity: 0.2; }
.app-shell .viewport-canvas:focus-visible { outline-offset: -2px; }
.app-shell .viewport-hidden-count {
  flex: 0 0 auto;
  min-height: 24px;
  height: 24px;
  padding: 0 6px;
  line-height: 22px;
}
.app-shell .viewport-hidden-count:focus-visible { outline-offset: -2px; }
```
Canvas provides direct label span, data-dimmed, semantic viewport-hidden-count class/testid, and text `${hiddenCount} hidden · Show all` with the existing Show All dispatcher. Button/focus/selection chrome is not dimmed. These are bounded accessibility/target-fit rules, not other appearance tuning. Verify actual fit/focus with connected checks; declarations alone are not evidence.

Cadence: focused B3B browser after controller freeze; C3 bridge connected checks after integrated viewport is available; one combined native build/witness if readiness aligns. No full lanes per internal checkpoint, no performance timing or second-profile qualification. No core/schema/Runtime/DAG changes or product acceptance. All native/browser resources require ROOT reservation.
