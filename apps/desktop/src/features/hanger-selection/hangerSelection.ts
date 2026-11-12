import { canonicalJsonString } from "../../services/hashService";
import type { PreviewModel } from "../../types";
import type { OperationBatch } from "../../services/operationBatchService";
import { draftIntent } from "../offline-proposal-intake/workflowSupport";
import { hangerLibraryProvenance } from "../library/libraryApply";

type Provenance = Record<string, unknown>;
type ImportedQuantity = { magnitude: number; unit: string; dimension: string; provenance: Provenance };
export type HangerRecord = {
  hanger_id: string; name: string; provenance: Provenance;
  hanger: {
    hanger_type: "variable_spring_hanger" | "constant_effort_support";
    source_reference: string; manufacturer_reference?: string; load_side_review_reference?: string; mechanics_consumption?: string;
    stiffness?: { dof: string; value: ImportedQuantity };
    installed_load?: ImportedQuantity; cold_load?: ImportedQuantity; hot_load?: ImportedQuantity;
    constant_load?: ImportedQuantity; travel_range?: ImportedQuantity; movement_limit?: ImportedQuantity;
  };
};
export type HangerDocument = {
  schema_version: string;
  hanger_library: { library_id: string; name: string; provenance: Provenance; [key: string]: unknown };
  hanger_records: HangerRecord[];
};
export type HangerTarget = { supportId: string; restraints: string[]; newSupport?: { label: string; node: string } };

/** The sizing run a sized value comes from, named only by what the hanger-sizing result document binds
 * (DEL-04-11 REQ-007, REQ-008): the request digest, the bound solve request's hash, the library and the record. */
export type HangerSizingRunIdentity = Readonly<{
  sizingRequestSha256: string;
  solveRequestSha256: string;
  libraryId: string;
  librarySha256: string;
  hangerId: string;
  recordSha256: string;
}>;
/** The sized loads, SI newtons, as the result document publishes them. A variable record's cold and installed loads are
 * both the sized cold load; its hot load is the selection's operating load in the design operating case. A constant
 * record's constant load is the hot-load basis. */
export type SizedHangerLoads =
  | Readonly<{ family: "variable_spring"; coldLoadN: number; hotLoadN: number; designOperatingCase: string }>
  | Readonly<{ family: "constant_effort"; constantLoadN: number }>;
/** A sized selection to apply (DEL-04-11 REQ-008): the loads replace the record's own loads of the same members, and no
 * other record load enters the support. */
export type HangerSizingApplication = Readonly<{ run: HangerSizingRunIdentity; loads: SizedHangerLoads }>;

/** The presentation label of a sized selection's configuration change. Nothing decides on it: a reviewer may alter or
 * remove a label, so the routes that must refuse a hanger write read the batch's structure
 * (`writesSupportHangerSelection`). */
export const SIZED_HANGER_SELECTION_FIELD_LABEL = "Sized hanger selection";
/** The refusal of a requeued or imported batch that writes a support's hanger selection. */
export const SUPPORT_HANGER_WRITE_REFUSED_TEXT =
  "This batch writes a support's hanger selection and can only be applied from the hanger form or from its sizing result; make the selection again.";

/** The engine's names for a variable spring hanger and a constant-effort support (core/product_physics/src/lib.rs,
 * `is_variable_spring_hanger` and `is_constant_effort_support`); a test pins them to the engine's source. */
export const ENGINE_VARIABLE_SPRING_HANGER_TYPES: readonly string[] = ["variable_spring_hanger", "spring_hanger"];
export const ENGINE_CONSTANT_EFFORT_SUPPORT_TYPES: readonly string[] = ["constant_effort_support"];

const isRecord = (value: unknown): value is Record<string, unknown> => typeof value === "object" && value !== null && !Array.isArray(value);

/** The engine's classification of a support value, mirroring `support_hanger_type`: the trimmed `hanger.hanger_type`
 * when it is a text that is not blank, else the trimmed `family`; then the names above. */
export function engineHangerKind(value: unknown): "variable_spring" | "constant_effort" | null {
  if (!isRecord(value)) return null;
  const hangerType = isRecord(value.hanger) && typeof value.hanger.hanger_type === "string" ? value.hanger.hanger_type.trim() : "";
  const effective = hangerType !== "" ? hangerType : typeof value.family === "string" ? value.family.trim() : null;
  if (effective === null) return null;
  if (ENGINE_VARIABLE_SPRING_HANGER_TYPES.includes(effective)) return "variable_spring";
  if (ENGINE_CONSTANT_EFFORT_SUPPORT_TYPES.includes(effective)) return "constant_effort";
  return null;
}

