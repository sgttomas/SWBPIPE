import unicodeSparse from '../../../../../fixtures/product_preview/physics_source/n05_unicode-sparse_interactive.raw.json?raw';
import unicodeDense from '../../../../../fixtures/product_preview/physics_source/n05_unicode-dense_scrutiny.raw.json?raw';
import unicodeRequest from '../../../../../fixtures/product_preview/physics_source/n05_unicode.request.json?raw';
/// <reference types="vite/client" />
import { describe, expect, it } from 'vitest';
import n05Sparse from '../../../../../fixtures/product_preview/physics_source/n05-sparse_interactive.raw.json?raw';
import n05Dense from '../../../../../fixtures/product_preview/physics_source/n05-dense_scrutiny.raw.json?raw';
import n06Sparse from '../../../../../fixtures/product_preview/physics_source/n06-sparse_interactive.raw.json?raw';
import n06Dense from '../../../../../fixtures/product_preview/physics_source/n06-dense_scrutiny.raw.json?raw';
import fieldsSparse from '../../../../../fixtures/product_preview/physics_source/fields-sparse_interactive.raw.json?raw';
import fieldsDense from '../../../../../fixtures/product_preview/physics_source/fields-dense_scrutiny.raw.json?raw';
import mixedSparse from '../../../../../fixtures/product_preview/physics_source/mixed-sparse_interactive.raw.json?raw';
import mixedDense from '../../../../../fixtures/product_preview/physics_source/mixed-dense_scrutiny.raw.json?raw';
import n05UnitsSparse from '../../../../../fixtures/product_preview/physics_source/n05_units-sparse_interactive.raw.json?raw';
import n05UnitsDense from '../../../../../fixtures/product_preview/physics_source/n05_units-dense_scrutiny.raw.json?raw';
import mixedUnitsSparse from '../../../../../fixtures/product_preview/physics_source/mixed_units-sparse_interactive.raw.json?raw';
import mixedUnitsDense from '../../../../../fixtures/product_preview/physics_source/mixed_units-dense_scrutiny.raw.json?raw';
import n05UnitsRequest from '../../../../../fixtures/product_preview/physics_source/n05_units.request.json?raw';
import mixedUnitsRequest from '../../../../../fixtures/product_preview/physics_source/mixed_units.request.json?raw';
import n05Request from '../../../../../fixtures/product_preview/physics_source/n05.request.json?raw';
import n06Request from '../../../../../fixtures/product_preview/physics_source/n06.request.json?raw';
import fieldsRequest from '../../../../../fixtures/product_preview/physics_source/fields.request.json?raw';
import mixedRequest from '../../../../../fixtures/product_preview/physics_source/mixed.request.json?raw';
import oldSourceRaw from '../../../../../fixtures/product_preview/source_blocks/n05-sparse_interactive.raw.json?raw';
import oldSourceRequest from '../../../../../fixtures/product_preview/source_blocks/n05-sparse_interactive.request.json?raw';
import physicsRaw from '../../../../../fixtures/results/physics_connected_ui_mechanics_sparse.json?raw';
import { canonicalSha256HexCheckedV1 } from '../../services/hashService';
import type { MechanicsResult, PreviewModel } from '../../types';
import {
  PHYSICS_SOURCE_CONTRACT_ID, PHYSICS_SOURCE_MAX_BASIS, physicsSourceReceiptShape,
  validatePhysicsSourceRecovery, physicsSourceStanding, retainedPhysicsSourceInvocation,
  physicsSourceModeMatches, validatePhysicsSourceTransportMetadata, validatePhysicsSourceDerived, validatePhysicsSourceCase,
  physicsSourceMaterialPairRepresentable,
} from './physicsSourceRecovery';
import {
  sourceBlockReceiptShape, sourceBlockStanding, validateSourceBlockRecovery, physicsSourcePhysicalShape, physicsEvidenceTransportShape,
  validateSourceBlockTransportMetadata, type SourceBlockInvocation,
} from './sourceBlockRecovery';
import { validatePhysicsEvidence, validatePhysicsTransportMetadata } from './physicsResultEvidence';

