# Owner axis clarification and concrete layout proposal

Supersedes the pending-axis qualification in RETURN.md only; original brief/inputs/return bytes remain preserved. Parent /root/b3_manager reports ROOT received the owner's exact clarification: “Vertically (up/down)”. This is ROOT-reported chat text, not independently observed transport or a new worker ruling. Canonical record/hash has not yet been supplied. The compact vertical reproduction therefore matches the reported axis. It establishes whole-host vertical overflow/chaining, not failure of the separate horizontal CSS repair.

## Two distinct repair options

A. Short-grid mitigation: cap VirtualList height to its row content (five×36=180 rather than360). This removes180px excess in the measured small case, but does not meet the durable requirement for large grids, end-of-list chaining, expanded review, wrapped errors or short drawers. Do not present this alone as closure.

B. Recommended durable Grid ownership, bounded within current pane budgets:

- `shell-tree-host` / `ModelTree` in Grid mode: finite available block size, min-height:0, no whole-host scrolling. Scope to Grid mode; do not alter Tree or unrelated pane behavior.
- Stable chrome row(s): mode/filter/family controls. Height follows actual wrapping; do not hard-code the measured72/110px.
- Remaining work area: minmax(0,1fr), not intrinsic content height. Direct coordinate surface is a column of fixed header, bounded body slot, and fixed footer/error feedback. Existing horizontal scroller keeps common header/body width and controls stay outside it.
- A ResizeObserver on that local body slot supplies `min(availableBodyHeight, visibleRowCount×rowHeight)` through VirtualList's existing numeric height prop. Header/footer/error/wrapping changes resize the slot naturally. VirtualList implementation/API remain untouched. Overflowing body owns vertical scrolling; overscroll containment prevents end-of-list chaining. Short lists have no vertical work to scroll and no outer scrollable fallback.
- Retained review: a native details summary plus a bounded review work region. Its header/counts and Queue/Clear actions remain outside its own scrolling row body. In roomy views, primary and expanded review can share the remaining work region with finite minmax tracks. All existing bulk drafts and direct editor instances remain mounted; no auto-queue or rebase. Other families occupy that bounded review region, with primary nodes hidden/inert as today.

### Short Model drawer requires an explicit compact presentation

Finite-height CSS alone cannot fit an arbitrarily tall natural chrome stack into a short fixed drawer. Current compact Both chrome already uses three family-button rows, plus title/mode/filter; blindly constraining that hierarchy would leave zero usable body or hide controls. The short Model drawer was not separately measured in this diagnosis, so no fit claim is made for it.

Smallest concrete local adaptation to evaluate with ROOT before implementing that mode: compact Grid chrome (combine title/mode line; one filter line; a stable single-row horizontally scrollable family strip with every existing button retained and pointer/keyboard reachable). In a drawer too short to show both editable surfaces, expanded “Review multiple changes” becomes the visible bounded work area while the primary core is hidden/inert but mounted, rather than squeezing both bodies to zero. Closing it returns to the retained direct editor. Valid focus-leave Apply must keep current ordering; invalid/rejected editor ownership remains retained and must be visibly disclosed when returning. This changes presentation, not model semantics or pane budgets, but deserves explicit ROOT disposition because the earlier coexistence was drawn as sibling surfaces. If ROOT does not choose this compact/alternate-area behavior, return the concrete required chrome/body space conflict; do not hide controls, change pane budgets or silently introduce a minimum floor.

Hidden mounted states: ignore zero-size observer readings caused solely by hidden/inert presentation, retain last positive body height/scroll state, and remeasure on reveal without remounting the editor/list. Visible genuine zero available height is a layout conflict to solve in the compact presentation, not a reason to unmount, discard/rebase a draft or invent usable height. Filter threshold/row removal still uses existing captured identity and pinned editor rules.

Likely files: scoped ModelTree wrappers/mode state and chrome/review allocation; EngineeringTable body-slot observation/height input; scoped styles.css tracks/overflow/overscroll; existing focused table/transition tests and B4 browser spec. No VirtualList, engine, native, pane-budget or tolerance changes are proposed.

Focused proof should cover both profiles, short and virtualized models, body/control-area wheels, top/bottom chaining, wrapped error/footer, filter threshold, expanded retained review and short Model drawer. Measure fixed control/header/footer rectangles plus actual rowgroup scrollTop, table pane and viewport host/drawn canvas separately; maintain existing pointer Cancel, keyboard text/model Undo, selection, history, saved state and bulk filtered Queue oracles. Rebuilt native vertical pointer witness remains necessary. Repair is not authorized by this diagnosis return.
