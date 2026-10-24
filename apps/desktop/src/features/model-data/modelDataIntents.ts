// W-13 S2a (DEL-02-01 Design model-document-0.7.0.md §6): structured
// operations for the model-data slots. Every edit is an EditorOperationIntent
// that the applier validates, diffs and applies (and the session can undo);
// nothing here writes the model directly.
//
// Wire (core/model_operations/operation_applier/src/model_data_authoring.rs):
// - annotations ride each family's existing change kind (set_field;
//   update_load for loads, update_support for supports); `before` is the
//   canonical JSON of the current member or `not_present`, `after` is JSON
//   text and `null` clears (removes) the member;
// - record operations target the record's id with the collection as
//   field_path; create has before `not_present`, update and delete carry the
//   current record's canonical JSON, delete has after `not_present`.
import { useEffect } from "react";
import type {
  EditorOperationIntent,
  EditorOperationObjectType,
  EntityRef,
  EquipmentInterfaceRecord,
  ModelAssumption,
  ModelConstraint,
  ModelLine,
  PreviewModel,
  RulePackReference
} from "../../types";

export type AnnotationField = "tags" | "notes" | "provenance_record";
type ChangeKind = EditorOperationIntent["change"]["change_kind"];

/** A model-data control's disabled state with its stated reason (the
 * dead-control audit, TP-APP-R2-UXSHELL-001): disabled while an edit is being
 * queued or while a condition holds, titled with the first reason that applies. */
export function disabledBy(busy: boolean, ...conditions: Array<[boolean, string]>): { disabled: boolean; title?: string } {
  const reason = busy ? "An edit is being queued." : conditions.find(([holds]) => holds)?.[1];
  return reason === undefined ? { disabled: false } : { disabled: true, title: reason };
}

/** Key-order-independent JSON text, for comparing a draft with a stored record. */
export function stableJson(value: unknown): string {
  if (Array.isArray(value)) return `[${value.map(stableJson).join(",")}]`;
  if (value && typeof value === "object") {
    return `{${Object.keys(value).sort().map((key) => `${JSON.stringify(key)}:${stableJson((value as Record<string, unknown>)[key])}`).join(",")}}`;
  }
  return JSON.stringify(value) ?? "null";
}

/** Re-seed a row's draft whenever its stored record changes (an apply, an undo
 * or redo, or another session's document), so the draft never outlives the
 * record it was taken from. */
export function useReseed(record: unknown, reseed: () => void): void {
  const key = stableJson(record);
  useEffect(reseed, [key]);
}

/** The selection types that take tags and notes, with their operation object type. */
export const ANNOTATED_OBJECT_TYPES: Partial<Record<EntityRef["type"], EditorOperationObjectType>> = {
  project: "Project",
  node: "Node",
  pipe: "Element",
  section: "Section",
  material: "Material",
  support: "Support",
  component: "Component",
  load: "Load",
  combination: "Combination"
};

/** The selection types that take a structured provenance record (§3.5). */
export const PROVENANCE_RECORD_TYPES: ReadonlyArray<EntityRef["type"]> = ["material", "section", "component", "support"];

/** The annotation's change kind is its family's existing one (§6). */
export function annotationChangeKind(objectType: EditorOperationObjectType): ChangeKind {
  if (objectType === "Load") return "update_load";
  if (objectType === "Support") return "update_support";
  return "set_field";
}

export type RecordFamily = {
  objectType: EditorOperationObjectType;
  collection: "lines" | "unresolved_assumptions" | "constraints" | "equipment_interfaces" | "rule_pack_refs";
  idKey: string;
  create: ChangeKind;
  update: ChangeKind | null;
  remove: ChangeKind;
  label: string;
};

export const RECORD_FAMILIES = {
  line: { objectType: "Line", collection: "lines", idKey: "id", create: "create_line", update: "update_line", remove: "delete_line", label: "line" },
  assumption: { objectType: "Assumption", collection: "unresolved_assumptions", idKey: "assumption_id", create: "create_assumption", update: "update_assumption", remove: "delete_assumption", label: "assumption or open issue" },
  constraint: { objectType: "Constraint", collection: "constraints", idKey: "constraint_id", create: "create_constraint", update: "update_constraint", remove: "delete_constraint", label: "constraint" },
  equipmentInterface: { objectType: "EquipmentInterface", collection: "equipment_interfaces", idKey: "id", create: "create_equipment_interface", update: "update_equipment_interface", remove: "delete_equipment_interface", label: "equipment interface" },
  rulePackRef: { objectType: "RulePackRef", collection: "rule_pack_refs", idKey: "rule_pack_id", create: "add_rule_pack_ref", update: null, remove: "remove_rule_pack_ref", label: "rule-pack reference" }
} as const satisfies Record<string, RecordFamily>;

