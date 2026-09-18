// Agreement check, V1.3 (the document and specimen are V1.3; tokens.json stays 1.2): tokens.json is the source; the document's colour tables and contrast table, the
// specimen's three CSS variable blocks, its embedded token JSON and its embedded pair list must all carry
// the same names and values; the prose must carry the plain tokens it quotes; the change log must cite every
// decision, frame decision, departure and token gap, and for V1.2 every ruling, question, gap and R item, with
// rows 1 to 49 unchanged from V1.1; the label table in the document must equal a fresh generation and every
// label chip in the specimen must be a row of it; agent cards carry one of the five class words; no deliverable
// carries a retired string, an external reference or a machine path.
//   node agree.mjs <DESIGN_SYSTEM_V1.md> <tokens.json> <specimen.html>
import fs from "node:fs"; import crypto from "node:crypto"; import path from "node:path"; import { fileURLToPath } from "node:url";
import { pairs, computeRows, toMarkdown } from "./contrast.mjs";
import { cssFrom, colourTablesFrom, labelTableFrom } from "./gen.mjs";
const [doc, tok, spec] = process.argv.slice(2);
const here = path.dirname(fileURLToPath(import.meta.url));
const T = JSON.parse(fs.readFileSync(tok, "utf8")); const D = fs.readFileSync(doc, "utf8"); const S = fs.readFileSync(spec, "utf8");
const problems = [];
const push = (p) => problems.push(p);

// ---- document colour tables ----
const rows = [...D.matchAll(/^\| `([\w.]+)` \| `(--[\w-]+)` \| `([^`]+)` \| `([^`]+)`( \([a-z]+\))? \|$/gm)];
const docMap = Object.fromEntries(rows.map(m => [m[1], { v: m[2], light: m[3], dark: m[4] }]));
const cssVar = k => "--" + k.replace(/\./g, "-");
const names = Object.keys(T.color);
if (names.length !== Object.keys(docMap).length) push(`doc has ${Object.keys(docMap).length} colour rows, tokens.json ${names.length}`);
for (const k of names) {
  const t = T.color[k], d = docMap[k];
  if (!d) { push("doc missing " + k); continue; }
  if (d.light !== t.light || d.dark !== t.dark) push(`doc value differs for ${k}`);
  if (d.v !== cssVar(k)) push(`doc css variable differs for ${k}`);
}
for (const k of Object.keys(docMap)) if (!T.color[k]) push("doc extra " + k);
const genTables = colourTablesFrom(T);
const docTables = (D.match(/<!-- GENERATED:COLOUR_TABLES:BEGIN -->\n([\s\S]*?)\n<!-- GENERATED:COLOUR_TABLES:END -->/) || [])[1];
if (docTables !== genTables) push("doc colour tables differ from a fresh generation");

