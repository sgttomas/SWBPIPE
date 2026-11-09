import families from "../../../../../fixtures/reports/invented/report_composition_families.json";
import { canonicalDecimal } from "./reportQuantityDisplay";
import type { ReportResultRow } from "./reportResultRows";

/** Session-only report settings. Values always come from the complete bound row list. */
export const REPORT_COMPOSITION_CATALOGUE = [
  { id: "result_rows_all", title: "All result rows", views: ["all"] },
  { id: "result_displacements", title: "Displacements", views: ["all", "extremes"] },
  { id: "result_forces_moments", title: "Forces and moments", views: ["all"] },
  { id: "result_stresses", title: "Stresses", views: ["all", "largest"] },
  { id: "result_support_loads", title: "Support loads", views: ["all"] },
  { id: "result_other_rows", title: "Other result rows", views: ["all"] }
] as const;

export type ReportCompositionSectionId = typeof REPORT_COMPOSITION_CATALOGUE[number]["id"];
export type ReportCompositionSection = {
  section_id: ReportCompositionSectionId;
  cases: "all" | string[];
  view: { kind: "all" | "extremes" | "largest"; limit?: number };
};
export type ReportComposition = {
  composition_version: "1.0.0";
  sections: ReportCompositionSection[];
};

/** Preserve existing canonical bytes while the default controls remain unchanged. */
export function optionalReportComposition(sections: ReportCompositionSection[]): ReportComposition | null {
  if (sections.length === 1 && sections[0].section_id === "result_rows_all" &&
      sections[0].cases === "all" && sections[0].view.kind === "all" && sections[0].view.limit === undefined) return null;
  return { composition_version: "1.0.0", sections };
}

export const DEFAULT_REPORT_COMPOSITION_SECTION: ReportCompositionSection = {
  section_id: "result_rows_all", cases: "all", view: { kind: "all" }
};

const familyByKind = new Map(Object.entries(families.sections).flatMap(([section, kinds]) =>
  kinds.map((kind) => [kind, section] as const)));

export type ComposedReportRows = {
  sections: { section_id: ReportCompositionSectionId; row_ids: string[] }[];
  shown: number;
  bound_rows: number;
  disclosure_rows: string[];
};

/** Desktop preflight/coverage mirror of the renderer. Full rows continue to IPC;
 * this projection never replaces them. Both implementations use the shared vectors. */
export function composeReportRows(rows: ReportResultRow[], composition: ReportComposition): ComposedReportRows {
  const refusals: string[] = [];
  const fail = (suffix: string) => refusals.push(`REPORT-COMPOSITION-${suffix}`);
  const disclosures: ReportResultRow[] = [];
  const bound = rows.flatMap((row, index) => {
    if (row.kind === undefined && row.value_text === undefined && row.unit === undefined && row.entity_ref == null) {
      disclosures.push(row);
      return [];
    }
    if (typeof row.kind !== "string" || typeof row.value_text !== "string" || typeof row.unit !== "string" ||
        !/^-?(0|[1-9][0-9]*)(\.[0-9]+)?([eE][+-]?[0-9]+)?$/.test(row.value_text)) {
      fail("ROW-FIELDS-INVALID");
      return [];
    }
    const magnitude = canonicalDecimal(row.value_text, row.unit === "Pa" ? 6 : 0);
    if (!Number.isSafeInteger(magnitude.point)) {
      fail("ROW-FIELDS-INVALID");
      return [];
    }
    return [{ row, index, family: familyByKind.get(row.kind) ?? "result_other_rows",
      displayUnit: row.unit === "Pa" ? "MPa" : row.unit, magnitude }];
  });
  if (refusals.length) throw new Error(refusals.join("; "));
  if (composition.composition_version !== "1.0.0") throw new Error("REPORT-COMPOSITION-VERSION-UNSUPPORTED");
  const carried = new Set(bound.map(({ row }) => row.case_ref).filter((value) => value !== "TBD"));
  const listed = new Set<string>();
  for (const section of composition.sections) {
    const entry = REPORT_COMPOSITION_CATALOGUE.find((item) => item.id === section.section_id);
    if (!entry) { fail("SECTION-UNKNOWN"); continue; }
    if (listed.has(section.section_id)) fail("SECTION-DUPLICATE");
    listed.add(section.section_id);
    if (!(entry.views as readonly string[]).includes(section.view.kind)) fail("OPTION-INVALID");
    else if (section.view.kind === "largest") {
      if (!Number.isInteger(section.view.limit) || section.view.limit! < 1 || section.view.limit! > 1000) fail("OPTION-INVALID");
    } else if (section.view.limit !== undefined) fail("OPTION-INVALID");
    if (section.cases !== "all") {
      if (!section.cases.length) fail("OPTION-INVALID");
      const seen = new Set<string>();
      for (const value of section.cases) {
        if (seen.has(value) || value === "TBD") fail("OPTION-INVALID");
        else if (!carried.has(value)) fail("CASE-UNKNOWN");
        seen.add(value);
      }
    }
  }
  if (listed.has("result_rows_all") && listed.size > 1) fail("ROWS-OVERLAP");
  if (refusals.length) throw new Error(refusals.join("; "));
  type Bound = typeof bound[number];
  const compare = (left: Bound, right: Bound) => {
    const a = left.magnitude, b = right.magnitude;
    if (!a.digits || !b.digits) return Number(Boolean(a.digits)) - Number(Boolean(b.digits));
    return a.point - b.point || (a.digits < b.digits ? -1 : a.digits > b.digits ? 1 : 0);
  };
  function group<K>(items: Bound[], key: (row: Bound) => K): Map<K, Bound[]> {
    const groups = new Map<K, Bound[]>();
    for (const item of items) {
      const k = key(item);
      if (!groups.has(k)) groups.set(k, []);
      groups.get(k)!.push(item);
    }
    return groups;
  }
  const sections = composition.sections.map((section) => {
    let selected = bound.filter((item) => (section.section_id === "result_rows_all" || item.family === section.section_id) &&
      (section.cases === "all" || section.cases.includes(item.row.case_ref)));
    if (section.view.kind === "extremes") {
      selected = [...group(selected, (item) => JSON.stringify([item.row.case_ref, item.row.label, item.displayUnit])).values()]
        .map((items) => items.reduce((best, item) => compare(item, best) > 0 ? item : best));
    } else if (section.view.kind === "largest") {
      selected = [...group(selected, (item) => item.displayUnit).values()]
        .flatMap((items) => items.sort((a, b) => compare(b, a)).slice(0, section.view.limit));
    } else if (section.section_id === "result_support_loads") {
      const entityOrder = new Map([...new Set(selected.map(({ row }) => row.entity_ref ?? null))].map((value, index) => [value, index]));
      const caseOrder = new Map([...new Set(selected.map(({ row }) => row.case_ref))].map((value, index) => [value, index]));
      selected.sort((a, b) => entityOrder.get(a.row.entity_ref ?? null)! - entityOrder.get(b.row.entity_ref ?? null)! ||
        caseOrder.get(a.row.case_ref)! - caseOrder.get(b.row.case_ref)!);
    }
    return { section_id: section.section_id, row_ids: selected.map(({ row }) => row.row_id) };
  });
  return { sections, shown: sections.reduce((total, section) => total + section.row_ids.length, 0), bound_rows: bound.length,
    disclosure_rows: disclosures.map((row) => row.row_id) };
}
