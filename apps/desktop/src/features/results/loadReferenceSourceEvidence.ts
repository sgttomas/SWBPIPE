/** Closed admission of the joined load-reference-source-1 evidence.
 *
 * TypeScript peer of `core/analysis_runs/load_reference_source.py` and
 * `core/reporting/result_export/src/load_reference_source.rs`: the same checks
 * run in the same order. Contract
 * `openpipestress.result_semantics/0.3.0/load-reference-source-1`, profile
 * `resolved_straight_load_state_source_v1`, receipt policy
 * `LOAD-REFERENCE-SOURCE-1` (CP2_WIRE_ADDENDUM_2 section 5, CP4 corrections).
 *
 * 1. J0 identity and profile, then the load-reference pre-pass (S1-S13) in its
 *    joined form (`loadReferenceEvidence.prepass`).
 * 2. J2 the receipt on the received bytes: policy, closed shape, receipt hash,
 *    publication hash (raw only), case order, per-case method and requested
 *    mode, and the per-case physical-evidence hash over domain
 *    `load_reference_source_case_evidence_v1`.
 * 3. J3 the physics-source-1 checks on a projected copy, with no invocation, as
 *    Python `validate_physics_source(projected, None)` runs them:
 *    - transport statements (`validatePhysicsSourceTransportMetadata`),
 *    - the composite physical evidence (`validatePhysicsSourceEvidence`),
 *    - the source-blocks row ledger: an invocation-free, check-for-check port
 *      of Python `source_blocks._validate_source_blocks(source, None,
 *      context=physics_source)` below. It is used only on this route; the
 *      source-blocks-1 and physics-source-1 routes keep their own readers.
 *
 * Numerical eligibility is never granted: a reader cannot re-derive a 0.4.0
 * resolved case from a captured request, so an admitted envelope is
 * `needs_recompute` (T1_WAVE1_RULINGS.md section 7). This establishes internal
 * source consistency, never producer origin, solver accuracy or model freshness.
 */
import physicsSourceSchemaJson from "../../../../../schemas/physics_source_recovery.schema.json";
import physicsSourceTable from "../../../../../fixtures/results/semantic_contract_v0_3_physics_source_1.json";
import { canonicalSha256HexCheckedV1, checkedJsonText } from "../../services/hashService";
import { validatePhysicsSourceEvidence } from "./physicsResultEvidence";
import { validatePhysicsSourceDerived, validatePhysicsSourceTransportMetadata, PHYSICS_SOURCE_SUPPORT_SIGN } from "./physicsSourceRecovery";
import {
  LoadReferenceError, at, code, eq, fail, get, isObject, prepass, project as projectLoadReference, require_, schemaShape,
} from "./loadReferenceEvidence";
import type { MechanicsResult } from "../../types";

export const LOAD_REFERENCE_SOURCE_CONTRACT_ID = "openpipestress.result_semantics/0.3.0/load-reference-source-1";
export const LOAD_REFERENCE_SOURCE_PROFILE = "resolved_straight_load_state_source_v1";
export const LOAD_REFERENCE_SOURCE_TABLE_SHA256 = "d1628194a7730f427843b00228dd233cf92b8e7d26f3bc31c660a3ea59e28337";
export const LOAD_REFERENCE_SOURCE_POLICY = "LOAD-REFERENCE-SOURCE-1";
const CASE_EVIDENCE_DOMAIN = "load_reference_source_case_evidence_v1";
const PHYSICS_SOURCE_ID = "openpipestress.result_semantics/0.3.0/physics-source-1";
const PHYSICS_SOURCE_PROFILE = "exact_straight_pressure_v2";
const PHYSICS_SOURCE_POLICY = "PHYSICS-SOURCE-1";
const PHYSICS_SOURCE_CASE_DOMAIN = "physics_source_case_evidence_v1";
/** The T1 early `needs_recompute` reason for an admitted joined envelope. */
export const LOAD_REFERENCE_SOURCE_NOT_NUMERICALLY_ELIGIBLE = "LOAD_REFERENCE_SOURCE_NOT_NUMERICALLY_ELIGIBLE_IN_T1";
/** No completed joined-reader validation is registered for these exact bytes. */
export const LOAD_REFERENCE_SOURCE_VALIDATION_REQUIRED = "SOURCE_LOAD_REFERENCE_SOURCE_VALIDATION_REQUIRED";

type Json = any; // eslint-disable-line @typescript-eslint/no-explicit-any
const physicsSourceSchema: Json = physicsSourceSchemaJson;
const domainHash = (domain: string, payload: unknown) => canonicalSha256HexCheckedV1({ domain, payload });

async function sha256Hex(textValue: string): Promise<string> {
  const digest = await globalThis.crypto.subtle.digest("SHA-256", new TextEncoder().encode(textValue));
  return [...new Uint8Array(digest)].map(byte => byte.toString(16).padStart(2, "0")).join("");
}
/** Pinned table bytes: identity, profile, policy and sha256 are checked, never inferred. */
export async function verifyLoadReferenceSourceTable(bytes: string): Promise<Json> {
  if (await sha256Hex(bytes) !== LOAD_REFERENCE_SOURCE_TABLE_SHA256) throw new LoadReferenceError("SOURCE_LOAD_REFERENCE_SOURCE_TABLE_HASH");
  const table = JSON.parse(bytes);
  if (!isObject(table) || table.semantic_contract_id !== LOAD_REFERENCE_SOURCE_CONTRACT_ID || table.formulation_profile_id !== LOAD_REFERENCE_SOURCE_PROFILE || table.source_block_policy !== LOAD_REFERENCE_SOURCE_POLICY) {
    throw new LoadReferenceError("SOURCE_LOAD_REFERENCE_SOURCE_TABLE_IDENTITY");
  }
  return table;
}

/** Header dispatch receipt shape: policy, then the closed physics-source-1
 * receipt schema with only the policy constant substituted (J2 R1-R2). */
export function loadReferenceSourceReceiptShape(value: unknown): boolean {
  if (!isObject(value) || !eq(get(value.body, "policy"), LOAD_REFERENCE_SOURCE_POLICY)) return false;
  try {
    const shaped = structuredClone(value);
    shaped.body.policy = PHYSICS_SOURCE_POLICY;
    return schemaShape(shaped, physicsSourceSchema, physicsSourceSchema);
  } catch { return false; }
}

/* ------------------------------------------------------------------------ *
 * Private registration for synchronous standing. Only a completed validation
 * of these exact bytes registers; a copy, a header or a hash never does.
 * ------------------------------------------------------------------------ */
type Registration = { text: string; negativeZeros: string; error: string | null };
const registrations = new WeakMap<object, Registration>();
function negativeZeroPaths(value: unknown): string {
  const paths: string[] = [];
  const visit = (item: unknown, path: string) => {
    if (Object.is(item, -0)) paths.push(path);
    else if (item && typeof item === "object") for (const [key, child] of Object.entries(item)) visit(child, `${path}/${key}`);
  };
  visit(value, "");
  return JSON.stringify(paths);
}
function fingerprint(source: unknown): { text: string; negativeZeros: string } | null {
  try { return { text: checkedJsonText(source), negativeZeros: negativeZeroPaths(source) }; } catch { return null; }
}
/** Standing findings: the registered validation error, the missing-validation
 * reason, or (admitted) the declared T1 early needs_recompute reason. */
