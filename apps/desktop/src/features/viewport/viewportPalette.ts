import tokens from "../../design/tokens.json";

/**
 * The one place where the canvas reads design tokens.
 *
 * The numbers come from the product's token file, parsed once at module load into frozen
 * per-theme tables. Nothing here reads the DOM or a computed style, so a canvas and a test
 * resolve the same values.
 */

export type ViewportPaletteTheme = "light" | "dark";

export const VIEWPORT_PALETTE_THEMES: readonly ViewportPaletteTheme[] = Object.freeze(["light", "dark"] as const);

/** The theme a canvas object is painted for until a viewport resource paints it for its own. */
export const DEFAULT_VIEWPORT_PALETTE_THEME: ViewportPaletteTheme = "light";

/** A parsed token colour: `hex` is 0xRRGGBB in sRGB, `alpha` is 0 to 1 (1 when the token states none). */
export type TokenColour = Readonly<{ hex: number; alpha: number }>;

/** The 22 `canvas.*` tokens of the design system, by their name after the dot. */
export const CANVAS_TOKEN_KEYS = Object.freeze([
  "bg",
  "gridMajor",
  "gridMinor",
  "pipe",
  "pipeShade",
  "edge",
  "edgeAlt",
  "glyph",
  "glyphFill",
  "label",
  "labelBg",
  "hint",
  "vector",
  "selection",
  "hover",
  "draft",
  "proposalGhost",
  "deformGhost",
  "unsolved",
  "axisX",
  "axisY",
  "axisZ"
] as const);

export type CanvasTokenKey = (typeof CANVAS_TOKEN_KEYS)[number];

/** The eight categorical slots, `cat.1` to `cat.8`, in their fixed order. */
export const CATEGORY_TOKEN_SLOTS = Object.freeze([1, 2, 3, 4, 5, 6, 7, 8] as const);

export type CategoryTokenSlot = (typeof CATEGORY_TOKEN_SLOTS)[number];

/** The seven anchors of the result scale, `result.scale.1` (near zero) to `result.scale.7` (maximum). */
export const RESULT_SCALE_STEPS = Object.freeze([1, 2, 3, 4, 5, 6, 7] as const);

export type ResultScaleStep = (typeof RESULT_SCALE_STEPS)[number];

export type ViewportTokenName =
  | `canvas.${CanvasTokenKey}`
  | `cat.${CategoryTokenSlot}`
  | `result.scale.${ResultScaleStep}`
  | "surface.raised";

/** Every token the canvas may read. Several have no consumer yet; later slices only add consumers. */
export const VIEWPORT_TOKEN_NAMES: readonly ViewportTokenName[] = Object.freeze([
  ...CANVAS_TOKEN_KEYS.map((key): ViewportTokenName => `canvas.${key}`),
  ...CATEGORY_TOKEN_SLOTS.map((slot): ViewportTokenName => `cat.${slot}`),
  ...RESULT_SCALE_STEPS.map((step): ViewportTokenName => `result.scale.${step}`),
  "surface.raised"
]);

const SHORT_HEX_PATTERN = /^#([0-9a-f])([0-9a-f])([0-9a-f])$/i;
const LONG_HEX_PATTERN = /^#([0-9a-f]{6})$/i;
const FUNCTIONAL_PATTERN =
  /^(rgb|rgba)\(\s*(\d{1,3})\s*,\s*(\d{1,3})\s*,\s*(\d{1,3})\s*(?:,\s*(\d+(?:\.\d+)?|\.\d+)\s*)?\)$/i;

/**
 * Parses one token colour. Accepted forms: a three-digit or a six-digit hash colour, the
 * three-channel functional notation, and the four-channel functional notation with an alpha
 * of 0 to 1. Spaces inside the functional notations are tolerated. Anything else throws with
 * the offending value.
 */