/** The field paths through which an operation on a Support can write its hanger member or family, checked against the
 * operation route (core/model_operations/operation_applier): `configuration` (update_support, which replaces the
 * support's configuration) and `supports` (create_support). A support deletion also travels on `supports`, but writes no
 * hanger and is recognised by its kind (`isSupportDeletion`). The route's other Support paths (label, provenance, node,
 * restraints, tags, notes, provenance_record) cannot write a hanger or family. */
export const SUPPORT_HANGER_FIELD_PATHS: readonly string[] = ["configuration", "supports"];

/** A change's before or after as the predicate reads it: absent (`not_present`), a value, or unreadable. */
function changeValue(text: unknown): { kind: "absent" } | { kind: "value"; value: unknown } | { kind: "unreadable" } {
  if (text === "not_present") return { kind: "absent" };
  if (typeof text !== "string") return { kind: "unreadable" };
  try {
    return { kind: "value", value: JSON.parse(text) };
  } catch {
    return { kind: "unreadable" };
  }
}

/** Whether a value written or replaced on a support is a hanger: it has a `hanger` member at all (any value), or the
 * engine classifies it as one. A value that is unreadable, or not a support object (such as a label text where a
 * support was expected), counts as a hanger: it fails closed. */
function isHangerValue(read: ReturnType<typeof changeValue>): boolean {
  if (read.kind === "absent") return false;
  if (read.kind === "unreadable" || !isRecord(read.value)) return true;
  return "hanger" in read.value || engineHangerKind(read.value) !== null;
}

/** Whether an operation is one the operation route carries out only as a support deletion. The applier
 * (core/model_operations/operation_applier/src/lib.rs) dispatches on `change.change_kind`: in `run_with_context`, only
 * change_kind `delete_support` reaches `resolve_delete_support`, whose result is applied by `apply_deleted_support`, which
 * removes the support and writes nothing; `resolve_delete_support` blocks unless the target is a Support, the field path
 * `supports` and the after `not_present`; and `check_kinds` blocks a `delete_support` whose `operation_kind` is not
 * `delete`. The other support writers are keyed on other change kinds (`create_support` for `resolve_create_support`,
 * `update_support` for the `configuration` path, which blocks any other change_kind). So an operation with all of these
 * members is either a deletion or blocked with its batch, and cannot write a hanger; one that lacks any of them is read
 * as a possible write. */
function isSupportDeletion(operation: Record<string, unknown> & { target: Record<string, unknown>; change: Record<string, unknown> }): boolean {
  return operation.change.change_kind === "delete_support"
    && operation.operation_kind === "delete"
    && operation.target.object_type === "Support"
    && operation.change.field_path === "supports"
    && operation.change.after === "not_present";
}

/** Whether `batch` writes a support's hanger selection, by structure only: any operation on a Support, on one of
 * `SUPPORT_HANGER_FIELD_PATHS`, other than a support deletion, whose after or before value is a hanger. A value that
 * cannot be read as a support object counts as a hanger. No label, note or rationale is read. */
export function writesSupportHangerSelection(batch: OperationBatch): boolean {
  const operations: unknown = (batch as { operations?: unknown } | null | undefined)?.operations;
  if (!Array.isArray(operations)) return false;
  return operations.some((operation: unknown) => {
    if (!isRecord(operation) || !isRecord(operation.target) || !isRecord(operation.change)) return false;
    if (operation.target.object_type !== "Support" || !SUPPORT_HANGER_FIELD_PATHS.includes(String(operation.change.field_path))) return false;
    if (isSupportDeletion(operation as Record<string, unknown> & { target: Record<string, unknown>; change: Record<string, unknown> })) return false;
    return isHangerValue(changeValue(operation.change.after)) || isHangerValue(changeValue(operation.change.before));
  });
}

const FAMILY_OF = { variable_spring: "variable_spring_hanger", constant_effort: "constant_effort_support" } as const;
const SIZED_LOAD_UNIT = "N";
const RECORD_LOAD_MEMBERS = ["installed_load", "cold_load", "hot_load", "constant_load"] as const;

