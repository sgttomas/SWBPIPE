// Renders the specimen with a headless Chromium (Playwright from the piping project's node_modules, used read-only),
// with every non-file request blocked, in light and dark (system preference emulation and the manual switch),
// at 1440 and 720 wide. Writes per-section PNGs and a JSON report of layout facts.
//   node render.mjs <path to specimen.html> [out dir, default ./shots] [--playwright-from <dir>]
// Playwright is resolved from the piping project's package.json by relative path; when this instance sits in a
// checkout whose node_modules is not installed, pass --playwright-from <dir> (or set PLAYWRIGHT_FROM) at run time
// to name a directory that has it. The location is never stored in this file.
// V1.1 adds checks for the revision's elements: the stale band, the disabled case selector, the latched toggle,
// the draft row and draft ghost, the geometry strips, the run log, the banner, the outline's registered row,
// the data bar's track, and that the page's pair list is the embedded one.
// V1.2 adds: the label chips redrawn from the label table and equal to it, the label table's eight rows, the wrapped
// HUD's size, the toast's width, the Review header's icons and plain Export, the three run standings, the five
// agent card classes, the edge steps, and that the rendered text carries none of the retired strings.
// V1.3 adds: the menus and their widths, the run text button, the wrapped 737 px header, the stress components
// block, the drawer's one-line header, the units buttons, the edit chip's and the paste band's button faces.
import { pathToFileURL } from "node:url";
import { createRequire } from "node:module";
import fs from "node:fs";
import path from "node:path";
const args = process.argv.slice(2); const fromAt = args.indexOf("--playwright-from");
const from = fromAt >= 0 ? args.splice(fromAt, 2)[1] : process.env.PLAYWRIGHT_FROM;
const require = createRequire(from ? pathToFileURL(path.join(path.resolve(from), "package.json")) : new URL("../../../../../../../package.json", import.meta.url));
const { chromium } = require("playwright");
const file = pathToFileURL(path.resolve(args[0])).href;
const outDir = args[1] || "shots";
fs.mkdirSync(outDir, { recursive: true });
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
      const tok = JSON.parse(document.getElementById("tokens").textContent);
      const pairs = JSON.parse(document.getElementById("pairs").textContent);
      const v = (k) => cs.getPropertyValue("--" + k.replace(/\./g, "-")).trim();
      const theme = cs.colorScheme.includes("dark") ? "dark" : "light";
      const draftGhost = document.querySelector('.figure path[stroke="var(--canvas-draft)"][stroke-dasharray="6 4"]');
      return {
        scrollWidth: document.documentElement.scrollWidth, innerWidth, scrollHeight: document.documentElement.scrollHeight,
        colorScheme: cs.colorScheme, bg: getComputedStyle(document.body).backgroundColor, textPrimary: v("text.primary"),
        tokRows: document.querySelectorAll("#tokgrid tbody tr").length, tokCount: Object.keys(tok.color).length, tokVersion: tok.version,
        ctRows: document.querySelectorAll("#ctable tbody tr").length, pairsEmbedded: pairs.length,
        minFont: Math.min(...small), fontsBelow11: small.filter(s => s < 11).length,
        overflowing: [...document.querySelectorAll(".panel, .card, .probe, .legend, .shell, .tblwrap")].filter(e => e.scrollWidth > e.clientWidth + 1 && !e.classList.contains("tblwrap")).map(e => (e.id || e.className) + " " + e.scrollWidth + ">" + e.clientWidth).slice(0, 10),
        v11: {
          heading: document.querySelector("h1").textContent,
          staleband: !!document.querySelector(".staleband"), stalebandBg: document.querySelector(".staleband") && getComputedStyle(document.querySelector(".staleband")).backgroundColor,
          caseAllDisabled: !!document.querySelector(".combo.disabled"), caseAllText: document.querySelector(".combo.disabled")?.textContent.trim(),
          latched: document.querySelectorAll(".btn.latched").length, latchedBg: document.querySelector(".btn.latched") && getComputedStyle(document.querySelector(".btn.latched")).backgroundColor,
          draftRow: !!document.querySelector("table.ds tr.draft"), draftBar: document.querySelector("table.ds tr.draft td.gut") && getComputedStyle(document.querySelector("table.ds tr.draft td.gut"), "::before").borderLeftStyle,
          draftGhost: !!draftGhost, draftGhostWidth: draftGhost && draftGhost.getAttribute("stroke-width"),
          hintStrip: !!document.querySelector('.figure rect[fill="var(--canvas-hint)"]'),
          geoStrips: document.querySelectorAll(".geo").length, runlog: !!document.querySelector(".runlog"), banner: !!document.querySelector(".banner"),
          railCaptionStale: document.querySelector(".rail .cap.stale") && getComputedStyle(document.querySelector(".rail .cap.stale")).color,
          signoffRow: [...document.querySelectorAll(".outline div")].some(d => d.textContent.includes("Review/signoff block")),
          reportPreview: [...document.querySelectorAll("button")].some(b => b.textContent.trim() === "Report preview"),
          barTrack: document.querySelector(".bar") && getComputedStyle(document.querySelector(".bar")).backgroundColor,
          combinationEditor: [...document.querySelectorAll(".expand")].some(e => e.textContent.includes("Combination editor")),
          v12: (() => {
            const LAB = tok.labels; const q = (s) => document.querySelector(s); const qa = (s) => [...document.querySelectorAll(s)];
            const chips = qa(".chip.lbl[data-label]"); const text = document.body.innerText.toLowerCase();
            const retired = [["pip", "ing designer"], ["open", "pipestress"], ["cae", "pipe"], ["technical ", "preview"], ["not a released ", "product"], ["decision-support ", "information"], ["remain with the responsible ", "engineer"]].map(p => p.join(""));
            const wrap = q(".hud.wrap")?.getBoundingClientRect(); const narrowHud = q(".narrow .hud")?.getBoundingClientRect(); const narrow = q(".narrow")?.getBoundingClientRect();
            return {
              labelChips: chips.length,
              labelChipsEqualTable: chips.every(c => { const r = LAB[c.dataset.label]; return r && c.textContent === r.domain + "·" + r.label && c.title === c.dataset.label && c.classList.contains(r.chip); }),
              labelTableRows: qa("#labeltable tbody tr").length,
              wrappedHud: wrap && { w: Math.round(wrap.width), h: Math.round(wrap.height), tools: qa(".hud.wrap")[0].children.length, first: qa(".hud.wrap")[0].children[0].title },
              narrowHudInside: narrow && narrowHud && narrowHud.right <= narrow.right && narrowHud.left >= narrow.left,
              toastWidth: q(".toast") && Math.round(q(".toast").getBoundingClientRect().width), toastExpected: tok.layout["toast.width"],
              reviewHeaderIcons: ["Snapshot…", "Report preview", "Export…"].map(n => { const b = qa("#s10 ~ .panel button, button").find(x => x.textContent.trim() === n); return !!(b && b.querySelector("svg")); }),
              exportIsPlain: q("#exportbtn") && !q("#exportbtn").classList.contains("primary"),
              accentButtonsOnReviewHeader: qa("#exportbtn").length && [...q("#exportbtn").parentElement.querySelectorAll(".btn.primary")].length,
              histband: q(".histband")?.textContent.replace(/\s+/g, " ").trim(), runIdentityButtons: qa("button").filter(b => b.textContent.trim() === "Run identity").length,
              legendNoteCards: qa(".legend").filter(l => l.textContent.includes("No result colour on the current model")).length,
              agentClasses: [...new Set(qa("[data-agent-class]").map(e => e.dataset.agentClass))].sort(), agentClassChips: qa("#agentclasses .chip").map(c => c.textContent),
              edgeSteps: qa("#edgesteps div").map(d => d.title), edgeNote: q("#edgenote")?.textContent.length > 0,
              addRow: !!q("tr.addrow"), selGroupButtons: qa(".selgroup button").length, editChip: !!q(".editchip"), sendControl: !!q(".sendrow button"), issueRowTruncates: (() => { const m = q(".issuerow.sel .msg"); return m ? m.scrollWidth > m.clientWidth : null; })(),
              v13: { menus: qa(".menu").map(m => m.getAttribute("aria-label") + " " + Math.round(m.getBoundingClientRect().width)), runTextButtons: qa(".runtext").length, wrappedHeaderLines: (() => { const h = qa(".tblwrap").find(w => w.style.width === "737px")?.querySelector(".tblhead"); return h ? Math.round(h.getBoundingClientRect().height) : null; })(), stressComponents: qa(".expand").some(e => e.textContent.includes("Stress components")), drawerHeadOneLine: q(".drawerhead") && Math.round(q(".drawerhead").getBoundingClientRect().height), unitsButtons: qa(".unitsbtn").map(b => b.textContent.trim() + " " + Math.round(b.getBoundingClientRect().width)), editChipButtons: qa(".editchip button").map(b => b.textContent.trim()), pasteButtons: qa(".pasteband button").map(b => b.textContent.trim()), commitAsCopy: qa("button").filter(b => /commit/i.test(b.textContent)).length, reviewWhenChips: qa("#s10 ~ .panel .demo-row .chip.lbl").length },
              retiredFound: retired.filter(r => text.includes(r)),
            };
          })(),
          expected: { staleBand: tok.color["stale.band"][theme], pressedFill: tok.color["pressed.fill"][theme], captionStale: tok.color["rail.captionStale"][theme], barTrack: tok.color["bar.track"][theme] }
        }
      };
    });
    const key = `${width}-${scheme}`;
    report[key] = { ...facts, blockedRequests: net, consoleIssues: errors };
    for (const id of sections) {
      const box = await page.evaluate((id) => { const h = document.getElementById(id); let n = h, top = h.getBoundingClientRect().top + scrollY, bottom = top; while (n && (n === h || n.tagName !== "H2")) { const r = n.getBoundingClientRect(); bottom = Math.max(bottom, r.bottom + scrollY); n = n.nextElementSibling; } return { top, bottom }; }, id);
      const h = Math.min(4000, box.bottom - box.top + 8);
      await page.screenshot({ path: path.join(outDir, `${key}-${id}.png`), clip: { x: 0, y: box.top - 4, width, height: h }, fullPage: true });
    }
    // manual switch: opposite theme via the segmented control
    const other = scheme === "light" ? "dark" : "light";
    await page.click(`.spec-head .seg button[data-theme="${other}"]`);
    await page.waitForTimeout(100);
    const sw = await page.evaluate(() => ({ attr: document.documentElement.getAttribute("data-theme"), bg: getComputedStyle(document.body).backgroundColor, cs: getComputedStyle(document.documentElement).colorScheme, pressed: [...document.querySelectorAll('.spec-head .seg button')].map(b => b.getAttribute('aria-pressed')).join(','), stepColours: [...document.querySelectorAll('#steps div')].map(d => d.style.color).join(' ') }));
    report[key].manualSwitchTo = { other, ...sw };
    await page.screenshot({ path: path.join(outDir, `${key}-switched-${other}-top.png`), clip: { x: 0, y: 0, width, height: 900 } });
    await ctx.close();
  }
}
await browser.close();
fs.writeFileSync(path.join(outDir, "report.json"), JSON.stringify(report, null, 2));
console.log(JSON.stringify(report, null, 1));