export function parseTokenColour(value: unknown): TokenColour {
  if (typeof value === "string") {
    const text = value.trim();
    const short = SHORT_HEX_PATTERN.exec(text);
    if (short) {
      const [red, green, blue] = [short[1], short[2], short[3]].map((digit) => Number.parseInt(digit + digit, 16));
      return Object.freeze({ hex: (red << 16) | (green << 8) | blue, alpha: 1 });
    }
    const long = LONG_HEX_PATTERN.exec(text);
    if (long) return Object.freeze({ hex: Number.parseInt(long[1], 16), alpha: 1 });
    const functional = FUNCTIONAL_PATTERN.exec(text);
    if (functional) {
      const namesAlpha = functional[1].toLowerCase() === "rgba";
      const channels = [functional[2], functional[3], functional[4]].map((channel) => Number.parseInt(channel, 10));
      const alpha = functional[5] === undefined ? 1 : Number.parseFloat(functional[5]);
      const formMatches = namesAlpha === (functional[5] !== undefined);
      if (formMatches && channels.every((channel) => channel <= 255) && alpha >= 0 && alpha <= 1) {
        return Object.freeze({ hex: (channels[0] << 16) | (channels[1] << 8) | channels[2], alpha });
      }
    }
  }
  throw new Error(`Unsupported token colour: ${JSON.stringify(value)}`);
}

export type ViewportPaletteTable = Readonly<Record<ViewportTokenName, TokenColour>>;

function readThemeTable(theme: ViewportPaletteTheme): ViewportPaletteTable {
  const source = tokens.color as Readonly<Record<string, Readonly<Record<string, unknown>> | undefined>>;
  const table = {} as Record<ViewportTokenName, TokenColour>;
  for (const name of VIEWPORT_TOKEN_NAMES) {
    const entry = source[name];
    if (!entry) throw new Error(`Design token ${name} is absent from the token file.`);
    try {
      table[name] = parseTokenColour(entry[theme]);
    } catch (error) {
      throw new Error(`Design token ${name} (${theme}): ${error instanceof Error ? error.message : String(error)}`);
    }
  }
  return Object.freeze(table);
}

/** Every viewport token, parsed, for both themes. */
export const VIEWPORT_PALETTE: Readonly<Record<ViewportPaletteTheme, ViewportPaletteTable>> = Object.freeze({
  light: readThemeTable("light"),
  dark: readThemeTable("dark")
});

/**
 * The single binding from what the canvas draws to a token. Fittings take the tube's colour
 * and are told apart by shape; a rigid element is the shaded tube; a restraint glyph is the
 * glyph ink; load kinds take the first two categorical slots; the routing draft has its own
 * token and is never the selection's colour; the grid tokens are already stepped for the
 * ground and draw at full opacity.
 *
 * The deformed overlay's binding to `canvas.vector` is provisional. Design system 6.8 draws
 * the deformed shape solid "with the result colour or the pipe neutral" and the undeformed
 * shape as a dashed outline in `canvas.deformGhost`. The pipe neutral was weighed and cannot
 * be used yet: this product still draws both shapes solid at once, so a deformed tube in
 * `canvas.pipe` could not be told from the undeformed tube beside it. `canvas.deformGhost`
 * is the outline's token, and at the overlay's opacity it falls under 3:1 against the
 * ground in both themes. `canvas.vector` is also the ink of vectors drawn while result colour
 * is on (design system 2.1), which here are the support reaction arrows; they share the
 * deformed shape's colour and are told from it by form. Result colour (6.7) is drawn on the
 * undeformed tube only; when 6.8 is built (the undeformed shape becomes the dashed outline,
 * which needs the edge-line mechanism) this role goes and the deformed shape takes the
 * design's colours.
 *
 * Result colour itself is not a role: an element's colour is a position on the result scale
 * (`resultScaleHex`), or `resultNoValueHex` for an element with no colourable value.
 */
export const VIEWPORT_ROLE_TOKENS = Object.freeze({
  pipe: "canvas.pipe",
  node: "canvas.pipeShade",
  support: "canvas.glyph",
  componentBend: "canvas.pipe",
  componentBranch: "canvas.pipe",
  componentExpansion: "canvas.pipe",
  componentRigid: "canvas.pipeShade",
  deformedShape: "canvas.vector",
  groundGridMajor: "canvas.gridMajor",
  groundGridMinor: "canvas.gridMinor",
  routeGridAxis: "canvas.draft",
  routeGridLine: "canvas.gridMajor",
  routeDraft: "canvas.draft",
  loadForce: "cat.1",
  loadMoment: "cat.2",
  // Support reactions are result vectors: neutral ink (design system 2.1), told apart by form, a
  // single head for a force and the rotation glyph's double head for a moment.
  reactionForce: "canvas.vector",
  reactionMoment: "canvas.vector",
  gizmoAxisX: "canvas.axisX",
  gizmoAxisY: "canvas.axisY",
  gizmoAxisZ: "canvas.axisZ",
  gizmoBadge: "surface.raised"
} as const satisfies Readonly<Record<string, ViewportTokenName>>);

