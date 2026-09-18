// Generates the CSS variable block and the embedded JSON for the specimen from tokens.json.
import fs from "node:fs";
const file = process.argv[2];
const T = JSON.parse(fs.readFileSync(file, "utf8"));
const v = (k) => "--" + k.replace(/\./g, "-");
const px = (n) => (typeof n === "number" ? n + "px" : n);
let plain = [];
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
const css = `/* Generated from tokens.json (${n} colour tokens, version ${T.version}). Do not edit by hand. */
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
fs.writeFileSync("tokens.css", css + "\n");
fs.writeFileSync("tokens.embed.json", JSON.stringify(T));
console.log("css lines", css.split("\n").length, "| colour tokens", n, "| embed chars", JSON.stringify(T).length);
