// Standalone geometry probe of the bundled-reference Results filter row.
// Usage: node measure.cjs <width> <height> [label]
const { chromium } = require("/home/user/wt/engine/projects/chirality-piping/node_modules/playwright");
(async () => {
  const width = Number(process.argv[2] ?? 1280), height = Number(process.argv[3] ?? 800);
  const browser = await chromium.launch({ executablePath: process.env.PLAYWRIGHT_CHROMIUM_EXECUTABLE_PATH });
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
  const m = await page.evaluate(() => {
    const r = (el) => { if (!el) return null; const b = el.getBoundingClientRect(); return { l: +b.left.toFixed(1), r: +b.right.toFixed(1), w: +b.width.toFixed(1), t: +b.top.toFixed(1), h: +b.height.toFixed(1) }; };
    const ctx = document.querySelector('[data-testid="historical-run-context"]');
    const row = ctx.querySelector(".result-filter-row");
    const input = row.querySelector('[data-testid="result-filter-input"]');
    const ib = input.getBoundingClientRect();
    const top = document.elementFromPoint(ib.left + ib.width / 2, ib.top + ib.height / 2);
    return {
      toggles: { tree: document.querySelector('[data-testid="toggle-tree"]')?.getAttribute("aria-expanded"), inspector: document.querySelector('[data-testid="toggle-inspector"]')?.getAttribute("aria-expanded") },
      section: r(document.querySelector('[data-testid="workspace-section-results"]')),
      context: r(ctx),
      nestedResultsPanel: r(ctx.querySelector('[data-testid="results-panel"]')),
      row: r(row), rowScrollW: row.scrollWidth, rowClientW: row.clientWidth,
      rowDisplay: getComputedStyle(row).display, rowCols: getComputedStyle(row).gridTemplateColumns,
      label: r(row.querySelector("label")), labelCols: getComputedStyle(row.querySelector("label")).gridTemplateColumns,
      labelText: r(row.querySelector("label > span")), input: r(input),
      summary: r(row.querySelector('[data-testid="result-filter-summary"]')), clear: r(row.querySelector('[data-testid="clear-result-filter"]')),
      pageRow: r(ctx.querySelector(".result-page-row")),
      inputCenterOwner: top?.getAttribute("data-testid") ?? top?.tagName,
      docOverflowX: document.documentElement.scrollWidth - document.documentElement.clientWidth,
    };
  });
  console.log(JSON.stringify({ viewport: `${width}x${height}`, label: process.argv[4] ?? "", ...m }, null, 1));
  await browser.close();
})().catch((e) => { console.error(e); process.exit(1); });