// Unmodified producer capture pairs, not simulated Current/native UI evidence.
// The actual native origin gate is separate from these receipt-consistency tests.
const pairs = [
  ['n05', 'sparse_interactive', n05Sparse, n05Request], ['n05', 'dense_scrutiny', n05Dense, n05Request],
  ['n06', 'sparse_interactive', n06Sparse, n06Request], ['n06', 'dense_scrutiny', n06Dense, n06Request],
  ['fields', 'sparse_interactive', fieldsSparse, fieldsRequest], ['fields', 'dense_scrutiny', fieldsDense, fieldsRequest],
  ['mixed', 'sparse_interactive', mixedSparse, mixedRequest], ['mixed', 'dense_scrutiny', mixedDense, mixedRequest],
  ['n05_units', 'sparse_interactive', n05UnitsSparse, n05UnitsRequest], ['n05_units', 'dense_scrutiny', n05UnitsDense, n05UnitsRequest],
  ['mixed_units', 'sparse_interactive', mixedUnitsSparse, mixedUnitsRequest], ['mixed_units', 'dense_scrutiny', mixedUnitsDense, mixedUnitsRequest],
  ['n05_unicode', 'sparse_interactive', unicodeSparse, unicodeRequest], ['n05_unicode', 'dense_scrutiny', unicodeDense, unicodeRequest],
] as const;
function received(index = 0) {
  const [name, mode, raw, requestText] = pairs[index], source = JSON.parse(raw) as MechanicsResult, request = JSON.parse(requestText);
  const invocation: SourceBlockInvocation = { request, solver_mode: mode };
  return { name, mode, source, request, invocation, model: request.model as PreviewModel };
}
const evidence = (s: MechanicsResult) => s.contract_evidence as any;
const receipt = (s: MechanicsResult) => s.source_block_recovery as any;
const domainHash = (domain: string, payload: unknown) => canonicalSha256HexCheckedV1({ domain, payload });
async function rehashInvalid(source: MechanicsResult) {
  // Mutations are deliberately invalid negative controls, even after their
  // statement digests are recomputed. No mutation is admitted as producer data.
  const r = receipt(source);
  for (const c of r.body.cases) {
    const exact = evidence(source).exact_cases.find((x: any) => x.load_case_id === c.basis_ref.ref_id);
    c.physical_evidence_sha256 = await domainHash('physics_source_case_evidence_v1', { exact_case: exact, pressure: evidence(source).pressure.filter((p: any) => p.load_case_id === c.basis_ref.ref_id) });
  }
  const publication = { ...source }; delete publication.source_block_recovery;
  r.body.publication_sha256 = await domainHash('source_blocks_publication_v1', publication);
  r.receipt_sha256 = await domainHash('source_blocks_receipt_v1', r.body);
}

