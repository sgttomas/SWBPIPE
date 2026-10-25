// Comparison of two saved model states (W-12 S3a; DEL-14-01 Design
// `desktop-states-runs-comparison.md` §4.1; FR-CMP-003, and FR-CMP-006 for
// states). Deterministic and automatic: entities are mapped by their stable id
// within their family, with no manual mapping (FR-CMP-005/007 stay v0.2) and no
// tolerance or unit conversion (FR-CMP-008 stays v0.2). The reference behaviour
// is `core/comparison/model_state/engine.py`; the parity golden
// (`fixtures/comparison/model_state_comparison_golden.json`, written by
// `stateComparisonParity.test.ts`) is checked against it by
// `tests/test_model_state_comparison_parity.py`.
//
// Flattening is complete and fails closed: every top-level member of the 0.7.0
// model document is assigned to an entity family or to the single `model`
// entity (`MEMBER_ASSIGNMENT`), and a document holding any other member is
// refused (`COMPARISON-MEMBER-UNASSIGNED`). The `model` entity also holds the
// ordered id list of every collection, so equal model hashes hold exactly when
// no row is other than `unchanged`.
//
// The result is a difference report. It establishes nothing about either
// state; human review is still required for any use of it.

/** The entity families, in display order. */
export const ENTITY_FAMILIES = [
  "model",
  "node",
  "pipe_segment",
  "section",
  "material",
  "support",
  "component",
  "load_case",
  "primitive_load",
  "combination",
  "reference_configuration",
  "line",
  "unresolved_assumption",
  "constraint",
  "equipment_interface",
  "rule_pack_ref"
] as const;
export type EntityFamily = (typeof ENTITY_FAMILIES)[number];

export const FAMILY_LABELS: Record<EntityFamily, string> = {
  model: "Model",
  node: "Node",
  pipe_segment: "Pipe segment",
  section: "Section",
  material: "Material",
  support: "Support",
  component: "Component",
  load_case: "Load case",
  primitive_load: "Primitive load",
  combination: "Combination",
  reference_configuration: "Reference configuration",
  line: "Line",
  unresolved_assumption: "Unresolved assumption",
  constraint: "Constraint",
  equipment_interface: "Equipment interface",
  rule_pack_ref: "Rule-pack reference"
};

/** A collection member of the document: its family and the field holding each entity's stable id. */
type CollectionFamily = { family: Exclude<EntityFamily, "model" | "primitive_load">; idKey: string };

/** The document's collections. Rule-pack references are keyed by `rule_pack_id`,
 * as the operation applier keys them (`modelDataIntents.ts` RECORD_FAMILIES). */
export const COLLECTION_MEMBERS: Readonly<Record<string, CollectionFamily>> = {
  nodes: { family: "node", idKey: "id" },
  pipe_segments: { family: "pipe_segment", idKey: "id" },
  sections: { family: "section", idKey: "id" },
  materials: { family: "material", idKey: "id" },
  supports: { family: "support", idKey: "id" },
  components: { family: "component", idKey: "id" },
  load_cases: { family: "load_case", idKey: "id" },
  combinations: { family: "combination", idKey: "id" },
  reference_configurations: { family: "reference_configuration", idKey: "id" },
  lines: { family: "line", idKey: "id" },
  unresolved_assumptions: { family: "unresolved_assumption", idKey: "assumption_id" },
  constraints: { family: "constraint", idKey: "constraint_id" },
  equipment_interfaces: { family: "equipment_interface", idKey: "id" },
  rule_pack_refs: { family: "rule_pack_ref", idKey: "rule_pack_id" }
};

/** The document-level members the single `model` entity holds as they are. */
export const MODEL_ENTITY_MEMBERS = [
  "schema_version",
  "document_kind",
  "project",
  "pressure_contract",
  "data_boundary",
  "analysis_status",
  "diagnostics",
  "model_settings"
] as const;

