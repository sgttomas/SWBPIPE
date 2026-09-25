// Independent review probe: filter-row line structure across viewport widths, both rails open,
// bundled reference inspected. For each width it records the row width, how many lines the row
// uses, which line each control is on, the input width, horizontal overflow, the DOM/Tab order of
// the three controls, and whether typing a filter (which narrows the match summary) changes the
// row's line count. Usage: node band_probe.cjs <height> <width> [<width> ...]
const { chromium } = require("/home/user/wt/engine/projects/chirality-piping/node_modules/playwright");
(async () => {
  const height = Number(process.argv[2]);
  const widths = process.argv.slice(3).map(Number);
  const browser = await chromium.launch({ executablePath: process.env.PLAYWRIGHT_CHROMIUM_EXECUTABLE_PATH });
  for (const width of widths) {
    const page = await browser.newPage({ viewport: { width, height } });
    await page.goto("http://127.0.0.1:5174/");
    await page.getByTestId("desktop-preview-shell").waitFor();
    for (const id of ["toggle-tree", "toggle-inspector"]) {
      const t = page.getByTestId(id);
      if (await t.getAttribute("aria-expanded") !== "true" && await t.getAttribute("aria-disabled") !== "true") await t.click();
    }
    await page.getByTestId("menu-view").click();
    await page.getByTestId("menu-item-view.section.results").click();
    await page.getByTestId("workspace-section-results").waitFor();
    await page.getByTestId("workspace-section-results").getByRole("button", { name: "Inspect bundled reference", exact: true }).click();
    const ref = page.getByTestId("historical-run-context");
    await ref.waitFor();
    const input = ref.getByTestId("result-filter-input");
    await input.scrollIntoViewIfNeeded();
    const measure = () => page.evaluate(() => {
      const ctx = document.querySelector('[data-testid="historical-run-context"]');
      const row = ctx.querySelector(".result-filter-row");
      const box = (el) => { const b = el.getBoundingClientRect(); return { l: +b.left.toFixed(1), t: +b.top.toFixed(1), w: +b.width.toFixed(1), h: +b.height.toFixed(1) }; };
      const label = row.querySelector("label"), inp = row.querySelector('[data-testid="result-filter-input"]');
      const sum = row.querySelector('[data-testid="result-filter-summary"]'), clr = row.querySelector('[data-testid="clear-result-filter"]');
      const tops = [label, sum, clr].map((el) => Math.round(el.getBoundingClientRect().top));
      const lineTops = [...new Set(tops)].sort((a, b) => a - b);
      const ib = inp.getBoundingClientRect();
      const owner = document.elementFromPoint(ib.left + ib.width / 2, ib.top + ib.height / 2);
      const focusables = [...row.querySelectorAll("input,button")].map((el) => el.getAttribute("data-testid"));
      const nested = ctx.querySelector('[data-testid="results-panel"]');
      return {
        // A Current ResultsPanel is not nested in the reference panel; its row is wider by this delta.
        referenceNestingDeltaW: +(ctx.getBoundingClientRect().width - nested.getBoundingClientRect().width).toFixed(1),
        row: box(row), rowScrollW: row.scrollWidth, rowClientW: row.clientWidth,
        lines: lineTops.length, lineOf: { label: lineTops.indexOf(tops[0]), summary: lineTops.indexOf(tops[1]), clear: lineTops.indexOf(tops[2]) },
        input: box(inp), summary: box(sum), clear: box(clr), summaryText: sum.textContent.trim(),
        inputCenterOwner: owner?.getAttribute("data-testid") ?? owner?.tagName, domOrder: focusables,
        docOverflowX: document.documentElement.scrollWidth - document.documentElement.clientWidth,
      };
    });
    const initial = await measure();
    await input.fill("displacement_magnitude");
    await page.getByTestId("result-filter-summary").first().waitFor();
    await ref.getByTestId("result-filter-summary").filter({ hasText: "15 of 830" }).waitFor();
    const filtered = await measure();
    // Tab order: focus the input, Tab once, record the focused control.
    await input.focus();
    await page.keyboard.press("Tab");
    const afterTab = await page.evaluate(() => document.activeElement?.getAttribute("data-testid"));
    console.log(JSON.stringify({ viewport: `${width}x${height}`, initial, filtered, tabFromInput: afterTab }));
    await page.close();
  }
  await browser.close();
})().catch((e) => { console.error(e); process.exit(1); });
