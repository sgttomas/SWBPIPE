# Supplemental native 500 N tuple

- Purpose: final evidence recovery after the required history/save/reopen sequence cleared the original successful result records from the dedicated store.
- Native job ID: `backend-solve-job-4`
- Canonical run ID: `run:preview-linear-static-001`
- Submitted model hash: `sha256:97bf60ed6c4541d379767a98c3c9a14132b2c3f4e6aa992bd1348b3b6b54a41b`
- Result model ref: `project:blank-local-20260908t174230z`
- Result model hash: `sha256:97bf60ed6c4541d379767a98c3c9a14132b2c3f4e6aa992bd1348b3b6b54a41b`
- Native identity observation: `match`
- Input manifest hash: `411df20dfa4ec6c6b0a967039970022e179e3ca6fb7fbf5a17605f39213a9a79`
- Analysis-run-record hash: `2c28cb397568e7f28ce54c84089fa33955fc2c534ee56da5ed751fe6c0232fa8`
- Result-envelope/backend hash: `338b776f77ed2113c68a22adf23cf21ccd36b8f454c3f3f0745cb554d318dec0`
- Terminal mechanics status: `MECHANICS_SOLVED`
- Visible result rows: `67`
- Native solve generation: `45`
- Maximum displacement: `2.375817 mm` at `node:UI-A-110`
- Maximum open-formula stress: `11.693473 MPa` at `pipe:UI-A-100`
- Result provenance: native solve proof reports `seam=tauri backend job`; canonical result envelope persisted in the exact dedicated store immediately after native Save.

Evidence:

- `SUPPLEMENTAL_500_APPLY_CLEARS_RESULT.png`
- `SUPPLEMENTAL_500_APPLY_CLEARS_RESULT_AX.txt`
- `SUPPLEMENTAL_500_SOLVE_NATIVE.png`
- `SUPPLEMENTAL_500_SOLVE_NATIVE_AX.txt`
- `SUPPLEMENTAL_500_ANALYZE.png`
- `SUPPLEMENTAL_500_ANALYZE_AX.txt`
- `SUPPLEMENTAL_500_STORE_ROW.json`

The native solve proof exposes one model SHA together with `result model=<project>` and `identity=match`; the persisted canonical model hash is identical. The result model hash above records that observed same-model binding rather than inventing a second transport field.
