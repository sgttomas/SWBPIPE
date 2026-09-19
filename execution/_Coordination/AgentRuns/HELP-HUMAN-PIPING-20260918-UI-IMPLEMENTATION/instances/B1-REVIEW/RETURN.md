B1-REVIEW return

Retained by ROOT. What follows is the child's final message as the host stored it, with the machine's repository path replaced by `{REPO_ROOT}`; nothing else is changed.

---

I have completed the review. The worktree is unchanged (`git status --short` empty, HEAD still `cbba8e875`), and I ran no state-changing git command, no Playwright, no evidence sweep and no dev server.

## Return — B1-REVIEW

**Model:** Claude Opus 5 (`claude-opus-5[1m]`), TASK (Type 2), read-only, working alone, no delegation.

**Brief verification:** I ran `shasum -a 256` on the sealed brief before starting; it returned `db1f5104f3a5c804fc52cc1fa50140bf023731c08693e66eef43a5102e83b515`, matching the sealed value. The skill body `.agents/skills/software-code-review/SKILL.md` read as `06c27b1be5cfbd9e638570918a8f837d8439c8073c40d3ef708e53874f95570a`, matching.

---

## Verdict: PASS

No actionable finding. The four observations below are non-blocking; three are for the shell lane or for record accuracy.

---

## The six checks

**1. The token file — PASS.** `cmp` reports `apps/desktop/src/design/tokens.json` byte-identical to `.../DESIGN-SYSTEM/tokens.json`; both hash `00c1afe97365973e6c05a2a5d938a25c46022a55c3cbc8d04ff3a9d2d68cf1e9`, which is what `TOKENS_SOURCE.md` and `tokens.test.ts`'s `RECORDED_SHA256` record. No product source, config or e2e file reads `execution/**` (the only mention is prose in `TOKENS_SOURCE.md`, with `{WORKING_ROOT}` placeholders).

**2. The generator — PASS, proven by execution.** I imported both generators and compared `cssFrom.toString()`: identical. Both produce byte-identical CSS from the same `tokens.json`, and `src/tokens.css` equals that output and equals `generatedCss()`. I proved the drift test actually fails, on an isolated scratchpad copy, against four mutations: one hex digit changed in `tokens.json` (2 assertions fail); a hand edit of `tokens.css` (1 fails); the `@import` removed; the `@import` moved after a rule. All four caught. `npx vitest run src/design/tokens.test.ts`: 5 passed.

**3. Theme resolution — PASS.** All six preference × OS combinations resolve to the right block. The cascade is decided by `:root[data-theme="light"]` and `:root:not([data-theme="light"])` at (0,2,0) beating bare `:root` at (0,1,0), with the explicit-dark block last in source order — so explicit light under a dark OS and explicit dark under a light OS both resolve correctly. No mismatch is possible: `.app-shell` is rendered from exactly one place (`src/App.tsx:2545`), and the `useLayoutEffect` at `src/App.tsx:759` runs before paint and restores the prior attribute on unmount (StrictMode-safe: cleanup removes, remount re-sets). `npm run test:desktop`: **75 files, 1190 tests, all pass**, including the pre-existing `uiPreferences.test.ts`. `npx tsc -p tsconfig.json --noEmit`: exit 0. A `vite build` to a scratchpad `outDir` succeeds and the built CSS carries all three theme blocks once each, inlined ahead of the `.app-shell` mapping.

**4. The variable mapping — PASS.** 25 `--ui-*` variables at base and at the candidate; 21 mapped, 4 kept (`--ui-canvas`, `--ui-viewport-selection-geometry`, `--ui-border`, `--ui-disabled-text`); every referenced token is defined in all three blocks; no `--ui-*` is referenced but undefined. I resolved every variable's old and new value in both themes and computed contrast for 47 pairs independently from `tokens.json`. **No text or control-role pair falls below its WCAG 2.2 AA threshold that did not before.** Lowest new text pair is 4.92:1; every graphic pair is ≥ 3.41:1 except the dividers, which were already below 3:1.

The four judgment calls each hold:
- `surface.header` for `--ui-raised` — correct. `surface.raised` is `#ffffff` in light, identical to `surface.panel`, which would erase the header/panel step; all 12 uses are header rows, strips and hover fills.
- `text.secondary` for `--ui-muted` — correct. §2.9 puts `text.muted` at 3.79:1 on the panel in light (verified against the table), below AA for body text.
- `selection.bar` for `--ui-accent` — correct and precisely the design's §1.3 ("a 3 px bar on the left edge of the row"). All 6 uses are selection bars, a selected border or an active outline; none is a fill.
- Status inks for the three `*-text` variables — correct. `status.reviewFill`/`incompleteFill`/`failedFill` are byte-identical to `issue.infoTint`/`warningTint`/`blockingTint`, so each ink sits on exactly the fill it was tuned for (6.47:1 to 7.82:1). Each is used only on its own surface, except `.required-flag` (styles.css:3916) on the panel at 7.23:1.

