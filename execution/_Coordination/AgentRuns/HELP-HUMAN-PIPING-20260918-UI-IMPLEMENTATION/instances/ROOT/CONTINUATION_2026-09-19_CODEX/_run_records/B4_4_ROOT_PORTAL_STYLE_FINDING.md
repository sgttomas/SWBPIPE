# ROOT source finding: detached enum popup loses shell tokens

Source inspected: Sections checkpoint 54176d28c5a5a25f4fe61ff8debc66d9d7b0f332.
The later 3687e4450a3e2cfa37616b9b1e7ee7b28075e1fe checkpoint changes
two tests only; this production finding applies to both.

`useEnumEditor.tsx` portals the new popup to `document.body`.
`styles.css` defines `--ui-border`, `--ui-surface` and `--ui-text`
on `.app-shell` (lines 3402–3414). The portal is not a descendant
and therefore does not inherit those custom properties. Its new CSS uses
all three without fallbacks. The option hover/selected rule also names
`--ui-surface-raised`, which has no definition in the desktop source;
the shell's existing raised alias is `--ui-raised`.

This is a concrete source-level styling defect. Browser computed colors,
background/border and selected-option visibility still need observation;
ROOT has not run a browser witness or claimed one. Existing CompactSelect
uses root-scoped design tokens for its own body portal and is a local
reference, not a source target for this repair.

ROOT routed a bounded repair to the owning manager: use a correct token
scope for this popup, preserve keyboard/pointer/lifetime behavior, and
add meaningful light/dark browser coverage of visible popup/selection
styling. No token-system or generic-picker rewrite is authorized. Retain
the source diagnosis, actual repair and affected verification. The
ordinary independent source review remains required.