export function loadReferenceSourceStanding(source: MechanicsResult): { eligible: false; findings: string[] } {
  const registration = registrations.get(source);
  const current = fingerprint(source);
  if (!registration || !current || current.text !== registration.text || current.negativeZeros !== registration.negativeZeros) {
    return { eligible: false, findings: [LOAD_REFERENCE_SOURCE_VALIDATION_REQUIRED] };
  }
  return { eligible: false, findings: [registration.error ?? LOAD_REFERENCE_SOURCE_NOT_NUMERICALLY_ELIGIBLE] };
}

/** Raw joined publication. Resolves to false: admitted, never numerically
 * eligible. Registers the outcome (admitted or its error) for standing. */
export async function validateLoadReferenceSourceEvidence(source: MechanicsResult): Promise<false> {
  const before = fingerprint(source);
  try {
    await guardedJoined(() => validate(structuredClone(source), true));
    if (before && isObject(source)) registrations.set(source, { ...before, error: null });
    return false;
  } catch (error) {
    if (before && isObject(source)) registrations.set(source, { ...before, error: error instanceof Error ? error.message : "SOURCE_LOAD_REFERENCE_MALFORMED" });
    throw error;
  }
}
/** Retained joined statements without raw rows: pre-pass, retained receipt,
 * physics-source-1 transport. */
export async function validateLoadReferenceSourceTransportMetadata(source: Partial<MechanicsResult>): Promise<void> {
  await guardedJoined(() => validate(structuredClone(source) as MechanicsResult, false));
}
async function guardedJoined(action: () => Promise<void>): Promise<void> {
  try { await action(); }
  catch (error) {
    if (error instanceof LoadReferenceError) throw error;
    // The checked canonical-JSON profile refuses its own inputs by name (as
    // Python's domain_hash does); every other failure is a malformed input.
    if (error instanceof Error && error.message.startsWith("CHECKED-JSON-")) throw error;
    throw new LoadReferenceError("SOURCE_LOAD_REFERENCE_MALFORMED");
  }
}

async function validate(source: Json, raw: boolean): Promise<void> {
  // J0 the joined identity and profile. The projection below replaces both, so
  // the direct validator checks them itself.
  require_(eq(get(get(source, "producer"), "semantic_contract_id"), LOAD_REFERENCE_SOURCE_CONTRACT_ID) && eq(get(get(source, "formulation_basis"), "profile_id"), LOAD_REFERENCE_SOURCE_PROFILE), "JOIN_IDENTITY");
  // J1 joined pre-pass (S1-S13).
  prepass(source, raw, "joined");
  // J2 receipt on the received bytes.
  await receipt(source, raw);
  // J3 physics-source-1 on the projected copy.
  const projected = await project(source, raw);
  try {
    await validatePhysicsSourceTransportMetadata(projected);
    if (raw) {
      validatePhysicsSourceEvidence(projected);
      await validateSourceBlocksComposite(projected);
    }
  } catch (error) {
    throw new LoadReferenceError(`${code("JOIN_PHYSICS_SOURCE")}: ${error instanceof Error && error.message ? error.message : "PHYSICS_SOURCE_MALFORMED_VALUE"}`);
  }
}

const casePressure = (pressure: Json[], caseId: unknown) => pressure.filter(p => isObject(p) && pyEq(pget(p, "load_case_id"), caseId));
const listEq = (a: unknown[], b: unknown[]) => a.length === b.length && a.every((value, i) => pyEq(value, b[i]));

async function receipt(source: Json, raw: boolean): Promise<void> {
  const received = at(source, "source_block_recovery");
  const body = get(received, "body");
  // R1 policy, then R2 the closed physics-source-1 receipt shape with only the
  // policy constant substituted.
  require_(eq(get(body, "policy"), LOAD_REFERENCE_SOURCE_POLICY), "JOIN_RECEIPT_POLICY");
  const shaped = structuredClone(received);
  at(shaped, "body").policy = PHYSICS_SOURCE_POLICY;
  if (!schemaShape(shaped, physicsSourceSchema, physicsSourceSchema)) throw fail("JOIN_RECEIPT_SHAPE");
  // R3-R4 receipt and publication hashes.
  require_(at(received, "receipt_sha256") === await domainHash("source_blocks_receipt_v1", body), "JOIN_RECEIPT_HASH");
  if (raw) {
    const publication = Object.fromEntries(Object.entries(source).filter(([key]) => key !== "source_block_recovery"));
    require_(at(body, "publication_sha256") === await domainHash("source_blocks_publication_v1", publication), "JOIN_PUBLICATION_HASH");
  }
  // R5 one receipt case, exact case and record per case, in case order.
  const evidence = at(source, "contract_evidence");
  const cases = at(body, "cases"), exact = at(evidence, "exact_cases"), records = at(evidence, "load_reference_states"), pressure = at(evidence, "pressure");
  const exactIds = exact.map((c: Json) => at(c, "load_case_id"));
  require_(listEq(cases.map((c: Json) => at(at(c, "basis_ref"), "ref_id")), exactIds) && listEq(records.map((r: Json) => at(r, "load_case_id")), exactIds), "JOIN_CASE_ORDER");
  // R6-R7 per case: selected method, requested mode and physical-evidence hash.
  for (let i = 0; i < Math.min(cases.length, exact.length, records.length); i++) {
    const c = cases[i], exactCase = exact[i], record = records[i];
    require_(pyEq(at(c, "selected_method"), at(exactCase, "recovery_method")), "JOIN_RECOVERY_METHOD");
    require_(pyEq(at(c, "requested_mode"), at(at(record, "solve"), "requested_mode")), "JOIN_REQUESTED_MODE");
    // The producer hashes pressure [] for a selected case; hashing the case's
    // slice is equivalent only because J3 refuses a selected case with a
    // non-empty pressure inventory (physics-source-1 SOURCE_PRESSURE_INVENTORY).
    const proof = { exact_case: exactCase, pressure: casePressure(pressure, at(exactCase, "load_case_id")), load_reference_state: record };
    require_(at(c, "physical_evidence_sha256") === await domainHash(CASE_EVIDENCE_DOMAIN, proof), "JOIN_PHYSICAL_CASE_HASH");
  }
}

/** The physics-source-1 form of verified joined bytes (after steps 1-2). */
async function project(source: Json, raw: boolean): Promise<MechanicsResult> {
  const projected = projectLoadReference(source) as Json;
  at(projected, "producer").semantic_contract_id = PHYSICS_SOURCE_ID;
  at(projected, "formulation_basis").profile_id = PHYSICS_SOURCE_PROFILE;
  const evidence = at(projected, "contract_evidence");
  const body = at(at(projected, "source_block_recovery"), "body");
  body.policy = PHYSICS_SOURCE_POLICY;
  const cases = at(body, "cases"), exact = at(evidence, "exact_cases");
  for (let i = 0; i < Math.min(cases.length, exact.length); i++) {
    cases[i].physical_evidence_sha256 = await domainHash(PHYSICS_SOURCE_CASE_DOMAIN, { exact_case: exact[i], pressure: casePressure(at(evidence, "pressure"), at(exact[i], "load_case_id")) });
  }
  if (raw) body.publication_sha256 = await domainHash("source_blocks_publication_v1", Object.fromEntries(Object.entries(projected).filter(([key]) => key !== "source_block_recovery")));
  at(projected, "source_block_recovery").receipt_sha256 = await domainHash("source_blocks_receipt_v1", body);
  return projected;
}

