B1-TOKENS correction 1 return

Retained by ROOT. What follows is the child's final message as the host stored it, with the machine's repository path replaced by `{REPO_ROOT}`; nothing else is changed.

---

## B1-TOKENS addendum — correction 1

I am Claude Fable 5.1 (`claude-fable-5-1`). `--ui-disabled-text` is back at its base values, with no other change. I ran no git command that changes state.

**The change, in `apps/desktop/src/styles.css` only**
- `--ui-disabled-text` is `#667680` in the `.app-shell` block and `#9baab2` in the `.app-shell[data-theme="dark"]` override.
- `--ui-disabled`, the fill, stays mapped to `disabled.fill`.
- The stylesheet comment now lists four kept variables, each with its dark value in the override: `--ui-canvas`, `--ui-viewport-selection-geometry`, `--ui-border` and `--ui-disabled-text`.
- The reason recorded in the comment is that a control disabled with its reason has to be findable. The comment also notes that §2.9 names `text.disabled` the one token to lift, and that §8 item 6 leaves the contrast target open. I read item 6 before citing it.
- The count is now 21 mapped and 4 kept, of 25.

New SHA-256 of `styles.css`: `2bad6e87646ae6d1bd0b277722ca5d0bbfc7a686ce524fc12d02e1dc1c8d311c`. Every other file hash in my return is unchanged, and `main.tsx` is still untouched.

**What the kept value measures on `disabled.fill`**

I used the same method as before on the running product through the lock: composited text colour against the first opaque ancestor, WCAG 2.x, across the same four states in both themes.

| Theme | Kept value on `disabled.fill` | With `text.disabled` (earlier) | Base commit (earlier) |
|---|---|---|---|
| Light | `#667680` on `#e9ebee` = **3.94:1** | 1.94:1 | 3.74:1 |
| Dark | `#9baab2` on `#2b2e32` = **5.71:1** | 2.28:1 | above 4.5:1 |

- Light is slightly above the base because `disabled.fill` is a little lighter than the old `#e1e6e9`.
- In the start state, eight disabled buttons read at these values in each theme (Hide, Isolate, Show All, Fit Selection and four more).
- One other reading appears inside disabled controls, and the correction does not touch it. That text is drawn in `text.secondary` on the panel and reads 6.26:1 in light and 6.91:1 in dark.
- The built CSS carries both kept values once each, and the fill mapping once.

**Semantic changes.** The third row of my return's table, disabled control text contrast, no longer applies. The other two rows stand: `data-theme` on the root, and `color-scheme` on `:root`.

**Tests of mine touched by the change.** None. Neither `tokens.test.ts` nor `themeAttribute.test.tsx` mentions the disabled variables or `text.disabled`.

**Checks on the corrected candidate**

| Command | Result |
|---|---|
| from `apps/desktop`: `npx vitest run src/design` | exit 0; 2 files, 6 tests passed |
| from `{WORKING_ROOT}`: `npm run test:desktop` | exit 0; 75 files, 1190 tests passed |
| from `{WORKING_ROOT}`: `npm run build:desktop` (not asked for) | exit 0 |

- As you directed, I did not re-run the Playwright lanes. Their results in my return are for the candidate before this correction.
- I started the dev server once more through the lock on port 5183, only to measure. I stopped it and confirmed the lock was released and nothing was listening.

**One thing in `git status` that is not mine.** `execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/B1-TOKENS/RETURN.md` is now untracked. Its heading says ROOT retained my return there. I wrote nothing under `execution/**`.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
