import { describe, expect, it } from 'vitest';
import corpusText from '../../../../../fixtures/results/retained_precision_cases.json?raw';
import { absoluteBound, upwardProduct, upwardSmallSum, binary64Bits, decodeBinary64 } from './retainedPrecision';
const corpus = JSON.parse(corpusText);
type Rational = readonly [bigint, bigint];
// Independent test oracle: exact fractions plus a binary search over ordered
// positive binary64 words. It shares neither the reader's quantum rounding nor
// its significand extraction/rounding implementation.
const MAX_FINITE = 0x7fefffffffffffffn;
function fraction(word: bigint): Rational {
  const exponent = Number((word >> 52n) & 2047n), tail = word & 0xfffffffffffffn;
  const significand = exponent === 0 ? tail : 0x10000000000000n + tail;
  const power = exponent === 0 ? -1074 : exponent - 1075;
  return power < 0 ? [significand, 1n << BigInt(-power)] : [significand << BigInt(power), 1n];
}
function compare(a: Rational, b: Rational): number {
  const delta = a[0] * b[1] - b[0] * a[1]; return delta < 0n ? -1 : delta > 0n ? 1 : 0;
}
const add = (a: Rational, b: Rational): Rational => [a[0] * b[1] + b[0] * a[1], a[1] * b[1]];
const multiply = (a: Rational, b: Rational): Rational => [a[0] * b[0], a[1] * b[1]];
function upperWord(value: Rational): bigint {
  if (compare(value, fraction(MAX_FINITE)) > 0) throw new Error('finite upper does not exist');
  let lo = 0n, hi = MAX_FINITE;
  while (lo < hi) { const mid = (lo + hi) / 2n; if (compare(fraction(mid), value) < 0) lo = mid + 1n; else hi = mid; }
  return lo;
}
function nearestWord(value: Rational): bigint {
  const high = upperWord(value);
  if (high === 0n || compare(fraction(high), value) === 0) return high;
  const low = high - 1n, midpoint = multiply(add(fraction(low), fraction(high)), [1n, 2n]);
  const direction = compare(value, midpoint);
  return direction < 0 || (direction === 0 && low % 2n === 0n) ? low : high;
}
function scaledOracle(word: bigint, power: number): bigint {
  const input = fraction(word), factor: Rational = [1n << BigInt(power), 1n];
  const nearest = nearestWord(multiply(input, [1n, factor[0]]));
  const back = nearestWord(multiply(fraction(nearest), factor));
  return compare(fraction(back), input) < 0 ? nearest + 1n : nearest;
}
function expectLeastUpper(actual: number, exact: Rational): void {
  const word = BigInt('0x' + binary64Bits(actual));
  expect(compare(fraction(word), exact)).toBeGreaterThanOrEqual(0);
  if (word !== 0n) expect(compare(fraction(word - 1n), exact)).toBeLessThan(0);
}
describe('synthetic prepared receipt arithmetic controls, not execution evidence', () => {
  it('preserves the independently derived small-bound bits', () => {
    for (const row of corpus.arithmetic.small_bounds) {
      expect(binary64Bits(absoluteBound(decodeBinary64(row.value), decodeBinary64(row.scale))), row.id).toBe(row.expected);
    }
  });
  it('preserves finite exact-product controls and overflow refusal', () => {
    for (const row of corpus.arithmetic.products) {
      const run = () => upwardProduct(decodeBinary64(row.a), decodeBinary64(row.b));
      if (row.expected === null) expect(run).toThrow(); else expect(binary64Bits(run())).toBe(row.expected);
    }
  });
  it('retains the distant low tail and canonicalizes bound zero', () => {
    expect(binary64Bits(upwardSmallSum(1, Number.MIN_VALUE))).toBe('3ff0000000000001');
    expect(binary64Bits(upwardProduct(-0, 1))).toBe('0000000000000000');
    for (const value of [NaN, Infinity, -Infinity, -1]) expect(() => upwardProduct(value, 1)).toThrow();
  });
  it('independently brackets every protected small-bound result with exact fractions', () => {
    for (const row of corpus.arithmetic.small_bounds) {
      const value = BigInt('0x' + row.value) & 0x7fffffffffffffffn, scale = BigInt('0x' + row.scale);
      const base = scaledOracle(scale, 64), rounding = scaledOracle(value, 53);
      expect(base.toString(16).padStart(16, '0')).toBe(row.b0);
      expect(rounding.toString(16).padStart(16, '0')).toBe(row.rounding);
      const exact = add(add(fraction(base), fraction(rounding)), fraction(1n));
      const actual = absoluteBound(decodeBinary64(row.value), decodeBinary64(row.scale));
      expectLeastUpper(actual, exact);
      expect(BigInt('0x' + binary64Bits(actual))).toBe(upperWord(exact));
    }
  });
  it('independently brackets products, the normal boundary and distant-tail sums', () => {
    for (const row of corpus.arithmetic.products) {
      const exact = multiply(fraction(BigInt('0x' + row.a)), fraction(BigInt('0x' + row.b)));
      if (row.expected === null) { expect(() => upperWord(exact)).toThrow(); continue; }
      expectLeastUpper(upwardProduct(decodeBinary64(row.a), decodeBinary64(row.b)), exact);
    }
    for (const [a, b] of [[1, Number.MIN_VALUE], [2 ** -1022, Number.MIN_VALUE], [0, -0]]) {
      const exact = add(add(fraction(BigInt('0x' + binary64Bits(a))), fraction(BigInt('0x' + binary64Bits(b)))), fraction(1n));
      expectLeastUpper(upwardSmallSum(a, b), exact);
    }
    expect(() => upwardSmallSum(Number.MAX_VALUE, 0)).toThrow();
    expect(() => upwardSmallSum(Number.MAX_VALUE, Number.MAX_VALUE)).toThrow();
  });
  it('keeps zero/sign behavior explicit at every finite helper entrypoint', () => {
    for (const zero of [0, -0]) {
      for (const value of [0, -0, 1, -1, Number.MAX_VALUE, -Number.MAX_VALUE]) expect(binary64Bits(absoluteBound(value, zero))).toBe('0000000000000000');
      expect(binary64Bits(upwardProduct(zero, Number.MAX_VALUE))).toBe('0000000000000000');
      expect(binary64Bits(upwardProduct(Number.MAX_VALUE, zero))).toBe('0000000000000000');
      expect(binary64Bits(upwardSmallSum(zero, zero))).toBe('0000000000000001');
    }
    for (const invalid of [-1, NaN, Infinity, -Infinity]) {
      expect(() => absoluteBound(0, invalid)).toThrow();
      expect(() => upwardSmallSum(invalid, 0)).toThrow(); expect(() => upwardSmallSum(0, invalid)).toThrow();
    }
    for (const invalid of [NaN, Infinity, -Infinity]) expect(() => absoluteBound(invalid, 0)).toThrow();
    expect(binary64Bits(absoluteBound(-16, Number.MIN_VALUE))).toBe(binary64Bits(absoluteBound(16, Number.MIN_VALUE)));
  });
});

import { validateRetainedPrecision, validateRetainedPrecisionTransport, RetainedPrecisionError, phi512, eHat, stopFeasible, nativeSchedule, ordinaryAttempts, accountingRules, nativeRuns, productAttempts, errorStageRecordAgrees, EXACT_RETAINED_ID, EXACT_RETAINED_PROFILE, EXACT_DEFINITION_ID, EXACT_DEFINITION_HASH, RETAINED_PRECISION_ID, RETAINED_PRECISION_PROFILE, PREPARED_DEFINITION_ID, tableBindsReaderConstants } from './retainedPrecision';
import n05Text from '../../../../../fixtures/product_preview/physics_source/n05-sparse_interactive.raw.json?raw';
import exactTableForTests from '../../../../../fixtures/results/semantic_contract_v0_3_physics_retained_1.json';
import { readFileSync, writeFileSync } from 'node:fs';
import physicsTableForTests from '../../../../../fixtures/results/semantic_contract_v0_3_physics_1.json';
import { PHYSICS_RETAINED_CONTRACT_ID, PHYSICS_RETAINED_CONTRACT_SHA256, PHYSICS_RETAINED_PROFILE, numericalResultStanding, retainedPrecisionDowngrade, sourceContract, sourceContractTransport, sourceSemanticBinding } from './numericalResultQuality';
import { RETAINED_PRECISION_VALIDATION_REQUIRED } from './retainedPrecisionStanding';
import { semanticContractForSource } from './resultSemantics';
import { knownSemanticNotices, resultRowLabel, ruleBindingRefusal, RULE_QUANTITY_NOT_COVERED } from './knownSemanticLimitations';
import milestoneSparseText from '../../../../../fixtures/results/retained_precision_milestone_successor_sparse_interactive.json?raw';
import milestoneDenseText from '../../../../../fixtures/results/retained_precision_milestone_successor_dense_scrutiny.json?raw';
import { canonicalSha256HexCheckedV1, canonicalSha256Hex } from '../../services/hashService';
import { createHash } from 'node:crypto';
import m3xSparseText from '../../../../../fixtures/results/retained_precision_exact_successor_sparse_interactive.json?raw';
import m3xDenseText from '../../../../../fixtures/results/retained_precision_exact_successor_dense_scrutiny.json?raw';
/** Snapshot 07e format rule (RV78-N1): a rehash index is a strict integral value, a JSON number that is never a
 * boolean, finite, integral, >= 0 and not -0 (0.0 is index 0; 0.5, true and -0 are not). A reference that is not
 * an index, or does not resolve, is skipped and left for the reader to report. */
function rehashRef(items: any[], ref: unknown): any {
  return typeof ref === 'number' && Number.isInteger(ref) && ref >= 0 && !Object.is(ref, -0) && ref < items.length ? items[ref] : undefined;
}
/** S-1 (REVISION_01 §2): every preparation hash carries the route's definition H: DEF-E's on the exact identity, DEF-O's
 * otherwise. */
const routeDefinitionHash = (source: any) => source?.producer?.semantic_contract_id === EXACT_RETAINED_ID ? EXACT_DEFINITION_HASH : 'a7ed7ca0bf0bba6e8b821ca4befa00a0fa9541a83694be8b28ac63e39b1d0349';
async function rehash(source: any, definitionHash = routeDefinitionHash(source)) {
  // Snapshot 07 format: an entry that removes retained_precision or its body (a G0 pin) has nothing to rehash.
  const body = source.retained_precision?.body;
  if (!body || typeof body !== 'object') return;
  for (const s of body.sources) if (s.preparation) {
    const a = rehashRef(body.product_attempts, s.preparation.attempt_ref);
    if (!a || !a.preparation.members.every((m: any) => m.result.kind === 'prepared')) continue;
    s.preparation.sha256 = await canonicalSha256HexCheckedV1({ domain: 'retained_precision_preparation_v1', payload: {
      definition_id: a.definition_id, definition_sha256: definitionHash, owner_ref: a.owner_ref, ordinary_attempt_ref: a.ordinary_attempt_ref, material_basis_ref: a.material_basis_ref,
      members: a.preparation.members.map((m: any) => ({ member: m.member, old_source: m.old_source, old_facts: m.old_facts, section: m.result.section })) } });
  }
  for (const c of body.cases) {
    const source = c.status === 'selected' ? rehashRef(body.sources, c.source_ref) : undefined;
    if (source) { const { index: _, ...s } = source; c.source_identity_sha256 = await canonicalSha256HexCheckedV1({ domain: 'retained_precision_source_mp_v2', payload: s }); }
  }
  const { retained_precision: _, ...publication } = source;
  body.publication_sha256 = await canonicalSha256HexCheckedV1({ domain: 'retained_precision_publication_mp_v2', payload: publication });
  source.retained_precision.receipt_sha256 = await canonicalSha256HexCheckedV1({ domain: 'retained_precision_receipt_mp_v2', payload: body });
}
function applyEdits(root: any, edits: any[] | undefined): void {
  // 07e format rule (edit_paths): an array index in an edit path is a strict index; anything else refuses the entry.
  const step = (value: any, key: unknown) => { if (Array.isArray(value) && !(typeof key === 'number' && Number.isInteger(key) && key >= 0 && !Object.is(key, -0))) throw new Error('edit path index ' + String(key)); return key as any; };
  for (const edit of edits ?? []) {
    let value = root; for (const key of edit.path.slice(0, -1)) value = value[step(value, key)];
    const key = step(value, edit.path.at(-1));
    // RV78-N5: an array removal splices (no hole); unknown operations are refused.
    if (edit.op === 'remove') { if (Array.isArray(value)) value.splice(key, 1); else delete value[key]; }
    else if (edit.op === 'set') value[key] = structuredClone(edit.value);
    else throw new Error('unsupported edit op ' + edit.op);
  }
}
/** SHARED_SNAPSHOT_06C format_change: source edits; invocation edits on a copy of the base
 * invocation; a non-empty invocation edit list rebinds the receipt's invocation digest; then rehash. */
async function applyEntry(m: any): Promise<{ base: any; source: any; invocation: any }> {
  const base = corpus.cases.find((c: any) => c.id === m.base), source = structuredClone(base.source), invocation = structuredClone(base.invocation);
  applyEdits(source, m.edits);
  applyEdits(invocation, m.invocation_edits);
  if (m.invocation_edits?.length) source.retained_precision.body.invocation.value = await canonicalSha256HexCheckedV1({ domain: 'source_blocks_invocation_v1', payload: invocation });
  // D11/RV78-N5: the shared format admits only rehash "all"; any other value is refused.
  if (m.rehash !== 'all') throw new Error('unsupported rehash ' + m.rehash);
  await rehash(source);
  // D24: the optional after_rehash edit list is applied after rehash "all" (forged-hash G1 pins).
  applyEdits(source, m.after_rehash);
  return { base, source, invocation };
}
describe('shared synthetic prepared receipt controls, never solver execution evidence', () => {
  for (const c of corpus.cases) {
    it(c.id, async () => {
      const source = structuredClone(c.source), invocation = structuredClone(c.invocation);
      const before = structuredClone({ source, invocation });
      const result = await validateRetainedPrecision(source, invocation);
      expect({ invocation_bound: result.invocation_bound, numerical_eligible: result.numerical_eligible, standing: result.standing }).toEqual(c.expected);
      expect(result.classifications).toEqual(c.expected_classifications);
      expect({ source, invocation }).toEqual(before);
      expect(Object.isFrozen(result)).toBe(true);
    });
    it(c.id + ' missing invocation and metadata transport remain needs-recompute', async () => {
      const unbound = await validateRetainedPrecision(c.source);
      expect(unbound.invocation_bound).toBe(false); expect(unbound.numerical_eligible).toBe(false);
      const transport = structuredClone(c.source); delete transport.results;
      const result = await validateRetainedPrecisionTransport(transport);
      expect(result.numerical_eligible).toBe(false); expect(result.classifications).toEqual([]);
    });
    it(c.id + ' snapshots all inputs before awaiting', async () => {
      const s = structuredClone(c.source), invocation = structuredClone(c.invocation);
      const pending = validateRetainedPrecision(s, invocation);
      s.results.length = 0; invocation.request.model.nodes.length = 0;
      expect((await pending).classifications).toEqual(c.expected_classifications);
    });
  }
  for (const m of corpus.mutations ?? []) it(m.id, async () => {
    const { source, invocation } = await applyEntry(m);
    let error: unknown; try { await validateRetainedPrecision(source, invocation); } catch (e) { error = e; }
    expect(error).toBeInstanceOf(RetainedPrecisionError);
    // Per-reader expectations (06b G7 settlement) override the shared expectation for TypeScript.
    expect({ gate: (error as RetainedPrecisionError).gate, code: (error as RetainedPrecisionError).code }).toEqual(m.expected_by_reader?.typescript ?? m.expected);
  });
  // Snapshot 05a: publicly consistent or permitted failure-path rewrites the reader must accept.
  for (const m of corpus.must_pass ?? []) it('must pass: ' + m.id, async () => {
    expect(m.expected).toBe('pass');
    const { base, source, invocation } = await applyEntry(m);
    const result = await validateRetainedPrecision(source, invocation);
    // U7 (07i): each entry's own eligibility per C1:160.
    expect({ invocation_bound: result.invocation_bound, numerical_eligible: result.numerical_eligible, standing: result.standing }).toEqual(m.expected_eligibility);
    expect(result.classifications).toEqual(base.expected_classifications);
  });
});

// Reader-logic controls mirroring Python's for checklist branches without a native-faithful
// shared base yet (N5 verification-pass terminal, N8 Ceiling, N10 idle entry, O5 source_decline).
describe('reader-logic checklist controls, not corpus or producer evidence', () => {
  const caseOf = (id: string) => structuredClone(corpus.cases.find((c: any) => c.id === id));
  const rejects = (run: () => void) => { let error: unknown; try { run(); } catch (e) { error = e; } expect(error).toBeInstanceOf(RetainedPrecisionError); expect({ gate: (error as any).gate, code: (error as any).code }).toEqual({ gate: 'G5', code: 'RETAINED_PRECISION_ATTEMPT_MISMATCH' }); };
  const stopReason = { space: 'attempt', tag: 'stop_rule', quantity: { tag: 'displacement', dof: { node: 1, component: 'UX' } }, body: 0, kind: 'translation' };
  it('N10/N1: an idle invocation-entry run has no records or charge and never selects', () => {
    const base = caseOf('ordinary_prepared_synthetic').source.retained_precision.body, source = base.sources[0];
    const idle = { ...base.cases[0].run, records: [], attempts: [], case_charge: 0, invocation_increment: 0, kernel_terminal: { kind: 'unresolved', reason: { space: 'unresolved', tag: 'budget', scope: 'invocation' } } };
    nativeSchedule(idle, source);
    rejects(() => nativeSchedule({ ...idle, kernel_terminal: { kind: 'selected', reason: null } }, source));
    rejects(() => nativeSchedule({ ...idle, case_charge: 1 }, source));
    // A meter fault at invocation entry is never emitted (C1:66-68): an idle WorkAccounting run is refused.
    rejects(() => nativeSchedule({ ...idle, kernel_terminal: { kind: 'unresolved', reason: { space: 'unresolved', tag: 'work_accounting', fault: 'overflow' } } }, source));
  });
  it('N6: a rejected p128 candidate must hand its verification on, so a Ceiling there is refused', () => {
    const base = caseOf('ordinary_prepared_synthetic').source.retained_precision.body, run = base.cases[0].run;
    run.attempts[0].outcome = run.records[0].outcome = { kind: 'rejected', reason: stopReason };
    run.records[1].outcome = { kind: 'solved' };
    run.kernel_terminal = { kind: 'unresolved', reason: { space: 'unresolved', tag: 'ceiling' } };
    rejects(() => nativeSchedule(run, base.sources[0]));
  });
  it('N5: a non-escalating verification-pass failure is terminal and never selects', () => {
    const base = caseOf('ordinary_prepared_synthetic').source.retained_precision.body, run = base.cases[0].run;
    const stop = { space: 'attempt', tag: 'stop', stop: { space: 'stop', tag: 'structure' } };
    run.attempts[0].outcome = run.records[0].outcome = { kind: 'rejected', reason: { space: 'attempt', tag: 'verification_failed' } };
    run.attempts[0].verification = { record: 1, precision: 256, phase: 'failed', reason: stop };
    run.records[1].outcome = { kind: 'failed', reason: stop };
    run.kernel_terminal = { kind: 'refused', reason: { space: 'refusal', tag: 'structure' } };
    nativeSchedule(run, base.sources[0]);
    rejects(() => nativeSchedule({ ...run, kernel_terminal: { kind: 'selected', reason: null } }, base.sources[0]));
    // terminal(stop) is exact: an Unresolved terminal for a Structure refusal is not native.
    rejects(() => nativeSchedule({ ...run, kernel_terminal: { kind: 'unresolved', reason: { space: 'unresolved', tag: 'ceiling' } } }, base.sources[0]));
    // An escalating verification-pass stop has no terminal() translation, and the work-fault
    // WorkAccounting alternative is never emitted (C1:66-68), so neither terminal is admitted.
    const pivot = { space: 'attempt', tag: 'stop', stop: { space: 'stop', tag: 'pivot', global_dof: 0 } };
    const escalating = structuredClone(run); escalating.attempts[0].verification.reason = pivot; escalating.records[1].outcome = { kind: 'failed', reason: pivot };
    escalating.kernel_terminal = { kind: 'unresolved', reason: { space: 'unresolved', tag: 'pivot', global_dof: 0 } };
    rejects(() => nativeSchedule(escalating, base.sources[0]));
    rejects(() => nativeSchedule({ ...escalating, kernel_terminal: { kind: 'unresolved', reason: { space: 'unresolved', tag: 'work_accounting', fault: 'inconsistent' } } }, base.sources[0]));
    rejects(() => nativeSchedule({ ...run, kernel_terminal: { kind: 'unresolved', reason: { space: 'unresolved', tag: 'work_accounting', fault: 'overflow' } } }, base.sources[0]));
  });
  it('N8: a rejected p512 candidate with a solved p1024 verification is the Ceiling', () => {
    const base = caseOf('p512_ladder_synthetic').source.retained_precision.body, run = base.cases[0].run, source = base.sources[base.cases[0].source_ref];
    run.attempts[2].outcome = run.records[2].outcome = { kind: 'rejected', reason: stopReason };
    run.records[3].outcome = { kind: 'solved' };
    run.kernel_terminal = { kind: 'unresolved', reason: { space: 'unresolved', tag: 'ceiling' } };
    nativeSchedule(run, source);
    rejects(() => nativeSchedule({ ...run, kernel_terminal: { kind: 'refused', reason: { space: 'refusal', tag: 'structure' } } }, source));
    // Leaving the ladder is never a work-accounting point, so no WorkAccounting terminal there.
    rejects(() => nativeSchedule({ ...run, kernel_terminal: { kind: 'unresolved', reason: { space: 'unresolved', tag: 'work_accounting', fault: 'overflow' } } }, source));
  });
  it('R1\'-R4 (D8): each accounting mutation falsifies exactly its own rule on one attempt', async () => {
    const intended: Record<string, boolean[]> = {
      adapter_fault_present: [false, true, true, true], accounting_cause_without_fault: [false, true, true, true],
      scalar_trace_lost_unavailable: [true, false, true, true], old_operational_accounting_not_lost: [true, false, true, true],
      work_accounting_cause_exact_status: [true, true, false, true], nested_stop_work_accounting_exact_status: [true, true, false, true], view_work_fault_exact_status: [true, true, false, true],
      section_accounting_exact_status: [true, true, true, false] };
    for (const c of corpus.cases) for (const a of c.source.retained_precision.body.product_attempts) expect(accountingRules(a), c.id).toEqual([true, true, true, true]);
    for (const [id, rules] of Object.entries(intended)) {
      const m = corpus.mutations.find((x: any) => x.id === id), { source } = await applyEntry(m);
      const failing = source.retained_precision.body.product_attempts.map((a: any) => accountingRules(a)).filter((r: boolean[]) => r.includes(false));
      expect(failing, id).toEqual([rules]);
    }
  });
  it('O5: a source decline names its own unavailable case and material basis', () => {
    const fixture = caseOf('two_case_preparation_failure_synthetic'), body = fixture.source.retained_precision.body;
    const decline = { input_owner: { case_index: 1, case_id: body.cases[1].basis_ref.ref_id, material_basis_ref: body.ordinary_attempts[1].material_basis_ref },
      constructor_counts: { nodes: 2, members: 1, springs: 0, constraints: 6, nodal_terms: 6, stations: 3, supports: 1, id_utf8_bytes: 0, directional_springs: 0 }, error: { tag: 'no_nodes' } };
    body.cases[1].source_decline = decline;
    ordinaryAttempts(body, fixture.source);
    body.cases[1].source_decline = { ...decline, input_owner: { ...decline.input_owner, case_index: 0 } };
    rejects(() => ordinaryAttempts(body, fixture.source));
    body.cases[1].source_decline = decline; body.cases[0].source_decline = { ...decline, input_owner: { ...decline.input_owner, case_index: 0, case_id: body.cases[0].basis_ref.ref_id } };
    rejects(() => ordinaryAttempts(body, fixture.source));
  });
});

