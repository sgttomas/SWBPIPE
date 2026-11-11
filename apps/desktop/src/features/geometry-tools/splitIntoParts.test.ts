/** B5 (DEL-07-01 Design `split-and-slope-entry.md` §2): the equal-part split planner and batch.
 * The generated batch equals a hand-built batch; the request checks and the mirrored applier
 * refusals queue nothing. Invented model; project unit ft. */
import { describe, expect, it } from "vitest";
import { canonicalJsonString } from "../../services/hashService";
import { splitPartsModel } from "../../test-support/splitPartsModel";
import type { EditorOperationIntent, PreviewModel } from "../../types";
import { makeRichIntent } from "../rich-authoring/formSupport";
import { buildSplitIntoPartsBatch, emptySplitPartsRequest, isValidSchemaId, planSplitIntoParts, SPLIT_PARTS_SOURCE_REF, type SplitPartsRequest } from "./splitIntoParts";

const request = (pipeId: string, parts: string, extra: Partial<SplitPartsRequest> = {}): SplitPartsRequest => ({
  ...emptySplitPartsRequest(pipeId), parts, nodeIdPattern: "node:S{k}", nodeLabelPattern: "S{k}",
  pipeIdPattern: "pipe:S{k}", pipeLabelPattern: "S-{k}", provenance: "invented split", firstNumber: "10", step: "10", ...extra
});

const PIPE_S = {
  id: "pipe:S", label: "S", from: "node:A", to: "node:B",
  section: { outside_diameter: { value: 4.5, unit: "in" }, wall_thickness: { value: 0.237, unit: "in" } },
  material: "material:steel", y_reference: { x: 0, y: 1, z: 0 }, provenance: "invented B5 split test input", tags: ["drain"]
};

function split(k: number, pipe: Record<string, unknown>, fraction: number, nodeNumber: number, before: string): EditorOperationIntent {
  const payload = { pipe_ref: pipe.id, fraction, new_node: { id: `node:S${nodeNumber}`, label: `S${nodeNumber}`, provenance: "invented split" }, new_pipe: { id: `pipe:S${nodeNumber}`, label: `S-${nodeNumber}`, provenance: "invented split" } };
  const intent = makeRichIntent({ object_type: "Element", ref: pipe.id as string }, "split_pipe_run", "pipe_segments", before, payload, `split pipe run (part ${k} of 3)`);
  intent.operation_id = `op:b:split-${k}`;
  intent.change.change_id = `change:b:split-${k}`;
  intent.source!.source_ref = SPLIT_PARTS_SOURCE_REF;
  intent.rationale = `Equal-part split of pipe:S into 3: node node:S${nodeNumber} at k = ${k}; geometry only.`;
  return intent;
}

describe("equal-part split: the batch", () => {
  it("equals a hand-built batch: two splits and one exact placement where the second split rounds differently", async () => {
    const model = splitPartsModel();
    const batch = await buildSplitIntoPartsBatch(model, request("pipe:S", "3"), canonicalJsonString, "b");
    const second = { ...PIPE_S, id: "pipe:S10", label: "S-10", provenance: "invented split", from: "node:S10" };
    const expected = [
      split(1, PIPE_S, 0.3333333333333333, 10, await canonicalJsonString(PIPE_S)),
      split(2, second, 0.5, 20, await canonicalJsonString(second)),
      {
        operation_id: "op:b:place-2-x", operation_kind: "modify", operation_status: "proposed", author_type: "user",
        source: { source_ref: SPLIT_PARTS_SOURCE_REF, source_channel: "local_desktop_preview", source_role: "gui_editor" },
        target: { object_type: "Node", ref: "node:S20" },
        change: {
          change_id: "change:b:place-2-x", change_kind: "set_field", field_label: "X position (equal-part split, exact)", field_path: "position.x",
          before: "4.9", after: "{\"value\":4.8999999999999995,\"unit\":\"ft\"}", unit: "ft", dimension: "length",
          source_note: "Equal-part split: the stated position A + (B − A) · (k / N), where the step's split rounded differently."
        },
        validation: { schema_validation: "not_run", constraint_validation: "not_run", unit_validation: "not_run", diff_preview_status: "not_generated", application_status: "not_applied" },
        audit_boundary: { mutation_route: "structured_operations_only", direct_model_mutation_allowed: false, requires_user_acceptance: true, mutates_accepted_model_state: false },
        professional_boundary: { human_review_required: true, software_makes_compliance_claim: false, software_makes_certification_claim: false, software_makes_sealing_claim: false, software_makes_approval_claim: false, software_makes_authentication_claim: false },
        rationale: "Places node:S20 at the stated equal-part position; geometry only."
      }
    ];
    expect(batch).toEqual({ batch_id: "b", operations: expected });
    expect(model).toEqual(splitPartsModel());
  });

  it("plans the stated positions (hand values) and the pieces in order", async () => {
    const planned = await planSplitIntoParts(splitPartsModel(), request("pipe:S", "3"));
    expect(planned.ok).toBe(true);
    if (!planned.ok) return;
    const bits = (v: number[]) => v.map((x) => Object.is(x, 0) ? "+0" : String(x));
    expect(planned.nodes.map((n) => [n.id, n.label, ...bits([n.position.x, n.position.y, n.position.z])])).toEqual([
      ["node:S10", "S10", "2.5", "-0.3999999999999999", "8.235666666666667"],
      ["node:S20", "S20", "4.8999999999999995", "-2.05", "4.126333333333333"]
    ]);
    expect(planned.pieces.map((p) => [p.id, p.from, p.to])).toEqual([
      ["pipe:S", "node:A", "node:S10"], ["pipe:S10", "node:S10", "node:S20"], ["pipe:S20", "node:S20", "node:B"]
    ]);
    expect(planned.corrections).toBe(1);
  });
});

