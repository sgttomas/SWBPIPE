// RT-R6 CSV adapter of the displacement extremes table (DEL-08-04-R14; Design `csv-export.md`
// §2). It reads the view the panel renders (`displacementExtremesView`): every page of the
// current filter, in the view's order. It recomputes nothing and reads nothing from the page.
import type { CsvExport } from "../result-export/CsvExportControl";
import { csvExportFileName, type CsvProvenance } from "../result-export/csvTable";
import { joinColumnGroups, quantityColumns, textColumn } from "../result-export/csvValues";
import {
  DISPLACEMENT_COMPONENTS,
  type DisplacementExtremesView
} from "./displacementExtremes";

export const DISPLACEMENT_EXTREMES_CSV_KIND = "displacement-extremes";
/** The panel's text for a cell with no extreme and no note, and a node without supports. */
export const DISPLACEMENT_NO_EXTREME = "none";
export const DISPLACEMENT_NO_SUPPORT = "no support";

export function displacementExtremesCsv(input: {
  view: DisplacementExtremesView;
  provenance: CsvProvenance;
  softwareVersion: string;
  unitBasis: string;
}): CsvExport {
  const { view } = input;
  const rows = view.rows;
  const groups = [
    textColumn("Node id", rows.map((row) => row.nodeId)),
    textColumn("Node", rows.map((row) => row.label)),
    textColumn("Supports", rows.map((row) => (row.supports.length > 0 ? row.supports.map((item) => `${item.label} (${item.supportId}): ${item.kindText}`).join("; ") : DISPLACEMENT_NO_SUPPORT)))
  ];
  for (const component of DISPLACEMENT_COMPONENTS) {
    for (const which of ["max", "min"] as const) {
      const label = `${component.label} ${which}`;
      const extremes = rows.map((row) => row.extremes[component.id]);
      groups.push(quantityColumns(label, extremes.map((extreme) => {
        const value = extreme[which];
        return value ? { value: value.value, unit: value.unit } : { absent: extreme.note ?? DISPLACEMENT_NO_EXTREME };
      })));
      groups.push(textColumn(`${label} case key`, extremes.map((extreme) => extreme[which]?.caseKey ?? "")));
      groups.push(textColumn(`${label} case`, extremes.map((extreme) => extreme[which]?.caseLabel ?? "")));
      groups.push(textColumn(`${label} result id`, extremes.map((extreme) => extreme[which]?.resultId ?? "")));
    }
  }
  const { columns, rows: cells } = joinColumnGroups(rows.length, groups);
  const cases = view.chosenCases.map((item) => item.label).join("; ");
  return {
    table: {
      preamble: {
        table: DISPLACEMENT_EXTREMES_CSV_KIND,
        softwareVersion: input.softwareVersion,
        ...input.provenance,
        caseOrFilter: `nodes: ${view.filter.label}; cases: ${cases || "none chosen"}; sort: ${view.sort.component ?? "node order"}`,
        unitBasis: input.unitBasis
      },
      columns,
      rows: cells
    },
    fileName: csvExportFileName(DISPLACEMENT_EXTREMES_CSV_KIND, view.filter.id)
  };
}