export type ViewportPaletteRole = keyof typeof VIEWPORT_ROLE_TOKENS;

export const VIEWPORT_PALETTE_ROLES: readonly ViewportPaletteRole[] = Object.freeze(
  Object.keys(VIEWPORT_ROLE_TOKENS) as ViewportPaletteRole[]
);

export function isViewportPaletteRole(value: unknown): value is ViewportPaletteRole {
  return typeof value === "string" && Object.prototype.hasOwnProperty.call(VIEWPORT_ROLE_TOKENS, value);
}

export function viewportTokenColour(theme: ViewportPaletteTheme, name: ViewportTokenName): TokenColour {
  return VIEWPORT_PALETTE[theme][name];
}

export function viewportRoleColour(theme: ViewportPaletteTheme, role: ViewportPaletteRole): TokenColour {
  return VIEWPORT_PALETTE[theme][VIEWPORT_ROLE_TOKENS[role]];
}

/** The role's colour as 0xRRGGBB in sRGB, as three's `Color.setHex` takes it. */
export function viewportRoleHex(theme: ViewportPaletteTheme, role: ViewportPaletteRole): number {
  return viewportRoleColour(theme, role).hex;
}

function linearChannel(hex: number, shift: number): number {
  const value = ((hex >> shift) & 0xff) / 255;
  return value <= 0.04045 ? value / 12.92 : ((value + 0.055) / 1.055) ** 2.4;
}

function shadeRatioFor(theme: ViewportPaletteTheme): readonly [number, number, number] {
  const pipe = VIEWPORT_PALETTE[theme]["canvas.pipe"].hex;
  const shade = VIEWPORT_PALETTE[theme]["canvas.pipeShade"].hex;
  const ratio = (shift: number): number => {
    const denominator = linearChannel(pipe, shift);
    return denominator > 0 ? linearChannel(shade, shift) / denominator : 1;
  };
  return Object.freeze([ratio(16), ratio(8), ratio(0)] as const);
}

const SHADE_RATIOS: Readonly<Record<ViewportPaletteTheme, readonly [number, number, number]>> = Object.freeze({
  light: shadeRatioFor("light"),
  dark: shadeRatioFor("dark")
});

/**
 * The silhouette darkening of the matte figure, per channel in linear terms:
 * linear(canvas.pipeShade) / linear(canvas.pipe). No other shade value exists.
 */
export function viewportShadeRatio(theme: ViewportPaletteTheme): readonly [number, number, number] {
  return SHADE_RATIOS[theme];
}

// ---------------------------------------------------------------------------
// The result colour scale (design system 2.6, 6.7).
// ---------------------------------------------------------------------------

type Oklch = Readonly<{ l: number; c: number; h: number }>;

function srgbChannelToLinear(value: number): number {
  return value <= 0.04045 ? value / 12.92 : ((value + 0.055) / 1.055) ** 2.4;
}

function linearChannelToSrgb(value: number): number {
  const clamped = Math.min(1, Math.max(0, value));
  return clamped <= 0.0031308 ? clamped * 12.92 : 1.055 * clamped ** (1 / 2.4) - 0.055;
}

/** An sRGB hex colour in OKLCH (Ottosson's OKLab, polar form). */
export function hexToOklch(hex: number): Oklch {
  const r = srgbChannelToLinear(((hex >> 16) & 0xff) / 255);
  const g = srgbChannelToLinear(((hex >> 8) & 0xff) / 255);
  const b = srgbChannelToLinear((hex & 0xff) / 255);
  const l = Math.cbrt(0.4122214708 * r + 0.5363325363 * g + 0.0514459929 * b);
  const m = Math.cbrt(0.2119034982 * r + 0.6806995451 * g + 0.1073969566 * b);
  const s = Math.cbrt(0.0883024619 * r + 0.2817188376 * g + 0.6299787005 * b);
  const lightness = 0.2104542553 * l + 0.793617785 * m - 0.0040720468 * s;
  const a = 1.9779984951 * l - 2.428592205 * m + 0.4505937099 * s;
  const bAxis = 0.0259040371 * l + 0.7827717662 * m - 0.808675766 * s;
  return Object.freeze({ l: lightness, c: Math.hypot(a, bAxis), h: Math.atan2(bAxis, a) });
}

