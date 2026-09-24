# C4 ordinary-check admission assessment

Read-only inspection by `/root/design_manager`, HELPS_HUMANS Type 1, at fixed commit `1c8f04039b5f41812ebd879fc80d810392123072`. Only this file written; no tests, builds, product/test/instrument edits, UI or Git mutation. `P` means `projects/chirality-piping`; `A` means `P/apps/desktop`. This supplements OWNER_RULING_APPLICATION.md; it does not change the adopted C4 semantics or protected D-72 criteria.

## Answer

**C4 has a viable ordinary merge-check path without changing protected first-profile fixtures/helpers/oracles or adding benchmark-only behavior.** Existing ordinary UI tests need legitimate updates from the old boolean toggle to the adopted three-mode controls; leaving every ordinary test unchanged is not a viable expectation. No required current-product assertion of universal Budget 138/230 or Off=0 was found in the inspected maintained suite. This is a static admission assessment, not a prediction that unimplemented code passes or a claim that checks ran.

The subtle distinction is that ordinary source E2E does include the old instrument's self-tests. Those tests preserve its historical Off=0/total-cap rejection semantics **against constructed or retained records**, not the current app. They can remain valid while the product changes. The actual timed benchmark is an explicit opt-in configuration, outside ordinary E2E/DEC-025 discovery.

## What each check actually covers

| Class | Source at the fixed commit | C4 consequence |
|---|---|---|
| Required merge orchestration | `P/software-workflow.json`; `P/tools/release/run_evidence_sweep.py:167–235`; desktop package scripts | Desktop build/Vitest plus complete DEC-025 source/dist Playwright where required. This task offers no waiver or reduced-gate substitute. |
| Ordinary source/dist discovery and CI | `A/playwright.config.ts:39–56` discovers ordinary spec/test files, excluding dist; dist config admits `*-dist.spec.ts`. `P/tools/ci/e2e_plan.py:54–58,76–98` inventories spec/test files and explicitly never executes benchmark importers; `:108–113,153–163` maps viewport to authoring consumers. Unknown/shared inputs can require full coverage. `.github/workflows/piping-desktop-e2e.yml` calls that planner and runner. | Do not alter CI selection to evade an affected test. Whether the actual C4 diff selects full or affected hosted coverage is determined later from that complete diff. Local full sweep still includes ordinary instrument self-tests. |
| Immutable first-profile fixtures/protocol | `A/e2e/ui-foundation/fixture-manifest.json` pins the historical 80 selected-label cap, OFF point protocol, fixture/member hashes and oracles. `README.md:8–20` describes fixture verification and separate timed invocation. | Preserve bytes and historical claims. C4 does not need to rewrite generated fixtures, point/box oracles or historical helper admission rules. |
| Legacy 80-cap helper unit test | `A/src/features/viewport/viewportSelection.test.ts:236–255` calls `prioritizedLabelKeys` directly and expects exactly 80. It is not a live PipeViewport test. | Keep the old helper/test unchanged as previously prepared; production C4 uses the new policy module instead of calling that historical capped helper. No fake runtime switch or benchmark detection is needed. |
| Untimed instrument metadata self-tests | `A/e2e/ui-foundation/characterization-commands.ts:47–78` rejects renderedCount > budget and enabled=false with renderedCount ≠ 0. `full-cohort-controller.spec.ts:1022–1034` feeds a constructed `metadataFixture()` and verifies rejection; `:1195–1212` verifies phase rejection with constructed records. `:1097–1128` also uses constructed records; `:1215–1238` is opt-in, hash-bound retained evidence. | These are tests of the old validator, not current C4 snapshots. Preserve their rejection behavior. The inspected spec has no live-app `page.goto`/`readCandidateDiagnostics`/`captureBoundary` call. A newly implemented product mode cannot itself change those constructed values. |
| Other untimed instrument self-tests | `performance-targets.spec.ts` scores synthetic evidence, including labelsOn expectations; causal-method tests use constructed DOM/data-page fixtures. Some historical-record validations require explicit D70 input environment variables and otherwise skip under their existing contracts. | Preserve existing self-tests and opt-in historical-record behavior. No new skip or synthetic replacement of a live test is proposed. |
| Ordinary diagnostics unit test | `A/src/features/workspace/uiDiagnostics.test.ts:68` contains `{enabled:true, renderedCount:3, budget:80}` in constructed state. | It is not an enforced live-product count maximum. Preserve attachment/freeze/identity semantics when adding truthful C4 mode/count fields. |
| Ordinary current-app interaction tests | `A/e2e/ui-foundation.spec.ts:1890–1891,1988–1989,2075,2106`; dist counterparts `:1530–1531,1628–1629,1715,1746`; `c3-viewport-visibility.spec.ts:159–195`. | These assume a single click/keyboard activation toggles boolean on/off, then exercise real picks, identity, hover, box and label activation. Update them to select and assert the intended real Budget/All/Off mode under the adopted ruling. Preserve pointer-target, exact selection, history/result and accessibility assertions; add context-retention/overflow coverage. Do not turn Off back into zero labels or hide the app's context to keep old setup code passing. |
| Explicit timed and controls-smoke driver | `A/e2e/ui-foundation/playwright.performance.config.ts:13–17` matches `ui-foundation-performance.benchmark.ts`; candidate config inherits that explicit target. Driver lines 37–66 require phase/evidence/cohort setup and optionally explicit characterization controls-smoke mode. `README.md:17–21` expressly says ordinary E2E/DEC-025 does not discover it. | The live old driver would meet the already identified C4 population/mode incompatibility, even in its explicit untimed controls-smoke mode. It is not a routine merge check. Do not run it and report qualification under changed semantics; prepare the separate second-profile adaptation/freeze. |

