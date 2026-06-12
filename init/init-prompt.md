Read `/Users/ryan/ai-env/projects/chirality/agents/AGENT_WORKING_ITEMS.md`.

Act in the `WORKING_ITEMS` persona for `/Users/ryan/ai-env/projects/chirality/projects/chirality-piping`.

Then read `/Users/ryan/ai-env/projects/chirality/projects/chirality-piping/execution/_Coordination/NEXT_INSTANCE_PROMPT.md` and follow the instructions.

A human-approved tranche is active: implement the **DEC-025 evidence sweep** (ruling in `execution/_Decomposition/SOFTWARE_DECOMP.md` §12; basis `execution/_Coordination/_DECISIONS/D-05_ci_provider_workflow.md` Option D): one deterministic entrypoint that runs the five evidence surfaces sequentially in F-4-safe order — cargo profile sweep, pytest, desktop Vitest (wasm build first), Playwright e2e, desktop production build — and writes a machine-readable summary bound to the commit hash; plus the F-4 atomic temp-write-and-rename fix in `apps/desktop/scripts/build-wasm-engine.mjs` (see `plans/VERIFICATION_2026-06-11_operation_seam_unification.md` F-4). Document it in `docs/BUILD_AND_RELEASE.md` and record run-record/SMOKE evidence.

After that lands, you have standing approval to continue executing development tranches without per-tranche approval, selected from the unblocked ruled lanes: A7 report renderer (DEC-021), the R2 from-scratch authoring set, C1 grammar implementation (DEC-022), D-03 solver (DEC-023). Decide sequencing and subagent parallelism yourself; run the evidence sweep before each push.

Do not select or begin `plans/PLAN_2026-06-12_caepipe_external_oracle_feedback_loop.md` / SCA-005 unless I instruct it.

Spawn TASK agents only for separable subscopes with explicit briefs and disjoint write scopes. Stop when any further progress requires a human ruling. Git commit and push whenever you complete a validated tranche.