// Review repair 07 (ruling D1-D7, D13, D14): reader-local relations, not shared corpus entries.
// RV78 probe entries (PROBES.json, review evidence; same grammar as the corpus plus post_rehash_edits).
const RV78_PROBES: any[] = [{"id":"R1a_execution_order_swapped","base":"two_case_synthetic","edits":[{"path":["retained_precision","body","work","execution_order"],"op":"set","value":[{"kind":"case","index":1},{"kind":"case","index":0}]}],"invocation_edits":[],"post_rehash_edits":[],"rehash":"all","expected":{"gate":"G3","code":"RETAINED_PRECISION_COVERAGE_MISMATCH"}},{"id":"R1b_run_id_not_position","base":"two_case_synthetic","edits":[{"path":["retained_precision","body","cases",1,"run","id"],"op":"set","value":5},{"path":["retained_precision","body","calls",0,"run_refs"],"op":"set","value":[0,5]}],"invocation_edits":[],"post_rehash_edits":[],"rehash":"all","expected":{"gate":"G3","code":"RETAINED_PRECISION_COVERAGE_MISMATCH"}},{"id":"R2_old_members_reordered","base":"two_body_synthetic","edits":[{"path":["retained_precision","body","product_attempts",0,"operational","old"],"op":"set","value":[{"member":1,"inputs":["0000000000000000","4014000000000000","0000000000000000","3ff0000000000000","4014000000000000","0000000000000000","42474876e8000000","4231ed8ec2000000","3f6c4f3caf32fd23","3ee61aa4872838c0"],"result":{"kind":"ready","length":"3ff0000000000000","axial_stiffness":"41c4990f17e516ad","torsional_stiffness":"4128c47ead23fa80","normalization":["3ff0000000000000","0000000000000000","0000000000000000"]},"work":{"entered":0,"checks":0,"lost":false}},{"member":0,"inputs":["0000000000000000","0000000000000000","0000000000000000","3ff0000000000000","0000000000000000","0000000000000000","42474876e8000000","4231ed8ec2000000","3f6c4f3caf32fd23","3ee61aa4872838c0"],"result":{"kind":"ready","length":"3ff0000000000000","axial_stiffness":"41c4990f17e516ad","torsional_stiffness":"4128c47ead23fa80","normalization":["3ff0000000000000","0000000000000000","0000000000000000"]},"work":{"entered":0,"checks":0,"lost":false}}]}],"invocation_edits":[],"post_rehash_edits":[],"rehash":"all","expected":{"gate":"G3","code":"RETAINED_PRECISION_COVERAGE_MISMATCH"}},{"id":"R3_complete_old_short_of_source","base":"two_body_synthetic","edits":[{"path":["retained_precision","body","product_attempts",0,"operational","old"],"op":"set","value":[{"member":0,"inputs":["0000000000000000","0000000000000000","0000000000000000","3ff0000000000000","0000000000000000","0000000000000000","42474876e8000000","4231ed8ec2000000","3f6c4f3caf32fd23","3ee61aa4872838c0"],"result":{"kind":"ready","length":"3ff0000000000000","axial_stiffness":"41c4990f17e516ad","torsional_stiffness":"4128c47ead23fa80","normalization":["3ff0000000000000","0000000000000000","0000000000000000"]},"work":{"entered":0,"checks":0,"lost":false}}]}],"invocation_edits":[],"post_rehash_edits":[],"rehash":"all","expected":{"gate":"G3","code":"RETAINED_PRECISION_COVERAGE_MISMATCH"}},{"id":"R4_unavailable_source_backref_foreign","base":"two_case_facade_after_certificate_synthetic","edits":[{"path":["retained_precision","body","sources",1,"preparation","attempt_ref"],"op":"set","value":0}],"invocation_edits":[],"post_rehash_edits":[],"rehash":"all","expected":{"gate":"G5","code":"RETAINED_PRECISION_PRODUCT_ATTEMPT_MISMATCH"}},{"id":"R5_attempt_and_source_basis_not_ordinary","base":"two_case_two_groups_synthetic","edits":[{"path":["retained_precision","body","product_attempts",1,"material_basis_ref"],"op":"set","value":0},{"path":["retained_precision","body","sources",1,"material_basis_ref"],"op":"set","value":0}],"invocation_edits":[],"post_rehash_edits":[],"rehash":"all","expected":{"gate":"G5","code":"RETAINED_PRECISION_PRODUCT_ATTEMPT_MISMATCH"}},{"id":"R6a_native_error_with_selected_run","base":"two_case_facade_after_certificate_synthetic","edits":[{"path":["retained_precision","body","product_attempts",1,"result","error"],"op":"set","value":{"kind":"native","run_ref":1}}],"invocation_edits":[],"post_rehash_edits":[],"rehash":"all","expected":{"gate":"G5","code":"RETAINED_PRECISION_PRODUCT_ATTEMPT_MISMATCH"}},{"id":"R8_group_call_out_of_range","base":"two_case_synthetic","edits":[{"path":["retained_precision","body","groups",0,"call"],"op":"set","value":3}],"invocation_edits":[],"post_rehash_edits":[],"rehash":"all","expected":{"gate":"G5","code":"RETAINED_PRECISION_ATTEMPT_MISMATCH"}},{"id":"T1_escalating_failed_verification_pass_entered","base":"verification_failure_skip_synthetic","edits":[{"path":["retained_precision","body","cases",0,"run","records",1],"op":"set","value":{"index":1,"precision":256,"role":"verification","outcome":{"kind":"failed","reason":{"space":"attempt","tag":"stop","stop":{"space":"stop","tag":"condition"}}},"residual_basis":320,"corrections":0,"pivot_margin_min":null,"rcond":null,"residual_worst":null,"gate":null,"work":{"wide_lme":1,"exact_sum_lme":0,"own_lme":1,"shared_lme":4,"stop_rule_lme":0,"verification_lme":1,"verification_shared_lme":0,"own_stages":{"formation":0,"assembly":0,"residual_formation":0,"factor":0,"condition":0,"rhs":0,"solve":0,"refinement":0,"recovery":0,"stop_rule":0,"bounded_gate":0,"scale":0,"estimate":0,"charge":0,"bound":1,"shift":0,"bounded_formation":0,"wide_formation":0,"uc":0},"shared_stages":{"formation":4,"assembly":0,"residual_formation":0,"factor":0,"condition":0,"rhs":0,"solve":0,"refinement":0,"recovery":0,"stop_rule":0,"bounded_gate":0,"scale":0,"estimate":0,"charge":0,"bound":0,"shift":0,"bounded_formation":0,"wide_formation":0,"uc":0},"shared_built_here":true,"verification_shared_built_here":false},"storage":{"pattern_entries":144,"profile_entries":21,"limbs_per_entry":4},"verification":null,"bound_refusals":[],"shared_build_ref":1,"verification_shared_build_ref":null}},{"path":["retained_precision","body","cases",0,"run","attempts",0],"op":"set","value":{"precision":128,"candidate_record":0,"origin":{"kind":"fresh"},"verification":{"record":1,"precision":256,"phase":"failed","reason":{"space":"attempt","tag":"stop","stop":{"space":"stop","tag":"condition"}}},"outcome":{"kind":"rejected","reason":{"space":"attempt","tag":"verification_failed"}},"charges":[{"record":0,"part":"solve_and_verification"},{"record":0,"part":"candidate_stop"},{"record":1,"part":"solve_and_verification"}],"case_charge":9,"invocation_increment":9}},{"path":["retained_precision","body","cases",0,"run","case_charge"],"op":"set","value":37},{"path":["retained_precision","body","cases",0,"run","invocation_increment"],"op":"set","value":37},{"path":["retained_precision","body","cases",0,"run","invocation_after"],"op":"set","value":37},{"path":["retained_precision","body","calls",0,"invocation_after"],"op":"set","value":37},{"path":["retained_precision","body","work","charged"],"op":"set","value":37}],"invocation_edits":[],"post_rehash_edits":[],"rehash":"all","expected":{"gate":"G5","code":"RETAINED_PRECISION_ATTEMPT_MISMATCH"}},{"id":"T2_stop_rule_quantity_other_body","base":"p512_ladder_synthetic","edits":[{"path":["retained_precision","body","cases",0,"run","records",0,"outcome","reason","quantity"],"op":"set","value":{"tag":"displacement","dof":{"node":3,"component":"UX"}}},{"path":["retained_precision","body","cases",0,"run","attempts",0,"outcome","reason","quantity"],"op":"set","value":{"tag":"displacement","dof":{"node":3,"component":"UX"}}}],"invocation_edits":[],"post_rehash_edits":[],"rehash":"all","expected":{"gate":"G5","code":"RETAINED_PRECISION_ATTEMPT_MISMATCH"}},{"id":"T3_candidate_record_with_verification","base":"ordinary_prepared_synthetic","edits":[{"path":["retained_precision","body","cases",0,"run","records",0,"verification"],"op":"set","value":{"resolution":[{"body":0,"force":"426d1a94a2000000","moment":"426d1a94a2000000"}],"theta":[{"body":0,"value":"0000000000000000"}],"bound":[{"body":0,"value":"3ff0000000000000"}],"data_blocks":1,"shift_factorizations":0,"g_max":0,"uc_missing":null,"g_violation":null}}],"invocation_edits":[],"post_rehash_edits":[],"rehash":"all","expected":{"gate":"G5","code":"RETAINED_PRECISION_ATTEMPT_MISMATCH"}},{"id":"T4a_ordinary_diagnostic_ref_duplicate","base":"ordinary_prepared_synthetic","edits":[{"path":["retained_precision","body","ordinary_attempts",0,"diagnostic_refs"],"op":"set","value":["diagnostic:numerical-integrity:case:six-component-load","diagnostic:numerical-integrity:case:six-component-load"]}],"invocation_edits":[],"post_rehash_edits":[],"rehash":"all","expected":{"gate":"G5","code":"RETAINED_PRECISION_ATTEMPT_MISMATCH"}},{"id":"T4b_ordinary_diagnostic_ref_dangling","base":"ordinary_prepared_synthetic","edits":[{"path":["retained_precision","body","ordinary_attempts",0,"diagnostic_refs"],"op":"set","value":["diagnostic:numerical-integrity:case:six-component-load","diagnostic:rv78:absent"]}],"invocation_edits":[],"post_rehash_edits":[],"rehash":"all","expected":{"gate":"G5","code":"RETAINED_PRECISION_ATTEMPT_MISMATCH"}},{"id":"T4c_source_identity_stale_receipt_rehashed","base":"ordinary_prepared_synthetic","edits":[],"invocation_edits":[],"post_rehash_edits":[{"path":["retained_precision","body","cases",0,"source_identity_sha256"],"op":"set","value":"0000000000000000000000000000000000000000000000000000000000000000"}],"rehash":"all","expected":{"gate":"G1","code":"RETAINED_PRECISION_RECEIPT_MISMATCH"}},{"id":"T4d_ordinary_dangling_plus_adapter_fault","base":"two_case_facade_after_certificate_synthetic","edits":[{"path":["retained_precision","body","ordinary_attempts",1,"diagnostic_refs"],"op":"set","value":["diagnostic:numerical-integrity:case:unavailable-row","diagnostic:rv78:absent"]},{"path":["retained_precision","body","product_attempts",1,"adapter","fault"],"op":"set","value":{"kind":"overflow","event":"map_write"}}],"invocation_edits":[],"post_rehash_edits":[],"rehash":"all","expected":{"gate":"G5","code":"RETAINED_PRECISION_ATTEMPT_MISMATCH"}},{"id":"R2b_unsourced_old_member_noncontiguous","base":"two_case_preparation_failure_synthetic","edits":[{"path":["retained_precision","body","product_attempts",1,"operational","old",0],"op":"set","value":{"member":1,"inputs":["0000000000000000","0000000000000000","0000000000000000","3ff0000000000000","0000000000000000","0000000000000000","42474876e8000000","4231ed8ec2000000","3f6c4f3caf32fd23","3ee61aa4872838c0"],"result":{"kind":"ready","length":"3ff0000000000000","axial_stiffness":"41c4990f17e516ad","torsional_stiffness":"4128c47ead23fa80","normalization":["3ff0000000000000","0000000000000000","0000000000000000"]},"work":{"entered":0,"checks":0,"lost":false}}}],"invocation_edits":[],"post_rehash_edits":[],"rehash":"all","expected":{"gate":"G3","code":"RETAINED_PRECISION_COVERAGE_MISMATCH"}},{"id":"R3b_complete_old_longer_than_source","base":"two_body_synthetic","edits":[{"path":["retained_precision","body","product_attempts",0,"operational","old"],"op":"set","value":[{"member":0,"inputs":["0000000000000000","0000000000000000","0000000000000000","3ff0000000000000","0000000000000000","0000000000000000","42474876e8000000","4231ed8ec2000000","3f6c4f3caf32fd23","3ee61aa4872838c0"],"result":{"kind":"ready","length":"3ff0000000000000","axial_stiffness":"41c4990f17e516ad","torsional_stiffness":"4128c47ead23fa80","normalization":["3ff0000000000000","0000000000000000","0000000000000000"]},"work":{"entered":0,"checks":0,"lost":false}},{"member":1,"inputs":["0000000000000000","4014000000000000","0000000000000000","3ff0000000000000","4014000000000000","0000000000000000","42474876e8000000","4231ed8ec2000000","3f6c4f3caf32fd23","3ee61aa4872838c0"],"result":{"kind":"ready","length":"3ff0000000000000","axial_stiffness":"41c4990f17e516ad","torsional_stiffness":"4128c47ead23fa80","normalization":["3ff0000000000000","0000000000000000","0000000000000000"]},"work":{"entered":0,"checks":0,"lost":false}},{"member":2,"inputs":["0000000000000000","4014000000000000","0000000000000000","3ff0000000000000","4014000000000000","0000000000000000","42474876e8000000","4231ed8ec2000000","3f6c4f3caf32fd23","3ee61aa4872838c0"],"result":{"kind":"ready","length":"3ff0000000000000","axial_stiffness":"41c4990f17e516ad","torsional_stiffness":"4128c47ead23fa80","normalization":["3ff0000000000000","0000000000000000","0000000000000000"]},"work":{"entered":0,"checks":0,"lost":false}}]}],"invocation_edits":[],"post_rehash_edits":[],"rehash":"all","expected":{"gate":"G3","code":"RETAINED_PRECISION_COVERAGE_MISMATCH"}},{"id":"B_interpolation_target_at_lower_point_equal_point_E","base":"ordinary_prepared_interpolated_material_synthetic","edits":[{"path":["retained_precision","body","material_bases",0,"selector","kelvin"],"op":"set","value":"4072c00000000000"},{"path":["retained_precision","body","material_bases",0,"materials",0,"selection","target_kelvin"],"op":"set","value":"4072c00000000000"}],"invocation_edits":[{"path":["request","model","materials",0,"temperature_points",0,"elastic_modulus"],"op":"set","value":{"value":200000000000.0,"unit":"Pa"}},{"path":["request","model","materials",0,"temperature_points",1,"elastic_modulus"],"op":"set","value":{"value":200000000000.0,"unit":"Pa"}},{"path":["request","model","load_cases",0,"modulus_basis_temperature"],"op":"set","value":{"value":300,"unit":"K"}}],"post_rehash_edits":[],"rehash":"all","expected":{"gate":"G8","code":"RETAINED_PRECISION_PREPARATION_MISMATCH"}},{"id":"B_interpolation_target_below_range_equal_point_E","base":"ordinary_prepared_interpolated_material_synthetic","edits":[{"path":["retained_precision","body","material_bases",0,"selector","kelvin"],"op":"set","value":"4072b00000000000"},{"path":["retained_precision","body","material_bases",0,"materials",0,"selection","target_kelvin"],"op":"set","value":"4072b00000000000"}],"invocation_edits":[{"path":["request","model","materials",0,"temperature_points",0,"elastic_modulus"],"op":"set","value":{"value":200000000000.0,"unit":"Pa"}},{"path":["request","model","materials",0,"temperature_points",1,"elastic_modulus"],"op":"set","value":{"value":200000000000.0,"unit":"Pa"}},{"path":["request","model","load_cases",0,"modulus_basis_temperature"],"op":"set","value":{"value":299,"unit":"K"}}],"post_rehash_edits":[],"rehash":"all","expected":{"gate":"G8","code":"RETAINED_PRECISION_PREPARATION_MISMATCH"}},{"id":"B_interpolation_target_at_upper_point_equal_point_E","base":"ordinary_prepared_interpolated_material_synthetic","edits":[{"path":["retained_precision","body","material_bases",0,"selector","kelvin"],"op":"set","value":"4073600000000000"},{"path":["retained_precision","body","material_bases",0,"materials",0,"selection","target_kelvin"],"op":"set","value":"4073600000000000"}],"invocation_edits":[{"path":["request","model","materials",0,"temperature_points",0,"elastic_modulus"],"op":"set","value":{"value":200000000000.0,"unit":"Pa"}},{"path":["request","model","materials",0,"temperature_points",1,"elastic_modulus"],"op":"set","value":{"value":200000000000.0,"unit":"Pa"}},{"path":["request","model","load_cases",0,"modulus_basis_temperature"],"op":"set","value":{"value":310,"unit":"K"}}],"post_rehash_edits":[],"rehash":"all","expected":{"gate":"G8","code":"RETAINED_PRECISION_PREPARATION_MISMATCH"}},{"id":"B_interpolation_target_above_range_equal_point_E","base":"ordinary_prepared_interpolated_material_synthetic","edits":[{"path":["retained_precision","body","material_bases",0,"selector","kelvin"],"op":"set","value":"4073700000000000"},{"path":["retained_precision","body","material_bases",0,"materials",0,"selection","target_kelvin"],"op":"set","value":"4073700000000000"}],"invocation_edits":[{"path":["request","model","materials",0,"temperature_points",0,"elastic_modulus"],"op":"set","value":{"value":200000000000.0,"unit":"Pa"}},{"path":["request","model","materials",0,"temperature_points",1,"elastic_modulus"],"op":"set","value":{"value":200000000000.0,"unit":"Pa"}},{"path":["request","model","load_cases",0,"modulus_basis_temperature"],"op":"set","value":{"value":311,"unit":"K"}}],"post_rehash_edits":[],"rehash":"all","expected":{"gate":"G8","code":"RETAINED_PRECISION_PREPARATION_MISMATCH"}},{"id":"B_control_equal_point_E_bracketed","base":"ordinary_prepared_interpolated_material_synthetic","edits":[],"invocation_edits":[{"path":["request","model","materials",0,"temperature_points",0,"elastic_modulus"],"op":"set","value":{"value":200000000000.0,"unit":"Pa"}},{"path":["request","model","materials",0,"temperature_points",1,"elastic_modulus"],"op":"set","value":{"value":200000000000.0,"unit":"Pa"}}],"post_rehash_edits":[],"rehash":"all","expected":"pass"},{"id":"T4e_selected_case_ordinary_checks_passed","base":"ordinary_prepared_synthetic","edits":[{"path":["numerical_quality","cases",0,"solve_quality"],"op":"set","value":"checks_passed"},{"path":["retained_precision","body","ordinary_attempts",0,"initial","outcome"],"op":"set","value":"checks_passed"}],"invocation_edits":[],"post_rehash_edits":[],"rehash":"all","expected":{"gate":"G5","code":"RETAINED_PRECISION_ATTEMPT_MISMATCH"}}];
async function rehashOuter(source: any) {
  const { retained_precision: _, ...publication } = source, body = source.retained_precision.body;
  body.publication_sha256 = await canonicalSha256HexCheckedV1({ domain: 'retained_precision_publication_mp_v2', payload: publication });
  source.retained_precision.receipt_sha256 = await canonicalSha256HexCheckedV1({ domain: 'retained_precision_receipt_mp_v2', payload: body });
}
// C1:160, computed here without the reader: an invocation, MECHANICS_SOLVED, and every receipt case selected or not_required.
const c160 = (source: any, invocation?: any) => invocation != null && source.status?.mechanics === 'MECHANICS_SOLVED' && source.retained_precision.body.cases.every((c: any) => c.status === 'selected' || c.status === 'not_required');
async function firstFailure(source: any, invocation?: any): Promise<{ gate: string; code: string } | 'pass'> {
  try { const r = await validateRetainedPrecision(source, invocation), eligible = c160(source, invocation); expect([r.numerical_eligible, r.standing]).toEqual([eligible, eligible ? 'eligible' : 'needs_recompute']); return 'pass'; }
  catch (e) { expect(e).toBeInstanceOf(RetainedPrecisionError); return { gate: (e as any).gate, code: (e as any).code }; }
}
async function edited(baseId: string, edit: (source: any, invocation: any) => void, rehashAll = true) {
  const base = structuredClone(corpus.cases.find((c: any) => c.id === baseId)); edit(base.source, base.invocation);
  if (rehashAll) await rehash(base.source);
  return base;
}
const G = (gate: string, suffix: string) => ({ gate, code: gate === 'G0' ? suffix : 'RETAINED_PRECISION_' + suffix });
describe('review repair 07: RV78 probe relations (reader-local)', () => {
  for (const p of RV78_PROBES) it(p.id, async () => {
    const { source, invocation } = await applyEntry(p);
    if (p.post_rehash_edits.length) { applyEdits(source, p.post_rehash_edits); await rehashOuter(source); }
    expect(await firstFailure(source, invocation)).toEqual(p.expected);
  });
});
describe('review repair 07: decisions without a probe (reader-local)', () => {
  it('D1/RV81-B1: unsourced old ids must be 0..len-1 at G3, with or without the invocation', async () => {
    const p = RV78_PROBES.find(x => x.id === 'R2b_unsourced_old_member_noncontiguous'), { source } = await applyEntry(p);
    expect(await firstFailure(source)).toEqual(G('G3', 'COVERAGE_MISMATCH'));
  });
  it('D1 (checkpoint A): unsourced complete old coverage matches every CaseSource member count, with no emptiness rule', async () => {
    // An empty list fails here only because a CaseSource of the same model has one member.
    const empty = await edited('two_case_preparation_failure_synthetic', s => { s.retained_precision.body.product_attempts[1].operational.old = []; });
    expect(await firstFailure(empty.source, empty.invocation)).toEqual(G('G3', 'COVERAGE_MISMATCH'));
    // (No corpus base lacks a CaseSource, so the no-inventory branch, left to G8, is not exercised here.)
    const longer = await edited('two_case_preparation_failure_synthetic', s => { const old = s.retained_precision.body.product_attempts[1].operational.old; old.push({ ...structuredClone(old[0]), member: old.length }); });
    expect(await firstFailure(longer.source, longer.invocation)).toEqual(G('G3', 'COVERAGE_MISMATCH'));
  });
  it('D1/RV81-S1: a captured prefix with a Run is a G5 association defect, not G3', async () => {
    const m = structuredClone(corpus.must_pass.find((x: any) => x.id === 'prefix_captured'));
    m.edits.push({ path: ['retained_precision', 'body', 'product_attempts', 1, 'run_ref'], op: 'set', value: 0 });
    const { source, invocation } = await applyEntry(m);
    expect(await firstFailure(source, invocation)).toEqual(G('G5', 'PRODUCT_ATTEMPT_MISMATCH'));
  });
  it('D2: G0 owns the producer component/schema versions, receipt constants and thresholds; body shape waits for G1', async () => {
    const at0 = async (edit: (s: any) => void) => { const c = structuredClone(corpus.cases[0]); edit(c.source); return firstFailure(c.source, c.invocation); };
    expect(await at0(s => { s.producer.component_version = '0.1.0'; })).toEqual(G('G0', 'SOURCE_PRODUCER_CONTRACT_UNSUPPORTED'));
    expect(await at0(s => { s.producer.component_name = 'other'; })).toEqual(G('G0', 'SOURCE_PRODUCER_CONTRACT_UNSUPPORTED'));
    expect(await at0(s => { s.schema_version = '0.3.0'; })).toEqual(G('G0', 'SOURCE_PRODUCER_CONTRACT_UNSUPPORTED'));
    expect(await at0(s => { s.retained_precision.body.canonicalization = 'other'; })).toEqual(G('G0', 'SOURCE_PRODUCER_CONTRACT_UNSUPPORTED'));
    expect(await at0(s => { delete s.retained_precision.body.work; })).toEqual(G('G0', 'SOURCE_PRODUCER_CONTRACT_UNSUPPORTED'));
    expect(await at0(s => { s.retained_precision.body.product_attempts = {}; })).toEqual(G('G1', 'RECEIPT_MISMATCH'));
    expect(await at0(s => { s.retained_precision.body.product_attempts[0] = 7; })).toEqual(G('G1', 'RECEIPT_MISMATCH'));
  });
  it('D3: a native class with an ATTEMPT defect reports ATTEMPT even when an earlier-coded WORK defect exists', async () => {
    const p = structuredClone(RV78_PROBES.find(x => x.id === 'R8_group_call_out_of_range'));
    p.edits.push({ path: ['retained_precision', 'body', 'calls', 0, 'invocation_before'], op: 'set', value: 1 });
    const { source, invocation } = await applyEntry(p);
    expect(await firstFailure(source, invocation)).toEqual(G('G5', 'ATTEMPT_MISMATCH'));
    const workOnly = await edited('two_case_synthetic', s => { s.retained_precision.body.calls[0].invocation_before = 1; });
    expect(await firstFailure(workOnly.source, workOnly.invocation)).toEqual(G('G5', 'WORK_MISMATCH'));
  });
  it('D4c/RV81-B2: a prepared_product_failure cause must name the case\'s own attempt', async () => {
    for (const cause of [1, 0]) {
      const c = await edited('two_case_preparation_failure_synthetic', s => {
        const b = s.retained_precision.body; b.product_attempts = [b.product_attempts[0]]; b.cases[1].product_attempt_ref = null; b.cases[1].reason.cause.product_attempt_ref = cause;
      });
      expect(await firstFailure(c.source, c.invocation), String(cause)).toEqual(G('G5', 'PRODUCT_ATTEMPT_MISMATCH'));
    }
  });
  it('D4e: run_ref is null exactly when no native call happened', async () => {
    const c = await edited('ordinary_prepared_synthetic', s => { s.retained_precision.body.product_attempts[0].run_ref = null; });
    expect(await firstFailure(c.source, c.invocation)).toEqual(G('G5', 'PRODUCT_ATTEMPT_MISMATCH'));
    const p = await edited('two_case_preparation_failure_synthetic', s => { s.retained_precision.body.product_attempts[1].run_ref = 0; });
    expect(await firstFailure(p.source, p.invocation)).toEqual(G('G5', 'PRODUCT_ATTEMPT_MISMATCH'));
  });
  it('D5c: rejected(verification_failed) requires a failed verification phase', () => {
    const base = structuredClone(corpus.cases.find((c: any) => c.id === 'p512_ladder_synthetic')).source.retained_precision.body, run = base.cases[0].run;
    run.attempts[2].outcome = run.records[2].outcome = { kind: 'rejected', reason: { space: 'attempt', tag: 'verification_failed' } };
    run.records[3].outcome = { kind: 'solved' }; run.kernel_terminal = { kind: 'unresolved', reason: { space: 'unresolved', tag: 'ceiling' } };
    let error: any; try { nativeSchedule(run, base.sources[base.cases[0].source_ref]); } catch (e) { error = e; }
    expect({ gate: error?.gate, code: error?.code }).toEqual(G('G5', 'ATTEMPT_MISMATCH'));
  });
  it('D5a/RV81-M04: a candidate record carrying verification work alone is an ATTEMPT defect', async () => {
    const c = await edited('ordinary_prepared_synthetic', s => { const r = s.retained_precision.body.cases[0].run.records[0]; r.work.verification_lme = 1; });
    expect(await firstFailure(c.source, c.invocation)).toEqual(G('G5', 'ATTEMPT_MISMATCH'));
  });
  it('D6c: a published W2 needs a nonzero exponent and the initial failure\'s trigger', () => {
    const fixture = structuredClone(corpus.cases[0]), body = fixture.source.retained_precision.body, o = body.ordinary_attempts[0];
    const error = { tag: 'synthetic_formation_error' }, ref = o.initial.report_diagnostic_ref;
    o.initial = { kind: 'formation_failure', error };
    o.w2 = { kind: 'published', trigger: { tag: 'formation', error }, force_scale_exponent: 1, report_diagnostic_ref: ref };
    ordinaryAttempts(body, fixture.source);
    const rejects = (edit: (w: any) => void) => { const b = structuredClone(body); edit(b.ordinary_attempts[0].w2); let e: any; try { ordinaryAttempts(b, fixture.source); } catch (x) { e = x; } expect({ gate: e?.gate, code: e?.code }).toEqual(G('G5', 'ATTEMPT_MISMATCH')); };
    rejects(w => { w.force_scale_exponent = 0; });
    rejects(w => { w.trigger.tag = 'evaluation'; });
    rejects(w => { w.trigger.error = { tag: 'other' }; });
  });
  it('D6a as amended by F5 (D-U6-7; A2): the untyped list is exactly the diagnostics naming the case; typed references stay strict', () => {
    const fixture = structuredClone(corpus.cases[0]), body = fixture.source.retained_precision.body, o = body.ordinary_attempts[0];
    const other = { ...structuredClone(fixture.source.diagnostics[0]), id: 'diagnostic:i64:model-level', affected_refs: null };
    fixture.source.diagnostics.push(other);
    const run = (edit: (b: any) => void) => { const b = structuredClone(body); edit(b); let e: any; try { ordinaryAttempts(b, fixture.source); } catch (x) { e = x; } return e ? { gate: e.gate, code: e.code } : 'pass'; };
    expect(run(() => {})).toBe('pass'); // the repaired base's exact list (snapshot 07g)
    expect(run(b => { b.ordinary_attempts[0].diagnostic_refs.push(other.id); })).toEqual(G('G5', 'ATTEMPT_MISMATCH')); // F5: a model-level diagnostic is not the case's
    expect(run(b => { b.ordinary_attempts[0].diagnostic_refs.push('diagnostic:i64:absent'); })).toEqual(G('G5', 'ATTEMPT_MISMATCH'));
    expect(run(b => { b.ordinary_attempts[0].diagnostic_refs.push(o.diagnostic_refs[0]); })).toEqual(G('G5', 'ATTEMPT_MISMATCH'));
    expect(run(b => { b.ordinary_attempts[0].diagnostic_refs.push(other.id); b.ordinary_attempts[0].formation.d5_diagnostic_ref = other.id; })).toEqual(G('G5', 'ATTEMPT_MISMATCH'));
  });
  it('D6b (checkpoint A): a selected case\'s quality is sensitive, unresolved or failed', () => {
    const fixture = structuredClone(corpus.cases[0]), body = fixture.source.retained_precision.body;
    const run = (quality: string) => { const s = structuredClone(fixture.source); s.numerical_quality.cases[0].solve_quality = quality; const b = structuredClone(body); if (b.ordinary_attempts[0].initial.kind === 'report') b.ordinary_attempts[0].initial.outcome = quality; let e: any; try { ordinaryAttempts(b, s); } catch (x) { e = x; } return e ? e.code : 'pass'; };
    for (const q of ['sensitive', 'unresolved', 'failed']) expect(run(q), q).toBe('pass');
    for (const q of ['checks_passed', 'not_assessed']) expect(run(q), q).toBe('RETAINED_PRECISION_ATTEMPT_MISMATCH');
  });
  it('D8 kernel scope: a work_accounting stop or reason anywhere in a Run, build or group preparation fails G5 ATTEMPT', async () => {
    const stop = { space: 'stop', tag: 'work_accounting', fault: 'overflow' };
    const inBuild = await edited('candidate_failure_skip_synthetic', s => { const b = s.retained_precision.body; const i = b.builds.findIndex((x: any) => x.state === 'nonbudget_failure'); expect(i).toBeGreaterThanOrEqual(0); b.builds[i].reason = stop; });
    expect(await firstFailure(inBuild.source, inBuild.invocation)).toEqual(G('G5', 'ATTEMPT_MISMATCH'));
    const body = structuredClone(corpus.cases[0]).source.retained_precision.body;
    body.groups[0].preparation = { kind: 'refused', reason: { space: 'unresolved', tag: 'work_accounting', fault: 'overflow' } };
    let e: any; try { nativeRuns(body); } catch (x) { e = x; }
    expect({ gate: e?.gate, code: e?.code }).toEqual(G('G5', 'ATTEMPT_MISMATCH'));
  });
  it('D18: each echoed G5b section term equals the source term and is positive, else G5b SECTION', async () => {
    for (const key of ['area', 'section_modulus', 'length', 'axial_stiffness', 'torsional_stiffness']) {
      const c = await edited('ordinary_prepared_synthetic', s => { const b = s.retained_precision.body; b.sources[0].section_terms[0][key] = '0000000000000000'; b.cases[0].selection.section_terms[0][key] = '0000000000000000'; });
      expect(await firstFailure(c.source, c.invocation), key).toEqual(G('G5b', 'SECTION_MISMATCH'));
    }
    // A positive term that differs from the source is an echo defect (base length is 1.0, so use 2.0).
    const echo = await edited('ordinary_prepared_synthetic', s => { s.retained_precision.body.cases[0].selection.section_terms[0].length = '4000000000000000'; });
    expect(await firstFailure(echo.source, echo.invocation)).toEqual(G('G5b', 'SECTION_MISMATCH'));
  });
  it('native class: each dangling reference reports the code of the check that follows it', async () => {
    const build = await edited('two_case_synthetic', s => { s.retained_precision.body.cases[0].run.records[0].shared_build_ref = 99; });
    expect(await firstFailure(build.source, build.invocation)).toEqual(G('G5', 'WORK_MISMATCH'));
    const record = await edited('two_case_synthetic', s => { s.retained_precision.body.cases[0].run.attempts[0].candidate_record = 9; });
    expect(await firstFailure(record.source, record.invocation)).toEqual(G('G5', 'ATTEMPT_MISMATCH'));
    const group = await edited('two_case_synthetic', s => { s.retained_precision.body.cases[1].run.origin.group = 9; });
    expect(await firstFailure(group.source, group.invocation)).toEqual(G('G5', 'ATTEMPT_MISMATCH'));
  });
  it('D6d/RV81-N2: legacy_source.work_ref is a reference check reported as ATTEMPT', () => {
    const fixture = structuredClone(corpus.cases[0]), body = fixture.source.retained_precision.body;
    const run = (work: any[], ref: number) => { const b = structuredClone(body); b.legacy_source_work = work; b.ordinary_attempts[0].legacy_source.work_ref = ref; let e: any; try { ordinaryAttempts(b, fixture.source); } catch (x) { e = x; } return e ? { gate: e.gate, code: e.code } : 'pass'; };
    expect(run([], 0)).toEqual(G('G5', 'ATTEMPT_MISMATCH'));
    expect(run([{ case_index: 1 }], 0)).toEqual(G('G5', 'ATTEMPT_MISMATCH'));
    expect(run([{ case_index: 0 }], 0)).toBe('pass');
  });
  it('D7: a RETAINED_PRECISION_UNAVAILABLE diagnostic naming no requested case fails G4', async () => {
    const c = await edited('ordinary_prepared_synthetic', s => {
      const d = structuredClone(s.diagnostics.find((x: any) => x.code === 'RETAINED_PRECISION_SELECTED'));
      s.diagnostics.push({ ...d, id: 'diagnostic:i64:foreign-case', code: 'RETAINED_PRECISION_UNAVAILABLE', affected_refs: ['case:not-requested'] });
    });
    expect(await firstFailure(c.source, c.invocation)).toEqual(G('G4', 'DIAGNOSTIC_MISMATCH'));
  });
  it('D13/RV81-M13: a no-data body needs theta = +0 in its verification record', async () => {
    const c = await edited('ordinary_prepared_no_data_synthetic', s => { const b = s.retained_precision.body; b.cases[0].selection.theta[0].value = '3fd0000000000000'; b.cases[0].run.records[1].verification.theta[0].value = '3fd0000000000000'; });
    expect(await firstFailure(c.source, c.invocation)).toEqual(G('G5a', 'SCALE_MISMATCH'));
  });
  it('D13/RV81-M16: a p128 verification-solve failure continues at p512, so it never ends in the Ceiling', () => {
    const base = structuredClone(corpus.cases.find((c: any) => c.id === 'verification_failure_skip_synthetic')).source.retained_precision.body, run = base.cases[0].run;
    const truncated = { ...run, attempts: [run.attempts[0]], records: run.records.slice(0, 2), kernel_terminal: { kind: 'unresolved', reason: { space: 'unresolved', tag: 'ceiling' } } };
    let error: any; try { nativeSchedule(truncated, base.sources[base.cases[0].source_ref]); } catch (e) { error = e; }
    expect({ gate: error?.gate, code: error?.code }).toEqual(G('G5', 'ATTEMPT_MISMATCH'));
  });
  it('D13/RV81-M08 and D8 owner scopes: R3\' needs every fault of every fault-bearing cause in its owner, including both', () => {
    const base = structuredClone(corpus.cases.find((c: any) => c.id === 'two_case_facade_after_certificate_synthetic')).source.retained_precision.body.product_attempts[1];
    // Causes on the certificate check belong to the whole ProofTrace; statuses are placed on lane 0's work, inside it.
    const attempt = (causes: any[], statuses: string[], where: 'proof' | 'member' = 'proof') => {
      const a = structuredClone(base); a.proof.checks.certificate = { kind: 'failed', error: { kind: 'proof', cause: causes.length === 1 ? causes[0] : { kind: 'numeric', cause: causes } } };
      const target = where === 'proof' ? a.proof.lanes[0].work : a.preparation.members[0].work;
      statuses.forEach((s, i) => { target['i64_status_' + i] = { sticky_status: s }; });
      return accountingRules(a)[2];
    };
    expect(attempt([{ kind: 'work_accounting', fault: 'both' }], ['overflow'])).toBe(false);
    expect(attempt([{ kind: 'work_accounting', fault: 'both' }], ['overflow', 'inconsistent'])).toBe(true);
    expect(attempt([{ kind: 'work_accounting', fault: 'both' }], ['both'])).toBe(true);
    expect(attempt([{ kind: 'work_accounting', fault: 'overflow' }, { kind: 'work_accounting', fault: 'inconsistent' }], ['overflow'])).toBe(false);
    expect(attempt([{ space: 'stop', tag: 'work_accounting', fault: 'overflow' }], ['overflow'])).toBe(true);
    expect(attempt([{ kind: 'view', issue: { kind: 'work', fault: 'inconsistent' } }], ['overflow'])).toBe(false);
    // A status outside the owner scope does not satisfy it: a member's work is not part of the ProofTrace.
    expect(attempt([{ kind: 'work_accounting', fault: 'overflow' }], ['overflow'], 'member')).toBe(false);
  });
  it('D13: the absolute bound switches at S = 2^-988 on both sides', () => {
    const value = 3, valueWord = BigInt('0x' + binary64Bits(value));
    const at = 2 ** -988, below = decodeBinary64((BigInt('0x' + binary64Bits(at)) - 1n).toString(16).padStart(16, '0'));
    expect(BigInt('0x' + binary64Bits(absoluteBound(value, at)))).toBe(scaledOracle(BigInt('0x' + binary64Bits(at)), 64));
    const exact = add(add(fraction(scaledOracle(BigInt('0x' + binary64Bits(below)), 64)), fraction(scaledOracle(valueWord, 53))), fraction(1n));
    expect(BigInt('0x' + binary64Bits(absoluteBound(value, below)))).toBe(upperWord(exact));
  });
  it('RV81-M07 (N17): the case scope wins when both budget limits are exceeded', () => {
    const body = structuredClone(corpus.cases[0]).source.retained_precision.body, run = body.cases[0].run;
    body.cases[0].status = 'unavailable'; body.work.case_limit = 10; body.work.invocation_limit = 10;
    const withScope = (scope: string) => { const b = structuredClone(body), r = b.cases[0].run; const stop = { space: 'attempt', tag: 'stop', stop: { space: 'stop', tag: 'budget', scope } };
      r.attempts[0].outcome = r.records[0].outcome = { kind: 'failed', reason: stop }; r.records[1].outcome = { kind: 'solved' };
      r.kernel_terminal = { kind: 'unresolved', reason: { space: 'unresolved', tag: 'budget', scope } }; let e: any; try { nativeRuns(b); } catch (x) { e = x; } return e ? { gate: e.gate, code: e.code } : 'pass'; };
    expect(run.case_charge).toBeGreaterThan(10);
    expect(withScope('case')).toBe('pass');
    expect(withScope('invocation')).toEqual(G('G5', 'WORK_MISMATCH'));
  });
  it('D12/N11: a refused group preparation gives a Run with its group index, no attempts and the refusal', () => {
    const body = structuredClone(corpus.cases[0]).source.retained_precision.body, run = body.cases[0].run, refusal = { space: 'refusal', tag: 'structure' };
    body.cases[0].status = 'unavailable'; body.groups[0].preparation = { kind: 'refused', reason: refusal }; body.builds = [];
    Object.assign(run, { records: [], attempts: [], case_charge: 0, invocation_increment: 0, invocation_after: 0, cache_after: [], kernel_terminal: { kind: 'refused', reason: refusal } });
    body.calls[0].invocation_after = 0; body.work.charged = 0;
    const result = (edit: (b: any) => void) => { const b = structuredClone(body); edit(b); let e: any; try { nativeRuns(b); } catch (x) { e = x; } return e ? { gate: e.gate, code: e.code } : 'pass'; };
    expect(run.origin.group).toBe(0);
    expect(result(() => undefined)).toBe('pass');
    expect(result(b => { b.cases[0].run.kernel_terminal = { kind: 'refused', reason: { space: 'refusal', tag: 'negative_energy', i: 0, j: 1 } }; })).toEqual(G('G5', 'ATTEMPT_MISMATCH'));
    expect(result(b => { b.cases[0].run.origin.group = null; })).toEqual(G('G5', 'ATTEMPT_MISMATCH'));
  });
  it('RV81-M12: a non-null empty roster fails G3 even against an empty body inventory', async () => {
    const c = await edited('ordinary_prepared_no_data_synthetic', s => { const b = s.retained_precision.body; b.sources[0].body_membership = []; b.product_attempts[0].proof.summary_coverage = []; });
    expect(await firstFailure(c.source, c.invocation)).toEqual(G('G3', 'COVERAGE_MISMATCH'));
  });
  it('D11/RV78-N5: the harness admits only rehash "all" and splices array removals', async () => {
    expect([...corpus.mutations, ...corpus.must_pass].every((m: any) => m.rehash === 'all')).toBe(true);
    await expect(applyEntry({ ...corpus.mutations[0], rehash: 'receipt' })).rejects.toThrow('unsupported rehash');
    const list = [1, 2, 3]; applyEdits({ list }, [{ path: ['list', 1], op: 'remove' }]); expect(list).toEqual([1, 3]);
  });
  it('D14: no non-test module imports the @internal reader exports', async () => {
    const fs = await import('node:fs'), path = await import('node:path');
    const root = path.resolve(__dirname, '../..'), internal = ['nativeSchedule', 'nativeRuns', 'ordinaryAttempts', 'accountingRules', 'stopFeasible', 'phi512', 'eHat'];
    const offenders: string[] = [];
    for (const rel of fs.readdirSync(root, { recursive: true }) as string[]) {
      if (!/\.(ts|tsx)$/.test(rel) || /\.test\.(ts|tsx)$/.test(rel) || rel.endsWith(path.join('results', 'retainedPrecision.ts'))) continue;
      const text = fs.readFileSync(path.join(root, rel), 'utf8');
      for (const m of text.matchAll(/import\s*\{([^}]*)\}\s*from\s*['"][^'"]*retainedPrecision['"]/g)) if (internal.some(name => new RegExp('\\b' + name + '\\b').test(m[1]))) offenders.push(rel);
    }
    expect(offenders).toEqual([]);
  });
});

