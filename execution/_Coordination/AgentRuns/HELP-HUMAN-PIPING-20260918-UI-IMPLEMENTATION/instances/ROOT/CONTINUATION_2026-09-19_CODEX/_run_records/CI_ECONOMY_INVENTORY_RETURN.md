Read-only follow-up complete at **713568e8db8ad830e86afc33ef9035addb983141**. Actual attribution: **gpt-6-astra / low**, TASK Type2 under ROOT. No edits, tests/builds, UI interaction, Git mutation or delegation. Brief hash verified `9897a447635d353b84b08a4f03790d08f41f6fcf41d9bc65f1cafd5df2d2faab`.

The best immediate savings are **removing identical browser matrix repetition, then balancing by duration**. Deleting behavioral requirements is unnecessary.

| Rank | Recommendation | Evidence and preserved coverage |
|---|---|---|
| 1 | Run the explicit appearance matrix once, retaining every combination | `ui-foundation.spec.ts:669–674` generates 2 themes × 2 densities × 3 windows and sets viewport **before navigation**. Both projects use the same Desktop Chrome settings except their overridden viewport. No project-name branch exists in this spec. Old run: 24 executions, ~326s; twelve repeated executions represent ~163s observed test work. Keep all twelve distinct combinations and their unchanged canvas, retained-draft, solver, clipping and control assertions. |
| 2 | Run explicitly sized workspace-layout cases once | `workspace-layout.spec.ts:128,210,282,340,360,401,523` establishes its own starting viewport before navigation. Four resize/persistence combinations contributed ~184s across both projects; duplicate half ~92s. Other explicitly sized cases contributed ~72s; duplicate half ~36s. Keep every geometry, persistence, focus and drawn-canvas assertion. |
| 3 | Rebalance remainder jobs using observed duration, not test counts | Current completed shard 3 test step took **11m16s**; shards 2 and 4 took **2m37s / 2m31s**. Their workloads are substantially unequal. More runners alone did not fix this. |
| 4 | Route instrumentation changes narrowly by dependency ownership | Pure instrumentation changes should select their contract/controller/oracle consumers and applicable browser instrumentation tests, not unrelated solve/authoring journeys. Unknown shared helpers still require full fallback. Savings from removing hermetic viewport repetition itself remain small. |

**Important exceptions to duplication**

- Keep `workspace-layout.spec.ts:27` in both projects: line 30 intentionally gives compact a **1024×768** start, versus desktop’s 1440×920.
- Do **not** mechanically deduplicate `ui-foundation.spec.ts:963`, “selection IDs and cardinality keep the viewport rectangle fixed at three workspace widths.” It navigates at the project viewport **before** entering its explicit resize loop. Initial camera/layout state therefore differs.
- `gotoRoutedFixture`, `ui-foundation-workflows.ts:84–98`, installs instrumentation and fixture routing before navigation, without selecting a viewport. This confirms why the setup order above matters.
- Source and dist journeys remain distinct asset-delivery coverage.
- `causal-method-contract.spec.ts` contains real browser/CDP/RAF tests; it is not wholly hermetic.

The mechanical deduplications preserve distinct scenarios. Updating the collection manifest should identify which remaining test owns each formerly duplicated scenario; do not represent removed repeated IDs as skipped failures or missing coverage.

**Proposed lean required CI set**

For ordinary, reviewed UI changes with known scope, retain:

- Entire `b3-accessibility.spec.ts` barrier.
- `b3a-session-status.spec.ts` — `canonical edit/save/Undo/Redo/reopen marker follows the persisted snapshot`.
- `workspace-layout.spec.ts` — `workspace preserves the visible model during discovery, routing, property editing and analysis navigation`, both projects.
- `linear-authoring.spec.ts` — `compact blank-to-straight authoring keeps the canvas and exact Add/Apply review`.
- `r2-smoke.spec.ts` — `R2 desktop preview smoke covers solve, results, report, and viewport overlay`.
- `result-compatibility.spec.ts` — `fresh Current exports strict 0.2 result and stress-neutral evidence`.
- `gui-workflow-validation.spec.ts` — both `shared drawer menu overlap preserves ordinary Close and active menu priority: …` cases.
- `ui-foundation.spec.ts` — `[preflight] maintained fixture supports typed selection and a keyboard-authored measurement`.

Retain existing project assignments for these journeys unless separately proven equivalent. This set targets failures that prevent ordinary use: inaccessible controls, lost saved state/history, obstructed workspace, broken authoring/review, solve/result/export corruption, menu interception and selection/measurement failure.

Run affected suites **in addition**:

