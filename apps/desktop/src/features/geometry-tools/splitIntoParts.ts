// B5 (DEL-07-01 Design `split-and-slope-entry.md` §2): split one pipe into N equal parts in one
// reviewed batch of existing operations. Nothing here writes the model.
//
// Operations, for k = 1 … N−1 in order:
//   - `split_pipe_run` on the piece that still ends at the original end B (k = 1: the source
//     pipe), at fraction g_k = 1 / (N − k + 1). The applier places its node at
//     S_k = n_{k−1} + (B − n_{k−1}) · g_k (geometry_operations.rs `split`).
//   - `set_field` Node `position.x|y|z` on the new node, for each component where S_k is not
//     bit-identical to the stated position P_k (the inspector's node position edit).
// The stated position is, per component, in binary64 with round to nearest:
//   P_k = A + (B − A) · (k / N)
// which is exactly what one existing split of the original pipe at fraction k / N would store.
// −0 becomes +0. Node k is never computed from node k−1, so rounding does not accumulate.
//
// Before queueing, the planner refuses what the applier refuses, from the same rules:
// `record_shape`, `endpoints`, `attachments` (split = true), `frame`, and `metadata` (ids
// valid and globally unique). Each split is checked on the model as it stands at that step.
import type { OperationBatch } from "../../services/operationBatchService";
import type { EditorOperationIntent, PreviewModel } from "../../types";
import { makeRichIntent } from "../rich-authoring/formSupport";
import { convertDisplayQuantities } from "../../services/displayQuantityService";
import { AXES, normalizeZero, type Axis, type Vec3 } from "./directionArithmetic";

export const SPLIT_PARTS_SOURCE_REF = "apps/desktop/src/features/geometry-tools/SplitIntoPartsPanel.tsx";
/** A bound on the batch size for review, not an engineering limit. */
export const MAX_PARTS = 100;
export const NUMBER_TOKEN = "{k}";

export type SplitPartsRequest = {
  pipeId: string;
  parts: string;
  nodeIdPattern: string;
  nodeLabelPattern: string;
  pipeIdPattern: string;
  pipeLabelPattern: string;
  provenance: string;
  firstNumber: string;
  step: string;
};
export const emptySplitPartsRequest = (pipeId = ""): SplitPartsRequest => ({
  pipeId, parts: "", nodeIdPattern: "", nodeLabelPattern: "", pipeIdPattern: "", pipeLabelPattern: "", provenance: "", firstNumber: "1", step: "1"
});

export type PlannedNode = { k: number; id: string; label: string; position: Vec3 };
export type PlannedPiece = { id: string; label: string; from: string; to: string };
export type PlannedCorrection = { node: string; axis: Axis; split: number; stated: number };
export type PlannedStep = {
  /** The pipe this split acts on, as it stands before the split (its canonical record is the `before`). */
  source: Record<string, unknown>;
  fraction: number;
  node: { id: string; label: string; provenance: string };
  pipe: { id: string; label: string; provenance: string };
  corrections: PlannedCorrection[];
};
export type SplitPartsPlan =
  | { ok: false; reason: string }
  | { ok: true; unit: string; parts: number; nodes: PlannedNode[]; pieces: PlannedPiece[]; steps: PlannedStep[]; corrections: number };

type Json = null | boolean | number | string | Json[] | { [key: string]: Json };
type JsonObject = { [key: string]: Json };
type Seg = string | number;

// --- Mirrors of geometry_operations.rs (the applier's rules) -------------------------------------

/** model_data_authoring.rs MODEL_DATA_COLLECTIONS: not attachments. */
const MODEL_DATA_COLLECTIONS = ["lines", "unresolved_assumptions", "constraints", "equipment_interfaces", "rule_pack_refs"];
/** Text metadata and annotations are not links (`reference_segments`). */
const NOT_LINKS = ["label", "name", "provenance", "tags", "notes", "provenance_record"];
const PIPE_REQUIRED = ["id", "from", "to", "section", "material"];
const PIPE_OPTIONAL = ["label", "provenance", "y_reference", "section_ref", "tags", "notes"];
const SECTION_REQUIRED = ["outside_diameter", "wall_thickness"];
const SECTION_OPTIONAL = ["mill_tolerance", "corrosion_allowance", "material_density", "contents_density", "insulation_thickness", "insulation_density"];
const NODE_REQUIRED = ["id", "position"];
const NODE_OPTIONAL = ["label", "provenance", "tags", "notes"];
/** The frame kernel's AXIS_TOLERANCE, in metres (`normalized`). */
const AXIS_TOLERANCE_M = 1e-12;

