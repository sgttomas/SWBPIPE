// Palette generator for the SWB Piping Designer design system.
// Specifies tokens in OKLCH, converts to sRGB hex, validates the categorical set
// with the dataviz validator, and prints the result ramp lightness checks.
import { validate, validateOrdinal, contrast } from "/private/tmp/claude-501/bundled-skills/2.1.275/5064f2b3898452ce4f4625072b878076/dataviz/scripts/validate_palette.js";

// ---- OKLCH -> sRGB ---------------------------------------------------------
function oklabToLin(L, a, b) {
  const l_ = L + 0.3963377774 * a + 0.2158037573 * b;
  const m_ = L - 0.1055613458 * a - 0.0638541728 * b;
  const s_ = L - 0.0894841775 * a - 1.2914855480 * b;
  const l = l_ ** 3, m = m_ ** 3, s = s_ ** 3;
  return [
    +4.0767416621 * l - 3.3077115913 * m + 0.2309699292 * s,
    -1.2684380046 * l + 2.6097574011 * m - 0.3413193965 * s,
    -0.0041960863 * l - 0.7034186147 * m + 1.7076147010 * s,
  ];
}
const lin2s = (c) => c <= 0.0031308 ? 12.92 * c : 1.055 * c ** (1 / 2.4) - 0.055;
export function oklch(L, C, h) {
  const rad = h * Math.PI / 180;
  let a = C * Math.cos(rad), b = C * Math.sin(rad);
  let rgb = oklabToLin(L, a, b);
  // gamut: reduce chroma until inside sRGB
  let c = C;
  while (rgb.some(v => v < -0.0005 || v > 1.0005) && c > 0) {
    c -= 0.002;
    a = c * Math.cos(rad); b = c * Math.sin(rad);
    rgb = oklabToLin(L, a, b);
  }
  const hex = "#" + rgb.map(v => Math.round(Math.max(0, Math.min(1, lin2s(v))) * 255).toString(16).padStart(2, "0")).join("");
  return { hex, clipped: c < C - 1e-9, c };
}
export const H = (L, C, h) => oklch(L, C, h).hex;

