# Proposed Owner amendment V2: one-file browser provenance sequence

Status: `PROPOSED — OWNER APPROVAL REQUIRED — CANDIDATE UNAPPLIED`

The approved six-path correction in [`PROPOSED_OWNER_ACT_V2.md`](../PROPOSED_OWNER_ACT_V2.md), SHA-256 `9dfa3140d7762514479eb3983178298977bb1f5fedfdcf91253547a42b62a400`, was applied exactly. F4 froze [`APPLICATION_MANIFEST_V1.json`](../../instances/F4/post_sweep_g0/application_v1/APPLICATION_MANIFEST_V1.json), SHA-256 `473d3fb987d0aaf0c776b9c34f0534afb264e406410bd8e0824b429d4c6c2b8b`, and [`MANAGER_RETURN_V1.md`](../../instances/F4/post_sweep_g0/application_v1/MANAGER_RETURN_V1.md), SHA-256 `1f3124c4fafe9d90a4d1c01c5e01e54cf4d1c02d28385ecb337d22f38da27c45`. Root verified all 17 live bindings and aggregate `2eb48c1af8d0d6d13e64204fb0fefd70da6ed3c89493ee206d3ae2fb97c5af8e`.

The focused Rust witness passed 1/1, the full `product_physics` crate passed 138/138, Python passed 9/9, preview-service passed 16/16, two App tests passed 1/1 each, and registered-generator parity passed. The two configured browser projects then failed before the changed fixture/deformation numeric assertion because the test left intentionally blank node provenance and Add correctly remained disabled. Production behavior is correct.

RU prepared an unapplied correction to the one already-authorized test file. Candidate path is [`r2-smoke.spec.ts`](../../instances/RU/post_g0_browser_diagnosis_v1/successors/V2/candidate/r2-smoke.spec.ts), post-image SHA-256 `be0650f640986a9282db43803107439f0c08d71a7c37639508e3758fa6c13248`. The applicable patch bytes must be decoded from the `payload_base64` field in [`CANDIDATE_TEST_CORRECTION_V2.diff.original.base64.json`](../../instances/RU/post_g0_browser_diagnosis_v1/successors/V2_PORTABLE_PUBLICATION_V1/archives/CANDIDATE_TEST_CORRECTION_V2.diff.original.base64.json), archive SHA-256 `020ec2c68f9fc317f7de0ffb0162563555947caff4b767a8feb90c424e255c46`, and must decode to SHA-256 `e7fc8179bd12f00969c8692ef2bc3ee5a6209f3a1f3dfa9014d79ea0d202fbcc`. The old physical `.diff` path is a non-applicable pointer. [`RESOLVER.json`](../../instances/RU/post_g0_browser_diagnosis_v1/successors/V2_PORTABLE_PUBLICATION_V2/RESOLVER.json), SHA-256 `e9c1ffc76e3289fba2c15a48bb3f9f9eb28155ce4b3de9e57730821a83173ab4`, preserves the frozen logical hash mapping; its [`OUTPUT_MANIFEST.sha256`](../../instances/RU/post_g0_browser_diagnosis_v1/successors/V2_PORTABLE_PUBLICATION_V2/OUTPUT_MANIFEST.sha256) has SHA-256 `2a5c3656ece5965f6132873e6fffce54c9489818a58e97015baaeaa48d7e37a1`.

Fresh independent RF static review returned `PASS`: [`REVIEW.md`](../../instances/RF/post_g0_browser_candidate_review_v1/REVIEW.md) `e8eb280f1081e4a885b6edeba4f5bdaa86fdd2d95f4c8151859e7eb24982b1ac`, [`RETURN.md`](../../instances/RF/post_g0_browser_candidate_review_v1/RETURN.md) `b506324b398c512e1c05a6064b5d5e8e3c1f896f107f5f88c6835b17eee6ede0`, and [`MANIFEST.json`](../../instances/RF/post_g0_browser_candidate_review_v1/MANIFEST.json) `fed498c70e8be6088975be02ee42ec8037c7912b10c8b9227e0e14a3ccfba43b`.

## Exact amendment

Approval would authorize changing only `projects/chirality-piping/apps/desktop/e2e/r2-smoke.spec.ts` from current applied SHA-256 `3f500eb5e061ab0581123d8cc28d581fa4a17b654afc247d624f60af4dfe6f17` to reviewed candidate SHA-256 `be0650f640986a9282db43803107439f0c08d71a7c37639508e3758fa6c13248` by applying only the archive-decoded patch with SHA-256 `e7fc8179bd12f00969c8692ef2bc3ee5a6209f3a1f3dfa9014d79ea0d202fbcc`. The patch removes nothing and adds exactly three lines:

1. Assert Add is disabled before provenance is entered.
2. Fill the canvas draft with `invented_synthetic_ui_acceptance_input` before the retained enabled assertion.
3. Make `fillNodeDraft` enter each rehearsal step's existing `payload.provenance`.

The other five approved post-images remain unchanged. Existing enabled and numeric assertions remain intact. No production source, fixture, API, behavior, threshold, unit, dependency, DAG, lifecycle, or engineering decision changes.

After application, run both named journeys — `R2 desktop preview smoke covers solve, results, report, and viewport overlay` and `R2 from-blank GUI journey authors the A12 rehearsal script` — in both configured Playwright projects, then obtain a fresh RF read-only applied-diff and test-evidence backcheck. All original V2 gates and order remain required: fresh packaged-native identities and isolated default-fixture smoke, authorized source-freeze/tested commits, clean full DEC-025 and practitioner PASS, Receipt 137 and evidence-only commit, push/PR, exact-head CI, then merge. Root verified the completed RU portability resolver and archives; global candidate-whitespace validation passed with zero findings, and path-anchor validation passed across 4,285 surfaces with zero findings. No native run, clean full sweep, Receipt 137, or Git action has occurred for this amendment.

If approved, this amendment extends the active exception only for the exact one-file three-line post-image above through the same accepted corrected-source fan-in and expiry defined by V2. It does not authorize any additional path or waive any gate.

## Requested Owner act

> I approve the one-file three-line browser provenance amendment in `browser_provenance_amendment/PROPOSED_OWNER_AMENDMENT_V2.md`, with its exact post-image, required focused browser checks, RF backcheck, preserved gates, and unchanged expiry.