const isObject = (value: unknown): value is JsonObject => typeof value === "object" && value !== null && !Array.isArray(value);
const nonblank = (value: unknown): value is string => typeof value === "string" && value.trim() !== "";
const byteOrder = (a: string, b: string) => a < b ? -1 : a > b ? 1 : 0;

/** `is_valid_schema_id` (lib.rs). */
export function isValidSchemaId(value: string): boolean {
  return /^[A-Za-z][A-Za-z0-9_.:-]*$/.test(value);
}

function fields(value: unknown, required: string[], optional: string[], what: string) {
  if (!isObject(value)) throw new Error(`${what} is not an explicit object.`);
  const keys = Object.keys(value);
  const missing = required.filter((key) => !keys.includes(key));
  const unknown = keys.filter((key) => !required.includes(key) && !optional.includes(key));
  if (missing.length || unknown.length) {
    throw new Error(`${what} cannot be split: ${[missing.length ? `missing ${missing.join(", ")}` : "", unknown.length ? `unsupported member ${unknown.join(", ")}` : ""].filter(Boolean).join("; ")}.`);
  }
}

function finiteNumber(value: unknown): value is number {
  return typeof value === "number" && Number.isFinite(value);
}

function vector(value: unknown, what: string): Vec3 {
  fields(value, ["x", "y", "z"], [], what);
  const record = value as JsonObject;
  if (!AXES.every((axis) => finiteNumber(record[axis]))) throw new Error(`${what} needs finite x, y and z.`);
  return { x: record.x as number, y: record.y as number, z: record.z as number };
}

/** `record_shape` for a pipe (node = false) or a node (node = true). */
function recordShape(record: unknown, node: boolean, what: string) {
  if (!isObject(record)) throw new Error(`${what} is not an explicit object.`);
  for (const key of ["label", "provenance"]) {
    if (record[key] !== undefined && record[key] !== null && typeof record[key] !== "string") throw new Error(`${what}: ${key} must be text or null.`);
  }
  if (node) {
    fields(record, NODE_REQUIRED, NODE_OPTIONAL, what);
    vector(record.position, `${what} position`);
    return;
  }
  fields(record, PIPE_REQUIRED, PIPE_OPTIONAL, what);
  fields(record.section, SECTION_REQUIRED, SECTION_OPTIONAL, `${what} section`);
  for (const [key, quantity] of Object.entries(record.section as JsonObject)) {
    if (key === "corrosion_allowance" || quantity !== null) {
      fields(quantity, ["value", "unit"], [], `${what} section ${key}`);
      const q = quantity as JsonObject;
      if (!finiteNumber(q.value) || !nonblank(q.unit)) throw new Error(`${what} section ${key} needs a finite value and a unit.`);
    }
  }
  for (const key of ["material", "from", "to"]) if (!nonblank(record[key])) throw new Error(`${what}: ${key} must be nonblank text.`);
  if (record.section_ref !== undefined && !nonblank(record.section_ref)) throw new Error(`${what}: section_ref must be nonblank text.`);
}

/** `render_suffix`: `.key`, `[i]`, and `["key"]` for a key that is not a plain identifier. */
function renderSuffix(path: Seg[]): string {
  return path.map((seg) => typeof seg === "number" ? `[${seg}]`
    : seg !== "" && /^[A-Za-z0-9_:-]+$/.test(seg) ? `.${seg}` : `[${JSON.stringify(seg)}]`).join("");
}

