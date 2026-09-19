import { readdirSync, readFileSync } from "node:fs";
import path from "node:path";
import { describe, expect, it } from "vitest";

// Vitest runs with the desktop package as its working directory.
const VIEWPORT_DIR = path.join(process.cwd(), "src", "features", "viewport");
const HELD_FILE = "viewportResource.ts";
const HELD_BEGIN = "// HELD-COLOURS:BEGIN";
const HELD_END = "// HELD-COLOURS:END";

/**
 * The exact set the held block may state, in source order: the scene background (light, dark),
 * the selected colour (light, dark) and the selection cue's rim (light, dark). They are held
 * for the benchmark instrument's second profile; every other canvas colour is a design token.
 */
const HELD_ALLOWED = ["0xdfe5e8", "0x0c1114", "0xa34400", "0xf08c22", "0xffffff", "0x0c1114"];

// A colour literal: a six-digit hexadecimal number, a quoted hash colour of any usual length,
// or either functional notation.
const COLOUR_LITERAL =
  /0[xX][0-9a-fA-F]{6}\b|["'`]#(?:[0-9a-fA-F]{3,4}|[0-9a-fA-F]{6}|[0-9a-fA-F]{8})["'`]|\brgba?\(/gi;

function isTestFile(name: string): boolean {
  return /\.test\.tsx?$/.test(name);
}

function nonTestFiles(directory: string): string[] {
  return readdirSync(directory, { withFileTypes: true }).flatMap((entry) => {
    const full = path.join(directory, entry.name);
    if (entry.isDirectory()) return nonTestFiles(full);
    return entry.isFile() && !isTestFile(entry.name) ? [full] : [];
  });
}

function literalsIn(text: string): string[] {
  return [...text.matchAll(COLOUR_LITERAL)].map((match) => match[0]);
}

function relative(file: string): string {
  return path.relative(VIEWPORT_DIR, file).split(path.sep).join("/");
}

describe("viewport colour literals", () => {
  it("recognises each literal form it is meant to refuse", () => {
    expect(literalsIn("const a = 0x24705a;")).toEqual(["0x24705a"]);
    expect(literalsIn("const a = 0X24705A;")).toEqual(["0X24705A"]);
    expect(literalsIn("fill: RGBA(1, 2, 3, 0.5)")).toEqual(["RGBA("]);
    expect(literalsIn('color: "#fff"')).toEqual(['"#fff"']);
    expect(literalsIn("color: '#1a2b3c'")).toEqual(["'#1a2b3c'"]);
    expect(literalsIn("color: `#1a2b3c80`")).toEqual(["`#1a2b3c80`"]);
    expect(literalsIn("fill: rgb(1, 2, 3)")).toEqual(["rgb("]);
    expect(literalsIn("fill: rgba(1, 2, 3, 0.5)")).toEqual(["rgba("]);
    // Not colours: a short mask, a longer number, a private field, a computed hash string.
    expect(literalsIn("value & 0xff")).toEqual([]);
    expect(literalsIn("const big = 0x1234567;")).toEqual([]);
    expect(literalsIn("this.#abc")).toEqual([]);
    expect(literalsIn('`#${value.toString(16).padStart(6, "0")}`')).toEqual([]);
  });

  it("finds the viewport sources, including the three this slice is about", () => {
    const files = nonTestFiles(VIEWPORT_DIR).map(relative);
    for (const expected of [HELD_FILE, "PipeViewport.tsx", "viewportPalette.ts", "viewportFigureMaterial.ts"]) {
      expect(files).toContain(expected);
    }
    expect(files.some((file) => isTestFile(file))).toBe(false);
  });

  it("states the held colours once, in one marked block, as exactly the allowed set", () => {
    const text = readFileSync(path.join(VIEWPORT_DIR, HELD_FILE), "utf8");
    expect(text.split(HELD_BEGIN)).toHaveLength(2);
    expect(text.split(HELD_END)).toHaveLength(2);
    const begin = text.indexOf(HELD_BEGIN);
    const end = text.indexOf(HELD_END);
    expect(end).toBeGreaterThan(begin);
    const block = text.slice(begin, end);
    expect(literalsIn(block)).toEqual(HELD_ALLOWED);
    // The block is constants and their comment, nothing else.
    const statements = block.split("\n").slice(1).filter((line) => line.trim() !== "" && !line.startsWith("//"));
    expect(statements).toHaveLength(HELD_ALLOWED.length);
    for (const statement of statements) {
      expect(statement).toMatch(/^const [A-Z_]+ = 0x[0-9a-f]{6};$/);
    }
  });

  it("holds no colour literal in any other non-test file, nor outside the held block", () => {
    const offences: string[] = [];
    for (const file of nonTestFiles(VIEWPORT_DIR)) {
      let text = readFileSync(file, "utf8");
      if (relative(file) === HELD_FILE) {
        const begin = text.indexOf(HELD_BEGIN);
        const end = text.indexOf(HELD_END);
        // Blank the held block and keep its line breaks, so a reported line number is the file's.
        text = text.slice(0, begin) + text.slice(begin, end).replace(/[^\n]/g, " ") + text.slice(end);
      }
      text.split("\n").forEach((line, index) => {
        for (const literal of literalsIn(line)) offences.push(`${relative(file)}:${index + 1}: ${literal}`);
      });
    }
    expect(offences).toEqual([]);
  });
});