export type ModelDataRecord = ModelLine | ModelAssumption | ModelConstraint | EquipmentInterfaceRecord | RulePackReference;

/** `model_constraint` kinds the format admits (constraint.schema.json minus TBD,
 * equipment_interface and missing_required_data). */
export const CONSTRAINT_KINDS = ["connectivity", "clearance", "no_go_volume", "support_zone", "route_conflict", "slope", "drain", "vent", "access"] as const;
export const CONSTRAINT_STATES = ["proposed", "active", "inactive", "superseded", "blocked_by_missing_data"] as const;
export const REDISTRIBUTION_STATUSES = ["private_only", "unknown", "public_permissive", "protected_suspected"] as const;
/** `review_status` values: the review of the source data only (UI label "Source review"). */
export const SOURCE_REVIEW_STATUSES = ["pending", "accepted", "rejected", "quarantined"] as const;

function operationKind(changeKind: ChangeKind): EditorOperationIntent["operation_kind"] {
  if (/^(create_|add_)/.test(changeKind)) return "create";
  if (/^(delete_|remove_)/.test(changeKind)) return "delete";
  return "modify";
}

function safeToken(value: string): string {
  return value.replace(/[^A-Za-z0-9_.:-]+/g, "-");
}

/** One model-data operation intent. `before` and `after` are wire strings. */
export function modelDataIntent({
  objectType,
  ref,
  changeKind,
  fieldPath,
  before,
  after,
  label
}: {
  objectType: EditorOperationObjectType;
  ref: string;
  changeKind: ChangeKind;
  fieldPath: string;
  before: string;
  after: string;
  label: string;
}): EditorOperationIntent {
  const token = `${safeToken(changeKind)}:${safeToken(ref)}:${safeToken(fieldPath)}`;
  return {
    operation_id: `op:model-data:${token}`,
    operation_kind: operationKind(changeKind),
    operation_status: "proposed",
    author_type: "user",
    source: {
      source_ref: "apps/desktop/src/features/model-data",
      source_channel: "local_desktop_preview",
      source_role: "gui_editor"
    },
    target: { object_type: objectType, ref },
    change: {
      change_id: `change:model-data:${token}`,
      change_kind: changeKind,
      field_label: label,
      field_path: fieldPath,
      before,
      after,
      unit: "none",
      dimension: "dimensionless",
      source_note: "User-entered model data; the record is stored with the model and is not evaluated."
    },
    validation: {
      schema_validation: "not_run",
      constraint_validation: "not_run",
      unit_validation: "not_required_dimensionless",
      diff_preview_status: "not_generated",
      application_status: "not_applied"
    },
    audit_boundary: {
      mutation_route: "structured_operations_only",
      direct_model_mutation_allowed: false,
      requires_user_acceptance: true,
      mutates_accepted_model_state: false
    },
    professional_boundary: {
      human_review_required: true,
      software_makes_compliance_claim: false,
      software_makes_certification_claim: false,
      software_makes_sealing_claim: false,
      software_makes_approval_claim: false,
      software_makes_authentication_claim: false
    },
    rationale: `User-authored ${label.toLowerCase()}; requires validation and review before application.`
  };
}

/** The record an annotation edits for this selection, or null. */
export function annotatedRecord(model: PreviewModel, selection: EntityRef): Record<string, unknown> | null {
  const find = (items: ReadonlyArray<{ id: string }> | undefined) =>
    (items?.find((item) => item.id === selection.id) as Record<string, unknown> | undefined) ?? null;
  switch (selection.type) {
    case "project":
      return model.project.id === selection.id ? (model.project as Record<string, unknown>) : null;
    case "node": return find(model.nodes);
    case "pipe": return find(model.pipe_segments);
    case "section": return find(model.sections);
    case "material": return find(model.materials);
    case "support": return find(model.supports);
    case "component": return find(model.components);
    case "load": return find(model.load_cases);
    case "combination": return find(model.combinations);
    default: return null;
  }
}

/** Tags as the user typed them (comma or newline separated): trimmed, blank
 * entries dropped, repeats dropped, order kept. Length is the validator's. */
export function parseTags(text: string): string[] {
  const seen = new Set<string>();
  const tags: string[] = [];
  for (const raw of text.split(/[,\n]/)) {
    const tag = raw.trim();
    if (tag && !seen.has(tag)) {
      seen.add(tag);
      tags.push(tag);
    }
  }
  return tags;
}

export type Canonicalize = (value: unknown) => Promise<string>;

