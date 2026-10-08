// I92 (B1 SR-TS) cascade census (PLAN_v2 §2.4, R5): the reader's observed outcome on every 07m entry.
// Scratch only: copied into a scratch archive's src/features/results, never into a maintained tree.
// The harness (applyEdits, rehash, applyEntry) is retainedPrecision.test.ts's, copied verbatim.
import { afterAll, describe, it } from 'vitest';
import { writeFileSync } from 'node:fs';
import { createHash } from 'node:crypto';
import corpusText from '../../../../../fixtures/results/retained_precision_cases.json?raw';
import { validateRetainedPrecision, validateRetainedPrecisionTransport, RetainedPrecisionError } from './retainedPrecision';
import { canonicalSha256HexCheckedV1 } from '../../services/hashService';
const corpus = JSON.parse(corpusText);
const OUT = process.env.I92_CENSUS_OUT as string;
const lines: unknown[] = [];
function rehashRef(items: any[], ref: unknown): any {
  return typeof ref === 'number' && Number.isInteger(ref) && ref >= 0 && !Object.is(ref, -0) && ref < items.length ? items[ref] : undefined;
}
async function rehash(source: any) {
  const body = source.retained_precision?.body;
  if (!body || typeof body !== 'object') return;
  for (const s of body.sources) if (s.preparation) {
    const a = rehashRef(body.product_attempts, s.preparation.attempt_ref);
    if (!a || !a.preparation.members.every((m: any) => m.result.kind === 'prepared')) continue;
    s.preparation.sha256 = await canonicalSha256HexCheckedV1({ domain: 'retained_precision_preparation_v1', payload: {
      definition_id: a.definition_id, definition_sha256: 'a7ed7ca0bf0bba6e8b821ca4befa00a0fa9541a83694be8b28ac63e39b1d0349', owner_ref: a.owner_ref, ordinary_attempt_ref: a.ordinary_attempt_ref, material_basis_ref: a.material_basis_ref,
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
  const step = (value: any, key: unknown) => { if (Array.isArray(value) && !(typeof key === 'number' && Number.isInteger(key) && key >= 0 && !Object.is(key, -0))) throw new Error('edit path index ' + String(key)); return key as any; };
  for (const edit of edits ?? []) {
    let value = root; for (const key of edit.path.slice(0, -1)) value = value[step(value, key)];
    const key = step(value, edit.path.at(-1));
    if (edit.op === 'remove') { if (Array.isArray(value)) value.splice(key, 1); else delete value[key]; }
    else if (edit.op === 'set') value[key] = structuredClone(edit.value);
    else throw new Error('unsupported edit op ' + edit.op);
  }
}
async function applyEntry(m: any): Promise<{ base: any; source: any; invocation: any }> {
  const base = corpus.cases.find((c: any) => c.id === m.base), source = structuredClone(base.source), invocation = structuredClone(base.invocation);
  applyEdits(source, m.edits);
  applyEdits(invocation, m.invocation_edits);
  if (m.invocation_edits?.length) source.retained_precision.body.invocation.value = await canonicalSha256HexCheckedV1({ domain: 'source_blocks_invocation_v1', payload: invocation });
  if (m.rehash !== 'all') throw new Error('unsupported rehash ' + m.rehash);
  await rehash(source);
  applyEdits(source, m.after_rehash);
  return { base, source, invocation };
}
// Key-order independent: the reader's rows and the corpus's expected rows order their members differently.
const stable = (v: unknown): string => Array.isArray(v) ? '[' + v.map(stable).join(',') + ']'
  : v !== null && typeof v === 'object' ? '{' + Object.keys(v).sort().map(k => JSON.stringify(k) + ':' + stable((v as any)[k])).join(',') + '}' : JSON.stringify(v);
const digest = (v: unknown) => createHash('sha256').update(stable(v)).digest('hex');
async function outcome(run: () => Promise<any>): Promise<unknown> {
  try {
    const r = await run();
    return { pass: { invocation_bound: r.invocation_bound, numerical_eligible: r.numerical_eligible, standing: r.standing, publication_sha256: r.publication_sha256, classifications_sha256: digest(r.classifications), classifications_count: r.classifications.length } };
  } catch (e) {
    if (e instanceof RetainedPrecisionError) return { gate: e.gate, code: e.code, detail: e.detail };
    return { thrown: String(e) };
  }
}
describe('I92 census over the shared corpus', () => {
  it('records every base, mutation and must-pass outcome', async () => {
    for (const c of corpus.cases) {
      lines.push({ kind: 'base', id: c.id, bound: await outcome(() => validateRetainedPrecision(structuredClone(c.source), structuredClone(c.invocation))),
        unbound: await outcome(() => validateRetainedPrecision(structuredClone(c.source))),
        transport: await outcome(() => { const t = structuredClone(c.source); delete t.results; return validateRetainedPrecisionTransport(t); }),
        expected_classifications_sha256: digest(c.expected_classifications) });
    }
    for (const m of corpus.mutations) {
      const { source, invocation } = await applyEntry(m);
      lines.push({ kind: 'mutation', id: m.id, observed: await outcome(() => validateRetainedPrecision(source, invocation)), expected: m.expected_by_reader?.typescript ?? m.expected });
    }
    for (const m of corpus.must_pass) {
      const { base, source, invocation } = await applyEntry(m);
      lines.push({ kind: 'must_pass', id: m.id, observed: await outcome(() => validateRetainedPrecision(source, invocation)), expected_eligibility: m.expected_eligibility, base_classifications_sha256: digest(base.expected_classifications) });
    }
  }, 600000);
  afterAll(() => { writeFileSync(OUT, lines.map(l => JSON.stringify(l)).join('\n') + '\n'); });
});
