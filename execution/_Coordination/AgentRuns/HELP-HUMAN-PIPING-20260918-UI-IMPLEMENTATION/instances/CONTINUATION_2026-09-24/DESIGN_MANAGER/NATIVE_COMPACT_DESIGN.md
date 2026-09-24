# Native compact table — smallest coherent portable repair

Design return by `/root/design_manager`, HELPS_HUMANS Type 1. Read-only source and the retained native screenshots; no build/test/native interaction or product edit. Reviewed source basis is `5cfd2257cafca0fb141fc7dd5085c627d8caae9e` as bound in the manager diagnosis. ROOT owns current native resources. Only this proposal is written; the shared implementation writer decides the exact component factoring.

**Recommendation:** keep the 127 px host and existing one-row allocation, replace height-consuming horizontal bars with explicit pan controls in existing chrome, and resolve header/body column widths from one measured body-content width. This addresses the vertical-gutter drift, wide Materials height loss and the same horizontal-bar risk in toolbar/footer. No owner question is needed on the inspected evidence: 30 + 29 + 36 + 32 = 127 is feasible when horizontal access consumes width rather than another line of height. This is a layout proposal, not measured repair success.

## Concrete allocation

All four heights are border-box totals; borders/padding must fit inside them. Comfortable rows remain 36 px and Compact rows 30 px; the body receives the remaining finite height rather than a new fixed intrinsic minimum.

```text
127 px compact host, Comfortable
┌─────────────────────────────────────────────────────────────┐
│ 30  [tool rail: Tree/Grid, filter, family, Review, Queue,     │
│      Clear, Details] [tools ‹ › if overflow] [columns ‹ ›]  │
├─────────────────────────────────────────────────────────────┤
│ 29  single-line readable headers, same tracks as body       │
├─────────────────────────────────────────────────────────────┤
│ 36  full row / editor                      native vertical ↕│
├─────────────────────────────────────────────────────────────┤
│ 32  [count/edit actions/sort/status rail] [status ‹ ›] [Info]│
└─────────────────────────────────────────────────────────────┘
```

The arrows are ordinary named buttons fitting the existing 28 px control boxes. At fitting widths each conditional pair disappears; no empty pan rail or additional row is reserved. The **columns** pair remains outside the scrollable tool rail whenever columns overflow, so users never have to find the column-scroll control by first scrolling the tools. Tool and status pairs also remain outside their own clipped rails. Use visually grouped pairs with accessible names “Earlier/later columns”, “Earlier/later table controls” and “Earlier/later table status”; end buttons disable at their respective boundaries. The exact glyph is presentation, not the semantic name.

At a narrow host this uses up to 112 px for the toolbar's two pairs, leaving the remaining width for one control at a time. The current filter is 120 px and family selector 100 px; do not let their containing rail shrink the controls themselves. Pointer pan exposes each entire control, with overlap between pages, and focus automatically reveals the focused control. If actual available width is smaller than a single control, permit finer continuous pan and keep the existing Details disclosure reachable; verify the actual supported minimum rather than infer a new minimum from this sketch. Nothing requires all controls to be simultaneously visible.

### One horizontal column position; vertical rows remain native

Keep one table-owned `columnOffset` bounded to the resolved content width minus the visible row-content width. Resolve one column-track vector from the column minima and **actual VirtualList client width**, including its real native vertical gutter. Apply that same vector to header and every row; header also reserves the measured gutter space. Do not independently resolve identical fractional templates against different containing widths. Do not hardcode a scrollbar width or force a permanent assumed gutter.

Keep the VirtualList vertical scroll host at the finite viewport width. Its row content may be wider, but it must not expand the host itself or create an inner horizontal scrollbar. A table-owned clipped header strip plus equally translated wide row content is a feasible way to share `columnOffset`: the VirtualList rendered row stays in its existing wrapper/height, while the row's contents and header move by the same horizontal offset. The visible vertical scrollbar stays at the right edge of the fixed viewport. An equivalent synchronized programmatic horizontal viewport is acceptable if it proves the same clipping/gutter behavior; it must not move the vertical scrollbar offscreen. No VirtualList internals or virtualization arithmetic need change.

Pointer clicks on the column pair move by a useful fraction of the visible width, with the last step clamped to expose the last column completely. The buttons support keyboard activation; table cell navigation additionally reveals the full newly focused cell/editor. Optional horizontal wheel/trackpad input moves the same offset; ordinary vertical wheel remains row scrolling, never horizontal remapping by default. No drag-only or trackpad-only access. Pan is transient view state, not a model operation or implicit Apply. Preserve draft ownership and editor placement; if the persistent editor remains active during pan, its measured anchor and clipping must update with the same offset.

### No horizontal native footprint in any compact chrome strip

The grid, 30 px toolbar and 32 px footer must not use an `overflow-x:auto` surface whose classic scrollbar consumes their height. Replace that affordance only where the explicit same-row pan buttons and automatic focus reveal are present. Clipping alone, scrollbar suppression alone, relying on OS overlay settings or requiring the user to change preferences is not an acceptable implementation.

The tool rail retains filter, clear-filter, family, direct/review toggle and Queue/Clear as real controls with their existing semantics and disabled reasons. Keep draft-bearing subtrees mounted; panning or changing visibility must not reset inputs. A long “Return to ... retained drafts” label should have a concise visible control label plus a separate retained-draft count/status, with its complete accessible meaning preserved. Do not let long copy force the fixed rail to grow vertically.

