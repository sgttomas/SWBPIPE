# WORKING_ITEMS Run Record — TP-APP-R2-CREATENODE-001

Date: 2026-06-10

Deliverable: DEL-07-01 — 3D viewport and centerline editor

Scope: Completion-plan Phase A3 sub-slice for explicit node creation from the
viewport editor surface.

Changes:

- Added an explicit node-geometry form to `PipeViewport.tsx` for node id,
  label, and finite x/y/z coordinate entry.
- Queued `create_node` intents with `field_path=nodes`, project length unit,
  `dimension=length`, and `provenance=user_entered_local_preview`.
- Preserved the older one-click viewport node gesture as an underspecified
  review intent that remains blocked rather than inventing geometry.
- Fixed viewport intent stacking so the form remains clickable over the
  Three.js canvas at the live browser viewport.

Validation:

- `npm test --workspace apps/desktop` passed, 28/28.
- `npm run build --workspace apps/desktop` passed.
- Browser smoke on `http://127.0.0.1:5174/` entered `node:N-150`,
  `User preview node`, and coordinates `8.4, 2.4, 2.8`; queue/apply created
  the node, selected it, and increased viewport selection targets from 14 to
  15. Timestamp-filtered browser warnings/errors after final reload: none.

Boundary:

- This tranche creates only an in-session model node through the structured
  operation seam. It does not implement canvas raycast geometry capture,
  straight-pipe connectivity, unit conversion, missing-value inference,
  protected/private data handling, telemetry, release readiness, professional
  approval, certification, sealing, authentication, or code-compliance claims.
