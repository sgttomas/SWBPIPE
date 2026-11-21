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
// apart from the names in FILE_NAME_EXCEPTIONS, each as many times as its entry says.
const SRC = path.resolve(__dirname, "..");

function sources(directory: string): string[] {
  return readdirSync(directory).flatMap((entry) => {
    const full = path.join(directory, entry);
    if (statSync(full).isDirectory()) return entry === "test" || entry === "test-support" ? [] : sources(full);
    return /\.(ts|tsx)$/.test(entry) && !/\.test\.(ts|tsx)$/.test(entry) ? [full] : [];
  });
}

/** The names a source composes with the product's prefix or, in a download attribute, "preview": each literal once,
 * however both patterns find it. */
function composedNames(text: string): string[] {
  const names = new Map<number, string>();
  // A string or template literal that starts with the product's prefix and ends in a file extension.
  for (const match of text.matchAll(/[`"'](openpipestress-[^`"'\n]*\.(?:json|csv|pcf|html|mbf|gltf\.json))[`"']/g)) names.set(match.index!, match[1]);
  // A download attribute whose literal name holds "preview".
  for (const match of text.matchAll(/download=\{?\s*([`"'])([^`"'\n]*)[`"']/g)) {
    if (/preview/i.test(match[2])) names.set(match.index! + match[0].indexOf(match[1]), match[2]);
  }
  return [...names.values()];
}

/** What is wrong with one source: a name no exception covers, or an exception whose count the file does not hold. */
function namesOutOfRule(file: string, text: string, exceptions: typeof FILE_NAME_EXCEPTIONS): string[] {
  const own = exceptions.filter((item) => item.file === file);
  const counts = new Map<(typeof own)[number], number>(own.map((item) => [item, 0]));
  const problems: string[] = [];
  for (const name of composedNames(text)) {
    const covering = own.find((item) => name.startsWith(item.starts));
    if (covering) counts.set(covering, counts.get(covering)! + 1);
    else problems.push(`${file}: ${name}`);
  }
  for (const [item, count] of counts) if (count !== item.count) problems.push(`${file}: ${count} names starting ${item.starts}, the exception says ${item.count}`);
  return problems;
}

describe("the product's prefix and \"preview\" in suggested file names", () => {
  const invented: typeof FILE_NAME_EXCEPTIONS = [{ file: "invented.tsx", starts: "openpipestress-preview-invented-", count: 1, reason: "invented" }];
  const source = "const x = 1;\n<a download={`openpipestress-preview-invented-${id}.json`} />\n";

  it("an exception holds wherever its name sits in the file: a line added above it changes nothing", () => {
    expect(namesOutOfRule("invented.tsx", source, invented)).toEqual([]);
    expect(namesOutOfRule("invented.tsx", `\n\n// invented comment\n${source}`, invented)).toEqual([]);
  });

  it("a stale exception, or a second such name, fails", () => {
    expect(namesOutOfRule("invented.tsx", "const x = 1;\n", invented)).toHaveLength(1);
    expect(namesOutOfRule("invented.tsx", `${source}${source}`, invented)).toHaveLength(1);
    expect(namesOutOfRule("other.tsx", source, invented)).toEqual(["other.tsx: openpipestress-preview-invented-${id}.json"]);
  });

  it("do not come back: no source composes such a name outside the exceptions", () => {
    const problems: string[] = [];
    for (const file of sources(SRC)) problems.push(...namesOutOfRule(path.relative(SRC, file).split(path.sep).join("/"), readFileSync(file, "utf8"), FILE_NAME_EXCEPTIONS));
    // Every exception names a file that exists, and was counted above.
    for (const item of FILE_NAME_EXCEPTIONS) expect(statSync(path.join(SRC, item.file)).isFile(), item.file).toBe(true);
    expect(problems).toEqual([]);
  });
});
