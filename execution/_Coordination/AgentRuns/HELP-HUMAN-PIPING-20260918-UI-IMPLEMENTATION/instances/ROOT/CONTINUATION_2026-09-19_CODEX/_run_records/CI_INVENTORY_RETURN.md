Read-only inventory complete. Actual execution: **gpt-6-astra / low**, TASK Type2 child of ROOT, native delegated harness. No writes, tests, UI interaction, or delegation.

Source checkout resolved to `/Users/ryan/dev/chirality/.claude/worktrees/swbpipe-wt2`; HEAD verified `002dff0f244976f98b36517d920b3761f6f88704`. Brief hash verified `91937ad15e627f47be708c687e5ac1bea2d2bd29eab1ec663765b49afd04e574`.

**Observed cost and failure**

[Run 35496887667](https://github.com/sgttomas/chirality/actions/runs/35496887667/job/106041394519) took 35m55s; test step 32m22s. Log confirms 408 passed, 20 skipped, 2 failed.

Summed per-test durations from log, including both projects:

| Spec | Timed entries | Seconds |
|---|---:|---:|
| `ui-foundation.spec.ts` | 98 | 713.0 |
| `b3-accessibility.spec.ts` | 24 | 439.9 |
| `workspace-layout.spec.ts` | 24 | 265.7 |
| `r2-smoke.spec.ts` | 20 | 218.3 |
| `result-compatibility.spec.ts` | 2 | 95.7 |
| `linear-authoring.spec.ts` | 2 | 73.8 |
| `ui-foundation/full-cohort-controller.spec.ts` | 86 | 57.0 |
| `ui-foundation/causal-method-contract.spec.ts` | 88 | 10.6 |
| `ui-foundation/performance-targets.spec.ts` | 52 | 1.2 |
| `ui-foundation/fresh-demo-policy.spec.ts` | 8 | 0.1 |
| `gui-workflow-validation.spec.ts` | 6 | 52.1 |

These are rounded test durations, not additive job wall-time predictions.

Both failures are exactly:

`b3-accessibility.spec.ts:56` — **“covered pages exclude retained stage controls from Tab and accessibility”**

Each consumed 180s. At line 79, `workspace-dock-close.click()` was intercepted by `<span role="tooltip" class="shell-reason" id="agent-strip-reason">`. This is concrete hit-testing failure evidence, not merely slow CI.

**Smallest credible repair proof**

If final repair remains narrowly limited to tooltip pointer-event behavior:

1. Run all `b3-accessibility.spec.ts` on both existing projects. It already covers the failure, disabled-reason clipping, keyboard Close focus restoration, inspector Escape, and popup ownership.
2. Run `workspace-layout.spec.ts` on both projects to cover shared CSS, narrow slide-over, pane containment, resizing and restored focus.
3. Add these exact `ui-foundation.spec.ts` tests on both projects:
   - `keyboard splitters stay named and bounded; narrow drawers restore opener focus and hide inactive controls`
   - `decorative viewport overlays pass real canvas gestures while view controls stay interactive`
   - `workspace Escape event ownership consumed palette`
   - `workspace Escape event ownership consumed drawer`
4. Include any new tooltip regression test without weakening existing tests or adding forced clicks.

That is a focused repair proof; it does **not** by itself cover the complete original B3 PR delta. If adopting reduced required CI for the candidate, explicitly bind prior full-run results, old/new SHAs, intervening diff, selection rationale and unchanged omitted tests. Preserve the clean-candidate full local DEC-025 sweep before product merge.

If repair touches general overlay stacking, shell navigation, shared event handling, or broader CSS, expand to all `ui-foundation.spec.ts` plus `r2-smoke.spec.ts`.

**Conservative future routing**

- Keep a required aggregator whose success requires every selected job and inventory validation. Never make skipped jobs silently equal sufficient coverage.
- For shell/accessibility/layout changes, select browser group: `b3-accessibility`, `workspace-layout`, `ui-foundation`, `r2-smoke`. Route application-wide files such as `App.tsx`, global CSS, shared workflow helpers and unknown desktop paths to the full suite unless a reviewed narrower dependency rule exists.
- Solver/WASM, schemas, fixtures, dependency locks, browser/config/build scripts: full source suite and applicable dist coverage.
- Results-specific changes: `result-compatibility`, relevant `r2-smoke`/GUI journeys, plus `result-compatibility-dist`; broaden for shared application changes.
- Preserve source/dist distinction: current hosted workflow runs **source only**. Dist config separately selects `*-dist.spec.ts`: `ui-foundation-dist`, `toolkit-batch-dist`, `wasm-engine-dist`, `result-compatibility-dist`, `report-package-dist`. Source success cannot establish built-asset correctness.
- First optimize with separate runner shards, retaining one worker per runner and both viewport projects. Existing config documents teardown instability under heavily loaded local multiworker sweeps; do not simply increase workers. Use balanced shards informed by measured durations, with no fail-fast cancellation hiding coverage.
- Hermetic candidates: `performance-targets`, `fresh-demo-policy`, and apparently `full-cohort-controller` can run once in a dedicated nonviewport lane after fixture/environment isolation review. Their present duplication costs only about 58 seconds total; removing it is tidying, not the main speed gain.
- **Do not classify all `causal-method-contract` as hermetic.** It includes real browser/CDP/page/RAF instrumentation alongside pure contract tests. Split by actual fixture dependency before removing browser coverage.
- Keep explicit full hosted checkpoints for routing/config changes, dependency/browser upgrades, broad cross-surface changes, unknown paths, and periodic baseline validation. Provide manual full mode.
- Upload coverage inventories and structured results on success and failure: candidate SHA, base SHA, changed files, selected/omitted test IDs with reasons, browser version, project, outcome and skip reason. Compare union of selected IDs against expected routing inventory; fail on unmatched new paths/specs.
- Use clean checkout and lockfile-pinned Playwright Chromium. Local config automatically prefers installed macOS Chrome; local success therefore does not refute hosted Chromium failure. Record actual executable/version and use pinned Chromium for hosted reproductions.
- Do not change timeouts, performance limits, oracles, or protected skip behavior to hit a runtime target. No speed guarantee follows from this inventory.

Piping `loop/LOOP_INIT.md:194–222` expressly preserves checks and clean-candidate DEC-025 sweep. Reduced hosted routing must leave that obligation intact unless separately changed by the owner.

**Consulted file SHA256 manifest**

All paths below are relative to the verified source checkout; E2E abbreviation means `projects/chirality-piping/apps/desktop/e2e/`.

```text
d151dad92a074abebf8e6225c92c4c6e88fd586f2283377b50b5051eff39be7b AGENTS.md
1a13a5b00b3ce01ff8519efe6b46bcbe0cd6a5b7985e24282fa7efa2c57c8fb7 agents/AGENT_TASK.md
eec1b9accc5dc4485fd0aab423678bed4c4c98396dc88b9089703965eb3d2005 projects/chirality-piping/AGENTS.md
f327d5c6fe4d964a15786813f6de72170bc5e94471753846f461f622281e5f94 projects/chirality-piping/loop/LOOP_INIT.md
3a6fd86bd362eed5e1fbcda05dcde961fca8ad46cb14375ce3dd79c3872e09b7 projects/chirality-piping/software-workflow.json
1c74f6b93e3c4f08930e043b9205d1e9f88889d8a252472986e3d03bf7eb5e0c .github/workflows/piping-desktop-e2e.yml
d06b8c52bad88e5299ca9dad1feb8170f85f7b76682d837ee209060b93460db0 projects/chirality-piping/apps/desktop/playwright.config.ts
d53c01817b11718850b2b2b52e0132825931741998d4214a0d0c7e533c2b5756 projects/chirality-piping/apps/desktop/playwright.dist.config.ts
bbefadc9725ce389dbf9da378f2fa4f144b78bdedd7fe346677ca18eac3cd7c0 E2E/b3-accessibility.spec.ts
b93ce5033c650265539d6ba91eb0929d9e236dd4385aabb447a94a2149ec1d97 E2E/workspace-layout.spec.ts
86661a853b2fbd2a7c1117c319fe1b0f86a3ceb4e867e5a4f0afb2f8209cfb03 E2E/r2-smoke.spec.ts
0f212e8de2eb8b5901e5fd159db5c4d220f4ba6ff9e8993c9f3987ed4bd9b754 E2E/ui-foundation.spec.ts
572bf26e6185bf229f6d140308e6efede40f45b5b710c76dfbf138505e7b7807 E2E/ui-foundation/causal-method-contract.spec.ts
15b286c3e50e93be2b0a5e0777ced742f277ba007fa56464de42d336e07b25e8 E2E/ui-foundation/full-cohort-controller.spec.ts
e645adbc092a8f432e7f69c3b955944f31eff7bae485e6ab5b7a5349aaeebe18 E2E/ui-foundation/performance-targets.spec.ts
ab1883d6975e0804d3abb34183f9f2c8d95fb8e6a6769a96c4c4403360a57cee E2E/ui-foundation/fresh-demo-policy.spec.ts
```

Limitations: no full dependency-graph proof, no proposed router execution, no tests run, no repair diff reviewed. Hermetic classification and shard balance require implementation validation.
