import { describe, expect, it } from "vitest";
import { applyOperationBatch, type OperationBatchOutcome } from "../../services/operationBatchService";
import { applyModelOperation } from "../../services/operationService";
import { loadPreviewModel } from "../../services/previewService";
import { computeModelHash } from "../../services/hashService";
import type { EditorOperationIntent, OperationOutcome } from "../../types";
import {
  DraftReviewGate,
  applyResultMatchesSubmission,
  buildRouteSubmission,
  validationMatchesSubmission,
  type RouteDraft
} from "./routeDraft";

function exactDiff(intent: EditorOperationIntent) {
  return [{
    entity_ref: intent.target.ref,
    object_type: intent.target.object_type,
    field_path: intent.change.field_path,
    before: intent.change.before,
    after: intent.change.after,
    unit: intent.change.unit,
    dimension: intent.change.dimension,
    change_kind: intent.change.change_kind
  }];
}

function singleValidation(intent: EditorOperationIntent): OperationOutcome {
  return {
    mode: "validate_only",
    operation_id: intent.operation_id,
    change_id: intent.change.change_id,
    target_ref: intent.target.ref,
    validation: {
      schema_validation: "passed",
      diff_preview_status: "generated",
      application_status: "not_applied"
    },
    diff_preview: exactDiff(intent),
    diagnostics: []
  } as unknown as OperationOutcome;
}

function batchValidation(
  batchId: string,
  operations: EditorOperationIntent[]
): OperationBatchOutcome {
  return {
    mode: "validate_only",
    batch_id: batchId,
    submitted_operations: structuredClone(operations),
    validation: {
      schema_validation: "passed",
      batch_validation_status: "passed",
      diff_preview_status: "generated",
      application_status: "not_applied"
    },
    simulation_disposition: "validation_only_discarded",
    diagnostics: [],
    operation_outcomes: operations.map((intent, index) => ({
      index,
      operation_id: intent.operation_id,
      change_id: intent.change.change_id,
      simulation_status: "validated_on_temporary_state",
      diff_preview: exactDiff(intent),
      diagnostics: []
    }))
  } as unknown as OperationBatchOutcome;
}

function draft(model: Awaited<ReturnType<typeof loadPreviewModel>>): RouteDraft {
  return {
    startNodeId: model.nodes[0].id,
    endMode: "existing",
    existingEndNodeId: model.nodes[1].id,
    newEnd: {
      id: "node:UI-A-110",
      label: "Loaded end",
      x: "3.2",
      y: "0",
      z: "0",
      coordinateUnit: "m",
      provenance: "synthetic_ui_acceptance_input"
    },
    pipe: {
      id: "pipe:UI-A-100",
      label: "Straight run",
      materialId: model.materials![0].id,
      outsideDiameter: "0.168",
      wallThickness: "0.007",
      lengthUnit: "m",
      yReferenceX: "0",
      yReferenceY: "0",
      yReferenceZ: "1",
      provenance: "synthetic_ui_acceptance_input"
    }
  };
}

