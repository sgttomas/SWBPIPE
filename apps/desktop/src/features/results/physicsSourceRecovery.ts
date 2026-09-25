import { validatePhysicsSourceTransportFacts } from './physicsResultEvidence';
import type { MechanicsResult, PreviewModel } from '../../types';
import { canonicalSha256HexCheckedV1, checkedJsonText } from '../../services/hashService';
import { loadWasmEngine } from '../../services/wasmEngine/loadWasmEngine';
import {
  physicsSourceReceiptShape, physicsSourcePhysicalShape, physicsSourceReceiptStanding,
  retainedPhysicsSourceReceiptInvocation, validatePhysicsSourceReceiptRecovery,
  type SourceBlockInvocation, type SourceRecoveryValidation,
} from './sourceBlockRecovery';

export { physicsSourceReceiptShape };
export const PHYSICS_SOURCE_CONTRACT_ID = 'openpipestress.result_semantics/0.3.0/physics-source-1';
export const PHYSICS_SOURCE_CONTRACT_SHA256 = 'ba13f2aefd7a38bd725e5f111e6ec30144bc8776aa957c6278ee7b1178298ba1';
export const PHYSICS_SOURCE_MAX_BASIS = 'retained_source_endpoint_normal_max_v1';
export const PHYSICS_SOURCE_MAX_SIGN = 'nonnegative maximum absolute axial-plus-bending normal stress over an unloaded circular straight span; retained endpoint actions with projected-action and arithmetic bounds; endpoint witness does not imply uniqueness; torsional shear separate';
export const PHYSICS_SOURCE_SUPPORT_SIGN = 'support-on-pipe; positive global force and right-hand couple about attached node; force and moment norms remain separate';
const EXACT = 'retained_source_blocks_exact_v1';
const PROFILE = 'exact_straight_pressure_v2';
const STRESS_LIMIT = (1e-9 - 128 * Number.EPSILON) / (1 + 128 * Number.EPSILON);
const LOCATIONS = ['end_i', 'quarter_1', 'midspan', 'quarter_3', 'end_j'];
type Obj = Record<string, any>;
type Row = MechanicsResult['results'][number];
type Transport = Pick<MechanicsResult, 'producer' | 'contract_evidence' | 'source_block_recovery'>;
function need(ok: unknown, code: string): asserts ok { if (!ok) throw new Error(`PHYSICS_SOURCE_${code}`); }
const unique = (values: unknown[]) => new Set(values).size === values.length;
const hash = (domain: string, payload: unknown) => canonicalSha256HexCheckedV1({ domain, payload });
function same(a: any, b: any): boolean {
  if (Object.is(a, b)) return true;
  if (!a || !b || typeof a !== 'object' || typeof b !== 'object' || Array.isArray(a) !== Array.isArray(b)) return false;
  const keys = Object.keys(a);
  return keys.length === Object.keys(b).length && keys.every(k => Object.hasOwn(b, k) && same(a[k], b[k]));
}
const physical = (source: Transport) => source.contract_evidence as Obj;
const receipt = (source: Transport) => source.source_block_recovery as Obj;
const receiptCase = (source: Transport, id: string): Obj => {
  const matches = receipt(source).body.cases.filter((c: Obj) => c.basis_ref.ref_id === id);
  need(matches.length === 1, 'CASE_ID'); return matches[0];
};
const functionalId = (caseId: string, index: number) => `source-functional:${new TextEncoder().encode(caseId).length}:${caseId}:${index}`;

/** Method-contract parity with pressure_exact::IsotropicENu::new and Scaled,
 * not a second material model. Operands have exact binary64 mantissas in
 * [.5,1). The admitted nu range makes 2*(1+nu) positive and normal, so its
 * ordered operations equal the producer's scaled add/multiply. The quotient
 * is normalized before applying Scaled::to_f64's exponent and rounding rules.
 * In particular exponent -1074 is multiplied/rounded; below it is zero.
 * Exponent 1024 uses the producer's split multiply; above it is infinite. */
export function physicsSourceMaterialPairRepresentable(e: number, nu: number): boolean {
  if (!Number.isFinite(e) || e <= 0 || !Number.isFinite(nu) || nu <= -1 || nu >= .5) return false;
  const parts = (value: number): [number, number] => {
    const view = new DataView(new ArrayBuffer(8));
    view.setFloat64(0, value, false);
    let bits = view.getBigUint64(0, false), exponent = Number((bits >> 52n) & 0x7ffn) - 1022;
    if (exponent === -1022) {
      view.setFloat64(0, value * 2 ** 54, false); bits = view.getBigUint64(0, false);
      exponent = Number((bits >> 52n) & 0x7ffn) - 1022 - 54;
    }
    view.setBigUint64(0, (1022n << 52n) | (bits & ((1n << 52n) - 1n)), false);
    return [view.getFloat64(0, false), exponent];
  };
  const [em, ee] = parts(e), [dm, de] = parts(2 * (1 + nu));
  const [mantissa, shift] = parts(em / dm), exponent = ee - de + shift;
  if (exponent < -1074 || exponent > 1024) return false;
  const g = exponent === 1024 ? (mantissa * 2 ** 1023) * 2 : mantissa * 2 ** exponent;
  return Number.isFinite(g) && g > 0;
}

