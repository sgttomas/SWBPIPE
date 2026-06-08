import { fireEvent, render, screen, within } from "@testing-library/react";
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
    const reportLint = await screen.findByLabelText("Report content lint");
    expect(within(reportLint).getByTestId("report-lint-summary").textContent).toContain("targets=4");
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
    expect(lintPacket.lint_run.summary.target_count).toBe(4);
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
      "6 of 11 local exports ready"
    );
    expect(within(exportReview).getByTestId("export-review-redaction").textContent).toContain(
      "protected content blocked=true"
    );
    expect(within(exportReview).getByTestId("export-review-boundary").textContent).toContain(
      "no release or professional claim"
    );
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
    expect(within(projectValidation).getByTestId("project-validation-schema").textContent).toContain(
      "schema_version=0.1.0"
    );
    expect(within(projectValidation).getByTestId("project-validation-round-trip").textContent).toContain(
      "6 categories"
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
      "TBD_canonical_project_hash_service_not_available"
    );
    expect(validationPacket.boundary.local_only_project_store).toBe(true);
    expect(validationPacket.boundary.repository_default_private_write).toBe(false);
    expect(validationPacket.boundary.accepted_model_state_mutated).toBe(false);
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
      "11 of 11 local exports ready"
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
    expect(within(exportReview).getByTestId("export-review-record-result_envelope").textContent).toContain(
      "available"
    );
    expect(within(exportReview).getByTestId("export-review-record-headless_runner_envelope").textContent).toContain(
      "available"
    );
    expect(within(exportReview).getByTestId("export-review-record-adapter_framework_envelope").textContent).toContain(
      "available"
    );
    expect(
      within(exportReview).getByTestId("export-review-record-external_prover_boundary_metadata").textContent
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
    expect(reviewManifest.deliverable_refs).toContain("DEL-08-04");
    expect(reviewManifest.deliverable_refs).toContain("DEL-08-05");
    expect(reviewManifest.deliverable_refs).toContain("DEL-10-02");
    expect(reviewManifest.deliverable_refs).toContain("DEL-10-05");
    expect(reviewManifest.deliverable_refs).toContain("DEL-15-04");
    expect(reviewManifest.deliverable_refs).toContain("DEL-17-02");
    expect(reviewManifest.deliverable_refs).toContain("DEL-17-03");
    expect(reviewManifest.scope_items).toContain("SOW-040");
    expect(reviewManifest.scope_items).toContain("SOW-050");
    expect(reviewManifest.scope_items).toContain("SOW-041");
    expect(reviewManifest.scope_items).toContain("SOW-029");
    expect(reviewManifest.scope_items).toContain("SOW-046");
    expect(reviewManifest.scope_items).toContain("SOW-043");
    expect(reviewManifest.scope_items).toContain("SOW-054");
    expect(reviewManifest.scope_items).toContain("SOW-032");
    expect(reviewManifest.scope_items).toContain("SOW-030");
    expect(reviewManifest.scope_items).toContain("SOW-075");
    expect(reviewManifest.objectives).toContain("OBJ-002");
    expect(reviewManifest.objectives).toContain("OBJ-008");
    expect(reviewManifest.objectives).toContain("OBJ-009");
    expect(reviewManifest.objectives).toContain("OBJ-017");
    expect(reviewManifest.summary.available_count).toBe(11);
    expect(reviewManifest.summary.operation_record_count).toBe(1);
    expect(reviewManifest.exports.map((item: { export_id: string }) => item.export_id)).toEqual([
      "project_storage_audit",
      "project_validation_preflight",
      "result_envelope",
      "headless_runner_envelope",
      "adapter_framework_envelope",
      "external_prover_boundary_metadata",
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
    const lintExport = reviewManifest.exports.find(
      (item: { export_id: string }) => item.export_id === "report_protected_content_lint"
    );
    expect(lintExport.document_kind).toBe("openpipestress.technical_preview.report_protected_content_lint_run");
    expect(lintExport.deliverable_refs).toContain("DEL-08-05");
    expect(lintExport.target_count).toBe(5);
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
    expect(auditPacket.summary.applied_operation_count).toBe(0);
    expect(auditPacket.summary.accepted_model_state_mutated).toBe(false);
    expect(auditPacket.summary.network_required).toBe(false);
    expect(auditPacket.summary.daemon_required).toBe(false);
    expect(auditPacket.summary.telemetry_enabled).toBe(false);
    expect(auditPacket.summary.copied_external_files).toBe(false);
    expect(auditPacket.project_summary.storage_mode).toBe("browser_memory_preview");
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
      "migration=current"
    );
    expect(within(projectValidation).getByTestId("project-validation-operations").textContent).toContain(
      "pending operations=1"
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
    expect(validationPacket.summary.migration_status).toBe("current");
    expect(validationPacket.summary.pending_operation_count).toBe(1);
    expect(validationPacket.summary.accepted_model_state_mutated).toBe(false);
    expect(validationPacket.service_operations.find((operation: { operation: string }) => operation.operation === "validate").operation_status).toBe(
      "preflight_generated_preview_current"
    );
    expect(
      validationPacket.service_operations.find((operation: { operation: string }) => operation.operation === "version_check")
        .operation_status
    ).toBe("supported_current_schema");
    expect(validationPacket.service_operations.find((operation: { operation: string }) => operation.operation === "migrate").operation_status).toBe(
      "not_run_migration_framework_tbd"
    );
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

    fireEvent.click(within(tree).getByRole("button", { name: /Invented carbon-steel-like material/i }));
    expect(
      within(screen.getByLabelText("Editor operation intent")).getByTestId("editor-intent-queue").textContent
    ).toContain("editor-intent-1");
  });

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
      "additional_formats=TBD"
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
    expect(resultExportPacket.export_format_status.additional_formats).toBe("TBD");
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
    expect(resultExportPacket.result_envelope.downstream_use.additional_export_formats).toBe("TBD");
    expect(resultExportPacket.result_envelope.professional_boundary.software_makes_compliance_claim).toBe(false);
    expect(resultExportPacket.result_envelope.professional_boundary.software_makes_certification_claim).toBe(false);
    expect(resultExportPacket.result_envelope.professional_boundary.software_makes_sealing_claim).toBe(false);
    expect(resultExportPacket.result_envelope.professional_boundary.software_makes_approval_claim).toBe(false);
    expect(resultExportPacket.result_envelope.professional_boundary.software_makes_authentication_claim).toBe(false);
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
    expect(within(nativePackage).getByTestId("native-package-validation").textContent).toContain("model_hash=TBD");
    expect(within(nativePackage).getByTestId("native-package-loss-report").textContent).toContain("1 TBD");
    expect(within(nativePackage).getByTestId("native-package-loss-report").textContent).toContain("1 unsupported");
    expect(within(nativePackage).getByTestId("native-package-storage").textContent).toContain("network=false");
    expect(within(nativePackage).getByTestId("native-package-storage").textContent).toContain(
      "repository_default_private_write=false"
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
    expect(nativePackagePacket.loss_report.summary.unsupported_count).toBe(1);
    expect(nativePackagePacket.loss_report.summary.tbd_count).toBe(1);
    expect(nativePackagePacket.validation_report.package_shape_status).toBe("review_manifest_complete");
    expect(nativePackagePacket.validation_report.model_hash_status).toBe("TBD_model_hash_not_available");
    expect(nativePackagePacket.validation_report.package_hash_status).toBe(
      "TBD_canonical_package_hash_service_not_available"
    );
    expect(nativePackagePacket.diagnostics).toHaveLength(7);
    expect(nativePackagePacket.generation_context.network_required).toBe(false);
    expect(nativePackagePacket.generation_context.telemetry_enabled).toBe(false);
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
      "10 of 11 local exports ready"
    );
    expect(within(report).getByTestId("report-export-readiness").textContent).toContain("storage=available");
    expect(within(report).getByTestId("report-export-readiness").textContent).toContain("validation=available");
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
    expect(exportPacket.deliverable_refs).toContain("DEL-08-03");
    expect(exportPacket.deliverable_refs).toContain("DEL-08-06");
    expect(exportPacket.deliverable_refs).toContain("DEL-15-04");
    expect(exportPacket.deliverable_refs).toContain("DEL-02-05");
    expect(exportPacket.deliverable_refs).toContain("DEL-12-01");
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
    expect(exportPacket.persistence_evidence.export_inventory.expected_export_count).toBe(11);
    expect(exportPacket.persistence_evidence.export_inventory.available_count).toBe(10);
    expect(exportPacket.persistence_evidence.export_inventory.readiness_by_export_id.project_storage_audit).toBe(
      "available"
    );
    expect(
      exportPacket.persistence_evidence.export_inventory.readiness_by_export_id.project_validation_preflight
    ).toBe("available");
    expect(
      exportPacket.persistence_evidence.export_inventory.readiness_by_export_id.report_protected_content_lint
    ).toBe("available");
    expect(exportPacket.persistence_evidence.export_inventory.readiness_by_export_id.headless_runner_envelope).toBe(
      "available"
    );
    expect(exportPacket.persistence_evidence.export_inventory.readiness_by_export_id.adapter_framework_envelope).toBe(
      "available"
    );
    expect(
      exportPacket.persistence_evidence.export_inventory.readiness_by_export_id.external_prover_boundary_metadata
    ).toBe("available");
    expect(exportPacket.persistence_evidence.export_inventory.readiness_by_export_id.operation_review_ledger).toBe(
      "empty_operation_queue"
    );
    expect(exportPacket.persistence_evidence.boundary.network_required).toBe(false);
    expect(exportPacket.persistence_evidence.boundary.telemetry_enabled).toBe(false);
    expect(exportPacket.persistence_evidence.boundary.private_payload_included).toBe(false);
    expect(exportPacket.persistence_evidence.boundary.protected_content_included).toBe(false);
    expect(exportPacket.persistence_evidence.boundary.release_or_professional_claim).toBe(false);
    const reportLint = await screen.findByLabelText("Report content lint");
    expect(within(reportLint).getByTestId("report-lint-summary").textContent).toContain("targets=5");
    expect(within(reportLint).getByTestId("report-lint-summary").textContent).toContain("findings=0");
    expect(within(reportLint).getByTestId("report-lint-summary").textContent).toContain("blocking=0");
    expect(within(reportLint).getByTestId("report-lint-clean-scan").textContent).toContain(
      "clearance=false"
    );
    const reportLintHref = within(reportLint).getByTestId("report-lint-export-link").getAttribute("href") ?? "";
    const reportLintPacket = JSON.parse(decodeURIComponent(reportLintHref.split(",", 2)[1]));
    expect(reportLintPacket.deliverable_id).toBe("DEL-08-05");
    expect(reportLintPacket.lint_run.run_id).toBe("lint:report-preview:run-preview-linear-static-001");
    expect(reportLintPacket.lint_run.summary.target_count).toBe(5);
    expect(reportLintPacket.lint_run.summary.scanned_target_count).toBe(5);
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
