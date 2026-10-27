import { loadBundledMechanicsReference, hasNativeMechanicsInvocation } from "../../services/previewService";
import { buildAnalysisRunV03, modelLoadBasisRefs } from "../../services/analysisRunCompatibility";
import { canonicalSha256HexCheckedV1 } from "../../services/hashService";
// DEC-021 (A7) coverage: the rendered-report seam routes through the Tauri
// renderer command only (no fallback renderer in browser preview), the
// adapter composes the renderer input from session envelopes with explicit
// TBD markers, and the panel refuses save/print while export is blocked.
// All fixture data below is invented.

import { fireEvent, render, screen, waitFor } from "@testing-library/react";
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import type { AnalysisRunEnvelope, MechanicsResult, PreviewModel } from "../../types";
import { renderCalculationReport } from "../../services/reportRenderService";
import { buildRenderableReportInput, buildUnitDisplaySummary, QUANTITY_DISPLAY_POLICY, type SessionRuleCheckRun } from "./renderableReportInput";
import { RenderedReportPanel } from "./RenderedReportPanel";
import { REPORT_PACKET_QUANTITY_DISPLAY_POLICY, ReportPanel, reportUnitSystemDisclosure } from "./ReportPanel";
import { numericalHealthView, readNumericalHealth } from "../results/numericalHealth";
import { numericalResultStanding } from "../results/resultStanding";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import * as reportRedactionProjector from "./reportRedactionProjector";

const PUBLIC_CONTEXT = { exportContext: "public_report", generatedAt: "2026-10-22T12:00:00.000Z" } as const;
const LOCAL_PRIVATE_CONTEXT = { exportContext: "local_private", generatedAt: "2026-10-22T12:00:00.000Z" } as const;

/** Route evidence of a controlled DREP-IPC-003 envelope (hand-built fixtures). */
function routeEvidence(exportContext: "public_report" | "local_private", explicitIntent: boolean) {
  return {
    route_id: "DREP-IPC-003",
    export_context: exportContext,
    storage_context: exportContext,
    action: "include_metadata_only" as const,
    reason_code: exportContext === "local_private" ? "PRIVATE_LOCAL_METADATA_ALLOWED" as const : "SAFE_PUBLIC_METADATA" as const,
    blocked: false,
    metadata_only: true as const,
    explicit_local_private_intent: explicitIntent
  };
}

const invokeMock = vi.hoisted(() => vi.fn());
vi.mock("@tauri-apps/api/core", async () => ({ invoke: (await import("../../test/nativeIngress")).ingressDecodingInvoke(invokeMock) }));