const positiveFinite = (value: number) => typeof value === "number" && Number.isFinite(value) && value > 0;

/** The source note of a sized selection's configuration change: the sizing run (its request digest and the bound solve
 * request's hash), the library and the record, and every sized member with its value and origin. Values are written
 * as JavaScript's shortest round-trip text of the published binary64, so the note states them exactly. */
export function hangerSizingSourceNote(application: HangerSizingApplication, libraryProvenance: string): string {
  const { run, loads } = application;
  const runText = `Hanger sizing run with sizing request sha256 ${run.sizingRequestSha256} and solve request sha256 ${run.solveRequestSha256}; ` +
    `library ${run.libraryId} sha256 ${run.librarySha256}; record ${run.hangerId} sha256 ${run.recordSha256}.`;
  const sized = loads.family === "variable_spring"
    ? `Sized values: hanger.cold_load ${String(loads.coldLoadN)} ${SIZED_LOAD_UNIT} and hanger.installed_load ${String(loads.coldLoadN)} ${SIZED_LOAD_UNIT}, ` +
      `both the sized cold load (the setting at the drawn position); hanger.hot_load ${String(loads.hotLoadN)} ${SIZED_LOAD_UNIT}, ` +
      `the sized operating load in the design operating case ${loads.designOperatingCase}.`
    : `Sized value: hanger.constant_load ${String(loads.constantLoadN)} ${SIZED_LOAD_UNIT}, the hot-load basis.`;
  return `${runText} ${sized} Family, stiffness, travel range and references from the library record ${libraryProvenance}.`;
}

function checkSizing(imported: HangerRecord["hanger"], recordId: string, libraryId: string, target: HangerTarget, sizing: HangerSizingApplication) {
  const { run, loads } = sizing;
  if (run.hangerId !== recordId || run.libraryId !== libraryId)
    throw new Error("The sizing result names another library or record than the one being applied. Size again.");
  if (imported.hanger_type !== FAMILY_OF[loads.family])
    throw new Error(`The sized selection is ${loads.family.replaceAll("_", " ")}, but record ${recordId} is ${imported.hanger_type.replaceAll("_", " ")}.`);
  if (target.restraints.length !== 1)
    throw new Error("A sized hanger acts on exactly one translational direction; give its one restraint direction.");
  const dof = target.restraints[0];
  if (loads.family === "variable_spring" && imported.stiffness === undefined)
    throw new Error(`Record ${recordId} declares no stiffness, so a sized variable spring cannot be applied from it.`);
  if (imported.stiffness !== undefined && imported.stiffness.dof.toUpperCase() !== dof)
    throw new Error(`Record ${recordId} declares its stiffness on ${imported.stiffness.dof}, but the sized support acts on ${dof}; nothing is applied.`);
  const values = loads.family === "variable_spring" ? [loads.coldLoadN, loads.hotLoadN] : [loads.constantLoadN];
  if (!values.every(positiveFinite)) throw new Error("A sized load is not a positive finite force; nothing is applied.");
}

