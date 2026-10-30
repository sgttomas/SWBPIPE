import { createHash } from "node:crypto";
import { readFileSync } from "node:fs";
import path from "node:path";
import { describe, expect, it } from "vitest";

// The hash recorded in TOKENS_SOURCE.md: design system V1.3, token file version 1.2.
const RECORDED_SHA256 = "00c1afe97365973e6c05a2a5d938a25c46022a55c3cbc8d04ff3a9d2d68cf1e9";

// Vitest runs with the desktop package as its working directory.
const desktopRoot = process.cwd();
const tokensJsonPath = path.join(desktopRoot, "src", "design", "tokens.json");
const tokensCssPath = path.join(desktopRoot, "src", "tokens.css");
const generatorPath = path.join(desktopRoot, "scripts", "generate-tokens.mjs");

type Generator = { cssFrom(tokens: unknown): string; generatedCss(): string };

async function loadGenerator(): Promise<Generator> {
  // A computed specifier: the generator is a plain Node script outside the TypeScript project.
  const specifier = `file://${generatorPath}`;
  return (await import(/* @vite-ignore */ specifier)) as Generator;
}

describe("design tokens", () => {
  it("tokens.json is the recorded design-system file, byte for byte", () => {
    const digest = createHash("sha256").update(readFileSync(tokensJsonPath)).digest("hex");
    expect(digest).toBe(RECORDED_SHA256);
  });

  it("TOKENS_SOURCE.md records the same hash", () => {
    const note = readFileSync(path.join(desktopRoot, "src", "design", "TOKENS_SOURCE.md"), "utf8");
    expect(note).toContain(RECORDED_SHA256);
  });

  it("tokens.css is exactly the generator's output", async () => {
    const generator = await loadGenerator();
    const tokens: unknown = JSON.parse(readFileSync(tokensJsonPath, "utf8"));
    const checkedIn = readFileSync(tokensCssPath, "utf8");
    expect(checkedIn).toBe(generator.cssFrom(tokens) + "\n");
    expect(checkedIn).toBe(generator.generatedCss());
  });

  it("styles.css loads tokens.css before any rule, so its workspace variables always resolve", () => {
    const styles = readFileSync(path.join(desktopRoot, "src", "styles.css"), "utf8");
    const withoutComments = styles.replace(/\/\*[\s\S]*?\*\//g, "").trimStart();
    expect(withoutComments.startsWith('@import "./tokens.css";')).toBe(true);
  });

  it("the generator emits light, system-dark and explicit-dark blocks on :root", async () => {
    const css = (await loadGenerator()).generatedCss();
    expect(css).toContain(':root, :root[data-theme="light"] {');
    expect(css).toContain('@media (prefers-color-scheme: dark) {\n  :root:not([data-theme="light"]) {');
    expect(css).toContain('\n:root[data-theme="dark"] {');
    expect(css).toContain("  --surface-base: #eef0f3;");
    expect(css).toContain("  --surface-base: #1b1e22;");
  });
});
