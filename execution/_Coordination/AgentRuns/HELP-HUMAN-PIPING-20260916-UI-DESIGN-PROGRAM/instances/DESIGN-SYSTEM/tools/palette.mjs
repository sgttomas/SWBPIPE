// Palette generator for the SWBPIPE design system, tokens V1.2.
// Specifies every colour token in OKLCH, converts to sRGB hex, checks the result scale as an
// ordinal ramp and the categorical set with the dataviz validator when one is supplied, and
// writes tokens.json. No absolute path is stored here: the validator is located through
// `--validator <path>` or the DATAVIZ_VALIDATOR environment variable; without it the ordinal
// checks run with the built-in twin (same thresholds) and the categorical CVD checks are
// reported as not run.
//
//   node palette.mjs [--validator <validate_palette.js>]          print tokens and checks
//   node palette.mjs --write <dest tokens.json> [--validator …]   also write the token file
//   node palette.mjs --search                                    re-run the categorical ordering search (V1)
import { pathToFileURL } from "node:url";

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
const s2lin = (c) => c <= 0.04045 ? c / 12.92 : ((c + 0.055) / 1.055) ** 2.4;
export function oklch(L, C, h) {
  const rad = h * Math.PI / 180;
  let a = C * Math.cos(rad), b = C * Math.sin(rad);
  let rgb = oklabToLin(L, a, b);
  let c = C;
  while (rgb.some(v => v < -0.0005 || v > 1.0005) && c > 0) { // gamut: reduce chroma until inside sRGB
    c -= 0.002;
    a = c * Math.cos(rad); b = c * Math.sin(rad);
    rgb = oklabToLin(L, a, b);
  }
  const hex = "#" + rgb.map(v => Math.round(Math.max(0, Math.min(1, lin2s(v))) * 255).toString(16).padStart(2, "0")).join("");
  return { hex, clipped: c < C - 1e-9, c };
}
export const H = (L, C, h) => oklch(L, C, h).hex;
// ---- sRGB hex -> OKLCH (for the checks) -------------------------------------
const hex2rgb = (h) => [0, 2, 4].map(i => parseInt(h.slice(1 + i, 3 + i), 16));
export function okLCH(hex) {
  const [r, g, b] = hex2rgb(hex).map(v => s2lin(v / 255));
  const l = Math.cbrt(0.4122214708 * r + 0.5363325363 * g + 0.0514459929 * b), m = Math.cbrt(0.2119034982 * r + 0.6806995451 * g + 0.1073969566 * b), s = Math.cbrt(0.0883024619 * r + 0.2817188376 * g + 0.6299787005 * b);
  const L = 0.2104542553 * l + 0.7936177850 * m - 0.0040720468 * s, A = 1.9779984951 * l - 2.4285922050 * m + 0.4505937099 * s, B = 0.0259040371 * l + 0.7827717662 * m - 0.8086757660 * s;
  return [L, Math.hypot(A, B), ((Math.atan2(B, A) * 180 / Math.PI) + 360) % 360];
}
const lum = (hex) => { const [r, g, b] = hex2rgb(hex).map(v => s2lin(v / 255)); return 0.2126 * r + 0.7152 * g + 0.0722 * b; };
export const wcag = (a, b) => { const [hi, lo] = [lum(a), lum(b)].sort((x, y) => y - x); return (hi + 0.05) / (lo + 0.05); };