describe('confirmation repair round (D19-D30, RV81-N1/N2): reader-local relations', () => {
  const facadeCause = (index: number) => ({ kind: 'facade_failure', owner_ref: { kind: 'case', index }, row_id: null, recipe: 'identity', operand_index: null, check: 'identity', predicate: null });
  const unavailableCase1 = (s: any, reason: any) => {
    const b = s.retained_precision.body, c = b.cases[1], diag = s.diagnostics.find((d: any) => d.code === 'RETAINED_PRECISION_SELECTED' && d.affected_refs?.includes(c.basis_ref.ref_id));
    delete c.method; delete c.selection; delete c.source_identity_sha256;
    Object.assign(c, { status: 'unavailable', reason, diagnostic_ref: diag.id }); diag.code = 'RETAINED_PRECISION_UNAVAILABLE';
  };
  it('D19: an unavailable C3 attempt must be reported through prepared_product_failure naming it', async () => {
    const c = await edited('two_case_facade_after_certificate_synthetic', s => { const r = s.retained_precision.body.cases[1].reason; r.cause = facadeCause(1); });
    expect(await firstFailure(c.source, c.invocation)).toEqual(G('G5', 'PRODUCT_ATTEMPT_MISMATCH'));
  });
  it('D19: a Ready attempt belongs to a selected case, or to an unavailable case with a receipt_failure cause', async () => {
    const facade = await edited('two_case_synthetic', s => unavailableCase1(s, { code: 'facade_certificate', phase: 'facade', cause: facadeCause(1) }));
    expect(await firstFailure(facade.source, facade.invocation)).toEqual(G('G5', 'PRODUCT_ATTEMPT_MISMATCH'));
    const receipt = await edited('two_case_synthetic', s => unavailableCase1(s, { code: 'receipt_encoding', phase: 'receipt', cause: { kind: 'receipt_failure', check: 'encoding', field_path: 'retained_precision.body' } }));
    expect(await firstFailure(receipt.source, receipt.invocation)).not.toEqual(G('G5', 'PRODUCT_ATTEMPT_MISMATCH'));
  });
  it('D20: a selected case without a C3 attempt is a G5 PRODUCT_ATTEMPT association defect', async () => {
    const c = await edited('ordinary_prepared_synthetic', s => { const b = s.retained_precision.body; b.cases[0].product_attempt_ref = null; b.product_attempts = []; b.sources[0].preparation = null; });
    expect(await firstFailure(c.source, c.invocation)).toEqual(G('G5', 'PRODUCT_ATTEMPT_MISMATCH'));
  });
  it('D22/RV81-S1: a dangling attempt source_ref is a G5 PRODUCT_ATTEMPT defect, not G3', async () => {
    for (const [baseId, ai] of [['ordinary_prepared_synthetic', 0], ['two_case_facade_after_certificate_synthetic', 1]] as const) {
      const c = await edited(baseId, s => { s.retained_precision.body.product_attempts[ai].source_ref = 9; });
      expect(await firstFailure(c.source, c.invocation), baseId).toEqual(G('G5', 'PRODUCT_ATTEMPT_MISMATCH'));
    }
  });
  it('D24: after_rehash edits apply after rehash "all", so a forged receipt hash fails G1', async () => {
    const m = { id: 'i64_forged_receipt', base: 'ordinary_prepared_synthetic', edits: [], rehash: 'all', after_rehash: [{ path: ['retained_precision', 'receipt_sha256'], op: 'set', value: '0'.repeat(64) }] };
    const { source, invocation } = await applyEntry(m);
    expect(source.retained_precision.receipt_sha256).toBe('0'.repeat(64));
    expect(await firstFailure(source, invocation)).toEqual(G('G1', 'RECEIPT_MISMATCH'));
  });
  it('D27: the idle-Run rule reads the recorded invocation_before; a broken meter chain reports WORK', () => {
    const body = structuredClone(corpus.cases.find((c: any) => c.id === 'two_case_synthetic')).source.retained_precision.body;
    const run1 = body.cases[1].run; body.work.invocation_limit = 10; run1.invocation_before = 5;
    let e: any; try { nativeRuns(body); } catch (x) { e = x; }
    expect({ gate: e?.gate, code: e?.code }).toEqual(G('G5', 'WORK_MISMATCH'));
  });
  it('D28: every quantity-bearing reason resolves to a layout row with the same body and kind', async () => {
    const p = RV78_PROBES.find(x => x.id === 'T2_stop_rule_quantity_other_body');
    for (const tag of ['stop_rule', 'verification_estimate', 'charge', 'publication_enclosure']) {
      const m = structuredClone(p);
      for (const e of m.edits) e.path = e.path.slice(0, -1).concat(['tag']), e.value = tag;
      m.edits.push(...structuredClone(p.edits));
      if (tag === 'publication_enclosure') for (const e of structuredClone(m.edits.slice(0, 2))) m.edits.push({ ...e, path: e.path.slice(0, -1).concat(['predicate']), value: 'absolute_bound' });
      const { source, invocation } = await applyEntry(m);
      expect(await firstFailure(source, invocation), tag).toEqual(G('G5', 'ATTEMPT_MISMATCH'));
    }
  });
  it('D29: a CaseSource with an empty body inventory fails G3', async () => {
    const c = await edited('ordinary_prepared_no_data_synthetic', s => { const b = s.retained_precision.body; b.sources[0].body_membership = []; b.product_attempts[0].proof.summary_coverage = null; });
    expect(await firstFailure(c.source, c.invocation)).toEqual(G('G3', 'COVERAGE_MISMATCH'));
  });
  it('D30: a native error names its own nonselected Run', () => {
    const fixture = structuredClone(corpus.cases.find((c: any) => c.id === 'two_case_facade_after_certificate_synthetic')), b = fixture.source.retained_precision.body;
    const c1 = b.cases[1], a = b.product_attempts[1];
    c1.run.kernel_terminal = { kind: 'unresolved', reason: { space: 'unresolved', tag: 'ceiling' } };
    c1.reason = { code: 'kernel_unresolved', phase: 'kernel', cause: { kind: 'prepared_product_failure', product_attempt_ref: 1 } };
    Object.assign(a, { proof: null, result: { kind: 'unavailable', error: { kind: 'native', run_ref: c1.run.id } } });
    a.adapter.fault = null;
    a.stages = { preparation: 'completed', native: 'failed', proof_start: 'not_entered', projection: 'not_entered', maxima: 'not_entered', values: 'not_entered', aliases: 'not_entered', certificate: 'not_entered', observables: 'not_entered', g5a: 'not_entered' };
    const ids = b.cases.map((c: any) => c.basis_ref.ref_id), rows = new Map<string, any[]>(ids.map((id: string) => [id, fixture.source.results.filter((r: any) => r.basis_ref.ref_id === id)]));
    const run = (runRef: number) => { const x = structuredClone(b); x.product_attempts[1].result.error.run_ref = runRef; let e: any; try { productAttempts(x, rows); } catch (y) { e = y; } return e ? { gate: e.gate, code: e.code } : 'pass'; };
    expect(run(c1.run.id)).toBe('pass');
    expect(run(c1.run.id === 0 ? 1 : 0)).toEqual(G('G5', 'PRODUCT_ATTEMPT_MISMATCH'));
  });
  it('D21/RV81-N1: a failed verification showing its pass ran (a verification build, zero verification_lme) cannot carry an escalating stop', async () => {
    const p = structuredClone(RV78_PROBES.find(x => x.id === 'T1_escalating_failed_verification_pass_entered'));
    const record = p.edits.find((e: any) => e.path.at(-1) === 1 && e.path.at(-2) === 'records').value;
    record.work.verification_lme = 0; record.verification_shared_build_ref = 0;
    const { source, invocation } = await applyEntry(p);
    expect(await firstFailure(source, invocation)).toEqual(G('G5', 'ATTEMPT_MISMATCH'));
  });
  it('D21 widened: a non-null verification summary on an escalating failed verification shows the pass ran', async () => {
    const summary = structuredClone(corpus.cases.find((c: any) => c.id === 'verification_failure_skip_synthetic')).source.retained_precision.body.cases[0].run.records.find((r: any) => r.verification !== null).verification;
    const c = await edited('verification_failure_skip_synthetic', s => { const r = s.retained_precision.body.cases[0].run.records[1]; expect(r.outcome.kind).toBe('failed'); r.verification = structuredClone(summary); });
    expect(await firstFailure(c.source, c.invocation)).toEqual(G('G5', 'ATTEMPT_MISMATCH'));
  });
  it('RV81-N2: the kernel scope alone decides for an unreferenced build carrying a work_accounting stop', () => {
    const body = structuredClone(corpus.cases[0]).source.retained_precision.body;
    const extra = (reason: any) => { const b = structuredClone(body); b.builds.push({ ...structuredClone(b.builds[0]), id: b.builds.length, state: 'nonbudget_failure', reason }); let e: any; try { nativeRuns(b); } catch (x) { e = x; } return { gate: e?.gate, code: e?.code }; };
    // Without the work_accounting tag, the unreferenced build is only a WORK defect (every build is seen once).
    expect(extra({ space: 'stop', tag: 'condition' })).toEqual(G('G5', 'WORK_MISMATCH'));
    expect(extra({ space: 'stop', tag: 'work_accounting', fault: 'overflow' })).toEqual(G('G5', 'ATTEMPT_MISMATCH'));
  });
});

