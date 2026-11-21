import { readdirSync, readFileSync, statSync } from "node:fs";
import path from "node:path";
import { describe, expect, it } from "vitest";
import slugCases from "../../../../fixtures/file_names/project_slug_cases.json";
import { FILE_KINDS, FILE_NAME_EXCEPTIONS, RESERVED_KINDS, projectSlug, suggestedFileName } from "./fileNames";

// The rule for every suggested file name. The slug's cases are the shared fixture, which a native implementation
// of the rule reads too. Invented names only.
describe("the project's name as a slug", () => {
  it.each(slugCases.cases.map((item) => [item.case_id, item] as const))("%s", (_id, item) => {
    expect(projectSlug(item.project)).toBe(item.expected_slug);
  });
});

describe("the suggested file name", () => {
  it("is the slug, then the kind and extension from the table", () => {
    expect(suggestedFileName({ id: "project:invented-demo-loop-01", name: "Invented Utility Loop Demo" }, "pcfPackage")).toBe("invented-utility-loop-demo-pcf-package.json");
    expect(suggestedFileName({ id: "project:x", name: "Loop" }, "pcfText")).toBe("loop-pcf.pcf");
    expect(suggestedFileName({ id: "project:x", name: "Loop" }, "stressNeutralRows")).toBe("loop-stress-neutral-rows.csv");
    // A name that already ends in the kind is kept as it is.
    expect(suggestedFileName({ id: "project:x", name: "Loop PCF package" }, "pcfPackage")).toBe("loop-pcf-package-pcf-package.json");
  });

  it("tells files of one project apart by their record's reference, at most 32 characters, and has none where it is empty", () => {
    expect(suggestedFileName({ id: "project:x", name: "Loop" }, "reportPacket", "run:preview-linear-static-001")).toBe("loop-report-packet-run-preview-linear-static-001.json");
    expect(suggestedFileName({ id: "project:x", name: "Loop" }, "reportPacket", `run:${"r".repeat(40)}`)).toBe(`loop-report-packet-run-${"r".repeat(28)}.json`);
    expect(suggestedFileName({ id: "project:x", name: "Loop" }, "reportPacket", ":::")).toBe("loop-report-packet.json");
    expect(suggestedFileName({ id: "project:x", name: "Loop" }, "reportPacket", null)).toBe("loop-report-packet.json");
  });

  it("has a kind and extension for every entry, with no product name, no \"preview\" and no reserved kind", () => {
    for (const { kind, extension } of Object.values(FILE_KINDS)) {
      expect(kind).toMatch(/^[a-z0-9]+(-[a-z0-9]+)*$/);
      expect(extension).toMatch(/^[a-z0-9]+(\.[a-z0-9]+)*$/);
      expect(`${kind}.${extension}`).not.toMatch(/openpipestress|swbpipe|preview/);
      expect(RESERVED_KINDS).not.toContain(kind);
    }
  });
});

// No file under the desktop's sources composes a suggested name with the product's prefix or "preview" for itself,
// apart from the lines named in FILE_NAME_EXCEPTIONS.
const SRC = path.resolve(__dirname, "..");

function sources(directory: string): string[] {
  return readdirSync(directory).flatMap((entry) => {
    const full = path.join(directory, entry);
    if (statSync(full).isDirectory()) return entry === "test" || entry === "test-support" ? [] : sources(full);
    return /\.(ts|tsx)$/.test(entry) && !/\.test\.(ts|tsx)$/.test(entry) ? [full] : [];
  });
}

function excepted(relative: string, line: number, name: string): boolean {
  return FILE_NAME_EXCEPTIONS.some((item) => item.file === relative && item.line === line && name.startsWith(item.starts));
}

describe("the product's prefix and \"preview\" in suggested file names", () => {
  it("each exception names the line that holds it", () => {
    for (const item of FILE_NAME_EXCEPTIONS) {
      const line = readFileSync(path.join(SRC, item.file), "utf8").split("\n")[item.line - 1] ?? "";
      expect(line, `${item.file}:${item.line}`).toContain(item.starts);
    }
  });

  it("do not come back: no source composes such a name outside the exceptions", () => {
    const found: string[] = [];
    for (const file of sources(SRC)) {
      const relative = path.relative(SRC, file).split(path.sep).join("/");
      const text = readFileSync(file, "utf8");
      const lineOf = (index: number) => text.slice(0, index).split("\n").length;
      // A string or template literal that starts with the product's prefix and ends in a file extension.
      for (const match of text.matchAll(/[`"'](openpipestress-[^`"'\n]*\.(?:json|csv|pcf|html|mbf|gltf\.json))[`"']/g)) {
        if (!excepted(relative, lineOf(match.index!), match[1])) found.push(`${relative}:${lineOf(match.index!)}: ${match[1]}`);
      }
      // A download attribute whose literal name holds "preview".
      for (const match of text.matchAll(/download=\{?\s*[`"']([^`"'\n]*)[`"']/g)) {
        if (/preview/i.test(match[1]) && !excepted(relative, lineOf(match.index!), match[1])) found.push(`${relative}:${lineOf(match.index!)}: ${match[1]}`);
      }
    }
    expect(found).toEqual([]);
  });
});