// ---- semantic tokens (light, dark), in the order tokens.json carries them ----
const N = 250; // neutral hue (slightly cool)
const T = [];
const put = (k, l, d) => { T.push([k, [l, d]]); };
const val = (k) => T.find(e => e[0] === k)[1];
// surfaces
put("surface.base",     H(0.955, 0.004, N), H(0.235, 0.008, N));
put("surface.panel",    "#ffffff",          H(0.275, 0.008, N));
put("surface.raised",   "#ffffff",          H(0.315, 0.009, N));
put("surface.sunken",   H(0.975, 0.003, N), H(0.255, 0.008, N));
put("surface.header",   H(0.962, 0.004, N), H(0.295, 0.009, N));
put("surface.rowAlt",   H(0.985, 0.002, N), H(0.285, 0.008, N));
put("surface.canvas",   H(0.945, 0.004, N), H(0.225, 0.008, N));
// text
put("text.primary",     H(0.27, 0.012, N),  H(0.93, 0.006, N));
put("text.secondary",   H(0.49, 0.014, N),  H(0.76, 0.010, N));
put("text.muted",       H(0.61, 0.014, N),  H(0.64, 0.012, N));
put("text.disabled",    H(0.74, 0.010, N),  H(0.50, 0.010, N));
put("text.inverse",     "#ffffff",          H(0.20, 0.010, N));
put("text.link",        H(0.50, 0.15, 255), H(0.78, 0.11, 255));
// borders
put("border.hairline",  H(0.88, 0.006, N),  H(0.36, 0.010, N));
put("border.strong",    H(0.78, 0.010, N),  H(0.46, 0.012, N));
put("border.focus",     H(0.56, 0.17, 255), H(0.74, 0.13, 255));
// accent, selection, hover, pressed (momentary wash and the latched toggle, V1.1 G-5), disabled
put("accent.fill",      H(0.54, 0.17, 255), H(0.66, 0.15, 255));
put("accent.fillHover", H(0.50, 0.17, 255), H(0.70, 0.15, 255));
put("accent.text",      H(0.48, 0.16, 255), H(0.80, 0.11, 255));
put("selection.band",   H(0.945, 0.035, 255), H(0.36, 0.070, 255));
put("selection.bar",    H(0.54, 0.17, 255), H(0.72, 0.14, 255));
put("hover.wash",       "rgba(30,36,48,0.06)", "rgba(255,255,255,0.06)");
put("pressed.wash",     "rgba(30,36,48,0.10)", "rgba(255,255,255,0.10)");
put("pressed.fill",     H(0.92, 0.045, 255), H(0.40, 0.085, 255));
put("pressed.ink",      H(0.45, 0.16, 255),  H(0.84, 0.10, 255));
put("disabled.fill",    H(0.94, 0.004, N),  H(0.30, 0.008, N));
put("focus.ring",       H(0.56, 0.17, 255), H(0.74, 0.13, 255));
put("scrim",            "rgba(20,22,28,0.32)", "rgba(0,0,0,0.55)");
// proposal (loud)
put("proposal.new",     H(0.47, 0.17, 300), H(0.78, 0.13, 300));
put("proposal.band",    H(0.955, 0.030, 300), H(0.335, 0.060, 300));
put("proposal.bar",     H(0.55, 0.19, 300), H(0.74, 0.15, 300));
put("proposal.old",     H(0.61, 0.014, N),  H(0.64, 0.012, N));
// checked
put("mark.checked",     H(0.49, 0.014, N),  H(0.76, 0.010, N));
put("mark.checkedStale",H(0.58, 0.14, 75),  H(0.80, 0.13, 80));
// origins and attachment marks (quiet)
put("mark.origin",      H(0.61, 0.014, N),  H(0.64, 0.012, N));
put("mark.attachment",  H(0.49, 0.014, N),  H(0.76, 0.010, N));
// issues by severity
put("issue.blocking",   H(0.52, 0.18, 25),  H(0.74, 0.15, 25));
put("issue.blockingTint",H(0.955, 0.025, 25), H(0.32, 0.055, 25));
put("issue.warning",    H(0.58, 0.14, 75),  H(0.80, 0.13, 80));
put("issue.warningTint",H(0.962, 0.040, 85), H(0.33, 0.050, 80));
put("issue.info",       H(0.50, 0.06, 240), H(0.76, 0.06, 240));
put("issue.infoTint",   H(0.955, 0.012, 240), H(0.32, 0.025, 240));
put("mark.required",    H(0.52, 0.18, 25),  H(0.74, 0.15, 25));
// statuses (M-08) chip fills and inks
put("status.incompleteFill", H(0.962, 0.040, 85), H(0.33, 0.050, 80));
put("status.incompleteInk",  H(0.46, 0.12, 75),  H(0.85, 0.11, 85));
put("status.solvedFill",     H(0.955, 0.004, N), H(0.33, 0.010, N));
put("status.solvedInk",      H(0.35, 0.014, N),  H(0.88, 0.008, N));
put("status.failedFill",     H(0.955, 0.025, 25), H(0.32, 0.055, 25));
put("status.failedInk",      H(0.46, 0.17, 25),  H(0.82, 0.12, 25));
put("status.reviewFill",     H(0.955, 0.012, 240), H(0.32, 0.025, 240));
put("status.reviewInk",      H(0.42, 0.07, 240), H(0.84, 0.06, 240));
// historical, display-only, stale (V1.1 G-1: the band after a model change and its ink)
put("historical.tint",  H(0.955, 0.012, 80), H(0.30, 0.015, 80));
put("historical.ink",   H(0.45, 0.03, 80),   H(0.82, 0.03, 80));
put("displayOnly.border",H(0.78, 0.010, N),  H(0.46, 0.012, N));
put("stale.stripe",     H(0.86, 0.006, N),   H(0.40, 0.010, N));
put("stale.band",       H(0.975, 0.022, 82), H(0.30, 0.030, 80));
put("stale.ink",        H(0.47, 0.10, 72),   H(0.85, 0.09, 85));
// rail captions by state (V1.1 G-2): aliases by design, kept as their own tokens so a caption can be retuned
put("rail.captionFailed",     ...val("status.failedInk"));
put("rail.captionStale",      ...val("stale.ink"));
put("rail.captionHistorical", ...val("historical.ink"));
// canvas
put("canvas.bg",        H(0.945, 0.004, N),  H(0.225, 0.008, N));
put("canvas.gridMajor", H(0.86, 0.006, N),   H(0.31, 0.010, N));
put("canvas.gridMinor", H(0.91, 0.004, N),   H(0.265, 0.008, N));
put("canvas.pipe",      H(0.74, 0.010, N),   H(0.58, 0.012, N));
put("canvas.pipeShade", H(0.62, 0.012, N),   H(0.46, 0.012, N));
put("canvas.edge",      H(0.40, 0.014, N),   H(0.84, 0.008, N));
// V1.2 (R-6): the alternate edge, the opposite polarity of canvas.edge in each theme. On a result-coloured
// element the line takes whichever of the two reads better against the element's fill (section 6.7).
put("canvas.edgeAlt",   H(0.93, 0.006, N),   H(0.30, 0.010, N));
put("canvas.glyph",     H(0.33, 0.014, N),   H(0.90, 0.006, N));
put("canvas.glyphFill", H(0.90, 0.006, N),   H(0.34, 0.010, N));
put("canvas.label",     H(0.27, 0.012, N),   H(0.93, 0.006, N));
put("canvas.labelBg",   "rgba(255,255,255,0.82)", "rgba(28,31,37,0.82)");
put("canvas.hint",      "rgba(255,255,255,0.90)", "rgba(46,50,54,0.92)"); // V1.1 G-4: the hint strip and key hints, not a node plate
put("canvas.vector",    H(0.40, 0.014, N),   H(0.84, 0.008, N));
put("canvas.selection", H(0.54, 0.17, 255),  H(0.74, 0.14, 255));
put("canvas.hover",     H(0.66, 0.12, 255),  H(0.62, 0.11, 255));
put("canvas.draft",     H(0.54, 0.17, 255),  H(0.74, 0.14, 255));
put("canvas.proposalGhost", H(0.55, 0.19, 300), H(0.74, 0.15, 300));
put("canvas.deformGhost",   H(0.62, 0.012, N), H(0.46, 0.012, N));
put("canvas.unsolved",  H(0.74, 0.010, N),   H(0.58, 0.012, N));
put("canvas.axisX",     H(0.55, 0.16, 25),   H(0.72, 0.14, 25));
put("canvas.axisY",     H(0.55, 0.14, 145),  H(0.74, 0.13, 145));
put("canvas.axisZ",     H(0.55, 0.16, 255),  H(0.74, 0.13, 255));
// the draft row's bar in tables (V1.1 G-3): equals canvas.draft by design, the engineer's draft in both places
put("draft.bar",        ...val("canvas.draft"));

