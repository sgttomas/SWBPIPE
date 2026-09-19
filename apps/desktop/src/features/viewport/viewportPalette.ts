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

export type ViewportTokenName = `canvas.${CanvasTokenKey}` | `cat.${CategoryTokenSlot}` | "surface.raised";

/** Every token the canvas may read. Several have no consumer yet; later slices only add consumers. */
export const VIEWPORT_TOKEN_NAMES: readonly ViewportTokenName[] = Object.freeze([
  ...CANVAS_TOKEN_KEYS.map((key): ViewportTokenName => `canvas.${key}`),
  ...CATEGORY_TOKEN_SLOTS.map((slot): ViewportTokenName => `cat.${slot}`),
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
 * ground in both themes. `canvas.vector` is also the load vectors' ink, so the two would
 * share a colour once result colour is drawn; that state is not reached, because when 6.8
 * is built (the undeformed shape becomes the dashed outline, which needs the edge-line
 * mechanism and result colour) this role goes and the deformed shape takes the design's
 * colours.
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