/** Transport statements and crosshashes only. This never registers a source,
 * reconstructs an omitted raw publication, or authenticates Current use. */
export async function validatePhysicsSourceTransportMetadata(source: Transport): Promise<void> {
  checkedJsonText(source);
  const received = source; source = structuredClone(source);
  need(same(source.producer, { component_name: 'open_pipe_stress_product_physics', component_version: '0.2.0', semantic_contract_id: PHYSICS_SOURCE_CONTRACT_ID }), 'PRODUCER');
  need(!Object.hasOwn(source, 'carrier_evidence'), 'FOREIGN_NAMESPACE');
  need(physicsSourceReceiptShape(source.source_block_recovery), 'RECEIPT_SHAPE');
  need(physicsSourcePhysicalShape(source.contract_evidence), 'PHYSICAL_SHAPE');
  const r = receipt(source), body = r.body, evidence = physical(source);
  validatePhysicsSourceTransportFacts(evidence);
  need(r.receipt_sha256 === await hash('source_blocks_receipt_v1', body), 'RECEIPT_HASH');
  const cases = evidence.exact_cases as Obj[], records = body.cases as Obj[];
  need(same(cases.map(c => c.load_case_id), records.map(c => c.basis_ref.ref_id)) && unique(cases.map(c => c.load_case_id)), 'CASE_ORDER');
  need(records.some(c => c.selected_method === EXACT), 'SOURCE_SELECTION_REQUIRED');
  for (let i = 0; i < cases.length; i++) {
    const c = cases[i], record = records[i], pressure = evidence.pressure.filter((p: Obj) => p.load_case_id === c.load_case_id);
    need(c.recovery_method === record.selected_method, 'RECOVERY_METHOD');
    need(record.physical_evidence_sha256 === await hash('physics_source_case_evidence_v1', { exact_case: c, pressure }), 'PHYSICAL_CASE_HASH');
    if (record.selected_method === EXACT) {
      need(!pressure.length && record.source, 'SOURCE_PRESSURE_INVENTORY');
      const plan = record.source, ids = plan.member_ids;
      need(ids.length > 0 && unique(ids) && same(c.pipe_sections.map((s: Obj) => s.pipe_id), ids)
        && same(c.pipe_materials.map((m: Obj) => m.pipe_id), ids) && same(plan.endpoint_sections.map((s: Obj) => s.pipe_id), ids), 'PHYSICAL_MEMBER_ORDER');
      need(same(c.stress_maximum_coverage, { complete: true, unavailable_pipe_ids: [] }), 'MAXIMUM_COMPLETE');
      need(c.pipe_stress_extrema.length === ids.length && unique(c.pipe_stress_extrema.map((x: Obj) => x.pipe_id))
        && c.pipe_stress_extrema.every((x: Obj) => ids.includes(x.pipe_id)), 'MAXIMUM_COVERAGE');
      sectionFunctionals(record);
      c.pipe_stress_extrema.forEach((x: Obj) => maximumLink(c, x, record));
    } else {
      need(!record.derived_checks.length && !record.section_stress_checks.length
        && record.selected_method === `ordinary_${record.requested_mode === 'dense_scrutiny' ? 'dense' : 'sparse'}_structural_v1`, 'ORDINARY_METHOD');
    }
  }
  need(evidence.pressure.every((p: Obj) => cases.some(c => c.load_case_id === p.load_case_id)), 'PRESSURE_CASE');
  need(same(received, source), 'TRANSPORT_CHANGED_DURING_VALIDATION');
}

/** Receipt consistency with an independently captured complete native request.
 * Native origin itself remains exclusively owned by previewService. */
export function validatePhysicsSourceRecovery(source: MechanicsResult, invocation: SourceBlockInvocation, callerModel?: PreviewModel): Promise<SourceRecoveryValidation> {
  return validatePhysicsSourceReceiptRecovery(source, invocation, callerModel);
}
export function physicsSourceStanding(source: MechanicsResult, model?: Pick<PreviewModel, "load_cases"> | null): SourceRecoveryValidation {
  return physicsSourceReceiptStanding(source, model);
}
export function retainedPhysicsSourceInvocation(source: MechanicsResult, model: PreviewModel): SourceBlockInvocation | null {
  return retainedPhysicsSourceReceiptInvocation(source, model);
}
export function physicsSourceModeMatches(source: MechanicsResult, model: PreviewModel, mode: string): boolean {
  return retainedPhysicsSourceInvocation(source, model)?.solver_mode === mode;
}

