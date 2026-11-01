// RT-R6 (DEL-08-04 AC-002): the guard. Every `<table` in a non-test `.tsx` under
// `features/results` and `features/comparison` carries `data-csv-export`, the stable marker of a
// table whose panel renders a CSV export control (`CsvExportControl`). A new table without CSV
// export fails here.
import { existsSync, mkdirSync, mkdtempSync, readdirSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import path from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, it } from "vitest";
import { CSV_EXPORT_ATTRIBUTE } from "./CsvExportControl";

const features = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..");

/** Every non-test `.tsx` under `root`'s `results` and `comparison` folders, nested folders
 * included, as paths relative to `root`. */
export function guardedSources(root: string): string[] {
  const walk = (dir: string): string[] => readdirSync(path.join(root, dir), { withFileTypes: true }).flatMap((entry) => {
    const rel = `${dir}/${entry.name}`;
    if (entry.isDirectory()) return walk(rel);
    return entry.isFile() && entry.name.endsWith(".tsx") && !/\.test\.tsx$/.test(entry.name) ? [rel] : [];
  });
  return ["results", "comparison"].filter((dir) => existsSync(path.join(root, dir))).flatMap(walk).sort();
}

/** Per file: the unmarked `<table` count; and the marked and control counts. */
export function scanTables(root: string): { missing: Record<string, number>; marked: number; controlsShort: string[] } {
  const missing: Record<string, number> = {};
  const controlsShort: string[] = [];
  let marked = 0;
  for (const file of guardedSources(root)) {
    const text = readFileSync(path.join(root, file), "utf8");
    const tables = text.match(/<table\b[^>]*>/g) ?? [];
    const unmarked = tables.filter((tag) => !tag.includes(`${CSV_EXPORT_ATTRIBUTE}=`));
    if (unmarked.length > 0) missing[file] = unmarked.length;
    const markedHere = tables.length - unmarked.length;
    marked += markedHere;
    if ((text.match(/<CsvExportControl\b/g)?.length ?? 0) < markedHere) controlsShort.push(file);
  }
  return { missing, marked, controlsShort };
}

describe("every results and comparison table exports to CSV", () => {
  it("marks each <table> with data-csv-export, and its panel renders the export control", () => {
    const scan = scanTables(features);
    expect(scan.missing).toEqual({});
    expect(scan.controlsShort).toEqual([]);
    // The 9 tables (RT-R6), with C1's native results panel (DEL-04-05 slice 4) rendering its own
    // marked copy of two of them, the results groups and the endpoint pair, each with its control.
    expect(scan.marked).toBe(11);
  });

  it("scans nested component folders, so an unmarked table there fails (Codex review of #1726)", () => {
    const root = mkdtempSync(path.join(tmpdir(), "csv-guard-"));
    try {
      mkdirSync(path.join(root, "results", "nested"), { recursive: true });
      mkdirSync(path.join(root, "comparison"));
      writeFileSync(path.join(root, "results", "Marked.tsx"), `<CsvExportControl /><table ${CSV_EXPORT_ATTRIBUTE}="weight">`);
      writeFileSync(path.join(root, "results", "nested", "X.tsx"), "export const X = () => <table><tbody /></table>;");
      writeFileSync(path.join(root, "results", "nested", "X.test.tsx"), "<table>");
      expect(guardedSources(root)).toEqual(["results/Marked.tsx", "results/nested/X.tsx"]);
      expect(scanTables(root).missing).toEqual({ "results/nested/X.tsx": 1 });
    } finally {
      rmSync(root, { recursive: true, force: true });
    }
  });
});