/* ========================================================================== *
 * Invocation-free port of Python `source_blocks._validate_source_blocks(
 * source, None, context=physics_source)`. Check names are those of Python:
 * `SOURCE_BLOCKS_*` for the ledger, `PHYSICS_SOURCE_*` for the composite
 * context. `k()` is Python `mapping[key]`: a missing key is malformed.
 * ========================================================================== */
const EXACT = "retained_source_blocks_exact_v1";
const MODES: Record<string, string> = { dense_scrutiny: "ordinary_dense_structural_v1", sparse_interactive: "ordinary_sparse_structural_v1" };
const COMPONENTS = ["Fx", "Fy", "Fz", "Mx", "My", "Mz"];
const EPSILON = Number.EPSILON, FLOAT_MIN = 2 ** -1022;
const NORM_ARITHMETIC_BOUND = 64.0 * EPSILON;
const NORM_INPUT_RELATIVE_LIMIT = (1.0e-9 - NORM_ARITHMETIC_BOUND) / (1.0 + NORM_ARITHMETIC_BOUND);
const STRESS_ARITHMETIC_BOUND = 128.0 * EPSILON;
const STRESS_INPUT_RELATIVE_LIMIT = (1.0e-9 - STRESS_ARITHMETIC_BOUND) / (1.0 + STRESS_ARITHMETIC_BOUND);
const STRESS_ACTION_KINDS: Record<string, string> = {
  element_local_axial_normal_stress: "element_local_axial_force",
  element_local_bending_normal_stress_y: "element_local_bending_moment_y",
  element_local_bending_normal_stress_z: "element_local_bending_moment_z",
  element_local_torsional_shear_stress: "element_local_torsional_moment",
};
const STRESS_LOCATIONS = ["end_i", "quarter_1", "midspan", "quarter_3", "end_j"];
const COMPOSITE_RECIPES = ["retained_source_endpoint_normal_max_v1", "support_force_norm_scaled_checked_v1", "support_moment_norm_scaled_checked_v1", "retained_source_straight_stress_v1"];
const QUALITY_RANK: Record<string, number> = { checks_passed: 0, sensitive: 1, not_assessed: 2, unresolved: 3, failed: 4 };
const RAW_FIELDS = ["schema_version", "producer", "numerical_quality", "formulation_basis", "document_kind", "run_id", "model_ref", "status", "summary", "results", "diagnostics", "professional_boundary", "accepted_model_state_mutated", "source_block_recovery", "contract_evidence"];

function blocks(ok: unknown, name: string): void { if (!ok) throw new Error(`SOURCE_BLOCKS_${name}`); }
function composite(ok: unknown, name: string): void { if (!ok) throw new Error(`PHYSICS_SOURCE_${name}`); }
class Malformed extends Error { constructor() { super("PHYSICS_SOURCE_MALFORMED_VALUE"); } }
function k(value: unknown, key: string | number): Json {
  if (typeof key === "number") {
    if (!Array.isArray(value) || key < 0 || key >= value.length) throw new Malformed();
    return value[key];
  }
  if (!isObject(value) || !Object.hasOwn(value, key)) throw new Malformed();
  return value[key];
}
/** Python `dict.get(key)`. */
const pget = (value: unknown, key: string): Json => (isObject(value) && Object.hasOwn(value, key) ? value[key] : null);
/** Python `==` on JSON values (numbers by value, True == 1). */
function pyEq(a: unknown, b: unknown): boolean {
  const num = (v: unknown) => typeof v === "boolean" ? Number(v) : v;
  const x = num(a), y = num(b);
  if (typeof x === "number" && typeof y === "number") return x === y;
  if (Array.isArray(x) && Array.isArray(y)) return x.length === y.length && x.every((v, i) => pyEq(v, y[i]));
  if (isObject(x) && isObject(y)) return Object.keys(x).length === Object.keys(y).length && Object.keys(x).every(key => Object.hasOwn(y, key) && pyEq(x[key], y[key]));
  return (x ?? null) === (y ?? null);
}
/** Python `set(a) == set(b)` over hashable JSON scalars. */
const setEq = (a: unknown[], b: unknown[]) => {
  const x = new Set(a.map(v => JSON.stringify(v))), y = new Set(b.map(v => JSON.stringify(v)));
  return x.size === y.size && [...x].every(v => y.has(v));
};
function hashable(value: unknown): string {
  if (value !== null && typeof value === "object") throw new Malformed(); // Python TypeError: unhashable
  return typeof value === "number" ? `n:${value === 0 ? 0 : value}` : JSON.stringify(value);
}
function unique(items: unknown[], name: string): void {
  blocks(new Set(items.map(item => Array.isArray(item) ? JSON.stringify(item.map(hashable)) : hashable(item))).size === items.length, name);
}
const isNum = (v: unknown): v is number => typeof v === "number";
const nonEmptyText = (v: unknown) => typeof v === "string" && v.length > 0;
function bitsHex(value: unknown): string {
  blocks(isNum(value) && Number.isFinite(value), "VALUE_NONFINITE");
  const view = new DataView(new ArrayBuffer(8)); view.setFloat64(0, value as number, false);
  return view.getBigUint64(0, false).toString(16).padStart(16, "0");
}
/** Python `_signature`: exactly one composite-table signature, else null. */
function signature(row: Json): Json {
  const md = pget(row, "metadata") || {};
  const found = physicsSourceTable.rows.filter((s: Json) => s.kind === pget(row, "kind") && s.unit === pget(row, "unit")
    && (s.component === null || s.component === pget(md, "component")) && (!Object.hasOwn(s, "source_basis") || s.source_basis === pget(md, "basis")));
  return found.length === 1 ? found[0] : null;
}
const normalOrExactZero = (value: number, logicalZero: boolean) => Number.isFinite(value) && (logicalZero ? value === 0 : Math.abs(value) >= FLOAT_MIN);

