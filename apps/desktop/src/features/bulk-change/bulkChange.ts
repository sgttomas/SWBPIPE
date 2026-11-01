// B2 (DEL-07-02 Design `bulk-change.md`): bulk change and replace over a selection. A plan
// reads the selection in the change's own family and sorts every entity it reaches into
// affected, unchanged, not matched and refused. The batch holds one existing operation per
// affected entity, with the wire fields its existing editor writes:
// - section: `assign_section` (toolkit/SectionAssignment.tsx);
// - material: `set_field` Element `material` (the inspector's Material field);
// - support kind: `update_support` Support `configuration` (SupportConfigurationForm.tsx);
// - support removal: `delete_support` (model-tree/removalIntents.ts, reused);
// - bend radius: `set_field` Component `geometry.bend_radius.value` (the inspector's field);
// - tag: the family's annotation kind on `tags` (model-data/modelDataIntents.ts, reused).
// Nothing here writes the model, and nothing is inferred: restraints are kept or entered.
import type { OperationBatch } from "../../services/operationBatchService";
import type { EditorOperationIntent, EditorOperationSource, EntityRef, PreviewModel } from "../../types";
import { buildDeleteSupportIntent } from "../model-tree/removalIntents";
import { ANNOTATED_OBJECT_TYPES, annotatedRecord, annotationIntent, type Canonicalize } from "../model-data/modelDataIntents";
import { makeRichIntent } from "../rich-authoring/formSupport";

type Support = PreviewModel["supports"][number];
type Pipe = PreviewModel["pipe_segments"][number];
type Component = PreviewModel["components"][number];

export const DOFS = ["UX", "UY", "UZ", "RX", "RY", "RZ"] as const;
export type Dof = typeof DOFS[number];

/** Every stored family, for matching. "" matches a support with no family recorded. */
export const SUPPORT_FAMILIES = [
  "anchor", "guide", "line_stop", "vertical_support", "spring", "variable_spring_hanger",
  "spring_hanger", "constant_effort_support", "nonlinear", "snubber"
] as const;
/** The kinds a bulk replacement writes: none needs stiffness, hanger or nonlinear data. */
export const TARGET_SUPPORT_FAMILIES = ["anchor", "guide", "line_stop", "vertical_support"] as const;
export type TargetSupportFamily = typeof TARGET_SUPPORT_FAMILIES[number];

export function familyLabel(family: string): string {
  return family ? family.replaceAll("_", " ") : "no kind recorded";
}

export type BulkChangeRequest =
  | { kind: "section"; sectionId: string }
  | { kind: "material"; materialId: string }
  | { kind: "support-kind"; fromFamily: string; toFamily: string; restraints: "keep" | readonly Dof[] }
  | { kind: "support-remove"; family: string }
  | { kind: "bend-radius"; value: string; unit: string }
  | { kind: "tag"; tag: string };
export type BulkChangeKind = BulkChangeRequest["kind"];

export const BULK_CHANGE_LABELS: Record<BulkChangeKind, string> = {
  section: "Assign a shared section",
  material: "Reassign the material",
  "support-kind": "Replace a support kind",
  "support-remove": "Remove a support kind",
  "bend-radius": "Set the bend radius",
  tag: "Add a tag"
};

export type PlanEntry = { ref: EntityRef; before: string; after: string };
export type ReasonEntry = { ref: EntityRef; reason: string };
export type BulkChangePlan = {
  requestError: string | null;
  affected: PlanEntry[];
  unchanged: ReasonEntry[];
  notMatched: ReasonEntry[];
  refused: ReasonEntry[];
};

export const BULK_SOURCE: EditorOperationSource = {
  source_ref: "apps/desktop/src/features/bulk-change",
  source_channel: "local_desktop_preview",
  source_role: "gui_editor"
};

const CONFIG = ["family", "restraints", "stiffness", "hanger", "nonlinear", "provenance"] as const;
const REALIZED_BEND = "curved_bend_macro_element";
const key = (ref: EntityRef) => `${ref.type}\u0000${ref.id}`;

/** The configuration projection `update_support configuration` replaces (rich_authoring.rs CONFIG). */
export function supportConfiguration(support: Support): Record<string, unknown> {
  const record = support as unknown as Record<string, unknown>;
  return Object.fromEntries(CONFIG.filter((member) => record[member] !== undefined).map((member) => [member, structuredClone(record[member])]));
}

