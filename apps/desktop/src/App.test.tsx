import { fireEvent, render, screen, within } from "@testing-library/react";
import { describe, expect, it } from "vitest";
import { App } from "./App";

describe("OpenPipeStress desktop preview", () => {
  it("renders the engineering workspace from invented local fixtures", async () => {
    render(<App />);

    expect(await screen.findByText("OpenPipeStress Technical Preview")).toBeInTheDocument();
    expect(await screen.findByTestId("desktop-preview-shell")).toBeInTheDocument();
    expect(await screen.findByTestId("solve-panel")).toBeInTheDocument();
    expect(await screen.findByLabelText("Model tree")).toBeInTheDocument();
    expect(await screen.findByLabelText("Three.js pipe centerline viewport")).toBeInTheDocument();
    expect(screen.getByText("Human review required")).toBeInTheDocument();
    expect(screen.getByText("SQLite + FTS5")).toBeInTheDocument();
    expect(screen.getByLabelText("Local project controls")).toBeInTheDocument();
    expect(screen.getByTestId("local-project-status").textContent).toContain("network=false");
    expect(screen.getByTestId("local-project-status").textContent).toContain("telemetry=false");
  });

  it("does not claim professional or release acceptance", async () => {
    render(<App />);

    const footer = await screen.findByText(/Technical preview only/i);
    expect(footer.textContent).toContain("no production-readiness");
    expect(footer.textContent).toContain("no licensed engineering reliance claim");
  });

  it("exposes minimal local create, open, and save project controls without external file copies", async () => {
    render(<App />);

    expect(await screen.findByText("OpenPipeStress Technical Preview")).toBeInTheDocument();
    const controls = screen.getByLabelText("Local project controls");

    fireEvent.click(within(controls).getByRole("button", { name: /Create local/i }));
    expect(await screen.findByTestId("local-project-message")).toHaveTextContent("without external file copies");

    fireEvent.click(within(controls).getByRole("button", { name: /Save local/i }));
    expect(await screen.findByTestId("local-project-message")).toHaveTextContent("without external file copies");

    fireEvent.click(within(controls).getByRole("button", { name: /Open local/i }));
    expect(await screen.findByTestId("local-project-message")).toHaveTextContent("No local project snapshot found.");
  });

  it("shows computed mechanics diagnostics in results, knowledge, and review-only proposal context", async () => {
    render(<App />);

    const runButton = await screen.findByRole("button", { name: /Run mechanics preview/i });
    fireEvent.click(runButton);

    expect(await screen.findByTestId("result-group-displacement", {}, { timeout: 10000 })).toBeInTheDocument();
    const results = await screen.findByLabelText("Results");
    expect(within(results).getByTestId("result-group-displacement")).toBeInTheDocument();
    expect(within(results).getByTestId("result-group-force")).toBeInTheDocument();
    expect(within(results).getByTestId("result-group-moment")).toBeInTheDocument();
    expect(within(results).getByTestId("result-row-result:force:pipe-P-120:axial")).toBeInTheDocument();
    expect(within(results).getByTestId("result-row-result:force:pipe-P-120:axial:end-j")).toBeInTheDocument();
    expect(within(results).getByTestId("result-row-result:force:pipe-P-120:shear-y")).toBeInTheDocument();
    expect(within(results).getByTestId("result-row-result:force:pipe-P-120:shear-y:end-j")).toBeInTheDocument();
    expect(within(results).getByTestId("result-row-result:force:pipe-P-120:quarter-1:shear-y")).toBeInTheDocument();
    expect(within(results).getByTestId("result-row-result:force:pipe-P-120:midspan:axial")).toBeInTheDocument();
    expect(within(results).getByTestId("result-row-result:force:pipe-P-120:midspan:shear-z")).toBeInTheDocument();
    expect(within(results).getByTestId("result-row-result:force:pipe-P-120:quarter-3:shear-z")).toBeInTheDocument();
    expect(within(results).getByTestId("result-row-result:moment:pipe-P-120:quarter-1:bending-z")).toBeInTheDocument();
    expect(within(results).getByTestId("result-row-result:combination:combination-C-OPER-ALT:force:pipe-P-120:axial")).toBeInTheDocument();
    expect(
      within(results).getByTestId(
        "result-row-result:combination:combination-C-OPER-ALT:force:pipe-P-120:quarter-1:shear-y"
      )
    ).toBeInTheDocument();
    expect(within(results).getByTestId("result-row-result:stress:pipe-P-120")).toBeInTheDocument();
    expect(within(results).getByTestId("result-row-result:stress:pipe-P-120:end-j:torsional-shear")).toBeInTheDocument();
    expect(within(results).getByTestId("result-row-result:stress:pipe-P-120:quarter-1:torsional-shear")).toBeInTheDocument();
    expect(within(results).getByTestId("result-row-result:stress:pipe-P-120:midspan:torsional-shear")).toBeInTheDocument();
    expect(within(results).getByText("result:disp:node-N-140")).toBeInTheDocument();
    expect(within(results).getByText("result:force:pipe-P-120:axial")).toBeInTheDocument();
    expect(within(results).getByText(/26.515341 mm/i)).toBeInTheDocument();

    fireEvent.click(within(results).getByTestId("result-row-result:force:pipe-P-120:axial"));
    const detail = within(results).getByTestId("result-detail-panel");
    expect(within(detail).getByTestId("selected-result-id").textContent).toContain("result:force:pipe-P-120:axial");
    expect(within(detail).getByTestId("selected-result-component").textContent).toContain("axial_force");
    expect(within(detail).getByTestId("selected-result-coordinate-system").textContent).toContain("element_local");
    expect(within(detail).getByTestId("selected-result-location").textContent).toContain("end_i");
    expect(within(detail).getByTestId("selected-result-entity-ref").textContent).toContain("pipe:P-120");
    expect(within(detail).getByTestId("selected-result-recovery-basis").textContent).toContain("recovered_from_local_element_stiffness");
    expect(within(detail).getByTestId("selected-result-recovery-basis").textContent).toContain("load_case:load:L-100");
    expect(within(detail).getByTestId("selected-result-sign-convention").textContent).toContain("positive value follows");
    expect(within(detail).getByTestId("selected-result-source-refs").textContent).toContain("not a combined result");
    expect(within(detail).getByTestId("endpoint-pair-table").textContent).toContain("end_i");
    expect(within(detail).getByTestId("endpoint-pair-table").textContent).toContain("result:force:pipe-P-120:axial:end-j");
    expect(await screen.findByRole("heading", { name: "Rack span" })).toBeInTheDocument();

    fireEvent.click(within(results).getByTestId("result-row-result:force:pipe-P-120:axial:end-j"));
    expect(within(detail).getByTestId("selected-result-id").textContent).toContain("result:force:pipe-P-120:axial:end-j");
    expect(within(detail).getByTestId("selected-result-location").textContent).toContain("end_j");
    expect(within(detail).getByTestId("selected-result-sign-convention").textContent).toContain("j-end");
    expect(within(detail).getByTestId("endpoint-pair-table").textContent).toContain("result:force:pipe-P-120:axial");

    fireEvent.click(within(results).getByTestId("result-row-result:force:pipe-P-120:shear-y"));
    expect(within(detail).getByTestId("selected-result-id").textContent).toContain("result:force:pipe-P-120:shear-y");
    expect(within(detail).getByTestId("selected-result-component").textContent).toContain("shear_force_y");
    expect(within(detail).getByTestId("endpoint-pair-table").textContent).toContain(
      "result:force:pipe-P-120:shear-y:end-j"
    );

    fireEvent.click(within(results).getByTestId("result-row-result:force:pipe-P-120:midspan:axial"));
    expect(within(detail).getByTestId("selected-result-id").textContent).toContain(
      "result:force:pipe-P-120:midspan:axial"
    );
    expect(within(detail).getByTestId("selected-result-location").textContent).toContain("midspan");
    expect(within(detail).getByTestId("selected-result-recovery-basis").textContent).toContain(
      "interpolated_from_endpoint_resultants"
    );
    expect(within(detail).queryByTestId("endpoint-pair-table")).not.toBeInTheDocument();

    fireEvent.click(within(results).getByTestId("result-row-result:force:pipe-P-120:quarter-1:shear-y"));
    expect(within(detail).getByTestId("selected-result-id").textContent).toContain(
      "result:force:pipe-P-120:quarter-1:shear-y"
    );
    expect(within(detail).getByTestId("selected-result-component").textContent).toContain("shear_force_y");
    expect(within(detail).getByTestId("selected-result-location").textContent).toContain("quarter_1");
    expect(within(detail).getByTestId("selected-result-recovery-basis").textContent).toContain(
      "interpolated_from_endpoint_resultants"
    );
    expect(within(detail).queryByTestId("endpoint-pair-table")).not.toBeInTheDocument();

    fireEvent.click(within(results).getByTestId("result-row-result:combination:combination-C-OPER-ALT:force:pipe-P-120:axial"));
    expect(within(detail).getByTestId("selected-result-id").textContent).toContain(
      "result:combination:combination-C-OPER-ALT:force:pipe-P-120:axial"
    );
    expect(within(detail).getByTestId("selected-result-recovery-basis").textContent).toContain(
      "explicit_user_linear_combination"
    );
    expect(within(detail).getByTestId("selected-result-recovery-basis").textContent).toContain(
      "combination:combination:C-OPER-ALT"
    );
    expect(within(detail).getByTestId("selected-result-source-refs").textContent).toContain(
      "result:force:pipe-P-120:axial"
    );
    expect(within(detail).getByTestId("selected-result-source-refs").textContent).toContain(
      "result:loadcase:load-L-200:force:pipe-P-120:axial"
    );

    fireEvent.click(within(results).getByTestId("result-row-result:stress:pipe-P-120:end-j:torsional-shear"));
    expect(within(detail).getByTestId("selected-result-id").textContent).toContain(
      "result:stress:pipe-P-120:end-j:torsional-shear"
    );
    expect(within(detail).getByTestId("selected-result-component").textContent).toContain("torsional_shear_stress");
    expect(within(detail).getByTestId("selected-result-coordinate-system").textContent).toContain("element_local");
    expect(within(detail).getByTestId("selected-result-location").textContent).toContain("end_j");
    expect(within(detail).getByTestId("selected-result-entity-ref").textContent).toContain("pipe:P-120");
    expect(within(detail).getByTestId("selected-result-recovery-basis").textContent).toContain(
      "recovered_from_open_mechanics_stress_components"
    );
    expect(within(detail).getByTestId("endpoint-pair-table").textContent).toContain(
      "result:stress:pipe-P-120:end-i:torsional-shear"
    );

    const gapLedger = within(results).getByTestId("mechanics-gap-ledger");
    expect(within(gapLedger).getByTestId("gap:endpoint-j-recovery").textContent).toContain("implemented");
    expect(within(gapLedger).getByTestId("gap:endpoint-stress-components").textContent).toContain("implemented");
    expect(within(gapLedger).getByTestId("gap:station-recovery").textContent).toContain("implemented");
    expect(within(gapLedger).getByTestId("gap:station-recovery").textContent).toContain("Fixed station-grid");
    expect(within(gapLedger).getByTestId("gap:station-recovery").textContent).toContain("shear stress recovery remain deferred");
    expect(within(gapLedger).getByTestId("gap:thermal-behavior").textContent).toContain("implemented");
    expect(within(gapLedger).getByTestId("gap:load-combinations").textContent).toContain("implemented");
    expect(within(gapLedger).getByTestId("gap:endpoint-j-recovery").textContent).not.toContain("compliance failure");

    const knowledge = await screen.findByLabelText("Design knowledge");
    expect(within(knowledge).getByText(/HIGH DISPLACEMENT REVIEW/i)).toBeInTheDocument();
    expect(within(knowledge).getByText(/result:disp:node-N-140 is 26.515341 mm/i)).toBeInTheDocument();
    expect(within(knowledge).getByText(/result:force:pipe-P-120:axial is/i)).toBeInTheDocument();

    const report = await screen.findByLabelText("Report packet");
    expect(within(report).getByTestId("report-packet-body")).toBeInTheDocument();
    expect(within(report).getByTestId("report-selected-result-refs").textContent).toContain("result:disp:node-N-140");
    expect(within(report).getByTestId("report-selected-result-refs").textContent).toContain("result:force:pipe-P-120:axial");
    expect(within(report).getByTestId("report-selected-result-refs").textContent).toContain("result:force:pipe-P-120:axial:end-j");
    expect(within(report).getByTestId("report-selected-result-refs").textContent).toContain(
      "result:force:pipe-P-120:midspan:axial"
    );
    expect(within(report).getByTestId("report-selected-result-refs").textContent).toContain(
      "result:force:pipe-P-120:quarter-1:shear-y"
    );
    expect(within(report).getByTestId("report-selected-result-refs").textContent).toContain(
      "result:force:pipe-P-120:shear-y"
    );
    expect(within(report).getByTestId("report-selected-result-refs").textContent).toContain(
      "result:combination:combination-C-OPER-ALT:force:pipe-P-120:axial"
    );
    expect(within(report).getByTestId("report-selected-result-refs").textContent).toContain(
      "result:combination:combination-C-OPER-ALT:force:pipe-P-120:quarter-1:shear-y"
    );
    expect(within(report).getByTestId("report-selected-result-refs").textContent).toContain(
      "result:stress:pipe-P-120:end-j:torsional-shear"
    );
    expect(within(report).getByTestId("report-selected-result-refs").textContent).toContain(
      "result:stress:pipe-P-120:quarter-1:torsional-shear"
    );
    expect(within(report).getByTestId("report-analysis-run").textContent).toContain("DEL-14-02");
    expect(within(report).getByTestId("report-analysis-run").textContent).toContain("run:preview-linear-static-001");
    expect(within(report).getByTestId("report-load-basis-refs").textContent).toContain("load:L-100");
    expect(within(report).getByTestId("report-load-basis-refs").textContent).toContain("load:L-200");
    expect(within(report).getByTestId("report-load-basis-refs").textContent).toContain("combination:C-OPER-ALT");
    expect(within(report).getByText(/result value hashes/i)).toBeInTheDocument();
    expect(within(report).getByText(/result_envelope/i)).toBeInTheDocument();
    expect(within(report).getByText(/no compliance or professional approval claim/i)).toBeInTheDocument();

    fireEvent.click(screen.getByRole("button", { name: /Generate review proposal/i }));
    const proposal = await screen.findByLabelText("Agentic proposal");
    expect(await within(proposal).findByText("proposal:physics-diagnostic-review")).toBeInTheDocument();
    expect(within(proposal).getByTestId("selected-review-target").textContent).toContain(
      "result: result:stress:pipe-P-120:end-j:torsional-shear"
    );
    expect(within(proposal).getAllByText(/result:stress:pipe-P-120:end-j:torsional-shear/i).length).toBeGreaterThan(0);
    expect(within(proposal).getByText(/review-only and does not mutate accepted model state/i)).toBeInTheDocument();
    expect(within(proposal).getByRole("button", { name: /Accept disabled/i })).toBeDisabled();

    expect(await within(report).findByText("proposal:physics-diagnostic-review")).toBeInTheDocument();
  }, 15000);

  it("links selected diagnostics to affected result and model context", async () => {
    render(<App />);

    fireEvent.click(await screen.findByRole("button", { name: /Run mechanics preview/i }));
    expect(await screen.findByTestId("diagnostic-HIGH_DISPLACEMENT_REVIEW")).toBeInTheDocument();

    const diagnostics = await screen.findByLabelText("Diagnostics");
    fireEvent.click(within(diagnostics).getByTestId("diagnostic-HIGH_DISPLACEMENT_REVIEW"));

    const diagnosticDetail = within(diagnostics).getByTestId("diagnostic-detail-panel");
    expect(within(diagnosticDetail).getByTestId("selected-diagnostic-id").textContent).toContain(
      "diagnostic:physics:high-displacement-review"
    );
    expect(within(diagnosticDetail).getByTestId("selected-diagnostic-affected-refs").textContent).toContain(
      "result:disp:node-N-140"
    );
    expect(within(diagnosticDetail).getByTestId("selected-diagnostic-affected-refs").textContent).toContain("node:N-140");
    expect(within(diagnosticDetail).getByTestId("selected-diagnostic-linked-results").textContent).toContain(
      "result:disp:node-N-140"
    );
    expect(within(diagnosticDetail).getByTestId("selected-diagnostic-explanation").textContent).toContain(
      "review threshold"
    );
    expect(await screen.findByRole("heading", { name: "Terminal tie-in" })).toBeInTheDocument();

    fireEvent.click(screen.getByRole("button", { name: /Generate review proposal/i }));
    const proposal = await screen.findByLabelText("Agentic proposal");
    expect(await within(proposal).findByText("proposal:physics-diagnostic-review")).toBeInTheDocument();
    expect(within(proposal).getByTestId("selected-review-target").textContent).toContain(
      "diagnostic: diagnostic:physics:high-displacement-review"
    );
    expect(within(proposal).getAllByText(/diagnostic:physics:high-displacement-review/i).length).toBeGreaterThan(0);
    expect(within(proposal).getByRole("button", { name: /Accept disabled/i })).toBeDisabled();
  });
});
