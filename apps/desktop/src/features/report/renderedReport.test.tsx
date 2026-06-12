// DEC-021 (A7) coverage: the rendered-report seam routes through the Tauri
// renderer command only (no fallback renderer in browser preview), the
// adapter composes the renderer input from session envelopes with explicit
// TBD markers, and the panel refuses save/print while export is blocked.
// All fixture data below is invented.

import { fireEvent, render, screen, waitFor } from "@testing-library/react";
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import type { AnalysisRunEnvelope, MechanicsResult, PreviewModel } from "../../types";
import { renderCalculationReport } from "../../services/reportRenderService";
import { buildRenderableReportInput } from "./renderableReportInput";
import { RenderedReportPanel } from "./RenderedReportPanel";

const invokeMock = vi.hoisted(() => vi.fn());
vi.mock("@tauri-apps/api/core", () => ({ invoke: invokeMock }));

function inventedModel(): PreviewModel {
  return {
    schema_version: "0.1.0",
    document_kind: "openpipestress.product_preview.model",
    data_boundary: {},
    project: {
      id: "project:render-test",
      name: "Invented render model",
      description: "Invented panel fixture",
      units: { length: "m", force: "N" }
    },
    analysis_status: {
      mechanics: "MECHANICS_SOLVED",
      rule_check: "not_performed",
      professional_acceptance: "not_provided"
    },
    materials: [],
    nodes: [],
    pipe_segments: [],
    supports: [],
    components: [],
    load_cases: [
      {
        id: "load_case:invented-weight",
        label: "Invented weight case",
        kind: "load_case",
        status: "ready",
        provenance: "invented_example"
      }
    ],
    diagnostics: []
  } as unknown as PreviewModel;
}

function inventedResult(): MechanicsResult {
  return {
    schema_version: "0.1.0",
    document_kind: "openpipestress.product_preview.mechanics_result",
    run_id: "run:render-test-001",
    model_ref: "project:render-test",
    status: {
      mechanics: "MECHANICS_SOLVED",
      rule_check: "RULE_INPUTS_INCOMPLETE",
      professional_acceptance: "NOT_PROVIDED"
    },
    summary: {},
    results: [
      {
        id: "result:disp:node-1",
        kind: "displacement",
        value: 0.0125,
        unit: "m",
        entity_ref: "node:N-1",
        basis_ref: { ref_type: "load_case", ref_id: "load_case:invented-weight" }
      }
    ],
    diagnostics: [
      {
        id: "diagnostic:invented-1",
        code: "INVENTED_WARNING",
        severity: "warning",
        message: "Invented diagnostic for render coverage.",
        source: "core/product_physics",
        affected_refs: ["result:disp:node-1"]
      }
    ]
  } as unknown as MechanicsResult;
}

function inventedRun(): AnalysisRunEnvelope {
  return {
    schema_version: "0.1.0",
    deliverable_id: "DEL-14-02",
    package_id: "PKG-14",
    scope_item: "SOW-072",
    objectives: ["OBJ-016"],
    run_contract_status: {},
    analysis_run: {
      run_id: "run:render-test-001",
      run_name: "invented render run",
      run_kind: "mechanics_solve",
      model_state_ref: { object_type: "ModelState", ref: "state:project:render-test:preview" },
      load_basis_refs: [{ object_type: "LoadCase", ref: "load_case:invented-weight" }],
      result_refs: [
        {
          result_ref: { object_type: "Result", ref: "result:disp:node-1" },
          result_family: "displacement",
          hash_refs: [
            {
              algorithm: "sha256",
              canonicalization: "rfc8785_jcs",
              payload_ref: { object_type: "Result", ref: "result:disp:node-1" },
              payload_scope: "result_value",
              value: "invented-result-hash"
            }
          ],
          privacy_classification: "invented_public_example"
        }
      ],
      hashes: [
        {
          algorithm: "sha256",
          canonicalization: "rfc8785_jcs",
          payload_ref: { object_type: "AnalysisRun", ref: "run:render-test-001" },
          payload_scope: "analysis_run_record",
          value: "invented-run-record-hash"
        },
        {
          algorithm: "sha256",
          canonicalization: "rfc8785_jcs",
          payload_ref: { object_type: "ResultEnvelope", ref: "result-envelope:run:render-test-001" },
          payload_scope: "result_envelope",
          value: "invented-result-envelope-hash"
        }
      ],
      analysis_status: ["HUMAN_REVIEW_REQUIRED", "MECHANICS_SOLVED", "RULE_INPUTS_INCOMPLETE"],
      reproducibility: {
        input_manifest_refs: [
          { object_type: "ResultEnvelope", ref: "result-envelope:run:render-test-001" }
        ],
        determinism_notes: [],
        unresolved_tbd: ["physical project container"]
      },
      immutability_policy: {
        run_record_is_read_only: true,
        mutation_policy: "changes_create_new_analysis_run",
        new_run_required_for_change: true,
        hash_invalidates_external_acceptance: true
      },
      professional_boundary: { human_review_required: true }
    }
  } as unknown as AnalysisRunEnvelope;
}