The footer retains Apply/Keep draft and Cancel, sort-clear and row counts. Show a concise state/error summary within the rail and a permanently reachable **Info** disclosure when full text exceeds it. Put complete table error/status/retained-draft/sort feedback into that disclosure, with source ordering and appropriate live announcement intact. The existing ModelTree Details content does not automatically contain EngineeringTable edit errors, so the repair must explicitly supply them or provide a table-owned disclosure. A tooltip/title alone is not complete keyboard-accessible feedback. The disclosure can grow outside the compact allocation as a bounded popover with full text/wrapping, close/Escape and focus return; it must not replace the live editor or apply anything.

### Readable headers inside 29 px

Force header content to one line and keep it inside its own header box; do not allow wrapping into the row. Preserve column/sort semantics and visible sort direction. Increase each column's minimum to fit its full compact heading, measured at the actual font, rather than letting a long heading spill across adjacent columns. Row and header use those same minima.

For mixed-unit fields use truthful short visible wording such as `Elastic [row unit]`, `Shear [row unit]`, `Thermal [row unit]`; the cells already carry their entered unit. The complete header description must remain available on keyboard focus/Info and to assistive technology, e.g. “Elastic modulus — unit as entered in each row.” This does not select or convert a unit. For an unusually long identity/field heading, provide the full text in the same disclosure and reveal it on focus; do not rely only on hover. The added horizontal width is handled by column pan, not header height or smaller row text. Keep ordinary noncompact layouts unchanged unless the shared gutter repair is required there too.

## Why this scope is sufficient

The native screenshots show exactly two independent pressures: unequal header/body content widths under a classic vertical gutter, and horizontal bars plus wrapped headings taking space from the one-row allocation. The same existing `overflow-x:auto` rule occurs in toolbar/footer, so repairing only the table grid would leave another reproducible height-loss path. A shared narrow overflow-rail helper reused for those two chrome strips plus one table column offset is a coherent repair; it does not require a new menu system, canonical state store or engine route.

The diagnosis's measured-width approach should cover bounded and unbounded header/body alignment. The special compact horizontal control allocation is limited to the compact presentation. Full-height views may retain their ordinary native horizontal scrollbar where it has real allocated space. Do not silently apply the compact suppression everywhere.

## Writer and verification handoff

One B4 shell/table writer owns `EngineeringTable.tsx`, table-scoped `styles.css`, a small colocated overflow-rail helper if useful, and the minimal ModelTree chrome target/disclosure wiring needed to render the active table's column pan controls. Active/hidden direct/review tables must not register competing controls or steal one another's offsets/focus. App/workspaceSession, engine/model semantics and VirtualList stay outside this repair unless a concrete new dependency is returned.

Risks requiring focused evidence: measurement loops when vertical overflow appears/disappears; track width crossing column minima; stale offsets after family/filter/resize; hidden-to-visible tables; virtual and nonvirtual row clipping; editor alignment after pan; accidental blur/Apply from pan buttons; keyboard focus into clipped controls; popover focus recovery and long feedback completeness. Keep values and history unchanged by every pan/disclosure action.

The repaired candidate needs browser geometry cases with a deliberately nonzero native gutter and actual native backcheck with classic horizontal/vertical scrollbar conditions: Nodes plus genuinely wide Materials/Sections, minimum/default/expanded, both densities, empty/fitting/overflowing/virtual row counts, first/last column and row by pointer and keyboard, full header/entered units, complete full-length feedback and active editor through pan. Verify every column boundary aligns; a full 36/30 px row/editor remains; only rows move vertically; tool/footer controls never lose client height to a hidden native bar. Preserve raw failing native evidence. No source-only proof or Chromium overlay result closes native behavior.

No protected budget or accessibility criterion is relaxed, and no OS setting change is part of the remedy. If measured repaired allocation cannot expose a usable control or full row at the actual protected minimum, return that exact measurement before changing the protected host; the present evidence does not establish such an impossibility.

## Source identities

SHA-256 values below bind read bytes. Screenshot inspection used the existing images (resized display), not new native measurement.

| Source relative to P | SHA-256 |
|---|---|
| `execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/B4_MANAGER/NATIVE_SCROLLBAR_DIAGNOSIS/RETURN.md` | `82eb5300c6ba88666d80fda1b1e53d630ea76814cf06c0f8b92071d8bc9bfd02` |
| `execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/B4_MANAGER/NATIVE_SCROLLBAR_DIAGNOSIS/TASK/DIAGNOSIS_RETURN.md` | `5968d4556d67f4bfa4b00bda86b572d488baa47ffce76506937a567d673936d2` |
| `apps/desktop/src/features/workspace/table/EngineeringTable.tsx` | `2c4bedcacb145287c132069ab50bb05d74ad74518ea45a35571b5d825a590326` |
| `apps/desktop/src/features/workspace/VirtualList.tsx` | `0ef58bdc7c98188be7b1a828125ac056de25bb7646ca38bff76f3ad11d49c92c` |
| `apps/desktop/src/features/model-tree/ModelTree.tsx` | `a677e236a7c9e4396b24ebed807b22567147d69fe89c9d8daf220311b1636a8d` |
| `apps/desktop/src/styles.css` | `4096d44dad9b84a0a7edff534290cbd4001a81cc477b8783a0a5a16d733df3b0` |
| `execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/_run_records/ROOT_NATIVE/10-model-comfortable-minimum.jpg` | `f8e9bb0ccb54716427a03f6484a09f9282ff96128c5a331eea87e8fba2c0852a` |
| `execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/_run_records/ROOT_NATIVE/13-material-minimum-native.jpg` | `5eda5c076dec807444fca95118fd3801cfb35709ff58e6c0dde2d8fc6c595f20` |