function finite(value: number): number { need(Number.isFinite(value), 'ARITHMETIC_RANGE'); return value; }
function outward(value: number, upper: boolean): number {
  finite(value); need(value >= 0, 'NEGATIVE_BOUND');
  if (!upper && value === 0) return 0;
  const view = new DataView(new ArrayBuffer(8)); view.setFloat64(0, value === 0 ? 0 : value, false);
  view.setBigUint64(0, view.getBigUint64(0, false) + (upper ? 1n : -1n), false);
  return finite(view.getFloat64(0, false));
}
const div = (a: number, b: number, upper: boolean): number => a === 0 || b === 1 ? a : a === b ? 1 : outward(a / b, upper);
const mul = (a: number, b: number, upper: boolean): number => a === 0 || b === 0 ? 0 : a === 1 ? b : b === 1 ? a : outward(a * b, upper);
const add = (a: number, b: number, upper: boolean): number => a === 0 ? b : b === 0 ? a : outward(a + b, upper);
function absInterval([lo, hi]: number[]): number[] { return lo >= 0 ? [lo, hi] : hi <= 0 ? [-hi, -lo] : [0, Math.max(-lo, hi)]; }
function scaledNorm(values: number[]): number {
  const scale = Math.max(...values.map(Math.abs)); if (scale === 0) return 0;
  const squares = values.map(v => (v / scale) * (v / scale));
  const sum = squares.length === 3 ? (squares[0] + squares[1]) + squares[2] : squares[0] + squares[1];
  return finite(scale * Math.sqrt(sum));
}
function normBound(values: number[], upper: boolean): number {
  const scale = Math.max(...values); if (scale === 0) return 0;
  const squares = values.map(v => { if (v === 0) return 0; if (v === scale) return 1; const n = outward(v / scale, upper); return outward(n * n, upper); });
  const total = add(add(squares[0], squares[1], upper), squares[2], upper);
  const root = total === 1 ? 1 : outward(Math.sqrt(total), upper);
  return root === 1 ? scale : outward(scale * root, upper);
}
function bounds(value: number, lo: number, hi: number): [number, number] {
  need([lo, value, hi].every(Number.isFinite) && 0 <= lo && lo <= value && value <= hi, 'ENCLOSURE');
  const absolute = lo === value && value === hi ? 0 : outward(Math.max(value - lo, hi - value), true);
  if (hi === 0) return [absolute, 0];
  need(Math.min(lo, value, hi) >= 2 ** -1022, 'PUBLICATION_RANGE');
  const relative = absolute === 0 ? 0 : outward(absolute / lo, true);
  need(relative <= 1e-9, 'PROTECTED_CRITERION'); return [absolute, relative];
}
function endpoint(area: number, z: number, actions: Obj[]): [number, number[]] {
  const values = actions.map(a => Math.abs(a.value));
  const value = finite(finite(values[0] / area) + scaledNorm([finite(values[1] / z), finite(values[2] / z)]));
  const intervals = actions.map(a => absInterval(a.interval));
  const interval = [false, true].map((upper, side) => {
    const axial = div(intervals[0][side], area, upper), y = div(intervals[1][side], z, upper), zed = div(intervals[2][side], z, upper);
    let norm = Math.max(y, zed);
    if (y !== 0 && zed !== 0) {
      const scale = norm, a = div(y, scale, upper), b = div(zed, scale, upper);
      norm = mul(scale, outward(Math.sqrt(add(mul(a, a, upper), mul(b, b, upper), upper)), upper), upper);
    }
    return add(axial, norm, upper);
  });
  need(interval[0] <= value && value <= interval[1], 'ENDPOINT_ENCLOSURE'); return [value, interval];
}
function maximumLink(c: Obj, ex: Obj, record: Obj): void {
  const plan = record.source;
  need(ex.load_case_id === c.load_case_id && ex.basis === PHYSICS_SOURCE_MAX_BASIS && ex.coefficient_basis === 'retained_section_functionals_binary64', 'MAXIMUM_METHOD');
  need(ex.source_identity_sha256 === plan.retained_identity_sha256, 'MAXIMUM_SOURCE_IDENTITY');
  const section = c.pipe_sections.find((s: Obj) => s.pipe_id === ex.pipe_id), saved = plan.endpoint_sections.find((s: Obj) => s.pipe_id === ex.pipe_id);
  need(section && saved && same(ex.area_m2, section.As_m2) && same(ex.section_modulus_m3, section.Z_m3), 'MAXIMUM_SECTION');
  ex.endpoints.forEach((end: Obj, index: number) => {
    const linkage = { station_fraction: end.station_fraction, functional_indices: end.functional_indices, functional_ids: end.functional_ids, actions: end.actions };
    need(same(linkage, saved.endpoints[index]) && end.station_fraction === index, 'MAXIMUM_ENDPOINT_SOURCE');
    end.functional_indices.forEach((n: number, i: number) => {
      need(n >= 0 && n < plan.functional_count && end.functional_ids[i] === functionalId(c.load_case_id, n), 'MAXIMUM_FUNCTIONAL_ID');
      need(end.actions[i].interval[0] <= end.actions[i].value && end.actions[i].value <= end.actions[i].interval[1], 'MAXIMUM_ACTION_INTERVAL');
    });
  });
  const indices = ex.endpoints.flatMap((e: Obj) => e.functional_indices);
  need(indices.length === 6 && unique(indices), 'MAXIMUM_FUNCTIONAL_BIJECTION');
}

