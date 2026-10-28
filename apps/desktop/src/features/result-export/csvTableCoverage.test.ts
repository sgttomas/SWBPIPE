// RT-R6 (DEL-08-04 AC-002): the guard. Every `<table` in a non-test `.tsx` under
// `features/results` and `features/comparison` carries `data-csv-export`, the stable marker of a
// table whose panel renders a CSV export control (`CsvExportControl`). A new table without CSV
// export fails here.
import { readdirSync, readFileSync } from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, it } from "vitest";
import { CSV_EXPORT_ATTRIBUTE } from "./CsvExportControl";

const features = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..");

/** Tables still without CSV export, each named with why. The results row tables of
 * `ResultsPanel.tsx` (the per-group tables and the endpoint pair) show values through the
 * session's display-unit converter (`useDisplayQuantity`), which converts SI and US units in
 * binary64 in the Rust unit engine; their exact basis is HELP_HUMAN's open choice (RT-R6 slice 3).
 * Nothing else may be added here. */
const PENDING: Readonly<Record<string, number>> = { "results/ResultsPanel.tsx": 2 };

function sources(): string[] {
  return ["results", "comparison"].flatMap((dir) =>
    readdirSync(path.join(features, dir)).filter((name) => name.endsWith(".tsx") && !/\.test\.tsx$/.test(name)).map((name) => `${dir}/${name}`));
}

describe("every results and comparison table exports to CSV", () => {
  it("marks each <table> with data-csv-export, and its panel renders the export control", () => {
    const missing: Record<string, number> = {};
    let marked = 0;
    for (const file of sources()) {
      const text = readFileSync(path.join(features, file), "utf8");
      const tables = text.match(/<table\b[^>]*>/g) ?? [];
      const unmarked = tables.filter((tag) => !tag.includes(`${CSV_EXPORT_ATTRIBUTE}=`));
      if (unmarked.length > 0) missing[file] = unmarked.length;
      const markedHere = tables.length - unmarked.length;
      marked += markedHere;
      if (markedHere > 0) expect(text.match(/<CsvExportControl\b/g)?.length ?? 0, file).toBeGreaterThanOrEqual(markedHere === 2 && file.includes("Weight") ? 2 : 1);
    }
    expect(marked).toBeGreaterThanOrEqual(7);
    expect(missing).toEqual(PENDING);
  });

  it("finds the tables it guards (the scan reads the real sources)", () => {
    const all = sources().flatMap((file) => readFileSync(path.join(features, file), "utf8").match(/<table\b[^>]*>/g) ?? []);
    expect(all.length).toBe(9);
  });
});