describe('07d round (D31-D33): reader-local relations', () => {
  // B3a (B3-D §6.3, decision B3D-10): 0.3.0 without a contract is outside D1.3 (the producer cannot emit it,
  // PRESSURE_CONTRACT_REQUIRED), so it is G8 INVOCATION_MISMATCH; 0.3.0 with the legacy contract is B3a's block below.
  it('D31: G8 admits model schema_version 0.1.0 and 0.2.0 without a contract, and refuses 0.3.0 without one and 0.4.0', async () => {
    const run = async (version: string) => { const { source, invocation } = await applyEntry({ id: 'i64_d31_' + version, base: 'ordinary_prepared_synthetic', edits: [], invocation_edits: [{ path: ['request', 'model', 'schema_version'], op: 'set', value: version }], rehash: 'all' }); return firstFailure(source, invocation); };
    for (const version of ['0.1.0', '0.2.0']) expect(await run(version), version).toBe('pass');
    expect(await run('0.3.0')).toEqual(G('G8', 'INVOCATION_MISMATCH'));
    expect(await run('0.4.0')).toEqual(G('G8', 'INVOCATION_MISMATCH'));
  });
  it('D32: integers and references written as integral floats are the same values (parse boundary)', async () => {
    const base = structuredClone(corpus.cases[0]);
    const receiptText = JSON.stringify(base.source.retained_precision);
    // Rewrite every bare integer literal in the receipt as an integral float, e.g. "source_ref":0 -> "source_ref":0.0.
    const floated = receiptText.replace(/([:\[,])(-?\d+)(?=[,\]}])/g, '$1$2.0');
    expect(floated).not.toBe(receiptText);
    expect(floated).toContain('"source_ref":0.0');
    const source = { ...base.source, retained_precision: JSON.parse(floated) };
    expect(source.retained_precision.body.cases[0].source_ref).toBe(0);
    const result = await validateRetainedPrecision(source, base.invocation);
    expect(result.classifications).toEqual(base.expected_classifications);
    // A float-written reference resolves: a forged source identity is still caught at G1 through source_ref 0.0.
    const forged = JSON.parse(floated); forged.body.cases[0].source_identity_sha256 = '0'.repeat(64);
    expect(await firstFailure({ ...base.source, retained_precision: forged }, base.invocation)).toEqual(G('G1', 'RECEIPT_MISMATCH'));
  });
  it('D33: a verification_estimate reason names a force or moment row', async () => {
    const edit = (tag: string) => ['records', 'attempts'].map(list => ({ path: ['retained_precision', 'body', 'cases', 0, 'run', list, 0, 'outcome', 'reason', 'tag'], op: 'set', value: tag }));
    const run = async (tag: string) => { const { source, invocation } = await applyEntry({ id: 'i64_d33_' + tag, base: 'p512_ladder_synthetic', edits: edit(tag), rehash: 'all' }); return firstFailure(source, invocation); };
    // The base reason names a translation row: legal for stop_rule and charge, never for verification_estimate.
    expect(await run('stop_rule')).toBe('pass');
    expect(await run('charge')).toBe('pass');
    expect(await run('verification_estimate')).toEqual(G('G5', 'ATTEMPT_MISMATCH'));
  });
});

describe('07e round (D34): -0 anywhere in the receipt fails G2 ENCODING', () => {
  const ENCODING = G('G2', 'ENCODING_MISMATCH');
  it('D34: the enum member G5aError.quantity_kind written as -0 fails G2; written as 0 it passes G2', async () => {
    const m = corpus.must_pass.find((x: any) => x.id === 'cert_failed_after_summary_storage');
    const run = async (cause: any) => {
      const entry = { ...structuredClone(m), id: 'i64_d34_quantity_kind', edits: [...structuredClone(m.edits), { path: ['retained_precision', 'body', 'product_attempts', 1, 'result', 'error'], op: 'set', value: { kind: 'g5a', cause } }] };
      const { source, invocation } = await applyEntry(entry);
      return { kind: source.retained_precision.body.product_attempts[1].result.error.cause.quantity_kind, failure: await firstFailure(source, invocation) };
    };
    for (const cause of [{ kind: 'sanity', body: 0 }, { kind: 'lower', member: 0 }]) {
      const negative = await run({ ...cause, quantity_kind: -0 });
      expect(Object.is(negative.kind, -0)).toBe(true);
      expect(negative.failure).toEqual(ENCODING);
      const positive = await run({ ...cause, quantity_kind: 0 });
      expect(positive.failure === 'pass' || !['G0', 'G1', 'G2'].includes(positive.failure.gate), JSON.stringify(positive.failure)).toBe(true);
    }
  });
  it('D34: the const member source_decline.constructor_counts.directional_springs written as -0 fails G2; written as 0 it keeps its entry outcome', async () => {
    const m = corpus.mutations.find((x: any) => x.id === 'unavailable_attempt_under_source_error_cause');
    const run = async (zero: number) => {
      const entry = { ...structuredClone(m), id: 'i64_d34_directional_springs', edits: [...structuredClone(m.edits), { path: ['retained_precision', 'body', 'cases', 1, 'source_decline', 'constructor_counts', 'directional_springs'], op: 'set', value: zero }] };
      const { source, invocation } = await applyEntry(entry);
      return firstFailure(source, invocation);
    };
    expect(await run(-0)).toEqual(ENCODING);
    expect(await run(0)).toEqual(m.expected);
  });
  it('D34: every numeric 0 in a base receipt, rewritten as -0, fails G2 (hashes are unchanged: canonical JSON writes 0)', async () => {
    const c = corpus.cases.find((x: any) => x.id === 'ordinary_prepared_synthetic');
    const zeros: (string | number)[][] = [];
    const walk = (v: any, path: (string | number)[]) => {
      if (typeof v === 'number') { if (v === 0) zeros.push(path); }
      else if (v !== null && typeof v === 'object') for (const [k, x] of Object.entries(v)) walk(x, [...path, Array.isArray(v) ? Number(k) : k]);
    };
    walk(c.source.retained_precision, []);
    expect(zeros.length).toBeGreaterThan(100);
    const outcomes = new Map<string, number>();
    for (const path of zeros) {
      const source = structuredClone(c.source); let v = source.retained_precision;
      for (const k of path.slice(0, -1)) v = v[k];
      v[path.at(-1)!] = -0;
      const failure = await firstFailure(source, c.invocation), key = JSON.stringify(failure);
      outcomes.set(key, (outcomes.get(key) ?? 0) + 1);
    }
    expect([...outcomes]).toEqual([[JSON.stringify(ENCODING), zeros.length]]);
  });
  it('RV78-N1: the harness rehash indexes only strict integral values and skips everything else', async () => {
    const items = ['a', 'b'];
    expect([0, 1, 1.0, 0.0].map(r => rehashRef(items, r))).toEqual(['a', 'b', 'b', 'a']);
    expect([true, false, 0.5, -0, -1, 2, NaN, Infinity, null, undefined, '0'].map(r => rehashRef(items, r))).toEqual(Array(11).fill(undefined));
    // A selected case whose source_ref is not an index keeps its recorded identity hash; the reader reports the reference.
    for (const [ref, gate] of [[true, 'G1'], [0.5, 'G2'], ['0', 'G1']] as const) {
      const { source, invocation } = await applyEntry({ id: 'i64_rv78_n1', base: 'ordinary_prepared_synthetic', edits: [{ path: ['retained_precision', 'body', 'cases', 0, 'source_ref'], op: 'set', value: ref }], rehash: 'all' });
      expect(source.retained_precision.body.cases[0].source_identity_sha256).toBe(corpus.cases[0].source.retained_precision.body.cases[0].source_identity_sha256);
      expect((await firstFailure(source, invocation) as any).gate, String(ref)).toBe(gate);
    }
    // The format's edit_paths rule: an array index in an edit path is a strict index too; anything else refuses the entry.
    for (const index of [true, 0.5, -0, '0'])
      await expect(applyEntry({ id: 'i64_rv78_n1_path', base: 'ordinary_prepared_synthetic', edits: [{ path: ['retained_precision', 'body', 'cases', index, 'source_ref'], op: 'set', value: 0 }], rehash: 'all' })).rejects.toThrow('edit path index');
    expect((await applyEntry({ id: 'i64_rv78_n1_path_ok', base: 'ordinary_prepared_synthetic', edits: [{ path: ['retained_precision', 'body', 'cases', 0.0, 'source_ref'], op: 'set', value: 0 }], rehash: 'all' })).source.retained_precision.body.cases[0].source_ref).toBe(0);
  });
});

describe('07f round (D37): an unavailable attempt error kind and its stage record agree in both directions', () => {
  const PA = G('G5', 'PRODUCT_ATTEMPT_MISMATCH'), A = ['retained_precision', 'body', 'product_attempts', 1];
  const KEYS = ['preparation', 'native', 'proof_start', 'projection', 'maxima', 'values', 'aliases', 'certificate', 'observables', 'g5a'];
  const NAMES: Record<string, string> = { '-': 'not_entered', C: 'completed', F: 'failed' };
  const OBS = { kind: 'observable', cause: { kind: 'storage', detail: 'observable view' } }, G5A = { kind: 'g5a', cause: { kind: 'sanity', body: 0, quantity_kind: 0 } };
  const check = (mark: string, error?: any) => (mark === 'P' ? { kind: 'passed' } : mark === 'F' ? { kind: 'failed', error } : { kind: 'not_entered' });
  // The facade base's attempt 1 (Capture after a passed certificate) with its error, stage record and three checks replaced.
  const run = async (error: any, record: string, checks: string) => {
    const edits = [
      { path: [...A, 'result'], op: 'set', value: { kind: 'unavailable', error } },
      { path: [...A, 'stages'], op: 'set', value: Object.fromEntries(KEYS.map((k, i) => [k, NAMES[record[i]]])) },
      { path: [...A, 'proof', 'checks'], op: 'set', value: { certificate: check(checks[0]), observables: check(checks[1], OBS), g5a: check(checks[2], G5A) } },
    ];
    const { source, invocation } = await applyEntry({ id: 'i64_d37', base: 'two_case_facade_after_certificate_synthetic', edits, rehash: 'all' });
    return firstFailure(source, invocation);
  };
  it('D37: each native-consistent post-certificate shape stays clear of G5 PRODUCT_ATTEMPT (Y6)', async () => {
    for (const [error, record, checks] of [
      [{ kind: 'capture', cause: { kind: 'storage', detail: 'adapter vector' } }, 'CCCCCCCC--', 'P--'],
      [{ kind: 'capture', cause: { kind: 'storage', detail: 'adapter vector' } }, 'CCCCCCCCCC', 'PPP'],
      [{ kind: 'numeric', cause: null }, 'CCCCCCCCCC', 'PPP'],
      [OBS, 'CCCCCCCCFC', 'PFP'], [OBS, 'CCCCCCCCFF', 'PFF'], [G5A, 'CCCCCCCCCF', 'PPF'],
    ] as const) expect(await run(error, record, checks), `${error.kind} ${record}`).not.toEqual(PA);
  });
  it('D37: a kind the stage record cannot produce, or a presupposed stage not recorded so, is G5 PRODUCT_ATTEMPT', async () => {
    const capture = { kind: 'capture', cause: { kind: 'storage', detail: 'adapter vector' } }, abandoned = { kind: 'abandoned', cause: { kind: 'storage', detail: 'x' }, proof: { kind: 'storage' } };
    const shapes = [
      [{ kind: 'numeric', cause: null }, 'CCCCCCCC--', 'P--'],   // X1: numeric with Observables and G5a not entered
      [{ kind: 'numeric', cause: null }, 'CCCCCCCCCF', 'PPF'],   // numeric needs both checks passed (native: G5a)
      [{ kind: 'numeric', cause: null }, 'CCCCCCCCFC', 'PFP'],   // native: Observable
      [G5A, 'CCCCCCCCFF', 'PFF'],                                 // g5a needs Observables passed (native: Observable)
      [G5A, 'CCCCCCCC--', 'P--'],                                 // X1: g5a with G5a not entered
      [OBS, 'CCCCCCCC--', 'P--'],                                 // X1: observable with Observables not entered
      [{ kind: 'proof', cause: { kind: 'storage' } }, 'CCCCCCCC--', 'P--'], // X1: proof with the certificate passed
      [{ kind: 'values', cause: { kind: 'storage' }, proof: { kind: 'storage' } }, 'CCCCCCCC--', 'P--'], // X1: values with Values completed
      [capture, 'CCCCCCCCCF', 'PPF'], [capture, 'CCCCCCCCFC', 'PFP'], // a failed check after the certificate is reported as its own kind
      [abandoned, 'CCCCCCCC--', 'P--'],                           // abandonment never follows a completed certificate
    ] as const;
    const outcomes: [string, unknown][] = [];
    for (const [error, record, checks] of shapes) outcomes.push([`${error.kind} ${record}`, await run(error, record, checks)]);
    expect(outcomes).toEqual(shapes.map(([error, record]) => [`${error.kind} ${record}`, PA]));
    // A failed preparation is reported as `preparation`, never `capture` (the old one-direction rule admitted both).
    const prepared = async (error: any) => { const { source, invocation } = await applyEntry({ id: 'i64_d37_preparation', base: 'two_case_preparation_failure_synthetic', edits: [{ path: [...A, 'result'], op: 'set', value: { kind: 'unavailable', error } }], rehash: 'all' }); return firstFailure(source, invocation); };
    expect(await prepared({ kind: 'capture', cause: { kind: 'storage', detail: 'adapter vector' } })).toEqual(PA);
    expect(await prepared(corpus.cases.find((c: any) => c.id === 'two_case_preparation_failure_synthetic').source.retained_precision.body.product_attempts[1].result.error)).toBe('pass');
  });
});

describe('07f round (RV81-N1 R35): only -0 maps to 0 in the G1 const/enum comparison', () => {
  it('RV81-N1: a tiny non-zero value at a const-0 or enum position fails G1, not G2', async () => {
    const decline = corpus.mutations.find((x: any) => x.id === 'unavailable_attempt_under_source_error_cause');
    const storage = corpus.must_pass.find((x: any) => x.id === 'cert_failed_after_summary_storage');
    const springs = async (v: number) => { const { source, invocation } = await applyEntry({ ...structuredClone(decline), id: 'i64_r35_springs', edits: [...structuredClone(decline.edits), { path: ['retained_precision', 'body', 'cases', 1, 'source_decline', 'constructor_counts', 'directional_springs'], op: 'set', value: v }] }); return firstFailure(source, invocation); };
    const kind = async (v: number) => { const { source, invocation } = await applyEntry({ ...structuredClone(storage), id: 'i64_r35_kind', edits: [...structuredClone(storage.edits), { path: ['retained_precision', 'body', 'product_attempts', 1, 'result', 'error'], op: 'set', value: { kind: 'g5a', cause: { kind: 'sanity', body: 0, quantity_kind: v } } }] }); return firstFailure(source, invocation); };
    for (const tiny of [Number.MIN_VALUE, -Number.MIN_VALUE, 2 ** -1022, Number.EPSILON]) {
      expect(await springs(tiny), String(tiny)).toEqual(G('G1', 'RECEIPT_MISMATCH'));
      expect(await kind(tiny), String(tiny)).toEqual(G('G1', 'RECEIPT_MISMATCH'));
    }
    // Next to the enum member 1 (1 + tiny rounds to 1 for the subnormal and 2^-1022, so use the neighbours of 1).
    for (const near of [1 + Number.EPSILON, 1 - Number.EPSILON / 2]) expect(await kind(near), String(near)).toEqual(G('G1', 'RECEIPT_MISMATCH'));
    // Controls: the exact members pass G1 and G2; -0 is G2 (D34).
    expect(await springs(-0)).toEqual(G('G2', 'ENCODING_MISMATCH'));
    expect(['G0', 'G1', 'G2'].includes((await kind(1) as any).gate ?? 'pass')).toBe(false);
  });
});

describe('native p512 floor rounding, independent of any corpus case', () => {
  // Literal transcription of verify.rs phi_512 and adaptive.rs next_up, for comparison only.
  const fromWord = (word: bigint) => { const v = new DataView(new ArrayBuffer(8)); v.setBigUint64(0, word); return v.getFloat64(0); };
  function nativePhi(hat: number): number {
    const nearest = hat * fromWord(0x2490000000000000n);
    if (!(nearest * fromWord(0x5b50000000000000n) < hat)) return nearest;
    return nearest === 0 ? fromWord(1n) : fromWord(BigInt('0x' + binary64Bits(nearest)) + 1n);
  }
  const words = [0n, 1n, 3n, 0x000fffffffffffffn, 0x0010000000000000n, 0x1b6fffffffffffffn, 0x1b70000000000001n, 0x1b80000000000003n, 0x2000000000000001n,
    0x249fffffffffffffn, 0x3ff0000000000000n, 0x3ff0000000000001n, 0x426d1a94a2000000n, 0x7fefffffffffffffn];
  let seed = 0x9e3779b97f4a7c15n;
  for (let i = 0; i < 200; i++) { seed = (seed * 6364136223846793005n + 1442695040888963407n) & ((1n << 64n) - 1n); words.push(seed % 0x7ff0000000000000n); }
  it('phi512 is the least binary64 at or above 2^-438 * e_hat and equals the native transcription', () => {
    for (const word of words) {
      const hat = fromWord(word), phi = phi512(hat);
      expect(binary64Bits(phi), word.toString(16)).toBe(binary64Bits(nativePhi(hat)));
      const exact = multiply(fraction(word), [1n, 1n << 438n]);
      expect(BigInt('0x' + binary64Bits(phi)), word.toString(16)).toBe(upperWord(exact));
      expect(phi > 0).toBe(hat > 0);
    }
  });
  // Row-level transcription of final_case.rs summary_coverage_data (D = false in C3): every
  // kind is absent, input-only, non-input zero or non-input nonzero; L = 0 or not; floor none or
  // any sign pair. The feasible stop set must equal the natively generated set exactly.
  it('stopFeasible admits exactly the natively generated stop vectors, including L = 0', () => {
    const bits = (n: number) => [0, 1, 2, 3].map(k => ((n >> k) & 1) === 1);
    let groups = 0;
    for (let state = 0; state < 256; state++) {
      const kinds = [0, 1, 2, 3].map(k => (state >> (2 * k)) & 3); // 0 absent, 1 input-only, 2 non-input zero, 3 non-input nonzero
      const present = kinds.map(s => s > 0), nonInput = kinds.map(s => s >= 2);
      for (const extentNonzero of [false, true]) for (const floor of [null, [false, false], [false, true], [true, false], [true, true]] as (boolean[] | null)[]) {
        const native = (nonzero: boolean[]) => {
          let positive = [0, 1, 2, 3].map(k => nonInput[k] && nonzero[k]);
          if (extentNonzero) positive = [positive[0] || positive[1], positive[0] || positive[1], positive[2] || positive[3], positive[2] || positive[3]];
          if (floor) { positive[2] ||= floor[0]; positive[3] ||= floor[1]; }
          return [0, 1, 2, 3].map(k => present[k] && (positive[k] || (nonInput[k] && nonzero[k])));
        };
        const generated = new Set<string>();
        for (let a = 0; a < 16; a++) generated.add(JSON.stringify(native(bits(a))));
        const floors = [floor ?? [false, false]];
        for (let s = 0; s < 16; s++) {
          const stop = bits(s);
          expect(stopFeasible(stop, present, nonInput, extentNonzero, floors), JSON.stringify({ kinds, extentNonzero, floor, stop })).toBe(generated.has(JSON.stringify(stop)));
        }
        groups++;
      }
    }
    expect(groups).toBe(2560);
    // L = 0 keeps translation/rotation independent; L != 0 couples them.
    expect(stopFeasible([true, false, false, false], [true, true, true, true], [true, true, true, true], false, [[false, false]])).toBe(true);
    expect(stopFeasible([true, false, false, false], [true, true, true, true], [true, true, true, true], true, [[false, false]])).toBe(false);
  });
  it('e_hat couples force and moment only for a nonzero extent', () => {
    expect(eHat(2, 3, 0)).toEqual([2, 3]);
    expect(eHat(2, 3, 4)).toEqual([2, 8]);
    expect(eHat(1, 0, 1)).toEqual([1, 1]);
    expect(eHat(0, 0, 5)).toEqual([0, 0]);
  });
});

describe('source-bound recovery controls', () => {
  const base = () => structuredClone(corpus.cases[0]);
  async function rejectedAfterRehash(edit: (source: any) => void, gate: string, code: string) {
    const c = base(); edit(c.source); await rehash(c.source);
    await expect(validateRetainedPrecision(c.source, c.invocation)).rejects.toMatchObject({ gate, code });
  }
  it('rederives K4SRC independently of repaired semantic receipt hashes', async () => {
    await rejectedAfterRehash(source => {
      source.retained_precision.body.sources[0].kernel_source_sha256 = '1'.repeat(64);
    }, 'G8', 'RETAINED_PRECISION_PREPARATION_MISMATCH');
  });
  it('rederives K4STF even when source and group repeat the same forged digest', async () => {
    await rejectedAfterRehash(source => {
      const b = source.retained_precision.body;
      b.sources[0].stiffness_sha256 = '2'.repeat(64); b.groups[0].stiffness_sha256 = '2'.repeat(64);
    }, 'G8', 'RETAINED_PRECISION_PREPARATION_MISMATCH');
  });
  it('keeps signed bound zero invalid at G2 despite a later row-method defect', async () => {
    await rejectedAfterRehash(source => {
      source.retained_precision.body.cases[0].selection.floor_ratio = '8000000000000000';
      source.results[0].recovery_method = '';
    }, 'G2', 'RETAINED_PRECISION_ENCODING_MISMATCH');
  });
  it('binds residual precision to the actual native solve width', async () => {
    await rejectedAfterRehash(source => {
      source.retained_precision.body.cases[0].run.records[0].residual_basis = 1024;
    }, 'G5', 'RETAINED_PRECISION_ATTEMPT_MISMATCH');
  });
  it('reconciles stop-rule stages independently of the total own work', async () => {
    await rejectedAfterRehash(source => {
      const stages = source.retained_precision.body.cases[0].run.records[0].work.own_stages;
      stages.formation += stages.stop_rule; stages.stop_rule = 0;
    }, 'G5', 'RETAINED_PRECISION_WORK_MISMATCH');
  });
  it('reconciles every shared stage with both contributing build records', async () => {
    await rejectedAfterRehash(source => {
      const stages = source.retained_precision.body.builds[0].stages;
      stages.assembly += stages.formation; stages.formation = 0;
    }, 'G5', 'RETAINED_PRECISION_WORK_MISMATCH');
  });
  it('requires the first actual candidate to start at128', async () => {
    await rejectedAfterRehash(source => {
      const b = source.retained_precision.body, run = b.cases[0].run;
      run.records[0].precision = 256; run.records[0].residual_basis = 320;
      run.records[1].precision = 512; run.records[1].residual_basis = 576; run.records[1].storage.limbs_per_entry = 8;
      run.attempts[0].precision = 256; run.attempts[0].verification.precision = 512;
      b.cases[0].selection.precision = 256; b.cases[0].selection.verification_precision = 512;
    }, 'G5', 'RETAINED_PRECISION_ATTEMPT_MISMATCH');
  });
  // I57 §2/§4: G5a rederives the canonical layout and +0 prescription relation from
  // the bound source maps before compact-flag feasibility; G8 binds them later.
  it('rejects a purported input-derived force row at G5a before invocation binding', async () => {
    await rejectedAfterRehash(source => {
      const row = source.retained_precision.body.sources[0].layout.find((r: any) => r.kind === 'force');
      row.input_derived = true;
    }, 'G5a', 'RETAINED_PRECISION_SCALE_MISMATCH');
  });
  it('rejects a constrained displacement row not marked input-derived at G5a', async () => {
    await rejectedAfterRehash(source => {
      const row = source.retained_precision.body.sources[0].layout.find((r: any) => r.input_derived);
      row.input_derived = false;
    }, 'G5a', 'RETAINED_PRECISION_SCALE_MISMATCH');
  });
  it('rejects a nonzero prescription at G5a because D is false only for exact +0', async () => {
    await rejectedAfterRehash(source => {
      source.retained_precision.body.sources[0].constraints[0].value = '3ff0000000000000';
    }, 'G5a', 'RETAINED_PRECISION_SCALE_MISMATCH');
  });
  // I57 §5 trust boundary: publicly consistent attestation rewrites must not be
  // over-rejected; only producer custody/replay can catch them.
  for (const [name, index, edit] of [
    ['stop [T,T,F,F] with its exact stop roster', 0, (b: any) => { b.product_attempts[0].proof.summary_coverage[0].stop = [true, true, false, false]; b.cases[0].selection.stop_rule = b.cases[0].selection.stop_rule.slice(0, 2); }],
    ['all-false stop with an empty stop roster', 0, (b: any) => { b.product_attempts[0].proof.summary_coverage[0].stop = [false, false, false, false]; b.cases[0].selection.stop_rule = []; }],
    ['no-data all-true stop with four zero stop entries', 2, (b: any) => { b.product_attempts[0].proof.summary_coverage[0].stop = [true, true, true, true]; b.cases[0].selection.stop_rule = ['translation', 'rotation', 'force', 'moment'].map(kind => ({ body: 0, kind, value: '0000000000000000' })); }],
    ['no-data body attesting one data block with its bound', 2, (b: any) => {
      const bound = [{ body: 0, value: '3ff0000000000000' }], record = b.cases[0].run.records[1].verification;
      b.product_attempts[0].proof.summary_coverage[0].has_data = true; b.cases[0].selection.certified_bound = bound; record.bound = structuredClone(bound); record.data_blocks = 1;
    }],
  ] as const) it('admits the publicly consistent rewrite: ' + name, async () => {
    const c = structuredClone(corpus.cases[index]); edit(c.source.retained_precision.body); await rehash(c.source);
    expect((await validateRetainedPrecision(c.source, c.invocation)).classifications).toEqual(c.expected_classifications);
  });
  it('requires actual entered proof state and checks it before product work', async () => {
    await rejectedAfterRehash(source => {
      const a = source.retained_precision.body.product_attempts[0];
      a.stages.proof_start = 'not_entered';
      a.preparation.members[0].work.conversions.value = 0;
    }, 'G5', 'RETAINED_PRECISION_PRODUCT_ATTEMPT_MISMATCH');
  });
});

