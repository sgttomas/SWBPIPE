# F4-I4 browser provenance repair brief

Status: `SEALED_FOR_DISPATCH`

## Identity and authority

- Parent: `/root/friction_execution`, WORKING_ITEMS Agent 1 F4.
- Executor: bounded Agent 2, `gpt-5.6-sol`, high reasoning, no delegation.
- Working root: `{WORKING_ROOT}`, current implementation lane at HEAD `7b73460c5e2d85a9f050344069d211fea4af7b3e` on `codex/piping-physics-ui-implementation-20260908`.
- Owner direction, as relayed by root, authorizes routine bounded repairs needed to continue this initiative without treating each reviewed patch hash as a separate approval boundary. It supersedes only the pending-approval status text in the proposal below.
- Exact reviewed plan: `{RUN_ROOT}/post_g0/browser_provenance_amendment/PROPOSED_OWNER_AMENDMENT_V2.md`, SHA-256 `fcc4a837799cc67ff8bec32866b965edf0313b3fc516611c7f460322c3b84316`.
- RF static candidate PASS: `{RUN_ROOT}/instances/RF/post_g0_browser_candidate_review_v1/REVIEW.md`, SHA-256 `e8eb280f1081e4a885b6edeba4f5bdaa86fdd2d95f4c8151859e7eb24982b1ac`.
- Prior six-path application remains bound by `{RUN_ROOT}/instances/F4/post_sweep_g0/application_v1/APPLICATION_MANIFEST_V1.json`, SHA-256 `473d3fb987d0aaf0c776b9c34f0534afb264e406410bd8e0824b429d4c6c2b8b`; its browser failure remains immutable evidence.

`{RUN_ROOT}` means `projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260908-PHYSICS-UI-IMPLEMENTATION`. `{DEL_RUN}` means `projects/chirality-piping/execution/PKG-04_Solver Core and Numerical Methods/1_Working/DEL-04-04_Nonlinear support active-set solver/_run_records/PHYSICS_UI_IMPLEMENTATION_20260908`. Resolve all paths inside `{WORKING_ROOT}`.

## Exact correction and write fence

The only source/test write is `projects/chirality-piping/apps/desktop/e2e/r2-smoke.spec.ts`, from preimage SHA-256 `3f500eb5e061ab0581123d8cc28d581fa4a17b654afc247d624f60af4dfe6f17` to post-image SHA-256 `be0650f640986a9282db43803107439f0c08d71a7c37639508e3758fa6c13248`.

Read archive `{RUN_ROOT}/instances/RU/post_g0_browser_diagnosis_v1/successors/V2_PORTABLE_PUBLICATION_V1/archives/CANDIDATE_TEST_CORRECTION_V2.diff.original.base64.json`, require SHA-256 `020ec2c68f9fc317f7de0ffb0162563555947caff4b767a8feb90c424e255c46`, decode only its `payload_base64`, and require decoded patch SHA-256 `e7fc8179bd12f00969c8692ef2bc3ee5a6209f3a1f3dfa9014d79ea0d202fbcc`. The physical `.diff` file is a pointer and must not be applied.

The patch must add exactly three lines and remove none: the disabled-before-provenance assertion, the explicit invented provenance fill for the main smoke draft, and the existing payload provenance fill in `fillNodeDraft`. Preserve every existing assertion, including the V4 numeric update.

Evidence writes are limited to `{RUN_ROOT}/instances/F4/post_sweep_g0/application_v2/children/I4/**` and `{DEL_RUN}/post_sweep_g0/application_v2/child_I4/**`. No other write is permitted.

## Execution and checks

1. Rehash the exact inputs and current one-file preimage. Verify the other five V4 post-images and the other 16 entries in the prior 17-member binding are unchanged before mutation.
2. Decode the patch to a disposable path, apply it exactly once, require the one post-image hash, and prove the scoped live diff equals the decoded patch. Do not hand-edit.
3. Run exactly the two Playwright journeys `R2 desktop preview smoke covers solve, results, report, and viewport overlay` and `R2 from-blank GUI journey authors the A12 rehearsal script` in both configured browser projects. Preserve failures exactly; do not weaken assertions or retry a failed run.
4. Update the complete 17-member source/test/fixture/generator binding by changing only the `r2-smoke.spec.ts` hash and byte count. Recompute its aggregate. Confirm all other 16 entries are unchanged from application V1.
5. Store any raw patch or command log as lossless base64 JSON from the outset. Keep active summaries, manifests, briefs, returns, and statuses portable, LF-clean, with no trailing whitespace or surplus EOF line. Use temporary output directories outside the repository for browser-generated run artifacts unless a failure requires evidence preservation; if a failure occurs, encode raw textual evidence losslessly and bind any trace by hash.

## Exclusions and return

No production behavior, physics, fixture, API, engineering decision, acceptance criterion, dependency, DAG, lifecycle, receipt, native build, full DEC-025, practitioner suite, Git commit, push, PR, CI, publication, or merge. Do not rerun the already passed Rust, Python, service, or App checks. Report any need outside this one-file routine test-flow repair before effect.

Return exact application proof, both-test/both-project results, updated 17-member binding and aggregate, outside-fence preservation, evidence hashes, residual risks, and blockers. Stop for manager validation and root-routed RF post-application review.
