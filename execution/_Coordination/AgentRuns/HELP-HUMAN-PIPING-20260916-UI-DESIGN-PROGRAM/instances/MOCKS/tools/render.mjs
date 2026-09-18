// Opens every frame headlessly with every non-file request blocked, in the frame's theme, at the
// nominal window, at a 1:1 stage viewport and at a narrower width; writes screenshots and a report
// of layout facts (overflow, small text, console errors, the computed scale, the manual theme switch).
// Also checks that every HTML file is self-contained (no <link>, no script src, no @import, no
// external url() or src attribute) and opens frames/index.html offline to confirm its links resolve.
import { createRequire } from "node:module";
import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";
const here = path.dirname(fileURLToPath(import.meta.url));
const root = path.resolve(here, "..");
const require = createRequire(path.resolve(root, "../../../../../../package.json"));
const { chromium } = require("playwright");
const { frames } = await import("./frames.mjs");
const shots = path.join(root, "shots");
fs.mkdirSync(shots, { recursive: true });
const only = process.argv.slice(2);
const list = only.length ? frames.filter((f) => only.includes(f.file)) : frames;
const viewports = [{ tag: "1440", width: 1440, height: 900 }, { tag: "stage", width: 1472, height: 964 }, { tag: "960", width: 960, height: 700 }];
const browser = await chromium.launch();
const report = {};
for (const f of list) {
  report[f.file] = {};
  for (const vp of viewports) {
    const ctx = await browser.newContext({ viewport: { width: vp.width, height: vp.height }, colorScheme: f.theme, deviceScaleFactor: 1 });
    const page = await ctx.newPage();
    const net = [], errors = [];
    await page.route("**/*", (route) => { const u = route.request().url(); if (u.startsWith("file://")) return route.continue(); net.push(u); return route.abort(); });
    page.on("console", (m) => { if (m.type() === "error" || m.type() === "warning") errors.push(m.type() + ": " + m.text()); });
    page.on("pageerror", (e) => errors.push("pageerror: " + e.message));
    await page.goto("file://" + path.join(root, "frames", f.file + ".html"), { waitUntil: "load" });
    await page.waitForTimeout(120);
    const facts = await page.evaluate(() => {
      const root = document.documentElement;
      const stage = document.getElementById("stage");
      const sr = stage.getBoundingClientRect();
      const all = [...stage.querySelectorAll("*")];
      const text = all.filter((e) => e.children.length === 0 && e.textContent.trim() && getComputedStyle(e).display !== "none" && !(e instanceof SVGElement));
      const sizes = text.map((e) => parseFloat(getComputedStyle(e).fontSize));
      const scale = parseFloat(root.getAttribute("data-scale"));
      // elements whose box leaves the stage (in stage coordinates), ignoring the sprite
      const out = [];
      for (const e of all) {
        if (e.closest("svg")) continue;
        const cs = getComputedStyle(e); if (cs.display === "none" || cs.visibility === "hidden") continue;
        const r = e.getBoundingClientRect(); if (r.width === 0 || r.height === 0) continue;
        const x1 = (r.left - sr.left) / scale, y1 = (r.top - sr.top) / scale, x2 = (r.right - sr.left) / scale, y2 = (r.bottom - sr.top) / scale;
        if (x1 < -1.5 || y1 < -1.5 || x2 > 1441.5 || y2 > 901.5) out.push(`${e.tagName.toLowerCase()}${e.className && typeof e.className === "string" ? "." + e.className.split(" ").slice(0, 2).join(".") : ""} ${Math.round(x1)},${Math.round(y1)}–${Math.round(x2)},${Math.round(y2)}`);
      }
      // text clipped by an ellipsis or overflow in table cells, and controls overflowing their flex parents
      const clipped = [];
      for (const e of stage.querySelectorAll("td, th, .btn, .chip, .combo, .tip, .pop, .tfoot, .tblhead, .toolbar, .statusbar, .banner, .issues .row, .insp .r .v, .legend, .probe, .card, .comment, .mockbar")) {
        if (e.scrollWidth > e.clientWidth + 1 && getComputedStyle(e).overflow !== "visible") clipped.push(`${e.tagName.toLowerCase()}${e.className && typeof e.className === "string" ? "." + e.className.split(" ").slice(0, 2).join(".") : ""} ${e.scrollWidth}>${e.clientWidth} "${e.textContent.trim().slice(0, 40)}"`);
      }
      // regions that overflow their container vertically (content taller than the region)
      const tall = [];
      for (const e of stage.querySelectorAll(".region, .tables, .drawer, .issues, .insp .body, .agentcol .body, .reviewpage .cols > div, .tblscroll, .modelview .left, .canvas")) {
        if (e.scrollHeight > e.clientHeight + 1) tall.push(`${e.className.split(" ").slice(0, 2).join(".")} ${e.scrollHeight}>${e.clientHeight}`);
      }
      const forbidden = /\b(certif|seal|approv|authenticat|compl(y|iant)|non-authoritative|not authoritative)/i;
      const words = (stage.innerText.replace(/Results are engineering decision-support information\. Acceptance, professional judgment, and any certification, sealing, or code-compliance determination remain with the responsible engineer and project authority\./g, "").match(forbidden) || [])[0] || null;
      return {
        scale, scrollWidth: root.scrollWidth, innerWidth, scrollHeight: root.scrollHeight, innerHeight,
        theme: root.getAttribute("data-theme"), colorScheme: getComputedStyle(root).colorScheme, bodyBg: getComputedStyle(document.body).backgroundColor,
        stageBox: { w: Math.round(sr.width), h: Math.round(sr.height) }, textElements: text.length, minFont: Math.min(...sizes), fontsBelow11: sizes.filter((s) => s < 11).length,
        outsideStage: out.slice(0, 12), clipped: clipped.slice(0, 20), tallRegions: tall, forbiddenWord: words,
        acceptanceCount: (stage.innerText.match(/Results are engineering decision-support information/g) || []).length,
        maturityCount: (stage.innerText.match(/Technical preview — not a released product\./g) || []).length,
        draftUntilAccepted: (stage.innerText.match(/draft until accepted/g) || []).length,
        oldName: /OpenPipe/i.test(document.documentElement.outerHTML),
      };
    });
    report[f.file][vp.tag] = { ...facts, blockedRequests: net, consoleIssues: errors };
    if (vp.tag === "stage") report[f.file].selfContained = selfContained(path.join(root, "frames", f.file + ".html"));
    if (vp.tag === "stage") {
      const stage = await page.$("#stage");
      await stage.screenshot({ path: path.join(shots, `${f.file}.png`) });
      // the manual theme switch to the other theme, then back
      const other = f.theme === "light" ? "dark" : "light";
      await page.click(`#themeseg span[data-theme="${other}"]`);
      await page.waitForTimeout(60);
      report[f.file][vp.tag].manualSwitch = await page.evaluate(() => ({ theme: document.documentElement.getAttribute("data-theme"), bodyBg: getComputedStyle(document.body).backgroundColor, colorScheme: getComputedStyle(document.documentElement).colorScheme }));
      await page.click(`#themeseg span[data-theme="system"]`);
      await page.waitForTimeout(60);
      report[f.file][vp.tag].systemSwitch = await page.evaluate(() => ({ theme: document.documentElement.getAttribute("data-theme"), colorScheme: getComputedStyle(document.documentElement).colorScheme }));
    } else {
      await page.screenshot({ path: path.join(shots, `${f.file}-${vp.tag}.png`) });
    }
    await ctx.close();
  }
}
// the index page: opened offline, every link must resolve to a file in the directory
{
  const ctx = await browser.newContext({ viewport: { width: 1440, height: 900 }, deviceScaleFactor: 1 });
  const page = await ctx.newPage();
  const net = [], errors = [];
  await page.route("**/*", (route) => { const u = route.request().url(); if (u.startsWith("file://")) return route.continue(); net.push(u); return route.abort(); });
  page.on("console", (m) => { if (m.type() === "error" || m.type() === "warning") errors.push(m.type() + ": " + m.text()); });
  page.on("pageerror", (e) => errors.push("pageerror: " + e.message));
  await page.goto("file://" + path.join(root, "frames", "index.html"), { waitUntil: "load" });
  const hrefs = await page.evaluate(() => [...document.querySelectorAll("a[href]")].map((a) => a.getAttribute("href")).filter((h) => !h.startsWith("#")));
  const missing = hrefs.filter((h) => !fs.existsSync(path.join(root, "frames", h)));
  report["index.html"] = { links: hrefs.length, missingLinks: missing, blockedRequests: net, consoleIssues: errors, selfContained: selfContained(path.join(root, "frames", "index.html")) };
  await ctx.close();
}
await browser.close();
function selfContained(file) {
  const html = fs.readFileSync(file, "utf8");
  const external = [];
  // resources the page would need to fetch in order to render; <a href> hyperlinks are navigation, not dependencies
  for (const m of html.matchAll(/<link\b[^>]*>|<script\b[^>]*\bsrc=[^>]*>|@import\b[^;]*;|\burl\(\s*["']?(?!#)[^)]*\)|\bsrc=["'][^"']*["']/g)) external.push(m[0].slice(0, 60));
  const navigation = [...html.matchAll(/<a\b[^>]*href=["'](?!#)([^"']*)["']/g)].map((m) => m[1]);
  return { bytes: html.length, inlineStyleBlocks: (html.match(/<style>/g) || []).length, externalDependencies: external, navigationLinks: navigation };
}
const prev = fs.existsSync(path.join(shots, "report.json")) && only.length ? JSON.parse(fs.readFileSync(path.join(shots, "report.json"), "utf8")) : {};
fs.writeFileSync(path.join(shots, "report.json"), JSON.stringify({ ...prev, ...report }, null, 2));
for (const [file, byVp] of Object.entries(report)) {
  if (file === "index.html") { const r = byVp; console.log(`index.html links ${r.links} missing ${r.missingLinks.length} blocked ${r.blockedRequests.length} console ${r.consoleIssues.length} styleBlocks ${r.selfContained.inlineStyleBlocks} externalDeps ${r.selfContained.externalDependencies.length}`); continue; }
  for (const [tag, r] of Object.entries(byVp)) {
    if (tag === "selfContained") { if (r.externalDependencies.length) console.log(`${file} EXTERNAL DEPENDENCIES: ${r.externalDependencies.join(" | ")}`); continue; }
    const flags = [];
    if (r.blockedRequests.length) flags.push(`blocked ${r.blockedRequests.length}`);
    if (r.consoleIssues.length) flags.push(`console ${r.consoleIssues.length}`);
    if (r.scrollWidth > r.innerWidth) flags.push(`hscroll ${r.scrollWidth}>${r.innerWidth}`);
    if (r.scrollHeight > r.innerHeight) flags.push(`vscroll ${r.scrollHeight}>${r.innerHeight}`);
    if (r.fontsBelow11) flags.push(`fonts<11: ${r.fontsBelow11}`);
    if (r.outsideStage.length) flags.push(`outside: ${r.outsideStage.join(" | ")}`);
    if (r.clipped.length) flags.push(`clipped: ${r.clipped.join(" | ")}`);
    if (r.tallRegions.length) flags.push(`tall: ${r.tallRegions.join(" | ")}`);
    if (r.forbiddenWord) flags.push(`FORBIDDEN "${r.forbiddenWord}"`);
    if (r.oldName) flags.push("OLD NAME");
    console.log(`${file} @${tag} scale ${r.scale} theme ${r.theme}/${r.colorScheme} accept ${r.acceptanceCount} maturity ${r.maturityCount} draft ${r.draftUntilAccepted} minFont ${r.minFont}${flags.length ? "\n   " + flags.join("\n   ") : ""}`);
  }
}