The inspected source contains no implemented second-profile `profiles/d72` suite or executable 138/230 label-cap assertion. Those values currently belong to the adopted normative D-72 profile, rather than a mandatory ordinary live-app test. This does not remove that profile obligation.

## Implementation admission and escalation boundary

Ordinary admission requires the C4 implementation and ordinary current-product tests to agree with the actual owner ruling: choose explicit modes, retain contextual annotations in Off, disclose contextual overflow, keep Hide/non-overlap/containment/picking protections, and preserve substantive picking and state assertions. Updating the old boolean-control setup for the authorized new control is test maintenance, not permission to weaken a protected oracle. New policy tests must cover actual context-first allocation and omitted identities; the retained 80-helper test alone cannot verify C4.

No **additional earlier owner profile disposition** is presently required just to implement and merge C4 on the inspected check topology. The already required second-profile checkpoint still must resolve the normative Off-zero and total-cap conflict before timed qualification. The ordinary suite's synthetic historical-validator tests supply no evidence that the new product passed that qualification.

If implementation reveals an actually mandatory **live-product protected** check applying the old population rule, stop only that affected acceptance/merge claim and return its exact command, candidate, assertion and protected source. A changed expectation, skipped test, raised budget field, hidden context or fabricated zero count is not a repair. Bring forward a bounded owner profile/method disposition that explicitly:

1. Keeps the first profile and its historical validation unchanged, identifying the precise live admission test/instrument now incompatible with C4.
2. Adopts a separate successor population contract for that named check: Off retains supported context; Budget's 138/230 are nominal allocations; actual context/ordinary/overflow/unplaced counts are recorded; placement/picking guarantees remain.
3. Authorizes the exact prospective helper/control-adapter and current-product test binding under that successor contract, preserving historical self-tests and source/oracle identities. Any replacement of a mandatory admission check must be named and justified as equivalent for its retained obligations, not silently removed.
4. Preserves numeric timing/resource limits and run/pass rules, declares the narrower observation/comparability limits, and retains the full second-profile/casing freeze before any timed qualification claim. If the owner elects to combine these dispositions with the full freeze, that freeze moves earlier; otherwise the narrow ordinary-admission disposition itself grants no timed-run authority.

That contingency is not an observed blocker or a new question to the user. Compact work continues independently. Fresh independent review and actual candidate-bound ordinary checks are still needed after C4 exists.

## Fixed source identities

Every hash below is SHA-256 of `git show 1c8f04039b5f41812ebd879fc80d810392123072:<path>`, not a concurrently edited worktree file. These are inspection inputs, not execution outputs.

