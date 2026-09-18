// Contrast findings: reads tokens.json and reports the WCAG 2.x ratio of every text and mark
// pairing in both themes. Findings only; no target is named. The pair list is exported so that
// gen.mjs embeds the same list in the specimen and agree.mjs checks both against it.
//   node contrast.mjs <tokens.json> <out.md>
import fs from "node:fs";

const hex2rgb = (h) => { h = h.replace("#", ""); return [0, 2, 4].map(i => parseInt(h.slice(i, i + 2), 16)); };
const parseColor = (s) => {
  if (s.startsWith("#")) return { rgb: hex2rgb(s), a: 1 };
  const m = s.match(/rgba?\(([^)]+)\)/);
  const p = m[1].split(",").map(Number);
  return { rgb: p.slice(0, 3), a: p.length > 3 ? p[3] : 1 };
};
const over = (c, b) => c.rgb.map((v, i) => Math.round(c.a * v + (1 - c.a) * b.rgb[i]));
const lum = (rgb) => { const [r, g, b] = rgb.map(v => { v /= 255; return v <= 0.04045 ? v / 12.92 : ((v + 0.055) / 1.055) ** 2.4; }); return 0.2126 * r + 0.7152 * g + 0.0722 * b; };
const ratio = (fg, bg) => { const [hi, lo] = [lum(fg), lum(bg)].sort((x, y) => y - x); return (hi + 0.05) / (lo + 0.05); };

// name may be "a" or "a/b": a composited over b (and b over white if translucent)
function resolve(C, name, theme) {
  let acc = { rgb: [255, 255, 255], a: 1 };
  for (const p of name.split("/").reverse()) {
    const v = C[p]?.[theme];
    if (!v) throw new Error("unknown token " + p);
    acc = { rgb: over(parseColor(v), acc), a: 1 };
  }
  return acc.rgb;
}

