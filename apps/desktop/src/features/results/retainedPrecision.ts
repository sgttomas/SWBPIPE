import receiptSchema from '../../../../../schemas/retained_precision_mp_v2.schema.json';
import definition from '../../../../../fixtures/results/retained_precision_prepared_ordinary_v1.json';
import table from '../../../../../fixtures/results/semantic_contract_v0_3_preview_physics_retained_1.json';
import tableBytes from '../../../../../fixtures/results/semantic_contract_v0_3_preview_physics_retained_1.json?raw';
import inheritedTableBytes from '../../../../../fixtures/results/semantic_contract_v0_3_preview_physics_1.json?raw';
import { canonicalSha256HexCheckedV1, checkedJsonText } from '../../services/hashService';
import { loadWasmEngine } from '../../services/wasmEngine/loadWasmEngine';
import { validatePreviewPhysicsEvidence, validatePreviewPhysicsTransportMetadata, compareCodePoints } from './previewPhysicsEvidence';
import { sourceContract } from './numericalResultQuality';
import type { MechanicsResult } from '../../types';

/** Standalone statement validation. No native registration or carrier mutation. */
export const RETAINED_PRECISION_ID = 'openpipestress.result_semantics/0.3.0/preview-physics-retained-1';
export const RETAINED_PRECISION_PROFILE = 'product_preview_retained_w1a_v2';
export const PREPARED_DEFINITION_ID = 'RP-PREPARED-ORDINARY-DUAL-v1';
export const PREPARED_DEFINITION_HASH = 'a7ed7ca0bf0bba6e8b821ca4befa00a0fa9541a83694be8b28ac63e39b1d0349';
export const RETAINED_METHOD = 'contribution_preserving_multiprecision_v1';
const MAX_BITS = 0x7fefffffffffffffn;

export class RetainedPrecisionError extends Error {
  constructor(readonly gate: string, readonly code: string, readonly detail: string | null = null) { super(code); }
}
/** G7 ruling: the bare base code is the error code; any validator detail is carried separately. */
function baseError(gate: string, error: unknown, fallback: string): RetainedPrecisionError {
  const message = error instanceof Error ? error.message : '';
  const match = /^([A-Z][A-Z0-9_]*)(?::\s*([\s\S]*))?$/.exec(message);
  return match ? new RetainedPrecisionError(gate, match[1], match[2] ?? null) : new RetainedPrecisionError(gate, fallback, message || null);
}
export function binary64Bits(value: number): string {
  const view = new DataView(new ArrayBuffer(8)); view.setFloat64(0, value, false);
  return view.getBigUint64(0, false).toString(16).padStart(16, '0');
}
function numberFromWord(word: bigint): number {
  const view = new DataView(new ArrayBuffer(8)); view.setBigUint64(0, word, false); return view.getFloat64(0, false);
}
export function decodeBinary64(value: string): number {
  if (typeof value !== 'string' || !/^[0-9a-f]{16}$/.test(value)) throw new Error('binary64 encoding');
  const result = numberFromWord(BigInt(`0x${value}`));
  if (!Number.isFinite(result)) throw new Error('nonfinite binary64');
  return result;
}
function positiveParts(value: number): [bigint, number] {
  if (!Number.isFinite(value) || value < 0) throw new Error('nonnegative finite operand required');
  const word = BigInt(`0x${binary64Bits(value)}`) & ((1n << 63n) - 1n);
  const e = Number(word >> 52n), m = word & ((1n << 52n) - 1n);
  return e === 0 ? [m, -1074] : [m | (1n << 52n), e - 1075];
}
function ceilDyadic(m: bigint, exponent: number): number {
  if (m === 0n) return 0;
  let top = m.toString(2).length - 1 + exponent;
  let quantum = Math.max(top - 52, -1074);
  const shift = quantum - exponent;
  let q: bigint;
  if (shift > 0) {
    const s = BigInt(shift); q = m >> s;
    if ((m - (q << s)) !== 0n) q++;
  } else q = m << BigInt(-shift);
  if (q >= 1n << 53n) {
    if (q !== 1n << 53n) throw new Error('rounding invariant');
    q >>= 1n; quantum++;
  }
  top = q.toString(2).length - 1 + quantum;
  if (top > 1023) throw new Error('binary64 upper bound overflows');
  const word = top < -1022 ? q << BigInt(quantum + 1074)
    : (BigInt(top + 1023) << 52n) | ((q << BigInt(53 - q.toString(2).length)) - (1n << 52n));
  if (word > MAX_BITS) throw new Error('binary64 upper bound overflows');
  return numberFromWord(word);
}
export function upwardProduct(a: number, b: number): number {
  const [ma, ea] = positiveParts(a), [mb, eb] = positiveParts(b);
  return ceilDyadic(ma * mb, ea + eb);
}
export function upwardSmallSum(b0: number, rounding: number): number {
  const [ma, ea] = positiveParts(b0), [mb, eb] = positiveParts(rounding);
  return ceilDyadic((ma << BigInt(ea + 1074)) + (mb << BigInt(eb + 1074)) + 1n, -1074);
}
function scaledComponent(x: number, power: 53 | 64): number {
  if (!Number.isFinite(x) || x < 0) throw new Error('scaled component input');
  if (x === 0) return 0; // Accepted helper magnitudes canonicalize either signed zero.
  const d = 2 ** power, nearest = x / d, back = nearest * d;
  return back < x ? numberFromWord(BigInt(`0x${binary64Bits(nearest)}`) + 1n) : nearest;
}
export function absoluteBound(value: number, scale: number): number {
  if (!Number.isFinite(value) || !Number.isFinite(scale) || scale < 0) throw new Error('bound input');
  const b0 = scaledComponent(scale, 64);
  return scale > 0 && scale < 2 ** -988 ? upwardSmallSum(b0, scaledComponent(Math.abs(value), 53)) : b0;
}

type Obj = Record<string, any>;
export type AccuracyClass = 'relative_verified' | 'absolute_verified' | 'input_derived' | 'non_quantity' | 'not_covered';
export type RowClassification = Readonly<{ result_id: string; basis_ref: Readonly<{ ref_type: string; ref_id: string }>; normalized_bits: string; scale_bits: string | null; bound_bits: string | null; class: AccuracyClass }>;
export type RetainedPrecisionValidation = Readonly<{ invocation_bound: boolean; numerical_eligible: boolean; standing: 'eligible' | 'needs_recompute'; publication_sha256: string; classifications: readonly RowClassification[] }>;
const SCHEMA = receiptSchema as Obj;
const SAFE = BigInt(Number.MAX_SAFE_INTEGER);
// Eligibility is on since U7 (D-U7-5);
// every gate runs regardless.
const SUMMARY_COVERAGE_COMPLETE = true;
const COVERAGE_KEYS = ['body', 'has_data', 'stop'];
const BASE_ID = 'openpipestress.result_semantics/0.3.0/preview-physics-1';
const BASE_HASH = 'ae55503d44a4750714a35c423623e38cf4132099134097193024d1635bfbc88a';
const TABLE_HASH = 'c74742ce6a936384e00986006e6a0b2e6bb11f190451e876eed9ffa11903c6a8';
const COMPONENTS = ['UX', 'UY', 'UZ', 'RX', 'RY', 'RZ'];
const KINDS = ['translation', 'rotation', 'force', 'moment'];
const SLOTS = ['s128', 's256', 's512', 's1024', 'v256', 'v512', 'v1024'];
const ZERO = '0000000000000000';
const MIN_NORMAL = 2 ** -1022;
const NONQUANTITY = new Set(['linear_solver_mode_basis', 'sparse_live_path_dense_parity_relative_delta', 'modulus_basis_record', 'combination_modulus_basis_record']);
// C3:178-181: hull projection covers quantity rows except observed ancillary rows, support norms and maxima.
const HULL_EXCLUDED = new Set([...NONQUANTITY, 'support_reaction_force_magnitude_v2', 'support_reaction_moment_magnitude_v2', 'pipe_elastic_normal_stress_maximum_v2']);
const INPUT = new Set(['pipe_lame_hoop_stress_v2', 'pipe_lame_radial_stress_v2', 'pipe_section_pressure_hoop_stress', 'pipe_section_pressure_longitudinal_stress', 'constant_effort_support_applied_load', 'component_user_stress_multiplier_review', 'component_user_stiffness_macro_element_review', 'constant_effort_user_input_review', 'spring_hanger_user_input_review', 'expansion_joint_pressure_thrust_load_review']);
const FORCE = new Set(['element_local_axial_force', 'element_local_shear_force_y', 'element_local_shear_force_z', 'pipe_wall_axial_force_v2', 'pipe_effective_axial_force_v2', 'reaction_resultant', 'support_reaction_force_magnitude_v2']);
const MOMENT = new Set(['element_local_torsional_moment', 'element_local_bending_moment_y', 'element_local_bending_moment_z', 'support_reaction_moment_magnitude_v2']);
const STRESS = new Set(['element_local_axial_normal_stress', 'element_local_bending_normal_stress_y', 'element_local_bending_normal_stress_z', 'element_local_torsional_shear_stress', 'pipe_axial_membrane_stress_v2', 'pipe_elastic_normal_stress_maximum_v2', 'component_equal_factor_intensified_bending_stress_v1', 'open_formula_stress_summary']);
const hash = (domain: string, payload: unknown) => canonicalSha256HexCheckedV1({ domain, payload });
function need(ok: unknown, gate: string, suffix: string): asserts ok {
  if (!ok) throw new RetainedPrecisionError(gate, suffix.startsWith('SOURCE_') ? suffix : 'RETAINED_PRECISION_' + suffix);
}
function same(a: any, b: any): boolean {
  if (Object.is(a, b)) return true;
  if (!a || !b || typeof a !== 'object' || typeof b !== 'object' || Array.isArray(a) !== Array.isArray(b)) return false;
  const keys = Object.keys(a); return keys.length === Object.keys(b).length && keys.every(k => Object.hasOwn(b, k) && same(a[k], b[k]));
}
const sequence = (n: number) => Array.from({ length: n }, (_, i) => i);
const unique = (a: any[]) => new Set(a).size === a.length;
function at(a: Obj[], i: number, gate = 'G5', code = 'PRODUCT_ATTEMPT_MISMATCH'): Obj {
  need(Number.isSafeInteger(i) && i >= 0 && i < a.length, gate, code); return a[i];
}
function uint(v: number): bigint { need(Number.isSafeInteger(v) && v >= 0 && !Object.is(v, -0), 'G2', 'ENCODING_MISMATCH'); return BigInt(v); }
function sum(a: number[]): bigint { return a.reduce((n, v) => n + uint(v), 0n); }
function shape(v: any, spec: Obj): boolean {
  if (spec.$ref) return shape(v, SCHEMA.$defs[spec.$ref.split('/').at(-1)]);
  if (spec.oneOf) return spec.oneOf.filter((s: Obj) => shape(v, s)).length === 1;
  // D34: G1 compares const and enum members as JSON values, so -0 matches 0 here; G2 (negativeZeroFree) rejects it.
  const value = Object.is(v, -0) ? 0 : v;
  if (Object.hasOwn(spec, 'const') && !same(value, spec.const)) return false;
  if (spec.enum && !spec.enum.some((x: any) => same(value, x))) return false;
  switch (spec.type) {
    case 'object': return v !== null && typeof v === 'object' && !Array.isArray(v) && (spec.required ?? []).every((k: string) => Object.hasOwn(v, k)) && Object.keys(v).every(k => Object.hasOwn(spec.properties, k) && shape(v[k], spec.properties[k]));
    case 'array': return Array.isArray(v) && v.length >= (spec.minItems ?? 0) && v.length <= (spec.maxItems ?? Number.MAX_SAFE_INTEGER) && v.every(x => shape(x, spec.items));
    case 'string': return typeof v === 'string' && [...v].length >= (spec.minLength ?? 0);
    case 'number': case 'integer': return typeof v === 'number';
    case 'boolean': return typeof v === 'boolean';
    case 'null': return v === null;
    default: return true;
  }
}
function encoding(v: any, spec: Obj): void {
  if (spec.$ref) return encoding(v, SCHEMA.$defs[spec.$ref.split('/').at(-1)]);
  if (spec.oneOf) return encoding(v, spec.oneOf.find((s: Obj) => shape(v, s)));
  const tag = spec['x-rp-encoding'];
  if (tag === 'uint' || tag === 'i32') need(Number.isSafeInteger(v) && !Object.is(v, -0) && v >= spec.minimum && v <= spec.maximum, 'G2', 'ENCODING_MISMATCH');
  if (tag === 'bits' || tag === 'nonnegative_bits') {
    let n: number; try { n = decodeBinary64(v); } catch { throw new RetainedPrecisionError('G2', 'RETAINED_PRECISION_ENCODING_MISMATCH'); }
    if (tag === 'nonnegative_bits') need(n >= 0 && !Object.is(n, -0), 'G2', 'ENCODING_MISMATCH');
  }
  if (tag === 'hash') need(/^[0-9a-f]{64}$/.test(v), 'G2', 'ENCODING_MISMATCH');
  if (spec.type === 'object') for (const k of Object.keys(v)) encoding(v[k], spec.properties[k]);
  if (spec.type === 'array') for (const item of v) encoding(item, spec.items);
}
/** D34 (C1 §4 canonical +0; C1 G2 row "no -0 counter"): a JSON number equal to -0 anywhere in the receipt fails G2,
 * whatever its schema position (counters, indexes, enum and const members, untyped items). `x === 0` holds for -0. */
function negativeZeroFree(v: unknown): void {
  if (typeof v === 'number') need(!Object.is(v, -0), 'G2', 'ENCODING_MISMATCH');
  else if (v !== null && typeof v === 'object') for (const x of Object.values(v)) negativeZeroFree(x);
}
function freeze<T>(v: T): T { if (v && typeof v === 'object') { for (const x of Object.values(v)) freeze(x); Object.freeze(v); } return v; }
/** Validate descriptors before cloning; JSON serialization alone would erase -0 counters. */
function snapshot(v: unknown): Obj { checkedJsonText(v); return structuredClone(v) as Obj; }
function prepPayload(a: Obj): Obj {
  return { definition_id: a.definition_id, definition_sha256: PREPARED_DEFINITION_HASH, owner_ref: a.owner_ref, ordinary_attempt_ref: a.ordinary_attempt_ref, material_basis_ref: a.material_basis_ref,
    members: a.preparation.members.map((m: Obj) => ({ member: m.member, old_source: m.old_source, old_facts: m.old_facts, section: m.result.section })) };
}
const sha256Text = async (text: string): Promise<string> => {
  const value = await crypto.subtle.digest('SHA-256', new TextEncoder().encode(text).buffer as ArrayBuffer);
  return [...new Uint8Array(value)].map(x => x.toString(16).padStart(2, '0')).join('');
};
const isObj = (v: unknown): v is Obj => v !== null && typeof v === 'object' && !Array.isArray(v);
/** G0 (ruling D2; C1 and C3 G0 rows): exactly the producer identity (contract, profile, component and
 * schema versions), the definition and inherited-table hashes over the bound bytes, receipt_version,
 * the policy ids, the canonicalization profile and the 20B/60B thresholds. A G0 field that is absent
 * or mistyped fails G0; every other shape defect waits for G1, so the body is never walked here. */