/** `reference_segments`: every exact occurrence of `ids` below `value`, skipping text metadata. */
function referenceSegments(value: unknown, ids: ReadonlySet<string>, path: Seg[], out: Seg[][]) {
  if (typeof value === "string") { if (ids.has(value)) out.push([...path]); return; }
  if (Array.isArray(value)) { value.forEach((item, index) => referenceSegments(item, ids, [...path, index], out)); return; }
  if (isObject(value)) {
    for (const key of Object.keys(value).sort(byteOrder)) {
      if (!NOT_LINKS.includes(key)) referenceSegments(value[key], ids, [...path, key], out);
    }
  }
}

function references(value: unknown, ids: ReadonlySet<string>, prefix: string, out: string[]) {
  const hits: Seg[][] = [];
  referenceSegments(value, ids, [], hits);
  out.push(...hits.map((segs) => `${prefix}${renderSuffix(segs)}`));
}

/** `attachments(model, [pipe], endpoints, split = true, in_place = false)`: the references a
 * geometry-only split cannot repartition, as the applier names them. */
export function splitAttachmentBlockers(model: PreviewModel, pipe: { id: string; from: string; to: string }): string[] {
  const root = model as unknown as JsonObject;
  const pipeIds = new Set([pipe.id]);
  const nodeIds = new Set([pipe.from, pipe.to]);
  const all = new Set([...pipeIds, ...nodeIds]);
  const found: string[] = [];
  for (const key of Object.keys(root).sort(byteOrder)) {
    if (MODEL_DATA_COLLECTIONS.includes(key)) continue;
    const collection = root[key];
    if (key === "nodes") {
      for (const node of model.nodes) if (!nodeIds.has(node.id)) references(node, all, "nodes", found);
      continue;
    }
    if (key === "pipe_segments") {
      for (const other of model.pipe_segments) {
        if (pipeIds.has(other.id)) continue;
        const rest: Record<string, unknown> = { ...other };
        delete rest.from;
        delete rest.to;
        references(rest, all, key, found);
      }
      continue;
    }
    references(collection, key !== "components" ? pipeIds : all, key, found);
  }
  return found;
}

/** `global_ids`: every `id` text anywhere in the model. */
export function globalIds(value: unknown, ids = new Set<string>()): Set<string> {
  if (Array.isArray(value)) value.forEach((item) => globalIds(item, ids));
  else if (isObject(value)) {
    if (typeof value.id === "string") ids.add(value.id);
    Object.values(value).forEach((item) => globalIds(item, ids));
  }
  return ids;
}

/** Converts many lengths at once through the engine's convert_for_dimension. */
export type ConvertLengths = (values: readonly number[], from: string, to: string) => Promise<number[]>;
export const engineConvertLengths: ConvertLengths = async (values, from, to) => {
  if (values.length === 0) return [];
  const items = await convertDisplayQuantities(values.map((value, index) => ({ id: String(index), value, from_unit: from, to_unit: to, dimension_id: "length" })));
  return values.map((_, index) => {
    const item = items[index];
    if (!item || item.status !== "converted" || item.id !== String(index)) throw new Error(`Cannot convert lengths from ${from} to ${to}: ${item && item.status === "unavailable" ? item.message : "conversion unavailable"}.`);
    return item.value;
  });
};

type FrameCheck = { from: Vec3; to: Vec3; yReference: unknown; what: string };

/** `frame` (require_y = false) for each check, in order: the span converted to metres must be
 * longer than the axis tolerance, and a y_reference, when present, must not be parallel to it.
 * The arithmetic is the applier's: x = m · (1 / |m|), y − x · (y · x). */
async function frameIssues(checks: readonly FrameCheck[], unit: string, convert: ConvertLengths): Promise<string | null> {
  const deltas = checks.flatMap((check) => AXES.map((axis) => check.to[axis] - check.from[axis]));
  const metres = await convert(deltas, unit, "m");
  for (const [index, check] of checks.entries()) {
    const [mx, my, mz] = metres.slice(index * 3, index * 3 + 3);
    if (![mx, my, mz].every(Number.isFinite)) return `${check.what}: the span cannot be converted to metres.`;
    const norm = Math.sqrt(mx * mx + my * my + mz * mz);
    if (!Number.isFinite(norm) || norm <= AXIS_TOLERANCE_M) return `${check.what} would be no longer than the engine's axis tolerance (1e-12 m).`;
    if (check.yReference !== undefined) {
      const inverse = 1 / norm;
      const x = { x: mx * inverse, y: my * inverse, z: mz * inverse };
      const y = vector(check.yReference, `${check.what} y_reference`);
      const along = y.x * x.x + y.y * x.y + y.z * x.z;
      const p = { x: y.x - x.x * along, y: y.y - x.y * along, z: y.z - x.z * along };
      const pn = Math.sqrt(p.x * p.x + p.y * p.y + p.z * p.z);
      if (!Number.isFinite(pn) || pn <= AXIS_TOLERANCE_M) return `${check.what} would be parallel to its y_reference.`;
    }
  }
  return null;
}

