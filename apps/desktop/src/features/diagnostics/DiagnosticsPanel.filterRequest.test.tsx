// DEL-07-05 numerical health §3: "Show solver diagnostic" opens the Diagnostics panel
// filtered to that diagnostic; each new request applies once.
import { fireEvent, render, screen } from "@testing-library/react";
import { expect, it } from "vitest";
import { DiagnosticsPanel } from "./DiagnosticsPanel";
import type { MechanicsResult, PreviewModel } from "../../types";

const model = { diagnostics: [] } as unknown as PreviewModel;
const result = { run_id: "run:invented", summary: {}, results: [], diagnostics: [
  { id: "diagnostic:numerical-integrity:LC1", code: "NUMERICAL_INTEGRITY_SENSITIVE", severity: "warning", message: "a", affected_refs: ["LC1"] },
  { id: "diagnostic:other", code: "OTHER", severity: "info", message: "b", affected_refs: [] }
] } as unknown as MechanicsResult;

it("filters to the requested diagnostic and re-applies a later request", () => {
  const view = render(<DiagnosticsPanel model={model} knowledge={null} result={result} selectedDiagnosticId="diagnostic:numerical-integrity:LC1"
    filterRequest={{ text: "diagnostic:numerical-integrity:LC1", sequence: 1 }} onSelectDiagnostic={() => {}} />);
  expect(screen.getByTestId("diagnostic-filter-input")).toHaveValue("diagnostic:numerical-integrity:LC1");
  expect(screen.getByTestId("diagnostic-filter-summary")).toHaveTextContent("1 of 2 diagnostics visible");
  expect(screen.getByTestId("selected-diagnostic-id")).toHaveTextContent("diagnostic:numerical-integrity:LC1");
  fireEvent.change(screen.getByTestId("diagnostic-filter-input"), { target: { value: "" } });
  expect(screen.getByTestId("diagnostic-filter-summary")).toHaveTextContent("2 of 2 diagnostics visible");
  view.rerender(<DiagnosticsPanel model={model} knowledge={null} result={result} selectedDiagnosticId={null}
    filterRequest={{ text: "diagnostic:numerical-integrity:LC1", sequence: 2 }} onSelectDiagnostic={() => {}} />);
  expect(screen.getByTestId("diagnostic-filter-summary")).toHaveTextContent("1 of 2 diagnostics visible");
});