async function header(source: Obj): Promise<void> {
  const fail = (ok: unknown, code = 'SOURCE_PRODUCER_CONTRACT_UNSUPPORTED') => need(ok, 'G0', code);
  const producer = isObj(source) ? source.producer : undefined;
  fail(isObj(producer) && producer.semantic_contract_id === RETAINED_PRECISION_ID && source.formulation_basis?.profile_id === RETAINED_PRECISION_PROFILE);
  fail(source.schema_version === '0.2.0' && producer.component_name === 'open_pipe_stress_product_physics' && producer.component_version === '0.2.0');
  fail(table.semantic_contract_id === RETAINED_PRECISION_ID && table.formulation_profile_id === RETAINED_PRECISION_PROFILE && table.inherited_semantic_contract_sha256 === BASE_HASH && same(table.product_formation_definitions, [{ id: PREPARED_DEFINITION_ID, sha256: PREPARED_DEFINITION_HASH }]), 'FORMATION_MISMATCH');
  fail(await hash('retained_precision_formation_v1', definition) === PREPARED_DEFINITION_HASH, 'FORMATION_MISMATCH');
  fail(await sha256Text(tableBytes) === TABLE_HASH && await sha256Text(inheritedTableBytes) === table.inherited_semantic_contract_sha256);
  // Settled reading (D2): with the retained-precision contract named, an absent or mistyped
  // retained_precision or body is an absent G0 field and fails G0.
  const b = isObj(source.retained_precision) ? source.retained_precision.body : undefined;
  fail(isObj(b));
  {
    for (const [k, v] of Object.entries({ receipt_version: 1, policy: 'M03-INTEGRITY-MP-v2', projection_policy: 'RP-LOGICAL-ATTEMPTS-v1', work_policy: 'W1-LME-20B-60B-v1', facade_policy: 'RP-FACADE-SI-v2', canonicalization: 'openpipestress_jcs_ijson_v1' })) fail(b[k] === v);
    fail(isObj(b.work) && b.work.case_limit === 20_000_000_000 && b.work.invocation_limit === 60_000_000_000);
    if (Array.isArray(b.product_attempts)) for (const a of b.product_attempts) if (isObj(a)) fail(a.definition_id === PREPARED_DEFINITION_ID);
  }
}
/** I57 §1: summary_coverage is required, null or [{ body, stop: [bool;4], has_data: bool }] with no other member. */
function coverageShape(proof: Obj): boolean {
  if (!Object.hasOwn(proof, 'summary_coverage')) return false;
  const cov = proof.summary_coverage;
  return cov === null || (Array.isArray(cov) && cov.every(e => e !== null && typeof e === 'object' && !Array.isArray(e) && same(Object.keys(e).sort(), COVERAGE_KEYS)
    && typeof e.body === 'number' && Array.isArray(e.stop) && e.stop.length === 4 && e.stop.every((f: unknown) => typeof f === 'boolean') && typeof e.has_data === 'boolean'));
}
async function integrity(source: Obj, transport: boolean): Promise<Obj> {
  const r = source.retained_precision;
  need(shape(r, SCHEMA) && (transport || (Array.isArray(source.results) && source.results.every((row: Obj) => shape(row, SCHEMA.$defs.RawRow)))), 'G1', 'RECEIPT_MISMATCH');
  const b = r.body;
  // I57 §4 G1, independent of the schema walker: exactly the required closed member.
  for (const a of b.product_attempts) if (a.proof !== null) need(coverageShape(a.proof), 'G1', 'RECEIPT_MISMATCH');
  need(await hash('retained_precision_receipt_mp_v2', b) === r.receipt_sha256, 'G1', 'RECEIPT_MISMATCH');
  if (!transport) { const { retained_precision: _omit, ...publication } = source; need(await hash('retained_precision_publication_mp_v2', publication) === b.publication_sha256, 'G1', 'RECEIPT_MISMATCH'); }
  for (const c of b.cases) if (c.status === 'selected' && b.sources[c.source_ref]) {
    const { index: _omit, ...s } = b.sources[c.source_ref]; need(await hash('retained_precision_source_mp_v2', s) === c.source_identity_sha256, 'G1', 'RECEIPT_MISMATCH');
  }
  for (const s of b.sources) if (s.preparation && b.product_attempts[s.preparation.attempt_ref]) {
    const a = b.product_attempts[s.preparation.attempt_ref];
    if (a.preparation.members.every((m: Obj) => m.result.kind === 'prepared')) need(await hash('retained_precision_preparation_v1', prepPayload(a)) === s.preparation.sha256, 'G1', 'RECEIPT_MISMATCH');
  }
  negativeZeroFree(r); encoding(r, SCHEMA); return b;
}
function coverage(b: Obj, source: Obj, invocation?: Obj): Map<string, Obj[]> {
  const fail = (ok: unknown) => need(ok, 'G3', 'COVERAGE_MISMATCH');
  const ids = b.cases.map((c: Obj) => c.basis_ref.ref_id);
  fail(unique(ids) && same(b.cases.map((c: Obj) => c.basis_ref), source.numerical_quality?.cases?.map((c: Obj) => c.basis_ref)) && b.cases.some((c: Obj) => c.status === 'selected'));
  if (invocation) fail(same(ids, invocation.request?.model?.load_cases?.map((c: Obj) => c.id)) && !(invocation.request?.model?.combinations?.length));
  const rows = new Map<string, Obj[]>(ids.map((id: string) => [id, []])); const seen = new Set();
  for (const row of source.results) { fail(!seen.has(row.id) && row.basis_ref?.ref_type === 'load_case' && rows.has(row.basis_ref.ref_id)); seen.add(row.id); rows.get(row.basis_ref.ref_id)!.push(row); }
  fail(b.ordinary_attempts.length === b.cases.length);
  const refs = b.cases.flatMap((c: Obj) => c.product_attempt_ref === null ? [] : [c.product_attempt_ref]);
  fail(same([...refs].sort((a, z) => a - z), sequence(b.product_attempts.length)));
  b.cases.forEach((c: Obj, i: number) => {
    const o = b.ordinary_attempts[i]; fail(c.ordinary.attempt_ref === i && same(c.ordinary.quality_binding, { kind: 'present', index: i }) && o.case_index === i && o.case_id === ids[i]);
    if (c.product_attempt_ref !== null) {
      const a = b.product_attempts[c.product_attempt_ref]; fail(a.id === c.product_attempt_ref && same(a.owner_ref, { kind: 'case', index: i }));
      const old = a.operational.old, pm = a.preparation.members, fresh = a.operational.new;
      // D1 (C2:98; retained_product.rs `ProductCapture::capture_case_source`; F1:130): old, prepared and new member ids are exactly 0..len-1 in
      // native order, so prepared and new are prefixes of old.
      fail(fresh.length <= pm.length && pm.length <= old.length && [old, pm, fresh].every(list => same(list.map((m: Obj) => m.member), sequence(list.length))));
      // D1: a captured prefix owns no prepared or new member here; its null source/run and
      // unavailable result are the G5 association (F1:97, 130-131; C3:304).
      if (a.operational.old_coverage === 'captured_prefix') fail(!pm.length && !fresh.length);
      // D22 (D16; C3:146-148): a source_ref that does not resolve is a G5 association defect, so the
      // source-dependent G3 comparisons below run only when it resolves.
      const sourced = a.source_ref !== null ? (Number.isSafeInteger(a.source_ref) && a.source_ref >= 0 && a.source_ref < b.sources.length ? b.sources[a.source_ref] : undefined) : null;
      if (a.source_ref !== null) { if (sourced) fail(same(old.map((m: Obj) => m.member), sourced.id_maps.members.map((m: Obj) => m.kernel_member))); }
      // D1 (checkpoint A correction): unsourced complete old coverage matches every CaseSource's member
      // count (one model). No emptiness rule: no native rejection of an empty inventory is cited. With no
      // CaseSource, G8 compares the list with the invocation's members.
      else if (a.operational.old_coverage === 'complete') fail(b.sources.every((s: Obj) => s.id_maps.members.length === old.length));
      // C3 G3 row-index coverage (ruling): every index names a hull-projected row of this case,
      // strictly ascending. A valid sorted subset on Ready is the later G5 exact-set check.
      if (a.proof) { const indices = a.proof.projection_outcomes.map((x: Obj) => x.row_index), caseRows = rows.get(ids[i])!; fail(indices.every((x: number, j: number) => x < caseRows.length && !HULL_EXCLUDED.has(caseRows[x].kind) && (!j || x > indices[j - 1]))); }
      // I57 §4 G3: a non-null roster has exactly one entry per native body of the
      // associated source, ascending 0..body_count-1. No source: G5 owns the null rule.
      const cov = a.proof?.summary_coverage;
      if (Array.isArray(cov) && sourced) fail(cov.length >= 1 && cov.length === sourced.body_membership.length && same(cov.map((e: Obj) => e.body), sequence(cov.length)));
    }
  });
  // D29: a CaseSource with an empty body inventory cannot be emitted (source.rs `PrimitiveSource::new` NoNodes; I57 §1).
  b.sources.forEach((s: Obj, i: number) => fail(s.body_membership.length >= 1 && s.index === i && s.owner.kind === 'case' && ids[s.owner.case_index] === s.owner.case_id));
  b.material_bases.forEach((m: Obj, i: number) => fail(m.index === i && unique(m.case_indices) && m.case_indices.every((c: number) => c < ids.length)));
  const runs = b.cases.filter((c: Obj) => c.run).map((c: Obj) => c.run).sort((a: Obj, z: Obj) => a.id - z.id);
  fail(same(runs.map((r: Obj) => r.id), sequence(runs.length)) && same(b.work.execution_order, runs.map((r: Obj) => r.origin.owner_ref)));
  return rows;
}
function diagnostics(b: Obj, source: Obj): void {
  const fail = (ok: unknown) => need(ok, 'G4', 'DIAGNOSTIC_MISMATCH'); const ds: Obj[] = source.diagnostics;
  fail(Array.isArray(ds) && unique(ds.map(d => d.id)) && !ds.some(d => d.code === 'SOURCE_BLOCK_RECOVERY_SELECTED'));
  for (const c of b.cases) {
    const cid = c.basis_ref.ref_id, selected = ds.filter(d => d.code === 'RETAINED_PRECISION_SELECTED' && d.affected_refs?.includes(cid)), unavailable = ds.filter(d => d.code === 'RETAINED_PRECISION_UNAVAILABLE' && d.affected_refs?.includes(cid));
    fail(selected.length === Number(c.status === 'selected') && unavailable.length === Number(c.status === 'unavailable') && [...selected, ...unavailable].every(d => same(d.affected_refs, [cid])));
    if (c.status === 'unavailable') fail(unavailable[0].id === c.diagnostic_ref);
    if (c.status === 'selected') fail(!ds.some(d => d.code === 'SOURCE_BLOCK_RECOVERY_UNAVAILABLE' && d.affected_refs?.includes(cid)));
  }
  for (const d of ds.filter(d => ['RETAINED_PRECISION_SELECTED', 'RETAINED_PRECISION_UNAVAILABLE'].includes(d.code))) fail(d.affected_refs?.length === 1 && b.cases.some((c: Obj) => c.basis_ref.ref_id === d.affected_refs[0]));
}

/** The native AttemptStop carried by a failed physical/logical outcome, or null (C1:114). */
const outcomeStop = (outcome: Obj): Obj | null => outcome?.kind === 'failed' && outcome.reason?.space === 'attempt' && outcome.reason.tag === 'stop' ? outcome.reason.stop : null;
const INVOCATION_BUDGET = { space: 'unresolved', tag: 'budget', scope: 'invocation' };
/** @internal Exported only for reader-logic unit tests (checklist N5/N8/N10); not a public entry point. */
export function nativeSchedule(run: Obj, source: Obj): void {
  const fail = (ok: unknown) => need(ok, 'G5', 'ATTEMPT_MISMATCH');
  const records: Obj[] = run.records, attempts: Obj[] = run.attempts;
  // C1 actual native schedule: at most the p128/p256/p512 logical attempts, records exist iff
  // attempts do, and the ladder opens with a fresh p128 candidate in record 0 (below).
  fail(attempts.length <= 3 && (records.length === 0) === (attempts.length === 0));
  // N14: at most three iterative corrections per physical record (adaptive.rs run! table).
  fail(records.every(r => Number.isSafeInteger(r.corrections) && r.corrections >= 0 && r.corrections <= 3));
  // C1:66-68 (ruling): a WorkAccounting terminal exists natively only with a non-exact work status
  // (adaptive.rs `run_schedule_inner`, `finish_terminal`, `solve_cases_projected`), and such a run is never emitted. It lies outside the emitted
  // terminal domain (C1:148) wherever it appears, idle or not.
  fail(!(run.kernel_terminal.kind === 'unresolved' && run.kernel_terminal.reason?.space === 'unresolved' && run.kernel_terminal.reason?.tag === 'work_accounting'));
  const stopOf = outcomeStop;
  const escalates = (stop: Obj | null) => stop && ['pivot', 'condition', 'residual_gate'].includes(stop.tag);
  // D5b/D21: a failed verification is a solve failure only if nothing shows its pass ran: no verification
  // shared build (adaptive.rs `verify_precision`), no verification work, and no verification summary (adaptive.rs `verify_precision`).
  const solveFailure = (vr: Obj) => vr.verification_shared_build_ref === null && vr.work.verification_lme === 0 && vr.verification === null;
  const terminalFor = (stop: Obj): Obj => {
    const { space: _space, tag, ...payload } = stop;
    const refused = ['negative_energy', 'structure'].includes(tag);
    return { kind: refused ? 'refused' : 'unresolved', reason: {
      space: refused ? 'refusal' : 'unresolved',
      tag: ({ span: 'exact_sum_span', exponent: 'exponent_range', resolution_scale: 'resolution_scale_unencodable' } as Obj)[tag] ?? tag,
      ...payload,
    } };
  };
  for (const record of records) {
    const reason = record.outcome.reason;
    if (reason?.quantity) fail(source.layout.some((row: Obj) => same(row.quantity, reason.quantity) && row.body === reason.body && row.kind === reason.kind));
    // D33: the native verification estimate exists only for force and moment rows (verify.rs `verify_state`).
    if (reason?.space === 'attempt' && reason.tag === 'verification_estimate') fail(['force', 'moment'].includes(reason.kind));
    if (record.role === 'candidate') fail(record.verification === null && record.verification_shared_build_ref === null && record.work.verification_lme === 0);
  }
  for (let ai = 0; ai < attempts.length; ai++) {
    const a = attempts[ai], cr = at(records, a.candidate_record, 'G5', 'ATTEMPT_MISMATCH');
    if (ai === 0) fail(a.precision === 128 && a.origin.kind === 'fresh' && cr.index === 0);
    else {
      const previous = attempts[ai - 1], v = previous.verification;
      if (v === null) {
        fail(escalates(stopOf(previous.outcome)) && a.precision === previous.precision * 2 && a.origin.kind === 'fresh' && cr.index === previous.candidate_record + 1);
      } else if (v.phase === 'failed') {
        const vr = at(records, v.record, 'G5', 'ATTEMPT_MISMATCH');
        fail(escalates(stopOf(vr.outcome)) && solveFailure(vr));
        fail(a.precision === previous.precision * 4 && a.origin.kind === 'fresh' && cr.index === v.record + 1);
      } else {
        fail(previous.outcome.kind === 'rejected' && previous.outcome.reason.tag !== 'verification_failed');
        fail(a.precision === previous.precision * 2 && same(a.origin, { kind: 'reused_verification', attempt: ai - 1 }) && cr.index === v.record);
      }
    }
    if (a.verification === null) fail(stopOf(a.outcome) !== null);
    else {
      const vr = at(records, a.verification.record, 'G5', 'ATTEMPT_MISMATCH');
      fail(vr.index === cr.index + 1);
      if (a.verification.phase === 'failed') fail(same(a.outcome, { kind: 'rejected', reason: { space: 'attempt', tag: 'verification_failed' } }) && stopOf(vr.outcome) !== null);
      else {
        fail(vr.verification !== null);
        // N13: a plain verification record is Verified only for the accepted candidate and
        // otherwise Solved; a reused verification keeps its later candidate outcome.
        if (vr.role === 'verification') fail(vr.outcome.kind === (a.outcome.kind === 'accepted' ? 'verified' : 'solved'));
        else fail(vr.role === 'verification_then_candidate' && a.outcome.kind === 'rejected');
      }
    }
    fail(['accepted', 'rejected', 'failed'].includes(a.outcome.kind));
    // D5c (adaptive.rs `run_schedule_inner`): rejected(verification_failed) only with a failed verification phase.
    if (a.outcome.kind === 'rejected' && a.outcome.reason?.space === 'attempt' && a.outcome.reason.tag === 'verification_failed') fail(a.verification?.phase === 'failed');
  }
  const last = attempts.at(-1), terminal = run.kernel_terminal;
  if (!last) {
    // N1/N10/N11: a pre-schedule return (invocation entry or group refusal) has no record or charge.
    fail(run.case_charge === 0 && run.invocation_increment === 0 && terminal.kind !== 'selected' && terminal.reason !== null);
    return;
  }
  // N7: the ladder selects exactly when its last logical attempt is accepted.
  fail((terminal.kind === 'selected') === (last.outcome.kind === 'accepted'));
  if (terminal.kind === 'selected') return;
  let stop = stopOf(last.outcome), expected: Obj | null = null;
  if (last.verification?.phase === 'failed') {
    const vr = at(records, last.verification.record, 'G5', 'ATTEMPT_MISMATCH');
    stop = stopOf(vr.outcome);
    if (escalates(stop) && solveFailure(vr)) {
      fail(last.precision * 4 > 512); expected = { kind: 'unresolved', reason: { space: 'unresolved', tag: 'ceiling' } };
    }
  } else if (last.verification === null && escalates(stop)) {
    fail(last.precision === 512); expected = { kind: 'unresolved', reason: { space: 'unresolved', tag: 'ceiling' } };
  } else if (last.outcome.kind === 'rejected') {
    fail(last.precision === 512 && last.verification?.phase === 'completed');
    expected = { kind: 'unresolved', reason: { space: 'unresolved', tag: 'ceiling' } };
  }
  if (expected === null && stop) { fail(!escalates(stop)); expected = terminalFor(stop); }
  if (expected) fail(same(run.kernel_terminal, expected));
}

/** G5 class 1 (C3:304): native schedule and origin, with native WORK. Ruling D3: a class containing an
 * ATTEMPT defect reports ATTEMPT, and reports WORK only when it has none, so native WORK predicates are
 * deferred to the end of the class. Every reference the class follows is resolved explicitly and fails
 * with the code of the check that follows it (ATTEMPT lookups through `at`, build references through a
 * deferred WORK predicate). The catch-all is only a fail-closed fallback with the class code ATTEMPT.
 * @internal Exported only for reader-logic tests. */