function storedFamily(support: Support): string {
  return typeof support.family === "string" ? support.family : "";
}

function isBend(component: Component): boolean {
  return component.kind === "bend" || component.kind === "elbow";
}

function isRealizedBend(component: Component): boolean {
  const consumption = (component as { mechanics_interface?: { solver_consumption?: string } }).mechanics_interface?.solver_consumption;
  return consumption === REALIZED_BEND;
}

/** Entities reached once each, in selection order. */
class Reach {
  private readonly seen = new Set<string>();
  readonly refs: EntityRef[] = [];
  add(ref: EntityRef) {
    if (this.seen.has(key(ref))) return;
    this.seen.add(key(ref));
    this.refs.push(ref);
  }
}

function pipeOf(model: PreviewModel, id: string): Pipe | undefined {
  return model.pipe_segments.find((pipe) => pipe.id === id);
}

/** Nodes in scope: the selected nodes and both end nodes of the selected pipes, in order. */
function scopeNodes(model: PreviewModel, scope: readonly EntityRef[]): string[] {
  const nodes: string[] = [];
  const add = (id: string) => { if (!nodes.includes(id)) nodes.push(id); };
  for (const ref of scope) {
    if (ref.type === "node") add(ref.id);
    if (ref.type === "pipe") {
      const pipe = pipeOf(model, ref.id);
      if (pipe) { add(pipe.from); add(pipe.to); }
    }
  }
  return nodes;
}

function refusedTypes(scope: readonly EntityRef[], accepted: readonly EntityRef["type"][], what: string): ReasonEntry[] {
  return scope.filter((ref) => !accepted.includes(ref.type)).map((ref) => ({ ref, reason: `${what}; ${ref.type} ${ref.id} is not one.` }));
}

function missing(ref: EntityRef): ReasonEntry {
  return { ref, reason: `${ref.type} ${ref.id} is not in the current model.` };
}

function tagError(tag: string): string | null {
  if (!tag.trim()) return "Enter a tag.";
  if (tag !== tag.trim()) return "A tag has no leading or trailing spaces.";
  if (tag.length > 64) return "A tag is at most 64 characters.";
  return null;
}

function radiusError(value: string, unit: string): string | null {
  const number = Number(value.trim());
  if (!value.trim() || !Number.isFinite(number) || number <= 0) return "Enter a finite positive bend radius.";
  if (!unit.trim()) return "Enter the radius unit.";
  return null;
}

function restraintText(restraints: readonly string[]): string {
  return restraints.length ? restraints.join(", ") : "none";
}

function supportSummary(family: string, restraints: readonly string[]): string {
  return `${familyLabel(family)}; restraints ${restraintText(restraints)}`;
}

/** The same degrees of freedom, whatever their order or spelling (UX, Ux, ux). */
function sameSet(a: readonly string[], b: readonly string[]): boolean {
  const left = new Set(a.map((dof) => dof.toUpperCase()));
  const right = new Set(b.map((dof) => dof.toUpperCase()));
  return left.size === right.size && [...left].every((dof) => right.has(dof));
}

const utf8 = new TextEncoder();
function compareBytes(a: string, b: string): number {
  const left = utf8.encode(a);
  const right = utf8.encode(b);
  for (let index = 0; index < Math.min(left.length, right.length); index += 1) {
    if (left[index] !== right[index]) return left[index] - right[index];
  }
  return left.length - right.length;
}
function sortedUniqueByBytes(items: readonly string[]): string[] {
  return [...items].sort(compareBytes).filter((item, index, sorted) => index === 0 || item !== sorted[index - 1]);
}

/** What `delete_support` refuses on (operation_applier lib.rs `resolve_delete_support`,
 * OP-SUPPORT-DELETE-REFERENCED), in its order: primitive loads targeting the support (sorted by
 * byte and deduplicated, as `support_primitive_load_references` does), supports
 * naming it as their normal-reaction source, and load-case `analysis_state.support_states`
 * entries naming it (load_state_authoring.rs `support_references`). Keep the two in step. */