function ordinary(c: Json, quality: Json, diagnostics: Map<string, Json>, evidence: Set<string>): boolean {
  const attempt = k(c, "ordinary_attempt");
  blocks(pyEq(k(attempt, "requested_mode"), k(c, "requested_mode")), "ORDINARY_MODE");
  const report = k(attempt, "structural_report_diagnostic_ref"), failure = k(attempt, "failure"), outcome = k(attempt, "outcome");
  blocks(report === null || diagnostics.has(hashable(report)), "ORDINARY_REPORT_REF");
  blocks(failure === null || diagnostics.has(hashable(k(failure, "diagnostic_ref"))), "ORDINARY_FAILURE_REF");
  if (outcome === "not_attempted") {
    blocks(report === null && failure === null && k(quality, "solve_quality") === "not_assessed", "ORDINARY_NOT_ATTEMPTED");
  } else if (outcome === "checks_passed" || outcome === "sensitive") {
    blocks(report !== null && failure === null && pyEq(k(quality, "solve_quality"), outcome) && k(quality, "evidence_refs").some((r: unknown) => pyEq(r, report)), "ORDINARY_REPORT");
    blocks(pget(diagnostics.get(hashable(report)), "code") === (outcome === "checks_passed" ? "NUMERICAL_INTEGRITY_CHECKS_PASSED" : "NUMERICAL_INTEGRITY_SENSITIVE"), "ORDINARY_REPORT_KIND");
  } else {
    blocks(failure !== null && ["unresolved", "failed"].includes(k(quality, "solve_quality")) && k(quality, "evidence_refs").some((r: unknown) => pyEq(r, k(failure, "diagnostic_ref"))), "ORDINARY_REJECTION");
  }
  const refs = k(quality, "evidence_refs");
  return k(quality, "solve_quality") === "checks_passed" && k(quality, "structural_status") === "passive_model_basis" && k(quality, "model_matrix_fidelity") === "represented_equations_retained"
    && ["not_claimed", "reference_verified"].includes(k(quality, "accuracy_evidence")) && refs.length > 0 && refs.every((ref: unknown) => typeof ref === "string" && evidence.has(ref));
}

function sourcePlan(plan: Json, projections: Json[]): void {
  const n = k(plan, "dof_count");
  blocks(0 < n && n <= 256 && k(plan, "stiffness_term_count") > 0 && k(plan, "stiffness_term_count") + k(plan, "force_term_count") <= 16384, "SOURCE_COUNTS");
  blocks(projections.length <= k(plan, "functional_count") && k(plan, "functional_count") <= 16384, "FUNCTIONAL_COUNT");
  const numeric = (a: number, b: number) => a - b;
  const partition = [...k(plan, "free_dofs"), ...k(plan, "prescribed_dofs")].sort(numeric);
  blocks(pyEq(partition, Array.from({ length: Math.max(0, Math.floor(n)) }, (_, i) => i)) && Number.isInteger(n), "SOURCE_PARTITION");
  const flattened = k(plan, "free_blocks").flatMap((block: Json[]) => [...block]).sort(numeric);
  blocks(pyEq(flattened, [...k(plan, "free_dofs")].sort(numeric)), "BLOCK_PARTITION");
  unique(k(plan, "member_ids"), "MEMBER_IDENTITY");
  unique(k(plan, "support_ids"), "SUPPORT_IDENTITY");
  blocks(k(plan, "member_ids").length > 0, "MEMBER_COVERAGE");
}

function projectionCheck(projection: Json, row: Json): void {
  const value = k(projection, "value"), interval = k(projection, "interval");
  if (!Array.isArray(interval) || interval.length !== 2) throw new Malformed();
  const [lo, hi] = interval;
  blocks(k(projection, "value_bits") === bitsHex(value) && bitsHex(value) === bitsHex(k(row, "value")) && pyEq(k(projection, "unit"), k(row, "unit")), "PROJECTION_VALUE_BINDING");
  const sign = value > 0 ? 1 : -1;
  blocks(lo <= value && value <= hi && (value === 0 || (lo * sign > 0 && hi * sign > 0)), "PROJECTION_INTERVAL");
  const basis = k(projection, "basis");
  if (basis === "exact_zero" || basis === "exact_identity") {
    blocks(lo === hi && hi === value && k(projection, "absolute_error_bound") === k(projection, "relative_error_bound") && k(projection, "relative_error_bound") === 0 && (basis === "exact_zero" ? value === 0 : value !== 0), "PROJECTION_EXACT_BASIS");
  } else {
    blocks(value !== 0 && k(projection, "absolute_error_bound") >= Math.max(Math.abs(value - lo), Math.abs(hi - value)), "PROJECTION_ENCLOSURE");
    blocks(k(projection, "relative_error_bound") >= k(projection, "absolute_error_bound") / Math.min(Math.abs(lo), Math.abs(hi)), "PROJECTION_RELATIVE_BOUND");
  }
  blocks(k(projection, "relative_limit") === 1e-9 && k(projection, "relative_error_bound") <= 1e-9, "PROJECTION_CRITERION");
  const md = pget(row, "metadata") || {};
  const quantity = k(projection, "quantity"), kind = k(row, "kind"), unit = k(row, "unit");
  const valid: Record<string, boolean> = {
    nodal_translation: ["global_nodal_displacement_x", "global_nodal_displacement_y", "global_nodal_displacement_z"].includes(kind) && unit === "mm" && pget(md, "coordinate_system") === "global" && pget(md, "location") === "node",
    nodal_rotation: ["global_nodal_rotation_x", "global_nodal_rotation_y", "global_nodal_rotation_z"].includes(kind) && unit === "rad" && pget(md, "coordinate_system") === "global" && pget(md, "location") === "node",
    member_end_action: typeof kind === "string" && kind.startsWith("element_local_") && ["N", "N*m"].includes(unit) && ["end_i", "end_j"].includes(pget(md, "location")) && pget(md, "coordinate_system") === "element_local",
    member_station_action: typeof kind === "string" && kind.startsWith("element_local_") && ["N", "N*m"].includes(unit) && ["quarter_1", "midspan", "quarter_3"].includes(pget(md, "location")) && pget(md, "coordinate_system") === "element_local",
    support_action_component: kind === "support_reaction_component_v2" && COMPONENTS.includes(pget(md, "component")),
    constraint_reaction: false,
  };
  if (typeof quantity !== "string" || !Object.hasOwn(valid, quantity)) throw new Malformed(); // Python KeyError
  blocks(valid[quantity] && signature(row) !== null, "PROJECTION_TYPED_QUANTITY");
}