export function nativeRuns(b: Obj): void {
  const deferred: boolean[] = [];
  try { nativeClass(b, ok => { deferred.push(!!ok); }); } catch (error) {
    if (error instanceof RetainedPrecisionError) throw error;
    throw new RetainedPrecisionError('G5', 'RETAINED_PRECISION_ATTEMPT_MISMATCH');
  }
  need(deferred.every(Boolean), 'G5', 'WORK_MISMATCH');
}
function nativeClass(b: Obj, work: (ok: unknown) => void): void {
  const fail = (ok: unknown, code = 'ATTEMPT_MISMATCH') => need(ok, 'G5', code);
  const checked = (n: bigint): bigint => { work(n >= 0n && n <= SAFE); return n; };
  // D8 kernel scope (checkpoint A; C1:66-68; adaptive.rs `run_schedule_inner`, `finish_terminal`, `solve_cases_projected`): a work_accounting stop or
  // reason anywhere in a Run, a build or a group preparation is outside the emitted domain (class 1, ATTEMPT).
  const kernelRuns = b.cases.filter((c: Obj) => c.run).map((c: Obj) => c.run);
  fail(!locate([kernelRuns, b.builds, b.groups.map((g: Obj) => g.preparation)]).some(([, o]) => o.tag === 'work_accounting'));
  // A build reference is followed by WORK checks (slot, group, work, stages), so a dangling one fails WORK.
  const buildOf = (bi: unknown): Obj | null => Number.isSafeInteger(bi) && (bi as number) >= 0 && (bi as number) < b.builds.length ? b.builds[bi as number] : null;
  const runs: Obj[] = b.cases.filter((c: Obj) => c.run).map((c: Obj) => c.run).sort((a: Obj, z: Obj) => a.id - z.id);
  fail(same(b.calls.flatMap((c: Obj) => c.run_refs), sequence(runs.length)));
  const live = new Map<number, Map<string, number>>(); const seenBuilds = new Set<number>();
  let current = 0n;
  for (let ci = 0; ci < b.calls.length; ci++) {
    const call = b.calls[ci]; fail(call.id === ci && call.kind === 'case_batch' && call.result.kind === 'runs');
    work(uint(call.invocation_before) === current); fail(call.owner_refs.length === call.source_refs.length && call.run_refs.length === call.source_refs.length && unique(call.source_refs));
    let previousCase = -1;
    for (let pos = 0; pos < call.run_refs.length; pos++) {
      const ri = call.run_refs[pos], si = call.source_refs[pos], oi = call.owner_refs[pos];
      const run = at(runs, ri, 'G5', 'ATTEMPT_MISMATCH'), c = at(b.cases, oi.index, 'G5', 'ATTEMPT_MISMATCH'), s = at(b.sources, si, 'G5', 'ATTEMPT_MISMATCH');
      fail(oi.kind === 'case' && oi.index > previousCase); previousCase = oi.index;
      fail(same(run.origin, { call: ci, position: pos, group: run.origin.group, source_ref: si, owner_ref: oi }) && same(c.run, run) && c.source_ref === si && s.owner.case_index === oi.index);
      work(uint(run.invocation_before) === current);
      // N10 (adaptive.rs `solve_cases_projected`): an invocation-entry return is idle with group null. Its only
      // emitted form is Budget(invocation) once invocation_before >= Li (a meter fault is never emitted).
      // D27: an ATTEMPT check reads the Run's recorded invocation_before, never the WORK-derived running sum
      // (a broken meter chain is reported by the deferred WORK predicate above).
      const exhausted = uint(run.invocation_before) >= uint(b.work.invocation_limit);
      if (exhausted) fail(run.origin.group === null);
      if (run.origin.group === null) {
        fail(exhausted && !run.records.length && !run.attempts.length && !run.cache_before.length && !run.cache_after.length && run.kernel_terminal.kind === 'unresolved'
          && same(run.kernel_terminal.reason, INVOCATION_BUDGET));
      } else {
        const group = at(b.groups, run.origin.group, 'G5', 'ATTEMPT_MISMATCH');
        fail(group.call === ci && group.source_refs.includes(si) && group.stiffness_sha256 === s.stiffness_sha256);
        if (group.preparation.kind === 'refused') fail(!run.records.length && !run.attempts.length && run.kernel_terminal.kind === 'refused' && same(run.kernel_terminal.reason, group.preparation.reason));
        // N10 (adaptive.rs `solve_cases_projected`): a run with no attempt in a ready group is the CasePrep refusal.
        else if (!run.attempts.length) fail(run.kernel_terminal.kind === 'refused' && run.kernel_terminal.reason?.tag === 'ledger_unavailable');
      }
      const cache = live.get(run.origin.group) ?? new Map<string, number>();
      const inventory = () => SLOTS.filter(slot => cache.has(slot)).map(slot => ({ slot, build: cache.get(slot) }));
      work(same(run.cache_before, inventory()));
      const records: Obj[] = run.records, attempts: Obj[] = run.attempts;
      fail(records.length <= 4 && same(records.map(r => r.index), sequence(records.length)) && records.every((r, i) => !i || r.precision > records[i - 1].precision));
      nativeSchedule(run, s);
      const amounts: [bigint, bigint][] = [];
      for (const r of records) {
        const w = r.work, own = checked(uint(w.wide_lme) + uint(w.exact_sum_lme));
        work(own === uint(w.own_lme) && own === sum(Object.values(w.own_stages)) && uint(w.stop_rule_lme) + uint(w.verification_lme) <= own);
        work(w.own_stages.stop_rule === w.stop_rule_lme);
        work(sum(['scale', 'estimate', 'charge', 'bound', 'shift'].map(stage => w.own_stages[stage])) === uint(w.verification_lme));
        work(sum(Object.values(w.shared_stages)) === uint(w.shared_lme) + uint(w.verification_shared_lme));
        fail(r.residual_basis === (r.precision === 1024 ? 1024 : r.precision + 64));
        fail(r.storage.limbs_per_entry === (r.precision <= 256 ? 4 : r.precision === 512 ? 8 : 16));
        if (r.role === 'verification') work(w.stop_rule_lme === 0);
        amounts.push([checked(own + uint(w.shared_lme) + uint(w.verification_shared_lme)), checked(own + (w.shared_built_here ? uint(w.shared_lme) : 0n) + (w.verification_shared_built_here ? uint(w.verification_shared_lme) : 0n))]);
        fail(r.precision !== 1024 || r.role === 'verification');
        for (const [field, phase, flag, cost, prefix] of [['shared_build_ref', 'shared', 'shared_built_here', 'shared_lme', 's'], ['verification_shared_build_ref', 'verification_shared', 'verification_shared_built_here', 'verification_shared_lme', 'v']]) {
          const bi = r[field]; if (bi === null) { work(!w[flag] && w[cost] === 0); continue; }
          const build = buildOf(bi);
          if (!build) { work(false); continue; }
          const slot = prefix + r.precision;
          work(build.group === run.origin.group && build.slot === slot && build.work === w[cost]);
          // C1 (adaptive.rs obtain 3984-4026): state/reason agree, and a failed build fails the
          // requesting record with the same stop, whether built here or reused from the cache.
          work((build.state === 'success') === (build.reason === null) && (build.state === 'budget_failure') === (build.reason?.tag === 'budget'));
          if (build.state !== 'success') fail(same(outcomeStop(r.outcome), build.reason));
          if (w[flag]) {
            work(!seenBuilds.has(bi) && same(build.origin, { call: ci, run: ri, physical_record: r.index, phase })); seenBuilds.add(bi);
            work(!cache.has(slot)); if (build.state !== 'budget_failure') cache.set(slot, bi);
          } else work(cache.get(slot) === bi && build.state !== 'budget_failure');
        }
        for (const stage of Object.keys(w.shared_stages)) {
          const sharedBuild = buildOf(r.shared_build_ref), verificationBuild = buildOf(r.verification_shared_build_ref);
          const shared = sharedBuild ? uint(sharedBuild.stages[stage]) : 0n, verification = verificationBuild ? uint(verificationBuild.stages[stage]) : 0n;
          work(uint(w.shared_stages[stage]) === shared + verification);
        }
      }
      work(same(run.cache_after, inventory())); if (run.origin.group !== null) live.set(run.origin.group, cache);
      const fragments = new Set<string>();
      for (let ai = 0; ai < attempts.length; ai++) {
        const a = attempts[ai], cr = at(records, a.candidate_record, 'G5', 'ATTEMPT_MISMATCH');
        fail(a.precision === cr.precision && [128, 256, 512].includes(a.precision) && ['candidate', 'verification_then_candidate'].includes(cr.role) && same(a.outcome, cr.outcome));
        fail(!ai || a.precision > attempts[ai - 1].precision);
        if (a.origin.kind === 'reused_verification') { const prior = at(attempts.slice(0, ai), a.origin.attempt, 'G5', 'ATTEMPT_MISMATCH'); fail(prior.verification?.record === cr.index && cr.role === 'verification_then_candidate'); }
        else fail(cr.role === 'candidate');
        const v = a.verification;
        if (v !== null) {
          const vr = at(records, v.record, 'G5', 'ATTEMPT_MISMATCH'); fail(vr.index > cr.index && vr.precision === v.precision && v.precision === 2 * a.precision && ['verification', 'verification_then_candidate'].includes(vr.role));
          if (vr.role === 'verification_then_candidate') fail(v.phase === 'completed' && v.reason === null);
          else if (vr.outcome.kind === 'failed') fail(v.phase === 'failed' && same(v.reason, vr.outcome.reason));
          else fail(v.phase === 'completed' && v.reason === null);
        } else fail(a.outcome.kind === 'failed');
        let charge = 0n, debit = 0n;
        for (const f of a.charges) {
          const key = f.record + ':' + f.part; fail(!fragments.has(key)); fragments.add(key);
          const fr = at(records, f.record, 'G5', 'ATTEMPT_MISMATCH');
          if (f.part === 'candidate_stop') { fail(f.record === cr.index); charge += uint(fr.work.stop_rule_lme); debit += uint(fr.work.stop_rule_lme); }
          else { fail((f.record === cr.index && cr.role === 'candidate') || v?.record === f.record); charge += amounts[f.record][0] - uint(fr.work.stop_rule_lme); debit += amounts[f.record][1] - uint(fr.work.stop_rule_lme); }
        }
        work(checked(charge) === uint(a.case_charge) && checked(debit) === uint(a.invocation_increment));
        if (a.outcome.kind === 'accepted') fail(ai === attempts.length - 1 && run.kernel_terminal.kind === 'selected');
      }
      const required = records.flatMap(r => [r.index + ':solve_and_verification', ...(r.role !== 'verification' ? [r.index + ':candidate_stop'] : [])]);
      fail(fragments.size === required.length && required.every(x => fragments.has(x)));
      const charge = checked(amounts.reduce((v, x) => v + x[0], 0n)), debit = checked(amounts.reduce((v, x) => v + x[1], 0n));
      work(charge === uint(run.case_charge) && charge === sum(attempts.map(a => a.case_charge)) && debit === uint(run.invocation_increment) && debit === sum(attempts.map(a => a.invocation_increment)));
      current = checked(current + debit); work(current === uint(run.invocation_after));
      const terminal = run.kernel_terminal;
      fail((terminal.kind === 'selected') === (terminal.reason === null));
      if (terminal.kind === 'selected') {
        const last = at(attempts, attempts.length - 1, 'G5', 'ATTEMPT_MISMATCH'); fail(last.outcome.kind === 'accepted' && last.verification !== null);
        const cr = at(records, last.candidate_record, 'G5', 'ATTEMPT_MISMATCH'), vr = at(records, last.verification.record, 'G5', 'ATTEMPT_MISMATCH');
        fail(vr.outcome.kind === 'verified' && last.verification.phase === 'completed' && last.verification.reason === null && vr.verification !== null);
        work(charge <= uint(b.work.case_limit) && current <= uint(b.work.invocation_limit));
        if (c.status === 'selected') {
          const selection = c.selection; fail(selection.precision === last.precision && selection.verification_precision === last.verification.precision);
          fail(['pivot_margin_min', 'rcond', 'residual_worst', 'corrections'].every(k => same(selection[k], cr[k])) && selection.rcond_label === 'sensitivity to matrix-entry perturbation, not to authored parameters');
          fail(same(selection.resolution_scale, vr.verification.resolution) && same(selection.theta, vr.verification.theta) && same(selection.certified_bound, vr.verification.bound.filter((v: Obj) => v.value !== null)));
        }
      } else {
        fail(c.status !== 'selected');
        if (terminal.reason?.tag === 'budget') {
          const caseOver = charge > uint(b.work.case_limit), invOver = current > uint(b.work.invocation_limit);
          work(terminal.reason.scope === 'case' ? caseOver : (!caseOver && (invOver || uint(run.invocation_before) >= uint(b.work.invocation_limit))));
        }
      }
    }
    work(current === uint(call.invocation_after));
  }
  work(current === uint(b.work.charged) && seenBuilds.size === b.builds.length);
  b.groups.forEach((g: Obj, i: number) => {
    fail(g.id === i && g.source_refs.length && g.first_source_ref === g.source_refs[0] && unique(g.source_refs));
    const call = at(b.calls, g.call, 'G5', 'ATTEMPT_MISMATCH');
    fail(g.source_refs.every((si: number) => call.source_refs.includes(si) && b.sources[si]?.stiffness_sha256 === g.stiffness_sha256));
    fail(same(g.source_refs, runs.filter(r => r.origin.group === i).map(r => r.origin.source_ref)));
  });
  // C5 (C2:143; adaptive.rs `solve_cases_projected`): call-local groups partition the call's non-idle sources by
  // full stiffness bytes in first-seen order, and each run names its own group.
  b.calls.forEach((call: Obj, ci: number) => {
    const order: string[] = [], members = new Map<string, number[]>(), runGroups: [Obj, number][] = [];
    call.run_refs.forEach((ri: number, pos: number) => {
      const run = at(runs, ri, 'G5', 'ATTEMPT_MISMATCH'); if (run.origin.group === null) return;
      const si = call.source_refs[pos], key = at(b.sources, si, 'G5', 'ATTEMPT_MISMATCH').stiffness_sha256;
      if (!members.has(key)) { order.push(key); members.set(key, []); }
      members.get(key)!.push(si); runGroups.push([run, order.indexOf(key)]);
    });
    const groups = b.groups.filter((g: Obj) => g.call === ci);
    fail(same(groups.map((g: Obj) => [g.stiffness_sha256, g.source_refs]), order.map(k => [k, members.get(k)])));
    for (const [run, gi] of runGroups) fail(run.origin.group === groups[gi].id);
  });
  b.builds.forEach((build: Obj, i: number) => { work(build.id === i && uint(build.work) === sum(Object.values(build.stages))); fail((build.state === 'success') === (build.reason === null)); if (build.state === 'budget_failure') fail(build.reason?.tag === 'budget'); });
}
function exactWork(v: any): boolean {
  if (!v || typeof v !== 'object') return true;
  if (v.kind === 'unavailable' && v.fault) return false;
  if ((v.sticky_status && v.sticky_status !== 'exact') || v.lost === true) return false;
  return Object.values(v).every(exactWork);
}
const STATUS_FAULTS: Record<string, string[]> = { exact: [], overflow: ['overflow'], inconsistent: ['inconsistent'], both: ['overflow', 'inconsistent'] };
function objects(v: any, out: Obj[] = []): Obj[] {
  if (Array.isArray(v)) for (const x of v) objects(x, out);
  else if (v && typeof v === 'object') { out.push(v); for (const x of Object.values(v)) objects(x, out); }
  return out;
}
/** Ruling 06d (I62 ACCOUNTING_CAUSES R1-R3), G5 work class (C3:304), for every product attempt.
 * R1: an adapter overflow fault (retained_product.rs `AdapterWork::enter`) or any CaptureError/G5aError accounting{event} cause
 *     built from it (retained_product.rs `AdapterWork::require`, `ProductCapture::solver_observations`, `ProductCapture::g5a`) is never emittable (C3:233-236).
 * R2: ScalarTrace.lost is set only at u64::MAX (retained_product.rs `ScalarWork::check` through `ScalarWork::operation`), so it is never emittable.
 * R3: a work_accounting{fault} cause comes from its owning trace's status (final_case.rs `ProductCertificateSpent::visit`, `ProductCertificateSpent::f64_op`, `begin_prepared_product`), so the
 *     attempt's emitted unavailable-Count faults and sticky statuses must contain that fault. */
