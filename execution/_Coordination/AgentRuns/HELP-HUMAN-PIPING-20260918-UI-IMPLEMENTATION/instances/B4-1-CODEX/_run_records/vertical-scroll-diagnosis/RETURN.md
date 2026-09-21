# Read-only vertical scroll-owner diagnosis

TASK /root/b3_manager/b4_1_table, requested Astra/low; no descendants. Assigned wt3 HEADc8f8e503383e15b42b171c4957c6f458d3697150, product1ad964e30b08bed57bba8f08e3a75e8c883c8b0b. Sealed addendum cbe782f512b1f4fc87c8dffc3aa94f27096c81328b5264fa8624044d15983b1e and INPUTS5030ba28a42e069b2640b6f41c1b6344adce6353ebf8aa2fd6aa41f8c363a58b read/verified before actions; all listed source hashes reverified unchanged afterward. Owner correction origin/hash verified as supplied. Software-defect-diagnosis applied under its retained f2815587… hash. No maintained edits, injected styles/DOM changes, Git mutations, tests/build/native/CUA/full-suite work. Native31977 untouched.

## Actual browser result

Pinned Chromium148.0.7778.96, executable and binary/script hashes in browser/RUN.json, sequential one-browser diagnosis under existing shared lock and5174. Each scenario starts a fresh context in Both+Inspector with the five-node invented model. Real input is `page.mouse.wheel(0,600)` over an exact recorded pointer/hit target; no scroll setter. Two profiles ×body/control comparison only; no broad matrix.

| Profile | Target | Actual moving owner | Filter y | Family controls y |
|---|---|---|---|---|
| 1440×920 | grid body | none |187.796875→187.796875|251.1875→251.1875|
| 1440×920 | family controls | none |187.796875→187.796875|251.1875→251.1875|
| 1280×800 | grid body | `.shell-tree-host`, scrollTop0→68 |187.796875→119.796875|251.1875→183.1875|
| 1280×800 | family controls | `.shell-tree-host`, scrollTop0→68 |187.796875→119.796875|251.1875→183.1875|

At compact, host clientHeight667/scrollHeight735, ModelTree full bounds y109→41 with height734.984375; ModelTree itself scrollTop stays0. At desktop, host and ModelTree client/scroll height787/787, so there is no outer overflow. Wrapped family controls are110px high compact versus72px desktop (38px difference). Both rowgroups reserve360px client/scroll height for five36px rows/content180px, with inline overflowY:hidden. Header, grid, footer and review disclosure all move together with outer host at compact; rowgroup and horizontal scroll positions do not change. Viewport host DIV and actual drawn canvas full bounds remain unchanged in these scenarios; no drawing-buffer/D72 or timing qualification follows.

Raw full bounds/client/scroll dimensions/overflow/ancestry, exact hit HTML and pointer coordinates: browser/{desktop,compact}-{grid-body,family-controls}.json. Before/after screenshots and trace per scenario are adjacent; stdout.txt summarizes. Compact body pointer[168,502.1875] hits node:N-120 rowheader; control pointer[168,263.1875] hits Pipes. Inspected compact-grid-body-after.png corroborates clipped upper controls. Full command is existing `with_e2e_lock.sh node <this-dir>/probe.mjs <this-dir>/browser`, executed from assigned apps/desktop; RUN.json retains actual Node argv/cwd/source/executable/version/hashes. The wrapper path is the unchanged owning-run tools/with_e2e_lock.sh.

## Cause and confidence

High confidence for this browser vertical behavior: fixed360 body slot plus wrapping/other fixed controls exceeds compact host; overflow:auto on shell-tree-host permits the whole ModelTree to scroll. `.shell-tree-host > .panel {min-height:100%}` supplies a minimum, not a finite maximum. `.model-tree {overflow:auto}` is another potential owner, but measured movement here is its parent host, not ModelTree. VirtualList correctly makes the short nonoverflowing rowgroup overflowY:hidden; the wheel therefore finds the outer scrollable host. The control-area comparison produces the same68px shift, isolating the outer-owner mechanism from table input handling.