/** Ordered binary64 recipe checks over retained transported actions, never a
 * hidden exact solver replay or a reconstruction from rounded public forces. */
export function validatePhysicsSourceMaximum(source: MechanicsResult, c: Obj, ex: Obj, rows: Map<string, Row>): void {
  const record = receiptCase(source, c.load_case_id); maximumLink(c, ex, record);
  const row = rows.get(ex.result_id);
  need(row?.kind === 'pipe_elastic_normal_stress_maximum_v2' && row.entity_ref === ex.pipe_id && same(row.basis_ref, record.basis_ref) && row.unit === 'Pa' && same(row.value, ex.value_pa), 'MAXIMUM_ROW');
  const ends = ex.endpoints;
  need(ends[0].actions[0].interval[0] <= ends[1].actions[0].interval[1] && ends[1].actions[0].interval[0] <= ends[0].actions[0].interval[1], 'CONSTANT_AXIAL_BOUND');
  for (const end of ends) { const [value, interval] = endpoint(ex.area_m2, ex.section_modulus_m3, end.actions); need(same(value, end.value_pa) && same(interval, end.interval_pa), 'MAXIMUM_ENDPOINT_RECIPE'); }
  const value = Math.max(ends[0].value_pa, ends[1].value_pa), lo = Math.max(ends[0].interval_pa[0], ends[1].interval_pa[0]), hi = Math.max(ends[0].interval_pa[1], ends[1].interval_pa[1]);
  const [absolute, relative] = bounds(value, lo, hi);
  need(same([ex.value_pa, ex.value_lower_pa, ex.value_upper_pa, ex.absolute_error_bound_pa, ex.relative_error_bound], [value, lo, hi, absolute, relative]) && ex.relative_limit === 1e-9, 'MAXIMUM_BOUNDS');
  need(ex.station_fraction === (ends[1].value_pa > ends[0].value_pa ? 1 : 0), 'MAXIMUM_WITNESS');
  const equal = (i: number, absoluteValue: boolean) => {
    const a = ends[0].actions[i], b = ends[1].actions[i];
    return a.interval[0] === a.interval[1] && b.interval[0] === b.interval[1] && (absoluteValue ? Math.abs(a.value) === Math.abs(b.value) : a.value === b.value);
  };
  let locations: Obj;
  if ([1, 2].every(i => equal(i, false))) locations = { kind: 'whole_span_constant' };
  else if (ends[0].interval_pa[0] > ends[1].interval_pa[1]) locations = { kind: 'strict_endpoint', endpoint: 'i' };
  else if (ends[1].interval_pa[0] > ends[0].interval_pa[1]) locations = { kind: 'strict_endpoint', endpoint: 'j' };
  else locations = { kind: 'endpoint_candidates', exact_tie_proven: [1, 2].every(i => equal(i, true)), interior_equal_possible: [1, 2].every(i => ends[0].actions[i].interval[0] <= ends[1].actions[i].interval[1] && ends[1].actions[i].interval[0] <= ends[0].actions[i].interval[1]) };
  need(same(ex.locations, locations), 'MAXIMUM_LOCATION_CLAIM');
}