| Changed area | Additional coverage |
|---|---|
| Layout, appearance, drawers, inspector or shell CSS | All workspace-layout cases and full twelve-combination appearance matrix, each distinct setup once; relevant Escape/overlay cases |
| Selection, viewport, tree, properties or routing | Full `ui-foundation.spec.ts` and linear authoring |
| Results, persistence, export or compatibility | Full result/GUI journeys and applicable dist specs |
| Only added/modified source spec files | Changed specs plus barrier; deleted/renamed specs require inventory review/full fallback |
| Instrumentation/controller/measurement code | Its complete dependent contract/controller/browser-instrumentation group; protected benchmark obligations unchanged |
| Dependencies, config, WASM/solver, schemas, shared application state, unknown inputs | Full coverage |
| CI routing/deduplication changes | Full inventory validation and full coverage to establish the revised mapping |

This is an **execution-frequency change**, not equal-coverage substitution on every PR. ROOT should record that distinction explicitly. No new replacement test is needed merely because a longer existing test runs less often.

**Full validation and integration cadence**

- Preserve the full local clean-candidate DEC-025 sweep before product merge under the current loop instruction.
- Run exhaustive hosted source coverage at completed integration milestones, broad-risk changes and before any governed release/qualification checkpoint that requires it. Keep manual full mode.
- Batch related, reviewed changes into one coherent integration candidate and one final sweep/PR. Avoid separate merges solely to publish evidence generated for the same slice. Preserve independent review coverage of the final complete diff.
- Prefer milestone-triggered exhaustive runs over an unconditional nightly run when no product changes occurred. ROOT owns the exact cadence.
- Retain exhaustive tests rather than delete geometry, oracle, integrity, history or persistence assertions because they are costly.
- Do not reuse prior results across changed inputs without explicit candidate/delta provenance.

**Duration balancing**

Keep one worker per isolated runner, without `fullyParallel`. Assign by exact collected IDs or stable named groups, rather than native file-count sharding:

1. Split heavy `ui-foundation` work by named behavioral groups.
2. Spread appearance cases across groups while ensuring each distinct combination runs exactly once.
3. Distribute workspace-layout cases alongside shorter journeys.
4. Place low-cost contract tests in the lighter groups.
5. Validate disjointness and union against the expected inventory before execution.

Current completed-job log evidence, obtained through the read-only jobs log API:

- Barrier `106119698012`: ~124.7s listed accessibility tests; whole execution step 136s.
- Shard 3 `106120524039`: ui-foundation ~354.8s, controller ~80.7s, smoke ~112.9s, result compatibility ~48.9s, authoring ~36.9s, GUI ~28.2s.
- Shard 2 `106120524059`: workspace-layout ~132.4s, causal contracts ~6.4s, session-status ~5.5s.
- Shard 4 `106120524083`: workspace-layout ~132.8s, causal contracts ~6.1s.
- Shard 1 was still running when inspected; no incomplete log used.

Runner setup is additional cost: barrier setup 175s; completed remainder setup 67–75s, plus checkout. The old serial failure, current parallel run and reported local 12-minute run are unlike measurements; none establishes a speed guarantee.

**Context provenance**

Root, TASK, Piping AGENTS and loop instructions were checked against the previous consulted basis and are unchanged. Their hashes remain those in my prior return.

New/current consulted hashes:

```text
22e65b241f6c3e91620409c538a1952cb0f5ba5c501eb8a4aaeb83f8634461e0 .github/workflows/piping-desktop-e2e.yml
23421f3aa241652155d1a3975fd1bf309e724ade4ef176f90423f1d8ad06ec74 projects/chirality-piping/tools/ci/e2e_plan.py
d06b8c52bad88e5299ca9dad1feb8170f85f7b76682d837ee209060b93460db0 projects/chirality-piping/apps/desktop/playwright.config.ts
0f212e8de2eb8b5901e5fd159db5c4d220f4ba6ff9e8993c9f3987ed4bd9b754 projects/chirality-piping/apps/desktop/e2e/ui-foundation.spec.ts
b93ce5033c650265539d6ba91eb0929d9e236dd4385aabb447a94a2149ec1d97 projects/chirality-piping/apps/desktop/e2e/workspace-layout.spec.ts
24892f8e86fcb5ded0c45b4c65b67569da595a2c5d04d0aa52c171225eef5261 projects/chirality-piping/apps/desktop/e2e/gui-workflow-validation.spec.ts
a2b6fd7415e2f5de95927f8084c320dc1ea05c12549edd2e2b5d64e0da20ade2 projects/chirality-piping/apps/desktop/e2e/ui-foundation-workflows.ts
```

Remaining implementation questions: exact instrument dependency map, final shard-1 duration, final lean-set collection count, and how ROOT records hosted milestone cadence. No claim of product qualification or release follows from this inventory.

