# Pointer and alignment limitation — existing evidence only

Written after final handback; no additional UI. Final repaired return8173d0123ac11e82331cb1752c0a50f4de94aa7436484881c43a9c32355a42c8 and manifest3320e4fae62ca302e2efb9a6923b110369030d3538837938c700b8ccbebcefba remain immutable. This addendum is intentionally outside that closed manifest.

Actual native horizontal method: app.scroll([270,430], 'right', 1) over the visible table; resulting23-horizontal-scroll.png showed no visible shift. After keyboard exposure of Z, app.scroll([260,430], 'left', 2) produced no visible shift in27-left-scroll.png. No speculative further scroll attempts.

Keyboard X→Y→Z and Return exposed Z and its editor, as25-keyboard-Z-editor.png shows. In that retained image, X/Y/Z header labels visually align with their respective body coordinate columns. This is a single visual observation after keyboard exposure, not a diagnostic measurement or independent proof of all nested scrolling alignment. The table's left identity column and left family controls are clipped in that shifted view.

Pointer-only reachability of initially clipped Z is **not established**. The pointer Cancel on the exposed Z editor passed, but keyboard first brought Z into view. Sorted by X Clear was reached by keyboard and activated with Space (32/33), not pointer. Pointer activation/reachability of the originally clipped Sort/Clear footer therefore remains **unobserved/pending**. These limits prevent a complete pointer-only minimum-fit claim.

Live app remains released for ROOT, source6dd, PID28041, binary15ff259be73d115b28321def1042c0eb70f3d4d5f61d47143e2d1d146e1fa4c5. Final current image34-final-handoff.png and AX34-final-handoff.ax.txt; Inspector closed, unchanged two-operation unsaved model described in final return. ROOT may inspect fit or grant a separate bounded followup.