This can explain a vertical version of the owner's corrected report and agrees with the cited post-owner screenshot direction. The owner’s actual axis remains pending; these controlled vertical gestures do not prove that axis, a simultaneous two-axis trackpad gesture, or exact native offsets. Native browser chrome/viewport height differs; screenshot pixels are not logical dimensions. Horizontal repair remains separate bounded evidence; no full fitPASS is inferred.

## Smallest robust stable-controls proposal — not implemented

1. Give Grid mode a finite height within the existing table pane. Use targeted ModelTree/Grid wrappers with flex/min-height:0 (or equivalent minmax tracks), bounded to the existing shell-tree-host height; disable whole-host/whole-ModelTree scrolling only for that bounded Grid layout. Preserve Tree mode’s established own virtual list. Do not change pane/canvas allocation or minimum budgets.
2. Reserve title/mode/filter/family controls and table header/footer as non-scrolling rows. Allocate remaining internal height to the coordinate body, retaining its established horizontal owner/header alignment. Feed the measured body slot height through VirtualList’s existing numeric `height` prop, optionally capped by visible row content to avoid a forced180px blank area for five rows. A narrowly local ResizeObserver on the body slot can supply that height; no VirtualList implementation change, global wheel interception, hard-coded viewport budget, or model store is needed.
3. Make rowgroup the vertical owner for overflowing rows and contain its vertical overscroll, so reaching either end cannot move external controls. Short content produces no row scrolling and no outer host to chain into. Merely reducing360 to180 for short rows or adding sticky filter text alone is insufficient: larger grids/end-of-list chaining and review expansion remain.
4. Treat native review details as a bounded sibling work region when expanded. Keep its summary and Queue/Clear controls outside its scrollable grid body; share remaining internal work height through minmax/flex rows rather than growing the outer ModelTree. Other families use that same bounded legacy-grid region while the hidden primary core remains mounted. Retain all lifted bulk drafts and direct edit state; do not unmount on disclosure/family/Tree changes or auto-queue. Exact expanded-region allocation should be part of the repair design, not an implicit overflow fallback. If a proposed allocation cannot keep both edit/review controls reachable at the accepted minimum, return that concrete conflict rather than introducing a new pane budget.

Likely maintained surfaces for repair: ModelTree.tsx mode/body/review wrappers and local sizing; EngineeringTable.tsx body-slot height integration; scoped styles.css finite tracks/overscroll; focused table/transition tests and existing B4 spec. VirtualList API/implementation, engines, selection/history/project handlers and native menus remain unchanged. Other family/review consumers are affected by bounded sizing, so retain their six source journeys and all original oracles. Captured row/before/unit/generation and pinned invalid editor still own edits; resizing/filter threshold changes must never retarget or discard them.

## Acceptance checks to request with repair

- Actual vertical wheel in both profiles over small body and controls: filter/families/header/footer stay at their original full bounds; no outer-host movement. Short rows need not scroll.
- Large fixture: body scrollTop changes, fixed controls do not; top/bottom additional wheel does not chain. Filter across virtualization threshold, pinned invalid/engine-rejected draft and correction/Cancel survive with same captured identity.
- Expanded/collapsed review and other families: fixed controls reachable, internal grid scroll works, retained draft count/filtered Queue and Tree/family round trips preserved.
- Keep current horizontal Z-pointer alignment/Cancel/history/persistence/selection tests; assert table pane, viewport host and drawn-canvas bounds separately without claiming D72.
- Later rebuilt native WebKit at actual configured minimum, Inspector open: real vertical wheel small/large/top/bottom, pointer footer and type controls, disclosure expanded; owner’s axis clarification must inform the final witness. Existing source passes cannot substitute for this.

Browser/server/lock explicitly released; VERIFICATION_AND_CLEANUP.json shows no5174 listener, no shared lock, no pinned Chromium root process and unchanged source hashes. The live native app remains ROOT/owner-owned and untouched. This is diagnosis/proposal only; repair disposition and owner-axis answer remain with parent/ROOT. No B4 acceptance, fit closure, C4/CLI/Runtime activation or release claim.