describe("equal-part split: refusals queue nothing", () => {
  const refused = async (model: PreviewModel, req: SplitPartsRequest, text: string) => {
    const planned = await planSplitIntoParts(model, req);
    expect(planned.ok).toBe(false);
    if (!planned.ok) expect(planned.reason).toContain(text);
    await expect(buildSplitIntoPartsBatch(model, req, canonicalJsonString, "b")).rejects.toThrow(text);
  };

  it.each([
    ["one part", request("pipe:H", "1"), "whole number of at least 2"],
    ["a fraction of parts", request("pipe:H", "2.5"), "whole number of at least 2"],
    ["too many parts", request("pipe:H", "101"), "at most 100 parts"],
    ["no {k} in the node id", request("pipe:H", "3", { nodeIdPattern: "node:S" }), "Node ID pattern must contain {k}"],
    ["no {k} in the pipe id", request("pipe:H", "3", { pipeIdPattern: "pipe:S" }), "Pipe ID pattern must contain {k}"],
    ["a blank label", request("pipe:H", "3", { nodeLabelPattern: " " }), "Node label pattern is required"],
    ["blank provenance", request("pipe:H", "3", { provenance: "" }), "Provenance is required"],
    ["a zero step", request("pipe:H", "3", { step: "0" }), "Number step must be a whole number of at least 1"],
    ["an invalid id", request("pipe:H", "3", { nodeIdPattern: "9node{k}" }), "9node10 is not a valid ID"],
    ["ids that collide with each other", request("pipe:H", "3", { pipeIdPattern: "node:S{k}" }), "node:S10 is already used"],
    ["an unknown pipe", request("pipe:missing", "3"), "does not resolve"]
  ])("%s", async (_name, req, text) => {
    await refused(splitPartsModel(), req, text);
  });

  it("an id the model uses", async () => {
    const model = splitPartsModel();
    model.materials!.push({ id: "pipe:S20" } as never);
    await refused(model, request("pipe:H", "3"), "pipe:S20 is already used");
  });

  it("an id taken by a nested record (global ids, as the applier reads them)", async () => {
    const model = splitPartsModel();
    model.load_cases[0].primitive_loads = [{ id: "node:S10" } as never];
    await refused(model, request("pipe:H", "3"), "node:S10 is already used");
  });

  it("a pipe member the split does not understand (record_shape)", async () => {
    const model = splitPartsModel();
    (model.pipe_segments[3] as unknown as Record<string, unknown>).operating_set = "x";
    await refused(model, request("pipe:H", "3"), "unsupported member operating_set");
  });

  it("a pipe load names the pipe (attachments)", async () => {
    const model = splitPartsModel();
    model.load_cases[0].primitive_loads = [{ id: "load:1", target: { pipe: "pipe:H" } } as never];
    await refused(model, request("pipe:H", "3"), "load_cases[0].primitive_loads[0].target.pipe");
  });

  it("a realized bend names the pipe; a component names an end node", async () => {
    const model = splitPartsModel();
    model.components.push({ id: "component:BEND", label: "Bend", kind: "elbow", node: "node:A", geometry: { bend_pipe_ref: "pipe:H" } } as never);
    await refused(model, request("pipe:H", "3"), "components[1].geometry.bend_pipe_ref");
  });

  it("pieces no longer than the axis tolerance (frame), measured in metres", async () => {
    const model = splitPartsModel();
    model.nodes[4].position = { x: 20 + 1e-11, y: -3.7, z: 9 };
    await refused(model, request("pipe:H", "8"), "axis tolerance");
  });

  it("a y_reference parallel to the pipe (frame)", async () => {
    const model = splitPartsModel();
    model.pipe_segments[3].y_reference = { x: 1, y: 0, z: 0 };
    await refused(model, request("pipe:H", "3"), "parallel to its y_reference");
  });

  it("no project length unit", async () => {
    const model = splitPartsModel();
    delete (model.project.units as Record<string, unknown>).length;
    await refused(model, request("pipe:H", "3"), "project length unit");
  });

  it("schema ids follow the applier's rule", () => {
    expect(["node:S10", "a", "N_1.2-3:x"].every(isValidSchemaId)).toBe(true);
    expect(["", "1a", ":a", "a b", "a/b", "é"].some(isValidSchemaId)).toBe(false);
  });
});
