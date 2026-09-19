import { readFileSync } from "node:fs";
import path from "node:path";
import * as THREE from "three";
import { describe, expect, it } from "vitest";
import {
  CANVAS_TOKEN_KEYS,
  CATEGORY_TOKEN_SLOTS,
  isViewportPaletteRole,
  parseTokenColour,
  VIEWPORT_PALETTE,
  VIEWPORT_PALETTE_ROLES,
  VIEWPORT_PALETTE_THEMES,
  VIEWPORT_ROLE_TOKENS,
  VIEWPORT_TOKEN_NAMES,
  viewportRoleColour,
  viewportRoleHex,
  viewportShadeRatio,
  viewportTokenColour,
  type ViewportTokenName
} from "./viewportPalette";

// The token file is read from disk here, by a route that does not pass through the module under
// test. Vitest runs with the desktop package as its working directory.
type TokenFile = { color: Record<string, { light: string; dark: string }> };
const tokenFile = JSON.parse(
  readFileSync(path.join(process.cwd(), "src", "design", "tokens.json"), "utf8")
) as TokenFile;

const TRANSLUCENT_TOKENS: readonly ViewportTokenName[] = ["canvas.labelBg", "canvas.hint"];

function statedChannels(value: string): number[] {
  return value.slice(value.indexOf("(") + 1, value.lastIndexOf(")")).split(",").map((part) => Number(part.trim()));
}

describe("viewport palette tokens", () => {
  it("exposes all 22 canvas tokens, the eight categorical slots and surface.raised", () => {
    expect(CANVAS_TOKEN_KEYS).toHaveLength(22);
    expect(CATEGORY_TOKEN_SLOTS).toEqual([1, 2, 3, 4, 5, 6, 7, 8]);
    expect(VIEWPORT_TOKEN_NAMES).toHaveLength(31);
    expect(new Set(VIEWPORT_TOKEN_NAMES).size).toBe(31);
    expect(VIEWPORT_TOKEN_NAMES).toContain("surface.raised");
    // Every canvas and categorical token the file holds is exposed: a token added to the file
    // later cannot be missed silently.
    const fileCanvasTokens = Object.keys(tokenFile.color).filter((name) => name.startsWith("canvas.")).sort();
    expect(CANVAS_TOKEN_KEYS.map((key) => `canvas.${key}`).sort()).toEqual(fileCanvasTokens);
    const fileCategoryTokens = Object.keys(tokenFile.color).filter((name) => name.startsWith("cat.")).sort();
    expect(CATEGORY_TOKEN_SLOTS.map((slot) => `cat.${slot}`).sort()).toEqual(fileCategoryTokens);
  });

  it("resolves every token in both themes to the value the token file states", () => {
    for (const theme of VIEWPORT_PALETTE_THEMES) {
      for (const name of VIEWPORT_TOKEN_NAMES) {
        const stated = tokenFile.color[name][theme];
        const parsed = viewportTokenColour(theme, name);
        expect(Number.isInteger(parsed.hex), `${name} ${theme}`).toBe(true);
        expect(parsed.hex).toBeGreaterThanOrEqual(0);
        expect(parsed.hex).toBeLessThanOrEqual(0xffffff);
        if (TRANSLUCENT_TOKENS.includes(name)) continue;
        expect(stated.startsWith("#"), `${name} ${theme} is a hash colour`).toBe(true);
        expect(parsed.hex, `${name} ${theme}`).toBe(Number.parseInt(stated.slice(1), 16));
        expect(parsed.alpha, `${name} ${theme}`).toBe(1);
      }
    }
  });

  it("parses the two translucent tokens to the alpha the token file states", () => {
    for (const theme of VIEWPORT_PALETTE_THEMES) {
      for (const name of TRANSLUCENT_TOKENS) {
        const [red, green, blue, alpha] = statedChannels(tokenFile.color[name][theme]);
        const parsed = viewportTokenColour(theme, name);
        expect(parsed.alpha, `${name} ${theme}`).toBe(alpha);
        expect(parsed.alpha).toBeGreaterThan(0);
        expect(parsed.alpha).toBeLessThan(1);
        expect(parsed.hex, `${name} ${theme}`).toBe((red << 16) | (green << 8) | blue);
      }
    }
  });

  it("holds frozen tables, so no consumer can repaint the palette itself", () => {
    expect(Object.isFrozen(VIEWPORT_PALETTE)).toBe(true);
    for (const theme of VIEWPORT_PALETTE_THEMES) {
      expect(Object.isFrozen(VIEWPORT_PALETTE[theme])).toBe(true);
      expect(Object.isFrozen(VIEWPORT_PALETTE[theme]["canvas.pipe"])).toBe(true);
    }
    expect(Object.isFrozen(VIEWPORT_ROLE_TOKENS)).toBe(true);
  });
});

