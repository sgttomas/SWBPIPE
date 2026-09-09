# Python and UI test successor review

## Identity and disposition

Resume the independent `/root/combined_source_review` Agent 2 as `TASK + software-code-review`, using `gpt-5.6-sol` with `high` reasoning and no delegation. The initial reviewer was fresh and independent from implementation. Read `{REPO_ROOT}/agents/AGENT_TASK.md`, `{REPO_ROOT}/skills/software-code-review/SKILL.md`, and `{REPO_ROOT}/docs/SOFTWARE_WORKFLOW_PROFILE.md`. Record actual exposed identity and configuration.

Basis: `ab1016e653afd7db6e0d8243b83492c4a54ede31`. Root routed three stale test assertions for bounded correction; no production, schema, or fixture effect is authorized or inferred:

- `{REPO_ROOT}/projects/chirality-piping/tests/product_preview/test_product_preview_service.py`
- `{REPO_ROOT}/projects/chirality-piping/tests/test_results_schema.py`
- `{REPO_ROOT}/projects/chirality-piping/apps/desktop/src/App.test.tsx`

P5 owns the two Python files and U7 owns `App.test.tsx`. Before review, locate their frozen terminal returns and manifests under `{RUN_ROOT}/instances/P5/**` and `{RUN_ROOT}/instances/U7/**`; resolve and record every subject path, final hash, diff hash, and evidence hash. Do not review mutable or incomplete inputs.

Bind the failed G1 sweep `{PROJECT_ROOT}/validation/evidence/sweeps/SWEEP_20260909T085517Z_ab1016e653af.json` (SHA-256 `591c0f297abb12720d6f86a431e6543d205292767669b87f765f3849afe1b489`) and `{PROJECT_ROOT}/execution/_Change/VIEWPORT_ROUTING_20260909/DEC025_REMAINING_SURFACES_DIAGNOSTIC_V1.json` (SHA-256 `a5770d7670d7dd1f2b9705da46b751aeefbb76bdb2e2aec73b65e69413a61c07`). The diagnostic's G3 development run had 30 individually passing tests but exit 130 after manual interruption and is not PASS: it omitted the registered runner's `PLAYWRIGHT_WORKERS=1`, launched eight workers, and encountered the known configuration teardown hazard. G3 distribution `3 PASS` and G4 build PASS are diagnostic evidence only. Infer no product defect or configuration repair. The next clean full runner must use its registered single-worker setting.

## Review

Review 100% of the final three-file diff. Consume the accepted predecessor chain: `{RUN_ROOT}/instances/FINAL_REVIEW/review/ACTIVE_REVIEW_POINTER_V2.json` and `{RUN_ROOT}/instances/HEADLESS_TEST_REVIEW/review/MANIFEST_V1.json` (SHA-256 `c718b3cc98e950ba1af82ac65c149d5ce59b1d2a4def4c2f38d3f49f4d4207c2`). Verify its eleven frozen paths remain unchanged except the explicitly reviewed `App.test.tsx` test delta. Combined coverage must identify thirteen unique changed paths.

Confirm the changes are test-only and that runtime, schema, fixtures, and production behavior are byte-unchanged. Require truthful and meaningful category oracles without weakening required-field, enum, mechanical, pressure, selection, disclosure, or combination checks. Trace expectations to actual runtime outputs; flag broadened predicates, deleted coverage, or assertions that merely restate implementation.

Return `PASS` only with zero actionable findings; otherwise return concrete file-and-line findings. Full clean G0-G4, practitioner validation, self-check, and Receipt138 remain required and unfulfilled.

## Permissions and output

Read-only repository inspection and the active skill's read-only helpers are allowed. Do not run tests, Cargo, builds, browsers, servers, or ports. Do not write source, tests, Git, or manager packets. Do not delegate.

Write only `{RUN_ROOT}/instances/PYTHON_UI_TEST_REVIEW/review/**`: a review of at most 300 words, a compact exact manifest, and the required TASK run record. Preserve failed permitted commands. Validate hashes, portable anchors, JSON, LF, and whitespace.
