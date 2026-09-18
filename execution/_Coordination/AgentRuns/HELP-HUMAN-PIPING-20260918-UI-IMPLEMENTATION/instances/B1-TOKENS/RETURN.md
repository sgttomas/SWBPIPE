B1-TOKENS return

Retained by ROOT. What follows is the child's final message as the host stored it, with the machine's repository path replaced by `{REPO_ROOT}`; nothing else is changed.

---

## B1-TOKENS return

I am Claude Fable 5.1 (`claude-fable-5-1`), TASK (Type 2), and I worked alone with no delegation. I verified the brief with `shasum -a 256` before starting; the result was `dd0e1da42fa32b20b9bbf24950deeef4e7ef94702354f5fc7e0418f47d27b71f`, which matches. I ran no git command that changes state. At the end, `git status --short` for the worktree lists only the files below.

Worktree root is `{REPO_ROOT}`. `{WORKING_ROOT}` is that path plus `/projects/chirality-piping`, and every path below is relative to `{WORKING_ROOT}`. No file I authored contains an absolute machine path.

### What I read

- **Read whole:** the brief, the handoff, the design system's `tools/gen.mjs` and `tokens.json`, the activation record, the D-72 ruling and its addendum, and the `DEC-100` to `DEC-105` rows.
- **Read in part:** design system V1.3 §1, §2.1, §2.3 to §2.9 and the contrast findings. I searched the specification for theme, token and appearance rules: line 15 leaves appearance to the design system, and line 143 (a theme change never makes a run Current) is unaffected.
- **Not read:** the operations map, `MOCKS_V4.md` and the frames. This slice draws no surface, so I have no frame-against-specification difference to report.

### Files changed

| File | Status | SHA-256 |
|---|---|---|
| `apps/desktop/src/design/tokens.json` | new; byte-identical to the design system's file (`cmp`) | `00c1afe97365973e6c05a2a5d938a25c46022a55c3cbc8d04ff3a9d2d68cf1e9` |
| `apps/desktop/src/design/TOKENS_SOURCE.md` | new; source path with placeholders, hash, versions (V1.3, tokens 1.2), copy-only rule | `5849139f5cc2e1781fa872d1c5e631e12caa39febe0bba8d11e7c8f591e3b373` |
| `apps/desktop/src/design/tokens.test.ts` | new; 5 tests | `c1394cd8a4917451b1d59da2afed31a7a8de506b4efeba927482048ea852b251` |
| `apps/desktop/src/design/themeAttribute.test.tsx` | new; 1 test, not asked for by the brief, inside the write scope | `fba4cfbc157ccb0d5f5334f3a6ff985a4031efd0c26722535294deb98c0bb45a` |
| `apps/desktop/scripts/generate-tokens.mjs` | new; `cssFrom` carried verbatim | `f3cfd0fa69d2fd56679ca0f91bf588968a99cfcd55700c33652321dd80c49fa2` |
| `apps/desktop/src/tokens.css` | new, generated, 402 lines | `7eebb758894df1d2fe38d291c0496485a2c81b4711784441300d13052c349ad7` |
| `apps/desktop/package.json` | modified; one script, `tokens` | `c0e3ddd045e0ea7576a4cd01af86c4921bab7314e7b3d927b5eab11822e959fa` |
| `apps/desktop/src/App.tsx` | modified; +12 lines, one `useLayoutEffect` | `dc8712e9ce80b2f9f2f8613f77800e1f81a0e9eefa61d3f129ecf96d009fcf3f` |
| `apps/desktop/src/styles.css` | modified; one `@import` and the variable block | `425f1e2dfd9139da244df8e21923a76c389ff8db3d677a272385e49138e0e4ee` |

`apps/desktop/src/main.tsx` is in the write scope but is unchanged; the reason is in the next section. The generator's output is byte-identical (`cmp`) to what the design system's own `gen.mjs` produces from the same file.

### How the work was done