/** @internal Exported only for the reader-logic R1-R3 isolation test; not a public entry point. */
export function accountingRules(a: Obj): boolean[] {
  const located = locate(a), proof = a.proof;
  // R1': adapter overflow, or any CaptureError/G5aError accounting{event} cause (retained_product.rs `AdapterWork::enter`, `AdapterWork::require`, `ProductCapture::solver_observations`, `ProductCapture::g5a`).
  const r1 = a.adapter.fault === null && !located.some(([, o]) => o.kind === 'accounting' && Object.hasOwn(o, 'event'));
  // R2': lost, or an OperationalError accounting located as a MemberOperational error, a CaptureError
  // prepared_arithmetic cause or a G5aError operational/arithmetic cause (retained_product.rs `ScalarWork::check` through `ScalarWork::operation`).
  const error = a.result.kind === 'unavailable' ? a.result.error : null;
  const operational: unknown[] = ['old', 'new'].flatMap(side => a.operational[side].filter((m: Obj) => m.result.kind !== 'ready').map((m: Obj) => m.result.error));
  const g5aCauses = (cause: unknown) => locate(cause).filter(([, x]) => ['operational', 'arithmetic'].includes(x.kind) && isObj(x.cause)).map(([, x]) => x.cause);
  for (const [, o] of locate(error)) {
    if (o.kind === 'prepared_arithmetic') operational.push(o.cause);
    if (o.kind === 'g5a') operational.push(...g5aCauses(o.cause));
  }
  for (const check of Object.values(proof?.checks ?? {}) as Obj[]) if (check.kind === 'failed' && check.error?.kind === 'g5a') operational.push(...g5aCauses(check.error.cause));
  const r2 = !located.some(([, o]) => o.lost === true) && !operational.some(e => isObj(e) && e.kind === 'accounting' && !Object.hasOwn(e, 'event'));
  // R3': every fault-bearing cause, in any spelling, has its fault in its owner's emitted statuses.
  let r3 = true;
  for (const [path, o] of located) {
    if (!((o.kind === 'work_accounting' || o.tag === 'work_accounting' || o.kind === 'work') && Object.hasOwn(o, 'fault'))) continue;
    const owner = faultOwner(a, path);
    r3 &&= owner != null && Object.hasOwn(STATUS_FAULTS, o.fault) && STATUS_FAULTS[o.fault].every(x => statuses(owner).has(x));
  }
  // R4: a SectionError accounting needs a non-exact status in that member's PreparationWork (FK product_certificate.rs `SectionPreparationWork::check`).
  let r4 = true;
  for (const m of a.preparation.members) if (m.result.kind !== 'prepared' && m.result.error?.kind === 'accounting') r4 &&= statuses(m.work).size > 0;
  if (error?.kind === 'preparation' && error.section?.kind === 'accounting') { const members = a.preparation.members; r4 &&= members.length > 0 && statuses(members.at(-1).work).size > 0; }
  return [r1, r2, r3, r4];
}
/** Objects in a value with their key paths. */
function locate(v: unknown, path: (string | number)[] = [], out: [(string | number)[], Obj][] = []): [(string | number)[], Obj][] {
  if (Array.isArray(v)) v.forEach((x, i) => locate(x, [...path, i], out));
  else if (isObj(v)) { out.push([path, v]); for (const [k, x] of Object.entries(v)) locate(x, [...path, k], out); }
  return out;
}
/** Emitted non-exact statuses in a work value: unavailable Count faults and sticky statuses. */
function statuses(v: unknown): Set<string> {
  const seen = new Set<string>();
  for (const o of objects(v)) {
    if (o.kind === 'unavailable' && Object.hasOwn(STATUS_FAULTS, o.fault)) STATUS_FAULTS[o.fault].forEach(x => seen.add(x));
    if (typeof o.sticky_status === 'string' && Object.hasOwn(STATUS_FAULTS, o.sticky_status)) STATUS_FAULTS[o.sticky_status].forEach(x => seen.add(x));
  }
  return seen;
}
/** R3' owner scopes (checkpoint A, D8): a member's PreparationWork; a lane's work; the values completion; else the ProofTrace. */
function faultOwner(a: Obj, path: (string | number)[]): unknown {
  const proof = a.proof;
  if (path[0] === 'preparation' && path[1] === 'members') return a.preparation.members[path[2] as number]?.work;
  if (path[0] === 'result' && path[1] === 'error' && path[2] === 'section') return a.preparation.members.at(-1)?.work ?? null;
  if (path[0] === 'proof' && path[1] === 'lanes') return proof?.lanes?.[path[2] as number]?.work;
  if (path[0] === 'result' && path[1] === 'error' && path[2] === 'cause' && a.result.error?.kind === 'values') return proof ? proof.completion : null;
  return proof;
}
function count(v: Obj): bigint | null { return v.kind === 'exact' ? uint(v.value) : null; }
/** G2: conversion payloads decode as finite binary64 with nonnegative relative precision. */
function conversions(v: Obj): void {
  const fail = (ok: unknown) => need(ok, 'G2', 'ENCODING_MISMATCH');
  let ok = true;
  try {
    if (v.kind === 'normal' || v.kind === 'subnormal') decodeBinary64(v.value);
    if (v.kind === 'subnormal') ok = decodeBinary64(v.relative_precision) >= 0 && v.relative_precision !== '8000000000000000';
  } catch { ok = false; }
  fail(ok);
}
/** C3 P5 (G5 PRODUCT_ATTEMPT): Normal is normal or a signed zero; Subnormal has nonzero subnormal bits. */
function conversionKind(v: Obj): boolean {
  if (v.kind === 'normal') { const n = decodeBinary64(v.value); return n === 0 || Math.abs(n) >= MIN_NORMAL; }
  if (v.kind === 'subnormal') { const n = decodeBinary64(v.value); return Math.abs(n) > 0 && Math.abs(n) < MIN_NORMAL; }
  return true;
}
/** D37 (D35 widened; S06 §1; D4d): the stage records the native sequence can leave behind each unavailable error kind
 * (PP retained_product.rs prepare_owned_case 3136-3260, solve_native 3272-3291, project_candidate 3456-3550; I62's table).
 * One mark per stage in STAGE_ORDER: '-' not entered, 'C' completed, 'F' failed; '?' is completed or failed (Observables
 * and G5a are both entered and checked after a failed certificate whenever the verdicts were copied, 3501-3508). */
const STAGE_ORDER = ['preparation', 'native', 'proof_start', 'projection', 'maxima', 'values', 'aliases', 'certificate', 'observables', 'g5a'];
const STAGE_MARK: Record<string, string> = { not_entered: '-', completed: 'C', failed: 'F' };
const NATIVE_STAGE_RECORDS: Record<string, string[]> = {
  preparation: ['F---------'],                                      // prepare_owned_case fails
  native: ['CF--------'],                                           // solve_native, nonselected Run
  capture: ['CF--------', 'CC--------', 'CCCCCCCC--', 'CCCCCCCCCC'], // solve_native before a Run; before ProofStart; after a passed certificate; the commit
  proof: ['CCF-------', 'CCCF------', 'CCCCCCCF--', 'CCCCCCCF??'],   // begin_prepared_product; project; certify_final
  values: ['CCCCCF----'],                                           // complete_maxima
  abandoned: ['CCCCF-----', 'CCCCCCF---', 'CCCCCCC---'],             // maxima; aliases; bind_rows_view
  numeric: ['CCCCCCCCCC'],                                          // both checks passed, the case did not pass
  observable: ['CCCCCCCCF?'],                                       // Observables failed (it takes precedence over G5a)
  g5a: ['CCCCCCCCCF'],                                              // Observables passed, G5a failed
};
const stageRecordMatches = (pattern: string, record: string): boolean => [...pattern].every((m, i) => m === '?' ? 'CF'.includes(record[i]) : m === record[i]);
/** D37: whether an unavailable error kind can leave this stage record.
 * @internal Exported only for the RV79-N1 corpus-table test; not a public entry point. */
export function errorStageRecordAgrees(kind: string, stages: Obj): boolean {
  const record = STAGE_ORDER.map(k => STAGE_MARK[stages[k]]).join('');
  return NATIVE_STAGE_RECORDS[kind]?.some(pattern => stageRecordMatches(pattern, record)) === true;
}
/** R-D38 (4b) (B1; DESIGN_v2 §2; RR:8823; C1:103, C3:167), for an attempt whose native stage failed with no Run: a native
 * capture failure before any Run, beside a registered prepared source (C2 §3 registers a CaseSource once it is constructed
 * and its maps validate). G5 PRODUCT_ATTEMPT. The conjuncts are Rust's `d38_capture_before_run`: an unavailable `capture`
 * result; the stage record done(1, [failed]), preparation completed and every stage after native not_entered; the case
 * unavailable with `prepared_product_failure` naming this attempt and reason (source_unavailable, preparation), D4d's
 * mapping for a capture with no Run; and a source that binds this attempt and is the case's own.
 * Each conjunct is also enforced by a later check of `productAttempts` (D19, D4d's reason table, D37, the stage order,
 * the source association), with the same gate and code; it is kept so that (4b) reads as one predicate and does not
 * widen if one of those changes. The rest of (4b) holds for every receipt: the case's Run and the proof are null (the
 * association above), and no Run, Call or Group `source_refs` entry, Build or `execution_order` entry names the case or
 * its source (G3's execution order; G5 class 1's Call positions, Group sources and Builds). */
function d38CaptureBeforeRun(a: Obj, c: Obj, s: Obj | null): boolean {
  const stage = a.stages;
  return a.result.kind === 'unavailable' && a.result.error?.kind === 'capture'
    && stage.preparation === 'completed' && STAGE_ORDER.slice(2).every(k => stage[k] === 'not_entered')
    && c.status === 'unavailable' && c.reason?.cause?.kind === 'prepared_product_failure' && c.reason.cause.product_attempt_ref === a.id
    && c.reason.code === 'source_unavailable' && c.reason.phase === 'preparation'
    && s !== null && s.preparation?.attempt_ref === a.id && a.source_ref === c.source_ref;
}
/** @internal Exported only for the reader-logic D30 test (no faithful nonselected-Run base); not a public entry point. */
export function productAttempts(b: Obj, rows: Map<string, Obj[]>): void {
  // A later product association/check defect precedes product work consistency.
  // This preserves C3's prescribed within-gate order across every attempt.
  const workChecks: unknown[] = [];
  const fail = (ok: unknown) => need(ok, 'G5', 'PRODUCT_ATTEMPT_MISMATCH'), work = (ok: unknown) => { workChecks.push(ok); };
  const props = ['area', 'second_moment', 'polar_moment', 'section_modulus'].flatMap(p => [[p, 'lo'], [p, 'hi']]).concat([['radius', 'exact']]);
  // D4c (S06 §1; C3:165 "resolves once"): a prepared_product_failure cause names the case's own attempt.
  for (const c of b.cases) if (c.status === 'unavailable' && c.reason?.cause?.kind === 'prepared_product_failure') fail(c.product_attempt_ref !== null && c.product_attempt_ref === c.reason.cause.product_attempt_ref);
  for (const c of b.cases) {
    // D20 (C3:165): a selected case has its own C3 product attempt.
    if (c.status === 'selected') fail(c.product_attempt_ref !== null);
    if (c.product_attempt_ref === null) continue;
    const own = at(b.product_attempts, c.product_attempt_ref);
    // D19 (S06 §1, converse of D4c): an unavailable attempt is reported through prepared_product_failure
    // naming it; C2 cause branches apply only to outcomes without an actual C3 attempt.
    if (own.result.kind === 'unavailable') fail(c.status === 'unavailable' && c.reason?.cause?.kind === 'prepared_product_failure' && c.reason.cause.product_attempt_ref === own.id);
    // D19: a Ready attempt belongs to a selected case, or to an unavailable case with a receipt_failure cause.
    if (own.result.kind === 'ready') fail(c.status === 'selected' || (c.status === 'unavailable' && c.reason?.cause?.kind === 'receipt_failure'));
  }
  for (const a of b.product_attempts) {
    const c = at(b.cases, a.owner_ref.index), s = a.source_ref === null ? null : at(b.sources, a.source_ref), pm: Obj[] = a.preparation.members, old: Obj[] = a.operational.old, fresh: Obj[] = a.operational.new;
    const stage = a.stages, p = a.proof, ready = a.result.kind === 'ready';
    // D16: the ordinary reference resolves explicitly; a dangling one fails its C3 association check.
    const ordinary = at(b.ordinary_attempts, a.ordinary_attempt_ref);
    fail(a.ordinary_attempt_ref === c.ordinary.attempt_ref && a.material_basis_ref === ordinary.material_basis_ref);
    if (s) fail(s.owner.case_index === a.owner_ref.index && s.material_basis_ref === a.material_basis_ref && s.preparation?.attempt_ref === a.id && c.source_ref === a.source_ref);
    fail((a.run_ref === null) === !c.run); if (c.run) fail(c.run.id === a.run_ref && c.run.origin.source_ref === a.source_ref && same(c.run.origin.owner_ref, a.owner_ref));
    // R-D38 (B1; DESIGN_v2 §2): a native stage not entered has no Run (rule 2), a completed one has a Run (rule 3, its
    // terminal checked below), and a failed one has a non-selected Run (4a) or is D38's capture before any Run (4b).
    fail(stage.native === 'not_entered' ? a.run_ref === null : a.run_ref !== null || (stage.native === 'failed' && d38CaptureBeforeRun(a, c, s)));
    // D1 captured_prefix association (F1:97, 130-131): no source, no Run, an unavailable result.
    if (a.operational.old_coverage === 'captured_prefix') fail(a.source_ref === null && a.run_ref === null && a.result.kind === 'unavailable');
    if (a.run_ref !== null) fail(stage.native === (c.run.kernel_terminal.kind === 'selected' ? 'completed' : 'failed'));
    if (s) fail(stage.preparation === 'completed' && pm.length === s.id_maps.members.length && fresh.length === pm.length && a.operational.old_coverage === 'complete');
    if (stage.native !== 'not_entered') fail(stage.preparation === 'completed');
    fail((p === null) === (stage.proof_start === 'not_entered'));
    if (stage.proof_start !== 'not_entered') fail(stage.native === 'completed' && p !== null);
    if (p === null) fail(stage.proof_start === 'not_entered' && ['projection', 'maxima', 'values', 'aliases', 'certificate', 'observables', 'g5a'].every(k => stage[k] === 'not_entered'));
    // P2 (C3:196-201; retained_product.rs `PreparedCase::project_candidate`): only returned transitions are retained. Preparation completed iff
    // a source was built; observables and G5a are entered together, only after the certificate ended.
    fail((stage.preparation === 'completed') === (a.source_ref !== null));
    fail((stage.observables === 'not_entered') === (stage.g5a === 'not_entered'));
    if (stage.observables !== 'not_entered') fail(['completed', 'failed'].includes(stage.certificate));
    for (let j = 0; j < pm.length; j++) {
      const m = pm[j], prepared = m.result.kind === 'prepared';
      if (!prepared) fail(j === pm.length - 1 && j >= fresh.length && stage.preparation === 'failed');
      fail(same(m.conversions.map((x: Obj) => [x.property, x.endpoint]), props.slice(0, m.conversions.length)) && m.conversions.every((x: Obj) => conversionKind(x.outcome)));
      if (count(m.work.conversions) !== null) work(count(m.work.conversions) === BigInt(m.conversions.length));
      if (prepared) { fail(m.conversions.length === 9); m.conversions.forEach((x: Obj, k: number) => fail(x.outcome.kind === 'normal' && x.outcome.value === m.result.section[k < 8 ? Math.floor(k / 2) : 4] && decodeBinary64(x.outcome.value) >= MIN_NORMAL)); }
      if (j < fresh.length) fail(prepared);
    }
    if (p) {
      const lanes: Obj[] = p.lanes; fail(lanes.length <= 2 && same(lanes.map(l => l.law), ['admitted_k', 'annular_source'].slice(0, lanes.length)));
      lanes.forEach((l, i) => { fail((l.error === null) === (l.state === 'completed') && (l.state !== 'failed' || i === lanes.length - 1)); const calls = count(l.work.correction.calls); work(calls === null || calls <= 1n); work(l.work.data_capacity === l.work.view.data_capacity); });
      if (stage.proof_start === 'completed') fail(lanes.length === 2 && lanes.every(l => l.state === 'completed'));
      if (stage.projection !== 'not_entered') fail(stage.proof_start === 'completed');
      for (const [later, earlier] of [['maxima', 'projection'], ['values', 'maxima'], ['aliases', 'values'], ['certificate', 'aliases']]) if (stage[later] !== 'not_entered') fail(stage[earlier] === 'completed');
      for (const key of ['certificate', 'observables', 'g5a']) {
        const check = p.checks[key]; fail((stage[key] === 'not_entered') === (check.kind === 'not_entered'));
        if (check.kind !== 'not_entered') fail(stage[key] === (check.kind === 'passed' ? 'completed' : 'failed'));
      }
      if (stage.observables !== 'not_entered' || stage.g5a !== 'not_entered') fail(stage.aliases === 'completed');
      const outcomes: Obj[] = p.projection_outcomes; work(count(p.projection_conversions) === null || count(p.projection_conversions) === BigInt(outcomes.length));
      fail(outcomes.every(x => conversionKind(x.outcome)));
      const caseRows = rows.get(c.basis_ref.ref_id)!;
      for (const x of outcomes) if (ready) { const v = x.outcome; fail(v.kind !== 'overflow'); const n = v.kind === 'underflow' ? 0 : decodeBinary64(v.value); fail(binary64Bits(caseRows[x.row_index].value) === binary64Bits(n === 0 ? 0 : n)); }
      // abandon_values also merges a builder abandoned by failed maxima.
      if (p.completion.kind === 'merged') fail(stage.projection === 'completed');
      if (p.completion.kind === 'separate_failure') fail(stage.values === 'failed');
      if (stage.values === 'completed') fail(p.completion.kind === 'merged');
      // P6 (C3:253-257; retained_product.rs `PreparedCase::project_candidate`): a values failure keeps its own completion; abandonment after
      // maxima/aliases/bind-rows or any certificate entry merges; nothing before projection completes.
      if (stage.values === 'failed') fail(p.completion.kind === 'separate_failure');
      else if (stage.certificate !== 'not_entered' || stage.maxima === 'failed' || stage.aliases !== 'not_entered') fail(p.completion.kind === 'merged');
      else if (stage.projection !== 'completed') fail(p.completion.kind === 'not_entered');
      if (ready) { const expected = caseRows.flatMap((r, i) => HULL_EXCLUDED.has(r.kind) ? [] : [i]); fail(same(outcomes.map(x => x.row_index), expected)); }
      // I57 §3 custody/stage table. Non-null coverage needs this attempt's own source,
      // its selected native Run, both lanes in order, every pre-certificate stage
      // completed and the certificate entered. A completed certificate, a passed
      // G5a or Ready needs non-null coverage. Coverage never implies certificate success.
      const cov = p.summary_coverage;
      if (cov !== null) {
        fail(s && a.source_ref !== null && a.run_ref !== null && c.run?.kernel_terminal.kind === 'selected' && c.run.id === a.run_ref && c.run.origin.source_ref === a.source_ref && c.source_ref === a.source_ref && same(c.run.origin.owner_ref, a.owner_ref));
        fail(lanes.length === 2 && same(lanes.map(l => l.law), ['admitted_k', 'annular_source']) && lanes.every(l => l.state === 'completed'));
        fail(['proof_start', 'projection', 'maxima', 'values', 'aliases'].every(k => stage[k] === 'completed') && stage.certificate !== 'not_entered');
      }
      if (stage.certificate === 'completed' || p.checks.certificate.kind === 'passed' || stage.g5a === 'completed' || p.checks.g5a.kind === 'passed' || ready) fail(cov !== null);
    }
    // R1-R3 (ruling 06d) join the deferred C3 work equations, after association and typed checks.
    for (const ok of accountingRules(a)) work(ok);
    if (ready) {
      fail(s && c.run?.kernel_terminal.kind === 'selected' && Object.values(stage).every(v => v === 'completed') && p && Object.values(p.checks).every((v: any) => v.kind === 'passed'));
      fail(pm.length === old.length && pm.length === fresh.length && pm.every(m => m.result.kind === 'prepared') && fresh.every(m => m.result.kind === 'ready') && a.operational.old_coverage === 'complete');
      work(exactWork(p) && [...pm, ...old, ...fresh].every(m => exactWork(m.work)) && a.adapter.fault === null && !a.overlay_work.lost && !a.g5a_work.lost);
    }
    if (c.status === 'selected') fail(ready);
    if (c.status === 'unavailable' && c.reason.cause.kind === 'prepared_product_failure') {
      fail(c.reason.cause.product_attempt_ref === a.id && !ready);
      const error = a.result.error, run = c.run; let expected: string[];
      if (error.kind === 'preparation') { fail(stage.preparation === 'failed' && !run); expected = ['source_unavailable', 'preparation']; }
      else if (error.kind === 'native') { fail(run && run.kernel_terminal.kind !== 'selected' && error.run_ref === run.id); expected = ['kernel_' + run.kernel_terminal.kind, 'kernel']; }
      else if (error.kind === 'capture' && !run) expected = ['source_unavailable', 'preparation'];
      else if (error.kind === 'capture' && run.kernel_terminal.kind !== 'selected') expected = ['kernel_' + run.kernel_terminal.kind, 'kernel'];
      else { fail(run?.kernel_terminal.kind === 'selected'); expected = ['facade_certificate', 'facade']; }
      fail(same([c.reason.code, c.reason.phase], expected));
      if (error.kind === 'values') fail(stage.values === 'failed');
      if (error.kind === 'observable') fail(p?.checks.observables.kind === 'failed' && same(p.checks.observables.error, error));
      if (error.kind === 'g5a') fail(p?.checks.g5a.kind === 'failed' && same(p.checks.g5a.error, error));
      if (error.kind === 'proof') fail(stage.proof_start === 'failed' || (p?.checks.certificate.kind === 'failed' && same(p.checks.certificate.error, error)));
      if (error.kind === 'proof' && stage.proof_start === 'failed' && error.cause.kind === 'native_source') {
        fail(p?.lanes.at(-1)?.state === 'failed' && same(p.lanes.at(-1).error, error.cause.cause));
      }
      if (error.kind === 'g5a' && s) {
        const cause = error.cause;
        if (cause.kind === 'sanity') fail(s.body_membership.some((body: Obj) => body.body === cause.body));
        if (cause.kind === 'lower') fail(s.id_maps.members.some((m: Obj) => m.kernel_member === cause.member));
        if (cause.kind === 'operational') fail(cause.member_index < s.id_maps.members.length);
        if (cause.kind === 'zero') fail(cause.row < rows.get(c.basis_ref.ref_id)!.length);
      }
    }
  }
  // C3:304: typed check/result pairing (P9) follows every attempt's association.
  const pipeline = ['preparation', 'native', 'proof_start', 'projection', 'maxima', 'values', 'aliases', 'certificate'];
  const allowed: Record<string, string[]> = { preparation: ['preparation', 'capture'], native: ['native', 'capture'], proof_start: ['proof'], projection: ['proof'], maxima: ['abandoned'], values: ['values'], aliases: ['abandoned'], certificate: ['proof'] };
  for (const a of b.product_attempts) {
    if (a.proof) for (const [key, kind] of [['certificate', 'proof'], ['observables', 'observable'], ['g5a', 'g5a']]) if (a.proof.checks[key].kind === 'failed') fail(a.proof.checks[key].error.kind === kind);
    if (a.result.kind !== 'unavailable') continue;
    const first = pipeline.find(k => a.stages[k] === 'failed');
    if (first) fail(allowed[first].includes(a.result.error.kind));
    // D37 (D35 widened; RV79-X1): the error kind and the stage record agree in both directions.
    fail(errorStageRecordAgrees(a.result.error.kind, a.stages));
  }
  // C3 work equations, deferred until every attempt's association and typed checks.
  for (const ok of workChecks) need(ok, 'G5', 'WORK_MISMATCH');
}

