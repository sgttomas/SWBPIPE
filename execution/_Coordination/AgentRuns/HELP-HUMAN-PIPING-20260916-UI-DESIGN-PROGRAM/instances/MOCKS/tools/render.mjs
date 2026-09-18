// Opens every frame headlessly with every non-file request blocked, in the frame's theme, at the
// nominal window, at a 1:1 stage viewport and at a narrower width; writes one 1:1 stage screenshot
// per frame (no page screenshots at other sizes are retained) and a report of measured facts:
// overflow, text sizes and families, console errors, the computed scale, the manual theme switch,
// the shell regions' measured boxes, the docked inspector's widths, the HUD's rows, the result
// scale's seven steps in the frame's theme, the self-containment scan, the absolute-path scan and
// the copy lint with the disclosure counts each frame must have. Also opens frames/index.html
// offline to confirm its links resolve and writes index.png.
import { createRequire } from "node:module";
import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";
const here = path.dirname(fileURLToPath(import.meta.url));
const root = path.resolve(here, "..");
// Playwright is read from the piping project's node_modules, located relative to this file.
const require = createRequire(path.resolve(root, "../../../../../../package.json"));
const { chromium } = require("playwright");
const { frames } = await import("./frames.mjs");
const shots = path.join(root, "shots");
fs.mkdirSync(shots, { recursive: true });
const only = process.argv.slice(2);
const list = only.length ? frames.filter((f) => only.includes(f.file)) : frames;
const viewports = [{ tag: "1440", width: 1440, height: 900 }, { tag: "stage", width: 1472, height: 964 }, { tag: "960", width: 960, height: 700 }];