describe("straight route draft", () => {
  it("builds one exact connect intent for an existing endpoint", async () => {
    const model = await loadPreviewModel();
    const built = buildRouteSubmission(model, [], draft(model), 7, "length=passed");
    expect(built.ok).toBe(true);
    if (!built.ok || built.submission.kind !== "single") throw new Error("expected single route intent");
    expect(built.submission.intent.change.change_kind).toBe("connect_pipe_run");
    expect(JSON.parse(built.submission.intent.change.after)).toMatchObject({
      id: "pipe:UI-A-100",
      from: model.nodes[0].id,
      to: model.nodes[1].id,
      material: model.materials![0].id,
      section: {
        outside_diameter: { value: 0.168, unit: "m" },
        wall_thickness: { value: 0.007, unit: "m" }
      },
      y_reference: { x: 0, y: 0, z: 1 },
      provenance: "synthetic_ui_acceptance_input"
    });
  });

  it("builds the exact atomic create-node then connect-pipe batch and preserves zero coordinates", async () => {
    const model = await loadPreviewModel();
    const input = { ...draft(model), endMode: "new" as const };
    const built = buildRouteSubmission(model, [], input, 8, "length=passed");
    expect(built.ok).toBe(true);
    if (!built.ok || built.submission.kind !== "batch") throw new Error("expected route batch");
    expect(built.submission.batch.operations.map((intent) => intent.change.change_kind)).toEqual([
      "create_node",
      "connect_pipe_run"
    ]);
    expect(JSON.parse(built.submission.batch.operations[0].change.after)).toMatchObject({
      id: "node:UI-A-110",
      position: { x: 3.2, y: 0, z: 0 },
      provenance: "synthetic_ui_acceptance_input"
    });
    expect(JSON.parse(built.submission.batch.operations[1].change.after).to).toBe("node:UI-A-110");
  });

  it.each([
    ["ID collision", (input: RouteDraft, model: Awaited<ReturnType<typeof loadPreviewModel>>) => { input.newEnd.id = model.nodes[0].id; }],
    ["missing material", (input: RouteDraft) => { input.pipe.materialId = ""; }],
    ["invalid OD", (input: RouteDraft) => { input.pipe.outsideDiameter = "0"; }],
    ["invalid wall", (input: RouteDraft) => { input.pipe.wallThickness = "0.2"; }],
    ["invalid y-reference", (input: RouteDraft) => { input.pipe.yReferenceZ = "0"; }]
  ])("rejects %s without emitting an operation", async (_label, mutate) => {
    const model = await loadPreviewModel();
    const input = { ...draft(model), endMode: "new" as const, newEnd: { ...draft(model).newEnd }, pipe: { ...draft(model).pipe } };
    mutate(input, model);
    expect(buildRouteSubmission(model, [], input, 1, "length=passed").ok).toBe(false);
  });

  it("keeps typed coordinates authoritative and invalidates reviewed generations on edit or cancel", async () => {
    const model = await loadPreviewModel();
    const input = { ...draft(model), endMode: "new" as const, newEnd: { ...draft(model).newEnd, x: "0", y: "0", z: "0" } };
    const built = buildRouteSubmission(model, [], input, 1, "length=passed");
    expect(built.ok).toBe(true);
    if (!built.ok || built.submission.kind !== "batch") throw new Error("expected route batch");
    expect(JSON.parse(built.submission.batch.operations[0].change.after).position).toEqual({ x: 0, y: 0, z: 0 });
    const gate = new DraftReviewGate();
    const frozen = gate.current();
    gate.invalidate();
    expect(gate.isCurrent(frozen)).toBe(false);
    const afterEdit = gate.current();
    gate.invalidate();
    expect(gate.isCurrent(afterEdit)).toBe(false);
  });

  it("accepts only a single-operation validation diff bound exactly to the frozen submission", async () => {
    const model = await loadPreviewModel();
    const built = buildRouteSubmission(model, [], draft(model), 1, "length=passed");
    if (!built.ok || built.submission.kind !== "single") throw new Error("expected single route intent");
    const outcome = singleValidation(built.submission.intent);
    expect(validationMatchesSubmission(built.submission, outcome)).toBe(true);
    outcome.diff_preview[0].after = "{}";
    expect(validationMatchesSubmission(built.submission, outcome)).toBe(false);
  });

  it("binds every batch validation step and submitted operation to the frozen atomic batch", async () => {
    const model = await loadPreviewModel();
    const input = { ...draft(model), endMode: "new" as const };
    const built = buildRouteSubmission(model, [], input, 1, "length=passed");
    if (!built.ok || built.submission.kind !== "batch") throw new Error("expected route batch");
    const outcome = batchValidation(built.submission.batch.batch_id, built.submission.batch.operations);
    outcome.submitted_operations = built.submission.batch.operations.map((intent) => {
      const { rationale, ...rest } = intent;
      return { rationale, ...rest };
    });
    expect(validationMatchesSubmission(built.submission, outcome)).toBe(true);
    outcome.operation_outcomes[1].diff_preview[0].field_path = "pipes[wrong]";
    expect(validationMatchesSubmission(built.submission, outcome)).toBe(false);
    outcome.operation_outcomes[1].diff_preview = exactDiff(built.submission.batch.operations[1]);
    outcome.submitted_operations[0].change.after = "{}";
    expect(validationMatchesSubmission(built.submission, outcome)).toBe(false);
  });

  it("rejects route IDs already reserved by pending batch operations", async () => {
    const model = await loadPreviewModel();
    const input = { ...draft(model), endMode: "new" as const };
    const first = buildRouteSubmission(model, [], input, 1, "length=passed");
    if (!first.ok || first.submission.kind !== "batch") throw new Error("expected route batch");
    const second = buildRouteSubmission(model, first.submission.batch.operations, input, 2, "length=passed");
    expect(second.ok).toBe(false);
    if (second.ok) throw new Error("expected reserved ID rejection");
    expect(second.errors).toContain("The node ID is already reserved.");
    expect(second.errors).toContain("The pipe ID is already reserved.");
  });

  it("ignores malformed restored members only for reservation while preserving valid reservations", async () => {
    const model = await loadPreviewModel();
    const input = { ...draft(model), endMode: "new" as const };
    const malformed = [
      null,
      42,
      {},
      { operation_kind: "create", target: null },
      { operation_kind: "create", target: { ref: 7 } },
      { operation_kind: "unknown", target: { ref: input.pipe.id } }
    ];
    expect(() => buildRouteSubmission(model, malformed, input, 2, "length=passed")).not.toThrow();
    expect(buildRouteSubmission(model, malformed, input, 2, "length=passed").ok).toBe(true);

    const first = buildRouteSubmission(model, [], input, 1, "length=passed");
    if (!first.ok || first.submission.kind !== "batch") throw new Error("expected route batch");
    expect(buildRouteSubmission(model, [...malformed, ...first.submission.batch.operations], input, 2, "length=passed").ok).toBe(false);
  });

  it("does not reserve a ref from an incomplete target discriminator", async () => {
    const model = await loadPreviewModel();
    const input = { ...draft(model), pipe: { ...draft(model).pipe } };
    const malformed = [{ operation_kind: "connect", target: { ref: input.pipe.id } }];
    expect(buildRouteSubmission(model, malformed, input, 2, "length=passed").ok).toBe(true);
  });

  it("rejects a new endpoint and pipe that share one entity ID", async () => {
    const model = await loadPreviewModel();
    const input = {
      ...draft(model),
      endMode: "new" as const,
      newEnd: { ...draft(model).newEnd },
      pipe: { ...draft(model).pipe, id: draft(model).newEnd.id }
    };
    const built = buildRouteSubmission(model, [], input, 2, "length=passed");
    expect(built.ok).toBe(false);
    if (built.ok) throw new Error("expected intra-route collision rejection");
    expect(built.errors).toContain("The new endpoint and pipe IDs must be different.");
  });

  it("requires explicitly entered node and pipe provenance", async () => {
    const model = await loadPreviewModel();
    const input = {
      ...draft(model),
      endMode: "new" as const,
      newEnd: { ...draft(model).newEnd, provenance: "" },
      pipe: { ...draft(model).pipe, provenance: "" }
    };
    const built = buildRouteSubmission(model, [], input, 2, "length=passed");
    expect(built.ok).toBe(false);
    if (built.ok) throw new Error("expected explicit provenance rejection");
    expect(built.errors).toEqual(expect.arrayContaining(["Enter node provenance.", "Enter pipe provenance."]));
  });

  it("accepts only a completely bound successful single apply envelope", async () => {
    const model = await loadPreviewModel();
    const hash = await computeModelHash(model);
    const built = buildRouteSubmission(model, [], draft(model), 1, "length=passed");
    if (!built.ok || built.submission.kind !== "single" || !hash) throw new Error("expected single route");
    const complete = await applyModelOperation(model, built.submission.intent, hash);
    expect(applyResultMatchesSubmission(built.submission, complete, hash)).toBe(true);
    for (const mutate of [
      (value: OperationOutcome) => { value.acceptance = {} as OperationOutcome["acceptance"]; },
      (value: OperationOutcome) => { value.model_basis.backend_model_hash = "sha256:mismatch"; },
      (value: OperationOutcome) => { value.target_ref = "pipe:wrong"; },
      (value: OperationOutcome) => { value.diff_preview[0].after = "{}"; },
      (value: OperationOutcome) => { value.diagnostics = [{ severity: "warning" } as never]; },
      (value: OperationOutcome) => { value.diagnostics = [{ severity: "info" } as never]; },
      (value: OperationOutcome) => { value.schema_version = "1"; },
      (value: OperationOutcome) => { value.audit_boundary = {}; },
      (value: OperationOutcome) => { value.professional_boundary = {}; },
      (value: OperationOutcome) => { value.applied_model_backend_hash = null; },
      (value: OperationOutcome) => { value.applied_model = null; }
    ]) {
      const adversarial = structuredClone(complete);
      mutate(adversarial);
      expect(applyResultMatchesSubmission(built.submission, adversarial, hash)).toBe(false);
    }
  });

  it("accepts only a completely bound ordered atomic batch apply envelope", async () => {
    const model = await loadPreviewModel();
    const hash = await computeModelHash(model);
    const input = { ...draft(model), endMode: "new" as const };
    const built = buildRouteSubmission(model, [], input, 1, "length=passed");
    if (!built.ok || built.submission.kind !== "batch" || !hash) throw new Error("expected batch route");
    const complete = await applyOperationBatch(model, built.submission.batch, hash);
    expect(applyResultMatchesSubmission(built.submission, complete, hash)).toBe(true);
    for (const mutate of [
      (value: OperationBatchOutcome) => { value.acceptance = {} as OperationBatchOutcome["acceptance"]; },
      (value: OperationBatchOutcome) => { value.initial_model_hash.value = "sha256:mismatch"; },
      (value: OperationBatchOutcome) => { value.initial_model_hash.payload_ref = "model:wrong"; },
      (value: OperationBatchOutcome) => { value.submitted_initial_model_hash!.payload_ref = "model:wrong"; },
      (value: OperationBatchOutcome) => { value.batch_id = "batch:wrong"; },
      (value: OperationBatchOutcome) => { value.submitted_operations.reverse(); },
      (value: OperationBatchOutcome) => { value.operation_outcomes[1].target_ref = "pipe:wrong"; },
      (value: OperationBatchOutcome) => { value.operation_outcomes[1].diff_preview[0].after = "{}"; },
      (value: OperationBatchOutcome) => { value.operation_outcomes[0].diagnostics = [{ severity: "warning" } as never]; },
      (value: OperationBatchOutcome) => { value.diagnostics = [{ severity: "warning" } as never]; },
      (value: OperationBatchOutcome) => { value.diagnostics = [{ severity: "info" } as never]; },
      (value: OperationBatchOutcome) => { value.schema_version = "1"; },
      (value: OperationBatchOutcome) => { value.audit_boundary = {}; },
      (value: OperationBatchOutcome) => { value.professional_boundary = {}; },
      (value: OperationBatchOutcome) => { value.applied_model_backend_hash = null; },
      (value: OperationBatchOutcome) => { value.applied_model = null; }
    ]) {
      const adversarial = structuredClone(complete);
      mutate(adversarial);
      expect(applyResultMatchesSubmission(built.submission, adversarial, hash)).toBe(false);
    }
  });
});