/** Every top-level member of the 0.7.0 model document (`schemas/model.schema.yaml`
 * `properties`) and what holds it. A test fails when the schema gains a member
 * that is not assigned here. */
export const MEMBER_ASSIGNMENT: Readonly<Record<string, EntityFamily>> = {
  ...Object.fromEntries(MODEL_ENTITY_MEMBERS.map((member) => [member, "model" as const])),
  ...Object.fromEntries(Object.entries(COLLECTION_MEMBERS).map(([member, { family }]) => [member, family]))
};

/** The primitive loads of a load case (`load_cases[].primitive_loads[]`): their own
 * family, keyed `<load case id>/<load id>`; the load case entity excludes them. */
const PRIMITIVE_LOADS = "primitive_loads";

export const COMPARISON_MEMBER_UNASSIGNED = "COMPARISON-MEMBER-UNASSIGNED";
export const COMPARISON_MEMBER_MALFORMED = "COMPARISON-MEMBER-MALFORMED";
export const COMPARISON_ENTITY_ID_MISSING = "COMPARISON-ENTITY-ID-MISSING";
export const COMPARISON_DUPLICATE_ID = "COMPARISON-DUPLICATE-ID";
/** A received snapshot whose model hash in the webview differs from the stored one:
 * it holds values (such as integers beyond 2^53) the desktop cannot represent
 * exactly, so comparing it would hide differences. */
export const COMPARISON_STATE_NOT_EXACT = "COMPARISON-STATE-NOT-EXACT";

/** A comparison the document's content does not admit, refused by name. */
export class StateComparisonRefusal extends Error {
  constructor(readonly code: string, detail: string) {
    super(`${code}: ${detail}`);
    this.name = "StateComparisonRefusal";
  }
}

/** One flattened entity: its family, its stable key (the id; a primitive load's
 * key is its load case's id and its own), the id as shown, and its fields (the
 * record without its own id field). */
export type FlatEntity = { family: EntityFamily; key: string[]; id: string; fields: Record<string, unknown> };

type PlainObject = Record<string, unknown>;

function isPlainObject(value: unknown): value is PlainObject {
  if (value === null || typeof value !== "object" || Array.isArray(value)) return false;
  const prototype = Object.getPrototypeOf(value);
  return prototype === Object.prototype || prototype === null;
}

/** The most characters of a document value a refusal quotes. */
const SHOWN_CHARS = 120;
function shown(text: string): string {
  const characters = [...text];
  return characters.length > SHOWN_CHARS ? `${characters.slice(0, SHOWN_CHARS).join("")}…` : text;
}

/** A null-prototype record, so any key (`__proto__` included) is an own member. */
function record(): PlainObject {
  return Object.create(null) as PlainObject;
}

function stableId(value: unknown): string | null {
  return typeof value === "string" && /\S/.test(value) ? value : null;
}

function withoutKeys(entity: PlainObject, keys: readonly string[]): PlainObject {
  const fields = record();
  for (const [key, value] of Object.entries(entity)) if (!keys.includes(key)) fields[key] = value;
  return fields;
}

/** The ids of a collection's entities in document order. */
function entitiesOf(member: string, value: unknown, idKey: string, side: string): Array<{ id: string; entity: PlainObject }> {
  if (!Array.isArray(value)) throw new StateComparisonRefusal(COMPARISON_MEMBER_MALFORMED, `the ${side} model's \`${member}\` is not an array, so it cannot be compared.`);
  return value.map((entity, position) => {
    if (!isPlainObject(entity)) throw new StateComparisonRefusal(COMPARISON_MEMBER_MALFORMED, `the ${side} model's \`${member}[${position}]\` is not an object, so it cannot be compared.`);
    const id = stableId(entity[idKey]);
    if (id === null) throw new StateComparisonRefusal(COMPARISON_ENTITY_ID_MISSING, `the ${side} model's \`${member}[${position}]\` has no stable id (\`${idKey}\`), so it cannot be mapped.`);
    return { id, entity };
  });
}