// ---- specimen CSS blocks, embedded JSON, embedded pairs ----
const lightBlock = (S.match(/:root, :root\[data-theme="light"\] \{([\s\S]*?)\n\}/) || [])[1] || "";
const darkMedia = (S.match(/@media \(prefers-color-scheme: dark\) \{\s*:root:not\(\[data-theme="light"\]\) \{([\s\S]*?)\n  \}\n\}/) || [])[1] || "";
const darkBlock = (S.match(/\n:root\[data-theme="dark"\] \{([\s\S]*?)\n\}/) || [])[1] || "";
const grab = (block, v) => { const m = block.match(new RegExp("\\s" + v.replace(/-/g, "\\-") + ": ([^;]+);")); return m && m[1].trim(); };
for (const k of names) {
  const t = T.color[k], v = cssVar(k);
  for (const [blk, theme, label] of [[lightBlock, "light", "light block"], [darkMedia, "dark", "media block"], [darkBlock, "dark", "dark block"]]) { const g = grab(blk, v); if (g !== t[theme]) push(`specimen ${label} ${v}: ${g} vs ${t[theme]}`); }
}
const cssGen = (S.match(/\/\* GENERATED:TOKENS_CSS:BEGIN \*\/\n([\s\S]*?)\n\/\* GENERATED:TOKENS_CSS:END \*\//) || [])[1];
if (cssGen !== cssFrom(T)) push("specimen CSS block differs from a fresh generation");
const embed = JSON.parse((S.match(/<script id="tokens" type="application\/json">([\s\S]*?)<\/script>/) || [, "null"])[1]);
if (JSON.stringify(embed) !== JSON.stringify(T)) push("embedded token JSON differs from tokens.json");
const embedPairs = JSON.parse((S.match(/<script id="pairs" type="application\/json">([\s\S]*?)<\/script>/) || [, "null"])[1]);
if (JSON.stringify(embedPairs) !== JSON.stringify(pairs)) push("embedded pair list differs from contrast.mjs");
for (const [fg, bg] of pairs) for (const p of [fg, bg]) for (const q of p.split("/")) if (!T.color[q]) push("pair names an unknown token: " + q);

// ---- document contrast table ----
const fresh = toMarkdown(computeRows(T)).split("\n").filter(l => /^\| `.*:1 \|$/.test(l));
const inDoc = D.split("\n").filter(l => /^\| `.*:1 \|$/.test(l));
if (fresh.length !== inDoc.length || fresh.some((l, i) => l !== inDoc[i])) push(`contrast table differs: fresh ${fresh.length} rows, doc ${inDoc.length}`);

// ---- plain-token phrases in the prose ----
const L = T.layout;
const checks = [
  ["row 26 px, header row 28 px", L.row === 26 && L.headerRow === 28], ["gutter column 32 px", L.gutter === 32], ["marks column 72 px", L.marksColumn === 72],
  ["toolbar 48 px, status bar 24 px", L.toolbar === 48 && L.statusBar === 24], ["13 / 18", T.type["size.body"] === 13 && T.type["lineHeight.body"] === 18],
  ["22 / 28", T.type["size.figure"] === 22 && T.type["lineHeight.figure"] === 28], ["11 / 14", T.type["size.caption"] === 11 && T.type["lineHeight.caption"] === 14],
  ["`.9` 32", T.space["9"] === 32], ["`radius.card` 6", T.radius.card === 6],
  ["label in `text.secondary` at a fixed 88 px", L["inspector.label"] === 88], ["minimum drawing width of 220 px", L["canvas.min"] === 220],
  ["header cell padding 0 5", L.headerPadding === 5], ["Cell padding 0 8", L.cellPadding === 8],
  ["canvas 303", L["inspector.both"] === 300], ["docked inspector of 340", L["inspector.model"] === 340],
];
for (const [phrase, ok] of checks) { if (!D.includes(phrase)) push("doc phrase not found: " + phrase); if (!ok) push("tokens.json disagrees with: " + phrase); }
checks.length = 0;
for (const [phrase, ok] of [
  ["304 px wide (`layout.hud.width`)", L["hud.width"] === 304], ["154 px wide (`layout.hud.widthWrapped`)", L["hud.widthWrapped"] === 154],
  ["400 px or more (`layout.hud.wrapBelow`)", L["hud.wrapBelow"] === 400], ["inset 8 px (`layout.hud.inset`)", L["hud.inset"] === 8],
  ["320 px wide (`layout.toast.width`)", L["toast.width"] === 320], ["8 px (`layout.toast.inset`)", L["toast.inset"] === 8],
  ["360 px wide (`layout.runlog.width`)", L["runlog.width"] === 360], ["Duration: 6 s, or 10 s", T.motion["toast.ms"] === 6000 && T.motion["toastAction.ms"] === 10000],
]) { if (!D.includes(phrase)) push("doc phrase not found: " + phrase); if (!ok) push("tokens.json disagrees with: " + phrase); }
if (!D.includes("Status: V1.3")) push("doc status line is not V1.3");
if (T.version !== "1.2") push("tokens.json version is not 1.2");
if (!S.includes("specimen V1.3")) push("specimen heading is not V1.3");
if (!S.includes(`version ${T.version}`)) push("specimen CSS comment does not carry the token version");
const dl = T.color["result.scale.7"].dark, edge = T.color["canvas.edge"].dark;
if (!D.includes(dl) || !D.includes(edge)) push("doc 2.6 does not quote the re-anchored top step and the edge value");

// ---- change log coverage (section 9) ----
const log = D.split("## 9. Change log")[1] || "";
if (!log) push("doc lacks section 9");
const cited = (re) => new Set([...log.matchAll(re)].map(m => m[1]));
const dec = cited(/decision (\d+)/g); for (let i = 1; i <= 14; i++) if (!dec.has(String(i))) push("section 9 does not cite decision " + i);
const dd = cited(/\bD-(\d+)/g); for (let i = 1; i <= 11; i++) if (!dd.has(String(i))) push("section 9 does not cite D-" + i);
const pp = cited(/\bP-(\d+)/g); for (let i = 1; i <= 10; i++) if (!pp.has(String(i))) push("section 9 does not cite P-" + i);
const gg = cited(/\bG-(\d+)/g); for (let i = 1; i <= 6; i++) if (!gg.has(String(i))) push("section 9 does not cite G-" + i);
const logRows = log.split("\n").map(l => (l.match(/^\| (\d+) \| /) || [])[1]).filter(Boolean).map(Number);
logRows.forEach((n, i) => { if (n !== i + 1) push(`section 9 rows are not contiguous at position ${i + 1} (found ${n})`); });
if (logRows.length < 50) push("section 9 has no V1.2 rows");
if (logRows.length < 89) push("section 9 lacks the correction rows 88 and 89");
if (logRows.length < 90) push("section 9 lacks correction 2's row 90");
const first89 = log.split("\n").filter(l => { const m = l.match(/^\| (\d+) \| /); return m && Number(m[1]) <= 89; }).join("\n");
if (crypto.createHash("sha256").update(first89).digest("hex") !== "11df7f507ea58430e14ca2058e606e21ea3675dbbeb730d2c88a727b70f16f09") push("section 9 rows 1 to 89 differ from V1.2 after correction 1");
// V1.3: rows 91 on, rows 1 to 90 unchanged, every item of the brief DESIGN-SYSTEM-04 and its supplement cited
if (logRows.length < 110) push("section 9 lacks the V1.3 rows 91 to 110");
const first90 = log.split("\n").filter(l => { const m = l.match(/^\| (\d+) \| /); return m && Number(m[1]) <= 90; }).join("\n");
if (crypto.createHash("sha256").update(first90).digest("hex") !== "e834e393087aab16dae7c0da260653c299a0d3ea6ef54dea2d7f0570be0f65db") push("section 9 rows 1 to 90 differ from V1.2 after correction 2");
const v13 = log.split("\n").filter(l => { const m = l.match(/^\| (\d+) \| /); return m && Number(m[1]) >= 91; }).join("\n");
for (const n of [1, 2, 4, 5, 8, 9, 10, 11, 12, 13, 14, 15, 16]) if (!new RegExp("contradiction " + n + "\\b").test(v13)) push("V1.3 rows do not cite contradiction " + n);
for (const c of ["C-17", "C-20", "C-23", "C-24", "C-25"]) if (!v13.includes(c)) push("V1.3 rows do not cite " + c);
if (/owner's amendment to Q-20; general/.test(D) || /exactly these and no others/.test(D)) push("doc carries a framing that correction 2 removed");
const first87 = log.split("\n").filter(l => { const m = l.match(/^\| (\d+) \| /); return m && Number(m[1]) <= 87; }).join("\n");
if (crypto.createHash("sha256").update(first87).digest("hex") !== "c95d0c9ce6b0999d76c63269628f901d56d52563c91d4bd5de41becb77037616") push("section 9 rows 1 to 87 differ from V1.2 as returned");
const first49 = log.split("\n").filter(l => /^\| ([1-9]|[1-4][0-9]) \| /.test(l)).join("\n");
if (crypto.createHash("sha256").update(first49).digest("hex") !== "bb04eb1863385e1cddf0bda328228bbd960987602588b722d6801f19b0b3598d") push("section 9 rows 1 to 49 differ from V1.1");
const v12 = log.split("\n").filter(l => { const m = l.match(/^\| (\d+) \| /); return m && Number(m[1]) >= 50; }).join("\n");
const cited12 = (re) => new Set([...v12.matchAll(re)].map(m => m[1]));
const expand = (set, text, re) => { for (const m of text.matchAll(re)) for (let i = Number(m[1]); i <= Number(m[2]); i++) set.add(String(i)); return set; };
const rul = expand(cited12(/ruling (\d+)/g), v12, /rulings (\d+) to (\d+)/g); for (let i = 1; i <= 9; i++) if (!rul.has(String(i))) push("V1.2 rows do not cite ruling " + i);
const qq = expand(cited12(/\bQ-(\d+)/g), v12, /Q-(\d+) to Q-(\d+)/g); for (let i = 15; i <= 22; i++) if (!qq.has(String(i))) push("V1.2 rows do not cite Q-" + i);
const g12 = expand(cited12(/\bG-(\d+)/g), v12, /G-(\d+) to G-(\d+)/g); for (let i = 7; i <= 12; i++) if (!g12.has(String(i))) push("V1.2 rows do not cite G-" + i);
const rr = cited12(/\bR-(\d+)/g); for (const i of [4, 5, 6]) if (!rr.has(String(i))) push("V1.2 rows do not cite R-" + i);
for (const s of ["s1", "s2", "s3", "s4_both", "s4_table", "s5", "s6", "s7_both", "s7_table", "s8_table", "s8_model", "s9"]) if (!log.includes("§2 " + s) && !log.includes("§2 (" + s)) push("section 9 does not cite MOCKS_V1 §2 " + s);

// ---- the label table (section 2.3) and the label chips ----
const docLabels = (D.match(/<!-- GENERATED:LABEL_TABLE:BEGIN -->\n([\s\S]*?)\n<!-- GENERATED:LABEL_TABLE:END -->/) || [])[1];
if (docLabels !== labelTableFrom(T)) push("doc label table differs from a fresh generation");
const LAB = T.labels; const labelKeys = Object.keys(LAB);
if (labelKeys.length !== 8) push("the label table does not have eight rows");
const chipRe = /<span class="chip (\w+) lbl" data-label="(\w+)" title="(\w+)"><span class="dom">([^<]+)<\/span><span class="sep">·<\/span>([^<]+)<\/span>/g;
let chipCount = 0; const seen = new Set();
for (const m of S.matchAll(chipRe)) { chipCount++; seen.add(m[2]); const r = LAB[m[2]]; if (!r) { push("specimen chip names an unknown label: " + m[2]); continue; } if (m[1] !== r.chip || m[3] !== m[2] || m[4] !== r.domain || m[5] !== r.label) push("specimen chip differs from the label table: " + m[2]); }
if ((S.match(/data-label="/g) || []).length !== chipCount) push("a specimen label chip does not follow the chip markup");
for (const k of labelKeys) if (!seen.has(k)) push("specimen draws no chip for " + k);
// outside the chips and the two embedded JSON blocks, a label appears only as prose about the label, never as a bare chip
const Sbody = S.replace(/<script[\s\S]*?<\/script>/g, "").replace(chipRe, "");
for (const k of labelKeys) if (new RegExp('class="chip[^"]*"[^>]*>' + LAB[k].label.replace(/[-]/g, "\\-") + "<").test(Sbody)) push("specimen draws a label outside the chip markup: " + LAB[k].label);
for (const old of ["User rules checked", "User rule failed", "User-rules", "Rules checked"]) for (const [n, txt] of [["doc", D], ["specimen", S], ["tokens", JSON.stringify(T)]]) if (txt.toLowerCase().includes(old.toLowerCase())) push(`${n} carries a label form outside the table: ${old}`);
for (const k of labelKeys) if (!D.includes("| `" + k + "` | " + LAB[k].label + " |")) push("doc label table lacks " + k);

// ---- agent card classes (ruling 8) ----
const classes = T.agentCardClasses || [];
if (JSON.stringify(classes) !== JSON.stringify(["Check", "Open issue", "Draft", "Proposal", "Evidence summary"])) push("agentCardClasses is not the five ruled words");
const usedClasses = new Set();
for (const m of S.matchAll(/data-agent-class="([^"]+)"/g)) { usedClasses.add(m[1]); if (!classes.includes(m[1])) push("specimen agent card class outside the five: " + m[1]); }
for (const c of classes) { if (!usedClasses.has(c)) push("specimen shows no agent card of class " + c); if (!D.includes("| " + c + " |")) push("doc card class table lacks " + c); }
for (const m of S.matchAll(/<div data-agent-text>([\s\S]*?)<\/div>/g)) { const w = m[1].match(/\b(accepted|approved|verified|correct|compliant)\b/i); if (w) push("specimen agent text contains a barred word: " + w[0]); }
if (/>Agent · [^<]*<\/span><\/div>(?!<div data-agent-text>)/.test(S)) push("an agent card's text is not marked as agent text");

// ---- retired strings: built by concatenation so that this file does not carry them either ----
const retired = [["SWB Pip" + "ing Designer", "the longer name"], ["Pip" + "ing Designer", "the longer name"], ["Open" + "Pipe" + "Stress", "the old name"], ["CAE" + "PIPE", "another vendor's product"],
  ["Technical " + "preview", "the maturity sentence"], ["not a released " + "product", "the maturity sentence"],
  ["decision-support " + "information", "the acceptance sentence"], ["remain with the responsible " + "engineer", "the acceptance sentence"], ["Acceptance, professional " + "judgment", "the acceptance sentence"],
  ["stays the solve " + "basis", "the stale band's removed clause (correction 1)"]];
const instanceDir = path.dirname(path.resolve(doc));
for (const [n, txt] of [["doc", D], ["specimen", S], ["tokens", fs.readFileSync(tok, "utf8")]]) for (const [bad, what] of retired) if (txt.toLowerCase().includes(bad.toLowerCase())) push(`${n} carries ${what}`);

// ---- V1.3: "Commit" is no button's copy, and every tooltip names the control and then its key in parentheses ----
for (const m of S.matchAll(/<button\b([^>]*)>([\s\S]*?)<\/button>/g)) { const face = m[2].replace(/<title>[\s\S]*?<\/title>/g, "").replace(/<[^>]+>/g, ""); const tip = (m[1].match(/title="([^"]*)"/) || [, ""])[1]; if (/\bcommit\b/i.test(face) || /\bcommit\b/i.test(tip) || /aria-label="[^"]*\bcommit\b/i.test(m[1])) push("specimen shows Commit as button copy"); }
if (/>\s*Commit\s*</.test(Sbody)) push("specimen shows Commit as a control's text");
const KEYS = /[⌘⇧⌥↩⎋⇥⌫↓↑]/;
for (const m of S.matchAll(/title="([^"]*)"|<title>([^<]*)<\/title>/g)) { const tip = m[1] ?? m[2]; if (KEYS.test(tip.replace(/\([^)]*\)/g, ""))) push("specimen tooltip writes a key outside parentheses: " + tip); }
for (const m of S.matchAll(/<button\b[^>]*>([\s\S]*?)<\/button>/g)) { const face = m[1].replace(/<title>[\s\S]*?<\/title>/g, "").replace(/<[^>]+>/g, ""); if (KEYS.test(face)) push("specimen button face carries a key: " + face.trim()); }
for (const old of ["Historical saved run · Run 02", "Recorded on Run 02"]) if (S.includes(old) || D.split("## 9. Change log")[0].includes(old)) push("a Historical example still reads Run 02: " + old);

// ---- forbidden strings, sprite references and external references in the specimen ----
for (const bad of ["F-PIP-2", "claim fence", "DEC-081", "GF-TOKEN"]) if (S.includes(bad)) push("specimen contains " + bad);
for (const re of [/\bapprov/i, /\bcertif/i, /\bseal/i, /\bauthenticat/i, /\bcomplian/i, /\bsign[ -]off\b/i]) { const m = Sbody.match(re); if (m) push("specimen contains a forbidden word: " + m[0]); }
const symbols = new Set([...S.matchAll(/<symbol id="([\w-]+)"/g)].map(m => m[1]));
for (const m of S.matchAll(/<use href="#([\w-]+)"/g)) if (!symbols.has(m[1])) push("specimen uses a missing symbol: " + m[1]);
if (!/Standard claim fence applies \(F-PIP-2; claims taxonomy per DEC-081\)\.\s*$/.test(D)) push("doc does not end with the fence line");
for (const m of S.matchAll(/\b(?:src|href)=["'](https?:)?\/\//g)) push("external reference: " + m[0]);
if (/@import|url\(\s*["']?https?:/.test(S)) push("external css/url reference");
// no machine path in any file of the instance
const walk = (d) => fs.readdirSync(d, { withFileTypes: true }).flatMap(e => e.isDirectory() ? walk(path.join(d, e.name)) : [path.join(d, e.name)]);
const machine = new RegExp("/(Us" + "ers|ho" + "me|private|var/folders|tmp)/[\\w.-]+/");
for (const f of walk(instanceDir)) { if (!/\.(md|json|html|mjs|js|css|txt)$/.test(f)) continue; if (machine.test(fs.readFileSync(f, "utf8"))) push("a machine path appears in " + path.relative(instanceDir, f)); }
void here;

const leaves = (o) => Object.values(o).reduce((n, v) => n + (v && typeof v === "object" && !Array.isArray(v) && !("light" in v) ? leaves(v) : 1), 0);
const plain = Object.entries(T).filter(([k]) => !["name", "version", "date", "note", "color", "labels", "agentCardClasses"].includes(k)).reduce((n, [, v]) => n + leaves(v), 0);
const sha = f => crypto.createHash("sha256").update(fs.readFileSync(f)).digest("hex");
console.log(JSON.stringify({ colourTokens: names.length, labels: labelKeys.length, labelChips: chipCount, agentCardClasses: classes.length, changeLogRows: logRows.length, plainTokens: plain, docColourRows: rows.length, contrastRows: inDoc.length, pairs: pairs.length, problems, sha256: { tokens: sha(tok), doc: sha(doc), specimen: sha(spec) }, bytes: { doc: D.length, specimen: S.length, tokens: fs.statSync(tok).size } }, null, 1));
process.exitCode = problems.length ? 1 : 0;
