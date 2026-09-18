// Agreement check: tokens.json is the source; the document's colour tables, its contrast table,
// the specimen's three CSS variable blocks and its embedded JSON must all carry the same names and values.
import fs from "node:fs"; import { execSync } from "node:child_process"; import crypto from "node:crypto";
const [doc, tok, spec] = process.argv.slice(2);
const T = JSON.parse(fs.readFileSync(tok, "utf8")); const D = fs.readFileSync(doc, "utf8"); const S = fs.readFileSync(spec, "utf8");
const problems = [];
const rows = [...D.matchAll(/^\| `([\w.]+)` \| `(--[\w-]+)` \| `([^`]+)` \| `([^`]+)`( \([a-z]+\))? \|$/gm)];
const docMap = Object.fromEntries(rows.map(m => [m[1], { v: m[2], light: m[3], dark: m[4] }]));
const cssVar = k => "--" + k.replace(/\./g, "-");
const lightBlock = S.match(/:root, :root\[data-theme="light"\] \{([\s\S]*?)\n\}/)[1];
const darkMedia = S.match(/@media \(prefers-color-scheme: dark\) \{\s*:root:not\(\[data-theme="light"\]\) \{([\s\S]*?)\n  \}\n\}/)[1];
const darkBlock = S.match(/\n:root\[data-theme="dark"\] \{([\s\S]*?)\n\}/)[1];
const grab = (block, v) => { const m = block.match(new RegExp("\\s" + v.replace(/-/g, "\\-") + ": ([^;]+);")); return m && m[1].trim(); };
const embed = JSON.parse(S.match(/<script id="tokens" type="application\/json">([\s\S]*?)<\/script>/)[1]);
const names = Object.keys(T.color);
if (names.length !== Object.keys(docMap).length) problems.push(`doc has ${Object.keys(docMap).length} colour rows, tokens.json ${names.length}`);
for (const k of names) {
  const t = T.color[k], d = docMap[k], v = cssVar(k);
  if (!d) { problems.push("doc missing " + k); continue; }
  if (d.light !== t.light || d.dark !== t.dark) problems.push(`doc value differs for ${k}`);
  if (d.v !== v) problems.push(`doc css variable differs for ${k}`);
  for (const [blk, theme, label] of [[lightBlock, "light", "light block"], [darkMedia, "dark", "media block"], [darkBlock, "dark", "dark block"]]) { const g = grab(blk, v); if (g !== t[theme]) problems.push(`specimen ${label} ${v}: ${g} vs ${t[theme]}`); }
  const e = embed.color[k]; if (!e || e.light !== t.light || e.dark !== t.dark) problems.push("embed differs for " + k);
}
for (const k of Object.keys(docMap)) if (!T.color[k]) problems.push("doc extra " + k);
if (JSON.stringify(embed) !== JSON.stringify(T)) problems.push("embedded JSON differs from tokens.json");
// contrast table: recompute from tokens.json and compare with the document's rows
execSync(`node contrast.mjs "${tok}" contrast_check.md`, { stdio: "ignore" });
const fresh = fs.readFileSync("contrast_check.md", "utf8").split("\n").filter(l => /^\| `.*:1 \|$/.test(l));
const inDoc = D.split("\n").filter(l => /^\| `.*:1 \|$/.test(l));
if (fresh.length !== inDoc.length || fresh.some((l, i) => l !== inDoc[i])) problems.push(`contrast table differs: fresh ${fresh.length} rows, doc ${inDoc.length}`);
// plain-token spot checks against the document's prose
const checks = [["row 26 px, header row 28 px", T.layout.row === 26 && T.layout.headerRow === 28], ["gutter column 32 px", T.layout.gutter === 32], ["marks column 72 px", T.layout.marksColumn === 72], ["toolbar 48 px, status bar 24 px", T.layout.toolbar === 48 && T.layout.statusBar === 24], ["13 / 18", T.type["size.body"] === 13 && T.type["lineHeight.body"] === 18], ["22 / 28", T.type["size.figure"] === 22 && T.type["lineHeight.figure"] === 28], ["11 / 14", T.type["size.caption"] === 11 && T.type["lineHeight.caption"] === 14], ["`.9` 32", T.space["9"] === 32], ["`radius.card` 6", T.radius.card === 6]];
for (const [phrase, ok] of checks) { if (!D.includes(phrase)) problems.push("doc phrase not found: " + phrase); if (!ok) problems.push("tokens.json disagrees with: " + phrase); }
// forbidden strings in the specimen (a product-like surface)
for (const bad of ["F-PIP-2", "claim fence", "DEC-081", "OpenPipeStress", "GF-TOKEN"]) if (S.includes(bad)) problems.push("specimen contains " + bad);
if (!/Standard claim fence applies \(F-PIP-2/.test(D)) problems.push("doc lacks the fence line");
// external references in the specimen
for (const m of S.matchAll(/\b(?:src|href)=["'](https?:)?\/\//g)) problems.push("external reference: " + m[0]);
if (/@import|url\(\s*["']?https?:/.test(S)) problems.push("external css/url reference");
const leaves = (o) => Object.values(o).reduce((n, v) => n + (v && typeof v === "object" && !Array.isArray(v) && !("light" in v) ? leaves(v) : 1), 0);
const plain = Object.entries(T).filter(([k]) => !["name", "version", "date", "note", "color"].includes(k)).reduce((n, [, v]) => n + leaves(v), 0);
const sha = f => crypto.createHash("sha256").update(fs.readFileSync(f)).digest("hex");
console.log(JSON.stringify({ colourTokens: names.length, plainTokens: plain, docColourRows: rows.length, contrastRows: inDoc.length, problems, sha256: { tokens: sha(tok), doc: sha(doc), specimen: sha(spec) }, bytes: { doc: D.length, specimen: S.length, tokens: fs.statSync(tok).size } }, null, 1));
