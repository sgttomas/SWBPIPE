// Opens every frame headlessly with every non-file request blocked, in the frame's theme, at the
// nominal window, at a 1:1 stage viewport and at a narrower width; writes one 1:1 stage screenshot
// per frame (no page screenshots at other sizes are retained) and a report of measured facts:
// overflow, text sizes and families, console errors, the computed scale, the manual theme switch,
// the shell regions' measured boxes, the docked inspector's and the canvas's widths, the HUD's size,
// the toast's and the run log's boxes, the result scale's seven steps in the frame's theme, the edge
// line drawn on each result-coloured element, the pointer controls each frame must show, the run
// standing's facts, the label chips checked against the one table, the agent card classes, the
// self-containment scan, the absolute-path scan and the copy lint. Also opens frames/index.html
// offline to confirm its links resolve and writes index.png.
//   node render.mjs [frame …] [--playwright-from <dir>]
// Playwright is resolved from the piping project's package.json by relative path; when this instance
// sits in a checkout whose node_modules is not installed, pass --playwright-from <dir> (or set
// PLAYWRIGHT_FROM) at run time to name a directory that has it. The location is never stored in a file.
import { createRequire } from "node:module";
import { pathToFileURL } from "node:url";
import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";
const here = path.dirname(fileURLToPath(import.meta.url));
const root = path.resolve(here, "..");
const args = process.argv.slice(2); const fromAt = args.indexOf("--playwright-from");
const from = fromAt >= 0 ? args.splice(fromAt, 2)[1] : process.env.PLAYWRIGHT_FROM;
const require = createRequire(from ? pathToFileURL(path.join(path.resolve(from), "package.json")) : pathToFileURL(path.resolve(root, "../../../../../../package.json")));
const { chromium } = require("playwright");
const { frames } = await import("./frames.mjs");
const TOKENS = JSON.parse(fs.readFileSync(path.resolve(root, "..", "DESIGN-SYSTEM", "tokens.json"), "utf8"));
const shots = path.join(root, "shots");
fs.mkdirSync(shots, { recursive: true });
const only = args;
const list = only.length ? frames.filter((f) => only.includes(f.file)) : frames;
const viewports = [{ tag: "1440", width: 1440, height: 900 }, { tag: "stage", width: 1472, height: 964 }, { tag: "960", width: 960, height: 700 }];

// The strings the rulings removed from the product, built by concatenation so that this file does not
// carry them. The search is case-insensitive and runs over every frame's source, index.html, MOCKS_V3.md,
// RETURN.md, sample_model.md, the stylesheets and the tools. MOCKS_V1.md to MOCKS_V3.md are history.
const RETIRED = [["SWB Pip" + "ing Designer", "the longer product name"], ["Pip" + "ing Designer", "the longer product name"], ["Open" + "Pipe", "the former product name"],
  ["Technical " + "preview", "the maturity sentence"], ["not a released " + "product", "the maturity sentence"],
  ["decision-support " + "information", "the acceptance sentence"], ["remain with the responsible " + "engineer", "the acceptance sentence"], ["professional " + "judgment", "the acceptance sentence or a variant"],
  ["CAE" + "PIPE", "another vendor's product"], ["stays the solve " + "basis", "the stale band's removed clause"],
  ["User rule" + "s checked", "an unhyphenated short label"], ["User rule " + "checked", "an unhyphenated short label"], ["User rule " + "failed", "an unhyphenated short label"]];
const retiredIn = (text) => RETIRED.filter(([s]) => text.toLowerCase().includes(s.toLowerCase())).map(([, what]) => what);

