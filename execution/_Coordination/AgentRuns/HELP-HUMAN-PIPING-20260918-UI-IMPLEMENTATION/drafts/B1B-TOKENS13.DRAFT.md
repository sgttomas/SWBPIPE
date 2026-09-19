> **DRAFT — not sealed, not an instruction.** Re-seal before use: fill the placeholders, choose the model, hash and index it.

# Sealed brief — B1B-TOKENS13: the product adopts `tokens.json` 1.3 (the control boundary and the disabled ink)

Sealed by ROOT (HELP_HUMAN, Agent 0) on @DATE@ before launch. Role: TASK (Type 2), implementer, working alone; Type 2 does not delegate. Model requested: Claude Fable 5.1. Mechanism: Claude Code `Agent` tool, general-purpose type, background. Your return is your final message, which ROOT retains at `{RUN}/lanes/B1B-TOKENS13/returns/B1B-TOKENS13_RETURN.md`.

Path placeholders: resolve `{REPO_ROOT}` with `git rev-parse --show-toplevel` in the worktree ROOT's launch message names; `{WORKING_ROOT}` is `{REPO_ROOT}/projects/chirality-piping`; `{DESKTOP}` is `{WORKING_ROOT}/apps/desktop`; `{RUN}` is `{WORKING_ROOT}/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION`; `{DS}` is `{WORKING_ROOT}/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260916-UI-DESIGN-PROGRAM/instances/DESIGN-SYSTEM`. Write no absolute machine path in anything you produce.

## Why

Slice B1 put the design tokens into the product and kept four workspace variables off the tokens, two of them because the design system had not settled them: `--ui-border` (the design's `border.strong` measured under the 3:1 that a control's boundary needs) and `--ui-disabled-text` (the design's disabled ink measured 1.94:1 on the disabled fill). Design system V1.4 settles both (`{DS}/DESIGN_SYSTEM_V1.md` §1.3, §2.9, §5; `{DS}/tokens.json` version 1.3, SHA-256 `@TOKENS_SHA@`, merged to `origin/main` at `@DS_MERGE@`): `border.control` reads 3:1 or better on every surface and fill a control sits on, and `text.disabled` is lifted to `#8f949a` and `#767b80`. The lifted ink is ROOT's decision on the design child's recommendation, open to the owner (`{RUN}/lanes/DESIGN-SYSTEM-05/HANDOFF.md`). This slice adopts 1.3 and retires those two kept variables. It is small on purpose.

## The work

1. **The token file.** Copy `{DS}/tokens.json` to `{DESKTOP}/src/design/tokens.json` byte for byte. Update `{DESKTOP}/src/design/TOKENS_SOURCE.md` (hash, versions, date, this slice's name) and `RECORDED_SHA256` in `{DESKTOP}/src/design/tokens.test.ts`. Run `npm run tokens` from `{DESKTOP}` and check in the regenerated `src/tokens.css`. The generator needs no change: `cssFrom` in the design system's `tools/gen.mjs` is unchanged at V1.4; confirm that by diff and say so.
2. **The two variables.** In `{DESKTOP}/src/styles.css`, in the `.app-shell` block: `--ui-border: var(--border-control);` and `--ui-disabled-text: var(--text-disabled);`. Remove their two lines from the `.app-shell[data-theme="dark"]` override, which then holds only `--ui-canvas` and `--ui-viewport-selection-geometry`. Rewrite the comment above the block so it is true: two variables are kept, both the canvas lane's; say in one sentence each what `--ui-border` and `--ui-disabled-text` now map to and cite design system V1.4. The earlier comment's figure for `border.strong` was wrong (the independent review of B1 measured 1.75:1 to 2.36:1); the rewritten comment does not need the figure. Leave the sentence about `App.tsx` and `data-theme` as it is: the shell lane corrects it.
3. **Nothing else in the stylesheet.** `--ui-divider` stays on `border.hairline`. The shell lane owns the rest of `styles.css`, including the three framed things that the design system assigns to `border.strong`; do not touch them.
4. **A test that holds the mapping.** Add to `tokens.test.ts` (or a sibling) a test that reads `styles.css` and fails if `--ui-border` or `--ui-disabled-text` is ever again defined as a literal colour in the `.app-shell` blocks, and that the dark override defines exactly the two canvas variables.

## Named semantic changes (say each in your return, with your own measurements)

- **Control boundaries** change from the product's kept `#71838d` (light) and `#70828c` (dark) to `border.control` `#777e85` and `#90969c`. At rest both are 3:1 or better; the kept dark value fell to 2.35:1 on a pressed raised fill, which `border.control` does not. Measure the resolved border against its adjacent surface for the controls the e2e resolved-style check covers, in both themes.
- **Disabled labels** change from the kept `#667680` and `#9baab2` (3.94:1 and 5.71:1 on `disabled.fill`) to `#8f949a` and `#767b80` (2.56:1 and 3.19:1). This lowers the contrast of disabled labels in the product. It is deliberate: the kept ink was stronger than the muted text, so a disabled control read as enabled. WCAG exempts inactive components. Report what the eight disabled toolbar buttons of the start state measure in both themes, and attach one screenshot of the toolbar per theme under `{RUN}/instances/B1B-TOKENS13/shots/`.

## Limits

Write scope: `{DESKTOP}/src/design/**`, `{DESKTOP}/src/tokens.css`, `{DESKTOP}/src/styles.css` (the block and comment named above only), and your own folder `{RUN}/instances/B1B-TOKENS13/**`. Nothing under `src/features/viewport/**`, `e2e/ui-foundation/**`, `src-tauri/**`, `core/**` or the design run. No test weakened, skipped or deleted; if an existing assertion fails because it pins one of the two old values, stop and report it to ROOT with the assertion's text; do not edit it. Never alter a tolerance, an oracle or a limit to obtain a pass. No layout, structure, behaviour or copy change. Run no state-changing git command; ROOT commits.

## Checks (all from `{WORKING_ROOT}` unless stated; report the exact commands and results)

`npm run build:desktop`; `npm run test:desktop`; then both Playwright lanes on the final tree, each through the lock and with one worker: `PLAYWRIGHT_WORKERS=1 sh {RUN}/tools/with_e2e_lock.sh npm run test:e2e` and `PLAYWRIGHT_WORKERS=1 sh {RUN}/tools/with_e2e_lock.sh npm run test:e2e:dist`, from `{DESKTOP}`. Other browser work runs on this host; the lock serializes it, so a wait is normal. Re-hash your changed files after the last check and confirm nothing changed.

## Return

The model you are; every file changed with its SHA-256; the two named semantic changes with measurements; what you ran with results; anything you found and did not act on; what you did not do. Say plainly that you claim no usability, conformance or performance acceptance. End with: Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