export function supportDeleteReferences(model: PreviewModel, supportId: string): string[] {
  const references: string[] = [];
  const loadIds: string[] = [];
  const cases = (model.load_cases ?? []) as unknown as Array<Record<string, unknown>>;
  for (const loadCase of cases) {
    const loads = Array.isArray(loadCase.primitive_loads) ? (loadCase.primitive_loads as Array<Record<string, unknown>>) : [];
    for (const load of loads) {
      const target = load.target && typeof load.target === "object" ? (load.target as Record<string, unknown>) : null;
      if (target?.support === supportId) loadIds.push(typeof load.id === "string" ? load.id : "primitive_load:unknown");
    }
  }
  // Rust's `references.sort(); references.dedup();`: byte order of the UTF-8 ids, then adjacent
  // repeats removed. The other two categories keep model order, as in Rust.
  references.push(...sortedUniqueByBytes(loadIds));
  for (const other of model.supports) {
    if (other.nonlinear?.normal_reaction_source?.support_ref === supportId) references.push(typeof other.id === "string" ? other.id : "unknown nonlinear support");
  }
  for (const loadCase of cases) {
    const state = loadCase.analysis_state && typeof loadCase.analysis_state === "object" ? (loadCase.analysis_state as Record<string, unknown>) : null;
    const states = Array.isArray(state?.support_states) ? (state!.support_states as Array<Record<string, unknown>>) : [];
    states.forEach((entry, index) => {
      if (entry?.support_ref === supportId) references.push(`${typeof loadCase.id === "string" ? loadCase.id : "load_case:unknown"}.analysis_state.support_states.${index}.support_ref`);
    });
  }
  return references;
}

