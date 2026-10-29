// §8.30 correction 3 (RT-S3): the solver's SPRING_HANGER_LOAD_NOT_APPLIED_AS_PRELOAD warnings
// reach every surface where results are read: the Diagnostics panel, the results standing line
// and the report's diagnostics. The source is the bundled demo's regenerated result, whose
// variable spring hanger support:SH-140 enters installed, cold and hot loads.
import { cleanup, render, screen } from "@testing-library/react";
import { afterEach, describe, expect, it } from "vitest";
import demoJson from "../../../../../fixtures/product_preview/invented_demo_result_preview_physics_1_sparse.json";
import { DiagnosticsPanel } from "../diagnostics/DiagnosticsPanel";
import { diagnosticsForSections } from "../report/renderableReportInput";
import type { MechanicsResult, PreviewModel } from "../../types";
import { HANGER_LOAD_NOT_APPLIED_AS_PRELOAD, hangerPreloadStanding } from "./hangerPreload";
import { ResultsPanel } from "./ResultsPanel";

afterEach(cleanup);

const demo = () => structuredClone(demoJson) as unknown as MechanicsResult;
const FIELDS = ["hanger.installed_load", "hanger.cold_load", "hanger.hot_load"];
const SENTENCE =
  "Spring hanger loads are not applied as preload: support:SH-140 (installed, cold and hot load); results treat each hanger as a linear spring from zero load.";

describe("hanger loads not applied as preload", () => {
  it("is in the demo result once per entered load, as a warning naming the support and field", () => {
    const warnings = demo().diagnostics.filter((item) => item.code === HANGER_LOAD_NOT_APPLIED_AS_PRELOAD);
    expect(warnings.map((item) => item.affected_refs)).toEqual(FIELDS.map((field) => ["support:SH-140", field]));
    expect(warnings.every((item) => item.severity === "warning")).toBe(true);
    expect(warnings[0].message).toBe(
      "variable spring hanger support:SH-140: its installed load (hanger.installed_load) is not applied as preload; results treat the hanger as a linear spring from zero load, whose force is its stiffness times the displacement at its DOF, and the entered load remains review evidence only"
    );
  });

  it("ends the results standing line, after today's text", () => {
    const result = demo();
    expect(hangerPreloadStanding(result)).toBe(SENTENCE);
    render(<ResultsPanel result={result} knowledge={null} analysisRun={null} selectedResultId={null} onSelectResult={() => {}} />);
    const standing = screen.getByTestId("numerical-result-standing").textContent ?? "";
    expect(standing.startsWith(`Numerical integrity: ${result.numerical_quality!.status}. `)).toBe(true);
    expect(standing.endsWith(` Numerical checks do not establish engineering correctness. ${SENTENCE}`)).toBe(true);
  });

  it("leaves a result without the warning unchanged", () => {
    const result = demo();
    result.diagnostics = result.diagnostics.filter((item) => item.code !== HANGER_LOAD_NOT_APPLIED_AS_PRELOAD);
    expect(hangerPreloadStanding(result)).toBeNull();
    render(<ResultsPanel result={result} knowledge={null} analysisRun={null} selectedResultId={null} onSelectResult={() => {}} />);
    expect(screen.getByTestId("numerical-result-standing").textContent).not.toContain("preload");
  });

  it("names each support once with its fields in order", () => {
    const warning = (support: string, field: string) => ({ id: `${support}:${field}`, code: HANGER_LOAD_NOT_APPLIED_AS_PRELOAD, severity: "warning", message: "m", affected_refs: [support, field] });
    const result = { diagnostics: [warning("support:B", "hanger.hot_load"), warning("support:A", "hanger.cold_load"), warning("support:B", "hanger.installed_load")] } as unknown as MechanicsResult;
    expect(hangerPreloadStanding(result)).toBe(
      "Spring hanger loads are not applied as preload: support:B (hot and installed load); support:A (cold load); results treat each hanger as a linear spring from zero load."
    );
  });

  it("is listed by the Diagnostics panel", () => {
    render(<DiagnosticsPanel model={{ diagnostics: [] } as unknown as PreviewModel} knowledge={null} result={demo()} selectedDiagnosticId={null} onSelectDiagnostic={() => {}} />);
    expect(screen.getAllByTestId(`diagnostic-${HANGER_LOAD_NOT_APPLIED_AS_PRELOAD}`)).toHaveLength(3);
  });

  it("is a report diagnostic with its message", () => {
    const entries = diagnosticsForSections(demo(), { recorded_by: "invented" } as never).filter((item) => item.code === HANGER_LOAD_NOT_APPLIED_AS_PRELOAD);
    expect(entries.map((item) => [item.class, item.severity, item.affected_object.ref_id])).toEqual(
      FIELDS.map((field) => ["ASSUMPTION_WARNING", "warning", `support:SH-140, ${field}`])
    );
    expect(entries[2].message).toContain("its hot load (hanger.hot_load) is not applied as preload");
  });
});