// What each frame must carry (V1.1 §7.4 disclosure homes; the two decision aids differ by design).
const ACCEPT_HOMES = new Set(["s7_both_light", "s7_both_dark", "s9_table_light", "s9_table_dark"]);
const expect = (file) => ({
  acceptance: ACCEPT_HOMES.has(file) ? 1 : 0,
  acceptanceShort: file === "d71_item2_results_caption_light" ? 1 : 0,
  maturity: file === "s1_table_light" ? 1 : file === "d71_item1_status_bar_light" ? 2 : 0,
  displayOnly: file === "s5_table_light" ? 1 : 0,
  draftUntilAccepted: file.startsWith("s8_") ? 1 : 0,
  aidLabel: file.startsWith("d71_") ? 2 : 0, // once in the caption bar, once in the toolbar band
  chips: { s1_table_light: 1, s2_model_light: 1, s2_model_dark: 1, s3_table_light: 1, d71_item1_status_bar_light: 1, s7_both_light: 2, s7_both_dark: 2, s7_table_light: 2, s8_model_light: 2, s9_table_light: 2, s9_table_dark: 2, d71_item2_results_caption_light: 2 }[file] ?? 0,
});

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
      const scale = parseFloat(root.getAttribute("data-scale"));
      const all = [...stage.querySelectorAll("*")];
      const text = all.filter((e) => e.children.length === 0 && e.textContent.trim() && getComputedStyle(e).display !== "none" && !(e instanceof SVGElement));
      const sizes = text.map((e) => parseFloat(getComputedStyle(e).fontSize));
      const sizeHistogram = {};
      for (const s of sizes) sizeHistogram[s] = (sizeHistogram[s] || 0) + 1;
      const families = new Set(text.map((e) => getComputedStyle(e).fontFamily.split(",")[0].trim().replace(/^"|"$/g, "")));
      const box = (sel) => { const e = stage.querySelector(sel); if (!e) return null; const r = e.getBoundingClientRect(); return { w: Math.round(r.width / scale), h: Math.round(r.height / scale) }; };
      // elements whose box leaves the stage (in stage coordinates), ignoring the sprite
      const out = [];
      for (const e of all) {
        if (e.closest("svg")) continue;
        const cs = getComputedStyle(e); if (cs.display === "none" || cs.visibility === "hidden") continue;
        const r = e.getBoundingClientRect(); if (r.width === 0 || r.height === 0) continue;
        const x1 = (r.left - sr.left) / scale, y1 = (r.top - sr.top) / scale, x2 = (r.right - sr.left) / scale, y2 = (r.bottom - sr.top) / scale;
        if (x1 < -1.5 || y1 < -1.5 || x2 > 1441.5 || y2 > 901.5) out.push(`${e.tagName.toLowerCase()}${e.className && typeof e.className === "string" ? "." + e.className.split(" ").slice(0, 2).join(".") : ""} ${Math.round(x1)},${Math.round(y1)}–${Math.round(x2)},${Math.round(y2)}`);
      }
      // text clipped by an ellipsis or overflow in cells, and controls overflowing their boxes
      const clipped = [];
      for (const e of stage.querySelectorAll("td, th, .btn, .chip, .combo, .tip, .pop, .tfoot, .tblhead, .toolbar, .statusbar, .banner, .staleband, .issues .row, .insp .r .v, .legend, .probe, .card, .comment, .toast, .keys, .aid")) {
        if (e.scrollWidth > e.clientWidth + 1 && getComputedStyle(e).overflow !== "visible") clipped.push(`${e.tagName.toLowerCase()}${e.className && typeof e.className === "string" ? "." + e.className.split(" ").slice(0, 2).join(".") : ""} ${e.scrollWidth}>${e.clientWidth} "${e.textContent.trim().slice(0, 40)}"`);
      }
      // regions whose content is taller than the region
      const tall = [];
      for (const e of stage.querySelectorAll(".region, .tables, .drawer, .issues, .insp .body, .agentcol .body, .reviewpage .cols > div, .tblscroll, .modelview .left, .canvas, .bothview .inspector")) {
        if (e.scrollHeight > e.clientHeight + 1) tall.push(`${e.className.split(" ").slice(0, 2).join(".")} ${e.scrollHeight}>${e.clientHeight}`);
      }
      // the copy lint over the stage's visible text, with the registered sentences removed first
      const visible = stage.innerText;
      const canonical = /Results are engineering decision-support information\. Acceptance, professional judgment, and any certification, sealing, or code-compliance determination remain with the responsible engineer and project authority\./g;
      const forbidden = /\b(certif|seal|approv|authenticat|compl(y|iant)|non-authoritative|not authoritative|sign[ -]off|OpenPipe)/i;
      const words = (visible.replace(canonical, "").match(forbidden) || [])[0] || null;
      // the same lint over the whole source (attributes and tooltips included), minus the registered sentence
      const source = root.outerHTML.replace(canonical, "");
      const sourceWord = (source.replace(/Review\/signoff block/g, "").match(forbidden) || [])[0] || null;
      // the V1.1 §7.1 table's forbidden words as labels (case as the table writes them), over the visible text and over the source without its style and script blocks
      const forbidden71 = /\b(Pass|Passed|Fail|OK|Ready|Safe|Acceptable|Verified|Validated|validated|Validate|Apply|Approve|Approved|Confirm|Commit|Lock|Final|Finalise|Release|Released|Aborted|Error|Sign off|Signed|Certified|Compliant|Complies|Code compliant|Exceeds code|Invalid results|Check model|Engineer accepted|recommended by|reviewed and approved|F-PIP-2)\b/;
      const words71 = (visible.replace(canonical, "").match(forbidden71) || [])[0] || null;
      const sourceWord71 = (source.replace(/<style>[\s\S]*?<\/style>|<script>[\s\S]*?<\/script>/g, "").match(forbidden71) || [])[0] || null;
      const count = (re) => (visible.match(re) || []).length;
      // the seven result scale steps in this theme, read from the tokens
      const cs = getComputedStyle(root);
      const scaleSteps = [1, 2, 3, 4, 5, 6, 7].map((i) => cs.getPropertyValue(`--result-scale-${i}`).trim());
      const tokens = Object.fromEntries(["--pressed-fill", "--pressed-ink", "--stale-band", "--stale-ink", "--rail-captionFailed", "--rail-captionStale", "--rail-captionHistorical", "--canvas-hint", "--draft-bar", "--bar-track", "--layout-canvas-min", "--layout-inspector-both"].map((k) => [k, cs.getPropertyValue(k).trim()]));
      const canvas = stage.querySelector(".bothview .canvas");
      const hud = stage.querySelector(".hud");
      return {
        scale, scrollWidth: root.scrollWidth, innerWidth, scrollHeight: root.scrollHeight, innerHeight,
        theme: root.getAttribute("data-theme"), colorScheme: cs.colorScheme, bodyBg: getComputedStyle(document.body).backgroundColor,
        stageBox: { w: Math.round(sr.width), h: Math.round(sr.height) },
        fonts: { families: [...families], sizes: sizeHistogram, textElements: text.length, minFont: Math.min(...sizes), fontsBelow11: sizes.filter((s) => s < 11).length },
        regions: { toolbar: box(".toolbar"), statusbar: box(".statusbar"), rail: box(".rail"), surfaces: box(".surfaces"), agentStrip: box(".agentstrip"), agentColumn: box(".agentcol"), modelCanvas: box(".modelview .canvas"), modelInspector: box(".modelview .inspector"), tableDrawer: box(".drawer"), issuesDrawer: box(".issues"), reviewOutline: box(".reviewpage .outline"), reviewComments: box(".reviewpage .comments") },
        docked: canvas ? { tables: box(".bothview .tables"), canvas: box(".bothview .canvas"), inspector: box(".bothview .inspector"), strip: box(".agentstrip"), camera: canvas.getAttribute("data-camera"), pan: canvas.getAttribute("data-pan"), hudRows: hud ? Math.round(hud.getBoundingClientRect().height / scale / 30) : null } : null,
        scaleSteps, tokens,
        outsideStage: out.slice(0, 12), clipped: clipped.slice(0, 20), tallRegions: tall,
        lint: {
          forbiddenWordVisible: words, forbiddenWordSource: sourceWord, forbiddenLabelVisible: words71, forbiddenLabelSource: sourceWord71,
          acceptance: count(canonical), acceptanceShort: count(/Acceptance and professional judgment remain with the responsible engineer\./g),
          maturity: count(/Technical preview — not a released product\./g), draftUntilAccepted: count(/draft until accepted/g),
          displayOnly: count(/Display only, not accepted as input/g), aidLabel: (document.body.innerText.match(/Decision aid · D-71 item \d · option A/g) || []).length,
          chips: stage.querySelectorAll(".statusbar .chip").length, reviewSignoffRow: count(/Review\/signoff block/g),
        },
      };
    });
    report[f.file][vp.tag] = { ...facts, blockedRequests: net, consoleIssues: errors };
    if (vp.tag === "stage") {
      report[f.file].selfContained = selfContained(path.join(root, "frames", f.file + ".html"));
      report[f.file].expected = expect(f.file);
      const stage = await page.$("#stage");
      await stage.screenshot({ path: path.join(shots, `${f.file}.png`) });
      // the manual theme switch to the other theme, then back to System
      const other = f.theme === "light" ? "dark" : "light";
      await page.click(`#themeseg span[data-theme="${other}"]`);
      await page.waitForTimeout(60);
      report[f.file][vp.tag].manualSwitch = await page.evaluate(() => ({ theme: document.documentElement.getAttribute("data-theme"), bodyBg: getComputedStyle(document.body).backgroundColor, colorScheme: getComputedStyle(document.documentElement).colorScheme, scale1: getComputedStyle(document.documentElement).getPropertyValue("--result-scale-1").trim() }));
      await page.click(`#themeseg span[data-theme="system"]`);
      await page.waitForTimeout(60);
      report[f.file][vp.tag].systemSwitch = await page.evaluate(() => ({ theme: document.documentElement.getAttribute("data-theme"), colorScheme: getComputedStyle(document.documentElement).colorScheme }));
    }
    await ctx.close();
  }
}
// the index page: opened offline, every link must resolve to a file in the directory; index.png
if (!only.length) {
  const ctx = await browser.newContext({ viewport: { width: 1440, height: 900 }, deviceScaleFactor: 1 });
  const page = await ctx.newPage();
  const net = [], errors = [];
  await page.route("**/*", (route) => { const u = route.request().url(); if (u.startsWith("file://")) return route.continue(); net.push(u); return route.abort(); });
  page.on("console", (m) => { if (m.type() === "error" || m.type() === "warning") errors.push(m.type() + ": " + m.text()); });
  page.on("pageerror", (e) => errors.push("pageerror: " + e.message));
  await page.goto("file://" + path.join(root, "frames", "index.html"), { waitUntil: "load" });
  const hrefs = await page.evaluate(() => [...document.querySelectorAll("a[href]")].map((a) => a.getAttribute("href")).filter((h) => !h.startsWith("#")));
  const missing = hrefs.filter((h) => !fs.existsSync(path.join(root, "frames", h)));
  const frameLinks = hrefs.filter((h) => h.endsWith(".html")).length;
  await page.screenshot({ path: path.join(shots, "index.png"), fullPage: true });
  report["index.html"] = { links: hrefs.length, frameLinks, missingLinks: missing, blockedRequests: net, consoleIssues: errors, selfContained: selfContained(path.join(root, "frames", "index.html")) };
  await ctx.close();
}
await browser.close();
function selfContained(file) {
  const html = fs.readFileSync(file, "utf8");
  const external = [];
  // resources the page would need to fetch in order to render; <a href> hyperlinks are navigation, not dependencies
  for (const m of html.matchAll(/<link\b[^>]*>|<script\b[^>]*\bsrc=[^>]*>|@import\b[^;]*;|\burl\(\s*["']?(?!#)[^)]*\)|\bsrc=["'][^"']*["']/g)) external.push(m[0].slice(0, 60));
  const navigation = [...html.matchAll(/<a\b[^>]*href=["'](?!#)([^"']*)["']/g)].map((m) => m[1]);
  return { bytes: html.length, inlineStyleBlocks: (html.match(/<style>/g) || []).length, externalDependencies: external, navigationLinks: navigation, absolutePath: /\/Users\//.test(html) };
}
// the absolute-path scan over everything this instance ships (frames, tools, documents, the report)
const shipped = [];
for (const dir of ["frames", "tools", "."]) for (const name of fs.readdirSync(path.join(root, dir))) { const p = path.join(root, dir, name); if (fs.statSync(p).isFile() && /\.(html|css|mjs|md|json)$/.test(name)) shipped.push(path.join(dir, name)); }
const absolutePaths = shipped.filter((p) => /\/Users\//.test(fs.readFileSync(path.join(root, p), "utf8")));
const prev = fs.existsSync(path.join(shots, "report.json")) && only.length ? JSON.parse(fs.readFileSync(path.join(shots, "report.json"), "utf8")) : {};
const summary = { generated: new Date().toISOString().slice(0, 16), frames: list.length, viewports: viewports.map((v) => `${v.tag} ${v.width}×${v.height}`), screenshots: "1:1 stage only", absolutePathsInShippedFiles: absolutePaths };
fs.writeFileSync(path.join(shots, "report.json"), JSON.stringify({ ...prev, ...report, _summary: summary }, null, 2));
let failures = 0;
for (const [file, byVp] of Object.entries(report)) {
  if (file === "index.html") { const r = byVp; console.log(`index.html links ${r.links} (frames ${r.frameLinks}) missing ${r.missingLinks.length} blocked ${r.blockedRequests.length} console ${r.consoleIssues.length} styleBlocks ${r.selfContained.inlineStyleBlocks} externalDeps ${r.selfContained.externalDependencies.length} absPath ${r.selfContained.absolutePath}`); if (r.missingLinks.length || r.blockedRequests.length || r.selfContained.externalDependencies.length || r.selfContained.absolutePath) failures++; continue; }
  for (const [tag, r] of Object.entries(byVp)) {
    if (tag === "selfContained") { if (r.externalDependencies.length) { console.log(`${file} EXTERNAL DEPENDENCIES: ${r.externalDependencies.join(" | ")}`); failures++; } if (r.absolutePath) { console.log(`${file} ABSOLUTE PATH`); failures++; } continue; }
    if (tag === "expected") continue;
    const flags = [];
    if (r.blockedRequests.length) flags.push(`blocked ${r.blockedRequests.length}`);
    if (r.consoleIssues.length) flags.push(`console ${r.consoleIssues.length}: ${r.consoleIssues[0]}`);
    if (r.scrollWidth > r.innerWidth) flags.push(`hscroll ${r.scrollWidth}>${r.innerWidth}`);
    if (r.scrollHeight > r.innerHeight) flags.push(`vscroll ${r.scrollHeight}>${r.innerHeight}`);
    if (r.fonts.fontsBelow11) flags.push(`fonts<11: ${r.fonts.fontsBelow11}`);
    if (r.outsideStage.length) flags.push(`outside: ${r.outsideStage.join(" | ")}`);
    if (r.clipped.length) flags.push(`clipped: ${r.clipped.join(" | ")}`);
    if (r.tallRegions.length) flags.push(`tall: ${r.tallRegions.join(" | ")}`);
    if (r.lint.forbiddenWordVisible) flags.push(`FORBIDDEN visible "${r.lint.forbiddenWordVisible}"`);
    if (r.lint.forbiddenWordSource) flags.push(`FORBIDDEN in source "${r.lint.forbiddenWordSource}"`);
    if (r.lint.forbiddenLabelVisible) flags.push(`FORBIDDEN §7.1 label visible "${r.lint.forbiddenLabelVisible}"`);
    if (r.lint.forbiddenLabelSource) flags.push(`FORBIDDEN §7.1 label in source "${r.lint.forbiddenLabelSource}"`);
    const ex = byVp.expected;
    for (const k of ["acceptance", "acceptanceShort", "maturity", "displayOnly", "draftUntilAccepted", "aidLabel", "chips"]) if (r.lint[k] !== ex[k]) flags.push(`LINT ${k} ${r.lint[k]} expected ${ex[k]}`);
    if (flags.length) failures++;
    const docked = r.docked ? ` docked tables ${r.docked.tables?.w} canvas ${r.docked.canvas?.w} inspector ${r.docked.inspector?.w ?? "–"} strip ${r.docked.strip?.w ?? "–"} camera ${r.docked.camera ?? "–"} pan ${r.docked.pan ?? "–"} hudRows ${r.docked.hudRows}` : "";
    console.log(`${file} @${tag} scale ${r.scale} ${r.theme}/${r.colorScheme} accept ${r.lint.acceptance}+${r.lint.acceptanceShort} maturity ${r.lint.maturity} draft ${r.lint.draftUntilAccepted} chips ${r.lint.chips} minFont ${r.fonts.minFont}${tag === "stage" ? docked + ` scale1 ${r.scaleSteps[0]} scale7 ${r.scaleSteps[6]}` : ""}${flags.length ? "\n   " + flags.join("\n   ") : ""}`);
  }
}
if (absolutePaths.length) { console.log(`ABSOLUTE PATHS in shipped files: ${absolutePaths.join(", ")}`); failures++; }
console.log(failures ? `${failures} file(s) with findings` : "no findings");
