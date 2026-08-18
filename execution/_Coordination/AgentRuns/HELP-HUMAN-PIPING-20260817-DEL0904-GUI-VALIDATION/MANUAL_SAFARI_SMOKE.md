# Safari Computer-Use Smoke

Date: 2026-08-17

Case: invented saved project and edited load data

Product surface: local Vite desktop preview at `http://127.0.0.1:5174/` in Safari

Evidence class: manual/computer-use smoke; **not** Playwright automation

## Visible workflow executed

1. Opened the invented `project:invented-loop-01` desktop preview.
2. Ran the initial mechanics preview. The visible product reported
   `MECHANICS_SOLVED`, five nodes, and a 4.567557 mm maximum displacement.
3. Selected `load:L-100`, chose `primitive_loads.0.magnitude.value`, changed the
   explicit unit-bearing value from `-190 N/m` to `-225 N/m`, queued it, and
   applied it through the visible Operation Apply panel.
4. The receipt reported `local_wasm_engine`; the prior solve was cleared and
   the product returned to `not started; result rows=0`.
5. Used the visible `Save local`, `List local`, and `Open by ID` controls.
   Re-selecting the same field after reopen showed `-225` with unit `N/m`.
6. Ran the edited model again. The product reported `state=completed`,
   `result_rows=0`, `MODEL_INCOMPLETE`, viewport text
   `blocked; mechanics=model incomplete; rows=0`, and the blocking finding
   `BROWSER SOLVE BACKEND REQUIRED FOR EDITED MODEL`.
7. Expanded Local preview evidence. The visible product reported
   `Browser memory preview local store; network=false; daemon=false; telemetry=false; FTS5=false.`

## Calibration

This smoke demonstrates that the intended visible local workflow can be
completed in Safari and that the product preserves the edited value while
failing closed at the browser-only solve seam. It does not satisfy the ruled
Playwright acceptance gate because Chromium never launched in the managed
runner. No canonical validation evidence or DEL-09-04 residual narrowing is
claimed from this smoke. Standard claim fence applies (F-PIP-2; DEC-081 claims
taxonomy).
