import { readdirSync, readFileSync, statSync } from "node:fs";
import path from "node:path";
import { describe, expect, it } from "vitest";
import { FILE_KINDS, projectSlug, suggestedFileName } from "./fileNames";

// The rule for every suggested file name. Invented names only.
describe("the suggested file name", () => {
  it("is the project's display name as a slug, then the kind and extension from the table", () => {
    expect(suggestedFileName({ id: "project:invented-demo-loop-01", name: "Invented Utility Loop Demo" }, "pcfPackage")).toBe("invented-utility-loop-demo-pcf-package.json");
    expect(suggestedFileName({ id: "project:x", name: "Loop" }, "pcfText")).toBe("loop-pcf.pcf");
    expect(suggestedFileName({ id: "project:x", name: "Loop" }, "externalRunCsv")).toBe("loop-external-run.csv");
  });

  it("falls back to the id without its prefix where the display name is empty or only punctuation, and to \"project\" where that is empty too", () => {
    expect(projectSlug({ id: "project:invented-demo-loop-01", name: "" })).toBe("invented-demo-loop-01");
    expect(projectSlug({ id: "project:invented-demo-loop-01", name: null })).toBe("invented-demo-loop-01");
    expect(projectSlug({ id: "project:invented-demo-loop-01", name: " ¿?!… — " })).toBe("invented-demo-loop-01");
    expect(projectSlug({ id: "project:", name: "***" })).toBe("project");
  });

  it("drops non-Latin characters by the same rule, and keeps a name that already ends in the kind as it is", () => {
    expect(projectSlug({ id: "project:x", name: "Трубопровод 7" })).toBe("7");
    expect(projectSlug({ id: "project:loop-02", name: "配管" })).toBe("loop-02");
    expect(suggestedFileName({ id: "project:x", name: "Loop PCF package" }, "pcfPackage")).toBe("loop-pcf-package-pcf-package.json");
  });

  it("is at most 64 characters before the kind, cut with no trailing hyphen", () => {
    expect(projectSlug({ id: "project:x", name: `${"a".repeat(63)} b c d` })).toBe("a".repeat(63));
    expect(projectSlug({ id: "project:x", name: "b".repeat(80) })).toHaveLength(64);
  });

  it("tells files of one project apart by their record's reference, at most 32 characters, and has none where it is empty", () => {
    expect(suggestedFileName({ id: "project:x", name: "Loop" }, "report", "run:preview-linear-static-001")).toBe("loop-report-run-preview-linear-static-001.json");
    expect(suggestedFileName({ id: "project:x", name: "Loop" }, "report", `run:${"r".repeat(40)}`)).toBe(`loop-report-run-${"r".repeat(28)}.json`);
    expect(suggestedFileName({ id: "project:x", name: "Loop" }, "report", ":::")).toBe("loop-report.json");
    expect(suggestedFileName({ id: "project:x", name: "Loop" }, "report", null)).toBe("loop-report.json");
  });

  it("has a kind and extension for every entry, with no product name and no \"preview\"", () => {
    for (const { kind, extension } of Object.values(FILE_KINDS)) {
      expect(kind).toMatch(/^[a-z0-9]+(-[a-z0-9]+)*$/);
      expect(extension).toMatch(/^[a-z0-9]+(\.[a-z0-9]+)*$/);
      expect(`${kind}.${extension}`).not.toMatch(/openpipestress|swbpipe|preview/);
    }
  });
});

// No file under the desktop's sources composes a suggested name with the product's name or "preview" for itself,
// apart from the names a native save checks or forms and the rendered report's, which move with their owners.
const SRC = path.resolve(__dirname, "..");
const EXCLUDED: ReadonlyArray<[string, string]> = [
  ["features/result-export/ResultExportPanel.tsx", "openpipestress-preview-results-"],
  ["features/stress-neutral/StressNeutralExportPanel.tsx", "openpipestress-preview-stress-neutral-"],
  ["features/review-geometry/ReviewGeometryPanel.tsx", "openpipestress-preview-review-geometry-"],
  ["features/report/RenderedReportPanel.tsx", "openpipestress-local-private-report-"],
  ["features/report/RenderedReportPanel.tsx", "openpipestress-report-"]
];

function sources(directory: string): string[] {
  return readdirSync(directory).flatMap((entry) => {
    const full = path.join(directory, entry);
    if (statSync(full).isDirectory()) return entry === "test" || entry === "test-support" ? [] : sources(full);
    return /\.(ts|tsx)$/.test(entry) && !/\.test\.(ts|tsx)$/.test(entry) ? [full] : [];
  });
}

describe("the product's name and \"preview\" in suggested file names", () => {
  it("do not come back: no source composes such a name outside the excluded lines", () => {
    const found: string[] = [];
    for (const file of sources(SRC)) {
      const relative = path.relative(SRC, file).split(path.sep).join("/");
      const text = readFileSync(file, "utf8");
      // A string or template literal that starts with the product's name and ends in a file extension.
      for (const match of text.matchAll(/[`"'](openpipestress-[^`"'\n]*\.(?:json|csv|pcf|html|mbf|gltf\.json))[`"']/g)) {
        if (!EXCLUDED.some(([where, start]) => where === relative && match[1].startsWith(start))) found.push(`${relative}: ${match[1]}`);
      }
      // A download attribute whose literal name holds "preview".
      for (const match of text.matchAll(/download=\{?\s*[`"']([^`"'\n]*)[`"']/g)) {
        if (/preview/i.test(match[1]) && !EXCLUDED.some(([where, start]) => where === relative && match[1].startsWith(start))) found.push(`${relative}: ${match[1]}`);
      }
    }
    expect(found).toEqual([]);
  });
});