/** The change's plan over the scope. Pure: it reads the model and writes nothing. */
export function planBulkChange(model: PreviewModel, scope: readonly EntityRef[], request: BulkChangeRequest): BulkChangePlan {
  const plan: BulkChangePlan = { requestError: null, affected: [], unchanged: [], notMatched: [], refused: [] };
  switch (request.kind) {
    case "section":
    case "material": {
      const isSection = request.kind === "section";
      const chosen = isSection ? request.sectionId : request.materialId;
      if (isSection) {
        const section = model.sections?.find((item) => item.id === chosen);
        if (!section) plan.requestError = "Choose a shared section.";
        else if (section.section_type !== "pipe") plan.requestError = `Section ${chosen} is not a pipe section.`;
      } else if (!model.materials?.some((item) => item.id === chosen)) {
        plan.requestError = "Choose a material.";
      }
      plan.refused.push(...refusedTypes(scope, ["pipe"], isSection ? "A section is assigned to pipes only" : "A material is assigned to pipes only"));
      for (const ref of scope.filter((item) => item.type === "pipe")) {
        const pipe = pipeOf(model, ref.id);
        if (!pipe) { plan.refused.push(missing(ref)); continue; }
        const current = isSection ? pipe.section_ref ?? "" : pipe.material;
        if (current === chosen) plan.unchanged.push({ ref, reason: isSection ? `Already bound to ${chosen}.` : `Already uses ${chosen}.` });
        else plan.affected.push({ ref, before: current || "local dimensions", after: chosen });
      }
      return plan;
    }
    case "support-kind":
    case "support-remove": {
      const replace = request.kind === "support-kind";
      const fromFamily = replace ? request.fromFamily : request.family;
      if (replace) {
        if (!TARGET_SUPPORT_FAMILIES.includes(request.toFamily as TargetSupportFamily)) plan.requestError = "Choose the kind to write.";
        else if (request.restraints !== "keep" && request.restraints.length === 0) plan.requestError = "Tick the restraints to write; none are inferred from the kind.";
      }
      plan.refused.push(...refusedTypes(scope, ["support", "node", "pipe"], "A support change acts on supports, and on the supports at selected nodes and pipe ends"));
      const reach = new Reach();
      for (const ref of scope) if (ref.type === "support") reach.add(ref);
      const nodes = scopeNodes(model, scope);
      for (const node of nodes) for (const support of model.supports) if (support.node === node) reach.add({ type: "support", id: support.id });
      for (const ref of reach.refs) {
        const support = model.supports.find((item) => item.id === ref.id);
        if (!support) { plan.refused.push(missing(ref)); continue; }
        const family = storedFamily(support);
        if (family !== fromFamily) { plan.notMatched.push({ ref, reason: `Its kind is ${familyLabel(family)}.` }); continue; }
        if (support.boundary_association) {
          plan.refused.push({ ref, reason: `Support ${support.id} is an equipment boundary; change it in Boundary authoring.` });
          continue;
        }
        if (!replace) {
          const references = supportDeleteReferences(model, support.id);
          if (references.length) { plan.refused.push({ ref, reason: `Support ${support.id} is still referenced by ${references.join(", ")}; delete or retarget them first.` }); continue; }
          plan.affected.push({ ref, before: supportSummary(family, support.restraints), after: "removed" });
          continue;
        }
        const carried: string[] = [];
        if (support.stiffness !== undefined) carried.push("stiffness");
        if (support.hanger !== undefined) carried.push("hanger");
        if (support.nonlinear !== undefined && support.nonlinear !== null) carried.push("nonlinear");
        if (support.properties !== undefined) carried.push("properties");
        if (carried.length) {
          plan.refused.push({ ref, reason: `Support ${support.id} carries ${carried.join(", ")} data that a kind replacement would discard; change it in Support configuration.` });
          continue;
        }
        const restraints = request.restraints === "keep" ? support.restraints : [...request.restraints];
        if (family === request.toFamily && sameSet(restraints, support.restraints)) {
          plan.unchanged.push({ ref, reason: `Already ${familyLabel(family)} with these restraints.` });
          continue;
        }
        plan.affected.push({ ref, before: supportSummary(family, support.restraints), after: supportSummary(request.toFamily, restraints) });
      }
      return plan;
    }
    case "bend-radius": {
      plan.requestError = radiusError(request.value, request.unit);
      plan.refused.push(...refusedTypes(scope, ["component", "node", "pipe"], "A bend radius is set on components, and on the components at selected nodes and pipe ends"));
      const reach = new Reach();
      for (const ref of scope) if (ref.type === "component") reach.add(ref);
      const nodes = scopeNodes(model, scope);
      const pipes = new Set(scope.filter((ref) => ref.type === "pipe").map((ref) => ref.id));
      for (const component of model.components) {
        const span = component.geometry?.bend_pipe_ref;
        if (nodes.includes(component.node) || (span && pipes.has(span))) reach.add({ type: "component", id: component.id });
      }
      const entered = Number(request.value.trim());
      for (const ref of reach.refs) {
        const component = model.components.find((item) => item.id === ref.id);
        if (!component) { plan.refused.push(missing(ref)); continue; }
        if (!isBend(component)) { plan.notMatched.push({ ref, reason: `Its kind is ${component.kind}.` }); continue; }
        if (isRealizedBend(component)) {
          plan.refused.push({ ref, reason: `${component.id} is a realized bend; its radius forms the arc, so change it in the inspector.` });
          continue;
        }
        const current = component.geometry?.bend_radius;
        if (current && current.value === entered && current.unit === request.unit.trim()) {
          plan.unchanged.push({ ref, reason: `Already ${current.value} ${current.unit}.` });
          continue;
        }
        plan.affected.push({ ref, before: current ? `${current.value} ${current.unit}` : "not entered", after: `${entered} ${request.unit.trim()}` });
      }
      return plan;
    }
    case "tag": {
      plan.requestError = tagError(request.tag);
      for (const ref of scope) {
        if (!ANNOTATED_OBJECT_TYPES[ref.type]) { plan.refused.push({ ref, reason: `A ${ref.type} has no tags.` }); continue; }
        const record = annotatedRecord(model, ref);
        if (!record) { plan.refused.push(missing(ref)); continue; }
        const tags = Array.isArray(record.tags) ? (record.tags as string[]) : [];
        if (tags.includes(request.tag)) { plan.unchanged.push({ ref, reason: `Already tagged ${request.tag}.` }); continue; }
        plan.affected.push({ ref, before: tags.length ? tags.join(", ") : "no tags", after: [...tags, request.tag].join(", ") });
      }
      return plan;
    }
  }
}

/** Why the plan cannot be queued, or null when it can. */
export function planBlockedReason(plan: BulkChangePlan): string | null {
  if (plan.requestError) return plan.requestError;
  if (plan.refused.length) return `${plan.refused.length} ${plan.refused.length === 1 ? "entity is" : "entities are"} refused; narrow the selection or change the request. Nothing is queued while any entity is refused.`;
  if (!plan.affected.length) return "No entity in the selection would change.";
  return null;
}

function withBulkSource(intent: EditorOperationIntent): EditorOperationIntent {
  return { ...intent, source: { ...BULK_SOURCE } };
}

function safeToken(value: string): string {
  return value.replace(/[^A-Za-z0-9_.:-]+/g, "-");
}