function normalized(r: Obj): number { return r.unit === 'mm' ? r.value / 1000 : ['kN', 'kN*m'].includes(r.unit) ? r.value * 1000 : r.unit === 'MPa' ? r.value * 1_000_000 : r.value; }
function rowKind(r: Obj): string {
  if (NONQUANTITY.has(r.kind)) return 'non_quantity'; if (INPUT.has(r.kind)) return 'input_derived';
  if (['global_nodal_displacement_x', 'global_nodal_displacement_y', 'global_nodal_displacement_z', 'displacement_magnitude'].includes(r.kind) && ['m', 'mm'].includes(r.unit)) return 'translation';
  if (['global_nodal_rotation_x', 'global_nodal_rotation_y', 'global_nodal_rotation_z'].includes(r.kind) && r.unit === 'rad') return 'rotation';
  if (FORCE.has(r.kind) && ['N', 'kN'].includes(r.unit)) return 'force';
  if (MOMENT.has(r.kind) && ['N*m', 'kN*m'].includes(r.unit)) return 'moment';
  if (['support_reaction_component_v2', 'pipe_wall_endpoint_action_v2'].includes(r.kind)) return ['N', 'kN'].includes(r.unit) ? 'force' : ['N*m', 'kN*m'].includes(r.unit) ? 'moment' : 'not_covered';
  if (STRESS.has(r.kind) && ['Pa', 'MPa'].includes(r.unit)) return 'stress'; return 'not_covered';
}
function rowOwner(r: Obj, s: Obj): { body: number | null; member: Obj | null } {
  const member = s.id_maps.members.find((m: Obj) => m.id === r.entity_ref) ?? null;
  const node = s.id_maps.nodes.find((m: Obj) => m.id === r.entity_ref)?.kernel_node ?? member?.node_i ?? s.id_maps.support_ids.find((m: Obj) => m.id === r.entity_ref)?.node;
  return { body: s.body_membership.find((b: Obj) => b.nodes.includes(node))?.body ?? null, member };
}
function component(r: Obj): string | null {
  if (r.kind.startsWith('global_nodal_displacement_')) return 'U' + r.kind.at(-1).toUpperCase();
  if (r.kind.startsWith('global_nodal_rotation_')) return 'R' + r.kind.at(-1).toUpperCase(); return null;
}
function extent(points: number[][]): number { const d = [0, 1, 2].map(j => Math.max(...points.map(p => p[j])) - Math.min(...points.map(p => p[j]))); return Math.sqrt(((d[0] * d[0]) + (d[1] * d[1])) + (d[2] * d[2])); }
function couple(s: number[], length: number): number[] { const [tr, ro, fo, mo] = s; return length === 0 ? [...s] : [Math.max(tr, length * ro), Math.max(ro, tr / length), Math.max(fo, mo / length), Math.max(mo, length * fo)]; }
/** verify.rs e_hat: ê_fo = max(E_fo, fl(E_mo/L)), ê_mo = max(E_mo, fl(L·E_fo)); L = 0 keeps E.
 * @internal Exported only for reader-logic unit tests; not a public entry point. */
export function eHat(force: number, moment: number, extent: number): [number, number] {
  return extent === 0 ? [force, moment] : [Math.max(force, moment / extent), Math.max(moment, extent * force)];
}
/** verify.rs phi_512: Φ = fl↑(2^-438·ê). Take the nearest product, then adaptive::next_up
 * (zero steps to the least subnormal) when scaling back by 2^438 is below ê.
 * @internal Exported only for reader-logic unit tests; not a public entry point. */
export function phi512(hat: number): number {
  const nearest = hat * numberFromWord(0x2490000000000000n), back = numberFromWord(0x5b50000000000000n);
  if (!(nearest * back < hat)) return nearest;
  return nearest === 0 ? numberFromWord(1n) : numberFromWord(BigInt('0x' + binary64Bits(nearest)) + 1n);
}
type NumericCase = { c: Obj; s: Obj; rows: Obj[]; values: { kind: string; body: number | null; member: Obj | null; n: number; input: boolean }[]; lengths: number[]; original: number[][]; scales: number[][]; hats: number[][] };
function numericalCases(b: Obj, rows: Map<string, Obj[]>): NumericCase[] {
  return b.cases.filter((c: Obj) => c.status === 'selected').map((c: Obj) => {
    const s = at(b.sources, c.source_ref), rs = rows.get(c.basis_ref.ref_id)!, prescribed = s.constraints.map((x: Obj) => ({ node_id: s.id_maps.nodes[x.dof.node]?.id, component: x.dof.component }));
    const values = rs.map(r => ({ kind: rowKind(r), ...rowOwner(r, s), n: normalized(r), input: component(r) !== null && prescribed.some((x: Obj) => x.node_id === r.entity_ref && x.component === component(r)) }));
    const lengths = s.body_membership.map((body: Obj) => extent(body.nodes.map((i: number) => s.id_maps.nodes[i].coordinates.map(decodeBinary64))));
    const original = s.body_membership.map((body: Obj, i: number) => { const max = [0, 0, 0, 0]; values.forEach(v => { const k = KINDS.indexOf(v.kind); if (v.body === body.body && k >= 0 && !v.input) max[k] = Math.max(max[k], Math.abs(v.n)); }); return couple(max, lengths[i]); });
    const hats = c.selection.resolution_scale.map((e: Obj, i: number) => eHat(decodeBinary64(e.force), decodeBinary64(e.moment), lengths[i]));
    return { c, s, rows: rs, values, lengths, original, scales: original.map((x: number[]) => [...x]), hats };
  });
}
/** source.rs/recover::layout order. Only constrained displacement/rotation rows are input-derived. */
function sourceLayout(nodeCount: number, members: Obj[], stations: Obj[], springs: Obj[], constraints: Obj[], supports: Obj[], fixed: (key: string) => boolean, bodyOf: (node: number) => number): Obj[] {
  const layout: Obj[] = [];
  const add = (quantity: Obj, kind: string, node: number, input = false) => layout.push({ index: layout.length, quantity, kind, body: bodyOf(node), input_derived: input });
  for (let node = 0; node < nodeCount; node++) COMPONENTS.forEach((component, j) => add({ tag: 'displacement', dof: { node, component } }, KINDS[j < 3 ? 0 : 1], node, fixed(node + ':' + component)));
  for (let node = 0; node < nodeCount; node++) add({ tag: 'displacement_magnitude', node }, 'translation', node);
  for (const m of members) for (const end of ['i', 'j']) COMPONENTS.forEach((component, j) => add({ tag: 'end_action', member: m.kernel_member, end, component }, j < 3 ? 'force' : 'moment', m.node_i));
  for (const st of stations) COMPONENTS.forEach((component, j) => add({ tag: 'station_action', station: st.id, component }, j < 3 ? 'force' : 'moment', members[st.member]?.node_i));
  for (const spring of springs) add({ tag: 'spring_action', spring: spring.kernel_spring, component: spring.component }, COMPONENTS.indexOf(spring.component) < 3 ? 'force' : 'moment', spring.node);
  for (const c of constraints) add({ tag: 'reaction', dof: c.dof }, COMPONENTS.indexOf(c.dof.component) < 3 ? 'force' : 'moment', c.dof.node);
  for (const support of supports) { add({ tag: 'support_force_magnitude', support: support.id }, 'force', support.node); add({ tag: 'support_moment_magnitude', support: support.id }, 'moment', support.node); }
  return layout;
}
type CoverageFacts = { present: boolean[][]; nonInput: boolean[][]; lengths: number[]; free: boolean[]; loaded: boolean[] };
/** I57 §2/§4 public source facts per body, from the bound source maps only (never final rows):
 * canonical layout presence, non-input presence, native extent L, free DOFs and
 * individually nonzero original nodal terms at free DOFs (cancellation preserved). */
function coverageFacts(s: Obj): CoverageFacts {
  const fail = (ok: unknown) => need(ok, 'G5a', 'SCALE_MISMATCH');
  const bodies: Obj[] = s.body_membership, maps = s.id_maps;
  fail(bodies.length >= 1 && same(bodies.map(x => x.body), sequence(bodies.length)));
  const bodyOf = (node: number): number => { const hit = bodies.find(x => x.nodes.includes(node)); fail(hit); return hit!.body; };
  // C3 prescriptions are exact +0, so input-derived rows carry D = false.
  const fixed = new Set<string>(s.constraints.map((c: Obj) => c.dof.node + ':' + c.dof.component));
  fail(fixed.size === s.constraints.length && s.constraints.every((c: Obj) => c.value === ZERO));
  fail(same(s.layout, sourceLayout(maps.nodes.length, maps.members, s.stations, maps.springs, s.constraints, s.supports, key => fixed.has(key), bodyOf)));
  const present = bodies.map(() => [false, false, false, false]), nonInput = bodies.map(() => [false, false, false, false]);
  for (const row of s.layout) { const k = KINDS.indexOf(row.kind); present[row.body][k] = true; if (!row.input_derived) nonInput[row.body][k] = true; }
  const lengths = bodies.map(body => extent(body.nodes.map((i: number) => maps.nodes[i].coordinates.map(decodeBinary64))));
  fail(lengths.every(L => Number.isFinite(L) && L >= 0));
  const free = bodies.map(body => body.nodes.some((n: number) => COMPONENTS.some(c => !fixed.has(n + ':' + c))));
  const loaded = bodies.map(() => false);
  for (const t of s.nodal_terms) if (!fixed.has(t.dof.node + ':' + t.dof.component) && decodeBinary64(t.value) !== 0) loaded[bodyOf(t.dof.node)] = true;
  return { present, nonInput, lengths, free, loaded };
}
/** I57 §4 Boolean feasibility: some private A over non-input-present kinds (D = false)
 * reproduces the attested stop bits under L coupling and any positive floor.
 * @internal Exported only for reader-logic unit tests; not a public entry point. */
export function stopFeasible(stop: boolean[], present: boolean[], nonInput: boolean[], extentNonzero: boolean, floors: boolean[][]): boolean {
  for (let mask = 0; mask < 16; mask++) {
    const A = KINDS.map((_, k) => ((mask >> k) & 1) === 1);
    if (A.some((v, k) => v && !nonInput[k])) continue;
    for (const [force, moment] of floors) {
      const positive = extentNonzero ? [A[0] || A[1], A[0] || A[1], A[2] || A[3], A[2] || A[3]] : [...A];
      positive[2] ||= force; positive[3] ||= moment;
      if (same(KINDS.map((_, k) => present[k] && (positive[k] || A[k])), stop)) return true;
    }
  }
  return false;
}
const canonical = (bits: string, limit: number) => { const v = decodeBinary64(bits); return bits !== '8000000000000000' && v >= 0 && v <= limit; };
/** Exact verification-record relations (shared 05a rule): one bound entry per body in
 * body order, non-null iff has_data; the record's theta is +0 without data; data_blocks
 * is zero iff no body has data, otherwise at least the true-body count. */