// --- The plan ------------------------------------------------------------------------------------

function wholeNumber(text: string, label: string, min: number): number {
  const value = Number(text);
  if (!text.trim() || !Number.isInteger(value) || value < min) throw new Error(`${label} must be a whole number of at least ${min}.`);
  return value;
}

/** Each pattern's `{k}` is replaced by first + (k − 1) · step. */
export function patternText(pattern: string, number: number): string {
  return pattern.trim().replaceAll(NUMBER_TOKEN, String(number));
}

/** The stated position of node k: A + (B − A) · (k / N) per component, −0 as +0. */
export function statedSplitPosition(a: Vec3, b: Vec3, k: number, parts: number): Vec3 {
  const t = k / parts;
  return { x: normalizeZero(a.x + (b.x - a.x) * t), y: normalizeZero(a.y + (b.y - a.y) * t), z: normalizeZero(a.z + (b.z - a.z) * t) };
}

/** What the applier's split stores: from + (to − from) · fraction per component (no −0 handling). */
export function appliedSplitPosition(from: Vec3, to: Vec3, fraction: number): Vec3 {
  return { x: from.x + (to.x - from.x) * fraction, y: from.y + (to.y - from.y) * fraction, z: from.z + (to.z - from.z) * fraction };
}

function requestIssue(model: PreviewModel, request: SplitPartsRequest) {
  if (!model.project.units.length?.trim()) throw new Error("The project length unit is not set.");
  if (!request.pipeId) throw new Error("Choose the pipe to split.");
  const parts = wholeNumber(request.parts, "Number of parts", 2);
  if (parts > MAX_PARTS) throw new Error(`Split into at most ${MAX_PARTS} parts in one batch.`);
  for (const [pattern, label] of [[request.nodeIdPattern, "Node ID pattern"], [request.pipeIdPattern, "Pipe ID pattern"]] as const) {
    if (!pattern.includes(NUMBER_TOKEN)) throw new Error(`${label} must contain ${NUMBER_TOKEN}, which is replaced by each part's number.`);
  }
  if (!request.nodeLabelPattern.trim()) throw new Error("Node label pattern is required.");
  if (!request.pipeLabelPattern.trim()) throw new Error("Pipe label pattern is required.");
  if (!request.provenance.trim()) throw new Error("Provenance is required.");
  const first = wholeNumber(request.firstNumber, "First number", 0);
  const step = wholeNumber(request.step, "Number step", 1);
  return { parts, first, step };
}

/** The plan for a request on the current model, or the first reason it is refused. */
export async function planSplitIntoParts(model: PreviewModel, request: SplitPartsRequest, convert: ConvertLengths = engineConvertLengths): Promise<SplitPartsPlan> {
  try {
    return await plan(model, request, convert);
  } catch (error) {
    return { ok: false, reason: error instanceof Error ? error.message : String(error) };
  }
}

