// Probe: children of the bundled-reference result-page-row (overlap check only).
const { chromium } = require("/home/user/wt/engine/projects/chirality-piping/node_modules/playwright");
(async () => {
  const width = Number(process.argv[2] ?? 1280), height = Number(process.argv[3] ?? 800);
  const browser = await chromium.launch({ executablePath: process.env.PLAYWRIGHT_CHROMIUM_EXECUTABLE_PATH });
  const page = await browser.newPage({ viewport: { width, height } });
  await page.goto("http://127.0.0.1:5174/");
  await page.getByTestId("desktop-preview-shell").waitFor();
  for (const id of ["toggle-tree", "toggle-inspector"]) { const t = page.getByTestId(id); if (await t.getAttribute("aria-expanded") !== "true" && await t.getAttribute("aria-disabled") !== "true") await t.click(); }
  await page.getByTestId("menu-view").click();
  await page.getByTestId("menu-item-view.section.results").click();
  await page.getByTestId("workspace-section-results").getByRole("button", { name: "Inspect bundled reference", exact: true }).click();
  await page.getByTestId("historical-run-context").waitFor();
  const out = await page.evaluate(() => {
    const row = document.querySelector('[data-testid="historical-run-context"] .result-page-row');
    const kids = [...row.querySelectorAll("label, select, span, button")].map((el) => { const b = el.getBoundingClientRect(); return { tag: el.tagName, tid: el.getAttribute("data-testid"), l: +b.left.toFixed(1), r: +b.right.toFixed(1), w: +b.width.toFixed(1), text: el.textContent.slice(0, 60) }; });
    const rb = row.getBoundingClientRect();
    const owners = [...row.querySelectorAll("select, button")].map((el) => { const b = el.getBoundingClientRect(); const top = document.elementFromPoint(b.left + b.width / 2, b.top + b.height / 2); return { tid: el.getAttribute("data-testid"), ownedByTarget: el === top || el.contains(top) }; });
    return { row: { l: rb.left, r: rb.right, w: rb.width, scrollW: row.scrollWidth, clientW: row.clientWidth }, kids, owners };
  });
  console.log(JSON.stringify({ viewport: `${width}x${height}`, ...out }));
  await browser.close();
})().catch((e) => { console.error(e); process.exit(1); });
