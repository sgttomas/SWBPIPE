import { fireEvent, render, screen, waitFor, within } from "@testing-library/react";
import { describe, expect, it } from "vitest";
import { App } from "./App";
import { PropertyInspector } from "./features/model-tree/PropertyInspector";
import { loadPreviewModel } from "./services/previewService";

describe("OpenPipeStress desktop preview", () => {
  it("renders the engineering workspace from invented local fixtures", async () => {
    render(<App />);

    expect(await screen.findByText("OpenPipeStress Technical Preview")).toBeInTheDocument();
    expect(await screen.findByTestId("desktop-preview-shell")).toBeInTheDocument();
    expect(await screen.findByTestId("solve-panel")).toBeInTheDocument();
    const initialReadiness = screen.getByTestId("solve-readiness-summary");
    expect(within(initialReadiness).getByTestId("readiness-mechanics").textContent).toContain(
      "preview run not started"
    );
    expect(within(initialReadiness).getByTestId("readiness-rule").textContent).toContain("rule inputs incomplete");
    expect(within(initialReadiness).getByTestId("readiness-rule").textContent).toContain(
      "mechanics results remain reviewable only"
    );
    expect(within(initialReadiness).getByTestId("readiness-diagnostics").textContent).toContain("1 diagnostic");
    expect(within(initialReadiness).getByTestId("readiness-diagnostics").textContent).toContain("1 warning");
    expect(within(initialReadiness).getByTestId("readiness-professional").textContent).toContain(
      "human review required"
    );
    expect(screen.getByTestId("solve-job-summary").textContent).toContain("state=not_started");
    expect(screen.getByTestId("solve-job-summary").textContent).toContain("events=1");
    expect(screen.getByTestId("solve-job-summary").textContent).toContain("result_rows=0");
    expect(screen.getByTestId("solve-job-summary").textContent).toContain("cancellation_requested=false");
    expect(screen.getByTestId("solve-job-progress").textContent).toContain("not_started");
    expect(screen.getByTestId("solve-job-progress").textContent).toContain(
      "preview_service_event_state_only_no_percent_stream"
    );
    expect(screen.getByTestId("solve-job-progress").textContent).toContain("percentages_synthesized=false");
    expect(screen.getByTestId("solve-job-cancellation").textContent).toContain("token=TBD");
    expect(screen.getByTestId("solve-job-cancellation").textContent).toContain("success_claimed=false");
    expect(screen.getByTestId("solve-job-binding").textContent).toContain("not generated");
    expect(screen.getByTestId("solve-job-boundary").textContent).toContain("private payload=false");
    expect(screen.getByTestId("cancel-mechanics-preview")).toBeDisabled();
    const solveJobHref = screen.getByTestId("solve-job-export-link").getAttribute("href") ?? "";
    const solveJobPacket = JSON.parse(decodeURIComponent(solveJobHref.split(",", 2)[1]));
    expect(solveJobPacket.document_kind).toBe("openpipestress.technical_preview.solve_job_audit");
    expect(solveJobPacket.deliverable_refs).toContain("DEL-07-07");
    expect(solveJobPacket.scope_items).toContain("SOW-055");
    expect(solveJobPacket.summary.job_state).toBe("not_started");
    expect(solveJobPacket.summary.event_count).toBe(1);
    expect(solveJobPacket.summary.result_row_count).toBe(0);
    expect(solveJobPacket.summary.cancellation_requested).toBe(false);
    expect(solveJobPacket.progress_contract.percentages_synthesized).toBe(false);
    expect(solveJobPacket.progress_contract.backend_percent_stream_available).toBe(false);
    expect(solveJobPacket.cancellation.backend_cancellation_token).toBe("TBD");
    expect(solveJobPacket.cancellation.mutates_solver_process_directly).toBe(false);
    expect(solveJobPacket.cancellation.cancellation_success_claimed).toBe(false);
    expect(solveJobPacket.private_payload_included).toBe(false);
    expect(solveJobPacket.protected_content_included).toBe(false);
    expect(solveJobPacket.release_or_professional_claim).toBe(false);
    expect(solveJobPacket.professional_boundary.software_makes_compliance_claim).toBe(false);
    const ruleCheck = await screen.findByLabelText("Rule-check completeness");
    expect(within(ruleCheck).getByTestId("rule-check-summary").textContent).toContain("4 review findings");
    expect(within(ruleCheck).getByTestId("rule-check-summary").textContent).toContain("rule_check_blocked=true");
    expect(within(ruleCheck).getByTestId("rule-check-summary").textContent).toContain(
      "mechanics_reviewable=false"
    );
    expect(within(ruleCheck).getByTestId("rule-check-status").textContent).toContain(
      "not_performed_user_rule_inputs_missing"
    );
    expect(within(ruleCheck).getByTestId("rule-check-boundary").textContent).toContain(
      "silent defaults used=false"
    );
    expect(within(ruleCheck).getByTestId("rule-check-boundary").textContent).toContain("no compliance claim");
    expect(within(ruleCheck).getByTestId("rule-check-finding-required-rule-inputs-missing").textContent).toContain(
      "RULE_CHECK_BLOCKING"
    );
    expect(
      within(ruleCheck).getByTestId("rule-check-finding-component-flexibility-factor-provenance").textContent
    ).toContain("PROVENANCE_WARNING");
    expect(await screen.findByLabelText("Model tree")).toBeInTheDocument();
    expect(await screen.findByLabelText("Three.js pipe centerline viewport")).toBeInTheDocument();
    const runAudit = await screen.findByLabelText("Run audit");
    expect(within(runAudit).getByTestId("run-audit-empty").textContent).toContain(
      "Run mechanics preview to generate immutable model-state and analysis-run audit references"
    );
    expect(runAudit.textContent).toContain("not a release or professional acceptance record");
    const resultExport = await screen.findByLabelText("Result export audit");
    expect(within(resultExport).getByTestId("result-export-empty").textContent).toContain(
      "schema-first local result envelope"
    );
    const headlessRunner = await screen.findByLabelText("Headless runner envelope");
    expect(within(headlessRunner).getByTestId("headless-runner-summary").textContent).toContain("available");
    expect(within(headlessRunner).getByTestId("headless-runner-summary").textContent).toContain("job=TBD");
    expect(within(headlessRunner).getByTestId("headless-runner-summary").textContent).toContain("outputs=4");
    expect(within(headlessRunner).getByTestId("headless-runner-summary").textContent).toContain("result_refs=1");
    expect(within(headlessRunner).getByTestId("headless-runner-interface").textContent).toContain(
      "schema_first_headless_runner_contract"
    );
    expect(within(headlessRunner).getByTestId("headless-runner-interface").textContent).toContain("cli=TBD");
    expect(within(headlessRunner).getByTestId("headless-runner-runtime-tbds").textContent).toContain(
      "network=TBD"
    );
    expect(within(headlessRunner).getByTestId("headless-runner-boundary").textContent).toContain(
      "network_required=false"
    );
    expect(within(headlessRunner).getByTestId("headless-runner-boundary").textContent).toContain(
      "no compliance, certification, sealing, authentication, or approval claim"
    );
    const headlessHref = within(headlessRunner).getByTestId("headless-runner-export-link").getAttribute("href") ?? "";
    const headlessPacket = JSON.parse(decodeURIComponent(headlessHref.split(",", 2)[1]));
    expect(headlessPacket.deliverable_id).toBe("DEL-10-05");
    expect(headlessPacket.package_id).toBe("PKG-10");
    expect(headlessPacket.scope_items).toContain("SOW-054");
    expect(headlessPacket.scope_items).toContain("SOW-032");
    expect(headlessPacket.objectives).toContain("OBJ-008");
    expect(headlessPacket.objectives).toContain("OBJ-009");
    expect(headlessPacket.objectives).toContain("OBJ-012");
    expect(headlessPacket.runner_status.interface_kind).toBe("schema_first_headless_runner_contract");
    expect(headlessPacket.runner_status.physical_project_container.direct_sql_access_allowed).toBe(false);
    expect(headlessPacket.runner_status.physical_project_container.network_required).toBe(false);
    expect(headlessPacket.tbd_decisions.final_cli_command_syntax).toBe("TBD");
    expect(headlessPacket.tbd_decisions.process_invocation).toBe("TBD");
    expect(headlessPacket.tbd_decisions.network_access).toBe("TBD");
    expect(headlessPacket.tbd_decisions.filesystem_mutation_policy).toBe("TBD");
    expect(headlessPacket.request.operation).toBe("solve");
    expect(headlessPacket.request.requested_outputs).toContain("result_envelope");
    expect(headlessPacket.request.requested_outputs).toContain("audit_manifest");
    expect(headlessPacket.request.privacy.local_only).toBe(true);
    expect(headlessPacket.request.privacy.telemetry_allowed).toBe(false);
    expect(headlessPacket.result.job.state).toBe("TBD");
    expect(headlessPacket.result.analysis_status).toContain("MODEL_INCOMPLETE");
    expect(headlessPacket.result.analysis_status).toContain("HUMAN_REVIEW_REQUIRED");
    expect(headlessPacket.result.result_envelope_ref.compatibility).toBe("schema_first_json_result_envelope");
    expect(headlessPacket.result.result_refs).toHaveLength(1);
    expect(headlessPacket.result.checksums[0].algorithm).toBe("TBD");
    expect(headlessPacket.result.privacy.local_only).toBe(true);
    expect(headlessPacket.result.professional_boundary.software_makes_compliance_claim).toBe(false);
    const adapterFramework = await screen.findByLabelText("Adapter framework envelope");
    expect(within(adapterFramework).getByTestId("adapter-framework-summary").textContent).toContain("available");
    expect(within(adapterFramework).getByTestId("adapter-framework-summary").textContent).toContain(
      "capabilities=4"
    );
    expect(within(adapterFramework).getByTestId("adapter-framework-summary").textContent).toContain(
      "parse=not_parsed_by_framework"
    );
    expect(within(adapterFramework).getByTestId("adapter-framework-status").textContent).toContain(
      "schema_first_format_neutral_adapter_framework"
    );
    expect(within(adapterFramework).getByTestId("adapter-framework-status").textContent).toContain(
      "formats=TBD"
    );
    expect(within(adapterFramework).getByTestId("adapter-framework-capabilities").textContent).toContain(
      "validate_payload"
    );
    expect(within(adapterFramework).getByTestId("adapter-framework-validation").textContent).toContain(
      "schema=required"
    );
    expect(within(adapterFramework).getByTestId("adapter-framework-no-bypass").textContent).toContain(
      "direct_sql=false"
    );
    expect(within(adapterFramework).getByTestId("adapter-framework-no-bypass").textContent).toContain(
      "network=false"
    );
    expect(within(adapterFramework).getByTestId("adapter-framework-runtime-tbds").textContent).toContain(
      "plugin=TBD"
    );
    expect(within(adapterFramework).getByTestId("adapter-framework-boundary").textContent).toContain(
      "private_redacted=true"
    );
    const adapterHref =
      within(adapterFramework).getByTestId("adapter-framework-export-link").getAttribute("href") ?? "";
    const adapterPacket = JSON.parse(decodeURIComponent(adapterHref.split(",", 2)[1]));
    expect(adapterPacket.deliverable_id).toBe("DEL-10-02");
    expect(adapterPacket.package_id).toBe("PKG-10");
    expect(adapterPacket.scope_item).toBe("SOW-030");
    expect(adapterPacket.objective).toBe("OBJ-009");
    expect(adapterPacket.framework_status.interface_kind).toBe("schema_first_format_neutral_adapter_framework");
    expect(adapterPacket.framework_status.external_format_list).toBe("TBD");
    expect(adapterPacket.framework_status.public_transport_protocol).toBe("TBD");
    expect(adapterPacket.tbd_decisions.plugin_runtime).toBe("TBD");
    expect(adapterPacket.tbd_decisions.package_scripts).toBe("TBD");
    expect(adapterPacket.adapter_declaration.adapter_id).toBe("ops.adapter.desktop_preview");
    expect(adapterPacket.adapter_declaration.capabilities).toContain("export_model");
    expect(adapterPacket.adapter_declaration.capabilities).toContain("export_results");
    expect(adapterPacket.adapter_declaration.capabilities).toContain("validate_payload");
    expect(adapterPacket.adapter_declaration.no_bypass_controls.must_route_persistence_through_application_services).toBe(true);
    expect(adapterPacket.adapter_declaration.no_bypass_controls.must_not_expose_sql_or_raw_sqlite).toBe(true);
    expect(adapterPacket.adapter_declaration.no_bypass_controls.must_not_access_network).toBe(true);
    expect(adapterPacket.adapter_declaration.no_bypass_controls.must_not_claim_code_compliance).toBe(true);
    expect(adapterPacket.validation_plan.schema_validation).toBe("required");
    expect(adapterPacket.validation_plan.unit_validation).toBe("required");
    expect(adapterPacket.validation_plan.protected_content_screening).toBe("required");
    expect(adapterPacket.validation_plan.export_review).toBe("required_before_shared_payload");
    expect(adapterPacket.operation_result.parse_status).toBe("not_parsed_by_framework");
    expect(adapterPacket.operation_result.result_envelope_ref.compatibility).toBe(
      "schema_first_json_result_envelope"
    );
    expect(adapterPacket.operation_result.checksums[0].algorithm).toBe("TBD");
    expect(adapterPacket.operation_result.privacy.telemetry_allowed).toBe(false);
    expect(adapterPacket.operation_result.professional_boundary.software_makes_compliance_claim).toBe(false);
    expect(adapterPacket.operation_result.professional_boundary.software_makes_security_certification_claim).toBe(false);
    const localFea = await screen.findByLabelText("Local FEA handoff");
    expect(within(localFea).getByTestId("local-fea-empty").textContent).toContain("target-neutral local shell/solid FEA");
    expect(localFea.textContent).toContain("no mesh");
    expect(localFea.textContent).toContain("external solver");
    const externalProver = await screen.findByLabelText("External prover boundary metadata");
    expect(within(externalProver).getByTestId("external-prover-summary").textContent).toContain("available");
    expect(within(externalProver).getByTestId("external-prover-summary").textContent).toContain(
      "metadata=non_authoritative_workflow_metadata"
    );
    expect(within(externalProver).getByTestId("external-prover-summary").textContent).toContain("refs=1");
    expect(within(externalProver).getByTestId("external-prover-summary").textContent).toContain("diagnostics=4");
    expect(within(externalProver).getByTestId("external-prover-contract").textContent).toContain("DEL-15-04");
    expect(within(externalProver).getByTestId("external-prover-groups").textContent).toContain("attachments=1");
    expect(within(externalProver).getByTestId("external-prover-context-links").textContent).toContain(
      "handoff=1"
    );
    expect(within(externalProver).getByTestId("external-prover-unsupported-flags").textContent).toContain(
      "external_solver_not_invoked"
    );
    expect(within(externalProver).getByTestId("external-prover-run-boundary").textContent).toContain(
      "tool_invoked=false"
    );
    expect(within(externalProver).getByTestId("external-prover-authority-boundary").textContent).toContain(
      "compliance=false"
    );
    const externalHref =
      within(externalProver).getByTestId("external-prover-export-link").getAttribute("href") ?? "";
    const externalPacket = JSON.parse(decodeURIComponent(externalHref.split(",", 2)[1]));
    expect(externalPacket.deliverable_id).toBe("DEL-15-04");
    expect(externalPacket.package_id).toBe("PKG-15");
    expect(externalPacket.scope_item).toBe("SOW-075");
    expect(externalPacket.objectives).toContain("OBJ-017");
    expect(externalPacket.objectives).toContain("OBJ-018");
    expect(externalPacket.metadata_contract_status).toBe("non_authoritative_workflow_metadata");
    expect(externalPacket.tags).toContain("run-context-pending");
    expect(externalPacket.external_references[0].external_ref.ref).toBe("external:desktop-preview-metadata-only");
    expect(externalPacket.attachments[0].payload_embedded).toBe(false);
    expect(externalPacket.handoff_package_refs[0].ref.ref).toBe("handoff:TBD");
    expect(externalPacket.immutable_model_state_refs[0].ref.ref).toBe("state:TBD");
    expect(externalPacket.unsupported_target_flags).toHaveLength(3);
    expect(externalPacket.diagnostics).toHaveLength(4);
    expect(externalPacket.professional_boundary.external_tool_invoked).toBe(false);
    expect(externalPacket.professional_boundary.commercial_result_payload_ingested).toBe(false);
    expect(externalPacket.professional_boundary.software_creates_external_validation_record).toBe(false);
    expect(externalPacket.professional_boundary.software_makes_compliance_claim).toBe(false);
    const reviewGeometry = await screen.findByLabelText("Review geometry export");
    expect(within(reviewGeometry).getByTestId("review-geometry-summary").textContent).toContain("available");
    expect(within(reviewGeometry).getByTestId("review-geometry-summary").textContent).toContain(
      "format=glTF_2_0_json_preview"
    );
    expect(within(reviewGeometry).getByTestId("review-geometry-summary").textContent).toContain("segments=4");
    expect(within(reviewGeometry).getByTestId("review-geometry-summary").textContent).toContain("nodes=5");
    expect(within(reviewGeometry).getByTestId("review-geometry-summary").textContent).toContain("stable_ids=19");
    expect(within(reviewGeometry).getByTestId("review-geometry-format").textContent).toContain("asset=2.0");
    expect(within(reviewGeometry).getByTestId("review-geometry-format").textContent).toContain("units=m");
    expect(within(reviewGeometry).getByTestId("review-geometry-state-binding").textContent).toContain(
      "not generated"
    );
    expect(within(reviewGeometry).getByTestId("review-geometry-state-binding").textContent).toContain("state:TBD");
    expect(within(reviewGeometry).getByTestId("review-geometry-coverage").textContent).toContain("pipe_segments");
    expect(within(reviewGeometry).getByTestId("review-geometry-coverage").textContent).toContain("supports");
    expect(within(reviewGeometry).getByTestId("review-geometry-coverage").textContent).toContain("load_cases");
    expect(within(reviewGeometry).getByTestId("review-geometry-stable-ids").textContent).toContain("sidecar=required");
    expect(within(reviewGeometry).getByTestId("review-geometry-boundary").textContent).toContain(
      "visual_review_geometry_only"
    );
    expect(within(reviewGeometry).getByTestId("review-geometry-boundary").textContent).toContain(
      "solver_geometry_equivalence=false"
    );
    const geometryHref =
      within(reviewGeometry).getByTestId("review-geometry-export-link").getAttribute("href") ?? "";
    const geometryPacket = JSON.parse(decodeURIComponent(geometryHref.split(",", 2)[1]));
    expect(geometryPacket.document_kind).toBe("openpipestress.technical_preview.review_geometry_export");
    expect(geometryPacket.deliverable_id).toBe("DEL-17-08");
    expect(geometryPacket.package_id).toBe("PKG-17");
    expect(geometryPacket.scope_items).toContain("SOW-030");
    expect(geometryPacket.scope_items).toContain("SOW-074");
    expect(geometryPacket.objectives).toContain("OBJ-009");
    expect(geometryPacket.objectives).toContain("OBJ-017");
    expect(geometryPacket.review_geometry_status).toBe("visual_review_geometry_only");
    expect(geometryPacket.review_geometry_profile.gltf_version_basis).toBe("2.0");
    expect(geometryPacket.review_geometry_profile.target_artifact).toBe("glTF_2_0_json_preview");
    expect(geometryPacket.review_geometry_profile.glb_binary_writer_status).toBe("TBD");
    expect(geometryPacket.geometry_summary.pipe_segment_count).toBe(4);
    expect(geometryPacket.geometry_summary.stable_id_count).toBe(19);
    expect(geometryPacket.gltf_asset.asset.version).toBe("2.0");
    expect(geometryPacket.gltf_asset.meshes[0].primitives[0].mode).toBe(1);
    expect(geometryPacket.gltf_asset.meshes[1].primitives[0].mode).toBe(0);
    expect(geometryPacket.gltf_asset.buffers[0].byteLength).toBeGreaterThan(0);
    expect(geometryPacket.gltf_asset.extras.gltf_boundary).toBe("visual_review_only_not_solver_geometry");
    expect(geometryPacket.sidecar_id_map.entries).toHaveLength(19);
    expect(
      geometryPacket.sidecar_id_map.entries.find((entry: { stable_id: string }) => entry.stable_id === "pipe:P-120")
        .export_status
    ).toBe("emitted");
    expect(
      geometryPacket.sidecar_id_map.entries.find((entry: { stable_id: string }) => entry.stable_id === "support:S-120")
        .export_status
    ).toBe("approximated");
    expect(
      geometryPacket.loss_report.entries.find((entry: { loss_id: string }) => entry.loss_id === "loss:glb-and-viewer-behavior-tbd")
        .category
    ).toBe("TBD");
    expect(geometryPacket.private_payload_included).toBe(false);
    expect(geometryPacket.protected_content_included).toBe(false);
    expect(geometryPacket.solver_geometry_equivalence_claim).toBe(false);
    expect(geometryPacket.professional_validation_claim).toBe(false);
    expect(geometryPacket.target_compatibility_claim).toBe(false);
    expect(geometryPacket.professional_boundary.software_makes_compliance_claim).toBe(false);
    const pcfExport = await screen.findByLabelText("Conservative PCF export");
    expect(within(pcfExport).getByTestId("pcf-export-summary").textContent).toContain("available");
    expect(within(pcfExport).getByTestId("pcf-export-summary").textContent).toContain("segments=4");
    expect(within(pcfExport).getByTestId("pcf-export-summary").textContent).toContain("nodes=5");
    expect(within(pcfExport).getByTestId("pcf-export-summary").textContent).toContain(
      "validation=blocked_missing_explicit_pcf_target_fields"
    );
    expect(within(pcfExport).getByTestId("pcf-export-profile").textContent).toContain("target_version=TBD");
    expect(within(pcfExport).getByTestId("pcf-export-state-binding").textContent).toContain(
      "not generated"
    );
    expect(within(pcfExport).getByTestId("pcf-export-state-binding").textContent).toContain("state:TBD");
    expect(within(pcfExport).getByTestId("pcf-export-coverage").textContent).toContain("pipe_segments");
    expect(within(pcfExport).getByTestId("pcf-export-coverage").textContent).toContain("nominal_size");
    expect(within(pcfExport).getByTestId("pcf-export-package").textContent).toContain("members=6");
    expect(within(pcfExport).getByTestId("pcf-export-package").textContent).toContain("stable_ids=4");
    expect(within(pcfExport).getByTestId("pcf-export-package").textContent).toContain("diagnostics=4");
    await waitFor(() =>
      expect(within(pcfExport).getByTestId("pcf-export-package").textContent).toContain(
        "package_hash=computed_local_preview_sha256"
      )
    );
    expect(within(pcfExport).getByTestId("pcf-export-boundary").textContent).toContain(
      "target_compatibility=false"
    );
    expect(within(pcfExport).getByTestId("pcf-export-boundary").textContent).toContain(
      "solver_validation=false"
    );
    const pcfHref = within(pcfExport).getByTestId("pcf-export-link").getAttribute("href") ?? "";
    const pcfPacket = JSON.parse(decodeURIComponent(pcfHref.split(",", 2)[1]));
    expect(pcfPacket.document_kind).toBe("openpipestress.technical_preview.conservative_pcf_export_package");
    expect(pcfPacket.manifest.canonical_package_hash_status).toBe("computed_local_preview_sha256");
    expect(pcfPacket.manifest.canonical_package_hash.value).toMatch(/^sha256:[0-9a-f]{64}$/);
    expect(pcfPacket.validation_report.hash_validation_status).toBe(
      "package_hash_computed_local_preview_not_independently_validated"
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
    expect(pcfPacket.export_profile.identity_policy).toBe("authoritative_sidecar_id_map");
    expect(pcfPacket.pcf_payload.nodes).toHaveLength(5);
    expect(pcfPacket.pcf_payload.pipe_segments).toHaveLength(4);
    expect(pcfPacket.pcf_text).toContain("UNITS-CO-ORDS MM");
    expect(pcfPacket.pcf_text).toContain("COMPONENT-IDENTIFIER OPS-PIPE-P-120");
    expect(pcfPacket.pcf_text).toContain("TBD_SOURCE_REQUIRED");
    expect(pcfPacket.pcf_text).toContain("END-ISOGEN");
    expect(pcfPacket.stable_id_map).toHaveLength(4);
    expect(pcfPacket.loss_report).toHaveLength(6);
    expect(pcfPacket.loss_report.map((entry: { category: string }) => entry.category)).toContain("tbd");
    expect(pcfPacket.validation_report.validation_status).toBe("blocked_missing_explicit_pcf_target_fields");
    expect(pcfPacket.validation_report.schema_validation_status).toBe(
      "desktop_preview_shape_aligned_not_runtime_json_schema_validated"
    );
    expect(pcfPacket.diagnostics.map((item: { code: string }) => item.code)).toContain(
      "PCF-NOMINAL-SIZE-TBD"
    );
    expect(pcfPacket.private_payload_included).toBe(false);
    expect(pcfPacket.protected_content_included).toBe(false);
    expect(pcfPacket.target_compatibility_claim).toBe(false);
    expect(pcfPacket.solver_validation_claim).toBe(false);
    expect(pcfPacket.code_compliance_claim).toBe(false);
    expect(pcfPacket.professional_reliance_claim).toBe(false);
    const pcfTextHref = within(pcfExport).getByTestId("pcf-text-link").getAttribute("href") ?? "";
    expect(decodeURIComponent(pcfTextHref.split(",", 2)[1])).toContain("END-ISOGEN");
    const caepipeMbfExport = await screen.findByLabelText("CAEPIPE MBF export");
    expect(within(caepipeMbfExport).getByTestId("caepipe-mbf-summary").textContent).toContain("available");
    expect(within(caepipeMbfExport).getByTestId("caepipe-mbf-summary").textContent).toContain("nodes=5");
    expect(within(caepipeMbfExport).getByTestId("caepipe-mbf-summary").textContent).toContain("pipes=4");
    expect(within(caepipeMbfExport).getByTestId("caepipe-mbf-summary").textContent).toContain("supports=3");
    expect(within(caepipeMbfExport).getByTestId("caepipe-mbf-summary").textContent).toContain(
      "validation=boundary_checked"
    );
    expect(within(caepipeMbfExport).getByTestId("caepipe-mbf-profile").textContent).toContain(
      "target_version=TBD-17-01-001"
    );
    expect(within(caepipeMbfExport).getByTestId("caepipe-mbf-profile").textContent).toContain(
      "subset=TBD-17-01-002"
    );
    expect(within(caepipeMbfExport).getByTestId("caepipe-mbf-state-binding").textContent).toContain(
      "not generated"
    );
    expect(within(caepipeMbfExport).getByTestId("caepipe-mbf-state-binding").textContent).toContain("state:TBD");
    expect(within(caepipeMbfExport).getByTestId("caepipe-mbf-coverage").textContent).toContain("nodes");
    expect(within(caepipeMbfExport).getByTestId("caepipe-mbf-coverage").textContent).toContain(
      "direct_stable_id"
    );
    expect(within(caepipeMbfExport).getByTestId("caepipe-mbf-package").textContent).toContain("members=6");
    expect(within(caepipeMbfExport).getByTestId("caepipe-mbf-package").textContent).toContain("stable_ids=14");
    expect(within(caepipeMbfExport).getByTestId("caepipe-mbf-package").textContent).toContain("diagnostics=0");
    expect(within(caepipeMbfExport).getByTestId("caepipe-mbf-boundary").textContent).toContain(
      "caepipe_compatibility=false"
    );
    expect(within(caepipeMbfExport).getByTestId("caepipe-mbf-boundary").textContent).toContain(
      "external_tool_invoked=false"
    );
    await waitFor(() =>
      expect(within(caepipeMbfExport).getByTestId("caepipe-mbf-package").textContent).toContain(
        "package_hash=computed_local_preview_sha256"
      )
    );
    const caepipeMbfHref = within(caepipeMbfExport).getByTestId("caepipe-mbf-export-link").getAttribute("href") ?? "";
    const caepipeMbfPacket = JSON.parse(decodeURIComponent(caepipeMbfHref.split(",", 2)[1]));
    expect(caepipeMbfPacket.document_kind).toBe("openpipestress.technical_preview.caepipe_mbf_export_package");
    expect(caepipeMbfPacket.manifest.canonical_package_hash_status).toBe("computed_local_preview_sha256");
    expect(caepipeMbfPacket.manifest.canonical_package_hash.value).toMatch(/^sha256:[0-9a-f]{64}$/);
    expect(caepipeMbfPacket.validation_report.hash_validation_status).toBe(
      "package_hash_computed_local_preview_not_independently_validated"
    );
    expect(caepipeMbfPacket.deliverable_id).toBe("DEL-17-04");
    expect(caepipeMbfPacket.package_id).toBe("PKG-17");
    expect(caepipeMbfPacket.scope_items).toContain("SOW-030");
    expect(caepipeMbfPacket.scope_items).toContain("SOW-074");
    expect(caepipeMbfPacket.scope_items).toContain("SOW-075");
    expect(caepipeMbfPacket.export_profile.target_family).toBe("caepipe_mbf");
    expect(caepipeMbfPacket.export_profile.target_version_basis).toBe("TBD-17-01-001");
    expect(caepipeMbfPacket.export_profile.record_subset_basis).toBe("TBD-17-01-002");
    expect(caepipeMbfPacket.export_profile.stable_id_policy).toBe(
      "sidecar_mapping_until_direct_mbf_carrier_confirmed"
    );
    expect(caepipeMbfPacket.export_profile.carried_tbd_refs).toContain("TBD-17-01-003");
    expect(caepipeMbfPacket.model_payload.nodes).toHaveLength(5);
    expect(caepipeMbfPacket.model_payload.elements).toHaveLength(4);
    expect(caepipeMbfPacket.model_payload.supports).toHaveLength(3);
    expect(caepipeMbfPacket.mbf_text).toContain("UNITS");
    expect(caepipeMbfPacket.mbf_text).toContain("PIPE,P003,N003,N004");
    expect(caepipeMbfPacket.mbf_text).toContain("END");
    expect(caepipeMbfPacket.stable_id_map).toHaveLength(14);
    expect(caepipeMbfPacket.loss_report).toHaveLength(6);
    expect(caepipeMbfPacket.loss_report.map((entry: { category: string }) => entry.category)).toContain("tbd");
    expect(caepipeMbfPacket.validation_report.validation_status).toBe("boundary_checked");
    expect(caepipeMbfPacket.validation_report.schema_validation_status).toBe(
      "desktop_preview_shape_aligned_not_runtime_json_schema_validated"
    );
    expect(caepipeMbfPacket.diagnostics).toHaveLength(0);
    expect(caepipeMbfPacket.private_payload_included).toBe(false);
    expect(caepipeMbfPacket.protected_content_included).toBe(false);
    expect(caepipeMbfPacket.caepipe_compatibility_claim).toBe(false);
    expect(caepipeMbfPacket.solver_validation_claim).toBe(false);
    expect(caepipeMbfPacket.code_compliance_claim).toBe(false);
    expect(caepipeMbfPacket.professional_reliance_claim).toBe(false);
    const caepipeMbfTextHref = within(caepipeMbfExport).getByTestId("caepipe-mbf-text-link").getAttribute("href") ?? "";
    expect(decodeURIComponent(caepipeMbfTextHref.split(",", 2)[1])).toContain("LOAD_CASES");
    const caepipeExternal = await screen.findByLabelText("CAEPIPE external harness");
    expect(within(caepipeExternal).getByTestId("caepipe-external-summary").textContent).toContain("available");
    expect(within(caepipeExternal).getByTestId("caepipe-external-summary").textContent).toContain(
      "status=parser_only_evidence"
    );
    expect(within(caepipeExternal).getByTestId("caepipe-external-summary").textContent).toContain("rows=3");
    expect(within(caepipeExternal).getByTestId("caepipe-external-summary").textContent).toContain("sections=2");
    expect(within(caepipeExternal).getByTestId("caepipe-external-summary").textContent).toContain(
      "external_invoked=false"
    );
    expect(within(caepipeExternal).getByTestId("caepipe-external-harness").textContent).toContain(
      "config=absent"
    );
    expect(within(caepipeExternal).getByTestId("caepipe-external-harness").textContent).toContain(
      "TBD-17-05-invocation-profile"
    );
    expect(within(caepipeExternal).getByTestId("caepipe-external-state-binding").textContent).toContain(
      "del-17-04"
    );
    expect(within(caepipeExternal).getByTestId("caepipe-external-state-binding").textContent).toContain(
      "not generated"
    );
    expect(within(caepipeExternal).getByTestId("caepipe-external-parser").textContent).toContain(
      "parsed_parser_only_fixture"
    );
    expect(within(caepipeExternal).getByTestId("caepipe-external-parser").textContent).toContain(
      "correlation=canonical_id_map"
    );
    expect(within(caepipeExternal).getByTestId("caepipe-external-boundary").textContent).toContain(
      "compatibility=false"
    );
    expect(within(caepipeExternal).getByTestId("caepipe-external-boundary").textContent).toContain(
      "code_compliance=false"
    );
    const caepipeExternalHref =
      within(caepipeExternal).getByTestId("caepipe-external-export-link").getAttribute("href") ?? "";
    const caepipeExternalPacket = JSON.parse(decodeURIComponent(caepipeExternalHref.split(",", 2)[1]));
    expect(caepipeExternalPacket.document_kind).toBe(
      "openpipestress.technical_preview.caepipe_external_run_package"
    );
    expect(caepipeExternalPacket.deliverable_id).toBe("DEL-17-05");
    expect(caepipeExternalPacket.package_id).toBe("PKG-17");
    expect(caepipeExternalPacket.scope_items).toContain("SOW-030");
    expect(caepipeExternalPacket.scope_items).toContain("SOW-046");
    expect(caepipeExternalPacket.scope_items).toContain("SOW-075");
    expect(caepipeExternalPacket.package_status).toBe("parser_only_evidence");
    expect(caepipeExternalPacket.mbf_package_ref.ref).toContain("del-17-04");
    expect(caepipeExternalPacket.executable_config.configured_path_state).toBe("absent");
    expect(caepipeExternalPacket.executable_config.license_responsibility_acknowledged).toBe(false);
    expect(caepipeExternalPacket.executable_config.environment_responsibility_acknowledged).toBe(false);
    expect(caepipeExternalPacket.command_profile.profile_id).toBe("TBD-17-05-invocation-profile");
    expect(caepipeExternalPacket.execution_result.attempted).toBe(false);
    expect(caepipeExternalPacket.parser_coverage).toHaveLength(2);
    expect(caepipeExternalPacket.parsed_csv.row_count).toBe(3);
    expect(
      caepipeExternalPacket.parsed_csv.rows.map((row: { correlation_status: string }) => row.correlation_status)
    ).toEqual(["canonical_id_map", "canonical_id_map", "canonical_id_map"]);
    expect(caepipeExternalPacket.csv_text).toContain("NODE_DISPLACEMENTS");
    expect(caepipeExternalPacket.csv_text).toContain("ELEMENT_FORCES");
    expect(caepipeExternalPacket.validation_report.validation_status).toBe("boundary_checked");
    expect(caepipeExternalPacket.validation_report.checks.map((item: { check_id: string }) => item.check_id)).toContain(
      "external_execution_not_attempted"
    );
    expect(caepipeExternalPacket.diagnostics).toHaveLength(0);
    expect(caepipeExternalPacket.private_payload_included).toBe(false);
    expect(caepipeExternalPacket.protected_content_included).toBe(false);
    expect(caepipeExternalPacket.caepipe_compatibility_claim).toBe(false);
    expect(caepipeExternalPacket.solver_validation_claim).toBe(false);
    expect(caepipeExternalPacket.code_compliance_claim).toBe(false);
    expect(caepipeExternalPacket.professional_reliance_claim).toBe(false);
    const caepipeExternalCsvHref =
      within(caepipeExternal).getByTestId("caepipe-external-csv-link").getAttribute("href") ?? "";
    expect(decodeURIComponent(caepipeExternalCsvHref.split(",", 2)[1])).toContain("section,stable_id,target_id");
    const exportAdapterSdk = await screen.findByLabelText("Export adapter SDK");
    expect(within(exportAdapterSdk).getByTestId("export-adapter-sdk-summary").textContent).toContain("available");
    expect(within(exportAdapterSdk).getByTestId("export-adapter-sdk-summary").textContent).toContain("targets=5");
    expect(within(exportAdapterSdk).getByTestId("export-adapter-sdk-summary").textContent).toContain(
      "capabilities=4"
    );
    expect(within(exportAdapterSdk).getByTestId("export-adapter-sdk-summary").textContent).toContain(
      "validation=boundary_checked"
    );
    expect(within(exportAdapterSdk).getByTestId("export-adapter-sdk-profile").textContent).toContain(
      "runtime=TBD"
    );
    expect(within(exportAdapterSdk).getByTestId("export-adapter-sdk-targets").textContent).toContain(
      "preview_targets=5"
    );
    expect(within(exportAdapterSdk).getByTestId("export-adapter-sdk-targets").textContent).toContain(
      "admitted_support_claims=0"
    );
    expect(within(exportAdapterSdk).getByTestId("export-adapter-sdk-validation").textContent).toContain(
      "loss_report=true"
    );
    expect(within(exportAdapterSdk).getByTestId("export-adapter-sdk-permissions").textContent).toContain(
      "filesystem=false"
    );
    expect(within(exportAdapterSdk).getByTestId("export-adapter-sdk-permissions").textContent).toContain(
      "network=false"
    );
    expect(within(exportAdapterSdk).getByTestId("export-adapter-sdk-boundary").textContent).toContain(
      "compatibility=false"
    );
    await waitFor(() =>
      expect(within(exportAdapterSdk).getByTestId("export-adapter-sdk-validation").textContent).toContain(
        "package_hash=computed_local_preview_sha256"
      )
    );
    const exportAdapterSdkHref =
      within(exportAdapterSdk).getByTestId("export-adapter-sdk-export-link").getAttribute("href") ?? "";
    const exportAdapterSdkPacket = JSON.parse(decodeURIComponent(exportAdapterSdkHref.split(",", 2)[1]));
    expect(exportAdapterSdkPacket.document_kind).toBe(
      "openpipestress.technical_preview.export_adapter_sdk_registry"
    );
    expect(exportAdapterSdkPacket.manifest.canonical_package_hash_status).toBe("computed_local_preview_sha256");
    expect(exportAdapterSdkPacket.manifest.canonical_package_hash.value).toMatch(/^sha256:[0-9a-f]{64}$/);
    expect(exportAdapterSdkPacket.validation_report.hash_validation_status).toBe(
      "package_hash_computed_local_preview_not_independently_validated"
    );
    expect(
      exportAdapterSdkPacket.target_registry.targets.find(
        (item: { target_id: string }) => item.target_id === "stress_neutral_csv_json_package"
      ).unresolved_tbd_refs
    ).toEqual([]);
    expect(exportAdapterSdkPacket.deliverable_id).toBe("DEL-17-09");
    expect(exportAdapterSdkPacket.package_id).toBe("PKG-17");
    expect(exportAdapterSdkPacket.scope_items).toContain("SOW-030");
    expect(exportAdapterSdkPacket.scope_items).toContain("SOW-074");
    expect(exportAdapterSdkPacket.scope_items).toContain("SOW-075");
    expect(exportAdapterSdkPacket.registry_id).toBe("ops.export_adapter_sdk.registry_preview");
    expect(exportAdapterSdkPacket.sdk_contract.sdk_contract_status).toBe("contract_level_preview");
    expect(exportAdapterSdkPacket.sdk_contract.source_basis_refs.map((item: { ref: string }) => item.ref)).toContain(
      "DEL-17-02"
    );
    expect(exportAdapterSdkPacket.sdk_contract.source_basis_refs.map((item: { ref: string }) => item.ref)).toContain(
      "DEL-10-01"
    );
    expect(exportAdapterSdkPacket.sdk_contract.capabilities).toContain("export_model");
    expect(exportAdapterSdkPacket.sdk_contract.capabilities).toContain("validate_payload");
    expect(exportAdapterSdkPacket.sdk_contract.runtime_model.plugin_runtime).toBe("TBD");
    expect(exportAdapterSdkPacket.sdk_contract.runtime_model.permission_taxonomy).toBe("TBD");
    expect(exportAdapterSdkPacket.sdk_contract.deny_by_default_controls.filesystem_access_granted).toBe(false);
    expect(exportAdapterSdkPacket.sdk_contract.deny_by_default_controls.network_access_granted).toBe(false);
    expect(exportAdapterSdkPacket.sdk_contract.deny_by_default_controls.process_access_granted).toBe(false);
    expect(exportAdapterSdkPacket.sdk_contract.deny_by_default_controls.private_data_access_granted).toBe(false);
    expect(exportAdapterSdkPacket.adapter_template.sample_adapter_implementation_included).toBe(false);
    expect(exportAdapterSdkPacket.adapter_template.source_basis_required_before_support_claim).toBe(true);
    expect(exportAdapterSdkPacket.target_registry.summary.target_count).toBe(5);
    expect(exportAdapterSdkPacket.target_registry.summary.preview_panel_available_count).toBe(5);
    expect(exportAdapterSdkPacket.target_registry.summary.admitted_support_claim_count).toBe(0);
    expect(exportAdapterSdkPacket.target_registry.targets.map((item: { target_id: string }) => item.target_id)).toContain(
      "caepipe_mbf_export"
    );
    expect(exportAdapterSdkPacket.target_registry.targets.map((item: { target_id: string }) => item.target_id)).toContain(
      "conservative_pcf_export"
    );
    expect(exportAdapterSdkPacket.validation_report.validation_status).toBe("boundary_checked");
    expect(exportAdapterSdkPacket.validation_report.checks.loss_report_required).toBe(true);
    expect(exportAdapterSdkPacket.validation_report.checks.no_support_or_compatibility_claims).toBe(true);
    expect(exportAdapterSdkPacket.diagnostics).toHaveLength(0);
    expect(exportAdapterSdkPacket.private_payload_included).toBe(false);
    expect(exportAdapterSdkPacket.protected_content_included).toBe(false);
    expect(exportAdapterSdkPacket.target_compatibility_claim).toBe(false);
    expect(exportAdapterSdkPacket.solver_validation_claim).toBe(false);
    expect(exportAdapterSdkPacket.code_compliance_claim).toBe(false);
    expect(exportAdapterSdkPacket.professional_reliance_claim).toBe(false);
    const stressNeutral = await screen.findByLabelText("Stress-neutral CSV JSON export");
    expect(within(stressNeutral).getByTestId("stress-neutral-empty").textContent).toContain(
      "stress-neutral CSV/JSON package"
    );
    const buildReadiness = await screen.findByLabelText("Build package readiness");
    expect(within(buildReadiness).getByTestId("build-readiness-summary").textContent).toContain("available");
    expect(within(buildReadiness).getByTestId("build-readiness-summary").textContent).toContain("scripts=8");
    expect(within(buildReadiness).getByTestId("build-readiness-summary").textContent).toContain(
      "bundle_active=false"
    );
    expect(within(buildReadiness).getByTestId("build-readiness-shell").textContent).toContain(
      "OpenPipeStress Technical Preview"
    );
    expect(within(buildReadiness).getByTestId("build-readiness-commands").textContent).toContain("build:desktop");
    expect(within(buildReadiness).getByTestId("build-readiness-commands").textContent).toContain("tauri");
    expect(within(buildReadiness).getByTestId("build-readiness-profiles").textContent).toContain("skeleton");
    expect(within(buildReadiness).getByTestId("build-readiness-decisions").textContent).toContain("ci=TBD");
    expect(within(buildReadiness).getByTestId("build-readiness-boundary").textContent).toContain(
      "release_authorized=false"
    );
    const buildReadinessHref =
      within(buildReadiness).getByTestId("build-readiness-export-link").getAttribute("href") ?? "";
    const buildReadinessPacket = JSON.parse(decodeURIComponent(buildReadinessHref.split(",", 2)[1]));
    expect(buildReadinessPacket.document_kind).toBe(
      "openpipestress.technical_preview.build_package_readiness"
    );
    expect(buildReadinessPacket.deliverable_id).toBe("DEL-10-04");
    expect(buildReadinessPacket.package_id).toBe("PKG-10");
    expect(buildReadinessPacket.scope_item).toBe("SOW-032");
    expect(buildReadinessPacket.objectives).toContain("OBJ-008");
    expect(buildReadinessPacket.objectives).toContain("OBJ-009");
    expect(buildReadinessPacket.summary.tauri_shell_status).toBe("present");
    expect(buildReadinessPacket.summary.bundle_active).toBe(false);
    expect(buildReadinessPacket.workspace.root_scripts).toContain("build:desktop");
    expect(buildReadinessPacket.workspace.desktop_scripts).toContain("tauri");
    expect(buildReadinessPacket.tauri_shell.before_build_command).toBe("npm run build");
    expect(buildReadinessPacket.release_readiness_tool.default_profile).toBe("skeleton");
    expect(buildReadinessPacket.release_readiness_tool.browser_panel_runs_tool).toBe(false);
    expect(buildReadinessPacket.unresolved_decisions.ci_provider).toBe("TBD");
    expect(buildReadinessPacket.unresolved_decisions.release_matrix).toBe("TBD");
    expect(buildReadinessPacket.unresolved_decisions.signing_notarization).toBe("TBD");
    expect(buildReadinessPacket.boundary.network_required).toBe(false);
    expect(buildReadinessPacket.boundary.telemetry_enabled).toBe(false);
    expect(buildReadinessPacket.boundary.release_publication_authorized).toBe(false);
    expect(buildReadinessPacket.boundary.installer_or_binary_generated).toBe(false);
    expect(buildReadinessPacket.boundary.code_compliance_claim).toBe(false);
    expect(buildReadinessPacket.boundary.professional_reliance_claim).toBe(false);
    const validationEvidence = await screen.findByLabelText("Validation evidence review");
    expect(within(validationEvidence).getByTestId("validation-evidence-summary").textContent).toContain(
      "manual=10"
    );
    expect(within(validationEvidence).getByTestId("validation-evidence-summary").textContent).toContain(
      "evidence=8"
    );
    expect(within(validationEvidence).getByTestId("validation-evidence-summary").textContent).toContain(
      "profiles=5"
    );
    expect(within(validationEvidence).getByTestId("validation-evidence-manual").textContent).toContain(
      "DRAFT_EVIDENCE"
    );
    expect(within(validationEvidence).getByTestId("validation-evidence-inventory").textContent).toContain(
      "gui=PLANNED"
    );
    expect(within(validationEvidence).getByTestId("validation-evidence-release-checks").textContent).toContain(
      "skeleton_checks=2"
    );
    expect(within(validationEvidence).getByTestId("validation-evidence-gates").textContent).toContain(
      "thresholds=TBD"
    );
    expect(within(validationEvidence).getByTestId("validation-evidence-gates").textContent).toContain(
      "release_authorized=false"
    );
    expect(within(validationEvidence).getByTestId("validation-evidence-boundary").textContent).toContain(
      "release_claim=false"
    );
    const validationEvidenceHref =
      within(validationEvidence).getByTestId("validation-evidence-export-link").getAttribute("href") ?? "";
    const validationEvidencePacket = JSON.parse(decodeURIComponent(validationEvidenceHref.split(",", 2)[1]));
    expect(validationEvidencePacket.document_kind).toBe(
      "openpipestress.technical_preview.validation_release_evidence_review"
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
    expect(validationEvidencePacket.summary.required_release_path_count).toBe(7);
    expect(validationEvidencePacket.summary.skeleton_check_count).toBe(2);
    expect(validationEvidencePacket.validation_manual.professional_reliance_outside_software_authority).toBe(true);
    expect(validationEvidencePacket.release_readiness_tool.browser_panel_runs_tool).toBe(false);
    expect(validationEvidencePacket.release_readiness_tool.latest_dag_dependency_edges).toBe(
      "execution/_DAG/DAG-006/DependencyEdges.csv"
    );
    expect(validationEvidencePacket.release_quality_gates.release_publication_authorized).toBe(false);
    expect(validationEvidencePacket.release_quality_gates.final_threshold_policy).toBe("TBD");
    expect(validationEvidencePacket.gui_validation_context.current_tranche_smoke_record).toBe("TP-MAC-81");
    expect(validationEvidencePacket.private_payload_included).toBe(false);
    expect(validationEvidencePacket.protected_content_included).toBe(false);
    expect(validationEvidencePacket.release_or_professional_claim).toBe(false);
    const editorContract = await screen.findByLabelText("Editor contract review");
    expect(within(editorContract).getByTestId("editor-contract-summary").textContent).toContain("editors=4");
    expect(within(editorContract).getByTestId("editor-contract-summary").textContent).toContain("surfaces=7");
    expect(within(editorContract).getByTestId("editor-contract-summary").textContent).toContain("diagnostics=4");
    expect(within(editorContract).getByTestId("editor-contract-summary").textContent).toContain(
      "private_payload=false"
    );
    expect(within(editorContract).getByTestId("editor-contract-coverage").textContent).toContain(
      "materials:review_editor_available"
    );
    expect(within(editorContract).getByTestId("editor-contract-coverage").textContent).toContain(
      "viewport_gestures:operation_intent_editor_available"
    );
    expect(within(editorContract).getByTestId("editor-contract-coverage").textContent).toContain(
      "private_libraries:reference_slots_only_no_private_payload"
    );
    expect(within(editorContract).getByTestId("editor-contract-validation").textContent).toContain("ready=2");
    expect(within(editorContract).getByTestId("editor-contract-validation").textContent).toContain("blocked=2");
    expect(within(editorContract).getByTestId("editor-contract-rule-pack").textContent).toContain(
      "rule-pack:user-supplied:not-loaded"
    );
    expect(within(editorContract).getByTestId("editor-contract-rule-pack").textContent).toContain(
      "checksum=TBD_private_rule_pack_not_loaded"
    );
    expect(within(editorContract).getByTestId("editor-contract-mutation-boundary").textContent).toContain(
      "direct_model_mutation_allowed=false"
    );
    expect(within(editorContract).getByTestId("editor-contract-professional-boundary").textContent).toContain(
      "compliance=false"
    );
    const editorContractHref =
      within(editorContract).getByTestId("editor-contract-export-link").getAttribute("href") ?? "";
    const editorContractPacket = JSON.parse(decodeURIComponent(editorContractHref.split(",", 2)[1]));
    expect(editorContractPacket.document_kind).toBe("openpipestress.technical_preview.editor_contract_review");
    expect(editorContractPacket.deliverable_id).toBe("DEL-07-03");
    expect(editorContractPacket.package_id).toBe("PKG-07");
    expect(editorContractPacket.scope_item).toBe("SOW-021");
    expect(editorContractPacket.objectives).toContain("OBJ-006");
    expect(editorContractPacket.summary.editor_count).toBe(4);
    expect(editorContractPacket.summary.surface_count).toBe(7);
    expect(editorContractPacket.summary.ready_editor_count).toBe(2);
    expect(editorContractPacket.summary.blocked_editor_count).toBe(2);
    expect(editorContractPacket.summary.diagnostic_count).toBe(4);
    expect(editorContractPacket.summary.queued_intent_count).toBe(0);
    expect(editorContractPacket.editors.map((item: { editor_kind: string }) => item.editor_kind)).toContain(
      "material"
    );
    expect(editorContractPacket.editors.map((item: { editor_kind: string }) => item.editor_kind)).toContain(
      "component"
    );
    expect(editorContractPacket.editors.map((item: { editor_kind: string }) => item.editor_kind)).toContain(
      "rule_pack_reference"
    );
    expect(
      editorContractPacket.surface_inventory.map((item: { surface: string }) => item.surface)
    ).toContain("private_libraries");
    expect(
      editorContractPacket.diagnostics.map((item: { code: string }) => item.code)
    ).toContain("RULE_PACK_REQUIRED_INPUTS_MISSING");
    expect(
      editorContractPacket.diagnostics.map((item: { code: string }) => item.code)
    ).toContain("PRIVATE_LIBRARY_PAYLOAD_NOT_LOADED");
    expect(editorContractPacket.rule_pack_reference.private_payload_redacted).toBe(true);
    expect(editorContractPacket.source_of_truth_boundary.direct_model_mutation_allowed).toBe(false);
    expect(editorContractPacket.source_of_truth_boundary.accepted_model_state_mutated).toBe(false);
    expect(editorContractPacket.private_payload_included).toBe(false);
    expect(editorContractPacket.protected_content_included).toBe(false);
    expect(editorContractPacket.release_or_professional_claim).toBe(false);
    expect(editorContractPacket.professional_boundary.software_makes_compliance_claim).toBe(false);
    const missingData = await screen.findByLabelText("Missing-data blocking review");
    expect(within(missingData).getByTestId("missing-data-summary").textContent).toContain("classes=6");
    expect(within(missingData).getByTestId("missing-data-summary").textContent).toContain("active=5");
    expect(within(missingData).getByTestId("missing-data-summary").textContent).toContain("solve_blocked=false");
    expect(within(missingData).getByTestId("missing-data-summary").textContent).toContain("rule_blocked=true");
    expect(within(missingData).getByTestId("missing-data-class-coverage").textContent).toContain(
      "SOLVE_BLOCKING:available_no_active_preview_item"
    );
    expect(within(missingData).getByTestId("missing-data-class-coverage").textContent).toContain(
      "RULE_CHECK_BLOCKING:active"
    );
    expect(within(missingData).getByTestId("missing-data-class-coverage").textContent).toContain(
      "IP_BOUNDARY_WARNING:available_no_active_preview_item"
    );
    expect(within(missingData).getByTestId("missing-data-status-separation").textContent).toContain(
      "mechanics=ready_for_preview_diagnostics"
    );
    expect(within(missingData).getByTestId("missing-data-status-separation").textContent).toContain(
      "mechanics_reviewable=false"
    );
    expect(within(missingData).getByTestId("missing-data-blocking").textContent).toContain("solve=0");
    expect(within(missingData).getByTestId("missing-data-blocking").textContent).toContain("rule=2");
    expect(within(missingData).getByTestId("missing-data-assistive").textContent).toContain(
      "class,severity,affected_refs,message,remediation"
    );
    expect(within(missingData).getByTestId("missing-data-assistive").textContent).toContain("color_only=false");
    expect(within(missingData).getByTestId("missing-data-boundary").textContent).toContain(
      "silent_defaults=false"
    );
    expect(within(missingData).getByTestId("missing-data-boundary").textContent).toContain(
      "auto_fill=false"
    );
    expect(within(missingData).getByTestId("missing-data-warning-rule-check-required-inputs").textContent).toContain(
      "RULE_CHECK_BLOCKING"
    );
    const missingDataHref = within(missingData).getByTestId("missing-data-export-link").getAttribute("href") ?? "";
    const missingDataPacket = JSON.parse(decodeURIComponent(missingDataHref.split(",", 2)[1]));
    expect(missingDataPacket.document_kind).toBe(
      "openpipestress.technical_preview.missing_data_warning_blocking_review"
    );
    expect(missingDataPacket.deliverable_id).toBe("DEL-07-04");
    expect(missingDataPacket.package_id).toBe("PKG-07");
    expect(missingDataPacket.scope_item).toBe("SOW-022");
    expect(missingDataPacket.objectives).toContain("OBJ-011");
    expect(missingDataPacket.summary.warning_class_count).toBe(6);
    expect(missingDataPacket.summary.active_warning_count).toBe(5);
    expect(missingDataPacket.summary.solve_blocking_count).toBe(0);
    expect(missingDataPacket.summary.rule_check_blocking_count).toBe(2);
    expect(missingDataPacket.summary.mechanics_solve_blocked).toBe(false);
    expect(missingDataPacket.summary.rule_check_blocked).toBe(true);
    expect(missingDataPacket.summary.mechanics_results_reviewable).toBe(false);
    expect(missingDataPacket.summary.silent_defaults_used).toBe(false);
    expect(missingDataPacket.auto_fill_missing_data).toBe(false);
    expect(
      missingDataPacket.class_inventory.map((item: { warning_class: string }) => item.warning_class)
    ).toContain("NONLINEAR_WARNING");
    expect(missingDataPacket.analysis_boundary_contract.gui_warning_class_status_map.SOLVE_BLOCKING).toContain(
      "MODEL_INCOMPLETE"
    );
    expect(missingDataPacket.assistive_contract.color_only_signaling_allowed).toBe(false);
    expect(missingDataPacket.source_of_truth_boundary.direct_model_mutation_allowed).toBe(false);
    expect(missingDataPacket.private_payload_included).toBe(false);
    expect(missingDataPacket.protected_content_included).toBe(false);
    expect(missingDataPacket.release_or_professional_claim).toBe(false);
    expect(missingDataPacket.professional_boundary.software_makes_compliance_claim).toBe(false);
    const accessibilityBaseline = await screen.findByLabelText("Accessibility and usability baseline");
    expect(within(accessibilityBaseline).getByTestId("accessibility-baseline-summary").textContent).toContain(
      "surfaces=6"
    );
    expect(within(accessibilityBaseline).getByTestId("accessibility-baseline-summary").textContent).toContain(
      "findings=69"
    );
    expect(within(accessibilityBaseline).getByTestId("accessibility-baseline-workflow").textContent).toContain(
      "keyboard=4"
    );
    expect(within(accessibilityBaseline).getByTestId("accessibility-baseline-open-target").textContent).toContain(
      "gui=TBD"
    );
    expect(within(accessibilityBaseline).getByTestId("accessibility-baseline-boundary").textContent).toContain(
      "conformance_claim=false"
    );
    const accessibilityHref =
      within(accessibilityBaseline).getByTestId("accessibility-baseline-export-link").getAttribute("href") ?? "";
    const accessibilityPacket = JSON.parse(decodeURIComponent(accessibilityHref.split(",", 2)[1]));
    expect(accessibilityPacket.document_kind).toBe(
      "openpipestress.technical_preview.accessibility_usability_baseline_review"
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
    expect(accessibilityPacket.summary.accessibility_target_status).toBe("TBD_by_human_project_authority");
    expect(accessibilityPacket.summary.desktop_runtime_evaluation).toBe("not_performed");
    expect(accessibilityPacket.source_surfaces.map((item: { deliverable_id: string }) => item.deliverable_id)).toContain(
      "DEL-07-05"
    );
    expect(accessibilityPacket.reviewed_categories).toContain("keyboard_path");
    expect(accessibilityPacket.reviewed_categories).toContain("warning_visibility");
    expect(accessibilityPacket.review_policy.color_only_status_signaling_allowed).toBe(false);
    expect(accessibilityPacket.review_policy.software_makes_accessibility_conformance_claim).toBe(false);
    expect(accessibilityPacket.open_decisions.accessibility_conformance_target).toBe("TBD");
    expect(accessibilityPacket.open_decisions.desktop_runtime_accessibility_tree_audit).toBe("TBD");
    expect(accessibilityPacket.private_payload_included).toBe(false);
    expect(accessibilityPacket.protected_content_included).toBe(false);
    expect(accessibilityPacket.release_or_professional_claim).toBe(false);
    const designWorkspace = await screen.findByLabelText("Design-authoring workspace");
    expect(within(designWorkspace).getByTestId("design-workspace-summary").textContent).toContain(
      "knowledge=3"
    );
    expect(within(designWorkspace).getByTestId("design-workspace-summary").textContent).toContain("runs=0");
    expect(within(designWorkspace).getByTestId("design-workspace-summary").textContent).toContain(
      "comparisons=0"
    );
    expect(within(designWorkspace).getByTestId("design-workspace-core").textContent).toContain("records=2");
    expect(within(designWorkspace).getByTestId("design-workspace-core").textContent).toContain("states=2");
    expect(within(designWorkspace).getByTestId("design-workspace-current").textContent).toContain(
      "analysis_run=not generated"
    );
    expect(within(designWorkspace).getByTestId("design-workspace-operation").textContent).toContain(
      "accepted_mutation=false"
    );
    expect(within(designWorkspace).getByTestId("design-workspace-boundary").textContent).toContain(
      "professional_claim=false"
    );
    const designWorkspaceHref =
      within(designWorkspace).getByTestId("design-workspace-export-link").getAttribute("href") ?? "";
    const designWorkspacePacket = JSON.parse(decodeURIComponent(designWorkspaceHref.split(",", 2)[1]));
    expect(designWorkspacePacket.document_kind).toBe(
      "openpipestress.technical_preview.design_authoring_comparison_workspace"
    );
    expect(designWorkspacePacket.deliverable_id).toBe("DEL-07-08");
    expect(designWorkspacePacket.package_id).toBe("PKG-07");
    expect(designWorkspacePacket.scope_item).toBe("SOW-076");
    expect(designWorkspacePacket.objectives).toContain("OBJ-015");
    expect(designWorkspacePacket.objectives).toContain("OBJ-016");
    expect(designWorkspacePacket.summary.current_design_knowledge_record_count).toBe(3);
    expect(designWorkspacePacket.summary.current_model_state_count).toBe(0);
    expect(designWorkspacePacket.summary.current_analysis_run_count).toBe(0);
    expect(designWorkspacePacket.summary.current_comparison_pair_count).toBe(0);
    expect(designWorkspacePacket.core_contract_evidence.design_knowledge_record_count).toBe(2);
    expect(designWorkspacePacket.core_contract_evidence.constraint_warning_count).toBe(1);
    expect(designWorkspacePacket.core_contract_evidence.model_state_count).toBe(2);
    expect(designWorkspacePacket.core_contract_evidence.analysis_run_count).toBe(2);
    expect(designWorkspacePacket.core_contract_evidence.graphical_overlay_count).toBe(5);
    expect(designWorkspacePacket.current_workspace_state.state_run_browser).toBe("pending_mechanics_run");
    expect(designWorkspacePacket.mutation_boundary.workspace_mutates_accepted_model_state).toBe(false);
    expect(designWorkspacePacket.mutation_boundary.accepted_model_state_mutated).toBe(false);
    expect(designWorkspacePacket.private_payload_included).toBe(false);
    expect(designWorkspacePacket.protected_content_included).toBe(false);
    expect(designWorkspacePacket.release_or_professional_claim).toBe(false);
    const reportLint = await screen.findByLabelText("Report content lint");
    expect(within(reportLint).getByTestId("report-lint-summary").textContent).toContain("targets=21");
    expect(within(reportLint).getByTestId("report-lint-summary").textContent).toContain("findings=0");
    expect(within(reportLint).getByTestId("report-lint-summary").textContent).toContain("blocking=0");
    expect(within(reportLint).getByTestId("report-lint-scope").textContent).toContain(
      "explicit_public_surfaces"
    );
    expect(within(reportLint).getByTestId("report-lint-scope").textContent).toContain(
      "skip_unless_explicitly_authorized"
    );
    expect(within(reportLint).getByTestId("report-lint-clean-scan").textContent).toContain(
      "clearance=false"
    );
    expect(within(reportLint).getByTestId("report-lint-boundary").textContent).toContain("legal=false");
    expect(within(reportLint).getByTestId("report-lint-boundary").textContent).toContain(
      "professional=false"
    );
    const lintHref = within(reportLint).getByTestId("report-lint-export-link").getAttribute("href") ?? "";
    const lintPacket = JSON.parse(decodeURIComponent(lintHref.split(",", 2)[1]));
    expect(lintPacket.deliverable_id).toBe("DEL-08-05");
    expect(lintPacket.scope_item).toBe("SOW-043");
    expect(lintPacket.objectives).toContain("OBJ-002");
    expect(lintPacket.objectives).toContain("OBJ-007");
    expect(lintPacket.linter_status.heuristic_only).toBe(true);
    expect(lintPacket.linter_status.legal_clearance).toBe(false);
    expect(lintPacket.linter_status.professional_approval).toBe(false);
    expect(lintPacket.linter_status.ci_release_policy).toBe("TBD");
    expect(lintPacket.lint_run.configuration.private_surface_default).toBe(
      "skip_unless_explicitly_authorized"
    );
    expect(lintPacket.lint_run.configuration.clean_scan_disclaimer).toBe(
      "heuristic_review_evidence_not_legal_or_professional_clearance"
    );
    expect(lintPacket.lint_run.summary.target_count).toBe(21);
    expect(lintPacket.lint_run.summary.finding_count).toBe(0);
    expect(lintPacket.lint_run.summary.blocking_finding_count).toBe(0);
    expect(lintPacket.lint_run.summary.clean_scan_is_clearance).toBe(false);
    const nativePackage = await screen.findByLabelText("Native JSON package");
    expect(within(nativePackage).getByTestId("native-package-empty").textContent).toContain(
      "native JSON package review manifest"
    );
    const comparison = await screen.findByLabelText("Comparison workspace");
    expect(within(comparison).getByTestId("comparison-empty").textContent).toContain(
      "Run mechanics preview to populate the local comparison workspace"
    );
    const handoff = await screen.findByLabelText("Handoff package");
    expect(within(handoff).getByTestId("handoff-empty").textContent).toContain(
      "Run mechanics preview to assemble a local review handoff package"
    );
    const operationLedger = await screen.findByLabelText("Operation review ledger");
    expect(within(operationLedger).getByTestId("operation-ledger-empty").textContent).toContain(
      "No structured operations are queued for review"
    );
    expect(operationLedger.textContent).toContain("does not apply operations");
    const diffPreview = await screen.findByLabelText("Operation diff preview");
    expect(within(diffPreview).getByTestId("diff-preview-empty").textContent).toContain(
      "No operation diffs are queued"
    );
    expect(diffPreview.textContent).toContain("does not apply operations");
    const exportReview = await screen.findByLabelText("Export safety review");
    expect(within(exportReview).getByTestId("export-review-summary").textContent).toContain(
      "20 of 27 local exports ready"
    );
    expect(within(exportReview).getByTestId("export-review-redaction").textContent).toContain(
      "protected content blocked=true"
    );
    expect(within(exportReview).getByTestId("export-review-boundary").textContent).toContain(
      "no release or professional claim"
    );
    expect(within(exportReview).getByTestId("export-review-record-telemetry_boundary_review").textContent).toContain(
      "available"
    );
    expect(
      within(exportReview).getByTestId("export-review-record-secret_private_library_boundary_review").textContent
    ).toContain("available");
    expect(within(exportReview).getByTestId("export-review-record-security_threat_model_review").textContent).toContain(
      "available"
    );
    expect(
      within(exportReview).getByTestId("export-review-record-accessibility_usability_baseline_review").textContent
    ).toContain("available");
    expect(
      within(exportReview).getByTestId("export-review-record-design_authoring_comparison_workspace").textContent
    ).toContain("available");
    expect(
      within(exportReview).getByTestId("export-review-record-validation_release_evidence_review").textContent
    ).toContain("available");
    expect(screen.getByText("Human review required")).toBeInTheDocument();
    expect(screen.getByText("Local preview store")).toBeInTheDocument();
    expect(screen.getByLabelText("Local project controls")).toBeInTheDocument();
    expect(screen.getByTestId("local-project-status").textContent).toContain("Browser memory preview local store");
    expect(screen.getByTestId("local-project-status").textContent).toContain("network=false");
    expect(screen.getByTestId("local-project-status").textContent).toContain("telemetry=false");
    expect(screen.getByTestId("local-project-status").textContent).toContain("FTS5=false");
    const storageAudit = await screen.findByLabelText("Project storage audit");
    expect(within(storageAudit).getByTestId("project-storage-summary").textContent).toContain(
      "operation=not_started"
    );
    expect(within(storageAudit).getByTestId("project-storage-summary").textContent).toContain(
      "pending operations=0"
    );
    expect(within(storageAudit).getByTestId("project-storage-summary").textContent).toContain(
      "accepted_state_mutated=false"
    );
    expect(within(storageAudit).getByTestId("project-storage-capability").textContent).toContain(
      "Browser memory preview"
    );
    expect(within(storageAudit).getByTestId("project-storage-capability").textContent).toContain(
      "mode=not_persisted_this_session"
    );
    expect(within(storageAudit).getByTestId("project-storage-local-boundary").textContent).toContain(
      "network=false"
    );
    expect(within(storageAudit).getByTestId("project-storage-local-boundary").textContent).toContain(
      "repository_default_private_write=false"
    );
    expect(within(storageAudit).getByTestId("project-storage-project-index").textContent).toContain(
      "state=not_requested"
    );
    expect(within(storageAudit).getByTestId("project-storage-project-index").textContent).toContain(
      "listed_projects=0"
    );
    expect(within(storageAudit).getByTestId("project-storage-project-index").textContent).toContain("refs=none");
    expect(screen.queryByTestId("project-index-picker")).toBeNull();
    expect(within(storageAudit).getByTestId("project-storage-payload-boundary").textContent).toContain(
      "private payload=false"
    );
    expect(within(storageAudit).getByTestId("project-storage-payload-boundary").textContent).toContain(
      "release/professional claim=false"
    );
    const storageHref = within(storageAudit).getByTestId("project-storage-export-link").getAttribute("href") ?? "";
    const storagePacket = JSON.parse(decodeURIComponent(storageHref.split(",", 2)[1]));
    expect(storagePacket.document_kind).toBe("openpipestress.technical_preview.local_project_persistence_audit");
    expect(storagePacket.deliverable_refs).toContain("DEL-02-05");
    expect(storagePacket.deliverable_refs).toContain("DEL-12-01");
    expect(storagePacket.scope_items).toContain("SOW-050");
    expect(storagePacket.scope_items).toContain("SOW-029");
    expect(storagePacket.summary.last_operation).toBe("not_started");
    expect(storagePacket.summary.storage_mode).toBe("not_persisted_this_session");
    expect(storagePacket.summary.pending_operation_count).toBe(0);
    expect(storagePacket.summary.accepted_model_state_mutated).toBe(false);
    expect(storagePacket.summary.network_required).toBe(false);
    expect(storagePacket.summary.daemon_required).toBe(false);
    expect(storagePacket.summary.telemetry_enabled).toBe(false);
    expect(storagePacket.summary.project_index_state).toBe("not_requested");
    expect(storagePacket.summary.listed_project_count).toBe(0);
    expect(storagePacket.project_index).toEqual([]);
    expect(storagePacket.project_index_refs).toEqual([]);
    expect(storagePacket.boundary.repository_default_private_write).toBe(false);
    expect(storagePacket.private_payload_included).toBe(false);
    expect(storagePacket.protected_content_included).toBe(false);
    expect(storagePacket.release_or_professional_claim).toBe(false);
    const projectValidation = await screen.findByLabelText("Project validation preflight");
    expect(within(projectValidation).getByTestId("project-validation-summary").textContent).toContain(
      "validation=preview_not_persisted"
    );
    expect(within(projectValidation).getByTestId("project-validation-summary").textContent).toContain(
      "version=supported_current_schema"
    );
    expect(within(projectValidation).getByTestId("project-validation-summary").textContent).toContain(
      "migration=not_persisted_this_session"
    );
    await waitFor(() =>
      expect(within(projectValidation).getByTestId("project-validation-store-migration").textContent).toContain(
        "framework=browser_memory_preview_no_sqlite_migration_ledger"
      )
    );
    expect(within(projectValidation).getByTestId("project-validation-store-migration").textContent).toContain(
      "applied_on_open=0"
    );
    expect(within(projectValidation).getByTestId("project-validation-store-migration").textContent).toContain(
      "status=browser_memory_snapshot_no_sql_store_migrations_applicable"
    );
    expect(within(projectValidation).getByTestId("project-validation-schema").textContent).toContain(
      "schema_version=0.1.0"
    );
    expect(within(projectValidation).getByTestId("project-validation-round-trip").textContent).toContain(
      "6 categories"
    );
    await waitFor(() =>
      expect(within(projectValidation).getByTestId("project-validation-model-hash").textContent).toContain(
        "model_hash=model_hash_computed_not_persisted"
      )
    );
    expect(within(projectValidation).getByTestId("project-validation-model-hash").textContent).toContain(
      "persisted_model_hashes=0"
    );
    expect(within(projectValidation).getByTestId("project-validation-model-hash").textContent).toContain(
      "integrity=open_verification_not_run_this_session"
    );
    expect(within(projectValidation).getByTestId("project-validation-envelope-hash").textContent).toContain(
      "envelope_hash=project_envelope_hash_not_computed_no_save_this_session"
    );
    expect(within(projectValidation).getByTestId("project-validation-envelope-hash").textContent).toContain(
      "persisted_envelope_hashes=0"
    );
    expect(within(projectValidation).getByTestId("project-validation-envelope-hash").textContent).toContain(
      "integrity=open_verification_not_run_this_session"
    );
    expect(within(projectValidation).getByTestId("project-validation-round-trip").textContent).toContain(
      "reproducibility=model_hash_computed_not_persisted"
    );
    expect(within(projectValidation).getByTestId("project-validation-operations").textContent).toContain(
      "version_check=supported_current_schema"
    );
    expect(within(projectValidation).getByTestId("project-validation-boundary").textContent).toContain(
      "accepted_state_mutated=false"
    );
    const validationHref =
      within(projectValidation).getByTestId("project-validation-export-link").getAttribute("href") ?? "";
    const validationPacket = JSON.parse(decodeURIComponent(validationHref.split(",", 2)[1]));
    expect(validationPacket.document_kind).toBe("openpipestress.technical_preview.project_validation_preflight");
    expect(validationPacket.deliverable_refs).toContain("DEL-02-05");
    expect(validationPacket.deliverable_refs).toContain("DEL-12-01");
    expect(validationPacket.scope_items).toContain("SOW-041");
    expect(validationPacket.scope_items).toContain("SOW-029");
    expect(validationPacket.summary.validation_status).toBe("preview_not_persisted");
    expect(validationPacket.summary.version_check_status).toBe("supported_current_schema");
    expect(validationPacket.summary.migration_status).toBe("not_persisted_this_session");
    expect(validationPacket.store_migration.evidence_source).toBe("storage_capability_probe");
    expect(validationPacket.store_migration.migration_framework).toBe("browser_memory_preview_no_sqlite_migration_ledger");
    expect(validationPacket.store_migration.migrations_applied_on_open).toEqual([]);
    expect(validationPacket.store_migration.model_document_migration_status).toBe(
      "model_document_migrations_not_defined_tbd"
    );
    expect(
      validationPacket.service_operations.find((operation: { operation: string }) => operation.operation === "migrate")
        .operation_status
    ).toBe("not_run_no_local_snapshot_this_session");
    expect(validationPacket.validation_profile.model_document_migration_status).toBe(
      "model_document_migrations_not_defined_tbd"
    );
    expect(validationPacket.summary.round_trip_status).toBe("semantic_categories_declared");
    expect(validationPacket.round_trip_manifest.category_count).toBe(6);
    expect(validationPacket.round_trip_manifest.categories.map((category: { category: string }) => category.category)).toContain(
      "unit_metadata"
    );
    expect(validationPacket.service_operations.map((operation: { operation: string }) => operation.operation)).toContain(
      "validate"
    );
    expect(validationPacket.service_operations.map((operation: { operation: string }) => operation.operation)).toContain(
      "version_check"
    );
    expect(validationPacket.service_operations.map((operation: { operation: string }) => operation.operation)).toContain(
      "migrate"
    );
    expect(validationPacket.validation_profile.hash_service_status).toBe(
      "canonical_model_hash_service_available_model_payload_scope"
    );
    expect(validationPacket.validation_profile.project_envelope_hash_status).toBe(
      "project_envelope_hash_not_computed_no_save_this_session"
    );
    expect(validationPacket.validation_profile.project_envelope_hash_scope).toBe(
      "persisted_envelope_payload_excluding_storage_summary_and_hash_carrier"
    );
    expect(validationPacket.summary.model_hash_status).toBe("model_hash_computed_not_persisted");
    expect(validationPacket.summary.persisted_model_hash_count).toBe(0);
    expect(validationPacket.summary.persisted_model_hash_ref).toBe("not_persisted");
    expect(validationPacket.summary.model_hash_integrity_status).toBe("open_verification_not_run_this_session");
    expect(validationPacket.model_hash.value).toMatch(/^sha256:[0-9a-f]{64}$/);
    expect(validationPacket.model_hash_integrity).toBeNull();
    expect(validationPacket.summary.project_envelope_hash_status).toBe(
      "project_envelope_hash_not_computed_no_save_this_session"
    );
    expect(validationPacket.summary.persisted_project_envelope_hash_count).toBe(0);
    expect(validationPacket.summary.persisted_project_envelope_hash_ref).toBe("not_persisted");
    expect(validationPacket.summary.project_envelope_hash_integrity_status).toBe(
      "open_verification_not_run_this_session"
    );
    expect(validationPacket.project_envelope_hash).toBeNull();
    expect(validationPacket.project_envelope_hash_integrity).toBeNull();
    expect(
      validationPacket.diagnostics.map((diagnosticEntry: { code: string }) => diagnosticEntry.code)
    ).toContain("PROJECT-VALIDATION-ENVELOPE-HASH-NOT-COMPUTED");
    expect(
      validationPacket.round_trip_manifest.categories.find(
        (category: { category: string }) => category.category === "reproducibility_metadata"
      ).semantic_equality_status
    ).toBe("model_hash_computed_not_persisted");
    expect(
      validationPacket.diagnostics.map((diagnosticEntry: { code: string }) => diagnosticEntry.code)
    ).toContain("PROJECT-VALIDATION-MODEL-HASH-REVIEW-ONLY");
    expect(
      validationPacket.diagnostics.map((diagnosticEntry: { code: string }) => diagnosticEntry.code)
    ).not.toContain("PROJECT-VALIDATION-HASH-SERVICE-TBD");
    expect(validationPacket.boundary.local_only_project_store).toBe(true);
    expect(validationPacket.boundary.repository_default_private_write).toBe(false);
    expect(validationPacket.boundary.accepted_model_state_mutated).toBe(false);
    const telemetryBoundary = await screen.findByLabelText("Telemetry boundary review");
    expect(within(telemetryBoundary).getByTestId("telemetry-boundary-summary").textContent).toContain(
      "disabled=true"
    );
    expect(within(telemetryBoundary).getByTestId("telemetry-boundary-summary").textContent).toContain(
      "blocked=4"
    );
    expect(within(telemetryBoundary).getByTestId("telemetry-boundary-summary").textContent).toContain(
      "network=false"
    );
    expect(within(telemetryBoundary).getByTestId("telemetry-boundary-config").textContent).toContain(
      "opt_in=false"
    );
    expect(within(telemetryBoundary).getByTestId("telemetry-boundary-guard").textContent).toContain(
      "metadata_only=true"
    );
    expect(within(telemetryBoundary).getByTestId("telemetry-boundary-attempts").textContent).toContain(
      "allowed=0"
    );
    expect(within(telemetryBoundary).getByTestId("telemetry-boundary-no-bypass").textContent).toContain(
      "plugins"
    );
    expect(within(telemetryBoundary).getByTestId("telemetry-boundary-runtime").textContent).toContain(
      "endpoint=false"
    );
    const telemetryHref =
      within(telemetryBoundary).getByTestId("telemetry-boundary-export-link").getAttribute("href") ?? "";
    const telemetryPacket = JSON.parse(decodeURIComponent(telemetryHref.split(",", 2)[1]));
    expect(telemetryPacket.document_kind).toBe("openpipestress.technical_preview.telemetry_boundary_review");
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
    expect(telemetryPacket.metadata_only_guard.helper_authorizes_runtime_telemetry).toBe(false);
    expect(telemetryPacket.event_attempts.map((item: { event_name: string }) => item.event_name)).toContain(
      "report_export_attempted"
    );
    expect(
      telemetryPacket.event_attempts.every(
        (item: { decision: { payload_constructed: boolean; network_behavior_initialized: boolean } }) =>
          item.decision.payload_constructed === false && item.decision.network_behavior_initialized === false
      )
    ).toBe(true);
    expect(telemetryPacket.runtime_initialization.endpoint_initialized).toBe(false);
    expect(telemetryPacket.runtime_initialization.vendor_initialized).toBe(false);
    expect(telemetryPacket.runtime_initialization.upload_queue_initialized).toBe(false);
    expect(telemetryPacket.runtime_initialization.telemetry_persistence_initialized).toBe(false);
    expect(telemetryPacket.no_bypass_surfaces).toContain("reports");
    expect(telemetryPacket.forbidden_payload_classes).toContain("protected_standards_content");
    expect(telemetryPacket.private_payload_included).toBe(false);
    expect(telemetryPacket.protected_content_included).toBe(false);
    expect(telemetryPacket.security_certification_claim).toBe(false);
    const secretPrivateLibrary = await screen.findByLabelText("Secret and private-library review");
    expect(within(secretPrivateLibrary).getByTestId("secret-private-library-summary").textContent).toContain(
      "records=4"
    );
    expect(within(secretPrivateLibrary).getByTestId("secret-private-library-summary").textContent).toContain(
      "metadata_only=true"
    );
    expect(within(secretPrivateLibrary).getByTestId("secret-private-library-summary").textContent).toContain(
      "public_fixture_blocked=4"
    );
    expect(within(secretPrivateLibrary).getByTestId("secret-private-library-secrets").textContent).toContain(
      "provider=TBD"
    );
    expect(within(secretPrivateLibrary).getByTestId("secret-private-library-no-bypass").textContent).toContain(
      "direct_sql=false"
    );
    expect(within(secretPrivateLibrary).getByTestId("secret-private-library-boundary").textContent).toContain(
      "security_certification=false"
    );
    const secretPrivateHref =
      within(secretPrivateLibrary).getByTestId("secret-private-library-export-link").getAttribute("href") ?? "";
    const secretPrivatePacket = JSON.parse(decodeURIComponent(secretPrivateHref.split(",", 2)[1]));
    expect(secretPrivatePacket.document_kind).toBe(
      "openpipestress.technical_preview.secret_private_library_boundary_review"
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
    expect(secretPrivatePacket.guard_contexts.public_fixture.summary.blocked_count).toBe(4);
    expect(secretPrivatePacket.guard_contexts.public_report.summary.metadata_only_count).toBe(4);
    expect(secretPrivatePacket.guard_contexts.local_private.explicit_local_private_intent).toBe(true);
    expect(
      secretPrivatePacket.registry_records.every(
        (item: { contains_payload: boolean; secret_material_present: boolean; concrete_path_present: boolean }) =>
          item.contains_payload === false &&
          item.secret_material_present === false &&
          item.concrete_path_present === false
      )
    ).toBe(true);
    expect(secretPrivatePacket.runtime_deferrals.exact_secret_provider).toBe("TBD");
    expect(secretPrivatePacket.runtime_deferrals.encrypted_storage_default).toBe("TBD");
    expect(secretPrivatePacket.no_bypass_controls.direct_sql_allowed).toBe(false);
    expect(secretPrivatePacket.no_bypass_controls.storage_bypass_allowed).toBe(false);
    expect(secretPrivatePacket.private_payload_included).toBe(false);
    expect(secretPrivatePacket.protected_content_included).toBe(false);
    expect(secretPrivatePacket.security_certification_claim).toBe(false);
    const securityThreatModel = await screen.findByLabelText("Security threat model review");
    expect(within(securityThreatModel).getByTestId("security-threat-model-summary").textContent).toContain(
      "threats=19"
    );
    expect(within(securityThreatModel).getByTestId("security-threat-model-summary").textContent).toContain(
      "high=14"
    );
    expect(within(securityThreatModel).getByTestId("security-threat-model-coverage").textContent).toContain(
      "workflows=6"
    );
    expect(within(securityThreatModel).getByTestId("security-threat-model-open-decisions").textContent).toContain(
      "tbd=13"
    );
    expect(within(securityThreatModel).getByTestId("security-threat-model-boundary").textContent).toContain(
      "security_certification=false"
    );
    const threatModelHref =
      within(securityThreatModel).getByTestId("security-threat-model-export-link").getAttribute("href") ?? "";
    const threatModelPacket = JSON.parse(decodeURIComponent(threatModelHref.split(",", 2)[1]));
    expect(threatModelPacket.document_kind).toBe("openpipestress.technical_preview.security_threat_model_review");
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
    expect(threatModelPacket.asset_classes).toContain("import_export_and_local_fea_handoff_data");
    expect(threatModelPacket.trust_boundaries).toContain("plugin_import_export_fea_handoff");
    expect(threatModelPacket.export_workflows).toContain("export_adapter_sdk_and_additional_targets");
    expect(threatModelPacket.no_bypass_controls.direct_sql_allowed).toBe(false);
    expect(threatModelPacket.no_bypass_controls.plugin_manifest_grants_runtime_access).toBe(false);
    expect(threatModelPacket.open_decisions.plugin_permission_model).toBe("TBD");
    expect(threatModelPacket.open_decisions.telemetry_event_schema).toBe("TBD");
    expect(threatModelPacket.private_payload_included).toBe(false);
    expect(threatModelPacket.protected_content_included).toBe(false);
    expect(threatModelPacket.security_certification_claim).toBe(false);
    expect(validationPacket.private_payload_included).toBe(false);
    expect(validationPacket.protected_content_included).toBe(false);
    expect(validationPacket.release_or_professional_claim).toBe(false);

    const boundary = screen.getByTestId("preview-boundary-strip");
    expect(within(boundary).getByText("Public data")).toBeInTheDocument();
    expect(boundary.textContent).toContain("invented_or_cleared_data_only");
    expect(boundary.textContent).toContain("no_bundled_protected_owner_or_standards_data");
    expect(boundary.textContent).toContain("no_private_project_data");
    expect(boundary.textContent).toContain("technical_preview_requires_human_engineering_review");
  });

  it("records viewport editor intents without direct persisted-project mutation", async () => {
    render(<App />);

    expect(await screen.findByLabelText("Three.js pipe centerline viewport")).toBeInTheDocument();
    const intentPanel = screen.getByLabelText("Viewport editor intents");
    expect(screen.getByTestId("local-project-review-context").textContent).toContain("0 pending operations");

    expect(within(intentPanel).getByTestId("viewport-intent-empty").textContent).toContain(
      "do not mutate persisted project data directly"
    );

    fireEvent.click(within(intentPanel).getByRole("button", { name: /Node intent/i }));
    fireEvent.click(within(intentPanel).getByRole("button", { name: /Pipe-run intent/i }));
    fireEvent.click(within(intentPanel).getByRole("button", { name: /Component intent/i }));

    const nodeIntent = within(intentPanel).getByTestId("viewport-intent-create_node");
    const pipeIntent = within(intentPanel).getByTestId("viewport-intent-connect_pipe_run");
    const componentIntent = within(intentPanel).getByTestId("viewport-intent-insert_component_symbol");

    for (const intent of [nodeIntent, pipeIntent, componentIntent]) {
      expect(intent.textContent).toContain("pending_service_validation");
      expect(intent.textContent).toContain("unit_aware_domain_validation_required");
      expect(intent.textContent).toContain("does_not_mutate_persisted_project_payload");
    }
    expect(componentIntent.textContent).toContain("editor-intent-3");
    expect(pipeIntent.textContent).toContain("editor-intent-2");
    expect(nodeIntent.textContent).toContain("editor-intent-1");
    expect(screen.getByTestId("local-project-review-context").textContent).toContain("3 pending operations");

    const editorContract = screen.getByLabelText("Editor contract review");
    expect(within(editorContract).getByTestId("editor-contract-validation").textContent).toContain("queued=3");
    const editorContractHref =
      within(editorContract).getByTestId("editor-contract-export-link").getAttribute("href") ?? "";
    const editorContractPacket = JSON.parse(decodeURIComponent(editorContractHref.split(",", 2)[1]));
    expect(editorContractPacket.summary.queued_intent_count).toBe(3);
    expect(editorContractPacket.summary.surface_count).toBe(7);
    expect(editorContractPacket.queued_operation_intent_refs).toContain("op:viewport-intent-create_node-001");
    expect(editorContractPacket.surface_inventory.map((item: { surface: string }) => item.surface)).toContain(
      "viewport_gestures"
    );

    const diffPreview = await screen.findByLabelText("Operation diff preview");
    expect(within(diffPreview).getByTestId("diff-preview-summary").textContent).toContain("3 operations");
    expect(within(diffPreview).getByTestId("diff-preview-summary").textContent).toContain("3 diff rows");
    expect(within(diffPreview).getByTestId("diff-preview-validation").textContent).toContain("3 held");
    const diffHref = within(diffPreview).getByTestId("diff-preview-export-link").getAttribute("href") ?? "";
    const diffPacket = JSON.parse(decodeURIComponent(diffHref.split(",", 2)[1]));
    expect(diffPacket.summary.operation_count).toBe(3);
    expect(diffPacket.summary.diff_row_count).toBe(3);
    expect(diffPacket.previews.map((item: { operation_id: string }) => item.operation_id)).toContain(
      "op:viewport-intent-create_node-001"
    );
    expect(diffPacket.previews.every((item: { accepted_model_state_mutated: boolean }) => item.accepted_model_state_mutated === false)).toBe(
      true
    );

    const operationLedger = await screen.findByLabelText("Operation review ledger");
    expect(within(operationLedger).getByTestId("operation-ledger-export-summary").textContent).toContain(
      "3 review records"
    );
    expect(within(operationLedger).getByTestId("operation-ledger-decision-counts").textContent).toContain(
      "3 held_for_user_acceptance"
    );
    const ledgerHref = within(operationLedger).getByTestId("operation-ledger-export-link").getAttribute("href") ?? "";
    const ledgerPacket = JSON.parse(decodeURIComponent(ledgerHref.split(",", 2)[1]));
    expect(ledgerPacket.decision_counts.held_for_user_acceptance).toBe(3);
    expect(ledgerPacket.records[0].source.source_ref).toBe("apps/desktop/src/features/viewport/PipeViewport.tsx");
    expect(ledgerPacket.records[0].actor.source_role).toBe("viewport_editor");
    expect(ledgerPacket.records.every((item: { accepted_model_state_ref: { ref: string } }) => item.accepted_model_state_ref.ref === "not generated")).toBe(
      true
    );
    expect(ledgerPacket.accepted_model_state_unchanged).toBe(true);
    expect(ledgerPacket.release_or_professional_claim).toBe(false);

    const exportReview = await screen.findByLabelText("Export safety review");
    expect(within(exportReview).getByTestId("export-review-summary").textContent).toContain(
      "21 of 27 local exports ready"
    );
    const reviewHref = within(exportReview).getByTestId("export-review-link").getAttribute("href") ?? "";
    const reviewManifest = JSON.parse(decodeURIComponent(reviewHref.split(",", 2)[1]));
    expect(reviewManifest.deliverable_refs).toContain("DEL-07-01");
    expect(reviewManifest.scope_items).toContain("SOW-020");
    expect(reviewManifest.summary.operation_record_count).toBe(3);
    expect(
      reviewManifest.exports.find((item: { export_id: string }) => item.export_id === "operation_review_ledger")
        .readiness
    ).toBe("available");

    fireEvent.click(within(operationLedger).getByTestId("clear-operation-review-queue"));

    expect(screen.getByTestId("local-project-review-context").textContent).toContain("0 pending operations");
    expect(within(intentPanel).getByTestId("viewport-intent-empty").textContent).toContain(
      "do not mutate persisted project data directly"
    );
    expect(within(editorContract).getByTestId("editor-contract-validation").textContent).toContain("queued=0");
    expect(await within(operationLedger).findByTestId("operation-ledger-empty")).toHaveTextContent(
      "No structured operations are queued"
    );
    expect(within(operationLedger).queryByTestId("operation-ledger-export-link")).not.toBeInTheDocument();
    expect(await within(diffPreview).findByTestId("diff-preview-empty")).toHaveTextContent("No operation diffs");

    const clearedReviewHref = within(exportReview).getByTestId("export-review-link").getAttribute("href") ?? "";
    const clearedReviewManifest = JSON.parse(decodeURIComponent(clearedReviewHref.split(",", 2)[1]));
    expect(clearedReviewManifest.summary.operation_record_count).toBe(0);
    expect(
      clearedReviewManifest.exports.find((item: { export_id: string }) => item.export_id === "operation_review_ledger")
        .readiness
    ).toBe("empty_operation_queue");
  });

  it("exposes materials, components, load cases, and combinations in the model workspace", async () => {
    render(<App />);

    const tree = await screen.findByLabelText("Model tree");
    expect(within(tree).getByText("Materials")).toBeInTheDocument();
    expect(within(tree).getByText("Components")).toBeInTheDocument();
    expect(within(tree).getByText("Load Cases")).toBeInTheDocument();
    expect(within(tree).getByText("Combinations")).toBeInTheDocument();
    expect(within(tree).getByTestId("model-tree-filter-summary").textContent).toContain(
      "19 of 19 model entities visible"
    );

    fireEvent.change(within(tree).getByTestId("model-tree-filter-input"), {
      target: { value: "component:C-110" }
    });
    expect(within(tree).getByTestId("model-tree-filter-summary").textContent).toContain(
      "1 of 19 model entities visible"
    );
    expect(within(tree).getByTestId("tree-row-component:C-110")).toBeInTheDocument();
    expect(within(tree).queryByText("Materials")).not.toBeInTheDocument();
    expect(within(tree).queryByText("Load Cases")).not.toBeInTheDocument();

    fireEvent.click(within(tree).getByTestId("tree-row-component:C-110"));
    const inspector = screen.getByLabelText("Property inspector");
    expect(within(inspector).getByRole("heading", { name: "Invented elbow marker" })).toBeInTheDocument();

    fireEvent.change(within(tree).getByTestId("model-tree-filter-input"), {
      target: { value: "carbon-steel-like" }
    });
    expect(within(tree).getByTestId("model-tree-filter-summary").textContent).toContain(
      "1 of 19 model entities visible"
    );
    expect(within(tree).getByTestId("tree-row-material:invented-carbon-steel")).toBeInTheDocument();
    expect(within(tree).queryByTestId("tree-row-component:C-110")).not.toBeInTheDocument();
    expect(within(inspector).getByRole("heading", { name: "Invented elbow marker" })).toBeInTheDocument();

    fireEvent.click(within(tree).getByRole("button", { name: /Invented carbon-steel-like material/i }));
    expect(within(inspector).getByRole("heading", { name: "Invented carbon-steel-like material" })).toBeInTheDocument();
    expect(inspector.textContent).toContain("material:invented-carbon-steel");
    expect(inspector.textContent).toContain("200000000000 Pa");
    expect(inspector.textContent).toContain("77000000000 Pa");
    expect(inspector.textContent).toContain("0.000012 1/degC");
    expect(inspector.textContent).toContain("invented_example_no_material_standard");
    const intentPanel = within(inspector).getByLabelText("Editor operation intent");
    expect(within(intentPanel).getByTestId("editor-operation-preview").textContent).toContain(
      "op:editor-intent-material:invented-carbon-steel-label"
    );
    expect(within(intentPanel).getByTestId("editor-intent-audit-boundary").textContent).toContain(
      "direct_model_mutation_allowed=false"
    );
    expect(within(intentPanel).getByTestId("editor-intent-professional-boundary").textContent).toContain(
      "software_makes_compliance_claim=false"
    );
    expect(within(intentPanel).getByTestId("queue-editor-intent")).toBeDisabled();
    fireEvent.change(within(intentPanel).getByTestId("editor-intent-field"), {
      target: { value: "elastic_modulus.value" }
    });
    fireEvent.change(within(intentPanel).getByTestId("editor-intent-value"), {
      target: { value: "210000000000" }
    });
    expect(within(intentPanel).getByTestId("editor-operation-preview").textContent).toContain(
      "Elastic modulus"
    );
    expect(within(intentPanel).getByTestId("editor-operation-preview").textContent).toContain(
      "before=200000000000; after=210000000000"
    );
    expect(within(intentPanel).getByTestId("editor-intent-validation").textContent).toContain("not_applied");
    fireEvent.click(within(intentPanel).getByTestId("queue-editor-intent"));
    expect(within(intentPanel).getByTestId("editor-intent-queue").textContent).toContain("editor-intent-1");
    expect(within(intentPanel).getByTestId("editor-intent-queue").textContent).toContain(
      "mutates_accepted_model_state=false"
    );
    expect(within(intentPanel).getByRole("button", { name: /Queue review intent/i })).not.toBeDisabled();
    expect(within(inspector).getByRole("heading", { name: "Invented carbon-steel-like material" })).toBeInTheDocument();

    fireEvent.change(within(tree).getByTestId("model-tree-filter-input"), {
      target: { value: "no-match-token" }
    });
    expect(within(tree).getByTestId("model-tree-filter-summary").textContent).toContain(
      "0 of 19 model entities visible"
    );
    expect(within(tree).getByTestId("model-tree-filter-empty").textContent).toContain(
      "No model entities match this filter"
    );
    expect(within(inspector).getByRole("heading", { name: "Invented carbon-steel-like material" })).toBeInTheDocument();

    fireEvent.click(within(tree).getByTestId("clear-model-tree-filter"));
    expect(within(tree).getByTestId("model-tree-filter-summary").textContent).toContain(
      "19 of 19 model entities visible"
    );
    expect(within(tree).getByText("Load Cases")).toBeInTheDocument();

    fireEvent.click(within(tree).getByRole("button", { name: /Invented elbow marker/i }));
    expect(within(inspector).getByRole("heading", { name: "Invented elbow marker" })).toBeInTheDocument();
    expect(inspector.textContent).toContain("component:C-110");
    expect(inspector.textContent).toContain("bend");
    expect(inspector.textContent).toContain("node:N-110");
    expect(inspector.textContent).toContain("invented_example_no_flexibility_factor");

    fireEvent.click(within(tree).getByRole("button", { name: /Invented operating gravity and pressure preview/i }));
    expect(
      within(inspector).getByRole("heading", { name: "Invented operating gravity and pressure preview" })
    ).toBeInTheDocument();
    expect(inspector.textContent).toContain("load:L-100");
    expect(inspector.textContent).toContain("primitive_user_load");
    expect(inspector.textContent).toContain("preview_only");
    expect(inspector.textContent).toContain("weight, occasional, pressure, thermal");
    expect(inspector.textContent).toContain("element:pipe:P-120");
    expect(inspector.textContent).toContain("node:node:N-140");

    fireEvent.click(within(tree).getByRole("button", { name: /Invented explicit operating plus alternate preview/i }));
    expect(
      within(inspector).getByRole("heading", { name: "Invented explicit operating plus alternate preview" })
    ).toBeInTheDocument();
    expect(inspector.textContent).toContain("combination:C-OPER-ALT");
    expect(inspector.textContent).toContain("mechanics");
    expect(inspector.textContent).toContain("load:L-100 x 1");
    expect(inspector.textContent).toContain("load:L-200 x 0.5");
    expect(inspector.textContent).toContain("invented_example_user_defined_mechanics_combination_no_code_default");
  });

  it("renders an empty editor-intent queue when no app queue has been materialized", async () => {
    const model = await loadPreviewModel();

    render(
      <PropertyInspector
        model={model}
        onQueueIntent={() => undefined}
        selection={{ type: "material", id: "material:invented-carbon-steel" }}
      />
    );

    const intentPanel = screen.getByLabelText("Editor operation intent");
    expect(within(intentPanel).getByTestId("editor-intent-queue-empty").textContent).toContain(
      "No transient editor intents queued"
    );
  });

  it("does not claim professional or release acceptance", async () => {
    render(<App />);

    const footer = await screen.findByText(/Technical preview only/i);
    expect(footer.textContent).toContain("no production-readiness");
    expect(footer.textContent).toContain("no licensed engineering reliance claim");
  });

  it("carries queued editor intents into the report packet as review-only operation context", async () => {
    render(<App />);

    const tree = await screen.findByLabelText("Model tree");
    fireEvent.click(within(tree).getByRole("button", { name: /Invented carbon-steel-like material/i }));
    const inspector = screen.getByLabelText("Property inspector");
    const intentPanel = within(inspector).getByLabelText("Editor operation intent");

    fireEvent.change(within(intentPanel).getByTestId("editor-intent-field"), {
      target: { value: "elastic_modulus.value" }
    });
    fireEvent.change(within(intentPanel).getByTestId("editor-intent-value"), {
      target: { value: "210000000000" }
    });
    fireEvent.click(within(intentPanel).getByTestId("queue-editor-intent"));
    expect(within(intentPanel).getByTestId("editor-intent-queue").textContent).toContain("editor-intent-1");

    fireEvent.click(screen.getByRole("button", { name: /Run mechanics preview/i }));
    const report = await screen.findByLabelText("Report packet");
    expect(await within(report).findByTestId("report-editor-intent-summary")).toHaveTextContent("1 queued");
    expect(within(report).getByTestId("report-editor-intent-operation").textContent).toContain(
      "op:editor-intent-material:invented-carbon-steel-elastic_modulus.value"
    );
    expect(within(report).getByTestId("report-editor-intent-operation").textContent).toContain("not_applied");
    expect(within(report).getByTestId("report-editor-intent-boundary").textContent).toContain(
      "does not mutate accepted model state"
    );
    expect(within(report).getByTestId("report-editor-intent-boundary").textContent).toContain(
      "no compliance or professional approval claim"
    );

    const exportHref = within(report).getByTestId("report-export-link").getAttribute("href") ?? "";
    const exportPacket = JSON.parse(decodeURIComponent(exportHref.split(",", 2)[1]));
    expect(exportPacket.editor_intent_refs).toContain(
      "op:editor-intent-material:invented-carbon-steel-elastic_modulus.value"
    );
    expect(exportPacket.editor_intent_summary.queued_count).toBe(1);
    expect(exportPacket.editor_intent_summary.application_statuses).toContain("not_applied");
    expect(exportPacket.editor_intent_summary.mutates_accepted_model_state).toBe(false);
    expect(exportPacket.editor_intent_summary.direct_model_mutation_allowed).toBe(false);
    expect(exportPacket.editor_intent_summary.release_or_professional_claim).toBe(false);
    expect(exportPacket.editor_operation_intents[0].queue_id).toBe("editor-intent-1");
    expect(exportPacket.editor_operation_intents[0].change.field_path).toBe("elastic_modulus.value");
    expect(exportPacket.editor_operation_intents[0].change.after).toBe("210000000000");
    expect(exportPacket.editor_operation_intents[0].validation.application_status).toBe("not_applied");
    expect(exportPacket.editor_operation_intents[0].audit_boundary.mutates_accepted_model_state).toBe(false);
    expect(exportPacket.editor_operation_intents[0].professional_boundary.software_makes_compliance_claim).toBe(false);
    expect(exportPacket.private_payload_included).toBe(false);
    expect(exportPacket.protected_content_included).toBe(false);

    const handoff = await screen.findByLabelText("Handoff package");
    expect((await within(handoff).findByTestId("handoff-review-context")).textContent).toContain("1 pending operation");
    expect(within(handoff).getByTestId("handoff-boundary").textContent).toContain("no release or professional claim");
    const handoffHref = within(handoff).getByTestId("handoff-export-link").getAttribute("href") ?? "";
    const handoffPacket = JSON.parse(decodeURIComponent(handoffHref.split(",", 2)[1]));
    expect(handoffPacket.document_kind).toBe("openpipestress.technical_preview.handoff_package");
    expect(handoffPacket.editor_intent_refs).toContain(
      "op:editor-intent-material:invented-carbon-steel-elastic_modulus.value"
    );
    expect(handoffPacket.editor_operation_statuses).toContain("not_applied");
    expect(handoffPacket.private_payload_included).toBe(false);
    expect(handoffPacket.protected_content_included).toBe(false);
    expect(handoffPacket.release_or_professional_claim).toBe(false);

    const diffPreview = await screen.findByLabelText("Operation diff preview");
    expect(await within(diffPreview).findByText(/state:project:invented-loop-01:preview/i)).toBeInTheDocument();
    expect(within(diffPreview).getByTestId("diff-preview-summary").textContent).toContain("1 operations");
    expect(within(diffPreview).getByTestId("diff-preview-summary").textContent).toContain("1 diff rows");
    expect(within(diffPreview).getByTestId("diff-preview-summary").textContent).toContain(
      "accepted_state_mutated=false"
    );
    expect(within(diffPreview).getByTestId("diff-preview-validation").textContent).toContain("0 hash-bound rows");
    expect(within(diffPreview).getByTestId("diff-preview-validation").textContent).toContain("1 held");
    expect(within(diffPreview).getByTestId("diff-preview-boundary").textContent).toContain(
      "accepted model mutated=false"
    );
    expect(within(diffPreview).getByTestId("diff-preview-boundary").textContent).toContain(
      "no release or professional claim"
    );
    expect(
      within(diffPreview).getByTestId("diff-preview-record-op-editor-intent-material-invented-carbon-steel-elastic-modulus-value")
        .textContent
    ).toContain("200000000000 to 210000000000 Pa");
    const diffHref = within(diffPreview).getByTestId("diff-preview-export-link").getAttribute("href") ?? "";
    const diffPacket = JSON.parse(decodeURIComponent(diffHref.split(",", 2)[1]));
    expect(diffPacket.document_kind).toBe("openpipestress.technical_preview.operation_diff_preview");
    expect(diffPacket.deliverable_refs).toContain("DEL-16-02");
    expect(diffPacket.scope_items).toContain("SOW-069");
    expect(diffPacket.summary.operation_count).toBe(1);
    expect(diffPacket.summary.diff_row_count).toBe(1);
    expect(diffPacket.summary.accepted_model_state_mutated).toBe(false);
    expect(diffPacket.summary.hash_bound_diff_preview_count).toBe(0);
    expect(diffPacket.summary.local_visual_diff_preview_count).toBe(1);
    expect(diffPacket.previews[0].record_source).toBe("gui_editor_intent_queue");
    expect(diffPacket.previews[0].operation_id).toBe(
      "op:editor-intent-material:invented-carbon-steel-elastic_modulus.value"
    );
    expect(diffPacket.previews[0].application_status).toBe("not_applied");
    expect(diffPacket.previews[0].accepted_model_state_mutated).toBe(false);
    expect(diffPacket.previews[0].hash_bound_diff_preview).toBe(false);
    expect(diffPacket.previews[0].changes[0].field_path).toBe("elastic_modulus.value");
    expect(diffPacket.previews[0].changes[0].before).toBe("200000000000");
    expect(diffPacket.previews[0].changes[0].after).toBe("210000000000");
    expect(diffPacket.private_payload_included).toBe(false);
    expect(diffPacket.protected_content_included).toBe(false);
    expect(diffPacket.release_or_professional_claim).toBe(false);
    expect(diffPacket.professional_boundary.software_makes_compliance_claim).toBe(false);

    const operationLedger = await screen.findByLabelText("Operation review ledger");
    expect(await within(operationLedger).findByText(/state:project:invented-loop-01:preview/i)).toBeInTheDocument();
    expect(within(operationLedger).getByTestId("operation-ledger-export-summary").textContent).toContain(
      "1 review record"
    );
    expect(within(operationLedger).getByTestId("operation-ledger-decision-counts").textContent).toContain(
      "1 held_for_user_acceptance"
    );
    expect(within(operationLedger).getByTestId("operation-ledger-latest").textContent).toContain(
      "op:editor-intent-material:invented-carbon-steel-elastic_modulus.value"
    );
    expect(within(operationLedger).getByTestId("operation-ledger-latest").textContent).toContain("not_applied");
    expect(within(operationLedger).getByTestId("operation-ledger-boundary").textContent).toContain(
      "does not mutate accepted model state"
    );
    const ledgerHref = within(operationLedger).getByTestId("operation-ledger-export-link").getAttribute("href") ?? "";
    const ledgerPacket = JSON.parse(decodeURIComponent(ledgerHref.split(",", 2)[1]));
    expect(ledgerPacket.document_kind).toBe("openpipestress.technical_preview.operation_review_ledger");
    expect(ledgerPacket.deliverable_refs).toContain("DEL-16-03");
    expect(ledgerPacket.scope_items).toContain("SOW-069");
    expect(ledgerPacket.scope_items).toContain("SOW-070");
    expect(ledgerPacket.decision_counts.held_for_user_acceptance).toBe(1);
    expect(ledgerPacket.decision_counts.accepted).toBe(0);
    expect(ledgerPacket.decision_counts.rejected).toBe(0);
    expect(ledgerPacket.records[0].record_source).toBe("gui_editor_intent_queue");
    expect(ledgerPacket.records[0].decision.status).toBe("held_for_user_acceptance");
    expect(ledgerPacket.records[0].decision.explicit_user_acceptance).toBe(false);
    expect(ledgerPacket.records[0].affected_entities[0].ref).toBe("material:invented-carbon-steel");
    expect(ledgerPacket.records[0].validation_outcome.application_status).toBe("not_applied");
    expect(ledgerPacket.records[0].diagnostics.map((item: { code: string }) => item.code)).toContain(
      "AUDIT-EXPLICIT-USER-ACCEPTANCE-REQUIRED"
    );
    expect(ledgerPacket.accepted_model_state_unchanged).toBe(true);
    expect(ledgerPacket.private_payload_included).toBe(false);
    expect(ledgerPacket.protected_content_included).toBe(false);
    expect(ledgerPacket.release_or_professional_claim).toBe(false);

    const exportReview = await screen.findByLabelText("Export safety review");
    expect(await within(exportReview).findByText(/run:preview-linear-static-001/i)).toBeInTheDocument();
    expect(within(exportReview).getByTestId("export-review-summary").textContent).toContain(
      "27 of 27 local exports ready"
    );
    expect(within(exportReview).getByTestId("export-review-summary").textContent).toContain(
      "no private/protected payloads"
    );
    expect(within(exportReview).getByTestId("export-review-boundary").textContent).toContain(
      "no release or professional claim"
    );
    expect(within(exportReview).getByTestId("export-review-record-project_storage_audit").textContent).toContain(
      "available"
    );
    expect(within(exportReview).getByTestId("export-review-record-project_validation_preflight").textContent).toContain(
      "available"
    );
    expect(within(exportReview).getByTestId("export-review-record-telemetry_boundary_review").textContent).toContain(
      "available"
    );
    expect(
      within(exportReview).getByTestId("export-review-record-secret_private_library_boundary_review").textContent
    ).toContain("available");
    expect(within(exportReview).getByTestId("export-review-record-security_threat_model_review").textContent).toContain(
      "available"
    );
    expect(within(exportReview).getByTestId("export-review-record-editor_contract_review").textContent).toContain(
      "available"
    );
    expect(
      within(exportReview).getByTestId("export-review-record-missing_data_warning_blocking_review").textContent
    ).toContain("available");
    expect(
      within(exportReview).getByTestId("export-review-record-accessibility_usability_baseline_review").textContent
    ).toContain("available");
    expect(
      within(exportReview).getByTestId("export-review-record-design_authoring_comparison_workspace").textContent
    ).toContain("available");
    expect(within(exportReview).getByTestId("export-review-record-build_package_readiness").textContent).toContain(
      "available"
    );
    expect(
      within(exportReview).getByTestId("export-review-record-validation_release_evidence_review").textContent
    ).toContain("available");
    expect(within(exportReview).getByTestId("export-review-record-result_envelope").textContent).toContain(
      "available"
    );
    expect(
      within(exportReview).getByTestId("export-review-record-stress_neutral_csv_json_package").textContent
    ).toContain("available");
    expect(within(exportReview).getByTestId("export-review-record-headless_runner_envelope").textContent).toContain(
      "available"
    );
    expect(within(exportReview).getByTestId("export-review-record-adapter_framework_envelope").textContent).toContain(
      "available"
    );
    expect(within(exportReview).getByTestId("export-review-record-local_fea_handoff_package").textContent).toContain(
      "available"
    );
    expect(
      within(exportReview).getByTestId("export-review-record-external_prover_boundary_metadata").textContent
    ).toContain("available");
    expect(within(exportReview).getByTestId("export-review-record-review_geometry_export").textContent).toContain(
      "available"
    );
    expect(within(exportReview).getByTestId("export-review-record-conservative_pcf_export").textContent).toContain(
      "available"
    );
    expect(within(exportReview).getByTestId("export-review-record-caepipe_mbf_export").textContent).toContain(
      "available"
    );
    expect(
      within(exportReview).getByTestId("export-review-record-caepipe_external_run_evidence").textContent
    ).toContain("available");
    expect(
      within(exportReview).getByTestId("export-review-record-export_adapter_sdk_registry").textContent
    ).toContain("available");
    expect(within(exportReview).getByTestId("export-review-record-native_json_package").textContent).toContain(
      "available"
    );
    expect(within(exportReview).getByTestId("export-review-record-report_packet").textContent).toContain(
      "available"
    );
    expect(within(exportReview).getByTestId("export-review-record-report_protected_content_lint").textContent).toContain(
      "available"
    );
    expect(within(exportReview).getByTestId("export-review-record-handoff_package").textContent).toContain(
      "available"
    );
    expect(within(exportReview).getByTestId("export-review-record-operation_review_ledger").textContent).toContain(
      "available"
    );
    const reviewHref = within(exportReview).getByTestId("export-review-link").getAttribute("href") ?? "";
    const reviewManifest = JSON.parse(decodeURIComponent(reviewHref.split(",", 2)[1]));
    expect(reviewManifest.document_kind).toBe("openpipestress.technical_preview.export_review_manifest");
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
    expect(reviewManifest.summary.export_count).toBe(27);
    expect(reviewManifest.summary.available_count).toBe(27);
    expect(reviewManifest.summary.operation_record_count).toBe(1);
    expect(reviewManifest.exports.map((item: { export_id: string }) => item.export_id)).toEqual([
      "project_storage_audit",
      "project_validation_preflight",
      "telemetry_boundary_review",
      "secret_private_library_boundary_review",
      "security_threat_model_review",
      "editor_contract_review",
      "missing_data_warning_blocking_review",
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
      "operation_review_ledger"
    ]);
    expect(
      reviewManifest.exports.find((item: { export_id: string }) => item.export_id === "project_storage_audit")
        .storage_mode
    ).toBe("not_persisted_this_session");
    expect(
      reviewManifest.exports.find((item: { export_id: string }) => item.export_id === "project_validation_preflight")
        .round_trip_status
    ).toBe("semantic_categories_declared");
    const telemetryExport = reviewManifest.exports.find(
      (item: { export_id: string }) => item.export_id === "telemetry_boundary_review"
    );
    expect(telemetryExport.document_kind).toBe("openpipestress.technical_preview.telemetry_boundary_review");
    expect(telemetryExport.deliverable_refs).toContain("DEL-12-03");
    expect(telemetryExport.config_resolution).toBe("absent_or_preview_config_resolves_disabled");
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
    expect(telemetryExport.security_certification_claim).toBe(false);
    const secretPrivateExport = reviewManifest.exports.find(
      (item: { export_id: string }) => item.export_id === "secret_private_library_boundary_review"
    );
    expect(secretPrivateExport.document_kind).toBe(
      "openpipestress.technical_preview.secret_private_library_boundary_review"
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
    expect(secretPrivateExport.security_certification_claim).toBe(false);
    const securityThreatModelExport = reviewManifest.exports.find(
      (item: { export_id: string }) => item.export_id === "security_threat_model_review"
    );
    expect(securityThreatModelExport.document_kind).toBe(
      "openpipestress.technical_preview.security_threat_model_review"
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
    expect(securityThreatModelExport.direct_sql_access).toBe(false);
    expect(securityThreatModelExport.security_certification_claim).toBe(false);
    const editorContractExport = reviewManifest.exports.find(
      (item: { export_id: string }) => item.export_id === "editor_contract_review"
    );
    expect(editorContractExport.document_kind).toBe("openpipestress.technical_preview.editor_contract_review");
    expect(editorContractExport.deliverable_refs).toContain("DEL-07-03");
    expect(editorContractExport.deliverable_refs).toContain("DEL-07-01");
    expect(editorContractExport.deliverable_refs).toContain("DEL-16-01");
    expect(editorContractExport.source_refs).toContain("apps/desktop/src/features/viewport/PipeViewport.tsx");
    expect(editorContractExport.editor_count).toBe(4);
    expect(editorContractExport.editor_surface_count).toBe(7);
    expect(editorContractExport.queued_intent_count).toBe(1);
    expect(editorContractExport.ready_editor_count).toBe(2);
    expect(editorContractExport.blocked_editor_count).toBe(2);
    expect(editorContractExport.diagnostic_count).toBe(4);
    expect(editorContractExport.rule_pack_reference_status).toBe(
      "private_reference_only_missing_required_inputs"
    );
    expect(editorContractExport.private_library_payload_status).toBe(
      "reference_slots_only_no_private_payload"
    );
    expect(editorContractExport.direct_model_mutation_allowed).toBe(false);
    expect(editorContractExport.accepted_model_state_mutated).toBe(false);
    const missingDataExport = reviewManifest.exports.find(
      (item: { export_id: string }) => item.export_id === "missing_data_warning_blocking_review"
    );
    expect(missingDataExport.document_kind).toBe(
      "openpipestress.technical_preview.missing_data_warning_blocking_review"
    );
    expect(missingDataExport.deliverable_refs).toContain("DEL-07-04");
    expect(missingDataExport.warning_class_count).toBe(6);
    expect(missingDataExport.active_warning_count).toBe(5);
    expect(missingDataExport.solve_blocking_count).toBe(0);
    expect(missingDataExport.rule_check_blocking_count).toBe(2);
    expect(missingDataExport.mechanics_solve_blocked).toBe(false);
    expect(missingDataExport.rule_check_blocked).toBe(true);
    expect(missingDataExport.mechanics_results_reviewable).toBe(true);
    expect(missingDataExport.mechanics_results_qualified_by_rule_inputs).toBe(true);
    expect(missingDataExport.silent_defaults_used).toBe(false);
    expect(missingDataExport.auto_fill_missing_data).toBe(false);
    expect(missingDataExport.assistive_text_fields_available).toBe(true);
    expect(missingDataExport.color_only_signaling_allowed).toBe(false);
    const accessibilityBaselineExport = reviewManifest.exports.find(
      (item: { export_id: string }) => item.export_id === "accessibility_usability_baseline_review"
    );
    expect(accessibilityBaselineExport.document_kind).toBe(
      "openpipestress.technical_preview.accessibility_usability_baseline_review"
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
    expect(accessibilityBaselineExport.accessibility_target_status).toBe("TBD_by_human_project_authority");
    expect(accessibilityBaselineExport.desktop_runtime_evaluation).toBe("not_performed");
    expect(accessibilityBaselineExport.software_makes_accessibility_conformance_claim).toBe(false);
    expect(accessibilityBaselineExport.color_only_status_signaling_allowed).toBe(false);
    const designWorkspaceExport = reviewManifest.exports.find(
      (item: { export_id: string }) => item.export_id === "design_authoring_comparison_workspace"
    );
    expect(designWorkspaceExport.document_kind).toBe(
      "openpipestress.technical_preview.design_authoring_comparison_workspace"
    );
    expect(designWorkspaceExport.deliverable_refs).toContain("DEL-07-08");
    expect(designWorkspaceExport.deliverable_refs).toContain("DEL-14-02");
    expect(designWorkspaceExport.deliverable_refs).toContain("DEL-16-04");
    expect(designWorkspaceExport.current_design_knowledge_record_count).toBe(3);
    expect(designWorkspaceExport.current_model_state_count).toBe(1);
    expect(designWorkspaceExport.current_analysis_run_count).toBe(1);
    expect(designWorkspaceExport.current_result_row_count).toBe(647);
    expect(designWorkspaceExport.current_comparison_pair_count).toBe(213);
    expect(designWorkspaceExport.operation_review_record_count).toBe(1);
    expect(designWorkspaceExport.core_design_knowledge_record_count).toBe(2);
    expect(designWorkspaceExport.core_constraint_warning_count).toBe(1);
    expect(designWorkspaceExport.core_model_state_count).toBe(2);
    expect(designWorkspaceExport.core_analysis_run_count).toBe(2);
    expect(designWorkspaceExport.core_graphical_overlay_count).toBe(5);
    expect(designWorkspaceExport.workspace_mutates_accepted_model_state).toBe(false);
    expect(designWorkspaceExport.accepted_operation_requires_explicit_user_acceptance_record).toBe(true);
    expect(designWorkspaceExport.private_payload_included).toBe(false);
    expect(designWorkspaceExport.protected_content_included).toBe(false);
    expect(designWorkspaceExport.release_or_professional_claim).toBe(false);
    const buildReadinessExport = reviewManifest.exports.find(
      (item: { export_id: string }) => item.export_id === "build_package_readiness"
    );
    expect(buildReadinessExport.document_kind).toBe("openpipestress.technical_preview.build_package_readiness");
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
    const validationEvidenceExport = reviewManifest.exports.find(
      (item: { export_id: string }) => item.export_id === "validation_release_evidence_review"
    );
    expect(validationEvidenceExport.document_kind).toBe(
      "openpipestress.technical_preview.validation_release_evidence_review"
    );
    expect(validationEvidenceExport.deliverable_refs).toContain("DEL-09-04");
    expect(validationEvidenceExport.deliverable_refs).toContain("DEL-09-05");
    expect(validationEvidenceExport.deliverable_refs).toContain("DEL-10-04");
    expect(validationEvidenceExport.manual_section_count).toBe(10);
    expect(validationEvidenceExport.evidence_area_count).toBe(8);
    expect(validationEvidenceExport.release_profile_count).toBe(5);
    expect(validationEvidenceExport.required_release_path_count).toBe(7);
    expect(validationEvidenceExport.skeleton_check_count).toBe(2);
    expect(validationEvidenceExport.validation_manual_open_decision_count).toBe(5);
    expect(validationEvidenceExport.release_gate_family_count).toBe(5);
    expect(validationEvidenceExport.release_authorization_status).toBe("not_authorized");
    expect(validationEvidenceExport.final_threshold_policy).toBe("TBD");
    expect(validationEvidenceExport.browser_panel_runs_tool).toBe(false);
    expect(validationEvidenceExport.dry_run_default).toBe(true);
    expect(validationEvidenceExport.private_payload_included).toBe(false);
    expect(validationEvidenceExport.protected_content_included).toBe(false);
    expect(validationEvidenceExport.release_or_professional_claim).toBe(false);
    const stressNeutralExport = reviewManifest.exports.find(
      (item: { export_id: string }) => item.export_id === "stress_neutral_csv_json_package"
    );
    expect(stressNeutralExport.document_kind).toBe(
      "openpipestress.technical_preview.stress_neutral_csv_json_package"
    );
    expect(stressNeutralExport.deliverable_refs).toContain("DEL-17-06");
    expect(stressNeutralExport.deliverable_refs).toContain("DEL-08-04");
    expect(stressNeutralExport.deliverable_refs).toContain("DEL-14-02");
    expect(stressNeutralExport.result_ref_count).toBe(647);
    expect(stressNeutralExport.csv_column_count).toBe(11);
    expect(stressNeutralExport.stable_id_count).toBe(647);
    expect(stressNeutralExport.member_roles).toContain("loss_report");
    expect(stressNeutralExport.comparison_semantics).toBe("diagnostic_export_only_no_pass_fail");
    expect(stressNeutralExport.canonical_package_hash_status).toBe("computed_local_preview_sha256_by_target_panel");
    expect(stressNeutralExport.vendor_format_claim).toBe(false);
    expect(stressNeutralExport.solver_validation_claim).toBe(false);
    expect(stressNeutralExport.code_compliance_claim).toBe(false);
    expect(stressNeutralExport.professional_reliance_claim).toBe(false);
    const headlessExport = reviewManifest.exports.find(
      (item: { export_id: string }) => item.export_id === "headless_runner_envelope"
    );
    expect(headlessExport.document_kind).toBe("openpipestress.technical_preview.headless_runner_envelope");
    expect(headlessExport.deliverable_refs).toContain("DEL-10-05");
    expect(headlessExport.deliverable_refs).toContain("DEL-08-04");
    expect(headlessExport.result_ref_count).toBe(647);
    expect(headlessExport.runner_job_state).toBe("COMPLETED");
    expect(headlessExport.final_cli_command_syntax).toBe("TBD");
    expect(headlessExport.process_invocation).toBe("TBD");
    expect(headlessExport.network_access).toBe("TBD");
    expect(headlessExport.filesystem_mutation_policy).toBe("TBD");
    const adapterExport = reviewManifest.exports.find(
      (item: { export_id: string }) => item.export_id === "adapter_framework_envelope"
    );
    expect(adapterExport.document_kind).toBe("openpipestress.technical_preview.adapter_framework_envelope");
    expect(adapterExport.deliverable_refs).toContain("DEL-10-02");
    expect(adapterExport.deliverable_refs).toContain("DEL-02-04");
    expect(adapterExport.capabilities).toContain("validate_payload");
    expect(adapterExport.parse_status).toBe("not_parsed_by_framework");
    expect(adapterExport.result_ref_count).toBe(647);
    expect(adapterExport.external_format_list).toBe("TBD");
    expect(adapterExport.public_transport_protocol).toBe("TBD");
    expect(adapterExport.plugin_runtime).toBe("TBD");
    const localFeaExport = reviewManifest.exports.find(
      (item: { export_id: string }) => item.export_id === "local_fea_handoff_package"
    );
    expect(localFeaExport.document_kind).toBe("openpipestress.technical_preview.local_fea_handoff_package");
    expect(localFeaExport.deliverable_refs).toContain("DEL-10-03");
    expect(localFeaExport.contract_kind).toBe("schema_first_local_fea_handoff_contract");
    expect(localFeaExport.global_analysis_role).toBe("primary_global_centerline_frame_model");
    expect(localFeaExport.local_analysis_role).toBe("optional_specialized_shell_solid_handoff");
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
      (item: { export_id: string }) => item.export_id === "external_prover_boundary_metadata"
    );
    expect(externalProverExport.document_kind).toBe(
      "openpipestress.technical_preview.external_prover_boundary_metadata"
    );
    expect(externalProverExport.deliverable_refs).toContain("DEL-15-04");
    expect(externalProverExport.metadata_contract_status).toBe("non_authoritative_workflow_metadata");
    expect(externalProverExport.external_reference_count).toBe(1);
    expect(externalProverExport.unsupported_target_flag_count).toBe(3);
    expect(externalProverExport.external_tool_invoked).toBe(false);
    expect(externalProverExport.commercial_result_payload_ingested).toBe(false);
    expect(externalProverExport.software_creates_external_validation_record).toBe(false);
    const reviewGeometryExport = reviewManifest.exports.find(
      (item: { export_id: string }) => item.export_id === "review_geometry_export"
    );
    expect(reviewGeometryExport.document_kind).toBe("openpipestress.technical_preview.review_geometry_export");
    expect(reviewGeometryExport.deliverable_refs).toContain("DEL-17-08");
    expect(reviewGeometryExport.review_geometry_status).toBe("visual_review_geometry_only");
    expect(reviewGeometryExport.geometry_format).toBe("glTF_2_0_json_preview");
    expect(reviewGeometryExport.node_count).toBe(5);
    expect(reviewGeometryExport.pipe_segment_count).toBe(4);
    expect(reviewGeometryExport.line_primitive_count).toBe(4);
    expect(reviewGeometryExport.stable_id_count).toBe(19);
    expect(reviewGeometryExport.sidecar_id_map_required).toBe(true);
    expect(reviewGeometryExport.glb_binary_writer_status).toBe("TBD");
    expect(reviewGeometryExport.viewer_compatibility).toBe("TBD");
    expect(reviewGeometryExport.solver_geometry_equivalence_claim).toBe(false);
    expect(reviewGeometryExport.professional_validation_claim).toBe(false);
    expect(reviewGeometryExport.target_compatibility_claim).toBe(false);
    const pcfExportRecord = reviewManifest.exports.find(
      (item: { export_id: string }) => item.export_id === "conservative_pcf_export"
    );
    expect(pcfExportRecord.document_kind).toBe(
      "openpipestress.technical_preview.conservative_pcf_export_package"
    );
    expect(pcfExportRecord.deliverable_refs).toContain("DEL-17-07");
    expect(pcfExportRecord.target_family).toBe("pcf");
    expect(pcfExportRecord.profile_id).toBe("ops.pcf.conservative_subset");
    expect(pcfExportRecord.target_profile_version_basis).toBe("TBD");
    expect(pcfExportRecord.pipe_segment_count).toBe(4);
    expect(pcfExportRecord.node_count).toBe(5);
    expect(pcfExportRecord.stable_id_count).toBe(4);
    expect(pcfExportRecord.loss_category_count).toBe(6);
    expect(pcfExportRecord.validation_status).toBe("blocked_missing_explicit_pcf_target_fields");
    expect(pcfExportRecord.canonical_package_hash_status).toBe("computed_local_preview_sha256_by_target_panel");
    expect(pcfExportRecord.target_compatibility_claim).toBe(false);
    expect(pcfExportRecord.solver_validation_claim).toBe(false);
    expect(pcfExportRecord.code_compliance_claim).toBe(false);
    expect(pcfExportRecord.professional_reliance_claim).toBe(false);
    const caepipeMbfExportRecord = reviewManifest.exports.find(
      (item: { export_id: string }) => item.export_id === "caepipe_mbf_export"
    );
    expect(caepipeMbfExportRecord.document_kind).toBe(
      "openpipestress.technical_preview.caepipe_mbf_export_package"
    );
    expect(caepipeMbfExportRecord.deliverable_refs).toContain("DEL-17-04");
    expect(caepipeMbfExportRecord.target_family).toBe("caepipe_mbf");
    expect(caepipeMbfExportRecord.profile_id).toBe("ops.caepipe_mbf.smoke_tbd");
    expect(caepipeMbfExportRecord.target_version_basis).toBe("TBD-17-01-001");
    expect(caepipeMbfExportRecord.record_subset_basis).toBe("TBD-17-01-002");
    expect(caepipeMbfExportRecord.pipe_element_count).toBe(4);
    expect(caepipeMbfExportRecord.node_count).toBe(5);
    expect(caepipeMbfExportRecord.support_count).toBe(3);
    expect(caepipeMbfExportRecord.load_case_count).toBe(2);
    expect(caepipeMbfExportRecord.stable_id_count).toBe(14);
    expect(caepipeMbfExportRecord.loss_category_count).toBe(6);
    expect(caepipeMbfExportRecord.validation_status).toBe("boundary_checked");
    expect(caepipeMbfExportRecord.external_execution_policy).toBe("not_invoked_by_this_package");
    expect(caepipeMbfExportRecord.caepipe_compatibility_claim).toBe(false);
    expect(caepipeMbfExportRecord.solver_validation_claim).toBe(false);
    expect(caepipeMbfExportRecord.code_compliance_claim).toBe(false);
    expect(caepipeMbfExportRecord.professional_reliance_claim).toBe(false);
    const caepipeExternalRecord = reviewManifest.exports.find(
      (item: { export_id: string }) => item.export_id === "caepipe_external_run_evidence"
    );
    expect(caepipeExternalRecord.document_kind).toBe(
      "openpipestress.technical_preview.caepipe_external_run_package"
    );
    expect(caepipeExternalRecord.deliverable_refs).toContain("DEL-17-05");
    expect(caepipeExternalRecord.deliverable_refs).toContain("DEL-17-04");
    expect(caepipeExternalRecord.package_status).toBe("parser_only_evidence");
    expect(caepipeExternalRecord.mbf_package_ref).toContain("del-17-04");
    expect(caepipeExternalRecord.executable_path_state).toBe("absent");
    expect(caepipeExternalRecord.invocation_profile).toBe("TBD-17-05-invocation-profile");
    expect(caepipeExternalRecord.external_tool_invoked).toBe(false);
    expect(caepipeExternalRecord.parser_status).toBe("parsed_parser_only_fixture");
    expect(caepipeExternalRecord.parser_row_count).toBe(3);
    expect(caepipeExternalRecord.parser_section_count).toBe(2);
    expect(caepipeExternalRecord.parser_correlation_status).toBe("canonical_id_map");
    expect(caepipeExternalRecord.validation_status).toBe("boundary_checked");
    expect(caepipeExternalRecord.caepipe_compatibility_claim).toBe(false);
    expect(caepipeExternalRecord.solver_validation_claim).toBe(false);
    expect(caepipeExternalRecord.code_compliance_claim).toBe(false);
    expect(caepipeExternalRecord.professional_reliance_claim).toBe(false);
    const exportAdapterSdkRecord = reviewManifest.exports.find(
      (item: { export_id: string }) => item.export_id === "export_adapter_sdk_registry"
    );
    expect(exportAdapterSdkRecord.document_kind).toBe(
      "openpipestress.technical_preview.export_adapter_sdk_registry"
    );
    expect(exportAdapterSdkRecord.deliverable_refs).toContain("DEL-17-09");
    expect(exportAdapterSdkRecord.deliverable_refs).toContain("DEL-17-02");
    expect(exportAdapterSdkRecord.deliverable_refs).toContain("DEL-02-04");
    expect(exportAdapterSdkRecord.deliverable_refs).toContain("DEL-10-01");
    expect(exportAdapterSdkRecord.deliverable_refs).toContain("DEL-10-02");
    expect(exportAdapterSdkRecord.registry_id).toBe("ops.export_adapter_sdk.registry_preview");
    expect(exportAdapterSdkRecord.sdk_contract_status).toBe("contract_level_preview");
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
      (item: { export_id: string }) => item.export_id === "report_protected_content_lint"
    );
    expect(lintExport.document_kind).toBe("openpipestress.technical_preview.report_protected_content_lint_run");
    expect(lintExport.deliverable_refs).toContain("DEL-08-05");
    expect(lintExport.target_count).toBe(22);
    expect(lintExport.finding_count).toBe(0);
    expect(lintExport.blocking_finding_count).toBe(0);
    expect(lintExport.clean_scan_is_clearance).toBe(false);
    expect(
      reviewManifest.exports.every(
        (item: {
          private_payload_included: boolean;
          protected_content_included: boolean;
          release_or_professional_claim: boolean;
        }) =>
          item.private_payload_included === false &&
          item.protected_content_included === false &&
          item.release_or_professional_claim === false
      )
    ).toBe(true);
    expect(reviewManifest.redaction_policy.local_private_export_allowed).toBe(true);
    expect(reviewManifest.redaction_policy.protected_content_blocked).toBe(true);
    expect(reviewManifest.redaction_policy.source_model_mutated).toBe(false);
    expect(reviewManifest.data_boundary.private_data_policy).toBe("no_private_project_data");
    expect(reviewManifest.professional_boundary.software_makes_compliance_claim).toBe(false);
  }, 10000);

  it("round trips local create, save, and open project controls without external file copies", async () => {
    render(<App />);

    expect(await screen.findByText("OpenPipeStress Technical Preview")).toBeInTheDocument();
    const controls = screen.getByLabelText("Local project controls");
    const storageAudit = await screen.findByLabelText("Project storage audit");
    const projectValidation = await screen.findByLabelText("Project validation preflight");
    expect(screen.getByTestId("local-project-review-context").textContent).toContain(
      "0 pending operations; applied=false"
    );
    expect(within(storageAudit).getByTestId("model-hash-persistence").textContent).toContain(
      "persisted_model_hashes=0"
    );
    expect(within(storageAudit).getByTestId("model-hash-persistence").textContent).toContain(
      "persisted_model_hash_ref=not_persisted"
    );
    expect(within(storageAudit).getByTestId("model-hash-integrity").textContent).toContain(
      "no open-verification has run this session"
    );

    const tree = screen.getByLabelText("Model tree");
    fireEvent.click(within(tree).getByRole("button", { name: /Invented carbon-steel-like material/i }));
    const inspector = screen.getByLabelText("Property inspector");
    const intentPanel = within(inspector).getByLabelText("Editor operation intent");
    fireEvent.change(within(intentPanel).getByTestId("editor-intent-field"), {
      target: { value: "elastic_modulus.value" }
    });
    fireEvent.change(within(intentPanel).getByTestId("editor-intent-value"), {
      target: { value: "210000000000" }
    });
    fireEvent.click(within(intentPanel).getByTestId("queue-editor-intent"));
    expect(screen.getByTestId("local-project-review-context").textContent).toContain(
      "1 pending operation; applied=false"
    );

    fireEvent.click(within(controls).getByRole("button", { name: /Create local/i }));
    expect(await screen.findByTestId("local-project-message")).toHaveTextContent("without external file copies");
    expect(screen.getByTestId("local-project-review-context").textContent).toContain(
      "1 pending operation; applied=false"
    );
    expect(within(storageAudit).getByTestId("project-storage-summary").textContent).toContain("operation=create");
    expect(within(storageAudit).getByTestId("project-storage-summary").textContent).toContain(
      "pending operations=1"
    );
    expect(within(storageAudit).getByTestId("project-storage-snapshot").textContent).toContain(
      "browser_memory_preview"
    );
    expect(within(storageAudit).getByTestId("project-storage-snapshot").textContent).toContain(
      "persisted_editor_intents=1"
    );

    fireEvent.click(within(controls).getByRole("button", { name: /Save local/i }));
    expect(await screen.findByTestId("local-project-message")).toHaveTextContent("without external file copies");
    expect(screen.getByTestId("local-project-review-context").textContent).toContain(
      "1 pending operation; applied=false"
    );
    expect(within(storageAudit).getByTestId("project-storage-summary").textContent).toContain("operation=save");

    fireEvent.click(within(controls).getByRole("button", { name: /Open local/i }));
    expect(await screen.findByTestId("local-project-message")).toHaveTextContent(
      "Opened local browser-preview project snapshot."
    );
    expect(screen.getByTestId("local-project-review-context").textContent).toContain(
      "1 pending operation; applied=false"
    );
    expect(within(controls).getByText("Invented Utility Loop Preview")).toBeInTheDocument();
    expect(within(storageAudit).getByTestId("project-storage-summary").textContent).toContain("operation=open");
    expect(within(storageAudit).getByTestId("project-storage-snapshot").textContent).toContain(
      "persisted_editor_intents=1"
    );
    expect(within(storageAudit).getByTestId("project-storage-local-boundary").textContent).toContain(
      "network=false"
    );
    expect(within(storageAudit).getByTestId("project-storage-local-boundary").textContent).toContain(
      "telemetry=false"
    );
    expect(within(storageAudit).getByTestId("project-storage-snapshot").textContent).toContain(
      "copied_external_files=false"
    );
    expect(within(storageAudit).getByTestId("project-storage-payload-boundary").textContent).toContain(
      "protected content=false"
    );
    await waitFor(() =>
      expect(within(storageAudit).getByTestId("model-hash-integrity").textContent).toContain(
        "integrity_status=verified_match"
      )
    );
    expect(within(storageAudit).getByTestId("model-hash-integrity").textContent).toContain(
      "persisted_value=sha256:"
    );
    expect(within(storageAudit).getByTestId("model-hash-integrity").textContent).toContain(
      "recomputed_value=sha256:"
    );
    expect(within(storageAudit).getByTestId("model-hash-integrity").textContent).toContain(
      "verification_basis=recomputed_on_open_from_restored_model"
    );
    expect(within(storageAudit).getByTestId("model-hash-persistence").textContent).toContain(
      "persisted_model_hashes=1"
    );
    expect(within(storageAudit).getByTestId("model-hash-persistence").textContent).toContain(
      "persisted_model_hash_ref=sha256:"
    );
    const auditHref = within(storageAudit).getByTestId("project-storage-export-link").getAttribute("href") ?? "";
    const auditPacket = JSON.parse(decodeURIComponent(auditHref.split(",", 2)[1]));
    expect(auditPacket.document_kind).toBe("openpipestress.technical_preview.local_project_persistence_audit");
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
    expect(auditPacket.summary.applied_operation_count).toBe(0);
    expect(auditPacket.summary.accepted_model_state_mutated).toBe(false);
    expect(auditPacket.summary.network_required).toBe(false);
    expect(auditPacket.summary.daemon_required).toBe(false);
    expect(auditPacket.summary.telemetry_enabled).toBe(false);
    expect(auditPacket.summary.copied_external_files).toBe(false);
    expect(auditPacket.project_summary.storage_mode).toBe("browser_memory_preview");
    expect(auditPacket.project_summary.editor_intent_count).toBe(1);
    expect(auditPacket.project_summary.copied_external_files).toBe(false);
    expect(auditPacket.editor_intent_refs).toContain(
      "op:editor-intent-material:invented-carbon-steel-elastic_modulus.value"
    );
    expect(auditPacket.editor_operation_statuses).toContain("not_applied");
    expect(auditPacket.boundary.local_only_project_store).toBe(true);
    expect(auditPacket.boundary.repository_default_private_write).toBe(false);
    expect(auditPacket.boundary.external_file_copy_performed).toBe(false);
    expect(auditPacket.boundary.accepted_model_state_mutated).toBe(false);
    expect(auditPacket.private_payload_included).toBe(false);
    expect(auditPacket.protected_content_included).toBe(false);
    expect(auditPacket.release_or_professional_claim).toBe(false);
    expect(auditPacket.professional_boundary.software_makes_compliance_claim).toBe(false);
    expect(within(projectValidation).getByTestId("project-validation-summary").textContent).toContain(
      "validation=preview_current"
    );
    expect(within(projectValidation).getByTestId("project-validation-summary").textContent).toContain(
      "migration=browser_memory_snapshot_no_sql_store_migrations_applicable"
    );
    expect(within(projectValidation).getByTestId("project-validation-store-migration").textContent).toContain(
      "framework=browser_memory_preview_no_sqlite_migration_ledger"
    );
    expect(within(projectValidation).getByTestId("project-validation-store-migration").textContent).toContain(
      "applied_on_open=0"
    );
    expect(within(projectValidation).getByTestId("project-validation-store-migration").textContent).toContain(
      "status=browser_memory_snapshot_no_sql_store_migrations_applicable"
    );
    expect(within(projectValidation).getByTestId("project-validation-operations").textContent).toContain(
      "pending operations=1"
    );
    expect(within(projectValidation).getByTestId("project-validation-operations").textContent).toContain(
      "persisted editor intents=1"
    );
    expect(within(projectValidation).getByTestId("project-validation-model-hash").textContent).toContain(
      "model_hash=model_hash_verified_on_open"
    );
    expect(within(projectValidation).getByTestId("project-validation-model-hash").textContent).toContain(
      "persisted_model_hashes=1"
    );
    expect(within(projectValidation).getByTestId("project-validation-model-hash").textContent).toContain(
      "persisted_model_hash_ref=sha256:"
    );
    expect(within(projectValidation).getByTestId("project-validation-model-hash").textContent).toContain(
      "integrity=verified_match"
    );
    expect(within(projectValidation).getByTestId("project-validation-round-trip").textContent).toContain(
      "reproducibility=model_hash_verified_on_open"
    );
    await waitFor(() =>
      expect(within(projectValidation).getByTestId("project-validation-envelope-hash").textContent).toContain(
        "envelope_hash=project_envelope_hash_verified_on_open"
      )
    );
    expect(within(projectValidation).getByTestId("project-validation-envelope-hash").textContent).toContain(
      "persisted_envelope_hashes=1"
    );
    expect(within(projectValidation).getByTestId("project-validation-envelope-hash").textContent).toContain(
      "persisted_envelope_hash_ref=sha256:"
    );
    expect(within(projectValidation).getByTestId("project-validation-envelope-hash").textContent).toContain(
      "integrity=verified_match"
    );
    expect(within(projectValidation).getByTestId("project-validation-boundary").textContent).toContain(
      "private/protected payload=false"
    );
    const validationHref =
      within(projectValidation).getByTestId("project-validation-export-link").getAttribute("href") ?? "";
    const validationPacket = JSON.parse(decodeURIComponent(validationHref.split(",", 2)[1]));
    expect(validationPacket.document_kind).toBe("openpipestress.technical_preview.project_validation_preflight");
    expect(validationPacket.project_ref).toBe("project:invented-loop-01");
    expect(validationPacket.summary.validation_status).toBe("preview_current");
    expect(validationPacket.summary.last_operation).toBe("open");
    expect(validationPacket.summary.storage_mode).toBe("browser_memory_preview");
    expect(validationPacket.summary.migration_status).toBe("browser_memory_snapshot_no_sql_store_migrations_applicable");
    expect(validationPacket.store_migration.migration_framework).toBe("browser_memory_preview_no_sqlite_migration_ledger");
    expect(validationPacket.store_migration.evidence_source).toBe("local_project_summary");
    expect(validationPacket.store_migration.migration_scope).toBe("local_store_schema_only_not_model_document_schema");
    expect(validationPacket.store_migration.model_document_migration_status).toBe(
      "model_document_migrations_not_defined_tbd"
    );
    expect(validationPacket.store_migration.migrations_applied_on_open).toEqual([]);
    expect(validationPacket.store_migration.destructive_migration_performed).toBe(false);
    expect(validationPacket.project_summary.store_schema_version).toBe(0);
    expect(validationPacket.project_summary.store_schema_target_version).toBe(0);
    expect(validationPacket.summary.pending_operation_count).toBe(1);
    expect(validationPacket.summary.editor_intent_count).toBe(1);
    expect(validationPacket.summary.persisted_editor_intent_count).toBe(1);
    expect(validationPacket.project_summary.editor_intent_count).toBe(1);
    expect(validationPacket.summary.accepted_model_state_mutated).toBe(false);
    expect(validationPacket.service_operations.find((operation: { operation: string }) => operation.operation === "validate").operation_status).toBe(
      "preflight_generated_preview_current"
    );
    expect(
      validationPacket.service_operations.find((operation: { operation: string }) => operation.operation === "version_check")
        .operation_status
    ).toBe("supported_current_schema");
    expect(validationPacket.service_operations.find((operation: { operation: string }) => operation.operation === "migrate").operation_status).toBe(
      "browser_memory_snapshot_no_sql_store_migrations_applicable"
    );
    expect(
      validationPacket.service_operations.find((operation: { operation: string }) => operation.operation === "migrate")
        .result_available
    ).toBe(true);
    expect(
      validationPacket.diagnostics.map((diagnosticItem: { code: string }) => diagnosticItem.code)
    ).toContain("PROJECT-VALIDATION-STORE-MIGRATION-LEDGER-REVIEW-ONLY");
    expect(
      validationPacket.diagnostics.map((diagnosticItem: { code: string }) => diagnosticItem.code)
    ).not.toContain("PROJECT-VALIDATION-STORE-MIGRATED-ON-OPEN");
    expect(
      validationPacket.diagnostics.find(
        (diagnosticItem: { code: string }) => diagnosticItem.code === "PROJECT-VALIDATION-STORE-MIGRATION-LEDGER-REVIEW-ONLY"
      ).class
    ).toBe("MIGRATION");
    expect(validationPacket.summary.model_hash_status).toBe("model_hash_verified_on_open");
    expect(validationPacket.summary.persisted_model_hash_count).toBe(1);
    expect(validationPacket.summary.persisted_model_hash_ref).toMatch(/^sha256:[0-9a-f]{64}$/);
    expect(validationPacket.summary.model_hash_integrity_status).toBe("verified_match");
    expect(validationPacket.validation_profile.hash_service_status).toBe(
      "canonical_model_hash_service_available_model_payload_scope"
    );
    expect(validationPacket.model_hash_integrity.integrity_status).toBe("verified_match");
    expect(validationPacket.model_hash_integrity.verification_basis).toBe("recomputed_on_open_from_restored_model");
    expect(validationPacket.model_hash_integrity.persisted_value).toBe(validationPacket.model_hash_integrity.recomputed_value);
    expect(validationPacket.summary.project_envelope_hash_status).toBe("project_envelope_hash_verified_on_open");
    expect(validationPacket.summary.persisted_project_envelope_hash_count).toBe(1);
    expect(validationPacket.summary.persisted_project_envelope_hash_ref).toMatch(/^sha256:[0-9a-f]{64}$/);
    expect(validationPacket.summary.project_envelope_hash_integrity_status).toBe("verified_match");
    expect(validationPacket.validation_profile.project_envelope_hash_status).toBe(
      "project_envelope_hash_verified_on_open"
    );
    expect(validationPacket.project_envelope_hash.payload_scope).toBe("project_envelope_payload");
    expect(validationPacket.project_envelope_hash.payload_excludes).toBe(
      "storage_summary_and_envelope_hash_carrier_fields"
    );
    expect(validationPacket.project_envelope_hash.value).toMatch(/^sha256:[0-9a-f]{64}$/);
    expect(validationPacket.project_envelope_hash_integrity.integrity_status).toBe("verified_match");
    expect(validationPacket.project_envelope_hash_integrity.verification_basis).toBe(
      "recomputed_on_open_from_restored_envelope_payload"
    );
    expect(validationPacket.project_envelope_hash_integrity.persisted_value).toBe(
      validationPacket.project_envelope_hash_integrity.recomputed_value
    );
    expect(validationPacket.project_summary.persisted_project_envelope_hash_count).toBe(1);
    expect(
      validationPacket.diagnostics.map((diagnosticEntry: { code: string }) => diagnosticEntry.code)
    ).toContain("PROJECT-VALIDATION-ENVELOPE-HASH-REVIEW-ONLY");
    expect(
      validationPacket.diagnostics.map((diagnosticEntry: { code: string }) => diagnosticEntry.code)
    ).not.toContain("PROJECT-VALIDATION-ENVELOPE-HASH-MISMATCH");
    expect(
      validationPacket.diagnostics.find(
        (diagnosticEntry: { code: string }) => diagnosticEntry.code === "PROJECT-VALIDATION-ENVELOPE-HASH-REVIEW-ONLY"
      ).class
    ).toBe("REPRODUCIBILITY");
    expect(
      validationPacket.round_trip_manifest.categories.find(
        (category: { category: string }) => category.category === "reproducibility_metadata"
      ).semantic_equality_status
    ).toBe("model_hash_verified_on_open");
    expect(
      validationPacket.diagnostics.map((diagnosticEntry: { code: string }) => diagnosticEntry.code)
    ).not.toContain("PROJECT-VALIDATION-MODEL-HASH-MISMATCH");
    expect(
      validationPacket.round_trip_manifest.categories.find(
        (category: { category: string }) => category.category === "unit_metadata"
      ).semantic_equality_status
    ).toBe("ready_for_preview_round_trip");
    expect(
      validationPacket.round_trip_manifest.categories.find(
        (category: { category: string }) => category.category === "rule_pack_references"
      ).semantic_equality_status
    ).toBe("not_present_in_invented_fixture");
    expect(validationPacket.boundary.local_only_project_store).toBe(true);
    expect(validationPacket.boundary.repository_default_private_write).toBe(false);
    expect(validationPacket.boundary.external_file_copy_performed).toBe(false);
    expect(validationPacket.boundary.accepted_model_state_mutated).toBe(false);
    expect(validationPacket.private_payload_included).toBe(false);
    expect(validationPacket.protected_content_included).toBe(false);
    expect(validationPacket.release_or_professional_claim).toBe(false);
    expect(validationPacket.professional_boundary.software_makes_compliance_claim).toBe(false);

    const exportReview = await screen.findByLabelText("Export safety review");
    const reviewHref = within(exportReview).getByTestId("export-review-link").getAttribute("href") ?? "";
    const reviewManifest = JSON.parse(decodeURIComponent(reviewHref.split(",", 2)[1]));
    expect(
      reviewManifest.exports.find((item: { export_id: string }) => item.export_id === "project_storage_audit")
        .persisted_editor_intent_count
    ).toBe(1);
    expect(
      reviewManifest.exports.find(
        (item: { export_id: string }) => item.export_id === "project_validation_preflight"
      ).persisted_editor_intent_count
    ).toBe(1);

    fireEvent.click(within(tree).getByRole("button", { name: /Invented carbon-steel-like material/i }));
    expect(
      within(screen.getByLabelText("Editor operation intent")).getByTestId("editor-intent-queue").textContent
    ).toContain("editor-intent-1");
  });

  it("round trips review-only proposal operations through local save and open", async () => {
    render(<App />);

    const runButton = await screen.findByRole("button", { name: /Run mechanics preview/i });
    fireEvent.click(runButton);

    expect(await screen.findByTestId("result-group-displacement", {}, { timeout: 10000 })).toBeInTheDocument();
    const results = await screen.findByLabelText("Results");
    fireEvent.change(within(results).getByTestId("result-filter-input"), {
      target: { value: "torsional-shear" }
    });
    fireEvent.click(within(results).getByTestId("result-row-result:stress:pipe-P-120:end-j:torsional-shear"));

    fireEvent.click(screen.getByRole("button", { name: /Generate review proposal/i }));
    const proposalPanel = await screen.findByLabelText("Agentic proposal");
    expect(await within(proposalPanel).findByText("proposal:physics-diagnostic-review")).toBeInTheDocument();
    expect(within(proposalPanel).getByTestId("proposal-operation-summary").textContent).toContain(
      "op:review-computed-diagnostic"
    );
    expect(within(proposalPanel).getByTestId("selected-review-target").textContent).toContain(
      "result: result:stress:pipe-P-120:end-j:torsional-shear"
    );
    expect(screen.getByTestId("local-project-review-context").textContent).toContain(
      "1 pending operation; applied=false; editor_intents=0; agent_proposals=1"
    );

    const controls = screen.getByLabelText("Local project controls");
    fireEvent.click(within(controls).getByRole("button", { name: /Save local/i }));
    await waitFor(() =>
      expect(screen.getByTestId("local-project-message")).toHaveTextContent(
        "Saved local browser-preview project snapshot without external file copies."
      )
    );

    const storageAudit = await screen.findByLabelText("Project storage audit");
    expect(within(storageAudit).getByTestId("project-storage-summary").textContent).toContain("operation=save");
    expect(within(storageAudit).getByTestId("project-storage-summary").textContent).toContain(
      "pending operations=1"
    );
    expect(within(storageAudit).getByTestId("project-storage-snapshot").textContent).toContain(
      "persisted_proposals=1"
    );
    expect(within(storageAudit).getByTestId("project-storage-snapshot").textContent).toContain(
      "persisted_review_targets=1"
    );
    expect(within(storageAudit).getByTestId("project-storage-snapshot").textContent).toContain(
      "persisted_review_target_ref=result: result:stress:pipe-P-120:end-j:torsional-shear"
    );
    expect(within(storageAudit).getByTestId("project-storage-snapshot").textContent).toContain(
      "persisted_mechanics_results=1"
    );
    expect(within(storageAudit).getByTestId("project-storage-snapshot").textContent).toContain(
      "persisted_analysis_runs=1"
    );
    expect(within(storageAudit).getByTestId("project-storage-snapshot").textContent).toContain(
      "persisted_analysis_run_ref=run:preview-linear-static-001"
    );
    const savedStorageHref =
      within(storageAudit).getByTestId("project-storage-export-link").getAttribute("href") ?? "";
    const savedStoragePacket = JSON.parse(decodeURIComponent(savedStorageHref.split(",", 2)[1]));
    expect(savedStoragePacket.summary.pending_operation_count).toBe(1);
    expect(savedStoragePacket.summary.proposal_operation_count).toBe(1);
    expect(savedStoragePacket.summary.persisted_proposal_count).toBe(1);
    expect(savedStoragePacket.summary.persisted_selected_review_target_count).toBe(1);
    expect(savedStoragePacket.summary.persisted_selected_review_target_ref).toBe(
      "result: result:stress:pipe-P-120:end-j:torsional-shear"
    );
    expect(savedStoragePacket.project_summary.proposal_count).toBe(1);
    expect(savedStoragePacket.project_summary.selected_review_target_count).toBe(1);
    expect(savedStoragePacket.project_summary.selected_review_target_ref).toBe(
      "result: result:stress:pipe-P-120:end-j:torsional-shear"
    );
    expect(savedStoragePacket.summary.persisted_mechanics_result_count).toBe(1);
    expect(savedStoragePacket.summary.persisted_analysis_run_count).toBe(1);
    expect(savedStoragePacket.summary.persisted_analysis_run_ref).toBe("run:preview-linear-static-001");
    expect(savedStoragePacket.project_summary.persisted_mechanics_result_count).toBe(1);
    expect(savedStoragePacket.project_summary.persisted_analysis_run_count).toBe(1);
    expect(savedStoragePacket.project_summary.persisted_analysis_run_ref).toBe("run:preview-linear-static-001");
    expect(savedStoragePacket.proposal_refs).toContain("proposal:physics-diagnostic-review");
    expect(savedStoragePacket.summary.project_index_state).toBe("not_requested");
    expect(savedStoragePacket.summary.listed_project_count).toBe(0);

    fireEvent.click(within(controls).getByRole("button", { name: /List local/i }));
    await waitFor(() =>
      expect(screen.getByTestId("local-project-message")).toHaveTextContent(
        "Listed 1 local project snapshot from the local store index."
      )
    );
    expect(within(storageAudit).getByTestId("project-storage-project-index").textContent).toContain("state=listed");
    expect(within(storageAudit).getByTestId("project-storage-project-index").textContent).toContain(
      "listed_projects=1"
    );
    expect(within(storageAudit).getByTestId("project-storage-project-index").textContent).toContain(
      "refs=project:invented-loop-01"
    );
    const listedStorageHref =
      within(storageAudit).getByTestId("project-storage-export-link").getAttribute("href") ?? "";
    const listedStoragePacket = JSON.parse(decodeURIComponent(listedStorageHref.split(",", 2)[1]));
    expect(listedStoragePacket.summary.last_operation).toBe("list");
    expect(listedStoragePacket.summary.project_index_state).toBe("listed");
    expect(listedStoragePacket.summary.listed_project_count).toBe(1);
    expect(listedStoragePacket.project_index_refs).toEqual(["project:invented-loop-01"]);
    expect(listedStoragePacket.project_index).toHaveLength(1);
    expect(listedStoragePacket.project_index[0].project_id).toBe("project:invented-loop-01");
    expect(listedStoragePacket.project_index[0].project_name).toBe("Invented Utility Loop Preview");
    expect(listedStoragePacket.project_index[0].storage_mode).toBe("browser_memory_preview");
    expect(listedStoragePacket.project_index[0].created_at_unix).toBeGreaterThan(0);
    expect(listedStoragePacket.project_index[0].updated_at_unix).toBeGreaterThanOrEqual(
      listedStoragePacket.project_index[0].created_at_unix
    );

    const projectValidation = await screen.findByLabelText("Project validation preflight");
    expect(within(projectValidation).getByTestId("project-validation-operations").textContent).toContain(
      "persisted proposals=1"
    );
    expect(within(projectValidation).getByTestId("project-validation-operations").textContent).toContain(
      "persisted review targets=1"
    );
    expect(within(projectValidation).getByTestId("project-validation-operations").textContent).toContain(
      "persisted review target ref=result: result:stress:pipe-P-120:end-j:torsional-shear"
    );
    expect(within(projectValidation).getByTestId("project-validation-operations").textContent).toContain(
      "persisted mechanics results=1"
    );
    expect(within(projectValidation).getByTestId("project-validation-operations").textContent).toContain(
      "persisted analysis runs=1"
    );
    expect(within(projectValidation).getByTestId("project-validation-operations").textContent).toContain(
      "persisted analysis run ref=run:preview-linear-static-001"
    );
    const savedValidationHref =
      within(projectValidation).getByTestId("project-validation-export-link").getAttribute("href") ?? "";
    const savedValidationPacket = JSON.parse(decodeURIComponent(savedValidationHref.split(",", 2)[1]));
    expect(savedValidationPacket.summary.pending_operation_count).toBe(1);
    expect(savedValidationPacket.summary.proposal_operation_count).toBe(1);
    expect(savedValidationPacket.summary.persisted_proposal_count).toBe(1);
    expect(savedValidationPacket.summary.persisted_selected_review_target_count).toBe(1);
    expect(savedValidationPacket.summary.persisted_selected_review_target_ref).toBe(
      "result: result:stress:pipe-P-120:end-j:torsional-shear"
    );
    expect(savedValidationPacket.project_summary.proposal_count).toBe(1);
    expect(savedValidationPacket.project_summary.selected_review_target_count).toBe(1);
    expect(savedValidationPacket.project_summary.selected_review_target_ref).toBe(
      "result: result:stress:pipe-P-120:end-j:torsional-shear"
    );
    expect(savedValidationPacket.summary.persisted_mechanics_result_count).toBe(1);
    expect(savedValidationPacket.summary.persisted_analysis_run_count).toBe(1);
    expect(savedValidationPacket.summary.persisted_analysis_run_ref).toBe("run:preview-linear-static-001");
    expect(savedValidationPacket.proposal_refs).toContain("proposal:physics-diagnostic-review");

    const report = await screen.findByLabelText("Report packet");
    const savedReportHref = within(report).getByTestId("report-export-link").getAttribute("href") ?? "";
    const savedReportPacket = JSON.parse(decodeURIComponent(savedReportHref.split(",", 2)[1]));
    expect(savedReportPacket.persistence_evidence.storage_audit.persisted_selected_review_target_ref).toBe(
      "result: result:stress:pipe-P-120:end-j:torsional-shear"
    );
    expect(savedReportPacket.persistence_evidence.validation_preflight.persisted_selected_review_target_ref).toBe(
      "result: result:stress:pipe-P-120:end-j:torsional-shear"
    );
    expect(savedReportPacket.persistence_evidence.storage_audit.persisted_mechanics_result_count).toBe(1);
    expect(savedReportPacket.persistence_evidence.storage_audit.persisted_analysis_run_count).toBe(1);
    expect(savedReportPacket.persistence_evidence.storage_audit.persisted_analysis_run_ref).toBe(
      "run:preview-linear-static-001"
    );
    expect(savedReportPacket.persistence_evidence.validation_preflight.persisted_mechanics_result_count).toBe(1);
    expect(savedReportPacket.persistence_evidence.validation_preflight.persisted_analysis_run_count).toBe(1);
    expect(savedReportPacket.persistence_evidence.validation_preflight.persisted_analysis_run_ref).toBe(
      "run:preview-linear-static-001"
    );

    const savedNativePackage = await screen.findByLabelText("Native JSON package");
    expect(within(savedNativePackage).getByTestId("native-package-persisted-review-context").textContent).toContain(
      "editor_intents=0"
    );
    expect(within(savedNativePackage).getByTestId("native-package-persisted-review-context").textContent).toContain(
      "proposals=1"
    );
    expect(within(savedNativePackage).getByTestId("native-package-persisted-review-context").textContent).toContain(
      "selected_targets=1"
    );
    expect(within(savedNativePackage).getByTestId("native-package-persisted-review-context").textContent).toContain(
      "selected_ref=result: result:stress:pipe-P-120:end-j:torsional-shear"
    );
    expect(within(savedNativePackage).getByTestId("native-package-persisted-review-context").textContent).toContain(
      "mechanics_results=1"
    );
    expect(within(savedNativePackage).getByTestId("native-package-persisted-review-context").textContent).toContain(
      "analysis_runs=1"
    );
    expect(within(savedNativePackage).getByTestId("native-package-persisted-review-context").textContent).toContain(
      "run_ref=run:preview-linear-static-001"
    );
    const savedNativePackageHref =
      within(savedNativePackage).getByTestId("native-package-link").getAttribute("href") ?? "";
    const savedNativePackagePacket = JSON.parse(decodeURIComponent(savedNativePackageHref.split(",", 2)[1]));
    expect(savedNativePackagePacket.source_project.storage_summary.proposal_count).toBe(1);
    expect(savedNativePackagePacket.source_project.storage_summary.selected_review_target_count).toBe(1);
    expect(savedNativePackagePacket.source_project.storage_summary.selected_review_target_ref).toBe(
      "result: result:stress:pipe-P-120:end-j:torsional-shear"
    );
    expect(savedNativePackagePacket.operation_review.persisted_review_context.proposal_count).toBe(1);
    expect(savedNativePackagePacket.operation_review.persisted_review_context.selected_review_target_count).toBe(1);
    expect(savedNativePackagePacket.operation_review.persisted_review_context.selected_review_target_ref).toBe(
      "result: result:stress:pipe-P-120:end-j:torsional-shear"
    );
    expect(savedNativePackagePacket.generation_context.persisted_proposal_count).toBe(1);
    expect(savedNativePackagePacket.generation_context.persisted_selected_review_target_count).toBe(1);
    expect(savedNativePackagePacket.generation_context.persisted_selected_review_target_ref).toBe(
      "result: result:stress:pipe-P-120:end-j:torsional-shear"
    );
    expect(savedNativePackagePacket.source_project.storage_summary.persisted_mechanics_result_count).toBe(1);
    expect(savedNativePackagePacket.source_project.storage_summary.persisted_analysis_run_count).toBe(1);
    expect(savedNativePackagePacket.source_project.storage_summary.persisted_analysis_run_ref).toBe(
      "run:preview-linear-static-001"
    );
    expect(savedNativePackagePacket.generation_context.persisted_mechanics_result_count).toBe(1);
    expect(savedNativePackagePacket.generation_context.persisted_analysis_run_count).toBe(1);
    expect(savedNativePackagePacket.generation_context.persisted_analysis_run_ref).toBe(
      "run:preview-linear-static-001"
    );

    fireEvent.click(within(controls).getByRole("button", { name: /Open local/i }));
    await waitFor(() =>
      expect(screen.getByTestId("local-project-message")).toHaveTextContent(
        "Opened local browser-preview project snapshot."
      )
    );

    expect(screen.getByTestId("solve-job-summary").textContent).toContain("state=completed");
    expect(screen.getByTestId("solve-job-summary").textContent).toContain("events=1");
    expect(screen.getByTestId("solve-job-summary").textContent).toContain("result_rows=647");
    expect(screen.getByTestId("solve-job-progress").textContent).toContain(
      "restored_persisted_run_record_no_new_solve_executed"
    );
    expect(
      within(results).getByTestId("result-row-result:stress:pipe-P-120:end-j:torsional-shear")
    ).toBeInTheDocument();
    fireEvent.change(within(results).getByTestId("result-filter-input"), { target: { value: "" } });
    expect(await screen.findByTestId("result-group-displacement")).toBeInTheDocument();
    expect(screen.getByTestId("local-project-review-context").textContent).toContain(
      "1 pending operation; applied=false; editor_intents=0; agent_proposals=1"
    );
    expect(await within(proposalPanel).findByText("proposal:physics-diagnostic-review")).toBeInTheDocument();
    expect(within(proposalPanel).getByTestId("selected-review-target").textContent).toContain(
      "result: result:stress:pipe-P-120:end-j:torsional-shear"
    );
    expect(within(proposalPanel).getByTestId("proposal-affected-entities").textContent).toContain(
      "result:stress:pipe-P-120:end-j:torsional-shear"
    );

    expect(within(storageAudit).getByTestId("project-storage-summary").textContent).toContain("operation=open");
    expect(within(storageAudit).getByTestId("project-storage-summary").textContent).toContain(
      "pending operations=1"
    );
    expect(within(storageAudit).getByTestId("project-storage-snapshot").textContent).toContain(
      "persisted_proposals=1"
    );
    expect(within(storageAudit).getByTestId("project-storage-snapshot").textContent).toContain(
      "persisted_review_targets=1"
    );
    expect(within(storageAudit).getByTestId("project-storage-snapshot").textContent).toContain(
      "persisted_review_target_ref=result: result:stress:pipe-P-120:end-j:torsional-shear"
    );
    expect(within(storageAudit).getByTestId("project-storage-snapshot").textContent).toContain(
      "persisted_mechanics_results=1"
    );
    expect(within(storageAudit).getByTestId("project-storage-snapshot").textContent).toContain(
      "persisted_analysis_runs=1"
    );
    expect(within(storageAudit).getByTestId("project-storage-snapshot").textContent).toContain(
      "persisted_analysis_run_ref=run:preview-linear-static-001"
    );
    const openedStorageHref =
      within(storageAudit).getByTestId("project-storage-export-link").getAttribute("href") ?? "";
    const openedStoragePacket = JSON.parse(decodeURIComponent(openedStorageHref.split(",", 2)[1]));
    expect(openedStoragePacket.summary.last_operation).toBe("open");
    expect(openedStoragePacket.summary.proposal_operation_count).toBe(1);
    expect(openedStoragePacket.summary.persisted_proposal_count).toBe(1);
    expect(openedStoragePacket.summary.persisted_selected_review_target_count).toBe(1);
    expect(openedStoragePacket.summary.persisted_selected_review_target_ref).toBe(
      "result: result:stress:pipe-P-120:end-j:torsional-shear"
    );
    expect(openedStoragePacket.project_summary.proposal_count).toBe(1);
    expect(openedStoragePacket.project_summary.selected_review_target_count).toBe(1);
    expect(openedStoragePacket.project_summary.selected_review_target_ref).toBe(
      "result: result:stress:pipe-P-120:end-j:torsional-shear"
    );
    expect(openedStoragePacket.summary.persisted_mechanics_result_count).toBe(1);
    expect(openedStoragePacket.summary.persisted_analysis_run_count).toBe(1);
    expect(openedStoragePacket.summary.persisted_analysis_run_ref).toBe("run:preview-linear-static-001");
    expect(openedStoragePacket.project_summary.persisted_mechanics_result_count).toBe(1);
    expect(openedStoragePacket.project_summary.persisted_analysis_run_count).toBe(1);
    expect(openedStoragePacket.project_summary.persisted_analysis_run_ref).toBe("run:preview-linear-static-001");
    expect(openedStoragePacket.proposal_refs).toContain("proposal:physics-diagnostic-review");
    expect(openedStoragePacket.review_operation_statuses).toContain("not_applied");

    expect(within(projectValidation).getByTestId("project-validation-operations").textContent).toContain(
      "pending operations=1"
    );
    const openedValidationHref =
      within(projectValidation).getByTestId("project-validation-export-link").getAttribute("href") ?? "";
    const openedValidationPacket = JSON.parse(decodeURIComponent(openedValidationHref.split(",", 2)[1]));
    expect(openedValidationPacket.summary.last_operation).toBe("open");
    expect(openedValidationPacket.summary.proposal_operation_count).toBe(1);
    expect(openedValidationPacket.summary.persisted_proposal_count).toBe(1);
    expect(openedValidationPacket.summary.persisted_selected_review_target_count).toBe(1);
    expect(openedValidationPacket.summary.persisted_selected_review_target_ref).toBe(
      "result: result:stress:pipe-P-120:end-j:torsional-shear"
    );
    expect(openedValidationPacket.project_summary.proposal_count).toBe(1);
    expect(openedValidationPacket.project_summary.selected_review_target_count).toBe(1);
    expect(openedValidationPacket.project_summary.selected_review_target_ref).toBe(
      "result: result:stress:pipe-P-120:end-j:torsional-shear"
    );
    expect(openedValidationPacket.summary.persisted_mechanics_result_count).toBe(1);
    expect(openedValidationPacket.summary.persisted_analysis_run_count).toBe(1);
    expect(openedValidationPacket.summary.persisted_analysis_run_ref).toBe("run:preview-linear-static-001");
    expect(openedValidationPacket.proposal_refs).toContain("proposal:physics-diagnostic-review");

    const openedNativePackage = await screen.findByLabelText("Native JSON package");
    expect(within(openedNativePackage).getByTestId("native-package-persisted-review-context").textContent).toContain(
      "mechanics_results=1"
    );
    expect(within(openedNativePackage).getByTestId("native-package-persisted-review-context").textContent).toContain(
      "analysis_runs=1"
    );
    expect(within(openedNativePackage).getByTestId("native-package-persisted-review-context").textContent).toContain(
      "run_ref=run:preview-linear-static-001"
    );
    const openedNativePackageHref =
      within(openedNativePackage).getByTestId("native-package-link").getAttribute("href") ?? "";
    const openedNativePackagePacket = JSON.parse(decodeURIComponent(openedNativePackageHref.split(",", 2)[1]));
    expect(openedNativePackagePacket.source_project.storage_summary.persisted_mechanics_result_count).toBe(1);
    expect(openedNativePackagePacket.source_project.storage_summary.persisted_analysis_run_count).toBe(1);
    expect(openedNativePackagePacket.source_project.storage_summary.persisted_analysis_run_ref).toBe(
      "run:preview-linear-static-001"
    );
    expect(openedNativePackagePacket.generation_context.persisted_mechanics_result_count).toBe(1);
    expect(openedNativePackagePacket.generation_context.persisted_analysis_run_count).toBe(1);
    expect(openedNativePackagePacket.generation_context.persisted_analysis_run_ref).toBe(
      "run:preview-linear-static-001"
    );

    const operationLedger = await screen.findByLabelText("Operation review ledger");
    expect(await within(operationLedger).findByTestId("operation-ledger-export-summary")).toHaveTextContent(
      "1 review record"
    );
    expect(within(operationLedger).getByTestId("operation-ledger-state-binding").textContent).toContain(
      "state:project:invented-loop-01:preview; run:preview-linear-static-001"
    );
    expect(
      within(operationLedger).getByTestId("operation-ledger-record-op-review-computed-diagnostic").textContent
    ).toContain("result:stress:pipe-P-120:end-j:torsional-shear");
    const ledgerHref = within(operationLedger).getByTestId("operation-ledger-export-link").getAttribute("href") ?? "";
    const ledgerPacket = JSON.parse(decodeURIComponent(ledgerHref.split(",", 2)[1]));
    expect(ledgerPacket.selected_review_target).toEqual({
      target_type: "result",
      id: "result:stress:pipe-P-120:end-j:torsional-shear"
    });
    expect(ledgerPacket.records[0].selected_review_target).toEqual({
      target_type: "result",
      id: "result:stress:pipe-P-120:end-j:torsional-shear"
    });

    const exportReview = await screen.findByLabelText("Export safety review");
    const openedReviewHref = within(exportReview).getByTestId("export-review-link").getAttribute("href") ?? "";
    const openedReviewManifest = JSON.parse(decodeURIComponent(openedReviewHref.split(",", 2)[1]));
    expect(openedReviewManifest.summary.operation_record_count).toBe(1);
    expect(
      openedReviewManifest.exports.find((item: { export_id: string }) => item.export_id === "project_storage_audit")
        .proposal_operation_count
    ).toBe(1);
    expect(
      openedReviewManifest.exports.find((item: { export_id: string }) => item.export_id === "project_storage_audit")
        .persisted_proposal_count
    ).toBe(1);
    expect(
      openedReviewManifest.exports.find((item: { export_id: string }) => item.export_id === "project_storage_audit")
        .persisted_selected_review_target_count
    ).toBe(1);
    expect(
      openedReviewManifest.exports.find((item: { export_id: string }) => item.export_id === "project_storage_audit")
        .persisted_selected_review_target_ref
    ).toBe("result: result:stress:pipe-P-120:end-j:torsional-shear");
    expect(
      openedReviewManifest.exports.find(
        (item: { export_id: string }) => item.export_id === "project_validation_preflight"
      ).proposal_operation_count
    ).toBe(1);
    expect(
      openedReviewManifest.exports.find(
        (item: { export_id: string }) => item.export_id === "project_validation_preflight"
      ).persisted_proposal_count
    ).toBe(1);
    expect(
      openedReviewManifest.exports.find(
        (item: { export_id: string }) => item.export_id === "project_validation_preflight"
      ).persisted_selected_review_target_count
    ).toBe(1);
    expect(
      openedReviewManifest.exports.find(
        (item: { export_id: string }) => item.export_id === "project_validation_preflight"
      ).persisted_selected_review_target_ref
    ).toBe("result: result:stress:pipe-P-120:end-j:torsional-shear");
    expect(
      openedReviewManifest.exports.find((item: { export_id: string }) => item.export_id === "operation_review_ledger")
        .readiness
    ).toBe("available");

    expect(screen.getByTestId("project-index-picker")).toBeInTheDocument();
    fireEvent.click(screen.getByTestId("project-index-open-project:invented-loop-01"));
    await waitFor(() =>
      expect(screen.getByTestId("local-project-message")).toHaveTextContent(
        "Opened local browser-preview project snapshot by id project:invented-loop-01."
      )
    );
    expect(within(storageAudit).getByTestId("project-storage-summary").textContent).toContain(
      "operation=open_by_id"
    );
    const openedByIdStorageHref =
      within(storageAudit).getByTestId("project-storage-export-link").getAttribute("href") ?? "";
    const openedByIdStoragePacket = JSON.parse(decodeURIComponent(openedByIdStorageHref.split(",", 2)[1]));
    expect(openedByIdStoragePacket.summary.last_operation).toBe("open_by_id");
    expect(openedByIdStoragePacket.project_summary.project_id).toBe("project:invented-loop-01");
    expect(openedByIdStoragePacket.summary.persisted_proposal_count).toBe(1);
    expect(openedByIdStoragePacket.summary.persisted_selected_review_target_ref).toBe(
      "result: result:stress:pipe-P-120:end-j:torsional-shear"
    );
    expect(openedByIdStoragePacket.summary.persisted_analysis_run_ref).toBe("run:preview-linear-static-001");
  }, 10000);

  it("shows computed mechanics diagnostics in results, knowledge, and review-only proposal context", async () => {
    render(<App />);

    const runButton = await screen.findByRole("button", { name: /Run mechanics preview/i });
    fireEvent.click(runButton);

    expect(await screen.findByTestId("result-group-displacement", {}, { timeout: 10000 })).toBeInTheDocument();
    const solvedReadiness = screen.getByTestId("solve-readiness-summary");
    expect(within(solvedReadiness).getByTestId("readiness-mechanics").textContent).toContain(
      "647 computed result rows"
    );
    expect(within(solvedReadiness).getByTestId("readiness-mechanics").textContent).toContain("mechanics solved");
    expect(within(solvedReadiness).getByTestId("readiness-rule").textContent).toContain("rule inputs incomplete");
    expect(within(solvedReadiness).getByTestId("readiness-diagnostics").textContent).toContain("7 diagnostics");
    expect(within(solvedReadiness).getByTestId("readiness-diagnostics").textContent).toContain("7 warnings");
    expect(within(solvedReadiness).getByTestId("readiness-diagnostics").textContent).toContain("0 blocking/error");
    expect(within(solvedReadiness).getByTestId("readiness-professional").textContent).toContain(
      "no professional acceptance record"
    );
    expect(screen.getByTestId("solve-job-summary").textContent).toContain("state=completed");
    expect(screen.getByTestId("solve-job-summary").textContent).toContain("events=3");
    expect(screen.getByTestId("solve-job-summary").textContent).toContain("result_rows=647");
    expect(screen.getByTestId("solve-job-summary").textContent).toContain("cancellation_requested=false");
    expect(screen.getByTestId("solve-job-progress").textContent).toContain("completed");
    expect(screen.getByTestId("solve-job-progress").textContent).toContain("percentages_synthesized=false");
    expect(screen.getByTestId("solve-job-cancellation").textContent).toContain("enabled=false");
    expect(screen.getByTestId("solve-job-cancellation").textContent).toContain("requested=false");
    expect(screen.getByTestId("solve-job-cancellation").textContent).toContain("success_claimed=false");
    expect(screen.getByTestId("solve-job-binding").textContent).toContain("state:project:invented-loop-01:preview");
    expect(screen.getByTestId("solve-job-binding").textContent).toContain("run:preview-linear-static-001");
    expect(screen.getByTestId("solve-job-binding").textContent).toContain("result rows=647");
    expect(screen.getByTestId("solve-job-binding").textContent).toContain("hashes=647");
    expect(screen.getByTestId("solve-job-boundary").textContent).toContain("release/professional claim=false");
    const solveJobHref = screen.getByTestId("solve-job-export-link").getAttribute("href") ?? "";
    const solveJobPacket = JSON.parse(decodeURIComponent(solveJobHref.split(",", 2)[1]));
    expect(solveJobPacket.document_kind).toBe("openpipestress.technical_preview.solve_job_audit");
    expect(solveJobPacket.deliverable_refs).toContain("DEL-07-07");
    expect(solveJobPacket.deliverable_refs).toContain("DEL-14-02");
    expect(solveJobPacket.deliverable_refs).toContain("DEL-04-06");
    expect(solveJobPacket.scope_items).toContain("SOW-055");
    expect(solveJobPacket.scope_items).toContain("SOW-072");
    expect(solveJobPacket.scope_items).toContain("SOW-053");
    expect(solveJobPacket.summary.job_state).toBe("completed");
    expect(solveJobPacket.summary.event_count).toBe(3);
    expect(solveJobPacket.summary.result_row_count).toBe(647);
    expect(solveJobPacket.summary.diagnostic_count).toBe(7);
    expect(solveJobPacket.summary.cancellation_requested).toBe(false);
    expect(solveJobPacket.summary.cancellation_status).toBe("not_requested");
    expect(solveJobPacket.progress_contract.progress_basis).toBe(
      "preview_service_event_state_only_no_percent_stream"
    );
    expect(solveJobPacket.progress_contract.percentages_synthesized).toBe(false);
    expect(solveJobPacket.progress_contract.backend_percent_stream_available).toBe(false);
    expect(solveJobPacket.progress_contract.latest_event_state).toBe("completed");
    expect(solveJobPacket.cancellation.request_control_visible).toBe(true);
    expect(solveJobPacket.cancellation.request_enabled).toBe(false);
    expect(solveJobPacket.cancellation.requested).toBe(false);
    expect(solveJobPacket.cancellation.backend_cancellation_token).toBe("TBD");
    expect(solveJobPacket.cancellation.mutates_solver_process_directly).toBe(false);
    expect(solveJobPacket.cancellation.cancellation_success_claimed).toBe(false);
    expect(solveJobPacket.model_state_ref.ref).toBe("state:project:invented-loop-01:preview");
    expect(solveJobPacket.analysis_run_ref.ref).toBe("run:preview-linear-static-001");
    expect(solveJobPacket.run_kind).toBe("mechanics_solve");
    expect(solveJobPacket.analysis_status).toContain("HUMAN_REVIEW_REQUIRED");
    expect(solveJobPacket.analysis_status).toContain("MECHANICS_SOLVED");
    expect(solveJobPacket.analysis_status).toContain("RULE_INPUTS_INCOMPLETE");
    expect(solveJobPacket.result_hash_count).toBe(647);
    expect(solveJobPacket.hash_scopes).toContain("analysis_run_record");
    expect(solveJobPacket.hash_scopes).toContain("result_envelope");
    const resultExport = await screen.findByLabelText("Result export audit");
    expect(within(resultExport).getByTestId("result-export-summary").textContent).toContain("available");
    expect(within(resultExport).getByTestId("result-export-summary").textContent).toContain("rows=647");
    expect(within(resultExport).getByTestId("result-export-summary").textContent).toContain("sets=1");
    expect(within(resultExport).getByTestId("result-export-summary").textContent).toContain("diagnostics=7");
    expect(within(resultExport).getByTestId("result-export-format").textContent).toContain(
      "schema_first_json_result_envelope"
    );
    expect(within(resultExport).getByTestId("result-export-format").textContent).toContain(
      "additional_formats=stress_neutral_csv_json_preview_available"
    );
    expect(within(resultExport).getByTestId("result-export-state-binding").textContent).toContain(
      "project:invented-loop-01"
    );
    expect(within(resultExport).getByTestId("result-export-state-binding").textContent).toContain(
      "run:preview-linear-static-001"
    );
    expect(within(resultExport).getByTestId("result-export-units").textContent).toContain("explicit units");
    expect(within(resultExport).getByTestId("result-export-units").textContent).toContain("length");
    expect(within(resultExport).getByTestId("result-export-units").textContent).toContain("stress");
    expect(within(resultExport).getByTestId("result-export-reproducibility").textContent).toContain(
      "deterministic_ordering=true"
    );
    expect(within(resultExport).getByTestId("result-export-reproducibility").textContent).toContain("run_hashes=2");
    expect(within(resultExport).getByTestId("result-export-boundary").textContent).toContain(
      "no compliance, certification, sealing, authentication, or approval claim"
    );
    const resultExportHref = within(resultExport).getByTestId("result-export-link").getAttribute("href") ?? "";
    const resultExportPacket = JSON.parse(decodeURIComponent(resultExportHref.split(",", 2)[1]));
    expect(resultExportPacket.deliverable_id).toBe("DEL-08-04");
    expect(resultExportPacket.package_id).toBe("PKG-08");
    expect(resultExportPacket.scope_item).toBe("SOW-046");
    expect(resultExportPacket.objectives).toContain("OBJ-007");
    expect(resultExportPacket.objectives).toContain("OBJ-009");
    expect(resultExportPacket.export_format_status.baseline_format).toBe("schema_first_json_result_envelope");
    expect(resultExportPacket.export_format_status.additional_formats).toBe(
      "stress_neutral_csv_json_preview_available"
    );
    expect(resultExportPacket.export_format_status.public_transport_protocol).toBe("TBD");
    expect(resultExportPacket.result_envelope.envelope_id).toBe("result-envelope:run:preview-linear-static-001");
    expect(resultExportPacket.result_envelope.model_ref.ref_id).toBe("project:invented-loop-01");
    expect(resultExportPacket.result_envelope.run_ref.ref_id).toBe("run:preview-linear-static-001");
    expect(resultExportPacket.result_envelope.load_basis_refs.length).toBeGreaterThan(0);
    expect(resultExportPacket.result_envelope.result_sets).toHaveLength(1);
    expect(resultExportPacket.result_envelope.result_sets[0].set_type).toBe("mechanics");
    expect(resultExportPacket.result_envelope.result_sets[0].values).toHaveLength(647);
    expect(resultExportPacket.result_envelope.result_sets[0].values[0].result_id).toBeTruthy();
    expect(resultExportPacket.result_envelope.result_sets[0].values[0].unit).toBeTruthy();
    expect(resultExportPacket.result_envelope.result_sets[0].values[0].dimension).toBeTruthy();
    expect(resultExportPacket.result_envelope.diagnostics).toHaveLength(7);
    expect(resultExportPacket.result_envelope.reproducibility.deterministic_ordering).toBe(true);
    expect(resultExportPacket.result_envelope.reproducibility.run_hashes).toHaveLength(2);
    expect(resultExportPacket.result_envelope.analysis_status).toContain("HUMAN_REVIEW_REQUIRED");
    expect(resultExportPacket.result_envelope.analysis_status).toContain("MECHANICS_SOLVED");
    expect(resultExportPacket.result_envelope.analysis_status).toContain("RULE_INPUTS_INCOMPLETE");
    expect(resultExportPacket.result_envelope.rule_pack_refs[0].private_payload_redacted).toBe(true);
    expect(resultExportPacket.result_envelope.rule_pack_refs[0].completeness_status).toBe(
      "missing_required_inputs"
    );
    expect(resultExportPacket.result_envelope.downstream_use.additional_export_formats).toBe(
      "stress_neutral_csv_json_preview_available"
    );
    expect(resultExportPacket.result_envelope.professional_boundary.software_makes_compliance_claim).toBe(false);
    expect(resultExportPacket.result_envelope.professional_boundary.software_makes_certification_claim).toBe(false);
    expect(resultExportPacket.result_envelope.professional_boundary.software_makes_sealing_claim).toBe(false);
    expect(resultExportPacket.result_envelope.professional_boundary.software_makes_approval_claim).toBe(false);
    expect(resultExportPacket.result_envelope.professional_boundary.software_makes_authentication_claim).toBe(false);
    const stressNeutral = await screen.findByLabelText("Stress-neutral CSV JSON export");
    expect(within(stressNeutral).getByTestId("stress-neutral-summary").textContent).toContain("available");
    expect(within(stressNeutral).getByTestId("stress-neutral-summary").textContent).toContain("rows=647");
    expect(within(stressNeutral).getByTestId("stress-neutral-summary").textContent).toContain("csv_columns=11");
    expect(within(stressNeutral).getByTestId("stress-neutral-format").textContent).toContain(
      "stress_neutral_csv_json"
    );
    expect(within(stressNeutral).getByTestId("stress-neutral-format").textContent).toContain(
      "ops.stress_neutral.v1"
    );
    expect(within(stressNeutral).getByTestId("stress-neutral-state-binding").textContent).toContain(
      "project:invented-loop-01"
    );
    expect(within(stressNeutral).getByTestId("stress-neutral-state-binding").textContent).toContain(
      "run:preview-linear-static-001"
    );
    expect(within(stressNeutral).getByTestId("stress-neutral-units").textContent).toContain("explicit units");
    expect(within(stressNeutral).getByTestId("stress-neutral-units").textContent).toContain("stress");
    expect(within(stressNeutral).getByTestId("stress-neutral-package").textContent).toContain("members=7");
    expect(within(stressNeutral).getByTestId("stress-neutral-package").textContent).toContain("stable_ids=647");
    expect(within(stressNeutral).getByTestId("stress-neutral-package").textContent).toContain("validation=passed");
    await waitFor(() =>
      expect(within(stressNeutral).getByTestId("stress-neutral-package").textContent).toContain(
        "package_hash=computed_local_preview_sha256"
      )
    );
    expect(within(stressNeutral).getByTestId("stress-neutral-boundary").textContent).toContain(
      "vendor_format=false"
    );
    expect(within(stressNeutral).getByTestId("stress-neutral-boundary").textContent).toContain(
      "solver_validation=false"
    );
    const stressNeutralHref =
      within(stressNeutral).getByTestId("stress-neutral-export-link").getAttribute("href") ?? "";
    const stressNeutralPacket = JSON.parse(decodeURIComponent(stressNeutralHref.split(",", 2)[1]));
    expect(stressNeutralPacket.document_kind).toBe(
      "openpipestress.technical_preview.stress_neutral_csv_json_package"
    );
    expect(stressNeutralPacket.deliverable_id).toBe("DEL-17-06");
    expect(stressNeutralPacket.package_id).toBe("PKG-17");
    expect(stressNeutralPacket.scope_items).toContain("SOW-046");
    expect(stressNeutralPacket.scope_items).toContain("SOW-074");
    expect(stressNeutralPacket.objectives).toContain("OBJ-007");
    expect(stressNeutralPacket.objectives).toContain("OBJ-017");
    expect(stressNeutralPacket.objectives).toContain("OBJ-018");
    expect(stressNeutralPacket.package_status).toBe("stress_neutral_export_package");
    expect(stressNeutralPacket.export_profile.target_family).toBe("stress_neutral_csv_json");
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
      "correlation_status"
    ]);
    expect(stressNeutralPacket.result_rows).toHaveLength(647);
    expect(stressNeutralPacket.stable_id_map).toHaveLength(647);
    expect(stressNeutralPacket.csv_text.split("\n")[0]).toBe(
      "result_id,canonical_ref,row_kind,result_family,load_case_ref,station_ref,component_ref,value,unit,dimension,correlation_status"
    );
    expect(stressNeutralPacket.csv_text).toContain("result:force:pipe-P-120:axial");
    expect(stressNeutralPacket.loss_report.entries).toHaveLength(3);
    expect(stressNeutralPacket.loss_report.entries.map((entry: { category: string }) => entry.category)).toContain(
      "tbd"
    );
    expect(stressNeutralPacket.manifest.package_members).toHaveLength(7);
    expect(stressNeutralPacket.manifest.canonical_package_hash_status).toBe("computed_local_preview_sha256");
    expect(stressNeutralPacket.manifest.canonical_package_hash.value).toMatch(/^sha256:[0-9a-f]{64}$/);
    expect(stressNeutralPacket.manifest.canonical_package_hash.payload_scope).toBe("package_review_payload");
    expect(stressNeutralPacket.manifest.canonical_package_hash.payload_excludes).toBe(
      "manifest_and_validation_report_package_hash_carrier_fields"
    );
    expect(stressNeutralPacket.validation_report.hash_validation_status).toBe(
      "package_hash_computed_local_preview_not_independently_validated"
    );
    expect(stressNeutralPacket.validation_report.validation_status).toBe("passed");
    expect(stressNeutralPacket.validation_report.schema_validation_status).toBe(
      "desktop_preview_shape_aligned_not_runtime_json_schema_validated"
    );
    expect(stressNeutralPacket.result_rows.every((row: { unit: string; dimension: string }) => row.unit && row.dimension)).toBe(
      true
    );
    expect(stressNeutralPacket.private_payload_included).toBe(false);
    expect(stressNeutralPacket.protected_content_included).toBe(false);
    expect(stressNeutralPacket.vendor_format_claim).toBe(false);
    expect(stressNeutralPacket.solver_validation_claim).toBe(false);
    expect(stressNeutralPacket.code_compliance_claim).toBe(false);
    expect(stressNeutralPacket.professional_reliance_claim).toBe(false);
    const stressNeutralCsvHref = within(stressNeutral).getByTestId("stress-neutral-csv-link").getAttribute("href") ?? "";
    expect(decodeURIComponent(stressNeutralCsvHref.split(",", 2)[1]).split("\n")[0]).toBe(
      "result_id,canonical_ref,row_kind,result_family,load_case_ref,station_ref,component_ref,value,unit,dimension,correlation_status"
    );
    const headlessRunner = await screen.findByLabelText("Headless runner envelope");
    expect(within(headlessRunner).getByTestId("headless-runner-summary").textContent).toContain("job=COMPLETED");
    expect(within(headlessRunner).getByTestId("headless-runner-summary").textContent).toContain("outputs=4");
    expect(within(headlessRunner).getByTestId("headless-runner-summary").textContent).toContain("result_refs=647");
    expect(within(headlessRunner).getByTestId("headless-runner-job").textContent).toContain(
      "job:preview-linear-static:project-invented-loop-01"
    );
    expect(within(headlessRunner).getByTestId("headless-runner-job").textContent).toContain("3/3");
    expect(within(headlessRunner).getByTestId("headless-runner-result-handoff").textContent).toContain(
      "schema_first_json_result_envelope"
    );
    expect(within(headlessRunner).getByTestId("headless-runner-result-handoff").textContent).toContain(
      "audit=audit-manifest:run:preview-linear-static-001:preview"
    );
    expect(within(headlessRunner).getByTestId("headless-runner-result-handoff").textContent).toContain(
      "checksums=2"
    );
    const headlessHref = within(headlessRunner).getByTestId("headless-runner-export-link").getAttribute("href") ?? "";
    const headlessPacket = JSON.parse(decodeURIComponent(headlessHref.split(",", 2)[1]));
    expect(headlessPacket.deliverable_id).toBe("DEL-10-05");
    expect(headlessPacket.scope_items).toContain("SOW-054");
    expect(headlessPacket.scope_items).toContain("SOW-032");
    expect(headlessPacket.objectives).toContain("OBJ-012");
    expect(headlessPacket.request.operation).toBe("solve");
    expect(headlessPacket.request.load_basis_refs.length).toBeGreaterThan(0);
    expect(headlessPacket.request.input_manifest_ref.ref_id).toBe("result-envelope:run:preview-linear-static-001");
    expect(headlessPacket.result.run_id).toBe("run:preview-linear-static-001");
    expect(headlessPacket.result.job.state).toBe("COMPLETED");
    expect(headlessPacket.result.job.progress.current_step).toBe(3);
    expect(headlessPacket.result.job.cancellation.supported).toBe(true);
    expect(headlessPacket.result.job.cancellation.requested).toBe(false);
    expect(headlessPacket.result.analysis_status).toContain("HUMAN_REVIEW_REQUIRED");
    expect(headlessPacket.result.analysis_status).toContain("MECHANICS_SOLVED");
    expect(headlessPacket.result.analysis_status).toContain("RULE_INPUTS_INCOMPLETE");
    expect(headlessPacket.result.result_envelope_ref.schema_ref).toBe("schemas/results.schema.yaml");
    expect(headlessPacket.result.result_envelope_ref.envelope_ref.ref_id).toBe(
      "result-envelope:run:preview-linear-static-001"
    );
    expect(headlessPacket.result.result_refs).toHaveLength(647);
    expect(headlessPacket.result.audit_manifest_ref.ref_id).toBe("audit-manifest:run:preview-linear-static-001:preview");
    expect(headlessPacket.result.checksums).toHaveLength(2);
    expect(headlessPacket.result.diagnostics).toHaveLength(7);
    expect(headlessPacket.result.privacy.telemetry_allowed).toBe(false);
    expect(headlessPacket.result.professional_boundary.software_makes_compliance_claim).toBe(false);
    expect(headlessPacket.result.professional_boundary.software_makes_authentication_claim).toBe(false);
    const adapterFramework = await screen.findByLabelText("Adapter framework envelope");
    expect(within(adapterFramework).getByTestId("adapter-framework-summary").textContent).toContain(
      "capabilities=4"
    );
    expect(within(adapterFramework).getByTestId("adapter-framework-summary").textContent).toContain(
      "diagnostics=7"
    );
    expect(within(adapterFramework).getByTestId("adapter-framework-validation").textContent).toContain(
      "export=required_before_shared_payload"
    );
    const adapterHref =
      within(adapterFramework).getByTestId("adapter-framework-export-link").getAttribute("href") ?? "";
    const adapterPacket = JSON.parse(decodeURIComponent(adapterHref.split(",", 2)[1]));
    expect(adapterPacket.deliverable_id).toBe("DEL-10-02");
    expect(adapterPacket.scope_item).toBe("SOW-030");
    expect(adapterPacket.objective).toBe("OBJ-009");
    expect(adapterPacket.operation_result.operation_id).toBe(
      "adapter-operation:desktop-preview:run-preview-linear-static-001"
    );
    expect(adapterPacket.operation_result.operation_class).toBe("export");
    expect(adapterPacket.operation_result.parse_status).toBe("not_parsed_by_framework");
    expect(adapterPacket.operation_result.diagnostics).toHaveLength(7);
    expect(adapterPacket.operation_result.checksums).toHaveLength(2);
    expect(adapterPacket.operation_result.audit_manifest_refs[0].ref_id).toBe(
      "audit-manifest:run:preview-linear-static-001:preview"
    );
    expect(adapterPacket.operation_result.result_envelope_ref.ref.ref_id).toBe(
      "result-envelope:run:preview-linear-static-001"
    );
    expect(adapterPacket.adapter_declaration.no_bypass_controls.must_preserve_diagnostics).toBe(true);
    expect(adapterPacket.adapter_declaration.no_bypass_controls.must_preserve_report_controls).toBe(true);
    expect(adapterPacket.adapter_declaration.no_bypass_controls.must_preserve_human_acceptance_boundary).toBe(true);
    expect(adapterPacket.adapter_declaration.no_bypass_controls.must_not_execute_arbitrary_code).toBe(true);
    expect(adapterPacket.adapter_declaration.no_bypass_controls.must_not_transmit_private_data_by_default).toBe(true);
    expect(adapterPacket.operation_result.privacy.local_first).toBe(true);
    expect(adapterPacket.operation_result.privacy.private_payload_redacted).toBe(true);
    expect(adapterPacket.operation_result.professional_boundary.software_makes_security_certification_claim).toBe(false);
    const localFea = await screen.findByLabelText("Local FEA handoff");
    expect(within(localFea).getByTestId("local-fea-summary").textContent).toContain("available");
    expect(within(localFea).getByTestId("local-fea-summary").textContent).toContain("labels=4");
    expect(within(localFea).getByTestId("local-fea-summary").textContent).toContain("flags=5");
    expect(within(localFea).getByTestId("local-fea-summary").textContent).toContain("diagnostics=4");
    expect(within(localFea).getByTestId("local-fea-contract").textContent).toContain(
      "schema_first_local_fea_handoff_contract"
    );
    expect(within(localFea).getByTestId("local-fea-contract").textContent).toContain("format=TBD");
    expect(within(localFea).getByTestId("local-fea-state-binding").textContent).toContain(
      "state:project:invented-loop-01:preview"
    );
    expect(within(localFea).getByTestId("local-fea-state-binding").textContent).toContain(
      "result-envelope:run:preview-linear-static-001"
    );
    expect(within(localFea).getByTestId("local-fea-region").textContent).toContain("basis=diagnostic_suggested");
    expect(within(localFea).getByTestId("local-fea-region").textContent).toContain("elements=2");
    expect(within(localFea).getByTestId("local-fea-transfer").textContent).toContain(
      "method=result_reference_only"
    );
    expect(within(localFea).getByTestId("local-fea-transfer").textContent).toContain("loads=3");
    expect(within(localFea).getByTestId("local-fea-unsupported").textContent).toContain(
      "mesh_generation_not_performed"
    );
    expect(within(localFea).getByTestId("local-fea-unsupported").textContent).toContain(
      "external_solver_not_invoked"
    );
    expect(within(localFea).getByTestId("local-fea-boundary").textContent).toContain("human_review=true");
    expect(within(localFea).getByTestId("local-fea-boundary").textContent).toContain("compliance=false");
    const localFeaHref = within(localFea).getByTestId("local-fea-export-link").getAttribute("href") ?? "";
    const localFeaPacket = JSON.parse(decodeURIComponent(localFeaHref.split(",", 2)[1]));
    expect(localFeaPacket.deliverable_id).toBe("DEL-10-03");
    expect(localFeaPacket.package_id).toBe("PKG-10");
    expect(localFeaPacket.scope_items).toContain("SOW-031");
    expect(localFeaPacket.scope_items).toContain("SOW-049");
    expect(localFeaPacket.objective).toBe("OBJ-009");
    expect(localFeaPacket.contract_status.global_analysis_role).toBe("primary_global_centerline_frame_model");
    expect(localFeaPacket.contract_status.local_analysis_role).toBe("optional_specialized_shell_solid_handoff");
    expect(localFeaPacket.contract_status.concrete_export_format).toBe("TBD");
    expect(localFeaPacket.contract_status.mesh_generation).toBe("TBD");
    expect(localFeaPacket.contract_status.external_solver_invocation).toBe("TBD");
    expect(localFeaPacket.contract_status.professional_decision).toBe("human_review_required");
    expect(localFeaPacket.handoff_package.package_kind).toBe("local_shell_solid_fea_handoff");
    expect(localFeaPacket.handoff_package.source_refs.global_model_kind).toBe("centerline_frame_global_analysis");
    expect(localFeaPacket.handoff_package.local_region.selection_basis).toBe("diagnostic_suggested");
    expect(localFeaPacket.handoff_package.local_region.selected_entity_ids.element_ids).toContain("pipe:P-120");
    expect(localFeaPacket.handoff_package.local_region.selected_entity_ids.element_ids).toContain("pipe:P-130");
    expect(localFeaPacket.handoff_package.entity_ids.result_ids).toContain("result:force:pipe-P-120:axial");
    expect(localFeaPacket.handoff_package.entity_ids.result_ids).toContain(
      "result:stress:pipe-P-120:end-j:torsional-shear"
    );
    expect(localFeaPacket.handoff_package.units_manifest.dimension_basis).toBe("schemas/units.schema.yaml");
    expect(localFeaPacket.handoff_package.transfer_basis.transfer_method_label).toBe("result_reference_only");
    expect(localFeaPacket.handoff_package.guidance_assessment.labels).toContain("human_review_required");
    expect(localFeaPacket.handoff_package.guidance_assessment.labels).toContain(
      "global_to_local_transfer_inputs_incomplete"
    );
    expect(localFeaPacket.handoff_package.unsupported_behavior_flags.map((item: { behavior_label: string }) => item.behavior_label)).toContain(
      "target_format_not_selected"
    );
    expect(localFeaPacket.handoff_package.diagnostics.map((item: { code: string }) => item.code)).toContain(
      "LOCAL-FEA-EXTERNAL-SOLVER-NOT-INVOKED"
    );
    expect(localFeaPacket.handoff_package.privacy.local_only).toBe(true);
    expect(localFeaPacket.handoff_package.privacy.telemetry_allowed).toBe(false);
    expect(localFeaPacket.handoff_package.privacy.private_payload_embedded).toBe(false);
    expect(localFeaPacket.handoff_package.professional_boundary.software_makes_compliance_claim).toBe(false);
    expect(localFeaPacket.handoff_package.professional_boundary.software_makes_authentication_claim).toBe(false);
    const nativePackage = await screen.findByLabelText("Native JSON package");
    expect(within(nativePackage).getByTestId("native-package-summary").textContent).toContain("ready");
    expect(within(nativePackage).getByTestId("native-package-summary").textContent).toContain("members=9");
    expect(within(nativePackage).getByTestId("native-package-summary").textContent).toContain("entities=19");
    expect(within(nativePackage).getByTestId("native-package-summary").textContent).toContain("results=647");
    expect(within(nativePackage).getByTestId("native-package-profile").textContent).toContain(
      "native_open_json_preview"
    );
    expect(within(nativePackage).getByTestId("native-package-profile").textContent).toContain(
      "physical_container=TBD"
    );
    expect(within(nativePackage).getByTestId("native-package-members").textContent).toContain("manifest.json");
    expect(within(nativePackage).getByTestId("native-package-members").textContent).toContain(
      "maps/stable_id_map.json"
    );
    expect(within(nativePackage).getByTestId("native-package-validation").textContent).toContain(
      "review_manifest_complete"
    );
    await waitFor(() =>
      expect(within(nativePackage).getByTestId("native-package-validation").textContent).toContain(
        "model_hash=computed_local_preview_sha256"
      )
    );
    await waitFor(() =>
      expect(within(nativePackage).getByTestId("native-package-validation").textContent).toContain(
        "package_hash=computed_local_preview_sha256"
      )
    );
    expect(within(nativePackage).getByTestId("native-package-loss-report").textContent).toContain("1 TBD");
    expect(within(nativePackage).getByTestId("native-package-loss-report").textContent).toContain("1 unsupported");
    expect(within(nativePackage).getByTestId("native-package-storage").textContent).toContain("network=false");
    expect(within(nativePackage).getByTestId("native-package-storage").textContent).toContain(
      "repository_default_private_write=false"
    );
    expect(within(nativePackage).getByTestId("native-package-persisted-review-context").textContent).toContain(
      "editor_intents=0"
    );
    expect(within(nativePackage).getByTestId("native-package-persisted-review-context").textContent).toContain(
      "proposals=0"
    );
    expect(within(nativePackage).getByTestId("native-package-persisted-review-context").textContent).toContain(
      "selected_targets=0"
    );
    expect(within(nativePackage).getByTestId("native-package-persisted-review-context").textContent).toContain(
      "selected_ref=not_selected"
    );
    expect(within(nativePackage).getByTestId("native-package-persisted-review-context").textContent).toContain(
      "mechanics_results=0"
    );
    expect(within(nativePackage).getByTestId("native-package-persisted-review-context").textContent).toContain(
      "analysis_runs=0"
    );
    expect(within(nativePackage).getByTestId("native-package-persisted-review-context").textContent).toContain(
      "run_ref=not_persisted"
    );
    expect(within(nativePackage).getByTestId("native-package-boundary").textContent).toContain(
      "no private payload, protected content, release claim, compatibility claim, compliance claim, or professional approval claim"
    );
    const nativePackageHref = within(nativePackage).getByTestId("native-package-link").getAttribute("href") ?? "";
    const nativePackagePacket = JSON.parse(decodeURIComponent(nativePackageHref.split(",", 2)[1]));
    expect(nativePackagePacket.document_kind).toBe("openpipestress.technical_preview.native_json_package_review");
    expect(nativePackagePacket.deliverable_refs).toContain("DEL-17-02");
    expect(nativePackagePacket.deliverable_refs).toContain("DEL-17-03");
    expect(nativePackagePacket.deliverable_refs).toContain("DEL-02-05");
    expect(nativePackagePacket.deliverable_refs).toContain("DEL-12-01");
    expect(nativePackagePacket.scope_items).toContain("SOW-030");
    expect(nativePackagePacket.scope_items).toContain("SOW-074");
    expect(nativePackagePacket.scope_items).toContain("SOW-050");
    expect(nativePackagePacket.scope_items).toContain("SOW-029");
    expect(nativePackagePacket.export_profile.profile_id).toBe("native_open_json_preview");
    expect(nativePackagePacket.export_profile.physical_project_container).toBe("TBD");
    expect(nativePackagePacket.export_profile.public_transport_protocol).toBe("TBD");
    expect(nativePackagePacket.manifest.package_members).toHaveLength(9);
    expect(nativePackagePacket.manifest.package_members.map((item: { path: string }) => item.path)).toContain(
      "results/result_envelope_ref.json"
    );
    expect(nativePackagePacket.manifest.runtime_timestamp_fields_in_hash_inputs).toBe(false);
    expect(nativePackagePacket.stable_id_map.entity_ref_count).toBe(19);
    expect(nativePackagePacket.stable_id_map.result_ref_count).toBe(647);
    expect(nativePackagePacket.stable_id_map.operation_ref_count).toBe(0);
    expect(nativePackagePacket.stable_id_map.entity_refs).toContain("project:invented-loop-01");
    expect(nativePackagePacket.stable_id_map.result_refs).toContain("result:force:pipe-P-120:axial");
    expect(nativePackagePacket.source_project.storage_summary.editor_intent_count).toBe(0);
    expect(nativePackagePacket.source_project.storage_summary.proposal_count).toBe(0);
    expect(nativePackagePacket.source_project.storage_summary.selected_review_target_count).toBe(0);
    expect(nativePackagePacket.source_project.storage_summary.selected_review_target_ref).toBe("not_selected");
    expect(nativePackagePacket.source_project.storage_summary.persisted_mechanics_result_count).toBe(0);
    expect(nativePackagePacket.source_project.storage_summary.persisted_analysis_run_count).toBe(0);
    expect(nativePackagePacket.source_project.storage_summary.persisted_analysis_run_ref).toBe("not_persisted");
    expect(nativePackagePacket.loss_report.summary.unsupported_count).toBe(1);
    expect(nativePackagePacket.loss_report.summary.tbd_count).toBe(1);
    expect(nativePackagePacket.validation_report.package_shape_status).toBe("review_manifest_complete");
    expect(nativePackagePacket.validation_report.model_hash_status).toBe("computed_local_preview_sha256");
    expect(nativePackagePacket.validation_report.model_hash.value).toMatch(/^sha256:[0-9a-f]{64}$/);
    expect(nativePackagePacket.validation_report.model_hash.canonicalization).toBe("jcs_like_sorted_object_keys");
    expect(nativePackagePacket.validation_report.model_hash.payload_ref).toBe("project:invented-loop-01");
    expect(nativePackagePacket.manifest.source_model_version_or_hash_basis).toMatch(/^sha256:[0-9a-f]{64}$/);
    expect(
      nativePackagePacket.manifest.package_members.find(
        (item: { path: string }) => item.path === "model/project.json"
      ).hash_status
    ).toMatch(/^sha256:[0-9a-f]{64}$/);
    expect(nativePackagePacket.validation_report.package_hash_status).toBe("computed_local_preview_sha256");
    expect(nativePackagePacket.validation_report.package_hash.value).toMatch(/^sha256:[0-9a-f]{64}$/);
    expect(nativePackagePacket.validation_report.package_hash.value).not.toBe(
      nativePackagePacket.validation_report.model_hash.value
    );
    expect(nativePackagePacket.validation_report.package_hash.canonicalization).toBe("jcs_like_sorted_object_keys");
    expect(nativePackagePacket.validation_report.package_hash.payload_scope).toBe("package_review_payload");
    expect(nativePackagePacket.validation_report.package_hash.payload_excludes).toBe(
      "validation_report_package_hash_fields"
    );
    expect(nativePackagePacket.validation_report.package_hash.payload_ref).toBe(
      "native-json-preview:project:invented-loop-01"
    );
    expect(nativePackagePacket.loss_report.entries[2].affected_refs).not.toContain("canonical_package_hash");
    expect(nativePackagePacket.diagnostics).toHaveLength(7);
    expect(nativePackagePacket.generation_context.network_required).toBe(false);
    expect(nativePackagePacket.generation_context.telemetry_enabled).toBe(false);
    expect(nativePackagePacket.generation_context.persisted_editor_intent_count).toBe(0);
    expect(nativePackagePacket.generation_context.persisted_proposal_count).toBe(0);
    expect(nativePackagePacket.generation_context.persisted_selected_review_target_count).toBe(0);
    expect(nativePackagePacket.generation_context.persisted_selected_review_target_ref).toBe("not_selected");
    expect(nativePackagePacket.generation_context.persisted_mechanics_result_count).toBe(0);
    expect(nativePackagePacket.generation_context.persisted_analysis_run_count).toBe(0);
    expect(nativePackagePacket.generation_context.persisted_analysis_run_ref).toBe("not_persisted");
    expect(nativePackagePacket.generation_context.repository_default_private_write).toBe(false);
    expect(nativePackagePacket.run_refs.result_count).toBe(647);
    expect(nativePackagePacket.run_refs.hash_refs).toHaveLength(2);
    expect(nativePackagePacket.private_payload_included).toBe(false);
    expect(nativePackagePacket.protected_content_included).toBe(false);
    expect(nativePackagePacket.release_or_professional_claim).toBe(false);
    expect(nativePackagePacket.compatibility_claim_made).toBe(false);
    expect(nativePackagePacket.code_compliance_claim_made).toBe(false);
    expect(nativePackagePacket.professional_boundary.software_makes_compliance_claim).toBe(false);
    expect(solveJobPacket.events.map((item: { state: string }) => item.state)).toEqual([
      "queued",
      "running",
      "completed"
    ]);
    expect(solveJobPacket.private_payload_included).toBe(false);
    expect(solveJobPacket.protected_content_included).toBe(false);
    expect(solveJobPacket.release_or_professional_claim).toBe(false);
    expect(solveJobPacket.professional_boundary.software_makes_compliance_claim).toBe(false);
    const ruleCheck = await screen.findByLabelText("Rule-check completeness");
    expect(within(ruleCheck).getByTestId("rule-check-summary").textContent).toContain("5 review findings");
    expect(within(ruleCheck).getByTestId("rule-check-summary").textContent).toContain("rule_check_blocked=true");
    expect(within(ruleCheck).getByTestId("rule-check-summary").textContent).toContain(
      "mechanics_reviewable=true"
    );
    expect(within(ruleCheck).getByTestId("rule-check-status").textContent).toContain("RULE_INPUTS_INCOMPLETE");
    expect(within(ruleCheck).getByTestId("rule-check-mechanics-status").textContent).toContain("MECHANICS_SOLVED");
    expect(within(ruleCheck).getByTestId("rule-check-boundary").textContent).toContain(
      "bundled code values=false"
    );
    expect(
      within(ruleCheck).getByTestId("rule-check-finding-professional-acceptance-not-provided").textContent
    ).toContain("ASSUMPTION_WARNING");
    const ruleCheckHref = within(ruleCheck).getByTestId("rule-check-export-link").getAttribute("href") ?? "";
    const ruleCheckPacket = JSON.parse(decodeURIComponent(ruleCheckHref.split(",", 2)[1]));
    expect(ruleCheckPacket.document_kind).toBe("openpipestress.technical_preview.rule_completeness_review");
    expect(ruleCheckPacket.deliverable_refs).toContain("DEL-06-03");
    expect(ruleCheckPacket.deliverable_refs).toContain("DEL-07-04");
    expect(ruleCheckPacket.scope_items).toContain("SOW-004");
    expect(ruleCheckPacket.scope_items).toContain("SOW-022");
    expect(ruleCheckPacket.run_ref).toBe("run:preview-linear-static-001");
    expect(ruleCheckPacket.rule_check_status).toBe("RULE_INPUTS_INCOMPLETE");
    expect(ruleCheckPacket.summary.finding_count).toBe(5);
    expect(ruleCheckPacket.summary.rule_check_blocked).toBe(true);
    expect(ruleCheckPacket.summary.mechanics_results_reviewable).toBe(true);
    expect(ruleCheckPacket.summary.silent_defaults_used).toBe(false);
    expect(ruleCheckPacket.summary.bundled_code_values_used).toBe(false);
    expect(ruleCheckPacket.summary.compliance_claim_made).toBe(false);
    expect(ruleCheckPacket.findings.map((item: { warning_class: string }) => item.warning_class)).toContain(
      "RULE_CHECK_BLOCKING"
    );
    expect(ruleCheckPacket.findings.map((item: { warning_class: string }) => item.warning_class)).toContain(
      "PROVENANCE_WARNING"
    );
    expect(ruleCheckPacket.findings.map((item: { warning_class: string }) => item.warning_class)).toContain(
      "ASSUMPTION_WARNING"
    );
    expect(
      ruleCheckPacket.findings.every(
        (item: {
          protected_content_required: boolean;
          mechanics_solve_blocking: boolean;
        }) => item.protected_content_required === false && item.mechanics_solve_blocking === false
      )
    ).toBe(true);
    expect(ruleCheckPacket.private_payload_included).toBe(false);
    expect(ruleCheckPacket.protected_content_included).toBe(false);
    expect(ruleCheckPacket.release_or_professional_claim).toBe(false);
    expect(ruleCheckPacket.professional_boundary.software_makes_compliance_claim).toBe(false);
    const runAudit = await screen.findByLabelText("Run audit");
    expect(await within(runAudit).findByTestId("run-audit-model-state")).toHaveTextContent(
      "ModelState; state:project:invented-loop-01:preview"
    );
    expect(within(runAudit).getByTestId("run-audit-analysis-run").textContent).toContain("DEL-14-02");
    expect(within(runAudit).getByTestId("run-audit-analysis-run").textContent).toContain("mechanics_solve");
    expect(within(runAudit).getByTestId("run-audit-analysis-run").textContent).toContain(
      "run:preview-linear-static-001"
    );
    expect(within(runAudit).getByTestId("run-audit-status").textContent).toContain("HUMAN_REVIEW_REQUIRED");
    expect(within(runAudit).getByTestId("run-audit-status").textContent).toContain("MECHANICS_SOLVED");
    expect(within(runAudit).getByTestId("run-audit-status").textContent).toContain("RULE_INPUTS_INCOMPLETE");
    expect(within(runAudit).getByTestId("run-audit-hashes").textContent).toContain(
      "647 result rows; 647 result value hashes"
    );
    expect(within(runAudit).getByTestId("run-audit-hashes").textContent).toContain("analysis_run_record");
    expect(within(runAudit).getByTestId("run-audit-hashes").textContent).toContain("result_envelope");
    expect(within(runAudit).getByTestId("run-audit-input-manifest").textContent).toContain(
      "ResultEnvelope:result-envelope:run:preview-linear-static-001"
    );
    expect(within(runAudit).getByTestId("run-audit-reproducibility").textContent).toContain(
      "physical project container"
    );
    expect(within(runAudit).getByTestId("run-audit-reproducibility").textContent).toContain(
      "release-grade solver build provenance"
    );
    expect(within(runAudit).getByTestId("run-audit-immutability").textContent).toContain("read-only run record");
    expect(within(runAudit).getByTestId("run-audit-immutability").textContent).toContain(
      "changes_create_new_analysis_run"
    );
    expect(within(runAudit).getByTestId("run-audit-boundary").textContent).toContain(
      "no compliance, certification, sealing, authentication, or approval claim"
    );
    const comparison = await screen.findByLabelText("Comparison workspace");
    expect(within(comparison).getByTestId("comparison-summary").textContent).toContain("load:L-100; 217 rows");
    expect(within(comparison).getByTestId("comparison-summary").textContent).toContain(
      "combination:C-OPER-ALT; 213 rows"
    );
    expect(within(comparison).getByTestId("comparison-summary").textContent).toContain(
      "213 comparable pairs; 4 reference-only; 0 target-only"
    );
    expect(within(comparison).getByTestId("comparison-tolerance-status").textContent).toContain(
      "not_tolerance_checked"
    );
    expect(within(comparison).getByTestId("comparison-mapping-basis").textContent).toContain("source_result_refs");
    expect(within(comparison).getByTestId("comparison-boundary").textContent).toContain(
      "no compliance, certification, sealing, authentication, or approval claim"
    );
    const comparisonRow = within(comparison).getByTestId(
      "comparison-row-result:combination:combination-C-OPER-ALT:moment:pipe-P-120:bending-z"
    );
    expect(comparisonRow.textContent).toContain("459.8 N*m");
    expect(within(comparison).getByTestId("comparison-diagnostics").textContent).toContain(
      "1 comparison diagnostic"
    );
    fireEvent.click(
      within(comparison).getByTestId(
        "comparison-select-result:combination:combination-C-OPER-ALT:moment:pipe-P-120:bending-z"
      )
    );
    expect(await screen.findByRole("heading", { name: "Rack span" })).toBeInTheDocument();
    expect(within(comparison).getByTestId("comparison-summary").textContent).toContain("213 comparable pairs");
    const results = await screen.findByLabelText("Results");
    expect(within(results).getByTestId("result-group-displacement")).toBeInTheDocument();
    expect(within(results).getByTestId("result-group-force")).toBeInTheDocument();
    expect(within(results).getByTestId("result-group-moment")).toBeInTheDocument();
    expect(within(results).getByTestId("result-group-stress")).toBeInTheDocument();
    expect(within(results).getByTestId("result-row-result:force:pipe-P-100:axial")).toBeInTheDocument();
    expect(within(results).getByText("result:disp:node-N-140")).toBeInTheDocument();
    expect(within(results).getByText(/26.515341 mm/i)).toBeInTheDocument();
    expect(within(results).getByTestId("result-filter-summary").textContent).toContain(
      "647 of 647 results match filter"
    );
    expect(within(results).getByTestId("result-page-summary").textContent).toContain(
      "Showing 1 to 50 of 647 matching results; page 1 of 13"
    );
    expect(within(results).getByTestId("previous-result-page")).toBeDisabled();
    expect(within(results).getByTestId("next-result-page")).not.toBeDisabled();

    fireEvent.click(within(results).getByTestId("next-result-page"));
    expect(within(results).getByTestId("result-page-summary").textContent).toContain(
      "Showing 51 to 100 of 647 matching results; page 2 of 13"
    );
    expect(within(results).getByTestId("result-row-result:force:pipe-P-110:axial")).toBeInTheDocument();
    fireEvent.click(within(results).getByTestId("previous-result-page"));
    expect(within(results).getByTestId("result-page-summary").textContent).toContain(
      "Showing 1 to 50 of 647 matching results; page 1 of 13"
    );

    fireEvent.change(within(results).getByTestId("result-filter-input"), { target: { value: "torsional-shear" } });
    expect(within(results).getByTestId("result-filter-summary").textContent).toContain(
      "60 of 647 results match filter"
    );
    expect(within(results).getByTestId("result-page-summary").textContent).toContain(
      "Showing 1 to 50 of 60 matching results; page 1 of 2"
    );
    expect(within(results).getByTestId("result-group-stress")).toBeInTheDocument();
    expect(within(results).queryByTestId("result-group-force")).not.toBeInTheDocument();
    expect(within(results).getByTestId("result-row-result:stress:pipe-P-120:end-j:torsional-shear")).toBeInTheDocument();

    fireEvent.click(within(results).getByTestId("clear-result-filter"));
    expect(within(results).getByTestId("result-filter-summary").textContent).toContain(
      "647 of 647 results match filter"
    );
    expect(within(results).getByTestId("result-group-force")).toBeInTheDocument();

    fireEvent.change(within(results).getByTestId("result-filter-input"), { target: { value: "pipe-P-120" } });
    expect(within(results).getByTestId("result-filter-summary").textContent).toContain(
      "167 of 647 results match filter"
    );
    expect(within(results).getByTestId("result-page-summary").textContent).toContain(
      "Showing 1 to 50 of 167 matching results; page 1 of 4"
    );
    expect(within(results).getByTestId("result-row-result:force:pipe-P-120:axial")).toBeInTheDocument();
    expect(within(results).getByTestId("result-row-result:force:pipe-P-120:axial:end-j")).toBeInTheDocument();
    expect(within(results).getByTestId("result-row-result:force:pipe-P-120:shear-y")).toBeInTheDocument();
    expect(within(results).getByTestId("result-row-result:force:pipe-P-120:shear-y:end-j")).toBeInTheDocument();
    expect(within(results).getByTestId("result-row-result:force:pipe-P-120:quarter-1:shear-y")).toBeInTheDocument();
    expect(within(results).getByTestId("result-row-result:force:pipe-P-120:midspan:axial")).toBeInTheDocument();
    expect(within(results).getByTestId("result-row-result:force:pipe-P-120:midspan:shear-z")).toBeInTheDocument();
    expect(within(results).getByTestId("result-row-result:force:pipe-P-120:quarter-3:shear-z")).toBeInTheDocument();
    expect(within(results).getByTestId("result-row-result:moment:pipe-P-120:quarter-1:bending-z")).toBeInTheDocument();
    expect(within(results).getByTestId("result-row-result:stress:pipe-P-120:end-j:torsional-shear")).toBeInTheDocument();
    expect(within(results).getByTestId("result-row-result:stress:pipe-P-120:quarter-1:torsional-shear")).toBeInTheDocument();
    expect(within(results).getByTestId("result-row-result:stress:pipe-P-120:midspan:torsional-shear")).toBeInTheDocument();

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

    fireEvent.change(within(results).getByTestId("result-filter-input"), {
      target: { value: "combination-C-OPER-ALT:force:pipe-P-120" }
    });
    expect(within(results).getByTestId("result-filter-summary").textContent).toContain(
      "15 of 647 results match filter"
    );
    expect(within(results).getByTestId("result-row-result:combination:combination-C-OPER-ALT:force:pipe-P-120:axial")).toBeInTheDocument();
    expect(
      within(results).getByTestId(
        "result-row-result:combination:combination-C-OPER-ALT:force:pipe-P-120:quarter-1:shear-y"
      )
    ).toBeInTheDocument();
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

    fireEvent.change(within(results).getByTestId("result-filter-input"), {
      target: { value: "stress:pipe-P-120:end-j:torsional-shear" }
    });
    expect(within(results).getByTestId("result-filter-summary").textContent).toContain(
      "3 of 647 results match filter"
    );
    expect(within(results).getByTestId("result-row-result:stress:pipe-P-120:end-j:torsional-shear")).toBeInTheDocument();
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
    expect(within(report).getByTestId("report-selected-review-target").textContent).toContain(
      "result: result:stress:pipe-P-120:end-j:torsional-shear"
    );
    expect(within(report).getByTestId("report-analysis-run").textContent).toContain("DEL-14-02");
    expect(within(report).getByTestId("report-analysis-run").textContent).toContain("run:preview-linear-static-001");
    expect(within(report).getByTestId("report-load-basis-refs").textContent).toContain("load:L-100");
    expect(within(report).getByTestId("report-load-basis-refs").textContent).toContain("load:L-200");
    expect(within(report).getByTestId("report-load-basis-refs").textContent).toContain("combination:C-OPER-ALT");
    expect(within(report).getByTestId("report-diagnostic-scope").textContent).toContain(
      "model, design knowledge, and computed mechanics findings"
    );
    expect(within(report).getByText(/9 review findings/i)).toBeInTheDocument();
    expect(within(report).getByTestId("report-diagnostic-summary").textContent).toContain(
      "8 warnings; 1 info; 0 errors; 0 blocking"
    );
    expect(within(report).getByText(/result value hashes/i)).toBeInTheDocument();
    expect(within(report).getByText(/result_envelope/i)).toBeInTheDocument();
    expect(within(report).getByText(/no compliance or professional approval claim/i)).toBeInTheDocument();
    expect(within(report).getByTestId("report-comparison-summary").textContent).toContain(
      "213 mapped pairs"
    );
    expect(within(report).getByTestId("report-comparison-summary").textContent).toContain(
      "not_tolerance_checked"
    );
    expect(within(report).getByTestId("report-project-persistence").textContent).toContain(
      "storage=not_persisted_this_session"
    );
    expect(within(report).getByTestId("report-project-persistence").textContent).toContain(
      "validation=preview_not_persisted"
    );
    expect(within(report).getByTestId("report-export-readiness").textContent).toContain(
      "26 of 27 local exports ready"
    );
    expect(within(report).getByTestId("report-export-readiness").textContent).toContain("storage=available");
    expect(within(report).getByTestId("report-export-readiness").textContent).toContain("validation=available");
    expect(within(report).getByTestId("report-export-readiness").textContent).toContain("telemetry=available");
    expect(within(report).getByTestId("report-export-readiness").textContent).toContain("secrets=available");
    expect(within(report).getByTestId("report-export-readiness").textContent).toContain("threats=available");
    expect(within(report).getByTestId("report-export-readiness").textContent).toContain("accessibility=available");
    expect(within(report).getByTestId("report-export-readiness").textContent).toContain("workspace=available");
    expect(within(report).getByTestId("report-export-readiness").textContent).toContain("evidence=available");
    expect(within(report).getByTestId("report-storage-boundary").textContent).toContain("network=false");
    expect(within(report).getByTestId("report-storage-boundary").textContent).toContain(
      "private/protected payload=false"
    );
    expect(within(report).getByTestId("report-storage-boundary").textContent).toContain(
      "accepted_state_mutated=false"
    );
    expect(within(report).getByTestId("report-export-summary").textContent).toContain("no private payload");
    expect(within(report).getByTestId("report-export-summary").textContent).toContain("9 diagnostics");
    const exportHref = within(report).getByTestId("report-export-link").getAttribute("href") ?? "";
    expect(exportHref).toContain("data:application/json");
    const exportPacket = JSON.parse(decodeURIComponent(exportHref.split(",", 2)[1]));
    expect(exportPacket.document_kind).toBe("openpipestress.technical_preview.report_packet_export");
    expect(exportPacket.export_scope).toBe("local_browser_download_preview");
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
    expect(exportPacket.selected_result_refs).toContain("result:force:pipe-P-120:axial");
    expect(exportPacket.selected_result_refs).toContain("result:stress:pipe-P-120:end-j:torsional-shear");
    expect(exportPacket.selected_review_target).toEqual({
      target_type: "result",
      id: "result:stress:pipe-P-120:end-j:torsional-shear"
    });
    expect(exportPacket.diagnostic_refs).toContain("RULE_INPUTS_MISSING");
    expect(exportPacket.diagnostic_refs).toContain("SUPPORT_STIFFNESS_UNRESOLVED");
    expect(exportPacket.diagnostic_refs).toContain("diagnostic:physics:high-displacement-review");
    expect(exportPacket.diagnostic_summary.total).toBe(9);
    expect(exportPacket.diagnostic_summary.by_severity.warning).toBe(8);
    expect(exportPacket.diagnostic_summary.by_severity.info).toBe(1);
    expect(exportPacket.load_basis_refs).toContain("combination:C-OPER-ALT");
    expect(exportPacket.hash_refs.envelope_hash_scopes).toContain("result_envelope");
    expect(exportPacket.persistence_evidence.document_kind).toBe(
      "openpipestress.technical_preview.report_persistence_export_context"
    );
    expect(exportPacket.persistence_evidence.storage_audit.document_kind).toBe(
      "openpipestress.technical_preview.local_project_persistence_audit"
    );
    expect(exportPacket.persistence_evidence.storage_audit.storage_mode).toBe("not_persisted_this_session");
    expect(exportPacket.persistence_evidence.storage_audit.pending_operation_count).toBe(0);
    expect(exportPacket.persistence_evidence.validation_preflight.document_kind).toBe(
      "openpipestress.technical_preview.project_validation_preflight"
    );
    expect(exportPacket.persistence_evidence.validation_preflight.validation_status).toBe(
      "preview_not_persisted"
    );
    expect(exportPacket.persistence_evidence.validation_preflight.round_trip_status).toBe(
      "semantic_categories_declared"
    );
    expect(exportPacket.persistence_evidence.export_inventory.expected_export_count).toBe(27);
    expect(exportPacket.persistence_evidence.export_inventory.available_count).toBe(26);
    expect(exportPacket.persistence_evidence.export_inventory.readiness_by_export_id.project_storage_audit).toBe(
      "available"
    );
    expect(
      exportPacket.persistence_evidence.export_inventory.readiness_by_export_id.project_validation_preflight
    ).toBe("available");
    expect(exportPacket.persistence_evidence.export_inventory.readiness_by_export_id.telemetry_boundary_review).toBe(
      "available"
    );
    expect(
      exportPacket.persistence_evidence.export_inventory.readiness_by_export_id
        .secret_private_library_boundary_review
    ).toBe("available");
    expect(
      exportPacket.persistence_evidence.export_inventory.readiness_by_export_id.security_threat_model_review
    ).toBe("available");
    expect(exportPacket.persistence_evidence.export_inventory.readiness_by_export_id.editor_contract_review).toBe(
      "available"
    );
    expect(
      exportPacket.persistence_evidence.export_inventory.readiness_by_export_id.missing_data_warning_blocking_review
    ).toBe("available");
    expect(
      exportPacket.persistence_evidence.export_inventory.readiness_by_export_id
        .accessibility_usability_baseline_review
    ).toBe("available");
    expect(
      exportPacket.persistence_evidence.export_inventory.readiness_by_export_id
        .design_authoring_comparison_workspace
    ).toBe("available");
    expect(exportPacket.persistence_evidence.export_inventory.readiness_by_export_id.build_package_readiness).toBe(
      "available"
    );
    expect(
      exportPacket.persistence_evidence.export_inventory.readiness_by_export_id
        .validation_release_evidence_review
    ).toBe("available");
    expect(
      exportPacket.persistence_evidence.export_inventory.readiness_by_export_id.report_protected_content_lint
    ).toBe("available");
    expect(
      exportPacket.persistence_evidence.export_inventory.readiness_by_export_id.stress_neutral_csv_json_package
    ).toBe("available");
    expect(exportPacket.persistence_evidence.export_inventory.readiness_by_export_id.headless_runner_envelope).toBe(
      "available"
    );
    expect(exportPacket.persistence_evidence.export_inventory.readiness_by_export_id.adapter_framework_envelope).toBe(
      "available"
    );
    expect(exportPacket.persistence_evidence.export_inventory.readiness_by_export_id.local_fea_handoff_package).toBe(
      "available"
    );
    expect(
      exportPacket.persistence_evidence.export_inventory.readiness_by_export_id.external_prover_boundary_metadata
    ).toBe("available");
    expect(exportPacket.persistence_evidence.export_inventory.readiness_by_export_id.review_geometry_export).toBe(
      "available"
    );
    expect(exportPacket.persistence_evidence.export_inventory.readiness_by_export_id.conservative_pcf_export).toBe(
      "available"
    );
    expect(exportPacket.persistence_evidence.export_inventory.readiness_by_export_id.caepipe_mbf_export).toBe(
      "available"
    );
    expect(exportPacket.persistence_evidence.export_inventory.readiness_by_export_id.caepipe_external_run_evidence).toBe(
      "available"
    );
    expect(exportPacket.persistence_evidence.export_inventory.readiness_by_export_id.export_adapter_sdk_registry).toBe(
      "available"
    );
    expect(exportPacket.persistence_evidence.export_inventory.readiness_by_export_id.operation_review_ledger).toBe(
      "empty_operation_queue"
    );
    expect(exportPacket.persistence_evidence.boundary.network_required).toBe(false);
    expect(exportPacket.persistence_evidence.boundary.telemetry_enabled).toBe(false);
    expect(exportPacket.persistence_evidence.boundary.private_payload_included).toBe(false);
    expect(exportPacket.persistence_evidence.boundary.protected_content_included).toBe(false);
    expect(exportPacket.persistence_evidence.boundary.release_or_professional_claim).toBe(false);
    const reportLint = await screen.findByLabelText("Report content lint");
    expect(within(reportLint).getByTestId("report-lint-summary").textContent).toContain("targets=22");
    expect(within(reportLint).getByTestId("report-lint-summary").textContent).toContain("findings=0");
    expect(within(reportLint).getByTestId("report-lint-summary").textContent).toContain("blocking=0");
    expect(within(reportLint).getByTestId("report-lint-clean-scan").textContent).toContain(
      "clearance=false"
    );
    const reportLintHref = within(reportLint).getByTestId("report-lint-export-link").getAttribute("href") ?? "";
    const reportLintPacket = JSON.parse(decodeURIComponent(reportLintHref.split(",", 2)[1]));
    expect(reportLintPacket.deliverable_id).toBe("DEL-08-05");
    expect(reportLintPacket.lint_run.run_id).toBe("lint:report-preview:run-preview-linear-static-001");
    expect(reportLintPacket.lint_run.summary.target_count).toBe(22);
    expect(reportLintPacket.lint_run.summary.scanned_target_count).toBe(22);
    expect(reportLintPacket.lint_run.summary.finding_count).toBe(0);
    expect(reportLintPacket.lint_run.summary.blocking_finding_count).toBe(0);
    expect(reportLintPacket.lint_run.summary.clean_scan_is_clearance).toBe(false);
    expect(
      reportLintPacket.lint_run.targets.map((item: { target_id: string }) => item.target_id)
    ).toContain("target:report-packet-preview-json");
    expect(
      reportLintPacket.lint_run.targets.every(
        (item: { privacy_classification: string }) => item.privacy_classification !== "private_project_data"
      )
    ).toBe(true);
    expect(exportPacket.run_audit.model_state_ref.ref).toBe("state:project:invented-loop-01:preview");
    expect(exportPacket.run_audit.analysis_run_ref.ref).toBe("run:preview-linear-static-001");
    expect(exportPacket.run_audit.run_kind).toBe("mechanics_solve");
    expect(exportPacket.run_audit.analysis_status).toContain("HUMAN_REVIEW_REQUIRED");
    expect(exportPacket.run_audit.analysis_status).toContain("MECHANICS_SOLVED");
    expect(exportPacket.run_audit.analysis_status).toContain("RULE_INPUTS_INCOMPLETE");
    expect(exportPacket.run_audit.result_row_count).toBe(647);
    expect(exportPacket.run_audit.result_ref_count).toBe(647);
    expect(exportPacket.run_audit.result_value_hash_count).toBe(647);
    expect(exportPacket.run_audit.hash_scopes).toContain("analysis_run_record");
    expect(exportPacket.run_audit.hash_scopes).toContain("result_envelope");
    expect(exportPacket.run_audit.input_manifest_refs[0].ref).toBe(
      "result-envelope:run:preview-linear-static-001"
    );
    expect(exportPacket.run_audit.unresolved_tbd).toContain("physical project container");
    expect(exportPacket.run_audit.unresolved_tbd).toContain("release-grade solver build provenance");
    expect(exportPacket.run_audit.immutability_policy.run_record_is_read_only).toBe(true);
    expect(exportPacket.run_audit.immutability_policy.new_run_required_for_change).toBe(true);
    expect(exportPacket.run_audit.professional_boundary.software_makes_compliance_claim).toBe(false);
    expect(exportPacket.run_audit.professional_boundary.software_makes_authentication_claim).toBe(false);
    expect(exportPacket.comparison_ref).toContain("comparison:run:preview-linear-static-001");
    expect(exportPacket.comparison_summary.comparable_result_pairs).toBe(213);
    expect(exportPacket.comparison_summary.unmatched_left_results).toBe(4);
    expect(exportPacket.comparison_summary.unmatched_right_results).toBe(0);
    expect(exportPacket.comparison_summary.tolerance_status).toBe("not_tolerance_checked");
    expect(exportPacket.comparison_summary.tolerance_profile_ref).toBe("TBD");
    expect(exportPacket.comparison_summary.release_or_professional_claim).toBe(false);
    expect(exportPacket.comparison_top_deltas[0].classification).toBe("not_tolerance_checked");
    expect(exportPacket.comparison_professional_boundary.software_makes_compliance_claim).toBe(false);
    expect(exportPacket.data_boundary.private_data_policy).toBe("no_private_project_data");
    expect(exportPacket.private_payload_included).toBe(false);
    expect(exportPacket.protected_content_included).toBe(false);
    expect(exportPacket.release_or_professional_claim).toBe(false);
    expect(exportPacket.proposal_operation).toBeNull();

    const solvedExternalProver = await screen.findByLabelText("External prover boundary metadata");
    expect(within(solvedExternalProver).getByTestId("external-prover-summary").textContent).toContain(
      "metadata=non_authoritative_workflow_metadata"
    );
    expect(within(solvedExternalProver).getByTestId("external-prover-context-links").textContent).toContain(
      "state=1"
    );
    expect(within(solvedExternalProver).getByTestId("external-prover-run-boundary").textContent).toContain(
      "commercial_results=false"
    );
    const solvedExternalHref =
      within(solvedExternalProver).getByTestId("external-prover-export-link").getAttribute("href") ?? "";
    const solvedExternalPacket = JSON.parse(decodeURIComponent(solvedExternalHref.split(",", 2)[1]));
    expect(solvedExternalPacket.tags).toContain("mechanics-context-bound");
    expect(solvedExternalPacket.handoff_package_refs[0].ref.ref).toContain("run:preview-linear-static-001");
    expect(solvedExternalPacket.immutable_model_state_refs[0].ref.ref).toBe("state:project:invented-loop-01:preview");
    expect(solvedExternalPacket.external_references[0].hash_refs[0].algorithm).toBe("sha256");
    expect(solvedExternalPacket.professional_boundary.external_tool_invoked).toBe(false);
    expect(solvedExternalPacket.professional_boundary.commercial_result_payload_ingested).toBe(false);
    expect(solvedExternalPacket.professional_boundary.software_creates_professional_reliance_record).toBe(false);

    const handoff = await screen.findByLabelText("Handoff package");
    const handoffExportSummary = await within(handoff).findByTestId("handoff-export-summary");
    expect(handoffExportSummary.textContent).toContain("19 entities");
    expect(handoffExportSummary.textContent).toContain("9 diagnostics");
    expect(within(handoff).getByTestId("handoff-target-profile").textContent).toContain("native_open_json_preview");
    expect(within(handoff).getByTestId("handoff-stable-ids").textContent).toContain("647 result refs");
    expect(within(handoff).getByTestId("handoff-loss-report").textContent).toContain(
      "target-specific commercial-solver field mapping is not generated"
    );
    expect(within(handoff).getByTestId("handoff-boundary").textContent).toContain("no private payload");
    const handoffHref = within(handoff).getByTestId("handoff-export-link").getAttribute("href") ?? "";
    expect(handoffHref).toContain("data:application/json");
    const handoffPacket = JSON.parse(decodeURIComponent(handoffHref.split(",", 2)[1]));
    expect(handoffPacket.document_kind).toBe("openpipestress.technical_preview.handoff_package");
    expect(handoffPacket.export_scope).toBe("local_browser_download_preview");
    expect(handoffPacket.deliverable_refs).toContain("DEL-15-01");
    expect(handoffPacket.deliverable_refs).toContain("DEL-17-03");
    expect(handoffPacket.model_state_ref.ref).toBe("state:project:invented-loop-01:preview");
    expect(handoffPacket.analysis_run_ref.ref).toBe("run:preview-linear-static-001");
    expect(handoffPacket.units_manifest.length).toBe("m");
    expect(handoffPacket.stable_id_map.entity_ref_count).toBe(19);
    expect(handoffPacket.stable_id_map.entity_refs).toContain("material:invented-carbon-steel");
    expect(handoffPacket.stable_id_map.result_ref_count).toBe(647);
    expect(handoffPacket.stable_id_map.selected_result_refs).toContain("result:force:pipe-P-120:axial");
    expect(handoffPacket.library_refs.private_library_payload_included).toBe(false);
    expect(handoffPacket.target_mapping.mapping_status).toBe("stable_ids_only_not_target_specific");
    expect(handoffPacket.target_mapping.target_field_coverage).toBe("TBD");
    expect(handoffPacket.run_audit_refs.hash_scopes).toContain("result_envelope");
    expect(handoffPacket.comparison_ref).toContain("comparison:run:preview-linear-static-001");
    expect(handoffPacket.comparison_summary.comparable_result_pairs).toBe(213);
    expect(handoffPacket.diagnostic_refs).toContain("diagnostic:physics:high-displacement-review");
    expect(handoffPacket.unresolved_assumptions).toContain("target-specific field coverage TBD");
    expect(handoffPacket.loss_report.unsupported_behavior_refs).toContain(
      "professional validation and acceptance are not software-generated"
    );
    expect(handoffPacket.private_payload_included).toBe(false);
    expect(handoffPacket.protected_content_included).toBe(false);
    expect(handoffPacket.release_or_professional_claim).toBe(false);
    expect(handoffPacket.professional_boundary.software_makes_compliance_claim).toBe(false);

    fireEvent.click(screen.getByRole("button", { name: /Generate review proposal/i }));
    const proposal = await screen.findByLabelText("Agentic proposal");
    expect(await within(proposal).findByText("proposal:physics-diagnostic-review")).toBeInTheDocument();
    expect(within(proposal).getByTestId("selected-review-target").textContent).toContain(
      "result: result:stress:pipe-P-120:end-j:torsional-shear"
    );
    expect(within(proposal).getAllByText(/result:stress:pipe-P-120:end-j:torsional-shear/i).length).toBeGreaterThan(0);
    expect(within(proposal).getByText(/review-only and does not mutate accepted model state/i)).toBeInTheDocument();
    expect(within(proposal).getByTestId("proposal-operation-summary").textContent).toContain(
      "op:review-computed-diagnostic"
    );
    expect(within(proposal).getByTestId("proposal-operation-summary").textContent).toContain(
      "attach_design_knowledge"
    );
    expect(within(proposal).getByTestId("proposal-operation-summary").textContent).toContain(
      "draft_user_review_required"
    );
    expect(within(proposal).getByTestId("proposal-affected-entities").textContent).toContain(
      "result:stress:pipe-P-120:end-j:torsional-shear"
    );
    expect(within(proposal).getByTestId("proposal-validation-status").textContent).toContain(
      "warning_computed_context_requires_human_review"
    );
    expect(within(proposal).getByTestId("proposal-validation-status").textContent).toContain(
      "generated_from_computed_context"
    );
    expect(within(proposal).getByTestId("proposal-validation-status").textContent).toContain("not_applied");
    expect(within(proposal).getByTestId("proposal-audit-boundary").textContent).toContain(
      "requires user acceptance"
    );
    expect(within(proposal).getByTestId("proposal-audit-boundary").textContent).toContain("true");
    expect(within(proposal).getByTestId("proposal-audit-boundary").textContent).toContain(
      "mutates accepted model state"
    );
    expect(within(proposal).getByTestId("proposal-audit-boundary").textContent).toContain("false");
    expect(within(proposal).getByTestId("proposal-professional-boundary").textContent).toContain(
      "human review required"
    );
    expect(within(proposal).getByTestId("proposal-professional-boundary").textContent).toContain(
      "software makes approval claim"
    );
    expect(within(proposal).getByTestId("proposal-professional-boundary").textContent).toContain("false");
    expect(within(proposal).getByRole("button", { name: /Accept disabled/i })).toBeDisabled();

    expect(await within(report).findByText("proposal:physics-diagnostic-review")).toBeInTheDocument();
    expect(within(report).getByTestId("report-proposal-operation").textContent).toContain(
      "op:review-computed-diagnostic"
    );
    expect(within(report).getByTestId("report-proposal-operation").textContent).toContain(
      "attach_design_knowledge"
    );
    expect(within(report).getByTestId("report-proposal-operation").textContent).toContain(
      "draft_user_review_required"
    );
    expect(within(report).getByTestId("report-proposal-operation").textContent).toContain("not_applied");
    expect(within(report).getByTestId("report-proposal-boundary").textContent).toContain(
      "review-only; requires user acceptance"
    );
    expect(within(report).getByTestId("report-proposal-boundary").textContent).toContain(
      "does not mutate accepted model state"
    );
    expect(within(report).getByTestId("report-proposal-boundary").textContent).toContain(
      "no compliance or professional approval claim"
    );

    const proposalExportHref = within(report).getByTestId("report-export-link").getAttribute("href") ?? "";
    const proposalExportPacket = JSON.parse(decodeURIComponent(proposalExportHref.split(",", 2)[1]));
    expect(proposalExportPacket.proposal_ref).toBe("proposal:physics-diagnostic-review");
    expect(proposalExportPacket.selected_review_target).toEqual({
      target_type: "result",
      id: "result:stress:pipe-P-120:end-j:torsional-shear"
    });
    expect(proposalExportPacket.proposal_operation.operation_id).toBe("op:review-computed-diagnostic");
    expect(proposalExportPacket.proposal_operation.operation_kind).toBe("attach_design_knowledge");
    expect(proposalExportPacket.proposal_operation.operation_status).toBe("draft_user_review_required");
    expect(proposalExportPacket.proposal_operation.affected_entity_ids).toContain(
      "result:stress:pipe-P-120:end-j:torsional-shear"
    );
    expect(proposalExportPacket.proposal_operation.validation.application_status).toBe("not_applied");
    expect(proposalExportPacket.proposal_operation.validation.diff_preview_status).toBe(
      "generated_from_computed_context"
    );
    expect(proposalExportPacket.proposal_operation.audit_boundary.requires_user_acceptance).toBe(true);
    expect(proposalExportPacket.proposal_operation.audit_boundary.mutates_accepted_model_state).toBe(false);
    expect(proposalExportPacket.proposal_operation.audit_boundary.acceptance_recorded_as_review_only).toBe(true);
    expect(proposalExportPacket.proposal_operation.professional_boundary.human_review_required).toBe(true);
    expect(proposalExportPacket.proposal_operation.professional_boundary.software_makes_compliance_claim).toBe(false);
    expect(proposalExportPacket.proposal_operation.professional_boundary.software_makes_approval_claim).toBe(false);
    expect(proposalExportPacket.persistence_evidence.storage_audit.pending_operation_count).toBe(1);
    expect(proposalExportPacket.persistence_evidence.storage_audit.editor_intent_count).toBe(0);
    expect(proposalExportPacket.persistence_evidence.storage_audit.proposal_operation_count).toBe(1);
    expect(screen.getByTestId("local-project-review-context").textContent).toContain(
      "1 pending operation; applied=false; editor_intents=0; agent_proposals=1"
    );

    const proposalStorageAudit = await screen.findByLabelText("Project storage audit");
    expect(within(proposalStorageAudit).getByTestId("project-storage-summary").textContent).toContain(
      "pending operations=1"
    );
    expect(within(proposalStorageAudit).getByTestId("project-storage-summary").textContent).toContain(
      "proposals=1"
    );
    const proposalStorageHref =
      within(proposalStorageAudit).getByTestId("project-storage-export-link").getAttribute("href") ?? "";
    const proposalStoragePacket = JSON.parse(decodeURIComponent(proposalStorageHref.split(",", 2)[1]));
    expect(proposalStoragePacket.summary.pending_operation_count).toBe(1);
    expect(proposalStoragePacket.summary.editor_intent_count).toBe(0);
    expect(proposalStoragePacket.summary.proposal_operation_count).toBe(1);
    expect(proposalStoragePacket.proposal_refs).toContain("proposal:physics-diagnostic-review");
    expect(proposalStoragePacket.review_operation_statuses).toContain("not_applied");

    const proposalProjectValidation = await screen.findByLabelText("Project validation preflight");
    expect(within(proposalProjectValidation).getByTestId("project-validation-operations").textContent).toContain(
      "pending operations=1"
    );
    expect(within(proposalProjectValidation).getByTestId("project-validation-operations").textContent).toContain(
      "proposals=1"
    );
    const proposalValidationHref =
      within(proposalProjectValidation).getByTestId("project-validation-export-link").getAttribute("href") ?? "";
    const proposalValidationPacket = JSON.parse(decodeURIComponent(proposalValidationHref.split(",", 2)[1]));
    expect(proposalValidationPacket.summary.pending_operation_count).toBe(1);
    expect(proposalValidationPacket.summary.editor_intent_count).toBe(0);
    expect(proposalValidationPacket.summary.proposal_operation_count).toBe(1);
    expect(proposalValidationPacket.proposal_refs).toContain("proposal:physics-diagnostic-review");
    expect(proposalValidationPacket.review_operation_statuses).toContain("not_applied");

    const operationLedger = await screen.findByLabelText("Operation review ledger");
    expect(await within(operationLedger).findByTestId("operation-ledger-export-summary")).toHaveTextContent(
      "1 review record"
    );
    expect(within(operationLedger).getByTestId("operation-ledger-decision-counts").textContent).toContain(
      "1 held_for_user_acceptance"
    );
    expect(within(operationLedger).getByTestId("operation-ledger-latest").textContent).toContain(
      "op:review-computed-diagnostic"
    );
    expect(within(operationLedger).getByTestId("operation-ledger-boundary").textContent).toContain(
      "requires explicit user acceptance"
    );
    expect(
      within(operationLedger).getByTestId("operation-ledger-record-op-review-computed-diagnostic").textContent
    ).toContain("result:stress:pipe-P-120:end-j:torsional-shear");
    const ledgerHref = within(operationLedger).getByTestId("operation-ledger-export-link").getAttribute("href") ?? "";
    const ledgerPacket = JSON.parse(decodeURIComponent(ledgerHref.split(",", 2)[1]));
    expect(ledgerPacket.document_kind).toBe("openpipestress.technical_preview.operation_review_ledger");
    expect(ledgerPacket.export_scope).toBe("local_browser_download_preview");
    expect(ledgerPacket.deliverable_refs).toContain("DEL-16-04");
    expect(ledgerPacket.decision_counts.held_for_user_acceptance).toBe(1);
    expect(ledgerPacket.records[0].record_source).toBe("agent_proposal");
    expect(ledgerPacket.records[0].proposal_ref).toBe("proposal:physics-diagnostic-review");
    expect(ledgerPacket.records[0].decision.status).toBe("held_for_user_acceptance");
    expect(ledgerPacket.records[0].actor.actor_type).toBe("agent");
    expect(ledgerPacket.records[0].affected_entities[0].ref).toBe("result:stress:pipe-P-120:end-j:torsional-shear");
    expect(ledgerPacket.records[0].validation_outcome.diff_preview_status).toBe("generated_from_computed_context");
    expect(ledgerPacket.records[0].validation_outcome.application_status).toBe("not_applied");
    expect(ledgerPacket.records[0].professional_boundary.software_makes_compliance_claim).toBe(false);
    expect(ledgerPacket.selected_review_target).toEqual({
      target_type: "result",
      id: "result:stress:pipe-P-120:end-j:torsional-shear"
    });
    expect(ledgerPacket.accepted_model_state_unchanged).toBe(true);
    expect(ledgerPacket.release_or_professional_claim).toBe(false);

    const nativePackageAfterProposal = await screen.findByLabelText("Native JSON package");
    expect(within(nativePackageAfterProposal).getByTestId("native-package-summary").textContent).toContain(
      "operations=1"
    );
    expect(within(nativePackageAfterProposal).getByTestId("native-package-stable-ids").textContent).toContain(
      "operations=1"
    );
    const nativePackageAfterProposalHref =
      within(nativePackageAfterProposal).getByTestId("native-package-link").getAttribute("href") ?? "";
    const nativePackageAfterProposalPacket = JSON.parse(
      decodeURIComponent(nativePackageAfterProposalHref.split(",", 2)[1])
    );
    expect(nativePackageAfterProposalPacket.stable_id_map.operation_ref_count).toBe(1);
    expect(nativePackageAfterProposalPacket.stable_id_map.operation_refs).toContain(
      "op:review-computed-diagnostic"
    );
    expect(nativePackageAfterProposalPacket.stable_id_map.proposal_refs).toContain(
      "proposal:physics-diagnostic-review"
    );
    expect(nativePackageAfterProposalPacket.operation_review.record_count).toBe(1);
    expect(nativePackageAfterProposalPacket.operation_review.editor_intent_count).toBe(0);
    expect(nativePackageAfterProposalPacket.operation_review.proposal_count).toBe(1);
    expect(nativePackageAfterProposalPacket.operation_review.held_for_user_acceptance_count).toBe(1);
    expect(nativePackageAfterProposalPacket.operation_review.accepted_count).toBe(0);
    expect(nativePackageAfterProposalPacket.operation_review.rejected_count).toBe(0);
    expect(nativePackageAfterProposalPacket.operation_review.operation_refs).toContain(
      "op:review-computed-diagnostic"
    );
    expect(nativePackageAfterProposalPacket.operation_review.proposal_refs).toContain(
      "proposal:physics-diagnostic-review"
    );
    expect(nativePackageAfterProposalPacket.operation_review.selected_review_target).toEqual({
      target_type: "result",
      id: "result:stress:pipe-P-120:end-j:torsional-shear"
    });
    expect(nativePackageAfterProposalPacket.operation_review.accepted_model_state_mutated).toBe(false);
    expect(nativePackageAfterProposalPacket.operation_review.operation_application_status).toBe("not_applied");
    expect(nativePackageAfterProposalPacket.operation_review.audit_boundary.requires_user_acceptance).toBe(true);
    expect(nativePackageAfterProposalPacket.operation_review.audit_boundary.preview_records_do_not_apply_operations).toBe(
      true
    );
    expect(nativePackageAfterProposalPacket.operation_review.audit_boundary.direct_model_mutation_allowed).toBe(false);
    expect(nativePackageAfterProposalPacket.validation_report.checks).toContain(
      "review-only operation refs declared when present"
    );
    expect(nativePackageAfterProposalPacket.release_or_professional_claim).toBe(false);
    expect(nativePackageAfterProposalPacket.professional_boundary.software_makes_approval_claim).toBe(false);

    const diffPreview = await screen.findByLabelText("Operation diff preview");
    expect(await within(diffPreview).findByTestId("diff-preview-summary")).toHaveTextContent("1 operations");
    expect(within(diffPreview).getByTestId("diff-preview-summary").textContent).toContain("1 diff rows");
    expect(within(diffPreview).getByTestId("diff-preview-summary").textContent).toContain(
      "accepted_state_mutated=false"
    );
    expect(within(diffPreview).getByTestId("diff-preview-validation").textContent).toContain("0 hash-bound rows");
    expect(within(diffPreview).getByTestId("diff-preview-boundary").textContent).toContain(
      "no release or professional claim"
    );
    const proposalDiffRecord = within(diffPreview).getByTestId(
      "diff-preview-record-op-review-computed-diagnostic"
    );
    expect(proposalDiffRecord.textContent).toContain("agent_proposal");
    expect(proposalDiffRecord.textContent).toContain("generated_from_computed_context");
    expect(proposalDiffRecord.textContent).toContain("result:stress:pipe-P-120:end-j:torsional-shear");
    const diffHref = within(diffPreview).getByTestId("diff-preview-export-link").getAttribute("href") ?? "";
    const diffPacket = JSON.parse(decodeURIComponent(diffHref.split(",", 2)[1]));
    expect(diffPacket.document_kind).toBe("openpipestress.technical_preview.operation_diff_preview");
    expect(diffPacket.deliverable_refs).toContain("DEL-16-02");
    expect(diffPacket.scope_items).toContain("SOW-069");
    expect(diffPacket.summary.operation_count).toBe(1);
    expect(diffPacket.summary.diff_row_count).toBe(1);
    expect(diffPacket.summary.held_for_user_acceptance_count).toBe(1);
    expect(diffPacket.summary.accepted_model_state_mutated).toBe(false);
    expect(diffPacket.previews[0].record_source).toBe("agent_proposal");
    expect(diffPacket.previews[0].operation_id).toBe("op:review-computed-diagnostic");
    expect(diffPacket.previews[0].diff_preview_status).toBe("generated_from_computed_context");
    expect(diffPacket.previews[0].application_status).toBe("not_applied");
    expect(diffPacket.previews[0].accepted_model_state_mutated).toBe(false);
    expect(diffPacket.previews[0].changes[0].target_ref).toBe(
      "result:stress:pipe-P-120:end-j:torsional-shear"
    );
    expect(diffPacket.private_payload_included).toBe(false);
    expect(diffPacket.protected_content_included).toBe(false);
    expect(diffPacket.release_or_professional_claim).toBe(false);

    const proposalExportReview = await screen.findByLabelText("Export safety review");
    const proposalReviewHref = within(proposalExportReview).getByTestId("export-review-link").getAttribute("href") ?? "";
    const proposalReviewManifest = JSON.parse(decodeURIComponent(proposalReviewHref.split(",", 2)[1]));
    expect(proposalReviewManifest.summary.operation_record_count).toBe(1);
    expect(
      proposalReviewManifest.exports.find((item: { export_id: string }) => item.export_id === "project_storage_audit")
        .pending_operation_count
    ).toBe(1);
    expect(
      proposalReviewManifest.exports.find((item: { export_id: string }) => item.export_id === "project_storage_audit")
        .proposal_operation_count
    ).toBe(1);
    expect(
      proposalReviewManifest.exports.find(
        (item: { export_id: string }) => item.export_id === "project_validation_preflight"
      ).pending_operation_count
    ).toBe(1);
    expect(
      proposalReviewManifest.exports.find(
        (item: { export_id: string }) => item.export_id === "project_validation_preflight"
      ).proposal_operation_count
    ).toBe(1);

    fireEvent.click(within(operationLedger).getByTestId("clear-operation-review-queue"));

    expect(screen.getByTestId("local-project-review-context").textContent).toContain(
      "0 pending operations; applied=false; editor_intents=0; agent_proposals=0"
    );
    expect(within(proposal).queryByTestId("proposal-body")).not.toBeInTheDocument();
    expect(await within(operationLedger).findByTestId("operation-ledger-empty")).toHaveTextContent(
      "No structured operations are queued"
    );
    expect(await within(diffPreview).findByTestId("diff-preview-empty")).toHaveTextContent("No operation diffs");
    expect(within(report).queryByTestId("report-proposal-operation")).not.toBeInTheDocument();
    expect(within(report).queryByTestId("report-proposal-boundary")).not.toBeInTheDocument();

    const clearedReportHref = within(report).getByTestId("report-export-link").getAttribute("href") ?? "";
    const clearedReportPacket = JSON.parse(decodeURIComponent(clearedReportHref.split(",", 2)[1]));
    expect(clearedReportPacket.proposal_ref).toBe("not generated");
    expect(clearedReportPacket.proposal_operation).toBeNull();
    expect(clearedReportPacket.selected_review_target).toEqual({
      target_type: "result",
      id: "result:stress:pipe-P-120:end-j:torsional-shear"
    });

    const nativePackageAfterClearHref =
      within(nativePackageAfterProposal).getByTestId("native-package-link").getAttribute("href") ?? "";
    const nativePackageAfterClearPacket = JSON.parse(decodeURIComponent(nativePackageAfterClearHref.split(",", 2)[1]));
    expect(nativePackageAfterClearPacket.stable_id_map.operation_ref_count).toBe(0);
    expect(nativePackageAfterClearPacket.stable_id_map.operation_refs).toEqual([]);
    expect(nativePackageAfterClearPacket.stable_id_map.proposal_refs).toEqual([]);
    expect(nativePackageAfterClearPacket.operation_review.record_count).toBe(0);
    expect(nativePackageAfterClearPacket.operation_review.proposal_count).toBe(0);
    expect(nativePackageAfterClearPacket.operation_review.held_for_user_acceptance_count).toBe(0);

    const clearedStorageHref =
      within(proposalStorageAudit).getByTestId("project-storage-export-link").getAttribute("href") ?? "";
    const clearedStoragePacket = JSON.parse(decodeURIComponent(clearedStorageHref.split(",", 2)[1]));
    expect(clearedStoragePacket.summary.pending_operation_count).toBe(0);
    expect(clearedStoragePacket.summary.proposal_operation_count).toBe(0);
    expect(clearedStoragePacket.proposal_refs).toEqual([]);

    const clearedValidationHref =
      within(proposalProjectValidation).getByTestId("project-validation-export-link").getAttribute("href") ?? "";
    const clearedValidationPacket = JSON.parse(decodeURIComponent(clearedValidationHref.split(",", 2)[1]));
    expect(clearedValidationPacket.summary.pending_operation_count).toBe(0);
    expect(clearedValidationPacket.summary.proposal_operation_count).toBe(0);
    expect(clearedValidationPacket.proposal_refs).toEqual([]);

    const exportReviewAfterClear = await screen.findByLabelText("Export safety review");
    const clearedReviewHref =
      within(exportReviewAfterClear).getByTestId("export-review-link").getAttribute("href") ?? "";
    const clearedReviewManifest = JSON.parse(decodeURIComponent(clearedReviewHref.split(",", 2)[1]));
    expect(clearedReviewManifest.summary.operation_record_count).toBe(0);
    expect(
      clearedReviewManifest.exports.find((item: { export_id: string }) => item.export_id === "operation_review_ledger")
        .readiness
    ).toBe("empty_operation_queue");
  }, 15000);

  it("links selected diagnostics to affected result and model context", async () => {
    render(<App />);

    fireEvent.click(await screen.findByRole("button", { name: /Run mechanics preview/i }));
    expect(await screen.findByTestId("diagnostic-HIGH_DISPLACEMENT_REVIEW")).toBeInTheDocument();

    const diagnostics = await screen.findByLabelText("Diagnostics");
    expect(within(diagnostics).getByTestId("diagnostic-filter-summary").textContent).toContain(
      "9 of 9 diagnostics visible"
    );
    expect(within(diagnostics).getByTestId("diagnostic-severity-warning").textContent).toContain("8 Warnings");
    expect(within(diagnostics).getByTestId("diagnostic-severity-info").textContent).toContain("1 Info");
    expect(within(diagnostics).getByTestId("diagnostic-severity-error").textContent).toContain("0 Errors");
    expect(within(diagnostics).getByTestId("diagnostic-severity-blocking").textContent).toContain("0 Blocking");

    fireEvent.change(within(diagnostics).getByTestId("diagnostic-filter-input"), {
      target: { value: "HIGH_DISPLACEMENT_REVIEW" }
    });
    expect(within(diagnostics).getByTestId("diagnostic-filter-summary").textContent).toContain(
      "1 of 9 diagnostics visible"
    );
    expect(within(diagnostics).getByTestId("diagnostic-severity-warning").textContent).toContain("1 Warnings");
    expect(within(diagnostics).getByTestId("diagnostic-severity-info").textContent).toContain("0 Info");
    expect(within(diagnostics).getByTestId("diagnostic-HIGH_DISPLACEMENT_REVIEW")).toBeInTheDocument();
    expect(within(diagnostics).queryByTestId("diagnostic-RULE_CHECK_NOT_PERFORMED")).not.toBeInTheDocument();

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

    fireEvent.click(within(diagnostics).getByTestId("clear-diagnostic-filter"));
    expect(within(diagnostics).getByTestId("diagnostic-filter-summary").textContent).toContain(
      "9 of 9 diagnostics visible"
    );
    expect(within(diagnostics).getByTestId("diagnostic-RULE_CHECK_NOT_PERFORMED")).toBeInTheDocument();

    fireEvent.click(screen.getByRole("button", { name: /Generate review proposal/i }));
    const proposal = await screen.findByLabelText("Agentic proposal");
    expect(await within(proposal).findByText("proposal:physics-diagnostic-review")).toBeInTheDocument();
    expect(within(proposal).getByTestId("selected-review-target").textContent).toContain(
      "diagnostic: diagnostic:physics:high-displacement-review"
    );
    expect(within(proposal).getAllByText(/diagnostic:physics:high-displacement-review/i).length).toBeGreaterThan(0);
    expect(within(proposal).getByTestId("proposal-affected-entities").textContent).toContain(
      "diagnostic:physics:high-displacement-review"
    );
    expect(within(proposal).getByTestId("proposal-audit-boundary").textContent).toContain(
      "acceptance recorded as review only"
    );
    expect(within(proposal).getByTestId("proposal-professional-boundary").textContent).toContain(
      "software makes compliance claim"
    );
    expect(within(proposal).getByTestId("proposal-professional-boundary").textContent).toContain("false");
    expect(within(proposal).getByRole("button", { name: /Accept disabled/i })).toBeDisabled();

    const report = await screen.findByLabelText("Report packet");
    expect(within(report).getByTestId("report-selected-review-target").textContent).toContain(
      "diagnostic: diagnostic:physics:high-displacement-review"
    );
    expect(within(report).getByTestId("report-proposal-operation").textContent).toContain(
      "op:review-computed-diagnostic"
    );
    const proposalExportHref = within(report).getByTestId("report-export-link").getAttribute("href") ?? "";
    const proposalExportPacket = JSON.parse(decodeURIComponent(proposalExportHref.split(",", 2)[1]));
    expect(proposalExportPacket.selected_review_target).toEqual({
      target_type: "diagnostic",
      id: "diagnostic:physics:high-displacement-review"
    });
    expect(proposalExportPacket.proposal_operation.affected_entity_ids).toContain(
      "diagnostic:physics:high-displacement-review"
    );
    expect(proposalExportPacket.proposal_operation.audit_boundary.mutates_accepted_model_state).toBe(false);
  });
});