describe('physics-source-1 actual producer captures and unit variants', () => {
  it.each(pairs.map((p, index) => [p[0], p[1], index] as const))('%s %s preserves statements and registers only receipt consistency', async (_name, _mode, index) => {
    const { source, model, invocation, mode } = received(index), before = structuredClone(source);
    expect(source.producer!.semantic_contract_id).toBe(PHYSICS_SOURCE_CONTRACT_ID);
    expect(receipt(source).body.cases.filter((c: any) => c.source).every((c: any) => c.work.limit === 8_000_000)).toBe(true);
    expect(physicsSourceReceiptShape(source.source_block_recovery)).toBe(true);
    expect(sourceBlockReceiptShape(source.source_block_recovery)).toBe(false);
    expect(physicsSourceStanding(source, model).eligible).toBe(false);
    await validatePhysicsSourceTransportMetadata(source);
    expect(physicsSourceStanding(source, model).eligible).toBe(false);
    await expect(validatePhysicsSourceRecovery(source, invocation)).resolves.toEqual({ eligible: true, findings: [] });
    expect(source).toEqual(before);
    expect(physicsSourceStanding(source, model).eligible).toBe(true);
    expect(sourceBlockStanding(source, model).eligible).toBe(false);
    expect(physicsSourceModeMatches(source, model, mode)).toBe(true);
    expect(physicsSourceModeMatches(source, model, mode === 'dense_scrutiny' ? 'sparse_interactive' : 'dense_scrutiny')).toBe(false);
    expect(retainedPhysicsSourceInvocation(source, model)).toEqual(invocation);
    for (const c of receipt(source).body.cases.filter((c: any) => c.source)) {
      expect(c.source.section_functionals.every((m: any) => m.stations.length === 5 && m.stations.every((s: any) => s.actions.length === 6))).toBe(true);
      expect(c.supports.every((s: any) => s.components.length === 6)).toBe(true);
      expect(c.section_stress_checks).toHaveLength(20 * c.source.member_ids.length);
      expect(c.derived_checks).toHaveLength(2 * c.source.support_ids.length);
    }
  });
  it('does not promote a mixed source fallback ordinary quality case', async () => {
    const { source, invocation } = received(6), before = structuredClone(source.numerical_quality);
    await validatePhysicsSourceRecovery(source, invocation);
    expect(source.numerical_quality).toEqual(before);
    expect(source.numerical_quality!.cases.some(c => c.solve_quality !== 'checks_passed')).toBe(true);
    expect(receipt(source).body.cases.map((c: any) => c.selected_method)).toEqual(['retained_source_blocks_exact_v1', 'ordinary_sparse_structural_v1']);
  });
  it('retains old source4M and old physics namespace boundaries', async () => {
    const old = JSON.parse(oldSourceRaw) as MechanicsResult, request = JSON.parse(oldSourceRequest);
    expect(sourceBlockReceiptShape(old.source_block_recovery)).toBe(true);
    expect(physicsSourceReceiptShape(old.source_block_recovery)).toBe(false);
    await expect(validateSourceBlockRecovery(old, { request, solver_mode: 'sparse_interactive' })).resolves.toMatchObject({ eligible: true });
    await validateSourceBlockTransportMetadata(old);
    const oldTooLarge = JSON.parse(oldSourceRaw); oldTooLarge.source_block_recovery.body.cases[0].work.limit = 8_000_000;
    expect(sourceBlockReceiptShape(oldTooLarge.source_block_recovery)).toBe(false);
    const pure = JSON.parse(physicsRaw) as MechanicsResult;
    expect(() => validatePhysicsEvidence(pure)).not.toThrow();
    expect(() => validatePhysicsTransportMetadata(pure.contract_evidence)).not.toThrow();
    const joined = received().source;
    expect(() => validatePhysicsEvidence(joined)).toThrow('UNSUPPORTED_SOURCE_NAMESPACE');
    expect(() => validatePhysicsTransportMetadata(joined.contract_evidence)).toThrow('TRANSPORT_SHAPE');
    await expect(validateSourceBlockTransportMetadata(joined)).rejects.toThrow();
    await expect(validateSourceBlockRecovery(joined, received().invocation)).rejects.toThrow('CONTRACT');
  });
});

