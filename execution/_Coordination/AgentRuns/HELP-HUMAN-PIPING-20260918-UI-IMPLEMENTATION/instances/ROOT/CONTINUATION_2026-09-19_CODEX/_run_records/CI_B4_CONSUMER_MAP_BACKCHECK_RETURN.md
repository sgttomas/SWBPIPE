**PASS — complete two-file delta `ce625f5c…ab3b057e…`.** No actionable findings.

- Layout and authoring changes now select the complete B4 browser spec.
- Both new regressions exercise an existing production path modified alone, with the spec present before the diff; accessibility and lean coverage remain asserted.
- Shared table paths retain the unchanged conservative full fallback.
- No prior oracle, threshold, selection mode, or exemption was weakened.

Bindings:

- Brief: `d5d29e3d72205b7ca232d7643b6addbb49f19466cf2189ca3fec963d24c74f25`
- Complete diff: `69b0bcbadaa79e49b8a08b7a6e393ca553c73eb2063b9e9f71a03aeb367bc982`
- Selector: `ce6f619b4cc9629aaca6338f155f9093c5b1b8006cc53b575f2d49459402eff6`
- Policy tests: `b9e5bc85bf02536fe9aaf7abe3104f685171b9f0de7dd9d68ec632e782189e8e`

Used pinned bytes because wt2 had advanced to `f4c17739…`. Later B4/native changes are excluded. ROOT’s reported 31 passing policy tests were not rerun; final collection and combined gates remain.

Same independent TASK, **Astra/xhigh**, with prior context retained. No tests, builds, UI, edits, mutations, network use, or delegation.