function sectionFunctionals(c: Obj): void {
  const plan = c.source, ids = new Set<number>(), cid = c.basis_ref.ref_id;
  need(same(plan.section_functionals.map((m: Obj) => m.pipe_id), plan.member_ids), 'SECTION_FUNCTIONAL_MEMBERS');
  plan.section_functionals.forEach((member: Obj, memberIndex: number) => {
    need(same(member.stations.map((s: Obj) => s.station_fraction), [0, .25, .5, .75, 1]), 'SECTION_FUNCTIONAL_STATIONS');
    for (const station of member.stations) station.functional_indices.forEach((n: number, i: number) => {
      need(!ids.has(n) && n >= 0 && n < plan.functional_count && station.functional_ids[i] === functionalId(cid, n), 'SECTION_FUNCTIONAL_ID'); ids.add(n);
      const a = station.actions[i], [lo, hi] = a.interval;
      need(lo <= a.value && a.value <= hi, 'SECTION_FUNCTIONAL_INTERVAL');
      if (a.value === 0) need(lo === 0 && hi === 0, 'SECTION_ZERO_CERTIFICATE');
      else need(((lo > 0 && hi > 0) || (lo < 0 && hi < 0)) && Math.max(Math.abs(a.value - lo), Math.abs(hi - a.value)) / Math.min(Math.abs(lo), Math.abs(hi)) <= STRESS_LIMIT, 'SECTION_FUNCTIONAL_CRITERION');
    });
    [member.stations[0], member.stations[4]].forEach((s: Obj, i: number) => need(same(plan.endpoint_sections[memberIndex].endpoints[i], {
      station_fraction: s.station_fraction, functional_indices: [0, 4, 5].map(n => s.functional_indices[n]), functional_ids: [0, 4, 5].map(n => s.functional_ids[n]), actions: [0, 4, 5].map(n => s.actions[n]),
    }), 'ENDPOINT_SECTION_LINK'));
  });
}
function stress(source: MechanicsResult, c: Obj, row: Row, inputs: Row[]): void {
  const checks = c.section_stress_checks.filter((v: Obj) => v.result_id === row.id);
  need(checks.length === 1 && !inputs.length, 'STRESS_CHECK_ID');
  const check = checks[0], mapping: Record<string, [number, string]> = {
    element_local_axial_normal_stress: [0, 'axial_normal_stress'], element_local_bending_normal_stress_y: [4, 'bending_normal_stress_y'],
    element_local_bending_normal_stress_z: [5, 'bending_normal_stress_z'], element_local_torsional_shear_stress: [3, 'torsional_shear_stress'],
  };
  need(Object.hasOwn(mapping, row.kind) && row.metadata, 'STRESS_KIND');
  const [component, label] = mapping[row.kind], location = row.metadata.location;
  need(LOCATIONS.includes(location) && row.metadata.component === label && row.unit === 'MPa' && row.metadata.coordinate_system === 'element_local'
    && row.metadata.basis === (['end_i','end_j'].includes(location) ? 'recovered_from_local_element_stiffness' : 'recovered_from_open_mechanics_stress_components')
    && check.pipe_id === row.entity_ref && check.location === location && check.component === label && check.recipe_id === 'retained_source_straight_stress_v1', 'STRESS_SIGNATURE');
  const member = c.source.section_functionals.find((m: Obj) => m.pipe_id === row.entity_ref); need(member, 'STRESS_MEMBER');
  const station = member.stations[LOCATIONS.indexOf(location)];
  need(check.functional_index === station.functional_indices[component] && check.functional_id === station.functional_ids[component] && same(check.action, station.actions[component]), 'STRESS_FUNCTIONAL_LINK');
  const exact = physical(source).exact_cases.find((x: Obj) => x.load_case_id === c.basis_ref.ref_id), section = exact.pipe_sections.find((s: Obj) => s.pipe_id === row.entity_ref);
  const p = check.parameters;
  need(same(p, { area_m2: section.As_m2, section_modulus_m3: section.Z_m3, torsion_radius_m: section.ro_m, torsion_constant_m4: section.J_m4, pa_per_mpa: 1e6 }), 'STRESS_SECTION_PARAMETERS');
  const action = check.action.value, product = component === 3 ? action * p.torsion_radius_m : action;
  // Producer rows.rs checks T*radius before division: a later normal Pa/MPa
  // result cannot recover precision lost in a subnormal nonzero product.
  const pa = product / (component === 0 ? p.area_m2 : component === 3 ? p.torsion_constant_m4 : p.section_modulus_m3), value = pa / 1e6;
  need([product, pa, value].every(v => Number.isFinite(v) && (action === 0 ? v === 0 : Math.abs(v) >= 2 ** -1022)) && same(row.value, value), 'STRESS_RECIPE_VALUE');
}
export function validatePhysicsSourceDerived(source: MechanicsResult, c: Obj, treatment: Obj, row: Row, inputs: Row[]): void {
  const recipe = treatment.recipe_id;
  if (recipe === PHYSICS_SOURCE_MAX_BASIS) {
    const exact = physical(source).exact_cases.find((x: Obj) => x.load_case_id === c.basis_ref.ref_id), ex = exact.pipe_stress_extrema.find((x: Obj) => x.result_id === row.id);
    need(ex && !inputs.length, 'MAXIMUM_DERIVED_BINDING'); validatePhysicsSourceMaximum(source, exact, ex, new Map(source.results.map(r => [r.id, r]))); return;
  }
  if (recipe === 'retained_source_straight_stress_v1') { stress(source, c, row, inputs); return; }
  need(['support_force_norm_scaled_checked_v1', 'support_moment_norm_scaled_checked_v1'].includes(recipe), 'DERIVED_RECIPE');
  const checks = c.derived_checks.filter((x: Obj) => x.result_id === row.id); need(checks.length === 1, 'NORM_CHECK_ID');
  const check = checks[0], force = recipe === 'support_force_norm_scaled_checked_v1', components = force ? ['Fx', 'Fy', 'Fz'] : ['Mx', 'My', 'Mz'];
  need(check.recipe_id === recipe && row.kind === (force ? 'support_reaction_force_magnitude_v2' : 'support_reaction_moment_magnitude_v2') && check.support_id === row.entity_ref && row.unit === (force ? 'N' : 'N*m'), 'NORM_SIGNATURE');
  need(inputs.length === 3 && inputs.every((r, i) => r.metadata?.component === components[i] && r.kind === 'support_reaction_component_v2' && r.entity_ref === row.entity_ref && r.unit === row.unit), 'NORM_INPUTS');
  inputs.forEach((r, i) => {
    const projections = c.projections.filter((p: Obj) => p.result_id === r.id); need(projections.length === 1, 'NORM_PROJECTION');
    const p = projections[0];
    need(check.functional_ids[i] === p.functional_id && same(check.values[i], p.value) && same(check.intervals[i], p.interval)
      && check.functional_ids[i] === functionalId(c.basis_ref.ref_id, check.functional_indices[i]) && check.functional_indices[i] < c.source.functional_count, 'NORM_PROJECTION_BINDING');
  });
  need(unique(check.functional_indices), 'NORM_FUNCTIONAL_BIJECTION');
  const intervals = check.intervals.map(absInterval), lo = normBound(intervals.map((p: number[]) => p[0]), false), hi = normBound(intervals.map((p: number[]) => p[1]), true), value = scaledNorm(check.values);
  const [absolute, relative] = bounds(value, lo, hi);
  need(same(row.value, value) && same(check.value, value) && same(check.interval, [lo, hi]) && same(check.absolute_error_bound, absolute) && same(check.relative_error_bound, relative) && check.relative_limit === 1e-9, 'NORM_RECIPE');
}