describe('07g round (I61 U6e): F5 and RV79-N1', () => {
  it('RV79-N1: D37 agrees with the corpus table (native source and C3, never this reader) on every well-formed record', () => {
    const table = corpus.d37, order = ['preparation', 'native', 'proof_start', 'projection', 'maxima', 'values', 'aliases', 'certificate', 'observables', 'g5a'];
    expect(table.stage_order).toEqual(order); expect(table.records).toHaveLength(25);
    const misses: string[] = [];
    for (const kind of [...Object.keys(table.kinds), ...table.unknown_kinds]) for (const record of table.records) {
      const stages = Object.fromEntries(order.map((k, i) => [k, table.marks[record[i]]]));
      if (errorStageRecordAgrees(kind, stages) !== (table.kinds[kind] ?? []).includes(record)) misses.push(`${kind} ${record}`);
    }
    expect(misses).toEqual([]);
  });
  it('F5: U1\'s M09, M10 and M20 are refused on the real milestone receipts (resealed like corpus entries)', async () => {
    for (const [text, classes] of [[milestoneSparseText, [25, 69, 3, 1]], [milestoneDenseText, [25, 69, 3, 2]]] as const) {
      const doc = JSON.parse(text), source = doc.source, cid = source.retained_precision.body.cases[0].basis_ref.ref_id;
      const accepted = await validateRetainedPrecision(structuredClone(source), structuredClone(doc.invocation));
      expect(['relative_verified', 'absolute_verified', 'input_derived', 'non_quantity'].map(k => accepted.classifications.filter((c: any) => c.class === k).length)).toEqual(classes);
      expect(accepted.numerical_eligible).toBe(true); expect(accepted.standing).toBe('eligible');
      const names = (d: any) => Array.isArray(d.affected_refs) && d.affected_refs.includes(cid), retained = (d: any) => String(d.code).startsWith('RETAINED_PRECISION_');
      const exact = source.diagnostics.filter((d: any) => names(d) && !retained(d)).map((d: any) => d.id);
      expect(source.retained_precision.body.ordinary_attempts[0].diagnostic_refs).toEqual(exact);
      const m09 = source.diagnostics.filter(names).map((d: any) => d.id), m10 = source.diagnostics.filter((d: any) => !retained(d)).map((d: any) => d.id);
      expect(m09).not.toEqual(exact); expect(m10).not.toEqual(exact);
      const refs = ['retained_precision', 'body', 'ordinary_attempts', 0, 'diagnostic_refs'];
      for (const [edits, want] of [
        [[{ path: refs, op: 'set', value: m09 }], G('G5', 'ATTEMPT_MISMATCH')],
        [[{ path: refs, op: 'set', value: m10 }], G('G5', 'ATTEMPT_MISMATCH')],
        [[{ path: ['results', 0, 'recovery_method'], op: 'set', value: 'other' }], G('G6', 'ROW_METHOD_MISMATCH')],
      ] as const) {
        const edited = structuredClone(source); applyEdits(edited, edits as any); await rehash(edited);
        expect(await firstFailure(edited, structuredClone(doc.invocation))).toEqual(want);
      }
      const resealed = structuredClone(source); await rehash(resealed); expect(resealed).toEqual(source);
    }
  });
  it('U7: a solved status is required before the eligibility conjunct; a blocked envelope is refused at G7 with TS\'s own base code', async () => {
    // I66 U7 slice F observation 3: each language's own base code (06b); the case file's scope names all three.
    for (const text of [milestoneSparseText, milestoneDenseText]) {
      const doc = JSON.parse(text);
      for (const status of ['MODEL_INCOMPLETE', 'MECHANICS_FAILED', 'NOT_RUN']) {
        const unsolved = structuredClone(doc.source); unsolved.status.mechanics = status; await rehash(unsolved);
        expect(unsolved.retained_precision.receipt_sha256, status).not.toBe(doc.source.retained_precision.receipt_sha256);
        let error: any; try { await validateRetainedPrecision(unsolved, structuredClone(doc.invocation)); } catch (e) { error = e; }
        expect(error, status).toBeInstanceOf(RetainedPrecisionError);
        // TS's base validator has one code; its detail names the blocked-envelope check.
        expect({ gate: error.gate, code: error.code, detail: error.detail }, status).toEqual({ gate: 'G7', code: 'SOURCE_PREVIEW_PHYSICS_EVIDENCE_INVALID', detail: 'blocked envelope carries evidence, rows or headlines' });
      }
    }
  });
  it('RV94 N-3 (B6, PLAN decision 11): an invalid enum value in a not_required case\'s quality is refused at G7 with the base readers\' code', async () => {
    // The class on 07j's not_required entry: TS's G7 header refusal now carries the base readers' own header code
    // (Python and Rust SOURCE_NUMERICAL_CASE_INVALID); the shared corpus probe g7_not_required_quality_enum_invalid
    // is accuracy_evidence, and 07m pins the class's other statuses and parts.
    const nr = corpus.must_pass.find((m: any) => m.id === 'not_required_second_case_checks_passed');
    const accepted = await applyEntry(nr); expect((await validateRetainedPrecision(accepted.source, accepted.invocation)).numerical_eligible).toBe(true);
    for (const [field, value] of [['accuracy_evidence', 'estimated'], ['structural_status', 'mechanism_detected'], ['model_matrix_fidelity', 'reduced']]) {
      const { source, invocation } = await applyEntry({ ...structuredClone(nr), id: 'rv94_n3_' + field, edits: [...structuredClone(nr.edits), { path: ['numerical_quality', 'cases', 1, field], op: 'set', value }] });
      expect(await firstFailure(source, invocation), field).toEqual({ gate: 'G7', code: 'SOURCE_NUMERICAL_CASE_INVALID' });
    }
    const probe = corpus.mutations.find((m: any) => m.id === 'g7_not_required_quality_enum_invalid');
    expect(probe.expected_by_reader).toEqual(Object.fromEntries(['python', 'typescript', 'rust'].map(k => [k, { gate: 'G7', code: 'SOURCE_NUMERICAL_CASE_INVALID' }])));
  });
});

// Snapshot 07l (U8-2): the count pins Python and Rust carry, and the appended L = 0 slices pinned here rather than
// read from the corpus, so a dropped, reordered or re-expected L = 0 entry fails even though the per-entry tests
// above follow the corpus's own expectation.
describe('07l round (U8-2): counts and the appended producer-solved L = 0 entries', () => {
  const MODES = ['sparse_interactive', 'dense_scrutiny'], SCALE = G('G5a', 'SCALE_MISMATCH');
  const ELIGIBLE = { invocation_bound: true, numerical_eligible: true, standing: 'eligible' };
  const l0 = (names: string[]) => MODES.flatMap(mode => names.map(name => `${name}_${mode}`));
  const baseOf = (id: string) => `u8_l0_isolated_node_${MODES.find(mode => id.endsWith('_' + mode))}`;
  const eligibility = (r: any) => ({ invocation_bound: r.invocation_bound, numerical_eligible: r.numerical_eligible, standing: r.standing });
  it('07l: 17 cases, 286 mutations and 28 must-pass entries (07m: 294 mutations); 15 bases and 18 must-pass entries are eligible', () => {
    expect([corpus.cases.length, corpus.mutations.length, corpus.must_pass.length]).toEqual([17, 294, 28]);
    expect([corpus.cases.filter((c: any) => c.expected.numerical_eligible).length, corpus.must_pass.filter((m: any) => m.expected_eligibility.numerical_eligible).length]).toEqual([15, 18]);
  });
  it('07l: the two appended producer-solved bases are eligible, with the class counts all three readers observed (I68)', async () => {
    expect(corpus.cases.slice(15).map((c: any) => c.id)).toEqual(MODES.map(mode => `u8_l0_isolated_node_${mode}`));
    for (const [index, classes] of [[15, [25, 78, 9, 1]], [16, [25, 78, 9, 2]]] as const) {
      const c = corpus.cases[index], result = await validateRetainedPrecision(structuredClone(c.source), structuredClone(c.invocation));
      expect(c.provenance.kind, c.id).toBe('producer_solved');
      expect(eligibility(result), c.id).toEqual(ELIGIBLE);
      expect(['relative_verified', 'absolute_verified', 'input_derived', 'non_quantity'].map(k => result.classifications.filter((x: any) => x.class === k).length), c.id).toEqual(classes);
      expect(result.classifications, c.id).toEqual(c.expected_classifications);
    }
  });
  it('07l: the 8 appended mutations are refused at G5a SCALE', async () => {
    const appended = corpus.mutations.slice(278, 286);
    expect(appended.map((m: any) => m.id)).toEqual([...l0(['isolated_rotation_stop', 'isolated_has_data', 'isolated_estimate_coupled']), ...l0(['isolated_translation_rotation_stop'])]);
    for (const m of appended) {
      expect([m.base, m.expected, m.expected_by_reader], m.id).toEqual([baseOf(m.id), SCALE, undefined]);
      const { source, invocation } = await applyEntry(m);
      expect(await firstFailure(source, invocation), m.id).toEqual(SCALE);
    }
  });
  it('07l: the 4 appended must-pass entries are admitted, eligible, with their base classifications', async () => {
    const appended = corpus.must_pass.slice(24);
    expect(appended.map((m: any) => m.id)).toEqual([...l0(['isolated_estimate_uncoupled']), ...l0(['isolated_translation_stop'])]);
    for (const m of appended) {
      expect([m.base, m.expected, m.expected_eligibility], m.id).toEqual([baseOf(m.id), 'pass', ELIGIBLE]);
      const { base, source, invocation } = await applyEntry(m);
      const result = await validateRetainedPrecision(source, invocation);
      expect(eligibility(result), m.id).toEqual(ELIGIBLE);
      expect(result.classifications, m.id).toEqual(base.expected_classifications);
    }
  });
});

// B6: mutation 277 (RV94 N-3's G7 probe, 07k) as its own one-entry slice, and 07m's eight appended G7 mutations,
// each observed by this reader against its own expectation and tallied against a literal, so a dropped, moved or
// re-expected entry fails even though the per-entry tests above follow the corpus's own expectation.
describe('07k and 07m slices (B6): the G7 base header codes, shared with Python and Rust (PLAN decision 11)', () => {
  async function slice(start: number, stop: number, ids: string[], want: Record<string, number>) {
    const entries = corpus.mutations.slice(start, stop), tally: Record<string, number> = {};
    expect(entries.map((m: any) => m.id)).toEqual(ids);
    for (const m of entries) {
      const { source, invocation } = await applyEntry(m), observed = await firstFailure(source, invocation);
      expect(observed, m.id).toEqual(m.expected_by_reader?.typescript ?? m.expected);
      const key = observed === 'pass' ? 'pass' : `${observed.gate} ${observed.code}`; tally[key] = (tally[key] ?? 0) + 1;
    }
    expect(tally).toEqual(want);
  }
  it('07k: mutation 277 is refused at G7 SOURCE_NUMERICAL_CASE_INVALID', async () => {
    await slice(277, 278, ['g7_not_required_quality_enum_invalid'], { 'G7 SOURCE_NUMERICAL_CASE_INVALID': 1 });
  });
  it('07m: the N-3 class at its full width, then the sibling header classes, each with the base readers\' code', async () => {
    await slice(286, 294, ['g7_selected_quality_enum_invalid', 'g7_unavailable_quality_enum_invalid', 'g7_quality_case_evidence_ref_empty', 'g7_quality_case_extra_member',
      'g7_quality_status_invalid', 'g7_formulation_limitations_empty', 'g7_contract_evidence_null', 'g7_source_block_recovery_present'],
      { 'G7 SOURCE_NUMERICAL_CASE_INVALID': 4, 'G7 SOURCE_NUMERICAL_QUALITY_INVALID': 1, 'G7 SOURCE_FORMULATION_BASIS_UNSUPPORTED': 1,
        'G7 SOURCE_PREVIEW_PHYSICS_EVIDENCE_REQUIRED': 1, 'G7 SOURCE_BLOCKS_LEGACY_DOWNGRADE_FORBIDDEN': 1 });
  });
});

// B1 SR-TS (I92; PLAN_v2 §2.4; DESIGN_v2 §2 and §3.2-§3.3): reader-local tests on synthetic n-case receipts, derived
// from the shared two-case bases and must-pass entries by edits and a full reseal, as Rust's b1_* tests (I90) derive
// theirs. They are not shared corpus entries: SC's 07n pins those (W-C2, d38_beside_selected with m1-m8, F-1's five and
// not_required's three). Each verdict is the reader's first failure, or { admitted: numerical_eligible }.
describe('B1 SR-TS: R-D38 (4b), F-1 text B per case and the not_required rule (reader-local, synthetic n-case receipts)', () => {
  const F_BASE = 'two_case_facade_after_certificate_synthetic', P_BASE = 'two_case_preparation_failure_synthetic', DENSE = 'ordinary_prepared_dense_synthetic';
  const NOT_REQUIRED = 'not_required_second_case_checks_passed', UNAVAILABLE_ROW = 'case:unavailable-row', SELECTED_ROW = 'case:six-component-load';
  const PREP = G('G8', 'PREPARATION_MISMATCH'), PRODUCT = G('G5', 'PRODUCT_ATTEMPT_MISMATCH'), ATTEMPT = G('G5', 'ATTEMPT_MISMATCH');
  const rb = (...tail: (string | number)[]) => ['retained_precision', 'body', ...tail];
  const set = (path: (string | number)[], value: unknown) => ({ path, op: 'set', value });
  const baseSource = (id: string) => structuredClone(corpus.cases.find((c: any) => c.id === id).source);
  /** An entry on `base`: a must-pass entry's edits (when named), then `edits`; invocation edits rebind the digest. */
  const entry = (base: string, must: string | null, edits: any[], invocationEdits: any[] = []) => {
    const pre = must === null ? null : corpus.must_pass.find((m: any) => m.id === must);
    if (pre) expect(pre.base).toBe(base);
    return { id: 'b1_probe', base, edits: [...structuredClone(pre?.edits ?? []), ...edits], invocation_edits: [...structuredClone(pre?.invocation_edits ?? []), ...invocationEdits], rehash: 'all' };
  };
  async function verdict(e: any): Promise<unknown> {
    const { source, invocation } = await applyEntry(e);
    try { return { admitted: (await validateRetainedPrecision(source, invocation)).numerical_eligible }; }
    catch (error) { expect(error).toBeInstanceOf(RetainedPrecisionError); return { gate: (error as any).gate, code: (error as any).code }; }
  }
  async function table(cases: [string, any, unknown][]): Promise<void> {
    const misses: string[] = [];
    for (const [name, e, want] of cases) { const got = await verdict(e); if (JSON.stringify(got) !== JSON.stringify(want)) misses.push(`${name}: got ${JSON.stringify(got)} want ${JSON.stringify(want)}`); }
    expect(misses).toEqual([]);
  }
  const rowIndex = (source: any, kind: string, caseId: string) => source.results.findIndex((r: any) => r.kind === kind && r.basis_ref.ref_id === caseId);
  /** The dense base's parity row moved to `caseId` with a fresh id; a non-selected case's row carries no recovery method (G6). */
  const parityRow = (caseId: string, id: string, keepMethod = false) => {
    const row = baseSource(DENSE).results[1]; expect(row.kind).toBe('sparse_live_path_dense_parity_relative_delta');
    row.id = id; row.basis_ref.ref_id = caseId; if (!keepMethod) delete row.recovery_method; return row;
  };
  const withRows = (source: any, extra: any[]) => set(['results'], [...source.results, ...extra]);
  /** R-D38 (4b) on F_BASE, as SC's d38_beside_selected rewrites W-C2's case C (Rust's d38_edits): case 1's native stage
   * failed before any Run, beside selected case 0. No Run, execution_order, Call or Group entry names it; the call's
   * after-value and charged are recomputed (case 1's Run built nothing); a typed capture cause; every hash resealed. */
  function d38Edits(): any[] {
    const body = baseSource(F_BASE).retained_precision.body, after = body.cases[0].run.invocation_after;
    expect(body.builds.every((b: any) => b.origin.run === 0)).toBe(true);
    const stages: Record<string, string> = Object.fromEntries(['preparation', 'native', 'proof_start', 'projection', 'maxima', 'values', 'aliases', 'certificate', 'observables', 'g5a'].map(k => [k, 'not_entered']));
    Object.assign(stages, { preparation: 'completed', native: 'failed' });
    return [
      set(rb('cases', 1, 'run'), null),
      set(rb('cases', 1, 'reason'), { code: 'source_unavailable', phase: 'preparation', cause: { kind: 'prepared_product_failure', product_attempt_ref: 1 } }),
      set(rb('product_attempts', 1, 'run_ref'), null),
      set(rb('product_attempts', 1, 'proof'), null),
      set(rb('product_attempts', 1, 'stages'), stages),
      set(rb('product_attempts', 1, 'result'), { kind: 'unavailable', error: { kind: 'capture', cause: { kind: 'origin', cause: { kind: 'capacity' } } } }),
      set(rb('calls', 0, 'owner_refs'), [{ kind: 'case', index: 0 }]),
      set(rb('calls', 0, 'source_refs'), [0]),
      set(rb('calls', 0, 'run_refs'), [0]),
      set(rb('calls', 0, 'invocation_after'), after),
      set(rb('groups', 0, 'source_refs'), [0]),
      set(rb('work', 'charged'), after),
      set(rb('work', 'execution_order'), [{ kind: 'case', index: 0 }]),
    ];
  }
  it('R-D38 (4b): a capture failure before any Run beside a selected case is admitted (needs_recompute); m1-m8 and the other (4b) conjuncts are refused', async () => {
    const d38 = entry(F_BASE, null, d38Edits());
    const { source, invocation } = await applyEntry(d38);
    const bound = await validateRetainedPrecision(source, invocation), unbound = await validateRetainedPrecision(structuredClone(source));
    expect([bound.invocation_bound, bound.numerical_eligible, bound.standing]).toEqual([true, false, 'needs_recompute']);
    expect([unbound.invocation_bound, unbound.numerical_eligible, unbound.standing]).toEqual([false, false, 'needs_recompute']);
    // Case 0 stays selected with its base classifications; the unavailable case has none.
    expect(bound.classifications).toEqual(corpus.cases.find((c: any) => c.id === F_BASE).expected_classifications);
    const transported = structuredClone(source); delete transported.results;
    expect((await validateRetainedPrecisionTransport(transported)).standing).toBe('needs_recompute');
    const attempt = (...tail: (string | number)[]) => rb('product_attempts', 1, ...tail);
    const variant = (...edits: any[]) => entry(F_BASE, null, [...d38Edits(), ...edits]);
    // Rust's first failures (I90 RETURN §5 test 1), except where noted; m1 with only error.kind changed is I90's G1 note.
    await table([
      ['m1 error {kind: native, run_ref: 1}', variant(set(attempt('result', 'error'), { kind: 'native', run_ref: 1 })), PRODUCT],
      ['m1 error.kind alone set to native (the capture cause stays)', variant(set(attempt('result', 'error', 'kind'), 'native')), G('G1', 'RECEIPT_MISMATCH')],
      ['m2 native completed', variant(set(attempt('stages', 'native'), 'completed')), PRODUCT],
      ['m3 run_ref while the case has no Run', variant(set(attempt('run_ref'), 1)), PRODUCT],
      ['m4 execution_order still lists the case', variant(set(rb('work', 'execution_order'), [{ kind: 'case', index: 0 }, { kind: 'case', index: 1 }])), G('G3', 'COVERAGE_MISMATCH')],
      ['m5 proof_start completed', variant(set(attempt('stages', 'proof_start'), 'completed')), PRODUCT],
      ['m6 source_ref null with preparation completed', variant(set(attempt('source_ref'), null)), PRODUCT],
      ["m7 the case's source in the call's source_refs", variant(set(rb('calls', 0, 'source_refs'), [0, 1])), ATTEMPT],
      ["m7 the case's source in the group's source_refs", variant(set(rb('groups', 0, 'source_refs'), [0, 1])), ATTEMPT],
      ["m8 the case's source_ref differs from the attempt's", variant(set(rb('cases', 1, 'source_ref'), 0)), PRODUCT],
      ['result ready', variant(set(attempt('result'), { kind: 'ready' })), PRODUCT],
      ['preparation failed', variant(set(attempt('stages', 'preparation'), 'failed')), PRODUCT],
      ['observables and G5a entered', variant(set(attempt('stages', 'observables'), 'failed'), set(attempt('stages', 'g5a'), 'failed')), PRODUCT],
      ['reason code kernel_unresolved', variant(set(rb('cases', 1, 'reason', 'code'), 'kernel_unresolved')), PRODUCT],
      ['reason phase kernel', variant(set(rb('cases', 1, 'reason', 'phase'), 'kernel')), PRODUCT],
      ['the cause names the other attempt', variant(set(rb('cases', 1, 'reason', 'cause', 'product_attempt_ref'), 0)), PRODUCT],
      // TS's ordinary class (G5, before the product class, as in Rust) also checks C2's cause branches: a receipt_failure
      // cause needs phase receipt. So this compound variant is G5 ATTEMPT here; Rust has no such branch rule and reports
      // G5 PRODUCT_ATTEMPT (D19; I90 RETURN §5). With the branch satisfied, TS too reports D19's PRODUCT_ATTEMPT.
      ['the cause is a receipt_failure (phase preparation)', variant(set(rb('cases', 1, 'reason', 'cause'), { kind: 'receipt_failure', check: 'association', field_path: 'b1' })), ATTEMPT],
      ['the cause is a receipt_failure (phase receipt, code receipt_encoding)', variant(set(rb('cases', 1, 'reason'), { code: 'receipt_encoding', phase: 'receipt', cause: { kind: 'receipt_failure', check: 'association', field_path: 'b1' } })), PRODUCT],
    ]);
  });
  it('G5 not_required (DESIGN_v2 §3.3, decision 9): a W2-published case with the verdict checks_passed is admitted and eligible', async () => {
    const report = baseSource(P_BASE).retained_precision.body.ordinary_attempts[1].initial.report_diagnostic_ref;
    const ordinary = (key: string) => rb('ordinary_attempts', 1, key);
    const evaluation = [
      set(ordinary('initial'), { kind: 'structural_failure', error: { tag: 'range', detail: 'b1' }, diagnostic_ref: null }),
      set(ordinary('w2'), { kind: 'published', trigger: { tag: 'evaluation', error: { tag: 'range', detail: 'b1' } }, force_scale_exponent: 3, report_diagnostic_ref: report }),
    ];
    const formation = [
      set(ordinary('initial'), { kind: 'formation_failure', error: { tag: 'numerical_range', name: 'b1' }, basis_index: 0 }),
      set(ordinary('w2'), { kind: 'published', trigger: { tag: 'formation', error: { tag: 'numerical_range', name: 'b1' } }, force_scale_exponent: -2, report_diagnostic_ref: report }),
    ];
    for (const [name, edits] of [['evaluation', evaluation], ['formation', formation]] as const) {
      const { source, invocation } = await applyEntry(entry(P_BASE, NOT_REQUIRED, structuredClone(edits)));
      const result = await validateRetainedPrecision(source, invocation);
      expect([result.invocation_bound, result.numerical_eligible, result.standing], name).toEqual([true, true, 'eligible']);
      expect(() => ordinaryAttempts(source.retained_precision.body, source), name).not.toThrow();
    }
    const nr = corpus.must_pass.find((m: any) => m.id === NOT_REQUIRED);
    const keep = nr.edits.filter((e: any) => !(e.op === 'remove' && JSON.stringify(e.path) === JSON.stringify(rb('product_attempts', 1))));
    expect(keep.length).toBe(nr.edits.length - 1);
    const ownAttempt = entry(P_BASE, null, [...structuredClone(keep), set(rb('cases', 1, 'product_attempt_ref'), 1)]);
    await table([
      ['W2-published, verdict sensitive', entry(P_BASE, NOT_REQUIRED, [...structuredClone(evaluation), set(['numerical_quality', 'cases', 1, 'solve_quality'], 'sensitive')]), ATTEMPT],
      ['initial not_attempted', entry(P_BASE, NOT_REQUIRED, [set(ordinary('initial'), { kind: 'not_attempted', cause: 'ineligible' })]), ATTEMPT],
      ['a report whose outcome differs from the verdict (the kept equality)', entry(P_BASE, NOT_REQUIRED, [set(rb('ordinary_attempts', 1, 'initial', 'outcome'), 'sensitive')]), ATTEMPT],
      // It names case 0's attempt, which G3's ownership check refuses first (as in Rust; I90's note).
      ['product_attempt_ref non-null', entry(P_BASE, NOT_REQUIRED, [set(rb('cases', 1, 'product_attempt_ref'), 0)]), G('G3', 'COVERAGE_MISMATCH')],
      // With its own attempt kept, G3 passes and the rule itself refuses (G5's ordinary class, before D19's PRODUCT_ATTEMPT).
      ['product_attempt_ref naming its own attempt', ownAttempt, ATTEMPT],
    ]);
  });
  it('G8 P1 and the requested mode, for every case, report PREPARATION; mode code 3 is refused', async () => {
    const f = baseSource(F_BASE), p = (await applyEntry(entry(P_BASE, NOT_REQUIRED, []))).source;
    const fm = rowIndex(f, 'linear_solver_mode_basis', UNAVAILABLE_ROW), fs = rowIndex(f, 'linear_solver_mode_basis', SELECTED_ROW), pm = rowIndex(p, 'linear_solver_mode_basis', UNAVAILABLE_ROW);
    const duplicate = { ...structuredClone(f.results[fm]), id: 'result:b1:duplicate-mode' };
    await table([
      ['unavailable case: the dense code in sparse_interactive', entry(F_BASE, null, [set(['results', fm, 'value'], 2)]), PREP],
      ['unavailable case: mode code 3', entry(F_BASE, null, [set(['results', fm, 'value'], 3)]), PREP],
      ['unavailable case: two mode rows', entry(F_BASE, null, [withRows(f, [duplicate])]), PREP],
      ['not_required case: no mode row', entry(P_BASE, NOT_REQUIRED, [{ path: ['results', pm], op: 'remove' }]), PREP],
      ['unavailable case: the requested mode flipped', entry(F_BASE, null, [set(rb('ordinary_attempts', 1, 'requested_mode'), 'dense_scrutiny')]), PREP],
      ['not_required case: the dense code in sparse_interactive', entry(P_BASE, NOT_REQUIRED, [set(['results', pm, 'value'], 2)]), PREP],
      ['not_required case: mode code 3', entry(P_BASE, NOT_REQUIRED, [set(['results', pm, 'value'], 3)]), PREP],
      ['selected case: mode code 3', entry(F_BASE, null, [set(['results', fs, 'value'], 3)]), PREP],
      ['selected case: the requested mode flipped', entry('ordinary_prepared_synthetic', null, [set(rb('ordinary_attempts', 0, 'requested_mode'), 'dense_scrutiny')]), PREP],
      ['selected dense case: the sparse code', entry(DENSE, null, [set(['results', rowIndex(baseSource(DENSE), 'linear_solver_mode_basis', SELECTED_ROW), 'value'], 1)]), PREP],
    ]);
  });
  it('G8 P2-P4 for every case: at most one parity row, none in sparse_interactive, none on a W2-published case; a dense b = 0 case may lack one', async () => {
    const p = (await applyEntry(entry(P_BASE, NOT_REQUIRED, []))).source, report = p.retained_precision.body.ordinary_attempts[1].initial.report_diagnostic_ref;
    // 07j's two-case statement made dense: the invocation's mode, both requested modes and both mode rows (no parity row).
    const denseRows = structuredClone(p); for (const id of [SELECTED_ROW, UNAVAILABLE_ROW]) denseRows.results[rowIndex(p, 'linear_solver_mode_basis', id)].value = 2;
    const dense = [set(rb('ordinary_attempts', 0, 'requested_mode'), 'dense_scrutiny'), set(rb('ordinary_attempts', 1, 'requested_mode'), 'dense_scrutiny')];
    const toDense = [set(['solver_mode'], 'dense_scrutiny')];
    const w2 = [
      set(rb('ordinary_attempts', 1, 'initial'), { kind: 'structural_failure', error: { tag: 'range', detail: 'b1' }, diagnostic_ref: null }),
      set(rb('ordinary_attempts', 1, 'w2'), { kind: 'published', trigger: { tag: 'evaluation', error: { tag: 'range', detail: 'b1' } }, force_scale_exponent: 3, report_diagnostic_ref: report }),
    ];
    const w2Failed = [w2[0], set(rb('ordinary_attempts', 1, 'w2'), { kind: 'failed', trigger: { tag: 'evaluation', error: { tag: 'range', detail: 'b1' } }, failure: { tag: 'not_engaged' }, diagnostic_ref: report })];
    const onDense = (rows: any[], extra: any[] = []) => entry(P_BASE, NOT_REQUIRED, [...dense, withRows(denseRows, rows), ...extra], toDense);
    const one = () => parityRow(UNAVAILABLE_ROW, 'result:b1:parity-1'), two = () => parityRow(UNAVAILABLE_ROW, 'result:b1:parity-2');
    const d = baseSource(DENSE), twice = { ...structuredClone(d.results[1]), id: 'result:b1:parity-twice' };
    const denseW2 = [
      set(rb('ordinary_attempts', 0, 'initial'), { kind: 'structural_failure', error: { tag: 'range', detail: 'b1' }, diagnostic_ref: null }),
      set(rb('ordinary_attempts', 0, 'w2'), { kind: 'published', trigger: { tag: 'evaluation', error: { tag: 'range', detail: 'b1' } }, force_scale_exponent: 3, report_diagnostic_ref: d.retained_precision.body.ordinary_attempts[0].initial.report_diagnostic_ref }),
    ];
    await table([
      ['dense b = 0, no parity row on either case', onDense([]), { admitted: true }],
      ['dense b = 0, one parity row on the not_required case', onDense([one()]), { admitted: true }],
      ['dense, a W2-published not_required case without a parity row', onDense([], structuredClone(w2)), { admitted: true }],
      // P4 reads published (b != 0), not triggered: a W2 that was triggered and failed published nothing, so b = 0.
      ['dense b = 0, one parity row on a not_required case whose W2 failed', onDense([one()], structuredClone(w2Failed)), { admitted: true }],
      ['P2: two parity rows on the not_required case', onDense([one(), two()]), PREP],
      ['P4: a parity row on the W2-published not_required case', onDense([one()], structuredClone(w2)), PREP],
      ['P2: two parity rows on the dense selected case', entry(DENSE, null, [withRows(d, [twice])]), PREP],
      ['P4: a parity row on a W2-published selected case', entry(DENSE, null, denseW2), PREP],
      ['P3: a parity row on the sparse unavailable case', entry(F_BASE, null, [withRows(baseSource(F_BASE), [parityRow(UNAVAILABLE_ROW, 'result:b1:sparse-parity')])]), PREP],
      ['P3: a parity row on the sparse selected case', entry('ordinary_prepared_synthetic', null, [withRows(baseSource('ordinary_prepared_synthetic'), [parityRow(SELECTED_ROW, 'result:b1:sparse-selected-parity', true)])]), PREP],
      // I90's note: a parity row copied onto a non-selected case with its recovery method is G6 ROW_METHOD first.
      ['a non-selected case\'s parity row keeping recovery_method', onDense([parityRow(UNAVAILABLE_ROW, 'result:b1:parity-method', true)]), G('G6', 'ROW_METHOD_MISMATCH')],
    ]);
  });
  it('RV108 N4: transport reads no rows, so a non-object results entry is admitted on transport (as in Rust and Python); the full reader refuses it at G1', async () => {
    const bases: [string, any][] = [['ordinary_prepared_synthetic', baseSource('ordinary_prepared_synthetic')], ['milestone_sparse_interactive', JSON.parse(milestoneSparseText).source], ['milestone_dense_scrutiny', JSON.parse(milestoneDenseText).source]];
    const forms: [string, (rows: any[]) => void][] = [['results[0]=null', r => { r[0] = null; }], ['results+=null', r => { r.push(null); }], ['results[0]=1', r => { r[0] = 1; }], ['results[0]="row"', r => { r[0] = 'row'; }], ['results[0]=[]', r => { r[0] = []; }]];
    for (const [name, source] of bases) {
      const plain = await validateRetainedPrecisionTransport(structuredClone(source));
      expect([plain.numerical_eligible, plain.standing, plain.publication_sha256], name).toEqual([false, 'needs_recompute', source.retained_precision.body.publication_sha256]);
      for (const [form, edit] of forms) {
        const t = structuredClone(source); edit(t.results);
        expect(await validateRetainedPrecisionTransport(t), `${name} ${form}`).toEqual(plain);
        expect(await firstFailure(t), `${name} ${form}`).toEqual(G('G1', 'RECEIPT_MISMATCH'));
      }
      // RV108's other row forms were already admitted on transport; they stay so.
      for (const [form, value] of [['results="rows"', 'rows'], ['results=null', null], ['results=[]', []]] as const) {
        const t = structuredClone(source); t.results = structuredClone(value);
        expect(await validateRetainedPrecisionTransport(t), `${name} ${form}`).toEqual(plain);
      }
    }
  });
  it('RV108 N6: the G7 header codes named by baseHeaderCode\'s doc, for list- and dict-valued enums and the two documented differences', async () => {
    const at = async (edits: any[]) => { const { source, invocation } = await applyEntry(entry('ordinary_prepared_synthetic', null, edits)); return firstFailure(source, invocation); };
    // A list- or dict-valued enum: TS gives the header code Rust gives (and Python with SR-PY's RV108 N1 guard).
    for (const value of [['passive_model_basis'], { value: 'passive_model_basis' }]) {
      for (const field of ['structural_status', 'model_matrix_fidelity', 'accuracy_evidence']) expect(await at([set(['numerical_quality', 'cases', 0, field], value)]), field).toEqual({ gate: 'G7', code: 'SOURCE_NUMERICAL_CASE_INVALID' });
      expect(await at([set(['numerical_quality', 'status'], value)]), 'status').toEqual({ gate: 'G7', code: 'SOURCE_NUMERICAL_QUALITY_INVALID' });
    }
    // The declared carrier_evidence class (Rust's header does not read it): TS, like Python, keeps the producer code.
    expect(await at([set(['carrier_evidence'], {}), set(['numerical_quality', 'cases', 0, 'structural_status'], ['x'])])).toEqual({ gate: 'G7', code: 'SOURCE_PRODUCER_CONTRACT_UNSUPPORTED' });
    // Two header defects: TS, like Python, reports contract_evidence first (Rust reports source_block_recovery; inherited).
    expect(await at([set(['contract_evidence'], null), set(['source_block_recovery'], {})])).toEqual({ gate: 'G7', code: 'SOURCE_PREVIEW_PHYSICS_EVIDENCE_REQUIRED' });
  });
});