The kept variables' reasons hold: §2.9 gives `border.strong` 2.00/2.09 and 1.75/2.36 (verified), below the 3:1 the e2e `expectResolvedStyleAndTargets` asserts; §2.9 names `text.disabled` "the one token to lift" and §8 item 6 leaves the target open. I independently reproduced correction 1's numbers exactly: **3.94:1 light, 5.71:1 dark**.

**5. Nothing else changed — PASS.** Nine app files, all inside the B1 write scope. No layout, structure, behaviour, operation, gate, picking or viewport file; `e2e/**` untouched; no existing test modified — the only test changes are two new files. The departure from `main.tsx` to an `@import` in `styles.css` is justified and verified: `e2e/ui-foundation/resource-lifecycle-page.tsx` imports the real `App` plus `src/styles.css` and not `main.tsx`, and `playwright.resource-lifecycle-source.config.ts:19-30` throws on a SHA-256 mismatch of that page file, so it genuinely cannot take a second import. `styles.css` is in the write scope, so this is not a scope breach.

**6. Records — PASS.** No absolute machine path in any file of the candidate range, nor in the later record commit.

---

## Findings (none actionable before merge)

**F1 — minor, not actionable before merge.** `apps/desktop/src/design/tokens.json` carries the design system's `labels` table: the same eight status and evidence rows as `src/features/workspace/statusLabels.ts`. I checked them row by row — token, label, domain and kind agree exactly today, and nothing in the product reads the token table, so no label is drawn from outside `statusLabels.ts`. But nothing binds them, so a later design-system copy could diverge silently. Copying the file byte for byte was an explicit instruction, so this is unavoidable in-slice. Smallest correction: one assertion in `src/design/tokens.test.ts` comparing `tokens.json.labels` to `REGISTERED_STATUS_LABELS`. The child raised the same risk in its return.

**F2 — minor, for the shell lane.** `--ui-divider` → `border.hairline` makes separators markedly fainter: 1.97:1 → 1.43:1 on the panel in light, 2.52:1 → 1.37:1 in dark. Nothing newly fails (both sides are below 3:1), and this is the design's own assignment — §1.2 gives `border.hairline` to "between regions and rows" and §2.9 calls hairlines "deliberately recessive". But the same variable also frames three things §1.2 assigns to `border.strong`: `.entity-grid-virtual-table` (styles.css:3676), `.virtual-target-picker-list` (:3674) and the deformation popover (:3488). One variable cannot carry two tokens, and splitting them is a restyle this slice was told not to do.

**F3 — trivial.** The styles.css comment (near line 3567) says `border.strong` "measures 2.0:1 to 2.4:1 (design system section 2.9)". The four §2.9 readings are 2.00, 2.09, 1.75 and 2.36; the true floor is 1.75:1 on `surface.base` in light. The comment's floor is 0.25 generous, which understates rather than overstates the problem, so the conclusion to keep `--ui-border` is unaffected. Smallest correction: "1.8:1 to 2.4:1".

**F4 — trivial, record accuracy only (records are ROOT's, outside the code review).** `RETURN.md` says "six of the seven uses" of `--ui-accent` and "all 14 uses" of `--ui-raised`; the file has 6 and 12 occurrences. `RETURN.md`'s mapping table, its "22 mapped and 3 kept", and the third row of its semantic-changes table all describe the pre-correction state; `CORRECTION_1_RETURN.md` supersedes all three and the code matches the correction. Raised only so the retained pair is read together.

---

## What I did not check

- **Playwright, the evidence sweep and a dev server** — all forbidden by my brief. The two lanes' green results in the return are for the state **before correction 1**; the child says ROOT directed no re-run. Correction 1 reverted one variable to its base value, and the e2e disabled check (`expectResolvedStyleAndTargets`, `e2e/ui-foundation-workflows.ts:306-308`) asserts target size, not a ratio, so the risk is low — but it is not zero, and the evidence sweep should cover it.
- **Live browser observation.** The "what changed visibly" and "what reads wrong" lists in the return, and the shell lane's hard-coded-colour inventory, are the child's and are unverified by me. I did verify the built bundle inlines the tokens with all three blocks in order.
- **The CSS cascade in a real engine.** My theme-resolution conclusion is specificity reasoning plus the jsdom attribute test, not a rendered check. The test also does not cover explicit-light-under-a-dark-OS (jsdom gives no `matchMedia`, so `systemDark` is always false); that case rests on `:root:not([data-theme="light"])`, which I reasoned through.
- **`npm run build:desktop` as written** — I built to a scratchpad `outDir` instead, to avoid writing `dist/` in the repository, and type-checked with `--noEmit`.
- **The run's own records** (`WORK_GRAPH.json`, `HANDOFF_STATE.md`, the two lane briefs, `tools/with_e2e_lock.sh`) beyond check 6.
- **The 604 lines of `tokens.json` on their merits** — it is byte-identical to the accepted design-system file, which is the check the brief asked for. I computed my contrast figures independently from it and cross-checked four rows against §2.9; they agreed.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081). I claim no usability, conformance or performance acceptance; PDU-045 and PDU-046 remain holds.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
