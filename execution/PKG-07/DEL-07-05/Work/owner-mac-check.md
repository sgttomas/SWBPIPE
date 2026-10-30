# Owner-held product check

G10 remains an owner-held check requiring the owner's Mac, as specified in the owner-approved Phase 4 freeze instructions (2026-10-09). Migration does not mark it complete. The bundled results from flawed pressure/joint premises were replaced under #1168; that replacement does not itself demonstrate the owner-host check.

**What G10 is.** G10 is the milestone's ordinary-route native witness on the owner's Mac. The successor-panel witness belongs to B8, which stays dormant; B8's activation is owner-held (DEL-07-11 Design, scope fence).

**Decided readings.** Owner, 2026-10-24: "I accept your recommendations for each of these three: G10, W-12, W-18." For G10 this decides:
- **The milestone model.** The desktop-shaped file `fixtures/product_preview/rf_skew_t_cant_off_122_r1e-04.desktop_model.json` is "the milestone model". Its display-only additions strip back to the pinned request, it is a valid 0.7.0 document, and its ordinary results equal the pins (80,884 B sparse, 80,908 B dense).
- **"Current display".** This reads as the milestone's unchanged ordinary standing and its present display, not literal Current standing. The milestone is Sensitive: the red "Numerical health: 1 sensitive" indicator shows, and result JSON export is unavailable.

**Procedure.** The walking skeleton (DEL-07-11 Design, #1246) turns the check into a short native run:
- From `projects/chirality-piping`, run `node apps/desktop/native-smoke/run.mjs`. Its values in `native-smoke/expected.json` are the native observations the owner's 2026-10-24 ruling set.
- Then, by hand on the owner's Mac:
  1. Open `fixtures/product_preview/rf_skew_t_cant_off_122_r1e-04.desktop_model.json` (File › Open Model Document…).
  2. Solve sparse, then dense.
  3. Each solve shows 125 result rows, the Sensitive standing and the red "Numerical health: 1 sensitive" indicator. The `result:disp:N1` probe reads "Entered: 0.09426732782247199 mm" sparse and "Entered: 0.0942673274792288 mm" dense.
  4. Result JSON export is unavailable.

The smoke test is evidence, not the owner's witness (F-PIP-3). The check is the owner's act and is recorded only by the owner.

Source for the product replacement: [NUM frozen rulings: M07 option A, G10/D-3 and U3 demo lane](https://github.com/sgttomas/chirality/blob/e85d383b64/projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/NUMERICAL_INTEGRITY_T3/ROOT_RULINGS_V1.md).