describe('physics-source malformed statements do not acquire a registration', () => {
  const mutations: [string, (s: MechanicsResult) => void][] = [
    ['wrong receipt policy', s => { receipt(s).body.policy = 'SOURCE-BLOCKS-1'; }],
    ['work over8M', s => { receipt(s).body.cases[0].work.limit = 8_000_001; }],
    ['Boolean number', s => { receipt(s).body.cases[0].work.limit = true; }],
    ['wrong recovery method', s => { evidence(s).exact_cases[0].recovery_method = 'ordinary_sparse_structural_v1'; }],
    ['foreign namespace', s => { (s as any).carrier_evidence = null; }],
    ['wrong member geometry', s => { evidence(s).exact_cases[0].pipe_sections[0].As_m2 *= 2; }],
    ['wrong selected material', s => { evidence(s).exact_cases[0].pipe_materials[0].E_pa *= 2; evidence(s).exact_cases[0].pipe_materials[0].G_pa *= 2; }],
    ['retained identity mismatch', s => { receipt(s).body.cases[0].source.retained_identity_sha256 = '0'.repeat(64); }],
    ['missing functional station', s => { receipt(s).body.cases[0].source.section_functionals[0].stations.pop(); }],
    ['endpoint section mismatch', s => { receipt(s).body.cases[0].source.endpoint_sections[0].endpoints[0].functional_ids[0] += ':foreign'; }],
    ['section stress action mismatch', s => { receipt(s).body.cases[0].section_stress_checks[0].action.value = 1; }],
    ['section stress parameter mismatch', s => { receipt(s).body.cases[0].section_stress_checks[0].parameters.area_m2 *= 2; }],
    ['missing stress check', s => { receipt(s).body.cases[0].section_stress_checks.pop(); }],
    ['norm projection mismatch', s => { receipt(s).body.cases[0].derived_checks[0].values[0] += 1; }],
    ['norm interval mismatch', s => { receipt(s).body.cases[0].derived_checks[0].interval[1] += 1; }],
    ['missing norm check', s => { receipt(s).body.cases[0].derived_checks.pop(); }],
    ['support law mismatch', s => { receipt(s).body.cases[0].supports[0].components[0].action_terms[0].kind = 'ground_spring'; }],
    ['maximum bound mismatch', s => { evidence(s).exact_cases[0].pipe_stress_extrema[0].value_upper_pa += 1; }],
    ['maximum location overclaim', s => { evidence(s).exact_cases[0].pipe_stress_extrema[0].locations = { kind: 'strict_endpoint', endpoint: 'j' }; }],
    ['maximum source row mutation', s => { s.results.find(r => r.kind === 'pipe_elastic_normal_stress_maximum_v2')!.value += 1; }],
    ['maximum headline mutation', s => { s.summary.max_open_formula_stress!.value += 1; }],
    ['old ordinary treatment token', s => { receipt(s).body.cases[0].rows[0].treatment = 'ordinary_checked'; }],
    ['pressure RHS source force', s => { evidence(s).exact_cases[0].pressure_rhs_assembly.assembled_pressure_rhs_global[0] += 1; }],
  ];
  it.each(mutations)('rejects rehashed %s', async (_name, mutate) => {
    const { source, invocation, model } = received(); mutate(source); await rehashInvalid(source);
    await expect(validatePhysicsSourceRecovery(source, invocation)).rejects.toThrow();
    expect(physicsSourceStanding(source, model).eligible).toBe(false);
    expect(retainedPhysicsSourceInvocation(source, model)).toBeNull();
  });
  it('rejects raw mutation without its publication hash and transported case mutation without its case hash', async () => {
    const a = received(); a.source.run_id += ':changed';
    await expect(validatePhysicsSourceRecovery(a.source, a.invocation)).rejects.toThrow('PUBLICATION_HASH');
    const b = received(); evidence(b.source).exact_cases[0].material_basis += ':changed';
    await expect(validatePhysicsSourceTransportMetadata(b.source)).rejects.toThrow('PHYSICAL_CASE_HASH');
  });
  it('never treats a future method or a copied source as a validated producer', async () => {
    const { source, invocation, model } = received();
    source.producer!.semantic_contract_id += '/future';
    await expect(validatePhysicsSourceRecovery(source, invocation)).rejects.toThrow('CONTRACT');
    const good = received(); await validatePhysicsSourceRecovery(good.source, good.invocation);
    for (const clone of [structuredClone(good.source), JSON.parse(JSON.stringify(good.source))]) {
      (clone as any).physics_source_validated = true;
      expect(physicsSourceStanding(clone, model).eligible).toBe(false);
    }
  });
});