function setFieldIntent(objectType: "Element" | "Component", ref: string, fieldPath: string, label: string, before: string, after: string, unit: string, dimension: string, sourceNote: string, requestText: string): EditorOperationIntent {
  const token = `${safeToken(ref)}-${safeToken(fieldPath)}`;
  return {
    operation_id: `op:bulk-${token}`,
    operation_kind: "modify",
    operation_status: "proposed",
    author_type: "user",
    source: { ...BULK_SOURCE },
    target: { object_type: objectType, ref },
    change: { change_id: `change:bulk-${token}`, change_kind: "set_field", field_label: label, field_path: fieldPath, before, after, unit, dimension, source_note: sourceNote },
    validation: { schema_validation: "not_run", constraint_validation: "not_run", unit_validation: dimension === "dimensionless" ? "not_required_dimensionless" : "not_run", diff_preview_status: "not_generated", application_status: "not_applied" },
    audit_boundary: { mutation_route: "structured_operations_only", direct_model_mutation_allowed: false, requires_user_acceptance: true, mutates_accepted_model_state: false },
    professional_boundary: {
      human_review_required: true,
      software_makes_compliance_claim: false,
      software_makes_certification_claim: false,
      software_makes_sealing_claim: false,
      software_makes_approval_claim: false,
      software_makes_authentication_claim: false
    },
    rationale: `Bulk change: ${requestText}; requires validation and review before application.`
  };
}

async function operationFor(model: PreviewModel, entry: PlanEntry, request: BulkChangeRequest, canonicalize: Canonicalize): Promise<EditorOperationIntent> {
  const ref = entry.ref;
  switch (request.kind) {
    case "section": {
      const pipe = pipeOf(model, ref.id)!;
      return withBulkSource(makeRichIntent({ object_type: "Element", ref: pipe.id }, "assign_section", "section_ref", await canonicalize(pipe), { section_ref: request.sectionId }, "Assign shared section"));
    }
    case "material": {
      const pipe = pipeOf(model, ref.id)!;
      return setFieldIntent("Element", pipe.id, "material", "Material", pipe.material, request.materialId, "none", "dimensionless", "material reference", `material ${request.materialId}`);
    }
    case "support-kind": {
      const support = model.supports.find((item) => item.id === ref.id)!;
      const before = supportConfiguration(support);
      const after = { ...before, family: request.toFamily, restraints: request.restraints === "keep" ? [...support.restraints] : [...request.restraints] };
      return withBulkSource(makeRichIntent({ object_type: "Support", ref: support.id }, "update_support", "configuration", await canonicalize(before), after, "Support configuration"));
    }
    case "support-remove": {
      const support = model.supports.find((item) => item.id === ref.id)!;
      return withBulkSource(buildDeleteSupportIntent(support, model));
    }
    case "bend-radius": {
      const component = model.components.find((item) => item.id === ref.id)!;
      const current = component.geometry?.bend_radius;
      const unit = request.unit.trim();
      return setFieldIntent("Component", component.id, "geometry.bend_radius.value", "Bend radius", current ? String(current.value) : "TBD",
        JSON.stringify({ value: Number(request.value.trim()), unit }), unit, "length", "unit metadata required; entered unit captured explicitly", `bend radius ${request.value.trim()} ${unit}`);
    }
    case "tag": {
      const objectType = ANNOTATED_OBJECT_TYPES[ref.type]!;
      const record = annotatedRecord(model, ref)!;
      const tags = Array.isArray(record.tags) ? (record.tags as string[]) : [];
      return withBulkSource(await annotationIntent(canonicalize, objectType, ref.id, record, "tags", [...tags, request.tag], "Tags"));
    }
  }
}

/** One batch with one operation per affected entity, in plan order. Throws, building nothing,
 * when the plan cannot be queued. */
export async function buildBulkChangeBatch(
  model: PreviewModel,
  scope: readonly EntityRef[],
  request: BulkChangeRequest,
  canonicalize: Canonicalize,
  batchId: string
): Promise<OperationBatch> {
  const plan = planBulkChange(model, scope, request);
  const blocked = planBlockedReason(plan);
  if (blocked) throw new Error(blocked);
  const operations: EditorOperationIntent[] = [];
  for (const entry of plan.affected) operations.push(await operationFor(model, entry, request, canonicalize));
  return { batch_id: batchId, operations };
}
