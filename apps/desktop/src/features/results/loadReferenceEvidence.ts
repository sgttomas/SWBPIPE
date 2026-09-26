/** Closed admission of the load-reference-1 raw evidence namespace.
 *
 * TypeScript peer of `core/analysis_runs/load_reference_evidence.py` and
 * `core/reporting/result_export/src/load_reference.rs`: the same checks run in
 * the same order and raise the same error strings. Arrays are walked in
 * document order so the first failure is deterministic. Change all three
 * together.
 *
 * After the load-reference-specific pre-pass, the unchanged desktop physics-1
 * validator checks rows, extrema, regions and the pressure RHS on a projected
 * copy. The projection only removes or neutralizes fields that the pre-pass
 * has already bound. This establishes internal source consistency, never
 * producer origin, solver accuracy or model freshness. A header never
 * authenticates its claimed producer.
 *
 * JavaScript holds every JSON number as a binary64 value, so the JSON text form
 * of a number (integer literal versus fraction) is not observable here; an
 * integral index is admitted when its binary64 value is an integer in the u64
 * range. Rust and Python additionally refuse a fractional literal such as
 * `1.0` there. The shared corpus never depends on that distinction.
 */
import transportSchemaJson from "../../../../../schemas/load_reference_state.schema.json";
import { validatePhysicsEvidence, validatePhysicsTransportMetadata } from "./physicsResultEvidence";
import type { MechanicsResult, PreviewModel } from "../../types";

export const LOAD_REFERENCE_CONTRACT_ID = "openpipestress.result_semantics/0.3.0/load-reference-1";
export const LOAD_REFERENCE_PROFILE = "resolved_straight_load_state_v1";
export const LOAD_REFERENCE_TABLE_SHA256 = "44bc41c06f589fab6ce931ac0eaa5344765ff64fd5f880cc2dd69ecb839c4f4d";
export const LOAD_REFERENCE_TRANSPORT_SCHEMA_SHA256 = "640fd4477ac2c84f3c02268cfccc3958f51ee6508b3a67e5538b52d84899af65";

export const RECORD_CONTRACT = "openpipestress.load_reference_state/1.0.0";
export const MATERIAL_BASIS = "resolved_per_member_load_reference_state_v1";
export const NOT_JOINED = "LOAD_STATE_SOURCE_RECOVERY_NOT_JOINED";
export const EXACT_METHOD = "retained_source_blocks_exact_v1";
export const SELECTED = "SOURCE_BLOCK_RECOVERY_SELECTED";
export const UNAVAILABLE = "SOURCE_BLOCK_RECOVERY_UNAVAILABLE";
export type LoadReferenceMethod = "load_reference" | "joined";
const REGION_TEMPERATURE_BASIS = "resolved_member_state";
const G_BASIS = "E/[2(1+nu)] from the selected pair";
const COMPOSITION = "lambda_fit*lambda_thermal-1";
const BOUNDARY = "every restrained DOF prescribed; reduced K_ff u_f = f_f - K_fc g_c; complete u includes g; reactions from unreduced K u - f";
const EIGENLOAD = "axial E_member*A_s*total_eigenstrain assembled once and removed once in recovery";

const EVIDENCE_KEYS = ["pressure", "connector", "exact_cases", "load_reference_states"];
const CASE_KEYS = ["load_case_id", "profile_mode", "material_basis", "pipe_materials", "pipe_sections", "pipe_stress_extrema", "stress_maximum_coverage", "pressure_rhs_assembly"];
const MATERIAL_KEYS = ["pipe_id", "material_id", "E_pa", "nu", "G_pa", "constitutive_basis", "material_selection_kind", "thermal_consumed", "alpha_per_kelvin", "resolved_eigenstrain", "provenance"];
const RECORD_KEYS = ["load_case_id", "contract", "profile", "reference_configuration_id", "provenance", "reference_geometry", "history", "solve", "source_recovery", "members", "support_components", "contributions", "excluded_sources"];
const MEMBER_KEYS = [
  "pipe_id", "material_id", "material_selection_kind", "consumed_material_points", "interpolation_fraction",
  "applicability_reference", "analysis_basis_override", "selected_E_pa", "selected_nu", "derived_G_pa", "G_basis",
  "retained_G_ignored", "operating_temperature_k", "material_selection_temperature_k", "reference_basis",
  "installation_temperature_k", "thermal_definition", "expansion_law_id", "coefficient_datum_k",
  "consumed_law_point_indices", "consumed_law_segments", "consulted_law_point_indices", "consulted_law_segments",
  "installation_datum_stretch", "operating_datum_stretch", "thermal_strain", "thermal_stretch", "fit_strain",
  "fit_stretch", "total_eigenstrain", "eigenstrain_composition", "fit_kind", "fit_input", "reference_length_m",
];
const POINT_KEYS = ["point_id", "temperature_k", "E_pa", "nu", "retained_G_ignored"];
const SEGMENT_KEYS = ["use", "lower_index", "upper_index", "start_k", "end_k"];
const SUPPORT_KEYS = ["support_id", "node_id", "dof", "global_dof", "law_kind", "prescribed_value", "unit", "meaning", "physical_state_source"];
const STORED_KEYS = ["source_id", "owner_kind", "classification", "factor", "category", "dimension", "authored_normalized_magnitude", "applied_magnitude"];
const MEMBER_STATE_KEYS = ["source_id", "owner_kind", "classification", "consumed_input_refs", "value"];
const SUPPORT_STATE_KEYS = ["source_id", "owner_kind", "classification", "value"];
const PRESSURE_REGION_KEYS = ["source_id", "owner_kind", "classification", "factor"];
const EXCLUDED_KEYS = ["source_id", "owner_kind", "classification", "category", "reason"];
const REGION_KEYS = ["profile_version", "profile_mode", "load_case_id", "region_id", "member_pipe_ids", "pressure_basis", "p_pa", "external_pressure_increment_pa", "approximation", "geometry_representation_guard", "geometry", "materials", "applied_loads", "terminals", "provenance", "result_ids"];
const REGION_MATERIAL_KEYS = ["pipe_id", "material_id", "E_pa", "nu", "G_pa", "constitutive_basis", "thermal_consumed", "alpha_per_kelvin", "provenance", "temperature_basis"];
const DOFS = ["UX", "UY", "UZ", "RX", "RY", "RZ"];
const LAW_DEFINITIONS = ["engineering_secant", "engineering_dilation", "differential_per_datum_length", "logarithmic_per_current_length"];
const DIRECT_DEFINITIONS = ["unchanged_reference", "explicit_interval_strain", "constant_alpha_interval"];
const U64_MAX = 18446744073709551615;