// What each frame must carry (V1.2 §7.4 disclosure homes; §2.3 chip policy; §5 the pointer rule; §5.1 run standing).
const expect = (file) => ({
  displayOnly: file === "s5_table_light" ? 1 : 0,
  draftUntilAccepted: file.startsWith("s8_") ? 1 : 0,
  chips: { s1_both_light: 1, s2_model_light: 1, s2_model_dark: 1, s3_table_light: 1, s7_both_light: 2, s7_both_dark: 2, s7_table_light: 2, s8_model_light: 2, s9_table_light: 3, s9_table_dark: 3 }[file] ?? 0,
  boundaryShort: file === "s8_table_light" ? 1 : 0,
  standing: { s8_table_light: "stale", s7_both_light_historical: "historical" }[file] || null,
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
    const facts = await page.evaluate(({ LABELS, CLASSES }) => {
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
      // a bar's children that run past the bar's own right edge (a clipped last button does not show as scrollWidth)
      for (const bar of stage.querySelectorAll(".reviewpage .hd, .toolbar, .statusbar, .tfoot, .tblhead, .agentcol .hd, .insp .hd, .filters, .issues .hd")) {
        const br = bar.getBoundingClientRect();
        for (const c of bar.children) { const cr = c.getBoundingClientRect(); if (cr.width && cr.right > br.right + 1) clipped.push(`past ${bar.className.split(" ")[0]}: ${c.className || c.tagName} ${Math.round(cr.right - br.right)}px`); }
      }
      // regions whose content is taller than the region
      const tall = [];
      for (const e of stage.querySelectorAll(".region, .tables, .drawer, .issues, .insp .body, .agentcol .body, .reviewpage .cols > div, .tblscroll, .modelview .left, .canvas, .bothview .inspector")) {
        if (e.scrollHeight > e.clientHeight + 1) tall.push(`${e.className.split(" ").slice(0, 2).join(".")} ${e.scrollHeight}>${e.clientHeight}`);
      }
      // the copy lint over the stage's visible text and over the whole source (attributes and tooltips included)
      const visible = stage.innerText;
      const forbidden = new RegExp("\\b(certif|seal|approv|authenticat|compl(y|iant)|non-authoritative|not authoritative|sign[ -]off|" + "Open" + "Pipe)", "i");
      const words = (visible.match(forbidden) || [])[0] || null;
      const source = root.outerHTML;
      const sourceWord = (source.replace(/Review\/signoff block/g, "").match(forbidden) || [])[0] || null;
      // the V1.2 §7.1 table's forbidden words as labels (case as the table writes them), over the visible text and over
      // the source without its style and script blocks. "Commit" is not in this list: §7.1 forbids it under Tables and
      // the canvas while §5.1 names the edit chip's button Commit; ROOT accepted the specification's reading (its
      // RETURN §6 item 5), so the edit chip's tooltip reads Commit.
      const forbidden71 = /\b(Pass|Passed|Fail|OK|Ready|Safe|Acceptable|Verified|Validated|validated|Validate|Commit|Approve|Approved|Confirm|Lock|Final|Finalise|Release|Released|Aborted|Error|Sign off|Signed|Certified|Compliant|Complies|Code compliant|Exceeds code|Invalid results|Check model|Engineer accepted|recommended by|reviewed and approved|F-PIP-2)\b/;
      const words71 = (visible.match(forbidden71) || [])[0] || null;
      const sourceWord71 = (source.replace(/<style>[\s\S]*?<\/style>|<script>[\s\S]*?<\/script>/g, "").match(forbidden71) || [])[0] || null;
      // "Press" is not a word the product uses to tell the engineer what to do (V1.2 §7.6)
      // V1.3 (contradiction 5; 15, variant A): "Commit" is no control's copy, no tooltip writes a key outside
      // parentheses, and no button face carries a key. The hint strip (.keys) is not a tooltip and keeps its keys.
      const KEYS = /[⌘⇧⌥↩⎋⇥⌫↓↑]/;
      const tips = [...stage.querySelectorAll("[title]")].map((e) => e.getAttribute("title")).concat([...stage.querySelectorAll("svg title")].map((e) => e.textContent), [...stage.querySelectorAll(".tip")].map((e) => e.textContent));
      const bareKeyTips = [...new Set(tips.filter((t) => KEYS.test(t.replace(/\([^)]*\)/g, ""))))];
      const face = (e) => { const c = e.cloneNode(true); c.querySelectorAll("title").forEach((t) => t.remove()); return c.textContent; };
      const controls = [...stage.querySelectorAll('[role="button"], .btn, .iconbtn, .chip.act')];
      const keyFaces = [...new Set(controls.filter((e) => !e.closest(".keys") && !e.closest(".search") && KEYS.test(face(e))).map((e) => face(e).trim().slice(0, 40)))];
      const commitCopy = [...new Set(controls.filter((e) => /\bcommit\b/i.test(face(e) + " " + (e.getAttribute("title") || "") + " " + (e.getAttribute("aria-label") || ""))).map((e) => (e.textContent.trim() || e.getAttribute("title")).slice(0, 40)))];
      const pressWord = /\bPress(es|ed|ing)?\b/.test(visible.replace(/pressed\.fill|pressed\.ink/g, ""));
      const count = (re) => (visible.match(re) || []).length;
      // the seven result scale steps in this theme, read from the tokens
      const cs = getComputedStyle(root);
      const scaleSteps = [1, 2, 3, 4, 5, 6, 7].map((i) => cs.getPropertyValue(`--result-scale-${i}`).trim());
      const tokens = Object.fromEntries(["--canvas-edge", "--canvas-edgeAlt", "--layout-hud-width", "--layout-hud-widthWrapped", "--layout-hud-wrapBelow", "--layout-toast-width", "--layout-runlog-width", "--pressed-fill", "--pressed-ink", "--stale-band", "--stale-ink", "--rail-captionFailed", "--rail-captionStale", "--rail-captionHistorical", "--canvas-hint", "--draft-bar", "--bar-track", "--layout-canvas-min", "--layout-inspector-both"].map((k) => [k, cs.getPropertyValue(k).trim()]));
      const canvas = stage.querySelector(".bothview .canvas");
      const hud = stage.querySelector(".hud");
      const rbox = (e) => { if (!e) return null; const r = e.getBoundingClientRect(); return { x: Math.round((r.left - sr.left) / scale), y: Math.round((r.top - sr.top) / scale), w: Math.round(r.width / scale), h: Math.round(r.height / scale) }; };
      // every label chip against the one table: the raw token, the domain, the label, the chip's tokens, the tooltip
      const labelProblems = [];
      const lbl = [...stage.querySelectorAll(".chip.lbl")];
      for (const c of lbl) {
        const raw = c.getAttribute("data-label"), L = LABELS[raw];
        if (!L) { labelProblems.push(`no such label ${raw}`); continue; }
        if (c.textContent !== `${L.domain}·${L.label}`) labelProblems.push(`${raw} reads "${c.textContent}"`);
        if (!c.classList.contains(L.chip)) labelProblems.push(`${raw} is not on ${L.chip}`);
        if (!(c.getAttribute("title") || "").startsWith(raw)) labelProblems.push(`${raw} has no token in its tooltip`);
      }
      for (const c of stage.querySelectorAll(".chip.incomplete, .chip.solved, .chip.failed, .chip.review")) if (!c.classList.contains("lbl")) labelProblems.push(`a status chip outside the table: "${c.textContent}"`);
      const labelWords = Object.values(LABELS).map((l) => l.label);
      // the agent's cards: one of the five class words, and never Note on an agent's card
      const kindProblems = [];
      for (const c of stage.querySelectorAll(".comment")) { const k = c.querySelector(".chip.kind")?.getAttribute("data-kind"), who = c.querySelector(".who")?.textContent || ""; if (who.startsWith("Agent") ? !CLASSES.includes(k) : k !== "Note") kindProblems.push(`${who}: ${k}`); }
      // the pointer controls (V1.2 §5): what every frame shows, and what a frame shows when it has the surface
      const has = (sel) => !!stage.querySelector(sel);
      const useOf = (sel) => [...stage.querySelectorAll(sel + " use")].map((u) => u.getAttribute("href"));
      const pointer = {
        undoRedo: useOf(".toolbar .undoredo").join() === "#i-undo,#i-redo",
        inspectorToggle: has("#insptoggle"), agentToggle: has("#agenttoggle"),
        togglesAfterIssues: (() => { const c = [...(stage.querySelector(".toolbar .centre")?.children || [])].map((e) => e.id || e.textContent.trim().split(" ")[0]); return c.join(); })(),
        aboutOpensDirectly: has(".statusbar .about") && !has(".statusbar .pop"),
        hudFitFirst: hud ? hud.querySelector("span")?.getAttribute("data-tool") === "fit" : null,
        hudTools: hud ? hud.querySelectorAll("span[data-tool]").length : null,
        compass: has(".lenrow") ? useOf(".lenrow .iconbtn").join() : null,
        footerSelectionGroup: [...stage.querySelectorAll(".tfoot .selgroup")].map((g) => g.querySelectorAll(".iconbtn").length),
        footerEditChip: [...stage.querySelectorAll(".tfoot .editchip")].map((g) => g.querySelectorAll(".iconbtn").length),
        addRowLines: stage.querySelectorAll("tr.addrow").length,
        expansionChevrons: stage.querySelectorAll(".xchev").length,
        expansionCloseControls: [...stage.querySelectorAll(".expand .block .cap")].map((c) => c.querySelectorAll(".iconbtn").length),
        probeClose: has(".probe") ? has(".probe .pclose") : null,
        toastClose: has(".toast") ? has(".toast .iconbtn") && has(".toast .act") : null,
        inspectorClose: has(".bothview .insp") ? has(".bothview .insp .hd .iconbtn") : null,
        modelInspectorHasNoClose: has(".modelview .insp") ? !has(".modelview .insp .hd .iconbtn") : null,
        drawerClose: has(".issues") ? has(".issues .hd .iconbtn") : null,
        drawerCollapse: has(".drawer") ? has(".drawer .tabs .iconbtn") : null,
        agentCollapseAndSend: has(".agentcol") ? has(".agentcol .hd .iconbtn") && has(".agentcol .sendrow .iconbtn") : null,
      };
      // run standing (V1.2 §5.1): what a Stale or a Historical run may not drive
      const standing = {
        header: stage.querySelector(".tblhead[data-standing]")?.getAttribute("data-standing") || null,
        band: has(".staleband") ? "stale" : has(".histband") ? "historical" : null,
        bandText: (stage.querySelector(".staleband .body, .histband > span")?.textContent || "").trim(),
        railCaption: stage.querySelector(".rail .cap")?.textContent || null,
        statusChips: stage.querySelectorAll(".statusbar .chip").length,
        evidenceChipInHeader: has(".tblhead .chip.lbl"),
        hatchedCells: stage.querySelectorAll("td.stale").length,
        resultColouredElements: stage.querySelectorAll(".canvas .rc-edge").length,
        legend: has(".legend.note") ? "note card" : has(".legend") ? "scale" : null,
        legendText: (stage.querySelector(".legend.note")?.innerText || "").replace(/\n/g, " · "),
        probe: has(".probe"),
        toolsNeedingRun: [...stage.querySelectorAll(".hud span.off")].map((e) => e.getAttribute("title")),
      };
      // the edge line on each result-coloured element, as computed in the theme shown (R-6)
      const edges = [...new Map([...stage.querySelectorAll(".canvas .rc-edge")].map((e) => [e.getAttribute("data-edge") + getComputedStyle(e).stroke, { choice: e.getAttribute("data-edge"), stroke: getComputedStyle(e).stroke }])).values()];
      const edgePlan = canvas?.getAttribute("data-edges") ? JSON.parse(canvas.getAttribute("data-edges")) : null;
      return {
        scale, scrollWidth: root.scrollWidth, innerWidth, scrollHeight: root.scrollHeight, innerHeight,
        theme: root.getAttribute("data-theme"), colorScheme: cs.colorScheme, bodyBg: getComputedStyle(document.body).backgroundColor,
        stageBox: { w: Math.round(sr.width), h: Math.round(sr.height) },
        fonts: { families: [...families], sizes: sizeHistogram, textElements: text.length, minFont: Math.min(...sizes), fontsBelow11: sizes.filter((s) => s < 11).length },
        regions: { toolbar: box(".toolbar"), statusbar: box(".statusbar"), rail: box(".rail"), surfaces: box(".surfaces"), agentStrip: box(".agentstrip"), agentColumn: box(".agentcol"), modelCanvas: box(".modelview .canvas"), modelInspector: box(".modelview .inspector"), tableDrawer: box(".drawer"), issuesDrawer: box(".issues"), reviewOutline: box(".reviewpage .outline"), reviewComments: box(".reviewpage .comments") },
        docked: canvas ? { tables: box(".bothview .tables"), canvas: box(".bothview .canvas"), inspector: box(".bothview .inspector"), slideOver: rbox(stage.querySelector(".insp.slideover")), strip: box(".agentstrip"), column: box(".agentcol"), camera: canvas.getAttribute("data-camera"), pan: canvas.getAttribute("data-pan"), hudRows: hud ? Math.round(hud.getBoundingClientRect().height / scale / 30) : null } : null,
        canvasBox: rbox(stage.querySelector(".canvas")), hud: rbox(hud), mockLine: rbox(stage.querySelector(".mocklabel")), toast: rbox(stage.querySelector(".toast")), runLog: rbox(stage.querySelector(".runlog")), runButton: rbox(stage.querySelector("#runbtn")),
        scaleSteps, tokens, edges, edgePlan, pointer, standing,
        outsideStage: out.slice(0, 12), clipped: clipped.slice(0, 20), tallRegions: tall,
        lint: {
          forbiddenWordVisible: words, forbiddenWordSource: sourceWord, forbiddenLabelVisible: words71, forbiddenLabelSource: sourceWord71,
          draftUntilAccepted: count(/draft until accepted/g), displayOnly: count(/Display only, not accepted as input/g),
          boundaryShort: count(/no protected standards content; code-specific data is user-supplied/g),
          pressWord, bareKeyTips, keyFaces, commitCopy, labelChips: lbl.map((c) => c.getAttribute("data-label")), labelProblems, kindProblems,
          labelWordsOutsideChips: labelWords.filter((w) => { let n = 0, i = -1; while ((i = visible.indexOf(w, i + 1)) >= 0) n++; return n > lbl.filter((c) => c.textContent.endsWith(w)).length; }),
          chips: stage.querySelectorAll(".statusbar .chip").length, reviewSignoffRow: count(/Review\/signoff block/g),
        },
      };
    }, { LABELS: TOKENS.labels, CLASSES: TOKENS.agentCardClasses });
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
// the retired-strings scan: every shipped file except the two history documents of the earlier passes
const HISTORY = new Set(["MOCKS_V1.md", "MOCKS_V2.md", "MOCKS_V3.md"]);
const retiredStrings = {};
for (const p of shipped) { if (HISTORY.has(path.basename(p)) || p === path.join("shots", "report.json")) continue; const hit = retiredIn(fs.readFileSync(path.join(root, p), "utf8")); if (hit.length) retiredStrings[p] = [...new Set(hit)]; }
const searched = shipped.filter((p) => !HISTORY.has(path.basename(p)));
// frames and shots agree with the frame list: nothing retired is left behind, nothing listed is missing
const onDisk = (dir, ext) => fs.readdirSync(path.join(root, dir)).filter((n) => n.endsWith(ext) && n !== "index" + ext).map((n) => n.slice(0, -ext.length)).sort();
const listed = frames.map((f) => f.file).sort();
const inventory = { frames: onDisk("frames", ".html"), shots: fs.existsSync(shots) ? onDisk("shots", ".png") : [], listed, agree: JSON.stringify(onDisk("frames", ".html")) === JSON.stringify(listed) };
const prev = fs.existsSync(path.join(shots, "report.json")) && only.length ? JSON.parse(fs.readFileSync(path.join(shots, "report.json"), "utf8")) : {};
const summary = { generated: new Date().toISOString().slice(0, 16), frames: list.length, viewports: viewports.map((v) => `${v.tag} ${v.width}×${v.height}`), screenshots: "1:1 stage only", tokensVersion: TOKENS.version, absolutePathsInShippedFiles: absolutePaths, retiredStringsSearchedIn: searched, retiredStringsFound: retiredStrings, inventory };
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
    for (const k of ["displayOnly", "draftUntilAccepted", "boundaryShort", "chips"]) if (r.lint[k] !== ex[k]) flags.push(`LINT ${k} ${r.lint[k]} expected ${ex[k]}`);
    if (r.lint.pressWord) flags.push("LINT the word Press is visible");
    if (r.lint.bareKeyTips.length) flags.push(`LINT tooltip with a key outside parentheses: ${r.lint.bareKeyTips.join(" | ")}`);
    if (r.lint.keyFaces.length) flags.push(`LINT control face carries a key: ${r.lint.keyFaces.join(" | ")}`);
    if (r.lint.commitCopy.length) flags.push(`LINT Commit as control copy: ${r.lint.commitCopy.join(" | ")}`);
    if (r.lint.labelProblems.length) flags.push(`LABELS ${r.lint.labelProblems.join(" | ")}`);
    if (r.lint.kindProblems.length) flags.push(`CARD CLASSES ${r.lint.kindProblems.join(" | ")}`);
    if (r.lint.labelWordsOutsideChips.length) flags.push(`LABEL WORDS outside a chip: ${r.lint.labelWordsOutsideChips.join(", ")}`);
    const pt = r.pointer;
    for (const k of ["undoRedo", "inspectorToggle", "agentToggle", "aboutOpensDirectly"]) if (!pt[k]) flags.push(`POINTER ${k} missing`);
    for (const k of ["hudFitFirst", "probeClose", "toastClose", "inspectorClose", "modelInspectorHasNoClose", "drawerClose", "drawerCollapse", "agentCollapseAndSend"]) if (pt[k] === false) flags.push(`POINTER ${k} false`);
    if (pt.hudTools != null && pt.hudTools !== 10) flags.push(`POINTER the HUD has ${pt.hudTools} tools`);
    if (pt.compass != null && pt.compass !== "#i-reverse,#i-check,#i-close") flags.push(`POINTER compass ${pt.compass}`);
    if (pt.expansionCloseControls.some((n) => n !== 2)) flags.push(`POINTER an expansion caption lacks its chevron or close control`);
    if (r.hud && r.canvasBox) { const want = r.canvasBox.w < 400 ? [154, 64] : [304, 34]; if (r.hud.w !== want[0] || r.hud.h !== want[1]) flags.push(`HUD ${r.hud.w}×${r.hud.h} in a ${r.canvasBox.w} px canvas, expected ${want.join("×")}`); }
    if (r.toast && r.toast.w !== 320) flags.push(`TOAST ${r.toast.w} wide`);
    if (r.runLog && r.runLog.w !== 360) flags.push(`RUN LOG ${r.runLog.w} wide`);
    if (r.docked && r.docked.canvas && r.docked.canvas.w < 220) flags.push(`CANVAS ${r.docked.canvas.w} px is under the 220 px minimum`);
    if (ex.standing) { const s = r.standing; const bad = []; if (s.band !== ex.standing) bad.push(`band ${s.band}`); if (s.statusChips) bad.push(`${s.statusChips} status chips`); if (s.evidenceChipInHeader) bad.push("an evidence chip"); if (s.resultColouredElements) bad.push("result colour"); if (s.probe) bad.push("a probe"); if (s.legend === "scale") bad.push("a legend scale"); if (s.railCaption !== (ex.standing === "stale" ? "Stale" : "Historical")) bad.push(`rail caption ${s.railCaption}`); if (ex.standing === "stale" && !s.hatchedCells) bad.push("no hatched cells"); if (bad.length) flags.push(`STANDING ${ex.standing}: ${bad.join(", ")}`); }
    if (r.edgePlan && r.edgePlan.length) { const theme = r.colorScheme.includes("dark") ? "dark" : "light"; const worst = Math.min(...r.edgePlan.map((e) => e[theme + "Contrast"])); if (worst < 2.7) flags.push(`EDGE worst drawn line ${worst}:1`); }
    if (flags.length) failures++;
    const docked = r.docked ? ` docked tables ${r.docked.tables?.w} canvas ${r.docked.canvas?.w} inspector ${r.docked.inspector?.w ?? "–"} strip ${r.docked.strip?.w ?? "–"} camera ${r.docked.camera ?? "–"} pan ${r.docked.pan ?? "–"} hudRows ${r.docked.hudRows}` : "";
    console.log(`${file} @${tag} scale ${r.scale} ${r.theme}/${r.colorScheme} draft ${r.lint.draftUntilAccepted} chips ${r.lint.chips} labels ${r.lint.labelChips.length} hud ${r.hud ? r.hud.w + "×" + r.hud.h : "–"} minFont ${r.fonts.minFont}${tag === "stage" ? docked + ` scale1 ${r.scaleSteps[0]} scale7 ${r.scaleSteps[6]}` : ""}${flags.length ? "\n   " + flags.join("\n   ") : ""}`);
  }
}
if (absolutePaths.length) { console.log(`ABSOLUTE PATHS in shipped files: ${absolutePaths.join(", ")}`); failures++; }
for (const [p, what] of Object.entries(retiredStrings)) { console.log(`RETIRED STRING in ${p}: ${what.join(", ")}`); failures++; }
console.log(`retired strings searched in ${searched.length} files: ${Object.keys(retiredStrings).length ? "FOUND" : "none found"}`);
if (!only.length && !inventory.agree) { console.log(`INVENTORY frames on disk differ from the frame list`); failures++; }
console.log(failures ? `${failures} file(s) with findings` : "no findings");