// ---- semantic tokens (light, dark) -----------------------------------------
const N = 250; // neutral hue (slightly cool)
export const tokens = {
  // surfaces
  "surface.base":     [H(0.955, 0.004, N), H(0.235, 0.008, N)],
  "surface.panel":    ["#ffffff",          H(0.275, 0.008, N)],
  "surface.raised":   ["#ffffff",          H(0.315, 0.009, N)],
  "surface.sunken":   [H(0.975, 0.003, N), H(0.255, 0.008, N)],
  "surface.header":   [H(0.962, 0.004, N), H(0.295, 0.009, N)],
  "surface.rowAlt":   [H(0.985, 0.002, N), H(0.285, 0.008, N)],
  "surface.canvas":   [H(0.945, 0.004, N), H(0.225, 0.008, N)],
  // text
  "text.primary":     [H(0.27, 0.012, N),  H(0.93, 0.006, N)],
  "text.secondary":   [H(0.49, 0.014, N),  H(0.76, 0.010, N)],
  "text.muted":       [H(0.61, 0.014, N),  H(0.64, 0.012, N)],
  "text.disabled":    [H(0.74, 0.010, N),  H(0.50, 0.010, N)],
  "text.inverse":     ["#ffffff",          H(0.20, 0.010, N)],
  "text.link":        [H(0.50, 0.15, 255), H(0.78, 0.11, 255)],
  // borders
  "border.hairline":  [H(0.88, 0.006, N),  H(0.36, 0.010, N)],
  "border.strong":    [H(0.78, 0.010, N),  H(0.46, 0.012, N)],
  "border.focus":     [H(0.56, 0.17, 255), H(0.74, 0.13, 255)],
  // accent, selection, hover, disabled
  "accent.fill":      [H(0.54, 0.17, 255), H(0.66, 0.15, 255)],
  "accent.fillHover": [H(0.50, 0.17, 255), H(0.70, 0.15, 255)],
  "accent.text":      [H(0.48, 0.16, 255), H(0.80, 0.11, 255)],
  "selection.band":   [H(0.945, 0.035, 255), H(0.36, 0.070, 255)],
  "selection.bar":    [H(0.54, 0.17, 255), H(0.72, 0.14, 255)],
  "hover.wash":       ["rgba(30,36,48,0.06)", "rgba(255,255,255,0.06)"],
  "pressed.wash":     ["rgba(30,36,48,0.10)", "rgba(255,255,255,0.10)"],
  "disabled.fill":    [H(0.94, 0.004, N),  H(0.30, 0.008, N)],
  "focus.ring":       [H(0.56, 0.17, 255), H(0.74, 0.13, 255)],
  "scrim":            ["rgba(20,22,28,0.32)", "rgba(0,0,0,0.55)"],
  // proposal (loud)
  "proposal.new":     [H(0.47, 0.17, 300), H(0.78, 0.13, 300)],
  "proposal.band":    [H(0.955, 0.030, 300), H(0.335, 0.060, 300)],
  "proposal.bar":     [H(0.55, 0.19, 300), H(0.74, 0.15, 300)],
  "proposal.old":     [H(0.61, 0.014, N),  H(0.64, 0.012, N)],
  // checked
  "mark.checked":     [H(0.49, 0.014, N),  H(0.76, 0.010, N)],
  "mark.checkedStale":[H(0.58, 0.14, 75),  H(0.80, 0.13, 80)],
  // origins and attachment marks (quiet)
  "mark.origin":      [H(0.61, 0.014, N),  H(0.64, 0.012, N)],
  "mark.attachment":  [H(0.49, 0.014, N),  H(0.76, 0.010, N)],
  // issues by severity
  "issue.blocking":   [H(0.52, 0.18, 25),  H(0.74, 0.15, 25)],
  "issue.blockingTint":[H(0.955, 0.025, 25), H(0.32, 0.055, 25)],
  "issue.warning":    [H(0.58, 0.14, 75),  H(0.80, 0.13, 80)],
  "issue.warningTint":[H(0.962, 0.040, 85), H(0.33, 0.050, 80)],
  "issue.info":       [H(0.50, 0.06, 240), H(0.76, 0.06, 240)],
  "issue.infoTint":   [H(0.955, 0.012, 240), H(0.32, 0.025, 240)],
  "mark.required":    [H(0.52, 0.18, 25),  H(0.74, 0.15, 25)],
  // statuses (M-08) chip fills and inks
  "status.incompleteFill": [H(0.962, 0.040, 85), H(0.33, 0.050, 80)],
  "status.incompleteInk":  [H(0.46, 0.12, 75),  H(0.85, 0.11, 85)],
  "status.solvedFill":     [H(0.955, 0.004, N), H(0.33, 0.010, N)],
  "status.solvedInk":      [H(0.35, 0.014, N),  H(0.88, 0.008, N)],
  "status.failedFill":     [H(0.955, 0.025, 25), H(0.32, 0.055, 25)],
  "status.failedInk":      [H(0.46, 0.17, 25),  H(0.82, 0.12, 25)],
  "status.reviewFill":     [H(0.955, 0.012, 240), H(0.32, 0.025, 240)],
  "status.reviewInk":      [H(0.42, 0.07, 240), H(0.84, 0.06, 240)],
  // historical, display-only, stale
  "historical.tint":  [H(0.955, 0.012, 80), H(0.30, 0.015, 80)],
  "historical.ink":   [H(0.45, 0.03, 80),   H(0.82, 0.03, 80)],
  "displayOnly.border":[H(0.78, 0.010, N),  H(0.46, 0.012, N)],
  "stale.stripe":     [H(0.86, 0.006, N),   H(0.40, 0.010, N)],
  // canvas
  "canvas.bg":        [H(0.945, 0.004, N),  H(0.225, 0.008, N)],
  "canvas.gridMajor": [H(0.86, 0.006, N),   H(0.31, 0.010, N)],
  "canvas.gridMinor": [H(0.91, 0.004, N),   H(0.265, 0.008, N)],
  "canvas.pipe":      [H(0.74, 0.010, N),   H(0.58, 0.012, N)],
  "canvas.pipeShade": [H(0.62, 0.012, N),   H(0.46, 0.012, N)],
  "canvas.edge":      [H(0.40, 0.014, N),   H(0.84, 0.008, N)],
  "canvas.glyph":     [H(0.33, 0.014, N),   H(0.90, 0.006, N)],
  "canvas.glyphFill": [H(0.90, 0.006, N),   H(0.34, 0.010, N)],
  "canvas.label":     [H(0.27, 0.012, N),   H(0.93, 0.006, N)],
  "canvas.labelBg":   ["rgba(255,255,255,0.82)", "rgba(28,31,37,0.82)"],
  "canvas.vector":    [H(0.40, 0.014, N),   H(0.84, 0.008, N)],
  "canvas.selection": [H(0.54, 0.17, 255),  H(0.74, 0.14, 255)],
  "canvas.hover":     [H(0.66, 0.12, 255),  H(0.62, 0.11, 255)],
  "canvas.draft":     [H(0.54, 0.17, 255),  H(0.74, 0.14, 255)],
  "canvas.proposalGhost": [H(0.55, 0.19, 300), H(0.74, 0.15, 300)],
  "canvas.deformGhost":   [H(0.62, 0.012, N), H(0.46, 0.012, N)],
  "canvas.unsolved":  [H(0.74, 0.010, N),   H(0.58, 0.012, N)],
  "canvas.axisX":     [H(0.55, 0.16, 25),   H(0.72, 0.14, 25)],
  "canvas.axisY":     [H(0.55, 0.14, 145),  H(0.74, 0.13, 145)],
  "canvas.axisZ":     [H(0.55, 0.16, 255),  H(0.74, 0.13, 255)],
};

