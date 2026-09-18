// Generates, from tokens.json, everything the specimen and the document embed:
//   tokens.css          the CSS variable blocks (light; dark by prefers-color-scheme; dark by the manual switch)
//   tokens.embed.json   the token file as one line, for the specimen's <script id="tokens">
//   pairs.embed.json    the contrast pair list, for the specimen's <script id="pairs">
//   colour_tables.md    the document's section 2.2 tables, one per token group
//   contrast_table.md   the document's section 2.9 table
//   node gen.mjs <tokens.json> <out dir>
import fs from "node:fs";
import path from "node:path";
import { pairs, computeRows, toMarkdown } from "./contrast.mjs";

export function cssFrom(T) {
  const v = (k) => "--" + k.replace(/\./g, "-");
  const px = (n) => (typeof n === "number" ? n + "px" : n);
  const plain = [];
  for (const [k, val] of Object.entries(T.type)) {
    if (k.startsWith("family.")) plain.push(`  --font-${k.slice(7)}: ${val};`);
    else if (k === "numeric.feature") continue;
    else if (k.startsWith("weight.")) plain.push(`  ${v("type." + k)}: ${val};`);
    else plain.push(`  ${v("type." + k)}: ${px(val)};`);
  }
  for (const [k, val] of Object.entries(T.space)) plain.push(`  --space-${k}: ${px(val)};`);
  for (const [k, val] of Object.entries(T.radius)) plain.push(`  --radius-${k}: ${px(val)};`);
  for (const [k, val] of Object.entries(T.border)) plain.push(`  --border-width-${k}: ${px(val)};`);
  for (const [k, val] of Object.entries(T.layout)) { if (Array.isArray(val)) continue; plain.push(`  ${v("layout." + k)}: ${px(val)};`); }
  for (const [k, val] of Object.entries(T.motion)) {
    if (k.endsWith(".ms")) plain.push(`  ${v("motion." + k)}: ${val}ms;`);
    else if (k.startsWith("easing.")) plain.push(`  --easing-${k.slice(7)}: ${val};`);
  }
  plain.push(`  --icon-stroke: ${px(T.icon.stroke)};`);
  const themed = (theme) => {
    const out = [];
    for (const [k, val] of Object.entries(T.color)) out.push(`  ${v(k)}: ${val[theme]};`);
    for (const [k, val] of Object.entries(T.elevation)) if (typeof val === "object") out.push(`  --elevation-${k}: ${val[theme]};`);
    return out.join("\n");
  };
  const n = Object.keys(T.color).length;
  return `/* Generated from tokens.json (${n} colour tokens, version ${T.version}). Do not edit by hand. */
:root {
${plain.join("\n")}
}
:root, :root[data-theme="light"] {
  color-scheme: light;
${themed("light")}
}
@media (prefers-color-scheme: dark) {
  :root:not([data-theme="light"]) {
    color-scheme: dark;
${themed("dark").replace(/^/gm, "  ")}
  }
}
:root[data-theme="dark"] {
  color-scheme: dark;
${themed("dark")}
}`;
}

// Section 2.2: one table per group (the part of the name before the first dot), in order of first appearance.
export function colourTablesFrom(T) {
  const groups = new Map();
  for (const k of Object.keys(T.color)) { const g = k.split(".")[0]; if (!groups.has(g)) groups.set(g, []); groups.get(g).push(k); }
  const out = [];
  for (const [g, keys] of groups) {
    out.push(`**${g}**`, "", "| Token | CSS variable | Light | Dark |", "|---|---|---|---|");
    for (const k of keys) { const c = T.color[k]; out.push(`| \`${k}\` | \`--${k.replace(/\./g, "-")}\` | \`${c.light}\` | \`${c.dark}\`${c.hue ? ` (${c.hue})` : ""} |`); }
    out.push("");
  }
  return out.join("\n").trimEnd();
}

if (process.argv[1] && /gen\.mjs$/.test(process.argv[1])) {
  const T = JSON.parse(fs.readFileSync(process.argv[2], "utf8"));
  const outDir = process.argv[3] || ".";
  fs.mkdirSync(outDir, { recursive: true });
  fs.writeFileSync(path.join(outDir, "tokens.css"), cssFrom(T) + "\n");
  fs.writeFileSync(path.join(outDir, "tokens.embed.json"), JSON.stringify(T));
  fs.writeFileSync(path.join(outDir, "pairs.embed.json"), JSON.stringify(pairs));
  fs.writeFileSync(path.join(outDir, "colour_tables.md"), colourTablesFrom(T) + "\n");
  fs.writeFileSync(path.join(outDir, "contrast_table.md"), toMarkdown(computeRows(T)) + "\n");
  console.log("generated in", outDir, "| colour tokens", Object.keys(T.color).length, "| pairs", pairs.length, "| version", T.version);
}