describe('composite invocation and bit-sensitive lifetime', () => {
  it.each(['mode', 'model', 'request'] as const)('rejects an independently captured %s mismatch', async changed => {
    const { source, invocation } = received(), altered = structuredClone(invocation) as any;
    if (changed === 'mode') altered.solver_mode = 'dense_scrutiny';
    if (changed === 'model') altered.request.model.nodes[0].position.x = 1;
    if (changed === 'request') altered.request.extra_received_key = 1;
    await expect(validatePhysicsSourceRecovery(source, altered)).rejects.toThrow();
  });
  it('accepts only the observed caller JSON normalization and invalidates later signed-zero changes', async () => {
    const { source, invocation, model } = received(), caller = structuredClone(model); caller.nodes[0].position.x = -0;
    await validatePhysicsSourceRecovery(source, invocation, caller);
    expect(physicsSourceStanding(source, caller).eligible).toBe(true);
    expect(physicsSourceStanding(source, model).eligible).toBe(true);
    caller.nodes[0].position.x = 0;
    expect(physicsSourceStanding(source, model).eligible).toBe(false);
  });
  it.each(['source_zero', 'dispatch_zero', 'source_value', 'model_value'] as const)('invalidates %s after checking', async change => {
    const { source, invocation, model } = received(); await validatePhysicsSourceRecovery(source, invocation);
    if (change === 'source_zero') { const row = source.results.find(r => Object.is(r.value, 0)); expect(row).toBeDefined(); row!.value = -0; }
    if (change === 'dispatch_zero') model.nodes[0].position.x = -0;
    if (change === 'source_value') source.results[0].value += 1;
    if (change === 'model_value') model.nodes[0].position.x = .125;
    expect(physicsSourceStanding(source, model).eligible).toBe(false);
    expect(retainedPhysicsSourceInvocation(source, model)).toBeNull();
  });
  it('rejects mutation during asynchronous validation and unobserved caller aliases', async () => {
    const { source, invocation, model } = received();
    const pending = validatePhysicsSourceRecovery(source, invocation); source.run_id += ':racing';
    await expect(pending).rejects.toThrow('CHANGED_DURING_VALIDATION');
    const other = received(), caller = structuredClone(model); caller.nodes[0].position.x = 1;
    await expect(validatePhysicsSourceRecovery(other.source, other.invocation, caller)).rejects.toThrow('CALLER_DISPATCH_NORMALIZATION_MISMATCH');
  });
  it('does not grant eligibility from an independently checked transport packet', async () => {
    const { source, model } = received();
    const transport = { producer: source.producer, contract_evidence: source.contract_evidence, source_block_recovery: source.source_block_recovery };
    await validatePhysicsSourceTransportMetadata(transport);
    expect(physicsSourceStanding(source, model).eligible).toBe(false);
    expect(source.results.some(r => r.metadata?.basis === PHYSICS_SOURCE_MAX_BASIS)).toBe(true);
  });
});