function recordCoverage(cov: Obj[], record: Obj): void {
  const fail = (ok: unknown) => need(ok, 'G5a', 'SCALE_MISMATCH');
  const withData = cov.filter(e => e.has_data).length;
  fail(Number.isSafeInteger(record.data_blocks) && (record.data_blocks === 0) === (withData === 0) && record.data_blocks >= withData);
  // Reader parity rule: the record's resolution and theta list bodies 0..n-1 in order.
  const ids = cov.map(e => e.body);
  fail(same(record.resolution.map((v: Obj) => v.body), ids) && same(record.theta.map((v: Obj) => v.body), ids));
  fail(same(record.bound.map((v: Obj) => v.body), ids) && cov.every((e, i) => (record.bound[i].value !== null) === e.has_data));
  for (const e of cov) if (!e.has_data) fail(record.theta.find((v: Obj) => v.body === e.body)?.value === ZERO);
}
/** Direct data facts: no free DOF forces false; a nonzero original free-DOF term forces true. */
function dataCoverage(e: Obj, facts: CoverageFacts, bi: number): void {
  if (!facts.free[bi]) need(!e.has_data, 'G5a', 'SCALE_MISMATCH');
  if (facts.loaded[bi]) need(e.has_data, 'G5a', 'SCALE_MISMATCH');
}
/** I57 §4 G5a for a selected case, after the existing encodings/ranges and p/P/floor rules. */
function selectedCoverage(x: NumericCase, b: Obj): void {
  const fail = (ok: unknown) => need(ok, 'G5a', 'SCALE_MISMATCH');
  const sel = x.c.selection, cov: Obj[] = at(b.product_attempts, x.c.product_attempt_ref, 'G5a', 'SCALE_MISMATCH').proof?.summary_coverage;
  const facts = coverageFacts(x.s); fail(Array.isArray(cov) && cov.length === facts.lengths.length);
  const stops: [number, string][] = [], estimates: [number, string][] = [], charges: [number, string][] = [];
  for (let bi = 0; bi < cov.length; bi++) {
    const e = cov[bi], coupled = facts.lengths[bi] !== 0;
    const floors = sel.floor === null ? [[false, false]] : [[decodeBinary64(sel.floor[bi].force) > 0, decodeBinary64(sel.floor[bi].moment) > 0]];
    fail(stopFeasible(e.stop, facts.present[bi], facts.nonInput[bi], coupled, floors));
    const r = sel.resolution_scale[bi]; let hats = [decodeBinary64(r.force) > 0, decodeBinary64(r.moment) > 0];
    if (coupled) hats = [hats[0] || hats[1], hats[0] || hats[1]];
    const estimate = [facts.present[bi][2] && hats[0], facts.present[bi][3] && hats[1]];
    const charge = sel.precision === 512 ? [e.stop[2], e.stop[3]] : estimate;
    KINDS.forEach((kind, k) => { if (e.stop[k]) stops.push([bi, kind]); });
    ['force', 'moment'].forEach((kind, k) => { if (estimate[k]) estimates.push([bi, kind]); if (charge[k]) charges.push([bi, kind]); });
  }
  // Exact rosters: one entry per true bit and none per false bit, whatever the value.
  for (const [key, expected, limit] of [['stop_rule', stops, 2 ** -64], ['verification_estimate', estimates, .25], ['verification_charge', charges, 1]] as const) {
    fail(same(sel[key].map((v: Obj) => [v.body, v.kind]), expected) && sel[key].every((v: Obj) => canonical(v.value, limit)));
  }
  fail(sel.resolution_scale.every((v: Obj) => canonical(v.force, Infinity) && canonical(v.moment, Infinity)) && sel.theta.every((v: Obj) => canonical(v.value, .5)));
  fail(same(sel.certified_bound.map((v: Obj) => v.body), cov.filter(e => e.has_data).map(e => e.body)) && sel.certified_bound.every((v: Obj) => decodeBinary64(v.value) > 0));
  const last = x.c.run.attempts.at(-1);
  recordCoverage(cov, at(x.c.run.records, last.verification.record, 'G5a', 'SCALE_MISMATCH').verification);
  cov.forEach((e, bi) => dataCoverage(e, facts, bi));
}
/** Non-selected attempts retaining a complete roster (05a): canonical layout, feasibility,
 * record relations and direct data facts; no Selection rosters are applied or invented.
 * At native p512 floor positivity comes from the Run's verification record: Φ > 0 iff ê(E, L) > 0. */
function unselectedCoverage(b: Obj): void {
  for (const a of b.product_attempts) {
    const c = b.cases[a.owner_ref.index], cov = a.proof?.summary_coverage;
    if (c.status === 'selected' || !Array.isArray(cov)) continue;
    const facts = coverageFacts(b.sources[a.source_ref]), last = c.run.attempts.at(-1);
    need(cov.length === facts.lengths.length, 'G5a', 'SCALE_MISMATCH');
    const record = at(c.run.records, last.verification.record, 'G5a', 'SCALE_MISMATCH').verification;
    cov.forEach((e: Obj, bi: number) => {
      let floor = [false, false];
      if (last.precision === 512) {
        const r = record.resolution.find((v: Obj) => v.body === e.body); need(r, 'G5a', 'SCALE_MISMATCH');
        floor = eHat(decodeBinary64(r.force), decodeBinary64(r.moment), facts.lengths[bi]).map(v => v > 0);
      }
      need(stopFeasible(e.stop, facts.present[bi], facts.nonInput[bi], facts.lengths[bi] !== 0, [floor]), 'G5a', 'SCALE_MISMATCH');
    });
    recordCoverage(cov, record);
    cov.forEach((e: Obj, bi: number) => dataCoverage(e, facts, bi));
  }
}
function numericSummaries(cases: NumericCase[], b: Obj): void {
  const fail = (ok: unknown) => need(ok, 'G5a', 'SCALE_MISMATCH');
  for (const x of cases) {
    const sel = x.c.selection, bodies: Obj[] = x.s.body_membership, ids = bodies.map(b => b.body);
    fail(same(ids, sequence(bodies.length)) && sel.verification_precision === 2 * sel.precision && sel.floor_ratio === '3dd0000000000000' && decodeBinary64(sel.pivot_margin_min) > 0 && decodeBinary64(sel.rcond) > 0);
    for (const [key, kinds, limit] of [['stop_rule', KINDS, 2 ** -64], ['verification_estimate', ['force', 'moment'], .25], ['verification_charge', ['force', 'moment'], 1]] as const) {
      const items: Obj[] = sel[key]; fail(unique(items.map(v => v.body + ':' + v.kind)) && items.every(v => ids.includes(v.body) && (kinds as readonly string[]).includes(v.kind) && decodeBinary64(v.value) <= limit));
      fail(items.every((v, i) => !i || v.body > items[i - 1].body || (v.body === items[i - 1].body && KINDS.indexOf(v.kind) > KINDS.indexOf(items[i - 1].kind))));
    }
    for (const key of ['resolution_scale', 'theta', 'body_scales']) fail(same(sel[key].map((e: Obj) => e.body), ids));
    fail(sel.theta.every((v: Obj) => decodeBinary64(v.value) <= .5));
    fail(unique(sel.certified_bound.map((v: Obj) => v.body)) && sel.certified_bound.every((v: Obj) => ids.includes(v.body) && decodeBinary64(v.value) > 0));
    fail((sel.floor !== null) === (sel.precision === 512)); if (sel.floor !== null) fail(same(sel.floor.map((v: Obj) => v.body), ids));
    selectedCoverage(x, b);
    // G5a resolution-test prerequisites (reader parity): non-empty bodies, finite normalized rows.
    fail(bodies.every(body => body.nodes.length > 0) && x.values.every(v => Number.isFinite(v.n)));
    for (let bi = 0; bi < bodies.length; bi++) {
      fail(x.lengths[bi] >= 0 && Number.isFinite(x.lengths[bi]) && [...x.original[bi], ...x.hats[bi]].every(Number.isFinite));
      const e = [decodeBinary64(sel.resolution_scale[bi].force), decodeBinary64(sel.resolution_scale[bi].moment)], upper = x.hats[bi].map(v => v * decodeBinary64('3ff0000000001000'));
      fail(upper.every(Number.isFinite) && upper[0] >= x.original[bi][2] && upper[1] >= x.original[bi][3]);
      for (const v of x.values) if (v.body === bi && ['force', 'moment'].includes(v.kind) && e[v.kind === 'force' ? 0 : 1] === 0) fail(binary64Bits(v.n) === ZERO);
      for (const section of x.s.section_terms) {
        const member = x.s.id_maps.members.find((m: Obj) => m.kernel_member === section.member); fail(member);
        if (!bodies[bi].members.includes(member.kernel_member)) continue;
        for (let kind = 0; kind < 2; kind++) {
          const norms = [member.node_i, member.node_j].map(node => {
            const id = x.s.id_maps.nodes[node].id;
            const parts = ['x', 'y', 'z'].map(axis => { const hits = x.rows.filter(r => r.entity_ref === id && r.kind === 'global_nodal_' + (kind === 0 ? 'displacement_' : 'rotation_') + axis); fail(hits.length === 1); return Math.abs(normalized(hits[0])); });
            return (parts[0] + parts[1]) + parts[2];
          });
          const n = norms[0] + norms[1], threshold = (2 ** -59) * x.original[bi][kind];
          const lower = n <= threshold ? 0 : decodeBinary64(section[kind === 0 ? 'axial_stiffness' : 'torsional_stiffness']) * (n - (2 ** -60) * x.original[bi][kind]);
          fail(Number.isFinite(n) && Number.isFinite(lower) && upper[kind] >= lower);
        }
      }
    }
  }
}
function numericalScales(cases: NumericCase[], source: Obj): void {
  const fail = (ok: unknown, code = 'SCALE_MISMATCH') => need(ok, 'G5b', code);
  for (const x of cases) {
    const sel = x.c.selection;
    for (let bi = 0; bi < x.scales.length; bi++) {
      if (sel.precision === 512) {
        // C1 G5b: at native p512 the floor is exactly Φ = phi_512(ê(E, L)) per body.
        fail(same([sel.floor[bi].force, sel.floor[bi].moment], x.hats[bi].map(v => binary64Bits(phi512(v)))));
        x.scales[bi][2] = Math.max(x.scales[bi][2], decodeBinary64(sel.floor[bi].force)); x.scales[bi][3] = Math.max(x.scales[bi][3], decodeBinary64(sel.floor[bi].moment));
      }
      fail(x.scales[bi].every(Number.isFinite) && same(x.scales[bi].map(binary64Bits), KINDS.map(k => sel.body_scales[bi][k])));
    }
    fail(sel.section_terms.length === x.s.section_terms.length, 'SECTION_MISMATCH');
    x.s.section_terms.forEach((section: Obj, i: number) => {
      const m = x.s.id_maps.members.find((v: Obj) => v.kernel_member === section.member), actual = sel.section_terms[i];
      // D18 (G5b section truth; C1 G5b row): each echoed term equals the source's term and is positive, else
      // SECTION. Native rejects nonpositive inputs (retained_product.rs `evaluate_operational`), range-checks EA/L and GJ/L (retained_product.rs `ScalarWork::operation`) and
      // needs area, Z > 0 (endpoint_maximum.rs `endpoint_maximum`). The term-versus-member-map identity is G8's binding.
      fail(m && actual.member_id === m.id && ['area', 'section_modulus', 'length', 'axial_stiffness', 'torsional_stiffness'].every(k => actual[k] === section[k] && decodeBinary64(section[k]) > 0), 'SECTION_MISMATCH');
    });
    // Ensure every prospective stress scale is finite before any G5c list checks.
    for (let i = 0; i < x.rows.length; i++) if (x.values[i].kind === 'stress' && x.values[i].member && x.values[i].body !== null) stressScale(x, i, source);
  }
}
function stressScale(x: NumericCase, i: number, source: Obj): number {
  const fail = (ok: unknown, code = 'SECTION_MISMATCH') => need(ok, 'G5b', code);
  const v = x.values[i], r = x.rows[i], section = x.s.section_terms.find((s: Obj) => s.member === v.member?.kernel_member); fail(section && v.body !== null);
  let k = r.kind === 'pipe_elastic_normal_stress_maximum_v2' ? decodeBinary64('4006a09e667f3bcd') : r.kind === 'open_formula_stress_summary' ? 4 : 1;
  if (r.kind === 'component_equal_factor_intensified_bending_stress_v1') {
    const measures = source.contract_evidence?.preview_cases?.find((c: Obj) => c.load_case_id === x.c.basis_ref.ref_id)?.intensified_measures ?? [];
    const m = measures.find((m: Obj) => m.result_id === r.id); fail(m && Number.isFinite(m.sif) && m.sif > 0); k = upwardProduct(decodeBinary64('3ff6a09e667f3bcd'), m.sif);
  }
  const sc = x.scales[v.body!], scale = (sc[2] / decodeBinary64(section.area)) + (k * (sc[3] / decodeBinary64(section.section_modulus)));
  fail(Number.isFinite(scale) && scale >= 0, 'SCALE_MISMATCH'); return scale;
}
function classifications(cases: NumericCase[], source: Obj): RowClassification[] {
  const result: RowClassification[] = [];
  for (const x of cases) {
    const actual = x.s.constraints.map((c: Obj) => ({ node_id: x.s.id_maps.nodes[c.dof.node]?.id, component: c.dof.component }));
    need(same(x.c.selection.input_derived_dofs, actual), 'G5c', 'INPUT_DOF_MISMATCH');
    const absolute: Obj[] = [], uncovered: string[] = [];
    for (let i = 0; i < x.rows.length; i++) {
      const r = x.rows[i], v = x.values[i]; let scale: number | null = null, bound: number | null = null, cls: AccuracyClass;
      if (v.kind === 'non_quantity') cls = 'non_quantity';
      else if (v.kind === 'input_derived' || v.input) { cls = 'input_derived'; if (v.input) need(binary64Bits(v.n) === ZERO, 'G5c', 'INPUT_DOF_MISMATCH'); }
      else if (v.kind === 'not_covered' || v.body === null || (v.kind === 'stress' && !v.member)) { cls = 'not_covered'; uncovered.push(r.id); }
      else {
        scale = v.kind === 'stress' ? stressScale(x, i, source) : x.scales[v.body][KINDS.indexOf(v.kind)];
        need(Number.isFinite(scale) && Number.isFinite(v.n), 'G5c', 'CLASSIFICATION_MISMATCH');
        if (scale >= 2 ** -988 && !(Math.abs(v.n) < (2 ** -34) * scale)) cls = 'relative_verified';
        else { cls = 'absolute_verified'; bound = absoluteBound(v.n, scale); absolute.push({ result_id: r.id, bound: binary64Bits(bound) }); }
      }
      result.push({ result_id: r.id, basis_ref: r.basis_ref, normalized_bits: binary64Bits(v.n), scale_bits: scale === null ? null : binary64Bits(scale), bound_bits: bound === null ? null : binary64Bits(bound), class: cls });
    }
    need(same(x.c.selection.absolute_verified, absolute) && same(x.c.selection.not_covered, uncovered), 'G5c', 'CLASSIFICATION_MISMATCH');
  }
  return result;
}

/** source.rs PrimitiveSource::{encoding,stiffness_encoding}: deterministic bytes,
 * not a solve, ledger replay, or authentication of the producing process. */
async function nativeSourceHashes(s: Obj): Promise<{ source: string; stiffness: string }> {
  const maps = s.id_maps;
  const encode = (withValues: boolean): Uint8Array => {
    const bytes: number[] = [...new TextEncoder().encode(withValues ? 'K4SRC\x01' : 'K4STF\x01')];
    const u32 = (v: number) => {
      need(Number.isSafeInteger(v) && v >= 0 && v <= 0xffffffff, 'G8', 'PREPARATION_MISMATCH');
      // Count arithmetic stays exact before this explicitly bounded u32 encoding.
      for (let j = 0; j < 4; j++) bytes.push(Math.floor(v / (2 ** (8 * j))) % 256);
    };
    const bits = (v: string) => {
      const word = BigInt('0x' + v);
      for (let j = 0n; j < 8n; j++) bytes.push(Number((word >> (8n * j)) & 255n));
    };
    const dof = (v: Obj) => {
      u32(v.node); const component = COMPONENTS.indexOf(v.component);
      need(component >= 0, 'G8', 'PREPARATION_MISMATCH'); bytes.push(component);
    };
    u32(maps.nodes.length); for (const n of maps.nodes) for (const v of n.coordinates) bits(v);
    u32(maps.members.length);
    for (const m of maps.members) {
      u32(m.kernel_member); u32(m.node_i); u32(m.node_j);
      for (const k of ['E', 'G', 'A_K', 'Iy_K', 'Iz_K', 'J_K']) bits(m[k]);
      for (const v of m.y_reference) bits(v);
    }
    u32(maps.springs.length);
    for (const spring of maps.springs) {
      u32(spring.kernel_spring); dof(spring); bits(spring.stiffness);
    }
    u32(0); // The selected straight ordinary source excludes directional springs.
    u32(s.constraints.length);
    for (const constraint of s.constraints) { dof(constraint.dof); if (withValues) bits(constraint.value); }
    if (withValues) {
      u32(s.nodal_terms.length);
      for (const term of s.nodal_terms) {
        dof(term.dof); const id = new TextEncoder().encode(term.source_id);
        u32(id.length); for (const byte of id) bytes.push(byte); bits(term.value);
      }
      u32(s.stations.length);
      for (const station of s.stations) { u32(station.id); u32(station.member); bits(station.fraction); }
      u32(s.supports.length);
      for (const support of s.supports) {
        u32(support.id); u32(support.node); for (const restrained of support.restrained) bytes.push(Number(restrained));
        u32(support.springs.length); for (const id of support.springs) u32(id);
        u32(support.directional_springs.length); for (const id of support.directional_springs) u32(id);
      }
    }
    return Uint8Array.from(bytes);
  };
  const digest = async (bytes: Uint8Array): Promise<string> => {
    const value = await crypto.subtle.digest('SHA-256', Uint8Array.from(bytes).buffer);
    return [...new Uint8Array(value)].map(b => b.toString(16).padStart(2, '0')).join('');
  };
  return { source: await digest(encode(true)), stiffness: await digest(encode(false)) };
}