type Json = any; // eslint-disable-line @typescript-eslint/no-explicit-any

/** A shared-code rejection; `message` is the exact Rust/Python error string. */
export class LoadReferenceError extends Error {
  constructor(message: string) { super(message); this.name = "LoadReferenceError"; }
}
/** Python KeyError/TypeError/IndexError peer: never a reader verdict by itself. */
class MalformedAccess extends Error {}

export const code = (name: string) => `SOURCE_LOAD_REFERENCE_${name}`;
export const fail = (name: string) => new LoadReferenceError(code(name));
export function require_(ok: unknown, name: string): void {
  if (!ok) throw fail(name);
}
export const isObject = (value: unknown): value is Record<string, Json> =>
  value !== null && typeof value === "object" && !Array.isArray(value);
/** Python `mapping[key]`: a missing key or a non-mapping raises (malformed). */
export function at(value: unknown, key: string): Json {
  if (!isObject(value) || !Object.hasOwn(value, key)) throw new MalformedAccess(key);
  return value[key];
}
/** Rust `Value[key]` semantics: missing key or non-object reads as null. */
export function get(value: unknown, key: string): Json {
  return isObject(value) && Object.hasOwn(value, key) ? value[key] ?? null : null;
}
export function keys(value: unknown, required: string[]): boolean {
  return isObject(value) && Object.keys(value).length === required.length && required.every(key => Object.hasOwn(value, key));
}
const isNumber = (value: unknown): value is number => typeof value === "number";
/** Rust `Value::as_f64`: the binary64 value of a JSON number, else null. */
function asF64(value: unknown): number | null {
  return isNumber(value) ? value : null;
}
export function text(value: unknown): string {
  if (!(typeof value === "string" && value.length > 0)) throw fail("STRING_INVALID");
  return value;
}
function number(value: unknown): number {
  const n = asF64(value);
  if (n === null || !Number.isFinite(n)) throw fail("NUMBER_INVALID");
  return n;
}
const optNumber = (value: unknown): number | null => value === null || value === undefined ? null : number(value);
const optText = (value: unknown): string | null => value === null || value === undefined ? null : text(value);
function boolean(value: unknown): boolean {
  if (typeof value !== "boolean") throw fail("BOOLEAN_INVALID");
  return value;
}
function index(value: unknown): number {
  if (!(isNumber(value) && Number.isInteger(value) && value >= 0 && value <= U64_MAX)) throw fail("INTEGER_INVALID");
  return value;
}
export function array(value: unknown): Json[] {
  if (!Array.isArray(value)) throw fail("ARRAY_INVALID");
  return value;
}
function numEq(a: unknown, b: unknown): boolean {
  const x = asF64(a), y = asF64(b);
  return x !== null && y !== null && Number.isFinite(x) && Number.isFinite(y) && x === y;
}
/** Structural equality comparing numbers by binary64 value (Rust `same`). */
export function same(a: unknown, b: unknown): boolean {
  if (typeof a === "boolean" || typeof b === "boolean") return typeof a === "boolean" && typeof b === "boolean" && a === b;
  if (isNumber(a) && isNumber(b)) return numEq(a, b);
  if (Array.isArray(a) && Array.isArray(b)) return a.length === b.length && a.every((x, i) => same(x, b[i]));
  if (isObject(a) && isObject(b)) {
    const entries = Object.entries(a);
    return entries.length === Object.keys(b).length && entries.every(([key, value]) => Object.hasOwn(b, key) && same(value, b[key]));
  }
  if (isNumber(a) || isNumber(b) || Array.isArray(a) || Array.isArray(b) || isObject(a) || isObject(b)) return false;
  return (a ?? null) === (b ?? null);
}
/** Rust `Value == literal` for string/bool/null literals. */
export function eq(value: unknown, expected: string | boolean | null): boolean {
  if (typeof expected === "boolean" || typeof value === "boolean") return typeof value === typeof expected && value === expected;
  if (expected === null) return value === null || value === undefined;
  return typeof value === "string" && value === expected;
}
function without(value: unknown, removed: string[]): Json {
  if (!isObject(value)) return value;
  return Object.fromEntries(Object.entries(value).filter(([key]) => !removed.includes(key)));
}
const sha64 = (value: unknown) => typeof value === "string" && /^[0-9a-f]{64}$/.test(value);
function finiteTree(value: unknown): void {
  if (typeof value === "boolean" || value === null || typeof value === "string") return;
  if (isNumber(value)) { number(value); return; }
  if (Array.isArray(value)) { value.forEach(finiteTree); return; }
  if (isObject(value)) { Object.values(value).forEach(finiteTree); return; }
  throw fail("NUMBER_INVALID");
}
function bits(value: number): bigint {
  const view = new DataView(new ArrayBuffer(8)); view.setFloat64(0, value, false);
  return view.getBigUint64(0, false);
}
const positive = (value: number | null) => value === null || value > 0;
const setEq = (a: Iterable<string>, b: Iterable<string>) => {
  const x = new Set(a), y = new Set(b);
  return x.size === y.size && [...x].every(v => y.has(v));
};

