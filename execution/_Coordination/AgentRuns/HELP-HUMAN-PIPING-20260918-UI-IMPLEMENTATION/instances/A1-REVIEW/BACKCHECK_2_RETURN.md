A1-REVIEW backcheck 2 return

Retained by ROOT. What follows is the child's final message as the host stored it, with the machine's repository path replaced by `{REPO_ROOT}`; nothing else is changed.

---

## Return — A1-REVIEW backcheck 2: dist-lane twin at `07ece3f9d3d7fdc0da4ceca1293ed5d634c9f687`

**Model:** Claude Opus 5 (1M context), `claude-opus-5[1m]`. TASK (Type 2), read-only, working alone. I delegated nothing, wrote nothing in the repository, and ran no build, test or state-changing git command — this round was inspection only.

**Reviewed:** `git diff 626fa91ebea74f63edee7a55c4b7502237fc4624..07ece3f9d3d7fdc0da4ceca1293ed5d634c9f687` in full. One file, one hunk, as expected: `projects/chirality-piping/apps/desktop/e2e/ui-foundation-dist.spec.ts`.

---

# Verdict: **PASS** — no finding.

The change is the same change I already judged, applied to the same test in the other lane, and the dist lane's own failure numbers independently confirm the 21 px figure I had derived only from CSS.

---

## (1) Only the window height changed

`projects/chirality-piping/apps/desktop/e2e/ui-foundation-dist.spec.ts:1457` — `height: 920` → `height: 899`, plus the four comment lines at 1453–1456. That is the entire hunk.

Untouched, and I checked each by eye against the pre-image: the frozen endpoints at 1471–1472 (`rect.width * 0.29 / rect.height * 0.58` and `rect.width * 0.08 / rect.height * 0.81`); all four containment assertions at 1474–1475, including the `toBeGreaterThan` / `toBeLessThan` operators and their operands; the `scalePoint` derivation at 1473; every camera, box and pointer assertion downstream. No tolerance, oracle, benchmark limit or `toBeCloseTo` precision appears in the diff. `e2e/ui-foundation/**` is untouched — the commit changes one file.

## (2) The comment matches the source twin exactly

I extracted the five lines from each spec (`ui-foundation.spec.ts:1925–1929`, `ui-foundation-dist.spec.ts:1453–1457`) and diffed them: **byte-identical**, comment and `setViewportSize` call together.

I also diffed the two test bodies to confirm the dist test really is the twin whose source counterpart I judged. Through the geometric part — the comment, the viewport, the canvas/axis/scale bounding boxes, the endpoints and all four containment assertions — they are identical. They diverge only at the wheel-target comparison further down, and that divergence is pre-existing at `origin/main` (see the observation below).

## (3) The geometry reasoning holds, and the dist lane's numbers confirm it

The dist lane runs `*-dist.spec.ts` under `playwright.dist.config.ts` against the built `dist/` served by `vite preview` — the production bundle, built from the same `src/styles.css`. Minification does not change computed values. Its single project `chromium-desktop-dist` defaults to 1280 × 900, but this test overrides to 1440 × 920 → now 1440 × 899, so both twins run at exactly the same size. That is why both lanes failed by the same amount.

More than transferring, the reported failure lets me re-derive the compensation from the other direction. The relevant CSS: `.viewport-axis-triad { position: absolute; left: 12px; bottom: 12px; width: 96px; height: 96px }` (`src/styles.css:1345–1350`), absolutely positioned in the same container whose bottom edge the canvas shares (`.viewport-canvas { position: absolute; inset: 0 }` at 1303–1307, `height: 100%` at 3403). So with `H` the canvas height:

- `end.y = rect.y + 0.81·H`
- `axisRect.y = rect.y + H − 108` (12 px bottom offset + 96 px height)
- the assertion `end.y > axisRect.y` holds iff `0.19·H < 108`

The sweep reported `end.y = 754.971875` against `axisRect.y = 757.40625`, a shortfall of **2.434375 px**. Therefore `0.19·H = 110.434375`, so `H = 581.2336` at window height 920 with the footer gone; the threshold is `H < 568.42`, which is why it failed. Reducing the window by 21 px reduces the canvas by 21 px — every other row in the shell's flex column is fixed-height — giving `H = 560.2336` and slack `108 − 106.4444 = +1.5556 px`.

