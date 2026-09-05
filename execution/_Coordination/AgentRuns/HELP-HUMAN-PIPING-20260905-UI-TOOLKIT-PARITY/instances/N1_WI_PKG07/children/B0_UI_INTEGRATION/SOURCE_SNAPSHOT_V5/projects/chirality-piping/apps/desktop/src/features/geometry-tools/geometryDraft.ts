import type { PreviewModel } from "../../types";
import type { OperationBatch } from "../../services/operationBatchService";
import { canonicalJsonString } from "../../services/hashService";
import { makeRichIntent } from "../rich-authoring/formSupport";
import { number, text } from "./batchFormSupport";

export type NewIdentity = { id: string; label: string; provenance: string };
export type CopyIdentity = NewIdentity & { source_ref: string };
export type GeometryDraft = {
  action: "" | "split" | "translate" | "rotate" | "mirror";
  pipeRefs: string[];
  mode: "" | "copy" | "in_place";
  coordinateSystem: string;
  fraction: string;
  newNode: NewIdentity;
  newPipe: NewIdentity;
  vector: { x: string; y: string; z: string; unit: string };
  direction: { x: string; y: string; z: string };
  angle: { value: string; unit: string };
  copyNodes: CopyIdentity[];
  copyPipes: CopyIdentity[];
};
export const emptyIdentity = (): NewIdentity => ({ id: "", label: "", provenance: "" });
export const emptyGeometryDraft = (): GeometryDraft => ({
  action: "", pipeRefs: [], mode: "", coordinateSystem: "", fraction: "",
  newNode: emptyIdentity(), newPipe: emptyIdentity(),
  vector: { x: "", y: "", z: "", unit: "" }, direction: { x: "", y: "", z: "" }, angle: { value: "", unit: "" },
  copyNodes: [], copyPipes: []
});
function identity(value: NewIdentity, label: string): NewIdentity {
  return { id: text(value.id, `${label} ID`), label: text(value.label, `${label} label`), provenance: text(value.provenance, `${label} provenance`) };
}
export function selectedGeometry(model: PreviewModel, refs: string[]) {
  if (!refs.length || new Set(refs).size !== refs.length) throw new Error("Select one or more distinct pipe runs.");
  const pipes = refs.map(ref => {
    const matches = model.pipe_segments.filter(p => p.id === ref);
    if (matches.length !== 1) throw new Error(`Pipe ${ref} no longer resolves uniquely.`);
    return matches[0];
  }).sort((a, b) => a.id < b.id ? -1 : a.id > b.id ? 1 : 0);
  const ids = [...new Set(pipes.flatMap(p => [p.from, p.to]))].sort();
  const nodes = ids.map(id => {
    const matches = model.nodes.filter(n => n.id === id);
    if (matches.length !== 1) throw new Error(`Endpoint ${id} no longer resolves uniquely.`);
    return matches[0];
  });
  return { nodes, pipe_segments: pipes };
}
export function geometryPayload(model: PreviewModel, draft: GeometryDraft): Record<string, unknown> {
  const selected = selectedGeometry(model, draft.pipeRefs);
  if (draft.action === "split") {
    if (draft.pipeRefs.length !== 1) throw new Error("Split requires exactly one pipe run.");
    const fraction = number(draft.fraction, "Split fraction");
    if (fraction <= 0 || fraction >= 1) throw new Error("Split fraction must be strictly between 0 and 1.");
    return { pipe_ref: draft.pipeRefs[0], fraction, new_node: identity(draft.newNode, "New node"), new_pipe: identity(draft.newPipe, "New pipe") };
  }
  if (!["translate", "rotate", "mirror"].includes(draft.action)) throw new Error("Choose a geometry tool.");
  if (!["copy", "in_place"].includes(draft.mode)) throw new Error("Choose copy or in-place mode explicitly.");
  if (draft.coordinateSystem !== "global") throw new Error("Confirm the global coordinate system.");
  const vector = { x: number(draft.vector.x, "X"), y: number(draft.vector.y, "Y"), z: number(draft.vector.z, "Z"), unit: text(draft.vector.unit, "Length unit") };
  let transform: Record<string, unknown> = { kind: "translate", translation: vector };
  if (draft.action !== "translate") {
    const direction = { x: number(draft.direction.x, "Direction X"), y: number(draft.direction.y, "Direction Y"), z: number(draft.direction.z, "Direction Z") };
    if (direction.x === 0 && direction.y === 0 && direction.z === 0) throw new Error("Axis or plane normal must be nonzero.");
    transform = draft.action === "rotate"
      ? { kind: "rotate", origin: vector, axis: direction, angle: { value: number(draft.angle.value, "Angle"), unit: text(draft.angle.unit, "Angle unit") } }
      : { kind: "mirror", origin: vector, normal: direction };
  }
  const payload: Record<string, unknown> = { mode: draft.mode, pipe_refs: [...draft.pipeRefs], transform };
  if (draft.mode === "copy") {
    const map = (rows: CopyIdentity[], expected: string[], label: string) => {
      if (rows.length !== expected.length || expected.some(id => rows.filter(r => r.source_ref === id).length !== 1)) throw new Error(`${label} needs an explicit mapping for every source.`);
      return expected.map(id => ({ source_ref: id, ...identity(rows.find(r => r.source_ref === id)!, `${label} ${id}`) }));
    };
    payload.copy_nodes = map(draft.copyNodes, selected.nodes.map(n => n.id), "Copied node");
    payload.copy_pipes = map(draft.copyPipes, selected.pipe_segments.map(p => p.id), "Copied pipe");
  }
  return payload;
}
export async function buildGeometryBatch(model: PreviewModel, draft: GeometryDraft, id: string): Promise<OperationBatch> {
  const payload = geometryPayload(model, draft);
  const source = draft.action === "split" ? selectedGeometry(model, draft.pipeRefs).pipe_segments[0] : selectedGeometry(model, draft.pipeRefs);
  const before = await canonicalJsonString(source);
  const kind = draft.action === "split" ? "split_pipe_run" : "transform_pipe_run";
  const intent = makeRichIntent({ object_type: "Element", ref: draft.pipeRefs[0] }, kind, "pipe_segments", before, payload, `${draft.action} pipe run`);
  intent.operation_id = `op:${id}:geometry`;
  intent.change.change_id = `change:${id}:geometry`;
  intent.source!.source_ref = "apps/desktop/src/features/geometry-tools/GeometryToolsPanel.tsx";
  intent.rationale = `Explicit ${draft.action} geometry only; source pipes ${draft.pipeRefs.join(", ")}; attachments and references must pass engine validation.`;
  return { batch_id: id, operations: [intent] };
}