function inventedModel(): PreviewModel {
  return {
    schema_version: "0.7.0",
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
    components: [
      {
        id: "component:C-110",
        label: "Invented elbow marker",
        kind: "bend",
        node: "node:N-110",
        geometry: {
          bend_geometry_source_reference: "invented_user_entered_preview_geometry"
        },
        modifiers: {
          source_reference: "invented_user_entered_preview_no_code_table"
        },
        provenance: "invented_example_user_entered_bend_values_no_code_table"
      },
      {
        id: "component:C-999",
        label: "Invented component with missing provenance",
        kind: "rigid",
        node: "node:N-999",
        provenance: ""
      }
    ],
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

/** An invented record of the one current analysis-run version (0.3.0), hand-built for render coverage. */
function inventedRun(): AnalysisRunEnvelope {
  return {
    schema_version: "0.3.0",
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
          payload_scope: "received_result",
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
  vi.restoreAllMocks();
  delete (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__;
});

describe("reportRenderService", () => {
  it("rejects a raw renderer input before browser/native routing", async () => {
    const route = await renderCalculationReport({ any: "input" });

    expect(route.route).toBe("redaction_blocked");
    if (route.route === "redaction_blocked") {
      expect(route.diagnostic).toContain("REPORT-REDACTION-CONTROL-REQUIRED");
    }
    expect(invokeMock).not.toHaveBeenCalled();
  });

  it("preserves the explicit desktop-only route for controlled browser input", async () => {
    const route = await renderCalculationReport({
      payload: { report_title: "invented", report_context: { export_context: "public_report" } },
      findings: [],
      blocked: false,
      summary: { route_id: "DREP-IPC-003", local_first: routeEvidence("public_report", false) }
    });
    expect(route.route).toBe("unavailable_browser_preview");
    if (route.route === "unavailable_browser_preview") {
      expect(route.diagnostic).toContain("REPORT-RENDERER-DESKTOP-ONLY");
    }
    expect(invokeMock).not.toHaveBeenCalled();
  });

  it("invokes the render_calculation_report command in the Tauri runtime", async () => {
    (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__ = {};
    invokeMock.mockResolvedValue(unblockedOutcome());

    const payload = { report_title: "invented", report_context: { export_context: "local_private" } };
    const route = await renderCalculationReport({
      payload,
      findings: [],
      blocked: false,
      summary: { route_id: "DREP-IPC-003", local_first: routeEvidence("local_private", true) }
    });

    expect(invokeMock).toHaveBeenCalledWith("render_calculation_report", { input: payload });
    expect(route.route).toBe("tauri_renderer");
  });

  it("refuses envelopes whose route evidence and recorded report context disagree", async () => {
    (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__ = {};
    invokeMock.mockResolvedValue(unblockedOutcome());
    const cases = [
      // Local-private evidence cannot render a document recorded as public, or the reverse.
      { evidence: routeEvidence("local_private", true), recorded: "public_report" },
      { evidence: routeEvidence("public_report", false), recorded: "local_private" },
      // Local-private evidence without explicit wrapper intent never renders.
      { evidence: routeEvidence("local_private", false), recorded: "local_private" },
      // A blocked local-first decision never renders.
      { evidence: { ...routeEvidence("local_private", true), blocked: true }, recorded: "local_private" },
      // No recorded context at all.
      { evidence: routeEvidence("public_report", false), recorded: undefined }
    ];
    for (const { evidence, recorded } of cases) {
      const route = await renderCalculationReport({
        payload: { report_title: "invented", report_context: recorded ? { export_context: recorded } : null },
        findings: [],
        blocked: false,
        summary: { route_id: "DREP-IPC-003", local_first: evidence }
      });
      expect(route.route).toBe("redaction_blocked");
      if (route.route === "redaction_blocked") expect(route.diagnostic).toContain("REPORT-REDACTION-CONTEXT-MISMATCH");
    }
    const missingEvidence = await renderCalculationReport({
      payload: { report_title: "invented", report_context: { export_context: "public_report" } },
      findings: [],
      blocked: false,
      summary: { route_id: "DREP-IPC-003" }
    });
    expect(missingEvidence.route).toBe("redaction_blocked");
    expect(invokeMock).not.toHaveBeenCalled();
  });
});

describe("buildRenderableReportInput", () => {
  it("maps session envelopes into the renderer input with explicit TBD markers", async () => {
    const input = await buildRenderableReportInput({
      model: inventedModel(),
      result: inventedResult(),
      analysisRun: inventedRun(),
      projectSummary: null,
      renderContext: PUBLIC_CONTEXT
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
        "display result-row values and stress headlines with explicit units; values stored in Pa are shown in MPa (1 MPa = 10^6 Pa, exact); displayed values are rounded to six significant digits; stored values and units are unchanged",
      // No displayed value is stored in Pa (the only result unit is m).
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
    expect(input.report_sections.user_supplied_values).toEqual([
      expect.objectContaining({
        value_id: "component-provenance:component:C-110",
        value_category: "component_provenance:bend",
        source: { ref_type: "component", ref_id: "component:C-110" },
        quantity: null,
        privacy_classification: "private_project_data",
        required_for: ["reporting", "human_review"],
        review_status: "pending",
        missing_data_finding: false
      }),
      expect.objectContaining({
        value_id: "component-provenance:component:C-999",
        value_category: "component_provenance:rigid",
        source: { ref_type: "component", ref_id: "component:C-999" },
        quantity: null,
        privacy_classification: "private_project_data",
        required_for: ["reporting", "human_review"],
        review_status: "pending",
        missing_data_finding: true
      })
    ]);
    expect(input.report_sections.provenance_notes).toEqual(
      expect.arrayContaining([
        expect.objectContaining({
          source_name: "Invented elbow marker component provenance",
          source_location: expect.stringContaining(
            "component.provenance=invented_example_user_entered_bend_values_no_code_table"
          )
        })
      ])
    );
    expect(input.report_sections.provenance_notes[1].source_location).toContain(
      "geometry.bend_geometry_source_reference=invented_user_entered_preview_geometry"
    );
    expect(input.report_sections.provenance_notes[1].source_location).toContain(
      "modifiers.source_reference=invented_user_entered_preview_no_code_table"
    );
    expect(input.result_rows[0]).toEqual({
      row_id: "row:result:disp:node-1",
      label: "displacement",
      case_ref: "load_case:load_case:invented-weight",
      quantity_display: "0.0125 m",
      source_ref: "result:result:disp:node-1"
    });
    expect(input.report_sections.diagnostics[0].code).toBe("INVENTED_WARNING");
    expect(input.report_sections.diagnostics).toEqual(
      expect.arrayContaining([
        expect.objectContaining({
          code: "COMPONENT_PROVENANCE_MISSING",
          class: "PROVENANCE_WARNING",
          severity: "warning",
          affected_object: { ref_type: "component", ref_id: "component:C-999" }
        })
      ])
    );
    expect(
      input.report_sections.analysis_status_disclosures.some(
        (item) => item.status === "HUMAN_REVIEW_REQUIRED"
      )
    ).toBe(true);
  });

  it("does not infer public classification from user-controlled invented or cleared provenance text", async () => {
    const model = inventedModel();
    model.components[0].provenance = "invented cleared user-controlled component text";
    model.components[0].geometry = {
      bend_geometry_source_reference: "cleared-looking user-controlled reference"
    };
    model.supports = [
      {
        id: "support:SH-TEXT",
        label: "Text-controlled spring hanger",
        node: "node:N-110",
        family: "variable_spring_hanger",
        restraints: [],
        provenance: "invented and cleared user-controlled support text",
        hanger: {
          hanger_type: "variable_spring_hanger",
          source_reference: "cleared-looking user-controlled hanger reference",
          stiffness: { dof: "UY", value: { value: 1, unit: "N/m" } }
        }
      }
    ];

    const input = await buildRenderableReportInput({
      model,
      result: inventedResult(),
      analysisRun: inventedRun(),
      projectSummary: null,
      renderContext: PUBLIC_CONTEXT
    });
    const component = input.report_sections.user_supplied_values.find(
      (item) => item.value_id === "component-provenance:component:C-110"
    );
    const spring = input.report_sections.user_supplied_values.find(
      (item) => item.value_id === "spring-hanger:support:SH-TEXT"
    );

    expect(component).toMatchObject({
      privacy_classification: "private_project_data",
      review_status: "pending"
    });
    expect(component?.provenance).toMatchObject({
      source_license: "user_supplied_or_private",
      redistribution_status: "private_only",
      privacy_classification: "private_project_data",
      review_status: "pending"
    });
    expect(spring).toMatchObject({
      privacy_classification: "private_project_data",
      review_status: "pending"
    });
    expect(spring?.provenance).toMatchObject({
      source_license: "user_supplied_or_private",
      redistribution_status: "private_only",
      privacy_classification: "private_project_data",
      review_status: "pending"
    });
  });

  it("uses the saved project persistence reference when a project summary exists", async () => {
    const input = await buildRenderableReportInput({
      model: inventedModel(),
      result: inventedResult(),
      analysisRun: inventedRun(),
      projectSummary: { project_id: "project:render-test" } as never,
      renderContext: PUBLIC_CONTEXT
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

  it("blocks restored user-local report data before native IPC or exposure", async () => {
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

    expect(screen.getByTestId("rendered-report-unit-basis")).toHaveTextContent(
      "unit_system=unit-system:dec-018-si-dual-display"
    );
    expect(screen.getByTestId("rendered-report-unit-basis")).toHaveTextContent("model=force=N,length=m");
    expect(screen.getByTestId("rendered-report-unit-basis")).toHaveTextContent("results=m");
    expect(screen.getByTestId("rendered-report-unit-basis")).toHaveTextContent("conversion=false");

    fireEvent.click(screen.getByTestId("rendered-report-render"));

    await waitFor(() => {
      expect(screen.getByTestId("rendered-report-redaction-summary")).toHaveTextContent("blocked=true");
    });
    expect(invokeMock).not.toHaveBeenCalled();
    expect(screen.queryByTestId("rendered-report-hash")).not.toBeInTheDocument();
    expect(screen.queryByTestId("rendered-report-save")).not.toBeInTheDocument();
    expect(screen.queryByTestId("rendered-report-print")).not.toBeInTheDocument();
    expect(screen.queryByTestId("rendered-report-preview")).not.toBeInTheDocument();
  });

  it("suppresses iframe, save, and print for an unmasked renderer-blocked outcome", async () => {
    (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__ = {};
    const projectorSpy = vi.spyOn(reportRedactionProjector, "controlReportRendererInput").mockReturnValue({
      payload: { report_title: "Invented renderer-blocked fixture", report_context: { export_context: "public_report" } },
      decisions: [],
      findings: [],
      blocked: false,
      summary: {
        decision_count: 0,
        finding_count: 0,
        redacted_count: 0,
        omitted_count: 0,
        warning_count: 0,
        blocking_count: 0,
        cloud_transmission_attempted: false,
        professional_claims_made: false,
        route_id: "DREP-IPC-003",
        materialization_withheld: false,
        local_first: routeEvidence("public_report", false)
      }
    });
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
      expect(invokeMock).toHaveBeenCalledWith("render_calculation_report", {
        input: { report_title: "Invented renderer-blocked fixture", report_context: { export_context: "public_report" } }
      });
    });
    expect(screen.getByTestId("rendered-report-redaction-summary")).toHaveTextContent("blocked=false");
    expect(screen.getByTestId("rendered-report-gate")).toHaveTextContent("EXPORT BLOCKED");
    expect(screen.queryByTestId("rendered-report-save")).not.toBeInTheDocument();
    expect(screen.queryByTestId("rendered-report-print")).not.toBeInTheDocument();
    expect(screen.queryByTestId("rendered-report-preview")).not.toBeInTheDocument();
    expect(screen.queryByTestId("rendered-report-print-frame")).not.toBeInTheDocument();
    projectorSpy.mockRestore();
  });

  it("surfaces redaction evidence before considering the browser-only route", async () => {
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
      expect(screen.getByTestId("rendered-report-redaction-summary")).toHaveTextContent("blocked=true");
    });
    expect(screen.queryByTestId("rendered-report-route")).not.toBeInTheDocument();
    expect(invokeMock).not.toHaveBeenCalled();
  });
});

// Pure received-hash inspection of preserved reference data. This constructs
// an analysis record, never a native invocation or Current/qualified export.
async function referenceAnalysis() {
  // The bundled reference carries its own demo model; the session model is not its basis.
  const reference = await loadBundledMechanicsReference(), model = reference.model;
  const result = reference.source;
  const manifest = {
    model_basis: { model_ref: model.project.id, model_payload: model },
    solver_basis: { solver_name: result.producer!.component_name, solver_version: result.producer!.component_version, solver_build_ref: "test:preserved-reference-not-native-invocation" },
  };
  const basis = { manifest, manifest_ref: { object_type: "InputManifest", ref: "test:reference-analysis-inputs" }, manifest_sha256: await canonicalSha256HexCheckedV1(manifest) };
  const analysisRun = await buildAnalysisRunV03(result, basis, undefined, modelLoadBasisRefs(model));
  const receivedHash = analysisRun.analysis_run.hashes.find(hash => hash.payload_scope === "received_result")!.value;
  expect(reference.standing).toBe("reference_only");
  expect(reference.provenance.current_use_eligible).toBe(false);
  expect(hasNativeMechanicsInvocation(result, model)).toBe(false);
  expect(analysisRun.schema_version).toBe("0.3.0");
  return { model, result, analysisRun, receivedHash };
}

describe("renderer received-result hash binding", () => {
  it("uses the reference received hash and rejects unknown analysis versions", async () => {
    const session = await referenceAnalysis();
    const input = await buildRenderableReportInput({ ...session, projectSummary: null, renderContext: PUBLIC_CONTEXT });
    expect(input.calculation_report.audit_manifest_refs[1].checksum.value).toBe(session.receivedHash);
    const future = structuredClone(session.analysisRun);
    future.schema_version = "0.4.0";
    await expect(buildRenderableReportInput({ ...session, analysisRun: future, projectSummary: null, renderContext: PUBLIC_CONTEXT })).rejects.toThrow("REPORT-ANALYSIS-VERSION-UNSUPPORTED");
  });
});

// W-3 (DEL-12-02 REXC-REQ-006; PRD §19.2): local-private rendered report and
// the §19.2 report-context content. All data invented.

function renderPanel(props: Partial<Parameters<typeof RenderedReportPanel>[0]> = {}) {
  return render(
    <RenderedReportPanel
      model={inventedModel()}
      result={inventedResult()}
      analysisRun={inventedRun()}
      projectSummary={null}
      {...props}
    />
  );
}

function renderedInput(): Record<string, unknown> {
  const call = invokeMock.mock.calls.find(([command]) => command === "render_calculation_report");
  expect(call).toBeDefined();
  return (call![1] as { input: Record<string, unknown> }).input;
}

async function inventedRuleCheckRun(aggregate: "RULE_INPUTS_INCOMPLETE" | "USER_RULE_CHECKED" = "RULE_INPUTS_INCOMPLETE"): Promise<SessionRuleCheckRun> {
  const { loadDemoRuleCheckPack } = await import("../../services/ruleCheckService");
  const rulePackDocument = await loadDemoRuleCheckPack();
  return {
    rulePackDocument,
    result: {
      document_kind: "openpipestress.rule_check_run_result",
      rule_pack_id: "invented_demo_rule_pack",
      grammar_version: "1.0.0",
      aggregate_status: aggregate,
      checks: [
        {
          check_id: "demo_training_check",
          status: aggregate,
          computed_value: { value: 0.123456, dimension: "dimensionless", unit_ref: "ratio" },
          limit_value: { value: 0.987654, dimension: "dimensionless", unit_ref: "ratio" },
          acceptability_relation: "invented_relation_text",
          bound_inputs: [
            { input_id: "demo_actual_quantity", source_kind: "solver_result", supplied: true, value: 42.4242, unit: "demo_unit" },
            { input_id: "demo_limit_quantity", source_kind: "user_value", supplied: aggregate !== "RULE_INPUTS_INCOMPLETE" }
          ],
          completeness_findings: [],
          evaluator_findings: [],
          diagnostic_codes: []
        }
      ],
      professional_boundary_notice: "Invented notice."
    }
  };
}

describe("local-private rendered report (W-3)", () => {
  it("keeps the unselected render in the public-report context, blocked for private input", async () => {
    (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__ = {};
    invokeMock.mockResolvedValue(unblockedOutcome());
    renderPanel();

    expect(screen.getByTestId("rendered-report-local-private-intent")).not.toBeChecked();
    fireEvent.click(screen.getByTestId("rendered-report-render"));

    await waitFor(() => {
      expect(screen.getByTestId("rendered-report-redaction-summary")).toHaveTextContent("blocked=true");
    });
    expect(screen.getByTestId("rendered-report-redaction-summary")).toHaveTextContent(
      "route=DREP-IPC-003; context=public_report; explicit_intent=false"
    );
    expect(invokeMock).not.toHaveBeenCalled();
    expect(screen.queryByTestId("rendered-report-local-private-warning")).not.toBeInTheDocument();
    expect(screen.queryByTestId("rendered-report-preview")).not.toBeInTheDocument();
  });

  it("renders under explicit local-private intent with warning evidence and controlled sinks", async () => {
    (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__ = {};
    invokeMock.mockResolvedValue(unblockedOutcome());
    renderPanel();

    fireEvent.click(screen.getByTestId("rendered-report-local-private-intent"));
    fireEvent.click(screen.getByTestId("rendered-report-render"));

    await waitFor(() => expect(screen.getByTestId("rendered-report-preview")).toBeInTheDocument());
    const summary = screen.getByTestId("rendered-report-redaction-summary");
    expect(summary).toHaveTextContent("route=DREP-IPC-003; context=local_private; explicit_intent=true");
    expect(summary).toHaveTextContent("blocking=0; blocked=false");
    expect(summary.textContent).toMatch(/warnings=[1-9]/);
    expect(screen.getByTestId("rendered-report-local-private-warning")).toHaveTextContent(
      "contains private project values for your own local review. It is not for sharing"
    );
    const input = renderedInput();
    expect(input.report_context).toMatchObject({ export_context: "local_private" });
    expect(screen.getByTestId("rendered-report-preview")).toHaveAttribute("srcdoc", unblockedOutcome().html);

    // The save sink is the local-private HTML route under the panel's own intent;
    // it shows no second intent control and the public save link is absent.
    expect(screen.queryByTestId("rendered-report-save")).not.toBeInTheDocument();
    const save = screen.getByTestId("rendered-report-save-local-private");
    expect(save.closest("[data-route-id]")).toHaveAttribute("data-route-id", "DREP-HTML-SAVE-005");
    expect(save).toHaveAttribute("href", `data:text/html;charset=utf-8,${encodeURIComponent(unblockedOutcome().html)}`);
    expect(screen.queryByTestId("rendered-report-save-local-private-local-private-intent")).not.toBeInTheDocument();
    expect(screen.getByTestId("rendered-report-save-local-private-redaction-summary")).toHaveTextContent("blocked=false");

    fireEvent.click(screen.getByTestId("rendered-report-print"));
    expect(screen.getByTestId("rendered-report-print-frame")).toHaveAttribute(
      "srcdoc",
      unblockedOutcome().derived_print_html
    );
  });

  it("clears a shown private render when the selection is turned off and never reuses it publicly", async () => {
    (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__ = {};
    invokeMock.mockResolvedValue(unblockedOutcome());
    renderPanel();

    fireEvent.click(screen.getByTestId("rendered-report-local-private-intent"));
    fireEvent.click(screen.getByTestId("rendered-report-render"));
    await waitFor(() => expect(screen.getByTestId("rendered-report-preview")).toBeInTheDocument());
    fireEvent.click(screen.getByTestId("rendered-report-print"));

    fireEvent.click(screen.getByTestId("rendered-report-local-private-intent"));

    expect(screen.getByTestId("rendered-report-local-private-intent")).not.toBeChecked();
    for (const id of [
      "rendered-report-preview",
      "rendered-report-print-frame",
      "rendered-report-print",
      "rendered-report-save-local-private",
      "rendered-report-save",
      "rendered-report-hash",
      "rendered-report-local-private-warning",
      "rendered-report-redaction-summary"
    ]) {
      expect(screen.queryByTestId(id)).not.toBeInTheDocument();
    }
    // Re-selecting does not resurrect the discarded render either.
    fireEvent.click(screen.getByTestId("rendered-report-local-private-intent"));
    expect(screen.queryByTestId("rendered-report-preview")).not.toBeInTheDocument();
  });

  it("discards an in-flight private render when the selection changes before it returns", async () => {
    (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__ = {};
    let resolveRender: (value: unknown) => void = () => undefined;
    invokeMock.mockImplementation(() => new Promise((resolve) => { resolveRender = resolve; }));
    renderPanel();

    fireEvent.click(screen.getByTestId("rendered-report-local-private-intent"));
    fireEvent.click(screen.getByTestId("rendered-report-render"));
    await waitFor(() => expect(invokeMock).toHaveBeenCalled());
    fireEvent.click(screen.getByTestId("rendered-report-local-private-intent"));
    resolveRender(unblockedOutcome());
    await new Promise((resolve) => setTimeout(resolve, 0));

    expect(screen.queryByTestId("rendered-report-preview")).not.toBeInTheDocument();
    expect(screen.queryByTestId("rendered-report-hash")).not.toBeInTheDocument();
    expect(screen.getByTestId("rendered-report-render")).not.toBeDisabled();
  });

  it("exposes no sink for a renderer-blocked local-private outcome", async () => {
    (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__ = {};
    invokeMock.mockResolvedValue({
      ...unblockedOutcome(),
      export_blocked: true,
      blocking_reasons: ["pre-render lint: ProhibitedProfessionalClaim"]
    });
    renderPanel();

    fireEvent.click(screen.getByTestId("rendered-report-local-private-intent"));
    fireEvent.click(screen.getByTestId("rendered-report-render"));

    await waitFor(() => expect(screen.getByTestId("rendered-report-gate")).toHaveTextContent("EXPORT BLOCKED"));
    for (const id of [
      "rendered-report-preview",
      "rendered-report-print",
      "rendered-report-print-frame",
      "rendered-report-save-local-private",
      "rendered-report-save"
    ]) {
      expect(screen.queryByTestId(id)).not.toBeInTheDocument();
    }
  });

  it("blocks a local-private render before IPC when a boundary finding blocks the input", async () => {
    (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__ = {};
    invokeMock.mockResolvedValue(unblockedOutcome());
    const real = await buildRenderableReportInput({
      model: inventedModel(),
      result: inventedResult(),
      analysisRun: inventedRun(),
      projectSummary: null,
      renderContext: LOCAL_PRIVATE_CONTEXT
    });
    const inputModule = await import("./renderableReportInput");
    const builderSpy = vi.spyOn(inputModule, "buildRenderableReportInput").mockResolvedValue({
      ...real,
      report_title: { field_id: "invented-title", value: "Invented title", professional_claim: true }
    } as unknown as typeof real);
    renderPanel();

    fireEvent.click(screen.getByTestId("rendered-report-local-private-intent"));
    fireEvent.click(screen.getByTestId("rendered-report-render"));

    await waitFor(() => {
      expect(screen.getByTestId("rendered-report-redaction-summary")).toHaveTextContent("blocked=true");
    });
    expect(screen.getByTestId("rendered-report-redaction-summary")).toHaveTextContent("context=local_private");
    expect(invokeMock).not.toHaveBeenCalled();
    expect(screen.queryByTestId("rendered-report-local-private-warning")).not.toBeInTheDocument();
    expect(screen.queryByTestId("rendered-report-preview")).not.toBeInTheDocument();
    builderSpy.mockRestore();
  });

  it("ignores source-carried intent: only the wrapper's selection opens the local-private route", async () => {
    const input = await buildRenderableReportInput({
      model: inventedModel(),
      result: inventedResult(),
      analysisRun: inventedRun(),
      projectSummary: null,
      renderContext: LOCAL_PRIVATE_CONTEXT
    });
    const carrying = {
      ...input,
      local_private_intent: true,
      explicit_local_private_intent: true,
      user_intent: "explicit_local_private",
      report_context: { ...input.report_context, explicit_local_private_intent: true }
    };
    const snapshot = structuredClone(carrying);

    const unselected = reportRedactionProjector.controlReportRendererInput(carrying);
    expect(unselected.blocked).toBe(true);
    expect(unselected.payload).toBeNull();
    expect(unselected.summary.local_first?.export_context).toBe("public_report");

    const selected = reportRedactionProjector.controlReportRendererInput(carrying, true);
    expect(selected.blocked).toBe(false);
    expect(selected.summary.local_first).toMatchObject({
      export_context: "local_private",
      explicit_local_private_intent: true
    });
    expect(selected.payload).not.toHaveProperty("local_private_intent");
    expect(selected.payload).not.toHaveProperty("explicit_local_private_intent");
    expect(selected.payload).not.toHaveProperty("user_intent");
    expect((selected.payload as typeof input).report_context).not.toHaveProperty("explicit_local_private_intent");
    expect(selected.payload).toMatchObject({ report_title: input.report_title });
    expect(carrying).toEqual(snapshot);

    // The local-private HTML sink needs the same wrapper intent.
    expect(reportRedactionProjector.controlLocalPrivateRenderedHtml("<p>invented</p>", false).blocked).toBe(true);
    const html = reportRedactionProjector.controlLocalPrivateRenderedHtml("<p>invented</p>", true);
    expect(html.blocked).toBe(false);
    expect(html.payload).toBe("<p>invented</p>");
    expect(html.summary.route_id).toBe("DREP-HTML-SAVE-005");
  });
});

describe("PRD §19.2 report context (W-3)", () => {
  it("sources software, solver, run, settings, date and coordinate content from session data", async () => {
    const { buildCurrentSessionInputManifest } = await import("../../services/inputManifestService");
    const model = inventedModel();
    const manifest = await buildCurrentSessionInputManifest({
      model,
      solver: {
        solver_name: "invented_solver",
        solver_version: "0.2.0",
        solver_build_ref: "invented_solver@0.2.0",
        solver_mode: "sparse_interactive",
        settings: { sparse_evidence_lane: true, invented_policy: "invented-policy-v1" }
      },
      active_rule_packs: [],
      external_assets: []
    });
    const run = inventedRun();
    run.analysis_run.solver_version = {
      solver_name: "invented_solver",
      solver_version: "0.2.0",
      build_ref: { object_type: "ExternalReference", ref: "invented_solver@0.2.0" }
    };
    run.analysis_run.settings_ref = { object_type: "SolverSettings", ref: "solver-settings:invented" };
    run.analysis_run.created_at = null;
    run.analysis_run.reproducibility.input_manifest_refs = [structuredClone(manifest.manifest_ref)];
    run.analysis_run.reproducibility.input_manifest_hashes = [{
      algorithm: "sha256",
      canonicalization: "rfc8785_jcs",
      payload_ref: structuredClone(manifest.manifest_ref),
      payload_scope: "input_manifest",
      value: manifest.manifest_sha256
    }];
    const result = inventedResult();
    result.results[0].metadata = {
      component: "nodal_force_x",
      coordinate_system: "global",
      location: "node",
      basis: "solved_from_global_linear_system",
      sign_convention: "invented"
    };

    const input = await buildRenderableReportInput({
      model,
      result,
      analysisRun: run,
      projectSummary: null,
      inputManifest: manifest,
      renderContext: LOCAL_PRIVATE_CONTEXT
    });

    expect(input.report_context).toEqual({
      export_context: "local_private",
      generated_at: "2026-10-22T12:00:00.000Z",
      software_name: "SWBPIPE",
      software_version: "0.1.0",
      build_release_identifier: null,
      solver_name: "invented_solver",
      solver_version: "0.2.0",
      solver_build_ref: "invented_solver@0.2.0",
      analysis_run_name: "invented render run",
      analysis_run_recorded_at: null,
      solver_mode: "sparse_interactive",
      solver_settings_ref: "solver-settings:invented",
      solver_settings: [
        { key: "invented_policy", value: "\"invented-policy-v1\"" },
        { key: "sparse_evidence_lane", value: "true" }
      ],
      model_coordinate_system: null,
      result_coordinate_systems: ["global"]
    });
    expect(input.user_rule_checks).toBeNull();
    expect(input.calculation_report.rule_pack_refs).toEqual([]);

    // A manifest the run does not bind is not a settings source.
    const unbound = await buildRenderableReportInput({
      model,
      result,
      analysisRun: inventedRun(),
      projectSummary: null,
      inputManifest: manifest,
      renderContext: PUBLIC_CONTEXT
    });
    expect(unbound.report_context).toMatchObject({
      export_context: "public_report",
      solver_mode: null,
      solver_settings: [],
      solver_name: null
    });
  });

  it("limits rule-pack content to safe metadata and per-check identity, status and missing inputs", async () => {
    const ruleCheckRun = await inventedRuleCheckRun();
    const input = await buildRenderableReportInput({
      model: inventedModel(),
      result: inventedResult(),
      analysisRun: inventedRun(),
      projectSummary: null,
      ruleCheckRun,
      renderContext: LOCAL_PRIVATE_CONTEXT
    });

    expect(input.calculation_report.rule_pack_refs).toHaveLength(1);
    const ref = input.calculation_report.rule_pack_refs[0];
    expect(Object.keys(ref).sort()).toEqual([
      "checksum",
      "completeness_status",
      "name",
      "private_payload_redacted",
      "redistribution_status",
      "rule_pack_id",
      "source_notice",
      "version"
    ]);
    expect(ref).toMatchObject({
      rule_pack_id: "invented_demo_rule_pack",
      // Display name from the unchanged demo fixture's metadata.name.
      name: "Invented Demonstration Rule Pack",
      version: "0.3.0",
      checksum: {
        algorithm: "sha256",
        value: "c96d28d94a1d4eee99ae2f501c3d4d5748fb81bfa7feb034cc8d96d60651e68f"
      },
      redistribution_status: "invented_non_engineering_example",
      completeness_status: "incomplete",
      private_payload_redacted: true
    });
    expect(input.user_rule_checks).toEqual({
      rule_pack_id: "invented_demo_rule_pack",
      grammar_version: "1.0.0",
      aggregate_status: "RULE_INPUTS_INCOMPLETE",
      checks: [{ check_id: "demo_training_check", status: "RULE_INPUTS_INCOMPLETE", missing_inputs: ["demo_limit_quantity"] }]
    });
    const serialized = JSON.stringify(input);
    for (const excluded of [
      "declaration_payload",
      "demo_ratio_expression",
      "demo_limit_slot",
      "invented_relation_text",
      "0.123456",
      "0.987654",
      "42.4242",
      "formula"
    ]) {
      expect(serialized).not.toContain(excluded);
    }

    const mismatched = { ...ruleCheckRun, result: { ...ruleCheckRun.result, rule_pack_id: "invented_other_pack" } };
    await expect(buildRenderableReportInput({
      model: inventedModel(),
      result: inventedResult(),
      analysisRun: inventedRun(),
      projectSummary: null,
      ruleCheckRun: mismatched,
      renderContext: LOCAL_PRIVATE_CONTEXT
    })).rejects.toThrow("REPORT-RULE-CHECK-PACK-MISMATCH");
  });

  it("projects each outcome's load case or combination, one per check and basis (rule-pack format 0.5.0)", async () => {
    const ruleCheckRun = await inventedRuleCheckRun();
    const [outcome] = ruleCheckRun.result.checks;
    ruleCheckRun.result.checks = [
      { ...outcome, category_ref: "demo_category_a", basis_ref: { ref_type: "load_case", ref_id: "load:L-100" }, status: "USER_RULE_CHECKED" },
      { ...outcome, category_ref: "demo_category_a", basis_ref: { ref_type: "load_case", ref_id: "load:L-200" } }
    ];
    const input = await buildRenderableReportInput({
      model: inventedModel(),
      result: inventedResult(),
      analysisRun: inventedRun(),
      projectSummary: null,
      ruleCheckRun,
      renderContext: LOCAL_PRIVATE_CONTEXT
    });
    expect(input.user_rule_checks?.checks).toEqual([
      { check_id: "demo_training_check", basis_ref: { ref_type: "load_case", ref_id: "load:L-100" }, status: "USER_RULE_CHECKED", missing_inputs: ["demo_limit_quantity"] },
      { check_id: "demo_training_check", basis_ref: { ref_type: "load_case", ref_id: "load:L-200" }, status: "RULE_INPUTS_INCOMPLETE", missing_inputs: ["demo_limit_quantity"] }
    ]);
  });

  it("marks required inputs incomplete when any check lacks inputs, even under a failed aggregate", async () => {
    const base = await inventedRuleCheckRun("RULE_INPUTS_INCOMPLETE");
    const [incomplete] = base.result.checks;
    const mixed: SessionRuleCheckRun = {
      rulePackDocument: base.rulePackDocument,
      result: {
        ...base.result,
        // The runner ranks a failed check above an incomplete one.
        aggregate_status: "USER_RULE_FAILED",
        checks: [
          { ...incomplete, check_id: "demo_failed_check", status: "USER_RULE_FAILED", bound_inputs: [] },
          incomplete
        ]
      }
    };
    const input = await buildRenderableReportInput({
      model: inventedModel(),
      result: inventedResult(),
      analysisRun: inventedRun(),
      projectSummary: null,
      ruleCheckRun: mixed,
      renderContext: LOCAL_PRIVATE_CONTEXT
    });
    expect(input.calculation_report.rule_pack_refs[0].completeness_status).toBe("incomplete");
    expect(input.user_rule_checks?.checks.map((check) => check.status)).toEqual([
      "USER_RULE_FAILED",
      "RULE_INPUTS_INCOMPLETE"
    ]);

    const allSupplied: SessionRuleCheckRun = {
      rulePackDocument: base.rulePackDocument,
      result: { ...mixed.result, checks: [mixed.result.checks[0]] }
    };
    const complete = await buildRenderableReportInput({
      model: inventedModel(),
      result: inventedResult(),
      analysisRun: inventedRun(),
      projectSummary: null,
      ruleCheckRun: allSupplied,
      renderContext: LOCAL_PRIVATE_CONTEXT
    });
    expect(complete.calculation_report.rule_pack_refs[0].completeness_status).toBe("complete");
  });

  it("sends the session rule-check run only when the analysis record carries its aggregate", async () => {
    (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__ = {};
    invokeMock.mockResolvedValue(unblockedOutcome());
    const { unmount } = renderPanel({ ruleCheckRun: await inventedRuleCheckRun("RULE_INPUTS_INCOMPLETE") });
    fireEvent.click(screen.getByTestId("rendered-report-local-private-intent"));
    fireEvent.click(screen.getByTestId("rendered-report-render"));
    await waitFor(() => expect(screen.getByTestId("rendered-report-preview")).toBeInTheDocument());
    expect(renderedInput().user_rule_checks).toMatchObject({ aggregate_status: "RULE_INPUTS_INCOMPLETE" });
    unmount();

    invokeMock.mockClear();
    // The invented run records RULE_INPUTS_INCOMPLETE, not USER_RULE_CHECKED.
    renderPanel({ ruleCheckRun: await inventedRuleCheckRun("USER_RULE_CHECKED") });
    fireEvent.click(screen.getByTestId("rendered-report-local-private-intent"));
    fireEvent.click(screen.getByTestId("rendered-report-render"));
    await waitFor(() => expect(screen.getByTestId("rendered-report-preview")).toBeInTheDocument());
    expect(renderedInput().user_rule_checks).toBeNull();
    expect((renderedInput().calculation_report as { rule_pack_refs: unknown[] }).rule_pack_refs).toEqual([]);
  });
});

describe("pending render work is owned by the panel instance and its source (W-3 repair)", () => {
  async function pendingPreparation() {
    const real = await buildRenderableReportInput({
      model: inventedModel(),
      result: inventedResult(),
      analysisRun: inventedRun(),
      projectSummary: null,
      renderContext: LOCAL_PRIVATE_CONTEXT
    });
    let release: () => void = () => undefined;
    const gate = new Promise<void>((resolve) => { release = resolve; });
    const inputModule = await import("./renderableReportInput");
    const builderSpy = vi.spyOn(inputModule, "buildRenderableReportInput").mockImplementation(async () => {
      await gate;
      return real;
    });
    return { release: () => release(), builderSpy };
  }

  it("never sends a private render to IPC after the panel unmounts during preparation", async () => {
    (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__ = {};
    invokeMock.mockResolvedValue(unblockedOutcome());
    const { release, builderSpy } = await pendingPreparation();
    const { unmount } = renderPanel();

    fireEvent.click(screen.getByTestId("rendered-report-local-private-intent"));
    fireEvent.click(screen.getByTestId("rendered-report-render"));
    await waitFor(() => expect(builderSpy).toHaveBeenCalled());
    unmount();
    release();
    await new Promise((resolve) => setTimeout(resolve, 0));
    await new Promise((resolve) => setTimeout(resolve, 0));

    expect(invokeMock).not.toHaveBeenCalledWith("render_calculation_report", expect.anything());
    builderSpy.mockRestore();
  });

  it("never sends a private render to IPC after its source is replaced during preparation", async () => {
    (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__ = {};
    invokeMock.mockResolvedValue(unblockedOutcome());
    const { release, builderSpy } = await pendingPreparation();
    const model = inventedModel();
    const result = inventedResult();
    const { rerender } = render(
      <RenderedReportPanel model={model} result={result} analysisRun={inventedRun()} projectSummary={null} />
    );

    fireEvent.click(screen.getByTestId("rendered-report-local-private-intent"));
    fireEvent.click(screen.getByTestId("rendered-report-render"));
    await waitFor(() => expect(builderSpy).toHaveBeenCalled());
    // A replacement analysis record (same panel instance) retires the pending render.
    rerender(<RenderedReportPanel model={model} result={result} analysisRun={inventedRun()} projectSummary={null} />);
    release();
    await new Promise((resolve) => setTimeout(resolve, 0));
    await new Promise((resolve) => setTimeout(resolve, 0));

    expect(invokeMock).not.toHaveBeenCalledWith("render_calculation_report", expect.anything());
    expect(screen.queryByTestId("rendered-report-redaction-summary")).not.toBeInTheDocument();
    expect(screen.queryByTestId("rendered-report-preview")).not.toBeInTheDocument();
    expect(screen.getByTestId("rendered-report-render")).not.toBeDisabled();
    builderSpy.mockRestore();
  });

  it("retires a shown render when its source is replaced", async () => {
    (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__ = {};
    invokeMock.mockResolvedValue(unblockedOutcome());
    const model = inventedModel();
    const result = inventedResult();
    const { rerender } = render(
      <RenderedReportPanel model={model} result={result} analysisRun={inventedRun()} projectSummary={null} />
    );
    fireEvent.click(screen.getByTestId("rendered-report-local-private-intent"));
    fireEvent.click(screen.getByTestId("rendered-report-render"));
    await waitFor(() => expect(screen.getByTestId("rendered-report-preview")).toBeInTheDocument());

    rerender(<RenderedReportPanel model={model} result={inventedResult()} analysisRun={inventedRun()} projectSummary={null} />);

    expect(screen.queryByTestId("rendered-report-preview")).not.toBeInTheDocument();
    expect(screen.queryByTestId("rendered-report-save-local-private")).not.toBeInTheDocument();
  });

  // W-3 repair round 2 (Codex): a headroom refusal is re-sent by the desktop
  // (nativeIngress.ts); a request retired during that pause is never re-sent.
  const headroomRefusal = { code: "RESOURCE_UNAVAILABLE", refusal: { kind: "headroom_insufficient", required_bytes: 2 ** 24, available_bytes: 0 }, pool: null };
  const renderCalls = () => invokeMock.mock.calls.filter(([command]) => command === "render_calculation_report").length;
  async function firstSendRefused() {
    (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__ = {};
    invokeMock.mockImplementation((command: string) =>
      command === "render_calculation_report" && renderCalls() === 1 ? Promise.reject(headroomRefusal) : Promise.resolve(unblockedOutcome()));
    const view = renderPanel();
    fireEvent.click(screen.getByTestId("rendered-report-local-private-intent"));
    fireEvent.click(screen.getByTestId("rendered-report-render"));
    await waitFor(() => expect(renderCalls()).toBe(1));
    return view;
  }
  const afterResendWindow = () => new Promise((resolve) => setTimeout(resolve, 400));

  it("never re-sends a private render after the selection is revoked during a headroom re-send pause", async () => {
    await firstSendRefused();
    fireEvent.click(screen.getByTestId("rendered-report-local-private-intent"));
    await afterResendWindow();
    expect(renderCalls()).toBe(1);
    expect(screen.queryByTestId("rendered-report-preview")).not.toBeInTheDocument();
  });

  it("never re-sends a private render after the panel unmounts during a headroom re-send pause", async () => {
    const { unmount } = await firstSendRefused();
    unmount();
    await afterResendWindow();
    expect(renderCalls()).toBe(1);
  });

  it("never re-sends a private render after its source is replaced during a headroom re-send pause", async () => {
    const { rerender } = await firstSendRefused();
    rerender(<RenderedReportPanel model={inventedModel()} result={inventedResult()} analysisRun={inventedRun()} projectSummary={null} />);
    await afterResendWindow();
    expect(renderCalls()).toBe(1);
    expect(screen.queryByTestId("rendered-report-preview")).not.toBeInTheDocument();
  });

  it("re-sends a current private render after a headroom refusal and shows it", async () => {
    await firstSendRefused();
    await waitFor(() => expect(screen.getByTestId("rendered-report-preview")).toBeInTheDocument(), { timeout: 3000 });
    expect(renderCalls()).toBe(2);
  });

  it("refuses a retired request immediately before IPC", async () => {
    (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__ = {};
    invokeMock.mockResolvedValue(unblockedOutcome());
    const route = await renderCalculationReport(
      {
        payload: { report_title: "invented", report_context: { export_context: "local_private" } },
        findings: [],
        blocked: false,
        summary: { route_id: "DREP-IPC-003", local_first: routeEvidence("local_private", true) }
      },
      { stillCurrent: () => false }
    );
    expect(route.route).toBe("redaction_blocked");
    expect(invokeMock).not.toHaveBeenCalled();
  });
});

// DEL-07-05 numerical health §5: one line per load case before the diagnostics section
// in the rendered report's emitted renderer input, on the generic route only; the
// report package's input (built without the indicator) is unchanged; no conditioning
// figure is exported. The indicators are the wasm export's readings of committed
// producer outputs. The Rust renderer itself runs only in the native shell (a Tauri
// command), so its HTML is not reachable from Vitest.
describe("numerical health report line", () => {
  const fixture = (path: string) => JSON.parse(readFileSync(resolve(__dirname, "../../../../../fixtures", path), "utf8")) as MechanicsResult;
  const generic = () => fixture("results/preview_physics_unicode_ids_sparse.json");
  const pressureSource = () => fixture("product_preview/pressure_source/physics_source/n05-dense_scrutiny.raw.json");
  const healthOf = (result: MechanicsResult, eligible: boolean) =>
    numericalHealthView(readNumericalHealth(result, numericalResultStanding(result).findings), { eligible, verdictHold: false });
  const LINES = [
    { row_id: "numerical-health:load:é", label: "Numerical health of the ordinary solve: checks passed.", case_ref: "load_case:load:é", quantity_display: "", source_ref: "diagnostic:numerical-integrity:load:é" },
    { row_id: "numerical-health:load:𝔫", label: "Numerical health of the ordinary solve: checks passed.", case_ref: "load_case:load:𝔫", quantity_display: "", source_ref: "diagnostic:numerical-integrity:load:𝔫" }
  ];

  it("appends one line per load case after the result rows, so before the diagnostics section, and only when supplied", async () => {
    const result = generic();
    const base = { model: inventedModel(), result, analysisRun: inventedRun(), projectSummary: null, renderContext: PUBLIC_CONTEXT };
    const without = await buildRenderableReportInput(base);
    const withHealth = await buildRenderableReportInput({ ...base, numericalHealth: healthOf(result, true) });
    expect(withHealth.result_rows.slice(0, without.result_rows.length)).toEqual(without.result_rows);
    expect(withHealth.result_rows.slice(without.result_rows.length)).toEqual(LINES);
    expect({ ...withHealth, result_rows: without.result_rows }).toEqual(without);
    expect(JSON.stringify(withHealth)).not.toMatch(/roughly|decimal digits|rcond/);
  });

  // W-11 slice (b) (DEL-08-01 Design `exact-route-report-sections.md` §3.1): an exact-route
  // source is built only with its Export Result JSON admission, so the builder refuses this
  // unadmitted pressure-source result whether or not the line is supplied (it formerly
  // built a generic input). The exact path's rows are pinned below the gate in
  // `exactRenderedReport.test.tsx`.
  it("refuses an unadmitted pressure-source result, with or without the line", async () => {
    const result = pressureSource();
    const health = healthOf(result, true);
    expect(health?.route).toBe("pressure_source");
    const base = { model: inventedModel(), result, analysisRun: inventedRun(), projectSummary: null, renderContext: PUBLIC_CONTEXT };
    for (const input of [{ ...base, numericalHealth: health }, base]) {
      await expect(buildRenderableReportInput(input)).rejects.toThrow(/^REPORT-EXACT-ADMISSION-REQUIRED: /);
    }
  });

  it("emits the line in the renderer input the panel sends", async () => {
    (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__ = {};
    invokeMock.mockResolvedValue(unblockedOutcome());
    const projectorSpy = vi.spyOn(reportRedactionProjector, "controlReportRendererInput");
    const result = generic();
    render(<RenderedReportPanel model={inventedModel()} result={result} analysisRun={inventedRun()} projectSummary={null} numericalHealth={healthOf(result, true)} />);
    fireEvent.click(screen.getByTestId("rendered-report-render"));
    await waitFor(() => expect(projectorSpy).toHaveBeenCalled());
    const emitted = projectorSpy.mock.calls[0][0] as { result_rows: { label: string }[] };
    expect(emitted.result_rows.filter((row) => row.label.startsWith("Numerical health"))).toEqual(LINES);
  });

  function unredacted() {
    // The invented session is private project data; show the DOM body unredacted here.
    vi.spyOn(reportRedactionProjector, "controlReportDomAndJson").mockImplementation((payload) => ({
      payload, decisions: [], findings: [], blocked: false,
      summary: { decision_count: 0, finding_count: 0, redacted_count: 0, omitted_count: 0, warning_count: 0, blocking_count: 0, cloud_transmission_attempted: false, professional_claims_made: false, route_id: "test", materialization_withheld: false }
    }) as unknown as ReturnType<typeof reportRedactionProjector.controlReportDomAndJson>);
  }
  const reportPanel = (result: MechanicsResult, eligible: boolean) => (
    <ReportPanel model={inventedModel()} knowledge={null} result={result} analysisRun={inventedRun()} comparison={null}
      editorIntents={[]} projectOperation="idle" projectSummary={null} proposal={null} selectedReviewTarget={null} storageCapability={null} numericalHealth={healthOf(result, eligible)} />
  );

  it("shows the line in the report packet body before its diagnostics lines", () => {
    unredacted();
    render(reportPanel(generic(), true));
    expect(screen.getByTestId("report-numerical-health-load:é")).toHaveTextContent("Numerical health of the ordinary solve: checks passed.");
    const body = screen.getByTestId("report-packet-body").textContent ?? "";
    expect(body.indexOf("Numerical health of the ordinary solve")).toBeLessThan(body.indexOf("Diagnostic scope"));
  });

  it("shows no line in the report packet body for a pressure-source result", () => {
    unredacted();
    render(reportPanel(pressureSource(), true));
    expect(screen.getByTestId("report-packet-body").textContent).not.toContain("Numerical health");
    expect(screen.queryAllByTestId(/^report-numerical-health-/)).toHaveLength(0);
  });
});

// ---------------------------------------------------------------------------
// W-11 slice (c) (DEL-08-01 Design `exact-route-report-sections.md` §4, §6.2, §7):
// FR-REP-003 on every route. Material, section, component and support records under
// one provenance predicate, library references and the store's findings, and the
// disposition and context boundary. Targeted regression checks for a data-boundary
// change; every record, library and finding below is invented.
// ---------------------------------------------------------------------------

const LIBRARY_VERSION = `sha256:${"7".repeat(64)}`;

function inventedRecord(sourceName: string, extra: Record<string, unknown> = {}) {
  return {
    source_name: sourceName,
    source_location: "invented-datasheet-shelf",
    source_license: "invented-license",
    contributor: "invented contributor",
    contributor_certification: "invented certification",
    redistribution_status: "private_only",
    review_status: "pending",
    ...extra
  };
}

const MODULUS = { value: 1, unit: "Pa" };

/** The invented render model with materials, sections, a component and a support each
 * with and without a `provenance_record`, and two library references with one identity. */
function recordModel(): PreviewModel {
  const model = inventedModel() as unknown as Record<string, unknown> & PreviewModel;
  const libraryRef = { library_kind: "material", library_id: "invented-material-library", library_version: LIBRARY_VERSION, record_id: "steel-a" };
  model.materials = [
    {
      id: "material:with-record", label: "Invented recorded steel", elastic_modulus: MODULUS, provenance: "",
      provenance_record: inventedRecord("Invented steel datasheet", {
        review_status: "accepted", reviewed_by: "Invented Reviewer", reviewed_on: "2026-01-01", verification_note: "invented verification note", library_ref: libraryRef
      })
    },
    { id: "material:note-only", label: "Invented note steel", elastic_modulus: MODULUS, provenance: " invented_user_note " },
    { id: "material:missing", label: "Invented unsourced steel", elastic_modulus: MODULUS, provenance: "  " }
  ] as never;
  model.sections = [
    { id: "section:with-record", name: "Invented recorded section", section_type: "pipe", properties: {}, provenance: "", provenance_record: inventedRecord("Invented section table") },
    { id: "section:object-note", name: "Invented object section", section_type: "pipe", properties: {}, provenance: { source: "invented", page: 3 } },
    { id: "section:missing", name: "Invented bare section", section_type: "pipe", properties: {}, provenance: {} }
  ] as never;
  // C-999 has no note; with a record it has provenance by the predicate.
  model.components[1].provenance_record = inventedRecord("Invented rigid record") as never;
  model.supports = [
    {
      id: "support:with-record", label: "Invented recorded support", node: "node:N-110", restraints: [], provenance: "",
      provenance_record: inventedRecord("Invented support record", { library_ref: { ...libraryRef } })
    },
    { id: "support:plain", label: "Invented plain support", node: "node:N-110", restraints: [], provenance: "invented support note" }
  ] as never;
  return model;
}

const SESSION_RECORD = {
  source_name: "OpenPipeStress desktop session (Invented render model)",
  source_location: "local desktop session",
  source_license: "user_supplied_or_private",
  contributor: "user_local_session",
  contributor_certification: "not_asserted",
  redistribution_status: "private_only",
  review_status: "pending",
  privacy_classification: "private_project_data"
};

function patternRecord(kind: "material" | "section", label: string, location: string) {
  return {
    source_name: `${label} ${kind} provenance`,
    source_location: location,
    source_license: "user_supplied_or_private",
    contributor: "user_local_session",
    contributor_certification: `User-local ${kind} provenance metadata; redistribution remains private or pending until separately cleared.`,
    redistribution_status: "private_only",
    review_status: "pending",
    privacy_classification: "private_project_data"
  };
}

function mappedRecord(sourceName: string, review = "pending") {
  return {
    source_name: sourceName,
    source_location: "invented-datasheet-shelf",
    source_license: "invented-license",
    contributor: "invented contributor",
    contributor_certification: "invented certification",
    redistribution_status: "private_only",
    review_status: review,
    privacy_classification: "private_project_data"
  };
}

const buildFor = (model: PreviewModel, extra: Partial<Parameters<typeof buildRenderableReportInput>[0]> = {}) =>
  buildRenderableReportInput({ model, result: inventedResult(), analysisRun: inventedRun(), projectSummary: null, renderContext: LOCAL_PRIVATE_CONTEXT, ...extra });

describe("FR-REP-003 records (W-11 slice (c), §4.1)", () => {
  it("records materials, sections, components and supports under one provenance predicate", async () => {
    const input = await buildFor(recordModel());
    const sections = input.report_sections;
    expect(sections.provenance_notes).toEqual([
      SESSION_RECORD,
      expect.objectContaining({ source_name: "Invented elbow marker component provenance" }),
      mappedRecord("Invented rigid record"),
      mappedRecord("Invented steel datasheet", "accepted"),
      patternRecord("material", "Invented note steel", "material.provenance=invented_user_note"),
      patternRecord("material", "Invented unsourced steel", "material provenance missing"),
      mappedRecord("Invented section table"),
      patternRecord("section", "Invented object section", 'section.provenance={"page":3,"source":"invented"}'),
      patternRecord("section", "Invented bare section", "section provenance missing"),
      mappedRecord("Invented support record")
    ]);
    // reviewed_by, reviewed_on, verification_note and library_ref are not carried.
    expect(JSON.stringify(sections)).not.toMatch(/Invented Reviewer|2026-01-01|invented verification note|steel-a/);

    const value = (kind: string, id: string, record: unknown, missing: boolean, review = "pending") => ({
      value_id: `${kind}-provenance:${id}`,
      value_category: "report_metadata",
      source: { ref_type: kind, ref_id: id },
      provenance: record,
      privacy_classification: "private_project_data",
      required_for: ["reporting", "human_review"],
      review_status: review,
      missing_data_finding: missing
    });
    expect(sections.user_supplied_values.slice(2)).toEqual([
      value("material", "material:with-record", mappedRecord("Invented steel datasheet", "accepted"), false, "accepted"),
      value("material", "material:note-only", sections.provenance_notes[4], false),
      value("material", "material:missing", sections.provenance_notes[5], true),
      value("section", "section:with-record", mappedRecord("Invented section table"), false),
      value("section", "section:object-note", sections.provenance_notes[7], false),
      value("section", "section:missing", sections.provenance_notes[8], true),
      value("support", "support:with-record", mappedRecord("Invented support record"), false)
    ]);
    // No quantity member on the new entries (schema `ValueQuantity` is an object).
    for (const entry of sections.user_supplied_values.slice(2)) expect("quantity" in entry).toBe(false);
    // The component's entry keeps its id, category and form; its record follows the predicate.
    expect(sections.user_supplied_values[1]).toEqual({
      value_id: "component-provenance:component:C-999",
      value_category: "component_provenance:rigid",
      source: { ref_type: "component", ref_id: "component:C-999" },
      quantity: null,
      provenance: mappedRecord("Invented rigid record"),
      privacy_classification: "private_project_data",
      required_for: ["reporting", "human_review"],
      review_status: "pending",
      missing_data_finding: false
    });

    const provenanceDiagnostics = sections.diagnostics.filter((item) => item.class === "PROVENANCE_WARNING");
    expect(provenanceDiagnostics).toEqual([
      {
        code: "MATERIAL_PROVENANCE_MISSING", class: "PROVENANCE_WARNING", severity: "warning",
        source: { ref_type: "material", ref_id: "material:missing" }, affected_object: { ref_type: "material", ref_id: "material:missing" },
        message: "Material Invented unsourced steel has neither a provenance_record nor a provenance note; report provenance is incomplete for this material.",
        remediation: "Enter material source/provenance before relying on this report.", provenance: SESSION_RECORD
      },
      {
        code: "SECTION_PROVENANCE_MISSING", class: "PROVENANCE_WARNING", severity: "warning",
        source: { ref_type: "section", ref_id: "section:missing" }, affected_object: { ref_type: "section", ref_id: "section:missing" },
        message: "Section Invented bare section has neither a provenance_record nor a provenance note; report provenance is incomplete for this section.",
        remediation: "Enter section source/provenance before relying on this report.", provenance: SESSION_RECORD
      }
    ]);
  });

  it("gives a component with a record and no note neither a missing-data flag nor COMPONENT_PROVENANCE_MISSING", async () => {
    const without = await buildFor(inventedModel());
    expect(without.report_sections.diagnostics.find((item) => item.code === "COMPONENT_PROVENANCE_MISSING")?.message).toBe(
      "Component Invented component with missing provenance has neither a provenance_record nor a component.provenance note; report provenance is incomplete for this component."
    );
    const model = inventedModel();
    model.components[1].provenance_record = inventedRecord("Invented rigid record") as never;
    const withRecord = await buildFor(model);
    expect(withRecord.report_sections.diagnostics.some((item) => item.code === "COMPONENT_PROVENANCE_MISSING")).toBe(false);
    expect(withRecord.report_sections.user_supplied_values[1].missing_data_finding).toBe(false);
    // A record whose source name is blank is not provenance by the predicate.
    model.components[1].provenance_record = inventedRecord("   ") as never;
    const blank = await buildFor(model);
    expect(blank.report_sections.user_supplied_values[1]).toMatchObject({ missing_data_finding: true, provenance: { source_name: "not recorded" } });
    expect(blank.report_sections.diagnostics.some((item) => item.code === "COMPONENT_PROVENANCE_MISSING")).toBe(true);
  });

  it("copies an accepted review: not a missing-data finding when sourced; pending on a missing-data value, whose record keeps accepted", async () => {
    const model = inventedModel();
    model.materials = [
      { id: "material:accepted", label: "Invented accepted steel", elastic_modulus: MODULUS, provenance: "", provenance_record: inventedRecord("Invented accepted datasheet", { review_status: "accepted" }) },
      { id: "material:accepted-blank", label: "Invented blank steel", elastic_modulus: MODULUS, provenance: "", provenance_record: inventedRecord(" ", { review_status: "accepted" }) }
    ] as never;
    const values = (await buildFor(model)).report_sections.user_supplied_values;
    const sourced = values.find((item) => item.value_id === "material-provenance:material:accepted")!;
    expect(sourced).toMatchObject({ review_status: "accepted", missing_data_finding: false, provenance: { review_status: "accepted" } });
    const blank = values.find((item) => item.value_id === "material-provenance:material:accepted-blank")!;
    expect(blank).toMatchObject({ review_status: "pending", missing_data_finding: true, provenance: { source_name: "not recorded", review_status: "accepted" } });
    // No value is both a missing-data finding and accepted
    // (REPORT_SECTION_MISSING_VALUE_MARKED_ACCEPTED; checked in Rust on the schema fixture).
    expect(values.filter((item) => item.missing_data_finding && item.review_status === "accepted")).toEqual([]);
  });

  it("fills absent record members with not recorded, unknown and pending", async () => {
    const model = inventedModel();
    model.materials = [{ id: "material:sparse", label: "Invented sparse steel", elastic_modulus: MODULUS, provenance: "", provenance_record: { source_name: "Invented sparse datasheet", source_location: "  " } }] as never;
    const note = (await buildFor(model)).report_sections.provenance_notes.at(-1);
    expect(note).toEqual({
      source_name: "Invented sparse datasheet",
      source_location: "not recorded",
      source_license: "not recorded",
      contributor: "not recorded",
      contributor_certification: "not recorded",
      redistribution_status: "unknown",
      review_status: "pending",
      privacy_classification: "private_project_data"
    });
  });

  it("leaves an input without materials, sections, records or library references byte-identical", async () => {
    // Captured from the builder at 35f537355f (before slice (c)) for the same arguments;
    // the model's components all carry a note, so no changed diagnostic text enters.
    const expected = {
      public_report: "d1fb38303582b15517dac4ccc44942252f157568e2c724b8439dedb06cd52177",
      local_private: "f612cf9d351462c5794eff15e8e150fcc8b4545adae8fd1e17555f8b607a73b0"
    } as const;
    for (const renderContext of [PUBLIC_CONTEXT, { exportContext: "local_private", generatedAt: null } as const]) {
      const model = inventedModel();
      model.components[1].provenance = "invented_user_note";
      const input = await buildFor(model, { renderContext });
      expect("library_references" in input).toBe(false);
      const { canonicalSha256Hex } = await import("../../services/hashService");
      // The readability slice (DEL-08-01 Design `exact-route-report-sections.md` §6.3 item 3)
      // changed only the display policy text; with the earlier text the input is the captured one.
      const summary = input.calculation_report.model_input_summary.unit_display_summary;
      expect(summary.quantity_display_policy).toBe(QUANTITY_DISPLAY_POLICY);
      expect(summary.conversion_performed).toBe(false);
      const earlier = structuredClone(input);
      earlier.calculation_report.model_input_summary.unit_display_summary.quantity_display_policy =
        "display result-row values with their explicit units; no report-time conversion";
      expect(await canonicalSha256Hex(earlier)).toBe(expected[renderContext.exportContext]);
    }
  });
});

describe("library references and the store's findings (W-11 slice (c), §4.2)", () => {
  afterEach(async () => {
    const { publishLibraryRefFindings } = await import("../library/libraryRefFindings");
    publishLibraryRefFindings("", []);
  });

  it("lists each distinct reference once, in first-occurrence order, with its owners sorted", async () => {
    const model = recordModel();
    model.sections![0].provenance_record = inventedRecord("Invented section table", {
      library_ref: { library_kind: "section", library_id: "invented-section-library" }
    }) as never;
    const input = await buildFor(model);
    expect(input.library_references).toEqual([
      {
        reference_id: "library-ref:1", library_kind: "material", library_id: "invented-material-library", library_version: LIBRARY_VERSION,
        record_id: "steel-a", entity_refs: ["material:with-record", "support:with-record"], provenance: SESSION_RECORD
      },
      {
        reference_id: "library-ref:2", library_kind: "section", library_id: "invented-section-library", library_version: "not recorded",
        record_id: "not recorded", entity_refs: ["section:with-record"], provenance: SESSION_RECORD
      }
    ]);
  });

  it("projects each finding as a provenance warning, and the panel passes the store's current findings", async () => {
    const model = recordModel();
    const finding = {
      entity_ref: "material:with-record", library_kind: "material" as const, library_id: "invented-material-library", library_version: LIBRARY_VERSION,
      record_id: "steel-a", code: "MODEL-LIBRARY-REF-VERSION-CHANGED" as const, severity: "warning" as const,
      recorded_version: LIBRARY_VERSION, stored_version: `sha256:${"8".repeat(64)}`, message: "Invented library version changed."
    };
    const expected = {
      code: "MODEL-LIBRARY-REF-VERSION-CHANGED", class: "PROVENANCE_WARNING", severity: "warning",
      source: { ref_type: "diagnostic_source", ref_id: "apps/desktop/src-tauri/src/lib.rs check_model_library_refs" },
      affected_object: { ref_type: "model_entity", ref_id: "material:with-record" },
      message: "Invented library version changed.",
      remediation: "Compare the library reference with the local library store before relying on this report.",
      provenance: SESSION_RECORD
    };
    const direct = await buildFor(model, { libraryRefFindings: [finding] });
    expect(direct.report_sections.diagnostics.at(-1)).toEqual(expected);
    expect(JSON.stringify(direct)).not.toContain(finding.stored_version);

    // Through the panel: the published finding that still applies reaches the native input;
    // one raised on a reference the model no longer carries does not.
    const { publishLibraryRefFindings } = await import("../library/libraryRefFindings");
    publishLibraryRefFindings(model.project.id, [finding, { ...finding, entity_ref: "material:retired", message: "Invented stale finding." }]);
    (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__ = {};
    invokeMock.mockResolvedValue(unblockedOutcome());
    renderPanel({ model });
    fireEvent.click(screen.getByTestId("rendered-report-local-private-intent"));
    fireEvent.click(screen.getByTestId("rendered-report-render"));
    await waitFor(() => expect(screen.getByTestId("rendered-report-preview")).toBeInTheDocument());
    const sent = renderedInput() as { report_sections: { diagnostics: unknown[] }; library_references: unknown[] };
    expect(sent.report_sections.diagnostics.filter((item) => (item as { code: string }).code.startsWith("MODEL-LIBRARY-REF-"))).toEqual([expected]);
    expect(sent.library_references).toHaveLength(1);
  });
});

describe("FR-REP-003 disposition and context boundary (W-11 slice (c), §4.3, §6.2)", () => {
  /** A record of each disposition class on a material, a section and a support. */
  function dispositionModel(record: Record<string, unknown>): PreviewModel {
    const model = recordModel();
    model.materials![0].provenance_record = { ...model.materials![0].provenance_record!, ...record } as never;
    return model;
  }
  const CLASSES: [string, Record<string, unknown>][] = [
    ["private_only", { redistribution_status: "private_only" }],
    ["unknown", { redistribution_status: "unknown" }],
    ["protected_suspected", { redistribution_status: "protected_suspected" }],
    ["quarantined", { review_status: "quarantined" }],
    ["rejected", { review_status: "rejected" }]
  ];

  it("carries each record's disposition verbatim to the native render, which refuses the blocking ones", async () => {
    for (const [name, record] of CLASSES) {
      const input = await buildFor(dispositionModel(record));
      const note = input.report_sections.provenance_notes.find((item) => item.source_name === "Invented steel datasheet")!;
      const value = input.report_sections.user_supplied_values.find((item) => item.value_id === "material-provenance:material:with-record")!;
      expect(note).toMatchObject(record);
      expect(value.provenance).toMatchObject(record);
      if ("review_status" in record) expect(value.review_status).toBe(record.review_status);
      // The webview's local-private control under the panel's intent passes the record
      // through unchanged (its route projection makes it opaque leaves); the native renderer
      // refuses quarantined, rejected and protected_suspected records in every context, and
      // shows an unknown one with a non-blocking warning only under local_private (owner
      // ruling on #1639, "C1 b, C2 a"; `report_renderer` tests
      // `fr_rep_003_records_render_only_locally_and_refuse_blocking_dispositions_in_every_context`,
      // `an_unresolved_record_warns_in_local_private_and_blocks_elsewhere`).
      const controlled = reportRedactionProjector.controlReportRendererInput(input, true);
      expect(controlled.blocked, name).toBe(false);
      expect(JSON.stringify(controlled.payload)).toContain(JSON.stringify(record).slice(1, -1));
    }
  });

  it("screens structured notes and records at any depth before flattening, as the package save screens the model", async () => {
    const cases: [string, (model: PreviewModel) => void, Record<string, string>][] = [
      ["a note's own review", (model) => { model.sections![1].provenance = { source: "invented", review_status: "quarantined" }; }, { review_status: "quarantined" }],
      ["a nested record in a note", (model) => { model.sections![1].provenance = { source: "invented", records: [{ note: "x", review_status: "rejected" }] }; }, { review_status: "rejected" }],
      ["a nested redistribution in a note", (model) => { model.sections![1].provenance = { source: { redistribution_status: "protected_suspected" } }; }, { redistribution_status: "protected_suspected" }],
      ["a nested privacy class in a note", (model) => { model.sections![1].provenance = { source: { privacy_classification: "protected_suspected" } }; }, { redistribution_status: "protected_suspected" }],
      ["a lifecycle status in a note", (model) => { model.sections![1].provenance = { pack: { lifecycle_status: "rejected" } }; }, { review_status: "rejected" }],
      ["a record's own privacy class", (model) => { (model.materials![0].provenance_record as Record<string, unknown>).privacy_classification = "protected_suspected"; }, { redistribution_status: "protected_suspected" }],
      ["a record's nested member", (model) => { (model.materials![0].provenance_record as Record<string, unknown>).extra = { deep: [{ review_status: "quarantined" }] }; }, { review_status: "quarantined" }]
    ];
    for (const [name, edit, carried] of cases) {
      const model = recordModel();
      edit(model);
      const input = await buildFor(model);
      const subject = name.includes("note") ? "section-provenance:section:object-note" : "material-provenance:material:with-record";
      const value = input.report_sections.user_supplied_values.find((item) => item.value_id === subject)!;
      expect(value.provenance, name).toMatchObject(carried);
      expect(input.report_sections.provenance_notes, name).toContainEqual(value.provenance);
      if (carried.review_status) expect(value.review_status, name).toBe(carried.review_status);
      // The webview's own control passes it on; the native renderer refuses it.
      expect(reportRedactionProjector.controlReportRendererInput(input, true).blocked, name).toBe(false);
    }
    // A malformed disposition member refuses the build, naming the entity and member.
    const malformed: [(model: PreviewModel) => void, string][] = [
      [(model) => { model.sections![1].provenance = { source: { review_status: ["quarantined"] } }; }, "section section:object-note: provenance.source.review_status is an array"],
      [(model) => { (model.materials![0].provenance_record as Record<string, unknown>).review_status = { value: "rejected" }; }, "material material:with-record: provenance_record.review_status is an object"],
      [(model) => { (model.components[1].provenance_record as Record<string, unknown>).classification = {}; }, "component component:C-999: provenance_record.classification is an object"]
    ];
    for (const [edit, detail] of malformed) {
      const model = recordModel();
      edit(model);
      await expect(buildFor(model)).rejects.toThrow(`REPORT-PROVENANCE-DISPOSITION-MALFORMED: ${detail}`);
    }
    // A clean structured note is unchanged.
    expect((await buildFor(recordModel())).report_sections.provenance_notes[7]).toMatchObject({ redistribution_status: "private_only", review_status: "pending" });
  });

  it("withholds every record from the public-report context: blocked before IPC, nothing sent", async () => {
    for (const [name, record] of CLASSES) {
      const input = await buildFor(dispositionModel(record), { renderContext: PUBLIC_CONTEXT });
      const controlled = reportRedactionProjector.controlReportRendererInput(input, false);
      expect(controlled.blocked, name).toBe(true);
      expect(controlled.payload, name).toBeNull();
    }
    (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__ = {};
    invokeMock.mockResolvedValue(unblockedOutcome());
    renderPanel({ model: recordModel() });
    fireEvent.click(screen.getByTestId("rendered-report-render"));
    await waitFor(() => expect(screen.getByTestId("rendered-report-redaction-summary")).toHaveTextContent("context=public_report"));
    expect(screen.getByTestId("rendered-report-redaction-summary")).toHaveTextContent("blocked=true");
    expect(invokeMock).not.toHaveBeenCalled();
  });

  // Owner ruling on #1639 ("C1 b, C2 a", owner, 2026-10-26): authored record text, a
  // user-typed path in `source_location` included, stays as written in local-private outputs,
  // and reaches no other: the public render is blocked before IPC and the report packet
  // carries no record (the native public render refuses it: `report_renderer` test
  // `a_typed_path_stays_local`).
  it("keeps a typed source_location path as written locally and out of every other output", async () => {
    const path = "/Users/invented-user/datasheets/steel-a.pdf";
    const model = recordModel();
    (model.materials![0].provenance_record as Record<string, unknown>).source_location = path;
    const local = await buildFor(model);
    expect(local.report_sections.provenance_notes.find((note) => note.source_name === "Invented steel datasheet")?.source_location).toBe(path);
    const controlled = reportRedactionProjector.controlReportRendererInput(local, true);
    expect(controlled.blocked).toBe(false);
    expect(JSON.stringify(controlled.payload)).toContain(JSON.stringify(path));
    // Public render: blocked before IPC, no payload, nothing sent.
    const publicInput = await buildFor(model, { renderContext: PUBLIC_CONTEXT });
    const publicControlled = reportRedactionProjector.controlReportRendererInput(publicInput, false);
    expect(publicControlled.blocked).toBe(true);
    expect(publicControlled.payload).toBeNull();
    (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__ = {};
    invokeMock.mockResolvedValue(unblockedOutcome());
    const { unmount } = renderPanel({ model });
    fireEvent.click(screen.getByTestId("rendered-report-render"));
    await waitFor(() => expect(screen.getByTestId("rendered-report-redaction-summary")).toHaveTextContent("blocked=true"));
    expect(invokeMock).not.toHaveBeenCalled();
    expect(document.body.textContent ?? "").not.toContain(path);
    unmount();
    // The report packet (public DOM/JSON route) carries no record.
    vi.spyOn(reportRedactionProjector, "controlReportDomAndJson").mockImplementation((payload) => ({
      payload, decisions: [], findings: [], blocked: false,
      summary: { decision_count: 0, finding_count: 0, redacted_count: 0, omitted_count: 0, warning_count: 0, blocking_count: 0, cloud_transmission_attempted: false, professional_claims_made: false, route_id: "test", materialization_withheld: false }
    }) as unknown as ReturnType<typeof reportRedactionProjector.controlReportDomAndJson>);
    render(<ReportPanel model={model} knowledge={null} result={inventedResult()} analysisRun={inventedRun()} comparison={null}
      editorIntents={[]} projectOperation="idle" projectSummary={null} proposal={null} selectedReviewTarget={null} storageCapability={null} />);
    expect(document.body.textContent ?? "").not.toContain(path);
    expect(document.body.innerHTML).not.toContain(path);
  });

  // Codex review round 3 of #1639: the native refusal names records by position only; the
  // panel resolves the position to its entity locally from the input it sent.
  it("resolves a refused record's position to its entity in the panel only", async () => {
    const { provenanceReasonEntity } = await import("./provenanceReasonEntities");
    const model = recordModel();
    const input = await buildFor(model);
    const reason = (place: string) => `report provenance: REPORT_PROVENANCE_DISPOSITION_BLOCKED (${place} review_status quarantined; review that record's disposition in the model before reporting)`;
    expect(provenanceReasonEntity(reason("user_supplied_values[2]"), input)).toBe("material-provenance:material:with-record");
    expect(provenanceReasonEntity(reason("user_supplied_values[2].provenance"), input)).toBe("material-provenance:material:with-record");
    expect(provenanceReasonEntity(reason("library_references[0].provenance"), input)).toBe("library invented-material-library, referenced by material:with-record, support:with-record");
    expect(provenanceReasonEntity(reason("provenance_notes[3]"), input)).toBe("Invented steel datasheet");
    expect(provenanceReasonEntity(reason("user_supplied_values[99]"), input)).toBeNull();
    expect(provenanceReasonEntity("section validation: REPORT_SECTION_PROTECTED_CONTENT_SUSPECTED", input)).toBeNull();
    (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__ = {};
    invokeMock.mockResolvedValue({ ...unblockedOutcome(), html: "", derived_print_html: "", export_blocked: true, content_withheld: true, blocking_reasons: [reason("user_supplied_values[2]")] });
    renderPanel({ model });
    fireEvent.click(screen.getByTestId("rendered-report-local-private-intent"));
    fireEvent.click(screen.getByTestId("rendered-report-render"));
    await waitFor(() => expect(screen.getByTestId("rendered-report-blocking-reasons")).toBeInTheDocument());
    expect(screen.getByTestId("rendered-report-blocking-reasons")).toHaveTextContent(`${reason("user_supplied_values[2]")} (material-provenance:material:with-record)`);
    expect(screen.queryByTestId("rendered-report-preview")).not.toBeInTheDocument();
    expect(screen.queryByTestId("rendered-report-save-local-private")).not.toBeInTheDocument();
  });

  it("keeps the report packet free of the new records", () => {
    vi.spyOn(reportRedactionProjector, "controlReportDomAndJson").mockImplementation((payload) => ({
      payload, decisions: [], findings: [], blocked: false,
      summary: { decision_count: 0, finding_count: 0, redacted_count: 0, omitted_count: 0, warning_count: 0, blocking_count: 0, cloud_transmission_attempted: false, professional_claims_made: false, route_id: "test", materialization_withheld: false }
    }) as unknown as ReturnType<typeof reportRedactionProjector.controlReportDomAndJson>);
    render(<ReportPanel model={recordModel()} knowledge={null} result={inventedResult()} analysisRun={inventedRun()} comparison={null}
      editorIntents={[]} projectOperation="idle" projectSummary={null} proposal={null} selectedReviewTarget={null} storageCapability={null} />);
    const body = document.body.textContent ?? "";
    for (const text of ["Invented steel datasheet", "Invented section table", "Invented support record", "Invented rigid record", "invented-material-library", "material-provenance:"]) {
      expect(body).not.toContain(text);
    }
  });
});

describe("schema check of representative records (W-11 slice (c), §7)", () => {
  it("writes the representative report_sections; stale fails", async () => {
    const model = inventedModel();
    model.materials = [
      { id: "material:accepted", label: "Invented accepted steel", elastic_modulus: MODULUS, provenance: "", provenance_record: inventedRecord("Invented accepted datasheet", { review_status: "accepted" }) },
      { id: "material:accepted-blank", label: "Invented blank steel", elastic_modulus: MODULUS, provenance: "", provenance_record: inventedRecord(" ", { review_status: "accepted" }) },
      { id: "material:note", label: "Invented note steel", elastic_modulus: MODULUS, provenance: "invented_user_note" },
      { id: "material:missing", label: "Invented unsourced steel", elastic_modulus: MODULUS, provenance: "" }
    ] as never;
    model.sections = [
      { id: "section:rejected", name: "Invented rejected section", section_type: "pipe", properties: {}, provenance: "", provenance_record: inventedRecord("Invented rejected table", { review_status: "rejected" }) },
      { id: "section:object-note", name: "Invented object section", section_type: "pipe", properties: {}, provenance: { source: "invented", page: 3 } },
      // Structured notes and a record whose blocking dispositions are nested (§6.2).
      { id: "section:nested-rejected", name: "Invented nested section", section_type: "pipe", properties: {}, provenance: { source: "invented", records: [{ note: "x", review_status: "rejected" }] } },
      { id: "section:note-quarantined", name: "Invented quarantined note section", section_type: "pipe", properties: {}, provenance: { source: "invented", review_status: "quarantined" } },
      { id: "section:nested-suspected", name: "Invented suspected section", section_type: "pipe", properties: {}, provenance: "", provenance_record: inventedRecord("Invented suspected table", { extra: { redistribution_status: "protected_suspected" } }) }
    ] as never;
    model.components[1].provenance_record = inventedRecord("Invented quarantined component record", { review_status: "quarantined" }) as never;
    model.supports = [
      { id: "support:public", label: "Invented public support", node: "node:N-110", restraints: [], provenance: "", provenance_record: inventedRecord("Invented support record", { redistribution_status: "public_permissive" }) },
      { id: "support:plain", label: "Invented plain support", node: "node:N-110", restraints: [], provenance: "invented support note" }
    ] as never;
    const input = await buildFor(model);
    const content = `${JSON.stringify({
      schema_version: "1.0.0",
      generator: "apps/desktop/src/features/report/renderedReport.test.tsx",
      description: "W-11 slice (c) (DEL-08-01 Design exact-route-report-sections.md sections 4.1 and 7): the desktop builder's report_sections for an invented model with materials, sections, a component and supports, each with and without a provenance_record, and records marked accepted, rejected and quarantined, including blocking dispositions nested in structured notes and records. Validated against schemas/report_sections.schema.yaml by tests/test_report_sections_provenance_records.py and read by the renderer's tests. Invented data only.",
      report_sections: input.report_sections
    }, null, 2)}\n`;
    const name = "report_sections_provenance_records.json";
    const outDir = process.env.OPENPIPESTRESS_WRITE_REPORT_SECTIONS_PROVENANCE_RECORDS;
    if (outDir) {
      const { mkdirSync, writeFileSync } = await import("node:fs");
      mkdirSync(outDir, { recursive: true });
      writeFileSync(resolve(outDir, name), content);
      return;
    }
    const committed = readFileSync(resolve(__dirname, "../../../../../fixtures/reports/invented", name), "utf8");
    expect(committed === content, `fixtures/reports/invented/${name} is stale; regenerate it with OPENPIPESTRESS_WRITE_REPORT_SECTIONS_PROVENANCE_RECORDS=<dir>`).toBe(true);
  });
});

// The display rule (DEL-08-01 Design `report-package-current-results.md` §2.3 items 5-6):
// `conversion_performed` is true only when the rendered report displays a value stored in Pa;
// the report JSON packet displays no values and keeps its own disclosure.
describe("the display rule's conversion disclosure", () => {
  function withPaRow(): MechanicsResult {
    const result = inventedResult();
    result.results.push({ ...result.results[0], id: "result:stress:invented", kind: "stress", value: 41354908.75465696, unit: "Pa" });
    return result;
  }

  it("states a conversion in the rendered report only for a displayed Pa value, with the stored units", async () => {
    const model = inventedModel();
    expect(buildUnitDisplaySummary(model, inventedResult())).toMatchObject({ quantity_display_policy: QUANTITY_DISPLAY_POLICY, conversion_performed: false, result_units: ["m"] });
    expect(buildUnitDisplaySummary(model, withPaRow())).toMatchObject({ quantity_display_policy: QUANTITY_DISPLAY_POLICY, conversion_performed: true, result_units: ["Pa", "m"] });
  });

  it("keeps the report JSON packet's disclosure: its own policy and no conversion", () => {
    const disclosure = reportUnitSystemDisclosure(inventedModel(), withPaRow());
    expect(disclosure.quantity_display_policy).toBe(REPORT_PACKET_QUANTITY_DISPLAY_POLICY);
    expect(disclosure.conversion_performed).toBe(false);
    expect(disclosure.result_units).toEqual(["Pa", "m"]);
  });
});
