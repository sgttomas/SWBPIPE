import type { EditorOperationIntent, PreviewModel } from "../../types";
import type { OperationBatch } from "../../services/operationBatchService";
import { canonicalJsonString } from "../../services/hashService";
import { makeRichIntent } from "../rich-authoring/formSupport";
import { number, text } from "./batchFormSupport";

export type NewIdentity = { id: string; label: string; provenance: string };
export type CopyIdentity = NewIdentity & { source_ref: string };
export type GeometryDraft = {
  action: "" | "split" | "translate" | "rotate" | "mirror" | "reconnect";
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
  /** Reconnect (W-10): which end of the one source pipe moves. */
  reconnectEnd: "" | "from" | "to";
  /** Reconnect: an existing target node, unless `toNewPoint`. */
  targetNode: string;
  /** Reconnect to a new point: create_node (newNode identity at newPoint), then reconnect. */
  toNewPoint: boolean;
  newPoint: { x: string; y: string; z: string };
  /** Optional y_reference; all three blank keeps the pipe's own. */
  yReference: { x: string; y: string; z: string };
  /** Append delete_node for the node the pipe leaves (offered only when nothing else names it). */
  deleteOrphan: boolean;
};
export const emptyIdentity = (): NewIdentity => ({ id: "", label: "", provenance: "" });
export const emptyGeometryDraft = (): GeometryDraft => ({
  action: "", pipeRefs: [], mode: "", coordinateSystem: "", fraction: "",
  newNode: emptyIdentity(), newPipe: emptyIdentity(),
  vector: { x: "", y: "", z: "", unit: "" }, direction: { x: "", y: "", z: "" }, angle: { value: "", unit: "" },
  copyNodes: [], copyPipes: [],
  reconnectEnd: "", targetNode: "", toNewPoint: false, newPoint: { x: "", y: "", z: "" },
  yReference: { x: "", y: "", z: "" }, deleteOrphan: false
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
export const RECONNECT_CHANGE_KIND = "reconnect_pipe_run";
/** The reconnect payload (DEL-16-06 Design `pipe-reconnect.md` §1): the engine enforces every rule. */
export function reconnectPayload(model: PreviewModel, draft: GeometryDraft): Record<string, unknown> {
  if (draft.pipeRefs.length !== 1) throw new Error("Reconnect requires exactly one source pipe.");
  const [pipe] = selectedGeometry(model, draft.pipeRefs).pipe_segments;
  if (draft.reconnectEnd !== "from" && draft.reconnectEnd !== "to") throw new Error("Choose which end of the pipe moves (from or to).");
  const target = draft.toNewPoint ? text(draft.newNode.id, "New node ID") : text(draft.targetNode, "Target node");
  if (draft.toNewPoint) {
    identity(draft.newNode, "New node");
    for (const axis of ["x", "y", "z"] as const) number(draft.newPoint[axis], `New point ${axis.toUpperCase()}`);
  } else if (!model.nodes.some(n => n.id === target)) throw new Error(`Target node ${target} is not a node of the model.`);
  const payload: Record<string, unknown> = { pipe_ref: pipe.id, end: draft.reconnectEnd, target_node_ref: target };
  const y = draft.yReference;
  const entered = [y.x, y.y, y.z].filter(v => v.trim()).length;
  if (entered === 3) payload.y_reference = { x: number(y.x, "Y reference X"), y: number(y.y, "Y reference Y"), z: number(y.z, "Y reference Z") };
  else if (entered !== 0) throw new Error("Enter all three y_reference components, or leave all three blank to keep the pipe's own.");
  return payload;
}
/** The node the reconnect leaves, when "Also delete node" can be offered:
 * no other pipe ends there and the engine's node-reference set (supports,
 * component nodes, region terminals, nodal primitive loads) names nothing. */
export function orphanCleanupNode(model: PreviewModel, draft: GeometryDraft): string | null {
  if (draft.action !== "reconnect" || draft.pipeRefs.length !== 1 || (draft.reconnectEnd !== "from" && draft.reconnectEnd !== "to")) return null;
  const pipe = model.pipe_segments.find(p => p.id === draft.pipeRefs[0]);
  if (!pipe) return null;
  const old = pipe[draft.reconnectEnd];
  if (model.pipe_segments.some(p => p.id !== pipe.id && (p.from === old || p.to === old))) return null;
  if (model.supports.some(s => s.node === old) || model.components.some(c => c.node === old)) return null;
  for (const lc of model.load_cases) {
    if (lc.pressure_regions?.some(r => r.terminals.some(t => t.node_ref === old))) return null;
    if (lc.primitive_loads?.some(l => (l.target as { node?: string } | undefined)?.node === old)) return null;
  }
  return old;
}
const BOUNDARY = {
  audit_boundary: { mutation_route: "structured_operations_only", direct_model_mutation_allowed: false, requires_user_acceptance: true, mutates_accepted_model_state: false },
  professional_boundary: {
    human_review_required: true,
    software_makes_compliance_claim: false,
    software_makes_certification_claim: false,
    software_makes_sealing_claim: false,
    software_makes_approval_claim: false,
    software_makes_authentication_claim: false
  }
} as const;
const SOURCE = { source_ref: "apps/desktop/src/features/geometry-tools/GeometryToolsPanel.tsx", source_channel: "local_desktop_preview", source_role: "gui_editor" };
const PENDING = { schema_validation: "not_run", constraint_validation: "not_run", unit_validation: "not_run", diff_preview_status: "not_generated", application_status: "not_applied" };
function createNodeIntent(model: PreviewModel, draft: GeometryDraft, id: string): EditorOperationIntent {
  const node = identity(draft.newNode, "New node");
  const position = { x: number(draft.newPoint.x, "New point X"), y: number(draft.newPoint.y, "New point Y"), z: number(draft.newPoint.z, "New point Z") };
  return {
    operation_id: `op:${id}:create-node`, operation_kind: "create", operation_status: "proposed", author_type: "user", source: { ...SOURCE },
    target: { object_type: "Node", ref: node.id },
    change: { change_id: `change:${id}:create-node`, change_kind: "create_node", field_label: "Reconnect target node", field_path: "nodes", before: "not_present",
      after: JSON.stringify({ id: node.id, label: node.label, position, provenance: node.provenance }), unit: model.project.units.length, dimension: "length",
      source_note: "Explicit user-entered reconnect target point in project length units." },
    validation: { ...PENDING }, ...structuredClone(BOUNDARY),
    rationale: `Explicit new node ${node.id} for a reconnect; engine validation required.`
  } as EditorOperationIntent;
}
function deleteNodeIntent(model: PreviewModel, nodeId: string, id: string): EditorOperationIntent {
  const node = model.nodes.find(n => n.id === nodeId);
  if (!node) throw new Error(`Node ${nodeId} no longer resolves.`);
  return {
    operation_id: `op:${id}:delete-node`, operation_kind: "delete", operation_status: "proposed", author_type: "user", source: { ...SOURCE },
    target: { object_type: "Node", ref: nodeId },
    change: { change_id: `change:${id}:delete-node`, change_kind: "delete_node", field_label: "Delete the node the reconnect leaves", field_path: "nodes",
      before: `${node.label}; x=${node.position.x}; y=${node.position.y}; z=${node.position.z}`, after: "not_present", unit: "none", dimension: "dimensionless",
      source_note: "Explicit user choice to delete the node a reconnect leaves with no pipe." },
    validation: { ...PENDING }, ...structuredClone(BOUNDARY),
    rationale: `Explicit deletion of ${nodeId}, left with no pipe by the reconnect in the same batch.`
  } as EditorOperationIntent;
}
/** [create_node?, reconnect_pipe_run, delete_node?]: one reviewed batch. */
export async function buildReconnectBatch(model: PreviewModel, draft: GeometryDraft, id: string): Promise<OperationBatch> {
  const payload = reconnectPayload(model, draft);
  const [pipe] = selectedGeometry(model, draft.pipeRefs).pipe_segments;
  const intent = makeRichIntent({ object_type: "Element", ref: pipe.id }, RECONNECT_CHANGE_KIND, "pipe_segments", await canonicalJsonString(pipe), payload, "reconnect pipe end");
  intent.operation_id = `op:${id}:reconnect`;
  intent.change.change_id = `change:${id}:reconnect`;
  intent.source!.source_ref = SOURCE.source_ref;
  intent.rationale = `Explicit reconnect of the ${draft.reconnectEnd} end of ${pipe.id} to ${String(payload.target_node_ref)}; attachments and references must pass engine validation.`;
  const operations: EditorOperationIntent[] = [];
  if (draft.toNewPoint) operations.push(createNodeIntent(model, draft, id));
  operations.push(intent);
  const orphan = draft.deleteOrphan ? orphanCleanupNode(model, draft) : null;
  if (orphan) operations.push(deleteNodeIntent(model, orphan, id));
  return { batch_id: id, operations };
}
export function geometryPayload(model: PreviewModel, draft: GeometryDraft): Record<string, unknown> {
  if (draft.action === "reconnect") return reconnectPayload(model, draft);
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
  if (draft.action === "reconnect") return buildReconnectBatch(model, draft, id);
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
