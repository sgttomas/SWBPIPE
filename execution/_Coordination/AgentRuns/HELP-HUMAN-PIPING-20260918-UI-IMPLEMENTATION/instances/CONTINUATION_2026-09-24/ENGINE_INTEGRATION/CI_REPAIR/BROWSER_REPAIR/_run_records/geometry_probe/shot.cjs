// Screenshot the bundled-reference result controls and report hit ownership after scrolling into view.
const { chromium } = require("/home/user/wt/engine/projects/chirality-piping/node_modules/playwright");
(async () => {
  const [w, h, tag] = [Number(process.argv[2]), Number(process.argv[3]), process.argv[4]];
  const browser = await chromium.launch({ executablePath: process.env.PLAYWRIGHT_CHROMIUM_EXECUTABLE_PATH });
  const page = await browser.newPage({ viewport: { width: w, height: h } });
  await page.goto("http://127.0.0.1:5174/");
  await page.getByTestId("desktop-preview-shell").waitFor();
  for (const id of ["toggle-tree", "toggle-inspector"]) { const t = page.getByTestId(id); if (await t.getAttribute("aria-expanded") !== "true" && await t.getAttribute("aria-disabled") !== "true") await t.click(); }
  await page.getByTestId("menu-view").click();
  await page.getByTestId("menu-item-view.section.results").click();
  await page.getByTestId("workspace-section-results").getByRole("button", { name: "Inspect bundled reference", exact: true }).click();
  const ctx = page.getByTestId("historical-run-context");
  await ctx.waitFor();
  const controls = ctx.locator(".result-controls");
  await controls.scrollIntoViewIfNeeded();
  await controls.screenshot({ path: `controls-${tag}-${w}x${h}.png` });
  const owners = await page.evaluate(() => {
    const ctx = document.querySelector('[data-testid="historical-run-context"]');
    return [...ctx.querySelectorAll('.result-filter-row input, .result-filter-row button, .result-page-row select, .result-page-row button, [data-testid="result-filter-summary"], [data-testid="result-page-summary"]')].map((el) => {
      const b = el.getBoundingClientRect(); const top = document.elementFromPoint(b.left + b.width / 2, b.top + b.height / 2);
      return { tid: el.getAttribute("data-testid"), w: +b.width.toFixed(1), h: +b.height.toFixed(1), l: +b.left.toFixed(1), t: +b.top.toFixed(1), owned: el === top || el.contains(top), owner: top?.getAttribute("data-testid") ?? top?.tagName };
    });
  });
  const rows = await page.evaluate(() => [...document.querySelectorAll('[data-testid="historical-run-context"] .result-filter-row, [data-testid="historical-run-context"] .result-page-row')].map((r) => ({ cls: r.className, w: r.clientWidth, scrollW: r.scrollWidth, h: r.getBoundingClientRect().height })));
  console.log(JSON.stringify({ viewport: `${w}x${h}`, tag, rows, owners }));
  await browser.close();
})().catch((e) => { console.error(e); process.exit(1); });
