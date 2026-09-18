// Generates src/tokens.css from the product's own token file, src/design/tokens.json.
//   npm run tokens
// `cssFrom` is carried over from the design system's generator (DESIGN-SYSTEM/tools/gen.mjs,
// design system V1.3, tokens 1.2): the same variable naming and the same three theme blocks
// (light; dark by prefers-color-scheme unless light is forced; dark by the explicit attribute).
// The product build never reads the design run folder; see src/design/TOKENS_SOURCE.md.
// src/design/tokens.test.ts asserts that src/tokens.css equals this generator's output.
import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

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

const desktopRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..");
export const TOKENS_JSON = path.join(desktopRoot, "src", "design", "tokens.json");
export const TOKENS_CSS = path.join(desktopRoot, "src", "tokens.css");

export function generatedCss() {
  return cssFrom(JSON.parse(fs.readFileSync(TOKENS_JSON, "utf8"))) + "\n";
}

if (process.argv[1] && path.resolve(process.argv[1]) === fileURLToPath(import.meta.url)) {
  const css = generatedCss();
  fs.writeFileSync(TOKENS_CSS, css);
  console.log("generated src/tokens.css from src/design/tokens.json |", css.split("\n").length - 1, "lines");
}