async function plan(model: PreviewModel, request: SplitPartsRequest, convert: ConvertLengths): Promise<SplitPartsPlan> {
  const { parts, first, step } = requestIssue(model, request);
  const unit = model.project.units.length!.trim();
  const matches = model.pipe_segments.filter((pipe) => pipe.id === request.pipeId);
  if (matches.length !== 1) throw new Error(`Pipe ${request.pipeId} does not resolve to exactly one pipe.`);
  const source = matches[0] as unknown as JsonObject;
  recordShape(source, false, `Pipe ${request.pipeId}`);
  const endpoint = (id: string) => {
    const nodes = model.nodes.filter((node) => node.id === id);
    if (nodes.length !== 1) throw new Error(`Endpoint ${id} does not resolve to exactly one node.`);
    recordShape(nodes[0], true, `Node ${id}`);
    // The engine reads the JSON the desktop sends, where −0 is written as 0.
    const position = vector(nodes[0].position, `Node ${id} position`);
    return { x: normalizeZero(position.x), y: normalizeZero(position.y), z: normalizeZero(position.z) };
  };
  const fromId = source.from as string;
  const toId = source.to as string;
  const a = endpoint(fromId);
  const b = endpoint(toId);

  // Identities: valid, distinct, and new to the whole model.
  const taken = globalIds(model);
  const identity = (pattern: string, labelPattern: string, k: number) => {
    const number = first + (k - 1) * step;
    return { id: patternText(pattern, number), label: patternText(labelPattern, number), provenance: request.provenance.trim() };
  };
  const steps: PlannedStep[] = [];
  const identities: Array<{ node: PlannedStep["node"]; pipe: PlannedStep["pipe"] }> = [];
  for (let k = 1; k < parts; k++) {
    const node = identity(request.nodeIdPattern, request.nodeLabelPattern, k);
    const pipe = identity(request.pipeIdPattern, request.pipeLabelPattern, k);
    for (const id of [node.id, pipe.id]) {
      if (!isValidSchemaId(id)) throw new Error(`${id} is not a valid ID: it must start with a letter and use only letters, digits, _ . : and -.`);
      if (taken.has(id)) throw new Error(`${id} is already used in the model or in this split.`);
      taken.add(id);
    }
    identities.push({ node, pipe });
  }

  // Simulate each step on the model as it stands, as the batch applies it.
  let current = structuredClone(model) as PreviewModel;
  const positions = new Map<string, Vec3>([[fromId, a], [toId, b]]);
  let piece = structuredClone(source);
  let start = a;
  let startId = fromId;
  const nodes: PlannedNode[] = [];
  const pieces: PlannedPiece[] = [];
  let corrections = 0;
  const frames: FrameCheck[] = [];
  for (let k = 1; k < parts; k++) {
    const pieceId = piece.id as string;
    const blockers = splitAttachmentBlockers(current, { id: pieceId, from: startId, to: toId });
    if (blockers.length) throw new Error(`The split of ${pieceId} would be refused: a geometry-only split cannot repartition ${blockers.join(", ")}.`);
    frames.push({ from: start, to: b, yReference: piece.y_reference, what: `Pipe ${pieceId}` });
    const fraction = 1 / (parts - k + 1);
    const split = appliedSplitPosition(start, b, fraction);
    if (!AXES.every((axis) => Number.isFinite(split[axis]))) throw new Error("A split coordinate would overflow.");
    const { node, pipe } = identities[k - 1];
    frames.push({ from: start, to: split, yReference: piece.y_reference, what: `The piece of ${pieceId} before ${node.id}` },
      { from: split, to: b, yReference: piece.y_reference, what: `The piece from ${node.id}` });
    const stated = statedSplitPosition(a, b, k, parts);
    const stepCorrections: PlannedCorrection[] = AXES.filter((axis) => !Object.is(split[axis], stated[axis]))
      .map((axis) => ({ node: node.id, axis, split: split[axis], stated: stated[axis] }));
    corrections += stepCorrections.length;
    steps.push({ source: structuredClone(piece), fraction, node, pipe, corrections: stepCorrections });

    // The model after the split and the corrections.
    const firstPiece = { ...piece, to: node.id };
    const secondPiece: JsonObject = { ...structuredClone(piece), from: node.id, id: pipe.id, label: pipe.label, provenance: pipe.provenance };
    current = {
      ...current,
      nodes: [...current.nodes, { id: node.id, label: node.label, provenance: node.provenance, position: { ...stated } }],
      pipe_segments: [...current.pipe_segments.map((p) => p.id === pieceId ? firstPiece as unknown as PreviewModel["pipe_segments"][number] : p), secondPiece as unknown as PreviewModel["pipe_segments"][number]]
    } as PreviewModel;
    positions.set(node.id, stated);
    nodes.push({ k, id: node.id, label: node.label, position: stated });
    pieces.push({ id: pieceId, label: String(piece.label ?? pieceId), from: startId, to: node.id });
    piece = secondPiece;
    start = stated;
    startId = node.id;
  }
  pieces.push({ id: piece.id as string, label: String(piece.label ?? piece.id), from: startId, to: toId });
  // The final pieces, at the stated positions.
  for (const p of pieces) frames.push({ from: positions.get(p.from)!, to: positions.get(p.to)!, yReference: source.y_reference, what: `Piece ${p.id}` });
  const frameIssue = await frameIssues(frames, unit, convert);
  if (frameIssue) throw new Error(frameIssue);
  return { ok: true, unit, parts, nodes, pieces, steps, corrections };
}