// B1 SR-TS repair 01 (I92): B1's three-reader alignment set, TS's side (RR "RV113's three returns verified; I3 made at
// `2ba2f81863`; the three-reader alignment set ruled", items 2 and 3; RV113 SR-TS S-1 and N-2). Reader-local: SC's 07n
// pins the shared entries. Each verdict is TS's first failure, or { admitted: numerical_eligible }.
describe('B1 SR-TS repair 01: (g) at G8, and the C2 cause table keyed by precondition', () => {
  const P_BASE = 'two_case_preparation_failure_synthetic', ORD = 'ordinary_prepared_synthetic';
  const INVOCATION = G('G8', 'INVOCATION_MISMATCH'), ATTEMPT = G('G5', 'ATTEMPT_MISMATCH');
  const rb = (...tail: (string | number)[]) => ['retained_precision', 'body', ...tail];
  const set = (path: (string | number)[], value: unknown) => ({ path, op: 'set', value });
  const remove = (path: (string | number)[]) => ({ path, op: 'remove' });
  const entry = (base: string, edits: any[], invocationEdits: any[] = []) => ({ id: 'b1_r1_probe', base, edits, invocation_edits: invocationEdits, rehash: 'all' });
  const outcome = async (run: () => Promise<any>): Promise<unknown> => {
    try { return { admitted: (await run()).numerical_eligible }; }
    catch (error) { expect(error).toBeInstanceOf(RetainedPrecisionError); return { gate: (error as any).gate, code: (error as any).code }; }
  };
  async function table(cases: [string, () => Promise<unknown>, unknown][]): Promise<void> {
    const misses: string[] = [];
    for (const [name, run, want] of cases) { const got = await run(); if (JSON.stringify(got) !== JSON.stringify(want)) misses.push(`${name}: got ${JSON.stringify(got)} want ${JSON.stringify(want)}`); }
    expect(misses).toEqual([]);
  }
  const bound = (e: any) => async () => { const { source, invocation } = await applyEntry(e); return outcome(() => validateRetainedPrecision(source, invocation)); };
  const unbound = (e: any) => async () => { const { source } = await applyEntry(e); return outcome(() => validateRetainedPrecision(source)); };

  it('(g): the model-scope members are PP\'s acceptance, at G8 INVOCATION before any PREPARATION check; unbound reads admit them', async () => {
    const model = (key: string) => ['request', 'model', key];
    const refused: [string, any[]][] = [
      ['reference_configurations null', [set(model('reference_configurations'), null)]],
      ['reference_configurations []', [set(model('reference_configurations'), [])]],
      ['reference_configurations {}', [set(model('reference_configurations'), {})]],
      ['pressure_contract false', [set(model('pressure_contract'), false)]],
      ['pressure_contract {}', [set(model('pressure_contract'), {})]],
      ['pressure_contract 0', [set(model('pressure_contract'), 0)]],
      ['combinations null', [set(model('combinations'), null)]],
      ['combinations {"x": 1}', [set(model('combinations'), { x: 1 })]],
      ['components null', [set(model('components'), null)]],
      ['components "x"', [set(model('components'), 'x')]],
      ['components {}', [set(model('components'), {})]],
      // Before any PREPARATION check: with an unnormalized coordinate too (07m's mm_unnormalized_coordinate is G8 PREPARATION alone).
      ['combinations null beside a PREPARATION defect', [set(model('combinations'), null), set(['request', 'model', 'nodes', 1, 'position', 'x'], 1)]],
    ];
    const admitted: [string, any[]][] = [
      ['pressure_contract null', [set(model('pressure_contract'), null)]],
      ['combinations and components []', [set(model('combinations'), []), set(model('components'), [])]],
      ['combinations and components absent', [remove(model('combinations')), remove(model('components'))]],
    ];
    await table([
      // A non-empty list (or any value with a non-zero length) is G8's too: TS's G3 combination conjunct moved here
      // (ROOT on REPAIR_01 §6), as in RS and PY.
      ['bound: combinations [{}]', bound(entry(ORD, [], [set(model('combinations'), [{}])])), INVOCATION],
      ['bound: combinations "x"', bound(entry(ORD, [], [set(model('combinations'), 'x')])), INVOCATION],
      ['unbound: combinations [{}]', unbound(entry(ORD, [], [set(model('combinations'), [{}])])), { admitted: false }],
      ['unbound: combinations "x"', unbound(entry(ORD, [], [set(model('combinations'), 'x')])), { admitted: false }],
      ...refused.map(([name, edits]) => [`bound: ${name}`, bound(entry(ORD, [], edits)), INVOCATION] as [string, () => Promise<unknown>, unknown]),
      ...refused.map(([name, edits]) => [`unbound: ${name}`, unbound(entry(ORD, [], edits)), { admitted: false }] as [string, () => Promise<unknown>, unknown]),
      ...admitted.map(([name, edits]) => [`bound: ${name}`, bound(entry(ORD, [], edits)), { admitted: true }] as [string, () => Promise<unknown>, unknown]),
    ]);
  });

  it('C2\'s cause table: an unavailable_precondition code is keyed one-to-one by its precondition (N-2); the receipt_failure set form stands', async () => {
    const KEYED: Record<string, string> = { caller: 'caller_not_qualified', resource_admission: 'resource_admission_not_available',
      upstream_no_wrap: 'upstream_no_wrap_not_established', capture: 'source_unavailable', source_family: 'source_unavailable' };
    const CODES = ['caller_not_qualified', 'resource_admission_not_available', 'upstream_no_wrap_not_established', 'source_unavailable'];
    const base = corpus.cases.find((c: any) => c.id === P_BASE).source, cid = base.retained_precision.body.cases[1].basis_ref.ref_id;
    // Reader logic: G5's ordinary class on case 1 (unavailable) with each precondition, code and phase.
    const ordinary = (reason: any, run: any = null) => {
      const s = structuredClone(base); Object.assign(s.retained_precision.body.cases[1], { reason, run });
      try { ordinaryAttempts(s.retained_precision.body, s); return 'pass'; } catch (e) { return { gate: (e as any).gate, code: (e as any).code }; }
    };
    const misses: string[] = [];
    for (const [precondition, keyed] of Object.entries(KEYED)) for (const phase of ['routing', 'preparation', 'kernel']) for (const code of CODES) {
      const cause = { kind: 'unavailable_precondition', precondition, affected_refs: [cid] };
      const want = phase !== 'kernel' && code === keyed ? 'pass' : ATTEMPT, got = ordinary({ code, phase, cause });
      if (JSON.stringify(got) !== JSON.stringify(want)) misses.push(`${precondition}/${phase}/${code}: got ${JSON.stringify(got)}`);
    }
    // A precondition cannot sit beside a Run.
    const run = base.retained_precision.body.cases[0].run;
    if (JSON.stringify(ordinary({ code: 'caller_not_qualified', phase: 'routing', cause: { kind: 'unavailable_precondition', precondition: 'caller', affected_refs: [cid] } }, run)) !== JSON.stringify(ATTEMPT)) misses.push('beside a Run');
    expect(misses).toEqual([]);
    // The full reader: case 1 without its product attempt, unavailable through a precondition, is admitted when keyed.
    const precondition = (p: string, code: string, phase = 'preparation') => entry(P_BASE, [
      set(rb('cases', 1, 'product_attempt_ref'), null),
      set(rb('cases', 1, 'reason'), { code, phase, cause: { kind: 'unavailable_precondition', precondition: p, affected_refs: [cid] } }),
      remove(rb('product_attempts', 1)),
    ]);
    await table([
      ...Object.entries(KEYED).map(([p, code]) => [`${p} → ${code}`, bound(precondition(p, code)), { admitted: false }] as [string, () => Promise<unknown>, unknown]),
      ['capture → caller_not_qualified', bound(precondition('capture', 'caller_not_qualified')), ATTEMPT],
      ['caller → source_unavailable', bound(precondition('caller', 'source_unavailable')), ATTEMPT],
      ['source_family → upstream_no_wrap_not_established (routing)', bound(precondition('source_family', 'upstream_no_wrap_not_established', 'routing')), ATTEMPT],
      ['resource_admission → its code in phase receipt', bound(precondition('resource_admission', 'resource_admission_not_available', 'receipt')), ATTEMPT],
    ]);
  });
});

// B1 SR-TS repair 01, item 3 (RR "RV113's three returns verified; …", alignment set item 4; RV113 SR-TS N-1): the
// transport reading checks the projected base's header at G2 with Rust's code, then its preview-physics metadata at G7.
describe('B1 SR-TS repair 01: the transport reading\'s base header at G2 (Rust\'s code), then its metadata at G7', () => {
  const ORD = 'ordinary_prepared_synthetic', S = (gate: string, code: string) => ({ gate, code });
  const set = (path: (string | number)[], value: unknown) => ({ path, op: 'set', value });
  const entry = (base: string, edits: any[]) => ({ id: 'b1_r1_transport_probe', base, edits, invocation_edits: [], rehash: 'all' });
  const outcome = async (run: () => Promise<any>): Promise<unknown> => {
    try { return { admitted: (await run()).numerical_eligible }; }
    catch (error) { expect(error).toBeInstanceOf(RetainedPrecisionError); return { gate: (error as any).gate, code: (error as any).code }; }
  };
  async function table(cases: [string, () => Promise<unknown>, unknown][]): Promise<void> {
    const misses: string[] = [];
    for (const [name, run, want] of cases) { const got = await run(); if (JSON.stringify(got) !== JSON.stringify(want)) misses.push(`${name}: got ${JSON.stringify(got)} want ${JSON.stringify(want)}`); }
    expect(misses).toEqual([]);
  }
  const bound = (e: any) => async () => { const { source, invocation } = await applyEntry(e); return outcome(() => validateRetainedPrecision(source, invocation)); };
  const transport = (e: any) => async () => { const { source } = await applyEntry(e); return outcome(() => validateRetainedPrecisionTransport(source)); };
  it('the header at G2 with Rust\'s code, then the preview-physics metadata at G7; the raw path is unchanged', async () => {
    const quality = (...tail: (string | number)[]) => ['numerical_quality', ...tail];
    // [name, edits, transport (Rust's G2 code or G7 metadata), raw (TS's own G7 code, unchanged)]
    const rows: [string, any[], unknown, unknown][] = [
      ['quality status invalid', [set(quality('status'), 'bogus')], S('G2', 'SOURCE_NUMERICAL_QUALITY_INVALID'), S('G7', 'SOURCE_NUMERICAL_QUALITY_INVALID')],
      ['quality status a list', [set(quality('status'), ['checks_passed'])], S('G2', 'SOURCE_NUMERICAL_QUALITY_INVALID'), S('G7', 'SOURCE_NUMERICAL_QUALITY_INVALID')],
      ['a case enum invalid', [set(quality('cases', 0, 'structural_status'), 'bogus')], S('G2', 'SOURCE_NUMERICAL_CASE_INVALID'), S('G7', 'SOURCE_NUMERICAL_CASE_INVALID')],
      ['a case enum a dict', [set(quality('cases', 0, 'accuracy_evidence'), { value: 'not_claimed' })], S('G2', 'SOURCE_NUMERICAL_CASE_INVALID'), S('G7', 'SOURCE_NUMERICAL_CASE_INVALID')],
      ['a case member added', [set(quality('cases', 0, 'extra'), 1)], S('G2', 'SOURCE_NUMERICAL_CASE_INVALID'), S('G7', 'SOURCE_NUMERICAL_CASE_INVALID')],
      ['an empty evidence ref', [set(quality('cases', 0, 'evidence_refs'), [''])], S('G2', 'SOURCE_NUMERICAL_CASE_INVALID'), S('G7', 'SOURCE_NUMERICAL_CASE_INVALID')],
      ['limitations empty', [set(['formulation_basis', 'limitations'], [])], S('G2', 'SOURCE_FORMULATION_BASIS_UNSUPPORTED'), S('G7', 'SOURCE_FORMULATION_BASIS_UNSUPPORTED')],
      ['contract_evidence null', [set(['contract_evidence'], null)], S('G2', 'SOURCE_PREVIEW_PHYSICS_EVIDENCE_REQUIRED'), S('G7', 'SOURCE_PREVIEW_PHYSICS_EVIDENCE_REQUIRED')],
      ['source_block_recovery present', [set(['source_block_recovery'], {})], S('G2', 'SOURCE_BLOCKS_LEGACY_DOWNGRADE_FORBIDDEN'), S('G7', 'SOURCE_BLOCKS_LEGACY_DOWNGRADE_FORBIDDEN')],
      ['an extra producer member', [set(['producer', 'extra'], 'x')], S('G2', 'SOURCE_PRODUCER_CONTRACT_UNSUPPORTED'), S('G7', 'SOURCE_PRODUCER_CONTRACT_UNSUPPORTED')],
      // Rust's order with two header defects; TS's raw G7 keeps Python's (RV108 N6's declared difference).
      ['contract_evidence null and source_block_recovery', [set(['contract_evidence'], null), set(['source_block_recovery'], {})], S('G2', 'SOURCE_BLOCKS_LEGACY_DOWNGRADE_FORBIDDEN'), S('G7', 'SOURCE_PREVIEW_PHYSICS_EVIDENCE_REQUIRED')],
      // Rust's header does not read carrier_evidence: a case defect beside it is the header's code; alone, G7's metadata check refuses it.
      ['carrier_evidence and a case defect', [set(['carrier_evidence'], {}), set(quality('cases', 0, 'structural_status'), ['x'])], S('G2', 'SOURCE_NUMERICAL_CASE_INVALID'), S('G7', 'SOURCE_PRODUCER_CONTRACT_UNSUPPORTED')],
      ['carrier_evidence alone', [set(['carrier_evidence'], {})], S('G7', 'SOURCE_PREVIEW_PHYSICS_EVIDENCE_INVALID'), S('G7', 'SOURCE_PRODUCER_CONTRACT_UNSUPPORTED')],
      // A header-valid but non-table limitation list passes G2 and is refused by the metadata check.
      ['limitations another non-empty list', [set(['formulation_basis', 'limitations'], ['other'])], S('G7', 'SOURCE_PREVIEW_PHYSICS_EVIDENCE_INVALID'), S('G7', 'SOURCE_PREVIEW_PHYSICS_EVIDENCE_INVALID')],
      ['the base', [], { admitted: false }, { admitted: true }],
    ];
    await table(rows.flatMap(([name, edits, t, raw]) => [[`transport: ${name}`, transport(entry(ORD, edits)), t], [`raw: ${name}`, bound(entry(ORD, edits)), raw]] as [string, () => Promise<unknown>, unknown][]));
    // 07m's G7 header mutations (277 and 286-293) read on transport at G2 with the Rust reader's own expected code.
    const header = corpus.mutations.filter((m: any) => (m.expected_by_reader?.rust ?? m.expected).gate === 'G7' && /^SOURCE_(NUMERICAL|FORMULATION_BASIS|BLOCKS_LEGACY|PREVIEW_PHYSICS_EVIDENCE_REQUIRED)/.test((m.expected_by_reader?.rust ?? m.expected).code));
    expect(header.map((m: any) => corpus.mutations.indexOf(m))).toEqual([277, 286, 287, 288, 289, 290, 291, 292, 293]);
    await table(header.map((m: any) => [m.id, transport(m), { gate: 'G2', code: (m.expected_by_reader?.rust ?? m.expected).code }] as [string, () => Promise<unknown>, unknown]));
  });
});