/** An OKLCH colour back to an sRGB hex, each channel clamped to the gamut and rounded. */
export function oklchToHex({ l: lightness, c, h }: Oklch): number {
  const a = c * Math.cos(h);
  const bAxis = c * Math.sin(h);
  const l = (lightness + 0.3963377774 * a + 0.2158037573 * bAxis) ** 3;
  const m = (lightness - 0.1055613458 * a - 0.0638541728 * bAxis) ** 3;
  const s = (lightness - 0.0894841775 * a - 1.291485548 * bAxis) ** 3;
  const channel = (value: number) => Math.round(linearChannelToSrgb(value) * 255);
  const red = channel(4.0767416621 * l - 3.3077115913 * m + 0.2309699292 * s);
  const green = channel(-1.2684380046 * l + 2.6097574011 * m - 0.3413193965 * s);
  const blue = channel(-0.0041960863 * l - 0.7034186147 * m + 1.707614701 * s);
  return (red << 16) | (green << 8) | blue;
}

const RESULT_SCALE_ANCHORS: Readonly<Record<ViewportPaletteTheme, readonly Oklch[]>> = Object.freeze({
  light: Object.freeze(RESULT_SCALE_STEPS.map((step) => hexToOklch(VIEWPORT_PALETTE.light[`result.scale.${step}`].hex))),
  dark: Object.freeze(RESULT_SCALE_STEPS.map((step) => hexToOklch(VIEWPORT_PALETTE.dark[`result.scale.${step}`].hex)))
});

/**
 * The result scale at a position from 0 (near zero, `result.scale.1`) to 1 (the maximum,
 * `result.scale.7`): continuous interpolation in OKLCH between the seven anchors, as design
 * system 2.6 states, with the hue taken along the shorter arc. A position on an anchor is that
 * token exactly; a position outside 0 to 1, or not finite, is clamped to the nearer end.
 */
export function resultScaleHex(theme: ViewportPaletteTheme, position: number): number {
  const t = Number.isFinite(position) ? Math.min(1, Math.max(0, position)) : 0;
  const scaled = t * (RESULT_SCALE_STEPS.length - 1);
  const lower = Math.floor(scaled);
  const fraction = scaled - lower;
  if (fraction === 0) return VIEWPORT_PALETTE[theme][`result.scale.${RESULT_SCALE_STEPS[lower]}`].hex;
  const from = RESULT_SCALE_ANCHORS[theme][lower];
  const to = RESULT_SCALE_ANCHORS[theme][lower + 1];
  let hueStep = to.h - from.h;
  if (hueStep > Math.PI) hueStep -= 2 * Math.PI;
  if (hueStep < -Math.PI) hueStep += 2 * Math.PI;
  return oklchToHex({
    l: from.l + (to.l - from.l) * fraction,
    c: from.c + (to.c - from.c) * fraction,
    h: from.h + hueStep * fraction
  });
}

/** The opacity at which an element with no colourable value draws (design system 2.6, 6.7). */
export const RESULT_NO_VALUE_OPACITY = 0.6;

function blendChannel(over: number, under: number, shift: number, opacity: number): number {
  return Math.round((((over >> shift) & 0xff) * opacity) + (((under >> shift) & 0xff) * (1 - opacity)));
}

/**
 * An element with no colourable value: `canvas.unsolved` at 60% opacity over `canvas.bg`, the
 * colour that opacity gives against the ground, resolved here because an instanced figure has one
 * opacity for all its instances. Derived from the two tokens alone, as the shade ratio is.
 */
export function resultNoValueHex(theme: ViewportPaletteTheme): number {
  const over = VIEWPORT_PALETTE[theme]["canvas.unsolved"].hex;
  const under = VIEWPORT_PALETTE[theme]["canvas.bg"].hex;
  return (blendChannel(over, under, 16, RESULT_NO_VALUE_OPACITY) << 16)
    | (blendChannel(over, under, 8, RESULT_NO_VALUE_OPACITY) << 8)
    | blendChannel(over, under, 0, RESULT_NO_VALUE_OPACITY);
}