// This maps a record AFTER native import validation. It is not a parallel library validator.
// With `sizing`, the sized loads replace the record's own loads (DEL-04-11 REQ-008); without it, the record's own values
// are copied as they are.
export async function buildHangerSelectionBatch(model: PreviewModel, document: HangerDocument, recordId: string, target: HangerTarget,
  sizing?: HangerSizingApplication): Promise<OperationBatch> {
  const selected = document.hanger_records.find(record => record.hanger_id === recordId);
  if (!selected) throw new Error("The selected hanger record is missing. Refresh the library.");
  if (!target.supportId.trim()) throw new Error("Select or enter a support ID.");
  if (!target.restraints.length || target.restraints.some(dof => !["UX", "UY", "UZ"].includes(dof)))
    throw new Error("Explicitly select the translational restraint directions for this hanger.");
  const old = model.supports.find(support => support.id === target.supportId);
  if (!target.newSupport && !old) throw new Error("The target support no longer exists.");
  if (target.newSupport && (!target.newSupport.label.trim() || !model.nodes.some(node => node.id === target.newSupport?.node)))
    throw new Error("A new support needs a label and an existing node.");
  // only the short library reference and the combined provenance
  // record enter the model; library metadata and the record snapshot never do.
  const imported = selected.hanger;
  if (sizing) checkSizing(imported, recordId, document.hanger_library.library_id, target, sizing);
  // A sized selection copies no record load and no movement limit (sizing reads neither, REQ-006); the provenance
  // record lists only what is copied from the record.
  const copiedMembers = sizing ? (["travel_range"] as const) : ([...RECORD_LOAD_MEMBERS, "travel_range", "movement_limit"] as const);
  const quantities = [...copiedMembers.map(key => imported[key]), imported.stiffness?.value].filter((q): q is ImportedQuantity => q !== undefined);
  const { provenance: source, provenanceRecord } = await hangerLibraryProvenance(
    document.hanger_library as Record<string, unknown>, selected as unknown as Record<string, unknown>, quantities as unknown as Record<string, unknown>[]);
  const hanger: Record<string, unknown> = { hanger_type: imported.hanger_type, source_reference: imported.source_reference };
  for (const key of ["manufacturer_reference", "load_side_review_reference", "mechanics_consumption"] as const)
    if (imported[key] !== undefined) hanger[key] = imported[key];
  for (const key of copiedMembers) {
    const q = imported[key];
    if (q !== undefined) hanger[key] = { value: q.magnitude, unit: q.unit };
  }
  if (sizing?.loads.family === "variable_spring") {
    hanger.installed_load = { value: sizing.loads.coldLoadN, unit: SIZED_LOAD_UNIT };
    hanger.cold_load = { value: sizing.loads.coldLoadN, unit: SIZED_LOAD_UNIT };
    hanger.hot_load = { value: sizing.loads.hotLoadN, unit: SIZED_LOAD_UNIT };
  } else if (sizing?.loads.family === "constant_effort") {
    hanger.constant_load = { value: sizing.loads.constantLoadN, unit: SIZED_LOAD_UNIT };
  }
  if (imported.stiffness !== undefined) hanger.stiffness = {
    dof: imported.stiffness.dof,
    value: { value: imported.stiffness.value.magnitude, unit: imported.stiffness.value.unit }
  };
  const configuration = { family: imported.hanger_type, restraints: [...target.restraints], hanger, provenance: source };
  const after = target.newSupport ? { id: target.supportId, ...target.newSupport, ...configuration, provenance_record: provenanceRecord } : configuration;
  const before = old ? Object.fromEntries(["family", "restraints", "stiffness", "hanger", "nonlinear", "provenance"]
    .filter(key => (old as unknown as Record<string, unknown>)[key] !== undefined)
    .map(key => [key, (old as unknown as Record<string, unknown>)[key]])) : undefined;
  const operations = [draftIntent({
    operation_kind: target.newSupport ? "create" : "modify",
    target: { object_type: "Support", ref: target.supportId },
    change: {
      change_id: `change:${crypto.randomUUID()}`, change_kind: target.newSupport ? "create_support" : "update_support",
      field_label: sizing ? SIZED_HANGER_SELECTION_FIELD_LABEL : "Imported hanger selection", field_path: target.newSupport ? "supports" : "configuration",
      before: target.newSupport ? "not_present" : await canonicalJsonString(before),
      after: await canonicalJsonString(after), unit: "none", dimension: "dimensionless",
      source_note: sizing ? hangerSizingSourceNote(sizing, source) : source
    },
    rationale: sizing
      ? "The person's application of a record that hanger sizing selected from their own library by their own tests, with the sized loads; replaces the support configuration and clears previous top-level stiffness and nonlinear settings."
      : "Manual user selection of an imported hanger record; replaces the support configuration and clears previous top-level stiffness and nonlinear settings."
  })];
  const recordChanged = !old || old.provenance_record === undefined ||
    await canonicalJsonString(old.provenance_record) !== await canonicalJsonString(provenanceRecord);
  if (old && recordChanged) operations.push(draftIntent({
    operation_kind: "modify",
    target: { object_type: "Support", ref: target.supportId },
    change: {
      change_id: `change:${crypto.randomUUID()}`, change_kind: "update_support",
      field_label: "Imported hanger provenance record", field_path: "provenance_record",
      before: old.provenance_record === undefined ? "not_present" : await canonicalJsonString(old.provenance_record),
      after: await canonicalJsonString(provenanceRecord), unit: "none", dimension: "dimensionless", source_note: source
    },
    rationale: "The structured provenance of the selected hanger record (W-13 §3.5)."
  }));
  return { batch_id: `batch:hanger:${crypto.randomUUID()}`, operations };
}