/** Case-specific model custody and row coverage. Unit conversion stays with the
 * maintained WASM authority, never a second JavaScript unit registry. */
export async function validatePhysicsSourceCase(source: MechanicsResult, c: Obj, invocation: SourceBlockInvocation): Promise<void> {
  need(['sensitive', 'rejected'].includes(c.ordinary_attempt.outcome), 'SOURCE_FALLBACK_TRIGGER');
  const exact = physical(source).exact_cases.find((x: Obj) => x.load_case_id === c.basis_ref.ref_id), rhs = exact.pressure_rhs_assembly;
  need(!rhs.groups.length && ['assembled_pressure_rhs_global', 'rounded_cap_rhs_global', 'rounded_poisson_rhs_global'].every(k => rhs[k].every((v: number) => v === 0)), 'SOURCE_PRESSURE_RHS');
  sectionFunctionals(c);
  for (const [checks, recipes, count] of [[c.derived_checks, ['support_force_norm_scaled_checked_v1', 'support_moment_norm_scaled_checked_v1'], 2 * c.source.support_ids.length], [c.section_stress_checks, ['retained_source_straight_stress_v1'], 20 * c.source.member_ids.length]] as const) {
    const expected = c.rows.filter((r: Obj) => (recipes as readonly string[]).includes(r.recipe_id)).map((r: Obj) => r.result_id);
    need(expected.length === count && checks.length === count && unique(checks.map((x: Obj) => x.result_id)) && checks.every((x: Obj) => expected.includes(x.result_id)), 'DERIVED_CHECK_COVERAGE');
  }
  const request = invocation.request as Obj, model = request.model as Obj, actual = model.load_cases.find((x: Obj) => x.id === c.basis_ref.ref_id);
  need(model.schema_version === '0.3.0' && same(model.pressure_contract, { version: '2.0.0', mode: PROFILE }), 'ACTUAL_EXACT_PROFILE');
  need(actual && same(actual.pressure_regions, []), 'ACTUAL_EMPTY_PRESSURE_INVENTORY');
  need(!(model.components?.length) && !(model.combinations?.length) && actual.equivalent_static == null, 'ACTUAL_SOURCE_FAMILY');
  need((actual.primitive_loads ?? []).every((l: Obj) => l.target?.type === 'node' && l.category !== 'thermal'), 'ACTUAL_NODAL_LOADS');
  need(model.supports.every((s: Obj) => s.nonlinear == null && s.family !== 'constant_effort_support' && s.hanger?.constant_load == null), 'ACTUAL_SUPPORT_FAMILY');
  await validateAuthoredCaseFacts(source, invocation);
  supportLaws(c, model, new Map(source.results.map(r => [r.id, r])));
}

