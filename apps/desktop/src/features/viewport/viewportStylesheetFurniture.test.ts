import { readFileSync } from "node:fs";
import path from "node:path";
import { describe, expect, it } from "vitest";

// Vitest runs with the desktop package as its working directory.
const STYLES = readFileSync(path.join(process.cwd(), "src", "styles.css"), "utf8");

const COLOUR_LITERAL = /#[0-9a-fA-F]{3,8}\b|\brgba?\(|\bhsla?\(/;

/**
 * Every declaration block whose selector list is exactly `selector`, at any nesting depth.
 * Comments are dropped first; a block is the text between a `{` and the next `}`, and its
 * selector is what stands between the previous brace or semicolon-free boundary and that `{`.
 */
function blocksFor(selector: string): string[] {
  const text = STYLES.replace(/\/\*[\s\S]*?\*\//g, "");
  const blocks: string[] = [];
  for (const match of text.matchAll(/([^{}]*)\{([^{}]*)\}/g)) {
    if (match[1].trim() === selector) blocks.push(match[2]);
  }
  return blocks;
}

function declarations(block: string): Map<string, string> {
  const map = new Map<string, string>();
  for (const part of block.split(";")) {
    const at = part.indexOf(":");
    if (at > 0) map.set(part.slice(0, at).trim(), part.slice(at + 1).trim());
  }
  return map;
}

describe("canvas furniture drawn by the stylesheet", () => {
  // The furniture's colours are tokens; a shadow is not a furniture colour and is left alone.
  // `.viewport-shell` also shares a rule with `.panel`, whose values are the shell lane's; the
  // shell's own later block overrides them, and only blocks that are the selector's own are read.
  it.each([".viewport-shell", ".viewport-scale-bar", ".viewport-fallback"])(
    "%s states no colour literal for its ink, ground or border",
    (selector) => {
      const blocks = blocksFor(selector);
      expect(blocks.length).toBeGreaterThan(0);
      for (const block of blocks) {
        for (const [property, value] of declarations(block)) {
          if (!/^(color|background|background-color|border|border-[a-z-]*)$/.test(property)) continue;
          expect(`${selector} { ${property}: ${value} }`).not.toMatch(COLOUR_LITERAL);
        }
      }
    }
  );

  it("draws the furniture in the requested tokens", () => {
    const all = (selector: string) => declarations(blocksFor(selector).join(";"));
    expect(all(".viewport-shell").get("background")).toBe("var(--ui-canvas)");
    expect(all(".viewport-shell").get("border-color")).toBe("var(--ui-divider)");
    expect(all(".viewport-scale-bar").get("color")).toBe("var(--canvas-label)");
    expect(all(".viewport-scale-bar").get("background")).toBe("var(--canvas-labelBg)");
    expect(all(".viewport-scale-bar").get("border-bottom")).toBe("4px solid var(--canvas-label)");
    expect(all(".viewport-fallback").get("color")).toBe("var(--ui-muted)");
    expect(all(".viewport-fallback").get("background")).toBe("var(--ui-canvas)");
  });

  it("keeps each mirror of the viewport's colours equal to its token, stated once", () => {
    const canvas = [...STYLES.matchAll(/--ui-canvas\s*:\s*([^;]+);/g)].map((match) => match[1].trim());
    const selection = [...STYLES.matchAll(/--ui-viewport-selection-geometry\s*:\s*([^;]+);/g)].map((match) => match[1].trim());
    expect(canvas).toEqual(["var(--canvas-bg)"]);
    expect(selection).toEqual(["var(--canvas-selection)"]);
  });
});
