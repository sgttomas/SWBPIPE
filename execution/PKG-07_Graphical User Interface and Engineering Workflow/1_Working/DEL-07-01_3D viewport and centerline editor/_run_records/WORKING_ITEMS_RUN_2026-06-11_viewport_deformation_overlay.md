# WORKING_ITEMS Run Record — Viewport Displacement Overlay

- Date: 2026-06-11
- Agent: WORKING_ITEMS (Type 1 persona), app-integration tranche under
  `execution/_Coordination/_COORDINATION.md` Application Integration And
  Issuance Loop.
- Tranche: `TP-APP-R2-DEFORMEDVIEW-001`, completion plan Phase A6 first
  sub-slice.
- Deliverable context: DEL-07-01 (3D viewport and centerline editor);
  related context DEL-07-05 (results viewer).

## What Changed

- `PipeViewport` now accepts the current `MechanicsResult`.
- When solved results contain node `displacement_magnitude` rows, the Three.js
  scene draws an overlaid centerline and node marker set using normalized
  display offsets.
- The viewport toolbar reports overlay status. Solved fixture status is
  `available; nodes=5; max=33.211157 mm`; incomplete mechanics results report
  `blocked` and render no overlay.
- The visible boundary records
  `scale=normalized_display_offset_not_physical_length`,
  `vector_direction=TBD`, `unit_basis=mm`, and
  `professional_claim=false`.
- CSS layout was adjusted so the viewport toolbar, canvas, operation controls,
  and bottom panels reserve distinct vertical space without overlap.

## Validation Evidence

- `npm test --workspace apps/desktop`: 31 passed, 0 failed.
- `npm run build --workspace apps/desktop`: TypeScript check and Vite
  production build passed.
- `git diff --check -- . ':!init/init-prompt.md'`: passed for touched scope.
- Browser smoke on `http://127.0.0.1:5174/` using system Chrome:
  - Desktop `1280x900`: canvas `578x320`, opaque pixels `184960`, sampled
    unique colors `1000`, animation diff `5567` changed pixels.
  - Mobile `390x844`: canvas `458x240`, opaque pixels `109920`, sampled
    unique colors `1000`, animation diff `6480` changed pixels.
  - Both runs reached overlay status `available; nodes=5; max=33.211157 mm`,
    confirmed no viewport/control/bottom-panel overlap, and recorded no
    browser warnings/errors/page errors beyond Vite debug and React DevTools
    info messages.

## Boundary Review

- This is a normalized visual review overlay from displacement magnitudes
  only. It is not a physical directional deformed-shape solver or validation
  plot.
- No protected standards text, private project data, code-specific defaults,
  network/cloud/telemetry path, release claim, professional approval,
  certification, sealing, authentication, or code-compliance claim was added.
- No lifecycle state change: DEL-07-01 `_STATUS.md` remains `CHECKING`.