// ---- result scale (sequential, teal) ---------------------------------------
const RH = 190;
const lightRamp = [[0.72,0.10],[0.655,0.11],[0.59,0.115],[0.525,0.115],[0.46,0.11],[0.395,0.10],[0.33,0.085]];
const darkRamp  = [[0.50,0.09],[0.57,0.10],[0.64,0.11],[0.71,0.115],[0.78,0.11],[0.85,0.09],[0.92,0.06]];
export const resultLight = lightRamp.map(([L,C]) => H(L,C,RH));
export const resultDark  = darkRamp.map(([L,C]) => H(L,C,RH));

// ---- categorical (load kinds / cases) --------------------------------------
// slot spec: name, hue, light [L,C], dark [L,C]
const catSpec = [
  ["orange",  50,  [0.62,0.16], [0.66,0.15]],
  ["green",   150, [0.60,0.13], [0.62,0.13]],
  ["magenta", 340, [0.58,0.17], [0.64,0.16]],
  ["azure",   225, [0.60,0.13], [0.64,0.12]],
  ["olive",   105, [0.60,0.13], [0.64,0.12]],
  ["purple",  268, [0.50,0.15], [0.62,0.14]],
  ["rose",    5,   [0.60,0.16], [0.66,0.15]],
  ["cyan",    200, [0.61,0.102], [0.60,0.11]],
];
export const catLight = catSpec.map(([,h,[L,C]]) => H(L,C,h));
export const catDark  = catSpec.map(([,h,,[L,C]]) => H(L,C,h));
export const catNames = catSpec.map(s => s[0]);

if (process.argv[1] && process.argv[1].endsWith("palette.mjs")) {
  const clipped = [];
  for (const [k,[l,d]] of Object.entries(tokens)) {
    // nothing to do; gamut reduction happens in H(); report is below
  }
  console.log("== semantic tokens ==");
  for (const [k,[l,d]] of Object.entries(tokens)) console.log(k.padEnd(24), l.padEnd(22), d);
  console.log("\n== result ramp light ==", resultLight.join(","));
  console.log("== result ramp dark  ==", resultDark.join(","));
  const canvasL = tokens["canvas.bg"][0], canvasD = tokens["canvas.bg"][1];
  console.log("\nordinal check light:", JSON.stringify(validateOrdinal(resultLight, {mode:"light", surface: canvasL})));
  console.log("ordinal check dark :", JSON.stringify(validateOrdinal([...resultDark].reverse(), {mode:"dark", surface: canvasD})));
  console.log("\n== categorical (spec order) ==", catNames.join(","));
  console.log("light:", catLight.join(","));
  console.log("dark :", catDark.join(","));
  console.log(JSON.stringify(validate(catLight, {mode:"light", surface: canvasL}), null, 1));
  console.log(JSON.stringify(validate(catDark, {mode:"dark", surface: canvasD}), null, 1));

  // enumerate orderings to maximise the min adjacent CVD dE across both modes
  const perms = (arr) => arr.length <= 1 ? [arr] : arr.flatMap((x,i) => perms([...arr.slice(0,i), ...arr.slice(i+1)]).map(p => [x, ...p]));
  const idx = perms([0,1,2,3,4,5,6,7]);
  const parse = (r) => { const m = r.report.find(x => x[0]==="CVD separation")[2].match(/ΔE ([\d.]+)/); const n = r.report.find(x => x[0]==="Normal-vision floor")[2].match(/ΔE ([\d.]+)/); return [parseFloat(m[1]), parseFloat(n[1])]; };
  const cands = [];
  for (const p of idx) {
    if (p[0] !== 0) continue;
    const L = p.map(i => catLight[i]), D = p.map(i => catDark[i]);
    const rl = validate(L, {mode:"light", surface: canvasL}), rd = validate(D, {mode:"dark", surface: canvasD});
    const [cl, nl] = parse(rl), [cd, nd] = parse(rd);
    const score = Math.min(cl, cd), nscore = Math.min(nl, nd);
    if (nscore < 15) continue;
    cands.push({ p, score, nscore, cl, cd, nl, nd });
  }
  cands.sort((a,b) => b.score - a.score || b.nscore - a.nscore);
  console.log("\n== top orderings (orange first, normal floor >= 15) ==");
  for (const c of cands.slice(0, 12)) console.log(c.p.map(i => catNames[i]).join(","), "cvd", c.score, "(l", c.cl, "d", c.cd, ") normal", c.nscore, "(l", c.nl, "d", c.nd, ")");
  const best = cands[0];
  console.log("\n== best ordering (orange fixed first) ==", best && best.p.map(i => catNames[i]).join(","), best);
}

