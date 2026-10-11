# Owner-held product check

G10 remains an owner-held check requiring the owner's Mac, as specified in the owner-approved Phase 4 freeze instructions (2026-10-09). Migration does not mark it complete. The bundled results from flawed pressure/joint premises were replaced under #1168; that replacement does not itself demonstrate the owner-host check.

**Procedure and open interpretation (2026-10-11).** The walking skeleton (DEL-07-11 Design, #1246) turns the check into a short native run:
- From `projects/chirality-piping`, run `node apps/desktop/native-smoke/run.mjs` (81 automated checks).
- Then open `fixtures/product_preview/rf_skew_t_cant_off_122_r1e-04.desktop_model.json` by hand.

Two readings await the owner:
- whether the desktop-shaped variant counts as "the milestone model" (it adds display fields only, and its ordinary bytes equal the committed pin in both modes);
- whether "Current display" means unchanged standing (the milestone is ordinarily `sensitive`, shown as "Needs recompute").

B7a changes the milestone's bytes, so a check run before B7a would be re-issued against the B7 head. The smoke test is evidence, not the owner's witness (F-PIP-3).

Source for the product replacement: [NUM frozen rulings: M07 option A, G10/D-3 and U3 demo lane](https://github.com/sgttomas/chirality/blob/e85d383b64/projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/NUMERICAL_INTEGRITY_T3/ROOT_RULINGS_V1.md).