**Theme attribute (item 3).**
- `App.tsx` sets `data-theme` on `document.documentElement` to the resolved theme in a layout effect and restores the prior state on unmount. The shell's own `data-theme` and `data-theme-preference` are untouched.
- I chose this because the generator then stays identical to the design system's three `:root` blocks, and the loading screen and anything rendered outside `.app-shell` receive the tokens.
- Before mount, the `prefers-color-scheme` block gives the system theme.
- In a live browser I checked the system preference under a light and a dark system, plus explicit dark under a light system and explicit light under a dark system. In every case the root attribute, the shell attribute, `--surface-base` and the shell background agreed.

**Token loading.**
- `styles.css` opens with `@import "./tokens.css";` instead of an import in `main.tsx`.
- The reason is `e2e/ui-foundation/resource-lifecycle-page.tsx`, which mounts the real App with `src/styles.css` alone. Its Playwright configuration hash-binds that page file, so it cannot take a second import.
- With a `main.tsx` import, the `--ui-*` variables would have resolved to nothing in that harness. I loaded the harness page in a live browser and the tokens resolve in both themes.
- The built CSS carries the token blocks once.
- If you prefer the brief's placement, the fix is a one-line import in `main.tsx` plus removing the `@import`, but the lifecycle harness then needs separate handling.

**Variable mapping (item 4).** Your count was 16; re-verified, the base defines 25 `--ui-*` variables. 22 are mapped and 3 are kept.

| Variable | Token |
|---|---|
| `--ui-bg` | `surface.base` |
| `--ui-surface` | `surface.panel` |
| `--ui-raised` | `surface.header` |
| `--ui-text` | `text.primary` |
| `--ui-muted` | `text.secondary` |
| `--ui-divider` | `border.hairline` |
| `--ui-accent` | `selection.bar` |
| `--ui-accent-text` | `text.inverse` |
| `--ui-selection` | `selection.band` |
| `--ui-selection-text` | `text.primary` |
| `--ui-info-surface`, `--ui-warning-surface`, `--ui-error-surface` | `issue.infoTint`, `issue.warningTint`, `issue.blockingTint` |
| `--ui-info-border`, `--ui-warning-border`, `--ui-error-border` | `issue.info`, `issue.warning`, `issue.blocking` |
| `--ui-info-text`, `--ui-warning-text`, `--ui-error-text` | `status.reviewInk`, `status.incompleteInk`, `status.failedInk` |
| `--ui-focus` | `focus.ring` |
| `--ui-disabled` | `disabled.fill` |
| `--ui-disabled-text` | `text.disabled` |

- **Kept:** `--ui-canvas`, `--ui-viewport-selection-geometry` and `--ui-border`. The reasons are under "Where the design could not be followed".
- **Dark override:** the dark block now holds only those three values; every mapped variable is defined once.
- **Type, spacing, radii:** no `--ui-*` variable carried a type, spacing or radius role, so none of those tokens are applied. They are available as `--font-*`, `--type-*`, `--space-*`, `--radius-*`, `--border-width-*`, `--layout-*`, `--motion-*`, `--easing-*`, `--elevation-*` and `--icon-stroke`.
- **Name collisions:** no existing custom-property name in `src` collides with a token name.

Four mapping choices are my judgement and yours to overrule:
- **`surface.header` for `--ui-raised`.** `surface.raised` equals the panel colour in light, and all 14 uses are header rows, strips and hover fills.
- **`text.secondary` for `--ui-muted`.** `text.muted` measures 3.79:1 on the panel in light and the design reserves it for captions.
- **`selection.bar` for `--ui-accent`.** Six of the seven uses are a selection bar, selected border or active outline; none is a fill. The exception is under "What reads wrong".
- **Status inks for the three `*-text` variables.** Each is the ink drawn for a fill whose value equals that tint in both themes, measuring 6.5:1 to 7.8:1. The design's issue rows use `text.primary` with the hue on a glyph.

### Checks

All results are for the final candidate, run from `{WORKING_ROOT}` unless stated.

| Command | Result |
|---|---|
| `npm run test:desktop` | exit 0; 75 files, 1190 tests passed (6 of the tests are mine) |
| `npm run build:desktop` | exit 0 |
| from `apps/desktop`: `PLAYWRIGHT_WORKERS=1 sh {RUN}/tools/with_e2e_lock.sh npm run test:e2e` | exit 0; 394 tests: 374 passed, 20 skipped, 0 failed (8.9 m) |
| from `apps/desktop`: `PLAYWRIGHT_WORKERS=1 sh {RUN}/tools/with_e2e_lock.sh npm run test:e2e:dist` | exit 0; 53 passed (2.9 m) |
| from `apps/desktop`: `npx vitest run src/design` | 2 files, 6 tests passed |
| `python3 tools/validation/validate_claims_language.py`, from the repository root | exit 0; 321 files scanned. Not asked for by the brief; run because I added text to the product tree |