// ---- result scale (sequential, teal) ---------------------------------------
// Light: near-zero is the lightest step. Dark (V1.1, decision 11): re-anchored one step down so the
// lightest step (L 0.80) sits below the edge line (canvas.edge dark, L 0.84); steps of 0.062 in L so
// that the 8-bit rounding never brings an adjacent gap under the validator's 0.06.
const RH = 190;
const lightRamp = [[0.72,0.10],[0.655,0.11],[0.59,0.115],[0.525,0.115],[0.46,0.11],[0.395,0.10],[0.33,0.085]];
const darkRamp  = [0, 1, 2, 3, 4, 5, 6].map(i => [0.43 + 0.062 * i, [0.085, 0.10, 0.11, 0.115, 0.11, 0.10, 0.09][i]]);
export const resultLight = lightRamp.map(([L,C]) => H(L,C,RH));
export const resultDark  = darkRamp.map(([L,C]) => H(L,C,RH));
// the ratio data bar's track (V1.1 G-6): a recessive neutral under the scale colour
const barTrack = [H(0.915, 0.005, N), H(0.37, 0.010, N)];

// ---- categorical (load kinds / cases) --------------------------------------
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
export const ORDER = ["orange", "magenta", "green", "purple", "rose", "azure", "olive", "cyan"]; // the V1 search result, fixed

