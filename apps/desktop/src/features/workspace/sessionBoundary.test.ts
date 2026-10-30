import { readdirSync, readFileSync, statSync } from "node:fs";
import path from "node:path";
import { describe, expect, it } from "vitest";

// The import guard of slice B3. The six `*SessionState.ts` hooks hold the
// session's cells with their full setter sets; only the session
// (`workspaceSession.ts`) may call them. A component that imported one would
// get a second, separate copy of the cells and every setter with it.

// Vitest runs with the desktop package as its working directory.
const srcRoot = path.join(process.cwd(), "src");
const SESSION = "features/workspace/workspaceSession.ts";

function sourceFiles(dir: string): string[] {
  return readdirSync(dir).flatMap((name) => {
    const full = path.join(dir, name);
    if (statSync(full).isDirectory()) return sourceFiles(full);
    return /\.(ts|tsx|mts|cts|js|jsx|mjs)$/.test(name) ? [full] : [];
  });
}

// Any module specifier ending in "SessionState", with or without an extension:
// `from "…"`, `import("…")`, `import "…"`, `require("…")`, `vi.mock("…")`.
const SESSION_STATE_SPECIFIER = /["']([^"'\n]*SessionState(?:\.[cm]?[jt]sx?)?)["']/g;

function sessionStateSpecifiers(text: string): string[] {
  return [...text.matchAll(SESSION_STATE_SPECIFIER)].map((match) => match[1]);
}

/** `fooSessionState.test.ts` may import `./fooSessionState`, and nothing else of the kind. */
function ownModuleOfTestFile(relative: string): string | null {
  const match = /([A-Za-z]+SessionState)\.test\.tsx?$/.exec(relative);
  return match ? match[1] : null;
}

describe("session-state import guard", () => {
  const files = sourceFiles(srcRoot).map((full) => ({
    relative: path.relative(srcRoot, full).split(path.sep).join("/"),
    text: readFileSync(full, "utf8")
  }));

  it("finds the six state hooks and the session that calls them", () => {
    const hooks = files.filter((file) => /SessionState\.ts$/.test(file.relative)).map((file) => file.relative).sort();
    expect(hooks).toEqual([
      "features/workspace/chromeSessionState.ts",
      "features/workspace/modelSessionState.ts",
      "features/workspace/operationsSessionState.ts",
      "features/workspace/projectSessionState.ts",
      "features/workspace/resultsSessionState.ts",
      "features/workspace/selectionSessionState.ts"
    ]);
    const session = files.find((file) => file.relative === SESSION);
    expect(session).toBeDefined();
    expect(new Set(sessionStateSpecifiers(session!.text)).size).toBe(6);
  });

  it("lets no file but workspaceSession.ts import a *SessionState module", () => {
    const offenders = files.flatMap((file) => {
      if (file.relative === SESSION) return [];
      // This file names the pattern in its own prose and expectations.
      if (file.relative === "features/workspace/sessionBoundary.test.ts") return [];
      const own = ownModuleOfTestFile(file.relative);
      return sessionStateSpecifiers(file.text)
        .filter((specifier) => !(own && specifier.replace(/\.[cm]?[jt]sx?$/, "").endsWith(`/${own}`)))
        .map((specifier) => `${file.relative} imports ${specifier}`);
    });
    expect(offenders).toEqual([]);
  });

  it("recognises every import form it must catch", () => {
    expect(sessionStateSpecifiers('import { useChromeSessionState } from "./chromeSessionState";')).toEqual(["./chromeSessionState"]);
    expect(sessionStateSpecifiers("const m = await import('../workspace/modelSessionState.ts');")).toEqual([
      "../workspace/modelSessionState.ts"
    ]);
    expect(sessionStateSpecifiers('export * from "./resultsSessionState";')).toEqual(["./resultsSessionState"]);
    expect(sessionStateSpecifiers('import type { X } from "./workspaceSession";')).toEqual([]);
    expect(ownModuleOfTestFile("features/workspace/chromeSessionState.test.ts")).toBe("chromeSessionState");
    expect(ownModuleOfTestFile("features/workspace/shell/Toolbar.test.tsx")).toBeNull();
  });
});