- **Skipped tests.** All 20 are in `e2e/ui-foundation/full-cohort-controller.spec.ts` and are gated on `D70_WRITER_*` environment inputs (`test.skip(!process.env…)`), 10 per project.
- **Test sensitivity.** A hand edit of `tokens.css`, an added byte in `tokens.json`, and removal of the `setAttribute` line each made the matching test fail. Each file was then restored and re-hashed to its prior value.
- **Earlier runs.**
  - I stopped my first source-lane run myself at test 5 to correct `--ui-border`; no failure had appeared.
  - I ran `e2e/ui-foundation.spec.ts` alone as a precheck: 98 passed.
  - A complete pair on the earlier candidate (tokens imported in `main.tsx`) gave 374 passed and 20 skipped on the source lane, and 53 passed on the dist lane.
- **Servers.** I started the dev server four times on port 5183 through the lock, only for looking. I stopped each one, and each time I confirmed the lock was released and nothing was listening.

**Reply to ROOT's two notes.**
1. No lane failed, so nothing relates to your three files; `git status` shows them at HEAD.
2. I met no problem wait at the lock.

### Semantic changes

No engineering semantics changed. No operation, mutation route, result standing, generation gate, persistence or copy changed, and no control was added, removed, enabled or disabled.

Three presentation behaviours changed; none is a restyle of a component:

| Behaviour | Today | New |
|---|---|---|
| `data-theme` on the document root | only `.app-shell` carries it | the root mirrors the resolved theme while the App is mounted and is restored on unmount |
| `color-scheme` | declared nowhere | declared on `:root`, so native controls, scrollbars and select menus follow the theme |
| Disabled control text contrast | 3.74:1 in light, above 4.5:1 in dark | 1.94:1 in light, 2.28:1 in dark; this is the design's `text.disabled` on `disabled.fill` |

On the disabled-text change, the design system's §2.9 calls `text.disabled` "the one token to lift" if the owner's target requires more.

### Controls left absent or disabled

None. This slice touches no control.

### Where the design could not be followed

1. **`--ui-border` keeps its values (`#71838d` light, `#70828c` dark).**
   - Measured live with the e2e helper's own method on `[data-testid="workspace-review"]`, `border.strong` gives 2.00:1 in light and 2.09:1 in dark. The shipped values give 3.94:1 and 3.71:1.
   - `expectResolvedStyleAndTargets` in `e2e/ui-foundation-workflows.ts` asserts 3:1 or better on that border; I did not run the failing configuration.
   - The design system's §2.9 states 1.8:1 to 2.4:1 for `border.strong`, which it says "frames inputs and the table".
   - Settling this needs either a control-boundary token at 3:1 or better, or a ruling that lets that check move. I did neither.
2. **`--ui-canvas` and `--ui-viewport-selection-geometry` keep their values.**
   - They mirror constants in `features/viewport/viewportResource.ts`.
   - The frozen characterization reads them back: `candidate-camera-preflight.benchmark.ts`, and `freeze-candidate-point-oracle.mjs`, which asserts equality.
   - They belong to the canvas lane, C1. This slice changes no canvas code or colour.
3. **The disabled-text pairing is not in §2.9's pair list.** `text.disabled` on `disabled.fill` sits just under the stated 2.0:1 to 2.8:1 range in light.

### What changed visibly, and what reads wrong

**What changed.**
- Light: surfaces go from green-tinted greys to neutral cool greys, headers are lighter, and dividers are softer (`#aebbc2` to `#d5d8db`).
- Dark: surfaces go from blue-black to neutral charcoal.
- Selection and active marks go from teal to blue in both themes.
- Warning, error and info surfaces take the design's amber, red and blue-grey with their inks.