// B1's reader follow-up toward I4' (I101; RR "I4 made at `30f3d1b24a`; …", rulings 2 and 3), through the retained
// reader: PY's extrema-number demand refuses a non-number global_upper_bound_pa or certified_gap_pa at G7 with TS's
// base code, bound, unbound and on transport (RV113's t_extrema_global_upper_string and t_extrema_certified_gap_null).
describe('B1 I4\' (I101): the extrema-number demand at G7, bound, unbound and on transport', () => {
  const ORD = 'ordinary_prepared_synthetic';
  const x = (member: string) => ['contract_evidence', 'preview_cases', 0, 'pipe_stress_extrema', 0, member];
  const entry = (edits: any[]) => ({ id: 'b1_i4p_extrema_probe', base: ORD, edits, invocation_edits: [], rehash: 'all' });
  const outcome = async (run: () => Promise<any>): Promise<unknown> => {
    try { return { admitted: (await run()).numerical_eligible }; }
    catch (error) { expect(error).toBeInstanceOf(RetainedPrecisionError); const e = error as RetainedPrecisionError; return { gate: e.gate, code: e.code, detail: e.detail }; }
  };
  const verdicts = async (edits: any[]) => {
    const { source, invocation } = await applyEntry(entry(edits));
    return { bound: await outcome(() => validateRetainedPrecision(structuredClone(source), structuredClone(invocation))),
      unbound: await outcome(() => validateRetainedPrecision(structuredClone(source))),
      transport: await outcome(() => validateRetainedPrecisionTransport(structuredClone(source))) };
  };
  const g7 = (detail: string) => ({ gate: 'G7', code: 'SOURCE_PREVIEW_PHYSICS_EVIDENCE_INVALID', detail });
  it('a non-number member is refused at G7 "extrema numbers" by every entry point; JSON integers are admitted', async () => {
    const rows: [string, any[], unknown][] = [
      ['t_extrema_global_upper_string', [{ path: x('global_upper_bound_pa'), op: 'set', value: 'x' }], { bound: g7('extrema numbers'), unbound: g7('extrema numbers'), transport: g7('extrema numbers') }],
      ['t_extrema_certified_gap_null', [{ path: x('certified_gap_pa'), op: 'set', value: null }], { bound: g7('extrema numbers'), unbound: g7('extrema numbers'), transport: g7('extrema numbers') }],
      ['global_upper_bound_pa null', [{ path: x('global_upper_bound_pa'), op: 'set', value: null }], { bound: g7('extrema numbers'), unbound: g7('extrema numbers'), transport: g7('extrema numbers') }],
      ['certified_gap_pa a string', [{ path: x('certified_gap_pa'), op: 'set', value: '0' }], { bound: g7('extrema numbers'), unbound: g7('extrema numbers'), transport: g7('extrema numbers') }],
      ['both members JSON integers', [{ path: x('global_upper_bound_pa'), op: 'set', value: 41354909 }, { path: x('certified_gap_pa'), op: 'set', value: 0 }], { bound: { admitted: true }, unbound: { admitted: false }, transport: { admitted: false } }],
      ['the base', [], { bound: { admitted: true }, unbound: { admitted: false }, transport: { admitted: false } }],
    ];
    const misses: string[] = [];
    for (const [name, edits, want] of rows) { const got = await verdicts(edits); if (JSON.stringify(got) !== JSON.stringify(want)) misses.push(`${name}: got ${JSON.stringify(got)} want ${JSON.stringify(want)}`); }
    expect(misses).toEqual([]);
  });
});

describe('B3a (I101): G8\'s namespace predicate, type-strict (B3-D §6.3, §6.4; REVISION_01 §3, N-4; reader-local synthetic receipts)', () => {
  // Branch L (0.1.0 or 0.2.0, pressure_contract absent or JSON null) and branch L3 (0.3.0, exactly
  // {"version":"1.0.0","mode":"legacy_pressure_v1"}) are admitted; any other value is G8 INVOCATION_MISMATCH. A
  // zero-magnitude element pressure load on L3 is outside D1.7 (N-11's reading): G8 PREPARATION_MISMATCH. The shapes and
  // expectations are the Rust reader's `b3a_legacy_pressure_contract_namespace_at_g8`, entry for entry.
  const l3 = { version: '1.0.0', mode: 'legacy_pressure_v1' };
  const schema = (v: string) => ({ path: ['request', 'model', 'schema_version'], op: 'set', value: v });
  const contract = (v: unknown) => ({ path: ['request', 'model', 'pressure_contract'], op: 'set', value: v });
  const noContract = () => ({ path: ['request', 'model', 'pressure_contract'], op: 'remove' });
  const INV = G('G8', 'INVOCATION_MISMATCH'), PREP = G('G8', 'PREPARATION_MISMATCH');
  const zeroPressure = (id: string) => {
    const loads = structuredClone(corpus.cases.find((c: any) => c.id === id).invocation.request.model.load_cases[0].primitive_loads);
    loads.push({ id: 'load:b3a-zero-pressure', category: 'pressure', target: { type: 'element', pipe: 'pipe:fixture-span' },
      magnitude: { value: 0, unit: 'Pa' }, dimension: 'pressure', provenance: 'synthetic_integration_input_not_library_data' });
    return { path: ['request', 'model', 'load_cases', 0, 'primitive_loads'], op: 'set', value: loads };
  };
  const ORD = 'ordinary_prepared_synthetic', DENSE = 'ordinary_prepared_dense_synthetic';
  const entries: [string, string, any[], any][] = [
    ...[ORD, DENSE, 'two_case_synthetic', 'u8_l0_isolated_node_sparse_interactive', 'u8_l0_isolated_node_dense_scrutiny']
      .map((base): [string, string, any[], any] => [`L3 on ${base}`, base, [schema('0.3.0'), contract(l3)], 'pass']),
    ['L: 0.2.0, contract null', ORD, [contract(null)], 'pass'],
    ['L: 0.1.0, contract absent', ORD, [schema('0.1.0'), noContract()], 'pass'],
    ['L3: mode exact_straight_pressure_v2, version 1.0.0', ORD, [schema('0.3.0'), contract({ ...l3, mode: 'exact_straight_pressure_v2' })], INV],
    ['L3: version 1.0.1', ORD, [schema('0.3.0'), contract({ ...l3, version: '1.0.1' })], INV],
    ['L3: the exact contract 2.0.0', ORD, [schema('0.3.0'), contract({ version: '2.0.0', mode: 'exact_straight_pressure_v2' })], INV],
    ['0.3.0, contract null', ORD, [schema('0.3.0'), contract(null)], INV],
    ['0.3.0, contract absent', ORD, [schema('0.3.0'), noContract()], INV],
    ['L3: an extra key', ORD, [schema('0.3.0'), contract({ ...l3, extra: 'x' })], INV],
    ['L3: an extra key valued null', ORD, [schema('0.3.0'), contract({ ...l3, extra: null })], INV],
    ['L3: version only', ORD, [schema('0.3.0'), contract({ version: '1.0.0' })], INV],
    ['L3: mode only', ORD, [schema('0.3.0'), contract({ mode: 'legacy_pressure_v1' })], INV],
    ['L3: version a number', ORD, [schema('0.3.0'), contract({ ...l3, version: 1.0 })], INV],
    ['L3: mode null', ORD, [schema('0.3.0'), contract({ ...l3, mode: null })], INV],
    ['0.3.0, contract {}', ORD, [schema('0.3.0'), contract({})], INV],
    ['0.2.0 keeping the L3 contract', ORD, [contract(l3)], INV],
    ['0.1.0 keeping the L3 contract', ORD, [schema('0.1.0'), contract(l3)], INV],
    ['0.4.0 with the L3 contract', ORD, [schema('0.4.0'), contract(l3)], INV],
    ['N-4: 0.2.0, contract {}', ORD, [contract({})], INV],
    ['N-4: 0.2.0, contract false', ORD, [contract(false)], INV],
    ['N-4: 0.2.0, contract []', ORD, [contract([])], INV],
    ['N-4: 0.2.0, contract ""', ORD, [contract('')], INV],
    ['N-4: 0.2.0, contract 0', ORD, [contract(0)], INV],
    ['N-11: L3 with a zero-magnitude element pressure load', ORD, [schema('0.3.0'), contract(l3), zeroPressure(ORD)], PREP],
    ['N-11: L3 dense with a zero-magnitude element pressure load', DENSE, [schema('0.3.0'), contract(l3), zeroPressure(DENSE)], PREP],
  ];
  it('has the Rust test\'s 29 entries', () => { expect(entries.length).toBe(29); });
  for (const [name, base, edits, want] of entries) it(name, async () => {
    const { source, invocation } = await applyEntry({ id: 'i101_b3a', base, edits: [], invocation_edits: edits, rehash: 'all' });
    expect(await firstFailure(source, invocation)).toEqual(want);
  });
});

