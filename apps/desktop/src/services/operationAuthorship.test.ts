// DEL-16-03 / DEL-16-06 author routes: offline intake and retained-record requeue
// stamp agent or imported content `agent` host-side, so a file or stored record that
// claims `user` cannot satisfy the applier's refusals of an agent-written k or other
// component modifier after the person's Apply. The final case runs the stamped import
// through the desktop batch service on the wasm engine. All data are invented.
import { describe, expect, it } from "vitest";
import type { OperationBatch } from "./operationBatchService";
import { applyOperationBatch } from "./operationBatchService";
import { computeModelHash } from "./hashService";
import { loadPreviewModel } from "./previewService";
import { parseOfflineBatch } from "../features/offline-proposal-intake/OfflineProposalIntakePanel";
import { requeueRetainedRecord, stampImportedBatch, writesUserEnteredModifier } from "./operationAuthorship";

const op = (over: Record<string, unknown>) => ({ operation_id: "op:x", operation_kind: "modify", operation_status: "proposed",
  target: { object_type: "Component", ref: "component:C-110" },
  change: { change_id: "c:x", change_kind: "set_field", field_label: "x", field_path: "modifiers.sif_user_value.value", before: "TBD", after: "{\"value\":1.5,\"unit\":\"none\"}", unit: "none", dimension: "dimensionless", source_note: "file" },
  validation: { schema_validation: "not_run", constraint_validation: "not_run", unit_validation: "not_run", diff_preview_status: "not_generated", application_status: "not_applied" },
  rationale: "file rationale",
  audit_boundary: { mutation_route: "structured_operations_only", direct_model_mutation_allowed: false, requires_user_acceptance: true, mutates_accepted_model_state: false },
  professional_boundary: { human_review_required: true, software_makes_compliance_claim: false, software_makes_certification_claim: false, software_makes_sealing_claim: false, software_makes_approval_claim: false, software_makes_authentication_claim: false },
  ...over });

describe("operation authorship on agent and imported routes", () => {
  it("recognizes every route that writes a value only the person enters", () => {
    expect(writesUserEnteredModifier(op({}))).toBe(true);
    expect(writesUserEnteredModifier(op({ change: { change_kind: "set_field", field_path: "bend_realization" } }))).toBe(true);
    expect(writesUserEnteredModifier(op({ target: { object_type: "Node", ref: "n" }, change: { change_kind: "insert_bend_at_corner", field_path: "bend_at_corner" } }))).toBe(true);
    expect(writesUserEnteredModifier(op({ change: { change_kind: "insert_component_symbol", field_path: "components", after: "{\"modifiers\":{}}" } }))).toBe(true);
    expect(writesUserEnteredModifier(op({ change: { change_kind: "apply_library_record", field_path: "components", after: "{\"modifiers.sif_user_value\":{}}" } }))).toBe(true);
    expect(writesUserEnteredModifier(op({ change: { change_kind: "set_field", field_path: "geometry.bend_radius.value" } }))).toBe(false);
    expect(writesUserEnteredModifier(op({ target: { object_type: "Node", ref: "n" }, change: { change_kind: "set_field", field_path: "position.x" } }))).toBe(false);
  });

  it("requeues stored records with a host-derived author", () => {
    const agent = requeueRetainedRecord(op({ author_type: "user", source: { source_ref: "s", source_channel: "embedded_agent", source_role: "agent" } }), 3);
    expect("batch" in agent && agent.batch.operations[0].author_type).toBe("agent");
    const imported = requeueRetainedRecord(op({ author_type: "user", source: { source_ref: "f", source_channel: "offline_intake", source_role: "imported" } }), 0);
    expect("batch" in imported && imported.batch.operations[0].author_type).toBe("agent");
    const claimedUser = requeueRetainedRecord(op({ author_type: "user" }), 1);
    expect(claimedUser).toEqual({ refused: expect.stringContaining("author cannot be verified") });
    const ordinary = requeueRetainedRecord(op({ author_type: "user", target: { object_type: "Node", ref: "n" }, change: { change_kind: "set_field", field_path: "position.x", before: "0", after: "1" } }), 2);
    expect("batch" in ordinary && ordinary.batch.operations[0].author_type).toBe("user");
    expect(requeueRetainedRecord(op({ author_type: undefined }), 4)).toEqual({ refused: expect.stringContaining("does not say who proposed it") });
    expect(requeueRetainedRecord("not an operation", 5)).toEqual({ refused: expect.any(String) });
  });

  it("a file claiming the user cannot write k or a SIF after Apply", async () => {
    const model = await loadPreviewModel();
    const file = { batch_id: "batch:claimed-user", operations: [
      op({ operation_id: "op:sif", author_type: "user", source: { source_ref: "me", source_channel: "local_desktop_preview", source_role: "gui_editor" } }),
      op({ operation_id: "op:k", author_type: "user", change: { ...op({}).change, change_id: "c:k", field_path: "modifiers.flexibility_factor_user_value.value", after: "{\"value\":2,\"unit\":\"none\"}" } })
    ] } as unknown as OperationBatch;
    const { batch } = stampImportedBatch(parseOfflineBatch(JSON.stringify(file)));
    for (const operation of [0, 1]) {
      const single = { batch_id: `import-${operation}`, operations: [batch.operations[operation]] };
      const hash = await computeModelHash(model);
      const outcome = await applyOperationBatch(model, single, hash);
      expect(outcome.applied_model).toBeNull();
      const codes = outcome.operation_outcomes.flatMap(step => step.diagnostics.map(d => d.code)).concat(outcome.diagnostics.map(d => d.code));
      expect(codes).toContain(operation === 0 ? "OP-COMPONENT-MODIFIER-USER-ENTRY-REQUIRED" : "OP-BEND-FLEXIBILITY-USER-ENTRY-REQUIRED");
    }
  });
});