describe("token colour parser", () => {
  it("accepts short and long hash colours and both functional notations, spaces tolerated", () => {
    expect(parseTokenColour("#fff")).toEqual({ hex: 0xffffff, alpha: 1 });
    expect(parseTokenColour("#1a2")).toEqual({ hex: 0x11aa22, alpha: 1 });
    expect(parseTokenColour("#0C1114")).toEqual({ hex: 0x0c1114, alpha: 1 });
    expect(parseTokenColour("rgb(1,2,3)")).toEqual({ hex: 0x010203, alpha: 1 });
    expect(parseTokenColour("rgb( 255 , 128 , 0 )")).toEqual({ hex: 0xff8000, alpha: 1 });
    expect(parseTokenColour("rgba(28,31,37,0.82)")).toEqual({ hex: 0x1c1f25, alpha: 0.82 });
    expect(parseTokenColour(" rgba( 255, 255, 255, .5 ) ")).toEqual({ hex: 0xffffff, alpha: 0.5 });
    expect(parseTokenColour("rgba(0,0,0,1)")).toEqual({ hex: 0x000000, alpha: 1 });
  });

  it("rejects anything else, naming the offending value", () => {
    const malformed: unknown[] = [
      "",
      "#",
      "#ff",
      "#ffff",
      "#fffff",
      "#fffffff",
      "#gggggg",
      "ffffff",
      "0xffffff",
      "red",
      "transparent",
      "rgb(1,2)",
      "rgb(1,2,3,0.5)",
      "rgba(1,2,3)",
      "rgb(256,0,0)",
      "rgba(0,0,0,1.5)",
      "rgba(0,0,0,-0.5)",
      "rgb(1.5,2,3)",
      "rgb(10%,20%,30%)",
      "hsl(0,0%,0%)",
      "#fff; background: red",
      undefined,
      null,
      0xffffff,
      { light: "#ffffff" }
    ];
    for (const value of malformed) {
      expect(() => parseTokenColour(value), JSON.stringify(value)).toThrow("Unsupported token colour");
    }
    expect(() => parseTokenColour("rgb(256,0,0)")).toThrow('"rgb(256,0,0)"');
    expect(() => parseTokenColour("not-a-colour")).toThrow('"not-a-colour"');
  });
});

describe("viewport palette roles", () => {
  it("binds each thing the canvas draws to its one token", () => {
    expect(VIEWPORT_ROLE_TOKENS).toEqual({
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
    });
    // Exposed and not consumed: its use is a later feasibility probe.
    expect(Object.values(VIEWPORT_ROLE_TOKENS)).not.toContain("canvas.edgeAlt");
  });

  it("resolves every role in both themes to its token's value in the token file", () => {
    expect(VIEWPORT_PALETTE_ROLES).toHaveLength(19);
    for (const theme of VIEWPORT_PALETTE_THEMES) {
      for (const role of VIEWPORT_PALETTE_ROLES) {
        const stated = tokenFile.color[VIEWPORT_ROLE_TOKENS[role]][theme];
        expect(viewportRoleHex(theme, role), `${role} ${theme}`).toBe(Number.parseInt(stated.slice(1), 16));
        expect(viewportRoleColour(theme, role).alpha, `${role} ${theme}`).toBe(1);
      }
    }
    expect(isViewportPaletteRole("pipe")).toBe(true);
    expect(isViewportPaletteRole("toString")).toBe(false);
    expect(isViewportPaletteRole("canvas.pipe")).toBe(false);
    expect(isViewportPaletteRole(undefined)).toBe(false);
  });

  it("spot-checks two values against the design system's printed tokens", () => {
    expect(viewportRoleHex("light", "pipe")).toBe(0xa6abb1);
    expect(viewportRoleHex("dark", "routeDraft")).toBe(0x6dadff);
  });

  it("derives the shade ratio from the tube and its shade alone, per channel in linear terms", () => {
    for (const theme of VIEWPORT_PALETTE_THEMES) {
      // three's Color converts an sRGB hex to linear working values: an independent route.
      const pipe = new THREE.Color(viewportTokenColour(theme, "canvas.pipe").hex);
      const shade = new THREE.Color(viewportTokenColour(theme, "canvas.pipeShade").hex);
      const ratio = viewportShadeRatio(theme);
      expect(ratio[0]).toBeCloseTo(shade.r / pipe.r, 6);
      expect(ratio[1]).toBeCloseTo(shade.g / pipe.g, 6);
      expect(ratio[2]).toBeCloseTo(shade.b / pipe.b, 6);
      for (const channel of ratio) {
        expect(channel).toBeGreaterThan(0);
        expect(channel).toBeLessThan(1);
      }
      expect(Object.isFrozen(ratio)).toBe(true);
    }
  });
});
