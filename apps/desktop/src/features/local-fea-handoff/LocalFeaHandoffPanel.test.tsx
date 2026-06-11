// Rider coverage for TP-SEAM-CORPUS-001 (operation-seam unification plan §3
// T1 rider; assessment §5.3): when the mechanics result summary does not
// carry location references, the Local FEA handoff package must emit an
// explicit incomplete-state finding and degrade honestly — it must never
// invent fixture entity references (the former `pipe:P-120` / `node:N-140`
// fallbacks). All fixture data below is invented.

import { render, screen, within } from "@testing-library/react";
import { describe, expect, it } from "vitest";
import type { AnalysisRunEnvelope, MechanicsResult, PreviewModel } from "../../types";
import { LocalFeaHandoffPanel } from "./LocalFeaHandoffPanel";

function inventedModel(): PreviewModel {
  return {
    schema_version: "0.1.0",
    document_kind: "openpipestress.product_preview.model",
    data_boundary: {},
    project: {
      id: "project:rider-test",
      name: "Invented rider model",
      description: "Invented panel fixture",
      units: { length: "m", force: "N", angle: "rad", pressure: "Pa", temperature: "degC" }
    },
    analysis_status: { mechanics: "ready", rule_check: "not_performed", professional_acceptance: "not_provided" },
    materials: [],
    nodes: [
      { id: "node:N-1", label: "Anchor", position: { x: 0, y: 0, z: 0 }, provenance: "invented_example" },
      { id: "node:N-2", label: "Elbow", position: { x: 3.2, y: 0, z: 0 }, provenance: "invented_example" }
    ],
    pipe_segments: [
      {
        id: "pipe:P-1",
        label: "Run",
        from: "node:N-1",
        to: "node:N-2",
        section: {
          outside_diameter: { value: 0.168, unit: "m" },
          wall_thickness: { value: 0.007, unit: "m" }
        },
        material: "material:M-1",
        provenance: "invented_example"
      }
    ],
    supports: [
      { id: "support:S-1", label: "Anchor support", node: "node:N-1", restraints: ["UX", "UY", "UZ"], provenance: "invented_example" }
    ],
    components: [],
    load_cases: [],
    diagnostics: []
  } as unknown as PreviewModel;
}

function inventedResult(summary: MechanicsResult["summary"]): MechanicsResult {
  return {
    schema_version: "0.1.0",
    document_kind: "openpipestress.product_preview.mechanics_result",
    run_id: "run:rider-test-001",
    model_ref: "project:rider-test",
    status: { mechanics: "MECHANICS_SOLVED", rule_check: "not_performed", professional_acceptance: "not_provided" },
    summary,
    results: [],
    diagnostics: []
  } as unknown as MechanicsResult;
}

function inventedAnalysisRun(): AnalysisRunEnvelope {
  return {
    analysis_run: {
      run_id: "run:rider-test-001",
      run_name: "Invented rider run",
      run_kind: "preview_linear_static",
      model_state_ref: { object_type: "ModelState", ref: "state:rider-test" },
      load_basis_refs: [],
      result_refs: [],
      hashes: [],
      analysis_status: ["MECHANICS_SOLVED"],
      reproducibility: { input_manifest_refs: [], determinism_notes: [], unresolved_tbd: [] },
      immutability_policy: {
        run_record_is_read_only: true,
        mutation_policy: "new_run_required",
        new_run_required_for_change: true,
        hash_invalidates_external_acceptance: true
      },
      professional_boundary: { human_review_required: true }
    }
  } as unknown as AnalysisRunEnvelope;
}

function exportedPacket(): string {
  const href = screen.getByTestId("local-fea-export-link").getAttribute("href") ?? "";
  return decodeURIComponent(href.replace("data:application/json;charset=utf-8,", ""));
}

describe("LocalFeaHandoffPanel result-summary boundary hygiene", () => {
  it("emits an explicit incomplete-state finding instead of invented entity references when summary location refs are missing", () => {
    render(
      <LocalFeaHandoffPanel
        model={inventedModel()}
        result={inventedResult({})}
        analysisRun={inventedAnalysisRun()}
      />
    );

    const panel = screen.getByTestId("local-fea-panel");
    const region = within(panel).getByTestId("local-fea-region").textContent ?? "";
    expect(region).toContain("local-region:stress-location-ref-missing:displacement-location-ref-missing");
    expect(region).toContain("nodes=0");
    expect(region).toContain("elements=0");
    expect(within(panel).getByTestId("local-fea-summary").textContent).toContain("diagnostics=5");

    const packet = exportedPacket();
    expect(packet).toContain("LOCAL-FEA-RESULT-SUMMARY-REF-MISSING");
    expect(packet).toContain("summary.max_open_formula_stress.location_ref");
    expect(packet).toContain("summary.max_displacement.location_ref");
    expect(packet).toContain("no substitute entity references are invented");
    // The former fixture fallbacks must not appear anywhere in the package.
    expect(packet).not.toContain("pipe:P-120");
    expect(packet).not.toContain("node:N-140");
  });

  it("keeps the standing four findings and real references when the summary carries location refs", () => {
    render(
      <LocalFeaHandoffPanel
        model={inventedModel()}
        result={inventedResult({
          max_displacement: { value: 0.004, unit: "m", location_ref: "node:N-2", result_ref: "result:disp:node-N-2" },
          max_open_formula_stress: { value: 12500000, unit: "Pa", location_ref: "pipe:P-1", result_ref: "result:stress:pipe-P-1" }
        })}
        analysisRun={inventedAnalysisRun()}
      />
    );

    const panel = screen.getByTestId("local-fea-panel");
    const region = within(panel).getByTestId("local-fea-region").textContent ?? "";
    expect(region).toContain("local-region:pipe:P-1:node:N-2");
    expect(within(panel).getByTestId("local-fea-summary").textContent).toContain("diagnostics=4");

    const packet = exportedPacket();
    expect(packet).not.toContain("LOCAL-FEA-RESULT-SUMMARY-REF-MISSING");
    expect(packet).not.toContain("pipe:P-120");
    expect(packet).not.toContain("node:N-140");
  });
});