function derived(recipe: unknown, row: Json, inputs: Json[]): void {
  const kind = k(row, "kind");
  const sig = signature(row);
  blocks(sig !== null, "DERIVED_SIGNATURE");
  if (recipe === "translation_norm_scaled_v1") {
    blocks(kind === "displacement_magnitude" && inputs.length === 3 && pyEq(inputs.map(r => k(r, "kind")), ["global_nodal_displacement_x", "global_nodal_displacement_y", "global_nodal_displacement_z"]), "TRANSLATION_NORM_INPUTS");
  } else if (recipe === "support_force_norm_scaled_v1") {
    blocks(kind === "reaction_resultant" && inputs.length === 3 && inputs.every(r => k(r, "kind") === "support_reaction_component_v2") && pyEq(inputs.map(r => k(k(r, "metadata"), "component")), ["Fx", "Fy", "Fz"]), "SUPPORT_NORM_INPUTS");
  } else if (recipe === "straight_open_stress_v1") {
    blocks(typeof kind === "string" && Object.hasOwn(STRESS_ACTION_KINDS, kind) && inputs.length === 1 && k(inputs[0], "kind") === STRESS_ACTION_KINDS[kind], "STRESS_RECIPE_INPUTS");
  } else if (recipe === "reviewed_stress_summary_v1") {
    blocks(kind === "open_formula_stress_summary" && inputs.length > 0, "SUMMARY_RECIPE_INPUTS");
  } else if (recipe === "section_property_from_source_v1") {
    blocks(k(sig, "family") === "section_property" || (typeof kind === "string" && kind.startsWith("pipe_section_")), "SECTION_RECIPE_KIND");
  } else {
    blocks(false, "DERIVED_RECIPE");
  }
  blocks(inputs.every(r => pyEq(pget(r, "entity_ref"), pget(row, "entity_ref"))), "DERIVED_ENTITY");
  if (recipe === "straight_open_stress_v1") stressObservation(row, inputs[0]);
  else if (recipe === "reviewed_stress_summary_v1") stressSummaryObservation(row, inputs);
  if (recipe === "translation_norm_scaled_v1" || recipe === "support_force_norm_scaled_v1") {
    // Ordered max-scaled binary64 computation; one normalized component is one.
    const values = inputs.map(item => { const v = k(item, "value"); if (!isNum(v)) throw new Malformed(); return v; });
    blocks(values.every(Number.isFinite), "DERIVED_NORM_RANGE");
    const scale = Math.max(...values.map(Math.abs));
    let expected: number;
    if (scale === 0) expected = 0;
    else {
      const normalized = values.map(v => v / scale), squares = normalized.map(v => v * v);
      const firstTwo = squares[0] + squares[1], squared = firstTwo + squares[2], root = Math.sqrt(squared);
      expected = scale * root;
      blocks([...normalized, ...squares, firstTwo, squared, root, expected].every(Number.isFinite) && expected >= FLOAT_MIN, "DERIVED_NORM_RANGE");
    }
    blocks(bitsHex(k(row, "value")) === bitsHex(expected), "DERIVED_NORM_VALUE");
  }
}
function stressObservation(row: Json, action: Json): void {
  const location = pget(pget(row, "metadata") || {}, "location");
  blocks(k(row, "unit") === "MPa" && STRESS_LOCATIONS.includes(location) && pget(pget(action, "metadata") || {}, "location") === location
    && k(action, "unit") === (k(row, "kind") === "element_local_axial_normal_stress" ? "N" : "N*m"), "STRESS_ACTION_BINDING");
  const sourceAction = k(action, "value"), value = k(row, "value");
  if (!isNum(sourceAction) || !isNum(value)) throw new Malformed();
  blocks(Number.isFinite(sourceAction), "STRESS_ACTION_RANGE");
  const logicalZero = sourceAction === 0;
  blocks(normalOrExactZero(value, logicalZero), "STRESS_OUTPUT_RANGE");
  const signedAction = location === "end_i" ? -sourceAction : sourceAction;
  blocks(logicalZero || (value > 0) === (signedAction > 0), "STRESS_ACTION_SIGN");
  blocks(normalOrExactZero(value * 1_000_000.0, logicalZero), "STRESS_PA_OBSERVATION_RANGE");
}
function stressSummaryObservation(row: Json, inputs: Json[]): void {
  blocks(k(row, "unit") === "MPa" && inputs.length === 20, "SUMMARY_STRESS_COVERAGE");
  const grouped = new Map<string, number>();
  for (const item of inputs) {
    const location = pget(pget(item, "metadata") || {}, "location");
    const key = JSON.stringify([location, k(item, "kind")]);
    blocks(STRESS_LOCATIONS.includes(location) && Object.hasOwn(STRESS_ACTION_KINDS, k(item, "kind")) && k(item, "unit") === "MPa" && !grouped.has(key), "SUMMARY_STRESS_COVERAGE");
    const value = k(item, "value"); if (!isNum(value)) throw new Malformed();
    blocks(normalOrExactZero(value, value === 0), "SUMMARY_INPUT_RANGE");
    const observedPa = value * 1_000_000.0;
    blocks(normalOrExactZero(observedPa, value === 0), "SUMMARY_PA_OBSERVATION_RANGE");
    grouped.set(key, observedPa);
  }
  let anyNormal = false;
  for (const location of STRESS_LOCATIONS) {
    const value = (kind: string) => { const v = grouped.get(JSON.stringify([location, kind])); if (v === undefined) throw new Malformed(); return v; };
    const axial = value("element_local_axial_normal_stress"), by = value("element_local_bending_normal_stress_y"), bz = value("element_local_bending_normal_stress_z");
    const logicalZero = axial === 0 && by === 0 && bz === 0;
    anyNormal ||= !logicalZero;
    const baseNormal = axial + 0.0, bendingTotal = Math.abs(by) + Math.abs(bz);
    const plus = baseNormal + bendingTotal, minus = baseNormal - bendingTotal;
    blocks([baseNormal, bendingTotal, plus, minus].every(Number.isFinite), "SUMMARY_SUBTOTAL_RANGE");
    const subtotal = Math.max(Math.abs(plus), Math.abs(minus));
    blocks(normalOrExactZero(subtotal, logicalZero) && normalOrExactZero(subtotal / 1_000_000.0, logicalZero), "SUMMARY_SUBTOTAL_RANGE");
  }
  const value = k(row, "value"); if (!isNum(value)) throw new Malformed();
  blocks(value >= 0 && normalOrExactZero(value, !anyNormal), "SUMMARY_OUTPUT_RANGE");
}

function supports(c: Json, raw: Map<string, Json>, projections: Map<string, Json>): void {
  const records = k(c, "supports"), plan = k(c, "source");
  blocks(pyEq(records.map((s: Json) => k(s, "support_id")), k(plan, "support_ids")), "SUPPORT_COVERAGE");
  const ideals = new Set<number>(), springSources = new Set<string>();
  const projectionByResult = new Map<string, Json>([...projections.values()].map(p => [hashable(k(p, "result_id")), p]));
  for (const support of records) {
    blocks(setEq(k(support, "components").map((x: Json) => k(x, "component")), COMPONENTS), "SIX_SUPPORT_COMPONENTS");
    let inferredNode: number | null = null;
    for (const component of k(support, "components")) {
      const slot = COMPONENTS.indexOf(k(component, "component"));
      const row = raw.get(hashable(k(component, "result_id"))), projection = projectionByResult.get(hashable(k(component, "result_id")));
      blocks(row !== undefined && projection !== undefined && pyEq(k(projection, "functional_id"), k(component, "functional_id")) && k(projection, "quantity") === "support_action_component", "SUPPORT_PROJECTION");
      const md = pget(row, "metadata") || {};
      blocks(k(row, "kind") === "support_reaction_component_v2" && pyEq(k(row, "entity_ref"), k(support, "support_id")) && pyEq(k(row, "basis_ref"), k(c, "basis_ref"))
        && k(row, "unit") === (slot < 3 ? "N" : "N*m")
        && pyEq(md, { component: k(component, "component"), coordinate_system: "global", location: "node", basis: "recovered_from_assembled_support_law", sign_convention: PHYSICS_SOURCE_SUPPORT_SIGN }), "SUPPORT_ROW_SEMANTICS");
      const terms = k(component, "action_terms");
      blocks(terms.length > 0, "SUPPORT_ACTION_TERMS");
      unique(terms.map((t: Json) => [k(t, "kind"), k(t, "source_id"), k(t, "global_dof")]), "SUPPORT_TERM_DUPLICATE");
      for (const term of terms) {
        const dof = k(term, "global_dof");
        blocks(dof < k(plan, "dof_count") && ((dof % 6) + 6) % 6 === slot && pyEq(k(term, "source_id"), k(support, "support_id")), "SUPPORT_ACTION_OWNER");
        inferredNode = inferredNode === null ? Math.floor(dof / 6) : inferredNode;
        blocks(Math.floor(dof / 6) === inferredNode, "SUPPORT_ACTION_DOF");
        if (k(term, "kind") === "ideal_constraint") {
          blocks(k(plan, "prescribed_dofs").some((d: unknown) => pyEq(d, dof)) && !ideals.has(dof), "IDEAL_ATTRIBUTION_AMBIGUOUS"); ideals.add(dof);
        } else if (k(term, "kind") === "ground_spring") {
          const key = JSON.stringify([k(term, "source_id"), dof]); blocks(!springSources.has(key), "SPRING_ATTRIBUTION_DUPLICATE"); springSources.add(key);
        } else {
          blocks(terms.length === 1 && k(row, "value") === 0 && k(projection, "basis") === "exact_zero", "STRUCTURAL_ZERO");
        }
      }
    }
  }
  blocks(setEq([...ideals], k(plan, "prescribed_dofs")), "IDEAL_SOURCE_COVERAGE");
  blocks(setEq(records.flatMap((s: Json) => k(s, "components").map((x: Json) => k(x, "result_id"))), [...projections.values()].filter(p => k(p, "quantity") === "support_action_component").map(p => k(p, "result_id"))), "SUPPORT_RESULT_BIJECTION");
}

