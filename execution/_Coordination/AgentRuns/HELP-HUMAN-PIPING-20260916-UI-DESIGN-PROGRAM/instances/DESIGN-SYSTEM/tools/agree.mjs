// Agreement check, V1.1: tokens.json is the source; the document's colour tables and contrast table, the
// specimen's three CSS variable blocks, its embedded token JSON and its embedded pair list must all carry
// the same names and values; the prose must carry the plain tokens it quotes; the change log must cite every
// decision, frame decision, departure and token gap; the specimen must contain no forbidden string and no
// external reference.
//   node agree.mjs <DESIGN_SYSTEM_V1.md> <tokens.json> <specimen.html>
import fs from "node:fs"; import crypto from "node:crypto";
import { pairs, computeRows, toMarkdown } from "./contrast.mjs";
import { cssFrom, colourTablesFrom } from "./gen.mjs";
const [doc, tok, spec] = process.argv.slice(2);
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
if (!D.includes("Status: V1.1")) push("doc status line is not V1.1");
if (T.version !== "1.1") push("tokens.json version is not 1.1");
if (!S.includes("specimen V1.1")) push("specimen heading is not V1.1");
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
for (const s of ["s1", "s2", "s3", "s4_both", "s4_table", "s5", "s6", "s7_both", "s7_table", "s8_table", "s8_model", "s9"]) if (!log.includes("§2 " + s) && !log.includes("§2 (" + s)) push("section 9 does not cite MOCKS_V1 §2 " + s);

// ---- forbidden strings and external references in the specimen ----
const M02 = "Results are engineering decision-support information. Acceptance, professional judgment, and any certification, sealing, or code-compliance determination remain with the responsible engineer and project authority.";
const Sx = S.split(M02).join("");
for (const bad of ["F-PIP-2", "claim fence", "DEC-081", "OpenPipeStress", "GF-TOKEN"]) if (Sx.includes(bad)) push("specimen contains " + bad);
for (const re of [/\bapprov/i, /\bcertif/i, /\bseal/i, /\bauthenticat/i, /\bcomplian/i, /\bsign[ -]off\b/i]) { const m = Sx.match(re); if (m) push("specimen contains a forbidden word: " + m[0]); }
if (!/Standard claim fence applies \(F-PIP-2/.test(D)) push("doc lacks the fence line");
for (const m of S.matchAll(/\b(?:src|href)=["'](https?:)?\/\//g)) push("external reference: " + m[0]);
if (/@import|url\(\s*["']?https?:/.test(S)) push("external css/url reference");
if (/\/Users\//.test(S) || /\/Users\//.test(D)) push("an absolute user path appears in a deliverable");

const leaves = (o) => Object.values(o).reduce((n, v) => n + (v && typeof v === "object" && !Array.isArray(v) && !("light" in v) ? leaves(v) : 1), 0);
const plain = Object.entries(T).filter(([k]) => !["name", "version", "date", "note", "color"].includes(k)).reduce((n, [, v]) => n + leaves(v), 0);
const sha = f => crypto.createHash("sha256").update(fs.readFileSync(f)).digest("hex");
console.log(JSON.stringify({ colourTokens: names.length, plainTokens: plain, docColourRows: rows.length, contrastRows: inDoc.length, pairs: pairs.length, problems, sha256: { tokens: sha(tok), doc: sha(doc), specimen: sha(spec) }, bytes: { doc: D.length, specimen: S.length, tokens: fs.statSync(tok).size } }, null, 1));
process.exitCode = problems.length ? 1 : 0;