describe('CR01 normalized material custody and CR03 torsion intermediate range', () => {
  it.each([
    [Number.MIN_VALUE, .25, false], [Number.MIN_VALUE, 0, false],
    [Number.MIN_VALUE, -Number.EPSILON / 2, true], [Number.MIN_VALUE, -.25, true],
    [2 * Number.MIN_VALUE, .25, true],
    [Number.MAX_VALUE, -.5, true], [Number.MAX_VALUE, -.5000000000000001, false],
    [Number.MAX_VALUE / 2, -.75, true], [Number.MAX_VALUE / 2, -.7500000000000001, false],
  ])('matches the producer Scaled domain for E=%s nu=%s', (e, nu, expected) => {
    // Method-contract boundary checks; no fabricated receipt or physical oracle.
    expect(physicsSourceMaterialPairRepresentable(e as number, nu as number)).toBe(expected);
  });
  it('rejects a consumed lower endpoint with unrepresentable G even when interpolation stays E2000', async () => {
    const { source, invocation } = received(10), material = (invocation.request as any).model.materials[0];
    material.temperature_points[0].elastic_modulus = { value: Number.MIN_VALUE, unit: 'Pa' };
    material.temperature_points[1].elastic_modulus = { value: 4000, unit: 'Pa' };
    expect(.5 * Number.MIN_VALUE + .5 * 4000).toBe(2000);
    // Invalid input copy exercises the actual normalized-material check without
    // treating the altered invocation or untouched capture as a fresh solve.
    await expect(validatePhysicsSourceCase(source, receipt(source).body.cases[0], invocation)).rejects.toThrow('ACTUAL_MATERIAL_PAIR_RANGE');
    expect(physicsSourceStanding(source, (invocation.request as any).model as PreviewModel).eligible).toBe(false);
  });
  it.each([8, 9, 10, 11])('checks unchanged actual unit capture %s against normalized E/nu facts', async index => {
    const { source, invocation, model } = received(index), before = structuredClone(source);
    expect((model.materials![0] as any).elastic_modulus).toEqual({ value: 200, unit: 'GPa' });
    await expect(validatePhysicsSourceRecovery(source, invocation)).resolves.toEqual({ eligible: true, findings: [] });
    expect(evidence(source).exact_cases[0].pipe_materials[0].E_pa).toBe(200_000_000_000);
    if (index >= 10) {
      const ordinary = evidence(source).exact_cases[1];
      expect((model.load_cases[1] as any).modulus_basis_temperature).toEqual({ value: 68, unit: 'degF' });
      expect(ordinary.pipe_materials[0].E_pa).toBe(2000);
      expect(ordinary.material_basis).toContain('interpolated:point:lower..point:upper;temperature_kelvin=293.15');
      expect(ordinary.pipe_materials[0].nu).toBe(.25);
    }
    expect(source).toEqual(before);
  });
  it('rejects rehashed ordinary-case material facts that contradict the same captured mixed-unit request', async () => {
    const { source, invocation } = received(10), c = evidence(source).exact_cases[1], pressure = evidence(source).pressure.find((p: any) => p.load_case_id === c.load_case_id);
    // Preserve duplicated case/region facts and coherent E/nu/G. Only the actual
    // request's selected normalized material establishes that E remains wrong.
    for (const m of [...c.pipe_materials, ...pressure.materials]) { m.E_pa *= 2; m.G_pa *= 2; }
    await rehashInvalid(source);
    await expect(validatePhysicsSourceRecovery(source, invocation)).rejects.toThrow('ACTUAL_SELECTED_MATERIAL');
  });
  it.each(['endpoint', 'duplicate'] as const)('refuses temperature-route %s selection in the isolated actual-case binding check', async mode => {
    const { source, invocation } = received(10), request = invocation.request as any;
    // Negative subcheck only: the altered input is never submitted as an
    // authenticated invocation or admitted through the full receipt validator.
    if (mode === 'endpoint') request.model.load_cases[1].modulus_basis_temperature = { value: 0, unit: 'degC' };
    else request.model.materials[0].temperature_points[1].temperature = { value: 0, unit: 'degC' };
    await expect(validatePhysicsSourceCase(source, receipt(source).body.cases[0], invocation)).rejects.toThrow('ACTUAL_MATERIAL_TEMPERATURE');
  });
  it.each([
    ['base E dimension', (m: any) => { m.elastic_modulus.unit = 'm'; }],
    ['retained G dimension', (m: any) => { m.shear_modulus.unit = 'm'; }],
    ['point E dimension', (m: any) => { m.temperature_points[0].elastic_modulus.unit = 'degC'; }],
    ['point temperature dimension', (m: any) => { m.temperature_points[0].temperature.unit = 'm'; }],
    ['base alpha dimension', (m: any) => { m.thermal_expansion_coefficient = { value: 1e-5, unit: 'Pa' }; }],
    ['point alpha dimension', (m: any) => { m.temperature_points[0].thermal_expansion_coefficient = { value: 1e-5, unit: 'Pa' }; }],
    ['nu unit alias', (m: any) => { m.poisson_ratio.unit = 'none'; }],
    ['nu range', (m: any) => { m.poisson_ratio.value = .5; }],
    ['point nu unit', (m: any) => { m.temperature_points[0].poisson_ratio.unit = 'Pa'; }],
  ] as const)('refuses %s through the actual shared-unit/case-binding subcheck', async (_label, mutate) => {
    const { source, invocation } = received(10);
    mutate((invocation.request as any).model.materials[0]);
    // These are invalid input mutations. Receipt/publication bytes are retained;
    // only the model-binding subcheck is exercised to avoid masking the unit
    // diagnostic with the separately tested whole invocation digest refusal.
    await expect(validatePhysicsSourceCase(source, receipt(source).body.cases[0], invocation)).rejects.toThrow(/ACTUAL_QUANTITY_UNITS|ACTUAL_MATERIAL_POINT_INPUTS/);
  });
  function isolatedTorsion(action: number) {
    const { source } = received(), c = receipt(source).body.cases[0];
    const check = c.section_stress_checks.find((x: any) => x.component === 'torsional_shear_stress' && x.location === 'end_i');
    const row = source.results.find(r => r.id === check.result_id)!;
    const section = evidence(source).exact_cases[0].pipe_sections.find((s: any) => s.pipe_id === check.pipe_id);
    const member = c.source.section_functionals.find((s: any) => s.pipe_id === check.pipe_id);
    check.action = { value: action, interval: [action, action] }; member.stations[0].actions[3] = structuredClone(check.action);
    check.parameters.torsion_radius_m = section.ro_m = 1e-40;
    check.parameters.torsion_constant_m4 = section.J_m4 = 1e-160;
    row.value = ((action * 1e-40) / 1e-160) / 1e6;
    return { source, c, row, treatment: c.rows.find((r: any) => r.result_id === row.id) };
  }
  it('refuses a nonzero subnormal T*radius even when the published Pa/MPa are normal', () => {
    const x = isolatedTorsion(1e-280), product = 1e-280 * 1e-40;
    expect(product).toBeGreaterThan(0); expect(product).toBeLessThan(2 ** -1022);
    expect(Math.abs(x.row.value)).toBeGreaterThan(2 ** -1022);
    const stableComparison = (1e-280 / 1e-160) * (1e-40 / 1e6);
    expect(Math.abs(x.row.value - stableComparison) / Math.abs(stableComparison)).toBeGreaterThan(1e-9);
    // Isolated negative arithmetic control, not a fabricated accepted carrier.
    expect(() => validatePhysicsSourceDerived(x.source, x.c, x.treatment, x.row, [])).toThrow('STRESS_RECIPE_VALUE');
  });
  it('retains the structural-zero exception in the isolated torsion recipe', () => {
    const x = isolatedTorsion(0);
    expect(x.row.value).toBe(0);
    expect(() => validatePhysicsSourceDerived(x.source, x.c, x.treatment, x.row, [])).not.toThrow();
    // A successful zero recipe subcheck creates no receipt registration.
    expect(physicsSourceStanding(x.source, received().model).eligible).toBe(false);
  });
});