function requiredAffineRows(c: Json, raw: Map<string, Json>): void {
  const lookup = (id: unknown) => { const row = raw.get(hashable(id)); if (row === undefined) throw new Malformed(); return row; };
  const projected = k(c, "projections").map((p: Json) => lookup(k(p, "result_id")));
  const caseRows = k(c, "rows").map((r: Json) => lookup(k(r, "result_id")));
  const components = ["axial_force", "shear_force_y", "shear_force_z", "torsional_moment", "bending_moment_y", "bending_moment_z"];
  const locations = ["end_i", "end_j", "quarter_1", "midspan", "quarter_3"];
  const count = (rows: Json[], test: (r: Json) => boolean) => rows.filter(test).length;
  for (const member of k(k(c, "source"), "member_ids")) {
    for (const location of locations) for (const component of components) {
      blocks(count(projected, r => pyEq(k(r, "entity_ref"), member) && pget(pget(r, "metadata") || {}, "component") === component && pget(k(r, "metadata"), "location") === location) === 1, "MEMBER_PRIMARY_COVERAGE");
    }
    for (const location of locations) for (const kind of Object.keys(STRESS_ACTION_KINDS)) {
      blocks(count(caseRows, r => pyEq(k(r, "entity_ref"), member) && k(r, "kind") === kind && pget(pget(r, "metadata") || {}, "location") === location) === 1, "MEMBER_STRESS_COVERAGE");
    }
    blocks(count(caseRows, r => pyEq(k(r, "entity_ref"), member) && k(r, "kind") === "pipe_elastic_normal_stress_maximum_v2") === 1, "MEMBER_SUMMARY_COVERAGE");
  }
  for (const support of k(k(c, "source"), "support_ids")) {
    blocks(count(caseRows, r => pyEq(k(r, "entity_ref"), support) && k(r, "kind") === "support_reaction_force_magnitude_v2") === 1, "SUPPORT_MAGNITUDE_COVERAGE");
  }
}

/** Python `physics_source.validate_source_case(source, case, None)`. Its
 * `_section_functionals(case)` call repeats, on the same receipt case, the
 * identical check `validatePhysicsSourceTransportMetadata` already passed. */
function validateSourceCase(source: Json, c: Json): void {
  composite(["sensitive", "rejected"].includes(k(k(c, "ordinary_attempt"), "outcome")), "SOURCE_FALLBACK_TRIGGER");
  const physicalCase = k(k(source, "contract_evidence"), "exact_cases").find((x: Json) => pyEq(k(x, "load_case_id"), k(k(c, "basis_ref"), "ref_id")));
  if (physicalCase === undefined) throw new Malformed(); // Python StopIteration
  const rhs = k(physicalCase, "pressure_rhs_assembly");
  composite(!k(rhs, "groups").length && ["assembled_pressure_rhs_global", "rounded_cap_rhs_global", "rounded_poisson_rhs_global"].every(key => k(rhs, key).every((v: unknown) => pyEq(v, 0))), "SOURCE_PRESSURE_RHS");
  const checks = k(c, "derived_checks");
  const expected = k(c, "rows").filter((r: Json) => ["support_force_norm_scaled_checked_v1", "support_moment_norm_scaled_checked_v1"].includes(k(r, "recipe_id"))).map((r: Json) => k(r, "result_id"));
  composite(checks.length === new Set(expected.map(hashable)).size && setEq(checks.map((x: Json) => k(x, "result_id")), expected) && checks.length === 2 * k(k(c, "source"), "support_ids").length, "DERIVED_CHECK_COVERAGE");
  const stresses = k(c, "section_stress_checks");
  const expectedStresses = k(c, "rows").filter((r: Json) => k(r, "recipe_id") === "retained_source_straight_stress_v1").map((r: Json) => k(r, "result_id"));
  const expectedSize = new Set(expectedStresses.map(hashable)).size;
  composite(stresses.length === expectedSize && expectedSize === 20 * k(k(c, "source"), "member_ids").length && setEq(stresses.map((x: Json) => k(x, "result_id")), expectedStresses), "STRESS_CHECK_COVERAGE");
}

function summary(source: Json, raw: Map<string, Json>): void {
  const summaryValue = k(source, "summary");
  for (const [field, kind] of [["max_displacement", "displacement_magnitude"], ["max_open_formula_stress", "pipe_elastic_normal_stress_maximum_v2"]]) {
    const headline = pget(summaryValue, field);
    const candidates = [...raw.values()].filter(r => k(r, "kind") === kind);
    if (candidates.length) {
      blocks(isObject(headline) && setEq(Object.keys(headline), ["value", "unit", "location_ref", "result_ref"]) && Object.keys(headline).length === 4, "SUMMARY_HEADLINE");
      const selected = raw.get(hashable(k(headline, "result_ref")));
      const maximum = Math.max(...candidates.map(r => { const v = k(r, "value"); if (!isNum(v)) throw new Malformed(); return v; }));
      blocks(selected !== undefined && k(selected, "kind") === kind && pyEq(k(selected, "unit"), k(headline, "unit")) && pyEq(k(selected, "entity_ref"), k(headline, "location_ref"))
        && bitsHex(k(selected, "value")) === bitsHex(k(headline, "value")) && k(headline, "value") === maximum, "SUMMARY_RESULT_BINDING");
    } else {
      blocks(headline === null, "SUMMARY_UNSOURCED_HEADLINE");
    }
  }
}

