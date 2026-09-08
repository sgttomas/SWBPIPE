# Supplemental native 350 N tuple

- Purpose: evidence recovery after the original successful 350 N and 500 N solve records were cleared from the dedicated store by the required history/save/reopen sequence.
- Native job ID: `backend-solve-job-3`
- Canonical run ID: `run:preview-linear-static-001`
- Submitted model hash: `sha256:04f36970d3ef54a0cf87bc35fb86667e4e80e53ba2d6a5f240e3c25e5f201de8`
- Result model ref: `project:blank-local-20260908t174230z`
- Result model hash: `sha256:04f36970d3ef54a0cf87bc35fb86667e4e80e53ba2d6a5f240e3c25e5f201de8`
- Native identity observation: `match`
- Input manifest hash: `92f5087b882f66c1d129e165f1efbfba7caf303d91c24ba9d2a29e75fa46c67d`
- Analysis-run-record hash: `89e3c7a78d0a8a3ac5683645b65f8c5914a15c3fff7d53ce976696fba89c4f30`
- Result-envelope/backend hash: `8cfcca5a389a09e949b40aa6d0dc5aa8b54aae385f150d2128d29a8263e2117a`
- Terminal mechanics status: `MECHANICS_SOLVED`
- Visible result rows: `67`
- Native solve generation: `42`
- Result provenance: native solve proof reports `seam=tauri backend job`; canonical result envelope persisted in the exact dedicated store immediately after native Save.

Evidence:

- `SUPPLEMENTAL_350_SOLVE_NATIVE.png`
- `SUPPLEMENTAL_350_SOLVE_NATIVE_AX.txt`
- `SUPPLEMENTAL_350_ANALYZE.png`
- `SUPPLEMENTAL_350_ANALYZE_AX.txt`
- `SUPPLEMENTAL_350_STORE_ROW.json`

The native solve proof exposes one model SHA together with `result model=<project>` and `identity=match`; the persisted canonical model hash is identical. The result model hash above records that observed same-model binding rather than inventing a second transport field.