/* ---------------------------------------------------------------------------
 * Closed schema interpreter: a peer of Python `source_blocks._shape`, used only
 * with the pinned transport schema. It interprets the same keyword set with the
 * same defaults (array maxItems 16384, anchored string patterns).
 * ------------------------------------------------------------------------ */
function pyEqual(a: unknown, b: unknown): boolean {
  const num = (v: unknown) => typeof v === "boolean" ? Number(v) : v;
  const x = num(a), y = num(b);
  if (isNumber(x) && isNumber(y)) return x === y;
  if (Array.isArray(x) && Array.isArray(y)) return x.length === y.length && x.every((v, i) => pyEqual(v, y[i]));
  if (isObject(x) && isObject(y)) return Object.keys(x).length === Object.keys(y).length && Object.keys(x).every(k => Object.hasOwn(y, k) && pyEqual(x[k], y[k]));
  return (x ?? null) === (y ?? null);
}
export function schemaShape(value: unknown, schema: Json, root: Json): boolean {
  if (Object.hasOwn(schema, "$ref")) return schemaShape(value, root.$defs[String(schema.$ref).replace(/^#\/\$defs\//, "")], root);
  if (Object.hasOwn(schema, "anyOf") && !schema.anyOf.some((branch: Json) => schemaShape(value, branch, root))) return false;
  if (Object.hasOwn(schema, "oneOf") && schema.oneOf.filter((branch: Json) => schemaShape(value, branch, root)).length !== 1) return false;
  if (Object.hasOwn(schema, "allOf") && !schema.allOf.every((branch: Json) => schemaShape(value, branch, root))) return false;
  if (Object.hasOwn(schema, "const") && (!pyEqual(value, schema.const) || (typeof value === "boolean") !== (typeof schema.const === "boolean"))) return false;
  if (Object.hasOwn(schema, "enum") && !schema.enum.some((allowed: unknown) => pyEqual(value, allowed) && (typeof value === "boolean") === (typeof allowed === "boolean"))) return false;
  const kind = Object.hasOwn(schema, "type") ? schema.type : Object.hasOwn(schema, "properties") ? "object" : Object.hasOwn(schema, "items") ? "array" : null;
  if (Array.isArray(kind)) return kind.some(item => schemaShape(value, { ...schema, type: item }, root));
  if (kind === "null") return value === null;
  if (kind === "boolean") return typeof value === "boolean";
  if (kind === "object") {
    if (!isObject(value)) return false;
    const properties = schema.properties ?? {};
    if (!(schema.required ?? []).every((key: string) => Object.hasOwn(value, key))) return false;
    if (schema.additionalProperties === false && !Object.keys(value).every(key => Object.hasOwn(properties, key))) return false;
    return Object.entries(value).every(([key, item]) => !Object.hasOwn(properties, key) || schemaShape(item, properties[key], root));
  }
  if (kind === "array") {
    return Array.isArray(value) && (schema.minItems ?? 0) <= value.length && value.length <= (schema.maxItems ?? 16384)
      && value.every(item => schemaShape(item, schema.items, root));
  }
  if (kind === "string") {
    return typeof value === "string" && value.length >= (schema.minLength ?? 0)
      && (!Object.hasOwn(schema, "pattern") || new RegExp(`^(?:${String(schema.pattern).replace(/^\^/, "").replace(/\$$/, "")})$`, "u").test(value));
  }
  if (kind === "number" || kind === "integer") {
    return isNumber(value) && Number.isFinite(value) && (kind !== "integer" || Number.isInteger(value))
      && (schema.minimum ?? -Infinity) <= value && value <= (schema.maximum ?? Infinity) && value > (schema.exclusiveMinimum ?? -Infinity);
  }
  return true;
}

async function sha256Hex(textValue: string): Promise<string> {
  const digest = await globalThis.crypto.subtle.digest("SHA-256", new TextEncoder().encode(textValue));
  return [...new Uint8Array(digest)].map(byte => byte.toString(16).padStart(2, "0")).join("");
}
/** Pinned table bytes: identity, profile and sha256 are checked, never inferred. */
export async function verifyLoadReferenceTable(bytes: string): Promise<Json> {
  if (await sha256Hex(bytes) !== LOAD_REFERENCE_TABLE_SHA256) throw new LoadReferenceError("SOURCE_LOAD_REFERENCE_TABLE_HASH");
  const table = JSON.parse(bytes);
  if (!isObject(table) || table.semantic_contract_id !== LOAD_REFERENCE_CONTRACT_ID || table.formulation_profile_id !== LOAD_REFERENCE_PROFILE) throw new LoadReferenceError("SOURCE_LOAD_REFERENCE_TABLE_IDENTITY");
  return table;
}
/** Pinned transport schema bytes (the checked-in JSON file). */
export async function verifyLoadReferenceTransportSchema(bytes: string): Promise<Json> {
  if (await sha256Hex(bytes) !== LOAD_REFERENCE_TRANSPORT_SCHEMA_SHA256) throw new LoadReferenceError("SOURCE_LOAD_REFERENCE_TRANSPORT_SCHEMA_HASH");
  return JSON.parse(bytes);
}
const transportSchema: Json = transportSchemaJson;

type PhysicsModel = Pick<PreviewModel, "load_cases"> & Partial<Pick<PreviewModel, "pipe_segments" | "supports">>;

/** Raw load-reference-1 publication: closed evidence, joins, rows and diagnostics.
 * The optional model is passed to the physics-1 validator exactly as the
 * physics-1 route passes it (standing only; parity runs without a model). */
export function validateLoadReferenceEvidence(source: MechanicsResult, model?: PhysicsModel): void {
  guarded(() => validate(source, true, model));
}
/** Retained statements without raw rows: frozen schema shape and internal joins only. */
export function validateLoadReferenceTransportMetadata(source: Partial<MechanicsResult>): void {
  guarded(() => validate(source as MechanicsResult, false));
}
export function guarded<T>(action: () => T): T {
  try { return action(); }
  catch (error) {
    if (error instanceof LoadReferenceError) throw error;
    throw new LoadReferenceError("SOURCE_LOAD_REFERENCE_MALFORMED");
  }
}
function validate(source: MechanicsResult, raw: boolean, model?: PhysicsModel): void {
  prepass(source, raw, "load_reference");
  // S14 inherited physics-1 checks on the projected copy.
  const projected = project(source);
  try {
    if (raw) validatePhysicsEvidence(projected, model);
    else validatePhysicsTransportMetadata(projected.contract_evidence);
  } catch (error) {
    throw new LoadReferenceError(`${code("PHYSICS_EVIDENCE")}: ${error instanceof Error && error.message ? error.message : "SOURCE_PHYSICS_EVIDENCE_INVALID: malformed evidence"}`);
  }
}

const isSelected = (record: unknown) => eq(get(get(record, "solve"), "recovery_method"), EXACT_METHOD);

/** Steps S1-S13, shared with the joined reader (Rust `load_reference::prepass`). */
export function prepass(source: Json, raw: boolean, method: LoadReferenceMethod): void {
  const joined = method === "joined";
  // S1 foreign method namespaces.
  if (joined) {
    require_(!isObject(source) || !Object.hasOwn(source, "carrier_evidence"), "FOREIGN_METHOD_EVIDENCE");
    require_(isObject(source) && Object.hasOwn(source, "source_block_recovery"), "JOIN_RECEIPT_REQUIRED");
  } else {
    require_(!isObject(source) || (!Object.hasOwn(source, "source_block_recovery") && !Object.hasOwn(source, "carrier_evidence")), "FOREIGN_METHOD_EVIDENCE");
  }
  // S2 finite numbers everywhere.
  finiteTree(source);
  const evidence = get(source, "contract_evidence");
  if (!raw && !joined) require_(schemaShape(evidence, transportSchema.$defs.LoadReferenceContractEvidence, transportSchema), "TRANSPORT_SHAPE");
  // S3-S5 namespace.
  require_(keys(evidence, EVIDENCE_KEYS), "EVIDENCE_SHAPE");
  require_(!array(at(evidence, "connector")).length, "CONNECTOR_UNSUPPORTED");
  const pressure = array(at(evidence, "pressure"));
  const exact = array(at(evidence, "exact_cases"));
  const records = array(at(evidence, "load_reference_states"));
  // S6 numerical case identities.
  const qualityIds: string[] = [];
  for (const c of array(get(get(source, "numerical_quality"), "cases"))) {
    const basis = get(c, "basis_ref");
    require_(keys(basis, ["ref_type", "ref_id"]) && eq(basis.ref_type, "load_case"), "NUMERICAL_CASE_BASIS");
    const caseId = text(basis.ref_id);
    require_(!qualityIds.includes(caseId), "NUMERICAL_CASE_DUPLICATE");
    qualityIds.push(caseId);
  }
  // S7 exact cases, their resolved materials and section identities.
  const cases: [string, Json][] = [];
  const caseKeys = joined ? [...CASE_KEYS, "recovery_method"] : CASE_KEYS;
  for (const c of exact) {
    require_(keys(c, caseKeys), "CASE_SHAPE");
    const caseId = text(c.load_case_id);
    require_(cases.every(([known]) => known !== caseId), "CASE_DUPLICATE");
    require_(eq(c.material_basis, MATERIAL_BASIS), "MATERIAL_BASIS");
    const pipes: string[] = [];
    for (const material of array(c.pipe_materials)) {
      require_(keys(material, MATERIAL_KEYS), "MATERIAL_SHAPE");
      const pipe = text(material.pipe_id);
      require_(!pipes.includes(pipe), "MATERIAL_DUPLICATE");
      pipes.push(pipe);
    }
    const sections: string[] = [];
    for (const section of array(c.pipe_sections)) {
      const pipe = text(get(section, "pipe_id"));
      require_(!sections.includes(pipe), "SECTION_DUPLICATE");
      sections.push(pipe);
    }
    cases.push([caseId, c]);
  }
  // S8 record identities.
  const recordIds: string[] = [];
  for (const record of records) {
    require_(keys(record, RECORD_KEYS), "RECORD_SHAPE");
    const caseId = text(record.load_case_id);
    require_(!recordIds.includes(caseId), "RECORD_DUPLICATE");
    recordIds.push(caseId);
  }
  // S9 case trijection. An unsolved envelope retains the empty namespace.
  const solved = !raw || eq(get(get(source, "status"), "mechanics"), "MECHANICS_SOLVED");
  if (solved) {
    require_(recordIds.length > 0 && setEq(recordIds, qualityIds) && setEq(cases.map(([id]) => id), qualityIds), "CASE_COVERAGE");
  } else {
    require_(!records.length && !exact.length && !pressure.length, "UNSOLVED_EVIDENCE");
  }
  // S10 each record in document order.
  for (const record of records) validateRecord(record, cases, pressure, method);
  // S10b (joined) the published method of each case agrees with its exact case,
  // and at least one case publishes the selected response.
  if (joined) {
    for (const record of records) {
      const found = cases.find(([known]) => eq(record.load_case_id, known));
      if (!found) throw fail("RECORD_CASE_UNRESOLVED");
      require_(same(at(found[1], "recovery_method"), get(record.solve, "recovery_method")), "JOIN_RECOVERY_METHOD");
    }
    require_(records.some(isSelected), "JOIN_SELECTION_REQUIRED");
  }
  // S11 one model geometry and one requested mode per envelope.
  if (records.length) {
    const first = records[0];
    for (const record of records) {
      require_(same(get(record.reference_geometry, "projection_sha256"), get(first.reference_geometry, "projection_sha256")), "REFERENCE_GEOMETRY_CONSISTENCY");
      require_(same(get(record.solve, "requested_mode"), get(first.solve, "requested_mode")), "SOLVE_CONSISTENCY");
    }
  }
  // S12 pressure-region materials are the resolved member pair of their case.
  for (const region of pressure) {
    require_(keys(region, REGION_KEYS), "REGION_SHAPE");
    const caseId = text(region.load_case_id);
    text(region.region_id);
    const found = cases.find(([known]) => known === caseId);
    if (!found) throw fail("REGION_CASE_UNRESOLVED");
    for (const material of array(region.materials)) {
      require_(keys(material, REGION_MATERIAL_KEYS), "REGION_MATERIAL_SHAPE");
      require_(eq(material.temperature_basis, REGION_TEMPERATURE_BASIS), "REGION_TEMPERATURE_BASIS");
      const pipe = text(material.pipe_id);
      const member = array(at(found[1], "pipe_materials")).find(m => eq(get(m, "pipe_id"), pipe));
      if (member === undefined) throw fail("REGION_MATERIAL_BINDING");
      require_(same(without(material, ["temperature_basis"]), without(member, ["material_selection_kind", "resolved_eigenstrain"])), "REGION_MATERIAL_BINDING");
    }
  }
  // S13 exactly one not-joined info diagnostic per resolved case. In the joined
  // method a selected case instead carries exactly one info
  // SOURCE_BLOCK_RECOVERY_SELECTED diagnostic and no not-joined diagnostic.
  if (raw && solved) {
    const diagnostics = array(get(source, "diagnostics"));
    let notJoined = 0, selected = 0;
    recordIds.forEach((caseId, i) => {
      const record = records[i];
      let kind: string, expected: string, name: string;
      if (joined && isSelected(record)) {
        selected += 1;
        [kind, expected, name] = [SELECTED, `diagnostic:source-recovery:${caseId}:selected`, "JOIN_SELECTED_DIAGNOSTIC"];
      } else {
        notJoined += 1;
        [kind, expected, name] = [NOT_JOINED, `diagnostic:load-state:${caseId.replaceAll(":", "-")}:source-recovery-not-joined`, "NOT_JOINED_DIAGNOSTIC"];
      }
      const hits = diagnostics.filter(d => eq(get(d, "code"), kind) && eq(get(d, "id"), expected));
      require_(hits.length === 1 && eq(get(hits[0], "severity"), "info") && same(get(hits[0], "affected_refs"), [caseId]), name);
    });
    require_(diagnostics.filter(d => eq(get(d, "code"), NOT_JOINED)).length === notJoined, "NOT_JOINED_DIAGNOSTIC");
    if (joined) {
      require_(diagnostics.filter(d => eq(get(d, "code"), SELECTED)).length === selected, "JOIN_SELECTED_DIAGNOSTIC");
      // A selected case's attempt succeeded, so no UNAVAILABLE diagnostic may name it.
      const selectedIds = recordIds.filter((_, i) => isSelected(records[i]));
      require_(!diagnostics.some(d => eq(get(d, "code"), UNAVAILABLE) && Array.isArray(get(d, "affected_refs"))
        && at(d, "affected_refs").some((r: unknown) => typeof r === "string" && selectedIds.includes(r))), "JOIN_SELECTED_UNAVAILABLE_DIAGNOSTIC");
    }
  }
}

/** The physics-1 form of the pre-passed bytes: removes or neutralizes only what
 * the pre-pass bound. Never mutates its input. */
export function project(source: MechanicsResult): MechanicsResult {
  const projected = structuredClone(source) as Json;
  const evidence = isObject(projected) ? projected.contract_evidence : null;
  if (isObject(evidence)) {
    delete evidence.load_reference_states;
    for (const c of Array.isArray(evidence.exact_cases) ? evidence.exact_cases : []) {
      if (!isObject(c)) continue;
      c.material_basis = "base_material_common_E_nu";
      for (const material of Array.isArray(c.pipe_materials) ? c.pipe_materials : []) {
        if (isObject(material)) { delete material.material_selection_kind; delete material.resolved_eigenstrain; }
      }
    }
    for (const region of Array.isArray(evidence.pressure) ? evidence.pressure : []) {
      if (!isObject(region)) continue;
      for (const material of Array.isArray(region.materials) ? region.materials : []) {
        if (isObject(material)) material.temperature_basis = { selection: "base_material" };
      }
    }
  }
  return projected;
}

function validateRecord(record: Json, cases: [string, Json][], pressure: Json[], method: LoadReferenceMethod): void {
  const caseId = text(record.load_case_id);
  // R1-R7 record-level closed statements.
  require_(eq(record.contract, RECORD_CONTRACT), "RECORD_CONTRACT");
  require_(eq(record.profile, LOAD_REFERENCE_PROFILE), "RECORD_PROFILE");
  const configuration = text(record.reference_configuration_id);
  text(record.provenance);
  const geometry = record.reference_geometry;
  require_(keys(geometry, ["kind", "projection_sha256"]), "REFERENCE_GEOMETRY_SHAPE");
  require_(eq(geometry.kind, "authored_model_geometry") && sha64(geometry.projection_sha256), "REFERENCE_GEOMETRY");
  const history = record.history;
  require_(keys(history, ["kind"]), "HISTORY_SHAPE");
  require_(eq(history.kind, "independent_equilibrium"), "HISTORY");
  const solve = record.solve;
  require_(keys(solve, ["requested_mode", "recovery_method", "boundary", "eigenload"]), "SOLVE_SHAPE");
  const pair = [solve.requested_mode, solve.recovery_method];
  // The joined method also admits the selected retained-source response in
  // either requested mode (ADDENDUM_2 section 5.3).
  const selected = method === "joined" && eq(solve.recovery_method, EXACT_METHOD);
  const modes = ["sparse_interactive", "dense_scrutiny"];
  require_(pair.every(item => typeof item === "string")
    && ((pair[0] === "sparse_interactive" && pair[1] === "ordinary_sparse_structural_v1")
      || (pair[0] === "dense_scrutiny" && pair[1] === "ordinary_dense_structural_v1")
      || (selected && modes.includes(pair[0])))
    && eq(solve.boundary, BOUNDARY) && eq(solve.eigenload, EIGENLOAD), "SOLVE");
  const recovery = record.source_recovery;
  if (selected) {
    require_(keys(recovery, ["status", "method"]), "SOURCE_RECOVERY_SHAPE");
    require_(eq(recovery.status, "selected") && eq(recovery.method, EXACT_METHOD), "SOURCE_RECOVERY");
  } else {
    require_(keys(recovery, ["status", "code"]), "SOURCE_RECOVERY_SHAPE");
    require_(eq(recovery.status, "not_joined") && eq(recovery.code, NOT_JOINED), "SOURCE_RECOVERY");
  }
  // R8 members.
  const members = array(record.members);
  const memberIds: string[] = [];
  for (const member of members) {
    validateMember(member);
    const pipe = text(member.pipe_id);
    require_(!memberIds.includes(pipe), "MEMBER_DUPLICATE");
    memberIds.push(pipe);
  }
  // R9 prescribed support components.
  const components = array(record.support_components);
  const componentIds: [string, string][] = [];
  const supportNodes = new Map<string, string>(), nodeBases = new Map<string, number>();
  for (const component of components) {
    validateSupportComponent(component);
    const support = text(component.support_id), node = text(component.node_id), dof = text(component.dof);
    require_(!componentIds.some(([s, d]) => s === support && d === dof), "SUPPORT_COMPONENT_DUPLICATE");
    componentIds.push([support, dof]);
    const base = Math.floor(index(component.global_dof) / 6);
    if (!supportNodes.has(support)) supportNodes.set(support, node);
    const nodeOk = supportNodes.get(support) === node;
    if (nodeOk && !nodeBases.has(node)) nodeBases.set(node, base);
    require_(nodeOk && nodeBases.get(node) === base, "SUPPORT_COMPONENT_NODE");
  }
  // R10 contributions.
  const contributions = array(record.contributions);
  const contributionIds: string[] = [];
  for (const contribution of contributions) {
    validateContribution(contribution);
    const sourceId = text(contribution.source_id);
    require_(!contributionIds.includes(sourceId), "CONTRIBUTION_DUPLICATE");
    contributionIds.push(sourceId);
  }
  // R11 excluded sources.
  const excludedIds: string[] = [];
  for (const excluded of array(record.excluded_sources)) {
    require_(keys(excluded, EXCLUDED_KEYS), "EXCLUDED_SHAPE");
    require_(eq(excluded.owner_kind, "stored_primitive") && eq(excluded.classification, "excluded"), "EXCLUDED_CLASSIFICATION");
    const sourceId = text(excluded.source_id);
    text(excluded.category);
    text(excluded.reason);
    require_(!contributionIds.includes(sourceId) && !excludedIds.includes(sourceId), "EXCLUDED_OVERLAP");
    excludedIds.push(sourceId);
  }
  // R12-R14 binding to the exact case of the same load case.
  const found = cases.find(([known]) => known === caseId);
  if (!found) throw fail("RECORD_CASE_UNRESOLVED");
  const c = found[1];
  const sections = array(at(c, "pipe_sections")).map(s => typeof get(s, "pipe_id") === "string" ? s.pipe_id : "");
  require_(memberIds.length > 0 && setEq(memberIds, sections), "MEMBER_COVERAGE");
  const materials = array(at(c, "pipe_materials"));
  require_(setEq(materials.map(m => typeof get(m, "pipe_id") === "string" ? m.pipe_id : ""), memberIds), "MEMBER_MATERIAL_COVERAGE");
  for (const member of members) {
    const material = materials.find(m => same(get(m, "pipe_id"), member.pipe_id));
    if (material === undefined) throw fail("MEMBER_MATERIAL_COVERAGE");
    require_(numEq(material.E_pa, member.selected_E_pa)
      && numEq(material.nu, member.selected_nu)
      && numEq(material.G_pa, member.derived_G_pa)
      && same(material.material_id, member.material_id)
      && same(material.material_selection_kind, member.material_selection_kind)
      && numEq(material.resolved_eigenstrain, member.total_eigenstrain)
      && eq(material.thermal_consumed, false)
      && material.alpha_per_kelvin === null, "MEMBER_MATERIAL_BINDING");
    text(material.provenance);
  }
  // R15 contribution ledgers are bijections with their owners.
  const ofKind = (kind: string) => contributions.filter(x => eq(get(x, "owner_kind"), kind) && typeof get(x, "source_id") === "string").map(x => x.source_id as string);
  const find = (sourceId: string) => contributions.find(x => eq(get(x, "source_id"), sourceId));
  require_(setEq(ofKind("resolved_member_state"), memberIds.map(pipe => `member_state:${pipe}`)), "MEMBER_CONTRIBUTION_COVERAGE");
  for (const member of members) {
    const pipe = text(member.pipe_id);
    const contribution = find(`member_state:${pipe}`);
    if (contribution === undefined) throw fail("MEMBER_CONTRIBUTION_COVERAGE");
    require_(numEq(get(contribution, "value"), member.total_eigenstrain)
      && same(get(contribution, "consumed_input_refs"), [`${configuration}:${pipe}`, `${caseId}:element_state:${pipe}`]), "MEMBER_CONTRIBUTION_BINDING");
  }
  require_(setEq(ofKind("support_state"), componentIds.map(([support, dof]) => `support_state:${support}:${dof}`)), "SUPPORT_CONTRIBUTION_COVERAGE");
  for (const component of components) {
    const contribution = find(`support_state:${text(component.support_id)}:${text(component.dof)}`);
    if (contribution === undefined) throw fail("SUPPORT_CONTRIBUTION_COVERAGE");
    require_(numEq(get(contribution, "value"), component.prescribed_value), "SUPPORT_CONTRIBUTION_BINDING");
  }
  const expectedRegions = new Set<string>();
  for (const region of pressure) {
    if (eq(get(region, "load_case_id"), caseId)) expectedRegions.add(`pressure_region:${text(get(region, "region_id"))}`);
  }
  require_(setEq(ofKind("pressure_region"), expectedRegions), "PRESSURE_CONTRIBUTION_COVERAGE");
}

function validateMember(member: Json): void {
  // M1-M3 shape, identity and fixed statements.
  require_(keys(member, MEMBER_KEYS), "MEMBER_SHAPE");
  text(member.pipe_id);
  text(member.material_id);
  require_(eq(member.G_basis, G_BASIS) && eq(member.eigenstrain_composition, COMPOSITION), "MEMBER_BASIS");
  // M4 selected pair and derived G.
  const e = number(member.selected_E_pa), nu = number(member.selected_nu), g = number(member.derived_G_pa);
  require_(e > 0 && -1 < nu && nu < 0.5 && g > 0, "MEMBER_MATERIAL_RANGE");
  const direct = e / (2.0 * (1.0 + nu));
  const delta = Number.isFinite(direct) ? bits(g) - bits(direct) : 3n;
  require_(Number.isFinite(direct) && delta >= -2n && delta <= 2n, "MEMBER_G_BINDING");
  const retained = boolean(member.retained_G_ignored);
  // M6 consumed material points.
  const consumed: [number | null, number, number, boolean][] = [];
  for (const point of array(member.consumed_material_points)) {
    require_(keys(point, POINT_KEYS), "MATERIAL_POINT_SHAPE");
    text(point.point_id);
    const temperature = optNumber(point.temperature_k);
    require_(positive(temperature), "TEMPERATURE_RANGE");
    const pe = number(point.E_pa), pnu = number(point.nu);
    require_(pe > 0 && -1 < pnu && pnu < 0.5, "MATERIAL_POINT_RANGE");
    consumed.push([temperature, pe, pnu, boolean(point.retained_G_ignored)]);
  }
  // M7-M9 optional selection data and temperatures.
  const fraction = optNumber(member.interpolation_fraction);
  const applicability = optText(member.applicability_reference);
  const override = member.analysis_basis_override;
  if (override !== null) {
    require_(keys(override, ["reason", "provenance"]), "OVERRIDE_SHAPE");
    text(override.reason);
    text(override.provenance);
  }
  const operating = optNumber(member.operating_temperature_k);
  const selection = optNumber(member.material_selection_temperature_k);
  const installation = optNumber(member.installation_temperature_k);
  const datum = optNumber(member.coefficient_datum_k);
  require_([operating, selection, installation, datum].every(positive), "TEMPERATURE_RANGE");
  // M10 material selection.
  const kind = member.material_selection_kind;
  let selected: boolean;
  if (eq(kind, "explicit_base_properties")) {
    selected = !consumed.length && fraction === null && applicability !== null && selection === null;
  } else if (eq(kind, "exact_point")) {
    selected = consumed.length === 1 && fraction === null && applicability === null && e === consumed[0][1] && nu === consumed[0][2] && selection === consumed[0][0] && retained === consumed[0][3];
  } else if (eq(kind, "temperature_interpolation")) {
    let shaped: boolean;
    if (consumed.length === 1) shaped = fraction === null && e === consumed[0][1] && nu === consumed[0][2] && selection === consumed[0][0];
    else if (consumed.length === 2) shaped = fraction !== null && 0 < fraction && fraction < 1 && consumed.every(point => point[0] !== null);
    else shaped = false;
    selected = applicability === null && selection !== null && retained === consumed.some(point => point[3]) && shaped;
  } else {
    throw fail("MATERIAL_SELECTION_KIND");
  }
  require_(selected, "MATERIAL_SELECTION");
  // M11 reference basis.
  const basis = member.reference_basis;
  require_((eq(basis, "temperature_reference") && installation !== null) || (eq(basis, "direct_strain_reference") && installation === null), "REFERENCE_BASIS");
  // M12 strains and stretches.
  const thermalStrain = number(member.thermal_strain), thermalStretch = number(member.thermal_stretch);
  const fitStrain = number(member.fit_strain), fitStretch = number(member.fit_stretch), total = number(member.total_eigenstrain);
  require_(thermalStretch > 0 && fitStretch > 0 && thermalStretch === 1.0 + thermalStrain && fitStretch === 1.0 + fitStrain && 1.0 + total > 0, "STRETCH_BINDING");
  // M13 consumed and consulted law data.
  let lawDataEmpty = true;
  for (const key of ["consumed_law_point_indices", "consulted_law_point_indices"]) {
    let previous: number | null = null;
    for (const item of array(at(member, key))) {
      const i = index(item);
      require_(previous === null || previous < i, "LAW_INDICES");
      previous = i;
      lawDataEmpty = false;
    }
  }
  // CP3_WIRE_ADDENDUM section 1: adjacent indices, a sample at one temperature,
  // an interval over start < end, and no repeated entry within one list.
  for (const key of ["consumed_law_segments", "consulted_law_segments"]) {
    const entries: [boolean, number, number, number, number][] = [];
    for (const segment of array(at(member, key))) {
      require_(keys(segment, SEGMENT_KEYS), "LAW_SEGMENT_SHAPE");
      let sample: boolean;
      if (eq(segment.use, "interpolation_sample")) sample = true;
      else if (eq(segment.use, "integration_interval")) sample = false;
      else throw fail("LAW_SEGMENT");
      const lower = index(segment.lower_index), upper = index(segment.upper_index);
      const start = number(segment.start_k), end = number(segment.end_k);
      require_(upper === lower + 1 && (sample ? start === end : start < end), "LAW_SEGMENT");
      const entry: [boolean, number, number, number, number] = [sample, lower, upper, start, end];
      require_(!entries.some(other => other.every((v, i) => v === entry[i])), "LAW_SEGMENT_DUPLICATE");
      entries.push(entry);
      lawDataEmpty = false;
    }
  }
  const installationStretch = optNumber(member.installation_datum_stretch);
  const operatingStretch = optNumber(member.operating_datum_stretch);
  const lawId = optText(member.expansion_law_id);
  // M14 thermal definition.
  const definition = typeof member.thermal_definition === "string" ? member.thermal_definition : "";
  if (LAW_DEFINITIONS.includes(definition)) {
    require_(lawId !== null && datum !== null
      && installationStretch !== null && installationStretch > 0
      && operatingStretch !== null && operatingStretch > 0
      && installation !== null && operating !== null, "THERMAL_LAW_BINDING");
  } else if (DIRECT_DEFINITIONS.includes(definition)) {
    require_(lawId === null && datum === null && installationStretch === null && operatingStretch === null
      && lawDataEmpty && (definition !== "unchanged_reference" || thermalStrain === 0), "THERMAL_LAW_BINDING");
  } else {
    throw fail("THERMAL_DEFINITION");
  }
  // M15-M16 reference length and fit.
  const length = number(member.reference_length_m);
  require_(length > 0, "REFERENCE_LENGTH");
  const fit = member.fit_input, fitKind = member.fit_kind;
  let fitted: boolean;
  if (eq(fitKind, "none")) {
    fitted = fit === null && fitStrain === 0;
  } else if (eq(fitKind, "natural_length_change")) {
    const change = asF64(get(fit, "length_change_m"));
    fitted = keys(fit, ["length_change_m"]) && change !== null && Number.isFinite(change) && fitStrain === change / length && length + change > 0;
  } else if (eq(fitKind, "fit_strain")) {
    const strain = asF64(get(fit, "strain"));
    fitted = keys(fit, ["strain"]) && strain !== null && Number.isFinite(strain) && fitStrain === strain;
  } else {
    throw fail("FIT_KIND");
  }
  require_(fitted, "FIT_BINDING");
}

function validateSupportComponent(component: Json): void {
  require_(keys(component, SUPPORT_KEYS), "SUPPORT_COMPONENT_SHAPE");
  text(component.support_id);
  text(component.node_id);
  const dof = component.dof;
  if (!(typeof dof === "string" && DOFS.includes(dof))) throw fail("SUPPORT_COMPONENT_DOF");
  const position = DOFS.indexOf(dof);
  const globalDof = index(component.global_dof);
  require_(globalDof % 6 === position, "SUPPORT_COMPONENT_DOF");
  require_(eq(component.law_kind, "rigid_prescribed") && eq(component.meaning, "absolute_reference_displacement") && eq(component.physical_state_source, "support_state.boundary_motion"), "SUPPORT_COMPONENT_LAW");
  require_(eq(component.unit, position < 3 ? "m" : "rad"), "SUPPORT_COMPONENT_UNIT");
  number(component.prescribed_value);
}

const CONTRIBUTION_SHAPES = new Map<string, [string[], string]>([
  ["stored_primitive", [STORED_KEYS, "ordinary_applied"]],
  ["resolved_member_state", [MEMBER_STATE_KEYS, "eigenstrain"]],
  ["support_state", [SUPPORT_STATE_KEYS, "prescribed_boundary"]],
  ["pressure_region", [PRESSURE_REGION_KEYS, "pressure_eigen_and_closure"]],
]);
function validateContribution(contribution: Json): void {
  const kind = get(contribution, "owner_kind");
  if (!(typeof kind === "string" && CONTRIBUTION_SHAPES.has(kind))) throw fail("CONTRIBUTION_KIND");
  const [shape, classification] = CONTRIBUTION_SHAPES.get(kind)!;
  require_(keys(contribution, shape), "CONTRIBUTION_SHAPE");
  require_(eq(contribution.classification, classification), "CONTRIBUTION_CLASSIFICATION");
  text(contribution.source_id);
  if (kind === "stored_primitive") {
    const factor = asF64(contribution.factor);
    if (!(factor !== null && Number.isFinite(factor) && factor !== 0)) throw fail("CONTRIBUTION_FACTOR");
    text(contribution.category);
    text(contribution.dimension);
    const authored = number(contribution.authored_normalized_magnitude);
    const applied = number(contribution.applied_magnitude);
    require_(applied === authored * factor, "CONTRIBUTION_APPLIED_MAGNITUDE");
  } else if (kind === "resolved_member_state") {
    for (const reference of array(contribution.consumed_input_refs)) text(reference);
    number(contribution.value);
  } else if (kind === "support_state") {
    number(contribution.value);
  } else {
    require_(contribution.factor === null, "CONTRIBUTION_FACTOR");
  }
}