/** Flattens a 0.7.0 model document into its entities: the single `model` entity
 * first, then each collection's entities in document order. Fails closed: a
 * member no family holds, a collection that is not an array of objects or an
 * entity without a stable id refuses the comparison by name. */
export function flattenModelDocument(document: unknown, side = "compared"): FlatEntity[] {
  if (!isPlainObject(document)) throw new StateComparisonRefusal(COMPARISON_MEMBER_MALFORMED, `the ${side} model is not a JSON object.`);
  const unassigned = Object.keys(document).filter((member) => !Object.prototype.hasOwnProperty.call(MEMBER_ASSIGNMENT, member)).sort();
  if (unassigned.length) {
    const named = unassigned.slice(0, 3).map((member) => `\`${shown(member)}\``).join(", ");
    throw new StateComparisonRefusal(
      COMPARISON_MEMBER_UNASSIGNED,
      `the ${side} model holds ${unassigned.length === 1 ? "member" : "members"} ${named}${unassigned.length > 3 ? ` and ${unassigned.length - 3} more` : ""} that no comparison family holds, so the comparison would not be complete.`
    );
  }
  const modelFields = record();
  for (const member of MODEL_ENTITY_MEMBERS) if (Object.prototype.hasOwnProperty.call(document, member)) modelFields[member] = document[member];
  const order = record();
  const primitiveLoadOrder: Array<{ load_case: string; loads: string[] }> = [];
  const entities: FlatEntity[] = [];
  for (const [member, { family, idKey }] of Object.entries(COLLECTION_MEMBERS)) {
    if (!Object.prototype.hasOwnProperty.call(document, member)) continue;
    const listed = entitiesOf(member, document[member], idKey, side);
    order[member] = listed.map(({ id }) => id);
    for (const { id, entity } of listed) {
      if (family === "load_case") {
        entities.push({ family, key: [id], id, fields: withoutKeys(entity, [idKey, PRIMITIVE_LOADS]) });
        if (!Object.prototype.hasOwnProperty.call(entity, PRIMITIVE_LOADS)) continue;
        const loads = entitiesOf(`${member}[${shown(id)}].${PRIMITIVE_LOADS}`, entity[PRIMITIVE_LOADS], "id", side);
        primitiveLoadOrder.push({ load_case: id, loads: loads.map((load) => load.id) });
        for (const load of loads) {
          entities.push({ family: "primitive_load", key: [id, load.id], id: `${id}/${load.id}`, fields: withoutKeys(load.entity, ["id"]) });
        }
      } else {
        entities.push({ family, key: [id], id, fields: withoutKeys(entity, [idKey]) });
      }
    }
  }
  modelFields.collection_order = order;
  modelFields.primitive_load_order = primitiveLoadOrder;
  return [{ family: "model", key: ["model"], id: "model", fields: modelFields }, ...entities];
}

export type PathSegment = string | number;
type Leaf = { path: PathSegment[]; value: unknown };

/** A quantity as the document authors it: `{value, unit}` or `{x, y, z, unit}`:
 * a text unit and only numbers besides. */
export function isQuantity(value: unknown): value is PlainObject & { unit: string } {
  if (!isPlainObject(value) || typeof value.unit !== "string") return false;
  const others = Object.entries(value).filter(([key]) => key !== "unit");
  return others.length > 0 && others.every(([, item]) => typeof item === "number");
}

function isScalar(value: unknown): boolean {
  return value === null || typeof value !== "object";
}

/** A leaf of the field diff: a scalar, a quantity, an empty object or an array
 * of scalars (an id list, restraints, tags). Objects recurse by key and arrays
 * holding objects or arrays by index. */
function isLeaf(value: unknown): boolean {
  if (isScalar(value) || isQuantity(value)) return true;
  if (Array.isArray(value)) return value.every(isScalar);
  return isPlainObject(value) ? Object.keys(value).length === 0 : true;
}