// [fg, bg, role]. V1.1 rows are marked; the order is the document's.
export const pairs = [
  ["text.primary", "surface.panel", "text"], ["text.primary", "surface.base", "text"], ["text.primary", "surface.sunken", "text"],
  ["text.primary", "surface.header", "text"], ["text.primary", "surface.rowAlt", "text"], ["text.primary", "surface.raised", "text"],
  ["text.primary", "selection.band", "text"], ["text.primary", "proposal.band", "text"], ["text.primary", "hover.wash/surface.panel", "text"],
  ["text.primary", "issue.blockingTint", "text"], ["text.primary", "issue.warningTint", "text"], ["text.primary", "issue.infoTint", "text"],
  ["text.primary", "historical.tint", "text"], ["text.primary", "stale.band", "text"], ["text.primary", "disabled.fill", "text"],
  ["text.secondary", "surface.panel", "text"], ["text.secondary", "surface.base", "text"], ["text.secondary", "surface.header", "text"],
  ["text.secondary", "surface.sunken", "text"], ["text.secondary", "selection.band", "text"], ["text.secondary", "proposal.band", "text"],
  ["text.secondary", "canvas.hint/canvas.bg", "canvas hint text"],
  ["text.muted", "surface.panel", "text"], ["text.muted", "surface.base", "text"], ["text.muted", "surface.header", "text"], ["text.muted", "surface.sunken", "text"],
  ["text.disabled", "surface.panel", "text (disabled)"], ["text.disabled", "surface.base", "text (disabled)"], ["text.disabled", "surface.sunken", "text (disabled)"],
  ["text.link", "surface.panel", "text"], ["text.link", "surface.base", "text"],
  ["accent.text", "surface.panel", "text"], ["accent.text", "selection.band", "text"],
  ["text.inverse", "accent.fill", "text on button"], ["text.inverse", "accent.fillHover", "text on button"],
  ["pressed.ink", "pressed.fill", "latched toggle"], ["pressed.fill", "surface.base", "latched toggle fill"],
  ["proposal.new", "surface.panel", "text"], ["proposal.new", "proposal.band", "text"], ["proposal.new", "surface.base", "text"],
  ["proposal.old", "surface.panel", "text (struck)"], ["proposal.old", "proposal.band", "text (struck)"],
  ["mark.checked", "surface.panel", "mark"], ["mark.checkedStale", "surface.panel", "mark"], ["mark.checkedStale", "surface.base", "mark"],
  ["mark.origin", "surface.panel", "mark"], ["mark.origin", "surface.rowAlt", "mark"], ["mark.origin", "surface.header", "mark"],
  ["mark.attachment", "surface.panel", "mark"], ["mark.required", "surface.panel", "mark"], ["mark.required", "surface.sunken", "mark"],
  ["draft.bar", "surface.panel", "draft row bar"], ["draft.bar", "selection.band", "draft row bar"],
  ["issue.blocking", "surface.panel", "text"], ["issue.blocking", "issue.blockingTint", "text"], ["issue.blocking", "surface.base", "text"],
  ["issue.warning", "surface.panel", "text"], ["issue.warning", "issue.warningTint", "text"], ["issue.warning", "surface.base", "text"],
  ["issue.info", "surface.panel", "text"], ["issue.info", "issue.infoTint", "text"],
  ["status.incompleteInk", "status.incompleteFill", "chip text"], ["status.solvedInk", "status.solvedFill", "chip text"],
  ["status.failedInk", "status.failedFill", "chip text"], ["status.reviewInk", "status.reviewFill", "chip text"],
  ["status.incompleteInk", "surface.panel", "chip text"], ["status.failedInk", "surface.panel", "chip text"], ["status.reviewInk", "surface.panel", "chip text"],
  ["rail.captionFailed", "surface.base", "rail caption"], ["rail.captionStale", "surface.base", "rail caption"], ["rail.captionHistorical", "surface.base", "rail caption"],
  ["historical.ink", "historical.tint", "text"], ["historical.ink", "surface.panel", "text"],
  ["stale.ink", "stale.band", "band text"], ["stale.ink", "surface.panel", "band text"],
  ["border.hairline", "surface.panel", "hairline"], ["border.hairline", "surface.base", "hairline"],
  ["border.strong", "surface.panel", "border"], ["border.strong", "surface.base", "border"],
  ["focus.ring", "surface.panel", "focus ring"], ["focus.ring", "surface.base", "focus ring"], ["focus.ring", "selection.band", "focus ring"],
  ["selection.bar", "selection.band", "mark"], ["proposal.bar", "proposal.band", "mark"], ["displayOnly.border", "surface.sunken", "border"], ["stale.stripe", "surface.panel", "mark"],
  ["bar.track", "surface.panel", "bar track"], ["bar.track", "selection.band", "bar track"],
  ["canvas.label", "canvas.labelBg/canvas.bg", "canvas text"], ["canvas.label", "canvas.bg", "canvas text"],
  ["canvas.gridMajor", "canvas.bg", "hairline"], ["canvas.gridMinor", "canvas.bg", "hairline"],
  ["canvas.pipe", "canvas.bg", "canvas mark"], ["canvas.pipeShade", "canvas.bg", "canvas mark"], ["canvas.edge", "canvas.pipe", "canvas mark"], ["canvas.edge", "canvas.bg", "canvas mark"],
  ["canvas.glyph", "canvas.bg", "canvas mark"], ["canvas.glyph", "canvas.glyphFill", "canvas mark"], ["canvas.glyphFill", "canvas.bg", "canvas mark"],
  ["canvas.vector", "canvas.bg", "canvas mark"], ["canvas.selection", "canvas.bg", "canvas mark"], ["canvas.selection", "canvas.pipe", "canvas mark"],
  ["canvas.hover", "canvas.bg", "canvas mark"], ["canvas.draft", "canvas.bg", "canvas mark"], ["canvas.proposalGhost", "canvas.bg", "canvas mark"],
  ["canvas.deformGhost", "canvas.bg", "canvas mark"], ["canvas.unsolved", "canvas.bg", "canvas mark"],
  ["canvas.axisX", "canvas.bg", "canvas mark"], ["canvas.axisY", "canvas.bg", "canvas mark"], ["canvas.axisZ", "canvas.bg", "canvas mark"],
  ...[1,2,3,4,5,6,7].map(i => [`result.scale.${i}`, "canvas.bg", "result scale on canvas"]),
  ["result.scale.1", "canvas.unsolved", "result scale beside unsolved"], ["result.scale.7", "canvas.unsolved", "result scale beside unsolved"],
  ["canvas.edge", "result.scale.1", "edge line on the near-zero step"], ["canvas.edge", "result.scale.7", "edge line on the brightest step"],
  ...[1,2,3,4,5,6,7].map(i => [`result.scale.${i}`, "surface.raised", "legend swatch"]),
  ["result.scale.1", "bar.track", "data bar on its track"], ["result.scale.7", "bar.track", "data bar on its track"],
  ...[1,2,3,4,5,6,7,8].map(i => [`cat.${i}`, "canvas.bg", "load vector on canvas"]),
  ...[1,2,3,4,5,6,7,8].map(i => [`cat.${i}`, "surface.panel", "case dot in table"]),
];

export function computeRows(T) {
  const C = T.color;
  return pairs.map(([fg, bg, role]) => ({ fg, bg, role, l: ratio(resolve(C, fg, "light"), resolve(C, bg, "light")), d: ratio(resolve(C, fg, "dark"), resolve(C, bg, "dark")) }));
}
export function toMarkdown(rows) {
  return ["| Foreground | Background | Role | Light | Dark |", "|---|---|---|---|---|",
    ...rows.map(r => `| \`${r.fg}\` | \`${r.bg}\` | ${r.role} | ${r.l.toFixed(2)}:1 | ${r.d.toFixed(2)}:1 |`)].join("\n");
}

if (process.argv[1] && /contrast\.mjs$/.test(process.argv[1])) {
  const T = JSON.parse(fs.readFileSync(process.argv[2], "utf8"));
  const rows = computeRows(T);
  fs.writeFileSync(process.argv[3], toMarkdown(rows) + "\n");
  const lowest = [...rows].sort((a, b) => Math.min(a.l, a.d) - Math.min(b.l, b.d)).slice(0, 16);
  console.log("pairs:", rows.length);
  console.log("lowest (min of the two themes):");
  for (const r of lowest) console.log(`  ${r.fg} on ${r.bg} [${r.role}]  light ${r.l.toFixed(2)}  dark ${r.d.toFixed(2)}`);
  const textRows = rows.filter(r => r.role.startsWith("text") || r.role.includes("chip") || r.role.includes("canvas text") || r.role.includes("button") || r.role.includes("caption") || r.role.includes("band text") || r.role.includes("toggle") && !r.role.includes("fill"));
  console.log("text pairings:", textRows.length, "| lowest text light:", Math.min(...textRows.map(r => r.l)).toFixed(2), "dark:", Math.min(...textRows.map(r => r.d)).toFixed(2));
}
