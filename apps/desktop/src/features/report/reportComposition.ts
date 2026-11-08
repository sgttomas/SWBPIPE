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