function collectLeaves(value: unknown, path: PathSegment[], out: Map<string, Leaf>) {
  if (path.length > 0 && isLeaf(value)) {
    out.set(JSON.stringify(path), { path, value });
    return;
  }
  if (Array.isArray(value)) value.forEach((item, index) => collectLeaves(item, [...path, index], out));
  else for (const [key, item] of Object.entries(value as PlainObject)) collectLeaves(item, [...path, key], out);
}

/** The entity's fields as leaves, by path. */
export function fieldLeaves(fields: Record<string, unknown>): Map<string, Leaf> {
  const out = new Map<string, Leaf>();
  collectLeaves(fields, [], out);
  return out;
}

/** JSON with keys in code-point order: equal exactly when the JCS texts are. */
function stableText(value: unknown): string {
  if (isScalar(value)) return JSON.stringify(value);
  if (Array.isArray(value)) return `[${value.map(stableText).join(",")}]`;
  const keys = Object.keys(value as PlainObject).sort(codePointCompare);
  return `{${keys.map((key) => `${JSON.stringify(key)}:${stableText((value as PlainObject)[key])}`).join(",")}}`;
}

export function codePointCompare(left: string, right: string): number {
  const a = [...left];
  const b = [...right];
  for (let index = 0; index < Math.min(a.length, b.length); index += 1) {
    const difference = a[index].codePointAt(0)! - b[index].codePointAt(0)!;
    if (difference !== 0) return difference;
  }
  return a.length - b.length;
}

function pathCompare(left: PathSegment[], right: PathSegment[]): number {
  for (let index = 0; index < Math.min(left.length, right.length); index += 1) {
    const a = left[index];
    const b = right[index];
    if (a === b) continue;
    if (typeof a === "number" && typeof b === "number") return a - b;
    if (typeof a === "number") return -1;
    if (typeof b === "number") return 1;
    return codePointCompare(a, b);
  }
  return left.length - right.length;
}

/** A field path as shown: `a.b[0].c`, with a key that is not a plain name quoted. */
export function pathText(path: PathSegment[]): string {
  return path.map((segment, index) => {
    if (typeof segment === "number") return `[${segment}]`;
    if (/^[A-Za-z_][A-Za-z0-9_]*$/.test(segment)) return index === 0 ? segment : `.${segment}`;
    return `[${JSON.stringify(segment)}]`;
  }).join("");
}

export type FieldChangeKind = "added" | "removed" | "changed" | "unit_changed";

/** One field that differs: its path, how, the two values and, for a quantity,
 * its units. A pair of quantities with different units is `unit_changed`: it is
 * reported with both values and units and never converted or subtracted. */
export type FieldChange = {
  path: PathSegment[];
  kind: FieldChangeKind;
  left?: unknown;
  right?: unknown;
  left_unit?: string;
  right_unit?: string;
};

function fieldChanges(left: Record<string, unknown>, right: Record<string, unknown>): FieldChange[] {
  const leftLeaves = fieldLeaves(left);
  const rightLeaves = fieldLeaves(right);
  const changes: FieldChange[] = [];
  const keys = new Set([...leftLeaves.keys(), ...rightLeaves.keys()]);
  for (const key of keys) {
    const a = leftLeaves.get(key);
    const b = rightLeaves.get(key);
    const unitOf = (leaf: Leaf | undefined) => (leaf && isQuantity(leaf.value) ? { unit: leaf.value.unit } : null);
    const units = { ...(unitOf(a) ? { left_unit: unitOf(a)!.unit } : {}), ...(unitOf(b) ? { right_unit: unitOf(b)!.unit } : {}) };
    if (a && !b) changes.push({ path: a.path, kind: "removed", left: a.value, ...units });
    else if (b && !a) changes.push({ path: b.path, kind: "added", right: b.value, ...units });
    else if (a && b && stableText(a.value) !== stableText(b.value)) {
      const unitChanged = isQuantity(a.value) && isQuantity(b.value) && a.value.unit !== b.value.unit;
      changes.push({ path: a.path, kind: unitChanged ? "unit_changed" : "changed", left: a.value, right: b.value, ...units });
    }
  }
  return changes.sort((x, y) => pathCompare(x.path, y.path));
}