export function buildTokens(order = ORDER) {
  const color = {};
  for (const [k,[l,d]] of T) color[k] = { light: l, dark: d };
  resultLight.forEach((h,i) => { color[`result.scale.${i+1}`] = { light: h, dark: resultDark[i] }; });
  color["bar.track"] = { light: barTrack[0], dark: barTrack[1] };
  order.map(n => catNames.indexOf(n)).forEach((slot,i) => { color[`cat.${i+1}`] = { light: catLight[slot], dark: catDark[slot], hue: catNames[slot] }; });
  return color;
}

export function tokenFile() {
  return {
    name: "SWBPIPE design system tokens",
    version: "1.2",
    date: "2026-09-18",
    note: "Sample values in the specimen are placeholders. Colour tokens carry a light and a dark value; type, spacing and layout tokens are plain values in CSS px unless stated. V1.1: the dark result scale is re-anchored one step down (decision 11); stale.band, stale.ink, rail.caption*, draft.bar, canvas.hint, pressed.fill, pressed.ink and bar.track close the six token gaps of MOCKS_V1 section 5; layout gains canvas.min, inspector.label, headerPadding and cellPadding. V1.2: canvas.edgeAlt is the alternate edge line for result-coloured fills (R-6); layout gains the narrow-canvas HUD, toast and run log sizes (G-8, G-10, Q-16) and motion the toast durations; labels is the one table of status and evidence labels (ruling 4) and agentCardClasses the five class words of agent cards (ruling 8).",
    color: buildTokens(),
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
      "disclosure.ms": 120, "drawer.ms": 180, "proposalArrive.ms": 240, "hover.ms": 80, "toast.ms": 6000, "toastAction.ms": 10000,
      "easing.standard": "cubic-bezier(0.2, 0, 0, 1)", "easing.exit": "cubic-bezier(0.4, 0, 1, 1)",
      "reducedMotion": "all durations 0 under prefers-reduced-motion"
    },
    layout: {
      "window.default": [1440, 900], "window.min": [1280, 800],
      "toolbar": 48, "statusBar": 24, "rail": 56, "agent.open": 340, "agent.strip": 44,
      "inspector.both": 300, "inspector.model": 340, "inspector.label": 88, "canvas.min": 220,
      "drawer.table": 280, "drawer.issues": 200,
      "review.outline": 280, "review.comments": 320,
      "row": 26, "headerRow": 28, "control": 26, "controlCompact": 22, "gutter": 32, "marksColumn": 72,
      "cellPadding": 8, "headerPadding": 5,
      "hud.button": 28, "hud.inset": 8, "hud.width": 304, "hud.widthWrapped": 154, "hud.wrapBelow": 400,
      "toast.width": 320, "toast.inset": 8, "runlog.width": 360,
      "probe.width": 300, "legend.width": 220, "sideGutter": 16
    },
    // Ruling 4: the one table of status and evidence labels. A label is shown with its authority domain, its raw
    // token is reachable in place, and no label exists outside this table. "chip" names the status.* token pair.
    labels: {
      "MODEL_INCOMPLETE":       { label: "Model incomplete",       domain: "Solver",    chip: "incomplete", kind: "status" },
      "MECHANICS_SOLVED":       { label: "Mechanics solved",       domain: "Solver",    chip: "solved",     kind: "status" },
      "RULE_INPUTS_INCOMPLETE": { label: "Rule inputs incomplete", domain: "Rule pack", chip: "incomplete", kind: "status" },
      "USER_RULE_CHECKED":      { label: "User-rule checked",      domain: "Rule pack", chip: "solved",     kind: "status" },
      "USER_RULE_FAILED":       { label: "User-rule failed",       domain: "Rule pack", chip: "failed",     kind: "status" },
      "HUMAN_REVIEW_REQUIRED":  { label: "Human review required",  domain: "Human",     chip: "review",     kind: "status" },
      "INTERNALLY_VERIFIED":    { label: "Internally verified",    domain: "Evidence",  chip: "solved",     kind: "evidence" },
      "PROVER_CORRELATED":      { label: "Prover correlated",      domain: "Evidence",  chip: "solved",     kind: "evidence" }
    },
    // Ruling 8: an agent card carries exactly one of these five class words as its label.
    agentCardClasses: ["Check", "Open issue", "Draft", "Proposal", "Evidence summary"],
    icon: { "grid": 16, "stroke": 1.5, "cornerRadius": 1.5, "sizes": [12, 16, 20], "capsAndJoins": "round" }
  };
}