function unblockedOutcome() {
  return {
    html: "<!DOCTYPE html>\n<html><body>invented rendered report</body></html>",
    sha256_hex: "a".repeat(64),
    export_blocked: false,
    blocking_reasons: [],
    report_validation_diagnostics: [],
    section_validation_diagnostics: [],
    pre_render_findings: [],
    post_render_findings: [],
    derived_print_html: "<div>DERIVED VIEW aaaa</div>"
  };
}

beforeEach(() => {
  invokeMock.mockReset();
});

afterEach(() => {
  delete (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__;
});

describe("reportRenderService", () => {
  it("reports an explicit desktop-only route in browser preview without invoking", async () => {
    const route = await renderCalculationReport({ any: "input" });

    expect(route.route).toBe("unavailable_browser_preview");
    if (route.route === "unavailable_browser_preview") {
      expect(route.diagnostic).toContain("REPORT-RENDERER-DESKTOP-ONLY");
    }
    expect(invokeMock).not.toHaveBeenCalled();
  });

  it("invokes the render_calculation_report command in the Tauri runtime", async () => {
    (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__ = {};
    invokeMock.mockResolvedValue(unblockedOutcome());

    const route = await renderCalculationReport({ report_title: "invented" });

    expect(invokeMock).toHaveBeenCalledWith("render_calculation_report", {
      input: { report_title: "invented" }
    });
    expect(route.route).toBe("tauri_renderer");
  });
});

describe("buildRenderableReportInput", () => {
  it("maps session envelopes into the renderer input with explicit TBD markers", async () => {
    const input = await buildRenderableReportInput({
      model: inventedModel(),
      result: inventedResult(),
      analysisRun: inventedRun(),
      projectSummary: null
    });

    expect(input.calculation_report.report_id).toBe("report:run:render-test-001");
    expect(input.calculation_report.analysis_status).toEqual([
      "HUMAN_REVIEW_REQUIRED",
      "MECHANICS_SOLVED",
      "RULE_INPUTS_INCOMPLETE"
    ]);
    expect(input.calculation_report.model_input_summary.persistence_ref.ref_id).toBe("TBD");
    expect(input.calculation_report.model_input_summary.unit_system_ref.ref_id).toBe(
      "unit-system:dec-018-si-dual-display"
    );
    expect(input.calculation_report.model_input_summary.unit_display_summary).toEqual({
      storage_convention: "entered_units_preserved",
      model_units: { force: "N", length: "m" },
      result_units: ["m"],
      quantity_display_policy:
        "display result-row values with their explicit units; no report-time conversion",
      conversion_performed: false
    });
    expect(input.calculation_report.load_case_summary).toHaveLength(1);
    expect(input.calculation_report.audit_manifest_refs[0].checksum.value).toBe(
      "invented-run-record-hash"
    );
    expect(input.calculation_report.audit_manifest_refs[1].checksum.value).toBe(
      "invented-result-envelope-hash"
    );
    expect(input.calculation_report.template_slots).toHaveLength(8);
    expect(input.calculation_report.rendered_sections).toHaveLength(8);
    expect(input.result_rows[0]).toEqual({
      row_id: "row:result:disp:node-1",
      label: "displacement",
      case_ref: "load_case:load_case:invented-weight",
      quantity_display: "0.0125 m",
      source_ref: "result:result:disp:node-1"
    });
    expect(input.report_sections.diagnostics[0].code).toBe("INVENTED_WARNING");
    expect(
      input.report_sections.analysis_status_disclosures.some(
        (item) => item.status === "HUMAN_REVIEW_REQUIRED"
      )
    ).toBe(true);
  });

  it("uses the saved project persistence reference when a project summary exists", async () => {
    const input = await buildRenderableReportInput({
      model: inventedModel(),
      result: inventedResult(),
      analysisRun: inventedRun(),
      projectSummary: { project_id: "project:render-test" } as never
    });

    expect(input.calculation_report.model_input_summary.persistence_ref.ref_id).toBe(
      "local_sqlite:project:render-test"
    );
  });
});

describe("RenderedReportPanel", () => {
  it("requires a solve before rendering", () => {
    render(
      <RenderedReportPanel
        model={inventedModel()}
        result={null}
        analysisRun={null}
        projectSummary={null}
      />
    );

    expect(screen.getByTestId("rendered-report-render")).toBeDisabled();
    expect(screen.getByTestId("rendered-report-precondition")).toBeInTheDocument();
  });

  it("shows the canonical hash and save/print actions for an unblocked render", async () => {
    (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__ = {};
    invokeMock.mockResolvedValue(unblockedOutcome());
    render(
      <RenderedReportPanel
        model={inventedModel()}
        result={inventedResult()}
        analysisRun={inventedRun()}
        projectSummary={null}
      />
    );

    fireEvent.click(screen.getByTestId("rendered-report-render"));

    await waitFor(() => {
      expect(screen.getByTestId("rendered-report-hash")).toHaveTextContent("a".repeat(64));
    });
    expect(screen.getByTestId("rendered-report-gate")).toHaveTextContent("export gates passed");
    expect(screen.getByTestId("rendered-report-save")).toHaveAttribute(
      "download",
      `openpipestress-report-${"a".repeat(12)}.html`
    );
    expect(screen.getByTestId("rendered-report-print")).toBeInTheDocument();
    expect(screen.getByTestId("rendered-report-preview")).toBeInTheDocument();
  });

  it("refuses save and print while export is blocked and lists the reasons", async () => {
    (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__ = {};
    invokeMock.mockResolvedValue({
      ...unblockedOutcome(),
      export_blocked: true,
      blocking_reasons: ["pre-render lint: ProhibitedProfessionalClaim"]
    });
    render(
      <RenderedReportPanel
        model={inventedModel()}
        result={inventedResult()}
        analysisRun={inventedRun()}
        projectSummary={null}
      />
    );

    fireEvent.click(screen.getByTestId("rendered-report-render"));

    await waitFor(() => {
      expect(screen.getByTestId("rendered-report-gate")).toHaveTextContent("EXPORT BLOCKED");
    });
    expect(screen.getByTestId("rendered-report-blocking-reasons")).toHaveTextContent(
      "ProhibitedProfessionalClaim"
    );
    expect(screen.queryByTestId("rendered-report-save")).not.toBeInTheDocument();
    expect(screen.queryByTestId("rendered-report-print")).not.toBeInTheDocument();
  });

  it("surfaces the explicit desktop-only diagnostic in browser preview", async () => {
    render(
      <RenderedReportPanel
        model={inventedModel()}
        result={inventedResult()}
        analysisRun={inventedRun()}
        projectSummary={null}
      />
    );

    fireEvent.click(screen.getByTestId("rendered-report-render"));

    await waitFor(() => {
      expect(screen.getByTestId("rendered-report-route")).toHaveTextContent(
        "REPORT-RENDERER-DESKTOP-ONLY"
      );
    });
    expect(invokeMock).not.toHaveBeenCalled();
  });
});