async function invocationBinding(b: Obj, source: Obj, invocation: Obj): Promise<void> {
  const fail = (ok: unknown, code = 'PREPARATION_MISMATCH') => need(ok, 'G8', code);
  fail(same(Object.keys(invocation).sort(), ['request', 'solver_mode']) && ['dense_scrutiny', 'sparse_interactive'].includes(invocation.solver_mode), 'INVOCATION_MISMATCH');
  fail(await hash('source_blocks_invocation_v1', invocation) === b.invocation.value, 'INVOCATION_MISMATCH');
  const request = invocation.request, model = request.model;
  // D31: the model schema_version is 0.1.0, 0.2.0 or 0.3.0 (PP pressure_runtime.rs `validate_profile` treats 0.1.0 and
  // 0.2.0 on one branch); 0.4.0 stays excluded (C1 G8 row, "no 0.4 extension").
  fail(model?.project?.id === source.model_ref && ['0.1.0', '0.2.0', '0.3.0'].includes(model.schema_version), 'INVOCATION_MISMATCH');
  fail(!model.pressure_contract && !model.combinations?.length && !model.components?.length, 'INVOCATION_MISMATCH');
  const nodes: Obj[] = model.nodes, pipes: Obj[] = model.pipe_segments, supports: Obj[] = model.supports, cases: Obj[] = model.load_cases;
  fail([nodes, pipes, supports, cases].every(xs => Array.isArray(xs) && unique(xs.map(x => x.id))));
  const materials: Obj[] = request.materials?.length ? request.materials : model.materials; fail(Array.isArray(materials) && unique(materials.map(m => m.id)));
  const engine = await loadWasmEngine();
  const units: Record<string, string> = { length: 'm', stress: 'Pa', temperature: 'K', force: 'N', moment: 'N*m', linear_stiffness: 'N/m', rotational_stiffness: 'N*m/rad' };
  const convert = (q: Obj, dimension: string): number => {
    fail(q && typeof q === 'object' && typeof q.value === 'number' && typeof q.unit === 'string');
    const unit = units[dimension];
    const output = JSON.parse(engine.convertDisplayQuantitiesJson(checkedJsonText({ items: [{ id: 'retained', value: q.value, from_unit: q.unit, to_unit: unit, dimension_id: dimension }] })));
    fail(!output.error && output.items?.length === 1 && output.items[0].id === 'retained' && output.items[0].status === 'converted' && output.items[0].unit === unit && Number.isFinite(output.items[0].value)); return output.items[0].value;
  };
  const pair = (m: Obj): number[] => { const result = [convert(m.elastic_modulus, 'stress'), convert(m.shear_modulus, 'stress')]; fail(result.every(x => Number.isFinite(x) && x > 0)); return result; };
  function selectedMaterial(m: Obj, c: Obj): { pair: number[]; selection: Obj } {
    const base = pair(m); fail(c.modulus_basis_ref == null || c.modulus_basis_temperature == null);
    if (c.modulus_basis_ref == null && c.modulus_basis_temperature == null) return { pair: base, selection: { kind: 'base' } };
    const points: Obj[] = m.temperature_points ?? []; fail(unique(points.map(p => p.id)));
    if (c.modulus_basis_ref != null) { const p = points.find(x => x.id === c.modulus_basis_ref); fail(p); return { pair: pair(p!), selection: { kind: 'named_point', point_id: p!.id } }; }
    const t = convert(c.modulus_basis_temperature, 'temperature'), ordered = points.filter(p => p.temperature != null).map(p => ({ p, t: convert(p.temperature, 'temperature') })).sort((a, z) => a.t - z.t);
    fail(unique(ordered.map(p => p.t)));
    const i = ordered.findIndex((p, j) => j + 1 < ordered.length && p.t < t && t < ordered[j + 1].t); fail(i >= 0);
    const lo = ordered[i], hi = ordered[i + 1];
    // lib.rs `materials_for_modulus_basis_observed` requires all three source quantities at both bracket points, even on the
    // nonthermal ordinary route, before interpolating; a missing one is a source refusal.
    fail([lo.p, hi.p].every(p => ['elastic_modulus', 'shear_modulus', 'thermal_expansion_coefficient'].every(k => p[k] != null)));
    const ratio = (t - lo.t) / (hi.t - lo.t), a = pair(lo.p), z = pair(hi.p);
    const values = a.map((v, j) => v + (ratio * (z[j] - v))); fail(values.every(v => Number.isFinite(v) && v > 0));
    return { pair: values, selection: { kind: 'interpolated', lower_point_id: lo.p.id, upper_point_id: hi.p.id, target_kelvin: binary64Bits(t) } };
  }
  const coordinates = nodes.map(n => ['x', 'y', 'z'].map(k => { fail(typeof n.position?.[k] === 'number' && Number.isFinite(n.position[k])); return binary64Bits(convert({ value: n.position[k], unit: model.project.units?.length }, 'length')); }));
  const nodeIndex = (id: string) => { const i = nodes.findIndex(n => n.id === id); fail(i >= 0); return i; };
  const geometry = pipes.map(p => { fail(p.arc == null && p.curve == null); const od = convert(p.section.outside_diameter, 'length'), wall = convert(p.section.wall_thickness, 'length') - (p.section.mill_tolerance != null ? convert(p.section.mill_tolerance, 'length') : 0); fail(Number.isFinite(od) && 0 < wall && wall < od / 2); return [binary64Bits(od), binary64Bits(wall)]; });
  const selectors = cases.map(c => c.modulus_basis_ref != null ? { kind: 'named', id: c.modulus_basis_ref } : c.modulus_basis_temperature != null ? { kind: 'temperature', kelvin: binary64Bits(convert(c.modulus_basis_temperature, 'temperature')) } : { kind: 'base' });
  const knownSelectors: Obj[] = [];
  // B1 (DESIGN_v2 §3.2-§3.3, decisions 8 and 9; F-1 text B): for every case, in request order, the requested mode, the
  // material basis, P1 and P2-P4, each G8 PREPARATION. The case's ordinary attempt is ordinary_attempts[ci] (G3 binds
  // cases[ci].ordinary.attempt_ref to ci). Selected, unavailable and not_required cases alike: their rows all come from
  // the one ordinary run.
  for (let ci = 0; ci < cases.length; ci++) {
    const c = cases[ci], ordinary = b.ordinary_attempts[ci]; fail(ordinary.requested_mode === invocation.solver_mode);
    let bi = knownSelectors.findIndex(x => same(x, selectors[ci])); if (bi < 0) { bi = knownSelectors.length; knownSelectors.push(selectors[ci]); }
    fail(ordinary.material_basis_ref === bi && b.material_bases[bi]?.case_indices.includes(ci));
    const own = (kind: string) => source.results.filter((r: Obj) => r.basis_ref?.ref_id === c.id && r.kind === kind);
    // P1: exactly one mode row, valued 1 in sparse_interactive and 2 in dense_scrutiny.
    const modeRows = own('linear_solver_mode_basis');
    fail(modeRows.length === 1 && modeRows[0].value === (invocation.solver_mode === 'dense_scrutiny' ? 2 : 1));
    // P2-P4: at most one parity row (P2); none in sparse_interactive (P3); none when the ordinary W2 published, b != 0
    // (P4; OQ5). An absent parity row in dense mode at b = 0 is admitted (the disclosed limit; P5 deferred).
    const parityRows = own('sparse_live_path_dense_parity_relative_delta');
    fail(parityRows.length <= 1);
    fail(!parityRows.length || invocation.solver_mode === 'dense_scrutiny');
    fail(!parityRows.length || ordinary.w2?.kind !== 'published');
  }
  fail(b.material_bases.length === knownSelectors.length);
  b.material_bases.forEach((mb: Obj, bi: number) => {
    fail(same(mb.selector, knownSelectors[bi]) && same(mb.case_indices, cases.flatMap((_, ci) => same(selectors[ci], mb.selector) ? [ci] : [])));
    const used = new Set(pipes.map(p => p.material)); fail(same(mb.materials.map((m: Obj) => m.input_index), materials.flatMap((m, i) => used.has(m.id) ? [i] : [])));
    for (const m of mb.materials) {
      const raw = materials[m.input_index]; fail(raw && m.id === raw.id && same(m.shear_origin, { kind: 'explicit_g' }));
      for (const ci of mb.case_indices) { const selected = selectedMaterial(raw, cases[ci]); fail(same(m.selection, selected.selection) && same([m.elastic_modulus, m.shear_modulus], selected.pair.map(binary64Bits))); }
    }
  });
  function operational(inputs: string[]): Obj {
    const x = inputs.map(decodeBinary64), d = [0, 1, 2].map(i => x[i + 3] - x[i]), length = Math.sqrt(((d[0] * d[0]) + (d[1] * d[1])) + (d[2] * d[2])); fail(Number.isFinite(length) && length > 1e-12);
    const inverse = 1 / length, axialProduct = x[6] * x[8], torsionProduct = x[7] * x[9];
    // ScalarWork::coefficient requires a normal product before dividing by L.
    fail([axialProduct, torsionProduct].every(v => Number.isFinite(v) && Math.abs(v) >= MIN_NORMAL));
    const axial = axialProduct / length, torsion = torsionProduct / length;
    fail(Number.isFinite(axial) && Number.isFinite(torsion) && Math.abs(axial) >= MIN_NORMAL && Math.abs(torsion) >= MIN_NORMAL);
    return { kind: 'ready', length: binary64Bits(length), axial_stiffness: binary64Bits(axial), torsional_stiffness: binary64Bits(torsion), normalization: d.map(v => binary64Bits(v * inverse)) };
  }
  for (const s of b.sources) {
    const ci = s.owner.case_index, c = cases[ci], maps = s.id_maps, mb = b.material_bases[s.material_basis_ref];
    fail(c.id === s.owner.case_id && !c.pressure_regions?.length && c.equivalent_static == null && c.pressure == null);
    fail(mb?.case_indices.includes(ci) && maps.nodes.length === nodes.length && maps.members.length === pipes.length && maps.support_ids.length === supports.length && BigInt(nodes.length) * 6n <= 0xffffffffn && BigInt(pipes.length) * 3n <= 0xffffffffn);
    maps.nodes.forEach((n: Obj, i: number) => fail(n.model_index === i && n.kernel_node === i && n.id === nodes[i].id && same(n.coordinates, coordinates[i])));
    fail(s.section_terms.length === pipes.length && unique(maps.members.map((m: Obj) => m.built_pipe_index)));
    maps.members.forEach((m: Obj, i: number) => {
      const p = pipes[i], section = s.section_terms[i], geo = section.geometry, mat = mb.materials.find((v: Obj) => v.input_index === m.material_index);
      fail(m.model_index === i && m.kernel_member === i && section.member === i && m.id === p.id && m.built_pipe_index >= 0 && m.built_pipe_index < pipes.length);
      fail(m.node_i === nodeIndex(p.from) && m.node_j === nodeIndex(p.to) && m.node_i !== m.node_j && same(m.y_reference, ['x', 'y', 'z'].map(k => binary64Bits(p.y_reference[k]))));
      fail(mat && mat.id === p.material && m.E === mat.elastic_modulus && m.G === mat.shear_modulus);
      fail(geo.route === 'preview' && same([geo.normalized_od, geo.effective_wall], geometry[i]) && geo.actual_radius === binary64Bits(decodeBinary64(geometry[i][0]) / 2));
      fail(section.area === m.A_K && geo.actual_second_moment === m.Iy_K && m.Iy_K === m.Iz_K && geo.actual_polar_moment === m.J_K && ['E', 'G', 'A_K', 'Iy_K', 'Iz_K', 'J_K'].every(k => decodeBinary64(m[k]) >= MIN_NORMAL));
    });
    const parent = sequence(nodes.length); const root = (n: number): number => { while (parent[n] !== n) n = parent[n]; return n; };
    for (const m of maps.members) { const a = root(m.node_i), z = root(m.node_j); parent[Math.max(a, z)] = Math.min(a, z); }
    const roots = [...new Set(nodes.map((_, i) => root(i)))].sort((a, z) => a - z);
    const bodies = roots.map((r, body) => { const ns = sequence(nodes.length).filter(i => root(i) === r); return { body, nodes: ns, members: maps.members.filter((m: Obj) => ns.includes(m.node_i)).map((m: Obj) => m.kernel_member) }; }); fail(same(s.body_membership, bodies));
    const fixed = new Map<string, { dof: Obj; support_indices: number[] }>(), springs: Obj[] = [], supportRows: Obj[] = [];
    supports.forEach((raw, i) => {
      fail(raw.nonlinear == null && raw.hanger == null && raw.imposed_displacement == null && [undefined, null, 'anchor', 'guide', 'line_stop', 'vertical_support', 'spring'].includes(raw.family));
      const node = nodeIndex(raw.node); fail(same(maps.support_ids[i], { model_index: i, kernel_support: i, id: raw.id, node }));
      const restrained = [false, false, false, false, false, false], ids: number[] = [];
      fail(Array.isArray(raw.restraints) && unique(raw.restraints) && raw.restraints.every((c: string) => COMPONENTS.includes(c)));
      if (raw.family === 'spring') {
        const component = raw.stiffness.dof, index = COMPONENTS.indexOf(component); fail(index >= 0 && same(raw.restraints, [component]));
        const stiffness = convert(raw.stiffness.value, index < 3 ? 'linear_stiffness' : 'rotational_stiffness'); fail(stiffness > 0);
        const id = springs.length; ids.push(id); springs.push({ boundary_index: id, kernel_spring: id, support_index: i, node, component, stiffness: binary64Bits(stiffness) });
      } else for (const component of raw.restraints) { const key = node + ':' + component; fail(!fixed.has(key)); restrained[COMPONENTS.indexOf(component)] = true; fixed.set(key, { dof: { node, component }, support_indices: [i] }); }
      supportRows.push({ id: i, node, restrained, springs: ids, directional_springs: [] });
    });
    const constraints = [...fixed.values()].sort((a, z) => a.dof.node - z.dof.node || COMPONENTS.indexOf(a.dof.component) - COMPONENTS.indexOf(z.dof.component)).map(c => ({ dof: c.dof, value: ZERO, support_indices: c.support_indices }));
    fail(same(maps.springs, springs) && same(s.supports, supportRows) && same(s.constraints, constraints));
    const stations = pipes.flatMap((_, i) => ['quarter_1', 'midspan', 'quarter_3'].map((location, j) => ({ id: 3 * i + j, member: i, location, fraction: binary64Bits([.25, .5, .75][j]) }))); fail(same(s.stations, stations));
    const directions: Obj = Object.fromEntries([...COMPONENTS.map(c => [c, c]), ...['global_x', 'global_y', 'global_z', 'rotation_x', 'rotation_y', 'rotation_z'].map((k, i) => [k, COMPONENTS[i]])]);
    const terms = (c.primitive_loads ?? []).map((load: Obj, i: number) => {
      fail(load.target?.type === 'node' && load.category !== 'thermal' && ['force', 'moment'].includes(load.dimension));
      const node = nodeIndex(load.target.node), component = directions[load.direction]; fail(component && (COMPONENTS.indexOf(component) < 3) === (load.dimension === 'force'));
      return { constructor_ordinal: i, source_id: load.id, primitive_load_index: i, dof: { node, component }, value: binary64Bits(convert(load.magnitude, load.dimension)) };
    }).sort((a: Obj, z: Obj) => (a.dof.node * 6 + COMPONENTS.indexOf(a.dof.component)) - (z.dof.node * 6 + COMPONENTS.indexOf(z.dof.component)) || compareCodePoints(a.source_id, z.source_id) || (a.value < z.value ? -1 : a.value > z.value ? 1 : 0) || a.constructor_ordinal - z.constructor_ordinal);
    fail(same(s.nodal_terms, terms));
    const bodyOf = (node: number) => { const hit = bodies.find(b => b.nodes.includes(node)); fail(hit); return hit!.body; };
    fail(same(s.layout, sourceLayout(nodes.length, maps.members, stations, springs, constraints, supportRows, key => fixed.has(key), bodyOf)));
    const identities = await nativeSourceHashes(s);
    fail(identities.source === s.kernel_source_sha256 && identities.stiffness === s.stiffness_sha256);
  }
  // Failed helper/new-evaluator prefixes are also bound without inventing a source.
  for (const a of b.product_attempts) {
    const mb = b.material_bases[a.material_basis_ref]; fail(mb && mb.case_indices.includes(a.owner_ref.index));
    if (a.operational.old_coverage === 'complete') fail(same(a.operational.old.map((m: Obj) => m.member), sequence(pipes.length)));
    else fail(same(a.operational.old.map((m: Obj) => m.member), sequence(a.operational.old.length)));
    // P7 settlement (F1:101-106; C3:155-158): old inputs are bound wherever a PreparedMember exists,
    // for every attempt (below); other retained old entries stay producer attestations.
    for (let j = 0; j < a.preparation.members.length; j++) {
      const m = a.preparation.members[j], p = pipes[m.member], old = m.old_source, facts = m.old_facts, material = mb.materials.find((v: Obj) => v.id === p.material);
      fail(material && same(old.slice(0, 2), [material.elastic_modulus, material.shear_modulus]) && old[2] === facts[2] && old[3] === facts[3] && old[4] === facts[3] && old[5] === facts[4] && same(facts.slice(0, 2), geometry[m.member]));
      const positions = [...coordinates[nodeIndex(p.from)], ...coordinates[nodeIndex(p.to)]];
      fail(same(a.operational.old[j].inputs, [...positions, old[0], old[1], old[2], old[5]]));
      if (j < a.operational.new.length) {
        fail(m.result.kind === 'prepared'); const fresh = a.operational.new[j], section = m.result.section;
        fail(same(fresh.inputs, [...positions, old[0], old[1], section[0], section[2]]));
        if (fresh.result.kind === 'ready') fail(same(fresh.result, operational(fresh.inputs)));
      }
      if (a.source_ref !== null) {
        const s = b.sources[a.source_ref], section = s.section_terms[j], fresh = a.operational.new[j];
        fail(s.preparation?.attempt_ref === a.id && same(m.result.section, [section.area, section.geometry.actual_second_moment, section.geometry.actual_polar_moment, section.section_modulus, section.geometry.actual_radius]));
        if (fresh.result.kind === 'ready') fail(['length', 'axial_stiffness', 'torsional_stiffness'].every(k => fresh.result[k] === section[k]));
      }
    }
  }
}