// ---- checks -----------------------------------------------------------------
// Built-in twin of the dataviz validator's ordinal check (same four checks and thresholds): monotone L,
// adjacent ΔL ≥ 0.06 on the raw values, the step nearest the surface ≥ 2.0:1 against it, one hue (≤ 40°).
export function ordinalTwin(palette, { mode, surface }) {
  const Ls = palette.map(h => okLCH(h)[0]);
  const order = [...Ls.keys()].sort((a, b) => Ls[a] - Ls[b]);
  const mono = order.every((v, i) => v === i) || order.every((v, i) => v === Ls.length - 1 - i);
  const gaps = Ls.slice(1).map((l, i) => Math.abs(l - Ls[i]));
  const thin = gaps.filter(g => g < 0.06).length === 0;
  const byL = [...palette].sort((a, b) => okLCH(a)[0] - okLCH(b)[0]);
  const nearest = mode === "light" ? byL[byL.length - 1] : byL[0];
  const cr = wcag(nearest, surface);
  const hues = palette.map(h => okLCH(h)[2]);
  let spread = Math.max(...hues) - Math.min(...hues); if (spread > 180) spread = 360 - spread;
  const one = spread <= 40;
  return { ok: mono && thin && cr >= 2.0 && one, report: [["Lightness monotone", mono, Ls.map(l => +l.toFixed(3)).join(" ")], ["Adjacent ΔL", thin, "min " + Math.min(...gaps).toFixed(3)], ["Light-end contrast", cr >= 2.0, `${nearest} at ${cr.toFixed(2)}:1 vs surface`], ["Single hue", one, `hue spread ${spread.toFixed(0)}°`]] };
}

