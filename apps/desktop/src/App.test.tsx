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
    expect(await screen.findByLabelText("Model tree")).toBeInTheDocument();
    expect(await screen.findByLabelText("Three.js pipe centerline viewport")).toBeInTheDocument();
    const runAudit = await screen.findByLabelText("Run audit");
    expect(within(runAudit).getByTestId("run-audit-empty").textContent).toContain(
      "Run mechanics preview to generate immutable model-state and analysis-run audit references"
    );
    expect(runAudit.textContent).toContain("not a release or professional acceptance record");
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
    const exportReview = await screen.findByLabelText("Export safety review");
    expect(within(exportReview).getByTestId("export-review-summary").textContent).toContain(
      "0 of 3 local exports ready"
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
      "3 of 3 local exports ready"
    );
    expect(within(exportReview).getByTestId("export-review-summary").textContent).toContain(
      "no private/protected payloads"
    );
    expect(within(exportReview).getByTestId("export-review-boundary").textContent).toContain(
      "no release or professional claim"
    );
    expect(within(exportReview).getByTestId("export-review-record-report_packet").textContent).toContain(
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
    expect(reviewManifest.deliverable_refs).toContain("DEL-08-04");
    expect(reviewManifest.deliverable_refs).toContain("DEL-17-03");
    expect(reviewManifest.scope_items).toContain("SOW-040");
    expect(reviewManifest.scope_items).toContain("SOW-046");
    expect(reviewManifest.summary.available_count).toBe(3);
    expect(reviewManifest.summary.operation_record_count).toBe(1);
    expect(reviewManifest.exports.map((item: { export_id: string }) => item.export_id)).toEqual([
      "report_packet",
      "handoff_package",
      "operation_review_ledger"
    ]);
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

    fireEvent.click(within(controls).getByRole("button", { name: /Save local/i }));
    expect(await screen.findByTestId("local-project-message")).toHaveTextContent("without external file copies");
    expect(screen.getByTestId("local-project-review-context").textContent).toContain(
      "1 pending operation; applied=false"
    );

    fireEvent.click(within(controls).getByRole("button", { name: /Open local/i }));
    expect(await screen.findByTestId("local-project-message")).toHaveTextContent(
      "Opened local browser-preview project snapshot."
    );
    expect(screen.getByTestId("local-project-review-context").textContent).toContain(
      "1 pending operation; applied=false"
    );
    expect(within(controls).getByText("Invented Utility Loop Preview")).toBeInTheDocument();

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
    expect(within(report).getByTestId("report-export-summary").textContent).toContain("no private payload");
    expect(within(report).getByTestId("report-export-summary").textContent).toContain("9 diagnostics");
    const exportHref = within(report).getByTestId("report-export-link").getAttribute("href") ?? "";
    expect(exportHref).toContain("data:application/json");
    const exportPacket = JSON.parse(decodeURIComponent(exportHref.split(",", 2)[1]));
    expect(exportPacket.document_kind).toBe("openpipestress.technical_preview.report_packet_export");
    expect(exportPacket.export_scope).toBe("local_browser_download_preview");
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