The point that matters: `560.2336` is also exactly what `H` was **before** the footer was removed, provided the footer was 21 px. In my first review I derived 21 px purely from the stylesheet — the top-level `.app-footer { flex: 0 0 auto; min-height: 21px; padding: 4px 10px; font-size: 10px }` under `* { box-sizing: border-box }`, where the 1 px `border-top` plus 8 px padding plus a 12 px line at 10 px/normal exactly fills the 21 px minimum. The dist lane's measured shortfall now reproduces that same 21 px from live layout. Two independent derivations agreeing is the evidence I wanted and did not have last time.

So 899 restores the characterized canvas exactly, and restores the test's original slack of ~1.56 px rather than manufacturing new slack. That is the signature of a preservation rather than a mask: a masking fix would have picked a height that made the assertion comfortable.

## (4) Nothing else in the dist spec is being masked

I enumerated every height-sensitive construct in the spec:

- **`ui-foundation-dist.spec.ts:1586–1587`**, in the `begin()` helper of the `captured Box lifetime` describe (1551), reuses the **same** frozen pair `0.29/0.58 → 0.08/0.81`. It was not changed and did not fail, correctly: its only geometric requirement is `elementFromPoint(point) === the main canvas`, and `.viewport-axis-triad` and `.viewport-scale-bar` are `pointer-events: none` (`styles.css:1340–1343`), so whether the endpoint lands inside the gizmo rect is irrelevant to it. That block sets no viewport, so it runs at the dist default 1280 × 900; at any plausible `H` the point at `0.81·H` is ~455–470 px down a canvas that is hundreds of pixels tall. The slack is structural, not accidental. The source spec has the same untouched pair at 2066–2067.
- **`ui-foundation-dist.spec.ts:1397–1398`**, in the test at 1319 which does run at 1440 × 920, uses `0.08/0.10 → 0.29/0.28` — the upper-left of the canvas, again only asserted to be over the canvas. The 21 px growth moves those points down 2.1 px and 5.9 px, away from the bottom-anchored overlays and nowhere near the top-right view cube.
- **Loose height assertions** at 210, 360 (`canvas.height >= 200`) and 1198 (`canvas.height > 0`) are unaffected in the direction that changed.
- **No visual baselines.** There is no `toHaveScreenshot` or `toMatchSnapshot` anywhere in the spec; the `page.screenshot({ path: testInfo.outputPath(...) })` calls write artifacts, not comparisons.
- **No test in either spec references `app-footer` or `grid-template-rows`** — I grepped the whole `e2e/` tree.

Every other test still runs at its original size and therefore now sees a 21 px taller canvas. That is correct: the product changed, and tests that assert behaviour rather than frozen geometry should follow it. Only the one test with explicitly frozen, characterized endpoints needed pinning, and it is the one that was pinned.

## Observation — not a finding, out of scope

The two twins' wheel-target assertions differ: the source twin (`ui-foundation.spec.ts:2003–2011`) allows a binary64 roundoff band, the dist twin (`ui-foundation-dist.spec.ts:1533`) uses an exact `toEqual`. I confirmed this divergence exists at `origin/main` (dist line 1529, source lines 2005–2008), so it is pre-existing, untouched by this commit, and unrelated to geometry. I raise it only because twin specs that have drifted apart on tolerance are a maintenance trap, and the dist lane is the one that has now bitten twice.

## What I did not check

- **I ran nothing.** No Playwright, no build, no `vite preview`, no `cargo`/`vitest`/`pytest`. The conclusion that the dist twin now passes is a geometric argument from the stylesheet, the configs and the reported failure numbers — it is not an observed pass. The dist lane still needs to be run.
- **I took the two reported figures on trust** (`757.40625`, `754.971875`); they are the sweep's, not mine. They are mutually consistent with the CSS and with the source lane's independently reported ~2.4 px miss, which is why I relied on them.
- **The derivation assumes** the canvas absorbs window-height change 1:1 and that the gizmo's containing block shares the canvas's bottom edge. Both follow from the CSS I read, and the cross-lane agreement supports them, but I did not measure a live layout.
- **I did not re-review the rest of the candidate.** This covers only `626fa91eb..07ece3f9d`; my earlier findings and judgments stand unchanged, and nothing in this commit touches them.
- **I did not check the `chromium-compact` (1280 × 800) or dist-default (1280 × 900) lanes for footer-geometry effects beyond the constructs listed above**, nor the `r2-smoke`, `linear-authoring`, `gui-workflow-validation` or `workspace-layout` specs, which this commit does not touch.
- **The pre-existing 1.56 px slack** on this assertion is restored, not widened. That thinness is not this change's doing and I did not assess whether it should be widened — doing so would change a characterized endpoint relationship, which the brief forbids.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
