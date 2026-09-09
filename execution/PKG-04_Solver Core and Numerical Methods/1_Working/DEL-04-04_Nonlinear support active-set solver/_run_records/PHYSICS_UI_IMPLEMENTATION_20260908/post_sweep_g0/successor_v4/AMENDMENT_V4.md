# V4 held-candidate amendment

Status: `SIX_PATH_CANDIDATE_COMPLETE_PENDING_RF_AND_OWNER_ACT`

V2 and V3 remain immutable. V4 makes two bounded corrections:

1. Exact `Fraction` arithmetic replaces Decimal precision for affine interval intersection. Historical no-spring L-200 has two independently feasible normal publication cells, `24.476359` and `24.476360 N`; the actual target is therefore correctly classified as observed `24.476359 N`, matching the already calibrated retained-spring L-200 treatment. Both L-200 friction publications remain independently unique. Both L-100 pairs remain independently unique.
2. Four active UI assertions identified by RF are added: `App.test.tsx` changes one `4.927109` to `4.927112` and two `3.977299` values to `3.977301`; `r2-smoke.spec.ts` changes one `4.927109` to `4.927112`.

The complete held patch now has exactly six paths: the four V3 paths plus these two test files. V3 Rust, generated fixture, Python, and preview-service TypeScript post-images remain byte-identical. The generated fixture remains a runtime/browser and native-invocation-failure fallback asset, so the old native bundle does not establish the new asset identity. No live file changed.