| Source (P/A abbreviations above) | SHA-256 |
|---|---|
| `P/software-workflow.json` | `3a6fd86bd362eed5e1fbcda05dcde961fca8ad46cb14375ce3dd79c3872e09b7` |
| `P/tools/release/run_evidence_sweep.py` | `cdb7bd4914652e6e5497f09f084d755107977dbbf58968dadfebda9f44ddcd3c` |
| `P/tools/ci/e2e_plan.py` | `23ffd49f1f8f845c9b0e716143fe6a91d8064c00f46c7060eae1af6e316167a8` |
| `.github/workflows/piping-desktop-e2e.yml` | `dee6e956e0fc0b42a0bb243dab8629954960695017bf983acfca3439fd31f40d` |
| `A/playwright.config.ts` | `d861a8567ce54c803e2bef914a5b9965906f90e0eda58ab4d907160d10eed31a` |
| `A/playwright.dist.config.ts` | `d53c01817b11718850b2b2b52e0132825931741998d4214a0d0c7e533c2b5756` |
| `A/vite.config.ts` | `9e59361cfb4e89b53d2ce1acfe7a9b000b6d87c38fd7cae283019efc13d04bf9` |
| `A/e2e/ui-foundation/README.md` | `9b4fc8adcbecaf7a94368a21e299c7212f2b9d720dc0e727cd31411b75c89261` |
| `A/e2e/ui-foundation/characterization-commands.ts` | `3bd12296b03a2faee580bf4ccba4ef0db1c1d8ddbd31e24fc32bc78e1e76b421` |
| `A/e2e/ui-foundation/full-cohort-controller.spec.ts` | `3d92c1d27380216bc8dbc2906778b31b0b098bc6a6bb24b954d0b8a07ed7b725` |
| `A/e2e/ui-foundation/performance-targets.spec.ts` | `e645adbc092a8f432e7f69c3b955944f31eff7bae485e6ab5b7a5349aaeebe18` |
| `A/e2e/ui-foundation/playwright.performance.config.ts` | `5b587689281763f4524c376e201dbc93e1d7307160769d95ded7abcaf6a7f190` |
| `A/e2e/ui-foundation/ui-foundation-performance.benchmark.ts` | `3607cd84b64003e30d61fd57f76c932b45f6f3b5147e7e88c2b2737c44bddb0c` |
| `A/e2e/ui-foundation/fixture-manifest.json` | `6e7fba8fdba11853ad7aa558c7e7c29ffb1a82e79c11a1b7633827335f458739` |
| `A/src/features/viewport/viewportSelection.test.ts` | `bff08f70bdf0259e94e56eaac2e8911c76f1e7be77f3adc9f8335eb6b36b1aa0` |
| `A/src/features/viewport/viewportSelection.ts` | `fdf3eaa49b9685b932bce404421086c45a82fa22e5c8abc3f8146fc09cb846d5` |
| `A/src/features/workspace/uiDiagnostics.test.ts` | `719056fca2a43e8199ae077c0e77b5b1926d7a5ac26b37886528b048e2cd74cc` |
| `A/e2e/ui-foundation.spec.ts` | `0bbfd7c51482a5243e75a347b4ed39cfd5a698ef233b5f172c87fa4dc6a95fd3` |
| `A/e2e/ui-foundation-dist.spec.ts` | `d5b67fae02e77cc5f97d5475fb93aa6358f314030546cab819529af662b8cfea` |
| `A/e2e/c3-viewport-visibility.spec.ts` | `251ed3228801e2929e92343978df711e091b029c5ebbe9707c5e221d4549ccd1` |
| `A/e2e/ui-foundation/causal-method-contract.spec.ts` | `572bf26e6185bf229f6d140308e6efede40f45b5b710c76dfbf138505e7b7807` |
| `A/e2e/ui-foundation/playwright.candidate-performance.config.ts` | `7cad30cbc11ab3c536af658b70e30cd7fba94f719e2ca26ea017011ded0690e4` |
| `A/e2e/ui-foundation/benchmark-harness.ts` | `fd4e463b2b2c9b644f415824106b5da501d6d027b95b314a09431f788228946a` |
| `A/package.json` | `c0e3ddd045e0ea7576a4cd01af86c4921bab7314e7b3d927b5eab11822e959fa` |