export type RowClassification = "added" | "removed" | "changed" | "unchanged";
export const ROW_CLASSIFICATIONS: readonly RowClassification[] = ["added", "removed", "changed", "unchanged"];

/** One mapped or unmatched entity: same stable id on both sides is `unchanged`
 * or `changed`; left only is `removed`; right only is `added`. */
export type StateComparisonRow = {
  family: EntityFamily;
  id: string;
  key: string[];
  classification: RowClassification;
  changes: FieldChange[];
};

export type StateComparisonSummary = Record<RowClassification, number> & {
  total: number;
  /** Field changes whose quantity units differ (reported, never converted). */
  unit_changed_fields: number;
};

export type StateComparison = {
  summary: StateComparisonSummary;
  rows: StateComparisonRow[];
  entity_counts: { left: number; right: number };
};

function indexKey(entity: Pick<FlatEntity, "family" | "key">): string {
  return JSON.stringify([entity.family, ...entity.key]);
}

function indexEntities(entities: FlatEntity[], side: string): Map<string, FlatEntity> {
  const index = new Map<string, FlatEntity>();
  for (const entity of entities) {
    const key = indexKey(entity);
    if (index.has(key)) {
      throw new StateComparisonRefusal(
        COMPARISON_DUPLICATE_ID,
        `the ${side} model holds more than one ${FAMILY_LABELS[entity.family].toLowerCase()} with stable id \`${shown(entity.id)}\`, so its entities cannot be mapped.`
      );
    }
    index.set(key, entity);
  }
  return index;
}

function rowCompare(left: StateComparisonRow, right: StateComparisonRow): number {
  const family = ENTITY_FAMILIES.indexOf(left.family) - ENTITY_FAMILIES.indexOf(right.family);
  if (family !== 0) return family;
  for (let index = 0; index < Math.min(left.key.length, right.key.length); index += 1) {
    const difference = codePointCompare(left.key[index], right.key[index]);
    if (difference !== 0) return difference;
  }
  return left.key.length - right.key.length;
}

/** Compares two flattened entity lists. A stable id repeated within one family on
 * one side refuses the comparison (as the reference engine does). */
export function compareFlattened(left: FlatEntity[], right: FlatEntity[]): StateComparison {
  const leftIndex = indexEntities(left, "left");
  const rightIndex = indexEntities(right, "right");
  const rows: StateComparisonRow[] = [];
  for (const [key, entity] of leftIndex) {
    const counterpart = rightIndex.get(key);
    if (!counterpart) {
      rows.push({ family: entity.family, id: entity.id, key: entity.key, classification: "removed", changes: [] });
      continue;
    }
    const changes = fieldChanges(entity.fields, counterpart.fields);
    rows.push({ family: entity.family, id: entity.id, key: entity.key, classification: changes.length ? "changed" : "unchanged", changes });
  }
  for (const [key, entity] of rightIndex) {
    if (!leftIndex.has(key)) rows.push({ family: entity.family, id: entity.id, key: entity.key, classification: "added", changes: [] });
  }
  rows.sort(rowCompare);
  const summary: StateComparisonSummary = { added: 0, removed: 0, changed: 0, unchanged: 0, total: rows.length, unit_changed_fields: 0 };
  for (const row of rows) {
    summary[row.classification] += 1;
    summary.unit_changed_fields += row.changes.filter((change) => change.kind === "unit_changed").length;
  }
  return { summary, rows, entity_counts: { left: left.length, right: right.length } };
}

/** Compares two model documents: each is flattened (fail-closed), then entities
 * are mapped by stable id within their family. */
export function compareModelDocuments(left: unknown, right: unknown): StateComparison {
  return compareFlattened(flattenModelDocument(left, "left"), flattenModelDocument(right, "right"));
}
