# Sealed brief — B1-TOKENS: design tokens into the product

Sealed by ROOT (HELP_HUMAN, Agent 0) on 2026-09-18 before launch. Role: TASK (Type 2), implementer, working alone; Type 2 does not delegate. Model requested: Claude Fable 5.1. Mechanism: Claude Code `Agent` tool, general-purpose type, background.

Path placeholders: resolve `{REPO_ROOT}` with `git rev-parse --show-toplevel` in the worktree you are started in; `{WORKING_ROOT}` is `{REPO_ROOT}/projects/chirality-piping`; `{RUN}` is `{WORKING_ROOT}/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION`; `{DESIGN}` is `{WORKING_ROOT}/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260916-UI-DESIGN-PROGRAM`. Write no absolute machine path in any file you author.

## Authority and design basis

The owner authorized this program to implement on 2026-09-18 (`{RUN}/instances/ROOT/ACTIVATION_2026-09-18.md`). The design basis is merged on `main` and is read, never edited: the handoff `{DESIGN}/instances/ROOT/IMPLEMENTATION_HANDOFF_2026-09-18.md` (read it whole first: §3's ten constraints, §4's gap classes, §5's semantic changes, §6's verification and §7's open items all bind you); design system V1.3 `{DESIGN}/instances/DESIGN-SYSTEM/DESIGN_SYSTEM_V1.md` with `tokens.json`; specification V1.2 `{DESIGN}/instances/UX-SPEC/UX_SPEC_V1.md` with `OPERATIONS_MAP.md`; the eighteen frames under `{DESIGN}/instances/MOCKS/frames/` with `MOCKS_V4.md`. The owner's rulings are `DEC-100` to `DEC-105` in `{WORKING_ROOT}/execution/_Decomposition/SOFTWARE_DECOMP.md` §12 and D-72 (`{WORKING_ROOT}/execution/_Coordination/_DECISIONS/D-72_RULING_2026-09-18.md` and its addendum). Where the frames and the specification differ, the specification governs and you report the difference. A frame is a drawing of intent, not a pixel contract. You never attribute to the owner anything those records do not say.

## Limits that bind this work

- **One mutation route.** Every engineering action is a typed operation through `applyModelOperation` / `applyOperationBatch` into the Rust applier. No component mutates the model any other way. Tables and canvas are projections; neither holds state the model does not.
- **Result integrity.** Preserve the Current and Historical designation, the exact solve-input basis, the generation gates (`SolveRunGenerationGate`, `RuleRevisionGenerationGate`, the `stillCurrent` idiom), `commitModelAfterSolveInvalidation`, `clearComputedModelState`, reviewed application, undo and redo, and persistence compatibility. A Historical record never gains a current-model overlay, a chip or a readiness cue through a presentation change. The Stale standing is not drawn in this tranche (G-11 is outside it): after a model change there are no results to show, as today.
- **Picking repair preserved.** PR #794's shared closest-point computation and its tests stay. Never alter a tolerance, an oracle expectation, a benchmark limit or a frozen characterization value to make something pass. If a test of frozen geometry breaks because layout changed, stop and report it to your parent with the numbers; do not adjust it yourself.
- **Rendering foundation retained**: persistent renderer, instancing and chunking, invalidation scheduler, resource ownership ledger, typed model index, selection and picking. Changing any of these needs separate justification returned to ROOT, not a decision inside a slice.
- **Every control maps to an operation or a classed gap.** A control whose gap is outside this tranche (typed-interface gaps, engine gaps, the host gap) is absent, or disabled with its reason shown. It is never faked, and never wired to placeholder data.
- **Semantic changes are named** in your return as changes, with today's behaviour and the new behaviour. Nothing semantic is delivered as restyling.
- **Separately scoped work stays separate**: pressure runtime, connector mechanics, sparse execution, export implementation, schemas, `core/**`, `src-tauri/**` unless your brief names it. The identity layer (document kinds, schema ids, crate names, `caepipe` identifiers, the four kept identifiers) is Tranche A2 and is not touched.
- **Copy.** The product is SWBPIPE. Status labels come only from `src/features/workspace/statusLabels.ts` with their domains. "Accept", never "Approve", as a control; none of certify, seal, approve, authenticate, comply, compliant or sign-off as a control; no maturity sentence, no acceptance sentence, no other vendor's product named; Canadian spelling. Every action has a pointer control; a key is an accelerator only. Tooltips take the form the design system's §7.6 gives.
- **Accessibility.** WCAG 2.2 AA for every control you touch: name, role, state, focus order, visible focus, contrast from tokens, target size. PDU-045 and PDU-046 remain holds; claim no usability acceptance.
- **Browser tests share the host.** The Playwright configurations use fixed ports and reuse a listening server. Every Playwright run, every dev or preview server you start, goes through `sh {RUN}/tools/with_e2e_lock.sh <command>`, with `PLAYWRIGHT_WORKERS=1`. Run **both** lanes when you touch anything a spec exercises: the source lane (`npm run test:e2e` from `apps/desktop`) and the dist lane (`npm run test:e2e:dist`). Stop servers you start. No build or test runs while ROOT has announced a timed benchmark.
- **Never weaken a test.** A test that pinned display text or structure that the design changes moves to the new text or structure, and you say so; a test of behaviour is never deleted or loosened. `src/App.test.tsx` is the safety net for behaviour.
- **Frozen history** is never edited: `{WORKING_ROOT}/execution/**` outside your own instance folder, `validation/evidence/**`, `plans/**`, `docs/_history/**`, `docs/_ScopeChange/**`.

- Work only in the worktree you are started in, on the branch checked out. Run no git command that changes state. ROOT integrates and commits.

## Write scope

`{WORKING_ROOT}/apps/desktop/src/design/**` (new), `apps/desktop/scripts/generate-tokens.mjs` (new), `apps/desktop/src/tokens.css` (new, generated), `apps/desktop/src/styles.css`, `apps/desktop/src/main.tsx` (the import only), `apps/desktop/package.json` (one script), and the smallest change in `apps/desktop/src/App.tsx` needed for the theme attribute (see 3). Nothing under `features/viewport/**`: canvas colours are the canvas lane's slice C1.

## The work

1. **The product owns its token file.** Copy `{DESIGN}/instances/DESIGN-SYSTEM/tokens.json` byte for byte to `apps/desktop/src/design/tokens.json`. The product build must not read from `execution/**`. Beside it write `TOKENS_SOURCE.md`: the source path, the source file's SHA-256, the design system version, and the rule that the file changes only by copying a later design-system version. (This is ROOT's departure from the plan's wording "import `cssFrom` from the design system's `tools/gen.mjs`": a product build that imports from a governance run folder is fragile. The generator's logic is carried over, not the MOCKS output.)
2. **The generator.** `apps/desktop/scripts/generate-tokens.mjs` carries `cssFrom` from `{DESIGN}/instances/DESIGN-SYSTEM/tools/gen.mjs` (same variable naming, same three theme blocks), reads the product's `tokens.json`, writes `src/tokens.css`. Add the npm script `tokens`. `src/tokens.css` is checked in. A Vitest test asserts that `tokens.json` has the recorded SHA-256 and that `tokens.css` equals the generator's output, so neither drifts by hand.
3. **Theme attribute.** The generator scopes themes on `:root[data-theme]` and on `prefers-color-scheme`; the product today sets `data-theme` on `.app-shell` (`App.tsx` near 2535, `styles.css` near 3566) from a resolved preference. Make the token blocks follow the product's resolved theme in both themes and under the "system" preference, with the smallest change that keeps the existing preference behaviour and its tests. State which way you did it and why.
4. **Map the existing variables.** `styles.css` defines 16 `--ui-*` variables (ROOT's count at the base; re-verify) and its dark overrides. Redefine each in terms of the token it corresponds to in the design system's §2, so the running product takes the design's colours without a rewrite of 3,900 lines. Where no token corresponds, keep the value and list it. Do not restyle components in this slice: layout, sizes and structure stay. Typography tokens, spacing and radii are made available but applied only where a `--ui-*` variable already carried that role.
5. **Look at it.** Start the app (through the lock), view it in light and dark, and report what changed visibly and anything that reads wrong (contrast, a hard-coded colour that now clashes). List the hard-coded colours in `styles.css` that the shell lane must replace later; do not replace them here beyond the variable mapping.

## Checks (from `{WORKING_ROOT}`)

`npm run test:desktop`; `npm run build:desktop`; both Playwright lanes through the lock with `PLAYWRIGHT_WORKERS=1`; your new test. All green, or each failure shown to pre-exist on the base commit.

## Return

Your final message is your return; ROOT or your parent retains it. Include: the model you are; every file you changed, grouped; every check with its exact command and result; every semantic change, named; every control you left absent or disabled and its gap; every place the design could not be followed and why; anything outside your scope that someone else must handle. End with the claim fence line below.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