export function buildTokens(order) {
  const color = {};
  for (const [k,[l,d]] of Object.entries(tokens)) color[k] = { light: l, dark: d };
  resultLight.forEach((h,i) => { color[`result.scale.${i+1}`] = { light: h, dark: resultDark[i] }; });
  order.forEach((slot,i) => { color[`cat.${i+1}`] = { light: catLight[slot], dark: catDark[slot], hue: catNames[slot] }; });
  return color;
}
if (process.argv.includes("--write")) {
  const order = process.argv[process.argv.indexOf("--write")+1].split(",").map(n => catNames.indexOf(n));
  const color = buildTokens(order);
  const out = {
    name: "SWB Piping Designer design system tokens",
    version: "1.0",
    date: "2026-09-18",
    note: "Sample values in the specimen are placeholders. Colour tokens carry a light and a dark value; type, spacing and layout tokens are plain values in CSS px unless stated.",
    color,
    type: {
      "family.ui": "-apple-system, BlinkMacSystemFont, system-ui, \"SF Pro Text\", \"Helvetica Neue\", Helvetica, Arial, sans-serif",
      "family.numeric": "-apple-system, BlinkMacSystemFont, system-ui, \"SF Pro Text\", \"Helvetica Neue\", Helvetica, Arial, sans-serif",
      "family.mono": "ui-monospace, \"SF Mono\", Menlo, Monaco, Consolas, monospace",
      "numeric.feature": "font-variant-numeric: tabular-nums",
      "size.caption": 11, "size.chip": 12, "size.body": 13, "size.title": 15, "size.heading": 18, "size.figure": 22,
      "lineHeight.caption": 14, "lineHeight.chip": 16, "lineHeight.body": 18, "lineHeight.title": 20, "lineHeight.heading": 24, "lineHeight.figure": 28,
      "weight.regular": 400, "weight.medium": 500, "weight.semibold": 600,
      "minimumRepeatedlyRead": 13,
      "letterSpacing.figure": -0.2
    },
    space: { "0": 0, "1": 2, "2": 4, "3": 6, "4": 8, "5": 12, "6": 16, "7": 20, "8": 24, "9": 32, "10": 40 },
    radius: { "none": 0, "control": 3, "card": 6, "sheet": 10, "pill": 999 },
    border: { "hairline": 1, "strong": 1, "focusRing": 2, "selectionBar": 3, "proposalBar": 3 },
    elevation: {
      "0": "none",
      "1": { light: "0 1px 2px rgba(20,24,30,0.08)", dark: "0 1px 2px rgba(0,0,0,0.40)" },
      "2": { light: "0 4px 12px rgba(20,24,30,0.12), 0 1px 2px rgba(20,24,30,0.06)", dark: "0 4px 12px rgba(0,0,0,0.50), 0 0 0 1px rgba(255,255,255,0.06)" },
      "3": { light: "0 12px 32px rgba(20,24,30,0.18), 0 2px 4px rgba(20,24,30,0.08)", dark: "0 12px 32px rgba(0,0,0,0.60), 0 0 0 1px rgba(255,255,255,0.08)" }
    },
    focus: { "ringWidth": 2, "ringOffset": 1, "ringColorToken": "focus.ring", "cellInset": true },
    motion: {
      "disclosure.ms": 120, "drawer.ms": 180, "proposalArrive.ms": 240, "hover.ms": 80,
      "easing.standard": "cubic-bezier(0.2, 0, 0, 1)", "easing.exit": "cubic-bezier(0.4, 0, 1, 1)",
      "reducedMotion": "all durations 0 under prefers-reduced-motion"
    },
    layout: {
      "window.default": [1440, 900], "window.min": [1280, 800],
      "toolbar": 48, "statusBar": 24, "rail": 56, "agent.open": 340, "agent.strip": 44,
      "inspector.both": 300, "inspector.model": 340, "drawer.table": 280, "drawer.issues": 200,
      "review.outline": 280, "review.comments": 320,
      "row": 26, "headerRow": 28, "control": 26, "controlCompact": 22, "gutter": 32, "marksColumn": 72,
      "hud.button": 28, "probe.width": 300, "legend.width": 220, "sideGutter": 16
    },
    icon: { "grid": 16, "stroke": 1.5, "cornerRadius": 1.5, "sizes": [12, 16, 20], "capsAndJoins": "round" }
  };
  const fs = await import("node:fs");
  const dest = process.argv[process.argv.indexOf("--write")+2];
  fs.writeFileSync(dest, JSON.stringify(out, null, 2) + "\n");
  console.log("wrote", dest, Object.keys(color).length, "colour tokens");
}