**Contrast.**
- I compared current values against the base's variable values injected live, across four states (start, node inspector, legacy grid, Issues drawer) in both themes.
- No enabled text newly falls below 4.5:1.
- Ten light captions that measured 4.31:1 on the base now pass.
- Apart from disabled text, every reading below 4.5:1 that remains also fails on the base.

**What reads wrong.** Every item below is a hard-coded colour in `styles.css` that pre-exists the slice, except the first bullet, which is a side effect of mapping `--ui-accent`.
- **Deformation cue.** `.viewport-deformation-status.available`, a results-availability cue, is drawn with `--ui-accent`. It was teal and is now blue, which the design reserves for interaction. The shell lane should re-token it, probably to neutral ink.
- **Title bar.** `.titlebar` (`#263331` with `#f8faf7` and `#c7d0cb` text) is now the only green surface in either theme. Its paragraph colour also leaks into the Units popover, where it reads 1.58:1 on white.
- **Latched toggles.** They use orange: `#d57d16` border, `#fff4df` fill and `#141f1d` ink, across `.command-buttons`, `.viewport-select-target`, `.viewport-endpoint-pick` and `.mode-button`. There is also a global `button:focus-visible { outline: #f08c22 }`. In the design, amber means warning or stale, and a latched toggle takes `pressed.fill` with `pressed.ink`.
- **Legacy green.** `#2d5b50` appears 10 times as an accent.
- **Dark theme, white boxes.**
  - The legacy grid: `.entity-grid-scroll` `#fff`, `th` `#eef3ef`, `tr.active` `#e9f3ee`. Its values read 1.08:1, against 1.04:1 on the base.
  - `.editor-intent-preview` `#fff`.
  - Six support-form label backgrounds in `#fff`; the UX to RZ captions read 2.14:1.
  - `.viewport-shell` `#fbfcfa` behind the canvas.
  - `.viewport-scale-bar`, a white plate at 80% opacity.
- **Dark theme, hard-coded muted greys.** `#5d6b66` reads 2.65:1 and `#65716c` reads 2.73:1.
- **Page ground.** `:root { color: #1d2424; background: #ecefeb }` stays pale in dark.

Inspector inputs and selects are themed correctly in both themes; I checked because I first expected otherwise.

**Inventory for the shell lane.**
- `styles.css` has 440 hard-coded colour occurrences, 149 distinct values, on 417 lines, across 139 selector families. Six of the 440 are the kept values above.
- Most used values:
  - `#ffffff` or `#fff` (56), panel and field backgrounds.
  - `#c8d0cb` (28), `#d9dfdb` (14) and `#d6ded9` (8), borders and dividers.
  - `#b8c3bd` (19).
  - `#263331` (24), text and the title-bar ground.
  - `#31403d` (19) and `#25312f` (17).
  - `#65716c` (15), muted text.
  - `#2d5b50` (10), the legacy green accent.
- Densest families: editor-intent 19, app-menu 14, viewport-select 12, entity-grid 11, status-pill 11, load-editor 10, rule-pack 10.
- Full list: `grep -nE '#[0-9a-fA-F]{3,8}\b|rgba?\(' apps/desktop/src/styles.css`.

### For someone else

- **ROOT or the design-system owner:**
  - the control-boundary contrast question (item 1 above);
  - whether to lift `text.disabled`;
  - the four mapping choices I made.
- **Canvas lane C1:**
  - the two kept canvas variables and their coupling to the frozen oracle;
  - `.viewport-shell` and `.viewport-scale-bar` live in `styles.css`, so decide which lane owns them;
  - `tokens.json` can be imported from TypeScript for numeric colours (`resolveJsonModule` is on), but I built no consumer.
- **Status labels.**
  - The `labels` table in `tokens.json` agrees with `src/features/workspace/statusLabels.ts` on all eight rows.
  - Nothing in the product reads the `tokens.json` table; `statusLabels.ts` remains the only source. I flag this so the two tables are not left to drift.
- **Carried unchanged from the handoff's §7:** the dataviz validator was not run on tokens 1.2, and all renders are Chromium only.

I attribute nothing to the owner beyond the records. PDU-045 and PDU-046 remain holds, and I make no usability, conformance or performance claim.

Screenshots and logs are in this session's scratchpad only and are not retained in the repository.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