// --- The batch -----------------------------------------------------------------------------------

const PENDING = { schema_validation: "not_run", constraint_validation: "not_run", unit_validation: "not_run", diff_preview_status: "not_generated", application_status: "not_applied" } as const;

/** The inspector's node position edit (`set_field`, Node, `position.<axis>`), in the project unit. */
function correctionIntent(correction: PlannedCorrection, unit: string, batchId: string, k: number): EditorOperationIntent {
  return {
    operation_id: `op:${batchId}:place-${k}-${correction.axis}`,
    operation_kind: "modify",
    operation_status: "proposed",
    author_type: "user",
    source: { source_ref: SPLIT_PARTS_SOURCE_REF, source_channel: "local_desktop_preview", source_role: "gui_editor" },
    target: { object_type: "Node", ref: correction.node },
    change: {
      change_id: `change:${batchId}:place-${k}-${correction.axis}`,
      change_kind: "set_field",
      field_label: `${correction.axis.toUpperCase()} position (equal-part split, exact)`,
      field_path: `position.${correction.axis}`,
      before: String(correction.split),
      after: JSON.stringify({ value: correction.stated, unit }),
      unit,
      dimension: "length",
      source_note: "Equal-part split: the stated position A + (B − A) · (k / N), where the step's split rounded differently."
    },
    validation: { ...PENDING },
    audit_boundary: { mutation_route: "structured_operations_only", direct_model_mutation_allowed: false, requires_user_acceptance: true, mutates_accepted_model_state: false },
    professional_boundary: {
      human_review_required: true,
      software_makes_compliance_claim: false,
      software_makes_certification_claim: false,
      software_makes_sealing_claim: false,
      software_makes_approval_claim: false,
      software_makes_authentication_claim: false
    },
    rationale: `Places ${correction.node} at the stated equal-part position; geometry only.`
  } as EditorOperationIntent;
}

/** [split_pipe_run, set_field position…] for k = 1 … N−1: one batch, applied whole. */
export async function buildSplitIntoPartsBatch(
  model: PreviewModel,
  request: SplitPartsRequest,
  canonicalize: (value: unknown) => Promise<string>,
  batchId: string,
  convert: ConvertLengths = engineConvertLengths
): Promise<OperationBatch> {
  const planned = await planSplitIntoParts(model, request, convert);
  if (!planned.ok) throw new Error(planned.reason);
  const operations: EditorOperationIntent[] = [];
  for (const [index, step] of planned.steps.entries()) {
    const k = index + 1;
    const pieceId = step.source.id as string;
    const payload = { pipe_ref: pieceId, fraction: step.fraction, new_node: step.node, new_pipe: step.pipe };
    const intent = makeRichIntent({ object_type: "Element", ref: pieceId }, "split_pipe_run", "pipe_segments", await canonicalize(step.source), payload, `split pipe run (part ${k} of ${planned.parts})`);
    intent.operation_id = `op:${batchId}:split-${k}`;
    intent.change.change_id = `change:${batchId}:split-${k}`;
    intent.source!.source_ref = SPLIT_PARTS_SOURCE_REF;
    intent.rationale = `Equal-part split of ${request.pipeId} into ${planned.parts}: node ${step.node.id} at k = ${k}; geometry only.`;
    operations.push(intent, ...step.corrections.map((correction) => correctionIntent(correction, planned.unit, batchId, k)));
  }
  return { batch_id: batchId, operations };
}