/** An annotation intent: `next` null clears the member; an authored empty tag list is kept as `[]`. */
export async function annotationIntent(
  canonicalize: Canonicalize,
  objectType: EditorOperationObjectType,
  ref: string,
  current: Record<string, unknown>,
  field: AnnotationField,
  next: unknown,
  label: string
): Promise<EditorOperationIntent> {
  const existing = current[field];
  const before = existing === undefined ? "not_present" : await canonicalize(existing);
  return modelDataIntent({
    objectType,
    ref,
    changeKind: annotationChangeKind(objectType),
    fieldPath: field,
    before,
    after: JSON.stringify(next ?? null),
    label
  });
}

/** A record operation intent. `current` is the stored record (update, delete). */
export async function recordIntent(
  canonicalize: Canonicalize,
  family: RecordFamily,
  action: "create" | "update" | "delete",
  record: ModelDataRecord,
  current?: ModelDataRecord
): Promise<EditorOperationIntent> {
  const id = String((record as Record<string, unknown>)[family.idKey] ?? "");
  const changeKind = action === "create" ? family.create : action === "delete" ? family.remove : family.update;
  if (!changeKind) throw new Error(`A ${family.label} is not updated in place; remove and add it again.`);
  const before = action === "create" ? "not_present" : await canonicalize(current ?? record);
  const after = action === "delete" ? "not_present" : JSON.stringify(record);
  const verb = action === "create" ? (family.objectType === "RulePackRef" ? "Use" : "Add") : action === "delete" ? "Remove" : "Edit";
  return modelDataIntent({ objectType: family.objectType, ref: id, changeKind, fieldPath: family.collection, before, after, label: `${verb} ${family.label}` });
}

/** The next free id `prefix-NNN` that no record, entity or queued intent uses. */
export function nextModelDataId(model: PreviewModel, prefix: string, queued: EditorOperationIntent[] = []): string {
  const used = new Set<string>();
  const collect = (value: unknown) => {
    if (Array.isArray(value)) value.forEach(collect);
    else if (value && typeof value === "object") {
      for (const [key, item] of Object.entries(value)) {
        if ((key === "id" || key === "assumption_id" || key === "constraint_id") && typeof item === "string") used.add(item);
        collect(item);
      }
    }
  };
  collect(model);
  for (const intent of queued) used.add(intent.target.ref);
  for (let index = 1; ; index += 1) {
    const candidate = `${prefix}-${String(index).padStart(3, "0")}`;
    if (!used.has(candidate)) return candidate;
  }
}

/** The pipe ids of an ordered selection. */
export function selectedPipeIds(model: PreviewModel, refs: ReadonlyArray<EntityRef>): string[] {
  const pipes = new Set(model.pipe_segments.map((pipe) => pipe.id));
  return refs.filter((ref) => ref.type === "pipe" && pipes.has(ref.id)).map((ref) => ref.id);
}

/** The line a pipe belongs to, if any. */
export function lineOfPipe(model: PreviewModel, pipeId: string): ModelLine | undefined {
  return model.lines?.find((line) => line.pipe_refs.includes(pipeId));
}

/** A rule-pack reference from a stored pack document: identity and the pack's
 * stamped checksum projected to the validator's closed subset. Rule content
 * never enters the model (PRD §20.3). */
export function rulePackReferenceFromDocument(document: Record<string, unknown>): RulePackReference {
  const record = (value: unknown): Record<string, unknown> =>
    value && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : {};
  const metadata = record(document.metadata);
  const classification = record(document.classification);
  const stamped = record(record(document.checksums).rule_pack_checksum);
  const text = (value: unknown, label: string) => {
    if (typeof value !== "string" || !value.trim()) throw new Error(`The stored pack has no ${label}.`);
    return value;
  };
  if (typeof stamped.value !== "string" || !/^[0-9a-f]{64}$/.test(stamped.value)) {
    throw new Error("The stored pack carries no stamped checksum; compute and stamp it, then save the pack first.");
  }
  const reference: RulePackReference = {
    rule_pack_id: text(metadata.rule_pack_id, "rule_pack_id"),
    name: text(metadata.name, "name"),
    rule_pack_version: text(metadata.rule_pack_version, "rule_pack_version"),
    checksum: {
      algorithm: stamped.algorithm as "sha256",
      canonicalization: stamped.canonicalization as "JCS",
      payload_scope: stamped.payload_scope as "rule_pack_payload",
      payload_excludes: stamped.payload_excludes as ["checksums"],
      value: stamped.value
    },
    redistribution_status: text(classification.redistribution_status, "redistribution status") as RulePackReference["redistribution_status"]
  };
  if (typeof metadata.source_notice === "string" && metadata.source_notice.trim()) reference.source_notice = metadata.source_notice;
  return reference;
}
