// Renders the specimen with a headless Chromium (Playwright from the piping project's node_modules, used read-only),
// with every non-file request blocked, in light and dark (system preference emulation and the manual switch),
// at 1440 and 720 wide. Writes per-section PNGs and a JSON report of layout facts.
import { createRequire } from "node:module";
import fs from "node:fs";
const require = createRequire(new URL("../../../../../../../package.json", import.meta.url));
const { chromium } = require("playwright");
const file = "file://" + process.argv[2];
const outDir = "shots";
const browser = await chromium.launch();
const report = {};
const sections = ["s1","s2","s3","s4","s5","s6","s7","s8","s9","s10","s11","s12"];
for (const width of [1440, 720]) {
  for (const scheme of ["light", "dark"]) {
    const ctx = await browser.newContext({ viewport: { width, height: 900 }, colorScheme: scheme, deviceScaleFactor: 1 });
    const page = await ctx.newPage();
    const net = []; const errors = [];
    await page.route("**/*", (route) => { const u = route.request().url(); if (u.startsWith("file://")) return route.continue(); net.push(u); return route.abort(); });
    page.on("console", (m) => { if (m.type() === "error" || m.type() === "warning") errors.push(m.type() + ": " + m.text()); });
    page.on("pageerror", (e) => errors.push("pageerror: " + e.message));
    await page.goto(file, { waitUntil: "load" });
    await page.waitForTimeout(150);
    const facts = await page.evaluate(() => {
      const cs = getComputedStyle(document.documentElement);
      const small = [...document.querySelectorAll("body *")].filter(e => e.children.length === 0 && e.textContent.trim() && getComputedStyle(e).display !== "none").map(e => parseFloat(getComputedStyle(e).fontSize));
      return {
        scrollWidth: document.documentElement.scrollWidth, innerWidth, scrollHeight: document.documentElement.scrollHeight,
        colorScheme: cs.colorScheme, bg: getComputedStyle(document.body).backgroundColor, textPrimary: cs.getPropertyValue("--text-primary").trim(),
        tokRows: document.querySelectorAll("#tokgrid tbody tr").length, ctRows: document.querySelectorAll("#ctable tbody tr").length,
        minFont: Math.min(...small), fontsBelow11: small.filter(s => s < 11).length,
        overflowing: [...document.querySelectorAll(".panel, .card, .probe, .legend, .shell, .tblwrap")].filter(e => e.scrollWidth > e.clientWidth + 1 && !e.classList.contains("tblwrap")).map(e => (e.id || e.className) + " " + e.scrollWidth + ">" + e.clientWidth).slice(0, 10)
      };
    });
    const key = `${width}-${scheme}`;
    report[key] = { ...facts, blockedRequests: net, consoleIssues: errors };
    for (const id of sections) {
      const el = await page.$("#" + id);
      // section = heading plus following siblings until next h2
      const box = await page.evaluate((id) => { const h = document.getElementById(id); let n = h, top = h.getBoundingClientRect().top + scrollY, bottom = top; while (n && (n === h || n.tagName !== "H2")) { const r = n.getBoundingClientRect(); bottom = Math.max(bottom, r.bottom + scrollY); n = n.nextElementSibling; } return { top, bottom }; }, id);
      const h = Math.min(4000, box.bottom - box.top + 8);
      await page.screenshot({ path: `${outDir}/${key}-${id}.png`, clip: { x: 0, y: box.top - 4, width, height: h }, fullPage: true });
    }
    // manual switch: opposite theme via the segmented control
    const other = scheme === "light" ? "dark" : "light";
    await page.click(`.spec-head .seg button[data-theme="${other}"]`);
    await page.waitForTimeout(100);
    const sw = await page.evaluate(() => ({ attr: document.documentElement.getAttribute("data-theme"), bg: getComputedStyle(document.body).backgroundColor, cs: getComputedStyle(document.documentElement).colorScheme, pressed: [...document.querySelectorAll('.spec-head .seg button')].map(b => b.getAttribute('aria-pressed')).join(',') }));
    report[key].manualSwitchTo = { other, ...sw };
    await page.screenshot({ path: `${outDir}/${key}-switched-${other}-top.png`, clip: { x: 0, y: 0, width, height: 900 } });
    await ctx.close();
  }
}
await browser.close();
fs.writeFileSync("shots/report.json", JSON.stringify(report, null, 2));
console.log(JSON.stringify(report, null, 1));