describe('closed composite physical schema keyword regression', () => {
  it.each(pairs.map((p, index) => [p[0], p[1], index] as const))('admits unchanged %s %s physical statements with structured constants', (_name, _mode, index) => {
    const { source } = received(index);
    expect(physicsSourcePhysicalShape(source.contract_evidence)).toBe(true);
    expect(physicsSourcePhysicalShape(structuredClone(source.contract_evidence))).toBe(true);
  });
  it('uses object value equality independent of property order and preserves closed constants', () => {
    const source = received(6).source, pressure = evidence(source).pressure[0];
    pressure.geometry_representation_guard = { meaning: 'arithmetic_representation_only', epsilon_multiplier: 64 };
    expect(physicsSourcePhysicalShape(source.contract_evidence)).toBe(true);
    pressure.geometry_representation_guard.extra = true;
    expect(physicsSourcePhysicalShape(source.contract_evidence)).toBe(false);
  });
  it('preserves old physics structured constants and old source receipt admission', () => {
    const old = JSON.parse(physicsRaw) as MechanicsResult;
    expect(physicsEvidenceTransportShape(old.contract_evidence)).toBe(true);
    expect(sourceBlockReceiptShape(JSON.parse(oldSourceRaw).source_block_recovery)).toBe(true);
    (old.contract_evidence as any).connector = [{}];
    expect(physicsEvidenceTransportShape(old.contract_evidence)).toBe(false);
  });
  it.each([null, 0, -0, 1e-5])('keeps nullable number schema value %s without conferring a material/receipt verdict', value => {
    for (const [e, accepts] of [[evidence(received().source), physicsSourcePhysicalShape], [(JSON.parse(physicsRaw) as MechanicsResult).contract_evidence, physicsEvidenceTransportShape]] as const) {
      (e as any).exact_cases[0].pipe_materials[0].alpha_per_kelvin = value;
      expect(accepts(e)).toBe(true);
    }
  });
  it.each(['not a number', false, true])('rejects nullable-number type confusion %s for old and new evidence', value => {
    for (const [e, accepts] of [[evidence(received().source), physicsSourcePhysicalShape], [(JSON.parse(physicsRaw) as MechanicsResult).contract_evidence, physicsEvidenceTransportShape]] as const) {
      (e as any).exact_cases[0].pipe_materials[0].alpha_per_kelvin = value;
      expect(accepts(e)).toBe(false);
    }
  });
  it.each(['connector', 'dof_order', 'nullable_number', 'method_items'] as const)('rejects %s corruption through the fixed interpreter', mutation => {
    const { source } = received(6), e = evidence(source), c = e.exact_cases[0];
    if (mutation === 'connector') e.connector.push({});
    if (mutation === 'dof_order') c.pressure_rhs_assembly.dof_order.reverse();
    if (mutation === 'nullable_number') c.pipe_materials[0].alpha_per_kelvin = 'not a nullable number';
    if (mutation === 'method_items') c.pipe_stress_extrema = structuredClone(e.exact_cases[1].pipe_stress_extrema);
    expect(physicsSourcePhysicalShape(source.contract_evidence)).toBe(false);
  });
});
