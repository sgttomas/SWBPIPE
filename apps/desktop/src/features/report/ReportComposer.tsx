import type { MechanicsResult } from "../../types";
import {
  composeReportRows, DEFAULT_REPORT_COMPOSITION_SECTION, FAMILY_SECTION_IDS, optionalReportComposition,
  REPORT_COMPOSITION_CATALOGUE,
  type ReportComposition, type ReportCompositionSection
} from "./reportComposition";
import { reportResultRows } from "./reportResultRows";

export function ReportComposer({ composition, result, onChange }: {
  composition: ReportComposition | null;
  result: MechanicsResult | null;
  onChange: (value: ReportComposition | null) => void;
}) {
  const sections = composition?.sections ?? [DEFAULT_REPORT_COMPOSITION_SECTION];
  const cases = [...new Set(result?.results.flatMap((row) => row.basis_ref
    ? [`${row.basis_ref.ref_type}:${row.basis_ref.ref_id}`] : []) ?? [])];
  // Keep stale choices visible after a re-solve; the renderer refuses unknown cases.
  const chosenCases = sections.flatMap((section) => section.cases === "all" ? [] : section.cases);
  const caseOptions = [...new Set([...cases, ...chosenCases])];
  let summary: string | null = null;
  let refusal: string | null = null;
  if (result && composition) {
    try {
      const selected = composeReportRows(reportResultRows(result), composition);
      summary = `Result rows shown: ${selected.shown} of ${selected.bound_rows}.`;
    } catch (error) { refusal = error instanceof Error ? error.message : String(error); }
  }
  function update(index: number, value: Partial<ReportCompositionSection>) {
    onChange(optionalReportComposition(sections.map((section, i) => i === index ? { ...section, ...value } : section)));
  }
  function move(index: number, direction: number) {
    const ordered = [...sections];
    [ordered[index], ordered[index + direction]] = [ordered[index + direction], ordered[index]];
    onChange(optionalReportComposition(ordered));
  }
  return <fieldset data-testid="report-composer">
    <legend>Report sections</legend>
    <p>Session only. Model identity, run binding, warnings, references, limitations and the professional-review notice always remain. These controls apply to local private reports and packages. Model-input tables list the model's values as entered, in their stored units.</p>
    <div>
      {REPORT_COMPOSITION_CATALOGUE.map((entry) => <label key={entry.id}>
        <input type="checkbox" checked={sections.some((section) => section.section_id === entry.id)}
          onChange={(event) => {
            // All result rows and the family sections exclude each other; the model-input
            // sections (slice C5) show no result row and exclude nothing.
            const next = event.currentTarget.checked
              ? [...sections.filter((section) => entry.id === "result_rows_all" ? !FAMILY_SECTION_IDS.includes(section.section_id)
                : FAMILY_SECTION_IDS.includes(entry.id) ? section.section_id !== "result_rows_all" : true),
                { section_id: entry.id, cases: "all" as const, view: { kind: "all" as const } }]
              : sections.filter((section) => section.section_id !== entry.id);
            onChange(optionalReportComposition(next));
          }} />{entry.title}
      </label>)}
    </div>
    {summary ? <p role="status">{summary}</p> : null}
    {refusal ? <p role="alert">{refusal}</p> : null}
    <ol>{sections.map((section, index) => {
      const entry = REPORT_COMPOSITION_CATALOGUE.find((item) => item.id === section.section_id)!;
      return <li key={section.section_id}>
        <strong>{entry.title}</strong>{" "}
        <button type="button" aria-label={`Move ${entry.title} up`} disabled={index === 0} onClick={() => move(index, -1)}>Up</button>{" "}
        <button type="button" aria-label={`Move ${entry.title} down`} disabled={index === sections.length - 1} onClick={() => move(index, 1)}>Down</button>{" "}
        {entry.parent === "results" ? <label>Cases for {entry.title}
          <select multiple value={section.cases === "all" ? ["all"] : section.cases} onChange={(event) => {
            const selected = Array.from(event.currentTarget.selectedOptions, (option) => option.value);
            update(index, { cases: selected.includes("all") ? "all" : selected });
          }}>
            <option value="all">All cases (including unassigned rows)</option>
            {caseOptions.map((value) => <option key={value} value={value}>{value}{cases.includes(value) ? "" : " (not in current result)"}</option>)}
          </select>
        </label> : <span>Values as entered in the model, in model order</span>}{" "}
        <label>View for {entry.title}
          <select value={section.view.kind} onChange={(event) => update(index, {
            view: event.currentTarget.value === "largest" ? { kind: "largest", limit: 25 }
              : { kind: event.currentTarget.value as "all" | "extremes" }
          })}>
            {entry.views.map((view) => <option key={view} value={view}>{view}</option>)}
          </select>
        </label>{" "}
        {section.view.kind === "largest" ? <label>Rows per display unit
          <input type="number" min={1} max={1000} value={section.view.limit ?? 25}
            onChange={(event) => update(index, { view: { kind: "largest", limit: event.currentTarget.valueAsNumber } })} />
        </label> : null}
        {section.cases !== "all" && section.cases.length === 0 ? <p role="alert">Select at least one case or All cases.</p> : null}
      </li>;
    })}</ol>
    <button type="button" onClick={() => onChange(null)}>Reset report sections</button>
  </fieldset>;
}