/** Producer normalization uses the shared units crate before selecting a point
 * or a strict interior bracket. Apply it to every case, including ordinary
 * cases transported alongside source fallback; no material default is inferred. */
async function validateAuthoredCaseFacts(source: MechanicsResult, invocation: SourceBlockInvocation): Promise<void> {
  const request = invocation.request as Obj, model = request.model as Obj;
  const materials: Obj[] = request.materials?.length ? request.materials : model.materials;
  const engine = await loadWasmEngine();
  const convert = (q: Obj, unit: string, dimension: string): number => {
    need(q && typeof q === 'object', 'ACTUAL_QUANTITY_REQUIRED');
    const output = JSON.parse(engine.convertDisplayQuantitiesJson(checkedJsonText({ items: [{ id: 'source', value: q.value, from_unit: q.unit, to_unit: unit, dimension_id: dimension }] })));
    need(!output.error && output.items?.length === 1 && output.items[0].id === 'source' && output.items[0].status === 'converted' && output.items[0].unit === unit && Number.isFinite(output.items[0].value), 'ACTUAL_QUANTITY_UNITS'); return output.items[0].value;
  };
  const normalizeMaterial = (authored: Obj, base: boolean) => {
    // Retained authored G is nonauthoritative in this profile, but its explicit
    // unit must still survive the producer's ordinary normalization phase.
    if (authored.shear_modulus != null) convert(authored.shear_modulus, 'Pa', 'stress');
    return {
      id: authored.id, E_pa: base || authored.elastic_modulus != null ? convert(authored.elastic_modulus, 'Pa', 'stress') : undefined,
      nu: authored.poisson_ratio, alpha: authored.thermal_expansion_coefficient != null ? convert(authored.thermal_expansion_coefficient, '1/K', 'thermal_expansion_coefficient') : undefined,
      temperature_kelvin: authored.temperature != null ? convert(authored.temperature, 'K', 'temperature') : undefined,
    };
  };
  const normalized = materials.map(authored => ({
    authored, ...normalizeMaterial(authored, true),
    points: (authored.temperature_points ?? []).map((point: Obj) => normalizeMaterial(point, false)),
  }));
  const validPair = (point: { E_pa?: number; nu?: Obj } | undefined) => {
    need(point && point.E_pa !== undefined && Number.isFinite(point.E_pa) && point.E_pa > 0
      && point.nu?.unit === '1' && Number.isFinite(point.nu.value) && point.nu.value > -1 && point.nu.value < .5, 'ACTUAL_MATERIAL_POINT_INPUTS');
    need(physicsSourceMaterialPairRepresentable(point.E_pa, point.nu.value), 'ACTUAL_MATERIAL_PAIR_RANGE');
  };
  for (const exact of physical(source).exact_cases as Obj[]) {
    const actual = model.load_cases.find((c: Obj) => c.id === exact.load_case_id); need(actual, 'ACTUAL_CASE');
    need(actual.modulus_basis_ref == null || actual.modulus_basis_temperature == null, 'ACTUAL_MATERIAL_SELECTION_CONFLICT');
    model.pipe_segments.forEach((pipe: Obj, i: number) => {
      const section = exact.pipe_sections[i], material = exact.pipe_materials[i];
      need(pipe.id === section.pipe_id && pipe.id === material.pipe_id && pipe.material === material.material_id, 'ACTUAL_MATERIAL_MEMBER');
      const input = pipe.section;
      need(same(convert(input.outside_diameter, 'm', 'length'), section.outside_diameter_m)
        && same(convert(input.wall_thickness, 'm', 'length') - (input.mill_tolerance ? convert(input.mill_tolerance, 'm', 'length') : 0), section.effective_wall_thickness_m), 'ACTUAL_SOURCE_GEOMETRY');
      const authored = normalized.find(m => m.authored.id === pipe.material);
      need(authored && authored.authored.constitutive_basis === material.constitutive_basis, 'ACTUAL_CONSTITUTIVE_BASIS');
      validPair(authored);
      let selected: { E_pa?: number; nu?: Obj; alpha?: number } | undefined = authored;
      if (actual.modulus_basis_ref != null) selected = authored.points.find((p: Obj) => p.id === actual.modulus_basis_ref);
      else if (actual.modulus_basis_temperature != null) {
        const points: Obj[] = authored.points.filter((p: Obj) => p.temperature_kelvin !== undefined).sort((a: Obj, b: Obj) => a.temperature_kelvin - b.temperature_kelvin);
        need(unique(points.map(p => p.temperature_kelvin)), 'ACTUAL_MATERIAL_TEMPERATURE_DUPLICATE');
        const t = convert(actual.modulus_basis_temperature, 'K', 'temperature');
        const index = points.findIndex((p, j) => j < points.length - 1 && p.temperature_kelvin < t && t < points[j + 1].temperature_kelvin);
        need(index >= 0, 'ACTUAL_MATERIAL_TEMPERATURE');
        const a = points[index], b = points[index + 1], f = (t - a.temperature_kelvin) / (b.temperature_kelvin - a.temperature_kelvin);
        validPair(a); validPair(b);
        selected = { E_pa: (1 - f) * a.E_pa + f * b.E_pa, nu: { value: (1 - f) * a.nu.value + f * b.nu.value, unit: '1' },
          alpha: a.alpha !== undefined && b.alpha !== undefined ? (1 - f) * a.alpha + f * b.alpha : undefined };
      }
      validPair(selected);
      need(selected && selected.nu?.unit === '1' && same(selected.E_pa, material.E_pa) && same(selected.nu.value, material.nu), 'ACTUAL_SELECTED_MATERIAL');
      const thermalConsumed = (actual.primitive_loads ?? []).some((load: Obj) => load.category === 'thermal' && ['temperature_change', 'temperature_interval'].includes(load.dimension) && load.target?.type === 'element' && load.target.pipe === pipe.id);
      need(material.thermal_consumed === thermalConsumed && (thermalConsumed ? selected.alpha !== undefined && Number.isFinite(selected.alpha) && same(selected.alpha, material.alpha_per_kelvin) : material.alpha_per_kelvin === null), 'ACTUAL_SELECTED_THERMAL');
      if (exact.recovery_method === EXACT) need(material.thermal_consumed === false, 'ACTUAL_SOURCE_THERMAL');
    });
  }
}

