# WORKING_ITEMS Run Record - TP-UNITS-B2-UNITPICKERS-001

Date: 2026-06-12
Agent: WORKING_ITEMS
Plan item: B2 unit-aware I/O
Deliverable: DEL-07-02 Model tree and property inspector

## Scope

Added visible unit selectors to the Property Inspector material and
pipe-section creation forms:

- Section creation exposes `Length unit`.
- Material creation exposes `Modulus unit` and `Thermal expansion unit`.
- Existing unit-basis labels and the `Unit basis` status panel now follow the
  selected unit.
- Browser preview remains model-metadata-only with no fallback catalog.
- Desktop/Tauri mode uses the accepted DEC-018 catalog route for options.

## Evidence

- `apps/desktop/src/features/model-tree/PropertyInspector.tsx`
- `apps/desktop/src/App.test.tsx`
- `apps/desktop/SMOKE.md` TP-MAC-134
- Corresponding DEL-02-02, DEL-16-02, and DEL-16-03 run records.

## Validation

- Focused desktop Vitest passed 165/165.
- Full desktop Vitest passed 216/216.
- Desktop production build passed with the pre-existing Vite chunk-size
  warning.
- Playwright R2 smoke passed 2/2 after wasm engine build.
- Tauri Rust tests passed 32/32.

## Boundaries

No direct model mutation, durable persistence, project-wide unit-system
change, hidden browser fallback catalog, protected/private data, release
readiness, professional approval, certification, sealing, authentication, or
code-compliance claim was added.