const argv = process.argv;
const isMain = argv[1] && /palette\.mjs$/.test(argv[1]);
if (isMain) {
  const vpath = process.env.DATAVIZ_VALIDATOR || (argv.includes("--validator") ? argv[argv.indexOf("--validator") + 1] : null);
  let ext = null;
  if (vpath) { try { ext = await import(pathToFileURL(vpath).href); } catch (e) { console.log("validator not loaded:", e.message); } }
  const canvasL = val("canvas.bg")[0], canvasD = val("canvas.bg")[1];
  console.log("== semantic tokens ==");
  for (const [k,[l,d]] of T) console.log(k.padEnd(24), l.padEnd(22), d);
  console.log("\n== result ramp light ==", resultLight.join(","));
  console.log("== result ramp dark  ==", resultDark.join(","));
  console.log("   dark L:", resultDark.map(h => okLCH(h)[0].toFixed(3)).join(" "), "| canvas.edge dark L", okLCH(val("canvas.edge")[1])[0].toFixed(3));
  const vo = ext ? ext.validateOrdinal : (p, o) => ordinalTwin(p, o);
  console.log("\nordinal check light (" + (ext ? "dataviz validator" : "built-in twin") + "):", JSON.stringify(vo(resultLight, { mode: "light", surface: canvasL })));
  console.log("ordinal check dark  (" + (ext ? "dataviz validator" : "built-in twin") + "):", JSON.stringify(vo(resultDark, { mode: "dark", surface: canvasD })));
  console.log("\n== categorical (fixed order) ==", ORDER.join(","));
  const L8 = ORDER.map(n => catLight[catNames.indexOf(n)]), D8 = ORDER.map(n => catDark[catNames.indexOf(n)]);
  console.log("light:", L8.join(",")); console.log("dark :", D8.join(","));
  if (ext) {
    console.log(JSON.stringify(ext.validate(L8, { mode: "light", surface: canvasL }), null, 1));
    console.log(JSON.stringify(ext.validate(D8, { mode: "dark", surface: canvasD }), null, 1));
  } else console.log("categorical CVD checks: not run (no validator supplied)");
  if (argv.includes("--search") && ext) {
    const perms = (arr) => arr.length <= 1 ? [arr] : arr.flatMap((x,i) => perms([...arr.slice(0,i), ...arr.slice(i+1)]).map(p => [x, ...p]));
    const parse = (r) => { const m = r.report.find(x => x[0]==="CVD separation")[2].match(/ΔE ([\d.]+)/); const n = r.report.find(x => x[0]==="Normal-vision floor")[2].match(/ΔE ([\d.]+)/); return [parseFloat(m[1]), parseFloat(n[1])]; };
    const cands = [];
    for (const p of perms([0,1,2,3,4,5,6,7])) {
      if (p[0] !== 0) continue;
      const [cl, nl] = parse(ext.validate(p.map(i => catLight[i]), {mode:"light", surface: canvasL})), [cd, nd] = parse(ext.validate(p.map(i => catDark[i]), {mode:"dark", surface: canvasD}));
      if (Math.min(nl, nd) < 15) continue;
      cands.push({ p, score: Math.min(cl, cd), nscore: Math.min(nl, nd) });
    }
    cands.sort((a,b) => b.score - a.score || b.nscore - a.nscore);
    for (const c of cands.slice(0, 8)) console.log(c.p.map(i => catNames[i]).join(","), "cvd", c.score, "normal", c.nscore);
  }
  if (argv.includes("--write")) {
    const fs = await import("node:fs");
    const dest = argv[argv.indexOf("--write") + 1];
    const out = tokenFile();
    fs.writeFileSync(dest, JSON.stringify(out, null, 2) + "\n");
    console.log("wrote", dest, Object.keys(out.color).length, "colour tokens, version", out.version);
  }
}