describe('B3b (I101): the exact successor <physics-retained> on reader-local synthetic receipts and lane P\'s m3x successors (B3-D §6; REVISION_01 §4.3)', () => {
  // RN64(2e11 / (2 RN64(1 + nu))) is exactly the bases' G = 7.7e10 Pa, so the synthetic exact successor keeps every receipt
  // number of its preview base. The transform, the shapes and the expected first failures are the Rust reader's
  // `exact_successor` and `b3b_shapes`, entry for entry; only G7's base codes are TS's own (C1 G7 settlement).
  const NU = 0.2987012987012987;
  const ORD = 'ordinary_prepared_synthetic', BASES = [ORD, 'ordinary_prepared_dense_synthetic', 'two_case_synthetic'];
  const decode = (bits: string) => decodeBinary64(bits);
  const ulps = (v: number, n: number) => { const view = new DataView(new ArrayBuffer(8)); view.setFloat64(0, v); view.setBigUint64(0, view.getBigUint64(0) + BigInt(n)); return view.getFloat64(0); };
  async function exactSuccessor(baseId: string): Promise<{ source: any; invocation: any }> {
    const base = corpus.cases.find((c: any) => c.id === baseId), source = structuredClone(base.source), invocation = structuredClone(base.invocation);
    const model = invocation.request.model;
    model.schema_version = '0.3.0'; model.pressure_contract = { version: '2.0.0', mode: 'exact_straight_pressure_v2' };
    for (const c of model.load_cases) c.pressure_regions = [];
    for (const m of model.materials) { m.poisson_ratio = { value: NU, unit: '1' }; m.constitutive_basis = 'homogeneous_isotropic_E_nu_v1'; }
    const body = source.retained_precision.body, nodes = model.nodes.map((n: any) => n.id), exactCases: any[] = [];
    for (const rc of body.cases) {
      expect(rc.status).toBe('selected');
      const id = rc.basis_ref.ref_id, s = body.sources[rc.source_ref], mb = body.material_bases[body.product_attempts[rc.product_attempt_ref].material_basis_ref];
      const sections: any[] = [], materials: any[] = [];
      model.pipe_segments.forEach((p: any, i: number) => {
        const st = s.section_terms[i], g = st.geometry; expect(s.id_maps.members[i].id).toBe(p.id);
        const od = decode(g.normalized_od), wall = decode(g.effective_wall), ri = od * 0.5 - wall;
        sections.push({ pipe_id: p.id, geometry_basis: 'authored_normalized_od_wall_v1', outside_diameter_m: od, effective_wall_thickness_m: wall, ri_m: ri,
          ro_m: decode(g.actual_radius), Ai_m2: Math.PI * ri * ri, As_m2: decode(st.area), I_m4: decode(g.actual_second_moment), J_m4: decode(g.actual_polar_moment), Z_m3: decode(st.section_modulus) });
        const m = mb.materials.find((x: any) => x.id === p.material), e = decode(m.elastic_modulus), gHat = e / (2 * (1 + NU));
        expect(binary64Bits(gHat)).toBe(m.shear_modulus);
        const authored = model.materials.find((x: any) => x.id === p.material);
        materials.push({ pipe_id: p.id, material_id: p.material, E_pa: e, nu: NU, G_pa: gHat, constitutive_basis: 'homogeneous_isotropic_E_nu_v1', thermal_consumed: false, alpha_per_kelvin: null, provenance: authored.provenance });
      });
      const preview = source.contract_evidence.preview_cases.find((c: any) => c.load_case_id === id), zeros = Array(nodes.length * 6).fill(0);
      exactCases.push({ load_case_id: id, profile_mode: 'exact_straight_pressure_v2', material_basis: 'base_material_common_E_nu', pipe_materials: materials, pipe_sections: sections,
        pipe_stress_extrema: structuredClone(preview.pipe_stress_extrema),
        stress_maximum_coverage: { complete: preview.stress_maximum_coverage.complete, unavailable_pipe_ids: structuredClone(preview.stress_maximum_coverage.unavailable_pipe_ids) },
        pressure_rhs_assembly: { method: 'source_factor_grouped_pressure_rhs_v1', load_case_id: id, node_order: nodes, dof_order: ['Fx', 'Fy', 'Fz', 'Mx', 'My', 'Mz'],
          dof_units: ['N', 'N', 'N', 'N*m', 'N*m', 'N*m'], assembled_pressure_rhs_global: zeros, groups: [], rounded_cap_rhs_global: zeros, rounded_poisson_rhs_global: zeros,
          rounded_cap_and_eigen_ledgers_are_observational: true, cancellation_screen: 0, screen_limit: 1e-9, screen_roundoff_multiplier: 32, screen_is_not_numerical_qualification: true } });
    }
    source.producer.semantic_contract_id = EXACT_RETAINED_ID;
    source.formulation_basis = { profile_id: EXACT_RETAINED_PROFILE, limitations: JSON.parse(n05Text).formulation_basis.limitations };
    source.contract_evidence = structuredClone({ pressure: [], connector: [], exact_cases: exactCases });
    for (const mb of body.material_bases) for (const m of mb.materials) m.shear_origin = { kind: 'derived_e_nu', poisson_ratio: binary64Bits(NU), constitutive_basis: 'homogeneous_isotropic_E_nu_v1' };
    for (const s of body.sources) for (const st of s.section_terms) st.geometry.route = 'exact';
    for (const a of body.product_attempts) a.definition_id = EXACT_DEFINITION_ID;
    body.invocation.value = await canonicalSha256HexCheckedV1({ domain: 'source_blocks_invocation_v1', payload: invocation });
    await rehash(source);
    return { source, invocation };
  }
  type Shape = { name: string; base: string; edits?: any[]; invocation_edits?: any[] };
  const set = (path: (string | number)[], value: unknown) => ({ path, op: 'set', value }), remove = (path: (string | number)[]) => ({ path, op: 'remove' });
  const rb = (...tail: (string | number)[]) => ['retained_precision', 'body', ...tail], ce = (...tail: (string | number)[]) => ['contract_evidence', 'exact_cases', 0, ...tail];
  const md = (...tail: (string | number)[]) => ['request', 'model', ...tail];
  const G0 = { gate: 'G0', code: 'SOURCE_PRODUCER_CONTRACT_UNSUPPORTED' }, INV = G('G8', 'INVOCATION_MISMATCH'), PREP = G('G8', 'PREPARATION_MISMATCH'), SEC = G('G5b', 'SECTION_MISMATCH');
  /** REVISION_01 §4.3's entries and the added shapes on one exact base, every edited value taken from that base. */
  const rows = (base: string, x: { source: any; invocation: any }): [Shape, any][] => {
    const entry = x.source.contract_evidence.exact_cases[0], sec = (k: string) => entry.pipe_sections[0][k] as number;
    const pm = entry.pipe_materials[0], receiptG = decode(x.source.retained_precision.body.material_bases[0].materials[0].shear_modulus);
    const receiptE = decode(x.source.retained_precision.body.material_bases[0].materials[0].elastic_modulus);
    const model = x.invocation.request.model, authoredNu = model.materials[0].poisson_ratio.value, caseId = model.load_cases[0].id, pipeId = model.pipe_segments[0].id;
    return [
      [{ name: '01 identity -> the preview id', base, edits: [set(['producer', 'semantic_contract_id'], RETAINED_PRECISION_ID)] }, G0],
      [{ name: '02 profile -> the preview profile', base, edits: [set(['formulation_basis', 'profile_id'], RETAINED_PRECISION_PROFILE)] }, G0],
      [{ name: '02b producer component_version 0.2.1', base, edits: [set(['producer', 'component_version'], '0.2.1')] }, G0],
      [{ name: "03 an attempt's definition_id -> the ordinary id", base, edits: [set(rb('product_attempts', 0, 'definition_id'), PREPARED_DEFINITION_ID)] }, G0],
      [{ name: '04 projection_policy changed', base, edits: [set(rb('projection_policy'), 'RP-LOGICAL-ATTEMPTS-v2')] }, G0],
      [{ name: '04b policy changed', base, edits: [set(rb('policy'), 'M03-INTEGRITY-MP-v3')] }, G0],
      [{ name: '04c facade_policy changed', base, edits: [set(rb('facade_policy'), 'RP-FACADE-SI-v3')] }, G0],
      [{ name: '05 work_policy changed', base, edits: [set(rb('work_policy'), 'W1-LME-20B-60B-v2')] }, G0],
      [{ name: '06 canonicalization changed', base, edits: [set(rb('canonicalization'), 'openpipestress_jcs_ijson_v2')] }, G0],
      [{ name: '07 work.case_limit changed', base, edits: [set(rb('work', 'case_limit'), 19_999_999_999)] }, G0],
      [{ name: '08 work.invocation_limit changed', base, edits: [set(rb('work', 'invocation_limit'), 60_000_000_001)] }, G0],
      [{ name: '08b receipt_version 2', base, edits: [set(rb('receipt_version'), 2)] }, G0],
      [{ name: '10 the exact successor relabelled preview (identity and profile)', base, edits: [set(['producer', 'semantic_contract_id'], RETAINED_PRECISION_ID), set(['formulation_basis', 'profile_id'], RETAINED_PRECISION_PROFILE)] }, G0],
      [{ name: "12 owner entry's As_m2 one ulp", base, edits: [set(ce('pipe_sections', 0, 'As_m2'), ulps(sec('As_m2'), 1))] }, SEC],
      [{ name: "13 owner entry's Z_m3 one ulp", base, edits: [set(ce('pipe_sections', 0, 'Z_m3'), ulps(sec('Z_m3'), 1))] }, SEC],
      [{ name: "14 owner entry's I_m4 one ulp", base, edits: [set(ce('pipe_sections', 0, 'I_m4'), ulps(sec('I_m4'), 1))] }, SEC],
      [{ name: "15 owner entry's ro_m one ulp", base, edits: [set(ce('pipe_sections', 0, 'ro_m'), ulps(sec('ro_m'), 1))] }, SEC],
      [{ name: "15b owner entry's J_m4 one ulp", base, edits: [set(ce('pipe_sections', 0, 'J_m4'), ulps(sec('J_m4'), 1))] }, SEC],
      [{ name: "15c owner entry's outside_diameter_m one ulp", base, edits: [set(ce('pipe_sections', 0, 'outside_diameter_m'), ulps(sec('outside_diameter_m'), 1))] }, SEC],
      [{ name: "15d owner entry's effective_wall_thickness_m one ulp", base, edits: [set(ce('pipe_sections', 0, 'effective_wall_thickness_m'), ulps(sec('effective_wall_thickness_m'), 1))] }, SEC],
      [{ name: "15e the owner entry's load_case_id renamed (no entry for the selected case)", base, edits: [set(ce('load_case_id'), 'case:other')] }, SEC],
      [{ name: "15f the owner entry's pipe section listed twice", base, edits: [set(ce('pipe_sections'), [entry.pipe_sections[0], entry.pipe_sections[0]])] }, SEC],
      [{ name: "15g the owner entry's As_m2 a string", base, edits: [set(ce('pipe_sections', 0, 'As_m2'), String(sec('As_m2')))] }, SEC],
      [{ name: "15h the owner entry listed twice", base, edits: [set(['contract_evidence', 'exact_cases'], [entry, entry])] }, SEC],
      [{ name: '16b contract_evidence removed', base, edits: [remove(['contract_evidence'])] }, SEC],
      [{ name: '16 connector non-empty', base, edits: [set(['contract_evidence', 'connector'], [{ id: 'connector:x' }])] }, { gate: 'G7', code: 'PHYSICS_EVIDENCE_UNSUPPORTED_COMPOSITION' }],
      [{ name: "17 an entry's G_pa three ulps", base, edits: [set(ce('pipe_materials', 0, 'G_pa'), ulps(pm.G_pa, 3))] }, { gate: 'G7', code: 'PHYSICS_EVIDENCE_MATERIAL_INVALID' }],
      [{ name: '18 recovery_method added to an exact_cases entry', base, edits: [set(ce('recovery_method'), 'retained_source_blocks_exact_v1')] }, { gate: 'G7', code: 'PHYSICS_EVIDENCE_CASE_SHAPE' }],
      [{ name: '19 invocation contract -> legacy 1.0.0', base, invocation_edits: [set(md('pressure_contract'), { version: '1.0.0', mode: 'legacy_pressure_v1' })] }, INV],
      [{ name: '20 invocation schema -> 0.4.0', base, invocation_edits: [set(md('schema_version'), '0.4.0')] }, INV],
      [{ name: '21 invocation pressure_contract false', base, invocation_edits: [set(md('pressure_contract'), false)] }, INV],
      [{ name: '21b invocation contract with an extra key', base, invocation_edits: [set(md('pressure_contract'), { version: '2.0.0', mode: 'exact_straight_pressure_v2', extra: null })] }, INV],
      [{ name: '21c invocation contract version 2.0.1', base, invocation_edits: [set(md('pressure_contract'), { version: '2.0.1', mode: 'exact_straight_pressure_v2' })] }, INV],
      [{ name: '21d invocation contract removed (0.3.0, absent)', base, invocation_edits: [remove(md('pressure_contract'))] }, INV],
      [{ name: '22 a combination added to the invocation', base, invocation_edits: [set(md('combinations'), [{ id: 'combination:x', kind: 'algebraic', terms: [{ load_case: caseId, factor: 1.0 }] }])] }, INV],
      [{ name: '22b a component added to the invocation', base, invocation_edits: [set(md('components'), [{ id: 'component:x' }])] }, INV],
      [{ name: '23 a case naming modulus_basis_ref', base, invocation_edits: [set(md('load_cases', 0, 'modulus_basis_ref'), 'point:x')] }, PREP],
      // D1.5: only the base common E/nu, even a named point equal to the base, with the receipt's selector naming it alike.
      [{ name: "23b a case naming a point equal to the base, the receipt's selector alike", base, edits: [set(rb('material_bases', 0, 'selector'), { kind: 'named', id: 'point:base' })],
        invocation_edits: [set(md('materials', 0, 'temperature_points'), [{ id: 'point:base', elastic_modulus: structuredClone(model.materials[0].elastic_modulus), poisson_ratio: structuredClone(model.materials[0].poisson_ratio) }]),
          set(md('load_cases', 0, 'modulus_basis_ref'), 'point:base')] }, PREP],
      [{ name: "24 a material's shear_origin -> explicit_g", base, edits: [set(rb('material_bases', 0, 'materials', 0, 'shear_origin'), { kind: 'explicit_g' })] }, PREP],
      [{ name: "24b a material's selection -> named_point", base, edits: [set(rb('material_bases', 0, 'materials', 0, 'selection'), { kind: 'named_point', point_id: 'point:x' })] }, PREP],
      [{ name: "25b a material's elastic_modulus one ulp", base, edits: [set(rb('material_bases', 0, 'materials', 0, 'elastic_modulus'), binary64Bits(ulps(receiptE, 1)))] }, PREP],
      [{ name: "25 a material's shear_modulus one ulp", base, edits: [set(rb('material_bases', 0, 'materials', 0, 'shear_modulus'), binary64Bits(ulps(receiptG, 1)))] }, PREP],
      [{ name: '26 shear_origin.poisson_ratio bits changed', base, edits: [set(rb('material_bases', 0, 'materials', 0, 'shear_origin', 'poisson_ratio'), binary64Bits(ulps(pm.nu, 1)))] }, PREP],
      [{ name: '27 authored nu changed in the invocation', base, invocation_edits: [set(md('materials', 0, 'poisson_ratio', 'value'), ulps(authoredNu, 1))] }, PREP],
      [{ name: "28 S-C only: an entry's pipe_materials nu one ulp", base, edits: [set(ce('pipe_materials', 0, 'nu'), ulps(pm.nu, 1))] }, PREP],
      [{ name: "29 N-6: an entry's G_pa one ulp", base, edits: [set(ce('pipe_materials', 0, 'G_pa'), ulps(pm.G_pa, 1))] }, PREP],
      [{ name: "30 a case's pressure_regions -> null", base, invocation_edits: [set(md('load_cases', 0, 'pressure_regions'), null)] }, PREP],
      [{ name: "30b a case's pressure_regions absent", base, invocation_edits: [remove(md('load_cases', 0, 'pressure_regions'))] }, PREP],
      [{ name: "31 a case's pressure_regions -> one region", base, invocation_edits: [set(md('load_cases', 0, 'pressure_regions'), [{ id: 'region:x', member_pipe_ids: [pipeId], pressure: { value: 0, unit: 'Pa' } }])] }, PREP],
      [{ name: "32 a member's geometry.route -> preview", base, edits: [set(rb('sources', 0, 'section_terms', 0, 'geometry', 'route'), 'preview')] }, PREP],
      [{ name: '32b the authored poisson_ratio unit -> ""', base, invocation_edits: [set(md('materials', 0, 'poisson_ratio', 'unit'), '')] }, PREP],
      [{ name: '32c the authored constitutive_basis removed (S-C)', base, invocation_edits: [remove(md('materials', 0, 'constitutive_basis'))] }, PREP],
    ];
  };
  /** The optional file base: B3B_EXACT_BASE names a `{source, invocation}` file (for example lane P's `m3x` exact
   * successor); every shape is then also read on it. */
  const FILE_BASE = '<file>';
  /** Lane P's m3x exact successors (B3b-P; PP retained_facade_tests.rs `EXACT_PINNED`): base name, document text,
   * document sha256 and receipt sha256. Every shape is also read on each. */
  const M3X: [string, string, string, string][] = [
    ['m3x_sparse_interactive', m3xSparseText, '02465c6c92ac2e4360a77910cb54803590b5a11042dfddb223bf78f9e856e5d6', 'b1b4a6682260ca6bc499950b30f0f7179a77c038e266cc4b42045ed86ed3896f'],
    ['m3x_dense_scrutiny', m3xDenseText, '31f10f04f6f335dfb1a7e5f904198972903bfc9208660031bbfaa5c547d347cc', 'eabd2fc57b42158ad415ae664e7c712c1c4db21258b667251758172f3a5b776d'],
  ];
  const m3x = (name: string): { source: any; invocation: any } => {
    const [, text, documentSha, receiptSha] = M3X.find(m => m[0] === name)!;
    expect(createHash('sha256').update(text).digest('hex'), `${name}: lane P's pinned document`).toBe(documentSha);
    const doc = JSON.parse(text);
    expect(doc.source.retained_precision.receipt_sha256, name).toBe(receiptSha);
    return { source: doc.source, invocation: doc.invocation };
  };
  const exactBase = async (base: string) => base === FILE_BASE ? JSON.parse(readFileSync(process.env.B3B_EXACT_BASE!, 'utf8'))
    : M3X.some(m => m[0] === base) ? m3x(base) : exactSuccessor(base);
  /** RV120's F2 forgeries (REVIEW_RV120 b3_readers_01: forge_eg.py, forge_eg_inputs.jsonl), as the Rust reader's
   * `rv120_forgeries`: E or G-hat one ulp in all five receipt copies, the derived operational stiffness with it, the
   * evidence G_pa for G-hat, and the native hashes resealed; the 07e rehash recomputes the rest. Only G8 step 4's E and
   * G-hat bits refuse them (mutants B28, B29). */
  const RV120_FORGERIES: [string, string, any[]][] = [
    ["RV120 F2: E +1 ulp in every receipt copy, stiffness and native hashes resealed", 'ordinary_prepared_synthetic', [
      set(['retained_precision', 'body', 'cases', 0, 'selection', 'section_terms', 0, 'axial_stiffness'], '41c4990f17e516ae'),
      set(['retained_precision', 'body', 'groups', 0, 'stiffness_sha256'], '1fbb32395a897d401b673f4527d6858876a6c6668ff3779fa9484855ad2285b2'),
      set(['retained_precision', 'body', 'material_bases', 0, 'materials', 0, 'elastic_modulus'], '42474876e8000001'),
      set(['retained_precision', 'body', 'product_attempts', 0, 'operational', 'new', 0, 'inputs', 6], '42474876e8000001'),
      set(['retained_precision', 'body', 'product_attempts', 0, 'operational', 'new', 0, 'result', 'axial_stiffness'], '41c4990f17e516ae'),
      set(['retained_precision', 'body', 'product_attempts', 0, 'operational', 'old', 0, 'inputs', 6], '42474876e8000001'),
      set(['retained_precision', 'body', 'product_attempts', 0, 'operational', 'old', 0, 'result', 'axial_stiffness'], '41c4990f17e516ae'),
      set(['retained_precision', 'body', 'product_attempts', 0, 'preparation', 'members', 0, 'old_source', 0], '42474876e8000001'),
      set(['retained_precision', 'body', 'sources', 0, 'id_maps', 'members', 0, 'E'], '42474876e8000001'),
      set(['retained_precision', 'body', 'sources', 0, 'kernel_source_sha256'], '57dc66560ce90fb976a633b07f55c736b11f2b78265d791a3a4123c0751be478'),
      set(['retained_precision', 'body', 'sources', 0, 'section_terms', 0, 'axial_stiffness'], '41c4990f17e516ae'),
      set(['retained_precision', 'body', 'sources', 0, 'stiffness_sha256'], '1fbb32395a897d401b673f4527d6858876a6c6668ff3779fa9484855ad2285b2'),
    ]],
    ["RV120 F2: G-hat +1 ulp in every receipt copy, stiffness and native hashes resealed", 'ordinary_prepared_synthetic', [
      set(['contract_evidence', 'exact_cases', 0, 'pipe_materials', 0, 'G_pa'], decodeBinary64('4231ed8ec2000001')),
      set(['retained_precision', 'body', 'cases', 0, 'selection', 'section_terms', 0, 'torsional_stiffness'], '4128c47ead23fa81'),
      set(['retained_precision', 'body', 'groups', 0, 'stiffness_sha256'], '77a160acc4ffcf3ed919fe1e59ab3b670cc3ca4dcacbe192d9f932042c9e0980'),
      set(['retained_precision', 'body', 'material_bases', 0, 'materials', 0, 'shear_modulus'], '4231ed8ec2000001'),
      set(['retained_precision', 'body', 'product_attempts', 0, 'operational', 'new', 0, 'inputs', 7], '4231ed8ec2000001'),
      set(['retained_precision', 'body', 'product_attempts', 0, 'operational', 'new', 0, 'result', 'torsional_stiffness'], '4128c47ead23fa81'),
      set(['retained_precision', 'body', 'product_attempts', 0, 'operational', 'old', 0, 'inputs', 7], '4231ed8ec2000001'),
      set(['retained_precision', 'body', 'product_attempts', 0, 'operational', 'old', 0, 'result', 'torsional_stiffness'], '4128c47ead23fa82'),
      set(['retained_precision', 'body', 'product_attempts', 0, 'preparation', 'members', 0, 'old_source', 1], '4231ed8ec2000001'),
      set(['retained_precision', 'body', 'sources', 0, 'id_maps', 'members', 0, 'G'], '4231ed8ec2000001'),
      set(['retained_precision', 'body', 'sources', 0, 'kernel_source_sha256'], 'd098e88140c2f23621802f449f296928412138208b2adf72553b0a4790fe8f58'),
      set(['retained_precision', 'body', 'sources', 0, 'section_terms', 0, 'torsional_stiffness'], '4128c47ead23fa81'),
      set(['retained_precision', 'body', 'sources', 0, 'stiffness_sha256'], '77a160acc4ffcf3ed919fe1e59ab3b670cc3ca4dcacbe192d9f932042c9e0980'),
    ]],
    ["RV120 F2: E +1 ulp in every receipt copy, stiffness and native hashes resealed", 'm3x_sparse_interactive', [
      set(['retained_precision', 'body', 'cases', 0, 'selection', 'section_terms', 0, 'axial_stiffness'], '41b7b801dd7467b1'),
      set(['retained_precision', 'body', 'groups', 0, 'stiffness_sha256'], '1fbbf4ed8e9842ff8f6dcbe94517a8527eead7b775277e327e62b6e4538d712e'),
      set(['retained_precision', 'body', 'material_bases', 0, 'materials', 0, 'elastic_modulus'], '42474876e8000001'),
      set(['retained_precision', 'body', 'product_attempts', 0, 'operational', 'new', 0, 'inputs', 6], '42474876e8000001'),
      set(['retained_precision', 'body', 'product_attempts', 0, 'operational', 'new', 0, 'result', 'axial_stiffness'], '41b7b801dd7467b1'),
      set(['retained_precision', 'body', 'product_attempts', 0, 'operational', 'old', 0, 'inputs', 6], '42474876e8000001'),
      set(['retained_precision', 'body', 'product_attempts', 0, 'operational', 'old', 0, 'result', 'axial_stiffness'], '41b7b801dd7467b1'),
      set(['retained_precision', 'body', 'product_attempts', 0, 'preparation', 'members', 0, 'old_source', 0], '42474876e8000001'),
      set(['retained_precision', 'body', 'sources', 0, 'id_maps', 'members', 0, 'E'], '42474876e8000001'),
      set(['retained_precision', 'body', 'sources', 0, 'kernel_source_sha256'], 'ab4596d0b1a50b195b0368d573dbf26ed4c2cb6c1e76da6490052217b12b8bb2'),
      set(['retained_precision', 'body', 'sources', 0, 'section_terms', 0, 'axial_stiffness'], '41b7b801dd7467b1'),
      set(['retained_precision', 'body', 'sources', 0, 'stiffness_sha256'], '1fbbf4ed8e9842ff8f6dcbe94517a8527eead7b775277e327e62b6e4538d712e'),
    ]],
    ["RV120 F2: G-hat +1 ulp in every receipt copy, stiffness and native hashes resealed", 'm3x_sparse_interactive', [
      set(['contract_evidence', 'exact_cases', 0, 'pipe_materials', 0, 'G_pa'], decodeBinary64('4232a05f20000001')),
      set(['retained_precision', 'body', 'cases', 0, 'selection', 'section_terms', 0, 'torsional_stiffness'], '4135fb0cf390a830'),
      set(['retained_precision', 'body', 'groups', 0, 'stiffness_sha256'], 'd7aa429e5be25210f15a34f7138f7178efbbc443d6a016cc35f857268d9ec194'),
      set(['retained_precision', 'body', 'material_bases', 0, 'materials', 0, 'shear_modulus'], '4232a05f20000001'),
      set(['retained_precision', 'body', 'product_attempts', 0, 'operational', 'new', 0, 'inputs', 7], '4232a05f20000001'),
      set(['retained_precision', 'body', 'product_attempts', 0, 'operational', 'new', 0, 'result', 'torsional_stiffness'], '4135fb0cf390a830'),
      set(['retained_precision', 'body', 'product_attempts', 0, 'operational', 'old', 0, 'inputs', 7], '4232a05f20000001'),
      set(['retained_precision', 'body', 'product_attempts', 0, 'operational', 'old', 0, 'result', 'torsional_stiffness'], '4135fb0cf390a831'),
      set(['retained_precision', 'body', 'product_attempts', 0, 'preparation', 'members', 0, 'old_source', 1], '4232a05f20000001'),
      set(['retained_precision', 'body', 'sources', 0, 'id_maps', 'members', 0, 'G'], '4232a05f20000001'),
      set(['retained_precision', 'body', 'sources', 0, 'kernel_source_sha256'], '7f965da3aaa6d9720c549022c8947fa602a2065c26415d5663f5dc398e8f7b84'),
      set(['retained_precision', 'body', 'sources', 0, 'section_terms', 0, 'torsional_stiffness'], '4135fb0cf390a830'),
      set(['retained_precision', 'body', 'sources', 0, 'stiffness_sha256'], 'd7aa429e5be25210f15a34f7138f7178efbbc443d6a016cc35f857268d9ec194'),
    ]],
  ];
  /** sha256 of the canonical JSON of each forgery's `[source, invocation]`: RV120's input bytes (the Rust test's pins). */
  const RV120_FORGERY_DIGESTS = ['97566866cef65597109de17d34c69508ef3889a1d526321c8feedf5dc88b1f4a', '6e67243c49548db2cbcd10be6156c30182c1e884addbf95a264e33965431ef13', '3828c07c73d41e33324ec6be349ab4d265e57b439222962d1df1a91c974eb1ff', 'c17c8283efeb034afe82c4e9fdb4f46aad54ffef676c181733fbc6eec43226e6'];
  const shapes = async (): Promise<[Shape, any][]> => [
    ...BASES.map((base): [Shape, any] => [{ name: 'base', base }, 'pass']),
    ...rows(ORD, await exactSuccessor(ORD)),
    [{ name: '09 the preview successor relabelled physics-retained-1', base: '<preview:ordinary_prepared_synthetic>' }, G0],
    [{ name: "11 S-1: the preparation hashed with DEF-O's H", base: '<s1:ordinary_prepared_synthetic>' }, G('G1', 'RECEIPT_MISMATCH')],
    ...M3X.flatMap(([name]): [Shape, any][] => [
      [{ name: 'base', base: name }, 'pass'], ...rows(name, m3x(name)),
      [{ name: "11 S-1: the preparation hashed with DEF-O's H", base: `<s1:${name}>` }, G('G1', 'RECEIPT_MISMATCH')],
    ]),
    ...RV120_FORGERIES.map(([name, base, edits]): [Shape, any] => [{ name, base, edits }, PREP]),
    ...(process.env.B3B_EXACT_BASE ? [
      [{ name: 'base', base: FILE_BASE }, 'pass'] as [Shape, any], ...rows(FILE_BASE, await exactBase(FILE_BASE)),
      [{ name: "11 S-1: the preparation hashed with DEF-O's H", base: '<s1:file>' }, G('G1', 'RECEIPT_MISMATCH')] as [Shape, any],
    ] : []),
  ];
  async function input(shape: Shape): Promise<{ source: any; invocation: any }> {
    if (shape.base === '<preview:ordinary_prepared_synthetic>') {
      const base = corpus.cases.find((c: any) => c.id === ORD), source = structuredClone(base.source);
      source.producer.semantic_contract_id = EXACT_RETAINED_ID; source.formulation_basis.profile_id = EXACT_RETAINED_PROFILE; await rehash(source);
      return { source, invocation: structuredClone(base.invocation) };
    }
    if (shape.base.startsWith('<s1:')) {
      const name = shape.base.slice(4, -1), x = await exactBase(name === 'file' ? FILE_BASE : name); await rehash(x.source, 'a7ed7ca0bf0bba6e8b821ca4befa00a0fa9541a83694be8b28ac63e39b1d0349'); return x;
    }
    const x = await exactBase(shape.base);
    applyEdits(x.source, shape.edits); applyEdits(x.invocation, shape.invocation_edits);
    if (shape.invocation_edits?.length) x.source.retained_precision.body.invocation.value = await canonicalSha256HexCheckedV1({ domain: 'source_blocks_invocation_v1', payload: x.invocation });
    await rehash(x.source);
    return x;
  }
  const reading = async (run: () => Promise<any>) => { try { const r = await run(); return { ok: { eligible: r.numerical_eligible } }; } catch (e) { expect(e).toBeInstanceOf(RetainedPrecisionError); return { gate: (e as any).gate, code: (e as any).code }; } };
  it('pins the Rust test\'s 169 shapes (57 synthetic, 54 on each m3x successor, RV120\'s 4 forgeries), their first failures, and their three readings', async () => {
    const list = await shapes(), misses: string[] = [], lines: any[] = [];
    expect(list.filter(([x]) => x.base !== FILE_BASE && x.base !== '<s1:file>').length).toBe(3 + 52 + 2 + 2 * (1 + 52 + 1) + 4);
    for (const [shape, want] of list) {
      const { source, invocation } = await input(shape);
      const got = await firstFailure(source, invocation);
      if (JSON.stringify(got) !== JSON.stringify(want)) misses.push(`${shape.name} [${shape.base}]: got ${JSON.stringify(got)}, want ${JSON.stringify(want)}`);
      lines.push({ name: shape.name, base: shape.base, bound: await reading(() => validateRetainedPrecision(source, invocation)), unbound: await reading(() => validateRetainedPrecision(source)), transport: await reading(() => validateRetainedPrecisionTransport(source)) });
    }
    const out = process.env.B3B_SHAPES_OUT; if (out) writeFileSync(out, lines.map(l => JSON.stringify(l)).join('\n') + '\n');
    // B3B_INPUTS_OUT: every shape's materialized statement and invocation, to compare with the Rust reader's bytes.
    const inputsOut = process.env.B3B_INPUTS_OUT;
    if (inputsOut) { const rows: string[] = []; for (const [shape, want] of list) { const { source, invocation } = await input(shape); rows.push(JSON.stringify({ name: shape.name, base: shape.base, expected_bound: want, source, invocation })); } writeFileSync(inputsOut, rows.join('\n') + '\n'); }
    expect(misses).toEqual([]);
  }, 120_000);
  it('reads the synthetic and m3x exact successors bound (eligible), unbound and on transport (never eligible)', async () => {
    for (const base of [...BASES, ...M3X.map(m => m[0])]) {
      const { source, invocation } = await exactBase(base);
      expect(await reading(() => validateRetainedPrecision(source, invocation)), base).toEqual({ ok: { eligible: true } });
      expect(await reading(() => validateRetainedPrecision(source)), base).toEqual({ ok: { eligible: false } });
      expect(await reading(() => validateRetainedPrecisionTransport(source)), base).toEqual({ ok: { eligible: false } });
    }
  });
  it('reads three shapes unbound and on transport as the Rust reader does (G7 with TS\'s own base code)', async () => {
    const list = await shapes();
    for (const [prefix, want] of [
      ['29 ', { bound: PREP, unbound: { ok: { eligible: false } }, transport: { ok: { eligible: false } } }],
      ['16b ', { bound: SEC, unbound: SEC, transport: { gate: 'G7', code: 'PHYSICS_EVIDENCE_TRANSPORT_SHAPE' } }],
      ['01 ', { bound: G0, unbound: G0, transport: G0 }],
    ] as [string, any][]) {
      const [shape] = list.find(([x]) => x.name.startsWith(prefix))!;
      const { source, invocation } = await input(shape);
      expect({ bound: await reading(() => validateRetainedPrecision(source, invocation)), unbound: await reading(() => validateRetainedPrecision(source)), transport: await reading(() => validateRetainedPrecisionTransport(source)) }, shape.name).toEqual(want);
    }
  });
  it("carries S-C's own code as the G8 failure's detail (B3D-13)", async () => {
    const [shape] = (await shapes()).find(([x]) => x.name.startsWith('28 '))!;
    const { source, invocation } = await input(shape);
    const error = await validateRetainedPrecision(source, invocation).then(() => null, e => e);
    expect([error?.gate, error?.code]).toEqual(['G8', 'RETAINED_PRECISION_PREPARATION_MISMATCH']);
    expect(String(error?.detail)).toContain('ACTUAL_SELECTED_MATERIAL');
  });
  it('B3b integration: the exact route `retained_physics` (header dispatch, binding, downgrade guard, transport, standing, table, notices)', async () => {
    const { source, invocation } = await exactSuccessor(ORD);
    // The route's literals equal the reader's constants.
    expect([PHYSICS_RETAINED_CONTRACT_ID, PHYSICS_RETAINED_PROFILE]).toEqual([EXACT_RETAINED_ID, EXACT_RETAINED_PROFILE]);
    expect(sourceContract(source)).toBe('retained_physics');
    expect(sourceSemanticBinding(source)).toEqual({ id: PHYSICS_RETAINED_CONTRACT_ID, sha256: PHYSICS_RETAINED_CONTRACT_SHA256 });
    expect(retainedPrecisionDowngrade(source)).toBe(false);
    expect(await sourceContractTransport(source)).toBe('retained_physics');
    // A tampered receipt is refused on transport with the reader's code.
    const tampered = structuredClone(source); tampered.retained_precision.body.work_policy = 'W1-LME-20B-60B-v2';
    await expect(sourceContractTransport(tampered)).rejects.toMatchObject({ gate: 'G0', code: 'SOURCE_PRODUCER_CONTRACT_UNSUPPORTED' });
    // The downgrade guard: physics-1 carrying a receipt is unsupported.
    const downgraded = structuredClone(source); downgraded.producer.semantic_contract_id = 'openpipestress.result_semantics/0.3.0/physics-1'; downgraded.formulation_basis.profile_id = 'exact_straight_pressure_v2';
    expect(sourceContract(downgraded)).toBe('unsupported');
    // The exact identity without its receipt is not the successor's route.
    const bare = structuredClone(source); delete bare.retained_precision;
    expect(sourceContract(bare)).toBe('unsupported');
    // Standing comes only from a registered validation: unregistered bytes need recompute.
    expect(numericalResultStanding(source, invocation.request.model)).toMatchObject({ contract: 'retained_physics', eligible: false, findings: [RETAINED_PRECISION_VALIDATION_REQUIRED] });
    // XTABLE's rows are physics-1's; the row semantics read XTABLE.
    expect(semanticContractForSource(source)).toEqual(exactTableForTests);
    expect(exactTableForTests.rows).toEqual(physicsTableForTests.rows);
    // Notices: no preview notice (physics-1's rows); the unvalidated successor notice; no kind label.
    expect(knownSemanticNotices(source).map(n => n.id)).toEqual(['retained-precision-unvalidated']);
    expect(resultRowLabel(source.results.find((r: any) => r.kind === 'pipe_elastic_normal_stress_maximum_v2'), source)).toBeNull();
    // Rule binding fails closed without a registered validation.
    expect(ruleBindingRefusal(source, source.results[0])).toBe(RULE_QUANTITY_NOT_COVERED);
  });
  it('step 6: the table against the reader\'s constants, on test-only tables (B3-D §2.4)', () => {
    expect(tableBindsReaderConstants(exactTableForTests)).toBe(true);
    const drifted: string[] = [];
    for (const [path, value] of [[['receipt_bindings', 'canonicalization'], 'openpipestress_jcs_ijson_v2'], [['receipt_bindings', 'method'], 'contribution_preserving_multiprecision_v2'],
      [['receipt_bindings', 'projection_policy'], 'RP-LOGICAL-ATTEMPTS-v2'], [['receipt_bindings', 'work', 'case_limit'], 19_999_999_999], [['receipt_bindings', 'work', 'invocation_limit'], 60_000_000_001],
      [['receipt_bindings', 'work_policy'], 'W1-LME-20B-60B-v2'], [['receipt_bindings', 'extra'], 1], [['receipt_policy'], 'M03-INTEGRITY-MP-v3'], [['accuracy_classification', 'policy'], 'RP-FACADE-SI-v3']] as [string[], unknown][]) {
      const t = structuredClone(exactTableForTests) as any; let at = t; for (const k of path.slice(0, -1)) at = at[k]; at[path.at(-1)!] = value;
      if (tableBindsReaderConstants(t)) drifted.push(path.join('.'));
    }
    expect(drifted).toEqual([]);
  });
  it("RV120 F2: the four forgeries are RV120's inputs, refused at G8 step 4 bound, never eligible unbound or on transport", async () => {
    const list = (await shapes()).filter(([x]) => x.name.startsWith('RV120 F2: '));
    expect(list.length).toBe(4);
    const digests: string[] = [];
    for (const [shape, want] of list) {
      expect(want).toEqual(PREP);
      const { source, invocation } = await input(shape);
      digests.push(await canonicalSha256Hex([source, invocation]));
      expect({ bound: await reading(() => validateRetainedPrecision(source, invocation)), unbound: await reading(() => validateRetainedPrecision(source)), transport: await reading(() => validateRetainedPrecisionTransport(source)) }, `${shape.name} [${shape.base}]`)
        .toEqual({ bound: PREP, unbound: { ok: { eligible: false } }, transport: { ok: { eligible: false } } });
    }
    expect(digests).toEqual(RV120_FORGERY_DIGESTS);
  });
  it("ROOT's ruling on I100's B3 addendum 01: G8's sourced-case check on the preview and exact routes", async () => {
    // A sourced case passes only with pressure_regions absent, null or [] (type-strict; [] only on the exact route),
    // equivalent_static absent or null, and no analysis_state member (null included); a case-level `pressure`, a key PP's
    // typed case lacks, is not read. I100's 22 values on PP's milestone successors, both modes (an invocation edit,
    // resealed with DEF-O's H), and its x08 and p02 on the synthetic and m3x exact successors (DEF-E's H).
    const preview: [string, unknown, boolean][] = [
      ['analysis_state', { kind: 'load_reference_state' }, false], ['analysis_state', null, false], ['analysis_state', {}, false],
      ['pressure', { value: 1000.0, unit: 'Pa' }, true], ['pressure', null, true], ['pressure', 0, true],
      ['pressure_regions', 'x', false], ['pressure_regions', {}, false], ['pressure_regions', { id: 'region:x' }, false], ['pressure_regions', 0, false],
      ['pressure_regions', 1, false], ['pressure_regions', true, false], ['pressure_regions', false, false], ['pressure_regions', '', false],
      ['pressure_regions', [], true], ['pressure_regions', null, true], ['pressure_regions', [{ id: 'region:x', member_pipe_ids: ['M1'] }], false],
      ['equivalent_static', {}, false], ['equivalent_static', false, false], ['equivalent_static', 0, false], ['equivalent_static', null, true],
      ['notes', 'free text', true],
    ];
    const exact: [string, unknown, boolean][] = [['analysis_state', { kind: 'load_reference' }, false], ['pressure', { value: 1000.0, unit: 'Pa' }, true]];
    const want = (passes: boolean) => ({ bound: passes ? { ok: { eligible: true } } : PREP, unbound: { ok: { eligible: false } }, transport: { ok: { eligible: false } } });
    const readAll = async (source: any, invocation: any) => ({ bound: await reading(() => validateRetainedPrecision(source, invocation)), unbound: await reading(() => validateRetainedPrecision(source)), transport: await reading(() => validateRetainedPrecisionTransport(source)) });
    const misses: string[] = [];
    let checked = 0;
    const read = async (label: string, base: { source: any; invocation: any }, rows: [string, unknown, boolean][]) => {
      expect(await readAll(base.source, base.invocation), `${label}: the base`).toEqual(want(true));
      for (const [key, value, passes] of rows) {
        const source = structuredClone(base.source), invocation = structuredClone(base.invocation);
        invocation.request.model.load_cases[0][key] = structuredClone(value);
        source.retained_precision.body.invocation.value = await canonicalSha256HexCheckedV1({ domain: 'source_blocks_invocation_v1', payload: invocation });
        await rehash(source);
        const got = await readAll(source, invocation);
        if (JSON.stringify(got) !== JSON.stringify(want(passes))) misses.push(`${label}: ${key} = ${JSON.stringify(value)}: got ${JSON.stringify(got)}`);
        checked += 1;
      }
    };
    for (const [mode, text] of [['sparse_interactive', milestoneSparseText], ['dense_scrutiny', milestoneDenseText]] as const) {
      const doc = JSON.parse(text);
      await read(`milestone ${mode}`, { source: doc.source, invocation: doc.invocation }, preview);
    }
    for (const base of [...BASES, ...M3X.map(m => m[0])]) await read(base, await exactBase(base), exact);
    expect(misses).toEqual([]);
    expect(checked).toBe(2 * 22 + 5 * 2);
  }, 60_000);
});