/** @internal Exported only for the reader-logic O5/O2 unit tests; not a public entry point. */
export function ordinaryAttempts(b: Obj, source: Obj): void {
  const fail = (ok: unknown, code = 'ATTEMPT_MISMATCH') => need(ok, 'G5', code);
  const ds: Obj[] = source.diagnostics;
  b.cases.forEach((c: Obj, ci: number) => {
    const a = b.ordinary_attempts[ci], cid = c.basis_ref.ref_id, q = source.numerical_quality.cases[ci];
    // O2 reference rule (06b settlement, C2:166): a typed diagnostic reference is listed in this
    // attempt's diagnostic_refs, resolves, and names this case; a required one is non-null.
    const diagnostic = (ref: string | null | undefined, _cid: string, required = false) => {
      if (ref == null) { fail(!required); return; }
      const d = ds.find(d => d.id === ref); fail(d && d.affected_refs?.includes(cid) && a.diagnostic_refs.includes(ref));
    };
    fail(b.material_bases[a.material_basis_ref]?.case_indices.includes(ci));
    // D6a (checkpoint A): untyped diagnostic_refs are unique and resolve.
    fail(unique(a.diagnostic_refs) && a.diagnostic_refs.every((ref: string) => ds.some(d => d.id === ref)));
    // F5 (D-U6-7; decision 2, A2, RR:8821, amending checkpoint A's D6a, RR:8117): the list is exactly the diagnostics
    // whose affected_refs name the case, once each, in envelope order, excluding RETAINED_PRECISION_* (a T1 (a)-omitted
    // disclosure is absent from the published envelope).
    const exact = ds.filter(d => Array.isArray(d.affected_refs) && d.affected_refs.includes(cid) && !String(d.code).startsWith('RETAINED_PRECISION_')).map(d => d.id);
    fail(a.diagnostic_refs.length === exact.length && a.diagnostic_refs.every((ref: string, i: number) => ref === exact[i]));
    if (a.initial.kind === 'report') { diagnostic(a.initial.report_diagnostic_ref, cid, true); fail(a.diagnostic_refs.includes(a.initial.report_diagnostic_ref) && a.initial.outcome === q.solve_quality); }
    if (a.initial.kind === 'structural_failure') diagnostic(a.initial.diagnostic_ref, cid);
    if (a.w2.kind !== 'not_triggered') {
      fail(['formation_failure', 'structural_failure'].includes(a.initial.kind));
      fail(a.w2.trigger.tag === (a.initial.kind === 'formation_failure' ? 'formation' : 'evaluation') && same(a.w2.trigger.error, a.initial.error));
      diagnostic(a.w2.kind === 'published' ? a.w2.report_diagnostic_ref : a.w2.diagnostic_ref, cid, true);
      if (a.w2.kind === 'published') fail(a.w2.force_scale_exponent !== 0);
    }
    if (a.formation.load_row_finding) diagnostic(a.formation.load_row_finding.diagnostic_ref, cid);
    diagnostic(a.formation.d5_diagnostic_ref, cid); diagnostic(a.legacy_source.diagnostic_ref, cid);
    // D6d (C2:166): legacy_source.work_ref resolves into legacy_source_work for this case; a reference check.
    if (a.legacy_source.work_ref !== null) { const w = at(b.legacy_source_work ?? [], a.legacy_source.work_ref, 'G5', 'ATTEMPT_MISMATCH'); fail(w.case_index === ci); }
    if (c.status === 'not_required') fail(c.product_attempt_ref === null && a.initial.kind !== 'not_attempted' && q.solve_quality === 'checks_passed');
    // D20: the selected case's C3 attempt is checked in the C3 association pass (PRODUCT_ATTEMPT), not here.
    if (c.status === 'selected') fail(a.initial.kind !== 'not_attempted' && ['sensitive', 'unresolved', 'failed'].includes(q.solve_quality));
    if (c.status === 'unavailable' && c.reason.cause.kind !== 'prepared_product_failure') {
      const cause = c.reason.cause;
      if (cause.kind === 'source_error') fail(c.reason.phase === 'preparation' && c.reason.code === 'source_unavailable' && !c.run && c.source_decline && same(c.source_decline.error, cause.error));
      else if (cause.kind === 'receipt_failure') fail(c.reason.phase === 'receipt' && ['receipt_encoding', 'publication_hash_range', 'invocation_not_representable'].includes(c.reason.code));
      else if (cause.kind === 'facade_failure') fail(c.reason.phase === 'facade' && c.reason.code === 'facade_certificate' && c.run?.kernel_terminal.kind === 'selected' && same(cause.owner_ref, { kind: 'case', index: ci }));
      else if (cause.kind === 'unavailable_precondition') fail(['routing', 'preparation'].includes(c.reason.phase) && !c.run && ['source_unavailable', 'resource_admission_not_available', 'upstream_no_wrap_not_established', 'caller_not_qualified'].includes(c.reason.code));
      else fail(c.reason.phase === 'kernel' && c.run && c.reason.code === 'kernel_' + c.run.kernel_terminal.kind && same(cause, c.run.kernel_terminal.reason));
    }
    // O5 (C2:115-119; S06:51): a source decline belongs to an unavailable case with no source or run.
    if (c.source_decline) fail(c.status === 'unavailable' && !c.run && c.source_ref == null && same(c.source_decline.input_owner, { case_index: ci, case_id: cid, material_basis_ref: a.material_basis_ref }));
  });
}
function conversionEncoding(b: Obj): void {
  for (const a of b.product_attempts) {
    for (const m of a.preparation.members) for (const c of m.conversions) conversions(c.outcome);
    for (const c of a.proof?.projection_outcomes ?? []) conversions(c.outcome);
    for (const e of a.proof?.summary_coverage ?? []) uint(e.body); // I57 §4 G2: body is a safe U.
  }
}
/** The base (preview-physics-1) view of a successor. RV108 N4 (B1 SR-TS): only object rows lose `recovery_method`; any
 * other `results` entry (null, a number, a string, an array) is left as it is. The full reader has already refused such
 * an entry at G1 (RawRow). The transport reading reads no rows, as Rust's transport does, so a transported successor
 * whose `results` holds one is admitted (Python's `_transport_g7` likewise skips a non-dict row). */
function projection(source: Obj): Obj {
  const p = structuredClone(source); delete p.retained_precision; p.producer.semantic_contract_id = BASE_ID; p.formulation_basis.profile_id = 'product_preview_mechanics_v1';
  if (Array.isArray(p.results)) for (const row of p.results) if (isObj(row) && Object.hasOwn(row, 'recovery_method')) delete row.recovery_method;
  return p;
}
const QUALITY_STATUSES = ['not_assessed', 'checks_passed', 'sensitive', 'unresolved', 'failed'];
/** RV94 N-3 (B6; PLAN decision 11): when TS's header dispatch refuses the projected base at G7, the refusal carries the
 * base readers' header code, branch by branch in Python's `_source_contract` order: an invalid numerical_quality case of
 * any status is SOURCE_NUMERICAL_CASE_INVALID, and so on. Any other header refusal keeps TS's
 * SOURCE_PRODUCER_CONTRACT_UNSUPPORTED. The projection fixes the base identity and profile, and G0 the schema and component
 * versions. Read only after the dispatch has refused.
 * What agrees (RV108 §1, N1 and its N6 ruling; reworded in B1 SR-TS):
 * - Rust (`semantic_contract::for_source`) gives the same code on every single-defect header refusal except the declared
 *   inherited `carrier_evidence` class, which Rust's header does not read. With both a `contract_evidence` and a
 *   `source_block_recovery` defect, Rust reports the latter (I83 mutant T9; inherited).
 * - Python gives the same code on every header refusal, except that a list- or dict-valued enum raised TypeError in
 *   `_source_contract` (Python's G7 fallback, SOURCE_PREVIEW_PHYSICS_INVALID) until B1 SR-PY's type guard (RV108 N1);
 *   with that guard Python gives this code there too. */
function baseHeaderCode(p: Obj): string {
  const exact = (v: unknown, keys: string[]): v is Obj => isObj(v) && Object.keys(v).length === keys.length && keys.every(k => Object.hasOwn(v, k));
  const text = (v: unknown) => typeof v === 'string' && v.length > 0;
  if (Object.hasOwn(p, 'carrier_evidence')) return 'SOURCE_PRODUCER_CONTRACT_UNSUPPORTED';
  if (p.schema_version !== '0.2.0') return 'SOURCE_SCHEMA_VERSION_UNSUPPORTED';
  const producer = p.producer;
  if (!exact(producer, ['component_name', 'component_version', 'semantic_contract_id']) || producer.component_name !== 'open_pipe_stress_product_physics'
    || producer.component_version !== '0.2.0' || producer.semantic_contract_id !== BASE_ID) return 'SOURCE_PRODUCER_CONTRACT_UNSUPPORTED';
  if (!isObj(p.contract_evidence)) return 'SOURCE_PREVIEW_PHYSICS_EVIDENCE_REQUIRED';
  if (Object.hasOwn(p, 'source_block_recovery')) return 'SOURCE_BLOCKS_LEGACY_DOWNGRADE_FORBIDDEN';
  const q = p.numerical_quality;
  if (!exact(q, ['value_representation', 'publication_quantization', 'integrity_policy', 'status', 'cases']) || q.value_representation !== 'finite_binary64'
    || q.publication_quantization !== 'none' || q.integrity_policy !== 'M03-INTEGRITY-v1' || !QUALITY_STATUSES.includes(q.status) || !Array.isArray(q.cases)) return 'SOURCE_NUMERICAL_QUALITY_INVALID';
  for (const c of q.cases) {
    if (!exact(c, ['basis_ref', 'structural_status', 'solve_quality', 'model_matrix_fidelity', 'accuracy_evidence', 'evidence_refs'])
      || !exact(c.basis_ref, ['ref_type', 'ref_id']) || !text(c.basis_ref.ref_type) || !text(c.basis_ref.ref_id) || !QUALITY_STATUSES.includes(c.solve_quality)
      || !['passive_model_basis', 'physical_mechanism_witnessed', 'negative_energy_witnessed', 'numerically_unresolved'].includes(c.structural_status)
      || !['represented_equations_retained', 'assembly_loss_detected', 'assembly_uncertainty', 'not_assessed'].includes(c.model_matrix_fidelity)
      || !['not_claimed', 'reference_verified', 'unresolved'].includes(c.accuracy_evidence)
      || !Array.isArray(c.evidence_refs) || !c.evidence_refs.every(text)) return 'SOURCE_NUMERICAL_CASE_INVALID';
  }
  const f = p.formulation_basis;
  if (!exact(f, ['profile_id', 'limitations']) || f.profile_id !== 'product_preview_mechanics_v1' || !Array.isArray(f.limitations) || !f.limitations.length
    || !f.limitations.every(text)) return 'SOURCE_FORMULATION_BASIS_UNSUPPORTED';
  return 'SOURCE_PRODUCER_CONTRACT_UNSUPPORTED';
}
const defaultErrors: Record<string, string> = { G0: 'SOURCE_PRODUCER_CONTRACT_UNSUPPORTED', G1: 'RETAINED_PRECISION_RECEIPT_MISMATCH', G2: 'RETAINED_PRECISION_ENCODING_MISMATCH', G3: 'RETAINED_PRECISION_COVERAGE_MISMATCH', G4: 'RETAINED_PRECISION_DIAGNOSTIC_MISMATCH', G5: 'RETAINED_PRECISION_PRODUCT_ATTEMPT_MISMATCH', G5a: 'RETAINED_PRECISION_SCALE_MISMATCH', G5b: 'RETAINED_PRECISION_SCALE_MISMATCH', G5c: 'RETAINED_PRECISION_CLASSIFICATION_MISMATCH', G6: 'RETAINED_PRECISION_ROW_METHOD_MISMATCH', G7: 'SOURCE_PREVIEW_PHYSICS_EVIDENCE_INVALID', G8: 'RETAINED_PRECISION_PREPARATION_MISMATCH' };
/** The accepted ordered reader (G0-G8). D-U6-1: every gate runs; since U7 a valid invocation-bound statement of a solved model whose cases are selected or not_required reads eligible.
 * Standing comes from the carriers (D2 4.9.4). Hashes bind the supplied statements; they do not establish producer origin (D-U7-6). No registration, mutable eligibility cache, or private proof replay. */
export async function validateRetainedPrecision(source: unknown, invocation?: unknown): Promise<RetainedPrecisionValidation> {
  let gate = 'G0';
  try {
    // Capture synchronously before the first await; later caller edits cannot alter this validation.
    gate = 'G1'; const s = snapshot(source); const actual = invocation == null ? undefined : snapshot(invocation);
    gate = 'G0'; await header(s);
    gate = 'G1'; const b = await integrity(s, false);
    gate = 'G2'; conversionEncoding(b);
    gate = 'G3'; const rows = coverage(b, s, actual);
    gate = 'G4'; diagnostics(b, s);
    gate = 'G5'; nativeRuns(b); ordinaryAttempts(b, s); productAttempts(b, rows);
    gate = 'G5a'; const numeric = numericalCases(b, rows); numericSummaries(numeric, b); unselectedCoverage(b);
    gate = 'G5b'; numericalScales(numeric, s);
    gate = 'G5c'; const classes = classifications(numeric, s);
    gate = 'G6'; for (const c of b.cases) for (const row of rows.get(c.basis_ref.ref_id)!) need(c.status === 'selected' ? row.recovery_method === RETAINED_METHOD : !Object.hasOwn(row, 'recovery_method'), gate, 'ROW_METHOD_MISMATCH');
    gate = 'G7'; const base = projection(s); if (sourceContract(base as MechanicsResult) !== 'preview_physics') throw new RetainedPrecisionError(gate, baseHeaderCode(base));
    try { validatePreviewPhysicsEvidence(base as MechanicsResult); } catch (error) { throw baseError(gate, error, defaultErrors.G7); }
    gate = 'G8'; if (actual) await invocationBinding(b, s, actual);
    const eligible = SUMMARY_COVERAGE_COMPLETE && actual !== undefined && s.status?.mechanics === 'MECHANICS_SOLVED' && b.cases.every((c: Obj) => ['selected', 'not_required'].includes(c.status));
    return freeze({ invocation_bound: actual !== undefined, numerical_eligible: eligible, standing: eligible ? 'eligible' : 'needs_recompute', publication_sha256: b.publication_sha256, classifications: classes });
  } catch (error) { if (error instanceof RetainedPrecisionError) throw error; throw new RetainedPrecisionError(gate, defaultErrors[gate]); }
}
/** G0-G2 and unchanged base metadata only: omitted raw publication bytes are never reconstructed or verified. */
export async function validateRetainedPrecisionTransport(source: unknown): Promise<RetainedPrecisionValidation> {
  let gate = 'G1';
  try {
    const s = snapshot(source); gate = 'G0'; await header(s); gate = 'G1'; const b = await integrity(s, true); gate = 'G2'; conversionEncoding(b);
    gate = 'G7'; try { validatePreviewPhysicsTransportMetadata(projection(s) as MechanicsResult); } catch (error) { throw baseError(gate, error, defaultErrors.G7); }
    return freeze({ invocation_bound: false, numerical_eligible: false, standing: 'needs_recompute', publication_sha256: b.publication_sha256, classifications: [] });
  } catch (error) { if (error instanceof RetainedPrecisionError) throw error; throw new RetainedPrecisionError(gate, defaultErrors[gate]); }
}
