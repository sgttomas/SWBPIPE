import {
  fireEvent,
  render,
  screen,
  waitFor,
  within,
} from "@testing-library/react";
import { describe, expect, it } from "vitest";
import { App } from "./App";
import { PropertyInspector } from "./features/model-tree/PropertyInspector";
import {
  buildDeformationOverlay,
  PipeViewport,
} from "./features/viewport/PipeViewport";
import {
  buildAnalysisRunPreview,
  buildPreviewComparison,
  loadPreviewModel,
  runPreviewMechanics,
} from "./services/previewService";
import type { EditorOperationIntent, MechanicsResult } from "./types";

function deformationResultRows(
  results: MechanicsResult["results"],
): MechanicsResult {
  return {
    schema_version: "0.1.0",
    document_kind: "openpipestress.product_preview.mechanics_result",
    run_id: "run:preview-overlay-unit-test",
    model_ref: "project:invented-demo-loop",
    status: {
      mechanics: "MECHANICS_SOLVED",
      rule_check: "RULE_INPUTS_INCOMPLETE",
      professional_acceptance: "NOT_PROVIDED",
    },
    summary: {},
    results,
    diagnostics: [],
  };
}

function displacementMagnitudeRow(
  nodeId: string,
  valueMm: number,
): MechanicsResult["results"][number] {
  return {
    id: `result:disp:${nodeId.replace(":", "-")}`,
    kind: "displacement_magnitude",
    value: valueMm,
    unit: "mm",
    entity_ref: nodeId,
    basis_ref: { ref_type: "load_case", ref_id: "load:L-100" },
  };
}

function displacementComponentRows(
  nodeId: string,
  vector: { x: number; y: number; z: number },
  basisRefId = "load:L-100",
): MechanicsResult["results"] {
  const suffix = nodeId.replace(":", "-");
  return (["x", "y", "z"] as const).map((axis) => ({
    id: `result:disp:${suffix}:u${axis}`,
    kind: `global_nodal_displacement_${axis}`,
    value: vector[axis],
    unit: "mm",
    entity_ref: nodeId,
    basis_ref: { ref_type: "load_case", ref_id: basisRefId },
    metadata: {
      component: `nodal_displacement_${axis}`,
      coordinate_system: "global",
      location: "node",
      basis: "solved_from_global_linear_system",
      sign_convention: `positive value follows the global cartesian ${axis.toUpperCase()} axis displacement of the node`,
    },
  }));
}

describe("OpenPipeStress desktop preview", () => {
  it("records comparison workspace unit policy evidence without conversion", async () => {
    const model = await loadPreviewModel();
    const result = await runPreviewMechanics(model);
    const analysisRun = await buildAnalysisRunPreview(result);
    const comparisonPacket = buildPreviewComparison({ result, analysisRun });

    expect(comparisonPacket.unit_policy_evidence.unit_system_ref.ref).toBe(
      "unit-system:dec-018-si-dual-display",
    );
    expect(comparisonPacket.unit_policy_evidence.storage_convention).toBe(
      "entered_units_preserved",
    );
    expect(comparisonPacket.unit_policy_evidence.comparison_unit_policy).toBe(
      "compare_only_rows_with_equal_explicit_result_units",
    );
    expect(comparisonPacket.unit_policy_evidence.conversion_policy).toBe(
      "comparison_workspace_preserves_result_units_without_conversion",
    );
    expect(comparisonPacket.unit_policy_evidence.conversion_performed).toBe(
      false,
    );
    expect(comparisonPacket.unit_policy_evidence.matched_result_units).toEqual([
      "MPa",
      "N",
      "N*m",
      "mm",
      "rad",
    ]);
    expect(
      comparisonPacket.unit_policy_evidence.unmatched_left_result_count,
    ).toBe(18);
    expect(
      comparisonPacket.unit_policy_evidence.unmatched_right_result_count,
    ).toBe(0);
    expect(comparisonPacket.unit_policy_evidence.tolerance_profile_ref).toBe(
      "TBD",
    );
    expect(
      comparisonPacket.unit_policy_evidence.decision_basis_refs.map(
        (item) => item.ref,
      ),
    ).toEqual(["DEC-018", "DEC-026", "DEL-14-05"]);
  });

  it("presents the CAD-shell menu workspace with the dock summoned from the View menu", async () => {
    render(<App />);

    expect(
      await screen.findByTestId("desktop-preview-shell"),
    ).toBeInTheDocument();
    expect(await screen.findByTestId("app-menu-bar")).toBeInTheDocument();
    // The dock starts collapsed so the spatial core owns the surface; the
    // workflow ribbon and earlier guided shells are gone.
    expect(screen.getByTestId("workspace-dock").className).toContain(
      "collapsed",
    );
    expect(screen.getByTestId("toggle-tree")).toHaveAttribute(
      "aria-expanded",
      "false",
    );
    expect(screen.getByTestId("toggle-inspector")).toHaveAttribute(
      "aria-expanded",
      "false",
    );
    expect(screen.getByTestId("agent-workbench-panel")).toBeInTheDocument();
    expect(screen.getByTestId("agent-focus-selection")).toHaveTextContent(
      "project: project:invented-loop-01",
    );
    expect(screen.getByTestId("agent-proposal-summary")).toHaveTextContent(
      "review_only_local_preview",
    );
    expect(screen.queryByTestId("workflow-ribbon")).toBeNull();
    expect(screen.queryByTestId("guided-workbench")).toBeNull();
    expect(screen.queryByTestId("workspace-nav")).toBeNull();
    expect(screen.queryByTestId("guided-journey-stack")).toBeNull();

    // Summon the Operation Apply section from the View menu.
    fireEvent.click(screen.getByTestId("menu-view"));
    fireEvent.click(screen.getByTestId("menu-item-view.section.operations"));
    expect(screen.getByTestId("workspace-dock").className).not.toContain(
      "collapsed",
    );

    const drawer = screen.getByTestId("review-apply-drawer");
    expect(drawer.className).not.toContain("open");
    fireEvent.click(screen.getByTestId("review-apply-drawer-toggle"));
    expect(drawer.className).toContain("open");
    expect(screen.getByLabelText("Editor contract review")).toBeInTheDocument();
    expect(screen.getByLabelText("Operation diff preview")).toBeInTheDocument();

    // Switch sections through the View menu.
    fireEvent.click(screen.getByTestId("menu-view"));
    fireEvent.click(screen.getByTestId("menu-item-view.section.rule-packs"));
    expect(screen.getByTestId("workspace-section-rule-packs").className).toBe(
      "workspace-dock-section",
    );

    fireEvent.click(screen.getByTestId("menu-view"));
    fireEvent.click(screen.getByTestId("menu-item-view.section.libraries"));
    expect(screen.getByTestId("workspace-section-libraries").className).toBe(
      "workspace-dock-section",
    );
    fireEvent.click(screen.getByTestId("library-load-template"));
    fireEvent.click(screen.getByTestId("library-validate"));

    fireEvent.click(screen.getByTestId("menu-view"));
    fireEvent.click(screen.getByTestId("menu-item-view.section.solve"));
    expect(screen.getByTestId("workspace-section-solve").className).toBe(
      "workspace-dock-section",
    );

    // Re-selecting the active section from the View menu collapses the dock.
    fireEvent.click(screen.getByTestId("menu-view"));
    fireEvent.click(screen.getByTestId("menu-item-view.section.solve"));
    expect(screen.getByTestId("workspace-dock").className).toContain(
      "collapsed",
    );

    // Insert commands arm object-creation tools; they do not merely navigate or
    // queue operations on click.
    fireEvent.click(screen.getByTestId("menu-insert"));
    fireEvent.click(screen.getByTestId("menu-item-insert.node"));
    expect(screen.getByTestId("armed-creation-tool")).toHaveTextContent(
      "Node tool armed",
    );
    expect(screen.getByTestId("command-node")).toHaveAttribute(
      "aria-pressed",
      "true",
    );
    expect(
      screen.getByTestId("local-project-review-context").textContent,
    ).toContain("0 pending operations");

    fireEvent.click(screen.getByTestId("menu-insert"));
    fireEvent.click(screen.getByTestId("menu-item-insert.load"));
    expect(screen.getByTestId("armed-creation-tool")).toHaveTextContent(
      "Load tool armed",
    );
    expect(screen.getByTestId("workspace-section-loads").className).toBe(
      "workspace-dock-section",
    );

    fireEvent.click(screen.getByTestId("issues-drawer-toggle"));
    expect(await screen.findByTestId("issues-home")).toBeInTheDocument();
    fireEvent.click(screen.getByTestId("audit-drawer-toggle"));
    expect(
      await screen.findByTestId("audit-boundary-drawer"),
    ).toBeInTheDocument();
    // Heavy full-<App/> Three.js render plus shell-level integration checks:
    // allow the same DEC-025/full-suite worker load that exercised this path.
  }, 60000);

  it("renders the engineering workspace from invented local fixtures", async () => {
    render(<App />);

    expect(await screen.findByText("OpenPipeStress")).toBeInTheDocument();
    expect(
      await screen.findByTestId("desktop-preview-shell"),
    ).toBeInTheDocument();
    expect(await screen.findByTestId("solve-panel")).toBeInTheDocument();
    expect(
      screen.getByTestId("viewport-deformation-status").textContent,
    ).toContain("not started; result rows=0");
    const initialReadiness = screen.getByTestId("solve-readiness-summary");
    expect(
      within(initialReadiness).getByTestId("readiness-mechanics").textContent,
    ).toContain("preview run not started");
    expect(
      within(initialReadiness).getByTestId("readiness-rule").textContent,
    ).toContain("rule inputs incomplete");
    expect(
      within(initialReadiness).getByTestId("readiness-rule").textContent,
    ).toContain("mechanics results remain reviewable only");
    expect(
      within(initialReadiness).getByTestId("readiness-diagnostics").textContent,
    ).toContain("1 diagnostic");
    expect(
      within(initialReadiness).getByTestId("readiness-diagnostics").textContent,
    ).toContain("1 warning");
    expect(
      within(initialReadiness).getByTestId("readiness-professional")
        .textContent,
    ).toContain("human review required");
    expect(screen.getByTestId("solve-job-summary").textContent).toContain(
      "state=not_started",
    );
    expect(screen.getByTestId("solve-job-summary").textContent).toContain(
      "events=1",
    );
    expect(screen.getByTestId("solve-job-summary").textContent).toContain(
      "result_rows=0",
    );
    expect(screen.getByTestId("solve-job-summary").textContent).toContain(
      "cancellation_requested=false",
    );
    expect(screen.getByTestId("solve-job-progress").textContent).toContain(
      "not_started",
    );
    expect(screen.getByTestId("solve-job-progress").textContent).toContain(
      "preview_service_event_state_only_no_percent_stream",
    );
    expect(screen.getByTestId("solve-job-progress").textContent).toContain(
      "percentages_synthesized=false",
    );
    expect(screen.getByTestId("solve-job-cancellation").textContent).toContain(
      "seam=no_job_started",
    );
    expect(screen.getByTestId("solve-job-cancellation").textContent).toContain(
      "token=none_no_active_backend_job",
    );
    expect(screen.getByTestId("solve-job-cancellation").textContent).toContain(
      "success_claimed=false",
    );
    expect(screen.getByTestId("solve-job-binding").textContent).toContain(
      "not generated",
    );
    expect(screen.getByTestId("solve-job-unit-policy").textContent).toContain(
      "model=angle=rad,force=N,length=m",
    );
    expect(screen.getByTestId("solve-job-unit-policy").textContent).toContain(
      "results=none",
    );
    expect(screen.getByTestId("solve-job-unit-policy").textContent).toContain(
      "rows=0",
    );
    expect(screen.getByTestId("solve-job-unit-policy").textContent).toContain(
      "conversion=false",
    );
    expect(screen.getByTestId("solve-job-boundary").textContent).toContain(
      "private payload=false",
    );
    expect(screen.getByTestId("cancel-mechanics-preview")).toBeDisabled();
    const solveJobHref =
      screen.getByTestId("solve-job-export-link").getAttribute("href") ?? "";
    const solveJobPacket = JSON.parse(
      decodeURIComponent(solveJobHref.split(",", 2)[1]),
    );
    expect(solveJobPacket.document_kind).toBe(
      "openpipestress.technical_preview.solve_job_audit",
    );
    expect(solveJobPacket.deliverable_refs).toContain("DEL-07-07");
    expect(solveJobPacket.scope_items).toContain("SOW-055");
    expect(solveJobPacket.summary.job_state).toBe("not_started");
    expect(solveJobPacket.summary.event_count).toBe(1);
    expect(solveJobPacket.summary.result_row_count).toBe(0);
    expect(solveJobPacket.summary.cancellation_requested).toBe(false);
    expect(solveJobPacket.progress_contract.percentages_synthesized).toBe(
      false,
    );
    expect(
      solveJobPacket.progress_contract.backend_percent_stream_available,
    ).toBe(false);
    expect(solveJobPacket.cancellation.backend_job_seam).toBe("no_job_started");
    expect(solveJobPacket.cancellation.backend_job_id).toBe(null);
    expect(solveJobPacket.cancellation.backend_cancellation_token).toBe(
      "none_no_active_backend_job",
    );
    expect(solveJobPacket.cancellation.cancellation_scope).toBe(
      "ui_request_record_only_no_backend_job",
    );
    expect(solveJobPacket.cancellation.mutates_solver_process_directly).toBe(
      false,
    );
    expect(solveJobPacket.cancellation.cancellation_success_claimed).toBe(
      false,
    );
    expect(solveJobPacket.unit_policy_evidence.unit_system_ref.ref).toBe(
      "unit-system:dec-018-si-dual-display",
    );
    expect(solveJobPacket.unit_policy_evidence.storage_convention).toBe(
      "entered_units_preserved",
    );
    expect(solveJobPacket.unit_policy_evidence.solve_job_unit_policy).toBe(
      "solve_job_audit_records_model_and_result_units_without_conversion",
    );
    expect(solveJobPacket.unit_policy_evidence.model_units).toEqual({
      angle: "rad",
      force: "N",
      length: "m",
      pressure: "Pa",
      stress: "MPa",
      temperature: "degC",
    });
    expect(solveJobPacket.unit_policy_evidence.result_units).toEqual([]);
    expect(solveJobPacket.unit_policy_evidence.result_row_count).toBe(0);
    expect(solveJobPacket.unit_policy_evidence.analysis_run_ref.ref).toBe(
      "not generated",
    );
    expect(solveJobPacket.unit_policy_evidence.conversion_performed).toBe(
      false,
    );
    expect(solveJobPacket.private_payload_included).toBe(false);
    expect(solveJobPacket.protected_content_included).toBe(false);
    expect(solveJobPacket.release_or_professional_claim).toBe(false);
    expect(
      solveJobPacket.professional_boundary.software_makes_compliance_claim,
    ).toBe(false);
    const ruleCheck = await screen.findByLabelText("Rule-check completeness");
    expect(
      within(ruleCheck).getByTestId("rule-check-summary").textContent,
    ).toContain("3 review findings");
    expect(
      within(ruleCheck).getByTestId("rule-check-summary").textContent,
    ).toContain("rule_check_blocked=true");
    expect(
      within(ruleCheck).getByTestId("rule-check-summary").textContent,
    ).toContain("mechanics_reviewable=false");
    expect(
      within(ruleCheck).getByTestId("rule-check-unit-policy").textContent,
    ).toContain("rule_input_units=explicit_or_blocking");
    expect(
      within(ruleCheck).getByTestId("rule-check-unit-policy").textContent,
    ).toContain("diagnostic=RULE_UNIT_MISMATCH");
    expect(
      within(ruleCheck).getByTestId("rule-check-unit-policy").textContent,
    ).toContain("conversion=false");
    expect(
      within(ruleCheck).getByTestId("rule-check-status").textContent,
    ).toContain("not_performed_user_rule_inputs_missing");
    expect(
      within(ruleCheck).getByTestId("rule-check-boundary").textContent,
    ).toContain("silent defaults used=false");
    expect(
      within(ruleCheck).getByTestId("rule-check-boundary").textContent,
    ).toContain("no compliance claim");
    expect(
      within(ruleCheck).getByTestId(
        "rule-check-finding-required-rule-inputs-missing",
      ).textContent,
    ).toContain("RULE_CHECK_BLOCKING");
    expect(await screen.findByLabelText("Model tree")).toBeInTheDocument();
    expect(
      await screen.findByLabelText("Three.js pipe centerline viewport"),
    ).toBeInTheDocument();
    const runAudit = await screen.findByLabelText("Run audit");
    expect(
      within(runAudit).getByTestId("run-audit-empty").textContent,
    ).toContain(
      "Run mechanics preview to generate immutable model-state and analysis-run audit references",
    );
    expect(runAudit.textContent).toContain(
      "not a release or professional acceptance record",
    );
    const resultExport = await screen.findByLabelText("Result export audit");
    expect(
      within(resultExport).getByTestId("result-export-empty").textContent,
    ).toContain("schema-first local result envelope");
    const headlessRunner = await screen.findByLabelText(
      "Headless runner envelope",
    );
    expect(
      within(headlessRunner).getByTestId("headless-runner-summary").textContent,
    ).toContain("available");
    expect(
      within(headlessRunner).getByTestId("headless-runner-summary").textContent,
    ).toContain("job=TBD");
    expect(
      within(headlessRunner).getByTestId("headless-runner-summary").textContent,
    ).toContain("outputs=4");
    expect(
      within(headlessRunner).getByTestId("headless-runner-summary").textContent,
    ).toContain("result_refs=1");
    expect(
      within(headlessRunner).getByTestId("headless-runner-interface")
        .textContent,
    ).toContain("schema_first_headless_runner_contract");
    expect(
      within(headlessRunner).getByTestId("headless-runner-interface")
        .textContent,
    ).toContain("cli=openpipestress-runner");
    expect(
      within(headlessRunner).getByTestId("headless-runner-units").textContent,
    ).toContain("results=none");
    expect(
      within(headlessRunner).getByTestId("headless-runner-units").textContent,
    ).toContain("conversion=false");
    expect(
      within(headlessRunner).getByTestId("headless-runner-unit-witnesses")
        .textContent,
    ).toContain("count=0");
    expect(
      within(headlessRunner).getByTestId("headless-runner-runtime-tbds")
        .textContent,
    ).toContain("network=SETTLED_DEC_065");
    expect(
      within(headlessRunner).getByTestId("headless-runner-boundary")
        .textContent,
    ).toContain("network_required=false");
    expect(
      within(headlessRunner).getByTestId("headless-runner-boundary")
        .textContent,
    ).toContain(
      "no compliance, certification, sealing, authentication, or approval claim",
    );
    const headlessHref =
      within(headlessRunner)
        .getByTestId("headless-runner-export-link")
        .getAttribute("href") ?? "";
    const headlessPacket = JSON.parse(
      decodeURIComponent(headlessHref.split(",", 2)[1]),
    );
    expect(headlessPacket.deliverable_id).toBe("DEL-10-05");
    expect(headlessPacket.package_id).toBe("PKG-10");
    expect(headlessPacket.scope_items).toContain("SOW-054");
    expect(headlessPacket.scope_items).toContain("SOW-032");
    expect(headlessPacket.objectives).toContain("OBJ-008");
    expect(headlessPacket.objectives).toContain("OBJ-009");
    expect(headlessPacket.objectives).toContain("OBJ-012");
    expect(headlessPacket.runner_status.interface_kind).toBe(
      "schema_first_headless_runner_contract",
    );
    expect(
      headlessPacket.runner_status.physical_project_container
        .direct_sql_access_allowed,
    ).toBe(false);
    expect(
      headlessPacket.runner_status.physical_project_container.network_required,
    ).toBe(false);
    expect(headlessPacket.runner_status.final_cli_command_syntax).toContain(
      "openpipestress-runner <solve|validate-input|export-results|run-benchmark|run-regression>",
    );
    expect(headlessPacket.runner_status.process_invocation).toBe(
      "single_foreground_local_process",
    );
    expect(headlessPacket.runner_status.network_access).toBe("none");
    expect(headlessPacket.runner_status.filesystem_mutation_policy).toBe(
      "stdout_default_explicit_output_path_only",
    );
    expect(headlessPacket.tbd_decisions.final_cli_command_syntax).toBe(
      "SETTLED_DEC_065",
    );
    expect(headlessPacket.tbd_decisions.process_invocation).toBe(
      "SETTLED_DEC_065",
    );
    expect(headlessPacket.tbd_decisions.network_access).toBe("SETTLED_DEC_065");
    expect(headlessPacket.tbd_decisions.filesystem_mutation_policy).toBe(
      "SETTLED_DEC_065",
    );
    expect(headlessPacket.request.operation).toBe("solve");
    expect(headlessPacket.request.requested_outputs).toContain(
      "result_envelope",
    );
    expect(headlessPacket.request.requested_outputs).toContain(
      "audit_manifest",
    );
    expect(headlessPacket.request.privacy.local_only).toBe(true);
    expect(headlessPacket.request.privacy.telemetry_allowed).toBe(false);
    expect(headlessPacket.result.job.state).toBe("TBD");
    expect(
      headlessPacket.result.unit_system_disclosure.unit_system_ref.ref,
    ).toBe("unit-system:dec-018-si-dual-display");
    expect(headlessPacket.result.unit_system_disclosure.result_units).toEqual(
      [],
    );
    expect(
      headlessPacket.result.unit_system_disclosure.conversion_performed,
    ).toBe(false);
    expect(headlessPacket.result.unit_preservation_witnesses).toEqual([]);
    expect(headlessPacket.result.analysis_status).toContain("MODEL_INCOMPLETE");
    expect(headlessPacket.result.analysis_status).toContain(
      "HUMAN_REVIEW_REQUIRED",
    );
    expect(headlessPacket.result.result_envelope_ref.compatibility).toBe(
      "schema_first_json_result_envelope",
    );
    expect(headlessPacket.result.result_refs).toHaveLength(1);
    expect(headlessPacket.result.checksums[0].algorithm).toBe("TBD");
    expect(headlessPacket.result.privacy.local_only).toBe(true);
    expect(
      headlessPacket.result.professional_boundary
        .software_makes_compliance_claim,
    ).toBe(false);
    const adapterFramework = await screen.findByLabelText(
      "Adapter framework envelope",
    );
    expect(
      within(adapterFramework).getByTestId("adapter-framework-summary")
        .textContent,
    ).toContain("available");
    expect(
      within(adapterFramework).getByTestId("adapter-framework-summary")
        .textContent,
    ).toContain("capabilities=4");
    expect(
      within(adapterFramework).getByTestId("adapter-framework-summary")
        .textContent,
    ).toContain("parse=not_parsed_by_framework");
    expect(
      within(adapterFramework).getByTestId("adapter-framework-status")
        .textContent,
    ).toContain("schema_first_format_neutral_adapter_framework");
    expect(
      within(adapterFramework).getByTestId("adapter-framework-status")
        .textContent,
    ).toContain("formats=TBD");
    expect(
      within(adapterFramework).getByTestId("adapter-framework-capabilities")
        .textContent,
    ).toContain("validate_payload");
    expect(
      within(adapterFramework).getByTestId("adapter-framework-validation")
        .textContent,
    ).toContain("schema=required");
    expect(
      within(adapterFramework).getByTestId("adapter-framework-units")
        .textContent,
    ).toContain("conversion=false");
    expect(
      within(adapterFramework).getByTestId("adapter-framework-units")
        .textContent,
    ).toContain("witnesses=1");
    expect(
      within(adapterFramework).getByTestId("adapter-framework-no-bypass")
        .textContent,
    ).toContain("direct_sql=false");
    expect(
      within(adapterFramework).getByTestId("adapter-framework-no-bypass")
        .textContent,
    ).toContain("network=false");
    expect(
      within(adapterFramework).getByTestId("adapter-framework-runtime-tbds")
        .textContent,
    ).toContain("plugin=TBD");
    expect(
      within(adapterFramework).getByTestId("adapter-framework-boundary")
        .textContent,
    ).toContain("private_redacted=true");
    const adapterHref =
      within(adapterFramework)
        .getByTestId("adapter-framework-export-link")
        .getAttribute("href") ?? "";
    const adapterPacket = JSON.parse(
      decodeURIComponent(adapterHref.split(",", 2)[1]),
    );
    expect(adapterPacket.deliverable_id).toBe("DEL-10-02");
    expect(adapterPacket.package_id).toBe("PKG-10");
    expect(adapterPacket.scope_item).toBe("SOW-030");
    expect(adapterPacket.objective).toBe("OBJ-009");
    expect(adapterPacket.framework_status.interface_kind).toBe(
      "schema_first_format_neutral_adapter_framework",
    );
    expect(adapterPacket.framework_status.external_format_list).toBe("TBD");
    expect(adapterPacket.framework_status.public_transport_protocol).toBe(
      "TBD",
    );
    expect(adapterPacket.tbd_decisions.plugin_runtime).toBe("TBD");
    expect(adapterPacket.tbd_decisions.package_scripts).toBe("TBD");
    expect(adapterPacket.adapter_declaration.adapter_id).toBe(
      "ops.adapter.desktop_preview",
    );
    expect(adapterPacket.adapter_declaration.capabilities).toContain(
      "export_model",
    );
    expect(adapterPacket.adapter_declaration.capabilities).toContain(
      "export_results",
    );
    expect(adapterPacket.adapter_declaration.capabilities).toContain(
      "validate_payload",
    );
    expect(
      adapterPacket.adapter_declaration.no_bypass_controls
        .must_route_persistence_through_application_services,
    ).toBe(true);
    expect(
      adapterPacket.adapter_declaration.no_bypass_controls
        .must_not_expose_sql_or_raw_sqlite,
    ).toBe(true);
    expect(
      adapterPacket.adapter_declaration.no_bypass_controls
        .must_not_access_network,
    ).toBe(true);
    expect(
      adapterPacket.adapter_declaration.no_bypass_controls
        .must_not_claim_code_compliance,
    ).toBe(true);
    expect(adapterPacket.validation_plan.schema_validation).toBe("required");
    expect(adapterPacket.validation_plan.unit_validation).toBe("required");
    expect(adapterPacket.validation_plan.protected_content_screening).toBe(
      "required",
    );
    expect(adapterPacket.validation_plan.export_review).toBe(
      "required_before_shared_payload",
    );
    expect(adapterPacket.unit_policy_evidence.unit_system_ref.ref).toBe(
      "unit-system:dec-018-si-dual-display",
    );
    expect(adapterPacket.unit_policy_evidence.storage_convention).toBe(
      "entered_units_preserved",
    );
    expect(adapterPacket.unit_policy_evidence.conversion_policy).toBe(
      "adapter_framework_declares_unit_validation_no_format_conversion",
    );
    expect(adapterPacket.unit_policy_evidence.conversion_performed).toBe(false);
    expect(adapterPacket.unit_policy_evidence.framework_scope).toBe(
      "format_neutral_adapter_contract_metadata_only",
    );
    expect(adapterPacket.unit_policy_evidence.framework_unit_policy).toBe(
      "unit_validation_required_before_adapter_payload_exchange",
    );
    expect(adapterPacket.unit_policy_evidence.witness_policy).toBe(
      "record_framework_unit_policy_without_claiming_target_writer_conversion",
    );
    expect(adapterPacket.unit_policy_evidence.witness_count).toBe(1);
    expect(adapterPacket.unit_policy_evidence.adapter_ref.ref_id).toBe(
      "ops.adapter.desktop_preview",
    );
    expect(adapterPacket.operation_result.parse_status).toBe(
      "not_parsed_by_framework",
    );
    expect(
      adapterPacket.operation_result.result_envelope_ref.compatibility,
    ).toBe("schema_first_json_result_envelope");
    expect(adapterPacket.operation_result.checksums[0].algorithm).toBe("TBD");
    expect(adapterPacket.operation_result.privacy.telemetry_allowed).toBe(
      false,
    );
    expect(
      adapterPacket.operation_result.professional_boundary
        .software_makes_compliance_claim,
    ).toBe(false);
    expect(
      adapterPacket.operation_result.professional_boundary
        .software_makes_security_certification_claim,
    ).toBe(false);
    const localFea = await screen.findByLabelText("Local FEA handoff");
    expect(
      within(localFea).getByTestId("local-fea-empty").textContent,
    ).toContain("target-neutral local shell/solid FEA");
    expect(localFea.textContent).toContain("no mesh");
    expect(localFea.textContent).toContain("external solver");
    const externalProver = await screen.findByLabelText(
      "External prover boundary metadata",
    );
    expect(
      within(externalProver).getByTestId("external-prover-summary").textContent,
    ).toContain("available");
    expect(
      within(externalProver).getByTestId("external-prover-summary").textContent,
    ).toContain("metadata=non_authoritative_workflow_metadata");
    expect(
      within(externalProver).getByTestId("external-prover-summary").textContent,
    ).toContain("refs=1");
    expect(
      within(externalProver).getByTestId("external-prover-summary").textContent,
    ).toContain("diagnostics=4");
    expect(
      within(externalProver).getByTestId("external-prover-contract")
        .textContent,
    ).toContain("DEL-15-04");
    expect(
      within(externalProver).getByTestId("external-prover-groups").textContent,
    ).toContain("attachments=1");
    expect(
      within(externalProver).getByTestId("external-prover-context-links")
        .textContent,
    ).toContain("handoff=1");
    expect(
      within(externalProver).getByTestId("external-prover-unsupported-flags")
        .textContent,
    ).toContain("external_solver_not_invoked");
    expect(
      within(externalProver).getByTestId("external-prover-run-boundary")
        .textContent,
    ).toContain("tool_invoked=false");
    expect(
      within(externalProver).getByTestId("external-prover-unit-policy")
        .textContent,
    ).toContain("length=m");
    expect(
      within(externalProver).getByTestId("external-prover-unit-policy")
        .textContent,
    ).toContain("conversion=false");
    expect(
      within(externalProver).getByTestId("external-prover-authority-boundary")
        .textContent,
    ).toContain("compliance=false");
    const externalHref =
      within(externalProver)
        .getByTestId("external-prover-export-link")
        .getAttribute("href") ?? "";
    const externalPacket = JSON.parse(
      decodeURIComponent(externalHref.split(",", 2)[1]),
    );
    expect(externalPacket.deliverable_id).toBe("DEL-15-04");
    expect(externalPacket.package_id).toBe("PKG-15");
    expect(externalPacket.scope_item).toBe("SOW-075");
    expect(externalPacket.objectives).toContain("OBJ-017");
    expect(externalPacket.objectives).toContain("OBJ-018");
    expect(externalPacket.metadata_contract_status).toBe(
      "non_authoritative_workflow_metadata",
    );
    expect(externalPacket.tags).toContain("run-context-pending");
    expect(externalPacket.external_references[0].external_ref.ref).toBe(
      "external:desktop-preview-metadata-only",
    );
    expect(externalPacket.attachments[0].payload_embedded).toBe(false);
    expect(externalPacket.handoff_package_refs[0].ref.ref).toBe("handoff:TBD");
    expect(externalPacket.immutable_model_state_refs[0].ref.ref).toBe(
      "state:TBD",
    );
    expect(externalPacket.unit_policy_evidence.storage_convention).toBe(
      "entered_units_preserved",
    );
    expect(externalPacket.unit_policy_evidence.conversion_policy).toBe(
      "external_prover_metadata_records_units_without_target_conversion",
    );
    expect(externalPacket.unit_policy_evidence.conversion_performed).toBe(
      false,
    );
    expect(externalPacket.unit_policy_evidence.external_prover_scope).toBe(
      "metadata_only_external_review_context",
    );
    expect(
      externalPacket.unit_policy_evidence.external_prover_unit_policy,
    ).toBe("record_units_for_external_reviewer_without_invoking_target_solver");
    expect(externalPacket.unit_policy_evidence.witness_policy).toBe(
      "record_external_prover_unit_policy_without_claiming_target_writer_conversion",
    );
    expect(externalPacket.unit_policy_evidence.witness_count).toBe(1);
    expect(externalPacket.unit_policy_evidence.analysis_run_ref.ref).toBe(
      "analysis-run:TBD",
    );
    expect(externalPacket.unit_policy_evidence.external_ref.ref).toBe(
      "external:desktop-preview-metadata-only",
    );
    expect(externalPacket.unsupported_target_flags).toHaveLength(3);
    expect(externalPacket.diagnostics).toHaveLength(4);
    expect(externalPacket.professional_boundary.external_tool_invoked).toBe(
      false,
    );
    expect(
      externalPacket.professional_boundary.commercial_result_payload_ingested,
    ).toBe(false);
    expect(
      externalPacket.professional_boundary
        .software_creates_external_validation_record,
    ).toBe(false);
    expect(
      externalPacket.professional_boundary.software_makes_compliance_claim,
    ).toBe(false);
    const reviewGeometry = await screen.findByLabelText(
      "Review geometry export",
    );
    expect(
      within(reviewGeometry).getByTestId("review-geometry-summary").textContent,
    ).toContain("available");
    expect(
      within(reviewGeometry).getByTestId("review-geometry-summary").textContent,
    ).toContain("format=glTF_2_0_json_preview");
    expect(
      within(reviewGeometry).getByTestId("review-geometry-summary").textContent,
    ).toContain("segments=4");
    expect(
      within(reviewGeometry).getByTestId("review-geometry-summary").textContent,
    ).toContain("nodes=5");
    expect(
      within(reviewGeometry).getByTestId("review-geometry-summary").textContent,
    ).toContain("stable_ids=26");
    expect(
      within(reviewGeometry).getByTestId("review-geometry-format").textContent,
    ).toContain("asset=2.0");
    expect(
      within(reviewGeometry).getByTestId("review-geometry-format").textContent,
    ).toContain("units=m");
    expect(
      within(reviewGeometry).getByTestId("review-geometry-state-binding")
        .textContent,
    ).toContain("not generated");
    expect(
      within(reviewGeometry).getByTestId("review-geometry-state-binding")
        .textContent,
    ).toContain("state:TBD");
    expect(
      within(reviewGeometry).getByTestId("review-geometry-coverage")
        .textContent,
    ).toContain("pipe_segments");
    expect(
      within(reviewGeometry).getByTestId("review-geometry-coverage")
        .textContent,
    ).toContain("supports");
    expect(
      within(reviewGeometry).getByTestId("review-geometry-coverage")
        .textContent,
    ).toContain("load_cases");
    expect(
      within(reviewGeometry).getByTestId("review-geometry-stable-ids")
        .textContent,
    ).toContain("sidecar=required");
    expect(
      within(reviewGeometry).getByTestId("review-geometry-unit-witnesses")
        .textContent,
    ).toContain("count=75");
    expect(
      within(reviewGeometry).getByTestId("review-geometry-unit-witnesses")
        .textContent,
    ).toContain("conversion=false");
    expect(
      within(reviewGeometry).getByTestId("review-geometry-unit-witnesses")
        .textContent,
    ).toContain("target=m");
    expect(
      within(reviewGeometry).getByTestId("review-geometry-boundary")
        .textContent,
    ).toContain("visual_review_geometry_only");
    expect(
      within(reviewGeometry).getByTestId("review-geometry-boundary")
        .textContent,
    ).toContain("solver_geometry_equivalence=false");
    const geometryHref =
      within(reviewGeometry)
        .getByTestId("review-geometry-export-link")
        .getAttribute("href") ?? "";
    const geometryPacket = JSON.parse(
      decodeURIComponent(geometryHref.split(",", 2)[1]),
    );
    expect(geometryPacket.document_kind).toBe(
      "openpipestress.technical_preview.review_geometry_export",
    );
    expect(geometryPacket.deliverable_id).toBe("DEL-17-08");
    expect(geometryPacket.package_id).toBe("PKG-17");
    expect(geometryPacket.scope_items).toContain("SOW-030");
    expect(geometryPacket.scope_items).toContain("SOW-074");
    expect(geometryPacket.objectives).toContain("OBJ-009");
    expect(geometryPacket.objectives).toContain("OBJ-017");
    expect(geometryPacket.review_geometry_status).toBe(
      "visual_review_geometry_only",
    );
    expect(geometryPacket.review_geometry_profile.gltf_version_basis).toBe(
      "2.0",
    );
    expect(geometryPacket.review_geometry_profile.target_artifact).toBe(
      "glTF_2_0_json_preview",
    );
    expect(
      geometryPacket.review_geometry_profile.glb_binary_writer_status,
    ).toBe("TBD");
    expect(geometryPacket.review_geometry_profile.unit_witness_policy).toBe(
      "coordinate_sidecar_required_for_emitted_positions",
    );
    expect(geometryPacket.unit_system_disclosure.unit_system_ref.ref).toBe(
      "unit-system:dec-018-si-dual-display",
    );
    expect(geometryPacket.unit_system_disclosure.conversion_performed).toBe(
      false,
    );
    expect(
      geometryPacket.unit_system_disclosure.target_export_units.coordinates,
    ).toBe("m");
    expect(geometryPacket.unit_system_disclosure.axis_transform_policy).toBe(
      "preview_z_up_to_gltf_y_up_rotation_x_minus_90",
    );
    expect(geometryPacket.geometry_summary.pipe_segment_count).toBe(4);
    expect(geometryPacket.geometry_summary.stable_id_count).toBe(26);
    expect(geometryPacket.gltf_asset.asset.version).toBe("2.0");
    expect(geometryPacket.gltf_asset.meshes[0].primitives[0].mode).toBe(1);
    expect(geometryPacket.gltf_asset.meshes[1].primitives[0].mode).toBe(0);
    expect(geometryPacket.gltf_asset.buffers[0].byteLength).toBeGreaterThan(0);
    expect(geometryPacket.gltf_asset.extras.gltf_boundary).toBe(
      "visual_review_only_not_solver_geometry",
    );
    expect(geometryPacket.sidecar_id_map.entries).toHaveLength(26);
    expect(
      geometryPacket.package_members.map(
        (member: { filename: string }) => member.filename,
      ),
    ).toContain("coordinate_unit_witnesses.json");
    expect(geometryPacket.coordinate_unit_witnesses).toHaveLength(75);
    expect(
      geometryPacket.coordinate_unit_witnesses.find(
        (witness: { witness_id: string }) =>
          witness.witness_id ===
          "review-geometry-unit:pipe:P-120:from.position.y",
      ),
    ).toMatchObject({
      source_ref: {
        ref_type: "pipe_segment_endpoint",
        ref_id: "pipe:P-120",
        source_node_ref: "node:N-120",
        field_path: "from.position.y",
      },
      target_ref: {
        group_name: "OpenPipeStress pipe centerline review geometry",
        vertex_index: 4,
        coordinate_index: 2,
        target_axis: "z",
      },
      source_quantity: {
        value: 2.4,
        unit: "m",
        dimension: "length",
        axis: "y",
      },
      target_quantity: {
        value: -2.4,
        unit: "m",
        dimension: "length",
        axis: "z",
      },
      conversion_performed: false,
      axis_transform: "preview_z_up_to_gltf_y_up_rotation_x_minus_90",
      conversion_status: "unit_preserved_axis_transformed",
    });
    expect(
      geometryPacket.sidecar_id_map.entries.find(
        (entry: { stable_id: string }) => entry.stable_id === "pipe:P-120",
      ).export_status,
    ).toBe("emitted");
    expect(
      geometryPacket.sidecar_id_map.entries.find(
        (entry: { stable_id: string }) => entry.stable_id === "support:S-120",
      ).export_status,
    ).toBe("approximated");
    expect(
      geometryPacket.loss_report.entries.find(
        (entry: { loss_id: string }) =>
          entry.loss_id === "loss:glb-and-viewer-behavior-tbd",
      ).category,
    ).toBe("TBD");
    expect(geometryPacket.private_payload_included).toBe(false);
    expect(geometryPacket.protected_content_included).toBe(false);
    expect(geometryPacket.solver_geometry_equivalence_claim).toBe(false);
    expect(geometryPacket.professional_validation_claim).toBe(false);
    expect(geometryPacket.target_compatibility_claim).toBe(false);
    expect(
      geometryPacket.professional_boundary.software_makes_compliance_claim,
    ).toBe(false);
    const pcfExport = await screen.findByLabelText("Conservative PCF export");
    expect(
      within(pcfExport).getByTestId("pcf-export-summary").textContent,
    ).toContain("available");
    expect(
      within(pcfExport).getByTestId("pcf-export-summary").textContent,
    ).toContain("segments=4");
    expect(
      within(pcfExport).getByTestId("pcf-export-summary").textContent,
    ).toContain("nodes=5");
    expect(
      within(pcfExport).getByTestId("pcf-export-summary").textContent,
    ).toContain("validation=blocked_missing_explicit_pcf_target_fields");
    expect(
      within(pcfExport).getByTestId("pcf-export-profile").textContent,
    ).toContain("target_version=TBD");
    expect(
      within(pcfExport).getByTestId("pcf-export-state-binding").textContent,
    ).toContain("not generated");
    expect(
      within(pcfExport).getByTestId("pcf-export-state-binding").textContent,
    ).toContain("state:TBD");
    expect(
      within(pcfExport).getByTestId("pcf-export-coverage").textContent,
    ).toContain("pipe_segments");
    expect(
      within(pcfExport).getByTestId("pcf-export-coverage").textContent,
    ).toContain("nominal_size");
    expect(
      within(pcfExport).getByTestId("pcf-export-units").textContent,
    ).toContain("source=angle=rad");
    expect(
      within(pcfExport).getByTestId("pcf-export-units").textContent,
    ).toContain("length=m");
    expect(
      within(pcfExport).getByTestId("pcf-export-units").textContent,
    ).toContain("target=bolt_diameter=MM");
    expect(
      within(pcfExport).getByTestId("pcf-export-units").textContent,
    ).toContain("coordinates=MM");
    expect(
      within(pcfExport).getByTestId("pcf-export-units").textContent,
    ).toContain("conversion=true");
    expect(
      within(pcfExport).getByTestId("pcf-export-conversion-witnesses")
        .textContent,
    ).toContain("count=23");
    expect(
      within(pcfExport).getByTestId("pcf-export-conversion-witnesses")
        .textContent,
    ).toContain("target_length=MM");
    expect(
      within(pcfExport).getByTestId("pcf-export-package").textContent,
    ).toContain("members=8");
    expect(
      within(pcfExport).getByTestId("pcf-export-package").textContent,
    ).toContain("stable_ids=4");
    expect(
      within(pcfExport).getByTestId("pcf-export-package").textContent,
    ).toContain("diagnostics=4");
    await waitFor(() =>
      expect(
        within(pcfExport).getByTestId("pcf-export-package").textContent,
      ).toContain("package_hash=computed_local_preview_sha256"),
    );
    expect(
      within(pcfExport).getByTestId("pcf-export-boundary").textContent,
    ).toContain("target_compatibility=false");
    expect(
      within(pcfExport).getByTestId("pcf-export-boundary").textContent,
    ).toContain("solver_validation=false");
    const pcfHref =
      within(pcfExport).getByTestId("pcf-export-link").getAttribute("href") ??
      "";
    const pcfPacket = JSON.parse(decodeURIComponent(pcfHref.split(",", 2)[1]));
    expect(pcfPacket.document_kind).toBe(
      "openpipestress.technical_preview.conservative_pcf_export_package",
    );
    expect(pcfPacket.manifest.canonical_package_hash_status).toBe(
      "computed_local_preview_sha256",
    );
    expect(pcfPacket.manifest.canonical_package_hash.value).toMatch(
      /^sha256:[0-9a-f]{64}$/,
    );
    expect(pcfPacket.validation_report.hash_validation_status).toBe(
      "package_hash_computed_local_preview_not_independently_validated",
    );
    expect(pcfPacket.deliverable_id).toBe("DEL-17-07");
    expect(pcfPacket.package_id).toBe("PKG-17");
    expect(pcfPacket.scope_items).toContain("SOW-030");
    expect(pcfPacket.scope_items).toContain("SOW-074");
    expect(pcfPacket.objectives).toContain("OBJ-009");
    expect(pcfPacket.objectives).toContain("OBJ-017");
    expect(pcfPacket.objectives).toContain("OBJ-018");
    expect(pcfPacket.export_profile.target_family).toBe("pcf");
    expect(pcfPacket.export_profile.target_profile_version_basis).toBe("TBD");
    expect(pcfPacket.export_profile.identity_policy).toBe(
      "authoritative_sidecar_id_map",
    );
    expect(pcfPacket.unit_system_disclosure.unit_system_ref.ref).toBe(
      "unit-system:dec-018-si-dual-display",
    );
    expect(pcfPacket.unit_system_disclosure.model_units.length).toBe("m");
    expect(pcfPacket.unit_system_disclosure.model_units.force).toBe("N");
    expect(
      pcfPacket.unit_system_disclosure.target_export_units.coordinates,
    ).toBe("MM");
    expect(pcfPacket.unit_system_disclosure.conversion_performed).toBe(true);
    expect(pcfPacket.unit_system_disclosure.conversion_scope).toContain(
      "node.coordinates",
    );
    expect(pcfPacket.unit_system_disclosure.protected_content_included).toBe(
      false,
    );
    expect(pcfPacket.conversion_witnesses).toHaveLength(23);
    expect(
      pcfPacket.manifest.package_members.map(
        (item: { member_role: string }) => item.member_role,
      ),
    ).toContain("conversion_witnesses");
    const pcfOdWitness = pcfPacket.conversion_witnesses.find(
      (item: { witness_id: string }) =>
        item.witness_id === "pcf-conversion:pipe-p-120:outside_diameter",
    );
    expect(pcfOdWitness.source_quantity).toEqual({
      value: 0.168,
      unit: "m",
      dimension: "length",
    });
    expect(pcfOdWitness.target_quantity).toEqual({
      value: 168,
      unit: "MM",
      target_field: "pcf_payload.pipe_segments.pipe:P-120.outside_diameter",
    });
    expect(pcfOdWitness.conversion_factor_to_target).toBe(1000);
    expect(pcfOdWitness.conversion_status).toBe("converted_to_target_unit");
    const pcfNodeWitness = pcfPacket.conversion_witnesses.find(
      (item: { witness_id: string }) =>
        item.witness_id === "pcf-conversion:node-n-100:position:x",
    );
    expect(pcfNodeWitness.source_quantity).toEqual({
      value: 0,
      unit: "m",
      dimension: "length",
    });
    expect(pcfNodeWitness.target_quantity.unit).toBe("MM");
    expect(pcfPacket.pcf_payload.nodes).toHaveLength(5);
    expect(pcfPacket.pcf_payload.pipe_segments).toHaveLength(4);
    expect(
      pcfPacket.pcf_payload.pipe_segments.find(
        (item: { element_id: string }) => item.element_id === "pipe:P-120",
      ).outside_diameter,
    ).toBe("168");
    expect(pcfPacket.pcf_text).toContain("UNITS-CO-ORDS MM");
    expect(pcfPacket.pcf_text).toContain("OUTSIDE-DIAMETER 168");
    expect(pcfPacket.pcf_text).toContain("COMPONENT-IDENTIFIER OPS-PIPE-P-120");
    expect(pcfPacket.pcf_text).toContain("TBD_SOURCE_REQUIRED");
    expect(pcfPacket.pcf_text).toContain("END-ISOGEN");
    expect(pcfPacket.stable_id_map).toHaveLength(4);
    expect(pcfPacket.loss_report).toHaveLength(6);
    expect(
      pcfPacket.loss_report.map(
        (entry: { category: string }) => entry.category,
      ),
    ).toContain("tbd");
    expect(pcfPacket.validation_report.validation_status).toBe(
      "blocked_missing_explicit_pcf_target_fields",
    );
    expect(pcfPacket.validation_report.schema_validation_status).toBe(
      "desktop_preview_shape_aligned_not_runtime_json_schema_validated",
    );
    expect(
      pcfPacket.validation_report.checks.map(
        (item: { check_id: string }) => item.check_id,
      ),
    ).toContain("conversion_witness_per_converted_length_field");
    expect(
      pcfPacket.diagnostics.map((item: { code: string }) => item.code),
    ).toContain("PCF-NOMINAL-SIZE-TBD");
    expect(pcfPacket.private_payload_included).toBe(false);
    expect(pcfPacket.protected_content_included).toBe(false);
    expect(pcfPacket.target_compatibility_claim).toBe(false);
    expect(pcfPacket.solver_validation_claim).toBe(false);
    expect(pcfPacket.code_compliance_claim).toBe(false);
    expect(pcfPacket.professional_reliance_claim).toBe(false);
    const pcfTextHref =
      within(pcfExport).getByTestId("pcf-text-link").getAttribute("href") ?? "";
    expect(decodeURIComponent(pcfTextHref.split(",", 2)[1])).toContain(
      "END-ISOGEN",
    );
    const caepipeMbfExport = await screen.findByLabelText("CAEPIPE MBF export");
    expect(
      within(caepipeMbfExport).getByTestId("caepipe-mbf-summary").textContent,
    ).toContain("available");
    expect(
      within(caepipeMbfExport).getByTestId("caepipe-mbf-summary").textContent,
    ).toContain("nodes=5");
    expect(
      within(caepipeMbfExport).getByTestId("caepipe-mbf-summary").textContent,
    ).toContain("pipes=4");
    expect(
      within(caepipeMbfExport).getByTestId("caepipe-mbf-summary").textContent,
    ).toContain("supports=7");
    expect(
      within(caepipeMbfExport).getByTestId("caepipe-mbf-summary").textContent,
    ).toContain("validation=boundary_checked");
    expect(
      within(caepipeMbfExport).getByTestId("caepipe-mbf-profile").textContent,
    ).toContain("target_version=TBD-17-01-001");
    expect(
      within(caepipeMbfExport).getByTestId("caepipe-mbf-profile").textContent,
    ).toContain("subset=TBD-17-01-002");
    expect(
      within(caepipeMbfExport).getByTestId("caepipe-mbf-state-binding")
        .textContent,
    ).toContain("not generated");
    expect(
      within(caepipeMbfExport).getByTestId("caepipe-mbf-state-binding")
        .textContent,
    ).toContain("state:TBD");
    expect(
      within(caepipeMbfExport).getByTestId("caepipe-mbf-coverage").textContent,
    ).toContain("nodes");
    expect(
      within(caepipeMbfExport).getByTestId("caepipe-mbf-coverage").textContent,
    ).toContain("direct_stable_id");
    expect(
      within(caepipeMbfExport).getByTestId("caepipe-mbf-units").textContent,
    ).toContain("source=angle=rad");
    expect(
      within(caepipeMbfExport).getByTestId("caepipe-mbf-units").textContent,
    ).toContain("length=m");
    expect(
      within(caepipeMbfExport).getByTestId("caepipe-mbf-units").textContent,
    ).toContain("target=force=N");
    expect(
      within(caepipeMbfExport).getByTestId("caepipe-mbf-units").textContent,
    ).toContain("length=mm");
    expect(
      within(caepipeMbfExport).getByTestId("caepipe-mbf-units").textContent,
    ).toContain("conversion=true");
    expect(
      within(caepipeMbfExport).getByTestId("caepipe-mbf-conversion-witnesses")
        .textContent,
    ).toContain("count=15");
    expect(
      within(caepipeMbfExport).getByTestId("caepipe-mbf-conversion-witnesses")
        .textContent,
    ).toContain("target_length=mm");
    expect(
      within(caepipeMbfExport).getByTestId("caepipe-mbf-package").textContent,
    ).toContain("members=8");
    expect(
      within(caepipeMbfExport).getByTestId("caepipe-mbf-package").textContent,
    ).toContain("stable_ids=18");
    expect(
      within(caepipeMbfExport).getByTestId("caepipe-mbf-package").textContent,
    ).toContain("diagnostics=0");
    expect(
      within(caepipeMbfExport).getByTestId("caepipe-mbf-boundary").textContent,
    ).toContain("caepipe_compatibility=false");
    expect(
      within(caepipeMbfExport).getByTestId("caepipe-mbf-boundary").textContent,
    ).toContain("external_tool_invoked=false");
    await waitFor(() =>
      expect(
        within(caepipeMbfExport).getByTestId("caepipe-mbf-package").textContent,
      ).toContain("package_hash=computed_local_preview_sha256"),
    );
    const caepipeMbfHref =
      within(caepipeMbfExport)
        .getByTestId("caepipe-mbf-export-link")
        .getAttribute("href") ?? "";
    const caepipeMbfPacket = JSON.parse(
      decodeURIComponent(caepipeMbfHref.split(",", 2)[1]),
    );
    expect(caepipeMbfPacket.document_kind).toBe(
      "openpipestress.technical_preview.caepipe_mbf_export_package",
    );
    expect(caepipeMbfPacket.manifest.canonical_package_hash_status).toBe(
      "computed_local_preview_sha256",
    );
    expect(caepipeMbfPacket.manifest.canonical_package_hash.value).toMatch(
      /^sha256:[0-9a-f]{64}$/,
    );
    expect(caepipeMbfPacket.validation_report.hash_validation_status).toBe(
      "package_hash_computed_local_preview_not_independently_validated",
    );
    expect(caepipeMbfPacket.deliverable_id).toBe("DEL-17-04");
    expect(caepipeMbfPacket.package_id).toBe("PKG-17");
    expect(caepipeMbfPacket.scope_items).toContain("SOW-030");
    expect(caepipeMbfPacket.scope_items).toContain("SOW-074");
    expect(caepipeMbfPacket.scope_items).toContain("SOW-075");
    expect(caepipeMbfPacket.export_profile.target_family).toBe("caepipe_mbf");
    expect(caepipeMbfPacket.export_profile.target_version_basis).toBe(
      "TBD-17-01-001",
    );
    expect(caepipeMbfPacket.export_profile.record_subset_basis).toBe(
      "TBD-17-01-002",
    );
    expect(caepipeMbfPacket.export_profile.stable_id_policy).toBe(
      "sidecar_mapping_until_direct_mbf_carrier_confirmed",
    );
    expect(caepipeMbfPacket.export_profile.carried_tbd_refs).toContain(
      "TBD-17-01-003",
    );
    expect(caepipeMbfPacket.unit_system_disclosure.unit_system_ref.ref).toBe(
      "unit-system:dec-018-si-dual-display",
    );
    expect(caepipeMbfPacket.unit_system_disclosure.model_units.length).toBe(
      "m",
    );
    expect(
      caepipeMbfPacket.unit_system_disclosure.target_export_units.length,
    ).toBe("mm");
    expect(
      caepipeMbfPacket.unit_system_disclosure.target_export_units.force,
    ).toBe("N");
    expect(caepipeMbfPacket.unit_system_disclosure.conversion_performed).toBe(
      true,
    );
    expect(caepipeMbfPacket.unit_system_disclosure.conversion_scope).toContain(
      "node.coordinates",
    );
    expect(
      caepipeMbfPacket.unit_system_disclosure.protected_content_included,
    ).toBe(false);
    expect(caepipeMbfPacket.conversion_witnesses).toHaveLength(15);
    expect(
      caepipeMbfPacket.manifest.package_members.map(
        (item: { member_role: string }) => item.member_role,
      ),
    ).toContain("conversion_witnesses");
    const caepipeNodeWitness = caepipeMbfPacket.conversion_witnesses.find(
      (item: { witness_id: string }) =>
        item.witness_id === "caepipe-mbf-conversion:node-n-120:position:x",
    );
    expect(caepipeNodeWitness.source_quantity).toEqual({
      value: 3.2,
      unit: "m",
      dimension: "length",
    });
    expect(caepipeNodeWitness.target_quantity).toEqual({
      value: 3200,
      unit: "mm",
      target_field: "model_payload.nodes.node:N-120.x",
    });
    expect(caepipeNodeWitness.conversion_factor_to_target).toBe(1000);
    expect(caepipeNodeWitness.conversion_status).toBe(
      "converted_to_target_unit",
    );
    expect(caepipeMbfPacket.model_payload.nodes).toHaveLength(5);
    expect(caepipeMbfPacket.model_payload.elements).toHaveLength(4);
    expect(caepipeMbfPacket.model_payload.supports).toHaveLength(7);
    expect(caepipeMbfPacket.mbf_text).toContain("UNITS");
    expect(caepipeMbfPacket.mbf_text).toContain("UNIT,mm,N,degC");
    expect(caepipeMbfPacket.mbf_text).toContain("NODE,N003,3200,2400,0");
    expect(caepipeMbfPacket.mbf_text).toContain("PIPE,P003,N003,N004");
    expect(caepipeMbfPacket.mbf_text).toContain("END");
    expect(caepipeMbfPacket.stable_id_map).toHaveLength(18);
    expect(caepipeMbfPacket.loss_report).toHaveLength(6);
    expect(
      caepipeMbfPacket.loss_report.map(
        (entry: { category: string }) => entry.category,
      ),
    ).toContain("tbd");
    expect(caepipeMbfPacket.validation_report.validation_status).toBe(
      "boundary_checked",
    );
    expect(caepipeMbfPacket.validation_report.schema_validation_status).toBe(
      "desktop_preview_shape_aligned_not_runtime_json_schema_validated",
    );
    expect(
      caepipeMbfPacket.validation_report.checks.map(
        (item: { check_id: string }) => item.check_id,
      ),
    ).toContain("conversion_witness_per_node_coordinate_field");
    expect(caepipeMbfPacket.diagnostics).toHaveLength(0);
    expect(caepipeMbfPacket.private_payload_included).toBe(false);
    expect(caepipeMbfPacket.protected_content_included).toBe(false);
    expect(caepipeMbfPacket.caepipe_compatibility_claim).toBe(false);
    expect(caepipeMbfPacket.solver_validation_claim).toBe(false);
    expect(caepipeMbfPacket.code_compliance_claim).toBe(false);
    expect(caepipeMbfPacket.professional_reliance_claim).toBe(false);
    const caepipeMbfTextHref =
      within(caepipeMbfExport)
        .getByTestId("caepipe-mbf-text-link")
        .getAttribute("href") ?? "";
    expect(decodeURIComponent(caepipeMbfTextHref.split(",", 2)[1])).toContain(
      "LOAD_CASES",
    );
    const caepipeExternal = await screen.findByLabelText(
      "CAEPIPE external harness",
    );
    expect(
      within(caepipeExternal).getByTestId("caepipe-external-summary")
        .textContent,
    ).toContain("available");
    expect(
      within(caepipeExternal).getByTestId("caepipe-external-summary")
        .textContent,
    ).toContain("status=parser_only_evidence");
    expect(
      within(caepipeExternal).getByTestId("caepipe-external-summary")
        .textContent,
    ).toContain("rows=3");
    expect(
      within(caepipeExternal).getByTestId("caepipe-external-summary")
        .textContent,
    ).toContain("sections=2");
    expect(
      within(caepipeExternal).getByTestId("caepipe-external-summary")
        .textContent,
    ).toContain("external_invoked=false");
    expect(
      within(caepipeExternal).getByTestId("caepipe-external-harness")
        .textContent,
    ).toContain("config=absent");
    expect(
      within(caepipeExternal).getByTestId("caepipe-external-harness")
        .textContent,
    ).toContain("TBD-17-05-invocation-profile");
    expect(
      within(caepipeExternal).getByTestId("caepipe-external-state-binding")
        .textContent,
    ).toContain("del-17-04");
    expect(
      within(caepipeExternal).getByTestId("caepipe-external-state-binding")
        .textContent,
    ).toContain("not generated");
    expect(
      within(caepipeExternal).getByTestId("caepipe-external-parser")
        .textContent,
    ).toContain("parsed_parser_only_fixture");
    expect(
      within(caepipeExternal).getByTestId("caepipe-external-parser")
        .textContent,
    ).toContain("correlation=canonical_id_map");
    expect(
      within(caepipeExternal).getByTestId("caepipe-external-units").textContent,
    ).toContain("unit-system:dec-018-si-dual-display");
    expect(
      within(caepipeExternal).getByTestId("caepipe-external-units").textContent,
    ).toContain("conversion=false");
    expect(
      within(caepipeExternal).getByTestId("caepipe-external-unit-witnesses")
        .textContent,
    ).toContain("count=3");
    expect(
      within(caepipeExternal).getByTestId("caepipe-external-unit-witnesses")
        .textContent,
    ).toContain("conversion=false");
    expect(
      within(caepipeExternal).getByTestId("caepipe-external-boundary")
        .textContent,
    ).toContain("compatibility=false");
    expect(
      within(caepipeExternal).getByTestId("caepipe-external-boundary")
        .textContent,
    ).toContain("code_compliance=false");
    const caepipeExternalHref =
      within(caepipeExternal)
        .getByTestId("caepipe-external-export-link")
        .getAttribute("href") ?? "";
    const caepipeExternalPacket = JSON.parse(
      decodeURIComponent(caepipeExternalHref.split(",", 2)[1]),
    );
    expect(caepipeExternalPacket.document_kind).toBe(
      "openpipestress.technical_preview.caepipe_external_run_package",
    );
    expect(caepipeExternalPacket.deliverable_id).toBe("DEL-17-05");
    expect(caepipeExternalPacket.package_id).toBe("PKG-17");
    expect(caepipeExternalPacket.scope_items).toContain("SOW-030");
    expect(caepipeExternalPacket.scope_items).toContain("SOW-046");
    expect(caepipeExternalPacket.scope_items).toContain("SOW-075");
    expect(caepipeExternalPacket.package_status).toBe("parser_only_evidence");
    expect(caepipeExternalPacket.mbf_package_ref.ref).toContain("del-17-04");
    expect(caepipeExternalPacket.executable_config.configured_path_state).toBe(
      "absent",
    );
    expect(
      caepipeExternalPacket.executable_config
        .license_responsibility_acknowledged,
    ).toBe(false);
    expect(
      caepipeExternalPacket.executable_config
        .environment_responsibility_acknowledged,
    ).toBe(false);
    expect(caepipeExternalPacket.command_profile.profile_id).toBe(
      "TBD-17-05-invocation-profile",
    );
    expect(caepipeExternalPacket.execution_result.attempted).toBe(false);
    expect(caepipeExternalPacket.parser_coverage).toHaveLength(2);
    expect(
      caepipeExternalPacket.unit_system_disclosure.unit_system_ref.ref,
    ).toBe("unit-system:dec-018-si-dual-display");
    expect(
      caepipeExternalPacket.unit_system_disclosure.target_export_units
        .node_displacements,
    ).toBe("m");
    expect(
      caepipeExternalPacket.unit_system_disclosure.target_export_units
        .element_forces,
    ).toBe("N");
    expect(
      caepipeExternalPacket.unit_system_disclosure.conversion_performed,
    ).toBe(false);
    expect(caepipeExternalPacket.unit_witness_policy).toBe(
      "preserve_parser_csv_row_value_unit_and_dimension_per_row",
    );
    expect(caepipeExternalPacket.unit_preservation_witnesses).toHaveLength(3);
    const caepipeExternalForceWitness =
      caepipeExternalPacket.unit_preservation_witnesses.find(
        (item: { source_row_ref: { ref: string } }) =>
          item.source_row_ref.ref === "csv-row:4",
      );
    expect(caepipeExternalForceWitness.source_quantity.unit).toBe("N");
    expect(caepipeExternalForceWitness.source_quantity.dimension).toBe("force");
    expect(caepipeExternalForceWitness.target_quantity.values.axial).toBe(12.5);
    expect(caepipeExternalForceWitness.target_quantity.unit).toBe("N");
    expect(caepipeExternalForceWitness.conversion_performed).toBe(false);
    expect(caepipeExternalPacket.parsed_csv.row_count).toBe(3);
    expect(
      caepipeExternalPacket.parsed_csv.rows.map(
        (row: { correlation_status: string }) => row.correlation_status,
      ),
    ).toEqual(["canonical_id_map", "canonical_id_map", "canonical_id_map"]);
    expect(caepipeExternalPacket.csv_text).toContain("NODE_DISPLACEMENTS");
    expect(caepipeExternalPacket.csv_text).toContain("ELEMENT_FORCES");
    expect(caepipeExternalPacket.validation_report.validation_status).toBe(
      "boundary_checked",
    );
    expect(
      caepipeExternalPacket.validation_report.checks.map(
        (item: { check_id: string }) => item.check_id,
      ),
    ).toContain("external_execution_not_attempted");
    expect(
      caepipeExternalPacket.validation_report.checks.map(
        (item: { check_id: string }) => item.check_id,
      ),
    ).toContain("unit_preservation_witness_per_parser_row");
    expect(caepipeExternalPacket.diagnostics).toHaveLength(0);
    expect(caepipeExternalPacket.private_payload_included).toBe(false);
    expect(caepipeExternalPacket.protected_content_included).toBe(false);
    expect(caepipeExternalPacket.caepipe_compatibility_claim).toBe(false);
    expect(caepipeExternalPacket.solver_validation_claim).toBe(false);
    expect(caepipeExternalPacket.code_compliance_claim).toBe(false);
    expect(caepipeExternalPacket.professional_reliance_claim).toBe(false);
    const caepipeExternalCsvHref =
      within(caepipeExternal)
        .getByTestId("caepipe-external-csv-link")
        .getAttribute("href") ?? "";
    expect(
      decodeURIComponent(caepipeExternalCsvHref.split(",", 2)[1]),
    ).toContain("section,stable_id,target_id");
    const exportAdapterSdk = await screen.findByLabelText("Export adapter SDK");
    expect(
      within(exportAdapterSdk).getByTestId("export-adapter-sdk-summary")
        .textContent,
    ).toContain("available");
    expect(
      within(exportAdapterSdk).getByTestId("export-adapter-sdk-summary")
        .textContent,
    ).toContain("targets=5");
    expect(
      within(exportAdapterSdk).getByTestId("export-adapter-sdk-summary")
        .textContent,
    ).toContain("capabilities=4");
    expect(
      within(exportAdapterSdk).getByTestId("export-adapter-sdk-summary")
        .textContent,
    ).toContain("validation=boundary_checked");
    expect(
      within(exportAdapterSdk).getByTestId("export-adapter-sdk-profile")
        .textContent,
    ).toContain("runtime=TBD");
    expect(
      within(exportAdapterSdk).getByTestId("export-adapter-sdk-targets")
        .textContent,
    ).toContain("preview_targets=5");
    expect(
      within(exportAdapterSdk).getByTestId("export-adapter-sdk-targets")
        .textContent,
    ).toContain("admitted_support_claims=0");
    expect(
      within(exportAdapterSdk).getByTestId("export-adapter-sdk-validation")
        .textContent,
    ).toContain("loss_report=true");
    expect(
      within(exportAdapterSdk).getByTestId("export-adapter-sdk-units")
        .textContent,
    ).toContain("conversion=false");
    expect(
      within(exportAdapterSdk).getByTestId("export-adapter-sdk-units")
        .textContent,
    ).toContain("witnesses=5");
    expect(
      within(exportAdapterSdk).getByTestId("export-adapter-sdk-permissions")
        .textContent,
    ).toContain("filesystem=false");
    expect(
      within(exportAdapterSdk).getByTestId("export-adapter-sdk-permissions")
        .textContent,
    ).toContain("network=false");
    expect(
      within(exportAdapterSdk).getByTestId("export-adapter-sdk-boundary")
        .textContent,
    ).toContain("compatibility=false");
    await waitFor(() =>
      expect(
        within(exportAdapterSdk).getByTestId("export-adapter-sdk-validation")
          .textContent,
      ).toContain("package_hash=computed_local_preview_sha256"),
    );
    const exportAdapterSdkHref =
      within(exportAdapterSdk)
        .getByTestId("export-adapter-sdk-export-link")
        .getAttribute("href") ?? "";
    const exportAdapterSdkPacket = JSON.parse(
      decodeURIComponent(exportAdapterSdkHref.split(",", 2)[1]),
    );
    expect(exportAdapterSdkPacket.document_kind).toBe(
      "openpipestress.technical_preview.export_adapter_sdk_registry",
    );
    expect(exportAdapterSdkPacket.manifest.canonical_package_hash_status).toBe(
      "computed_local_preview_sha256",
    );
    expect(
      exportAdapterSdkPacket.manifest.canonical_package_hash.value,
    ).toMatch(/^sha256:[0-9a-f]{64}$/);
    expect(
      exportAdapterSdkPacket.validation_report.hash_validation_status,
    ).toBe("package_hash_computed_local_preview_not_independently_validated");
    expect(
      exportAdapterSdkPacket.target_registry.targets.find(
        (item: { target_id: string }) =>
          item.target_id === "stress_neutral_csv_json_package",
      ).unresolved_tbd_refs,
    ).toEqual([]);
    expect(exportAdapterSdkPacket.deliverable_id).toBe("DEL-17-09");
    expect(exportAdapterSdkPacket.package_id).toBe("PKG-17");
    expect(exportAdapterSdkPacket.scope_items).toContain("SOW-030");
    expect(exportAdapterSdkPacket.scope_items).toContain("SOW-074");
    expect(exportAdapterSdkPacket.scope_items).toContain("SOW-075");
    expect(exportAdapterSdkPacket.registry_id).toBe(
      "ops.export_adapter_sdk.registry_preview",
    );
    expect(exportAdapterSdkPacket.sdk_contract.sdk_contract_status).toBe(
      "contract_level_preview",
    );
    expect(
      exportAdapterSdkPacket.sdk_contract.source_basis_refs.map(
        (item: { ref: string }) => item.ref,
      ),
    ).toContain("DEL-17-02");
    expect(
      exportAdapterSdkPacket.sdk_contract.source_basis_refs.map(
        (item: { ref: string }) => item.ref,
      ),
    ).toContain("DEL-10-01");
    expect(exportAdapterSdkPacket.sdk_contract.capabilities).toContain(
      "export_model",
    );
    expect(exportAdapterSdkPacket.sdk_contract.capabilities).toContain(
      "validate_payload",
    );
    expect(
      exportAdapterSdkPacket.sdk_contract.runtime_model.plugin_runtime,
    ).toBe("TBD");
    expect(
      exportAdapterSdkPacket.sdk_contract.runtime_model.permission_taxonomy,
    ).toBe("TBD");
    expect(
      exportAdapterSdkPacket.sdk_contract.deny_by_default_controls
        .filesystem_access_granted,
    ).toBe(false);
    expect(
      exportAdapterSdkPacket.sdk_contract.deny_by_default_controls
        .network_access_granted,
    ).toBe(false);
    expect(
      exportAdapterSdkPacket.sdk_contract.deny_by_default_controls
        .process_access_granted,
    ).toBe(false);
    expect(
      exportAdapterSdkPacket.sdk_contract.deny_by_default_controls
        .private_data_access_granted,
    ).toBe(false);
    expect(
      exportAdapterSdkPacket.adapter_template
        .sample_adapter_implementation_included,
    ).toBe(false);
    expect(
      exportAdapterSdkPacket.adapter_template
        .source_basis_required_before_support_claim,
    ).toBe(true);
    expect(exportAdapterSdkPacket.target_registry.summary.target_count).toBe(5);
    expect(
      exportAdapterSdkPacket.target_registry.summary
        .preview_panel_available_count,
    ).toBe(5);
    expect(
      exportAdapterSdkPacket.target_registry.summary
        .admitted_support_claim_count,
    ).toBe(0);
    expect(
      exportAdapterSdkPacket.target_registry.targets.map(
        (item: { target_id: string }) => item.target_id,
      ),
    ).toContain("caepipe_mbf_export");
    expect(
      exportAdapterSdkPacket.target_registry.targets.map(
        (item: { target_id: string }) => item.target_id,
      ),
    ).toContain("conservative_pcf_export");
    expect(exportAdapterSdkPacket.validation_report.validation_status).toBe(
      "boundary_checked",
    );
    expect(
      exportAdapterSdkPacket.validation_report.checks.loss_report_required,
    ).toBe(true);
    expect(
      exportAdapterSdkPacket.validation_report.checks
        .no_support_or_compatibility_claims,
    ).toBe(true);
    expect(
      exportAdapterSdkPacket.unit_policy_evidence.unit_system_ref.ref,
    ).toBe("unit-system:dec-018-si-dual-display");
    expect(exportAdapterSdkPacket.unit_policy_evidence.storage_convention).toBe(
      "entered_units_preserved",
    );
    expect(exportAdapterSdkPacket.unit_policy_evidence.conversion_policy).toBe(
      "no_adapter_sdk_conversion_performed",
    );
    expect(
      exportAdapterSdkPacket.unit_policy_evidence.conversion_performed,
    ).toBe(false);
    expect(exportAdapterSdkPacket.unit_policy_evidence.witness_count).toBe(5);
    expect(
      exportAdapterSdkPacket.unit_policy_evidence.target_refs,
    ).toHaveLength(5);
    expect(
      exportAdapterSdkPacket.unit_policy_evidence.decision_basis_refs.map(
        (item: { ref: string }) => item.ref,
      ),
    ).toEqual(["DEC-018", "DEL-02-02"]);
    expect(
      exportAdapterSdkPacket.manifest.package_members.map(
        (item: { role: string }) => item.role,
      ),
    ).toContain("unit_policy_evidence");
    expect(exportAdapterSdkPacket.diagnostics).toHaveLength(0);
    expect(exportAdapterSdkPacket.private_payload_included).toBe(false);
    expect(exportAdapterSdkPacket.protected_content_included).toBe(false);
    expect(exportAdapterSdkPacket.target_compatibility_claim).toBe(false);
    expect(exportAdapterSdkPacket.solver_validation_claim).toBe(false);
    expect(exportAdapterSdkPacket.code_compliance_claim).toBe(false);
    expect(exportAdapterSdkPacket.professional_reliance_claim).toBe(false);
    const stressNeutral = await screen.findByLabelText(
      "Stress-neutral CSV JSON export",
    );
    expect(
      within(stressNeutral).getByTestId("stress-neutral-empty").textContent,
    ).toContain("stress-neutral CSV/JSON package");
    const buildReadiness = await screen.findByLabelText(
      "Build package readiness",
    );
    expect(
      within(buildReadiness).getByTestId("build-readiness-summary").textContent,
    ).toContain("available");
    expect(
      within(buildReadiness).getByTestId("build-readiness-summary").textContent,
    ).toContain("scripts=8");
    expect(
      within(buildReadiness).getByTestId("build-readiness-summary").textContent,
    ).toContain("bundle_active=false");
    expect(
      within(buildReadiness).getByTestId("build-readiness-shell").textContent,
    ).toContain("OpenPipeStress Technical Preview");
    expect(
      within(buildReadiness).getByTestId("build-readiness-commands")
        .textContent,
    ).toContain("build:desktop");
    expect(
      within(buildReadiness).getByTestId("build-readiness-commands")
        .textContent,
    ).toContain("tauri");
    expect(
      within(buildReadiness).getByTestId("build-readiness-profiles")
        .textContent,
    ).toContain("skeleton");
    expect(
      within(buildReadiness).getByTestId("build-readiness-decisions")
        .textContent,
    ).toContain("ci=TBD");
    expect(
      within(buildReadiness).getByTestId("build-readiness-boundary")
        .textContent,
    ).toContain("release_authorized=false");
    const buildReadinessHref =
      within(buildReadiness)
        .getByTestId("build-readiness-export-link")
        .getAttribute("href") ?? "";
    const buildReadinessPacket = JSON.parse(
      decodeURIComponent(buildReadinessHref.split(",", 2)[1]),
    );
    expect(buildReadinessPacket.document_kind).toBe(
      "openpipestress.technical_preview.build_package_readiness",
    );
    expect(buildReadinessPacket.deliverable_id).toBe("DEL-10-04");
    expect(buildReadinessPacket.package_id).toBe("PKG-10");
    expect(buildReadinessPacket.scope_item).toBe("SOW-032");
    expect(buildReadinessPacket.objectives).toContain("OBJ-008");
    expect(buildReadinessPacket.objectives).toContain("OBJ-009");
    expect(buildReadinessPacket.summary.tauri_shell_status).toBe("present");
    expect(buildReadinessPacket.summary.bundle_active).toBe(false);
    expect(buildReadinessPacket.workspace.root_scripts).toContain(
      "build:desktop",
    );
    expect(buildReadinessPacket.workspace.desktop_scripts).toContain("tauri");
    expect(buildReadinessPacket.tauri_shell.before_build_command).toBe(
      "npm run build",
    );
    expect(buildReadinessPacket.release_readiness_tool.default_profile).toBe(
      "skeleton",
    );
    expect(
      buildReadinessPacket.release_readiness_tool.browser_panel_runs_tool,
    ).toBe(false);
    expect(buildReadinessPacket.unresolved_decisions.ci_provider).toBe("TBD");
    expect(buildReadinessPacket.unresolved_decisions.release_matrix).toBe(
      "TBD",
    );
    expect(buildReadinessPacket.unresolved_decisions.signing_notarization).toBe(
      "TBD",
    );
    expect(buildReadinessPacket.boundary.network_required).toBe(false);
    expect(buildReadinessPacket.boundary.telemetry_enabled).toBe(false);
    expect(buildReadinessPacket.boundary.release_publication_authorized).toBe(
      false,
    );
    expect(buildReadinessPacket.boundary.installer_or_binary_generated).toBe(
      false,
    );
    expect(buildReadinessPacket.boundary.code_compliance_claim).toBe(false);
    expect(buildReadinessPacket.boundary.professional_reliance_claim).toBe(
      false,
    );
    const validationEvidence = await screen.findByLabelText(
      "Validation evidence review",
    );
    expect(
      within(validationEvidence).getByTestId("validation-evidence-summary")
        .textContent,
    ).toContain("manual=10");
    expect(
      within(validationEvidence).getByTestId("validation-evidence-summary")
        .textContent,
    ).toContain("evidence=8");
    expect(
      within(validationEvidence).getByTestId("validation-evidence-summary")
        .textContent,
    ).toContain("profiles=5");
    expect(
      within(validationEvidence).getByTestId("validation-evidence-manual")
        .textContent,
    ).toContain("DRAFT_EVIDENCE");
    expect(
      within(validationEvidence).getByTestId("validation-evidence-inventory")
        .textContent,
    ).toContain("gui=PLANNED");
    expect(
      within(validationEvidence).getByTestId("validation-evidence-unit-policy")
        .textContent,
    ).toContain(
      "model=angle=rad,force=N,length=m,pressure=Pa,stress=MPa,temperature=degC",
    );
    expect(
      within(validationEvidence).getByTestId("validation-evidence-unit-policy")
        .textContent,
    ).toContain("records=44");
    expect(
      within(validationEvidence).getByTestId("validation-evidence-unit-policy")
        .textContent,
    ).toContain("conversion=false");
    expect(
      within(validationEvidence).getByTestId(
        "validation-evidence-release-checks",
      ).textContent,
    ).toContain("skeleton_checks=2");
    expect(
      within(validationEvidence).getByTestId("validation-evidence-gates")
        .textContent,
    ).toContain("thresholds=TBD");
    expect(
      within(validationEvidence).getByTestId("validation-evidence-gates")
        .textContent,
    ).toContain("release_authorized=false");
    expect(
      within(validationEvidence).getByTestId("validation-evidence-boundary")
        .textContent,
    ).toContain("release_claim=false");
    const validationEvidenceHref =
      within(validationEvidence)
        .getByTestId("validation-evidence-export-link")
        .getAttribute("href") ?? "";
    const validationEvidencePacket = JSON.parse(
      decodeURIComponent(validationEvidenceHref.split(",", 2)[1]),
    );
    expect(validationEvidencePacket.document_kind).toBe(
      "openpipestress.technical_preview.validation_release_evidence_review",
    );
    expect(validationEvidencePacket.deliverable_refs).toContain("DEL-09-04");
    expect(validationEvidencePacket.deliverable_refs).toContain("DEL-09-05");
    expect(validationEvidencePacket.scope_items).toContain("SOW-026");
    expect(validationEvidencePacket.scope_items).toContain("SOW-027");
    expect(validationEvidencePacket.objectives).toContain("OBJ-008");
    expect(validationEvidencePacket.objectives).toContain("OBJ-011");
    expect(validationEvidencePacket.summary.manual_section_count).toBe(10);
    expect(validationEvidencePacket.summary.evidence_area_count).toBe(8);
    expect(validationEvidencePacket.summary.release_profile_count).toBe(5);
    expect(validationEvidencePacket.summary.required_release_path_count).toBe(
      7,
    );
    expect(validationEvidencePacket.summary.skeleton_check_count).toBe(2);
    expect(validationEvidencePacket.summary.unit_bearing_record_count).toBe(44);
    expect(
      validationEvidencePacket.validation_manual
        .professional_reliance_outside_software_authority,
    ).toBe(true);
    expect(
      validationEvidencePacket.unit_policy_evidence.unit_system_ref.ref,
    ).toBe("unit-system:dec-018-si-dual-display");
    expect(
      validationEvidencePacket.unit_policy_evidence.storage_convention,
    ).toBe("entered_units_preserved");
    expect(
      validationEvidencePacket.unit_policy_evidence
        .validation_evidence_unit_policy,
    ).toBe(
      "validation_evidence_records_project_unit_context_without_conversion_or_release_threshold_claim",
    );
    expect(validationEvidencePacket.unit_policy_evidence.model_units).toEqual({
      angle: "rad",
      force: "N",
      length: "m",
      pressure: "Pa",
      stress: "MPa",
      temperature: "degC",
    });
    expect(
      validationEvidencePacket.unit_policy_evidence.unit_bearing_record_count,
    ).toBe(44);
    expect(
      validationEvidencePacket.unit_policy_evidence
        .unit_and_schema_manual_section,
    ).toBe("unit_and_schema_verification");
    expect(
      validationEvidencePacket.unit_policy_evidence.conversion_policy,
    ).toBe(
      "validation_evidence_inventory_records_unit_context_without_conversion",
    );
    expect(
      validationEvidencePacket.unit_policy_evidence.conversion_performed,
    ).toBe(false);
    expect(
      validationEvidencePacket.unit_policy_evidence
        .release_gate_threshold_policy,
    ).toBe("TBD");
    expect(
      validationEvidencePacket.unit_policy_evidence.decision_basis_refs.map(
        (item: { ref: string }) => item.ref,
      ),
    ).toEqual(["DEC-018", "DEL-02-02", "DEL-09-05"]);
    expect(
      validationEvidencePacket.release_readiness_tool.browser_panel_runs_tool,
    ).toBe(false);
    expect(
      validationEvidencePacket.release_readiness_tool
        .latest_dag_dependency_edges,
    ).toBe("execution/_DAG/DAG-006/DependencyEdges.csv");
    expect(
      validationEvidencePacket.release_quality_gates
        .release_publication_authorized,
    ).toBe(false);
    expect(
      validationEvidencePacket.release_quality_gates.final_threshold_policy,
    ).toBe("TBD");
    expect(
      validationEvidencePacket.gui_validation_context
        .current_tranche_smoke_record,
    ).toBe("TP-MAC-83");
    expect(validationEvidencePacket.private_payload_included).toBe(false);
    expect(validationEvidencePacket.protected_content_included).toBe(false);
    expect(validationEvidencePacket.release_or_professional_claim).toBe(false);
    const editorContract = await screen.findByLabelText(
      "Editor contract review",
    );
    expect(
      within(editorContract).getByTestId("editor-contract-summary").textContent,
    ).toContain("editors=7");
    expect(
      within(editorContract).getByTestId("editor-contract-summary").textContent,
    ).toContain("surfaces=7");
    expect(
      within(editorContract).getByTestId("editor-contract-summary").textContent,
    ).toContain("diagnostics=3");
    expect(
      within(editorContract).getByTestId("editor-contract-summary").textContent,
    ).toContain("private_payload=false");
    expect(
      within(editorContract).getByTestId("editor-contract-coverage")
        .textContent,
    ).toContain("materials:review_editor_available");
    expect(
      within(editorContract).getByTestId("editor-contract-coverage")
        .textContent,
    ).toContain("viewport_gestures:operation_intent_editor_available");
    expect(
      within(editorContract).getByTestId("editor-contract-coverage")
        .textContent,
    ).toContain("private_libraries:reference_slots_only_no_private_payload");
    expect(
      within(editorContract).getByTestId("editor-contract-validation")
        .textContent,
    ).toContain("ready=6");
    expect(
      within(editorContract).getByTestId("editor-contract-validation")
        .textContent,
    ).toContain("blocked=1");
    expect(
      within(editorContract).getByTestId("editor-contract-unit-contract")
        .textContent,
    ).toContain("contract=DEL-02-02");
    expect(
      within(editorContract).getByTestId("editor-contract-unit-contract")
        .textContent,
    ).toContain("schema=schemas/units.schema.yaml#/$defs/DimensionId");
    expect(
      within(editorContract).getByTestId("editor-contract-unit-contract")
        .textContent,
    ).toContain("policy=unit_bearing_values_require_explicit_unit_metadata");
    expect(
      within(editorContract).getByTestId("editor-contract-unit-contract")
        .textContent,
    ).toContain("missing=diagnostic_blocking");
    expect(
      within(editorContract).getByTestId("editor-contract-rule-pack")
        .textContent,
    ).toContain("rule-pack:user-supplied:not-loaded");
    expect(
      within(editorContract).getByTestId("editor-contract-rule-pack")
        .textContent,
    ).toContain("checksum=TBD_private_rule_pack_not_loaded");
    expect(
      within(editorContract).getByTestId("editor-contract-mutation-boundary")
        .textContent,
    ).toContain("direct_model_mutation_allowed=false");
    expect(
      within(editorContract).getByTestId(
        "editor-contract-professional-boundary",
      ).textContent,
    ).toContain("compliance=false");
    const editorContractHref =
      within(editorContract)
        .getByTestId("editor-contract-export-link")
        .getAttribute("href") ?? "";
    const editorContractPacket = JSON.parse(
      decodeURIComponent(editorContractHref.split(",", 2)[1]),
    );
    expect(editorContractPacket.document_kind).toBe(
      "openpipestress.technical_preview.editor_contract_review",
    );
    expect(editorContractPacket.deliverable_id).toBe("DEL-07-03");
    expect(editorContractPacket.package_id).toBe("PKG-07");
    expect(editorContractPacket.scope_item).toBe("SOW-021");
    expect(editorContractPacket.objectives).toContain("OBJ-006");
    expect(editorContractPacket.summary.editor_count).toBe(7);
    expect(editorContractPacket.summary.surface_count).toBe(7);
    expect(editorContractPacket.summary.ready_editor_count).toBe(6);
    expect(editorContractPacket.summary.blocked_editor_count).toBe(1);
    expect(editorContractPacket.summary.diagnostic_count).toBe(3);
    expect(editorContractPacket.summary.queued_intent_count).toBe(0);
    expect(editorContractPacket.unit_contract).toEqual({
      contract_ref: "DEL-02-02",
      schema_ref: "schemas/units.schema.yaml#/$defs/DimensionId",
      unit_policy: "unit_bearing_values_require_explicit_unit_metadata",
      missing_unit_behavior: "diagnostic_blocking",
    });
    expect(
      editorContractPacket.editors.map(
        (item: { editor_kind: string }) => item.editor_kind,
      ),
    ).toContain("material");
    expect(
      editorContractPacket.editors.map(
        (item: { editor_kind: string }) => item.editor_kind,
      ),
    ).toContain("component");
    expect(
      editorContractPacket.editors.map(
        (item: { editor_kind: string }) => item.editor_kind,
      ),
    ).toContain("rule_pack_reference");
    expect(
      editorContractPacket.surface_inventory.map(
        (item: { surface: string }) => item.surface,
      ),
    ).toContain("private_libraries");
    expect(
      editorContractPacket.diagnostics.map(
        (item: { code: string }) => item.code,
      ),
    ).toContain("RULE_PACK_REQUIRED_INPUTS_MISSING");
    expect(
      editorContractPacket.diagnostics.map(
        (item: { code: string }) => item.code,
      ),
    ).toContain("PRIVATE_LIBRARY_PAYLOAD_NOT_LOADED");
    expect(
      editorContractPacket.rule_pack_reference.private_payload_redacted,
    ).toBe(true);
    expect(
      editorContractPacket.source_of_truth_boundary
        .direct_model_mutation_allowed,
    ).toBe(false);
    expect(
      editorContractPacket.source_of_truth_boundary
        .accepted_model_state_mutated,
    ).toBe(false);
    expect(editorContractPacket.private_payload_included).toBe(false);
    expect(editorContractPacket.protected_content_included).toBe(false);
    expect(editorContractPacket.release_or_professional_claim).toBe(false);
    expect(
      editorContractPacket.professional_boundary
        .software_makes_compliance_claim,
    ).toBe(false);
    fireEvent.click(screen.getByTestId("issues-drawer-toggle"));
    const missingData = await screen.findByLabelText(
      "Missing-data blocking review",
    );
    expect(
      within(missingData).getByTestId("missing-data-summary").textContent,
    ).toContain("classes=6");
    expect(
      within(missingData).getByTestId("missing-data-summary").textContent,
    ).toContain("active=4");
    expect(
      within(missingData).getByTestId("missing-data-summary").textContent,
    ).toContain("solve_blocked=false");
    expect(
      within(missingData).getByTestId("missing-data-summary").textContent,
    ).toContain("rule_blocked=true");
    expect(
      within(missingData).getByTestId("missing-data-class-coverage")
        .textContent,
    ).toContain("SOLVE_BLOCKING:available_no_active_preview_item");
    expect(
      within(missingData).getByTestId("missing-data-class-coverage")
        .textContent,
    ).toContain("RULE_CHECK_BLOCKING:active");
    expect(
      within(missingData).getByTestId("missing-data-class-coverage")
        .textContent,
    ).toContain("IP_BOUNDARY_WARNING:available_no_active_preview_item");
    expect(
      within(missingData).getByTestId("missing-data-status-separation")
        .textContent,
    ).toContain("mechanics=ready_for_preview_diagnostics");
    expect(
      within(missingData).getByTestId("missing-data-status-separation")
        .textContent,
    ).toContain("mechanics_reviewable=false");
    expect(
      within(missingData).getByTestId("missing-data-unit-policy").textContent,
    ).toContain("required=true");
    expect(
      within(missingData).getByTestId("missing-data-unit-policy").textContent,
    ).toContain("default_units=false");
    expect(
      within(missingData).getByTestId("missing-data-unit-policy").textContent,
    ).toContain("conversion=false");
    expect(
      within(missingData).getByTestId("missing-data-blocking").textContent,
    ).toContain("solve=0");
    expect(
      within(missingData).getByTestId("missing-data-blocking").textContent,
    ).toContain("rule=2");
    expect(
      within(missingData).getByTestId("missing-data-assistive").textContent,
    ).toContain("class,severity,affected_refs,message,remediation");
    expect(
      within(missingData).getByTestId("missing-data-assistive").textContent,
    ).toContain("color_only=false");
    expect(
      within(missingData).getByTestId("missing-data-boundary").textContent,
    ).toContain("silent_defaults=false");
    expect(
      within(missingData).getByTestId("missing-data-boundary").textContent,
    ).toContain("auto_fill=false");
    expect(
      within(missingData).getByTestId(
        "missing-data-warning-rule-check-required-inputs",
      ).textContent,
    ).toContain("RULE_CHECK_BLOCKING");
    const missingDataHref =
      within(missingData)
        .getByTestId("missing-data-export-link")
        .getAttribute("href") ?? "";
    const missingDataPacket = JSON.parse(
      decodeURIComponent(missingDataHref.split(",", 2)[1]),
    );
    expect(missingDataPacket.document_kind).toBe(
      "openpipestress.technical_preview.missing_data_warning_blocking_review",
    );
    expect(missingDataPacket.deliverable_id).toBe("DEL-07-04");
    expect(missingDataPacket.package_id).toBe("PKG-07");
    expect(missingDataPacket.scope_item).toBe("SOW-022");
    expect(missingDataPacket.objectives).toContain("OBJ-011");
    expect(missingDataPacket.summary.warning_class_count).toBe(6);
    expect(missingDataPacket.summary.active_warning_count).toBe(4);
    expect(missingDataPacket.summary.solve_blocking_count).toBe(0);
    expect(missingDataPacket.summary.rule_check_blocking_count).toBe(2);
    expect(missingDataPacket.summary.mechanics_solve_blocked).toBe(false);
    expect(missingDataPacket.summary.rule_check_blocked).toBe(true);
    expect(missingDataPacket.summary.mechanics_results_reviewable).toBe(false);
    expect(missingDataPacket.summary.silent_defaults_used).toBe(false);
    expect(missingDataPacket.unit_input_policy_evidence.evidence_id).toBe(
      "unit-input-policy-evidence:missing-data-warning-blocking-review",
    );
    expect(missingDataPacket.unit_input_policy_evidence.policy_refs).toContain(
      "DEC-018",
    );
    expect(missingDataPacket.unit_input_policy_evidence.policy_refs).toContain(
      "DEL-02-02",
    );
    expect(
      missingDataPacket.unit_input_policy_evidence
        .unit_bearing_inputs_require_explicit_units,
    ).toBe(true);
    expect(
      missingDataPacket.unit_input_policy_evidence
        .unit_warning_remediation_requires_explicit_units,
    ).toBe(true);
    expect(
      missingDataPacket.unit_input_policy_evidence.default_units_inferred,
    ).toBe(false);
    expect(
      missingDataPacket.unit_input_policy_evidence.silent_unit_defaults_used,
    ).toBe(false);
    expect(
      missingDataPacket.unit_input_policy_evidence.conversion_performed,
    ).toBe(false);
    expect(
      missingDataPacket.unit_input_policy_evidence.model_units.length,
    ).toBe("m");
    expect(missingDataPacket.auto_fill_missing_data).toBe(false);
    expect(
      missingDataPacket.class_inventory.map(
        (item: { warning_class: string }) => item.warning_class,
      ),
    ).toContain("NONLINEAR_WARNING");
    expect(
      missingDataPacket.analysis_boundary_contract.gui_warning_class_status_map
        .SOLVE_BLOCKING,
    ).toContain("MODEL_INCOMPLETE");
    expect(
      missingDataPacket.assistive_contract.color_only_signaling_allowed,
    ).toBe(false);
    expect(
      missingDataPacket.source_of_truth_boundary.direct_model_mutation_allowed,
    ).toBe(false);
    expect(missingDataPacket.private_payload_included).toBe(false);
    expect(missingDataPacket.protected_content_included).toBe(false);
    expect(missingDataPacket.release_or_professional_claim).toBe(false);
    expect(
      missingDataPacket.professional_boundary.software_makes_compliance_claim,
    ).toBe(false);
    const accessibilityBaseline = await screen.findByLabelText(
      "Accessibility and usability baseline",
    );
    expect(
      within(accessibilityBaseline).getByTestId(
        "accessibility-baseline-summary",
      ).textContent,
    ).toContain("surfaces=6");
    expect(
      within(accessibilityBaseline).getByTestId(
        "accessibility-baseline-summary",
      ).textContent,
    ).toContain("findings=69");
    expect(
      within(accessibilityBaseline).getByTestId(
        "accessibility-baseline-workflow",
      ).textContent,
    ).toContain("keyboard=4");
    expect(
      within(accessibilityBaseline).getByTestId(
        "accessibility-baseline-unit-visibility",
      ).textContent,
    ).toContain(
      "policy=unit_bearing_values_keep_visible_unit_labels_in_review_surfaces",
    );
    expect(
      within(accessibilityBaseline).getByTestId(
        "accessibility-baseline-unit-visibility",
      ).textContent,
    ).toContain(
      "model=angle=rad,force=N,length=m,pressure=Pa,stress=MPa,temperature=degC",
    );
    expect(
      within(accessibilityBaseline).getByTestId(
        "accessibility-baseline-unit-visibility",
      ).textContent,
    ).toContain("conversion=false");
    expect(
      within(accessibilityBaseline).getByTestId(
        "accessibility-baseline-open-target",
      ).textContent,
    ).toContain("gui=TBD");
    expect(
      within(accessibilityBaseline).getByTestId(
        "accessibility-baseline-boundary",
      ).textContent,
    ).toContain("conformance_claim=false");
    const accessibilityHref =
      within(accessibilityBaseline)
        .getByTestId("accessibility-baseline-export-link")
        .getAttribute("href") ?? "";
    const accessibilityPacket = JSON.parse(
      decodeURIComponent(accessibilityHref.split(",", 2)[1]),
    );
    expect(accessibilityPacket.document_kind).toBe(
      "openpipestress.technical_preview.accessibility_usability_baseline_review",
    );
    expect(accessibilityPacket.deliverable_id).toBe("DEL-07-06");
    expect(accessibilityPacket.package_id).toBe("PKG-07");
    expect(accessibilityPacket.scope_item).toBe("SOW-036");
    expect(accessibilityPacket.objectives).toContain("OBJ-006");
    expect(accessibilityPacket.summary.source_surface_count).toBe(6);
    expect(accessibilityPacket.summary.total_findings).toBe(69);
    expect(accessibilityPacket.summary.pass_count).toBe(57);
    expect(accessibilityPacket.summary.warning_count).toBe(10);
    expect(accessibilityPacket.summary.fail_count).toBe(1);
    expect(accessibilityPacket.summary.blocking_count).toBe(1);
    expect(accessibilityPacket.summary.accessibility_target_status).toBe(
      "TBD_by_human_project_authority",
    );
    expect(accessibilityPacket.summary.desktop_runtime_evaluation).toBe(
      "not_performed",
    );
    expect(
      accessibilityPacket.source_surfaces.map(
        (item: { deliverable_id: string }) => item.deliverable_id,
      ),
    ).toContain("DEL-07-05");
    expect(accessibilityPacket.reviewed_categories).toContain("keyboard_path");
    expect(accessibilityPacket.reviewed_categories).toContain(
      "warning_visibility",
    );
    expect(accessibilityPacket.reviewed_categories).toContain(
      "unit_visibility",
    );
    expect(accessibilityPacket.unit_visibility_evidence.evidence_id).toBe(
      "unit-visibility-evidence:accessibility-baseline-preview",
    );
    expect(accessibilityPacket.unit_visibility_evidence.policy_refs).toContain(
      "DEC-018",
    );
    expect(accessibilityPacket.unit_visibility_evidence.policy_refs).toContain(
      "DEL-02-02",
    );
    expect(
      accessibilityPacket.unit_visibility_evidence.model_units.length,
    ).toBe("m");
    expect(
      accessibilityPacket.unit_visibility_evidence.default_units_inferred,
    ).toBe(false);
    expect(
      accessibilityPacket.unit_visibility_evidence.color_only_unit_signaling,
    ).toBe(false);
    expect(
      accessibilityPacket.unit_visibility_evidence.conversion_performed,
    ).toBe(false);
    expect(accessibilityPacket.review_policy.preserves_unit_labels).toBe(true);
    expect(
      accessibilityPacket.review_policy.color_only_status_signaling_allowed,
    ).toBe(false);
    expect(
      accessibilityPacket.review_policy
        .software_makes_accessibility_conformance_claim,
    ).toBe(false);
    expect(
      accessibilityPacket.open_decisions.accessibility_conformance_target,
    ).toBe("TBD");
    expect(
      accessibilityPacket.open_decisions
        .desktop_runtime_accessibility_tree_audit,
    ).toBe("TBD");
    expect(accessibilityPacket.private_payload_included).toBe(false);
    expect(accessibilityPacket.protected_content_included).toBe(false);
    expect(accessibilityPacket.release_or_professional_claim).toBe(false);
    const designWorkspace = await screen.findByLabelText(
      "Design-authoring workspace",
    );
    expect(
      within(designWorkspace).getByTestId("design-workspace-summary")
        .textContent,
    ).toContain("knowledge=3");
    expect(
      within(designWorkspace).getByTestId("design-workspace-summary")
        .textContent,
    ).toContain("runs=0");
    expect(
      within(designWorkspace).getByTestId("design-workspace-summary")
        .textContent,
    ).toContain("comparisons=0");
    expect(
      within(designWorkspace).getByTestId("design-workspace-core").textContent,
    ).toContain("records=2");
    expect(
      within(designWorkspace).getByTestId("design-workspace-core").textContent,
    ).toContain("states=2");
    expect(
      within(designWorkspace).getByTestId("design-workspace-current")
        .textContent,
    ).toContain("analysis_run=not generated");
    expect(
      within(designWorkspace).getByTestId("design-workspace-operation")
        .textContent,
    ).toContain("accepted_mutation=false");
    expect(
      within(designWorkspace).getByTestId("design-workspace-units").textContent,
    ).toContain(
      "model=angle=rad,force=N,length=m,pressure=Pa,stress=MPa,temperature=degC",
    );
    expect(
      within(designWorkspace).getByTestId("design-workspace-units").textContent,
    ).toContain("results=none");
    expect(
      within(designWorkspace).getByTestId("design-workspace-units").textContent,
    ).toContain("comparison=none");
    expect(
      within(designWorkspace).getByTestId("design-workspace-units").textContent,
    ).toContain("conversion=false");
    expect(
      within(designWorkspace).getByTestId("design-workspace-boundary")
        .textContent,
    ).toContain("professional_claim=false");
    const designWorkspaceHref =
      within(designWorkspace)
        .getByTestId("design-workspace-export-link")
        .getAttribute("href") ?? "";
    const designWorkspacePacket = JSON.parse(
      decodeURIComponent(designWorkspaceHref.split(",", 2)[1]),
    );
    expect(designWorkspacePacket.document_kind).toBe(
      "openpipestress.technical_preview.design_authoring_comparison_workspace",
    );
    expect(designWorkspacePacket.deliverable_id).toBe("DEL-07-08");
    expect(designWorkspacePacket.package_id).toBe("PKG-07");
    expect(designWorkspacePacket.scope_item).toBe("SOW-076");
    expect(designWorkspacePacket.objectives).toContain("OBJ-015");
    expect(designWorkspacePacket.objectives).toContain("OBJ-016");
    expect(
      designWorkspacePacket.summary.current_design_knowledge_record_count,
    ).toBe(3);
    expect(designWorkspacePacket.summary.current_model_state_count).toBe(0);
    expect(designWorkspacePacket.summary.current_analysis_run_count).toBe(0);
    expect(designWorkspacePacket.summary.current_comparison_pair_count).toBe(0);
    expect(
      designWorkspacePacket.core_contract_evidence
        .design_knowledge_record_count,
    ).toBe(2);
    expect(
      designWorkspacePacket.core_contract_evidence.constraint_warning_count,
    ).toBe(1);
    expect(designWorkspacePacket.core_contract_evidence.model_state_count).toBe(
      2,
    );
    expect(
      designWorkspacePacket.core_contract_evidence.analysis_run_count,
    ).toBe(2);
    expect(
      designWorkspacePacket.core_contract_evidence.graphical_overlay_count,
    ).toBe(5);
    expect(
      designWorkspacePacket.current_workspace_state.state_run_browser,
    ).toBe("pending_mechanics_run");
    expect(designWorkspacePacket.unit_policy_evidence.unit_system_ref.ref).toBe(
      "unit-system:dec-018-si-dual-display",
    );
    expect(designWorkspacePacket.unit_policy_evidence.storage_convention).toBe(
      "entered_units_preserved",
    );
    expect(
      designWorkspacePacket.unit_policy_evidence.workspace_unit_policy,
    ).toBe("compose_model_result_and_comparison_units_without_conversion");
    expect(designWorkspacePacket.unit_policy_evidence.model_units).toEqual({
      angle: "rad",
      force: "N",
      length: "m",
      pressure: "Pa",
      stress: "MPa",
      temperature: "degC",
    });
    expect(designWorkspacePacket.unit_policy_evidence.result_units).toEqual([]);
    expect(designWorkspacePacket.unit_policy_evidence.comparison_units).toEqual(
      [],
    );
    expect(
      designWorkspacePacket.unit_policy_evidence.conversion_performed,
    ).toBe(false);
    expect(
      designWorkspacePacket.unit_policy_evidence.comparison_unit_policy_ref,
    ).toBe("not generated");
    expect(
      designWorkspacePacket.unit_policy_evidence.decision_basis_refs.map(
        (item: { ref: string }) => item.ref,
      ),
    ).toEqual(["DEC-018", "DEC-026", "DEL-02-02", "DEL-14-04"]);
    expect(
      designWorkspacePacket.mutation_boundary
        .workspace_mutates_accepted_model_state,
    ).toBe(false);
    expect(
      designWorkspacePacket.mutation_boundary.accepted_model_state_mutated,
    ).toBe(false);
    expect(designWorkspacePacket.private_payload_included).toBe(false);
    expect(designWorkspacePacket.protected_content_included).toBe(false);
    expect(designWorkspacePacket.release_or_professional_claim).toBe(false);
    const reportLint = await screen.findByLabelText("Report content lint");
    expect(
      within(reportLint).getByTestId("report-lint-summary").textContent,
    ).toContain("targets=46");
    expect(
      within(reportLint).getByTestId("report-lint-summary").textContent,
    ).toContain("findings=0");
    expect(
      within(reportLint).getByTestId("report-lint-summary").textContent,
    ).toContain("blocking=0");
    expect(
      within(reportLint).getByTestId("report-lint-scope").textContent,
    ).toContain("explicit_public_surfaces");
    expect(
      within(reportLint).getByTestId("report-lint-scope").textContent,
    ).toContain("skip_unless_explicitly_authorized");
    expect(
      within(reportLint).getByTestId("report-lint-clean-scan").textContent,
    ).toContain("clearance=false");
    expect(
      within(reportLint).getByTestId("report-lint-unit-policy").textContent,
    ).toContain("unit_targets=44");
    expect(
      within(reportLint).getByTestId("report-lint-unit-policy").textContent,
    ).toContain("conversion_witness_targets=2");
    expect(
      within(reportLint).getByTestId("report-lint-unit-policy").textContent,
    ).toContain("lint_conversion=false");
    expect(
      within(reportLint).getByTestId("report-lint-boundary").textContent,
    ).toContain("legal=false");
    expect(
      within(reportLint).getByTestId("report-lint-boundary").textContent,
    ).toContain("professional=false");
    const lintHref =
      within(reportLint)
        .getByTestId("report-lint-export-link")
        .getAttribute("href") ?? "";
    const lintPacket = JSON.parse(
      decodeURIComponent(lintHref.split(",", 2)[1]),
    );
    expect(lintPacket.deliverable_id).toBe("DEL-08-05");
    expect(lintPacket.scope_item).toBe("SOW-043");
    expect(lintPacket.objectives).toContain("OBJ-002");
    expect(lintPacket.objectives).toContain("OBJ-007");
    expect(lintPacket.linter_status.heuristic_only).toBe(true);
    expect(lintPacket.linter_status.legal_clearance).toBe(false);
    expect(lintPacket.linter_status.professional_approval).toBe(false);
    expect(lintPacket.linter_status.ci_release_policy).toBe("TBD");
    expect(lintPacket.unit_policy_evidence.evidence_kind).toBe(
      "public_surface_unit_policy_inventory",
    );
    expect(lintPacket.unit_policy_evidence.unit_policy_target_count).toBe(44);
    expect(
      lintPacket.unit_policy_evidence.conversion_witness_target_count,
    ).toBe(2);
    expect(lintPacket.unit_policy_evidence.lint_performs_conversion).toBe(
      false,
    );
    expect(
      lintPacket.unit_policy_evidence.lint_asserts_target_format_compatibility,
    ).toBe(false);
    expect(lintPacket.unit_policy_evidence.target_refs).toEqual(
      expect.arrayContaining([
        expect.objectContaining({
          source_path:
            "apps/desktop/src/features/project-storage/ProjectStorageAuditPanel.tsx",
          unit_policy_surface_id: "project-storage-unit-round-trip",
        }),
        expect.objectContaining({
          source_path:
            "apps/desktop/src/features/project-validation/ProjectValidationPanel.tsx",
          unit_policy_surface_id: "project-validation-unit-policy",
        }),
        expect.objectContaining({
          source_path:
            "apps/desktop/src/features/native-package/NativePackagePanel.tsx",
          unit_policy_surface_id: "native-package-unit-witnesses",
        }),
        expect.objectContaining({
          source_path:
            "apps/desktop/src/features/diff-preview/DiffPreviewPanel.tsx",
          unit_policy_surface_id: "operation-diff-unit-witnesses",
        }),
        expect.objectContaining({
          source_path:
            "apps/desktop/src/features/operations/OperationApplyPanel.tsx",
          unit_policy_surface_id: "operation-apply-unit-policy",
        }),
        expect.objectContaining({
          source_path:
            "apps/desktop/src/features/operations/OperationLedgerPanel.tsx",
          unit_policy_surface_id: "operation-ledger-unit-policy",
        }),
        expect.objectContaining({
          source_path:
            "apps/desktop/src/features/report/RenderedReportPanel.tsx",
          unit_policy_surface_id: "rendered-report-unit-basis",
        }),
        expect.objectContaining({
          source_path: "apps/desktop/src/features/results/ResultsPanel.tsx",
          unit_policy_surface_id: "result-unit-policy",
        }),
        expect.objectContaining({
          source_path:
            "apps/desktop/src/features/comparison/ComparisonPanel.tsx",
          unit_policy_surface_id: "comparison-unit-policy",
        }),
        expect.objectContaining({
          source_path:
            "apps/desktop/src/features/agent-proposals/AgentProposalPanel.tsx",
          unit_policy_surface_id: "agent-proposal-unit-policy",
        }),
        expect.objectContaining({
          source_path:
            "apps/desktop/src/features/security-threat-model/SecurityThreatModelPanel.tsx",
          unit_policy_surface_id: "security-threat-model-unit-policy",
        }),
        expect.objectContaining({
          source_path: "apps/desktop/src/features/knowledge/KnowledgePanel.tsx",
          unit_policy_surface_id: "knowledge-unit-context",
        }),
        expect.objectContaining({
          source_path: "apps/desktop/src/features/run-audit/RunAuditPanel.tsx",
          unit_policy_surface_id: "run-audit-units",
        }),
        expect.objectContaining({
          source_path: "apps/desktop/src/features/solve/SolvePanel.tsx",
          unit_policy_surface_id: "solve-job-unit-policy",
        }),
        expect.objectContaining({
          source_path:
            "apps/desktop/src/features/rule-packs/DeclarationsEditor.tsx",
          unit_policy_surface_id: "rule-pack-declarations-unit-policy",
        }),
        expect.objectContaining({
          source_path:
            "apps/desktop/src/features/rule-packs/ExpressionComposer.tsx",
          unit_policy_surface_id: "rule-pack-expression-unit-policy",
        }),
        expect.objectContaining({
          source_path:
            "apps/desktop/src/features/library/LibraryManagerPanel.tsx",
          unit_policy_surface_id: "library-unit-helper-surfaces",
        }),
        expect.objectContaining({
          source_path:
            "apps/desktop/src/features/load-cases/LoadCaseManagerPanel.tsx",
          unit_policy_surface_id: "load-manager-unit-validation-surface",
        }),
        expect.objectContaining({
          source_path:
            "apps/desktop/src/features/model-tree/PropertyInspector.tsx",
          unit_policy_surface_id: "property-inspector-unit-validation-surface",
        }),
        expect.objectContaining({
          source_path:
            "apps/desktop/src/features/diagnostics/DiagnosticsPanel.tsx",
          unit_policy_surface_id: "diagnostic-unit-context",
        }),
        expect.objectContaining({
          source_path:
            "apps/desktop/src/features/rule-check/RuleCheckRunPanel.tsx",
          unit_policy_surface_id: "rule-check-unit-binding-policy",
        }),
        expect.objectContaining({
          source_path:
            "apps/desktop/src/features/rule-check/RuleCheckPanel.tsx",
          unit_policy_surface_id: "rule-completeness-unit-policy",
        }),
        expect.objectContaining({
          source_path:
            "apps/desktop/src/features/result-export/ResultExportPanel.tsx",
          unit_policy_surface_id: "result-export-unit-witnesses",
        }),
        expect.objectContaining({
          source_path:
            "apps/desktop/src/features/headless-runner/HeadlessRunnerPanel.tsx",
          unit_policy_surface_id: "headless-runner-unit-witnesses",
        }),
        expect.objectContaining({
          source_path:
            "apps/desktop/src/features/adapter-framework/AdapterFrameworkPanel.tsx",
          unit_policy_surface_id: "adapter-framework-units",
        }),
        expect.objectContaining({
          source_path: "apps/desktop/src/features/handoff/HandoffPanel.tsx",
          unit_policy_surface_id: "handoff-unit-witnesses",
        }),
        expect.objectContaining({
          source_path:
            "apps/desktop/src/features/pcf-export/PcfExportPanel.tsx",
          conversion_witness_surface_id: "pcf-export-conversion-witnesses",
        }),
        expect.objectContaining({
          source_path:
            "apps/desktop/src/features/caepipe-mbf/CaepipeMbfExportPanel.tsx",
          conversion_witness_surface_id: "caepipe-mbf-conversion-witnesses",
        }),
        expect.objectContaining({
          source_path: "fixtures/product_preview",
          unit_policy_surface_id: "product-preview-fixture-unit-policy",
        }),
      ]),
    );
    expect(lintPacket.lint_run.configuration.private_surface_default).toBe(
      "skip_unless_explicitly_authorized",
    );
    expect(lintPacket.lint_run.configuration.clean_scan_disclaimer).toBe(
      "heuristic_review_evidence_not_legal_or_professional_clearance",
    );
    expect(lintPacket.lint_run.summary.target_count).toBe(46);
    expect(lintPacket.lint_run.summary.finding_count).toBe(0);
    expect(lintPacket.lint_run.summary.blocking_finding_count).toBe(0);
    expect(lintPacket.lint_run.summary.clean_scan_is_clearance).toBe(false);
    const nativePackage = await screen.findByLabelText("Native JSON package");
    expect(
      within(nativePackage).getByTestId("native-package-empty").textContent,
    ).toContain("native JSON package review manifest");
    const comparison = await screen.findByLabelText("Comparison workspace");
    expect(
      within(comparison).getByTestId("comparison-empty").textContent,
    ).toContain(
      "Run mechanics preview to populate the local comparison workspace",
    );
    const handoff = await screen.findByLabelText("Handoff package");
    expect(within(handoff).getByTestId("handoff-empty").textContent).toContain(
      "Run mechanics preview to assemble a local review handoff package",
    );
    const operationLedger = await screen.findByLabelText(
      "Operation review ledger",
    );
    expect(
      within(operationLedger).getByTestId("operation-ledger-empty").textContent,
    ).toContain("No structured operations are queued for review");
    expect(operationLedger.textContent).toContain("does not apply operations");
    const diffPreview = await screen.findByLabelText("Operation diff preview");
    expect(
      within(diffPreview).getByTestId("diff-preview-empty").textContent,
    ).toContain("No operation diffs are queued");
    expect(diffPreview.textContent).toContain("does not apply operations");
    const exportReview = await screen.findByLabelText("Export safety review");
    expect(
      within(exportReview).getByTestId("export-review-summary").textContent,
    ).toContain("21 of 29 local exports ready");
    expect(
      within(exportReview).getByTestId("export-review-redaction").textContent,
    ).toContain("protected content blocked=true");
    expect(
      within(exportReview).getByTestId("export-review-boundary").textContent,
    ).toContain("no release or professional claim");
    expect(
      within(exportReview).getByTestId(
        "export-review-record-telemetry_boundary_review",
      ).textContent,
    ).toContain("available");
    expect(
      within(exportReview).getByTestId(
        "export-review-record-secret_private_library_boundary_review",
      ).textContent,
    ).toContain("available");
    expect(
      within(exportReview).getByTestId(
        "export-review-record-security_threat_model_review",
      ).textContent,
    ).toContain("available");
    expect(
      within(exportReview).getByTestId(
        "export-review-record-accessibility_usability_baseline_review",
      ).textContent,
    ).toContain("available");
    expect(
      within(exportReview).getByTestId(
        "export-review-record-design_authoring_comparison_workspace",
      ).textContent,
    ).toContain("available");
    expect(
      within(exportReview).getByTestId(
        "export-review-record-validation_release_evidence_review",
      ).textContent,
    ).toContain("available");
    expect(
      screen.getByTestId("status-pill-professional").textContent,
    ).toContain("HUMAN_REVIEW_REQUIRED");
    expect(screen.getByTestId("workspace-status-bar")).toBeInTheDocument();
    expect(screen.getByLabelText("Local project controls")).toBeInTheDocument();
    fireEvent.click(screen.getByTestId("audit-drawer-toggle"));
    expect(
      await screen.findByTestId("audit-boundary-drawer"),
    ).toBeInTheDocument();
    expect(screen.getByTestId("local-project-status").textContent).toContain(
      "Browser memory preview local store",
    );
    expect(screen.getByTestId("local-project-status").textContent).toContain(
      "network=false",
    );
    expect(screen.getByTestId("local-project-status").textContent).toContain(
      "telemetry=false",
    );
    expect(screen.getByTestId("local-project-status").textContent).toContain(
      "FTS5=false",
    );
    const storageAudit = await screen.findByLabelText("Project storage audit");
    expect(
      within(storageAudit).getByTestId("project-storage-summary").textContent,
    ).toContain("operation=not_started");
    expect(
      within(storageAudit).getByTestId("project-storage-summary").textContent,
    ).toContain("pending operations=0");
    expect(
      within(storageAudit).getByTestId("project-storage-summary").textContent,
    ).toContain("accepted_state_mutated=false");
    expect(
      within(storageAudit).getByTestId("project-storage-capability")
        .textContent,
    ).toContain("Browser memory preview");
    expect(
      within(storageAudit).getByTestId("project-storage-capability")
        .textContent,
    ).toContain("mode=not_persisted_this_session");
    expect(
      within(storageAudit).getByTestId("project-storage-local-boundary")
        .textContent,
    ).toContain("network=false");
    expect(
      within(storageAudit).getByTestId("project-storage-local-boundary")
        .textContent,
    ).toContain("repository_default_private_write=false");
    expect(
      within(storageAudit).getByTestId("project-storage-project-index")
        .textContent,
    ).toContain("state=not_requested");
    expect(
      within(storageAudit).getByTestId("project-storage-project-index")
        .textContent,
    ).toContain("listed_projects=0");
    expect(
      within(storageAudit).getByTestId("project-storage-project-index")
        .textContent,
    ).toContain("refs=none");
    expect(screen.queryByTestId("project-index-picker")).toBeNull();
    expect(
      within(storageAudit).getByTestId("project-storage-payload-boundary")
        .textContent,
    ).toContain("private payload=false");
    expect(
      within(storageAudit).getByTestId("project-storage-payload-boundary")
        .textContent,
    ).toContain("release/professional claim=false");
    expect(
      within(storageAudit).getByTestId("project-storage-unit-round-trip")
        .textContent,
    ).toContain("status=not_persisted_this_session");
    expect(
      within(storageAudit).getByTestId("project-storage-unit-round-trip")
        .textContent,
    ).toContain("model=angle=rad,force=N,length=m");
    expect(
      within(storageAudit).getByTestId("project-storage-unit-round-trip")
        .textContent,
    ).toContain("conversion=false");
    const storageHref =
      within(storageAudit)
        .getByTestId("project-storage-export-link")
        .getAttribute("href") ?? "";
    const storagePacket = JSON.parse(
      decodeURIComponent(storageHref.split(",", 2)[1]),
    );
    expect(storagePacket.document_kind).toBe(
      "openpipestress.technical_preview.local_project_persistence_audit",
    );
    expect(storagePacket.deliverable_refs).toContain("DEL-02-05");
    expect(storagePacket.deliverable_refs).toContain("DEL-12-01");
    expect(storagePacket.scope_items).toContain("SOW-050");
    expect(storagePacket.scope_items).toContain("SOW-029");
    expect(storagePacket.summary.last_operation).toBe("not_started");
    expect(storagePacket.summary.storage_mode).toBe(
      "not_persisted_this_session",
    );
    expect(storagePacket.summary.pending_operation_count).toBe(0);
    expect(storagePacket.summary.accepted_model_state_mutated).toBe(false);
    expect(storagePacket.summary.network_required).toBe(false);
    expect(storagePacket.summary.daemon_required).toBe(false);
    expect(storagePacket.summary.telemetry_enabled).toBe(false);
    expect(storagePacket.summary.project_index_state).toBe("not_requested");
    expect(storagePacket.summary.listed_project_count).toBe(0);
    expect(storagePacket.summary.unit_round_trip_status).toBe(
      "not_persisted_this_session",
    );
    expect(storagePacket.summary.unit_round_trip_checked_ref_count).toBe(0);
    expect(storagePacket.summary.unit_round_trip_signature).toBe(
      "not_persisted",
    );
    expect(storagePacket.unit_policy_evidence.unit_system_ref.ref).toBe(
      "unit-system:dec-018-si-dual-display",
    );
    expect(storagePacket.unit_policy_evidence.storage_convention).toBe(
      "entered_units_preserved",
    );
    expect(storagePacket.unit_policy_evidence.storage_unit_policy).toBe(
      "local_project_storage_audit_records_unit_round_trip_status_without_conversion",
    );
    expect(storagePacket.unit_policy_evidence.model_units).toEqual({
      angle: "rad",
      force: "N",
      length: "m",
      pressure: "Pa",
      stress: "MPa",
      temperature: "degC",
    });
    expect(storagePacket.unit_policy_evidence.unit_round_trip_status).toBe(
      "not_persisted_this_session",
    );
    expect(
      storagePacket.unit_policy_evidence.unit_round_trip_checked_ref_count,
    ).toBe(0);
    expect(storagePacket.unit_policy_evidence.unit_round_trip_signature).toBe(
      "not_persisted",
    );
    expect(storagePacket.unit_policy_evidence.conversion_performed).toBe(false);
    expect(storagePacket.project_index).toEqual([]);
    expect(storagePacket.project_index_refs).toEqual([]);
    expect(storagePacket.boundary.repository_default_private_write).toBe(false);
    expect(storagePacket.private_payload_included).toBe(false);
    expect(storagePacket.protected_content_included).toBe(false);
    expect(storagePacket.release_or_professional_claim).toBe(false);
    fireEvent.click(
      within(screen.getByTestId("audit-boundary-drawer")).getByRole("button", {
        name: /Close/i,
      }),
    );
    const projectValidation = await screen.findByLabelText(
      "Project validation preflight",
    );
    expect(
      within(projectValidation).getByTestId("project-validation-summary")
        .textContent,
    ).toContain("validation=preview_not_persisted");
    expect(
      within(projectValidation).getByTestId("project-validation-summary")
        .textContent,
    ).toContain("version=stale");
    expect(
      within(projectValidation).getByTestId("project-validation-summary")
        .textContent,
    ).toContain("migration=not_persisted_this_session");
    await waitFor(() =>
      expect(
        within(projectValidation).getByTestId(
          "project-validation-store-migration",
        ).textContent,
      ).toContain(
        "framework=browser_memory_preview_no_sqlite_migration_ledger",
      ),
    );
    expect(
      within(projectValidation).getByTestId(
        "project-validation-store-migration",
      ).textContent,
    ).toContain("applied_on_open=0");
    expect(
      within(projectValidation).getByTestId(
        "project-validation-store-migration",
      ).textContent,
    ).toContain(
      "status=browser_memory_snapshot_no_sql_store_migrations_applicable",
    );
    expect(
      within(projectValidation).getByTestId("project-validation-schema")
        .textContent,
    ).toContain("schema_version=0.1.0");
    expect(
      within(projectValidation).getByTestId("project-validation-round-trip")
        .textContent,
    ).toContain("6 categories");
    await waitFor(() =>
      expect(
        within(projectValidation).getByTestId("project-validation-model-hash")
          .textContent,
      ).toContain("model_hash=model_hash_computed_not_persisted"),
    );
    expect(
      within(projectValidation).getByTestId("project-validation-model-hash")
        .textContent,
    ).toContain("persisted_model_hashes=0");
    expect(
      within(projectValidation).getByTestId("project-validation-model-hash")
        .textContent,
    ).toContain("integrity=open_verification_not_run_this_session");
    expect(
      within(projectValidation).getByTestId("project-validation-envelope-hash")
        .textContent,
    ).toContain(
      "envelope_hash=project_envelope_hash_not_computed_no_save_this_session",
    );
    expect(
      within(projectValidation).getByTestId("project-validation-envelope-hash")
        .textContent,
    ).toContain("persisted_envelope_hashes=0");
    expect(
      within(projectValidation).getByTestId("project-validation-envelope-hash")
        .textContent,
    ).toContain("integrity=open_verification_not_run_this_session");
    expect(
      within(projectValidation).getByTestId("project-validation-round-trip")
        .textContent,
    ).toContain("reproducibility=model_hash_computed_not_persisted");
    expect(
      within(projectValidation).getByTestId(
        "project-validation-unit-round-trip",
      ).textContent,
    ).toContain("status=not_persisted_this_session");
    expect(
      within(projectValidation).getByTestId("project-validation-unit-policy")
        .textContent,
    ).toContain(
      "model=angle=rad,force=N,length=m,pressure=Pa,stress=MPa,temperature=degC",
    );
    expect(
      within(projectValidation).getByTestId("project-validation-unit-policy")
        .textContent,
    ).toContain("records=45");
    expect(
      within(projectValidation).getByTestId("project-validation-unit-policy")
        .textContent,
    ).toContain("round_trip=not_persisted_this_session");
    expect(
      within(projectValidation).getByTestId("project-validation-unit-policy")
        .textContent,
    ).toContain("conversion=false");
    expect(
      within(projectValidation).getByTestId("project-validation-operations")
        .textContent,
    ).toContain("version_check=stale");
    expect(
      within(projectValidation).getByTestId("project-validation-boundary")
        .textContent,
    ).toContain("accepted_state_mutated=false");
    const validationHref =
      within(projectValidation)
        .getByTestId("project-validation-export-link")
        .getAttribute("href") ?? "";
    const validationPacket = JSON.parse(
      decodeURIComponent(validationHref.split(",", 2)[1]),
    );
    expect(validationPacket.document_kind).toBe(
      "openpipestress.technical_preview.project_validation_preflight",
    );
    expect(validationPacket.deliverable_refs).toContain("DEL-02-05");
    expect(validationPacket.deliverable_refs).toContain("DEL-12-01");
    expect(validationPacket.scope_items).toContain("SOW-041");
    expect(validationPacket.scope_items).toContain("SOW-029");
    expect(validationPacket.summary.validation_status).toBe(
      "preview_not_persisted",
    );
    expect(validationPacket.summary.version_check_status).toBe(
      "stale",
    );
    expect(
      validationPacket.diagnostics.map(
        (item: { code: string }) => item.code,
      ),
    ).toContain("PROJECT-VALIDATION-STALE-SCHEMA");
    expect(
      validationPacket.diagnostics.map(
        (item: { code: string }) => item.code,
      ),
    ).not.toContain("PROJECT-VALIDATION-UNSUPPORTED-SCHEMA");
    expect(validationPacket.summary.migration_status).toBe(
      "not_persisted_this_session",
    );
    expect(validationPacket.summary.unit_round_trip_status).toBe(
      "not_persisted_this_session",
    );
    expect(validationPacket.unit_round_trip_evidence).toMatchObject({
      status: "not_persisted_this_session",
      checked_ref_count: 0,
      signature: "not_persisted",
      evidence_source: "not_persisted_this_session",
    });
    expect(validationPacket.unit_policy_evidence.unit_system_ref.ref).toBe(
      "unit-system:dec-018-si-dual-display",
    );
    expect(validationPacket.unit_policy_evidence.storage_convention).toBe(
      "entered_units_preserved",
    );
    expect(validationPacket.unit_policy_evidence.validation_unit_policy).toBe(
      "validate_round_trip_preserves_explicit_model_unit_metadata_without_conversion",
    );
    expect(validationPacket.unit_policy_evidence.model_units).toEqual({
      angle: "rad",
      force: "N",
      length: "m",
      pressure: "Pa",
      stress: "MPa",
      temperature: "degC",
    });
    expect(
      validationPacket.unit_policy_evidence.unit_bearing_record_count,
    ).toBe(45);
    expect(validationPacket.unit_policy_evidence.unit_round_trip_status).toBe(
      "not_persisted_this_session",
    );
    expect(
      validationPacket.unit_policy_evidence.unit_round_trip_signature,
    ).toBe("not_persisted");
    expect(validationPacket.unit_policy_evidence.conversion_performed).toBe(
      false,
    );
    expect(
      validationPacket.unit_policy_evidence.decision_basis_refs.map(
        (item: { ref: string }) => item.ref,
      ),
    ).toEqual(["DEC-018", "DEL-02-02", "DEL-02-05"]);
    expect(validationPacket.store_migration.evidence_source).toBe(
      "storage_capability_probe",
    );
    expect(validationPacket.store_migration.migration_framework).toBe(
      "browser_memory_preview_no_sqlite_migration_ledger",
    );
    expect(validationPacket.store_migration.migrations_applied_on_open).toEqual(
      [],
    );
    expect(validationPacket.model_document_migration.status).toBe("migrated");
    expect(validationPacket.model_document_migration.evidence_source).toBe(
      "session_document_local_evaluation",
    );
    expect(validationPacket.model_document_migration.version_authority).toBe(
      "in_document_schema_version_semver",
    );
    expect(validationPacket.model_document_migration.decision_basis).toBe(
      "DEC-019_model_document_schema_migration_policy",
    );
    expect(validationPacket.model_document_migration.ledger_record_count).toBe(
      0,
    );
    expect(
      validationPacket.model_document_migration.down_migration_performed,
    ).toBe(false);
    expect(
      validationPacket.service_operations.find(
        (operation: { operation: string }) => operation.operation === "migrate",
      ).operation_status,
    ).toBe("not_run_no_local_snapshot_this_session");
    expect(
      validationPacket.validation_profile.model_document_migration_status,
    ).toBe("migrated");
    expect(validationPacket.summary.round_trip_status).toBe(
      "semantic_categories_declared",
    );
    expect(validationPacket.round_trip_manifest.category_count).toBe(6);
    expect(
      validationPacket.round_trip_manifest.categories.map(
        (category: { category: string }) => category.category,
      ),
    ).toContain("unit_metadata");
    expect(
      validationPacket.service_operations.map(
        (operation: { operation: string }) => operation.operation,
      ),
    ).toContain("validate");
    expect(
      validationPacket.service_operations.map(
        (operation: { operation: string }) => operation.operation,
      ),
    ).toContain("version_check");
    expect(
      validationPacket.service_operations.map(
        (operation: { operation: string }) => operation.operation,
      ),
    ).toContain("migrate");
    expect(validationPacket.validation_profile.hash_service_status).toBe(
      "canonical_model_hash_service_available_model_payload_scope",
    );
    expect(
      validationPacket.validation_profile.project_envelope_hash_status,
    ).toBe("project_envelope_hash_not_computed_no_save_this_session");
    expect(
      validationPacket.validation_profile.project_envelope_hash_scope,
    ).toBe(
      "persisted_envelope_payload_excluding_storage_summary_and_hash_carrier",
    );
    expect(validationPacket.summary.model_hash_status).toBe(
      "model_hash_computed_not_persisted",
    );
    expect(validationPacket.summary.persisted_model_hash_count).toBe(0);
    expect(validationPacket.summary.persisted_model_hash_ref).toBe(
      "not_persisted",
    );
    expect(validationPacket.summary.model_hash_integrity_status).toBe(
      "open_verification_not_run_this_session",
    );
    expect(validationPacket.model_hash.value).toMatch(/^sha256:[0-9a-f]{64}$/);
    expect(validationPacket.model_hash_integrity).toBeNull();
    expect(validationPacket.summary.project_envelope_hash_status).toBe(
      "project_envelope_hash_not_computed_no_save_this_session",
    );
    expect(validationPacket.summary.persisted_project_envelope_hash_count).toBe(
      0,
    );
    expect(validationPacket.summary.persisted_project_envelope_hash_ref).toBe(
      "not_persisted",
    );
    expect(
      validationPacket.summary.project_envelope_hash_integrity_status,
    ).toBe("open_verification_not_run_this_session");
    expect(validationPacket.project_envelope_hash).toBeNull();
    expect(validationPacket.project_envelope_hash_integrity).toBeNull();
    expect(
      validationPacket.diagnostics.map(
        (diagnosticEntry: { code: string }) => diagnosticEntry.code,
      ),
    ).toContain("PROJECT-VALIDATION-ENVELOPE-HASH-NOT-COMPUTED");
    expect(
      validationPacket.round_trip_manifest.categories.find(
        (category: { category: string }) =>
          category.category === "reproducibility_metadata",
      ).semantic_equality_status,
    ).toBe("model_hash_computed_not_persisted");
    expect(
      validationPacket.diagnostics.map(
        (diagnosticEntry: { code: string }) => diagnosticEntry.code,
      ),
    ).toContain("PROJECT-VALIDATION-MODEL-HASH-REVIEW-ONLY");
    expect(
      validationPacket.diagnostics.map(
        (diagnosticEntry: { code: string }) => diagnosticEntry.code,
      ),
    ).not.toContain("PROJECT-VALIDATION-HASH-SERVICE-TBD");
    expect(validationPacket.boundary.local_only_project_store).toBe(true);
    expect(validationPacket.boundary.repository_default_private_write).toBe(
      false,
    );
    expect(validationPacket.boundary.accepted_model_state_mutated).toBe(false);
    const telemetryBoundary = (
      await screen.findAllByLabelText("Telemetry boundary review")
    )[0];
    expect(
      within(telemetryBoundary).getByTestId("telemetry-boundary-summary")
        .textContent,
    ).toContain("disabled=true");
    expect(
      within(telemetryBoundary).getByTestId("telemetry-boundary-summary")
        .textContent,
    ).toContain("blocked=4");
    expect(
      within(telemetryBoundary).getByTestId("telemetry-boundary-summary")
        .textContent,
    ).toContain("network=false");
    expect(
      within(telemetryBoundary).getByTestId("telemetry-boundary-config")
        .textContent,
    ).toContain("opt_in=false");
    expect(
      within(telemetryBoundary).getByTestId("telemetry-boundary-guard")
        .textContent,
    ).toContain("metadata_only=true");
    expect(
      within(telemetryBoundary).getByTestId("telemetry-boundary-attempts")
        .textContent,
    ).toContain("allowed=0");
    expect(
      within(telemetryBoundary).getByTestId("telemetry-boundary-no-bypass")
        .textContent,
    ).toContain("plugins");
    expect(
      within(telemetryBoundary).getByTestId("telemetry-boundary-runtime")
        .textContent,
    ).toContain("endpoint=false");
    const telemetryHref =
      within(telemetryBoundary)
        .getByTestId("telemetry-boundary-export-link")
        .getAttribute("href") ?? "";
    const telemetryPacket = JSON.parse(
      decodeURIComponent(telemetryHref.split(",", 2)[1]),
    );
    expect(telemetryPacket.document_kind).toBe(
      "openpipestress.technical_preview.telemetry_boundary_review",
    );
    expect(telemetryPacket.deliverable_id).toBe("DEL-12-03");
    expect(telemetryPacket.package_id).toBe("PKG-12");
    expect(telemetryPacket.scope_item).toBe("SOW-037");
    expect(telemetryPacket.objective).toBe("OBJ-010");
    expect(telemetryPacket.summary.telemetry_enabled).toBe(false);
    expect(telemetryPacket.summary.attempted_event_count).toBe(4);
    expect(telemetryPacket.summary.allowed_event_count).toBe(0);
    expect(telemetryPacket.summary.blocked_event_count).toBe(4);
    expect(telemetryPacket.summary.payload_constructed).toBe(false);
    expect(telemetryPacket.summary.network_transport_initialized).toBe(false);
    expect(telemetryPacket.config_resolution.product_config_schema).toBe("TBD");
    expect(telemetryPacket.config_resolution.explicit_opt_in).toBe(false);
    expect(telemetryPacket.config_resolution.allowlist_approved).toBe(false);
    expect(telemetryPacket.metadata_only_guard.guard_present).toBe(true);
    expect(
      telemetryPacket.metadata_only_guard.helper_authorizes_runtime_telemetry,
    ).toBe(false);
    expect(
      telemetryPacket.event_attempts.map(
        (item: { event_name: string }) => item.event_name,
      ),
    ).toContain("report_export_attempted");
    expect(
      telemetryPacket.event_attempts.every(
        (item: {
          decision: {
            payload_constructed: boolean;
            network_behavior_initialized: boolean;
          };
        }) =>
          item.decision.payload_constructed === false &&
          item.decision.network_behavior_initialized === false,
      ),
    ).toBe(true);
    expect(telemetryPacket.runtime_initialization.endpoint_initialized).toBe(
      false,
    );
    expect(telemetryPacket.runtime_initialization.vendor_initialized).toBe(
      false,
    );
    expect(
      telemetryPacket.runtime_initialization.upload_queue_initialized,
    ).toBe(false);
    expect(
      telemetryPacket.runtime_initialization.telemetry_persistence_initialized,
    ).toBe(false);
    expect(telemetryPacket.no_bypass_surfaces).toContain("reports");
    expect(telemetryPacket.forbidden_payload_classes).toContain(
      "protected_standards_content",
    );
    expect(telemetryPacket.private_payload_included).toBe(false);
    expect(telemetryPacket.protected_content_included).toBe(false);
    expect(telemetryPacket.security_certification_claim).toBe(false);
    const secretPrivateLibrary = await screen.findByLabelText(
      "Secret and private-library review",
    );
    expect(
      within(secretPrivateLibrary).getByTestId("secret-private-library-summary")
        .textContent,
    ).toContain("records=4");
    expect(
      within(secretPrivateLibrary).getByTestId("secret-private-library-summary")
        .textContent,
    ).toContain("metadata_only=true");
    expect(
      within(secretPrivateLibrary).getByTestId("secret-private-library-summary")
        .textContent,
    ).toContain("public_fixture_blocked=4");
    expect(
      within(secretPrivateLibrary).getByTestId("secret-private-library-secrets")
        .textContent,
    ).toContain("provider=TBD");
    expect(
      within(secretPrivateLibrary).getByTestId(
        "secret-private-library-unit-policy",
      ).textContent,
    ).toContain("unit_refs=2");
    expect(
      within(secretPrivateLibrary).getByTestId(
        "secret-private-library-unit-policy",
      ).textContent,
    ).toContain("required=true");
    expect(
      within(secretPrivateLibrary).getByTestId(
        "secret-private-library-unit-policy",
      ).textContent,
    ).toContain("payload=false");
    expect(
      within(secretPrivateLibrary).getByTestId(
        "secret-private-library-unit-policy",
      ).textContent,
    ).toContain("conversion=false");
    expect(
      within(secretPrivateLibrary).getByTestId(
        "secret-private-library-no-bypass",
      ).textContent,
    ).toContain("direct_sql=false");
    expect(
      within(secretPrivateLibrary).getByTestId(
        "secret-private-library-boundary",
      ).textContent,
    ).toContain("security_certification=false");
    const secretPrivateHref =
      within(secretPrivateLibrary)
        .getByTestId("secret-private-library-export-link")
        .getAttribute("href") ?? "";
    const secretPrivatePacket = JSON.parse(
      decodeURIComponent(secretPrivateHref.split(",", 2)[1]),
    );
    expect(secretPrivatePacket.document_kind).toBe(
      "openpipestress.technical_preview.secret_private_library_boundary_review",
    );
    expect(secretPrivatePacket.deliverable_id).toBe("DEL-12-04");
    expect(secretPrivatePacket.package_id).toBe("PKG-12");
    expect(secretPrivatePacket.scope_items).toContain("SOW-040");
    expect(secretPrivatePacket.scope_items).toContain("SOW-029");
    expect(secretPrivatePacket.objective).toBe("OBJ-010");
    expect(secretPrivatePacket.summary.reference_count).toBe(4);
    expect(secretPrivatePacket.summary.private_library_count).toBe(2);
    expect(secretPrivatePacket.summary.private_path_count).toBe(1);
    expect(secretPrivatePacket.summary.credential_reference_count).toBe(1);
    expect(secretPrivatePacket.summary.metadata_only).toBe(true);
    expect(secretPrivatePacket.summary.payload_present).toBe(false);
    expect(secretPrivatePacket.summary.secret_material_present).toBe(false);
    expect(secretPrivatePacket.summary.public_fixture_block_count).toBe(4);
    expect(secretPrivatePacket.unit_policy.private_unit_reference_count).toBe(
      2,
    );
    expect(
      secretPrivatePacket.unit_policy.explicit_unit_metadata_required,
    ).toBe(true);
    expect(secretPrivatePacket.unit_policy.unit_payload_included).toBe(false);
    expect(secretPrivatePacket.unit_policy.conversion_performed).toBe(false);
    expect(
      secretPrivatePacket.unit_policy.repository_default_private_write,
    ).toBe(false);
    expect(secretPrivatePacket.unit_policy.unit_metadata_statuses).toEqual([
      {
        reference_id: "spl:private-material-library:invented-preview",
        record_kind: "private_material_library",
        unit_metadata_status:
          "private_unit_bearing_values_withheld_explicit_unit_metadata_required_at_use",
      },
      {
        reference_id: "spl:private-rule-pack:invented-preview",
        record_kind: "private_rule_pack",
        unit_metadata_status:
          "private_rule_inputs_withheld_explicit_unit_metadata_required_at_use",
      },
    ]);
    expect(
      secretPrivatePacket.guard_contexts.public_fixture.summary.blocked_count,
    ).toBe(4);
    expect(
      secretPrivatePacket.guard_contexts.public_report.summary
        .metadata_only_count,
    ).toBe(4);
    expect(
      secretPrivatePacket.guard_contexts.local_private
        .explicit_local_private_intent,
    ).toBe(true);
    expect(
      secretPrivatePacket.registry_records.every(
        (item: {
          contains_payload: boolean;
          secret_material_present: boolean;
          concrete_path_present: boolean;
        }) =>
          item.contains_payload === false &&
          item.secret_material_present === false &&
          item.concrete_path_present === false,
      ),
    ).toBe(true);
    expect(secretPrivatePacket.runtime_deferrals.exact_secret_provider).toBe(
      "TBD",
    );
    expect(
      secretPrivatePacket.runtime_deferrals.encrypted_storage_default,
    ).toBe("TBD");
    expect(secretPrivatePacket.no_bypass_controls.direct_sql_allowed).toBe(
      false,
    );
    expect(secretPrivatePacket.no_bypass_controls.storage_bypass_allowed).toBe(
      false,
    );
    expect(secretPrivatePacket.private_payload_included).toBe(false);
    expect(secretPrivatePacket.protected_content_included).toBe(false);
    expect(secretPrivatePacket.security_certification_claim).toBe(false);
    const securityThreatModel = await screen.findByLabelText(
      "Security threat model review",
    );
    expect(
      within(securityThreatModel).getByTestId("security-threat-model-summary")
        .textContent,
    ).toContain("threats=19");
    expect(
      within(securityThreatModel).getByTestId("security-threat-model-summary")
        .textContent,
    ).toContain("high=14");
    expect(
      within(securityThreatModel).getByTestId("security-threat-model-coverage")
        .textContent,
    ).toContain("workflows=6");
    expect(
      within(securityThreatModel).getByTestId(
        "security-threat-model-unit-policy",
      ).textContent,
    ).toContain("unit_checks=true");
    expect(
      within(securityThreatModel).getByTestId(
        "security-threat-model-unit-policy",
      ).textContent,
    ).toContain("conversion=false");
    expect(
      within(securityThreatModel).getByTestId(
        "security-threat-model-open-decisions",
      ).textContent,
    ).toContain("tbd=13");
    expect(
      within(securityThreatModel).getByTestId("security-threat-model-boundary")
        .textContent,
    ).toContain("security_certification=false");
    const threatModelHref =
      within(securityThreatModel)
        .getByTestId("security-threat-model-export-link")
        .getAttribute("href") ?? "";
    const threatModelPacket = JSON.parse(
      decodeURIComponent(threatModelHref.split(",", 2)[1]),
    );
    expect(threatModelPacket.document_kind).toBe(
      "openpipestress.technical_preview.security_threat_model_review",
    );
    expect(threatModelPacket.deliverable_id).toBe("DEL-12-05");
    expect(threatModelPacket.package_id).toBe("PKG-12");
    expect(threatModelPacket.scope_item).toBe("SOW-040");
    expect(threatModelPacket.objective).toBe("OBJ-010");
    expect(threatModelPacket.summary.threat_count).toBe(19);
    expect(threatModelPacket.summary.high_risk_count).toBe(14);
    expect(threatModelPacket.summary.medium_high_risk_count).toBe(5);
    expect(threatModelPacket.summary.trust_boundary_count).toBe(10);
    expect(threatModelPacket.summary.asset_class_count).toBe(10);
    expect(threatModelPacket.summary.export_workflow_count).toBe(6);
    expect(threatModelPacket.summary.open_decision_count).toBe(14);
    expect(threatModelPacket.summary.tbd_decision_count).toBe(13);
    expect(threatModelPacket.asset_classes).toContain(
      "import_export_and_local_fea_handoff_data",
    );
    expect(threatModelPacket.trust_boundaries).toContain(
      "plugin_import_export_fea_handoff",
    );
    expect(threatModelPacket.export_workflows).toContain(
      "export_adapter_sdk_and_additional_targets",
    );
    expect(threatModelPacket.no_bypass_controls.direct_sql_allowed).toBe(false);
    expect(
      threatModelPacket.no_bypass_controls
        .plugin_manifest_grants_runtime_access,
    ).toBe(false);
    expect(threatModelPacket.unit_policy_evidence.evidence_id).toBe(
      "unit-policy-evidence:security-threat-model-no-bypass",
    );
    expect(threatModelPacket.unit_policy_evidence.unit_checks_required).toBe(
      true,
    );
    expect(threatModelPacket.unit_policy_evidence.export_workflow_count).toBe(
      6,
    );
    expect(threatModelPacket.unit_policy_evidence.conversion_performed).toBe(
      false,
    );
    expect(
      threatModelPacket.unit_policy_evidence.security_certification_claim,
    ).toBe(false);
    expect(threatModelPacket.open_decisions.plugin_permission_model).toBe(
      "TBD",
    );
    expect(threatModelPacket.open_decisions.telemetry_event_schema).toBe("TBD");
    expect(threatModelPacket.private_payload_included).toBe(false);
    expect(threatModelPacket.protected_content_included).toBe(false);
    expect(threatModelPacket.security_certification_claim).toBe(false);
    expect(validationPacket.private_payload_included).toBe(false);
    expect(validationPacket.protected_content_included).toBe(false);
    expect(validationPacket.release_or_professional_claim).toBe(false);

    fireEvent.click(screen.getByTestId("audit-drawer-toggle"));
    const boundary = await screen.findByTestId("audit-boundary-drawer");
    expect(within(boundary).getByText("Public data")).toBeInTheDocument();
    expect(boundary.textContent).toContain("invented_or_cleared_data_only");
    expect(boundary.textContent).toContain(
      "no_bundled_protected_owner_or_standards_data",
    );
    expect(boundary.textContent).toContain("no_private_project_data");
    expect(boundary.textContent).toContain(
      "technical_preview_requires_human_engineering_review",
    );
  });

  it("records viewport editor intents without direct persisted-project mutation", async () => {
    render(<App />);

    expect(
      await screen.findByLabelText("Three.js pipe centerline viewport"),
    ).toBeInTheDocument();
    expect(
      await screen.findByTestId("viewport-unit-catalog-status"),
    ).toHaveTextContent("browser preview uses model metadata");
    const intentPanel = screen.getByLabelText("Viewport editor intents");
    const commandBar = screen.getByTestId("command-bar");
    expect(
      screen.getByTestId("local-project-review-context").textContent,
    ).toContain("0 pending operations");

    expect(
      within(intentPanel).getByTestId("viewport-intent-empty").textContent,
    ).toContain("do not mutate persisted project data directly");

    // The unit-catalog status chip reads identically whether the catalog is
    // still loading (route === null) or has resolved to the browser-preview
    // fallback, so awaiting the chip text alone does not gate readiness. Wait
    // for the resolved route to reach the viewport draft — the node unit-basis
    // flips from "catalog loading" to "model metadata" only once loadUnitCatalog
    // settles — before queuing intents; otherwise, under parallel-suite load,
    // the queued unit_validation can capture the transient catalog_loading
    // status. Mirrors the unit-basis gating used elsewhere in this file.
    await waitFor(() =>
      expect(
        screen.getByTestId("viewport-create-node-unit-basis"),
      ).toHaveTextContent("model metadata"),
    );

    fireEvent.click(within(commandBar).getByRole("button", { name: /Node/i }));
    expect(
      within(commandBar).getByTestId("armed-creation-tool"),
    ).toHaveTextContent("Node tool armed");
    expect(
      screen.getByTestId("local-project-review-context").textContent,
    ).toContain("0 pending operations");
    fireEvent.click(
      within(commandBar).getByTestId("queue-armed-creation-intent"),
    );
    fireEvent.click(within(commandBar).getByRole("button", { name: /Pipe/i }));
    expect(
      within(commandBar).getByTestId("armed-creation-tool"),
    ).toHaveTextContent("Pipe tool armed");
    fireEvent.click(
      within(commandBar).getByTestId("queue-armed-creation-intent"),
    );
    fireEvent.click(
      within(commandBar).getByRole("button", { name: /Component/i }),
    );
    expect(
      within(commandBar).getByTestId("armed-creation-tool"),
    ).toHaveTextContent("Component tool armed");
    fireEvent.click(
      within(commandBar).getByTestId("queue-armed-creation-intent"),
    );

    const nodeIntent = within(intentPanel).getByTestId(
      "viewport-intent-create_node",
    );
    const pipeIntent = within(intentPanel).getByTestId(
      "viewport-intent-connect_pipe_run",
    );
    const componentIntent = within(intentPanel).getByTestId(
      "viewport-intent-insert_component_symbol",
    );

    for (const intent of [nodeIntent, pipeIntent]) {
      expect(intent.textContent).toContain("pending_service_validation");
      expect(intent.textContent).toContain(
        "unit_validation=length=model_metadata_unit_dimension_declared_catalog_unavailable_browser_preview",
      );
      expect(intent.textContent).toContain(
        "does_not_mutate_persisted_project_payload",
      );
    }
    expect(componentIntent.textContent).toContain("pending_service_validation");
    expect(componentIntent.textContent).toContain(
      "unit_validation=not_required_dimensionless",
    );
    expect(componentIntent.textContent).toContain(
      "does_not_mutate_persisted_project_payload",
    );
    expect(componentIntent.textContent).toContain("editor-intent-3");
    expect(pipeIntent.textContent).toContain("editor-intent-2");
    expect(nodeIntent.textContent).toContain("editor-intent-1");
    expect(
      screen.getByTestId("local-project-review-context").textContent,
    ).toContain("3 pending operations");

    const editorContract = screen.getByLabelText("Editor contract review");
    expect(
      within(editorContract).getByTestId("editor-contract-validation")
        .textContent,
    ).toContain("queued=3");
    const editorContractHref =
      within(editorContract)
        .getByTestId("editor-contract-export-link")
        .getAttribute("href") ?? "";
    const editorContractPacket = JSON.parse(
      decodeURIComponent(editorContractHref.split(",", 2)[1]),
    );
    expect(editorContractPacket.summary.queued_intent_count).toBe(3);
    expect(editorContractPacket.summary.surface_count).toBe(7);
    expect(editorContractPacket.queued_operation_intent_refs).toContain(
      "op:viewport-intent-create_node-001",
    );
    expect(
      editorContractPacket.surface_inventory.map(
        (item: { surface: string }) => item.surface,
      ),
    ).toContain("viewport_gestures");

    const diffPreview = await screen.findByLabelText("Operation diff preview");
    expect(
      within(diffPreview).getByTestId("diff-preview-summary").textContent,
    ).toContain("3 operations");
    expect(
      within(diffPreview).getByTestId("diff-preview-summary").textContent,
    ).toContain("3 diff rows");
    expect(
      within(diffPreview).getByTestId("diff-preview-validation").textContent,
    ).toContain("3 held");
    const diffHref =
      within(diffPreview)
        .getByTestId("diff-preview-export-link")
        .getAttribute("href") ?? "";
    const diffPacket = JSON.parse(
      decodeURIComponent(diffHref.split(",", 2)[1]),
    );
    expect(diffPacket.summary.operation_count).toBe(3);
    expect(diffPacket.summary.diff_row_count).toBe(3);
    expect(
      diffPacket.previews.map(
        (item: { operation_id: string }) => item.operation_id,
      ),
    ).toContain("op:viewport-intent-create_node-001");
    expect(
      diffPacket.previews.every(
        (item: { accepted_model_state_mutated: boolean }) =>
          item.accepted_model_state_mutated === false,
      ),
    ).toBe(true);

    const operationLedger = await screen.findByLabelText(
      "Operation review ledger",
    );
    expect(
      within(operationLedger).getByTestId("operation-ledger-export-summary")
        .textContent,
    ).toContain("3 review records");
    expect(
      within(operationLedger).getByTestId("operation-ledger-decision-counts")
        .textContent,
    ).toContain("3 held_for_user_acceptance");
    expect(
      within(operationLedger).getByTestId("operation-ledger-unit-policy")
        .textContent,
    ).toContain("records=3");
    expect(
      within(operationLedger).getByTestId("operation-ledger-unit-policy")
        .textContent,
    ).toContain("unit_bearing_changes=2");
    expect(
      within(operationLedger).getByTestId("operation-ledger-unit-policy")
        .textContent,
    ).toContain("dimensionless_changes=1");
    expect(
      within(operationLedger).getByTestId("operation-ledger-unit-policy")
        .textContent,
    ).toContain("conversion=false");
    const ledgerHref =
      within(operationLedger)
        .getByTestId("operation-ledger-export-link")
        .getAttribute("href") ?? "";
    const ledgerPacket = JSON.parse(
      decodeURIComponent(ledgerHref.split(",", 2)[1]),
    );
    expect(ledgerPacket.decision_counts.held_for_user_acceptance).toBe(3);
    expect(ledgerPacket.unit_policy_evidence.unit_bearing_change_count).toBe(2);
    expect(ledgerPacket.unit_policy_evidence.dimensionless_change_count).toBe(
      1,
    );
    expect(ledgerPacket.unit_policy_evidence.conversion_performed).toBe(false);
    expect(ledgerPacket.unit_policy_evidence.applied_receipt_units).toBe(
      "not_serialized_in_review_ledger",
    );
    expect(ledgerPacket.records[0].source.source_ref).toBe(
      "apps/desktop/src/features/viewport/PipeViewport.tsx",
    );
    expect(ledgerPacket.records[0].actor.source_role).toBe("viewport_editor");
    expect(
      ledgerPacket.records.every(
        (item: { accepted_model_state_ref: { ref: string } }) =>
          item.accepted_model_state_ref.ref === "not generated",
      ),
    ).toBe(true);
    expect(ledgerPacket.accepted_model_state_unchanged).toBe(true);
    expect(ledgerPacket.release_or_professional_claim).toBe(false);

    const exportReview = await screen.findByLabelText("Export safety review");
    expect(
      within(exportReview).getByTestId("export-review-summary").textContent,
    ).toContain("22 of 29 local exports ready");
    const reviewHref =
      within(exportReview)
        .getByTestId("export-review-link")
        .getAttribute("href") ?? "";
    const reviewManifest = JSON.parse(
      decodeURIComponent(reviewHref.split(",", 2)[1]),
    );
    expect(reviewManifest.deliverable_refs).toContain("DEL-07-01");
    expect(reviewManifest.scope_items).toContain("SOW-020");
    expect(reviewManifest.summary.operation_record_count).toBe(3);
    expect(
      reviewManifest.exports.find(
        (item: { export_id: string }) =>
          item.export_id === "operation_review_ledger",
      ).readiness,
    ).toBe("available");

    fireEvent.click(
      within(operationLedger).getByTestId("clear-operation-review-queue"),
    );

    expect(
      screen.getByTestId("local-project-review-context").textContent,
    ).toContain("0 pending operations");
    expect(
      within(intentPanel).getByTestId("viewport-intent-empty").textContent,
    ).toContain("do not mutate persisted project data directly");
    expect(
      within(editorContract).getByTestId("editor-contract-validation")
        .textContent,
    ).toContain("queued=0");
    expect(
      await within(operationLedger).findByTestId("operation-ledger-empty"),
    ).toHaveTextContent("No structured operations are queued");
    expect(
      within(operationLedger).queryByTestId("operation-ledger-export-link"),
    ).not.toBeInTheDocument();
    expect(
      await within(diffPreview).findByTestId("diff-preview-empty"),
    ).toHaveTextContent("No operation diffs");

    const clearedReviewHref =
      within(exportReview)
        .getByTestId("export-review-link")
        .getAttribute("href") ?? "";
    const clearedReviewManifest = JSON.parse(
      decodeURIComponent(clearedReviewHref.split(",", 2)[1]),
    );
    expect(clearedReviewManifest.summary.operation_record_count).toBe(0);
    expect(
      clearedReviewManifest.exports.find(
        (item: { export_id: string }) =>
          item.export_id === "operation_review_ledger",
      ).readiness,
    ).toBe("empty_operation_queue");
  });

  it("selects rendered viewport entities into the shared model inspector", async () => {
    render(<App />);

    const viewportSelection = await screen.findByTestId(
      "viewport-selection-layer",
    );
    const tree = await screen.findByLabelText("Model tree");
    const inspector = screen.getByLabelText("Property inspector");

    expect(within(viewportSelection).getAllByRole("button")).toHaveLength(21);

    fireEvent.click(
      within(viewportSelection).getByTestId("viewport-select-pipe:P-120"),
    );
    expect(
      within(inspector).getByRole("heading", { name: "Rack span" }),
    ).toBeInTheDocument();
    expect(inspector.textContent).toContain("pipe:P-120");
    expect(inspector.textContent).toContain("node:N-120");
    expect(within(tree).getByTestId("tree-row-pipe:P-120")).toHaveClass(
      "active",
    );
    expect(
      within(viewportSelection).getByTestId("viewport-select-pipe:P-120"),
    ).toHaveAttribute("aria-pressed", "true");

    fireEvent.click(
      within(viewportSelection).getByTestId("viewport-select-support:S-120"),
    );
    expect(
      within(inspector).getByRole("heading", { name: "Guide on riser" }),
    ).toBeInTheDocument();
    expect(inspector.textContent).toContain("support:S-120");
    expect(inspector.textContent).toContain("UX, UZ");
    expect(within(tree).getByTestId("tree-row-support:S-120")).toHaveClass(
      "active",
    );

    fireEvent.click(
      within(viewportSelection).getByTestId("viewport-select-component:C-110"),
    );
    expect(
      within(inspector).getByRole("heading", { name: "Invented elbow marker" }),
    ).toBeInTheDocument();
    expect(inspector.textContent).toContain("component:C-110");
    expect(inspector.textContent).toContain("0.45 m");
    expect(inspector.textContent).toContain("1.5707963268 rad");
    expect(inspector.textContent).toContain("mechanics_geometry_only");
    expect(within(tree).getByTestId("tree-row-component:C-110")).toHaveClass(
      "active",
    );

    fireEvent.click(
      within(viewportSelection).getByTestId("viewport-select-component:C-130"),
    );
    expect(
      within(inspector).getByRole("heading", {
        name: "Invented semi-rigid valve marker",
      }),
    ).toBeInTheDocument();
    expect(inspector.textContent).toContain("component:C-130");
    expect(inspector.textContent).toContain("pipe:P-130");
    expect(inspector.textContent).toContain("15000000 N/m");
    expect(inspector.textContent).toContain("mechanics_geometry_only");
    expect(within(tree).getByTestId("tree-row-component:C-130")).toHaveClass(
      "active",
    );

    fireEvent.click(
      within(viewportSelection).getByTestId("viewport-select-component:C-140"),
    );
    expect(
      within(inspector).getByRole("heading", {
        name: "Invented tie-in marker",
      }),
    ).toBeInTheDocument();
    expect(inspector.textContent).toContain("component:C-140");
    expect(inspector.textContent).toContain("terminal");
    expect(within(tree).getByTestId("tree-row-component:C-140")).toHaveClass(
      "active",
    );

    fireEvent.click(
      within(viewportSelection).getByTestId("viewport-select-component:C-150"),
    );
    expect(
      within(inspector).getByRole("heading", {
        name: "Invented expansion joint marker",
      }),
    ).toBeInTheDocument();
    expect(inspector.textContent).toContain("component:C-150");
    expect(inspector.textContent).toContain("expansion_joint");
    expect(inspector.textContent).toContain("pipe:P-130");
    expect(inspector.textContent).toContain("3200000 N/m");
    expect(inspector.textContent).toContain(
      "mechanics_geometry_and_user_flexibility",
    );
    expect(within(tree).getByTestId("tree-row-component:C-150")).toHaveClass(
      "active",
    );

    fireEvent.click(
      within(viewportSelection).getByTestId("viewport-select-node:N-140"),
    );
    expect(
      within(inspector).getByRole("heading", { name: "Terminal tie-in" }),
    ).toBeInTheDocument();
    expect(inspector.textContent).toContain("node:N-140");
    expect(inspector.textContent).toContain("7.6, 2.4, 2.2 m");
    expect(within(tree).getByTestId("tree-row-node:N-140")).toHaveClass(
      "active",
    );
  });

  it("exposes materials, components, load cases, and combinations in the model workspace", async () => {
    render(<App />);

    const tree = await screen.findByLabelText("Model tree");
    expect(within(tree).getByText("Materials")).toBeInTheDocument();
    expect(within(tree).getByText("Components")).toBeInTheDocument();
    expect(within(tree).getByText("Load Cases")).toBeInTheDocument();
    expect(within(tree).getByText("Combinations")).toBeInTheDocument();
    expect(
      within(tree).getByTestId("model-tree-filter-summary").textContent,
    ).toContain("26 of 26 model entities visible");

    fireEvent.change(within(tree).getByTestId("model-tree-filter-input"), {
      target: { value: "component:C-110" },
    });
    expect(
      within(tree).getByTestId("model-tree-filter-summary").textContent,
    ).toContain("1 of 26 model entities visible");
    expect(
      within(tree).getByTestId("tree-row-component:C-110"),
    ).toBeInTheDocument();
    expect(within(tree).queryByText("Materials")).not.toBeInTheDocument();
    expect(within(tree).queryByText("Load Cases")).not.toBeInTheDocument();

    fireEvent.click(within(tree).getByTestId("tree-row-component:C-110"));
    const inspector = screen.getByLabelText("Property inspector");
    expect(
      within(inspector).getByRole("heading", { name: "Invented elbow marker" }),
    ).toBeInTheDocument();
    expect(inspector.textContent).toContain("0.45 m");
    expect(inspector.textContent).toContain("1.5707963268 rad");
    expect(inspector.textContent).toContain("global_xy_preview");

    fireEvent.change(within(tree).getByTestId("model-tree-filter-input"), {
      target: { value: "carbon-steel-like" },
    });
    expect(
      within(tree).getByTestId("model-tree-filter-summary").textContent,
    ).toContain("1 of 26 model entities visible");
    expect(
      within(tree).getByTestId("tree-row-material:invented-carbon-steel"),
    ).toBeInTheDocument();
    expect(
      within(tree).queryByTestId("tree-row-component:C-110"),
    ).not.toBeInTheDocument();
    expect(
      within(inspector).getByRole("heading", { name: "Invented elbow marker" }),
    ).toBeInTheDocument();

    fireEvent.click(
      within(tree).getByRole("button", {
        name: /Invented carbon-steel-like material/i,
      }),
    );
    expect(
      within(inspector).getByRole("heading", {
        name: "Invented carbon-steel-like material",
      }),
    ).toBeInTheDocument();
    expect(inspector.textContent).toContain("material:invented-carbon-steel");
    expect(inspector.textContent).toContain("200000000000 Pa");
    expect(inspector.textContent).toContain("77000000000 Pa");
    expect(inspector.textContent).toContain("0.000012 1/degC");
    expect(inspector.textContent).toContain(
      "invented_example_no_material_standard",
    );
    const intentPanel = within(inspector).getByLabelText(
      "Editor operation intent",
    );
    expect(
      within(intentPanel).getByTestId("editor-operation-preview").textContent,
    ).toContain("op:editor-intent-material:invented-carbon-steel-label");
    expect(
      within(intentPanel).getByTestId("editor-intent-audit-boundary")
        .textContent,
    ).toContain("no direct model mutation");
    expect(
      within(intentPanel).getByTestId("editor-intent-professional-boundary")
        .textContent,
    ).toContain("no compliance claim");
    expect(
      within(intentPanel).getByTestId("queue-editor-intent"),
    ).toBeDisabled();
    fireEvent.change(within(intentPanel).getByTestId("editor-intent-field"), {
      target: { value: "elastic_modulus.value" },
    });
    expect(within(intentPanel).getByTestId("editor-intent-unit")).toHaveValue(
      "Pa",
    );
    expect(
      await within(intentPanel).findByText(
        "Proposed value (Pa, model metadata)",
      ),
    ).toBeInTheDocument();
    fireEvent.change(within(intentPanel).getByTestId("editor-intent-value"), {
      target: { value: "210000000000" },
    });
    const expectedMaterialEditAfter = JSON.stringify({
      value: 210000000000,
      unit: "Pa",
    });
    expect(
      within(intentPanel).getByTestId("editor-operation-preview").textContent,
    ).toContain("Elastic modulus");
    expect(
      within(intentPanel).getByTestId("editor-operation-preview").textContent,
    ).toContain(`before=200000000000; after=${expectedMaterialEditAfter}`);
    expect(
      within(intentPanel).getByTestId("editor-intent-validation").textContent,
    ).toContain("not_applied");
    expect(
      within(intentPanel).getByTestId("editor-intent-validation").textContent,
    ).toContain(
      "model_metadata_unit_dimension_declared_catalog_unavailable_browser_preview",
    );
    fireEvent.click(
      within(intentPanel).getByTestId("validate-editor-intent-inline"),
    );
    await waitFor(() =>
      expect(
        within(intentPanel).getByTestId(
          "editor-intent-inline-validation-status",
        ).textContent,
      ).toContain("application status: not_applied"),
    );
    expect(
      within(intentPanel).getByTestId("editor-intent-inline-validation-status")
        .textContent,
    ).toContain("schema: passed");
    expect(
      within(intentPanel).getByTestId("editor-intent-inline-validation-status")
        .textContent,
    ).toContain("units: passed");
    expect(
      within(intentPanel).getByTestId("editor-intent-inline-validation-diff")
        .textContent,
    ).toContain(
      `material:invented-carbon-steel elastic_modulus.value 200000000000 to ${expectedMaterialEditAfter} [Pa]`,
    );
    expect(
      within(intentPanel).getByTestId(
        "editor-intent-inline-validation-boundary",
      ).textContent,
    ).toContain("no accepted model mutation");
    fireEvent.click(within(intentPanel).getByTestId("queue-editor-intent"));
    expect(
      within(intentPanel).getByTestId("editor-intent-queue").textContent,
    ).toContain("editor-intent-1");
    expect(
      within(intentPanel).getByTestId("editor-intent-queue").textContent,
    ).toContain("no accepted model change");
    expect(
      within(intentPanel).getByRole("button", { name: /Queue review intent/i }),
    ).not.toBeDisabled();
    expect(
      within(inspector).getByRole("heading", {
        name: "Invented carbon-steel-like material",
      }),
    ).toBeInTheDocument();

    fireEvent.change(within(tree).getByTestId("model-tree-filter-input"), {
      target: { value: "no-match-token" },
    });
    expect(
      within(tree).getByTestId("model-tree-filter-summary").textContent,
    ).toContain("0 of 26 model entities visible");
    expect(
      within(tree).getByTestId("model-tree-filter-empty").textContent,
    ).toContain("No model entities match this filter");
    expect(
      within(inspector).getByRole("heading", {
        name: "Invented carbon-steel-like material",
      }),
    ).toBeInTheDocument();

    fireEvent.click(within(tree).getByTestId("clear-model-tree-filter"));
    expect(
      within(tree).getByTestId("model-tree-filter-summary").textContent,
    ).toContain("26 of 26 model entities visible");
    expect(within(tree).getByText("Load Cases")).toBeInTheDocument();

    fireEvent.click(
      within(tree).getByRole("button", { name: /Invented elbow marker/i }),
    );
    expect(
      within(inspector).getByRole("heading", { name: "Invented elbow marker" }),
    ).toBeInTheDocument();
    expect(inspector.textContent).toContain("component:C-110");
    expect(inspector.textContent).toContain("bend");
    expect(inspector.textContent).toContain("node:N-110");
    expect(inspector.textContent).toContain("0.45 m");
    expect(inspector.textContent).toContain("1.5707963268 rad");
    expect(inspector.textContent).toContain("global_xy_preview");
    expect(inspector.textContent).toContain("mechanics_geometry_only");
    expect(inspector.textContent).toContain("user_rule_pack_inputs_only");
    expect(inspector.textContent).toContain("1.15 none");
    expect(inspector.textContent).toContain("1.08 none");
    expect(inspector.textContent).toContain(
      "BEND_GEOMETRY_INCOMPLETE:complete",
    );
    expect(inspector.textContent).toContain(
      "invented_example_user_entered_bend_values_no_code_table",
    );
    expect(inspector.textContent).toContain(
      "No missing selected fields detected",
    );

    fireEvent.click(
      within(tree).getByRole("button", {
        name: /Invented operating gravity and pressure preview/i,
      }),
    );
    expect(
      within(inspector).getByRole("heading", {
        name: "Invented operating gravity and pressure preview",
      }),
    ).toBeInTheDocument();
    expect(inspector.textContent).toContain("load:L-100");
    expect(inspector.textContent).toContain("primitive_user_load");
    expect(inspector.textContent).toContain("preview_only");
    expect(inspector.textContent).toContain(
      "weight, occasional, pressure, pressure, thermal",
    );
    expect(inspector.textContent).toContain("element:pipe:P-120");
    expect(inspector.textContent).toContain("node:node:N-140");

    fireEvent.click(
      within(tree).getByRole("button", {
        name: /Invented explicit operating plus alternate preview/i,
      }),
    );
    expect(
      within(inspector).getByRole("heading", {
        name: "Invented explicit operating plus alternate preview",
      }),
    ).toBeInTheDocument();
    expect(inspector.textContent).toContain("combination:C-OPER-ALT");
    expect(inspector.textContent).toContain("mechanics");
    expect(inspector.textContent).toContain("load:L-100 x 1");
    expect(inspector.textContent).toContain("load:L-200 x 0.5");
    expect(inspector.textContent).toContain(
      "invented_example_user_defined_mechanics_combination_no_code_default",
    );
  });

  it("queues layout-grid cell edits as structured review intents", async () => {
    render(<App />);

    const tree = await screen.findByLabelText("Model tree");
    fireEvent.click(within(tree).getByTestId("layout-mode-grid"));
    expect(within(tree).getByTestId("entity-grid")).toBeInTheDocument();
    expect(
      within(tree).getByTestId("entity-grid-table-nodes"),
    ).toBeInTheDocument();

    fireEvent.click(within(tree).getByTestId("entity-grid-row-node:N-100"));
    expect(screen.getByLabelText("Property inspector").textContent).toContain(
      "node:N-100",
    );

    fireEvent.change(
      within(tree).getByTestId("entity-grid-input-node:N-100-x"),
      {
        target: { value: "1.25" },
      },
    );
    fireEvent.change(
      within(tree).getByTestId("entity-grid-input-node:N-100-y"),
      {
        target: { value: "0.5" },
      },
    );
    expect(
      within(tree).getByTestId("entity-grid-change-count").textContent,
    ).toContain("2 changed cells");

    fireEvent.click(within(tree).getByTestId("queue-entity-grid-intents"));
    expect(
      within(tree).getByTestId("entity-grid-queued-message").textContent,
    ).toContain("Queued 2 review intents");

    const applyPanel = screen.getByTestId("operation-apply-panel");
    expect(
      within(applyPanel).getByTestId("operation-apply-row-editor-intent-1")
        .textContent,
    ).toContain("op:grid-intent-node:N-100-position-x");
    expect(
      within(applyPanel).getByTestId("operation-apply-row-editor-intent-1")
        .textContent,
    ).toContain('"unit":"m"');
    expect(
      within(applyPanel).getByTestId("operation-apply-row-editor-intent-2")
        .textContent,
    ).toContain("op:grid-intent-node:N-100-position-y");
    expect(
      within(tree).getByTestId("entity-grid-boundary").textContent,
    ).toContain("fans each changed cell into a structured review intent");
  });

  it("queues and applies a load-case primitive magnitude through the manager panel", async () => {
    render(<App />);

    const manager = await screen.findByTestId("load-case-manager");
    expect(
      within(manager).getByTestId("load-case-manager-summary").textContent,
    ).toContain("2 load cases; 9 primitive loads; 1 combinations");
    expect(
      within(manager).getByTestId("load-case-manager-boundary").textContent,
    ).toContain("structured operations");
    expect(
      within(manager).getByTestId(
        "load-manager-combination-combination:C-OPER-ALT",
      ).textContent,
    ).toContain("load:L-100 x 1");

    fireEvent.click(
      within(manager).getByTestId("load-manager-primitive-load:L-100-P"),
    );
    expect(
      within(manager).getByTestId("load-manager-selected-primitive")
        .textContent,
    ).toContain("load:L-100-P");
    expect(
      within(manager).getByTestId("load-manager-selected-primitive")
        .textContent,
    ).toContain("primitive_loads.2.magnitude.value");
    expect(
      within(manager).getByTestId("load-manager-edit-preview").textContent,
    ).toContain("current=1200000 Pa");
    expect(
      within(manager).getByTestId("load-manager-magnitude-unit"),
    ).toHaveValue("Pa");
    expect(
      await within(manager).findByText("Magnitude (Pa, model metadata)"),
    ).toBeInTheDocument();

    fireEvent.change(
      within(manager).getByTestId("load-manager-magnitude-value"),
      {
        target: { value: "1500000" },
      },
    );
    expect(
      within(manager).getByTestId("load-manager-edit-preview").textContent,
    ).toContain("op:load-manager-load:L-100-load:L-100-P-magnitude");
    expect(
      within(manager).getByTestId("load-manager-edit-preview").textContent,
    ).toContain('before=1200000; after={"value":1500000,"unit":"Pa"}');
    expect(
      within(manager).getByTestId("load-manager-edit-preview").textContent,
    ).toContain(
      "unit_validation=model_metadata_unit_dimension_declared_catalog_unavailable_browser_preview",
    );
    fireEvent.click(within(manager).getByTestId("queue-load-magnitude-intent"));

    const applyPanel = screen.getByTestId("operation-apply-panel");
    expect(
      within(applyPanel).getByTestId("operation-apply-summary").textContent,
    ).toContain("1 queued; 0 applied");
    expect(
      within(applyPanel).getByTestId("operation-unit-policy-chip").textContent,
    ).toContain("1 unit-bearing queued");
    expect(
      within(applyPanel).getByTestId("operation-unit-policy-chip").textContent,
    ).toContain("0 dimensionless queued");
    expect(
      within(applyPanel).getByTestId("operation-unit-policy-chip").textContent,
    ).toContain("no unit validations yet");
    expect(
      within(applyPanel).getByTestId("operation-unit-policy-chip").textContent,
    ).toContain("0 applied receipts");
    expect(
      within(applyPanel).getByTestId("operation-apply-row-editor-intent-1")
        .textContent,
    ).toContain("primitive_loads.2.magnitude.value");

    fireEvent.click(
      within(applyPanel).getByTestId("apply-intent-editor-intent-1"),
    );
    await waitFor(() =>
      expect(
        within(applyPanel).getByTestId("operation-apply-message").textContent,
      ).toContain("Applied op:load-manager-load:L-100-load:L-100-P-magnitude"),
    );

    expect(
      within(manager).getByTestId("load-manager-primitive-load:L-100-P")
        .textContent,
    ).toContain("pressure; 1500000 Pa");
    expect(
      within(applyPanel).getByTestId(
        "applied-operation-route-applied-1-editor-intent-1",
      ).textContent,
    ).toContain("persistence session_state_only_not_yet_saved");
    expect(
      within(applyPanel).getByTestId("operation-unit-policy-chip").textContent,
    ).toContain("0 unit-bearing queued");
    expect(
      within(applyPanel).getByTestId("operation-unit-policy-chip").textContent,
    ).toContain("passed");
    expect(
      within(applyPanel).getByTestId("operation-unit-policy-chip").textContent,
    ).toContain("1 applied receipts");
    expect(
      screen.getByTestId("local-project-review-context").textContent,
    ).toContain("0 pending operations");
    expect(
      screen.getByTestId("local-project-review-context").textContent,
    ).toContain("applied_operations=1");
    expect(screen.getByTestId("solve-job-summary").textContent).toContain(
      "state=not_started",
    );
  });

  it("queues and applies an empty load case through the manager panel", async () => {
    render(<App />);

    const manager = await screen.findByTestId("load-case-manager");
    expect(
      within(manager).getByTestId("load-manager-create-load-id"),
    ).toHaveValue("load:L-300");
    expect(
      within(manager).getByTestId("load-manager-create-load-preview")
        .textContent,
    ).toContain("op:load-manager-create-load:L-300");
    expect(
      within(manager).getByTestId("load-manager-create-load-preview")
        .textContent,
    ).toContain("primitive_loads=0");
    expect(
      within(manager).getByTestId("load-manager-create-load-preview")
        .textContent,
    ).toContain("unit_validation=not_required_dimensionless");
    fireEvent.change(
      within(manager).getByTestId("load-manager-create-load-label"),
      {
        target: { value: "User operating case" },
      },
    );
    fireEvent.click(
      within(manager).getByTestId("queue-create-load-case-intent"),
    );

    const applyPanel = screen.getByTestId("operation-apply-panel");
    expect(
      within(applyPanel).getByTestId("operation-apply-row-editor-intent-1")
        .textContent,
    ).toContain("load_cases");
    fireEvent.click(
      within(applyPanel).getByTestId("apply-intent-editor-intent-1"),
    );
    await waitFor(() =>
      expect(
        within(applyPanel).getByTestId("operation-apply-message").textContent,
      ).toContain("Applied op:load-manager-create-load:L-300"),
    );

    expect(
      within(manager).getByTestId("load-case-manager-summary").textContent,
    ).toContain("3 load cases; 9 primitive loads; 1 combinations");
    expect(
      within(manager).getByTestId("load-manager-case-load:L-300").textContent,
    ).toContain("load:L-300; primitive_user_load; draft; primitives=0");
    expect(screen.getByLabelText("Property inspector").textContent).toContain(
      "User operating case",
    );
    expect(screen.getByLabelText("Property inspector").textContent).toContain(
      "load:L-300",
    );
    expect(
      within(applyPanel).getByTestId(
        "applied-operation-route-applied-1-editor-intent-1",
      ).textContent,
    ).toContain("persistence session_state_only_not_yet_saved");
    expect(
      screen.getByTestId("local-project-review-context").textContent,
    ).toContain("0 pending operations");
    expect(
      screen.getByTestId("local-project-review-context").textContent,
    ).toContain("applied_operations=1");
    expect(screen.getByTestId("solve-job-summary").textContent).toContain(
      "state=not_started",
    );
  });

  it("queues and applies an explicit load-case deletion through the manager panel", async () => {
    render(<App />);

    const manager = await screen.findByTestId("load-case-manager");
    fireEvent.change(
      within(manager).getByTestId("load-manager-create-load-label"),
      {
        target: { value: "Temporary user case" },
      },
    );
    fireEvent.click(
      within(manager).getByTestId("queue-create-load-case-intent"),
    );

    const applyPanel = screen.getByTestId("operation-apply-panel");
    fireEvent.click(
      within(applyPanel).getByTestId("apply-intent-editor-intent-1"),
    );
    await waitFor(() =>
      expect(
        within(applyPanel).getByTestId("operation-apply-message").textContent,
      ).toContain("Applied op:load-manager-create-load:L-300"),
    );

    fireEvent.click(
      within(manager).getByTestId("load-manager-case-load:L-300"),
    );
    expect(
      within(manager).getByTestId("load-manager-load-case-delete-preview")
        .textContent,
    ).toContain("op:load-manager-load:L-300-delete");
    expect(
      within(manager).getByTestId("load-manager-load-case-delete-preview")
        .textContent,
    ).toContain(
      "before=load:L-300; Temporary user case; primitive_user_load; draft; primitives=0; after=not_present; unit=none; dimensionless",
    );
    expect(
      within(manager).getByTestId("load-manager-load-case-delete-preview")
        .textContent,
    ).toContain("unit_validation=not_required_dimensionless");
    fireEvent.click(
      within(manager).getByTestId("queue-delete-load-case-intent"),
    );

    expect(
      within(applyPanel).getByTestId("operation-apply-row-editor-intent-2")
        .textContent,
    ).toContain("load_cases");
    fireEvent.click(
      within(applyPanel).getByTestId("apply-intent-editor-intent-2"),
    );
    await waitFor(() =>
      expect(
        within(applyPanel).getByTestId("operation-apply-message").textContent,
      ).toContain("Applied op:load-manager-load:L-300-delete"),
    );

    expect(screen.queryByTestId("load-manager-case-load:L-300")).toBeNull();
    expect(
      within(manager).getByTestId("load-case-manager-summary").textContent,
    ).toContain("2 load cases; 9 primitive loads; 1 combinations");
    expect(
      within(applyPanel).getByTestId(
        "applied-operation-route-applied-2-editor-intent-2",
      ).textContent,
    ).toContain("persistence session_state_only_not_yet_saved");
    expect(
      screen.getByTestId("local-project-review-context").textContent,
    ).toContain("0 pending operations");
    expect(
      screen.getByTestId("local-project-review-context").textContent,
    ).toContain("applied_operations=2");
    expect(screen.getByTestId("solve-job-summary").textContent).toContain(
      "state=not_started",
    );
  });

  it("blocks load-case deletion while a combination term still references it", async () => {
    render(<App />);

    const manager = await screen.findByTestId("load-case-manager");
    fireEvent.click(
      within(manager).getByTestId("load-manager-case-load:L-100"),
    );
    expect(
      within(manager).getByTestId("load-manager-load-case-delete-preview")
        .textContent,
    ).toContain("op:load-manager-load:L-100-delete");
    expect(
      within(manager).getByTestId("load-manager-load-case-delete-preview")
        .textContent,
    ).toContain(
      "before=load:L-100; Invented operating gravity and pressure preview; primitive_user_load; preview_only; primitives=5",
    );
    fireEvent.click(
      within(manager).getByTestId("queue-delete-load-case-intent"),
    );

    const applyPanel = screen.getByTestId("operation-apply-panel");
    fireEvent.click(
      within(applyPanel).getByTestId("apply-intent-editor-intent-1"),
    );
    await waitFor(() =>
      expect(
        within(applyPanel).getByTestId("operation-apply-message").textContent,
      ).toContain(
        "Operation op:load-manager-load:L-100-delete was not applied (blocked); see its diagnostics.",
      ),
    );

    expect(
      within(applyPanel).getByTestId(
        "operation-outcome-diagnostic-OP-LOAD-CASE-DELETE-REFERENCED",
      ).textContent,
    ).toContain("combination:C-OPER-ALT.terms.0");
    expect(
      within(manager).getByTestId("load-manager-case-load:L-100").textContent,
    ).toContain("load:L-100; primitive_user_load; preview_only; primitives=5");
    expect(
      screen.getByTestId("local-project-review-context").textContent,
    ).toContain("1 pending operation");
    expect(
      screen.getByTestId("local-project-review-context").textContent,
    ).toContain("applied_operations=0");
  });

  it("queues and applies a concentrated-force primitive load through the manager panel", async () => {
    render(<App />);

    const manager = await screen.findByTestId("load-case-manager");
    expect(
      within(manager).getByTestId("load-manager-create-primitive-id"),
    ).toHaveValue("load:L-100-F300");
    expect(
      within(manager).getByTestId("load-manager-create-primitive-load-case"),
    ).toHaveValue("load:L-100");
    expect(
      within(manager).getByTestId("load-manager-create-primitive-node"),
    ).toHaveValue("node:N-100");
    expect(
      within(manager).getByTestId("load-manager-create-primitive-preview")
        .textContent,
    ).toContain("op:load-manager-load:L-100-load:L-100-F300-primitive");
    expect(
      within(manager).getByTestId("load-manager-create-primitive-preview")
        .textContent,
    ).toContain("target=node:N-100; direction=global_y; unit=N; force");
    await waitFor(() =>
      expect(
        within(manager).getByTestId("load-manager-create-primitive-preview")
          .textContent,
      ).toContain(
        "unit_validation=model_metadata_unit_dimension_declared_catalog_unavailable_browser_preview",
      ),
    );

    fireEvent.change(
      within(manager).getByTestId("load-manager-create-primitive-magnitude"),
      {
        target: { value: "275" },
      },
    );
    fireEvent.click(
      within(manager).getByTestId("queue-create-primitive-intent"),
    );

    const applyPanel = screen.getByTestId("operation-apply-panel");
    expect(
      within(applyPanel).getByTestId("operation-apply-row-editor-intent-1")
        .textContent,
    ).toContain("primitive_loads");
    fireEvent.click(
      within(applyPanel).getByTestId("apply-intent-editor-intent-1"),
    );
    await waitFor(() =>
      expect(
        within(applyPanel).getByTestId("operation-apply-message").textContent,
      ).toContain(
        "Applied op:load-manager-load:L-100-load:L-100-F300-primitive",
      ),
    );

    expect(
      within(manager).getByTestId("load-case-manager-summary").textContent,
    ).toContain("2 load cases; 10 primitive loads; 1 combinations");
    expect(
      within(manager).getByTestId("load-manager-case-load:L-100").textContent,
    ).toContain("load:L-100; primitive_user_load; preview_only; primitives=6");
    expect(
      within(manager).getByTestId("load-manager-primitive-load:L-100-F300")
        .textContent,
    ).toContain("concentrated_force; 275 N");
    expect(
      within(manager).getByTestId("load-manager-primitive-load:L-100-F300")
        .textContent,
    ).toContain("load:L-100-F300; node:node:N-100; global_y; dimension=force");
    expect(
      within(applyPanel).getByTestId(
        "applied-operation-route-applied-1-editor-intent-1",
      ).textContent,
    ).toContain("persistence session_state_only_not_yet_saved");
    expect(
      screen.getByTestId("local-project-review-context").textContent,
    ).toContain("0 pending operations");
    expect(
      screen.getByTestId("local-project-review-context").textContent,
    ).toContain("applied_operations=1");
    expect(screen.getByTestId("solve-job-summary").textContent).toContain(
      "state=not_started",
    );
  });

  it("queues and applies a distributed-force primitive load through the manager panel", async () => {
    render(<App />);

    const manager = await screen.findByTestId("load-case-manager");
    fireEvent.change(
      within(manager).getByTestId("load-manager-create-primitive-category"),
      {
        target: { value: "distributed_force" },
      },
    );
    expect(
      within(manager).getByTestId("load-manager-create-primitive-id"),
    ).toHaveValue("load:L-100-D300");
    expect(
      within(manager).getByTestId("load-manager-create-primitive-load-case"),
    ).toHaveValue("load:L-100");
    expect(
      within(manager).getByTestId("load-manager-create-primitive-pipe"),
    ).toHaveValue("pipe:P-100");
    expect(
      within(manager).getByTestId("load-manager-create-primitive-preview")
        .textContent,
    ).toContain("op:load-manager-load:L-100-load:L-100-D300-primitive");
    expect(
      within(manager).getByTestId("load-manager-create-primitive-preview")
        .textContent,
    ).toContain(
      "target=pipe:P-100; direction=global_y; unit=N/m; force_per_length",
    );

    fireEvent.change(
      within(manager).getByTestId("load-manager-create-primitive-magnitude"),
      {
        target: { value: "120" },
      },
    );
    fireEvent.click(
      within(manager).getByTestId("queue-create-primitive-intent"),
    );

    const applyPanel = screen.getByTestId("operation-apply-panel");
    expect(
      within(applyPanel).getByTestId("operation-apply-row-editor-intent-1")
        .textContent,
    ).toContain("primitive_loads");
    fireEvent.click(
      within(applyPanel).getByTestId("apply-intent-editor-intent-1"),
    );
    await waitFor(() =>
      expect(
        within(applyPanel).getByTestId("operation-apply-message").textContent,
      ).toContain(
        "Applied op:load-manager-load:L-100-load:L-100-D300-primitive",
      ),
    );

    expect(
      within(manager).getByTestId("load-case-manager-summary").textContent,
    ).toContain("2 load cases; 10 primitive loads; 1 combinations");
    expect(
      within(manager).getByTestId("load-manager-case-load:L-100").textContent,
    ).toContain("load:L-100; primitive_user_load; preview_only; primitives=6");
    expect(
      within(manager).getByTestId("load-manager-primitive-load:L-100-D300")
        .textContent,
    ).toContain("distributed_force; 120 N/m");
    expect(
      within(manager).getByTestId("load-manager-primitive-load:L-100-D300")
        .textContent,
    ).toContain(
      "load:L-100-D300; element:pipe:P-100; global_y; dimension=force_per_length",
    );
    expect(
      within(applyPanel).getByTestId(
        "applied-operation-route-applied-1-editor-intent-1",
      ).textContent,
    ).toContain("persistence session_state_only_not_yet_saved");
    expect(
      screen.getByTestId("local-project-review-context").textContent,
    ).toContain("0 pending operations");
    expect(
      screen.getByTestId("local-project-review-context").textContent,
    ).toContain("applied_operations=1");
    expect(screen.getByTestId("solve-job-summary").textContent).toContain(
      "state=not_started",
    );
  });

  it("queues and applies a concentrated-moment primitive load through the manager panel", async () => {
    render(<App />);

    const manager = await screen.findByTestId("load-case-manager");
    fireEvent.change(
      within(manager).getByTestId("load-manager-create-primitive-category"),
      {
        target: { value: "concentrated_moment" },
      },
    );
    expect(
      within(manager).getByTestId("load-manager-create-primitive-id"),
    ).toHaveValue("load:L-100-M300");
    expect(
      within(manager).getByTestId("load-manager-create-primitive-load-case"),
    ).toHaveValue("load:L-100");
    expect(
      within(manager).getByTestId("load-manager-create-primitive-node"),
    ).toHaveValue("node:N-100");
    expect(
      within(manager).getByTestId("load-manager-create-primitive-direction"),
    ).toHaveValue("rotation_z");
    expect(
      within(manager).getByTestId("load-manager-create-primitive-preview")
        .textContent,
    ).toContain("op:load-manager-load:L-100-load:L-100-M300-primitive");
    expect(
      within(manager).getByTestId("load-manager-create-primitive-preview")
        .textContent,
    ).toContain("target=node:N-100; direction=rotation_z; unit=N*m; moment");

    fireEvent.change(
      within(manager).getByTestId("load-manager-create-primitive-magnitude"),
      {
        target: { value: "80" },
      },
    );
    fireEvent.click(
      within(manager).getByTestId("queue-create-primitive-intent"),
    );

    const applyPanel = screen.getByTestId("operation-apply-panel");
    expect(
      within(applyPanel).getByTestId("operation-apply-row-editor-intent-1")
        .textContent,
    ).toContain("primitive_loads");
    fireEvent.click(
      within(applyPanel).getByTestId("apply-intent-editor-intent-1"),
    );
    await waitFor(() =>
      expect(
        within(applyPanel).getByTestId("operation-apply-message").textContent,
      ).toContain(
        "Applied op:load-manager-load:L-100-load:L-100-M300-primitive",
      ),
    );

    expect(
      within(manager).getByTestId("load-case-manager-summary").textContent,
    ).toContain("2 load cases; 10 primitive loads; 1 combinations");
    expect(
      within(manager).getByTestId("load-manager-case-load:L-100").textContent,
    ).toContain("load:L-100; primitive_user_load; preview_only; primitives=6");
    expect(
      within(manager).getByTestId("load-manager-primitive-load:L-100-M300")
        .textContent,
    ).toContain("concentrated_moment; 80 N*m");
    expect(
      within(manager).getByTestId("load-manager-primitive-load:L-100-M300")
        .textContent,
    ).toContain(
      "load:L-100-M300; node:node:N-100; rotation_z; dimension=moment",
    );
    expect(
      within(applyPanel).getByTestId(
        "applied-operation-route-applied-1-editor-intent-1",
      ).textContent,
    ).toContain("persistence session_state_only_not_yet_saved");
    expect(
      screen.getByTestId("local-project-review-context").textContent,
    ).toContain("0 pending operations");
    expect(
      screen.getByTestId("local-project-review-context").textContent,
    ).toContain("applied_operations=1");
    expect(screen.getByTestId("solve-job-summary").textContent).toContain(
      "state=not_started",
    );
  });

  it("queues and applies a pressure primitive load through the manager panel", async () => {
    render(<App />);

    const manager = await screen.findByTestId("load-case-manager");
    fireEvent.change(
      within(manager).getByTestId("load-manager-create-primitive-category"),
      {
        target: { value: "pressure" },
      },
    );
    expect(
      within(manager).getByTestId("load-manager-create-primitive-id"),
    ).toHaveValue("load:L-100-P300");
    expect(
      within(manager).getByTestId("load-manager-create-primitive-load-case"),
    ).toHaveValue("load:L-100");
    expect(
      within(manager).getByTestId("load-manager-create-primitive-pipe"),
    ).toHaveValue("pipe:P-100");
    expect(
      within(manager).getByTestId("load-manager-create-primitive-direction"),
    ).toHaveValue("global_x");
    expect(
      within(manager).getByTestId("load-manager-create-primitive-unit"),
    ).toHaveValue("Pa");
    expect(
      await within(manager).findByText("Magnitude (Pa, model metadata)"),
    ).toBeInTheDocument();
    expect(
      within(manager).getByTestId("load-manager-create-primitive-preview")
        .textContent,
    ).toContain("op:load-manager-load:L-100-load:L-100-P300-primitive");
    expect(
      within(manager).getByTestId("load-manager-create-primitive-preview")
        .textContent,
    ).toContain("target=pipe:P-100; direction=global_x; unit=Pa; pressure");

    fireEvent.change(
      within(manager).getByTestId("load-manager-create-primitive-magnitude"),
      {
        target: { value: "1200000" },
      },
    );
    fireEvent.click(
      within(manager).getByTestId("queue-create-primitive-intent"),
    );

    const applyPanel = screen.getByTestId("operation-apply-panel");
    expect(
      within(applyPanel).getByTestId("operation-apply-row-editor-intent-1")
        .textContent,
    ).toContain("primitive_loads");
    fireEvent.click(
      within(applyPanel).getByTestId("apply-intent-editor-intent-1"),
    );
    await waitFor(() =>
      expect(
        within(applyPanel).getByTestId("operation-apply-message").textContent,
      ).toContain(
        "Applied op:load-manager-load:L-100-load:L-100-P300-primitive",
      ),
    );

    expect(
      within(manager).getByTestId("load-case-manager-summary").textContent,
    ).toContain("2 load cases; 10 primitive loads; 1 combinations");
    expect(
      within(manager).getByTestId("load-manager-case-load:L-100").textContent,
    ).toContain("load:L-100; primitive_user_load; preview_only; primitives=6");
    expect(
      within(manager).getByTestId("load-manager-primitive-load:L-100-P300")
        .textContent,
    ).toContain("pressure; 1200000 Pa");
    expect(
      within(manager).getByTestId("load-manager-primitive-load:L-100-P300")
        .textContent,
    ).toContain(
      "load:L-100-P300; element:pipe:P-100; global_x; dimension=pressure",
    );
    expect(
      within(applyPanel).getByTestId(
        "applied-operation-route-applied-1-editor-intent-1",
      ).textContent,
    ).toContain("persistence session_state_only_not_yet_saved");
    expect(
      screen.getByTestId("local-project-review-context").textContent,
    ).toContain("0 pending operations");
    expect(
      screen.getByTestId("local-project-review-context").textContent,
    ).toContain("applied_operations=1");
    expect(screen.getByTestId("solve-job-summary").textContent).toContain(
      "state=not_started",
    );
  });

  it("queues and applies a thermal primitive load through the manager panel", async () => {
    render(<App />);

    const manager = await screen.findByTestId("load-case-manager");
    fireEvent.change(
      within(manager).getByTestId("load-manager-create-primitive-category"),
      {
        target: { value: "thermal" },
      },
    );
    expect(
      within(manager).getByTestId("load-manager-create-primitive-id"),
    ).toHaveValue("load:L-100-T300");
    expect(
      within(manager).getByTestId("load-manager-create-primitive-load-case"),
    ).toHaveValue("load:L-100");
    expect(
      within(manager).getByTestId("load-manager-create-primitive-pipe"),
    ).toHaveValue("pipe:P-100");
    expect(
      within(manager).getByTestId("load-manager-create-primitive-direction"),
    ).toHaveValue("global_z");
    expect(
      within(manager).getByTestId("load-manager-create-primitive-unit"),
    ).toHaveValue("degC");
    expect(
      await within(manager).findByText("Magnitude (degC, model metadata)"),
    ).toBeInTheDocument();
    expect(
      within(manager).getByTestId("load-manager-create-primitive-preview")
        .textContent,
    ).toContain("op:load-manager-load:L-100-load:L-100-T300-primitive");
    expect(
      within(manager).getByTestId("load-manager-create-primitive-preview")
        .textContent,
    ).toContain(
      "target=pipe:P-100; direction=global_z; unit=degC; temperature_interval",
    );

    fireEvent.change(
      within(manager).getByTestId("load-manager-create-primitive-magnitude"),
      {
        target: { value: "12.5" },
      },
    );
    fireEvent.click(
      within(manager).getByTestId("queue-create-primitive-intent"),
    );

    const applyPanel = screen.getByTestId("operation-apply-panel");
    expect(
      within(applyPanel).getByTestId("operation-apply-row-editor-intent-1")
        .textContent,
    ).toContain("primitive_loads");
    fireEvent.click(
      within(applyPanel).getByTestId("apply-intent-editor-intent-1"),
    );
    await waitFor(() =>
      expect(
        within(applyPanel).getByTestId("operation-apply-message").textContent,
      ).toContain(
        "Applied op:load-manager-load:L-100-load:L-100-T300-primitive",
      ),
    );

    expect(
      within(manager).getByTestId("load-case-manager-summary").textContent,
    ).toContain("2 load cases; 10 primitive loads; 1 combinations");
    expect(
      within(manager).getByTestId("load-manager-case-load:L-100").textContent,
    ).toContain("load:L-100; primitive_user_load; preview_only; primitives=6");
    expect(
      within(manager).getByTestId("load-manager-primitive-load:L-100-T300")
        .textContent,
    ).toContain("thermal; 12.5 degC");
    expect(
      within(manager).getByTestId("load-manager-primitive-load:L-100-T300")
        .textContent,
    ).toContain(
      "load:L-100-T300; element:pipe:P-100; global_z; dimension=temperature_interval",
    );
    expect(
      within(applyPanel).getByTestId(
        "applied-operation-route-applied-1-editor-intent-1",
      ).textContent,
    ).toContain("persistence session_state_only_not_yet_saved");
    expect(
      screen.getByTestId("local-project-review-context").textContent,
    ).toContain("0 pending operations");
    expect(
      screen.getByTestId("local-project-review-context").textContent,
    ).toContain("applied_operations=1");
    expect(screen.getByTestId("solve-job-summary").textContent).toContain(
      "state=not_started",
    );
  });

  it("queues and applies an imposed-displacement primitive load through the manager panel", async () => {
    render(<App />);

    const manager = await screen.findByTestId("load-case-manager");
    fireEvent.change(
      within(manager).getByTestId("load-manager-create-primitive-category"),
      {
        target: { value: "imposed_displacement" },
      },
    );
    expect(
      within(manager).getByTestId("load-manager-create-primitive-id"),
    ).toHaveValue("load:L-100-I300");
    expect(
      within(manager).getByTestId("load-manager-create-primitive-load-case"),
    ).toHaveValue("load:L-100");
    expect(
      within(manager).getByTestId("load-manager-create-primitive-support"),
    ).toHaveValue("support:S-100");
    expect(
      within(manager).getByTestId("load-manager-create-primitive-direction"),
    ).toHaveValue("UZ");
    expect(
      within(manager).getByTestId("load-manager-create-primitive-preview")
        .textContent,
    ).toContain("op:load-manager-load:L-100-load:L-100-I300-primitive");
    expect(
      within(manager).getByTestId("load-manager-create-primitive-preview")
        .textContent,
    ).toContain("target=support:S-100; direction=UZ; unit=m; displacement");

    fireEvent.change(
      within(manager).getByTestId("load-manager-create-primitive-magnitude"),
      {
        target: { value: "-0.006" },
      },
    );
    fireEvent.click(
      within(manager).getByTestId("queue-create-primitive-intent"),
    );

    const applyPanel = screen.getByTestId("operation-apply-panel");
    expect(
      within(applyPanel).getByTestId("operation-apply-row-editor-intent-1")
        .textContent,
    ).toContain("primitive_loads");
    fireEvent.click(
      within(applyPanel).getByTestId("apply-intent-editor-intent-1"),
    );
    await waitFor(() =>
      expect(
        within(applyPanel).getByTestId("operation-apply-message").textContent,
      ).toContain(
        "Applied op:load-manager-load:L-100-load:L-100-I300-primitive",
      ),
    );

    expect(
      within(manager).getByTestId("load-case-manager-summary").textContent,
    ).toContain("2 load cases; 10 primitive loads; 1 combinations");
    expect(
      within(manager).getByTestId("load-manager-case-load:L-100").textContent,
    ).toContain("load:L-100; primitive_user_load; preview_only; primitives=6");
    expect(
      within(manager).getByTestId("load-manager-primitive-load:L-100-I300")
        .textContent,
    ).toContain("imposed_displacement; -0.006 m");
    expect(
      within(manager).getByTestId("load-manager-primitive-load:L-100-I300")
        .textContent,
    ).toContain(
      "load:L-100-I300; support:support:S-100; UZ; dimension=displacement",
    );
    expect(
      within(applyPanel).getByTestId(
        "applied-operation-route-applied-1-editor-intent-1",
      ).textContent,
    ).toContain("persistence session_state_only_not_yet_saved");
    expect(
      screen.getByTestId("local-project-review-context").textContent,
    ).toContain("0 pending operations");
    expect(
      screen.getByTestId("local-project-review-context").textContent,
    ).toContain("applied_operations=1");
    expect(screen.getByTestId("solve-job-summary").textContent).toContain(
      "state=not_started",
    );
  });

  it("queues and applies an explicit primitive load deletion through the manager panel", async () => {
    render(<App />);

    const manager = await screen.findByTestId("load-case-manager");
    const primitiveRow = within(manager).getByTestId(
      "load-manager-primitive-load:L-100-Y",
    );
    expect(primitiveRow.textContent).toContain("occasional; 350 N");
    fireEvent.click(primitiveRow);
    expect(
      await within(manager).findAllByText("Magnitude (N, model metadata)"),
    ).not.toHaveLength(0);

    expect(
      within(manager).getByTestId("load-manager-primitive-delete-preview")
        .textContent,
    ).toContain("op:load-manager-load:L-100-load:L-100-Y-delete");
    expect(
      within(manager).getByTestId("load-manager-primitive-delete-preview")
        .textContent,
    ).toContain(
      "before=load:L-100-Y; occasional; node:node:N-140; global_y; 350 N; force",
    );
    expect(
      within(manager).getByTestId("load-manager-primitive-delete-preview")
        .textContent,
    ).toContain(
      "unit_validation=model_metadata_unit_dimension_declared_catalog_unavailable_browser_preview",
    );
    fireEvent.click(
      within(manager).getByTestId("queue-delete-primitive-load-intent"),
    );

    const applyPanel = screen.getByTestId("operation-apply-panel");
    expect(
      within(applyPanel).getByTestId("operation-apply-row-editor-intent-1")
        .textContent,
    ).toContain("primitive_loads.1");
    expect(
      within(applyPanel).getByTestId("operation-apply-row-editor-intent-1")
        .textContent,
    ).toContain("not_present");

    fireEvent.click(
      within(applyPanel).getByTestId("apply-intent-editor-intent-1"),
    );
    await waitFor(() =>
      expect(
        within(applyPanel).getByTestId("operation-apply-message").textContent,
      ).toContain("Applied op:load-manager-load:L-100-load:L-100-Y-delete"),
    );

    expect(
      screen.queryByTestId("load-manager-primitive-load:L-100-Y"),
    ).toBeNull();
    expect(
      within(manager).getByTestId("load-case-manager-summary").textContent,
    ).toContain("2 load cases; 8 primitive loads; 1 combinations");
    expect(
      within(manager).getByTestId("load-manager-case-load:L-100").textContent,
    ).toContain("load:L-100; primitive_user_load; preview_only; primitives=4");
    expect(
      screen.getByTestId("local-project-review-context").textContent,
    ).toContain("0 pending operations");
    expect(
      screen.getByTestId("local-project-review-context").textContent,
    ).toContain("applied_operations=1");
    expect(screen.getByTestId("solve-job-summary").textContent).toContain(
      "state=not_started",
    );

    const receipt = within(applyPanel).getByTestId(
      "applied-operation-route-applied-1-editor-intent-1",
    );
    expect(receipt.textContent).toContain(
      "Acceptance basis user_initiated_apply_in_local_session",
    );
    expect(receipt.textContent).toContain(
      "persistence session_state_only_not_yet_saved",
    );
    expect(receipt.textContent).toContain("professional approval not recorded");
  });

  it("queues and applies load-case status metadata through the manager panel", async () => {
    render(<App />);

    const manager = await screen.findByTestId("load-case-manager");
    fireEvent.click(
      within(manager).getByTestId("load-manager-case-load:L-100"),
    );

    const queueButton = within(manager).getByTestId(
      "queue-load-metadata-intent",
    );
    expect(
      within(manager).getByTestId("load-manager-selected-case").textContent,
    ).toContain("load:L-100");
    expect(
      within(manager).getByTestId("load-manager-selected-case").textContent,
    ).toContain("field=status");
    expect(
      within(manager).getByTestId("load-manager-metadata-preview").textContent,
    ).toContain("current=preview_only");
    expect(queueButton).toBeDisabled();

    fireEvent.change(
      within(manager).getByTestId("load-manager-metadata-value"),
      {
        target: { value: "TBD" },
      },
    );
    expect(
      within(manager).getByTestId("load-manager-metadata-preview").textContent,
    ).toContain("op:load-manager-load:L-100-status");
    expect(
      within(manager).getByTestId("load-manager-metadata-preview").textContent,
    ).toContain("before=preview_only; after=TBD");
    expect(
      within(manager).getByTestId("load-manager-metadata-preview").textContent,
    ).toContain("unit_validation=not_required_dimensionless");
    expect(queueButton).not.toBeDisabled();
    fireEvent.click(queueButton);

    const applyPanel = screen.getByTestId("operation-apply-panel");
    expect(
      within(applyPanel).getByTestId("operation-apply-row-editor-intent-1")
        .textContent,
    ).toContain("status");
    fireEvent.click(
      within(applyPanel).getByTestId("apply-intent-editor-intent-1"),
    );
    await waitFor(() =>
      expect(
        within(applyPanel).getByTestId("operation-apply-message").textContent,
      ).toContain("Applied op:load-manager-load:L-100-status"),
    );

    expect(
      within(manager).getByTestId("load-manager-case-load:L-100").textContent,
    ).toContain("load:L-100; primitive_user_load; TBD; primitives=5");
    expect(screen.getByLabelText("Property inspector").textContent).toContain(
      "TBD",
    );
    expect(
      screen.getByTestId("local-project-review-context").textContent,
    ).toContain("0 pending operations");
    expect(
      screen.getByTestId("local-project-review-context").textContent,
    ).toContain("applied_operations=1");
    expect(screen.getByTestId("solve-job-summary").textContent).toContain(
      "state=not_started",
    );

    fireEvent.change(
      within(manager).getByTestId("load-manager-metadata-field"),
      {
        target: { value: "kind" },
      },
    );
    await waitFor(() =>
      expect(
        within(manager).getByTestId("load-manager-selected-case").textContent,
      ).toContain("field=kind; current=primitive_user_load"),
    );
    fireEvent.change(
      within(manager).getByTestId("load-manager-metadata-value"),
      {
        target: { value: "TBD" },
      },
    );
    expect(
      within(manager).getByTestId("load-manager-metadata-preview").textContent,
    ).toContain("op:load-manager-load:L-100-kind");
    expect(
      within(manager).getByTestId("load-manager-metadata-preview").textContent,
    ).toContain("before=primitive_user_load; after=TBD");
    expect(
      within(manager).getByTestId("queue-load-metadata-intent"),
    ).not.toBeDisabled();
  });

  it("queues and applies an existing combination term factor through the manager panel", async () => {
    render(<App />);

    const manager = await screen.findByTestId("load-case-manager");
    const termRow = within(manager).getByTestId(
      "load-manager-combination-term-combination:C-OPER-ALT-1",
    );
    expect(termRow.textContent).toContain("load:L-200 x 0.5");
    fireEvent.click(termRow);

    const queueButton = within(manager).getByTestId(
      "queue-combination-factor-intent",
    );
    expect(
      within(manager).getByTestId("load-manager-selected-combination-term")
        .textContent,
    ).toContain("combination:C-OPER-ALT");
    expect(
      within(manager).getByTestId("load-manager-selected-combination-term")
        .textContent,
    ).toContain("terms.1.factor");
    expect(
      within(manager).getByTestId("load-manager-combination-factor-preview")
        .textContent,
    ).toContain("current=0.5");
    expect(queueButton).toBeDisabled();

    fireEvent.change(
      within(manager).getByTestId("load-manager-combination-factor-value"),
      {
        target: { value: "0.75" },
      },
    );
    expect(
      within(manager).getByTestId("load-manager-combination-factor-preview")
        .textContent,
    ).toContain("op:load-manager-combination:C-OPER-ALT-term-1-factor");
    expect(
      within(manager).getByTestId("load-manager-combination-factor-preview")
        .textContent,
    ).toContain("before=0.5; after=0.75");
    expect(
      within(manager).getByTestId("load-manager-combination-factor-preview")
        .textContent,
    ).toContain("unit_validation=not_required_dimensionless");
    expect(queueButton).not.toBeDisabled();
    fireEvent.click(queueButton);

    const applyPanel = screen.getByTestId("operation-apply-panel");
    expect(
      within(applyPanel).getByTestId("operation-apply-row-editor-intent-1")
        .textContent,
    ).toContain("terms.1.factor");
    fireEvent.click(
      within(applyPanel).getByTestId("apply-intent-editor-intent-1"),
    );
    await waitFor(() =>
      expect(
        within(applyPanel).getByTestId("operation-apply-message").textContent,
      ).toContain(
        "Applied op:load-manager-combination:C-OPER-ALT-term-1-factor",
      ),
    );

    expect(
      within(manager).getByTestId(
        "load-manager-combination-combination:C-OPER-ALT",
      ).textContent,
    ).toContain("load:L-200 x 0.75");
    expect(screen.getByLabelText("Property inspector").textContent).toContain(
      "load:L-200 x 0.75",
    );
    expect(
      screen.getByTestId("local-project-review-context").textContent,
    ).toContain("0 pending operations");
    expect(
      screen.getByTestId("local-project-review-context").textContent,
    ).toContain("applied_operations=1");
    expect(screen.getByTestId("solve-job-summary").textContent).toContain(
      "state=not_started",
    );
  });

  it("queues a closed-set combination basis edit and surfaces the honest engine block for cross-shape changes", async () => {
    // Behavior change (TP-APP-R2-COMBEXPR-001): basis edits validate the
    // closed set {mechanics, result_state_subtraction, range_envelope} and
    // block cross-shape changes instead of accepting free text.
    render(<App />);

    const manager = await screen.findByTestId("load-case-manager");
    fireEvent.click(
      within(manager).getByTestId(
        "load-manager-combination-select-combination:C-OPER-ALT",
      ),
    );

    const queueButton = within(manager).getByTestId(
      "queue-combination-basis-intent",
    );
    expect(
      within(manager).getByTestId("load-manager-selected-combination")
        .textContent,
    ).toContain("combination:C-OPER-ALT");
    expect(
      within(manager).getByTestId("load-manager-selected-combination")
        .textContent,
    ).toContain("field=basis; current=mechanics");
    expect(
      within(manager).getByTestId("load-manager-combination-basis-preview")
        .textContent,
    ).toContain("current=mechanics");
    expect(queueButton).toBeDisabled();
    const basisSelect = within(manager).getByTestId(
      "load-manager-combination-basis-value",
    ) as HTMLSelectElement;
    expect(Array.from(basisSelect.options, (option) => option.value)).toEqual([
      "mechanics",
      "result_state_subtraction",
      "range_envelope",
    ]);

    fireEvent.change(basisSelect, {
      target: { value: "result_state_subtraction" },
    });
    expect(
      within(manager).getByTestId("load-manager-combination-basis-preview")
        .textContent,
    ).toContain("op:load-manager-combination:C-OPER-ALT-basis");
    expect(
      within(manager).getByTestId("load-manager-combination-basis-preview")
        .textContent,
    ).toContain("before=mechanics; after=result_state_subtraction");
    expect(
      within(manager).getByTestId("load-manager-combination-basis-preview")
        .textContent,
    ).toContain("unit_validation=not_required_dimensionless");
    expect(queueButton).not.toBeDisabled();
    fireEvent.click(queueButton);

    const applyPanel = screen.getByTestId("operation-apply-panel");
    expect(
      within(applyPanel).getByTestId("operation-apply-row-editor-intent-1")
        .textContent,
    ).toContain("basis");
    fireEvent.click(
      within(applyPanel).getByTestId("apply-intent-editor-intent-1"),
    );
    await waitFor(() =>
      expect(
        within(applyPanel).getByTestId("operation-apply-message").textContent,
      ).toContain(
        "Operation op:load-manager-combination:C-OPER-ALT-basis was not applied (blocked)",
      ),
    );

    expect(
      within(applyPanel).getByTestId(
        "operation-outcome-diagnostic-OP-COMBINATION-BASIS-SHAPE-MISMATCH",
      ).textContent,
    ).toContain("blocking");
    expect(
      within(manager).getByTestId(
        "load-manager-combination-combination:C-OPER-ALT",
      ).textContent,
    ).toContain("basis=mechanics");
    expect(
      screen.getByTestId("local-project-review-context").textContent,
    ).toContain("1 pending operation");
    expect(
      screen.getByTestId("local-project-review-context").textContent,
    ).toContain("applied_operations=0");
  });

  it("queues and applies an explicit combination term through the manager panel", async () => {
    render(<App />);

    const manager = await screen.findByTestId("load-case-manager");
    fireEvent.change(
      within(manager).getByTestId("load-manager-create-load-label"),
      {
        target: { value: "User operating case" },
      },
    );
    fireEvent.click(
      within(manager).getByTestId("queue-create-load-case-intent"),
    );

    const applyPanel = screen.getByTestId("operation-apply-panel");
    fireEvent.click(
      within(applyPanel).getByTestId("apply-intent-editor-intent-1"),
    );
    await waitFor(() =>
      expect(
        within(applyPanel).getByTestId("operation-apply-message").textContent,
      ).toContain("Applied op:load-manager-create-load:L-300"),
    );

    expect(
      within(manager).getByTestId("load-case-manager-summary").textContent,
    ).toContain("3 load cases; 9 primitive loads; 1 combinations");
    fireEvent.change(
      within(manager).getByTestId(
        "load-manager-create-combination-term-load-case",
      ),
      {
        target: { value: "load:L-300" },
      },
    );
    fireEvent.change(
      within(manager).getByTestId(
        "load-manager-create-combination-term-factor",
      ),
      {
        target: { value: "0.25" },
      },
    );

    expect(
      within(manager).getByTestId(
        "load-manager-create-combination-term-heading",
      ).textContent,
    ).toContain("combination:C-OPER-ALT");
    expect(
      within(manager).getByTestId(
        "load-manager-create-combination-term-preview",
      ).textContent,
    ).toContain("op:load-manager-combination:C-OPER-ALT-term-2-create");
    expect(
      within(manager).getByTestId(
        "load-manager-create-combination-term-preview",
      ).textContent,
    ).toContain(
      "before=not_present; after=load:L-300 x 0.25; unit=none; dimensionless",
    );
    expect(
      within(manager).getByTestId(
        "load-manager-create-combination-term-preview",
      ).textContent,
    ).toContain("unit_validation=not_required_dimensionless");
    fireEvent.click(
      within(manager).getByTestId("queue-create-combination-term-intent"),
    );

    expect(
      within(applyPanel).getByTestId("operation-apply-row-editor-intent-2")
        .textContent,
    ).toContain("terms");
    expect(
      within(applyPanel).getByTestId("operation-apply-row-editor-intent-2")
        .textContent,
    ).toContain('"load_case":"load:L-300"');
    fireEvent.click(
      within(applyPanel).getByTestId("apply-intent-editor-intent-2"),
    );
    await waitFor(() =>
      expect(
        within(applyPanel).getByTestId("operation-apply-message").textContent,
      ).toContain(
        "Applied op:load-manager-combination:C-OPER-ALT-term-2-create",
      ),
    );

    expect(
      within(manager).getByTestId(
        "load-manager-combination-combination:C-OPER-ALT",
      ).textContent,
    ).toContain("load:L-100 x 1; load:L-200 x 0.5; load:L-300 x 0.25");
    expect(screen.getByLabelText("Property inspector").textContent).toContain(
      "load:L-300 x 0.25",
    );
    expect(
      within(applyPanel).getByTestId(
        "applied-operation-route-applied-2-editor-intent-2",
      ).textContent,
    ).toContain("persistence session_state_only_not_yet_saved");
    expect(
      screen.getByTestId("local-project-review-context").textContent,
    ).toContain("0 pending operations");
    expect(
      screen.getByTestId("local-project-review-context").textContent,
    ).toContain("applied_operations=2");
    expect(screen.getByTestId("solve-job-summary").textContent).toContain(
      "state=not_started",
    );
  });

  it("queues and applies explicit mechanics combination creation through the manager panel", async () => {
    render(<App />);

    const manager = await screen.findByTestId("load-case-manager");
    expect(
      within(manager).getByTestId("load-manager-create-combination-heading")
        .textContent,
    ).toContain("combination:C-300");
    expect(
      within(manager).getByTestId("load-manager-create-combination-preview")
        .textContent,
    ).toContain("op:load-manager-create-combination:C-300");
    expect(
      within(manager).getByTestId("load-manager-create-combination-preview")
        .textContent,
    ).toContain(
      "before=not_present; after=combination:C-300; term=load:L-100 x 1; unit=none; dimensionless",
    );
    expect(
      within(manager).getByTestId("load-manager-create-combination-preview")
        .textContent,
    ).toContain("unit_validation=not_required_dimensionless");
    fireEvent.change(
      within(manager).getByTestId("load-manager-create-combination-label"),
      {
        target: { value: "User operating combination" },
      },
    );
    fireEvent.click(
      within(manager).getByTestId("queue-create-combination-intent"),
    );

    const applyPanel = screen.getByTestId("operation-apply-panel");
    expect(
      within(applyPanel).getByTestId("operation-apply-row-editor-intent-1")
        .textContent,
    ).toContain("combinations");
    expect(
      within(applyPanel).getByTestId("operation-apply-row-editor-intent-1")
        .textContent,
    ).toContain('"basis":"mechanics"');
    fireEvent.click(
      within(applyPanel).getByTestId("apply-intent-editor-intent-1"),
    );
    await waitFor(() =>
      expect(
        within(applyPanel).getByTestId("operation-apply-message").textContent,
      ).toContain("Applied op:load-manager-create-combination:C-300"),
    );

    expect(
      within(manager).getByTestId("load-case-manager-summary").textContent,
    ).toContain("2 load cases; 9 primitive loads; 2 combinations");
    const newCombination = within(manager).getByTestId(
      "load-manager-combination-combination:C-300",
    );
    expect(newCombination.textContent).toContain("User operating combination");
    expect(newCombination.textContent).toContain("basis=mechanics");
    expect(newCombination.textContent).toContain("load:L-100 x 1");
    fireEvent.click(
      within(manager).getByTestId(
        "load-manager-combination-select-combination:C-300",
      ),
    );
    expect(screen.getByLabelText("Property inspector").textContent).toContain(
      "combination:C-300",
    );
    expect(
      within(applyPanel).getByTestId(
        "applied-operation-route-applied-1-editor-intent-1",
      ).textContent,
    ).toContain("persistence session_state_only_not_yet_saved");
    expect(
      screen.getByTestId("local-project-review-context").textContent,
    ).toContain("0 pending operations");
    expect(
      screen.getByTestId("local-project-review-context").textContent,
    ).toContain("applied_operations=1");
    expect(screen.getByTestId("solve-job-summary").textContent).toContain(
      "state=not_started",
    );
  });

  it("queues and applies an explicit subtraction combination creation through the manager panel", async () => {
    render(<App />);

    const manager = await screen.findByTestId("load-case-manager");
    fireEvent.change(
      within(manager).getByTestId("load-manager-create-combination-basis"),
      {
        target: { value: "result_state_subtraction" },
      },
    );
    expect(
      within(manager).getByTestId("load-manager-create-combination-minuend"),
    ).toHaveValue("load:L-100");
    fireEvent.change(
      within(manager).getByTestId("load-manager-create-combination-subtrahend"),
      {
        target: { value: "load:L-200" },
      },
    );
    fireEvent.change(
      within(manager).getByTestId("load-manager-create-combination-label"),
      {
        target: { value: "User operating-minus-alternate subtraction" },
      },
    );
    expect(
      within(manager).getByTestId("load-manager-create-combination-heading")
        .textContent,
    ).toContain(
      "basis=result_state_subtraction; minuend=load:L-100; subtrahend=load:L-200",
    );
    expect(
      within(manager).getByTestId("load-manager-create-combination-preview")
        .textContent,
    ).toContain("op:load-manager-create-combination:C-300");
    expect(
      within(manager).getByTestId("load-manager-create-combination-preview")
        .textContent,
    ).toContain(
      "before=not_present; after=combination:C-300; minuend=load:L-100; subtrahend=load:L-200; unit=none; dimensionless",
    );
    expect(
      within(manager).getByTestId("load-manager-create-combination-preview")
        .textContent,
    ).toContain("unit_validation=not_required_dimensionless");
    fireEvent.click(
      within(manager).getByTestId("queue-create-combination-intent"),
    );

    const applyPanel = screen.getByTestId("operation-apply-panel");
    expect(
      within(applyPanel).getByTestId("operation-apply-row-editor-intent-1")
        .textContent,
    ).toContain('"basis":"result_state_subtraction"');
    expect(
      within(applyPanel).getByTestId("operation-apply-row-editor-intent-1")
        .textContent,
    ).toContain('"minuend_id":"load:L-100"');
    fireEvent.click(
      within(applyPanel).getByTestId("apply-intent-editor-intent-1"),
    );
    await waitFor(() =>
      expect(
        within(applyPanel).getByTestId("operation-apply-message").textContent,
      ).toContain("Applied op:load-manager-create-combination:C-300"),
    );

    expect(
      within(manager).getByTestId("load-case-manager-summary").textContent,
    ).toContain("2 load cases; 9 primitive loads; 2 combinations");
    const newCombination = within(manager).getByTestId(
      "load-manager-combination-combination:C-300",
    );
    expect(newCombination.textContent).toContain(
      "basis=result_state_subtraction",
    );
    expect(newCombination.textContent).toContain(
      "minuend=load:L-100; subtrahend=load:L-200",
    );
    expect(
      within(applyPanel).getByTestId(
        "applied-operation-route-applied-1-editor-intent-1",
      ).textContent,
    ).toContain("persistence session_state_only_not_yet_saved");
    expect(
      screen.getByTestId("local-project-review-context").textContent,
    ).toContain("applied_operations=1");
    expect(screen.getByTestId("solve-job-summary").textContent).toContain(
      "state=not_started",
    );
  });

  it("blocks the subtraction combination draft until the operands are distinct existing load cases", async () => {
    render(<App />);

    const manager = await screen.findByTestId("load-case-manager");
    fireEvent.change(
      within(manager).getByTestId("load-manager-create-combination-basis"),
      {
        target: { value: "result_state_subtraction" },
      },
    );
    fireEvent.change(
      within(manager).getByTestId("load-manager-create-combination-subtrahend"),
      {
        target: { value: "load:L-100" },
      },
    );

    expect(
      within(manager).getByTestId("queue-create-combination-intent"),
    ).toBeDisabled();
    expect(
      within(manager).getByTestId("load-manager-create-combination-preview")
        .textContent,
    ).toContain("select distinct minuend/subtrahend load cases");
  });

  it("queues and applies an explicit range envelope combination creation through the manager panel", async () => {
    render(<App />);

    const manager = await screen.findByTestId("load-case-manager");
    fireEvent.change(
      within(manager).getByTestId("load-manager-create-combination-basis"),
      {
        target: { value: "range_envelope" },
      },
    );
    const operandSelect = within(manager).getByTestId(
      "load-manager-create-combination-operands",
    ) as HTMLSelectElement;
    for (const option of Array.from(operandSelect.options)) {
      option.selected = true;
    }
    fireEvent.change(operandSelect);
    fireEvent.change(
      within(manager).getByTestId("load-manager-create-combination-mode"),
      {
        target: { value: "min_abs" },
      },
    );
    fireEvent.change(
      within(manager).getByTestId("load-manager-create-combination-label"),
      {
        target: { value: "User envelope over both cases" },
      },
    );
    expect(
      within(manager).getByTestId("load-manager-create-combination-heading")
        .textContent,
    ).toContain(
      "basis=range_envelope; mode=min_abs; operands=load:L-100, load:L-200",
    );
    expect(
      within(manager).getByTestId("load-manager-create-combination-preview")
        .textContent,
    ).toContain(
      "before=not_present; after=combination:C-300; mode=min_abs; operands=load:L-100, load:L-200; unit=none; dimensionless",
    );
    expect(
      within(manager).getByTestId("load-manager-create-combination-preview")
        .textContent,
    ).toContain("unit_validation=not_required_dimensionless");
    fireEvent.click(
      within(manager).getByTestId("queue-create-combination-intent"),
    );

    const applyPanel = screen.getByTestId("operation-apply-panel");
    expect(
      within(applyPanel).getByTestId("operation-apply-row-editor-intent-1")
        .textContent,
    ).toContain('"basis":"range_envelope"');
    expect(
      within(applyPanel).getByTestId("operation-apply-row-editor-intent-1")
        .textContent,
    ).toContain('"mode":"min_abs"');
    fireEvent.click(
      within(applyPanel).getByTestId("apply-intent-editor-intent-1"),
    );
    await waitFor(() =>
      expect(
        within(applyPanel).getByTestId("operation-apply-message").textContent,
      ).toContain("Applied op:load-manager-create-combination:C-300"),
    );

    expect(
      within(manager).getByTestId("load-case-manager-summary").textContent,
    ).toContain("2 load cases; 9 primitive loads; 2 combinations");
    const newCombination = within(manager).getByTestId(
      "load-manager-combination-combination:C-300",
    );
    expect(newCombination.textContent).toContain("basis=range_envelope");
    expect(newCombination.textContent).toContain(
      "mode=min_abs; operands=load:L-100, load:L-200",
    );
    expect(
      screen.getByTestId("local-project-review-context").textContent,
    ).toContain("applied_operations=1");
    expect(screen.getByTestId("solve-job-summary").textContent).toContain(
      "state=not_started",
    );
  });

  it("queues and applies an explicit combination term deletion through the manager panel", async () => {
    render(<App />);

    const manager = await screen.findByTestId("load-case-manager");
    const termRow = within(manager).getByTestId(
      "load-manager-combination-term-combination:C-OPER-ALT-1",
    );
    expect(termRow.textContent).toContain("load:L-200 x 0.5");
    fireEvent.click(termRow);

    expect(
      within(manager).getByTestId("load-manager-combination-delete-preview")
        .textContent,
    ).toContain("op:load-manager-combination:C-OPER-ALT-term-1-delete");
    expect(
      within(manager).getByTestId("load-manager-combination-delete-preview")
        .textContent,
    ).toContain(
      "before=load:L-200 x 0.5; after=not_present; unit=none; dimensionless",
    );
    expect(
      within(manager).getByTestId("load-manager-combination-delete-preview")
        .textContent,
    ).toContain("unit_validation=not_required_dimensionless");
    fireEvent.click(
      within(manager).getByTestId("queue-delete-combination-term-intent"),
    );

    const applyPanel = screen.getByTestId("operation-apply-panel");
    expect(
      within(applyPanel).getByTestId("operation-apply-row-editor-intent-1")
        .textContent,
    ).toContain("terms.1");
    expect(
      within(applyPanel).getByTestId("operation-apply-row-editor-intent-1")
        .textContent,
    ).toContain("not_present");
    fireEvent.click(
      within(applyPanel).getByTestId("apply-intent-editor-intent-1"),
    );
    await waitFor(() =>
      expect(
        within(applyPanel).getByTestId("operation-apply-message").textContent,
      ).toContain(
        "Applied op:load-manager-combination:C-OPER-ALT-term-1-delete",
      ),
    );

    const combinationRow = within(manager).getByTestId(
      "load-manager-combination-combination:C-OPER-ALT",
    );
    expect(combinationRow.textContent).toContain("load:L-100 x 1");
    expect(combinationRow.textContent).not.toContain("load:L-200 x 0.5");
    const inspector = screen.getByLabelText("Property inspector");
    expect(inspector.textContent).toContain("load:L-100 x 1");
    expect(inspector.textContent).not.toContain("load:L-200 x 0.5");
    expect(
      within(applyPanel).getByTestId(
        "applied-operation-route-applied-1-editor-intent-1",
      ).textContent,
    ).toContain("persistence session_state_only_not_yet_saved");
    expect(
      screen.getByTestId("local-project-review-context").textContent,
    ).toContain("0 pending operations");
    expect(
      screen.getByTestId("local-project-review-context").textContent,
    ).toContain("applied_operations=1");
    expect(screen.getByTestId("solve-job-summary").textContent).toContain(
      "state=not_started",
    );
  });

  it("queues and applies an explicit combination deletion through the manager panel", async () => {
    render(<App />);

    const manager = await screen.findByTestId("load-case-manager");
    const combinationRow = within(manager).getByTestId(
      "load-manager-combination-combination:C-OPER-ALT",
    );
    expect(combinationRow.textContent).toContain("load:L-100 x 1");
    expect(combinationRow.textContent).toContain("load:L-200 x 0.5");
    fireEvent.click(
      within(manager).getByTestId(
        "load-manager-combination-select-combination:C-OPER-ALT",
      ),
    );

    expect(
      within(manager).getByTestId(
        "load-manager-combination-entity-delete-preview",
      ).textContent,
    ).toContain("op:load-manager-combination:C-OPER-ALT-delete");
    expect(
      within(manager).getByTestId(
        "load-manager-combination-entity-delete-preview",
      ).textContent,
    ).toContain(
      "before=combination:C-OPER-ALT; Invented explicit operating plus alternate preview; basis=mechanics; terms=load:L-100 x 1; load:L-200 x 0.5",
    );
    expect(
      within(manager).getByTestId(
        "load-manager-combination-entity-delete-preview",
      ).textContent,
    ).toContain("unit_validation=not_required_dimensionless");
    fireEvent.click(
      within(manager).getByTestId("queue-delete-combination-intent"),
    );

    const applyPanel = screen.getByTestId("operation-apply-panel");
    expect(
      within(applyPanel).getByTestId("operation-apply-row-editor-intent-1")
        .textContent,
    ).toContain("op:load-manager-combination:C-OPER-ALT-delete");
    expect(
      within(applyPanel).getByTestId("operation-apply-row-editor-intent-1")
        .textContent,
    ).toContain("combinations");
    expect(
      within(applyPanel).getByTestId("operation-apply-row-editor-intent-1")
        .textContent,
    ).toContain("not_present");
    fireEvent.click(
      within(applyPanel).getByTestId("apply-intent-editor-intent-1"),
    );
    await waitFor(() =>
      expect(
        within(applyPanel).getByTestId("operation-apply-message").textContent,
      ).toContain("Applied op:load-manager-combination:C-OPER-ALT-delete"),
    );

    expect(
      screen.queryByTestId("load-manager-combination-combination:C-OPER-ALT"),
    ).toBeNull();
    expect(
      within(manager).getByTestId("load-case-manager-summary").textContent,
    ).toContain("2 load cases; 9 primitive loads; 0 combinations");
    expect(
      within(applyPanel).getByTestId(
        "applied-operation-route-applied-1-editor-intent-1",
      ).textContent,
    ).toContain("persistence session_state_only_not_yet_saved");
    expect(
      screen.getByTestId("local-project-review-context").textContent,
    ).toContain("0 pending operations");
    expect(
      screen.getByTestId("local-project-review-context").textContent,
    ).toContain("applied_operations=1");
    expect(screen.getByTestId("solve-job-summary").textContent).toContain(
      "state=not_started",
    );
  });

  it("renders an empty editor-intent queue when no app queue has been materialized", async () => {
    const model = await loadPreviewModel();

    render(
      <PropertyInspector
        model={model}
        onQueueIntent={() => undefined}
        selection={{ type: "material", id: "material:invented-carbon-steel" }}
      />,
    );

    const intentPanel = screen.getByLabelText("Editor operation intent");
    expect(
      within(intentPanel).getByTestId("editor-intent-queue-empty").textContent,
    ).toContain("No transient editor intents queued");
    await waitFor(() =>
      expect(
        screen.getByTestId("property-unit-catalog-status").textContent,
      ).toContain("browser preview uses model metadata"),
    );
    expect(
      screen.getByTestId("property-unit-basis-summary").textContent,
    ).toContain("m, model metadata");
    expect(
      screen.getByTestId("property-unit-basis-summary").textContent,
    ).toContain("Pa, model metadata");
  });

  it("queues node coordinate edits with an explicit unit payload", async () => {
    const model = await loadPreviewModel();
    const queued: EditorOperationIntent[] = [];

    render(
      <PropertyInspector
        model={model}
        onQueueIntent={(intent) => queued.push(intent)}
        selection={{ type: "node", id: "node:N-110" }}
      />,
    );

    await waitFor(() =>
      expect(
        screen.getByTestId("property-unit-catalog-status").textContent,
      ).toContain("browser preview uses model metadata"),
    );
    const intentPanel = screen.getByLabelText("Editor operation intent");
    fireEvent.change(within(intentPanel).getByTestId("editor-intent-field"), {
      target: { value: "position.y" },
    });

    expect(within(intentPanel).getByTestId("editor-intent-unit")).toHaveValue(
      "m",
    );
    expect(
      within(intentPanel).getByText("Proposed value (m, model metadata)"),
    ).toBeInTheDocument();

    fireEvent.change(within(intentPanel).getByTestId("editor-intent-value"), {
      target: { value: "1.25" },
    });
    fireEvent.click(within(intentPanel).getByTestId("queue-editor-intent"));

    expect(queued).toHaveLength(1);
    expect(queued[0].change.field_path).toBe("position.y");
    expect(queued[0].change.dimension).toBe("length");
    expect(queued[0].change.unit).toBe("m");
    expect(JSON.parse(queued[0].change.after)).toEqual({
      value: 1.25,
      unit: "m",
    });
    expect(queued[0].change.source_note).toContain(
      "entered unit captured explicitly",
    );
  });

  it("queues load-case primitive magnitude inspector edits with an explicit unit payload", async () => {
    const model = await loadPreviewModel();
    const queued: EditorOperationIntent[] = [];

    render(
      <PropertyInspector
        model={model}
        onQueueIntent={(intent) => queued.push(intent)}
        selection={{ type: "load", id: "load:L-100" }}
      />,
    );

    await waitFor(() =>
      expect(
        screen.getByTestId("property-unit-catalog-status").textContent,
      ).toContain("browser preview uses model metadata"),
    );
    const intentPanel = screen.getByLabelText("Editor operation intent");
    fireEvent.change(within(intentPanel).getByTestId("editor-intent-field"), {
      target: { value: "primitive_loads.0.magnitude.value" },
    });

    expect(within(intentPanel).getByTestId("editor-intent-unit")).toHaveValue(
      "N/m",
    );
    expect(
      within(intentPanel).getByText("Proposed value (N/m, model metadata)"),
    ).toBeInTheDocument();

    fireEvent.change(within(intentPanel).getByTestId("editor-intent-value"), {
      target: { value: "-225" },
    });
    fireEvent.click(within(intentPanel).getByTestId("queue-editor-intent"));

    expect(queued).toHaveLength(1);
    expect(queued[0].change.change_kind).toBe("update_load");
    expect(queued[0].change.field_path).toBe(
      "primitive_loads.0.magnitude.value",
    );
    expect(queued[0].change.dimension).toBe("force_per_length");
    expect(queued[0].change.unit).toBe("N/m");
    expect(JSON.parse(queued[0].change.after)).toEqual({
      value: -225,
      unit: "N/m",
    });
    expect(queued[0].change.source_note).toContain(
      "entered unit captured explicitly",
    );
  });

  it("applies the solved global displacement direction to the deformation overlay", async () => {
    const model = await loadPreviewModel();
    const overlay = buildDeformationOverlay(
      model,
      deformationResultRows([
        displacementMagnitudeRow("node:N-140", 20),
        displacementMagnitudeRow("node:N-130", 10),
        ...displacementComponentRows("node:N-140", { x: 0, y: 0, z: -20 }),
        ...displacementComponentRows("node:N-130", { x: 10, y: 0, z: 0 }),
      ]),
    );

    expect(overlay.state).toBe("available");
    expect(overlay.boundary).toContain(
      "vector_direction=global_cartesian_displacement_components",
    );
    expect(overlay.boundary).toContain(
      "scale=normalized_display_offset_not_physical_length",
    );
    // node:N-140 governs (20 mm) and moves along -Z by the full display offset.
    expect(overlay.nodePositions.get("node:N-140")).toEqual({
      x: 7.6,
      y: 2.4,
      z: 2.2 - 0.65,
    });
    // node:N-130 moves along +X by half the display offset, not vertically.
    expect(overlay.nodePositions.get("node:N-130")).toEqual({
      x: 7.6 + 0.325,
      y: 2.4,
      z: 0,
    });
    // Nodes without displacement rows stay at their undeformed position.
    expect(overlay.nodePositions.get("node:N-100")).toEqual({
      x: 0,
      y: 0,
      z: 0,
    });
  });

  it("falls back to the vertical display axis when displacement component rows are absent", async () => {
    const model = await loadPreviewModel();
    const overlay = buildDeformationOverlay(
      model,
      deformationResultRows([
        displacementMagnitudeRow("node:N-140", 20),
        displacementMagnitudeRow("node:N-130", 10),
      ]),
    );

    expect(overlay.state).toBe("available");
    expect(overlay.boundary).toContain(
      "vector_direction=vertical_display_axis_fallback_component_rows_unavailable",
    );
    expect(overlay.boundary).not.toContain("vector_direction=TBD");
    expect(overlay.nodePositions.get("node:N-140")).toEqual({
      x: 7.6,
      y: 2.4 + 0.65,
      z: 2.2,
    });
    expect(overlay.nodePositions.get("node:N-130")).toEqual({
      x: 7.6,
      y: 2.4 + 0.325,
      z: 0,
    });
  });

  it("requires direction components from the same result basis as the governing magnitude row", async () => {
    const model = await loadPreviewModel();
    const overlay = buildDeformationOverlay(
      model,
      deformationResultRows([
        displacementMagnitudeRow("node:N-140", 20),
        ...displacementComponentRows(
          "node:N-140",
          { x: 0, y: 0, z: -20 },
          "load:L-OTHER",
        ),
      ]),
    );

    expect(overlay.state).toBe("available");
    expect(overlay.boundary).toContain(
      "vector_direction=vertical_display_axis_fallback_component_rows_unavailable",
    );
    expect(overlay.nodePositions.get("node:N-140")).toEqual({
      x: 7.6,
      y: 2.4 + 0.65,
      z: 2.2,
    });
  });

  it("discloses the deformation overlay fallback in the viewport status line", async () => {
    const model = await loadPreviewModel();

    render(
      <PipeViewport
        model={model}
        onSelect={() => undefined}
        result={deformationResultRows([
          displacementMagnitudeRow("node:N-140", 20),
        ])}
        selection={{ type: "node", id: "node:N-100" }}
      />,
    );

    expect(
      screen.getByTestId("viewport-deformation-summary").textContent,
    ).toContain("available; nodes=1; max=20 mm");
    expect(
      screen.getByTestId("viewport-deformation-boundary").textContent,
    ).toContain(
      "vector_direction=vertical_display_axis_fallback_component_rows_unavailable",
    );
    expect(
      screen.getByTestId("viewport-deformation-boundary").textContent,
    ).toContain("professional_claim=false");
  });

  it("does not claim professional or release acceptance", async () => {
    render(<App />);

    const footer = await screen.findByText(/Technical preview only/i);
    expect(footer.textContent).toContain("no production-readiness");
    expect(footer.textContent).toContain(
      "no licensed engineering reliance claim",
    );
  });

  it("carries queued editor intents into the report packet as review-only operation context", async () => {
    render(<App />);

    const tree = await screen.findByLabelText("Model tree");
    fireEvent.click(
      within(tree).getByRole("button", {
        name: /Invented carbon-steel-like material/i,
      }),
    );
    const inspector = screen.getByLabelText("Property inspector");
    const intentPanel = within(inspector).getByLabelText(
      "Editor operation intent",
    );

    fireEvent.change(within(intentPanel).getByTestId("editor-intent-field"), {
      target: { value: "elastic_modulus.value" },
    });
    fireEvent.change(within(intentPanel).getByTestId("editor-intent-value"), {
      target: { value: "210000000000" },
    });
    const expectedMaterialEditAfter = JSON.stringify({
      value: 210000000000,
      unit: "Pa",
    });
    fireEvent.click(within(intentPanel).getByTestId("queue-editor-intent"));
    expect(
      within(intentPanel).getByTestId("editor-intent-queue").textContent,
    ).toContain("editor-intent-1");

    fireEvent.click(
      screen.getByRole("button", { name: /Run mechanics preview/i }),
    );
    const report = await screen.findByLabelText("Report packet");
    await waitFor(() =>
      expect(
        screen.getByTestId("viewport-deformation-status").textContent,
      ).toContain("available; nodes=5; max=4.567557 mm"),
    );
    expect(
      screen.getByTestId("viewport-deformation-boundary").textContent,
    ).toContain("scale=normalized_display_offset_not_physical_length");
    expect(
      screen.getByTestId("viewport-deformation-boundary").textContent,
    ).toContain("vector_direction=global_cartesian_displacement_components");
    expect(
      await within(report).findByTestId("report-editor-intent-summary"),
    ).toHaveTextContent("1 queued");
    expect(
      within(report).getByTestId("report-unit-system").textContent,
    ).toContain("unit-system:dec-018-si-dual-display");
    expect(
      within(report).getByTestId("report-unit-system").textContent,
    ).toContain("length=m");
    expect(
      within(report).getByTestId("report-unit-system").textContent,
    ).toContain("stress=MPa");
    expect(
      within(report).getByTestId("report-unit-system").textContent,
    ).toContain("conversion=false");
    expect(
      within(report).getByTestId("report-editor-intent-operation").textContent,
    ).toContain(
      "op:editor-intent-material:invented-carbon-steel-elastic_modulus.value",
    );
    expect(
      within(report).getByTestId("report-editor-intent-operation").textContent,
    ).toContain("not_applied");
    expect(
      within(report).getByTestId("report-editor-intent-boundary").textContent,
    ).toContain("does not mutate accepted model state");
    expect(
      within(report).getByTestId("report-editor-intent-boundary").textContent,
    ).toContain("no compliance or professional approval claim");

    const exportHref =
      within(report).getByTestId("report-export-link").getAttribute("href") ??
      "";
    const exportPacket = JSON.parse(
      decodeURIComponent(exportHref.split(",", 2)[1]),
    );
    expect(exportPacket.editor_intent_refs).toContain(
      "op:editor-intent-material:invented-carbon-steel-elastic_modulus.value",
    );
    expect(exportPacket.editor_intent_summary.queued_count).toBe(1);
    expect(exportPacket.editor_intent_summary.application_statuses).toContain(
      "not_applied",
    );
    expect(
      exportPacket.editor_intent_summary.mutates_accepted_model_state,
    ).toBe(false);
    expect(
      exportPacket.editor_intent_summary.direct_model_mutation_allowed,
    ).toBe(false);
    expect(
      exportPacket.editor_intent_summary.release_or_professional_claim,
    ).toBe(false);
    expect(exportPacket.unit_system_disclosure.unit_system_ref.ref_id).toBe(
      "unit-system:dec-018-si-dual-display",
    );
    expect(exportPacket.unit_system_disclosure.model_units.length).toBe("m");
    expect(exportPacket.unit_system_disclosure.model_units.stress).toBe("MPa");
    expect(exportPacket.unit_system_disclosure.result_units).toContain("mm");
    expect(exportPacket.unit_system_disclosure.result_units).toContain("MPa");
    expect(exportPacket.unit_system_disclosure.conversion_performed).toBe(
      false,
    );
    expect(exportPacket.unit_system_disclosure.protected_content_included).toBe(
      false,
    );
    expect(exportPacket.editor_operation_intents[0].queue_id).toBe(
      "editor-intent-1",
    );
    expect(exportPacket.editor_operation_intents[0].change.field_path).toBe(
      "elastic_modulus.value",
    );
    expect(exportPacket.editor_operation_intents[0].change.after).toBe(
      expectedMaterialEditAfter,
    );
    expect(
      exportPacket.editor_operation_intents[0].validation.application_status,
    ).toBe("not_applied");
    expect(
      exportPacket.editor_operation_intents[0].audit_boundary
        .mutates_accepted_model_state,
    ).toBe(false);
    expect(
      exportPacket.editor_operation_intents[0].professional_boundary
        .software_makes_compliance_claim,
    ).toBe(false);
    expect(exportPacket.private_payload_included).toBe(false);
    expect(exportPacket.protected_content_included).toBe(false);

    const handoff = await screen.findByLabelText("Handoff package");
    expect(
      (await within(handoff).findByTestId("handoff-review-context"))
        .textContent,
    ).toContain("1 pending operation");
    expect(
      within(handoff).getByTestId("handoff-boundary").textContent,
    ).toContain("no release or professional claim");
    const handoffHref =
      within(handoff).getByTestId("handoff-export-link").getAttribute("href") ??
      "";
    const handoffPacket = JSON.parse(
      decodeURIComponent(handoffHref.split(",", 2)[1]),
    );
    expect(handoffPacket.document_kind).toBe(
      "openpipestress.technical_preview.handoff_package",
    );
    expect(handoffPacket.editor_intent_refs).toContain(
      "op:editor-intent-material:invented-carbon-steel-elastic_modulus.value",
    );
    expect(handoffPacket.editor_operation_statuses).toContain("not_applied");
    expect(handoffPacket.private_payload_included).toBe(false);
    expect(handoffPacket.protected_content_included).toBe(false);
    expect(handoffPacket.release_or_professional_claim).toBe(false);

    const diffPreview = await screen.findByLabelText("Operation diff preview");
    expect(
      await within(diffPreview).findByText(
        /state:project:invented-loop-01:preview/i,
      ),
    ).toBeInTheDocument();
    expect(
      within(diffPreview).getByTestId("diff-preview-summary").textContent,
    ).toContain("1 operations");
    expect(
      within(diffPreview).getByTestId("diff-preview-summary").textContent,
    ).toContain("1 diff rows");
    expect(
      within(diffPreview).getByTestId("diff-preview-summary").textContent,
    ).toContain("accepted_state_mutated=false");
    expect(
      within(diffPreview).getByTestId("diff-preview-validation").textContent,
    ).toContain("0 hash-bound rows");
    expect(
      within(diffPreview).getByTestId("diff-preview-validation").textContent,
    ).toContain("1 held");
    expect(
      within(diffPreview).getByTestId("diff-preview-units").textContent,
    ).toContain("unit-system:dec-018-si-dual-display");
    expect(
      within(diffPreview).getByTestId("diff-preview-units").textContent,
    ).toContain("conversion=false");
    expect(
      within(diffPreview).getByTestId("diff-preview-unit-witnesses")
        .textContent,
    ).toContain("count=1");
    expect(
      within(diffPreview).getByTestId("diff-preview-unit-witnesses")
        .textContent,
    ).toContain("conversion=false");
    expect(
      within(diffPreview).getByTestId("diff-preview-boundary").textContent,
    ).toContain("accepted model mutated=false");
    expect(
      within(diffPreview).getByTestId("diff-preview-boundary").textContent,
    ).toContain("no release or professional claim");
    expect(
      within(diffPreview).getByTestId(
        "diff-preview-record-op-editor-intent-material-invented-carbon-steel-elastic-modulus-value",
      ).textContent,
    ).toContain(`200000000000 to ${expectedMaterialEditAfter} Pa`);
    const diffHref =
      within(diffPreview)
        .getByTestId("diff-preview-export-link")
        .getAttribute("href") ?? "";
    const diffPacket = JSON.parse(
      decodeURIComponent(diffHref.split(",", 2)[1]),
    );
    expect(diffPacket.document_kind).toBe(
      "openpipestress.technical_preview.operation_diff_preview",
    );
    expect(diffPacket.deliverable_refs).toContain("DEL-16-02");
    expect(diffPacket.scope_items).toContain("SOW-069");
    expect(diffPacket.summary.operation_count).toBe(1);
    expect(diffPacket.summary.diff_row_count).toBe(1);
    expect(diffPacket.summary.accepted_model_state_mutated).toBe(false);
    expect(diffPacket.summary.hash_bound_diff_preview_count).toBe(0);
    expect(diffPacket.summary.local_visual_diff_preview_count).toBe(1);
    expect(diffPacket.summary.unit_preservation_witness_count).toBe(1);
    expect(diffPacket.unit_system_disclosure.unit_system_ref.ref).toBe(
      "unit-system:dec-018-si-dual-display",
    );
    expect(
      diffPacket.unit_system_disclosure.target_export_units
        .diff_preview_change_rows,
    ).toBe("per_change_declared_unit");
    expect(diffPacket.unit_system_disclosure.conversion_performed).toBe(false);
    expect(diffPacket.unit_witness_policy).toBe(
      "preserve_operation_diff_change_value_unit_and_dimension_per_unit_bearing_row",
    );
    expect(diffPacket.unit_preservation_witnesses).toHaveLength(1);
    expect(
      diffPacket.unit_preservation_witnesses[0].source_quantity,
    ).toMatchObject({
      before: "200000000000",
      after: expectedMaterialEditAfter,
      unit: "Pa",
      dimension: "stress",
    });
    expect(
      diffPacket.unit_preservation_witnesses[0].target_quantity,
    ).toMatchObject({
      before: "200000000000",
      after: expectedMaterialEditAfter,
      unit: "Pa",
      dimension: "stress",
    });
    expect(diffPacket.unit_preservation_witnesses[0].conversion_performed).toBe(
      false,
    );
    expect(diffPacket.previews[0].record_source).toBe(
      "gui_editor_intent_queue",
    );
    expect(diffPacket.previews[0].operation_id).toBe(
      "op:editor-intent-material:invented-carbon-steel-elastic_modulus.value",
    );
    expect(diffPacket.previews[0].application_status).toBe("not_applied");
    expect(diffPacket.previews[0].accepted_model_state_mutated).toBe(false);
    expect(diffPacket.previews[0].hash_bound_diff_preview).toBe(false);
    expect(diffPacket.previews[0].changes[0].field_path).toBe(
      "elastic_modulus.value",
    );
    expect(diffPacket.previews[0].changes[0].before).toBe("200000000000");
    expect(diffPacket.previews[0].changes[0].after).toBe(
      expectedMaterialEditAfter,
    );
    expect(diffPacket.private_payload_included).toBe(false);
    expect(diffPacket.protected_content_included).toBe(false);
    expect(diffPacket.release_or_professional_claim).toBe(false);
    expect(
      diffPacket.professional_boundary.software_makes_compliance_claim,
    ).toBe(false);

    const operationLedger = await screen.findByLabelText(
      "Operation review ledger",
    );
    expect(
      await within(operationLedger).findByText(
        /state:project:invented-loop-01:preview/i,
      ),
    ).toBeInTheDocument();
    expect(
      within(operationLedger).getByTestId("operation-ledger-export-summary")
        .textContent,
    ).toContain("1 review record");
    expect(
      within(operationLedger).getByTestId("operation-ledger-decision-counts")
        .textContent,
    ).toContain("1 held_for_user_acceptance");
    expect(
      within(operationLedger).getByTestId("operation-ledger-latest")
        .textContent,
    ).toContain(
      "op:editor-intent-material:invented-carbon-steel-elastic_modulus.value",
    );
    expect(
      within(operationLedger).getByTestId("operation-ledger-latest")
        .textContent,
    ).toContain("not_applied");
    expect(
      within(operationLedger).getByTestId("operation-ledger-boundary")
        .textContent,
    ).toContain("does not mutate accepted model state");
    expect(
      within(operationLedger).getByTestId("operation-ledger-unit-policy")
        .textContent,
    ).toContain("unit_bearing_changes=1");
    const operationLedgerUnitPolicy =
      within(operationLedger).getByTestId("operation-ledger-unit-policy")
        .textContent ?? "";
    expect(operationLedgerUnitPolicy).toContain("unit_validations=");
    expect([
      "unit_validations=catalog_loading_unit_dimension_declared",
      "unit_validations=model_metadata_unit_dimension_declared_catalog_unavailable_browser_preview",
    ]).toContain(
      operationLedgerUnitPolicy
        .split("; ")
        .find((item) => item.startsWith("unit_validations=")),
    );
    expect(
      within(operationLedger).getByTestId("operation-ledger-unit-policy")
        .textContent,
    ).toContain("receipt_units=not_serialized_in_review_ledger");
    const ledgerHref =
      within(operationLedger)
        .getByTestId("operation-ledger-export-link")
        .getAttribute("href") ?? "";
    const ledgerPacket = JSON.parse(
      decodeURIComponent(ledgerHref.split(",", 2)[1]),
    );
    expect(ledgerPacket.document_kind).toBe(
      "openpipestress.technical_preview.operation_review_ledger",
    );
    expect(ledgerPacket.deliverable_refs).toContain("DEL-16-03");
    expect(ledgerPacket.scope_items).toContain("SOW-069");
    expect(ledgerPacket.scope_items).toContain("SOW-070");
    expect(ledgerPacket.decision_counts.held_for_user_acceptance).toBe(1);
    expect(ledgerPacket.decision_counts.accepted).toBe(0);
    expect(ledgerPacket.decision_counts.rejected).toBe(0);
    expect(ledgerPacket.unit_policy_evidence.evidence_id).toBe(
      "unit-policy-evidence:operation-review-ledger",
    );
    expect(ledgerPacket.unit_policy_evidence.unit_policy).toBe(
      "ledger_preserves_operation_unit_metadata_without_conversion",
    );
    expect(ledgerPacket.unit_policy_evidence.unit_bearing_change_count).toBe(1);
    expect(ledgerPacket.unit_policy_evidence.dimensionless_change_count).toBe(
      0,
    );
    expect(ledgerPacket.unit_policy_evidence.unit_validation_statuses).toHaveLength(1);
    expect([
      "catalog_loading_unit_dimension_declared",
      "model_metadata_unit_dimension_declared_catalog_unavailable_browser_preview",
    ]).toContain(ledgerPacket.unit_policy_evidence.unit_validation_statuses[0]);
    expect(ledgerPacket.unit_policy_evidence.conversion_performed).toBe(false);
    expect(ledgerPacket.records[0].record_source).toBe(
      "gui_editor_intent_queue",
    );
    expect(ledgerPacket.records[0].decision.status).toBe(
      "held_for_user_acceptance",
    );
    expect(ledgerPacket.records[0].decision.explicit_user_acceptance).toBe(
      false,
    );
    expect(ledgerPacket.records[0].affected_entities[0].ref).toBe(
      "material:invented-carbon-steel",
    );
    expect(ledgerPacket.records[0].validation_outcome.application_status).toBe(
      "not_applied",
    );
    expect(
      ledgerPacket.records[0].diagnostics.map(
        (item: { code: string }) => item.code,
      ),
    ).toContain("AUDIT-EXPLICIT-USER-ACCEPTANCE-REQUIRED");
    expect(ledgerPacket.accepted_model_state_unchanged).toBe(true);
    expect(ledgerPacket.private_payload_included).toBe(false);
    expect(ledgerPacket.protected_content_included).toBe(false);
    expect(ledgerPacket.release_or_professional_claim).toBe(false);

    const exportReview = await screen.findByLabelText("Export safety review");
    expect(
      await within(exportReview).findByText(/run:preview-linear-static-001/i),
    ).toBeInTheDocument();
    expect(
      within(exportReview).getByTestId("export-review-summary").textContent,
    ).toContain("28 of 29 local exports ready");
    expect(
      within(exportReview).getByTestId("export-review-summary").textContent,
    ).toContain("no private/protected payloads");
    expect(
      within(exportReview).getByTestId("export-review-boundary").textContent,
    ).toContain("no release or professional claim");
    expect(
      within(exportReview).getByTestId("export-review-units").textContent,
    ).toContain("unit-system:dec-018-si-dual-display");
    expect(
      within(exportReview).getByTestId("export-review-units").textContent,
    ).toContain("covered=26/27");
    expect(
      within(exportReview).getByTestId("export-review-units").textContent,
    ).toContain("conversion=false");
    expect(
      within(exportReview).getByTestId(
        "export-review-record-project_storage_audit",
      ).textContent,
    ).toContain("available");
    expect(
      within(exportReview).getByTestId(
        "export-review-record-project_validation_preflight",
      ).textContent,
    ).toContain("available");
    expect(
      within(exportReview).getByTestId(
        "export-review-record-telemetry_boundary_review",
      ).textContent,
    ).toContain("available");
    expect(
      within(exportReview).getByTestId(
        "export-review-record-secret_private_library_boundary_review",
      ).textContent,
    ).toContain("available");
    expect(
      within(exportReview).getByTestId(
        "export-review-record-security_threat_model_review",
      ).textContent,
    ).toContain("available");
    expect(
      within(exportReview).getByTestId(
        "export-review-record-editor_contract_review",
      ).textContent,
    ).toContain("available");
    expect(
      within(exportReview).getByTestId(
        "export-review-record-missing_data_warning_blocking_review",
      ).textContent,
    ).toContain("available");
    expect(
      within(exportReview).getByTestId(
        "export-review-record-rule_completeness_review",
      ).textContent,
    ).toContain("available");
    expect(
      within(exportReview).getByTestId(
        "export-review-record-accessibility_usability_baseline_review",
      ).textContent,
    ).toContain("available");
    expect(
      within(exportReview).getByTestId(
        "export-review-record-design_authoring_comparison_workspace",
      ).textContent,
    ).toContain("available");
    expect(
      within(exportReview).getByTestId(
        "export-review-record-build_package_readiness",
      ).textContent,
    ).toContain("available");
    expect(
      within(exportReview).getByTestId(
        "export-review-record-validation_release_evidence_review",
      ).textContent,
    ).toContain("available");
    expect(
      within(exportReview).getByTestId("export-review-record-result_envelope")
        .textContent,
    ).toContain("available");
    expect(
      within(exportReview).getByTestId(
        "export-review-record-stress_neutral_csv_json_package",
      ).textContent,
    ).toContain("available");
    expect(
      within(exportReview).getByTestId(
        "export-review-record-headless_runner_envelope",
      ).textContent,
    ).toContain("available");
    expect(
      within(exportReview).getByTestId(
        "export-review-record-adapter_framework_envelope",
      ).textContent,
    ).toContain("available");
    expect(
      within(exportReview).getByTestId(
        "export-review-record-local_fea_handoff_package",
      ).textContent,
    ).toContain("available");
    expect(
      within(exportReview).getByTestId(
        "export-review-record-external_prover_boundary_metadata",
      ).textContent,
    ).toContain("available");
    expect(
      within(exportReview).getByTestId(
        "export-review-record-review_geometry_export",
      ).textContent,
    ).toContain("available");
    expect(
      within(exportReview).getByTestId(
        "export-review-record-conservative_pcf_export",
      ).textContent,
    ).toContain("available");
    expect(
      within(exportReview).getByTestId(
        "export-review-record-caepipe_mbf_export",
      ).textContent,
    ).toContain("available");
    expect(
      within(exportReview).getByTestId(
        "export-review-record-caepipe_external_run_evidence",
      ).textContent,
    ).toContain("available");
    expect(
      within(exportReview).getByTestId(
        "export-review-record-export_adapter_sdk_registry",
      ).textContent,
    ).toContain("available");
    expect(
      within(exportReview).getByTestId(
        "export-review-record-native_json_package",
      ).textContent,
    ).toContain("available");
    expect(
      within(exportReview).getByTestId("export-review-record-report_packet")
        .textContent,
    ).toContain("available");
    expect(
      within(exportReview).getByTestId(
        "export-review-record-report_protected_content_lint",
      ).textContent,
    ).toContain("available");
    expect(
      within(exportReview).getByTestId("export-review-record-handoff_package")
        .textContent,
    ).toContain("available");
    expect(
      within(exportReview).getByTestId(
        "export-review-record-operation_review_ledger",
      ).textContent,
    ).toContain("available");
    const reviewHref =
      within(exportReview)
        .getByTestId("export-review-link")
        .getAttribute("href") ?? "";
    const reviewManifest = JSON.parse(
      decodeURIComponent(reviewHref.split(",", 2)[1]),
    );
    expect(reviewManifest.document_kind).toBe(
      "openpipestress.technical_preview.export_review_manifest",
    );
    expect(reviewManifest.deliverable_refs).toContain("DEL-12-02");
    expect(reviewManifest.deliverable_refs).toContain("DEL-02-05");
    expect(reviewManifest.deliverable_refs).toContain("DEL-12-01");
    expect(reviewManifest.deliverable_refs).toContain("DEL-12-03");
    expect(reviewManifest.deliverable_refs).toContain("DEL-12-04");
    expect(reviewManifest.deliverable_refs).toContain("DEL-08-04");
    expect(reviewManifest.deliverable_refs).toContain("DEL-08-05");
    expect(reviewManifest.deliverable_refs).toContain("DEL-02-04");
    expect(reviewManifest.deliverable_refs).toContain("DEL-07-02");
    expect(reviewManifest.deliverable_refs).toContain("DEL-07-03");
    expect(reviewManifest.deliverable_refs).toContain("DEL-07-04");
    expect(reviewManifest.deliverable_refs).toContain("DEL-07-06");
    expect(reviewManifest.deliverable_refs).toContain("DEL-07-08");
    expect(reviewManifest.deliverable_refs).toContain("DEL-10-01");
    expect(reviewManifest.deliverable_refs).toContain("DEL-10-02");
    expect(reviewManifest.deliverable_refs).toContain("DEL-10-03");
    expect(reviewManifest.deliverable_refs).toContain("DEL-10-04");
    expect(reviewManifest.deliverable_refs).toContain("DEL-10-05");
    expect(reviewManifest.deliverable_refs).toContain("DEL-15-04");
    expect(reviewManifest.deliverable_refs).toContain("DEL-17-02");
    expect(reviewManifest.deliverable_refs).toContain("DEL-17-03");
    expect(reviewManifest.deliverable_refs).toContain("DEL-17-04");
    expect(reviewManifest.deliverable_refs).toContain("DEL-17-05");
    expect(reviewManifest.deliverable_refs).toContain("DEL-17-06");
    expect(reviewManifest.deliverable_refs).toContain("DEL-17-07");
    expect(reviewManifest.deliverable_refs).toContain("DEL-17-08");
    expect(reviewManifest.deliverable_refs).toContain("DEL-17-09");
    expect(reviewManifest.deliverable_refs).toContain("DEL-12-05");
    expect(reviewManifest.deliverable_refs).toContain("DEL-09-01");
    expect(reviewManifest.deliverable_refs).toContain("DEL-09-02");
    expect(reviewManifest.deliverable_refs).toContain("DEL-09-03");
    expect(reviewManifest.deliverable_refs).toContain("DEL-09-04");
    expect(reviewManifest.deliverable_refs).toContain("DEL-09-05");
    expect(reviewManifest.deliverable_refs).toContain("DEL-13-01");
    expect(reviewManifest.deliverable_refs).toContain("DEL-14-02");
    expect(reviewManifest.deliverable_refs).toContain("DEL-16-04");
    expect(reviewManifest.scope_items).toContain("SOW-026");
    expect(reviewManifest.scope_items).toContain("SOW-027");
    expect(reviewManifest.scope_items).toContain("SOW-040");
    expect(reviewManifest.scope_items).toContain("SOW-050");
    expect(reviewManifest.scope_items).toContain("SOW-041");
    expect(reviewManifest.scope_items).toContain("SOW-029");
    expect(reviewManifest.scope_items).toContain("SOW-037");
    expect(reviewManifest.scope_items).toContain("SOW-046");
    expect(reviewManifest.scope_items).toContain("SOW-043");
    expect(reviewManifest.scope_items).toContain("SOW-054");
    expect(reviewManifest.scope_items).toContain("SOW-032");
    expect(reviewManifest.scope_items).toContain("SOW-021");
    expect(reviewManifest.scope_items).toContain("SOW-022");
    expect(reviewManifest.scope_items).toContain("SOW-030");
    expect(reviewManifest.scope_items).toContain("SOW-031");
    expect(reviewManifest.scope_items).toContain("SOW-049");
    expect(reviewManifest.scope_items).toContain("SOW-075");
    expect(reviewManifest.scope_items).toContain("SOW-076");
    expect(reviewManifest.objectives).toContain("OBJ-002");
    expect(reviewManifest.objectives).toContain("OBJ-006");
    expect(reviewManifest.objectives).toContain("OBJ-011");
    expect(reviewManifest.objectives).toContain("OBJ-008");
    expect(reviewManifest.objectives).toContain("OBJ-009");
    expect(reviewManifest.objectives).toContain("OBJ-016");
    expect(reviewManifest.objectives).toContain("OBJ-017");
    expect(reviewManifest.summary.export_count).toBe(29);
    expect(reviewManifest.summary.available_count).toBe(28);
    expect(reviewManifest.summary.operation_record_count).toBe(1);
    expect(reviewManifest.unit_policy_summary.evidence_id).toBe(
      "unit-policy-evidence:export-review-manifest",
    );
    expect(reviewManifest.unit_policy_summary.unit_system_ref.ref).toBe(
      "unit-system:dec-018-si-dual-display",
    );
    expect(reviewManifest.unit_policy_summary.conversion_policy).toBe(
      "export_review_manifest_inventory_only_no_target_conversion",
    );
    expect(reviewManifest.unit_policy_summary.conversion_performed).toBe(false);
    expect(
      reviewManifest.unit_policy_summary.summary.reviewed_export_count,
    ).toBe(29);
    expect(
      reviewManifest.unit_policy_summary.summary.unit_evidence_required_count,
    ).toBe(27);
    expect(
      reviewManifest.unit_policy_summary.summary.unit_evidence_present_count,
    ).toBe(26);
    expect(
      reviewManifest.unit_policy_summary.summary.conversion_performed_count,
    ).toBe(0);
    expect(reviewManifest.unit_policy_summary.covered_export_ids).toEqual([
      "project_storage_audit",
      "project_validation_preflight",
      "secret_private_library_boundary_review",
      "security_threat_model_review",
      "editor_contract_review",
      "missing_data_warning_blocking_review",
      "rule_completeness_review",
      "accessibility_usability_baseline_review",
      "design_authoring_comparison_workspace",
      "validation_release_evidence_review",
      "result_envelope",
      "stress_neutral_csv_json_package",
      "headless_runner_envelope",
      "adapter_framework_envelope",
      "local_fea_handoff_package",
      "external_prover_boundary_metadata",
      "review_geometry_export",
      "conservative_pcf_export",
      "caepipe_mbf_export",
      "caepipe_external_run_evidence",
      "export_adapter_sdk_registry",
      "native_json_package",
      "report_packet",
      "report_protected_content_lint",
      "handoff_package",
      "operation_review_ledger",
    ]);
    expect(
      reviewManifest.unit_policy_summary.unit_evidence_matrix.find(
        (item: { export_id: string }) =>
          item.export_id === "project_storage_audit",
      ).unit_evidence_status,
    ).toBe("covered_by_target_panel_or_export_packet");
    expect(
      reviewManifest.unit_policy_summary.unit_evidence_matrix.find(
        (item: { export_id: string }) =>
          item.export_id === "project_validation_preflight",
      ).unit_evidence_status,
    ).toBe("covered_by_target_panel_or_export_packet");
    expect(
      reviewManifest.unit_policy_summary.unit_evidence_matrix.find(
        (item: { export_id: string }) =>
          item.export_id === "agent_proposal_review",
      ).unit_evidence_status,
    ).toBe("pending_source_export_packet");
    expect(
      reviewManifest.unit_policy_summary.unit_evidence_matrix.find(
        (item: { export_id: string }) =>
          item.export_id === "secret_private_library_boundary_review",
      ).unit_evidence_status,
    ).toBe("covered_by_target_panel_or_export_packet");
    expect(
      reviewManifest.unit_policy_summary.unit_evidence_matrix.find(
        (item: { export_id: string }) =>
          item.export_id === "security_threat_model_review",
      ).unit_evidence_status,
    ).toBe("covered_by_target_panel_or_export_packet");
    expect(
      reviewManifest.unit_policy_summary.unit_evidence_matrix.find(
        (item: { export_id: string }) => item.export_id === "result_envelope",
      ).unit_evidence_status,
    ).toBe("covered_by_target_panel_or_export_packet");
    expect(
      reviewManifest.unit_policy_summary.unit_evidence_matrix.find(
        (item: { export_id: string }) =>
          item.export_id === "rule_completeness_review",
      ).unit_evidence_status,
    ).toBe("covered_by_target_panel_or_export_packet");
    expect(
      reviewManifest.unit_policy_summary.unit_evidence_matrix.find(
        (item: { export_id: string }) =>
          item.export_id === "accessibility_usability_baseline_review",
      ).unit_evidence_status,
    ).toBe("covered_by_target_panel_or_export_packet");
    expect(
      reviewManifest.unit_policy_summary.unit_evidence_matrix.find(
        (item: { export_id: string }) =>
          item.export_id === "design_authoring_comparison_workspace",
      ).unit_evidence_status,
    ).toBe("covered_by_target_panel_or_export_packet");
    expect(
      reviewManifest.unit_policy_summary.unit_evidence_matrix.find(
        (item: { export_id: string }) =>
          item.export_id === "validation_release_evidence_review",
      ).unit_evidence_status,
    ).toBe("covered_by_target_panel_or_export_packet");
    expect(
      reviewManifest.unit_policy_summary.unit_evidence_matrix.find(
        (item: { export_id: string }) =>
          item.export_id === "telemetry_boundary_review",
      ).unit_evidence_status,
    ).toBe("not_unit_bearing_metadata_or_boundary_review");
    expect(
      reviewManifest.unit_policy_summary.unit_evidence_matrix.find(
        (item: { export_id: string }) =>
          item.export_id === "telemetry_boundary_review",
      ).unit_boundary_reason,
    ).toBe(
      "telemetry_boundary_review_records_disabled_policy_metadata_only_without_quantities_units_dimensions_or_target_conversion",
    );
    expect(
      reviewManifest.unit_policy_summary.unit_evidence_matrix.find(
        (item: { export_id: string }) =>
          item.export_id === "build_package_readiness",
      ).unit_evidence_status,
    ).toBe("not_unit_bearing_metadata_or_boundary_review");
    expect(
      reviewManifest.unit_policy_summary.unit_evidence_matrix.find(
        (item: { export_id: string }) =>
          item.export_id === "build_package_readiness",
      ).unit_boundary_reason,
    ).toBe(
      "build_package_readiness_records_script_shell_and_release_decision_metadata_only_without_quantities_units_dimensions_or_target_conversion",
    );
    expect(
      reviewManifest.unit_policy_summary.not_unit_bearing_export_ids,
    ).toEqual(["telemetry_boundary_review", "build_package_readiness"]);
    expect(
      reviewManifest.unit_policy_summary.unit_evidence_matrix.find(
        (item: { export_id: string }) =>
          item.export_id === "external_prover_boundary_metadata",
      ).unit_evidence_status,
    ).toBe("covered_by_target_panel_or_export_packet");
    expect(
      reviewManifest.exports.map(
        (item: { export_id: string }) => item.export_id,
      ),
    ).toEqual([
      "project_storage_audit",
      "project_validation_preflight",
      "telemetry_boundary_review",
      "secret_private_library_boundary_review",
      "security_threat_model_review",
      "editor_contract_review",
      "missing_data_warning_blocking_review",
      "rule_completeness_review",
      "agent_proposal_review",
      "accessibility_usability_baseline_review",
      "design_authoring_comparison_workspace",
      "build_package_readiness",
      "validation_release_evidence_review",
      "result_envelope",
      "stress_neutral_csv_json_package",
      "headless_runner_envelope",
      "adapter_framework_envelope",
      "local_fea_handoff_package",
      "external_prover_boundary_metadata",
      "review_geometry_export",
      "conservative_pcf_export",
      "caepipe_mbf_export",
      "caepipe_external_run_evidence",
      "export_adapter_sdk_registry",
      "native_json_package",
      "report_packet",
      "report_protected_content_lint",
      "handoff_package",
      "operation_review_ledger",
    ]);
    expect(
      reviewManifest.exports.find(
        (item: { export_id: string }) =>
          item.export_id === "project_storage_audit",
      ).storage_mode,
    ).toBe("not_persisted_this_session");
    expect(
      reviewManifest.exports.find(
        (item: { export_id: string }) =>
          item.export_id === "project_validation_preflight",
      ).round_trip_status,
    ).toBe("semantic_categories_declared");
    const ruleCompletenessExport = reviewManifest.exports.find(
      (item: { export_id: string }) =>
        item.export_id === "rule_completeness_review",
    );
    expect(ruleCompletenessExport.document_kind).toBe(
      "openpipestress.technical_preview.rule_completeness_review",
    );
    expect(ruleCompletenessExport.deliverable_refs).toContain("DEL-06-03");
    expect(ruleCompletenessExport.deliverable_refs).toContain("DEL-02-02");
    expect(ruleCompletenessExport.unit_evidence_required).toBe(true);
    expect(ruleCompletenessExport.private_payload_included).toBe(false);
    expect(ruleCompletenessExport.protected_content_included).toBe(false);
    const agentProposalExport = reviewManifest.exports.find(
      (item: { export_id: string }) =>
        item.export_id === "agent_proposal_review",
    );
    expect(agentProposalExport.document_kind).toBe(
      "openpipestress.technical_preview.agent_proposal_review",
    );
    expect(agentProposalExport.deliverable_refs).toContain("DEL-16-04");
    expect(agentProposalExport.deliverable_refs).toContain("DEL-02-02");
    expect(agentProposalExport.readiness).toBe("pending_agent_proposal");
    expect(agentProposalExport.unit_validation_status).toBe("not generated");
    expect(agentProposalExport.unit_evidence_required).toBe(true);
    expect(agentProposalExport.private_payload_included).toBe(false);
    expect(agentProposalExport.protected_content_included).toBe(false);
    const telemetryExport = reviewManifest.exports.find(
      (item: { export_id: string }) =>
        item.export_id === "telemetry_boundary_review",
    );
    expect(telemetryExport.document_kind).toBe(
      "openpipestress.technical_preview.telemetry_boundary_review",
    );
    expect(telemetryExport.deliverable_refs).toContain("DEL-12-03");
    expect(telemetryExport.config_resolution).toBe(
      "absent_or_preview_config_resolves_disabled",
    );
    expect(telemetryExport.telemetry_enabled).toBe(false);
    expect(telemetryExport.explicit_opt_in).toBe(false);
    expect(telemetryExport.allowlist_approved).toBe(false);
    expect(telemetryExport.attempted_event_count).toBe(4);
    expect(telemetryExport.allowed_event_count).toBe(0);
    expect(telemetryExport.blocked_event_count).toBe(4);
    expect(telemetryExport.payload_constructed).toBe(false);
    expect(telemetryExport.network_transport_initialized).toBe(false);
    expect(telemetryExport.endpoint_initialized).toBe(false);
    expect(telemetryExport.vendor_initialized).toBe(false);
    expect(telemetryExport.telemetry_persistence_initialized).toBe(false);
    expect(telemetryExport.unit_evidence_required).toBe(false);
    expect(telemetryExport.unit_boundary_classification).toBe(
      "not_unit_bearing_metadata_or_boundary_review",
    );
    expect(telemetryExport.unit_boundary_reason).toBe(
      "telemetry_boundary_review_records_disabled_policy_metadata_only_without_quantities_units_dimensions_or_target_conversion",
    );
    expect(telemetryExport.default_units_inferred).toBe(false);
    expect(telemetryExport.conversion_performed).toBe(false);
    expect(telemetryExport.security_certification_claim).toBe(false);
    const secretPrivateExport = reviewManifest.exports.find(
      (item: { export_id: string }) =>
        item.export_id === "secret_private_library_boundary_review",
    );
    expect(secretPrivateExport.document_kind).toBe(
      "openpipestress.technical_preview.secret_private_library_boundary_review",
    );
    expect(secretPrivateExport.deliverable_refs).toContain("DEL-12-04");
    expect(secretPrivateExport.reference_count).toBe(4);
    expect(secretPrivateExport.private_library_count).toBe(2);
    expect(secretPrivateExport.private_path_count).toBe(1);
    expect(secretPrivateExport.credential_reference_count).toBe(1);
    expect(secretPrivateExport.public_fixture_block_count).toBe(4);
    expect(secretPrivateExport.public_report_metadata_only_count).toBe(4);
    expect(secretPrivateExport.local_private_metadata_only_count).toBe(4);
    expect(secretPrivateExport.exact_secret_provider).toBe("TBD");
    expect(secretPrivateExport.encrypted_storage_default).toBe("TBD");
    expect(secretPrivateExport.direct_sql_access).toBe(false);
    expect(secretPrivateExport.storage_bypass_requested).toBe(false);
    expect(secretPrivateExport.cloud_or_network_reference).toBe(false);
    expect(secretPrivateExport.secret_material_present).toBe(false);
    expect(secretPrivateExport.metadata_only).toBe(true);
    expect(secretPrivateExport.unit_evidence_required).toBe(true);
    expect(secretPrivateExport.unit_policy_ref).toBe(
      "unit-policy:secret-private-library-metadata-only-preview",
    );
    expect(secretPrivateExport.explicit_unit_metadata_required).toBe(true);
    expect(secretPrivateExport.unit_payload_included).toBe(false);
    expect(secretPrivateExport.conversion_performed).toBe(false);
    expect(secretPrivateExport.security_certification_claim).toBe(false);
    const securityThreatModelExport = reviewManifest.exports.find(
      (item: { export_id: string }) =>
        item.export_id === "security_threat_model_review",
    );
    expect(securityThreatModelExport.document_kind).toBe(
      "openpipestress.technical_preview.security_threat_model_review",
    );
    expect(securityThreatModelExport.deliverable_refs).toContain("DEL-12-05");
    expect(securityThreatModelExport.threat_count).toBe(19);
    expect(securityThreatModelExport.high_risk_count).toBe(14);
    expect(securityThreatModelExport.medium_high_risk_count).toBe(5);
    expect(securityThreatModelExport.trust_boundary_count).toBe(10);
    expect(securityThreatModelExport.asset_class_count).toBe(10);
    expect(securityThreatModelExport.export_workflow_count).toBe(6);
    expect(securityThreatModelExport.open_decision_count).toBe(14);
    expect(securityThreatModelExport.tbd_decision_count).toBe(13);
    expect(securityThreatModelExport.local_first).toBe(true);
    expect(securityThreatModelExport.telemetry_default_off).toBe(true);
    expect(securityThreatModelExport.no_bypass_controls_present).toBe(true);
    expect(securityThreatModelExport.unit_evidence_required).toBe(true);
    expect(securityThreatModelExport.unit_policy_ref).toBe(
      "unit-policy-evidence:security-threat-model-no-bypass",
    );
    expect(securityThreatModelExport.conversion_performed).toBe(false);
    expect(securityThreatModelExport.direct_sql_access).toBe(false);
    expect(securityThreatModelExport.security_certification_claim).toBe(false);
    const editorContractExport = reviewManifest.exports.find(
      (item: { export_id: string }) =>
        item.export_id === "editor_contract_review",
    );
    expect(editorContractExport.document_kind).toBe(
      "openpipestress.technical_preview.editor_contract_review",
    );
    expect(editorContractExport.deliverable_refs).toContain("DEL-07-03");
    expect(editorContractExport.deliverable_refs).toContain("DEL-07-01");
    expect(editorContractExport.deliverable_refs).toContain("DEL-16-01");
    expect(editorContractExport.source_refs).toContain(
      "apps/desktop/src/features/viewport/PipeViewport.tsx",
    );
    expect(editorContractExport.editor_count).toBe(4);
    expect(editorContractExport.editor_surface_count).toBe(7);
    expect(editorContractExport.queued_intent_count).toBe(1);
    expect(editorContractExport.ready_editor_count).toBe(2);
    expect(editorContractExport.blocked_editor_count).toBe(2);
    expect(editorContractExport.diagnostic_count).toBe(4);
    expect(editorContractExport.rule_pack_reference_status).toBe(
      "private_reference_only_missing_required_inputs",
    );
    expect(editorContractExport.private_library_payload_status).toBe(
      "reference_slots_only_no_private_payload",
    );
    expect(editorContractExport.direct_model_mutation_allowed).toBe(false);
    expect(editorContractExport.accepted_model_state_mutated).toBe(false);
    expect(editorContractExport.unit_evidence_required).toBe(true);
    expect(editorContractExport.unit_policy_ref).toBe(
      "DEL-02-02:unit_bearing_values_require_explicit_unit_metadata",
    );
    expect(editorContractExport.conversion_performed).toBe(false);
    const missingDataExport = reviewManifest.exports.find(
      (item: { export_id: string }) =>
        item.export_id === "missing_data_warning_blocking_review",
    );
    expect(missingDataExport.document_kind).toBe(
      "openpipestress.technical_preview.missing_data_warning_blocking_review",
    );
    expect(missingDataExport.deliverable_refs).toContain("DEL-07-04");
    expect(missingDataExport.warning_class_count).toBe(6);
    expect(missingDataExport.active_warning_count).toBe(5);
    expect(missingDataExport.solve_blocking_count).toBe(0);
    expect(missingDataExport.rule_check_blocking_count).toBe(2);
    expect(missingDataExport.mechanics_solve_blocked).toBe(false);
    expect(missingDataExport.rule_check_blocked).toBe(true);
    expect(missingDataExport.mechanics_results_reviewable).toBe(true);
    expect(missingDataExport.mechanics_results_qualified_by_rule_inputs).toBe(
      true,
    );
    expect(missingDataExport.silent_defaults_used).toBe(false);
    expect(missingDataExport.auto_fill_missing_data).toBe(false);
    expect(missingDataExport.unit_evidence_required).toBe(true);
    expect(missingDataExport.unit_policy_ref).toBe(
      "unit-input-policy-evidence:missing-data-warning-blocking-review",
    );
    expect(missingDataExport.conversion_performed).toBe(false);
    expect(missingDataExport.assistive_text_fields_available).toBe(true);
    expect(missingDataExport.color_only_signaling_allowed).toBe(false);
    const accessibilityBaselineExport = reviewManifest.exports.find(
      (item: { export_id: string }) =>
        item.export_id === "accessibility_usability_baseline_review",
    );
    expect(accessibilityBaselineExport.document_kind).toBe(
      "openpipestress.technical_preview.accessibility_usability_baseline_review",
    );
    expect(accessibilityBaselineExport.deliverable_refs).toContain("DEL-07-06");
    expect(accessibilityBaselineExport.source_surface_count).toBe(6);
    expect(accessibilityBaselineExport.total_finding_count).toBe(69);
    expect(accessibilityBaselineExport.pass_count).toBe(57);
    expect(accessibilityBaselineExport.warning_count).toBe(10);
    expect(accessibilityBaselineExport.fail_count).toBe(1);
    expect(accessibilityBaselineExport.blocking_count).toBe(1);
    expect(accessibilityBaselineExport.keyboard_path_finding_count).toBe(4);
    expect(accessibilityBaselineExport.focus_order_finding_count).toBe(3);
    expect(accessibilityBaselineExport.accessibility_target_status).toBe(
      "TBD_by_human_project_authority",
    );
    expect(accessibilityBaselineExport.desktop_runtime_evaluation).toBe(
      "not_performed",
    );
    expect(
      accessibilityBaselineExport.software_makes_accessibility_conformance_claim,
    ).toBe(false);
    expect(
      accessibilityBaselineExport.color_only_status_signaling_allowed,
    ).toBe(false);
    expect(accessibilityBaselineExport.unit_evidence_required).toBe(true);
    expect(accessibilityBaselineExport.unit_policy_ref).toBe(
      "unit-visibility-evidence:accessibility-baseline-preview",
    );
    expect(accessibilityBaselineExport.default_units_inferred).toBe(false);
    expect(accessibilityBaselineExport.conversion_performed).toBe(false);
    const designWorkspaceExport = reviewManifest.exports.find(
      (item: { export_id: string }) =>
        item.export_id === "design_authoring_comparison_workspace",
    );
    expect(designWorkspaceExport.document_kind).toBe(
      "openpipestress.technical_preview.design_authoring_comparison_workspace",
    );
    expect(designWorkspaceExport.deliverable_refs).toContain("DEL-07-08");
    expect(designWorkspaceExport.deliverable_refs).toContain("DEL-14-02");
    expect(designWorkspaceExport.deliverable_refs).toContain("DEL-16-04");
    expect(designWorkspaceExport.current_design_knowledge_record_count).toBe(3);
    expect(designWorkspaceExport.current_model_state_count).toBe(1);
    expect(designWorkspaceExport.current_analysis_run_count).toBe(1);
    expect(designWorkspaceExport.current_result_row_count).toBe(830);
    expect(designWorkspaceExport.current_comparison_pair_count).toBe(261);
    expect(designWorkspaceExport.operation_review_record_count).toBe(1);
    expect(designWorkspaceExport.core_design_knowledge_record_count).toBe(2);
    expect(designWorkspaceExport.core_constraint_warning_count).toBe(1);
    expect(designWorkspaceExport.core_model_state_count).toBe(2);
    expect(designWorkspaceExport.core_analysis_run_count).toBe(2);
    expect(designWorkspaceExport.core_graphical_overlay_count).toBe(5);
    expect(designWorkspaceExport.unit_evidence_required).toBe(true);
    expect(designWorkspaceExport.unit_policy_ref).toBe(
      "unit-policy-evidence:design-workspace-preview",
    );
    expect(designWorkspaceExport.default_units_inferred).toBe(false);
    expect(designWorkspaceExport.conversion_performed).toBe(false);
    expect(designWorkspaceExport.workspace_mutates_accepted_model_state).toBe(
      false,
    );
    expect(
      designWorkspaceExport.accepted_operation_requires_explicit_user_acceptance_record,
    ).toBe(true);
    expect(designWorkspaceExport.private_payload_included).toBe(false);
    expect(designWorkspaceExport.protected_content_included).toBe(false);
    expect(designWorkspaceExport.release_or_professional_claim).toBe(false);
    const buildReadinessExport = reviewManifest.exports.find(
      (item: { export_id: string }) =>
        item.export_id === "build_package_readiness",
    );
    expect(buildReadinessExport.document_kind).toBe(
      "openpipestress.technical_preview.build_package_readiness",
    );
    expect(buildReadinessExport.deliverable_refs).toContain("DEL-10-04");
    expect(buildReadinessExport.root_script_count).toBe(4);
    expect(buildReadinessExport.desktop_script_count).toBe(4);
    expect(buildReadinessExport.readiness_profile_count).toBe(5);
    expect(buildReadinessExport.tauri_shell_status).toBe("present");
    expect(buildReadinessExport.tauri_bundle_active).toBe(false);
    expect(buildReadinessExport.provider_neutral_readiness_tool).toBe(true);
    expect(buildReadinessExport.ci_provider).toBe("TBD");
    expect(buildReadinessExport.release_matrix).toBe("TBD");
    expect(buildReadinessExport.signing_status).toBe("TBD");
    expect(buildReadinessExport.release_publication_authorized).toBe(false);
    expect(buildReadinessExport.installer_or_binary_generated).toBe(false);
    expect(buildReadinessExport.unit_evidence_required).toBe(false);
    expect(buildReadinessExport.unit_boundary_classification).toBe(
      "not_unit_bearing_metadata_or_boundary_review",
    );
    expect(buildReadinessExport.unit_boundary_reason).toBe(
      "build_package_readiness_records_script_shell_and_release_decision_metadata_only_without_quantities_units_dimensions_or_target_conversion",
    );
    expect(buildReadinessExport.default_units_inferred).toBe(false);
    expect(buildReadinessExport.conversion_performed).toBe(false);
    const validationEvidenceExport = reviewManifest.exports.find(
      (item: { export_id: string }) =>
        item.export_id === "validation_release_evidence_review",
    );
    expect(validationEvidenceExport.document_kind).toBe(
      "openpipestress.technical_preview.validation_release_evidence_review",
    );
    expect(validationEvidenceExport.deliverable_refs).toContain("DEL-09-04");
    expect(validationEvidenceExport.deliverable_refs).toContain("DEL-09-05");
    expect(validationEvidenceExport.deliverable_refs).toContain("DEL-10-04");
    expect(validationEvidenceExport.manual_section_count).toBe(10);
    expect(validationEvidenceExport.evidence_area_count).toBe(8);
    expect(validationEvidenceExport.release_profile_count).toBe(5);
    expect(validationEvidenceExport.required_release_path_count).toBe(7);
    expect(validationEvidenceExport.skeleton_check_count).toBe(2);
    expect(validationEvidenceExport.validation_manual_open_decision_count).toBe(
      5,
    );
    expect(validationEvidenceExport.release_gate_family_count).toBe(5);
    expect(validationEvidenceExport.release_authorization_status).toBe(
      "not_authorized",
    );
    expect(validationEvidenceExport.final_threshold_policy).toBe("TBD");
    expect(validationEvidenceExport.browser_panel_runs_tool).toBe(false);
    expect(validationEvidenceExport.dry_run_default).toBe(true);
    expect(validationEvidenceExport.unit_evidence_required).toBe(true);
    expect(validationEvidenceExport.unit_policy_ref).toBe(
      "unit-policy-evidence:validation-release-evidence-review",
    );
    expect(validationEvidenceExport.default_units_inferred).toBe(false);
    expect(validationEvidenceExport.conversion_performed).toBe(false);
    expect(validationEvidenceExport.private_payload_included).toBe(false);
    expect(validationEvidenceExport.protected_content_included).toBe(false);
    expect(validationEvidenceExport.release_or_professional_claim).toBe(false);
    const stressNeutralExport = reviewManifest.exports.find(
      (item: { export_id: string }) =>
        item.export_id === "stress_neutral_csv_json_package",
    );
    expect(stressNeutralExport.document_kind).toBe(
      "openpipestress.technical_preview.stress_neutral_csv_json_package",
    );
    expect(stressNeutralExport.deliverable_refs).toContain("DEL-17-06");
    expect(stressNeutralExport.deliverable_refs).toContain("DEL-08-04");
    expect(stressNeutralExport.deliverable_refs).toContain("DEL-14-02");
    expect(stressNeutralExport.result_ref_count).toBe(830);
    expect(stressNeutralExport.csv_column_count).toBe(11);
    expect(stressNeutralExport.stable_id_count).toBe(830);
    expect(stressNeutralExport.member_roles).toContain("loss_report");
    expect(stressNeutralExport.comparison_semantics).toBe(
      "diagnostic_export_only_no_pass_fail",
    );
    expect(stressNeutralExport.canonical_package_hash_status).toBe(
      "computed_local_preview_sha256_by_target_panel",
    );
    expect(stressNeutralExport.vendor_format_claim).toBe(false);
    expect(stressNeutralExport.solver_validation_claim).toBe(false);
    expect(stressNeutralExport.code_compliance_claim).toBe(false);
    expect(stressNeutralExport.professional_reliance_claim).toBe(false);
    const headlessExport = reviewManifest.exports.find(
      (item: { export_id: string }) =>
        item.export_id === "headless_runner_envelope",
    );
    expect(headlessExport.document_kind).toBe(
      "openpipestress.technical_preview.headless_runner_envelope",
    );
    expect(headlessExport.deliverable_refs).toContain("DEL-10-05");
    expect(headlessExport.deliverable_refs).toContain("DEL-08-04");
    expect(headlessExport.result_ref_count).toBe(830);
    expect(headlessExport.runner_job_state).toBe("COMPLETED");
    expect(headlessExport.final_cli_command_syntax).toContain(
      "openpipestress-runner <solve|validate-input|export-results|run-benchmark|run-regression>",
    );
    expect(headlessExport.process_invocation).toBe("single_foreground_local_process");
    expect(headlessExport.network_access).toBe("none");
    expect(headlessExport.filesystem_mutation_policy).toBe(
      "stdout_default_explicit_output_path_only",
    );
    const adapterExport = reviewManifest.exports.find(
      (item: { export_id: string }) =>
        item.export_id === "adapter_framework_envelope",
    );
    expect(adapterExport.document_kind).toBe(
      "openpipestress.technical_preview.adapter_framework_envelope",
    );
    expect(adapterExport.deliverable_refs).toContain("DEL-10-02");
    expect(adapterExport.deliverable_refs).toContain("DEL-02-04");
    expect(adapterExport.capabilities).toContain("validate_payload");
    expect(adapterExport.parse_status).toBe("not_parsed_by_framework");
    expect(adapterExport.result_ref_count).toBe(830);
    expect(adapterExport.external_format_list).toBe("TBD");
    expect(adapterExport.public_transport_protocol).toBe("TBD");
    expect(adapterExport.plugin_runtime).toBe("TBD");
    const localFeaExport = reviewManifest.exports.find(
      (item: { export_id: string }) =>
        item.export_id === "local_fea_handoff_package",
    );
    expect(localFeaExport.document_kind).toBe(
      "openpipestress.technical_preview.local_fea_handoff_package",
    );
    expect(localFeaExport.deliverable_refs).toContain("DEL-10-03");
    expect(localFeaExport.contract_kind).toBe(
      "schema_first_local_fea_handoff_contract",
    );
    expect(localFeaExport.global_analysis_role).toBe(
      "primary_global_centerline_frame_model",
    );
    expect(localFeaExport.local_analysis_role).toBe(
      "optional_specialized_shell_solid_handoff",
    );
    expect(localFeaExport.package_kind).toBe("local_shell_solid_fea_handoff");
    expect(localFeaExport.selected_region_basis).toBe("diagnostic_suggested");
    expect(localFeaExport.guidance_label_count).toBe(4);
    expect(localFeaExport.unsupported_behavior_flag_count).toBe(5);
    expect(localFeaExport.diagnostic_count).toBe(4);
    expect(localFeaExport.selected_result_ref_count).toBe(5);
    expect(localFeaExport.concrete_export_format).toBe("TBD");
    expect(localFeaExport.target_solver_adapter).toBe("TBD");
    expect(localFeaExport.mesh_generation).toBe("TBD");
    expect(localFeaExport.external_solver_invocation).toBe("TBD");
    expect(localFeaExport.transfer_method_label).toBe("result_reference_only");
    expect(localFeaExport.human_review_required).toBe(true);
    expect(localFeaExport.software_makes_compliance_claim).toBe(false);
    expect(localFeaExport.software_makes_approval_claim).toBe(false);
    const externalProverExport = reviewManifest.exports.find(
      (item: { export_id: string }) =>
        item.export_id === "external_prover_boundary_metadata",
    );
    expect(externalProverExport.document_kind).toBe(
      "openpipestress.technical_preview.external_prover_boundary_metadata",
    );
    expect(externalProverExport.deliverable_refs).toContain("DEL-15-04");
    expect(externalProverExport.metadata_contract_status).toBe(
      "non_authoritative_workflow_metadata",
    );
    expect(externalProverExport.external_reference_count).toBe(1);
    expect(externalProverExport.unsupported_target_flag_count).toBe(3);
    expect(externalProverExport.external_tool_invoked).toBe(false);
    expect(externalProverExport.commercial_result_payload_ingested).toBe(false);
    expect(
      externalProverExport.software_creates_external_validation_record,
    ).toBe(false);
    expect(reviewManifest.unit_policy_summary.covered_export_ids).toContain(
      "external_prover_boundary_metadata",
    );
    const reviewGeometryExport = reviewManifest.exports.find(
      (item: { export_id: string }) =>
        item.export_id === "review_geometry_export",
    );
    expect(reviewGeometryExport.document_kind).toBe(
      "openpipestress.technical_preview.review_geometry_export",
    );
    expect(reviewGeometryExport.deliverable_refs).toContain("DEL-17-08");
    expect(reviewGeometryExport.review_geometry_status).toBe(
      "visual_review_geometry_only",
    );
    expect(reviewGeometryExport.geometry_format).toBe("glTF_2_0_json_preview");
    expect(reviewGeometryExport.node_count).toBe(5);
    expect(reviewGeometryExport.pipe_segment_count).toBe(4);
    expect(reviewGeometryExport.line_primitive_count).toBe(4);
    expect(reviewGeometryExport.stable_id_count).toBe(26);
    expect(reviewGeometryExport.sidecar_id_map_required).toBe(true);
    expect(reviewGeometryExport.glb_binary_writer_status).toBe("TBD");
    expect(reviewGeometryExport.viewer_compatibility).toBe("TBD");
    expect(reviewGeometryExport.solver_geometry_equivalence_claim).toBe(false);
    expect(reviewGeometryExport.professional_validation_claim).toBe(false);
    expect(reviewGeometryExport.target_compatibility_claim).toBe(false);
    const pcfExportRecord = reviewManifest.exports.find(
      (item: { export_id: string }) =>
        item.export_id === "conservative_pcf_export",
    );
    expect(pcfExportRecord.document_kind).toBe(
      "openpipestress.technical_preview.conservative_pcf_export_package",
    );
    expect(pcfExportRecord.deliverable_refs).toContain("DEL-17-07");
    expect(pcfExportRecord.target_family).toBe("pcf");
    expect(pcfExportRecord.profile_id).toBe("ops.pcf.conservative_subset");
    expect(pcfExportRecord.target_profile_version_basis).toBe("TBD");
    expect(pcfExportRecord.pipe_segment_count).toBe(4);
    expect(pcfExportRecord.node_count).toBe(5);
    expect(pcfExportRecord.stable_id_count).toBe(4);
    expect(pcfExportRecord.loss_category_count).toBe(6);
    expect(pcfExportRecord.validation_status).toBe(
      "blocked_missing_explicit_pcf_target_fields",
    );
    expect(pcfExportRecord.canonical_package_hash_status).toBe(
      "computed_local_preview_sha256_by_target_panel",
    );
    expect(pcfExportRecord.target_compatibility_claim).toBe(false);
    expect(pcfExportRecord.solver_validation_claim).toBe(false);
    expect(pcfExportRecord.code_compliance_claim).toBe(false);
    expect(pcfExportRecord.professional_reliance_claim).toBe(false);
    const caepipeMbfExportRecord = reviewManifest.exports.find(
      (item: { export_id: string }) => item.export_id === "caepipe_mbf_export",
    );
    expect(caepipeMbfExportRecord.document_kind).toBe(
      "openpipestress.technical_preview.caepipe_mbf_export_package",
    );
    expect(caepipeMbfExportRecord.deliverable_refs).toContain("DEL-17-04");
    expect(caepipeMbfExportRecord.target_family).toBe("caepipe_mbf");
    expect(caepipeMbfExportRecord.profile_id).toBe("ops.caepipe_mbf.smoke_tbd");
    expect(caepipeMbfExportRecord.target_version_basis).toBe("TBD-17-01-001");
    expect(caepipeMbfExportRecord.record_subset_basis).toBe("TBD-17-01-002");
    expect(caepipeMbfExportRecord.pipe_element_count).toBe(4);
    expect(caepipeMbfExportRecord.node_count).toBe(5);
    expect(caepipeMbfExportRecord.support_count).toBe(7);
    expect(caepipeMbfExportRecord.load_case_count).toBe(2);
    expect(caepipeMbfExportRecord.stable_id_count).toBe(18);
    expect(caepipeMbfExportRecord.loss_category_count).toBe(6);
    expect(caepipeMbfExportRecord.validation_status).toBe("boundary_checked");
    expect(caepipeMbfExportRecord.external_execution_policy).toBe(
      "not_invoked_by_this_package",
    );
    expect(caepipeMbfExportRecord.caepipe_compatibility_claim).toBe(false);
    expect(caepipeMbfExportRecord.solver_validation_claim).toBe(false);
    expect(caepipeMbfExportRecord.code_compliance_claim).toBe(false);
    expect(caepipeMbfExportRecord.professional_reliance_claim).toBe(false);
    const caepipeExternalRecord = reviewManifest.exports.find(
      (item: { export_id: string }) =>
        item.export_id === "caepipe_external_run_evidence",
    );
    expect(caepipeExternalRecord.document_kind).toBe(
      "openpipestress.technical_preview.caepipe_external_run_package",
    );
    expect(caepipeExternalRecord.deliverable_refs).toContain("DEL-17-05");
    expect(caepipeExternalRecord.deliverable_refs).toContain("DEL-17-04");
    expect(caepipeExternalRecord.package_status).toBe("parser_only_evidence");
    expect(caepipeExternalRecord.mbf_package_ref).toContain("del-17-04");
    expect(caepipeExternalRecord.executable_path_state).toBe("absent");
    expect(caepipeExternalRecord.invocation_profile).toBe(
      "TBD-17-05-invocation-profile",
    );
    expect(caepipeExternalRecord.external_tool_invoked).toBe(false);
    expect(caepipeExternalRecord.parser_status).toBe(
      "parsed_parser_only_fixture",
    );
    expect(caepipeExternalRecord.parser_row_count).toBe(3);
    expect(caepipeExternalRecord.parser_section_count).toBe(2);
    expect(caepipeExternalRecord.parser_correlation_status).toBe(
      "canonical_id_map",
    );
    expect(caepipeExternalRecord.validation_status).toBe("boundary_checked");
    expect(caepipeExternalRecord.caepipe_compatibility_claim).toBe(false);
    expect(caepipeExternalRecord.solver_validation_claim).toBe(false);
    expect(caepipeExternalRecord.code_compliance_claim).toBe(false);
    expect(caepipeExternalRecord.professional_reliance_claim).toBe(false);
    const exportAdapterSdkRecord = reviewManifest.exports.find(
      (item: { export_id: string }) =>
        item.export_id === "export_adapter_sdk_registry",
    );
    expect(exportAdapterSdkRecord.document_kind).toBe(
      "openpipestress.technical_preview.export_adapter_sdk_registry",
    );
    expect(exportAdapterSdkRecord.deliverable_refs).toContain("DEL-17-09");
    expect(exportAdapterSdkRecord.deliverable_refs).toContain("DEL-17-02");
    expect(exportAdapterSdkRecord.deliverable_refs).toContain("DEL-02-04");
    expect(exportAdapterSdkRecord.deliverable_refs).toContain("DEL-10-01");
    expect(exportAdapterSdkRecord.deliverable_refs).toContain("DEL-10-02");
    expect(exportAdapterSdkRecord.registry_id).toBe(
      "ops.export_adapter_sdk.registry_preview",
    );
    expect(exportAdapterSdkRecord.sdk_contract_status).toBe(
      "contract_level_preview",
    );
    expect(exportAdapterSdkRecord.target_registry_count).toBe(5);
    expect(exportAdapterSdkRecord.preview_panel_available_count).toBe(5);
    expect(exportAdapterSdkRecord.capability_count).toBe(4);
    expect(exportAdapterSdkRecord.validation_status).toBe("boundary_checked");
    expect(exportAdapterSdkRecord.plugin_runtime).toBe("TBD");
    expect(exportAdapterSdkRecord.permission_taxonomy).toBe("TBD");
    expect(exportAdapterSdkRecord.source_basis_admitted_target_count).toBe(0);
    expect(exportAdapterSdkRecord.target_support_claim_count).toBe(0);
    expect(exportAdapterSdkRecord.target_compatibility_claim).toBe(false);
    expect(exportAdapterSdkRecord.solver_validation_claim).toBe(false);
    expect(exportAdapterSdkRecord.code_compliance_claim).toBe(false);
    expect(exportAdapterSdkRecord.professional_reliance_claim).toBe(false);
    const lintExport = reviewManifest.exports.find(
      (item: { export_id: string }) =>
        item.export_id === "report_protected_content_lint",
    );
    expect(lintExport.document_kind).toBe(
      "openpipestress.technical_preview.report_protected_content_lint_run",
    );
    expect(lintExport.deliverable_refs).toContain("DEL-08-05");
    expect(lintExport.target_count).toBe(22);
    expect(lintExport.finding_count).toBe(0);
    expect(lintExport.blocking_finding_count).toBe(0);
    expect(lintExport.clean_scan_is_clearance).toBe(false);
    expect(lintExport.unit_evidence_required).toBe(true);
    expect(lintExport.unit_policy_ref).toBe(
      "unit-policy-evidence:report-lint-public-surfaces",
    );
    expect(lintExport.unit_policy).toBe(
      "lint_targets_include_public_unit_policy_and_conversion_witness_surfaces",
    );
    expect(lintExport.unit_policy_target_count).toBe(44);
    expect(lintExport.conversion_witness_target_count).toBe(2);
    expect(lintExport.lint_performs_conversion).toBe(false);
    expect(lintExport.lint_asserts_target_format_compatibility).toBe(false);
    expect(lintExport.default_units_inferred).toBe(false);
    expect(lintExport.conversion_performed).toBe(false);
    expect(
      reviewManifest.unit_policy_summary.unit_evidence_matrix.find(
        (item: { export_id: string }) =>
          item.export_id === "report_protected_content_lint",
      ).unit_evidence_status,
    ).toBe("covered_by_target_panel_or_export_packet");
    expect(
      reviewManifest.exports.every(
        (item: {
          private_payload_included: boolean;
          protected_content_included: boolean;
          release_or_professional_claim: boolean;
        }) =>
          item.private_payload_included === false &&
          item.protected_content_included === false &&
          item.release_or_professional_claim === false,
      ),
    ).toBe(true);
    expect(reviewManifest.redaction_policy.local_private_export_allowed).toBe(
      true,
    );
    expect(reviewManifest.redaction_policy.protected_content_blocked).toBe(
      true,
    );
    expect(reviewManifest.redaction_policy.source_model_mutated).toBe(false);
    expect(reviewManifest.data_boundary.private_data_policy).toBe(
      "no_private_project_data",
    );
    expect(
      reviewManifest.professional_boundary.software_makes_compliance_claim,
    ).toBe(false);
    // Heavy full-<App/> Three.js render: inherit the 30s global testTimeout
    // (vite.config.ts); a tight per-test override flaked under DEC-025 sweep load.
  });

  it("round trips local create, save, and open project controls without external file copies", async () => {
    render(<App />);

    expect(await screen.findByText("OpenPipeStress")).toBeInTheDocument();
    const controls = screen.getByLabelText("Local project controls");
    const storageAudit = await screen.findByLabelText("Project storage audit");
    const projectValidation = await screen.findByLabelText(
      "Project validation preflight",
    );
    expect(
      screen.getByTestId("local-project-review-context").textContent,
    ).toContain("0 pending operations; applied_operations=0");
    expect(
      within(storageAudit).getByTestId("model-hash-persistence").textContent,
    ).toContain("persisted_model_hashes=0");
    expect(
      within(storageAudit).getByTestId("model-hash-persistence").textContent,
    ).toContain("persisted_model_hash_ref=not_persisted");
    expect(
      within(storageAudit).getByTestId("model-hash-integrity").textContent,
    ).toContain("no open-verification has run this session");

    const tree = screen.getByLabelText("Model tree");
    fireEvent.click(
      within(tree).getByRole("button", {
        name: /Invented carbon-steel-like material/i,
      }),
    );
    const inspector = screen.getByLabelText("Property inspector");
    const intentPanel = within(inspector).getByLabelText(
      "Editor operation intent",
    );
    fireEvent.change(within(intentPanel).getByTestId("editor-intent-field"), {
      target: { value: "elastic_modulus.value" },
    });
    fireEvent.change(within(intentPanel).getByTestId("editor-intent-value"), {
      target: { value: "210000000000" },
    });
    fireEvent.click(within(intentPanel).getByTestId("queue-editor-intent"));
    expect(
      screen.getByTestId("local-project-review-context").textContent,
    ).toContain("1 pending operation; applied_operations=0");

    fireEvent.click(
      within(controls).getByRole("button", { name: /Create local/i }),
    );
    expect(
      await screen.findByTestId("local-project-message"),
    ).toHaveTextContent("without external file copies");
    expect(
      screen.getByTestId("local-project-review-context").textContent,
    ).toContain("1 pending operation; applied_operations=0");
    expect(
      within(storageAudit).getByTestId("project-storage-summary").textContent,
    ).toContain("operation=create");
    expect(
      within(storageAudit).getByTestId("project-storage-summary").textContent,
    ).toContain("pending operations=1");
    expect(
      within(storageAudit).getByTestId("project-storage-snapshot").textContent,
    ).toContain("browser_memory_preview");
    expect(
      within(storageAudit).getByTestId("project-storage-snapshot").textContent,
    ).toContain("persisted_editor_intents=1");

    fireEvent.click(
      within(controls).getByRole("button", { name: /Save local/i }),
    );
    expect(
      await screen.findByTestId("local-project-message"),
    ).toHaveTextContent("without external file copies");
    expect(
      screen.getByTestId("local-project-review-context").textContent,
    ).toContain("1 pending operation; applied_operations=0");
    await waitFor(() =>
      expect(
        within(storageAudit).getByTestId("project-storage-summary").textContent,
      ).toContain("operation=save"),
    );

    fireEvent.click(
      within(controls).getByRole("button", { name: /Open local/i }),
    );
    expect(
      await screen.findByTestId("local-project-message"),
    ).toHaveTextContent("Opened local browser-preview project snapshot.");
    expect(
      screen.getByTestId("local-project-review-context").textContent,
    ).toContain("1 pending operation; applied_operations=0");
    expect(
      within(controls).getByText("Invented Utility Loop Preview"),
    ).toBeInTheDocument();
    await waitFor(() =>
      expect(
        within(storageAudit).getByTestId("project-storage-summary").textContent,
      ).toContain("operation=open"),
    );
    expect(
      within(storageAudit).getByTestId("project-storage-snapshot").textContent,
    ).toContain("persisted_editor_intents=1");
    expect(
      within(storageAudit).getByTestId("project-storage-local-boundary")
        .textContent,
    ).toContain("network=false");
    expect(
      within(storageAudit).getByTestId("project-storage-local-boundary")
        .textContent,
    ).toContain("telemetry=false");
    expect(
      within(storageAudit).getByTestId("project-storage-snapshot").textContent,
    ).toContain("copied_external_files=false");
    expect(
      within(storageAudit).getByTestId("project-storage-payload-boundary")
        .textContent,
    ).toContain("protected content=false");
    await waitFor(() =>
      expect(
        within(storageAudit).getByTestId("model-hash-integrity").textContent,
      ).toContain("integrity_status=verified_match"),
    );
    expect(
      within(storageAudit).getByTestId("model-hash-integrity").textContent,
    ).toContain("persisted_value=sha256:");
    expect(
      within(storageAudit).getByTestId("model-hash-integrity").textContent,
    ).toContain("recomputed_value=sha256:");
    expect(
      within(storageAudit).getByTestId("model-hash-integrity").textContent,
    ).toContain("verification_basis=recomputed_on_open_from_restored_model");
    expect(
      within(storageAudit).getByTestId("model-hash-persistence").textContent,
    ).toContain("persisted_model_hashes=1");
    expect(
      within(storageAudit).getByTestId("model-hash-persistence").textContent,
    ).toContain("persisted_model_hash_ref=sha256:");
    expect(
      within(storageAudit).getByTestId("project-storage-unit-round-trip")
        .textContent,
    ).toContain("status=unit_metadata_preserved_in_local_project_envelope");
    expect(
      within(storageAudit).getByTestId("project-storage-unit-round-trip")
        .textContent,
    ).toContain("project.units.length=m");
    expect(
      within(storageAudit).getByTestId("project-storage-unit-round-trip")
        .textContent,
    ).toContain("conversion=false");
    const auditHref =
      within(storageAudit)
        .getByTestId("project-storage-export-link")
        .getAttribute("href") ?? "";
    const auditPacket = JSON.parse(
      decodeURIComponent(auditHref.split(",", 2)[1]),
    );
    expect(auditPacket.document_kind).toBe(
      "openpipestress.technical_preview.local_project_persistence_audit",
    );
    expect(auditPacket.deliverable_refs).toContain("DEL-02-05");
    expect(auditPacket.deliverable_refs).toContain("DEL-12-02");
    expect(auditPacket.scope_items).toContain("SOW-050");
    expect(auditPacket.scope_items).toContain("SOW-040");
    expect(auditPacket.project_ref).toBe("project:invented-loop-01");
    expect(auditPacket.summary.last_operation).toBe("open");
    expect(auditPacket.summary.storage_engine).toBe("Browser memory preview");
    expect(auditPacket.summary.storage_mode).toBe("browser_memory_preview");
    expect(auditPacket.summary.pending_operation_count).toBe(1);
    expect(auditPacket.summary.editor_intent_count).toBe(1);
    expect(auditPacket.summary.persisted_editor_intent_count).toBe(1);
    expect(auditPacket.summary.unit_round_trip_status).toBe(
      "unit_metadata_preserved_in_local_project_envelope",
    );
    expect(
      auditPacket.summary.unit_round_trip_checked_ref_count,
    ).toBeGreaterThan(0);
    expect(auditPacket.summary.unit_round_trip_signature).toContain(
      "project.units.length=m",
    );
    expect(auditPacket.unit_policy_evidence.unit_round_trip_status).toBe(
      "unit_metadata_preserved_in_local_project_envelope",
    );
    expect(
      auditPacket.unit_policy_evidence.unit_round_trip_checked_ref_count,
    ).toBeGreaterThan(0);
    expect(
      auditPacket.unit_policy_evidence.unit_round_trip_signature,
    ).toContain("project.units.length=m");
    expect(auditPacket.unit_policy_evidence.conversion_policy).toBe(
      "project_storage_audit_reports_persistence_unit_metadata_no_conversion",
    );
    expect(auditPacket.unit_policy_evidence.conversion_performed).toBe(false);
    expect(auditPacket.summary.applied_operation_count).toBe(0);
    expect(auditPacket.summary.accepted_model_state_mutated).toBe(false);
    expect(auditPacket.summary.network_required).toBe(false);
    expect(auditPacket.summary.daemon_required).toBe(false);
    expect(auditPacket.summary.telemetry_enabled).toBe(false);
    expect(auditPacket.summary.copied_external_files).toBe(false);
    expect(auditPacket.project_summary.storage_mode).toBe(
      "browser_memory_preview",
    );
    expect(auditPacket.project_summary.editor_intent_count).toBe(1);
    expect(auditPacket.project_summary.unit_round_trip_signature).toBe(
      auditPacket.summary.unit_round_trip_signature,
    );
    expect(auditPacket.project_summary.copied_external_files).toBe(false);
    expect(auditPacket.editor_intent_refs).toContain(
      "op:editor-intent-material:invented-carbon-steel-elastic_modulus.value",
    );
    expect(auditPacket.editor_operation_statuses).toContain("not_applied");
    expect(auditPacket.boundary.local_only_project_store).toBe(true);
    expect(auditPacket.boundary.repository_default_private_write).toBe(false);
    expect(auditPacket.boundary.external_file_copy_performed).toBe(false);
    expect(auditPacket.boundary.accepted_model_state_mutated).toBe(false);
    expect(auditPacket.private_payload_included).toBe(false);
    expect(auditPacket.protected_content_included).toBe(false);
    expect(auditPacket.release_or_professional_claim).toBe(false);
    expect(
      auditPacket.professional_boundary.software_makes_compliance_claim,
    ).toBe(false);
    expect(
      within(projectValidation).getByTestId("project-validation-summary")
        .textContent,
    ).toContain("validation=preview_current");
    expect(
      within(projectValidation).getByTestId("project-validation-summary")
        .textContent,
    ).toContain(
      "migration=browser_memory_snapshot_no_sql_store_migrations_applicable",
    );
    expect(
      within(projectValidation).getByTestId(
        "project-validation-store-migration",
      ).textContent,
    ).toContain("framework=browser_memory_preview_no_sqlite_migration_ledger");
    expect(
      within(projectValidation).getByTestId(
        "project-validation-store-migration",
      ).textContent,
    ).toContain("applied_on_open=0");
    expect(
      within(projectValidation).getByTestId(
        "project-validation-store-migration",
      ).textContent,
    ).toContain(
      "status=browser_memory_snapshot_no_sql_store_migrations_applicable",
    );
    expect(
      within(projectValidation).getByTestId("project-validation-operations")
        .textContent,
    ).toContain("pending operations=1");
    expect(
      within(projectValidation).getByTestId("project-validation-operations")
        .textContent,
    ).toContain("persisted editor intents=1");
    expect(
      within(projectValidation).getByTestId("project-validation-model-hash")
        .textContent,
    ).toContain("model_hash=model_hash_verified_on_open");
    expect(
      within(projectValidation).getByTestId("project-validation-model-hash")
        .textContent,
    ).toContain("persisted_model_hashes=1");
    expect(
      within(projectValidation).getByTestId("project-validation-model-hash")
        .textContent,
    ).toContain("persisted_model_hash_ref=sha256:");
    expect(
      within(projectValidation).getByTestId("project-validation-model-hash")
        .textContent,
    ).toContain("integrity=verified_match");
    expect(
      within(projectValidation).getByTestId("project-validation-round-trip")
        .textContent,
    ).toContain("reproducibility=model_hash_verified_on_open");
    expect(
      within(projectValidation).getByTestId(
        "project-validation-unit-round-trip",
      ).textContent,
    ).toContain("status=unit_metadata_preserved_in_local_project_envelope");
    expect(
      within(projectValidation).getByTestId(
        "project-validation-unit-round-trip",
      ).textContent,
    ).toContain("project.units.length=m");
    expect(
      within(projectValidation).getByTestId("project-validation-unit-policy")
        .textContent,
    ).toContain("round_trip=unit_metadata_preserved_in_local_project_envelope");
    expect(
      within(projectValidation).getByTestId("project-validation-unit-policy")
        .textContent,
    ).toContain("conversion=false");
    await waitFor(() =>
      expect(
        within(projectValidation).getByTestId(
          "project-validation-envelope-hash",
        ).textContent,
      ).toContain("envelope_hash=project_envelope_hash_verified_on_open"),
    );
    expect(
      within(projectValidation).getByTestId("project-validation-envelope-hash")
        .textContent,
    ).toContain("persisted_envelope_hashes=1");
    expect(
      within(projectValidation).getByTestId("project-validation-envelope-hash")
        .textContent,
    ).toContain("persisted_envelope_hash_ref=sha256:");
    expect(
      within(projectValidation).getByTestId("project-validation-envelope-hash")
        .textContent,
    ).toContain("integrity=verified_match");
    expect(
      within(projectValidation).getByTestId("project-validation-boundary")
        .textContent,
    ).toContain("private/protected payload=false");
    const validationHref =
      within(projectValidation)
        .getByTestId("project-validation-export-link")
        .getAttribute("href") ?? "";
    const validationPacket = JSON.parse(
      decodeURIComponent(validationHref.split(",", 2)[1]),
    );
    expect(validationPacket.document_kind).toBe(
      "openpipestress.technical_preview.project_validation_preflight",
    );
    expect(validationPacket.project_ref).toBe("project:invented-loop-01");
    expect(validationPacket.summary.validation_status).toBe("preview_current");
    expect(validationPacket.summary.last_operation).toBe("open");
    expect(validationPacket.summary.storage_mode).toBe(
      "browser_memory_preview",
    );
    expect(validationPacket.summary.migration_status).toBe(
      "browser_memory_snapshot_no_sql_store_migrations_applicable",
    );
    expect(validationPacket.summary.unit_round_trip_status).toBe(
      "unit_metadata_preserved_in_local_project_envelope",
    );
    expect(validationPacket.unit_round_trip_evidence.signature).toBe(
      validationPacket.summary.unit_round_trip_signature,
    );
    expect(
      validationPacket.unit_round_trip_evidence.checked_ref_count,
    ).toBeGreaterThan(0);
    expect(validationPacket.unit_policy_evidence.unit_round_trip_status).toBe(
      "unit_metadata_preserved_in_local_project_envelope",
    );
    expect(
      validationPacket.unit_policy_evidence.unit_round_trip_signature,
    ).toBe(validationPacket.summary.unit_round_trip_signature);
    expect(
      validationPacket.unit_policy_evidence.unit_round_trip_checked_ref_count,
    ).toBeGreaterThan(0);
    expect(validationPacket.unit_policy_evidence.conversion_policy).toBe(
      "project_validation_records_unit_round_trip_metadata_without_conversion",
    );
    expect(validationPacket.unit_policy_evidence.conversion_performed).toBe(
      false,
    );
    expect(validationPacket.store_migration.migration_framework).toBe(
      "browser_memory_preview_no_sqlite_migration_ledger",
    );
    expect(validationPacket.store_migration.evidence_source).toBe(
      "local_project_summary",
    );
    expect(validationPacket.store_migration.migration_scope).toBe(
      "local_store_schema_ddl_only_model_document_schema_tracked_separately_per_dec_019",
    );
    // DEC-033: the bundled preview model is a 0.1.0-era document; under the
    // supported 0.2.0 version it carries migrated in-memory evidence and the
    // browser preview keeps stored bytes unchanged (no ledger exists here).
    expect(validationPacket.model_document_migration.status).toBe("migrated");
    expect(validationPacket.model_document_migration.evidence_source).toBe(
      "persistence_operation_envelope",
    );
    expect(validationPacket.model_document_migration.persistence_state).toBe(
      "in_memory_only_not_yet_saved",
    );
    expect(validationPacket.model_document_migration.ledger_record_count).toBe(
      0,
    );
    expect(validationPacket.store_migration.migrations_applied_on_open).toEqual(
      [],
    );
    expect(
      validationPacket.store_migration.destructive_migration_performed,
    ).toBe(false);
    expect(validationPacket.project_summary.store_schema_version).toBe(0);
    expect(validationPacket.project_summary.store_schema_target_version).toBe(
      0,
    );
    expect(validationPacket.summary.pending_operation_count).toBe(1);
    expect(validationPacket.summary.editor_intent_count).toBe(1);
    expect(validationPacket.summary.persisted_editor_intent_count).toBe(1);
    expect(validationPacket.project_summary.editor_intent_count).toBe(1);
    expect(validationPacket.summary.accepted_model_state_mutated).toBe(false);
    expect(
      validationPacket.service_operations.find(
        (operation: { operation: string }) =>
          operation.operation === "validate",
      ).operation_status,
    ).toBe("preflight_generated_preview_current");
    expect(
      validationPacket.service_operations.find(
        (operation: { operation: string }) =>
          operation.operation === "version_check",
      ).operation_status,
    ).toBe("stale");
    expect(
      validationPacket.service_operations.find(
        (operation: { operation: string }) => operation.operation === "migrate",
      ).operation_status,
    ).toBe("browser_memory_snapshot_no_sql_store_migrations_applicable");
    expect(
      validationPacket.service_operations.find(
        (operation: { operation: string }) => operation.operation === "migrate",
      ).result_available,
    ).toBe(true);
    expect(
      validationPacket.diagnostics.map(
        (diagnosticItem: { code: string }) => diagnosticItem.code,
      ),
    ).toContain("PROJECT-VALIDATION-STORE-MIGRATION-LEDGER-REVIEW-ONLY");
    expect(
      validationPacket.diagnostics.map(
        (diagnosticItem: { code: string }) => diagnosticItem.code,
      ),
    ).not.toContain("PROJECT-VALIDATION-STORE-MIGRATED-ON-OPEN");
    expect(
      validationPacket.diagnostics.find(
        (diagnosticItem: { code: string }) =>
          diagnosticItem.code ===
          "PROJECT-VALIDATION-STORE-MIGRATION-LEDGER-REVIEW-ONLY",
      ).class,
    ).toBe("MIGRATION");
    expect(validationPacket.summary.model_hash_status).toBe(
      "model_hash_verified_on_open",
    );
    expect(validationPacket.summary.persisted_model_hash_count).toBe(1);
    expect(validationPacket.summary.persisted_model_hash_ref).toMatch(
      /^sha256:[0-9a-f]{64}$/,
    );
    expect(validationPacket.summary.model_hash_integrity_status).toBe(
      "verified_match",
    );
    expect(validationPacket.validation_profile.hash_service_status).toBe(
      "canonical_model_hash_service_available_model_payload_scope",
    );
    expect(validationPacket.model_hash_integrity.integrity_status).toBe(
      "verified_match",
    );
    expect(validationPacket.model_hash_integrity.verification_basis).toBe(
      "recomputed_on_open_from_restored_model",
    );
    expect(validationPacket.model_hash_integrity.persisted_value).toBe(
      validationPacket.model_hash_integrity.recomputed_value,
    );
    expect(validationPacket.summary.project_envelope_hash_status).toBe(
      "project_envelope_hash_verified_on_open",
    );
    expect(validationPacket.summary.persisted_project_envelope_hash_count).toBe(
      1,
    );
    expect(
      validationPacket.summary.persisted_project_envelope_hash_ref,
    ).toMatch(/^sha256:[0-9a-f]{64}$/);
    expect(
      validationPacket.summary.project_envelope_hash_integrity_status,
    ).toBe("verified_match");
    expect(
      validationPacket.validation_profile.project_envelope_hash_status,
    ).toBe("project_envelope_hash_verified_on_open");
    expect(validationPacket.project_envelope_hash.payload_scope).toBe(
      "project_envelope_payload",
    );
    expect(validationPacket.project_envelope_hash.payload_excludes).toBe(
      "storage_summary_and_envelope_hash_carrier_fields",
    );
    expect(validationPacket.project_envelope_hash.value).toMatch(
      /^sha256:[0-9a-f]{64}$/,
    );
    expect(
      validationPacket.project_envelope_hash_integrity.integrity_status,
    ).toBe("verified_match");
    expect(
      validationPacket.project_envelope_hash_integrity.verification_basis,
    ).toBe("recomputed_on_open_from_restored_envelope_payload");
    expect(
      validationPacket.project_envelope_hash_integrity.persisted_value,
    ).toBe(validationPacket.project_envelope_hash_integrity.recomputed_value);
    expect(
      validationPacket.project_summary.persisted_project_envelope_hash_count,
    ).toBe(1);
    expect(
      validationPacket.diagnostics.map(
        (diagnosticEntry: { code: string }) => diagnosticEntry.code,
      ),
    ).toContain("PROJECT-VALIDATION-ENVELOPE-HASH-REVIEW-ONLY");
    expect(
      validationPacket.diagnostics.map(
        (diagnosticEntry: { code: string }) => diagnosticEntry.code,
      ),
    ).not.toContain("PROJECT-VALIDATION-ENVELOPE-HASH-MISMATCH");
    expect(
      validationPacket.diagnostics.find(
        (diagnosticEntry: { code: string }) =>
          diagnosticEntry.code ===
          "PROJECT-VALIDATION-ENVELOPE-HASH-REVIEW-ONLY",
      ).class,
    ).toBe("REPRODUCIBILITY");
    expect(
      validationPacket.round_trip_manifest.categories.find(
        (category: { category: string }) =>
          category.category === "reproducibility_metadata",
      ).semantic_equality_status,
    ).toBe("model_hash_verified_on_open");
    expect(
      validationPacket.diagnostics.map(
        (diagnosticEntry: { code: string }) => diagnosticEntry.code,
      ),
    ).not.toContain("PROJECT-VALIDATION-MODEL-HASH-MISMATCH");
    expect(
      validationPacket.round_trip_manifest.categories.find(
        (category: { category: string }) =>
          category.category === "unit_metadata",
      ).semantic_equality_status,
    ).toBe("ready_for_preview_round_trip");
    expect(
      validationPacket.round_trip_manifest.categories.find(
        (category: { category: string }) =>
          category.category === "rule_pack_references",
      ).semantic_equality_status,
    ).toBe("not_present_in_invented_fixture");
    expect(validationPacket.boundary.local_only_project_store).toBe(true);
    expect(validationPacket.boundary.repository_default_private_write).toBe(
      false,
    );
    expect(validationPacket.boundary.external_file_copy_performed).toBe(false);
    expect(validationPacket.boundary.accepted_model_state_mutated).toBe(false);
    expect(validationPacket.private_payload_included).toBe(false);
    expect(validationPacket.protected_content_included).toBe(false);
    expect(validationPacket.release_or_professional_claim).toBe(false);
    expect(
      validationPacket.professional_boundary.software_makes_compliance_claim,
    ).toBe(false);

    const exportReview = await screen.findByLabelText("Export safety review");
    const reviewHref =
      within(exportReview)
        .getByTestId("export-review-link")
        .getAttribute("href") ?? "";
    const reviewManifest = JSON.parse(
      decodeURIComponent(reviewHref.split(",", 2)[1]),
    );
    expect(
      reviewManifest.exports.find(
        (item: { export_id: string }) =>
          item.export_id === "project_storage_audit",
      ).persisted_editor_intent_count,
    ).toBe(1);
    expect(
      reviewManifest.exports.find(
        (item: { export_id: string }) =>
          item.export_id === "project_validation_preflight",
      ).persisted_editor_intent_count,
    ).toBe(1);

    fireEvent.click(
      within(tree).getByRole("button", {
        name: /Invented carbon-steel-like material/i,
      }),
    );
    expect(
      within(screen.getByLabelText("Editor operation intent")).getByTestId(
        "editor-intent-queue",
      ).textContent,
    ).toContain("editor-intent-1");
  });

  it("creates a blank local model document as the active authoring target", async () => {
    render(<App />);

    expect(await screen.findByText("OpenPipeStress")).toBeInTheDocument();
    const controls = screen.getByLabelText("Local project controls");
    fireEvent.click(
      within(controls).getByRole("button", { name: /New blank/i }),
    );

    await waitFor(() =>
      expect(screen.getByTestId("local-project-message")).toHaveTextContent(
        "Created blank local model document without fixture entities or external file copies.",
      ),
    );
    expect(within(controls).getByText("Blank Local Model")).toBeInTheDocument();
    expect(screen.getByTestId("status-pill-mechanics").textContent).toContain(
      "MODEL_INCOMPLETE",
    );
    expect(screen.getByTestId("status-pill-rule-check").textContent).toContain(
      "RULE_INPUTS_INCOMPLETE",
    );
    expect(
      screen.getByTestId("status-pill-professional").textContent,
    ).toContain("HUMAN_REVIEW_REQUIRED");
    fireEvent.click(screen.getByTestId("audit-drawer-toggle"));
    const auditDrawer = await screen.findByTestId("audit-boundary-drawer");
    expect(auditDrawer.textContent).toContain(
      "blank_user_created_local_document_no_bundled_engineering_values",
    );
    expect(auditDrawer.textContent).toContain(
      "local_user_document_not_committed_to_repository",
    );
    expect(screen.getByTestId("app-menu-bar")).toBeInTheDocument();
    expect(screen.getByTestId("workspace-dock").className).toContain(
      "collapsed",
    );
    fireEvent.click(screen.getByTestId("menu-view"));
    fireEvent.click(screen.getByTestId("menu-item-view.section.loads"));
    expect(screen.getByTestId("workspace-section-loads").className).toBe(
      "workspace-dock-section",
    );

    const tree = screen.getByLabelText("Model tree");
    expect(
      within(tree).getByTestId("model-tree-filter-summary").textContent,
    ).toContain("1 of 1 model entities visible");
    expect(
      within(tree).getByRole("button", { name: /Blank Local Model/i }),
    ).toBeInTheDocument();
    const inspector = screen.getByLabelText("Property inspector");
    expect(within(inspector).getByText("Project ID")).toBeInTheDocument();
    expect(inspector.textContent).toContain("project:blank-local-");
    expect(inspector.textContent).toContain("MODEL_INCOMPLETE");

    const storageAudit = await screen.findByLabelText("Project storage audit");
    expect(
      within(storageAudit).getByTestId("project-storage-summary").textContent,
    ).toContain("operation=create_blank");
    expect(
      within(storageAudit).getByTestId("project-storage-summary").textContent,
    ).toContain("pending operations=0");
    expect(
      within(storageAudit).getByTestId("project-storage-snapshot").textContent,
    ).toContain("browser_memory_preview");
    expect(
      within(storageAudit).getByTestId("project-storage-snapshot").textContent,
    ).toContain("persisted_mechanics_results=0");
    expect(
      within(storageAudit).getByTestId("project-storage-local-boundary")
        .textContent,
    ).toContain("network=false");
    const storagePacket = JSON.parse(
      decodeURIComponent(
        (
          within(storageAudit)
            .getByTestId("project-storage-export-link")
            .getAttribute("href") ?? ""
        ).split(",", 2)[1],
      ),
    );
    expect(storagePacket.summary.last_operation).toBe("create_blank");
    expect(storagePacket.project_ref).toMatch(/^project:blank-local-/);
    expect(storagePacket.summary.persisted_mechanics_result_count).toBe(0);
    expect(storagePacket.boundary.repository_default_private_write).toBe(false);
    expect(storagePacket.private_payload_included).toBe(false);
    expect(storagePacket.protected_content_included).toBe(false);
    expect(storagePacket.release_or_professional_claim).toBe(false);

    const projectValidation = await screen.findByLabelText(
      "Project validation preflight",
    );
    expect(
      within(projectValidation).getByTestId("project-validation-summary")
        .textContent,
    ).toContain("validation=preview_current");
    expect(
      within(projectValidation).getByTestId(
        "project-validation-model-document-migration",
      ).textContent,
    ).toContain("status=current");
    expect(
      within(projectValidation).getByTestId("project-validation-operations")
        .textContent,
    ).toContain("persisted mechanics results=0");
    await waitFor(() =>
      expect(
        within(projectValidation).getByTestId("project-validation-model-hash")
          .textContent,
      ).toContain("persisted_model_hashes=1"),
    );

    fireEvent.click(
      within(controls).getByRole("button", { name: /List local/i }),
    );
    await waitFor(() =>
      expect(screen.getByTestId("local-project-message")).toHaveTextContent(
        "Listed 1 local project snapshot from the local store index.",
      ),
    );
    expect(screen.getByTestId("project-index-picker").textContent).toContain(
      "Blank Local Model",
    );

    fireEvent.click(
      screen.getByRole("button", { name: /Run mechanics preview/i }),
    );
    await waitFor(
      () => {
        expect(screen.getByTestId("solve-job-summary").textContent).toContain(
          "state=completed",
        );
        expect(screen.getByTestId("solve-job-summary").textContent).toContain(
          "result_rows=0",
        );
        expect(
          screen.getByTestId("status-pill-mechanics").textContent,
        ).toContain("MODEL_INCOMPLETE");
      },
      { timeout: 10000 },
    );
    fireEvent.click(screen.getByTestId("issues-drawer-toggle"));
    const diagnostics = await screen.findByLabelText("Diagnostics");
    expect(diagnostics.textContent).toContain(
      "BROWSER_SOLVE_BACKEND_REQUIRED_FOR_EDITED_MODEL",
    );
    const resultsPanel = await screen.findByTestId("results-panel");
    expect(
      within(resultsPanel).getByTestId("result-filter-summary").textContent,
    ).toContain("0 of 0 results match filter");
    const reportPanel = await screen.findByTestId("report-panel");
    expect(
      within(reportPanel).getByTestId("report-packet-body").textContent,
    ).toContain("MODEL INCOMPLETE");
    expect(
      within(reportPanel).getByTestId("report-export-summary").textContent,
    ).toContain("0 refs");
    // Heavy full-<App/> Three.js render: inherit the 30s global testTimeout
    // (vite.config.ts); a tight per-test override flaked under DEC-025 sweep load.
  });

  it("round trips review-only proposal operations through local save and open", async () => {
    render(<App />);

    const runButton = await screen.findByRole("button", {
      name: /Run mechanics preview/i,
    });
    fireEvent.click(runButton);

    expect(
      await screen.findByTestId(
        "result-group-displacement",
        {},
        { timeout: 10000 },
      ),
    ).toBeInTheDocument();
    const results = await screen.findByLabelText("Results");
    fireEvent.change(within(results).getByTestId("result-filter-input"), {
      target: { value: "torsional-shear" },
    });
    fireEvent.click(
      within(results).getByTestId(
        "result-row-result:stress:pipe-P-120:end-j:torsional-shear",
      ),
    );

    fireEvent.click(
      screen.getByRole("button", { name: /Generate review proposal/i }),
    );
    const proposalPanel = await screen.findByLabelText("Agentic proposal");
    expect(
      await within(proposalPanel).findByText(
        "proposal:physics-diagnostic-review",
      ),
    ).toBeInTheDocument();
    expect(
      within(proposalPanel).getByTestId("proposal-operation-summary")
        .textContent,
    ).toContain("op:review-computed-diagnostic");
    expect(
      within(proposalPanel).getByTestId("selected-review-target").textContent,
    ).toContain("result: result:stress:pipe-P-120:end-j:torsional-shear");
    expect(
      screen.getByTestId("local-project-review-context").textContent,
    ).toContain(
      "1 pending operation; applied_operations=0; editor_intents=0; agent_proposals=1",
    );

    const controls = screen.getByLabelText("Local project controls");
    fireEvent.click(
      within(controls).getByRole("button", { name: /Save local/i }),
    );
    await waitFor(() =>
      expect(screen.getByTestId("local-project-message")).toHaveTextContent(
        "Saved local browser-preview project snapshot without external file copies.",
      ),
    );

    const storageAudit = await screen.findByLabelText("Project storage audit");
    expect(
      within(storageAudit).getByTestId("project-storage-summary").textContent,
    ).toContain("operation=save");
    expect(
      within(storageAudit).getByTestId("project-storage-summary").textContent,
    ).toContain("pending operations=1");
    expect(
      within(storageAudit).getByTestId("project-storage-snapshot").textContent,
    ).toContain("persisted_proposals=1");
    expect(
      within(storageAudit).getByTestId("project-storage-snapshot").textContent,
    ).toContain("persisted_review_targets=1");
    expect(
      within(storageAudit).getByTestId("project-storage-snapshot").textContent,
    ).toContain(
      "persisted_review_target_ref=result: result:stress:pipe-P-120:end-j:torsional-shear",
    );
    expect(
      within(storageAudit).getByTestId("project-storage-snapshot").textContent,
    ).toContain("persisted_mechanics_results=1");
    expect(
      within(storageAudit).getByTestId("project-storage-snapshot").textContent,
    ).toContain("persisted_analysis_runs=1");
    expect(
      within(storageAudit).getByTestId("project-storage-snapshot").textContent,
    ).toContain("persisted_analysis_run_ref=run:preview-linear-static-001");
    const savedStorageHref =
      within(storageAudit)
        .getByTestId("project-storage-export-link")
        .getAttribute("href") ?? "";
    const savedStoragePacket = JSON.parse(
      decodeURIComponent(savedStorageHref.split(",", 2)[1]),
    );
    expect(savedStoragePacket.summary.pending_operation_count).toBe(1);
    expect(savedStoragePacket.summary.proposal_operation_count).toBe(1);
    expect(savedStoragePacket.summary.persisted_proposal_count).toBe(1);
    expect(
      savedStoragePacket.summary.persisted_selected_review_target_count,
    ).toBe(1);
    expect(
      savedStoragePacket.summary.persisted_selected_review_target_ref,
    ).toBe("result: result:stress:pipe-P-120:end-j:torsional-shear");
    expect(savedStoragePacket.project_summary.proposal_count).toBe(1);
    expect(
      savedStoragePacket.project_summary.selected_review_target_count,
    ).toBe(1);
    expect(savedStoragePacket.project_summary.selected_review_target_ref).toBe(
      "result: result:stress:pipe-P-120:end-j:torsional-shear",
    );
    expect(savedStoragePacket.summary.persisted_mechanics_result_count).toBe(1);
    expect(savedStoragePacket.summary.persisted_analysis_run_count).toBe(1);
    expect(savedStoragePacket.summary.persisted_analysis_run_ref).toBe(
      "run:preview-linear-static-001",
    );
    expect(
      savedStoragePacket.project_summary.persisted_mechanics_result_count,
    ).toBe(1);
    expect(
      savedStoragePacket.project_summary.persisted_analysis_run_count,
    ).toBe(1);
    expect(savedStoragePacket.project_summary.persisted_analysis_run_ref).toBe(
      "run:preview-linear-static-001",
    );
    expect(savedStoragePacket.proposal_refs).toContain(
      "proposal:physics-diagnostic-review",
    );
    expect(savedStoragePacket.summary.project_index_state).toBe(
      "not_requested",
    );
    expect(savedStoragePacket.summary.listed_project_count).toBe(0);

    fireEvent.click(
      within(controls).getByRole("button", { name: /List local/i }),
    );
    await waitFor(() =>
      expect(screen.getByTestId("local-project-message")).toHaveTextContent(
        "Listed 1 local project snapshot from the local store index.",
      ),
    );
    expect(
      within(storageAudit).getByTestId("project-storage-project-index")
        .textContent,
    ).toContain("state=listed");
    expect(
      within(storageAudit).getByTestId("project-storage-project-index")
        .textContent,
    ).toContain("listed_projects=1");
    expect(
      within(storageAudit).getByTestId("project-storage-project-index")
        .textContent,
    ).toContain("refs=project:invented-loop-01");
    const listedStorageHref =
      within(storageAudit)
        .getByTestId("project-storage-export-link")
        .getAttribute("href") ?? "";
    const listedStoragePacket = JSON.parse(
      decodeURIComponent(listedStorageHref.split(",", 2)[1]),
    );
    expect(listedStoragePacket.summary.last_operation).toBe("list");
    expect(listedStoragePacket.summary.project_index_state).toBe("listed");
    expect(listedStoragePacket.summary.listed_project_count).toBe(1);
    expect(listedStoragePacket.project_index_refs).toEqual([
      "project:invented-loop-01",
    ]);
    expect(listedStoragePacket.project_index).toHaveLength(1);
    expect(listedStoragePacket.project_index[0].project_id).toBe(
      "project:invented-loop-01",
    );
    expect(listedStoragePacket.project_index[0].project_name).toBe(
      "Invented Utility Loop Preview",
    );
    expect(listedStoragePacket.project_index[0].storage_mode).toBe(
      "browser_memory_preview",
    );
    expect(
      listedStoragePacket.project_index[0].created_at_unix,
    ).toBeGreaterThan(0);
    expect(
      listedStoragePacket.project_index[0].updated_at_unix,
    ).toBeGreaterThanOrEqual(
      listedStoragePacket.project_index[0].created_at_unix,
    );

    const projectValidation = await screen.findByLabelText(
      "Project validation preflight",
    );
    expect(
      within(projectValidation).getByTestId("project-validation-operations")
        .textContent,
    ).toContain("persisted proposals=1");
    expect(
      within(projectValidation).getByTestId("project-validation-operations")
        .textContent,
    ).toContain("persisted review targets=1");
    expect(
      within(projectValidation).getByTestId("project-validation-operations")
        .textContent,
    ).toContain(
      "persisted review target ref=result: result:stress:pipe-P-120:end-j:torsional-shear",
    );
    expect(
      within(projectValidation).getByTestId("project-validation-operations")
        .textContent,
    ).toContain("persisted mechanics results=1");
    expect(
      within(projectValidation).getByTestId("project-validation-operations")
        .textContent,
    ).toContain("persisted analysis runs=1");
    expect(
      within(projectValidation).getByTestId("project-validation-operations")
        .textContent,
    ).toContain("persisted analysis run ref=run:preview-linear-static-001");
    const savedValidationHref =
      within(projectValidation)
        .getByTestId("project-validation-export-link")
        .getAttribute("href") ?? "";
    const savedValidationPacket = JSON.parse(
      decodeURIComponent(savedValidationHref.split(",", 2)[1]),
    );
    expect(savedValidationPacket.summary.pending_operation_count).toBe(1);
    expect(savedValidationPacket.summary.proposal_operation_count).toBe(1);
    expect(savedValidationPacket.summary.persisted_proposal_count).toBe(1);
    expect(
      savedValidationPacket.summary.persisted_selected_review_target_count,
    ).toBe(1);
    expect(
      savedValidationPacket.summary.persisted_selected_review_target_ref,
    ).toBe("result: result:stress:pipe-P-120:end-j:torsional-shear");
    expect(savedValidationPacket.project_summary.proposal_count).toBe(1);
    expect(
      savedValidationPacket.project_summary.selected_review_target_count,
    ).toBe(1);
    expect(
      savedValidationPacket.project_summary.selected_review_target_ref,
    ).toBe("result: result:stress:pipe-P-120:end-j:torsional-shear");
    expect(savedValidationPacket.summary.persisted_mechanics_result_count).toBe(
      1,
    );
    expect(savedValidationPacket.summary.persisted_analysis_run_count).toBe(1);
    expect(savedValidationPacket.summary.persisted_analysis_run_ref).toBe(
      "run:preview-linear-static-001",
    );
    expect(savedValidationPacket.proposal_refs).toContain(
      "proposal:physics-diagnostic-review",
    );

    const report = await screen.findByLabelText("Report packet");
    const savedReportHref =
      within(report).getByTestId("report-export-link").getAttribute("href") ??
      "";
    const savedReportPacket = JSON.parse(
      decodeURIComponent(savedReportHref.split(",", 2)[1]),
    );
    expect(
      savedReportPacket.persistence_evidence.storage_audit
        .persisted_selected_review_target_ref,
    ).toBe("result: result:stress:pipe-P-120:end-j:torsional-shear");
    expect(
      savedReportPacket.persistence_evidence.validation_preflight
        .persisted_selected_review_target_ref,
    ).toBe("result: result:stress:pipe-P-120:end-j:torsional-shear");
    expect(
      savedReportPacket.persistence_evidence.storage_audit
        .persisted_mechanics_result_count,
    ).toBe(1);
    expect(
      savedReportPacket.persistence_evidence.storage_audit
        .persisted_analysis_run_count,
    ).toBe(1);
    expect(
      savedReportPacket.persistence_evidence.storage_audit
        .persisted_analysis_run_ref,
    ).toBe("run:preview-linear-static-001");
    expect(
      savedReportPacket.persistence_evidence.validation_preflight
        .persisted_mechanics_result_count,
    ).toBe(1);
    expect(
      savedReportPacket.persistence_evidence.validation_preflight
        .persisted_analysis_run_count,
    ).toBe(1);
    expect(
      savedReportPacket.persistence_evidence.validation_preflight
        .persisted_analysis_run_ref,
    ).toBe("run:preview-linear-static-001");

    const savedNativePackage = await screen.findByLabelText(
      "Native JSON package",
    );
    expect(
      within(savedNativePackage).getByTestId(
        "native-package-persisted-review-context",
      ).textContent,
    ).toContain("editor_intents=0");
    expect(
      within(savedNativePackage).getByTestId(
        "native-package-persisted-review-context",
      ).textContent,
    ).toContain("proposals=1");
    expect(
      within(savedNativePackage).getByTestId(
        "native-package-persisted-review-context",
      ).textContent,
    ).toContain("selected_targets=1");
    expect(
      within(savedNativePackage).getByTestId(
        "native-package-persisted-review-context",
      ).textContent,
    ).toContain(
      "selected_ref=result: result:stress:pipe-P-120:end-j:torsional-shear",
    );
    expect(
      within(savedNativePackage).getByTestId(
        "native-package-persisted-review-context",
      ).textContent,
    ).toContain("mechanics_results=1");
    expect(
      within(savedNativePackage).getByTestId(
        "native-package-persisted-review-context",
      ).textContent,
    ).toContain("analysis_runs=1");
    expect(
      within(savedNativePackage).getByTestId(
        "native-package-persisted-review-context",
      ).textContent,
    ).toContain("run_ref=run:preview-linear-static-001");
    const savedNativePackageHref =
      within(savedNativePackage)
        .getByTestId("native-package-link")
        .getAttribute("href") ?? "";
    const savedNativePackagePacket = JSON.parse(
      decodeURIComponent(savedNativePackageHref.split(",", 2)[1]),
    );
    expect(
      savedNativePackagePacket.source_project.storage_summary.proposal_count,
    ).toBe(1);
    expect(
      savedNativePackagePacket.source_project.storage_summary
        .selected_review_target_count,
    ).toBe(1);
    expect(
      savedNativePackagePacket.source_project.storage_summary
        .selected_review_target_ref,
    ).toBe("result: result:stress:pipe-P-120:end-j:torsional-shear");
    expect(
      savedNativePackagePacket.operation_review.persisted_review_context
        .proposal_count,
    ).toBe(1);
    expect(
      savedNativePackagePacket.operation_review.persisted_review_context
        .selected_review_target_count,
    ).toBe(1);
    expect(
      savedNativePackagePacket.operation_review.persisted_review_context
        .selected_review_target_ref,
    ).toBe("result: result:stress:pipe-P-120:end-j:torsional-shear");
    expect(
      savedNativePackagePacket.generation_context.persisted_proposal_count,
    ).toBe(1);
    expect(
      savedNativePackagePacket.generation_context
        .persisted_selected_review_target_count,
    ).toBe(1);
    expect(
      savedNativePackagePacket.generation_context
        .persisted_selected_review_target_ref,
    ).toBe("result: result:stress:pipe-P-120:end-j:torsional-shear");
    expect(
      savedNativePackagePacket.source_project.storage_summary
        .persisted_mechanics_result_count,
    ).toBe(1);
    expect(
      savedNativePackagePacket.source_project.storage_summary
        .persisted_analysis_run_count,
    ).toBe(1);
    expect(
      savedNativePackagePacket.source_project.storage_summary
        .persisted_analysis_run_ref,
    ).toBe("run:preview-linear-static-001");
    expect(
      savedNativePackagePacket.generation_context
        .persisted_mechanics_result_count,
    ).toBe(1);
    expect(
      savedNativePackagePacket.generation_context.persisted_analysis_run_count,
    ).toBe(1);
    expect(
      savedNativePackagePacket.generation_context.persisted_analysis_run_ref,
    ).toBe("run:preview-linear-static-001");

    fireEvent.click(
      within(controls).getByRole("button", { name: /Open local/i }),
    );
    await waitFor(() =>
      expect(screen.getByTestId("local-project-message")).toHaveTextContent(
        "Opened local browser-preview project snapshot.",
      ),
    );

    expect(screen.getByTestId("solve-job-summary").textContent).toContain(
      "state=completed",
    );
    expect(screen.getByTestId("solve-job-summary").textContent).toContain(
      "events=1",
    );
    expect(screen.getByTestId("solve-job-summary").textContent).toContain(
      "result_rows=830",
    );
    expect(screen.getByTestId("solve-job-progress").textContent).toContain(
      "restored_persisted_run_record_no_new_solve_executed",
    );
    expect(
      within(results).getByTestId(
        "result-row-result:stress:pipe-P-120:end-j:torsional-shear",
      ),
    ).toBeInTheDocument();
    fireEvent.change(within(results).getByTestId("result-filter-input"), {
      target: { value: "" },
    });
    expect(
      await screen.findByTestId("result-group-displacement"),
    ).toBeInTheDocument();
    expect(
      screen.getByTestId("local-project-review-context").textContent,
    ).toContain(
      "1 pending operation; applied_operations=0; editor_intents=0; agent_proposals=1",
    );
    expect(
      await within(proposalPanel).findByText(
        "proposal:physics-diagnostic-review",
      ),
    ).toBeInTheDocument();
    expect(
      within(proposalPanel).getByTestId("selected-review-target").textContent,
    ).toContain("result: result:stress:pipe-P-120:end-j:torsional-shear");
    expect(
      within(proposalPanel).getByTestId("proposal-affected-entities")
        .textContent,
    ).toContain("result:stress:pipe-P-120:end-j:torsional-shear");

    expect(
      within(storageAudit).getByTestId("project-storage-summary").textContent,
    ).toContain("operation=open");
    expect(
      within(storageAudit).getByTestId("project-storage-summary").textContent,
    ).toContain("pending operations=1");
    expect(
      within(storageAudit).getByTestId("project-storage-snapshot").textContent,
    ).toContain("persisted_proposals=1");
    expect(
      within(storageAudit).getByTestId("project-storage-snapshot").textContent,
    ).toContain("persisted_review_targets=1");
    expect(
      within(storageAudit).getByTestId("project-storage-snapshot").textContent,
    ).toContain(
      "persisted_review_target_ref=result: result:stress:pipe-P-120:end-j:torsional-shear",
    );
    expect(
      within(storageAudit).getByTestId("project-storage-snapshot").textContent,
    ).toContain("persisted_mechanics_results=1");
    expect(
      within(storageAudit).getByTestId("project-storage-snapshot").textContent,
    ).toContain("persisted_analysis_runs=1");
    expect(
      within(storageAudit).getByTestId("project-storage-snapshot").textContent,
    ).toContain("persisted_analysis_run_ref=run:preview-linear-static-001");
    const openedStorageHref =
      within(storageAudit)
        .getByTestId("project-storage-export-link")
        .getAttribute("href") ?? "";
    const openedStoragePacket = JSON.parse(
      decodeURIComponent(openedStorageHref.split(",", 2)[1]),
    );
    expect(openedStoragePacket.summary.last_operation).toBe("open");
    expect(openedStoragePacket.summary.proposal_operation_count).toBe(1);
    expect(openedStoragePacket.summary.persisted_proposal_count).toBe(1);
    expect(
      openedStoragePacket.summary.persisted_selected_review_target_count,
    ).toBe(1);
    expect(
      openedStoragePacket.summary.persisted_selected_review_target_ref,
    ).toBe("result: result:stress:pipe-P-120:end-j:torsional-shear");
    expect(openedStoragePacket.project_summary.proposal_count).toBe(1);
    expect(
      openedStoragePacket.project_summary.selected_review_target_count,
    ).toBe(1);
    expect(openedStoragePacket.project_summary.selected_review_target_ref).toBe(
      "result: result:stress:pipe-P-120:end-j:torsional-shear",
    );
    expect(openedStoragePacket.summary.persisted_mechanics_result_count).toBe(
      1,
    );
    expect(openedStoragePacket.summary.persisted_analysis_run_count).toBe(1);
    expect(openedStoragePacket.summary.persisted_analysis_run_ref).toBe(
      "run:preview-linear-static-001",
    );
    expect(
      openedStoragePacket.project_summary.persisted_mechanics_result_count,
    ).toBe(1);
    expect(
      openedStoragePacket.project_summary.persisted_analysis_run_count,
    ).toBe(1);
    expect(openedStoragePacket.project_summary.persisted_analysis_run_ref).toBe(
      "run:preview-linear-static-001",
    );
    expect(openedStoragePacket.proposal_refs).toContain(
      "proposal:physics-diagnostic-review",
    );
    expect(openedStoragePacket.review_operation_statuses).toContain(
      "not_applied",
    );

    expect(
      within(projectValidation).getByTestId("project-validation-operations")
        .textContent,
    ).toContain("pending operations=1");
    const openedValidationHref =
      within(projectValidation)
        .getByTestId("project-validation-export-link")
        .getAttribute("href") ?? "";
    const openedValidationPacket = JSON.parse(
      decodeURIComponent(openedValidationHref.split(",", 2)[1]),
    );
    expect(openedValidationPacket.summary.last_operation).toBe("open");
    expect(openedValidationPacket.summary.proposal_operation_count).toBe(1);
    expect(openedValidationPacket.summary.persisted_proposal_count).toBe(1);
    expect(
      openedValidationPacket.summary.persisted_selected_review_target_count,
    ).toBe(1);
    expect(
      openedValidationPacket.summary.persisted_selected_review_target_ref,
    ).toBe("result: result:stress:pipe-P-120:end-j:torsional-shear");
    expect(openedValidationPacket.project_summary.proposal_count).toBe(1);
    expect(
      openedValidationPacket.project_summary.selected_review_target_count,
    ).toBe(1);
    expect(
      openedValidationPacket.project_summary.selected_review_target_ref,
    ).toBe("result: result:stress:pipe-P-120:end-j:torsional-shear");
    expect(
      openedValidationPacket.summary.persisted_mechanics_result_count,
    ).toBe(1);
    expect(openedValidationPacket.summary.persisted_analysis_run_count).toBe(1);
    expect(openedValidationPacket.summary.persisted_analysis_run_ref).toBe(
      "run:preview-linear-static-001",
    );
    expect(openedValidationPacket.proposal_refs).toContain(
      "proposal:physics-diagnostic-review",
    );

    const openedNativePackage = await screen.findByLabelText(
      "Native JSON package",
    );
    expect(
      within(openedNativePackage).getByTestId(
        "native-package-persisted-review-context",
      ).textContent,
    ).toContain("mechanics_results=1");
    expect(
      within(openedNativePackage).getByTestId(
        "native-package-persisted-review-context",
      ).textContent,
    ).toContain("analysis_runs=1");
    expect(
      within(openedNativePackage).getByTestId(
        "native-package-persisted-review-context",
      ).textContent,
    ).toContain("run_ref=run:preview-linear-static-001");
    const openedNativePackageHref =
      within(openedNativePackage)
        .getByTestId("native-package-link")
        .getAttribute("href") ?? "";
    const openedNativePackagePacket = JSON.parse(
      decodeURIComponent(openedNativePackageHref.split(",", 2)[1]),
    );
    expect(
      openedNativePackagePacket.source_project.storage_summary
        .persisted_mechanics_result_count,
    ).toBe(1);
    expect(
      openedNativePackagePacket.source_project.storage_summary
        .persisted_analysis_run_count,
    ).toBe(1);
    expect(
      openedNativePackagePacket.source_project.storage_summary
        .persisted_analysis_run_ref,
    ).toBe("run:preview-linear-static-001");
    expect(
      openedNativePackagePacket.generation_context
        .persisted_mechanics_result_count,
    ).toBe(1);
    expect(
      openedNativePackagePacket.generation_context.persisted_analysis_run_count,
    ).toBe(1);
    expect(
      openedNativePackagePacket.generation_context.persisted_analysis_run_ref,
    ).toBe("run:preview-linear-static-001");

    const operationLedger = await screen.findByLabelText(
      "Operation review ledger",
    );
    expect(
      await within(operationLedger).findByTestId(
        "operation-ledger-export-summary",
      ),
    ).toHaveTextContent("1 review record");
    expect(
      within(operationLedger).getByTestId("operation-ledger-state-binding")
        .textContent,
    ).toContain(
      "state:project:invented-loop-01:preview; run:preview-linear-static-001",
    );
    expect(
      within(operationLedger).getByTestId(
        "operation-ledger-record-op-review-computed-diagnostic",
      ).textContent,
    ).toContain("result:stress:pipe-P-120:end-j:torsional-shear");
    const ledgerHref =
      within(operationLedger)
        .getByTestId("operation-ledger-export-link")
        .getAttribute("href") ?? "";
    const ledgerPacket = JSON.parse(
      decodeURIComponent(ledgerHref.split(",", 2)[1]),
    );
    expect(ledgerPacket.selected_review_target).toEqual({
      target_type: "result",
      id: "result:stress:pipe-P-120:end-j:torsional-shear",
    });
    expect(ledgerPacket.records[0].selected_review_target).toEqual({
      target_type: "result",
      id: "result:stress:pipe-P-120:end-j:torsional-shear",
    });

    const exportReview = await screen.findByLabelText("Export safety review");
    const openedReviewHref =
      within(exportReview)
        .getByTestId("export-review-link")
        .getAttribute("href") ?? "";
    const openedReviewManifest = JSON.parse(
      decodeURIComponent(openedReviewHref.split(",", 2)[1]),
    );
    expect(openedReviewManifest.summary.operation_record_count).toBe(1);
    expect(
      openedReviewManifest.exports.find(
        (item: { export_id: string }) =>
          item.export_id === "project_storage_audit",
      ).proposal_operation_count,
    ).toBe(1);
    expect(
      openedReviewManifest.exports.find(
        (item: { export_id: string }) =>
          item.export_id === "project_storage_audit",
      ).persisted_proposal_count,
    ).toBe(1);
    expect(
      openedReviewManifest.exports.find(
        (item: { export_id: string }) =>
          item.export_id === "project_storage_audit",
      ).persisted_selected_review_target_count,
    ).toBe(1);
    expect(
      openedReviewManifest.exports.find(
        (item: { export_id: string }) =>
          item.export_id === "project_storage_audit",
      ).persisted_selected_review_target_ref,
    ).toBe("result: result:stress:pipe-P-120:end-j:torsional-shear");
    expect(
      openedReviewManifest.exports.find(
        (item: { export_id: string }) =>
          item.export_id === "project_validation_preflight",
      ).proposal_operation_count,
    ).toBe(1);
    expect(
      openedReviewManifest.exports.find(
        (item: { export_id: string }) =>
          item.export_id === "project_validation_preflight",
      ).persisted_proposal_count,
    ).toBe(1);
    expect(
      openedReviewManifest.exports.find(
        (item: { export_id: string }) =>
          item.export_id === "project_validation_preflight",
      ).persisted_selected_review_target_count,
    ).toBe(1);
    expect(
      openedReviewManifest.exports.find(
        (item: { export_id: string }) =>
          item.export_id === "project_validation_preflight",
      ).persisted_selected_review_target_ref,
    ).toBe("result: result:stress:pipe-P-120:end-j:torsional-shear");
    expect(
      openedReviewManifest.exports.find(
        (item: { export_id: string }) =>
          item.export_id === "operation_review_ledger",
      ).readiness,
    ).toBe("available");

    expect(screen.getByTestId("project-index-picker")).toBeInTheDocument();
    fireEvent.click(
      screen.getByTestId("project-index-open-project:invented-loop-01"),
    );
    await waitFor(() =>
      expect(screen.getByTestId("local-project-message")).toHaveTextContent(
        "Opened local browser-preview project snapshot by id project:invented-loop-01.",
      ),
    );
    expect(
      within(storageAudit).getByTestId("project-storage-summary").textContent,
    ).toContain("operation=open_by_id");
    const openedByIdStorageHref =
      within(storageAudit)
        .getByTestId("project-storage-export-link")
        .getAttribute("href") ?? "";
    const openedByIdStoragePacket = JSON.parse(
      decodeURIComponent(openedByIdStorageHref.split(",", 2)[1]),
    );
    expect(openedByIdStoragePacket.summary.last_operation).toBe("open_by_id");
    expect(openedByIdStoragePacket.project_summary.project_id).toBe(
      "project:invented-loop-01",
    );
    expect(openedByIdStoragePacket.summary.persisted_proposal_count).toBe(1);
    expect(
      openedByIdStoragePacket.summary.persisted_selected_review_target_ref,
    ).toBe("result: result:stress:pipe-P-120:end-j:torsional-shear");
    expect(openedByIdStoragePacket.summary.persisted_analysis_run_ref).toBe(
      "run:preview-linear-static-001",
    );
    // Heavy full-<App/> Three.js render: inherit the 30s global testTimeout
    // (vite.config.ts); a tight per-test override flaked under DEC-025 sweep load.
  });

  it("shows computed mechanics diagnostics in results, knowledge, and review-only proposal context", async () => {
    render(<App />);

    const runButton = await screen.findByRole("button", {
      name: /Run mechanics preview/i,
    });
    fireEvent.click(runButton);

    expect(
      await screen.findByTestId(
        "result-group-displacement",
        {},
        { timeout: 10000 },
      ),
    ).toBeInTheDocument();
    const solvedReadiness = screen.getByTestId("solve-readiness-summary");
    expect(
      within(solvedReadiness).getByTestId("readiness-mechanics").textContent,
    ).toContain("830 computed result rows");
    expect(
      within(solvedReadiness).getByTestId("readiness-mechanics").textContent,
    ).toContain("mechanics solved");
    expect(
      within(solvedReadiness).getByTestId("readiness-rule").textContent,
    ).toContain("rule inputs incomplete");
    expect(
      within(solvedReadiness).getByTestId("readiness-diagnostics").textContent,
    ).toContain("31 diagnostics");
    expect(
      within(solvedReadiness).getByTestId("readiness-diagnostics").textContent,
    ).toContain("11 warnings");
    expect(
      within(solvedReadiness).getByTestId("readiness-diagnostics").textContent,
    ).toContain("0 blocking/error");
    expect(
      within(solvedReadiness).getByTestId("readiness-professional").textContent,
    ).toContain("no professional acceptance record");
    expect(screen.getByTestId("solve-job-summary").textContent).toContain(
      "state=completed",
    );
    expect(screen.getByTestId("solve-job-summary").textContent).toContain(
      "events=3",
    );
    expect(screen.getByTestId("solve-job-summary").textContent).toContain(
      "result_rows=830",
    );
    expect(screen.getByTestId("solve-job-summary").textContent).toContain(
      "cancellation_requested=false",
    );
    expect(screen.getByTestId("solve-job-progress").textContent).toContain(
      "completed",
    );
    expect(screen.getByTestId("solve-job-progress").textContent).toContain(
      "percentages_synthesized=false",
    );
    expect(screen.getByTestId("solve-job-cancellation").textContent).toContain(
      "enabled=false",
    );
    expect(screen.getByTestId("solve-job-cancellation").textContent).toContain(
      "requested=false",
    );
    expect(screen.getByTestId("solve-job-cancellation").textContent).toContain(
      "success_claimed=false",
    );
    expect(screen.getByTestId("solve-job-binding").textContent).toContain(
      "state:project:invented-loop-01:preview",
    );
    expect(screen.getByTestId("solve-job-binding").textContent).toContain(
      "run:preview-linear-static-001",
    );
    expect(screen.getByTestId("solve-job-binding").textContent).toContain(
      "result rows=830",
    );
    expect(screen.getByTestId("solve-job-binding").textContent).toContain(
      "hashes=830",
    );
    expect(screen.getByTestId("solve-job-unit-policy").textContent).toContain(
      "model=angle=rad,force=N,length=m",
    );
    expect(screen.getByTestId("solve-job-unit-policy").textContent).toContain(
      "N*m/rad,N/m",
    );
    expect(screen.getByTestId("solve-job-unit-policy").textContent).toContain(
      "rows=830",
    );
    expect(screen.getByTestId("solve-job-unit-policy").textContent).toContain(
      "conversion=false",
    );
    expect(screen.getByTestId("solve-job-boundary").textContent).toContain(
      "release/professional claim=false",
    );
    const solveJobHref =
      screen.getByTestId("solve-job-export-link").getAttribute("href") ?? "";
    const solveJobPacket = JSON.parse(
      decodeURIComponent(solveJobHref.split(",", 2)[1]),
    );
    expect(solveJobPacket.document_kind).toBe(
      "openpipestress.technical_preview.solve_job_audit",
    );
    expect(solveJobPacket.deliverable_refs).toContain("DEL-07-07");
    expect(solveJobPacket.deliverable_refs).toContain("DEL-14-02");
    expect(solveJobPacket.deliverable_refs).toContain("DEL-04-06");
    expect(solveJobPacket.scope_items).toContain("SOW-055");
    expect(solveJobPacket.scope_items).toContain("SOW-072");
    expect(solveJobPacket.scope_items).toContain("SOW-053");
    expect(solveJobPacket.summary.job_state).toBe("completed");
    expect(solveJobPacket.summary.event_count).toBe(3);
    expect(solveJobPacket.summary.result_row_count).toBe(830);
    expect(solveJobPacket.summary.diagnostic_count).toBe(31);
    expect(solveJobPacket.summary.cancellation_requested).toBe(false);
    expect(solveJobPacket.summary.cancellation_status).toBe("not_requested");
    expect(solveJobPacket.progress_contract.progress_basis).toBe(
      "preview_service_event_state_only_no_percent_stream",
    );
    expect(solveJobPacket.progress_contract.percentages_synthesized).toBe(
      false,
    );
    expect(
      solveJobPacket.progress_contract.backend_percent_stream_available,
    ).toBe(false);
    expect(solveJobPacket.progress_contract.latest_event_state).toBe(
      "completed",
    );
    expect(solveJobPacket.cancellation.request_control_visible).toBe(true);
    expect(solveJobPacket.cancellation.request_enabled).toBe(false);
    expect(solveJobPacket.cancellation.requested).toBe(false);
    expect(solveJobPacket.cancellation.backend_job_seam).toBe(
      "browser_fixture_no_backend_job",
    );
    expect(solveJobPacket.cancellation.backend_job_id).toBe(null);
    expect(solveJobPacket.cancellation.backend_cancellation_token).toBe(
      "unavailable_no_backend_job_browser_fixture_mode",
    );
    expect(solveJobPacket.cancellation.cancellation_scope).toBe(
      "ui_request_record_only_no_backend_job",
    );
    expect(solveJobPacket.cancellation.mutates_solver_process_directly).toBe(
      false,
    );
    expect(solveJobPacket.cancellation.cancellation_success_claimed).toBe(
      false,
    );
    expect(solveJobPacket.model_state_ref.ref).toBe(
      "state:project:invented-loop-01:preview",
    );
    expect(solveJobPacket.analysis_run_ref.ref).toBe(
      "run:preview-linear-static-001",
    );
    expect(solveJobPacket.run_kind).toBe("mechanics_solve");
    expect(solveJobPacket.analysis_status).toContain("HUMAN_REVIEW_REQUIRED");
    expect(solveJobPacket.analysis_status).toContain("MECHANICS_SOLVED");
    expect(solveJobPacket.analysis_status).toContain("RULE_INPUTS_INCOMPLETE");
    expect(solveJobPacket.result_hash_count).toBe(830);
    expect(solveJobPacket.hash_scopes).toContain("analysis_run_record");
    expect(solveJobPacket.hash_scopes).toContain("result_envelope");
    expect(solveJobPacket.unit_policy_evidence.unit_system_ref.ref).toBe(
      "unit-system:dec-018-si-dual-display",
    );
    expect(solveJobPacket.unit_policy_evidence.storage_convention).toBe(
      "entered_units_preserved",
    );
    expect(solveJobPacket.unit_policy_evidence.model_units).toEqual({
      angle: "rad",
      force: "N",
      length: "m",
      pressure: "Pa",
      stress: "MPa",
      temperature: "degC",
    });
    expect(solveJobPacket.unit_policy_evidence.result_units).toEqual([
      "MPa",
      "N",
      "N*m",
      "N*m/rad",
      "N/m",
      "boolean",
      "count",
      "m",
      "mm",
      "mode_code",
      "rad",
      "state_code",
    ]);
    expect(solveJobPacket.unit_policy_evidence.result_row_count).toBe(830);
    expect(solveJobPacket.unit_policy_evidence.analysis_run_ref.ref).toBe(
      "run:preview-linear-static-001",
    );
    expect(solveJobPacket.unit_policy_evidence.conversion_policy).toBe(
      "solve_job_audit_preserves_source_units_no_conversion",
    );
    expect(solveJobPacket.unit_policy_evidence.conversion_performed).toBe(
      false,
    );
    expect(
      solveJobPacket.unit_policy_evidence.decision_basis_refs.map(
        (item: { ref: string }) => item.ref,
      ),
    ).toEqual(["DEC-018", "DEL-02-02", "DEL-07-07", "DEL-14-02"]);
    const resultExport = await screen.findByLabelText("Result export audit");
    expect(
      within(resultExport).getByTestId("result-export-summary").textContent,
    ).toContain("available");
    expect(
      within(resultExport).getByTestId("result-export-summary").textContent,
    ).toContain("rows=830");
    expect(
      within(resultExport).getByTestId("result-export-summary").textContent,
    ).toContain("sets=1");
    expect(
      within(resultExport).getByTestId("result-export-summary").textContent,
    ).toContain("diagnostics=31");
    expect(
      within(resultExport).getByTestId("result-export-format").textContent,
    ).toContain("schema_first_json_result_envelope");
    expect(
      within(resultExport).getByTestId("result-export-format").textContent,
    ).toContain("additional_formats=stress_neutral_csv_json_preview_available");
    expect(
      within(resultExport).getByTestId("result-export-state-binding")
        .textContent,
    ).toContain("project:invented-loop-01");
    expect(
      within(resultExport).getByTestId("result-export-state-binding")
        .textContent,
    ).toContain("run:preview-linear-static-001");
    expect(
      within(resultExport).getByTestId("result-export-units").textContent,
    ).toContain("explicit units");
    expect(
      within(resultExport).getByTestId("result-export-units").textContent,
    ).toContain("length");
    expect(
      within(resultExport).getByTestId("result-export-units").textContent,
    ).toContain("stress");
    expect(
      within(resultExport).getByTestId("result-export-unit-witnesses")
        .textContent,
    ).toContain("count=830");
    expect(
      within(resultExport).getByTestId("result-export-unit-witnesses")
        .textContent,
    ).toContain("conversion=false");
    expect(
      within(resultExport).getByTestId("result-export-reproducibility")
        .textContent,
    ).toContain("deterministic_ordering=true");
    expect(
      within(resultExport).getByTestId("result-export-reproducibility")
        .textContent,
    ).toContain("run_hashes=2");
    expect(
      within(resultExport).getByTestId("result-export-boundary").textContent,
    ).toContain(
      "no compliance, certification, sealing, authentication, or approval claim",
    );
    const resultExportHref =
      within(resultExport)
        .getByTestId("result-export-link")
        .getAttribute("href") ?? "";
    const resultExportPacket = JSON.parse(
      decodeURIComponent(resultExportHref.split(",", 2)[1]),
    );
    expect(resultExportPacket.deliverable_id).toBe("DEL-08-04");
    expect(resultExportPacket.package_id).toBe("PKG-08");
    expect(resultExportPacket.scope_item).toBe("SOW-046");
    expect(resultExportPacket.objectives).toContain("OBJ-007");
    expect(resultExportPacket.objectives).toContain("OBJ-009");
    expect(resultExportPacket.export_format_status.baseline_format).toBe(
      "schema_first_json_result_envelope",
    );
    expect(resultExportPacket.export_format_status.additional_formats).toBe(
      "stress_neutral_csv_json_preview_available",
    );
    expect(
      resultExportPacket.export_format_status.public_transport_protocol,
    ).toBe("TBD");
    expect(resultExportPacket.result_envelope.envelope_id).toBe(
      "result-envelope:run:preview-linear-static-001",
    );
    expect(resultExportPacket.result_envelope.model_ref.ref_id).toBe(
      "project:invented-loop-01",
    );
    expect(resultExportPacket.result_envelope.run_ref.ref_id).toBe(
      "run:preview-linear-static-001",
    );
    expect(
      resultExportPacket.result_envelope.load_basis_refs.length,
    ).toBeGreaterThan(0);
    expect(resultExportPacket.result_envelope.result_sets).toHaveLength(1);
    expect(resultExportPacket.result_envelope.result_sets[0].set_type).toBe(
      "mechanics",
    );
    expect(
      resultExportPacket.result_envelope.result_sets[0].values,
    ).toHaveLength(830);
    expect(
      resultExportPacket.result_envelope.result_sets[0].values[0].result_id,
    ).toBeTruthy();
    expect(
      resultExportPacket.result_envelope.result_sets[0].values[0].unit,
    ).toBeTruthy();
    expect(
      resultExportPacket.result_envelope.result_sets[0].values[0].dimension,
    ).toBeTruthy();
    expect(resultExportPacket.result_envelope.unit_witness_policy).toBe(
      "preserve_source_result_value_unit_and_dimension_per_exported_result_row",
    );
    expect(
      resultExportPacket.result_envelope.unit_preservation_witnesses,
    ).toHaveLength(830);
    const resultExportUnitWitness =
      resultExportPacket.result_envelope.unit_preservation_witnesses.find(
        (item: { witness_id: string }) =>
          item.witness_id ===
          "result-export-unit:result:force:pipe-P-120:axial",
      );
    expect(resultExportUnitWitness.source_quantity).toEqual({
      value: 0,
      unit: "N",
      dimension: "force",
    });
    expect(resultExportUnitWitness.target_quantity).toEqual({
      value: 0,
      unit: "N",
      dimension: "force",
    });
    expect(resultExportUnitWitness.target_quantity_policy).toBe(
      "exported_result_row_preserves_source_value_unit_and_dimension",
    );
    expect(resultExportUnitWitness.conversion_performed).toBe(false);
    expect(resultExportPacket.result_envelope.diagnostics).toHaveLength(31);
    expect(
      resultExportPacket.result_envelope.reproducibility.deterministic_ordering,
    ).toBe(true);
    expect(
      resultExportPacket.result_envelope.reproducibility.run_hashes,
    ).toHaveLength(2);
    expect(resultExportPacket.result_envelope.analysis_status).toContain(
      "HUMAN_REVIEW_REQUIRED",
    );
    expect(resultExportPacket.result_envelope.analysis_status).toContain(
      "MECHANICS_SOLVED",
    );
    expect(resultExportPacket.result_envelope.analysis_status).toContain(
      "RULE_INPUTS_INCOMPLETE",
    );
    expect(
      resultExportPacket.result_envelope.rule_pack_refs[0]
        .private_payload_redacted,
    ).toBe(true);
    expect(
      resultExportPacket.result_envelope.rule_pack_refs[0].completeness_status,
    ).toBe("missing_required_inputs");
    expect(
      resultExportPacket.result_envelope.downstream_use
        .additional_export_formats,
    ).toBe("stress_neutral_csv_json_preview_available");
    expect(
      resultExportPacket.result_envelope.professional_boundary
        .software_makes_compliance_claim,
    ).toBe(false);
    expect(
      resultExportPacket.result_envelope.professional_boundary
        .software_makes_certification_claim,
    ).toBe(false);
    expect(
      resultExportPacket.result_envelope.professional_boundary
        .software_makes_sealing_claim,
    ).toBe(false);
    expect(
      resultExportPacket.result_envelope.professional_boundary
        .software_makes_approval_claim,
    ).toBe(false);
    expect(
      resultExportPacket.result_envelope.professional_boundary
        .software_makes_authentication_claim,
    ).toBe(false);
    const stressNeutral = await screen.findByLabelText(
      "Stress-neutral CSV JSON export",
    );
    expect(
      within(stressNeutral).getByTestId("stress-neutral-summary").textContent,
    ).toContain("available");
    expect(
      within(stressNeutral).getByTestId("stress-neutral-summary").textContent,
    ).toContain("rows=830");
    expect(
      within(stressNeutral).getByTestId("stress-neutral-summary").textContent,
    ).toContain("csv_columns=11");
    expect(
      within(stressNeutral).getByTestId("stress-neutral-format").textContent,
    ).toContain("stress_neutral_csv_json");
    expect(
      within(stressNeutral).getByTestId("stress-neutral-format").textContent,
    ).toContain("ops.stress_neutral.v1");
    expect(
      within(stressNeutral).getByTestId("stress-neutral-state-binding")
        .textContent,
    ).toContain("project:invented-loop-01");
    expect(
      within(stressNeutral).getByTestId("stress-neutral-state-binding")
        .textContent,
    ).toContain("run:preview-linear-static-001");
    expect(
      within(stressNeutral).getByTestId("stress-neutral-units").textContent,
    ).toContain("explicit units");
    expect(
      within(stressNeutral).getByTestId("stress-neutral-units").textContent,
    ).toContain("stress");
    expect(
      within(stressNeutral).getByTestId("stress-neutral-units").textContent,
    ).toContain("source=angle=rad");
    expect(
      within(stressNeutral).getByTestId("stress-neutral-units").textContent,
    ).toContain("length=m");
    expect(
      within(stressNeutral).getByTestId("stress-neutral-units").textContent,
    ).toContain("results=MPa");
    expect(
      within(stressNeutral).getByTestId("stress-neutral-units").textContent,
    ).toContain("conversion=false");
    expect(
      within(stressNeutral).getByTestId("stress-neutral-unit-witnesses")
        .textContent,
    ).toContain("count=830");
    expect(
      within(stressNeutral).getByTestId("stress-neutral-unit-witnesses")
        .textContent,
    ).toContain("conversion=false");
    expect(
      within(stressNeutral).getByTestId("stress-neutral-package").textContent,
    ).toContain("members=9");
    expect(
      within(stressNeutral).getByTestId("stress-neutral-package").textContent,
    ).toContain("stable_ids=830");
    expect(
      within(stressNeutral).getByTestId("stress-neutral-package").textContent,
    ).toContain("validation=passed");
    await waitFor(() =>
      expect(
        within(stressNeutral).getByTestId("stress-neutral-package").textContent,
      ).toContain("package_hash=computed_local_preview_sha256"),
    );
    expect(
      within(stressNeutral).getByTestId("stress-neutral-boundary").textContent,
    ).toContain("vendor_format=false");
    expect(
      within(stressNeutral).getByTestId("stress-neutral-boundary").textContent,
    ).toContain("solver_validation=false");
    const stressNeutralHref =
      within(stressNeutral)
        .getByTestId("stress-neutral-export-link")
        .getAttribute("href") ?? "";
    const stressNeutralPacket = JSON.parse(
      decodeURIComponent(stressNeutralHref.split(",", 2)[1]),
    );
    expect(stressNeutralPacket.document_kind).toBe(
      "openpipestress.technical_preview.stress_neutral_csv_json_package",
    );
    expect(stressNeutralPacket.deliverable_id).toBe("DEL-17-06");
    expect(stressNeutralPacket.package_id).toBe("PKG-17");
    expect(stressNeutralPacket.scope_items).toContain("SOW-046");
    expect(stressNeutralPacket.scope_items).toContain("SOW-074");
    expect(stressNeutralPacket.objectives).toContain("OBJ-007");
    expect(stressNeutralPacket.objectives).toContain("OBJ-017");
    expect(stressNeutralPacket.objectives).toContain("OBJ-018");
    expect(stressNeutralPacket.package_status).toBe(
      "stress_neutral_export_package",
    );
    expect(stressNeutralPacket.export_profile.target_family).toBe(
      "stress_neutral_csv_json",
    );
    expect(stressNeutralPacket.export_profile.csv_columns).toEqual([
      "result_id",
      "canonical_ref",
      "row_kind",
      "result_family",
      "load_case_ref",
      "station_ref",
      "component_ref",
      "value",
      "unit",
      "dimension",
      "correlation_status",
    ]);
    expect(stressNeutralPacket.unit_system_disclosure.unit_system_ref.ref).toBe(
      "unit-system:dec-018-si-dual-display",
    );
    expect(stressNeutralPacket.unit_system_disclosure.model_units.length).toBe(
      "m",
    );
    expect(stressNeutralPacket.unit_system_disclosure.result_units).toContain(
      "MPa",
    );
    expect(stressNeutralPacket.unit_system_disclosure.result_units).toContain(
      "mm",
    );
    expect(
      stressNeutralPacket.unit_system_disclosure.conversion_performed,
    ).toBe(false);
    expect(
      stressNeutralPacket.unit_system_disclosure.protected_content_included,
    ).toBe(false);
    expect(stressNeutralPacket.unit_preservation_witnesses).toHaveLength(830);
    expect(
      stressNeutralPacket.manifest.package_members.map(
        (item: { role: string }) => item.role,
      ),
    ).toContain("unit_preservation_witnesses");
    const stressNeutralUnitWitness =
      stressNeutralPacket.unit_preservation_witnesses.find(
        (item: { witness_id: string }) =>
          item.witness_id ===
          "stress-neutral-unit:result-force-pipe-p-120-axial",
      );
    expect(stressNeutralUnitWitness.source_quantity).toEqual({
      value: 0,
      unit: "N",
      dimension: "force",
    });
    expect(stressNeutralUnitWitness.target_quantity).toEqual({
      value: 0,
      unit: "N",
      dimension: "force",
    });
    expect(stressNeutralUnitWitness.export_unit_policy).toBe(
      "preserve_source_result_unit_and_dimension",
    );
    expect(stressNeutralUnitWitness.conversion_performed).toBe(false);
    expect(stressNeutralPacket.result_rows).toHaveLength(830);
    expect(stressNeutralPacket.stable_id_map).toHaveLength(830);
    expect(stressNeutralPacket.csv_text.split("\n")[0]).toBe(
      "result_id,canonical_ref,row_kind,result_family,load_case_ref,station_ref,component_ref,value,unit,dimension,correlation_status",
    );
    expect(stressNeutralPacket.csv_text).toContain(
      "result:force:pipe-P-120:axial",
    );
    expect(stressNeutralPacket.loss_report.entries).toHaveLength(3);
    expect(
      stressNeutralPacket.loss_report.entries.map(
        (entry: { category: string }) => entry.category,
      ),
    ).toContain("tbd");
    expect(stressNeutralPacket.manifest.package_members).toHaveLength(9);
    expect(stressNeutralPacket.manifest.canonical_package_hash_status).toBe(
      "computed_local_preview_sha256",
    );
    expect(stressNeutralPacket.manifest.canonical_package_hash.value).toMatch(
      /^sha256:[0-9a-f]{64}$/,
    );
    expect(
      stressNeutralPacket.manifest.canonical_package_hash.payload_scope,
    ).toBe("package_review_payload");
    expect(
      stressNeutralPacket.manifest.canonical_package_hash.payload_excludes,
    ).toBe("manifest_and_validation_report_package_hash_carrier_fields");
    expect(stressNeutralPacket.validation_report.hash_validation_status).toBe(
      "package_hash_computed_local_preview_not_independently_validated",
    );
    expect(stressNeutralPacket.validation_report.validation_status).toBe(
      "passed",
    );
    expect(stressNeutralPacket.validation_report.schema_validation_status).toBe(
      "desktop_preview_shape_aligned_not_runtime_json_schema_validated",
    );
    expect(
      stressNeutralPacket.validation_report.checks.map(
        (item: { check_id: string }) => item.check_id,
      ),
    ).toContain("unit_preservation_witness_per_row");
    expect(
      stressNeutralPacket.result_rows.every(
        (row: { unit: string; dimension: string }) => row.unit && row.dimension,
      ),
    ).toBe(true);
    expect(stressNeutralPacket.private_payload_included).toBe(false);
    expect(stressNeutralPacket.protected_content_included).toBe(false);
    expect(stressNeutralPacket.vendor_format_claim).toBe(false);
    expect(stressNeutralPacket.solver_validation_claim).toBe(false);
    expect(stressNeutralPacket.code_compliance_claim).toBe(false);
    expect(stressNeutralPacket.professional_reliance_claim).toBe(false);
    const stressNeutralCsvHref =
      within(stressNeutral)
        .getByTestId("stress-neutral-csv-link")
        .getAttribute("href") ?? "";
    expect(
      decodeURIComponent(stressNeutralCsvHref.split(",", 2)[1]).split("\n")[0],
    ).toBe(
      "result_id,canonical_ref,row_kind,result_family,load_case_ref,station_ref,component_ref,value,unit,dimension,correlation_status",
    );
    const headlessRunner = await screen.findByLabelText(
      "Headless runner envelope",
    );
    expect(
      within(headlessRunner).getByTestId("headless-runner-summary").textContent,
    ).toContain("job=COMPLETED");
    expect(
      within(headlessRunner).getByTestId("headless-runner-summary").textContent,
    ).toContain("outputs=4");
    expect(
      within(headlessRunner).getByTestId("headless-runner-summary").textContent,
    ).toContain("result_refs=830");
    expect(
      within(headlessRunner).getByTestId("headless-runner-job").textContent,
    ).toContain("job:preview-linear-static:project-invented-loop-01");
    expect(
      within(headlessRunner).getByTestId("headless-runner-job").textContent,
    ).toContain("3/3");
    expect(
      within(headlessRunner).getByTestId("headless-runner-result-handoff")
        .textContent,
    ).toContain("schema_first_json_result_envelope");
    expect(
      within(headlessRunner).getByTestId("headless-runner-result-handoff")
        .textContent,
    ).toContain("audit=audit-manifest:run:preview-linear-static-001:preview");
    expect(
      within(headlessRunner).getByTestId("headless-runner-result-handoff")
        .textContent,
    ).toContain("checksums=2");
    expect(
      within(headlessRunner).getByTestId("headless-runner-units").textContent,
    ).toContain("conversion=false");
    expect(
      within(headlessRunner).getByTestId("headless-runner-unit-witnesses")
        .textContent,
    ).toContain("count=830");
    expect(
      within(headlessRunner).getByTestId("headless-runner-unit-witnesses")
        .textContent,
    ).toContain("conversion=false");
    const headlessHref =
      within(headlessRunner)
        .getByTestId("headless-runner-export-link")
        .getAttribute("href") ?? "";
    const headlessPacket = JSON.parse(
      decodeURIComponent(headlessHref.split(",", 2)[1]),
    );
    expect(headlessPacket.deliverable_id).toBe("DEL-10-05");
    expect(headlessPacket.scope_items).toContain("SOW-054");
    expect(headlessPacket.scope_items).toContain("SOW-032");
    expect(headlessPacket.objectives).toContain("OBJ-012");
    expect(headlessPacket.request.operation).toBe("solve");
    expect(headlessPacket.request.load_basis_refs.length).toBeGreaterThan(0);
    expect(headlessPacket.request.input_manifest_ref.ref_id).toBe(
      "result-envelope:run:preview-linear-static-001",
    );
    expect(headlessPacket.result.run_id).toBe("run:preview-linear-static-001");
    expect(headlessPacket.result.job.state).toBe("COMPLETED");
    expect(headlessPacket.result.job.progress.current_step).toBe(3);
    expect(headlessPacket.result.job.cancellation.supported).toBe(true);
    expect(headlessPacket.result.job.cancellation.requested).toBe(false);
    expect(headlessPacket.result.analysis_status).toContain(
      "HUMAN_REVIEW_REQUIRED",
    );
    expect(headlessPacket.result.analysis_status).toContain("MECHANICS_SOLVED");
    expect(headlessPacket.result.analysis_status).toContain(
      "RULE_INPUTS_INCOMPLETE",
    );
    expect(headlessPacket.result.result_envelope_ref.schema_ref).toBe(
      "schemas/results.schema.yaml",
    );
    expect(headlessPacket.result.result_envelope_ref.envelope_ref.ref_id).toBe(
      "result-envelope:run:preview-linear-static-001",
    );
    expect(headlessPacket.result.result_refs).toHaveLength(830);
    expect(headlessPacket.result.audit_manifest_ref.ref_id).toBe(
      "audit-manifest:run:preview-linear-static-001:preview",
    );
    expect(headlessPacket.result.checksums).toHaveLength(2);
    expect(
      headlessPacket.result.unit_system_disclosure.unit_system_ref.ref,
    ).toBe("unit-system:dec-018-si-dual-display");
    expect(
      headlessPacket.result.unit_system_disclosure.model_units.length,
    ).toBe("m");
    expect(headlessPacket.result.unit_system_disclosure.result_units).toContain(
      "MPa",
    );
    expect(headlessPacket.result.unit_system_disclosure.result_units).toContain(
      "mm",
    );
    expect(
      headlessPacket.result.unit_system_disclosure.conversion_performed,
    ).toBe(false);
    expect(
      headlessPacket.result.unit_system_disclosure.protected_content_included,
    ).toBe(false);
    expect(headlessPacket.result.unit_witness_policy).toBe(
      "preserve_source_result_value_unit_and_dimension_per_headless_result_handoff_row",
    );
    expect(headlessPacket.result.unit_preservation_witnesses).toHaveLength(830);
    const headlessUnitWitness =
      headlessPacket.result.unit_preservation_witnesses.find(
        (item: { source_result_ref: { ref_id: string } }) =>
          item.source_result_ref.ref_id === "result:force:pipe-P-120:axial",
      );
    expect(headlessUnitWitness.source_quantity.unit).toBe("N");
    expect(headlessUnitWitness.source_quantity.dimension).toBe("force");
    expect(headlessUnitWitness.target_quantity.unit).toBe("N");
    expect(headlessUnitWitness.target_quantity.dimension).toBe("force");
    expect(headlessUnitWitness.conversion_performed).toBe(false);
    expect(headlessPacket.result.diagnostics).toHaveLength(31);
    expect(headlessPacket.result.privacy.telemetry_allowed).toBe(false);
    expect(
      headlessPacket.result.professional_boundary
        .software_makes_compliance_claim,
    ).toBe(false);
    expect(
      headlessPacket.result.professional_boundary
        .software_makes_authentication_claim,
    ).toBe(false);
    const adapterFramework = await screen.findByLabelText(
      "Adapter framework envelope",
    );
    expect(
      within(adapterFramework).getByTestId("adapter-framework-summary")
        .textContent,
    ).toContain("capabilities=4");
    expect(
      within(adapterFramework).getByTestId("adapter-framework-summary")
        .textContent,
    ).toContain("diagnostics=31");
    expect(
      within(adapterFramework).getByTestId("adapter-framework-validation")
        .textContent,
    ).toContain("export=required_before_shared_payload");
    expect(
      within(adapterFramework).getByTestId("adapter-framework-units")
        .textContent,
    ).toContain("conversion=false");
    expect(
      within(adapterFramework).getByTestId("adapter-framework-units")
        .textContent,
    ).toContain("witnesses=1");
    const adapterHref =
      within(adapterFramework)
        .getByTestId("adapter-framework-export-link")
        .getAttribute("href") ?? "";
    const adapterPacket = JSON.parse(
      decodeURIComponent(adapterHref.split(",", 2)[1]),
    );
    expect(adapterPacket.deliverable_id).toBe("DEL-10-02");
    expect(adapterPacket.scope_item).toBe("SOW-030");
    expect(adapterPacket.objective).toBe("OBJ-009");
    expect(adapterPacket.operation_result.operation_id).toBe(
      "adapter-operation:desktop-preview:run-preview-linear-static-001",
    );
    expect(adapterPacket.operation_result.operation_class).toBe("export");
    expect(adapterPacket.operation_result.parse_status).toBe(
      "not_parsed_by_framework",
    );
    expect(adapterPacket.operation_result.diagnostics).toHaveLength(31);
    expect(adapterPacket.operation_result.checksums).toHaveLength(2);
    expect(adapterPacket.operation_result.audit_manifest_refs[0].ref_id).toBe(
      "audit-manifest:run:preview-linear-static-001:preview",
    );
    expect(adapterPacket.operation_result.result_envelope_ref.ref.ref_id).toBe(
      "result-envelope:run:preview-linear-static-001",
    );
    expect(adapterPacket.unit_policy_evidence.unit_system_ref.ref).toBe(
      "unit-system:dec-018-si-dual-display",
    );
    expect(adapterPacket.unit_policy_evidence.result_units).toContain("MPa");
    expect(adapterPacket.unit_policy_evidence.result_units).toContain("mm");
    expect(adapterPacket.unit_policy_evidence.conversion_performed).toBe(false);
    expect(adapterPacket.unit_policy_evidence.framework_unit_policy).toBe(
      "unit_validation_required_before_adapter_payload_exchange",
    );
    expect(adapterPacket.unit_policy_evidence.witness_count).toBe(1);
    expect(
      adapterPacket.adapter_declaration.no_bypass_controls
        .must_preserve_diagnostics,
    ).toBe(true);
    expect(
      adapterPacket.adapter_declaration.no_bypass_controls
        .must_preserve_report_controls,
    ).toBe(true);
    expect(
      adapterPacket.adapter_declaration.no_bypass_controls
        .must_preserve_human_acceptance_boundary,
    ).toBe(true);
    expect(
      adapterPacket.adapter_declaration.no_bypass_controls
        .must_not_execute_arbitrary_code,
    ).toBe(true);
    expect(
      adapterPacket.adapter_declaration.no_bypass_controls
        .must_not_transmit_private_data_by_default,
    ).toBe(true);
    expect(adapterPacket.operation_result.privacy.local_first).toBe(true);
    expect(
      adapterPacket.operation_result.privacy.private_payload_redacted,
    ).toBe(true);
    expect(
      adapterPacket.operation_result.professional_boundary
        .software_makes_security_certification_claim,
    ).toBe(false);
    const localFea = await screen.findByLabelText("Local FEA handoff");
    expect(
      within(localFea).getByTestId("local-fea-summary").textContent,
    ).toContain("available");
    expect(
      within(localFea).getByTestId("local-fea-summary").textContent,
    ).toContain("labels=4");
    expect(
      within(localFea).getByTestId("local-fea-summary").textContent,
    ).toContain("flags=5");
    expect(
      within(localFea).getByTestId("local-fea-summary").textContent,
    ).toContain("diagnostics=4");
    expect(
      within(localFea).getByTestId("local-fea-contract").textContent,
    ).toContain("schema_first_local_fea_handoff_contract");
    expect(
      within(localFea).getByTestId("local-fea-contract").textContent,
    ).toContain("format=TBD");
    expect(
      within(localFea).getByTestId("local-fea-state-binding").textContent,
    ).toContain("state:project:invented-loop-01:preview");
    expect(
      within(localFea).getByTestId("local-fea-state-binding").textContent,
    ).toContain("result-envelope:run:preview-linear-static-001");
    expect(
      within(localFea).getByTestId("local-fea-region").textContent,
    ).toContain("basis=diagnostic_suggested");
    expect(
      within(localFea).getByTestId("local-fea-region").textContent,
    ).toContain("elements=2");
    expect(
      within(localFea).getByTestId("local-fea-transfer").textContent,
    ).toContain("method=result_reference_only");
    expect(
      within(localFea).getByTestId("local-fea-transfer").textContent,
    ).toContain("loads=3");
    expect(
      within(localFea).getByTestId("local-fea-unit-witnesses").textContent,
    ).toContain("count=3");
    expect(
      within(localFea).getByTestId("local-fea-unit-witnesses").textContent,
    ).toContain("conversion=false");
    expect(
      within(localFea).getByTestId("local-fea-unsupported").textContent,
    ).toContain("mesh_generation_not_performed");
    expect(
      within(localFea).getByTestId("local-fea-unsupported").textContent,
    ).toContain("external_solver_not_invoked");
    expect(
      within(localFea).getByTestId("local-fea-boundary").textContent,
    ).toContain("human_review=true");
    expect(
      within(localFea).getByTestId("local-fea-boundary").textContent,
    ).toContain("compliance=false");
    const localFeaHref =
      within(localFea)
        .getByTestId("local-fea-export-link")
        .getAttribute("href") ?? "";
    const localFeaPacket = JSON.parse(
      decodeURIComponent(localFeaHref.split(",", 2)[1]),
    );
    expect(localFeaPacket.deliverable_id).toBe("DEL-10-03");
    expect(localFeaPacket.package_id).toBe("PKG-10");
    expect(localFeaPacket.scope_items).toContain("SOW-031");
    expect(localFeaPacket.scope_items).toContain("SOW-049");
    expect(localFeaPacket.objective).toBe("OBJ-009");
    expect(localFeaPacket.contract_status.global_analysis_role).toBe(
      "primary_global_centerline_frame_model",
    );
    expect(localFeaPacket.contract_status.local_analysis_role).toBe(
      "optional_specialized_shell_solid_handoff",
    );
    expect(localFeaPacket.contract_status.concrete_export_format).toBe("TBD");
    expect(localFeaPacket.contract_status.mesh_generation).toBe("TBD");
    expect(localFeaPacket.contract_status.external_solver_invocation).toBe(
      "TBD",
    );
    expect(localFeaPacket.contract_status.professional_decision).toBe(
      "human_review_required",
    );
    expect(localFeaPacket.handoff_package.package_kind).toBe(
      "local_shell_solid_fea_handoff",
    );
    expect(localFeaPacket.handoff_package.source_refs.global_model_kind).toBe(
      "centerline_frame_global_analysis",
    );
    expect(localFeaPacket.handoff_package.local_region.selection_basis).toBe(
      "diagnostic_suggested",
    );
    expect(
      localFeaPacket.handoff_package.local_region.selected_entity_ids
        .element_ids,
    ).toContain("pipe:P-130");
    expect(localFeaPacket.handoff_package.entity_ids.result_ids).toContain(
      "result:force:pipe-P-120:axial",
    );
    expect(localFeaPacket.handoff_package.entity_ids.result_ids).toContain(
      "result:stress:pipe-P-120:end-j:torsional-shear",
    );
    expect(localFeaPacket.handoff_package.units_manifest.dimension_basis).toBe(
      "schemas/units.schema.yaml",
    );
    expect(
      localFeaPacket.handoff_package.transfer_basis.transfer_method_label,
    ).toBe("result_reference_only");
    expect(localFeaPacket.handoff_package.unit_witness_policy).toBe(
      "preserve_source_result_units_for_referenced_transfer_results",
    );
    expect(
      localFeaPacket.handoff_package.unit_preservation_witnesses,
    ).toHaveLength(3);
    const localFeaForceWitness =
      localFeaPacket.handoff_package.unit_preservation_witnesses.find(
        (item: { source_result_ref: { locator: string } }) =>
          item.source_result_ref.locator === "result:force:pipe-P-120:axial",
      );
    expect(localFeaForceWitness.source_quantity).toEqual({
      value: 0,
      unit: "N",
      dimension: "force",
    });
    expect(localFeaForceWitness.target_field_path).toBe(
      "handoff_package.transfer_basis.force_result_refs[]",
    );
    expect(localFeaForceWitness.target_quantity_policy).toBe(
      "referenced_result_value_and_unit_preserved_by_source_ref",
    );
    expect(localFeaForceWitness.conversion_performed).toBe(false);
    expect(localFeaPacket.handoff_package.guidance_assessment.labels).toContain(
      "human_review_required",
    );
    expect(localFeaPacket.handoff_package.guidance_assessment.labels).toContain(
      "global_to_local_transfer_inputs_incomplete",
    );
    expect(
      localFeaPacket.handoff_package.unsupported_behavior_flags.map(
        (item: { behavior_label: string }) => item.behavior_label,
      ),
    ).toContain("target_format_not_selected");
    expect(
      localFeaPacket.handoff_package.diagnostics.map(
        (item: { code: string }) => item.code,
      ),
    ).toContain("LOCAL-FEA-EXTERNAL-SOLVER-NOT-INVOKED");
    expect(localFeaPacket.handoff_package.privacy.local_only).toBe(true);
    expect(localFeaPacket.handoff_package.privacy.telemetry_allowed).toBe(
      false,
    );
    expect(
      localFeaPacket.handoff_package.privacy.private_payload_embedded,
    ).toBe(false);
    expect(
      localFeaPacket.handoff_package.professional_boundary
        .software_makes_compliance_claim,
    ).toBe(false);
    expect(
      localFeaPacket.handoff_package.professional_boundary
        .software_makes_authentication_claim,
    ).toBe(false);
    const nativePackage = await screen.findByLabelText("Native JSON package");
    expect(
      within(nativePackage).getByTestId("native-package-summary").textContent,
    ).toContain("ready");
    expect(
      within(nativePackage).getByTestId("native-package-summary").textContent,
    ).toContain("members=10");
    expect(
      within(nativePackage).getByTestId("native-package-summary").textContent,
    ).toContain("entities=26");
    expect(
      within(nativePackage).getByTestId("native-package-summary").textContent,
    ).toContain("results=830");
    expect(
      within(nativePackage).getByTestId("native-package-profile").textContent,
    ).toContain("native_open_json_preview");
    expect(
      within(nativePackage).getByTestId("native-package-profile").textContent,
    ).toContain("physical_container=TBD");
    expect(
      within(nativePackage).getByTestId("native-package-members").textContent,
    ).toContain("manifest.json");
    expect(
      within(nativePackage).getByTestId("native-package-members").textContent,
    ).toContain("maps/stable_id_map.json");
    expect(
      within(nativePackage).getByTestId("native-package-members").textContent,
    ).toContain("maps/unit_preservation_witnesses.json");
    expect(
      within(nativePackage).getByTestId("native-package-unit-witnesses")
        .textContent,
    ).toContain("project_units=6");
    expect(
      within(nativePackage).getByTestId("native-package-unit-witnesses")
        .textContent,
    ).toContain("model_quantities=50");
    expect(
      within(nativePackage).getByTestId("native-package-unit-witnesses")
        .textContent,
    ).toContain("result_quantities=832");
    expect(
      within(nativePackage).getByTestId("native-package-unit-witnesses")
        .textContent,
    ).toContain("conversion=false");
    expect(
      within(nativePackage).getByTestId("native-package-validation")
        .textContent,
    ).toContain("review_manifest_complete");
    await waitFor(() =>
      expect(
        within(nativePackage).getByTestId("native-package-validation")
          .textContent,
      ).toContain("model_hash=computed_local_preview_sha256"),
    );
    await waitFor(() =>
      expect(
        within(nativePackage).getByTestId("native-package-validation")
          .textContent,
      ).toContain("package_hash=computed_local_preview_sha256"),
    );
    expect(
      within(nativePackage).getByTestId("native-package-loss-report")
        .textContent,
    ).toContain("1 TBD");
    expect(
      within(nativePackage).getByTestId("native-package-loss-report")
        .textContent,
    ).toContain("1 unsupported");
    expect(
      within(nativePackage).getByTestId("native-package-storage").textContent,
    ).toContain("network=false");
    expect(
      within(nativePackage).getByTestId("native-package-storage").textContent,
    ).toContain("repository_default_private_write=false");
    expect(
      within(nativePackage).getByTestId(
        "native-package-persisted-review-context",
      ).textContent,
    ).toContain("editor_intents=0");
    expect(
      within(nativePackage).getByTestId(
        "native-package-persisted-review-context",
      ).textContent,
    ).toContain("proposals=0");
    expect(
      within(nativePackage).getByTestId(
        "native-package-persisted-review-context",
      ).textContent,
    ).toContain("selected_targets=0");
    expect(
      within(nativePackage).getByTestId(
        "native-package-persisted-review-context",
      ).textContent,
    ).toContain("selected_ref=not_selected");
    expect(
      within(nativePackage).getByTestId(
        "native-package-persisted-review-context",
      ).textContent,
    ).toContain("mechanics_results=0");
    expect(
      within(nativePackage).getByTestId(
        "native-package-persisted-review-context",
      ).textContent,
    ).toContain("analysis_runs=0");
    expect(
      within(nativePackage).getByTestId(
        "native-package-persisted-review-context",
      ).textContent,
    ).toContain("run_ref=not_persisted");
    expect(
      within(nativePackage).getByTestId("native-package-boundary").textContent,
    ).toContain(
      "no private payload, protected content, release claim, compatibility claim, compliance claim, or professional approval claim",
    );
    const nativePackageHref =
      within(nativePackage)
        .getByTestId("native-package-link")
        .getAttribute("href") ?? "";
    const nativePackagePacket = JSON.parse(
      decodeURIComponent(nativePackageHref.split(",", 2)[1]),
    );
    expect(nativePackagePacket.document_kind).toBe(
      "openpipestress.technical_preview.native_json_package_review",
    );
    expect(nativePackagePacket.deliverable_refs).toContain("DEL-17-02");
    expect(nativePackagePacket.deliverable_refs).toContain("DEL-17-03");
    expect(nativePackagePacket.deliverable_refs).toContain("DEL-02-05");
    expect(nativePackagePacket.deliverable_refs).toContain("DEL-12-01");
    expect(nativePackagePacket.scope_items).toContain("SOW-030");
    expect(nativePackagePacket.scope_items).toContain("SOW-074");
    expect(nativePackagePacket.scope_items).toContain("SOW-050");
    expect(nativePackagePacket.scope_items).toContain("SOW-029");
    expect(nativePackagePacket.export_profile.profile_id).toBe(
      "native_open_json_preview",
    );
    expect(nativePackagePacket.export_profile.physical_project_container).toBe(
      "TBD",
    );
    expect(nativePackagePacket.export_profile.public_transport_protocol).toBe(
      "TBD",
    );
    expect(nativePackagePacket.export_profile.unit_witness_policy).toBe(
      "required_sidecar_for_native_json_quantity_fields",
    );
    expect(nativePackagePacket.manifest.package_members).toHaveLength(10);
    expect(
      nativePackagePacket.manifest.package_members.map(
        (item: { path: string }) => item.path,
      ),
    ).toContain("results/result_envelope_ref.json");
    expect(
      nativePackagePacket.manifest.package_members.map(
        (item: { path: string }) => item.path,
      ),
    ).toContain("maps/unit_preservation_witnesses.json");
    expect(
      nativePackagePacket.manifest.runtime_timestamp_fields_in_hash_inputs,
    ).toBe(false);
    expect(nativePackagePacket.unit_preservation.unit_system_ref.ref).toBe(
      "unit-system:dec-018-si-dual-display",
    );
    expect(nativePackagePacket.unit_preservation.conversion_performed).toBe(
      false,
    );
    expect(
      nativePackagePacket.unit_preservation.project_unit_declarations,
    ).toHaveLength(6);
    expect(
      nativePackagePacket.unit_preservation.model_quantity_witnesses,
    ).toHaveLength(50);
    expect(
      nativePackagePacket.unit_preservation.result_quantity_witnesses,
    ).toHaveLength(832);
    expect(
      nativePackagePacket.unit_preservation.summary.total_witness_count,
    ).toBe(888);
    expect(
      nativePackagePacket.unit_preservation.model_quantity_witnesses.find(
        (witness: { witness_id: string }) =>
          witness.witness_id ===
          "native-unit:model:pipe:P-120:section.outside_diameter",
      ),
    ).toMatchObject({
      source_ref: {
        ref_type: "pipe_segment",
        ref_id: "pipe:P-120",
        field_path: "section.outside_diameter",
      },
      target_ref: {
        member_path: "model/project.json",
        field_path: "section.outside_diameter",
      },
      source_quantity: {
        value: 0.168,
        unit: "m",
        dimension: "length",
      },
      target_quantity: {
        value: 0.168,
        unit: "m",
        dimension: "length",
      },
      conversion_performed: false,
      preservation_status: "unit_and_value_preserved",
    });
    expect(
      nativePackagePacket.unit_preservation.model_quantity_witnesses.find(
        (witness: { witness_id: string }) =>
          witness.witness_id ===
          "native-unit:model:component:C-110:geometry.bend_radius",
      ),
    ).toMatchObject({
      source_ref: {
        ref_type: "component",
        ref_id: "component:C-110",
        field_path: "geometry.bend_radius",
      },
      source_quantity: {
        value: 0.45,
        unit: "m",
        dimension: "length",
      },
      conversion_performed: false,
      preservation_status: "unit_and_value_preserved",
    });
    expect(
      nativePackagePacket.unit_preservation.model_quantity_witnesses.find(
        (witness: { witness_id: string }) =>
          witness.witness_id ===
          "native-unit:model:component:C-130:modifiers.linear_stiffness_user_value",
      ),
    ).toMatchObject({
      source_ref: {
        ref_type: "component",
        ref_id: "component:C-130",
        field_path: "modifiers.linear_stiffness_user_value",
      },
      source_quantity: {
        value: 15000000,
        unit: "N/m",
        dimension: "linear_stiffness",
      },
      conversion_performed: false,
      preservation_status: "unit_and_value_preserved",
    });
    expect(
      nativePackagePacket.unit_preservation.model_quantity_witnesses.find(
        (witness: { witness_id: string }) =>
          witness.witness_id ===
          "native-unit:model:component:C-150:modifiers.axial_stiffness_user_value",
      ),
    ).toMatchObject({
      source_ref: {
        ref_type: "component",
        ref_id: "component:C-150",
        field_path: "modifiers.axial_stiffness_user_value",
      },
      source_quantity: {
        value: 3200000,
        unit: "N/m",
        dimension: "linear_stiffness",
      },
      conversion_performed: false,
      preservation_status: "unit_and_value_preserved",
    });
    expect(
      nativePackagePacket.unit_preservation.result_quantity_witnesses.find(
        (witness: { witness_id: string }) =>
          witness.witness_id ===
          "native-unit:result:result:force:pipe-P-120:axial:value",
      ),
    ).toMatchObject({
      source_ref: {
        ref_type: "result_row",
        ref_id: "result:force:pipe-P-120:axial",
        field_path: "value",
      },
      target_ref: {
        member_path: "results/result_envelope_ref.json",
        field_path: "value",
      },
      conversion_performed: false,
      preservation_status: "unit_and_value_preserved",
    });
    expect(nativePackagePacket.stable_id_map.entity_ref_count).toBe(26);
    expect(nativePackagePacket.stable_id_map.result_ref_count).toBe(830);
    expect(nativePackagePacket.stable_id_map.operation_ref_count).toBe(0);
    expect(nativePackagePacket.stable_id_map.entity_refs).toContain(
      "project:invented-loop-01",
    );
    expect(nativePackagePacket.stable_id_map.result_refs).toContain(
      "result:force:pipe-P-120:axial",
    );
    expect(
      nativePackagePacket.source_project.storage_summary.editor_intent_count,
    ).toBe(0);
    expect(
      nativePackagePacket.source_project.storage_summary.proposal_count,
    ).toBe(0);
    expect(
      nativePackagePacket.source_project.storage_summary
        .selected_review_target_count,
    ).toBe(0);
    expect(
      nativePackagePacket.source_project.storage_summary
        .selected_review_target_ref,
    ).toBe("not_selected");
    expect(
      nativePackagePacket.source_project.storage_summary
        .persisted_mechanics_result_count,
    ).toBe(0);
    expect(
      nativePackagePacket.source_project.storage_summary
        .persisted_analysis_run_count,
    ).toBe(0);
    expect(
      nativePackagePacket.source_project.storage_summary
        .persisted_analysis_run_ref,
    ).toBe("not_persisted");
    expect(nativePackagePacket.loss_report.summary.unsupported_count).toBe(1);
    expect(nativePackagePacket.loss_report.summary.tbd_count).toBe(1);
    expect(nativePackagePacket.validation_report.package_shape_status).toBe(
      "review_manifest_complete",
    );
    expect(nativePackagePacket.validation_report.model_hash_status).toBe(
      "computed_local_preview_sha256",
    );
    expect(nativePackagePacket.validation_report.model_hash.value).toMatch(
      /^sha256:[0-9a-f]{64}$/,
    );
    expect(
      nativePackagePacket.validation_report.model_hash.canonicalization,
    ).toBe("rfc8785_jcs");
    expect(nativePackagePacket.validation_report.model_hash.payload_ref).toBe(
      "project:invented-loop-01",
    );
    expect(
      nativePackagePacket.manifest.source_model_version_or_hash_basis,
    ).toMatch(/^sha256:[0-9a-f]{64}$/);
    expect(
      nativePackagePacket.manifest.package_members.find(
        (item: { path: string }) => item.path === "model/project.json",
      ).hash_status,
    ).toMatch(/^sha256:[0-9a-f]{64}$/);
    expect(nativePackagePacket.validation_report.package_hash_status).toBe(
      "computed_local_preview_sha256",
    );
    expect(nativePackagePacket.validation_report.package_hash.value).toMatch(
      /^sha256:[0-9a-f]{64}$/,
    );
    expect(nativePackagePacket.validation_report.package_hash.value).not.toBe(
      nativePackagePacket.validation_report.model_hash.value,
    );
    expect(
      nativePackagePacket.validation_report.package_hash.canonicalization,
    ).toBe("rfc8785_jcs");
    expect(
      nativePackagePacket.validation_report.package_hash.payload_scope,
    ).toBe("package_review_payload");
    expect(
      nativePackagePacket.validation_report.package_hash.payload_excludes,
    ).toBe("validation_report_package_hash_fields");
    expect(nativePackagePacket.validation_report.package_hash.payload_ref).toBe(
      "native-json-preview:project:invented-loop-01",
    );
    expect(
      nativePackagePacket.loss_report.entries[2].affected_refs,
    ).not.toContain("canonical_package_hash");
    expect(nativePackagePacket.diagnostics).toHaveLength(31);
    expect(nativePackagePacket.generation_context.network_required).toBe(false);
    expect(nativePackagePacket.generation_context.telemetry_enabled).toBe(
      false,
    );
    expect(
      nativePackagePacket.generation_context.persisted_editor_intent_count,
    ).toBe(0);
    expect(
      nativePackagePacket.generation_context.persisted_proposal_count,
    ).toBe(0);
    expect(
      nativePackagePacket.generation_context
        .persisted_selected_review_target_count,
    ).toBe(0);
    expect(
      nativePackagePacket.generation_context
        .persisted_selected_review_target_ref,
    ).toBe("not_selected");
    expect(
      nativePackagePacket.generation_context.persisted_mechanics_result_count,
    ).toBe(0);
    expect(
      nativePackagePacket.generation_context.persisted_analysis_run_count,
    ).toBe(0);
    expect(
      nativePackagePacket.generation_context.persisted_analysis_run_ref,
    ).toBe("not_persisted");
    expect(
      nativePackagePacket.generation_context.repository_default_private_write,
    ).toBe(false);
    expect(nativePackagePacket.run_refs.result_count).toBe(830);
    expect(nativePackagePacket.run_refs.hash_refs).toHaveLength(2);
    expect(nativePackagePacket.private_payload_included).toBe(false);
    expect(nativePackagePacket.protected_content_included).toBe(false);
    expect(nativePackagePacket.release_or_professional_claim).toBe(false);
    expect(nativePackagePacket.compatibility_claim_made).toBe(false);
    expect(nativePackagePacket.code_compliance_claim_made).toBe(false);
    expect(
      nativePackagePacket.professional_boundary.software_makes_compliance_claim,
    ).toBe(false);
    expect(
      solveJobPacket.events.map((item: { state: string }) => item.state),
    ).toEqual(["queued", "running", "completed"]);
    expect(solveJobPacket.private_payload_included).toBe(false);
    expect(solveJobPacket.protected_content_included).toBe(false);
    expect(solveJobPacket.release_or_professional_claim).toBe(false);
    expect(
      solveJobPacket.professional_boundary.software_makes_compliance_claim,
    ).toBe(false);
    const ruleCheck = await screen.findByLabelText("Rule-check completeness");
    expect(
      within(ruleCheck).getByTestId("rule-check-summary").textContent,
    ).toContain("4 review findings");
    expect(
      within(ruleCheck).getByTestId("rule-check-summary").textContent,
    ).toContain("rule_check_blocked=true");
    expect(
      within(ruleCheck).getByTestId("rule-check-summary").textContent,
    ).toContain("mechanics_reviewable=true");
    expect(
      within(ruleCheck).getByTestId("rule-check-status").textContent,
    ).toContain("RULE_INPUTS_INCOMPLETE");
    expect(
      within(ruleCheck).getByTestId("rule-check-mechanics-status").textContent,
    ).toContain("MECHANICS_SOLVED");
    expect(
      within(ruleCheck).getByTestId("rule-check-unit-policy").textContent,
    ).toContain("rule_input_units=explicit_or_blocking");
    expect(
      within(ruleCheck).getByTestId("rule-check-unit-policy").textContent,
    ).toContain("conversion=false");
    expect(
      within(ruleCheck).getByTestId("rule-check-boundary").textContent,
    ).toContain("bundled code values=false");
    expect(
      within(ruleCheck).getByTestId(
        "rule-check-finding-professional-acceptance-not-provided",
      ).textContent,
    ).toContain("ASSUMPTION_WARNING");
    const ruleCheckHref =
      within(ruleCheck)
        .getByTestId("rule-check-export-link")
        .getAttribute("href") ?? "";
    const ruleCheckPacket = JSON.parse(
      decodeURIComponent(ruleCheckHref.split(",", 2)[1]),
    );
    expect(ruleCheckPacket.document_kind).toBe(
      "openpipestress.technical_preview.rule_completeness_review",
    );
    expect(ruleCheckPacket.deliverable_refs).toContain("DEL-06-03");
    expect(ruleCheckPacket.deliverable_refs).toContain("DEL-07-04");
    expect(ruleCheckPacket.scope_items).toContain("SOW-004");
    expect(ruleCheckPacket.scope_items).toContain("SOW-022");
    expect(ruleCheckPacket.run_ref).toBe("run:preview-linear-static-001");
    expect(ruleCheckPacket.rule_check_status).toBe("RULE_INPUTS_INCOMPLETE");
    expect(ruleCheckPacket.summary.finding_count).toBe(4);
    expect(ruleCheckPacket.summary.rule_check_blocked).toBe(true);
    expect(ruleCheckPacket.summary.mechanics_results_reviewable).toBe(true);
    expect(ruleCheckPacket.unit_policy_evidence.unit_system_ref.ref).toBe(
      "unit-system:dec-018-si-dual-display",
    );
    expect(ruleCheckPacket.unit_policy_evidence.storage_convention).toBe(
      "entered_units_preserved",
    );
    expect(
      ruleCheckPacket.unit_policy_evidence.rule_completeness_unit_policy,
    ).toBe(
      "rule_completeness_review_records_rule_input_unit_requirements_without_conversion",
    );
    expect(ruleCheckPacket.unit_policy_evidence.model_units).toEqual({
      angle: "rad",
      force: "N",
      length: "m",
      pressure: "Pa",
      stress: "MPa",
      temperature: "degC",
    });
    expect(ruleCheckPacket.unit_policy_evidence.unit_bearing_record_count).toBe(
      38,
    );
    expect(ruleCheckPacket.unit_policy_evidence.rule_input_unit_policy).toBe(
      "required_rule_inputs_must_carry_explicit_units_or_block_user_rule_checks",
    );
    expect(
      ruleCheckPacket.unit_policy_evidence.unit_mismatch_diagnostic_code,
    ).toBe("RULE_UNIT_MISMATCH");
    expect(ruleCheckPacket.unit_policy_evidence.conversion_performed).toBe(
      false,
    );
    expect(ruleCheckPacket.summary.silent_defaults_used).toBe(false);
    expect(ruleCheckPacket.summary.bundled_code_values_used).toBe(false);
    expect(ruleCheckPacket.summary.compliance_claim_made).toBe(false);
    expect(
      ruleCheckPacket.findings.map(
        (item: { warning_class: string }) => item.warning_class,
      ),
    ).toContain("RULE_CHECK_BLOCKING");
    expect(
      ruleCheckPacket.findings.map(
        (item: { warning_class: string }) => item.warning_class,
      ),
    ).toContain("PROVENANCE_WARNING");
    expect(
      ruleCheckPacket.findings.map(
        (item: { warning_class: string }) => item.warning_class,
      ),
    ).toContain("ASSUMPTION_WARNING");
    expect(
      ruleCheckPacket.findings.every(
        (item: {
          protected_content_required: boolean;
          mechanics_solve_blocking: boolean;
        }) =>
          item.protected_content_required === false &&
          item.mechanics_solve_blocking === false,
      ),
    ).toBe(true);
    expect(ruleCheckPacket.private_payload_included).toBe(false);
    expect(ruleCheckPacket.protected_content_included).toBe(false);
    expect(ruleCheckPacket.release_or_professional_claim).toBe(false);
    expect(
      ruleCheckPacket.professional_boundary.software_makes_compliance_claim,
    ).toBe(false);
    const runAudit = await screen.findByLabelText("Run audit");
    expect(
      await within(runAudit).findByTestId("run-audit-model-state"),
    ).toHaveTextContent("ModelState; state:project:invented-loop-01:preview");
    expect(
      within(runAudit).getByTestId("run-audit-analysis-run").textContent,
    ).toContain("DEL-14-02");
    expect(
      within(runAudit).getByTestId("run-audit-analysis-run").textContent,
    ).toContain("mechanics_solve");
    expect(
      within(runAudit).getByTestId("run-audit-analysis-run").textContent,
    ).toContain("run:preview-linear-static-001");
    expect(
      within(runAudit).getByTestId("run-audit-status").textContent,
    ).toContain("HUMAN_REVIEW_REQUIRED");
    expect(
      within(runAudit).getByTestId("run-audit-status").textContent,
    ).toContain("MECHANICS_SOLVED");
    expect(
      within(runAudit).getByTestId("run-audit-status").textContent,
    ).toContain("RULE_INPUTS_INCOMPLETE");
    expect(
      within(runAudit).getByTestId("run-audit-hashes").textContent,
    ).toContain("830 result rows; 830 result value hashes");
    expect(
      within(runAudit).getByTestId("run-audit-hashes").textContent,
    ).toContain("analysis_run_record");
    expect(
      within(runAudit).getByTestId("run-audit-hashes").textContent,
    ).toContain("result_envelope");
    expect(
      within(runAudit).getByTestId("run-audit-units").textContent,
    ).toContain("model=angle=rad,force=N,length=m");
    expect(
      within(runAudit).getByTestId("run-audit-units").textContent,
    ).toContain("N*m/rad,N/m");
    expect(
      within(runAudit).getByTestId("run-audit-units").textContent,
    ).toContain("rows=830");
    expect(
      within(runAudit).getByTestId("run-audit-units").textContent,
    ).toContain("source=result_envelope");
    expect(
      within(runAudit).getByTestId("run-audit-units").textContent,
    ).toContain("conversion=false");
    expect(
      within(runAudit).getByTestId("run-audit-input-manifest").textContent,
    ).toContain("ResultEnvelope:result-envelope:run:preview-linear-static-001");
    expect(
      within(runAudit).getByTestId("run-audit-reproducibility").textContent,
    ).toContain("physical project container");
    expect(
      within(runAudit).getByTestId("run-audit-reproducibility").textContent,
    ).toContain("release-grade solver build provenance");
    expect(
      within(runAudit).getByTestId("run-audit-immutability").textContent,
    ).toContain("read-only run record");
    expect(
      within(runAudit).getByTestId("run-audit-immutability").textContent,
    ).toContain("changes_create_new_analysis_run");
    expect(
      within(runAudit).getByTestId("run-audit-boundary").textContent,
    ).toContain(
      "no compliance, certification, sealing, authentication, or approval claim",
    );
    const comparison = await screen.findByLabelText("Comparison workspace");
    expect(
      within(comparison).getByTestId("comparison-summary").textContent,
    ).toContain("load:L-100; 279 rows");
    expect(
      within(comparison).getByTestId("comparison-summary").textContent,
    ).toContain("combination:C-OPER-ALT; 261 rows");
    expect(
      within(comparison).getByTestId("comparison-summary").textContent,
    ).toContain("261 comparable pairs; 18 reference-only; 0 target-only");
    expect(
      within(comparison).getByTestId("comparison-tolerance-status").textContent,
    ).toContain("not_tolerance_checked");
    expect(
      within(comparison).getByTestId("comparison-unit-policy").textContent,
    ).toContain("units=MPa,N,N*m,mm,rad");
    expect(
      within(comparison).getByTestId("comparison-unit-policy").textContent,
    ).toContain("matching=equal_explicit_units");
    expect(
      within(comparison).getByTestId("comparison-unit-policy").textContent,
    ).toContain("conversion=false");
    const earlyResults = await screen.findByLabelText("Results");
    expect(
      within(earlyResults).getByTestId("result-unit-policy").textContent,
    ).toContain("MPa, N, N*m, mm, rad");
    expect(
      within(earlyResults).getByTestId("result-unit-policy").textContent,
    ).toContain("830 rows");
    expect(
      within(earlyResults).getByTestId("result-unit-policy").textContent,
    ).toContain("entered units preserved");
    expect(
      within(comparison).getByTestId("comparison-mapping-basis").textContent,
    ).toContain("source_result_refs");
    expect(
      within(comparison).getByTestId("comparison-boundary").textContent,
    ).toContain(
      "no compliance, certification, sealing, authentication, or approval claim",
    );
    const comparisonRow = within(comparison).getByTestId(
      "comparison-row-result:combination:combination-C-OPER-ALT:reaction:support-S-120",
    );
    expect(comparisonRow.textContent).toContain("192.514 N");
    expect(
      within(comparison).getByTestId("comparison-diagnostics").textContent,
    ).toContain("1 comparison diagnostic");
    const designWorkspace = await screen.findByLabelText(
      "Design-authoring workspace",
    );
    expect(
      within(designWorkspace).getByTestId("design-workspace-units").textContent,
    ).toContain("N*m/rad,N/m");
    expect(
      within(designWorkspace).getByTestId("design-workspace-units").textContent,
    ).toContain("comparison=MPa,N,N*m,mm,rad");
    expect(
      within(designWorkspace).getByTestId("design-workspace-units").textContent,
    ).toContain("conversion=false");
    const designWorkspaceHref =
      within(designWorkspace)
        .getByTestId("design-workspace-export-link")
        .getAttribute("href") ?? "";
    const solvedDesignWorkspacePacket = JSON.parse(
      decodeURIComponent(designWorkspaceHref.split(",", 2)[1]),
    );
    expect(
      solvedDesignWorkspacePacket.unit_policy_evidence.result_units,
    ).toEqual([
      "MPa",
      "N",
      "N*m",
      "N*m/rad",
      "N/m",
      "boolean",
      "count",
      "m",
      "mm",
      "mode_code",
      "rad",
      "state_code",
    ]);
    expect(
      solvedDesignWorkspacePacket.unit_policy_evidence.comparison_units,
    ).toEqual(["MPa", "N", "N*m", "mm", "rad"]);
    expect(
      solvedDesignWorkspacePacket.unit_policy_evidence.analysis_run_ref.ref,
    ).toBe("run:preview-linear-static-001");
    expect(
      solvedDesignWorkspacePacket.unit_policy_evidence
        .comparison_unit_policy_ref,
    ).toBe("unit-policy-evidence:comparison-workspace-preview");
    fireEvent.click(
      within(comparison).getByTestId(
        "comparison-select-result:combination:combination-C-OPER-ALT:moment:pipe-P-120:bending-z:end-j",
      ),
    );
    expect(
      await screen.findByRole("heading", { name: "Rack span" }),
    ).toBeInTheDocument();
    expect(
      within(comparison).getByTestId("comparison-summary").textContent,
    ).toContain("261 comparable pairs");
    const results = await screen.findByLabelText("Results");
    expect(
      within(results).getByTestId("result-unit-policy").textContent,
    ).toContain("MPa, N, N*m, mm, rad");
    expect(
      within(results).getByTestId("result-unit-policy").textContent,
    ).toContain("830 rows");
    expect(
      within(results).getByTestId("result-unit-policy").textContent,
    ).toContain("entered units preserved");
    expect(
      within(results).getByTestId("result-group-displacement"),
    ).toBeInTheDocument();
    expect(
      within(results).getByTestId("result-group-reaction"),
    ).toBeInTheDocument();
    expect(
      within(results).getByTestId("result-group-other"),
    ).toBeInTheDocument();
    expect(
      within(results).getByTestId("result-family-count-moment").textContent,
    ).toContain("182");
    expect(
      within(results).getByTestId("result-family-count-other").textContent,
    ).toContain("30");
    expect(
      within(results).getByTestId("result-row-result:disp:node-N-140:uz"),
    ).toBeInTheDocument();
    expect(
      within(results).getByText("result:disp:node-N-140"),
    ).toBeInTheDocument();
    expect(within(results).getByText(/3.686623 mm/i)).toBeInTheDocument();
    expect(
      within(results).getByTestId("result-filter-summary").textContent,
    ).toContain("830 of 830 results match filter");
    expect(
      within(results).getByTestId("result-page-summary").textContent,
    ).toContain("Showing 1 to 50 of 830 matching results; page 1 of 17");
    expect(within(results).getByTestId("previous-result-page")).toBeDisabled();
    expect(within(results).getByTestId("next-result-page")).not.toBeDisabled();
    expect(
      within(results).getByTestId("result-family-count-all").textContent,
    ).toContain("830");
    expect(
      within(results).getByTestId("result-family-count-displacement")
        .textContent,
    ).toContain("111");
    expect(
      within(results).getByTestId("result-family-count-reaction").textContent,
    ).toContain("33");
    expect(
      within(results).getByTestId("result-family-count-force").textContent,
    ).toContain("182");
    expect(
      within(results).getByTestId("result-family-count-moment").textContent,
    ).toContain("182");
    expect(
      within(results).getByTestId("result-family-count-stress").textContent,
    ).toContain("290");

    fireEvent.click(within(results).getByTestId("result-family-reaction"));
    expect(
      within(results).getByTestId("result-family-reaction"),
    ).toHaveAttribute("aria-pressed", "true");
    expect(
      within(results).getByTestId("result-filter-summary").textContent,
    ).toContain("33 of 830 results match filter");
    expect(
      within(results).getByTestId("result-page-summary").textContent,
    ).toContain("Showing 1 to 33 of 33 matching results; page 1 of 1");
    expect(
      within(results).getByTestId("result-group-reaction"),
    ).toBeInTheDocument();
    expect(
      within(results).queryByTestId("result-group-force"),
    ).not.toBeInTheDocument();
    expect(
      within(results).getByTestId("result-row-result:reaction:support-S-120"),
    ).toBeInTheDocument();

    fireEvent.click(within(results).getByTestId("result-family-stress"));
    expect(within(results).getByTestId("result-family-stress")).toHaveAttribute(
      "aria-pressed",
      "true",
    );
    expect(
      within(results).getByTestId("result-filter-summary").textContent,
    ).toContain("290 of 830 results match filter");
    expect(
      within(results).getByTestId("result-page-summary").textContent,
    ).toContain("Showing 1 to 50 of 290 matching results; page 1 of 6");
    expect(
      within(results).getByTestId("result-group-stress"),
    ).toBeInTheDocument();
    expect(
      within(results).queryByTestId("result-group-force"),
    ).not.toBeInTheDocument();
    expect(
      within(results).getByTestId(
        "result-row-result:stress:pipe-P-120:end-i:torsional-shear",
      ),
    ).toBeInTheDocument();

    fireEvent.click(within(results).getByTestId("result-family-all"));
    expect(within(results).getByTestId("result-family-all")).toHaveAttribute(
      "aria-pressed",
      "true",
    );
    expect(
      within(results).getByTestId("result-filter-summary").textContent,
    ).toContain("830 of 830 results match filter");

    fireEvent.click(within(results).getByTestId("next-result-page"));
    expect(
      within(results).getByTestId("result-page-summary").textContent,
    ).toContain("Showing 51 to 100 of 830 matching results; page 2 of 17");
    expect(
      within(results).getByTestId("result-row-result:force:pipe-P-100:axial"),
    ).toBeInTheDocument();
    fireEvent.click(within(results).getByTestId("previous-result-page"));
    expect(
      within(results).getByTestId("result-page-summary").textContent,
    ).toContain("Showing 1 to 50 of 830 matching results; page 1 of 17");

    fireEvent.change(within(results).getByTestId("result-filter-input"), {
      target: { value: "torsional-shear" },
    });
    expect(
      within(results).getByTestId("result-filter-summary").textContent,
    ).toContain("68 of 830 results match filter");
    expect(
      within(results).getByTestId("result-page-summary").textContent,
    ).toContain("Showing 1 to 50 of 68 matching results; page 1 of 2");
    expect(
      within(results).getByTestId("result-group-stress"),
    ).toBeInTheDocument();
    expect(
      within(results).queryByTestId("result-group-force"),
    ).not.toBeInTheDocument();
    expect(
      within(results).getByTestId(
        "result-row-result:stress:pipe-P-120:end-j:torsional-shear",
      ),
    ).toBeInTheDocument();

    fireEvent.click(within(results).getByTestId("clear-result-filter"));
    expect(
      within(results).getByTestId("result-filter-summary").textContent,
    ).toContain("830 of 830 results match filter");
    expect(
      within(results).getByTestId("result-group-other"),
    ).toBeInTheDocument();

    fireEvent.change(within(results).getByTestId("result-filter-input"), {
      target: { value: "pipe-P-120" },
    });
    expect(
      within(results).getByTestId("result-filter-summary").textContent,
    ).toContain("170 of 830 results match filter");
    expect(
      within(results).getByTestId("result-page-summary").textContent,
    ).toContain("Showing 1 to 50 of 170 matching results; page 1 of 4");
    expect(
      within(results).getByTestId("result-row-result:force:pipe-P-120:axial"),
    ).toBeInTheDocument();
    expect(
      within(results).getByTestId(
        "result-row-result:force:pipe-P-120:axial:end-j",
      ),
    ).toBeInTheDocument();
    expect(
      within(results).getByTestId("result-row-result:force:pipe-P-120:shear-y"),
    ).toBeInTheDocument();
    expect(
      within(results).getByTestId(
        "result-row-result:force:pipe-P-120:shear-y:end-j",
      ),
    ).toBeInTheDocument();
    expect(
      within(results).getByTestId(
        "result-row-result:force:pipe-P-120:quarter-1:shear-y",
      ),
    ).toBeInTheDocument();
    expect(
      within(results).getByTestId(
        "result-row-result:force:pipe-P-120:midspan:axial",
      ),
    ).toBeInTheDocument();
    expect(
      within(results).getByTestId(
        "result-row-result:force:pipe-P-120:midspan:shear-z",
      ),
    ).toBeInTheDocument();
    expect(
      within(results).getByTestId(
        "result-row-result:force:pipe-P-120:quarter-3:shear-z",
      ),
    ).toBeInTheDocument();
    expect(
      within(results).getByTestId(
        "result-row-result:moment:pipe-P-120:quarter-1:bending-z",
      ),
    ).toBeInTheDocument();
    expect(
      within(results).getByTestId(
        "result-row-result:stress:pipe-P-120:end-j:torsional-shear",
      ),
    ).toBeInTheDocument();
    expect(
      within(results).getByTestId(
        "result-row-result:stress:pipe-P-120:quarter-1:torsional-shear",
      ),
    ).toBeInTheDocument();
    expect(
      within(results).getByTestId(
        "result-row-result:stress:pipe-P-120:midspan:torsional-shear",
      ),
    ).toBeInTheDocument();

    fireEvent.click(
      within(results).getByTestId("result-row-result:force:pipe-P-120:axial"),
    );
    const detail = within(results).getByTestId("result-detail-panel");
    expect(
      within(detail).getByTestId("selected-result-id").textContent,
    ).toContain("result:force:pipe-P-120:axial");
    expect(
      within(detail).getByTestId("selected-result-component").textContent,
    ).toContain("axial_force");
    expect(
      within(detail).getByTestId("selected-result-coordinate-system")
        .textContent,
    ).toContain("element_local");
    expect(
      within(detail).getByTestId("selected-result-location").textContent,
    ).toContain("end_i");
    expect(
      within(detail).getByTestId("selected-result-entity-ref").textContent,
    ).toContain("pipe:P-120");
    expect(
      within(detail).getByTestId("selected-result-recovery-basis").textContent,
    ).toContain("recovered_from_local_element_stiffness");
    expect(
      within(detail).getByTestId("selected-result-recovery-basis").textContent,
    ).toContain("load_case:load:L-100");
    expect(
      within(detail).getByTestId("selected-result-sign-convention").textContent,
    ).toContain("positive value follows");
    expect(
      within(detail).getByTestId("selected-result-source-refs").textContent,
    ).toContain("not a combined result");
    expect(
      within(detail).getByTestId("endpoint-pair-table").textContent,
    ).toContain("end_i");
    expect(
      within(detail).getByTestId("endpoint-pair-table").textContent,
    ).toContain("result:force:pipe-P-120:axial:end-j");
    expect(
      await screen.findByRole("heading", { name: "Rack span" }),
    ).toBeInTheDocument();

    fireEvent.click(
      within(results).getByTestId(
        "result-row-result:force:pipe-P-120:axial:end-j",
      ),
    );
    expect(
      within(detail).getByTestId("selected-result-id").textContent,
    ).toContain("result:force:pipe-P-120:axial:end-j");
    expect(
      within(detail).getByTestId("selected-result-location").textContent,
    ).toContain("end_j");
    expect(
      within(detail).getByTestId("selected-result-sign-convention").textContent,
    ).toContain("j-end");
    expect(
      within(detail).getByTestId("endpoint-pair-table").textContent,
    ).toContain("result:force:pipe-P-120:axial");

    fireEvent.click(
      within(results).getByTestId("result-row-result:force:pipe-P-120:shear-y"),
    );
    expect(
      within(detail).getByTestId("selected-result-id").textContent,
    ).toContain("result:force:pipe-P-120:shear-y");
    expect(
      within(detail).getByTestId("selected-result-component").textContent,
    ).toContain("shear_force_y");
    expect(
      within(detail).getByTestId("endpoint-pair-table").textContent,
    ).toContain("result:force:pipe-P-120:shear-y:end-j");

    fireEvent.click(
      within(results).getByTestId(
        "result-row-result:force:pipe-P-120:midspan:axial",
      ),
    );
    expect(
      within(detail).getByTestId("selected-result-id").textContent,
    ).toContain("result:force:pipe-P-120:midspan:axial");
    expect(
      within(detail).getByTestId("selected-result-location").textContent,
    ).toContain("midspan");
    expect(
      within(detail).getByTestId("selected-result-recovery-basis").textContent,
    ).toContain("interpolated_from_endpoint_resultants");
    expect(
      within(detail).queryByTestId("endpoint-pair-table"),
    ).not.toBeInTheDocument();

    fireEvent.click(
      within(results).getByTestId(
        "result-row-result:force:pipe-P-120:quarter-1:shear-y",
      ),
    );
    expect(
      within(detail).getByTestId("selected-result-id").textContent,
    ).toContain("result:force:pipe-P-120:quarter-1:shear-y");
    expect(
      within(detail).getByTestId("selected-result-component").textContent,
    ).toContain("shear_force_y");
    expect(
      within(detail).getByTestId("selected-result-location").textContent,
    ).toContain("quarter_1");
    expect(
      within(detail).getByTestId("selected-result-recovery-basis").textContent,
    ).toContain("interpolated_from_endpoint_resultants");
    expect(
      within(detail).queryByTestId("endpoint-pair-table"),
    ).not.toBeInTheDocument();

    fireEvent.change(within(results).getByTestId("result-filter-input"), {
      target: { value: "combination-C-OPER-ALT:force:pipe-P-120" },
    });
    expect(
      within(results).getByTestId("result-filter-summary").textContent,
    ).toContain("15 of 830 results match filter");
    expect(
      within(results).getByTestId(
        "result-row-result:combination:combination-C-OPER-ALT:force:pipe-P-120:axial",
      ),
    ).toBeInTheDocument();
    expect(
      within(results).getByTestId(
        "result-row-result:combination:combination-C-OPER-ALT:force:pipe-P-120:quarter-1:shear-y",
      ),
    ).toBeInTheDocument();
    fireEvent.click(
      within(results).getByTestId(
        "result-row-result:combination:combination-C-OPER-ALT:force:pipe-P-120:axial",
      ),
    );
    expect(
      within(detail).getByTestId("selected-result-id").textContent,
    ).toContain(
      "result:combination:combination-C-OPER-ALT:force:pipe-P-120:axial",
    );
    expect(
      within(detail).getByTestId("selected-result-recovery-basis").textContent,
    ).toContain("explicit_user_linear_combination");
    expect(
      within(detail).getByTestId("selected-result-recovery-basis").textContent,
    ).toContain("combination:combination:C-OPER-ALT");
    expect(
      within(detail).getByTestId("selected-result-source-refs").textContent,
    ).toContain("result:force:pipe-P-120:axial");
    expect(
      within(detail).getByTestId("selected-result-source-refs").textContent,
    ).toContain("result:loadcase:load-L-200:force:pipe-P-120:axial");

    fireEvent.change(within(results).getByTestId("result-filter-input"), {
      target: { value: "stress:pipe-P-120:end-j:torsional-shear" },
    });
    expect(
      within(results).getByTestId("result-filter-summary").textContent,
    ).toContain("3 of 830 results match filter");
    expect(
      within(results).getByTestId(
        "result-row-result:stress:pipe-P-120:end-j:torsional-shear",
      ),
    ).toBeInTheDocument();
    fireEvent.click(
      within(results).getByTestId(
        "result-row-result:stress:pipe-P-120:end-j:torsional-shear",
      ),
    );
    expect(
      within(detail).getByTestId("selected-result-id").textContent,
    ).toContain("result:stress:pipe-P-120:end-j:torsional-shear");
    expect(
      within(detail).getByTestId("selected-result-component").textContent,
    ).toContain("torsional_shear_stress");
    expect(
      within(detail).getByTestId("selected-result-coordinate-system")
        .textContent,
    ).toContain("element_local");
    expect(
      within(detail).getByTestId("selected-result-location").textContent,
    ).toContain("end_j");
    expect(
      within(detail).getByTestId("selected-result-entity-ref").textContent,
    ).toContain("pipe:P-120");
    expect(
      within(detail).getByTestId("selected-result-recovery-basis").textContent,
    ).toContain("recovered_from_open_mechanics_stress_components");
    expect(
      within(detail).getByTestId("endpoint-pair-table").textContent,
    ).toContain("result:stress:pipe-P-120:end-i:torsional-shear");

    const gapLedger = within(results).getByTestId("mechanics-gap-ledger");
    expect(
      within(gapLedger).getByTestId("gap:endpoint-j-recovery").textContent,
    ).toContain("implemented");
    expect(
      within(gapLedger).getByTestId("gap:endpoint-stress-components")
        .textContent,
    ).toContain("implemented");
    expect(
      within(gapLedger).getByTestId("gap:station-recovery").textContent,
    ).toContain("implemented");
    expect(
      within(gapLedger).getByTestId("gap:station-recovery").textContent,
    ).toContain("Fixed station-grid");
    expect(
      within(gapLedger).getByTestId("gap:station-recovery").textContent,
    ).toContain("shear stress recovery remain deferred");
    expect(
      within(gapLedger).getByTestId("gap:thermal-behavior").textContent,
    ).toContain("implemented");
    expect(
      within(gapLedger).getByTestId("gap:load-combinations").textContent,
    ).toContain("implemented");
    expect(
      within(gapLedger).getByTestId("gap:endpoint-j-recovery").textContent,
    ).not.toContain("compliance failure");

    const knowledge = await screen.findByLabelText("Design knowledge");
    expect(
      within(knowledge).getByTestId("knowledge-unit-context").textContent,
    ).toContain("computed_unit_refs=2");
    expect(
      within(knowledge).getByTestId("knowledge-unit-context").textContent,
    ).toContain("units=N,mm");
    expect(
      within(knowledge).getByTestId("knowledge-unit-context").textContent,
    ).toContain("source=computed_preview_result");
    expect(
      within(knowledge).getByTestId("knowledge-unit-context").textContent,
    ).toContain("conversion=false");
    expect(
      within(knowledge).getByText(/Computed displacement review/i),
    ).toBeInTheDocument();
    expect(
      within(knowledge).getByText(/result:disp:node-N-140 is 3.686623 mm/i),
    ).toBeInTheDocument();
    expect(
      within(knowledge).getByText(/result:force:pipe-P-120:axial is/i),
    ).toBeInTheDocument();

    const report = await screen.findByLabelText("Report packet");
    expect(
      within(report).getByTestId("report-packet-body"),
    ).toBeInTheDocument();
    expect(
      within(report).getByTestId("report-selected-result-refs").textContent,
    ).toContain("result:disp:node-N-140");
    expect(
      within(report).getByTestId("report-selected-result-refs").textContent,
    ).toContain("result:force:pipe-P-120:axial");
    expect(
      within(report).getByTestId("report-selected-result-refs").textContent,
    ).toContain("result:force:pipe-P-120:axial:end-j");
    expect(
      within(report).getByTestId("report-selected-result-refs").textContent,
    ).toContain("result:force:pipe-P-120:midspan:axial");
    expect(
      within(report).getByTestId("report-selected-result-refs").textContent,
    ).toContain("result:force:pipe-P-120:quarter-1:shear-y");
    expect(
      within(report).getByTestId("report-selected-result-refs").textContent,
    ).toContain("result:force:pipe-P-120:shear-y");
    expect(
      within(report).getByTestId("report-selected-result-refs").textContent,
    ).toContain(
      "result:combination:combination-C-OPER-ALT:force:pipe-P-120:axial",
    );
    expect(
      within(report).getByTestId("report-selected-result-refs").textContent,
    ).toContain(
      "result:combination:combination-C-OPER-ALT:force:pipe-P-120:quarter-1:shear-y",
    );
    expect(
      within(report).getByTestId("report-selected-result-refs").textContent,
    ).toContain("result:pressure-thrust:component-C-150");
    expect(
      within(report).getByTestId("report-selected-result-refs").textContent,
    ).toContain(
      "result:combination:combination-C-OPER-ALT:pressure-thrust:component-C-150",
    );
    expect(
      within(report).getByTestId("report-selected-result-refs").textContent,
    ).toContain("result:stress:pipe-P-120:end-j:torsional-shear");
    expect(
      within(report).getByTestId("report-selected-result-refs").textContent,
    ).toContain("result:stress:pipe-P-120:quarter-1:torsional-shear");
    expect(
      within(report).getByTestId("report-selected-review-target").textContent,
    ).toContain("result: result:stress:pipe-P-120:end-j:torsional-shear");
    expect(
      within(report).getByTestId("report-analysis-run").textContent,
    ).toContain("DEL-14-02");
    expect(
      within(report).getByTestId("report-analysis-run").textContent,
    ).toContain("run:preview-linear-static-001");
    expect(
      within(report).getByTestId("report-load-basis-refs").textContent,
    ).toContain("load:L-100");
    expect(
      within(report).getByTestId("report-load-basis-refs").textContent,
    ).toContain("load:L-200");
    expect(
      within(report).getByTestId("report-load-basis-refs").textContent,
    ).toContain("combination:C-OPER-ALT");
    expect(
      within(report).getByTestId("report-diagnostic-scope").textContent,
    ).toContain("model, design knowledge, and computed mechanics findings");
    expect(
      within(report).getByTestId("report-component-provenance").textContent,
    ).toContain("component:C-110");
    expect(
      within(report).getByTestId("report-component-provenance").textContent,
    ).toContain("invented_user_entered_preview_no_code_table");
    expect(
      within(report).getByTestId("report-component-provenance").textContent,
    ).toContain("mechanics_geometry_only");
    expect(
      within(report).getByTestId("report-component-provenance").textContent,
    ).toContain("component:C-130");
    expect(
      within(report).getByTestId("report-component-provenance").textContent,
    ).toContain("component:C-130->pipe:P-130");
    expect(
      within(report).getByTestId("report-component-provenance").textContent,
    ).toContain("component:C-150->pipe:P-130");
    expect(
      within(report).getByTestId("report-component-stress-modifiers")
        .textContent,
    ).toContain("user-entered multiplier");
    expect(
      within(report).getByTestId("report-component-stress-modifiers")
        .textContent,
    ).toContain("component:C-110");
    expect(
      within(report).getByTestId("report-component-stiffness-inputs")
        .textContent,
    ).toContain("4 user-entered stiffness rows");
    expect(
      within(report).getByTestId("report-component-stiffness-inputs")
        .textContent,
    ).toContain("component:C-150");
    expect(
      within(report).getByTestId("report-component-pressure-thrust")
        .textContent,
    ).toContain("3 load-side pressure thrust rows");
    expect(
      within(report).getByTestId("report-component-pressure-thrust")
        .textContent,
    ).toContain("component:C-150");
    expect(within(report).getByText(/33 review findings/i)).toBeInTheDocument();
    expect(
      within(report).getByTestId("report-diagnostic-summary").textContent,
    ).toContain("12 warnings; 21 info; 0 errors; 0 blocking");
    expect(
      within(report).getByText(/result value hashes/i),
    ).toBeInTheDocument();
    expect(within(report).getByText(/result_envelope/i)).toBeInTheDocument();
    expect(
      within(report).getByText(/no compliance or professional approval claim/i),
    ).toBeInTheDocument();
    expect(
      within(report).getByTestId("report-comparison-summary").textContent,
    ).toContain("261 mapped pairs");
    expect(
      within(report).getByTestId("report-comparison-summary").textContent,
    ).toContain("not_tolerance_checked");
    expect(
      within(report).getByTestId("report-project-persistence").textContent,
    ).toContain("storage=not_persisted_this_session");
    expect(
      within(report).getByTestId("report-project-persistence").textContent,
    ).toContain("validation=preview_not_persisted");
    expect(
      within(report).getByTestId("report-export-readiness").textContent,
    ).toContain("27 of 29 local exports ready");
    expect(
      within(report).getByTestId("report-export-readiness").textContent,
    ).toContain("storage=available");
    expect(
      within(report).getByTestId("report-export-readiness").textContent,
    ).toContain("validation=available");
    expect(
      within(report).getByTestId("report-export-readiness").textContent,
    ).toContain("telemetry=available");
    expect(
      within(report).getByTestId("report-export-readiness").textContent,
    ).toContain("secrets=available");
    expect(
      within(report).getByTestId("report-export-readiness").textContent,
    ).toContain("threats=available");
    expect(
      within(report).getByTestId("report-export-readiness").textContent,
    ).toContain("accessibility=available");
    expect(
      within(report).getByTestId("report-export-readiness").textContent,
    ).toContain("workspace=available");
    expect(
      within(report).getByTestId("report-export-readiness").textContent,
    ).toContain("evidence=available");
    expect(
      within(report).getByTestId("report-storage-boundary").textContent,
    ).toContain("network=false");
    expect(
      within(report).getByTestId("report-storage-boundary").textContent,
    ).toContain("private/protected payload=false");
    expect(
      within(report).getByTestId("report-storage-boundary").textContent,
    ).toContain("accepted_state_mutated=false");
    expect(
      within(report).getByTestId("report-export-summary").textContent,
    ).toContain("no private payload");
    expect(
      within(report).getByTestId("report-export-summary").textContent,
    ).toContain("33 diagnostics");
    const exportHref =
      within(report).getByTestId("report-export-link").getAttribute("href") ??
      "";
    expect(exportHref).toContain("data:application/json");
    const exportPacket = JSON.parse(
      decodeURIComponent(exportHref.split(",", 2)[1]),
    );
    expect(exportPacket.document_kind).toBe(
      "openpipestress.technical_preview.report_packet_export",
    );
    expect(exportPacket.export_scope).toBe("local_browser_download_preview");
    expect(exportPacket.deliverable_refs).toContain("DEL-03-03");
    expect(exportPacket.deliverable_refs).toContain("DEL-03-04");
    expect(exportPacket.deliverable_refs).toContain("DEL-03-05");
    expect(exportPacket.deliverable_refs).toContain("DEL-03-06");
    expect(exportPacket.deliverable_refs).toContain("DEL-05-03");
    expect(exportPacket.deliverable_refs).toContain("DEL-08-01");
    expect(exportPacket.deliverable_refs).toContain("DEL-07-03");
    expect(exportPacket.deliverable_refs).toContain("DEL-07-04");
    expect(exportPacket.deliverable_refs).toContain("DEL-07-06");
    expect(exportPacket.deliverable_refs).toContain("DEL-07-08");
    expect(exportPacket.deliverable_refs).toContain("DEL-08-03");
    expect(exportPacket.deliverable_refs).toContain("DEL-08-06");
    expect(exportPacket.deliverable_refs).toContain("DEL-09-04");
    expect(exportPacket.deliverable_refs).toContain("DEL-09-05");
    expect(exportPacket.deliverable_refs).toContain("DEL-10-04");
    expect(exportPacket.deliverable_refs).toContain("DEL-15-04");
    expect(exportPacket.deliverable_refs).toContain("DEL-17-04");
    expect(exportPacket.deliverable_refs).toContain("DEL-17-06");
    expect(exportPacket.deliverable_refs).toContain("DEL-17-08");
    expect(exportPacket.deliverable_refs).toContain("DEL-02-05");
    expect(exportPacket.deliverable_refs).toContain("DEL-12-01");
    expect(exportPacket.deliverable_refs).toContain("DEL-12-03");
    expect(exportPacket.deliverable_refs).toContain("DEL-12-04");
    expect(exportPacket.deliverable_refs).toContain("DEL-12-05");
    expect(exportPacket.deliverable_refs).toContain("DEL-12-02");
    expect(exportPacket.selected_result_refs).toContain(
      "result:force:pipe-P-120:axial",
    );
    expect(exportPacket.selected_result_refs).toContain(
      "result:stress:component-C-110:pipe-P-100:end-j:user-multiplier",
    );
    expect(exportPacket.selected_result_refs).toContain(
      "result:stress:pipe-P-120:end-j:torsional-shear",
    );
    expect(exportPacket.selected_result_refs).toContain(
      "result:pressure-thrust:component-C-150",
    );
    expect(exportPacket.selected_result_refs).toContain(
      "result:combination:combination-C-OPER-ALT:pressure-thrust:component-C-150",
    );
    expect(exportPacket.selected_review_target).toEqual({
      target_type: "result",
      id: "result:stress:pipe-P-120:end-j:torsional-shear",
    });
    expect(exportPacket.diagnostic_refs).toContain("RULE_INPUTS_MISSING");
    expect(exportPacket.diagnostic_refs).toContain(
      "SUPPORT_STIFFNESS_UNRESOLVED",
    );
    expect(exportPacket.diagnostic_refs).toContain(
      "diagnostic:combination:combination-C-OPER-ALT:result-stress-pipe-P-130:COMBINATION_STRESS_SUMMARY_SKIPPED",
    );
    expect(exportPacket.diagnostic_refs).toContain(
      "diagnostic:component-stress-multiplier:component-C-110:pipe-P-100:end-j",
    );
    expect(exportPacket.diagnostic_refs).toContain(
      "diagnostic:component:component-C-150:expansion-joint-review",
    );
    expect(exportPacket.diagnostic_refs).toContain(
      "diagnostic:pressure-thrust:load-L-100:component-C-150",
    );
    expect(exportPacket.diagnostic_refs).toContain(
      "diagnostic:pressure-thrust:load-L-200:component-C-150",
    );
    expect(exportPacket.diagnostic_refs).toContain(
      "diagnostic:spring-hanger:support-SH-140:hanger",
    );
    expect(exportPacket.diagnostic_refs).toContain(
      "diagnostic:spring-hanger:support-CE-120:hanger",
    );
    expect(exportPacket.diagnostic_summary.total).toBe(33);
    expect(exportPacket.diagnostic_summary.by_severity.warning).toBe(12);
    expect(exportPacket.diagnostic_summary.by_severity.info).toBe(21);
    expect(exportPacket.component_stress_modifier_count).toBe(12);
    expect(exportPacket.component_provenance[0]).toEqual(
      expect.objectContaining({
        component_ref: "component:C-110",
        modifier_source_ref: "invented_user_entered_preview_no_code_table",
        solver_consumption: "mechanics_geometry_only",
        protected_content_included: false,
      }),
    );
    expect(
      exportPacket.component_provenance[0].stress_modifier_result_refs,
    ).toContain(
      "result:stress:component-C-110:pipe-P-100:end-j:user-multiplier",
    );
    expect(exportPacket.component_provenance).toEqual(
      expect.arrayContaining([
        expect.objectContaining({
          component_ref: "component:C-120",
          component_kind: "branch",
          geometry_source_ref: "invented_user_entered_branch_preview_geometry",
          modifier_source_ref:
            "invented_user_entered_branch_modifiers_no_code_table",
          user_entered_header_sif: { value: 1.22, unit: "none" },
          user_entered_branch_sif: { value: 1.31, unit: "none" },
          protected_content_included: false,
        }),
        expect.objectContaining({
          component_ref: "component:C-130",
          component_kind: "valve",
          geometry_source_ref:
            "invented_user_entered_rigid_component_preview_geometry",
          modifier_source_ref:
            "invented_user_entered_rigid_stiffness_no_catalog_or_code_table",
          rigid_pipe_ref: "pipe:P-130",
          user_entered_stiffness_scale: { value: 15, unit: "none" },
          user_entered_linear_stiffness: { value: 15000000, unit: "N/m" },
          user_entered_rotational_stiffness: { value: 850000, unit: "N*m/rad" },
          protected_content_included: false,
        }),
        expect.objectContaining({
          component_ref: "component:C-150",
          component_kind: "expansion_joint",
          geometry_source_ref:
            "invented_user_entered_expansion_joint_preview_geometry",
          modifier_source_ref:
            "invented_user_entered_expansion_joint_stiffness_no_catalog_or_code_table",
          expansion_joint_pipe_ref: "pipe:P-130",
          effective_area: { value: 0.018, unit: "m^2" },
          movement_limit: { value: 0.045, unit: "m" },
          pressure_thrust_reference:
            "load_side_pressure_thrust_user_review_required",
          user_entered_axial_stiffness: { value: 3200000, unit: "N/m" },
          user_entered_torsional_stiffness: { value: 620000, unit: "N*m/rad" },
          protected_content_included: false,
        }),
      ]),
    );
    expect(exportPacket.component_user_stiffness_macro_element_count).toBe(4);
    expect(
      exportPacket.component_user_stiffness_macro_element_evidence,
    ).toEqual(
      expect.arrayContaining([
        expect.objectContaining({
          result_ref: "result:component-stiffness:component-C-150:axial",
          component_ref: "component:C-150",
          component_kind: "expansion_joint",
          modifier_source_ref:
            "invented_user_entered_expansion_joint_stiffness_no_catalog_or_code_table",
          solver_consumption: "mechanics_geometry_and_user_flexibility",
          pressure_thrust_reference:
            "load_side_pressure_thrust_user_review_required",
          protected_content_included: false,
        }),
      ]),
    );
    expect(exportPacket.component_pressure_thrust_load_count).toBe(3);
    expect(exportPacket.component_pressure_thrust_evidence).toEqual(
      expect.arrayContaining([
        expect.objectContaining({
          result_ref: "result:pressure-thrust:component-C-150",
          component_ref: "component:C-150",
          component_kind: "expansion_joint",
          value: 21600,
          unit: "N",
          mapped_pipe_ref: "pipe:P-130",
          effective_area: { value: 0.018, unit: "m^2" },
          source_result_refs: ["load:L-100-P-EJ"],
          geometry_source_ref:
            "invented_user_entered_expansion_joint_preview_geometry",
          pressure_thrust_reference:
            "load_side_pressure_thrust_user_review_required",
          solver_consumption: "mechanics_geometry_and_user_flexibility",
          protected_content_included: false,
        }),
        expect.objectContaining({
          result_ref:
            "result:combination:combination-C-OPER-ALT:pressure-thrust:component-C-150",
          component_ref: "component:C-150",
          value: 27000,
          source_result_refs: [
            "result:pressure-thrust:component-C-150",
            "result:loadcase:load-L-200:pressure-thrust:component-C-150",
          ],
          protected_content_included: false,
        }),
      ]),
    );
    expect(exportPacket.component_stress_modifier_evidence).toEqual(
      expect.arrayContaining([
        expect.objectContaining({
          result_ref:
            "result:stress:component-C-110:pipe-P-100:end-j:user-multiplier",
          component_ref: "component:C-110",
          modifier_source_ref: "invented_user_entered_preview_no_code_table",
          solver_consumption: "mechanics_geometry_only",
          protected_content_included: false,
        }),
        expect.objectContaining({
          result_ref:
            "result:stress:component-C-120:pipe-P-120:end-i:user-multiplier",
          component_ref: "component:C-120",
          component_kind: "branch",
          modifier_source_ref:
            "invented_user_entered_branch_modifiers_no_code_table",
          solver_consumption: "mechanics_geometry_only",
          protected_content_included: false,
        }),
      ]),
    );
    expect(exportPacket.spring_hanger_evidence_count).toBe(2);
    expect(exportPacket.spring_hanger_evidence).toEqual(
      expect.arrayContaining([
        expect.objectContaining({
          support_ref: "support:SH-140",
          hanger_type: "variable_spring_hanger",
          user_entered_stiffness: { value: 42000, unit: "N/m" },
          installed_load: { value: 460, unit: "N" },
          cold_load: { value: 430, unit: "N" },
          hot_load: { value: 390, unit: "N" },
          source_reference:
            "invented_user_entered_spring_hanger_values_no_catalog",
          mechanics_consumption: "linear_spring_primitive_user_stiffness",
          protected_content_included: false,
        }),
        expect.objectContaining({
          support_ref: "support:CE-120",
          hanger_type: "constant_effort_support",
          constant_load: { value: 375, unit: "N" },
          travel_range: { value: 0.04, unit: "m" },
          mechanics_consumption:
            "load_side_review_only_no_global_solve_consumption",
          protected_content_included: false,
        }),
      ]),
    );
    expect(exportPacket.load_basis_refs).toContain("combination:C-OPER-ALT");
    expect(exportPacket.hash_refs.envelope_hash_scopes).toContain(
      "result_envelope",
    );
    expect(exportPacket.persistence_evidence.document_kind).toBe(
      "openpipestress.technical_preview.report_persistence_export_context",
    );
    expect(exportPacket.persistence_evidence.storage_audit.document_kind).toBe(
      "openpipestress.technical_preview.local_project_persistence_audit",
    );
    expect(exportPacket.persistence_evidence.storage_audit.storage_mode).toBe(
      "not_persisted_this_session",
    );
    expect(
      exportPacket.persistence_evidence.storage_audit.pending_operation_count,
    ).toBe(0);
    expect(
      exportPacket.persistence_evidence.validation_preflight.document_kind,
    ).toBe("openpipestress.technical_preview.project_validation_preflight");
    expect(
      exportPacket.persistence_evidence.validation_preflight.validation_status,
    ).toBe("preview_not_persisted");
    expect(
      exportPacket.persistence_evidence.validation_preflight.round_trip_status,
    ).toBe("semantic_categories_declared");
    expect(
      exportPacket.persistence_evidence.export_inventory.expected_export_count,
    ).toBe(29);
    expect(
      exportPacket.persistence_evidence.export_inventory.available_count,
    ).toBe(27);
    expect(
      exportPacket.persistence_evidence.export_inventory.readiness_by_export_id
        .project_storage_audit,
    ).toBe("available");
    expect(
      exportPacket.persistence_evidence.export_inventory.readiness_by_export_id
        .project_validation_preflight,
    ).toBe("available");
    expect(
      exportPacket.persistence_evidence.export_inventory.readiness_by_export_id
        .telemetry_boundary_review,
    ).toBe("available");
    expect(
      exportPacket.persistence_evidence.export_inventory.readiness_by_export_id
        .secret_private_library_boundary_review,
    ).toBe("available");
    expect(
      exportPacket.persistence_evidence.export_inventory.readiness_by_export_id
        .security_threat_model_review,
    ).toBe("available");
    expect(
      exportPacket.persistence_evidence.export_inventory.readiness_by_export_id
        .editor_contract_review,
    ).toBe("available");
    expect(
      exportPacket.persistence_evidence.export_inventory.readiness_by_export_id
        .missing_data_warning_blocking_review,
    ).toBe("available");
    expect(
      exportPacket.persistence_evidence.export_inventory.readiness_by_export_id
        .rule_completeness_review,
    ).toBe("available");
    expect(
      exportPacket.persistence_evidence.export_inventory.readiness_by_export_id
        .agent_proposal_review,
    ).toBe("pending_agent_proposal");
    expect(
      exportPacket.persistence_evidence.export_inventory.readiness_by_export_id
        .accessibility_usability_baseline_review,
    ).toBe("available");
    expect(
      exportPacket.persistence_evidence.export_inventory.readiness_by_export_id
        .design_authoring_comparison_workspace,
    ).toBe("available");
    expect(
      exportPacket.persistence_evidence.export_inventory.readiness_by_export_id
        .build_package_readiness,
    ).toBe("available");
    expect(
      exportPacket.persistence_evidence.export_inventory.readiness_by_export_id
        .validation_release_evidence_review,
    ).toBe("available");
    expect(
      exportPacket.persistence_evidence.export_inventory.readiness_by_export_id
        .report_protected_content_lint,
    ).toBe("available");
    expect(
      exportPacket.persistence_evidence.export_inventory.readiness_by_export_id
        .stress_neutral_csv_json_package,
    ).toBe("available");
    expect(
      exportPacket.persistence_evidence.export_inventory.readiness_by_export_id
        .headless_runner_envelope,
    ).toBe("available");
    expect(
      exportPacket.persistence_evidence.export_inventory.readiness_by_export_id
        .adapter_framework_envelope,
    ).toBe("available");
    expect(
      exportPacket.persistence_evidence.export_inventory.readiness_by_export_id
        .local_fea_handoff_package,
    ).toBe("available");
    expect(
      exportPacket.persistence_evidence.export_inventory.readiness_by_export_id
        .external_prover_boundary_metadata,
    ).toBe("available");
    expect(
      exportPacket.persistence_evidence.export_inventory.readiness_by_export_id
        .review_geometry_export,
    ).toBe("available");
    expect(
      exportPacket.persistence_evidence.export_inventory.readiness_by_export_id
        .conservative_pcf_export,
    ).toBe("available");
    expect(
      exportPacket.persistence_evidence.export_inventory.readiness_by_export_id
        .caepipe_mbf_export,
    ).toBe("available");
    expect(
      exportPacket.persistence_evidence.export_inventory.readiness_by_export_id
        .caepipe_external_run_evidence,
    ).toBe("available");
    expect(
      exportPacket.persistence_evidence.export_inventory.readiness_by_export_id
        .export_adapter_sdk_registry,
    ).toBe("available");
    expect(
      exportPacket.persistence_evidence.export_inventory.readiness_by_export_id
        .operation_review_ledger,
    ).toBe("empty_operation_queue");
    expect(exportPacket.persistence_evidence.boundary.network_required).toBe(
      false,
    );
    expect(exportPacket.persistence_evidence.boundary.telemetry_enabled).toBe(
      false,
    );
    expect(
      exportPacket.persistence_evidence.boundary.private_payload_included,
    ).toBe(false);
    expect(
      exportPacket.persistence_evidence.boundary.protected_content_included,
    ).toBe(false);
    expect(
      exportPacket.persistence_evidence.boundary.release_or_professional_claim,
    ).toBe(false);
    const reportLint = await screen.findByLabelText("Report content lint");
    expect(
      within(reportLint).getByTestId("report-lint-summary").textContent,
    ).toContain("targets=47");
    expect(
      within(reportLint).getByTestId("report-lint-summary").textContent,
    ).toContain("findings=0");
    expect(
      within(reportLint).getByTestId("report-lint-summary").textContent,
    ).toContain("blocking=0");
    expect(
      within(reportLint).getByTestId("report-lint-clean-scan").textContent,
    ).toContain("clearance=false");
    const reportLintHref =
      within(reportLint)
        .getByTestId("report-lint-export-link")
        .getAttribute("href") ?? "";
    const reportLintPacket = JSON.parse(
      decodeURIComponent(reportLintHref.split(",", 2)[1]),
    );
    expect(reportLintPacket.deliverable_id).toBe("DEL-08-05");
    expect(reportLintPacket.lint_run.run_id).toBe(
      "lint:report-preview:run-preview-linear-static-001",
    );
    expect(reportLintPacket.lint_run.summary.target_count).toBe(47);
    expect(reportLintPacket.lint_run.summary.scanned_target_count).toBe(47);
    expect(reportLintPacket.lint_run.summary.finding_count).toBe(0);
    expect(reportLintPacket.lint_run.summary.blocking_finding_count).toBe(0);
    expect(reportLintPacket.lint_run.summary.clean_scan_is_clearance).toBe(
      false,
    );
    expect(
      reportLintPacket.lint_run.targets.map(
        (item: { target_id: string }) => item.target_id,
      ),
    ).toContain("target:report-packet-preview-json");
    expect(
      reportLintPacket.lint_run.targets.every(
        (item: { privacy_classification: string }) =>
          item.privacy_classification !== "private_project_data",
      ),
    ).toBe(true);
    expect(exportPacket.run_audit.model_state_ref.ref).toBe(
      "state:project:invented-loop-01:preview",
    );
    expect(exportPacket.run_audit.analysis_run_ref.ref).toBe(
      "run:preview-linear-static-001",
    );
    expect(exportPacket.run_audit.run_kind).toBe("mechanics_solve");
    expect(exportPacket.run_audit.analysis_status).toContain(
      "HUMAN_REVIEW_REQUIRED",
    );
    expect(exportPacket.run_audit.analysis_status).toContain(
      "MECHANICS_SOLVED",
    );
    expect(exportPacket.run_audit.analysis_status).toContain(
      "RULE_INPUTS_INCOMPLETE",
    );
    expect(exportPacket.run_audit.result_row_count).toBe(830);
    expect(exportPacket.run_audit.result_ref_count).toBe(830);
    expect(exportPacket.run_audit.result_value_hash_count).toBe(830);
    expect(exportPacket.run_audit.hash_scopes).toContain("analysis_run_record");
    expect(exportPacket.run_audit.hash_scopes).toContain("result_envelope");
    expect(exportPacket.run_audit.input_manifest_refs[0].ref).toBe(
      "result-envelope:run:preview-linear-static-001",
    );
    expect(exportPacket.run_audit.unresolved_tbd).toContain(
      "physical project container",
    );
    expect(exportPacket.run_audit.unresolved_tbd).toContain(
      "release-grade solver build provenance",
    );
    expect(
      exportPacket.run_audit.immutability_policy.run_record_is_read_only,
    ).toBe(true);
    expect(
      exportPacket.run_audit.immutability_policy.new_run_required_for_change,
    ).toBe(true);
    expect(
      exportPacket.run_audit.professional_boundary
        .software_makes_compliance_claim,
    ).toBe(false);
    expect(
      exportPacket.run_audit.professional_boundary
        .software_makes_authentication_claim,
    ).toBe(false);
    expect(exportPacket.comparison_ref).toContain(
      "comparison:run:preview-linear-static-001",
    );
    expect(exportPacket.comparison_summary.comparable_result_pairs).toBe(261);
    expect(exportPacket.comparison_summary.unmatched_left_results).toBe(18);
    expect(exportPacket.comparison_summary.unmatched_right_results).toBe(0);
    expect(exportPacket.comparison_summary.tolerance_status).toBe(
      "not_tolerance_checked",
    );
    expect(exportPacket.comparison_summary.tolerance_profile_ref).toBe("TBD");
    expect(exportPacket.comparison_summary.release_or_professional_claim).toBe(
      false,
    );
    expect(exportPacket.comparison_top_deltas[0].classification).toBe(
      "not_tolerance_checked",
    );
    expect(
      exportPacket.comparison_professional_boundary
        .software_makes_compliance_claim,
    ).toBe(false);
    expect(exportPacket.data_boundary.private_data_policy).toBe(
      "no_private_project_data",
    );
    expect(exportPacket.private_payload_included).toBe(false);
    expect(exportPacket.protected_content_included).toBe(false);
    expect(exportPacket.release_or_professional_claim).toBe(false);
    expect(exportPacket.proposal_operation).toBeNull();

    const solvedExternalProver = await screen.findByLabelText(
      "External prover boundary metadata",
    );
    expect(
      within(solvedExternalProver).getByTestId("external-prover-summary")
        .textContent,
    ).toContain("metadata=non_authoritative_workflow_metadata");
    expect(
      within(solvedExternalProver).getByTestId("external-prover-context-links")
        .textContent,
    ).toContain("state=1");
    expect(
      within(solvedExternalProver).getByTestId("external-prover-run-boundary")
        .textContent,
    ).toContain("commercial_results=false");
    expect(
      within(solvedExternalProver).getByTestId("external-prover-unit-policy")
        .textContent,
    ).toContain("N*m/rad,N/m");
    expect(
      within(solvedExternalProver).getByTestId("external-prover-unit-policy")
        .textContent,
    ).toContain(
      "policy=record_units_for_external_reviewer_without_invoking_target_solver",
    );
    const solvedExternalHref =
      within(solvedExternalProver)
        .getByTestId("external-prover-export-link")
        .getAttribute("href") ?? "";
    const solvedExternalPacket = JSON.parse(
      decodeURIComponent(solvedExternalHref.split(",", 2)[1]),
    );
    expect(solvedExternalPacket.tags).toContain("mechanics-context-bound");
    expect(solvedExternalPacket.handoff_package_refs[0].ref.ref).toContain(
      "run:preview-linear-static-001",
    );
    expect(solvedExternalPacket.immutable_model_state_refs[0].ref.ref).toBe(
      "state:project:invented-loop-01:preview",
    );
    expect(
      solvedExternalPacket.external_references[0].hash_refs[0].algorithm,
    ).toBe("sha256");
    expect(solvedExternalPacket.unit_policy_evidence.result_units).toEqual([
      "MPa",
      "N",
      "N*m",
      "N*m/rad",
      "N/m",
      "boolean",
      "count",
      "m",
      "mm",
      "mode_code",
      "rad",
      "state_code",
    ]);
    expect(solvedExternalPacket.unit_policy_evidence.conversion_performed).toBe(
      false,
    );
    expect(solvedExternalPacket.unit_policy_evidence.analysis_run_ref.ref).toBe(
      "run:preview-linear-static-001",
    );
    expect(
      solvedExternalPacket.professional_boundary.external_tool_invoked,
    ).toBe(false);
    expect(
      solvedExternalPacket.professional_boundary
        .commercial_result_payload_ingested,
    ).toBe(false);
    expect(
      solvedExternalPacket.professional_boundary
        .software_creates_professional_reliance_record,
    ).toBe(false);

    const handoff = await screen.findByLabelText("Handoff package");
    const handoffExportSummary = await within(handoff).findByTestId(
      "handoff-export-summary",
    );
    expect(handoffExportSummary.textContent).toContain("26 entities");
    expect(handoffExportSummary.textContent).toContain("33 diagnostics");
    expect(
      within(handoff).getByTestId("handoff-target-profile").textContent,
    ).toContain("native_open_json_preview");
    expect(
      within(handoff).getByTestId("handoff-stable-ids").textContent,
    ).toContain("830 result refs");
    expect(
      within(handoff).getByTestId("handoff-loss-report").textContent,
    ).toContain(
      "target-specific commercial-solver field mapping is not generated",
    );
    expect(
      within(handoff).getByTestId("handoff-boundary").textContent,
    ).toContain("no private payload");
    const handoffHref =
      within(handoff).getByTestId("handoff-export-link").getAttribute("href") ??
      "";
    expect(handoffHref).toContain("data:application/json");
    const handoffPacket = JSON.parse(
      decodeURIComponent(handoffHref.split(",", 2)[1]),
    );
    expect(handoffPacket.document_kind).toBe(
      "openpipestress.technical_preview.handoff_package",
    );
    expect(handoffPacket.export_scope).toBe("local_browser_download_preview");
    expect(handoffPacket.deliverable_refs).toContain("DEL-15-01");
    expect(handoffPacket.deliverable_refs).toContain("DEL-17-03");
    expect(handoffPacket.model_state_ref.ref).toBe(
      "state:project:invented-loop-01:preview",
    );
    expect(handoffPacket.analysis_run_ref.ref).toBe(
      "run:preview-linear-static-001",
    );
    expect(handoffPacket.units_manifest.length).toBe("m");
    expect(within(handoff).getByTestId("handoff-units").textContent).toContain(
      "source=angle=rad",
    );
    expect(within(handoff).getByTestId("handoff-units").textContent).toContain(
      "length=m",
    );
    expect(within(handoff).getByTestId("handoff-units").textContent).toContain(
      "results=MPa",
    );
    expect(within(handoff).getByTestId("handoff-units").textContent).toContain(
      "conversion=false",
    );
    expect(
      within(handoff).getByTestId("handoff-unit-witnesses").textContent,
    ).toContain("count=830");
    expect(
      within(handoff).getByTestId("handoff-unit-witnesses").textContent,
    ).toContain("conversion=false");
    expect(handoffPacket.unit_system_disclosure.unit_system_ref.ref).toBe(
      "unit-system:dec-018-si-dual-display",
    );
    expect(handoffPacket.unit_system_disclosure.model_units.length).toBe("m");
    expect(handoffPacket.unit_system_disclosure.result_units).toContain("MPa");
    expect(handoffPacket.unit_system_disclosure.result_units).toContain("mm");
    expect(handoffPacket.unit_system_disclosure.conversion_performed).toBe(
      false,
    );
    expect(
      handoffPacket.unit_system_disclosure.protected_content_included,
    ).toBe(false);
    expect(handoffPacket.unit_witness_policy).toBe(
      "preserve_source_result_value_unit_and_dimension_per_handoff_result_ref",
    );
    expect(handoffPacket.unit_preservation_witnesses).toHaveLength(830);
    const handoffStressWitness = handoffPacket.unit_preservation_witnesses.find(
      (item: { source_ref: { ref: string } }) =>
        item.source_ref.ref ===
        "result:stress:pipe-P-120:end-j:torsional-shear",
    );
    expect(handoffStressWitness).toBeTruthy();
    expect(handoffStressWitness.source_quantity.unit).toBe("MPa");
    expect(handoffStressWitness.source_quantity.dimension).toBe("stress");
    expect(handoffStressWitness.target_quantity.unit).toBe("MPa");
    expect(handoffStressWitness.conversion_performed).toBe(false);
    expect(handoffPacket.stable_id_map.entity_ref_count).toBe(26);
    expect(handoffPacket.stable_id_map.entity_refs).toContain(
      "material:invented-carbon-steel",
    );
    expect(handoffPacket.stable_id_map.result_ref_count).toBe(830);
    expect(handoffPacket.stable_id_map.selected_result_refs).toContain(
      "result:force:pipe-P-120:axial",
    );
    expect(handoffPacket.library_refs.private_library_payload_included).toBe(
      false,
    );
    expect(handoffPacket.target_mapping.mapping_status).toBe(
      "stable_ids_only_not_target_specific",
    );
    expect(handoffPacket.target_mapping.target_field_coverage).toBe("TBD");
    expect(handoffPacket.run_audit_refs.hash_scopes).toContain(
      "result_envelope",
    );
    expect(handoffPacket.comparison_ref).toContain(
      "comparison:run:preview-linear-static-001",
    );
    expect(handoffPacket.comparison_summary.comparable_result_pairs).toBe(261);
    expect(handoffPacket.diagnostic_refs).toContain(
      "diagnostic:combination:combination-C-OPER-ALT:result-stress-pipe-P-130:COMBINATION_STRESS_SUMMARY_SKIPPED",
    );
    expect(handoffPacket.unresolved_assumptions).toContain(
      "target-specific field coverage TBD",
    );
    expect(handoffPacket.loss_report.unsupported_behavior_refs).toContain(
      "professional validation and acceptance are not software-generated",
    );
    expect(handoffPacket.private_payload_included).toBe(false);
    expect(handoffPacket.protected_content_included).toBe(false);
    expect(handoffPacket.release_or_professional_claim).toBe(false);
    expect(
      handoffPacket.professional_boundary.software_makes_compliance_claim,
    ).toBe(false);

    fireEvent.click(
      screen.getByRole("button", { name: /Generate review proposal/i }),
    );
    const proposal = await screen.findByLabelText("Agentic proposal");
    expect(
      await within(proposal).findByText("proposal:physics-diagnostic-review"),
    ).toBeInTheDocument();
    expect(
      within(proposal).getByTestId("selected-review-target").textContent,
    ).toContain("result: result:stress:pipe-P-120:end-j:torsional-shear");
    expect(
      within(proposal).getAllByText(
        /result:stress:pipe-P-120:end-j:torsional-shear/i,
      ).length,
    ).toBeGreaterThan(0);
    expect(
      within(proposal).getByText(
        /review-only and does not mutate accepted model state/i,
      ),
    ).toBeInTheDocument();
    expect(
      within(proposal).getByTestId("proposal-operation-summary").textContent,
    ).toContain("op:review-computed-diagnostic");
    expect(
      within(proposal).getByTestId("proposal-operation-summary").textContent,
    ).toContain("attach_design_knowledge");
    expect(
      within(proposal).getByTestId("proposal-operation-summary").textContent,
    ).toContain("draft_user_review_required");
    expect(
      within(proposal).getByTestId("proposal-affected-entities").textContent,
    ).toContain("result:stress:pipe-P-120:end-j:torsional-shear");
    expect(
      within(proposal).getByTestId("proposal-validation-status").textContent,
    ).toContain("warning_computed_context_requires_human_review");
    expect(
      within(proposal).getByTestId("proposal-validation-status").textContent,
    ).toContain("generated_from_computed_context");
    expect(
      within(proposal).getByTestId("proposal-validation-status").textContent,
    ).toContain("not_applied");
    expect(
      within(proposal).getByTestId("proposal-unit-policy").textContent,
    ).toContain("not_required_metadata_review_only");
    expect(
      within(proposal).getByTestId("proposal-unit-policy").textContent,
    ).toContain("Conversionfalse");
    expect(
      within(proposal).getByTestId("proposal-audit-boundary").textContent,
    ).toContain("requires user acceptance");
    expect(
      within(proposal).getByTestId("proposal-audit-boundary").textContent,
    ).toContain("true");
    expect(
      within(proposal).getByTestId("proposal-audit-boundary").textContent,
    ).toContain("mutates accepted model state");
    expect(
      within(proposal).getByTestId("proposal-audit-boundary").textContent,
    ).toContain("false");
    expect(
      within(proposal).getByTestId("proposal-professional-boundary")
        .textContent,
    ).toContain("human review required");
    expect(
      within(proposal).getByTestId("proposal-professional-boundary")
        .textContent,
    ).toContain("software makes approval claim");
    expect(
      within(proposal).getByTestId("proposal-professional-boundary")
        .textContent,
    ).toContain("false");
    expect(
      within(proposal).getByRole("button", { name: /Accept disabled/i }),
    ).toBeDisabled();

    expect(
      await within(report).findByText("proposal:physics-diagnostic-review"),
    ).toBeInTheDocument();
    expect(
      within(report).getByTestId("report-proposal-operation").textContent,
    ).toContain("op:review-computed-diagnostic");
    expect(
      within(report).getByTestId("report-proposal-operation").textContent,
    ).toContain("attach_design_knowledge");
    expect(
      within(report).getByTestId("report-proposal-operation").textContent,
    ).toContain("draft_user_review_required");
    expect(
      within(report).getByTestId("report-proposal-operation").textContent,
    ).toContain("not_applied");
    expect(
      within(report).getByTestId("report-proposal-boundary").textContent,
    ).toContain("review-only; requires user acceptance");
    expect(
      within(report).getByTestId("report-proposal-boundary").textContent,
    ).toContain("does not mutate accepted model state");
    expect(
      within(report).getByTestId("report-proposal-boundary").textContent,
    ).toContain("no compliance or professional approval claim");

    const proposalExportHref =
      within(report).getByTestId("report-export-link").getAttribute("href") ??
      "";
    const proposalExportPacket = JSON.parse(
      decodeURIComponent(proposalExportHref.split(",", 2)[1]),
    );
    expect(proposalExportPacket.proposal_ref).toBe(
      "proposal:physics-diagnostic-review",
    );
    expect(proposalExportPacket.selected_review_target).toEqual({
      target_type: "result",
      id: "result:stress:pipe-P-120:end-j:torsional-shear",
    });
    expect(proposalExportPacket.proposal_operation.operation_id).toBe(
      "op:review-computed-diagnostic",
    );
    expect(proposalExportPacket.proposal_operation.operation_kind).toBe(
      "attach_design_knowledge",
    );
    expect(proposalExportPacket.proposal_operation.operation_status).toBe(
      "draft_user_review_required",
    );
    expect(
      proposalExportPacket.proposal_operation.affected_entity_ids,
    ).toContain("result:stress:pipe-P-120:end-j:torsional-shear");
    expect(
      proposalExportPacket.proposal_operation.validation.application_status,
    ).toBe("not_applied");
    expect(
      proposalExportPacket.proposal_operation.validation.diff_preview_status,
    ).toBe("generated_from_computed_context");
    expect(
      proposalExportPacket.proposal_operation.audit_boundary
        .requires_user_acceptance,
    ).toBe(true);
    expect(
      proposalExportPacket.proposal_operation.audit_boundary
        .mutates_accepted_model_state,
    ).toBe(false);
    expect(
      proposalExportPacket.proposal_operation.audit_boundary
        .acceptance_recorded_as_review_only,
    ).toBe(true);
    expect(
      proposalExportPacket.proposal_operation.professional_boundary
        .human_review_required,
    ).toBe(true);
    expect(
      proposalExportPacket.proposal_operation.professional_boundary
        .software_makes_compliance_claim,
    ).toBe(false);
    expect(
      proposalExportPacket.proposal_operation.professional_boundary
        .software_makes_approval_claim,
    ).toBe(false);
    expect(
      proposalExportPacket.persistence_evidence.storage_audit
        .pending_operation_count,
    ).toBe(1);
    expect(
      proposalExportPacket.persistence_evidence.storage_audit
        .editor_intent_count,
    ).toBe(0);
    expect(
      proposalExportPacket.persistence_evidence.storage_audit
        .proposal_operation_count,
    ).toBe(1);
    expect(
      screen.getByTestId("local-project-review-context").textContent,
    ).toContain(
      "1 pending operation; applied_operations=0; editor_intents=0; agent_proposals=1",
    );

    const proposalStorageAudit = await screen.findByLabelText(
      "Project storage audit",
    );
    expect(
      within(proposalStorageAudit).getByTestId("project-storage-summary")
        .textContent,
    ).toContain("pending operations=1");
    expect(
      within(proposalStorageAudit).getByTestId("project-storage-summary")
        .textContent,
    ).toContain("proposals=1");
    const proposalStorageHref =
      within(proposalStorageAudit)
        .getByTestId("project-storage-export-link")
        .getAttribute("href") ?? "";
    const proposalStoragePacket = JSON.parse(
      decodeURIComponent(proposalStorageHref.split(",", 2)[1]),
    );
    expect(proposalStoragePacket.summary.pending_operation_count).toBe(1);
    expect(proposalStoragePacket.summary.editor_intent_count).toBe(0);
    expect(proposalStoragePacket.summary.proposal_operation_count).toBe(1);
    expect(proposalStoragePacket.proposal_refs).toContain(
      "proposal:physics-diagnostic-review",
    );
    expect(proposalStoragePacket.review_operation_statuses).toContain(
      "not_applied",
    );

    const proposalProjectValidation = await screen.findByLabelText(
      "Project validation preflight",
    );
    expect(
      within(proposalProjectValidation).getByTestId(
        "project-validation-operations",
      ).textContent,
    ).toContain("pending operations=1");
    expect(
      within(proposalProjectValidation).getByTestId(
        "project-validation-operations",
      ).textContent,
    ).toContain("proposals=1");
    const proposalValidationHref =
      within(proposalProjectValidation)
        .getByTestId("project-validation-export-link")
        .getAttribute("href") ?? "";
    const proposalValidationPacket = JSON.parse(
      decodeURIComponent(proposalValidationHref.split(",", 2)[1]),
    );
    expect(proposalValidationPacket.summary.pending_operation_count).toBe(1);
    expect(proposalValidationPacket.summary.editor_intent_count).toBe(0);
    expect(proposalValidationPacket.summary.proposal_operation_count).toBe(1);
    expect(proposalValidationPacket.proposal_refs).toContain(
      "proposal:physics-diagnostic-review",
    );
    expect(proposalValidationPacket.review_operation_statuses).toContain(
      "not_applied",
    );

    const operationLedger = await screen.findByLabelText(
      "Operation review ledger",
    );
    expect(
      await within(operationLedger).findByTestId(
        "operation-ledger-export-summary",
      ),
    ).toHaveTextContent("1 review record");
    expect(
      within(operationLedger).getByTestId("operation-ledger-decision-counts")
        .textContent,
    ).toContain("1 held_for_user_acceptance");
    expect(
      within(operationLedger).getByTestId("operation-ledger-latest")
        .textContent,
    ).toContain("op:review-computed-diagnostic");
    expect(
      within(operationLedger).getByTestId("operation-ledger-boundary")
        .textContent,
    ).toContain("requires explicit user acceptance");
    expect(
      within(operationLedger).getByTestId("operation-ledger-unit-policy")
        .textContent,
    ).toContain("records=1");
    expect(
      within(operationLedger).getByTestId("operation-ledger-unit-policy")
        .textContent,
    ).toContain("unit_bearing_changes=0");
    expect(
      within(operationLedger).getByTestId("operation-ledger-unit-policy")
        .textContent,
    ).toContain("unit_validations=not_required_metadata_review_only");
    expect(
      within(operationLedger).getByTestId(
        "operation-ledger-record-op-review-computed-diagnostic",
      ).textContent,
    ).toContain("result:stress:pipe-P-120:end-j:torsional-shear");
    const ledgerHref =
      within(operationLedger)
        .getByTestId("operation-ledger-export-link")
        .getAttribute("href") ?? "";
    const ledgerPacket = JSON.parse(
      decodeURIComponent(ledgerHref.split(",", 2)[1]),
    );
    expect(ledgerPacket.document_kind).toBe(
      "openpipestress.technical_preview.operation_review_ledger",
    );
    expect(ledgerPacket.export_scope).toBe("local_browser_download_preview");
    expect(ledgerPacket.deliverable_refs).toContain("DEL-16-04");
    expect(ledgerPacket.decision_counts.held_for_user_acceptance).toBe(1);
    expect(ledgerPacket.unit_policy_evidence.unit_bearing_change_count).toBe(0);
    expect(ledgerPacket.unit_policy_evidence.dimensionless_change_count).toBe(
      1,
    );
    expect(ledgerPacket.unit_policy_evidence.unit_validation_statuses).toEqual([
      "not_required_metadata_review_only",
    ]);
    expect(ledgerPacket.unit_policy_evidence.conversion_performed).toBe(false);
    expect(ledgerPacket.records[0].record_source).toBe("agent_proposal");
    expect(ledgerPacket.records[0].proposal_ref).toBe(
      "proposal:physics-diagnostic-review",
    );
    expect(ledgerPacket.records[0].decision.status).toBe(
      "held_for_user_acceptance",
    );
    expect(ledgerPacket.records[0].actor.actor_type).toBe("agent");
    expect(ledgerPacket.records[0].affected_entities[0].ref).toBe(
      "result:stress:pipe-P-120:end-j:torsional-shear",
    );
    expect(ledgerPacket.records[0].validation_outcome.diff_preview_status).toBe(
      "generated_from_computed_context",
    );
    expect(ledgerPacket.records[0].validation_outcome.application_status).toBe(
      "not_applied",
    );
    expect(
      ledgerPacket.records[0].professional_boundary
        .software_makes_compliance_claim,
    ).toBe(false);
    expect(ledgerPacket.selected_review_target).toEqual({
      target_type: "result",
      id: "result:stress:pipe-P-120:end-j:torsional-shear",
    });
    expect(ledgerPacket.accepted_model_state_unchanged).toBe(true);
    expect(ledgerPacket.release_or_professional_claim).toBe(false);

    const nativePackageAfterProposal = await screen.findByLabelText(
      "Native JSON package",
    );
    expect(
      within(nativePackageAfterProposal).getByTestId("native-package-summary")
        .textContent,
    ).toContain("operations=1");
    expect(
      within(nativePackageAfterProposal).getByTestId(
        "native-package-stable-ids",
      ).textContent,
    ).toContain("operations=1");
    const nativePackageAfterProposalHref =
      within(nativePackageAfterProposal)
        .getByTestId("native-package-link")
        .getAttribute("href") ?? "";
    const nativePackageAfterProposalPacket = JSON.parse(
      decodeURIComponent(nativePackageAfterProposalHref.split(",", 2)[1]),
    );
    expect(
      nativePackageAfterProposalPacket.stable_id_map.operation_ref_count,
    ).toBe(1);
    expect(
      nativePackageAfterProposalPacket.stable_id_map.operation_refs,
    ).toContain("op:review-computed-diagnostic");
    expect(
      nativePackageAfterProposalPacket.stable_id_map.proposal_refs,
    ).toContain("proposal:physics-diagnostic-review");
    expect(nativePackageAfterProposalPacket.operation_review.record_count).toBe(
      1,
    );
    expect(
      nativePackageAfterProposalPacket.operation_review.editor_intent_count,
    ).toBe(0);
    expect(
      nativePackageAfterProposalPacket.operation_review.proposal_count,
    ).toBe(1);
    expect(
      nativePackageAfterProposalPacket.operation_review
        .held_for_user_acceptance_count,
    ).toBe(1);
    expect(
      nativePackageAfterProposalPacket.operation_review.accepted_count,
    ).toBe(0);
    expect(
      nativePackageAfterProposalPacket.operation_review.rejected_count,
    ).toBe(0);
    expect(
      nativePackageAfterProposalPacket.operation_review.operation_refs,
    ).toContain("op:review-computed-diagnostic");
    expect(
      nativePackageAfterProposalPacket.operation_review.proposal_refs,
    ).toContain("proposal:physics-diagnostic-review");
    expect(
      nativePackageAfterProposalPacket.operation_review.selected_review_target,
    ).toEqual({
      target_type: "result",
      id: "result:stress:pipe-P-120:end-j:torsional-shear",
    });
    expect(
      nativePackageAfterProposalPacket.operation_review
        .accepted_model_state_mutated,
    ).toBe(false);
    expect(
      nativePackageAfterProposalPacket.operation_review
        .operation_application_status,
    ).toBe("not_applied");
    expect(
      nativePackageAfterProposalPacket.operation_review.audit_boundary
        .requires_user_acceptance,
    ).toBe(true);
    expect(
      nativePackageAfterProposalPacket.operation_review.audit_boundary
        .preview_records_do_not_apply_operations,
    ).toBe(true);
    expect(
      nativePackageAfterProposalPacket.operation_review.audit_boundary
        .direct_model_mutation_allowed,
    ).toBe(false);
    expect(nativePackageAfterProposalPacket.validation_report.checks).toContain(
      "review-only operation refs declared when present",
    );
    expect(nativePackageAfterProposalPacket.release_or_professional_claim).toBe(
      false,
    );
    expect(
      nativePackageAfterProposalPacket.professional_boundary
        .software_makes_approval_claim,
    ).toBe(false);

    const diffPreview = await screen.findByLabelText("Operation diff preview");
    expect(
      await within(diffPreview).findByTestId("diff-preview-summary"),
    ).toHaveTextContent("1 operations");
    expect(
      within(diffPreview).getByTestId("diff-preview-summary").textContent,
    ).toContain("1 diff rows");
    expect(
      within(diffPreview).getByTestId("diff-preview-summary").textContent,
    ).toContain("accepted_state_mutated=false");
    expect(
      within(diffPreview).getByTestId("diff-preview-validation").textContent,
    ).toContain("0 hash-bound rows");
    expect(
      within(diffPreview).getByTestId("diff-preview-boundary").textContent,
    ).toContain("no release or professional claim");
    const proposalDiffRecord = within(diffPreview).getByTestId(
      "diff-preview-record-op-review-computed-diagnostic",
    );
    expect(proposalDiffRecord.textContent).toContain("agent_proposal");
    expect(proposalDiffRecord.textContent).toContain(
      "generated_from_computed_context",
    );
    expect(proposalDiffRecord.textContent).toContain(
      "result:stress:pipe-P-120:end-j:torsional-shear",
    );
    const diffHref =
      within(diffPreview)
        .getByTestId("diff-preview-export-link")
        .getAttribute("href") ?? "";
    const diffPacket = JSON.parse(
      decodeURIComponent(diffHref.split(",", 2)[1]),
    );
    expect(diffPacket.document_kind).toBe(
      "openpipestress.technical_preview.operation_diff_preview",
    );
    expect(diffPacket.deliverable_refs).toContain("DEL-16-02");
    expect(diffPacket.scope_items).toContain("SOW-069");
    expect(diffPacket.summary.operation_count).toBe(1);
    expect(diffPacket.summary.diff_row_count).toBe(1);
    expect(diffPacket.summary.held_for_user_acceptance_count).toBe(1);
    expect(diffPacket.summary.accepted_model_state_mutated).toBe(false);
    expect(diffPacket.previews[0].record_source).toBe("agent_proposal");
    expect(diffPacket.previews[0].operation_id).toBe(
      "op:review-computed-diagnostic",
    );
    expect(diffPacket.previews[0].diff_preview_status).toBe(
      "generated_from_computed_context",
    );
    expect(diffPacket.previews[0].application_status).toBe("not_applied");
    expect(diffPacket.previews[0].accepted_model_state_mutated).toBe(false);
    expect(diffPacket.previews[0].changes[0].target_ref).toBe(
      "result:stress:pipe-P-120:end-j:torsional-shear",
    );
    expect(diffPacket.private_payload_included).toBe(false);
    expect(diffPacket.protected_content_included).toBe(false);
    expect(diffPacket.release_or_professional_claim).toBe(false);

    const proposalExportReview = await screen.findByLabelText(
      "Export safety review",
    );
    const proposalReviewHref =
      within(proposalExportReview)
        .getByTestId("export-review-link")
        .getAttribute("href") ?? "";
    const proposalReviewManifest = JSON.parse(
      decodeURIComponent(proposalReviewHref.split(",", 2)[1]),
    );
    expect(proposalReviewManifest.summary.export_count).toBe(29);
    expect(proposalReviewManifest.summary.available_count).toBe(29);
    expect(proposalReviewManifest.summary.operation_record_count).toBe(1);
    expect(
      proposalReviewManifest.unit_policy_summary.summary
        .unit_evidence_required_count,
    ).toBe(27);
    expect(
      proposalReviewManifest.unit_policy_summary.summary
        .unit_evidence_present_count,
    ).toBe(27);
    const proposalReviewExport = proposalReviewManifest.exports.find(
      (item: { export_id: string }) =>
        item.export_id === "agent_proposal_review",
    );
    expect(proposalReviewExport.readiness).toBe("available");
    expect(proposalReviewExport.proposal_ref).toBe(
      "proposal:physics-diagnostic-review",
    );
    expect(proposalReviewExport.unit_validation_status).toBe(
      "not_required_metadata_review_only",
    );
    expect(proposalReviewExport.accepted_model_state_mutated).toBe(false);
    expect(
      proposalReviewManifest.exports.find(
        (item: { export_id: string }) =>
          item.export_id === "project_storage_audit",
      ).pending_operation_count,
    ).toBe(1);
    expect(
      proposalReviewManifest.exports.find(
        (item: { export_id: string }) =>
          item.export_id === "project_storage_audit",
      ).proposal_operation_count,
    ).toBe(1);
    expect(
      proposalReviewManifest.exports.find(
        (item: { export_id: string }) =>
          item.export_id === "project_validation_preflight",
      ).pending_operation_count,
    ).toBe(1);
    expect(
      proposalReviewManifest.exports.find(
        (item: { export_id: string }) =>
          item.export_id === "project_validation_preflight",
      ).proposal_operation_count,
    ).toBe(1);

    fireEvent.click(
      within(operationLedger).getByTestId("clear-operation-review-queue"),
    );

    expect(
      screen.getByTestId("local-project-review-context").textContent,
    ).toContain(
      "0 pending operations; applied_operations=0; editor_intents=0; agent_proposals=0",
    );
    expect(
      within(proposal).queryByTestId("proposal-body"),
    ).not.toBeInTheDocument();
    expect(
      await within(operationLedger).findByTestId("operation-ledger-empty"),
    ).toHaveTextContent("No structured operations are queued");
    expect(
      await within(diffPreview).findByTestId("diff-preview-empty"),
    ).toHaveTextContent("No operation diffs");
    expect(
      within(report).queryByTestId("report-proposal-operation"),
    ).not.toBeInTheDocument();
    expect(
      within(report).queryByTestId("report-proposal-boundary"),
    ).not.toBeInTheDocument();

    const clearedReportHref =
      within(report).getByTestId("report-export-link").getAttribute("href") ??
      "";
    const clearedReportPacket = JSON.parse(
      decodeURIComponent(clearedReportHref.split(",", 2)[1]),
    );
    expect(clearedReportPacket.proposal_ref).toBe("not generated");
    expect(clearedReportPacket.proposal_operation).toBeNull();
    expect(clearedReportPacket.selected_review_target).toEqual({
      target_type: "result",
      id: "result:stress:pipe-P-120:end-j:torsional-shear",
    });

    const nativePackageAfterClearHref =
      within(nativePackageAfterProposal)
        .getByTestId("native-package-link")
        .getAttribute("href") ?? "";
    const nativePackageAfterClearPacket = JSON.parse(
      decodeURIComponent(nativePackageAfterClearHref.split(",", 2)[1]),
    );
    expect(
      nativePackageAfterClearPacket.stable_id_map.operation_ref_count,
    ).toBe(0);
    expect(nativePackageAfterClearPacket.stable_id_map.operation_refs).toEqual(
      [],
    );
    expect(nativePackageAfterClearPacket.stable_id_map.proposal_refs).toEqual(
      [],
    );
    expect(nativePackageAfterClearPacket.operation_review.record_count).toBe(0);
    expect(nativePackageAfterClearPacket.operation_review.proposal_count).toBe(
      0,
    );
    expect(
      nativePackageAfterClearPacket.operation_review
        .held_for_user_acceptance_count,
    ).toBe(0);

    const clearedStorageHref =
      within(proposalStorageAudit)
        .getByTestId("project-storage-export-link")
        .getAttribute("href") ?? "";
    const clearedStoragePacket = JSON.parse(
      decodeURIComponent(clearedStorageHref.split(",", 2)[1]),
    );
    expect(clearedStoragePacket.summary.pending_operation_count).toBe(0);
    expect(clearedStoragePacket.summary.proposal_operation_count).toBe(0);
    expect(clearedStoragePacket.proposal_refs).toEqual([]);

    const clearedValidationHref =
      within(proposalProjectValidation)
        .getByTestId("project-validation-export-link")
        .getAttribute("href") ?? "";
    const clearedValidationPacket = JSON.parse(
      decodeURIComponent(clearedValidationHref.split(",", 2)[1]),
    );
    expect(clearedValidationPacket.summary.pending_operation_count).toBe(0);
    expect(clearedValidationPacket.summary.proposal_operation_count).toBe(0);
    expect(clearedValidationPacket.proposal_refs).toEqual([]);

    const exportReviewAfterClear = await screen.findByLabelText(
      "Export safety review",
    );
    const clearedReviewHref =
      within(exportReviewAfterClear)
        .getByTestId("export-review-link")
        .getAttribute("href") ?? "";
    const clearedReviewManifest = JSON.parse(
      decodeURIComponent(clearedReviewHref.split(",", 2)[1]),
    );
    expect(clearedReviewManifest.summary.operation_record_count).toBe(0);
    expect(
      clearedReviewManifest.exports.find(
        (item: { export_id: string }) =>
          item.export_id === "operation_review_ledger",
      ).readiness,
    ).toBe("empty_operation_queue");
    // Heavy full-<App/> Three.js render: inherit the 30s global testTimeout
    // (vite.config.ts); a tight per-test override flaked under DEC-025 sweep load.
  });

  it("links selected diagnostics to affected result and model context", async () => {
    render(<App />);

    fireEvent.click(
      await screen.findByRole("button", { name: /Run mechanics preview/i }),
    );
    fireEvent.click(screen.getByTestId("issues-drawer-toggle"));
    expect(
      (
        await screen.findAllByTestId(
          "diagnostic-COMBINATION_STRESS_SUMMARY_SKIPPED",
        )
      ).length,
    ).toBeGreaterThan(0);

    const diagnostics = await screen.findByLabelText("Diagnostics");
    expect(
      within(diagnostics).getByTestId("diagnostic-filter-summary").textContent,
    ).toContain("33 of 33 diagnostics visible");
    expect(
      within(diagnostics).getByTestId("diagnostic-severity-warning")
        .textContent,
    ).toContain("12 Warnings");
    expect(
      within(diagnostics).getByTestId("diagnostic-severity-info").textContent,
    ).toContain("21 Info");
    expect(
      within(diagnostics).getByTestId("diagnostic-severity-error").textContent,
    ).toContain("0 Errors");
    expect(
      within(diagnostics).getByTestId("diagnostic-severity-blocking")
        .textContent,
    ).toContain("0 Blocking");

    fireEvent.change(
      within(diagnostics).getByTestId("diagnostic-filter-input"),
      {
        target: { value: "result:stress:pipe-P-130" },
      },
    );
    expect(
      within(diagnostics).getByTestId("diagnostic-filter-summary").textContent,
    ).toContain("1 of 33 diagnostics visible");
    expect(
      within(diagnostics).getByTestId("diagnostic-severity-warning")
        .textContent,
    ).toContain("1 Warnings");
    expect(
      within(diagnostics).getByTestId("diagnostic-severity-info").textContent,
    ).toContain("0 Info");
    expect(
      within(diagnostics).getByTestId(
        "diagnostic-COMBINATION_STRESS_SUMMARY_SKIPPED",
      ),
    ).toBeInTheDocument();
    expect(
      within(diagnostics).queryByTestId("diagnostic-RULE_CHECK_NOT_PERFORMED"),
    ).not.toBeInTheDocument();

    fireEvent.click(
      within(diagnostics).getByTestId(
        "diagnostic-COMBINATION_STRESS_SUMMARY_SKIPPED",
      ),
    );

    const diagnosticDetail = within(diagnostics).getByTestId(
      "diagnostic-detail-panel",
    );
    expect(
      within(diagnosticDetail).getByTestId("selected-diagnostic-id")
        .textContent,
    ).toContain(
      "diagnostic:combination:combination-C-OPER-ALT:result-stress-pipe-P-130:COMBINATION_STRESS_SUMMARY_SKIPPED",
    );
    expect(
      within(diagnosticDetail).getByTestId("selected-diagnostic-affected-refs")
        .textContent,
    ).toContain("result:stress:pipe-P-130");
    expect(
      within(diagnosticDetail).getByTestId("selected-diagnostic-linked-results")
        .textContent,
    ).toContain("result:stress:pipe-P-130");
    expect(
      within(diagnosticDetail).getByTestId("diagnostic-unit-context")
        .textContent,
    ).toContain("linked_results=1");
    expect(
      within(diagnosticDetail).getByTestId("diagnostic-unit-context")
        .textContent,
    ).toContain("units=MPa");
    expect(
      within(diagnosticDetail).getByTestId("diagnostic-unit-context")
        .textContent,
    ).toContain("source=result_envelope");
    expect(
      within(diagnosticDetail).getByTestId("diagnostic-unit-context")
        .textContent,
    ).toContain("conversion=false");
    expect(
      within(diagnosticDetail).getByTestId("selected-diagnostic-explanation")
        .textContent,
    ).toContain("stress summary rows are not linearly combined");
    expect(
      await screen.findByRole("heading", { name: "Tie-in rise" }),
    ).toBeInTheDocument();

    fireEvent.click(within(diagnostics).getByTestId("clear-diagnostic-filter"));
    expect(
      within(diagnostics).getByTestId("diagnostic-filter-summary").textContent,
    ).toContain("33 of 33 diagnostics visible");
    expect(
      within(diagnostics).getByTestId("diagnostic-RULE_CHECK_NOT_PERFORMED"),
    ).toBeInTheDocument();

    fireEvent.click(
      screen.getByRole("button", { name: /Generate review proposal/i }),
    );
    const proposal = await screen.findByLabelText("Agentic proposal");
    expect(
      await within(proposal).findByText("proposal:physics-diagnostic-review"),
    ).toBeInTheDocument();
    expect(
      within(proposal).getByTestId("selected-review-target").textContent,
    ).toContain(
      "diagnostic: diagnostic:combination:combination-C-OPER-ALT:result-stress-pipe-P-130:COMBINATION_STRESS_SUMMARY_SKIPPED",
    );
    expect(
      within(proposal).getAllByText(
        /diagnostic:combination:combination-C-OPER-ALT:result-stress-pipe-P-130:COMBINATION_STRESS_SUMMARY_SKIPPED/i,
      ).length,
    ).toBeGreaterThan(0);
    expect(
      within(proposal).getByTestId("proposal-affected-entities").textContent,
    ).toContain(
      "diagnostic:combination:combination-C-OPER-ALT:result-stress-pipe-P-130:COMBINATION_STRESS_SUMMARY_SKIPPED",
    );
    expect(
      within(proposal).getByTestId("proposal-audit-boundary").textContent,
    ).toContain("acceptance recorded as review only");
    expect(
      within(proposal).getByTestId("proposal-professional-boundary")
        .textContent,
    ).toContain("software makes compliance claim");
    expect(
      within(proposal).getByTestId("proposal-professional-boundary")
        .textContent,
    ).toContain("false");
    expect(
      within(proposal).getByRole("button", { name: /Accept disabled/i }),
    ).toBeDisabled();

    const report = await screen.findByLabelText("Report packet");
    expect(
      within(report).getByTestId("report-selected-review-target").textContent,
    ).toContain(
      "diagnostic: diagnostic:combination:combination-C-OPER-ALT:result-stress-pipe-P-130:COMBINATION_STRESS_SUMMARY_SKIPPED",
    );
    expect(
      within(report).getByTestId("report-proposal-operation").textContent,
    ).toContain("op:review-computed-diagnostic");
    const proposalExportHref =
      within(report).getByTestId("report-export-link").getAttribute("href") ??
      "";
    const proposalExportPacket = JSON.parse(
      decodeURIComponent(proposalExportHref.split(",", 2)[1]),
    );
    expect(proposalExportPacket.selected_review_target).toEqual({
      target_type: "diagnostic",
      id: "diagnostic:combination:combination-C-OPER-ALT:result-stress-pipe-P-130:COMBINATION_STRESS_SUMMARY_SKIPPED",
    });
    expect(
      proposalExportPacket.proposal_operation.affected_entity_ids,
    ).toContain(
      "diagnostic:combination:combination-C-OPER-ALT:result-stress-pipe-P-130:COMBINATION_STRESS_SUMMARY_SKIPPED",
    );
    expect(
      proposalExportPacket.proposal_operation.audit_boundary
        .mutates_accepted_model_state,
    ).toBe(false);
  });

  it("applies a queued inspector edit through the structured operation seam, clears stale results, and re-solves and saves the edited model", async () => {
    render(<App />);

    const tree = await screen.findByLabelText("Model tree");
    fireEvent.click(
      within(tree).getByRole("button", {
        name: /Invented carbon-steel-like material/i,
      }),
    );
    const inspector = screen.getByLabelText("Property inspector");
    const intentPanel = within(inspector).getByLabelText(
      "Editor operation intent",
    );

    // Queue two edits of the same field; applying the first must make the
    // second stale rather than silently double-applying.
    fireEvent.change(within(intentPanel).getByTestId("editor-intent-field"), {
      target: { value: "elastic_modulus.value" },
    });
    fireEvent.change(within(intentPanel).getByTestId("editor-intent-value"), {
      target: { value: "195000000000" },
    });
    fireEvent.click(within(intentPanel).getByTestId("queue-editor-intent"));
    fireEvent.change(within(intentPanel).getByTestId("editor-intent-value"), {
      target: { value: "210000000000" },
    });
    fireEvent.click(within(intentPanel).getByTestId("queue-editor-intent"));

    const applyPanel = screen.getByTestId("operation-apply-panel");
    expect(
      within(applyPanel).getByTestId("operation-apply-summary").textContent,
    ).toContain("2 queued; 0 applied");

    fireEvent.click(
      within(applyPanel).getByTestId("apply-intent-editor-intent-1"),
    );
    await waitFor(() =>
      expect(
        within(applyPanel).getByTestId("operation-apply-message").textContent,
      ).toContain(
        "Applied op:editor-intent-material:invented-carbon-steel-elastic_modulus.value",
      ),
    );

    // Applied receipt with honest route, acceptance, and persistence labels.
    const receipt = within(applyPanel).getByTestId(
      "applied-operation-route-applied-1-editor-intent-1",
    );
    expect(receipt.textContent).toContain("Applied through local_wasm_engine");
    expect(receipt.textContent).toContain(
      "Acceptance basis user_initiated_apply_in_local_session",
    );
    expect(receipt.textContent).toContain(
      "persistence session_state_only_not_yet_saved",
    );
    expect(receipt.textContent).toContain("professional approval not recorded");

    // The session model document changed and earlier solve context is gone.
    expect(inspector.textContent).toContain("195000000000 Pa");
    expect(
      screen.getByTestId("local-project-review-context").textContent,
    ).toContain("1 pending operation");
    expect(screen.getByTestId("solve-job-summary").textContent).toContain(
      "state=not_started",
    );
    const clearedSolveHref =
      screen.getByTestId("solve-job-export-link").getAttribute("href") ?? "";
    const clearedSolvePacket = JSON.parse(
      decodeURIComponent(clearedSolveHref.split(",", 2)[1]),
    );
    expect(clearedSolvePacket.events[0].message).toContain(
      "previous mechanics results were cleared",
    );

    // The remaining queued intent is now stale; applying it is blocked with a
    // visible finding and the model value is unchanged.
    fireEvent.click(
      within(applyPanel).getByTestId("apply-intent-editor-intent-2"),
    );
    await waitFor(() =>
      expect(
        within(applyPanel).getByTestId("operation-apply-message").textContent,
      ).toContain("was not applied"),
    );
    expect(
      within(applyPanel).getByTestId(
        "operation-outcome-diagnostic-OP-STALE-BEFORE-VALUE",
      ).textContent,
    ).toContain("blocking: OP-STALE-BEFORE-VALUE");
    expect(inspector.textContent).toContain("195000000000 Pa");
    expect(
      screen.getByTestId("local-project-review-context").textContent,
    ).toContain("1 pending operation");

    // Browser fixture mode must not publish stale solved rows for the edited model.
    fireEvent.click(screen.getByTestId("run-mechanics-preview"));
    await waitFor(() =>
      expect(screen.getByTestId("solve-job-summary").textContent).toContain(
        "state=completed",
      ),
    );
    expect(screen.getByTestId("status-pill-mechanics").textContent).toContain(
      "MODEL_INCOMPLETE",
    );
    expect(
      within(screen.getByTestId("solve-readiness-summary")).getByTestId(
        "readiness-mechanics",
      ).textContent,
    ).toContain("0 computed result rows; model incomplete");
    expect(
      within(screen.getByTestId("solve-readiness-summary")).getByTestId(
        "readiness-diagnostics",
      ).textContent,
    ).toContain("1 blocking/error");
    fireEvent.click(screen.getByTestId("issues-drawer-toggle"));
    expect(
      screen.getByTestId(
        "diagnostic-BROWSER_SOLVE_BACKEND_REQUIRED_FOR_EDITED_MODEL",
      ),
    ).toBeInTheDocument();
    const editedSolveHref =
      screen.getByTestId("solve-job-export-link").getAttribute("href") ?? "";
    const editedSolvePacket = JSON.parse(
      decodeURIComponent(editedSolveHref.split(",", 2)[1]),
    );
    expect(editedSolvePacket.summary.result_row_count).toBe(0);
    expect(
      editedSolvePacket.diagnostics.map((item: { code: string }) => item.code),
    ).toContain("BROWSER_SOLVE_BACKEND_REQUIRED_FOR_EDITED_MODEL");
    expect(
      screen.getByTestId("viewport-deformation-status").textContent,
    ).toContain("blocked; mechanics=model incomplete; rows=0");

    fireEvent.click(screen.getByRole("button", { name: /Save local/i }));
    await waitFor(() =>
      expect(screen.getByTestId("local-project-message").textContent).toContain(
        "Saved local browser-preview project snapshot",
      ),
    );

    // DEC-019/DEC-033 evidence: the saved (edited) 0.1.0-era model document
    // carries migrated in-memory schema-version evidence with the in-document
    // version authority; the browser preview has no migration ledger.
    const documentMigrationLine = screen.getByTestId(
      "project-validation-model-document-migration",
    );
    expect(documentMigrationLine.textContent).toContain("status=migrated");
    expect(documentMigrationLine.textContent).toContain(
      "framework=application_service_separate_db_and_product_schema",
    );
    expect(documentMigrationLine.textContent).toContain("ledger_records=0");
  });

  it("queues and applies explicit material creation through the structured operation seam", async () => {
    render(<App />);

    const inspector = await screen.findByLabelText("Property inspector");
    const createMaterialPanel = within(inspector).getByLabelText(
      "Create material intent",
    );
    await waitFor(() =>
      expect(
        screen.getByTestId("property-unit-catalog-status").textContent,
      ).toContain("browser preview uses model metadata"),
    );
    expect(
      screen.getByTestId("property-unit-basis-summary").textContent,
    ).toContain("Pa, model metadata");
    expect(
      screen.getByTestId("property-unit-basis-summary").textContent,
    ).toContain("1/degC, model metadata");
    expect(
      within(createMaterialPanel).getByTestId("create-material-stress-unit"),
    ).toHaveValue("Pa");
    expect(
      within(createMaterialPanel).getByTestId("create-material-thermal-unit"),
    ).toHaveValue("1/degC");
    expect(
      within(createMaterialPanel).getByText(
        "Elastic modulus (Pa, model metadata)",
      ),
    ).toBeInTheDocument();
    expect(
      within(createMaterialPanel).getByText(
        "Thermal expansion (1/degC, model metadata)",
      ),
    ).toBeInTheDocument();
    expect(
      within(createMaterialPanel).getByTestId("queue-create-material-intent"),
    ).toBeDisabled();

    fireEvent.change(
      within(createMaterialPanel).getByTestId("create-material-id"),
      {
        target: { value: "material:M-300" },
      },
    );
    fireEvent.change(
      within(createMaterialPanel).getByTestId("create-material-label"),
      {
        target: { value: "User alloy material" },
      },
    );
    fireEvent.change(
      within(createMaterialPanel).getByTestId("create-material-elastic"),
      {
        target: { value: "125000000000" },
      },
    );
    fireEvent.change(
      within(createMaterialPanel).getByTestId("create-material-shear"),
      {
        target: { value: "48000000000" },
      },
    );
    fireEvent.change(
      within(createMaterialPanel).getByTestId("create-material-thermal"),
      {
        target: { value: "0.00001" },
      },
    );
    expect(
      within(createMaterialPanel).getByTestId("queue-create-material-intent"),
    ).not.toBeDisabled();
    expect(
      within(createMaterialPanel).getByTestId("editor-operation-preview")
        .textContent,
    ).toContain("create_material");
    expect(
      within(createMaterialPanel).getByTestId("editor-intent-validation")
        .textContent,
    ).toContain(
      "stress=model_metadata_unit_dimension_declared_catalog_unavailable_browser_preview",
    );
    expect(
      within(createMaterialPanel).getByTestId("editor-intent-validation")
        .textContent,
    ).toContain(
      "thermal_expansion_coefficient=model_metadata_unit_dimension_declared_catalog_unavailable_browser_preview",
    );

    fireEvent.click(
      within(createMaterialPanel).getByTestId("queue-create-material-intent"),
    );
    const applyPanel = screen.getByTestId("operation-apply-panel");
    expect(
      within(applyPanel).getByTestId("operation-apply-summary").textContent,
    ).toContain("1 queued; 0 applied");

    fireEvent.click(
      within(applyPanel).getByTestId("apply-intent-editor-intent-1"),
    );
    await waitFor(() =>
      expect(
        within(applyPanel).getByTestId("operation-apply-message").textContent,
      ).toContain("Applied op:create-material-material:M-300"),
    );

    const createdMaterialRow = screen.getByTestId("tree-row-material:M-300");
    expect(createdMaterialRow.textContent).toContain("User alloy material");
    expect(createdMaterialRow).toHaveClass("active");
    const materialInspector = screen.getByLabelText("Property inspector");
    expect(materialInspector.textContent).toContain("125000000000 Pa");
    expect(materialInspector.textContent).toContain("48000000000 Pa");
    expect(materialInspector.textContent).toContain("0.00001 1/degC");
    expect(
      screen.getByTestId("local-project-review-context").textContent,
    ).toContain("0 pending operations");
    expect(
      screen.getByTestId("local-project-review-context").textContent,
    ).toContain("applied_operations=1");
    expect(screen.getByTestId("solve-job-summary").textContent).toContain(
      "state=not_started",
    );

    const receipt = within(applyPanel).getByTestId(
      "applied-operation-route-applied-1-editor-intent-1",
    );
    expect(receipt.textContent).toContain(
      "Acceptance basis user_initiated_apply_in_local_session",
    );
    expect(receipt.textContent).toContain(
      "persistence session_state_only_not_yet_saved",
    );
    expect(receipt.textContent).toContain("professional approval not recorded");
  });

  it("queues and applies explicit section creation through the structured operation seam", async () => {
    render(<App />);

    const inspector = await screen.findByLabelText("Property inspector");
    const createSectionPanel = within(inspector).getByLabelText(
      "Create section intent",
    );
    await waitFor(() =>
      expect(
        screen.getByTestId("property-unit-catalog-status").textContent,
      ).toContain("browser preview uses model metadata"),
    );
    expect(
      screen.getByTestId("property-unit-basis-summary").textContent,
    ).toContain("m, model metadata");
    expect(
      within(createSectionPanel).getByTestId("create-section-length-unit"),
    ).toHaveValue("m");
    expect(
      within(createSectionPanel).getByText(
        "Outside diameter (m, model metadata)",
      ),
    ).toBeInTheDocument();
    expect(
      within(createSectionPanel).getByText(
        "Wall thickness (m, model metadata)",
      ),
    ).toBeInTheDocument();
    expect(
      within(createSectionPanel).getByTestId("queue-create-section-intent"),
    ).toBeDisabled();

    fireEvent.change(
      within(createSectionPanel).getByTestId("create-section-id"),
      {
        target: { value: "section:S-300" },
      },
    );
    fireEvent.change(
      within(createSectionPanel).getByTestId("create-section-name"),
      {
        target: { value: "User pipe section" },
      },
    );
    fireEvent.change(
      within(createSectionPanel).getByTestId("create-section-od"),
      {
        target: { value: "0.114" },
      },
    );
    fireEvent.change(
      within(createSectionPanel).getByTestId("create-section-wall"),
      {
        target: { value: "0.006" },
      },
    );
    expect(
      within(createSectionPanel).getByTestId("queue-create-section-intent"),
    ).not.toBeDisabled();
    expect(
      within(createSectionPanel).getByTestId("editor-operation-preview")
        .textContent,
    ).toContain("create_section");
    expect(
      within(createSectionPanel).getByTestId("editor-intent-validation")
        .textContent,
    ).toContain(
      "length=model_metadata_unit_dimension_declared_catalog_unavailable_browser_preview",
    );

    fireEvent.click(
      within(createSectionPanel).getByTestId("queue-create-section-intent"),
    );
    const applyPanel = screen.getByTestId("operation-apply-panel");
    expect(
      within(applyPanel).getByTestId("operation-apply-summary").textContent,
    ).toContain("1 queued; 0 applied");

    fireEvent.click(
      within(applyPanel).getByTestId("apply-intent-editor-intent-1"),
    );
    await waitFor(() =>
      expect(
        within(applyPanel).getByTestId("operation-apply-message").textContent,
      ).toContain("Applied op:create-section-section:S-300"),
    );

    const createdSectionRow = screen.getByTestId("tree-row-section:S-300");
    expect(createdSectionRow.textContent).toContain("User pipe section");
    expect(createdSectionRow).toHaveClass("active");
    const sectionInspector = screen.getByLabelText("Property inspector");
    expect(sectionInspector.textContent).toContain("0.114 m");
    expect(sectionInspector.textContent).toContain("0.006 m");
    expect(
      screen.getByTestId("local-project-review-context").textContent,
    ).toContain("0 pending operations");
    expect(
      screen.getByTestId("local-project-review-context").textContent,
    ).toContain("applied_operations=1");
    expect(screen.getByTestId("solve-job-summary").textContent).toContain(
      "state=not_started",
    );

    const receipt = within(applyPanel).getByTestId(
      "applied-operation-route-applied-1-editor-intent-1",
    );
    expect(receipt.textContent).toContain(
      "Acceptance basis user_initiated_apply_in_local_session",
    );
    expect(receipt.textContent).toContain(
      "persistence session_state_only_not_yet_saved",
    );
    expect(receipt.textContent).toContain("professional approval not recorded");
  });

  it("queues and applies explicit support creation through the structured operation seam", async () => {
    render(<App />);

    const tree = await screen.findByLabelText("Model tree");
    fireEvent.click(within(tree).getByTestId("tree-row-node:N-100"));
    const inspector = screen.getByLabelText("Property inspector");
    const createSupportPanel = within(inspector).getByLabelText(
      "Create support intent",
    );

    fireEvent.change(
      within(createSupportPanel).getByTestId("create-support-id"),
      {
        target: { value: "support:S-150" },
      },
    );
    fireEvent.change(
      within(createSupportPanel).getByTestId("create-support-label"),
      {
        target: { value: "User guide support" },
      },
    );
    fireEvent.change(
      within(createSupportPanel).getByTestId("create-support-node"),
      {
        target: { value: "node:N-100" },
      },
    );
    fireEvent.click(
      within(createSupportPanel).getByTestId("create-support-restraint-RX"),
    );
    expect(
      within(createSupportPanel).getByTestId("create-support-stiffness-unit"),
    ).toHaveValue("N/m");
    await waitFor(() =>
      expect(
        within(createSupportPanel).getByText(
          "Linear stiffness (N/m, model metadata)",
        ),
      ).toBeInTheDocument(),
    );
    fireEvent.change(
      within(createSupportPanel).getByTestId("create-support-stiffness"),
      {
        target: { value: "12500" },
      },
    );
    expect(
      within(createSupportPanel).getByTestId("queue-create-support-intent"),
    ).not.toBeDisabled();
    expect(
      within(createSupportPanel).getByTestId("editor-operation-preview")
        .textContent,
    ).toContain("create_support");
    expect(
      within(createSupportPanel).getByTestId("editor-operation-preview")
        .textContent,
    ).toContain('"linear_stiffness":{"value":12500,"unit":"N/m"}');
    expect(
      within(createSupportPanel).getByTestId("editor-operation-preview")
        .textContent,
    ).toContain("linear_stiffness; unit=N/m");
    expect(
      within(createSupportPanel).getByTestId("editor-intent-validation")
        .textContent,
    ).toContain(
      "linear_stiffness=model_metadata_unit_dimension_declared_catalog_unavailable_browser_preview",
    );

    fireEvent.click(
      within(createSupportPanel).getByTestId("queue-create-support-intent"),
    );
    const applyPanel = screen.getByTestId("operation-apply-panel");
    expect(
      within(applyPanel).getByTestId("operation-apply-summary").textContent,
    ).toContain("1 queued; 0 applied");

    fireEvent.click(
      within(applyPanel).getByTestId("apply-intent-editor-intent-1"),
    );
    await waitFor(() =>
      expect(
        within(applyPanel).getByTestId("operation-apply-message").textContent,
      ).toContain("Applied op:create-support-support:S-150"),
    );

    const createdSupportRow = screen.getByTestId("tree-row-support:S-150");
    expect(createdSupportRow.textContent).toContain("User guide support");
    expect(createdSupportRow).toHaveClass("active");
    expect(screen.getByLabelText("Property inspector").textContent).toContain(
      "node:N-100",
    );
    expect(screen.getByLabelText("Property inspector").textContent).toContain(
      "UX, UY, UZ, RX",
    );
    expect(screen.getByLabelText("Property inspector").textContent).toContain(
      "12500 N/m",
    );
    expect(
      screen.getByTestId("local-project-review-context").textContent,
    ).toContain("0 pending operations");
    expect(
      screen.getByTestId("local-project-review-context").textContent,
    ).toContain("applied_operations=1");
    expect(screen.getByTestId("solve-job-summary").textContent).toContain(
      "state=not_started",
    );

    const receipt = within(applyPanel).getByTestId(
      "applied-operation-route-applied-1-editor-intent-1",
    );
    expect(receipt.textContent).toContain(
      "Acceptance basis user_initiated_apply_in_local_session",
    );
    expect(receipt.textContent).toContain(
      "persistence session_state_only_not_yet_saved",
    );
    expect(receipt.textContent).toContain("professional approval not recorded");
  });

  it("queues and applies explicit support deletion through the structured operation seam", async () => {
    render(<App />);

    const tree = await screen.findByLabelText("Model tree");
    const supportRow = within(tree).getByTestId("tree-row-support:S-120");
    expect(supportRow.textContent).toContain("Guide on riser");
    fireEvent.click(supportRow);

    const inspector = screen.getByLabelText("Property inspector");
    const deleteSupportPanel = within(inspector).getByLabelText(
      "Delete support intent",
    );
    expect(
      within(deleteSupportPanel).getByTestId("editor-operation-preview")
        .textContent,
    ).toContain("delete_support");
    expect(
      within(deleteSupportPanel).getByTestId("editor-operation-preview")
        .textContent,
    ).toContain("before=Guide on riser");
    expect(
      within(deleteSupportPanel).getByTestId("editor-operation-preview")
        .textContent,
    ).toContain("after=not_present");
    expect(
      within(deleteSupportPanel).getByTestId("editor-intent-validation")
        .textContent,
    ).toContain("not_required_dimensionless");

    fireEvent.click(
      within(deleteSupportPanel).getByTestId("queue-delete-support-intent"),
    );
    const applyPanel = screen.getByTestId("operation-apply-panel");
    expect(
      within(applyPanel).getByTestId("operation-apply-row-editor-intent-1")
        .textContent,
    ).toContain("not_present");

    fireEvent.click(
      within(applyPanel).getByTestId("apply-intent-editor-intent-1"),
    );
    await waitFor(() =>
      expect(
        within(applyPanel).getByTestId("operation-apply-message").textContent,
      ).toContain("Applied op:delete-support-support:S-120"),
    );

    expect(screen.queryByTestId("tree-row-support:S-120")).toBeNull();
    expect(screen.getByTestId("tree-row-project:invented-loop-01")).toHaveClass(
      "active",
    );
    expect(screen.getByLabelText("Property inspector").textContent).toContain(
      "project:invented-loop-01",
    );
    expect(
      screen.getByTestId("local-project-review-context").textContent,
    ).toContain("0 pending operations");
    expect(
      screen.getByTestId("local-project-review-context").textContent,
    ).toContain("applied_operations=1");
    expect(screen.getByTestId("solve-job-summary").textContent).toContain(
      "state=not_started",
    );

    const receipt = within(applyPanel).getByTestId(
      "applied-operation-route-applied-1-editor-intent-1",
    );
    expect(receipt.textContent).toContain(
      "Acceptance basis user_initiated_apply_in_local_session",
    );
    expect(receipt.textContent).toContain(
      "persistence session_state_only_not_yet_saved",
    );
    expect(receipt.textContent).toContain("professional approval not recorded");
  });

  it("queues and applies explicit node deletion through the structured operation seam", async () => {
    render(<App />);

    expect(
      await screen.findByLabelText("Three.js pipe centerline viewport"),
    ).toBeInTheDocument();
    const viewportIntentPanel = screen.getByLabelText(
      "Viewport editor intents",
    );
    fireEvent.change(
      within(viewportIntentPanel).getByTestId("viewport-create-node-id"),
      {
        target: { value: "node:N-150" },
      },
    );
    fireEvent.change(
      within(viewportIntentPanel).getByTestId("viewport-create-node-label"),
      {
        target: { value: "Delete target node" },
      },
    );
    fireEvent.change(
      within(viewportIntentPanel).getByTestId("viewport-create-node-x"),
      {
        target: { value: "9" },
      },
    );
    fireEvent.change(
      within(viewportIntentPanel).getByTestId("viewport-create-node-y"),
      {
        target: { value: "1" },
      },
    );
    fireEvent.change(
      within(viewportIntentPanel).getByTestId("viewport-create-node-z"),
      {
        target: { value: "0" },
      },
    );
    fireEvent.click(
      within(viewportIntentPanel).getByTestId("queue-explicit-node-intent"),
    );

    const applyPanel = screen.getByTestId("operation-apply-panel");
    fireEvent.click(
      within(applyPanel).getByTestId("apply-intent-editor-intent-1"),
    );
    await waitFor(() =>
      expect(
        within(applyPanel).getByTestId("operation-apply-message").textContent,
      ).toContain("Applied op:viewport-create-node-node:N-150-001"),
    );

    const tree = screen.getByLabelText("Model tree");
    const nodeRow = within(tree).getByTestId("tree-row-node:N-150");
    expect(nodeRow.textContent).toContain("Delete target node");
    expect(nodeRow).toHaveClass("active");

    const inspector = screen.getByLabelText("Property inspector");
    const deleteNodePanel =
      within(inspector).getByLabelText("Delete node intent");
    expect(
      within(deleteNodePanel).getByTestId("editor-operation-preview")
        .textContent,
    ).toContain("delete_node");
    expect(
      within(deleteNodePanel).getByTestId("editor-operation-preview")
        .textContent,
    ).toContain("before=Delete target node; x=9; y=1; z=0");
    expect(
      within(deleteNodePanel).getByTestId("editor-operation-preview")
        .textContent,
    ).toContain("after=not_present");
    expect(
      within(deleteNodePanel).getByTestId("editor-intent-validation")
        .textContent,
    ).toContain("not_required_dimensionless");

    fireEvent.click(
      within(deleteNodePanel).getByTestId("queue-delete-node-intent"),
    );
    expect(
      within(applyPanel).getByTestId("operation-apply-row-editor-intent-2")
        .textContent,
    ).toContain("not_present");

    fireEvent.click(
      within(applyPanel).getByTestId("apply-intent-editor-intent-2"),
    );
    await waitFor(() =>
      expect(
        within(applyPanel).getByTestId("operation-apply-message").textContent,
      ).toContain("Applied op:delete-node-node:N-150"),
    );

    expect(screen.queryByTestId("tree-row-node:N-150")).toBeNull();
    expect(screen.getByTestId("tree-row-project:invented-loop-01")).toHaveClass(
      "active",
    );
    expect(screen.getByLabelText("Property inspector").textContent).toContain(
      "project:invented-loop-01",
    );
    expect(
      screen.getByTestId("local-project-review-context").textContent,
    ).toContain("0 pending operations");
    expect(
      screen.getByTestId("local-project-review-context").textContent,
    ).toContain("applied_operations=2");
    expect(screen.getByTestId("solve-job-summary").textContent).toContain(
      "state=not_started",
    );

    const receipt = within(applyPanel).getByTestId(
      "applied-operation-route-applied-2-editor-intent-2",
    );
    expect(receipt.textContent).toContain(
      "Acceptance basis user_initiated_apply_in_local_session",
    );
    expect(receipt.textContent).toContain(
      "persistence session_state_only_not_yet_saved",
    );
    expect(receipt.textContent).toContain("professional approval not recorded");
  });

  it("blocks node deletion while model entities still reference it", async () => {
    render(<App />);

    const tree = await screen.findByLabelText("Model tree");
    const nodeRow = within(tree).getByTestId("tree-row-node:N-120");
    expect(nodeRow.textContent).toContain("Riser elbow");
    fireEvent.click(nodeRow);

    const deleteNodePanel = within(
      screen.getByLabelText("Property inspector"),
    ).getByLabelText("Delete node intent");
    expect(
      within(deleteNodePanel).getByTestId("editor-operation-preview")
        .textContent,
    ).toContain("delete_node");
    fireEvent.click(
      within(deleteNodePanel).getByTestId("queue-delete-node-intent"),
    );

    const applyPanel = screen.getByTestId("operation-apply-panel");
    expect(
      within(applyPanel).getByTestId("operation-apply-summary").textContent,
    ).toContain("1 queued; 0 applied");
    fireEvent.click(
      within(applyPanel).getByTestId("apply-intent-editor-intent-1"),
    );
    await waitFor(() =>
      expect(
        within(applyPanel).getByTestId("operation-apply-message").textContent,
      ).toContain(
        "Operation op:delete-node-node:N-120 was not applied (blocked); see its diagnostics.",
      ),
    );

    expect(
      within(applyPanel).getByTestId(
        "operation-outcome-diagnostic-OP-NODE-DELETE-REFERENCED",
      ).textContent,
    ).toContain("pipe:P-120");
    expect(screen.getByTestId("tree-row-node:N-120")).toHaveClass("active");
    expect(screen.getByLabelText("Property inspector").textContent).toContain(
      "Riser elbow",
    );
    expect(
      screen.getByTestId("local-project-review-context").textContent,
    ).toContain("1 pending operation");
    expect(
      screen.getByTestId("local-project-review-context").textContent,
    ).toContain("applied_operations=0");
  });

  it("queues and applies explicit pipe deletion through the structured operation seam", async () => {
    render(<App />);

    const tree = await screen.findByLabelText("Model tree");
    const pipeRow = within(tree).getByTestId("tree-row-pipe:P-100");
    expect(pipeRow.textContent).toContain("Pump discharge run");
    fireEvent.click(pipeRow);

    const inspector = screen.getByLabelText("Property inspector");
    const deletePipePanel =
      within(inspector).getByLabelText("Delete pipe intent");
    expect(
      within(deletePipePanel).getByTestId("editor-operation-preview")
        .textContent,
    ).toContain("delete_pipe_run");
    expect(
      within(deletePipePanel).getByTestId("editor-operation-preview")
        .textContent,
    ).toContain("before=Pump discharge run");
    expect(
      within(deletePipePanel).getByTestId("editor-operation-preview")
        .textContent,
    ).toContain("after=not_present");
    expect(
      within(deletePipePanel).getByTestId("editor-intent-validation")
        .textContent,
    ).toContain("not_required_dimensionless");

    fireEvent.click(
      within(deletePipePanel).getByTestId("queue-delete-pipe-intent"),
    );
    const applyPanel = screen.getByTestId("operation-apply-panel");
    expect(
      within(applyPanel).getByTestId("operation-apply-row-editor-intent-1")
        .textContent,
    ).toContain("not_present");

    fireEvent.click(
      within(applyPanel).getByTestId("apply-intent-editor-intent-1"),
    );
    await waitFor(() =>
      expect(
        within(applyPanel).getByTestId("operation-apply-message").textContent,
      ).toContain("Applied op:delete-pipe-pipe:P-100"),
    );

    expect(screen.queryByTestId("tree-row-pipe:P-100")).toBeNull();
    expect(screen.getByTestId("tree-row-project:invented-loop-01")).toHaveClass(
      "active",
    );
    expect(screen.getByLabelText("Property inspector").textContent).toContain(
      "project:invented-loop-01",
    );
    expect(
      screen.getByTestId("local-project-review-context").textContent,
    ).toContain("0 pending operations");
    expect(
      screen.getByTestId("local-project-review-context").textContent,
    ).toContain("applied_operations=1");
    expect(screen.getByTestId("solve-job-summary").textContent).toContain(
      "state=not_started",
    );

    const receipt = within(applyPanel).getByTestId(
      "applied-operation-route-applied-1-editor-intent-1",
    );
    expect(receipt.textContent).toContain(
      "Acceptance basis user_initiated_apply_in_local_session",
    );
    expect(receipt.textContent).toContain(
      "persistence session_state_only_not_yet_saved",
    );
    expect(receipt.textContent).toContain("professional approval not recorded");
  });

  it("blocks pipe deletion while primitive loads still reference it", async () => {
    render(<App />);

    const tree = await screen.findByLabelText("Model tree");
    const pipeRow = within(tree).getByTestId("tree-row-pipe:P-120");
    expect(pipeRow.textContent).toContain("Rack span");
    fireEvent.click(pipeRow);

    const deletePipePanel = within(
      screen.getByLabelText("Property inspector"),
    ).getByLabelText("Delete pipe intent");
    expect(
      within(deletePipePanel).getByTestId("editor-operation-preview")
        .textContent,
    ).toContain("delete_pipe_run");
    fireEvent.click(
      within(deletePipePanel).getByTestId("queue-delete-pipe-intent"),
    );

    const applyPanel = screen.getByTestId("operation-apply-panel");
    expect(
      within(applyPanel).getByTestId("operation-apply-summary").textContent,
    ).toContain("1 queued; 0 applied");
    fireEvent.click(
      within(applyPanel).getByTestId("apply-intent-editor-intent-1"),
    );
    await waitFor(() =>
      expect(
        within(applyPanel).getByTestId("operation-apply-message").textContent,
      ).toContain(
        "Operation op:delete-pipe-pipe:P-120 was not applied (blocked); see its diagnostics.",
      ),
    );

    expect(
      within(applyPanel).getByTestId(
        "operation-outcome-diagnostic-OP-PIPE-DELETE-REFERENCED",
      ).textContent,
    ).toContain("load:L-100-Z");
    expect(screen.getByTestId("tree-row-pipe:P-120")).toHaveClass("active");
    expect(screen.getByLabelText("Property inspector").textContent).toContain(
      "Rack span",
    );
    expect(
      screen.getByTestId("local-project-review-context").textContent,
    ).toContain("1 pending operation");
    expect(
      screen.getByTestId("local-project-review-context").textContent,
    ).toContain("applied_operations=0");
  });

  it("blocks support deletion while an imposed-displacement load still references it", async () => {
    render(<App />);

    const manager = await screen.findByTestId("load-case-manager");
    fireEvent.change(
      within(manager).getByTestId("load-manager-create-primitive-category"),
      {
        target: { value: "imposed_displacement" },
      },
    );
    fireEvent.change(
      within(manager).getByTestId("load-manager-create-primitive-magnitude"),
      {
        target: { value: "-0.006" },
      },
    );
    fireEvent.click(
      within(manager).getByTestId("queue-create-primitive-intent"),
    );

    const applyPanel = screen.getByTestId("operation-apply-panel");
    fireEvent.click(
      within(applyPanel).getByTestId("apply-intent-editor-intent-1"),
    );
    await waitFor(() =>
      expect(
        within(applyPanel).getByTestId("operation-apply-message").textContent,
      ).toContain(
        "Applied op:load-manager-load:L-100-load:L-100-I300-primitive",
      ),
    );

    const tree = screen.getByLabelText("Model tree");
    fireEvent.click(within(tree).getByTestId("tree-row-support:S-100"));
    const deleteSupportPanel = within(
      screen.getByLabelText("Property inspector"),
    ).getByLabelText("Delete support intent");
    expect(
      within(deleteSupportPanel).getByTestId("editor-operation-preview")
        .textContent,
    ).toContain("delete_support");
    fireEvent.click(
      within(deleteSupportPanel).getByTestId("queue-delete-support-intent"),
    );

    expect(
      within(applyPanel).getByTestId("operation-apply-summary").textContent,
    ).toContain("1 queued; 1 applied");
    fireEvent.click(
      within(applyPanel).getByTestId("apply-intent-editor-intent-2"),
    );
    await waitFor(() =>
      expect(
        within(applyPanel).getByTestId("operation-apply-message").textContent,
      ).toContain(
        "Operation op:delete-support-support:S-100 was not applied (blocked); see its diagnostics.",
      ),
    );

    expect(
      within(applyPanel).getByTestId(
        "operation-outcome-diagnostic-OP-SUPPORT-DELETE-REFERENCED",
      ).textContent,
    ).toContain("load:L-100-I300");
    expect(screen.getByTestId("tree-row-support:S-100")).toHaveClass("active");
    expect(screen.getByLabelText("Property inspector").textContent).toContain(
      "Anchor at pump nozzle",
    );
    expect(
      screen.getByTestId("local-project-review-context").textContent,
    ).toContain("1 pending operation");
    expect(
      screen.getByTestId("local-project-review-context").textContent,
    ).toContain("applied_operations=1");
  });

  it("queues and applies explicit viewport node geometry through the structured operation seam", async () => {
    render(<App />);

    expect(
      await screen.findByLabelText("Three.js pipe centerline viewport"),
    ).toBeInTheDocument();
    const viewportIntentPanel = screen.getByLabelText(
      "Viewport editor intents",
    );
    const queueButton = within(viewportIntentPanel).getByTestId(
      "queue-explicit-node-intent",
    );
    expect(queueButton).toBeDisabled();
    await waitFor(() =>
      expect(
        within(viewportIntentPanel).getByTestId("viewport-unit-catalog-status")
          .textContent,
      ).toContain("browser preview uses model metadata"),
    );
    expect(
      within(viewportIntentPanel).getByTestId("viewport-create-node-unit"),
    ).toHaveValue("m");
    await waitFor(() =>
      expect(
        within(viewportIntentPanel).getByTestId(
          "viewport-create-node-unit-basis",
        ).textContent,
      ).toContain("Coordinates: m, model metadata"),
    );

    fireEvent.change(
      within(viewportIntentPanel).getByTestId("viewport-create-node-id"),
      {
        target: { value: "node:N-150" },
      },
    );
    fireEvent.change(
      within(viewportIntentPanel).getByTestId("viewport-create-node-label"),
      {
        target: { value: "User preview node" },
      },
    );
    fireEvent.change(
      within(viewportIntentPanel).getByTestId("viewport-create-node-x"),
      {
        target: { value: "8.4" },
      },
    );
    fireEvent.change(
      within(viewportIntentPanel).getByTestId("viewport-create-node-y"),
      {
        target: { value: "2.4" },
      },
    );
    fireEvent.change(
      within(viewportIntentPanel).getByTestId("viewport-create-node-z"),
      {
        target: { value: "2.8" },
      },
    );
    expect(queueButton).not.toBeDisabled();
    fireEvent.click(queueButton);
    expect(
      within(viewportIntentPanel).getByTestId(
        "viewport-intent-unit-validation-create_node",
      ).textContent,
    ).toContain(
      "unit_validation=length=model_metadata_unit_dimension_declared_catalog_unavailable_browser_preview",
    );

    const applyPanel = screen.getByTestId("operation-apply-panel");
    expect(
      within(applyPanel).getByTestId("operation-apply-summary").textContent,
    ).toContain("1 queued; 0 applied");
    fireEvent.click(
      within(applyPanel).getByTestId("apply-intent-editor-intent-1"),
    );
    await waitFor(() =>
      expect(
        within(applyPanel).getByTestId("operation-apply-message").textContent,
      ).toContain("Applied op:viewport-create-node-node:N-150-001"),
    );

    const createdNodeRow = screen.getByTestId("tree-row-node:N-150");
    expect(createdNodeRow.textContent).toContain("User preview node");
    expect(createdNodeRow).toHaveClass("active");
    const inspector = screen.getByLabelText("Property inspector");
    expect(inspector.textContent).toContain("User preview node");
    expect(inspector.textContent).toContain("8.4, 2.4, 2.8 m");
    expect(
      screen.getByTestId("local-project-review-context").textContent,
    ).toContain("0 pending operations");
    expect(
      screen.getByTestId("local-project-review-context").textContent,
    ).toContain("applied_operations=1");

    const receipt = within(applyPanel).getByTestId(
      "applied-operation-route-applied-1-editor-intent-1",
    );
    expect(receipt.textContent).toContain(
      "Acceptance basis user_initiated_apply_in_local_session",
    );
    expect(receipt.textContent).toContain(
      "persistence session_state_only_not_yet_saved",
    );
    expect(receipt.textContent).toContain("professional approval not recorded");

    expect(
      within(applyPanel).getByTestId("session-history-chip").textContent,
    ).toContain("1 undo / 0 redo");
    expect(
      within(applyPanel).getByTestId("undo-session-model-edit"),
    ).not.toBeDisabled();
    expect(
      within(applyPanel).getByTestId("redo-session-model-edit"),
    ).toBeDisabled();

    fireEvent.click(within(applyPanel).getByTestId("undo-session-model-edit"));
    await waitFor(() =>
      expect(
        within(applyPanel).getByTestId("operation-apply-message").textContent,
      ).toContain("Undid op:viewport-create-node-node:N-150-001"),
    );
    expect(screen.queryByTestId("tree-row-node:N-150")).toBeNull();
    expect(
      within(applyPanel).getByTestId("session-history-chip").textContent,
    ).toContain("0 undo / 1 redo");
    expect(screen.getByTestId("solve-job-summary").textContent).toContain(
      "state=not_started",
    );

    fireEvent.click(within(applyPanel).getByTestId("redo-session-model-edit"));
    await waitFor(() =>
      expect(
        within(applyPanel).getByTestId("operation-apply-message").textContent,
      ).toContain("Redid op:viewport-create-node-node:N-150-001"),
    );
    const redoneNodeRow = screen.getByTestId("tree-row-node:N-150");
    expect(redoneNodeRow.textContent).toContain("User preview node");
    expect(redoneNodeRow).toHaveClass("active");
    expect(screen.getByLabelText("Property inspector").textContent).toContain(
      "8.4, 2.4, 2.8 m",
    );
    expect(
      within(applyPanel).getByTestId("session-history-chip").textContent,
    ).toContain("1 undo / 0 redo");
  });

  it("captures viewport pointer geometry into an explicit node draft before apply", async () => {
    render(<App />);

    const viewportCanvas = await screen.findByTestId("viewport-canvas");
    const viewportIntentPanel = screen.getByLabelText(
      "Viewport editor intents",
    );
    const queueButton = within(viewportIntentPanel).getByTestId(
      "queue-explicit-node-intent",
    );
    expect(queueButton).toBeDisabled();
    fireEvent.click(
      within(screen.getByTestId("command-bar")).getByRole("button", {
        name: /Node/i,
      }),
    );
    expect(screen.getByTestId("armed-creation-tool")).toHaveTextContent(
      "Node tool armed",
    );

    fireEvent.pointerDown(viewportCanvas, {
      button: 0,
      clientX: 300,
      clientY: 160,
    });

    const idInput = within(viewportIntentPanel).getByTestId(
      "viewport-create-node-id",
    ) as HTMLInputElement;
    const labelInput = within(viewportIntentPanel).getByTestId(
      "viewport-create-node-label",
    ) as HTMLInputElement;
    const xInput = within(viewportIntentPanel).getByTestId(
      "viewport-create-node-x",
    ) as HTMLInputElement;
    const yInput = within(viewportIntentPanel).getByTestId(
      "viewport-create-node-y",
    ) as HTMLInputElement;
    const zInput = within(viewportIntentPanel).getByTestId(
      "viewport-create-node-z",
    ) as HTMLInputElement;
    expect(idInput.value).toBe("node:V-001");
    expect(labelInput.value).toBe("Viewport node V-001");
    expect(
      [xInput.value, yInput.value, zInput.value].every((value) =>
        Number.isFinite(Number(value)),
      ),
    ).toBe(true);
    expect(yInput.value).toBe("0");
    const draftedPosition = `${xInput.value}, ${yInput.value}, ${zInput.value} m`;
    expect(queueButton).not.toBeDisabled();

    fireEvent.click(queueButton);
    const applyPanel = screen.getByTestId("operation-apply-panel");
    fireEvent.click(
      within(applyPanel).getByTestId("apply-intent-editor-intent-1"),
    );
    await waitFor(() =>
      expect(
        within(applyPanel).getByTestId("operation-apply-message").textContent,
      ).toContain("Applied op:viewport-create-node-node:V-001-001"),
    );

    const createdNodeRow = screen.getByTestId("tree-row-node:V-001");
    expect(createdNodeRow.textContent).toContain("Viewport node V-001");
    expect(createdNodeRow).toHaveClass("active");
    expect(screen.getByLabelText("Property inspector").textContent).toContain(
      draftedPosition,
    );
  });

  it("queues and applies explicit straight pipe connectivity through the structured operation seam", async () => {
    render(<App />);

    expect(
      await screen.findByLabelText("Three.js pipe centerline viewport"),
    ).toBeInTheDocument();
    const viewportIntentPanel = screen.getByLabelText(
      "Viewport editor intents",
    );
    const queueButton = within(viewportIntentPanel).getByTestId(
      "queue-explicit-pipe-intent",
    );
    expect(queueButton).toBeDisabled();
    await waitFor(() =>
      expect(
        within(viewportIntentPanel).getByTestId("viewport-unit-catalog-status")
          .textContent,
      ).toContain("browser preview uses model metadata"),
    );
    expect(
      within(viewportIntentPanel).getByTestId(
        "viewport-create-pipe-length-unit",
      ),
    ).toHaveValue("m");
    await waitFor(() =>
      expect(
        within(viewportIntentPanel).getByTestId(
          "viewport-create-pipe-unit-basis",
        ).textContent,
      ).toContain("Pipe geometry: m, model metadata"),
    );

    fireEvent.change(
      within(viewportIntentPanel).getByTestId("viewport-create-pipe-id"),
      {
        target: { value: "pipe:P-150" },
      },
    );
    fireEvent.change(
      within(viewportIntentPanel).getByTestId("viewport-create-pipe-label"),
      {
        target: { value: "User preview pipe" },
      },
    );
    fireEvent.change(
      within(viewportIntentPanel).getByTestId("viewport-create-pipe-from"),
      {
        target: { value: "node:N-100" },
      },
    );
    fireEvent.change(
      within(viewportIntentPanel).getByTestId("viewport-create-pipe-to"),
      {
        target: { value: "node:N-140" },
      },
    );
    fireEvent.change(
      within(viewportIntentPanel).getByTestId("viewport-create-pipe-material"),
      {
        target: { value: "material:invented-carbon-steel" },
      },
    );
    fireEvent.change(
      within(viewportIntentPanel).getByTestId("viewport-create-pipe-od"),
      {
        target: { value: "0.114" },
      },
    );
    fireEvent.change(
      within(viewportIntentPanel).getByTestId("viewport-create-pipe-wall"),
      {
        target: { value: "0.006" },
      },
    );
    fireEvent.change(
      within(viewportIntentPanel).getByTestId("viewport-create-pipe-yref-x"),
      {
        target: { value: "0" },
      },
    );
    fireEvent.change(
      within(viewportIntentPanel).getByTestId("viewport-create-pipe-yref-y"),
      {
        target: { value: "0" },
      },
    );
    fireEvent.change(
      within(viewportIntentPanel).getByTestId("viewport-create-pipe-yref-z"),
      {
        target: { value: "1" },
      },
    );
    expect(queueButton).not.toBeDisabled();
    fireEvent.click(queueButton);
    expect(
      within(viewportIntentPanel).getByTestId(
        "viewport-intent-unit-validation-connect_pipe_run",
      ).textContent,
    ).toContain(
      "unit_validation=length=model_metadata_unit_dimension_declared_catalog_unavailable_browser_preview",
    );

    const applyPanel = screen.getByTestId("operation-apply-panel");
    expect(
      within(applyPanel).getByTestId("operation-apply-summary").textContent,
    ).toContain("1 queued; 0 applied");
    fireEvent.click(
      within(applyPanel).getByTestId("apply-intent-editor-intent-1"),
    );
    await waitFor(() =>
      expect(
        within(applyPanel).getByTestId("operation-apply-message").textContent,
      ).toContain("Applied op:viewport-connect-pipe-pipe:P-150-001"),
    );

    const createdPipeRow = screen.getByTestId("tree-row-pipe:P-150");
    expect(createdPipeRow.textContent).toContain("User preview pipe");
    expect(createdPipeRow).toHaveClass("active");
    expect(screen.getByTestId("viewport-select-pipe:P-150")).toHaveAttribute(
      "aria-pressed",
      "true",
    );
    const inspector = screen.getByLabelText("Property inspector");
    expect(inspector.textContent).toContain("User preview pipe");
    expect(inspector.textContent).toContain("0.114 m");
    expect(inspector.textContent).toContain("0.006 m");
    expect(inspector.textContent).toContain("material:invented-carbon-steel");
    expect(inspector.textContent).toContain("user_entered_local_preview");
    expect(
      screen.getByTestId("local-project-review-context").textContent,
    ).toContain("0 pending operations");
    expect(
      screen.getByTestId("local-project-review-context").textContent,
    ).toContain("applied_operations=1");
    expect(screen.getByTestId("solve-job-summary").textContent).toContain(
      "state=not_started",
    );

    const receipt = within(applyPanel).getByTestId(
      "applied-operation-route-applied-1-editor-intent-1",
    );
    expect(receipt.textContent).toContain(
      "Acceptance basis user_initiated_apply_in_local_session",
    );
    expect(receipt.textContent).toContain(
      "persistence session_state_only_not_yet_saved",
    );
    expect(receipt.textContent).toContain("professional approval not recorded");
  });

  it("picks straight pipe endpoints from viewport node targets before apply", async () => {
    render(<App />);

    expect(
      await screen.findByLabelText("Three.js pipe centerline viewport"),
    ).toBeInTheDocument();
    const viewportIntentPanel = screen.getByLabelText(
      "Viewport editor intents",
    );
    const viewportSelection = screen.getByTestId("viewport-selection-layer");
    const queueButton = within(viewportIntentPanel).getByTestId(
      "queue-explicit-pipe-intent",
    );
    const fromSelect = within(viewportIntentPanel).getByTestId(
      "viewport-create-pipe-from",
    ) as HTMLSelectElement;
    const toSelect = within(viewportIntentPanel).getByTestId(
      "viewport-create-pipe-to",
    ) as HTMLSelectElement;
    const pickFrom = within(viewportIntentPanel).getByTestId(
      "viewport-pick-pipe-from",
    );
    const pickTo = within(viewportIntentPanel).getByTestId(
      "viewport-pick-pipe-to",
    );
    expect(queueButton).toBeDisabled();

    fireEvent.click(pickFrom);
    expect(pickFrom).toHaveAttribute("aria-pressed", "true");
    fireEvent.click(
      within(viewportSelection).getByTestId("viewport-select-node:N-100"),
    );
    await waitFor(() => expect(fromSelect.value).toBe("node:N-100"));
    expect(toSelect.value).toBe("");
    expect(pickFrom).toHaveAttribute("aria-pressed", "false");
    expect(pickTo).toHaveAttribute("aria-pressed", "true");

    fireEvent.click(
      within(viewportSelection).getByTestId("viewport-select-node:N-140"),
    );
    await waitFor(() => expect(toSelect.value).toBe("node:N-140"));
    expect(fromSelect.value).toBe("node:N-100");
    expect(pickTo).toHaveAttribute("aria-pressed", "false");

    fireEvent.change(
      within(viewportIntentPanel).getByTestId("viewport-create-pipe-id"),
      {
        target: { value: "pipe:P-151" },
      },
    );
    fireEvent.change(
      within(viewportIntentPanel).getByTestId("viewport-create-pipe-label"),
      {
        target: { value: "Viewport picked pipe" },
      },
    );
    fireEvent.change(
      within(viewportIntentPanel).getByTestId("viewport-create-pipe-material"),
      {
        target: { value: "material:invented-carbon-steel" },
      },
    );
    fireEvent.change(
      within(viewportIntentPanel).getByTestId("viewport-create-pipe-od"),
      {
        target: { value: "0.114" },
      },
    );
    fireEvent.change(
      within(viewportIntentPanel).getByTestId("viewport-create-pipe-wall"),
      {
        target: { value: "0.006" },
      },
    );
    fireEvent.change(
      within(viewportIntentPanel).getByTestId("viewport-create-pipe-yref-x"),
      {
        target: { value: "0" },
      },
    );
    fireEvent.change(
      within(viewportIntentPanel).getByTestId("viewport-create-pipe-yref-y"),
      {
        target: { value: "0" },
      },
    );
    fireEvent.change(
      within(viewportIntentPanel).getByTestId("viewport-create-pipe-yref-z"),
      {
        target: { value: "1" },
      },
    );
    expect(queueButton).not.toBeDisabled();
    fireEvent.click(queueButton);

    const applyPanel = screen.getByTestId("operation-apply-panel");
    fireEvent.click(
      within(applyPanel).getByTestId("apply-intent-editor-intent-1"),
    );
    await waitFor(() =>
      expect(
        within(applyPanel).getByTestId("operation-apply-message").textContent,
      ).toContain("Applied op:viewport-connect-pipe-pipe:P-151-001"),
    );
    const createdPipeRow = screen.getByTestId("tree-row-pipe:P-151");
    expect(createdPipeRow.textContent).toContain("Viewport picked pipe");
    expect(createdPipeRow).toHaveClass("active");
    expect(screen.getByTestId("viewport-select-pipe:P-151")).toHaveAttribute(
      "aria-pressed",
      "true",
    );
    const inspector = screen.getByLabelText("Property inspector");
    expect(inspector.textContent).toContain("Viewport picked pipe");
    expect(inspector.textContent).toContain("material:invented-carbon-steel");
  });

  it("blocks underspecified viewport node gestures at apply instead of inventing values", async () => {
    render(<App />);

    expect(
      await screen.findByLabelText("Three.js pipe centerline viewport"),
    ).toBeInTheDocument();
    const viewportIntentPanel = screen.getByLabelText(
      "Viewport editor intents",
    );
    const commandBar = screen.getByTestId("command-bar");
    fireEvent.click(within(commandBar).getByRole("button", { name: /Node/i }));
    expect(
      within(commandBar).getByTestId("armed-creation-tool"),
    ).toHaveTextContent("Node tool armed");
    fireEvent.click(
      within(commandBar).getByTestId("queue-armed-creation-intent"),
    );

    const applyPanel = screen.getByTestId("operation-apply-panel");
    fireEvent.click(
      within(applyPanel).getByTestId("apply-intent-editor-intent-1"),
    );
    await waitFor(() =>
      expect(
        within(applyPanel).getByTestId("operation-apply-message").textContent,
      ).toContain("was not applied"),
    );
    expect(
      within(applyPanel).getByTestId(
        "operation-outcome-diagnostic-OP-CREATE-NODE-SHAPE-INVALID",
      ).textContent,
    ).toContain("Create-node intents must target object_type");
    expect(
      screen.getByTestId("local-project-review-context").textContent,
    ).toContain("1 pending operation");
  });
});