async function validateSourceBlocksComposite(source: Json): Promise<boolean> {
  blocks(isObject(source) && setEq(Object.keys(source), RAW_FIELDS) && Object.keys(source).length === RAW_FIELDS.length, "RAW_FIELDS");
  blocks(k(source, "schema_version") === "0.2.0" && pyEq(k(source, "producer"), { component_name: "open_pipe_stress_product_physics", component_version: "0.2.0", semantic_contract_id: PHYSICS_SOURCE_ID }), "PRODUCER");
  const formulation = k(source, "formulation_basis");
  blocks(isObject(formulation) && setEq(Object.keys(formulation), ["profile_id", "limitations"]) && Object.keys(formulation).length === 2 && formulation.profile_id === PHYSICS_SOURCE_PROFILE
    && Array.isArray(formulation.limitations) && formulation.limitations.length > 0 && formulation.limitations.every(nonEmptyText), "FORMULATION");
  const received = k(source, "source_block_recovery");
  composite(schemaShape(received, physicsSourceSchema, physicsSourceSchema), "RECEIPT_SHAPE");
  const body = k(received, "body");
  blocks(k(received, "receipt_sha256") === await domainHash("source_blocks_receipt_v1", body), "RECEIPT_HASH");
  const publication = Object.fromEntries(Object.entries(source).filter(([key]) => key !== "source_block_recovery"));
  blocks(k(body, "publication_sha256") === await domainHash("source_blocks_publication_v1", publication), "PUBLICATION_HASH");
  const results = k(source, "results"), diagnosticsList = k(source, "diagnostics");
  blocks(Array.isArray(results) && Array.isArray(diagnosticsList) && results.length <= 16384 && diagnosticsList.length <= 16384, "ROWS");
  const allIds = [...results, ...diagnosticsList].filter(isObject).map(r => pget(r, "id"));
  blocks(allIds.length === results.length + diagnosticsList.length && allIds.every(nonEmptyText), "EVIDENCE_IDS");
  unique(allIds, "EVIDENCE_IDS");
  const raw = new Map<string, Json>(results.map((r: Json) => [hashable(k(r, "id")), r]));
  const diagnostics = new Map<string, Json>(diagnosticsList.map((r: Json) => [hashable(k(r, "id")), r]));
  const evidence = new Set<string>(allIds);
  for (const row of raw.values()) {
    bitsHex(pget(row, "value"));
    blocks(["kind", "unit", "entity_ref"].every(key => nonEmptyText(pget(row, key))), "ROW_FIELDS");
    const md = pget(row, "metadata");
    if (md !== null) blocks(isObject(md) && setEq(Object.keys(md), ["component", "coordinate_system", "location", "basis", "sign_convention"]) && Object.keys(md).length === 5 && Object.values(md).every(nonEmptyText), "ROW_METADATA");
  }
  const cases = k(body, "cases");
  unique(cases.map((c: Json) => k(k(c, "basis_ref"), "ref_id")), "CASE_IDS");
  const q = k(source, "numerical_quality");
  blocks(isObject(q) && setEq(Object.keys(q), ["value_representation", "publication_quantization", "integrity_policy", "status", "cases"]) && Object.keys(q).length === 5
    && q.value_representation === "finite_binary64" && q.publication_quantization === "none" && q.integrity_policy === "M03-INTEGRITY-v1" && Array.isArray(q.cases) && q.cases.length === cases.length, "ORDINARY_QUALITY");
  for (const quality of q.cases) {
    blocks(isObject(quality) && setEq(Object.keys(quality), ["basis_ref", "structural_status", "solve_quality", "model_matrix_fidelity", "accuracy_evidence", "evidence_refs"]) && Object.keys(quality).length === 6
      && typeof quality.solve_quality === "string" && Object.hasOwn(QUALITY_RANK, quality.solve_quality)
      && ["passive_model_basis", "physical_mechanism_witnessed", "negative_energy_witnessed", "numerically_unresolved"].includes(quality.structural_status)
      && ["represented_equations_retained", "assembly_loss_detected", "assembly_uncertainty", "not_assessed"].includes(quality.model_matrix_fidelity)
      && ["not_claimed", "reference_verified", "unresolved"].includes(quality.accuracy_evidence)
      && Array.isArray(quality.evidence_refs) && quality.evidence_refs.every((r: unknown) => typeof r === "string" && evidence.has(r)), "ORDINARY_CASE");
  }
  const aggregateQuality = q.cases.reduce((worst: string, c: Json) => QUALITY_RANK[c.solve_quality] > QUALITY_RANK[worst] ? c.solve_quality : worst, q.cases.length ? q.cases[0].solve_quality : "not_assessed");
  blocks(q.status === aggregateQuality, "ORDINARY_AGGREGATE");
  const covered = new Set<string>();
  let qualified = 0, invocationCharged = 0;
  for (const [index, c] of cases.entries()) {
    const basis = k(c, "basis_ref");
    blocks(pyEq(k(k(c, "ordinary_attempt"), "quality_case_index"), index) && pyEq(k(q.cases[index], "basis_ref"), basis), "QUALITY_CASE_BINDING");
    // REQUESTED_MODE is checked only against an actual invocation (absent here).
    const ordinaryOk = ordinary(c, q.cases[index], diagnostics, evidence);
    const work = k(c, "work"), rejected = k(work, "rejected_reservation");
    invocationCharged += k(work, "charged") + k(work, "reserved_unobserved_failure");
    blocks(invocationCharged <= 64_000_000, "INVOCATION_WORK_LIMIT");
    blocks(k(work, "charged") + k(work, "reserved_unobserved_failure") <= k(work, "limit") && k(work, "limit") <= 8_000_000
      && ((k(rejected, "kind") === "overflow" && k(rejected, "amount") === null) || (k(rejected, "kind") === "finite" && k(rejected, "amount") !== null)), "WORK_LEDGER");
    const success = k(c, "outcome") === "qualified", method = k(c, "selected_method");
    if (success) {
      qualified += 1;
      blocks(k(c, "failure") === null && method !== null && pyEq(rejected, { kind: "finite", amount: 0 }) && pyEq(k(work, "reserved_unobserved_failure"), 0), "QUALIFIED_OUTCOME");
      if (method !== EXACT) {
        const mode = k(c, "requested_mode");
        if (typeof mode !== "string" || !Object.hasOwn(MODES, mode)) throw new Malformed(); // Python KeyError
        blocks(method === MODES[mode] && ordinaryOk && k(c, "source") === null && !k(c, "projections").length && !k(c, "supports").length, "ORDINARY_SELECTION");
      } else {
        blocks(k(c, "source") !== null, "EXACT_SOURCE_REQUIRED");
      }
    } else {
      const failure = k(c, "failure");
      blocks(method === null && failure !== null && diagnostics.has(hashable(k(failure, "diagnostic_ref"))), "FAILED_OUTCOME");
      blocks((k(failure, "code") === "unsupported_block" && k(failure, "block_order") !== null && k(failure, "block_order") > 2 && k(c, "source") === null)
        || (k(failure, "code") !== "unsupported_block" && k(failure, "block_order") === null), "FAILURE_BLOCK_ORDER");
      blocks((k(c, "outcome") === "unsupported") === ["unsupported_family", "unsupported_block", "unsupported_source_closure", "unsupported_derived_quantity", "support_attribution_ambiguous"].includes(k(failure, "code")), "FAILURE_CATEGORY");
    }
    if (k(c, "source") !== null) {
      sourcePlan(k(c, "source"), k(c, "projections"));
      unique([k(k(c, "source"), "normalized_source_sha256"), k(k(c, "source"), "functional_plan_sha256"), k(k(body, "invocation"), "value"), k(body, "publication_sha256"), k(received, "receipt_sha256")], "COMMITMENT_DOMAIN_SEPARATION");
    }
    const projectionList = k(c, "projections");
    const projections = new Map<string, Json>(projectionList.map((p: Json) => [hashable(k(p, "projection_id")), p]));
    unique(projectionList.map((p: Json) => k(p, "projection_id")), "PROJECTION_IDS");
    unique(projectionList.map((p: Json) => k(p, "functional_id")), "FUNCTIONAL_IDS");
    unique(projectionList.map((p: Json) => k(p, "result_id")), "PROJECTION_RESULTS");
    const rowList = k(c, "rows");
    const treatment = new Map<string, Json>(rowList.map((r: Json) => [hashable(k(r, "result_id")), r]));
    unique(rowList.map((r: Json) => k(r, "result_id")), "ROW_IDS");
    if (rowList.some((r: Json) => ["straight_open_stress_v1", "reviewed_stress_summary_v1", "retained_source_straight_stress_v1"].includes(k(r, "recipe_id")))) {
      blocks(projectionList.length > 0 && Math.max(...projectionList.map((p: Json) => k(p, "relative_error_bound"))) <= STRESS_INPUT_RELATIVE_LIMIT, "STRESS_TOTAL_RELATIVE_BOUND");
    }
    const expected = [...raw.entries()].filter(([, r]) => pyEq(pget(r, "basis_ref"), basis)).map(([id]) => id);
    blocks(setEq([...treatment.keys()], expected) && expected.every(id => !covered.has(id)), "CASE_ROW_COVERAGE");
    expected.forEach(id => covered.add(id));
    const usedProjections = new Set<string>();
    for (const [resultId, treatmentRow] of treatment) {
      const row = raw.get(resultId); if (row === undefined) throw new Malformed();
      const sig = signature(row);
      const inputs = k(treatmentRow, "input_result_ids");
      unique(inputs, "ROW_INPUTS");
      blocks(inputs.every((i: unknown) => treatment.has(hashable(i)) && hashable(i) !== resultId), "SAME_CASE_INPUTS");
      const inputRows = () => inputs.map((i: unknown) => { const r = raw.get(hashable(i)); if (r === undefined) throw new Malformed(); return r; });
      const token = k(treatmentRow, "treatment");
      if (token === "qualified_projection") {
        const projection = projections.get(hashable(k(treatmentRow, "projection_id")));
        blocks(method === EXACT && success && projection !== undefined && hashable(k(projection, "result_id")) === resultId && k(treatmentRow, "recipe_id") === null && !inputs.length, "PROJECTION_ROW");
        projectionCheck(projection, row); usedProjections.add(hashable(k(projection, "projection_id")));
      } else if (token === "checked_derived") {
        blocks(method === EXACT && success && k(treatmentRow, "projection_id") === null && k(treatmentRow, "recipe_id") !== null, "DERIVED_ROW");
        const recipe = k(treatmentRow, "recipe_id");
        if (COMPOSITE_RECIPES.includes(recipe)) validatePhysicsSourceDerived(source, c, treatmentRow, row, inputRows());
        else derived(recipe, row, inputRows());
        if (recipe === "translation_norm_scaled_v1" || recipe === "support_force_norm_scaled_v1") {
          blocks(inputs.every((i: unknown) => k(treatment.get(hashable(i)), "treatment") === "qualified_projection"), "NORM_PROJECTION_INPUT");
          const relative = Math.max(...inputs.map((i: unknown) => {
            const p = projections.get(hashable(k(treatment.get(hashable(i)), "projection_id"))); if (p === undefined) throw new Malformed();
            return k(p, "relative_error_bound");
          }));
          blocks(relative <= NORM_INPUT_RELATIVE_LIMIT, "NORM_TOTAL_RELATIVE_BOUND");
        }
        blocks(inputs.every((i: unknown) => ["qualified_projection", "checked_derived"].includes(k(treatment.get(hashable(i)), "treatment"))), "UNQUALIFIED_DERIVED_INPUT");
      } else if (token === "ordinary_physics_checked") {
        blocks(success && method !== EXACT && k(treatmentRow, "projection_id") === null && k(treatmentRow, "recipe_id") === null && sig !== null, "ORDINARY_ROW");
      } else {
        blocks(k(treatmentRow, "projection_id") === null && k(treatmentRow, "recipe_id") === null, "INSPECTION_ROW");
        // (usable only affects eligibility, which needs an invocation.)
      }
    }
    blocks(setEq([...usedProjections], [...projections.keys()]), "PROJECTION_BIJECTION");
    // Dependency cycles cannot serve as evidence for checked derived rows.
    const pending = new Set(treatment.keys()), done = new Set<string>();
    while (pending.size) {
      const ready = [...pending].filter(id => k(treatment.get(id), "input_result_ids").every((i: unknown) => done.has(hashable(i))));
      blocks(ready.length > 0, "ROW_DEPENDENCY_CYCLE");
      ready.forEach(id => { done.add(id); pending.delete(id); });
    }
    if (success && method === EXACT) {
      supports(c, raw, projections);
      requiredAffineRows(c, raw);
      validateSourceCase(source, c);
    } else if (k(c, "supports").length) {
      blocks(false, "UNSELECTED_SUPPORT_CERTIFICATE");
    }
  }
  const invocationWork = k(body, "invocation_work");
  blocks(pyEq(k(invocationWork, "charged"), invocationCharged + k(invocationWork, "publication_charged")) && k(invocationWork, "charged") <= k(invocationWork, "limit") && k(invocationWork, "limit") <= 64_000_000, "INVOCATION_WORK_LEDGER");
  const observations = k(body, "envelope_observation_result_ids");
  unique(observations, "OBSERVATION_IDS");
  const observationKeys = observations.map(hashable);
  blocks(observationKeys.every((id: string) => !covered.has(id)) && setEq([...covered, ...observationKeys], [...raw.keys()]), "ENVELOPE_ROW_COVERAGE");
  for (const identity of observationKeys) {
    const row = raw.get(identity); if (row === undefined) throw new Malformed();
    const sig = signature(row);
    blocks(pget(row, "basis_ref") === null && sig !== null && sig.category !== "physical_quantity", "PHYSICAL_OBSERVATION_ESCAPE");
  }
  blocks(!cases.length || cases.some((c: Json) => k(c, "selected_method") === EXACT || k(c, "outcome") !== "qualified"), "SOURCE_METHOD_RECORD_REQUIRED");
  const aggregate = cases.length && qualified === cases.length ? "qualified" : qualified ? "partial" : "unavailable";
  blocks(k(body, "status") === aggregate, "AGGREGATE_STATUS");
  summary(source, raw);
  return false; // No actual invocation: never numerically eligible.
}

/** Test seam only: the physics-source-1 projection of verified joined bytes and
 * the invocation-free ledger port, so that each ported check can be reached
 * directly (loadReferenceSourceLedger.test.ts). Production callers use
 * `validateLoadReferenceSourceEvidence`. */
export const loadReferenceSourceLedgerTestSeam = Object.freeze({
  project: (source: MechanicsResult) => project(structuredClone(source), true),
  validateLedger: (projected: MechanicsResult) => validateSourceBlocksComposite(projected),
});