function supportLaws(c: Obj, model: Obj, rows: Map<string, Row>): void {
  const plan = c.source, ideals = new Set<number>(), springs = new Set<string>(), supported = new Set<string>();
  need(same(c.supports.map((s: Obj) => s.support_id), plan.support_ids), 'SUPPORT_COVERAGE');
  for (const s of c.supports) {
    const authored = model.supports.find((a: Obj) => a.id === s.support_id), node = model.nodes.findIndex((n: Obj) => n.id === s.node_id);
    need(authored?.node === s.node_id && node >= 0 && s.components.length === 6 && unique(s.components.map((p: Obj) => p.component)), 'SUPPORT_NODE');
    for (const part of s.components) {
      const slot = ['Fx', 'Fy', 'Fz', 'Mx', 'My', 'Mz'].indexOf(part.component), dof = node * 6 + slot, row = rows.get(part.result_id), projection = c.projections.find((p: Obj) => p.result_id === part.result_id);
      need(slot >= 0 && row && projection && !supported.has(row.id), 'SUPPORT_COMPONENT'); supported.add(row.id);
      const stiffness = authored.stiffness ?? authored.hanger?.stiffness, spring = ['spring', 'variable_spring_hanger', 'spring_hanger'].includes(authored.family) || ['variable_spring_hanger', 'spring_hanger'].includes(authored.hanger?.hanger_type);
      const name = ['UX', 'UY', 'UZ', 'RX', 'RY', 'RZ'][slot], expected = spring && stiffness?.dof === name ? 'ground_spring' : !spring && authored.restraints?.includes(name) ? 'ideal_constraint' : 'structural_zero';
      need(part.action_terms.length === 1 && part.action_terms[0].kind === expected, 'ACTUAL_SUPPORT_LAW');
      const term = part.action_terms[0]; need(term.global_dof === dof && dof < plan.dof_count && term.source_id === s.support_id, 'SUPPORT_ACTION_OWNER');
      if (expected === 'ideal_constraint') { need(plan.prescribed_dofs.includes(dof) && !ideals.has(dof), 'IDEAL_ATTRIBUTION_AMBIGUOUS'); ideals.add(dof); }
      else if (expected === 'ground_spring') { const key = JSON.stringify([s.support_id, dof]); need(!springs.has(key), 'SPRING_ATTRIBUTION_DUPLICATE'); springs.add(key); }
      else need(row.value === 0 && projection.basis === 'exact_zero', 'STRUCTURAL_ZERO');
    }
  }
  need(ideals.size === plan.prescribed_dofs.length && plan.prescribed_dofs.every((d: number) => ideals.has(d)), 'IDEAL_SOURCE_COVERAGE');
  const actions = c.projections.filter((p: Obj) => p.quantity === 'support_action_component');
  need(actions.length === supported.size && actions.every((p: Obj) => supported.has(p.result_id)), 'SUPPORT_RESULT_BIJECTION');
}
