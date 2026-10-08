// RV113 (RV-R) reviewer harness for the TypeScript reader. Not part of any candidate: copied into the
// reviewer's own archive copies only. It materializes every shared corpus entry (and, optionally, reviewer
// probes) from the snapshot format rule, written here independently (SHARED_SNAPSHOT_06C `format_change`;
// SHARED_SNAPSHOT_07E `format_rule`), and records each reader verdict as one JSON line.
// Environment: RV113_OUT (census, JSON lines); RV113_PROBES (a JSON array of entries) with RV113_PROBES_OUT.
import { it } from 'vitest';
import { createHash } from 'node:crypto';
import { readFileSync, writeFileSync } from 'node:fs';
import corpusText from '../../../../../fixtures/results/retained_precision_cases.json?raw';
import { canonicalSha256HexCheckedV1 } from '../../services/hashService';
import { RetainedPrecisionError, validateRetainedPrecision, validateRetainedPrecisionTransport } from './retainedPrecision';

const corpus = JSON.parse(corpusText);
const DEFINITION_SHA256 = 'a7ed7ca0bf0bba6e8b821ca4befa00a0fa9541a83694be8b28ac63e39b1d0349';
type J = any;

/** The strict index rule: a JSON number (never a boolean), finite, integral, >= 0 and not -0. */
function strictIndex(v: unknown): number | null {
  return typeof v === 'number' && Number.isFinite(v) && Number.isInteger(v) && v >= 0 && !Object.is(v, -0) ? v : null;
}
function step(v: J, key: unknown): J {
  const i = strictIndex(key);
  if (i !== null) { if (!Array.isArray(v) || i >= v.length) throw new Error(`index ${i}`); return v[i]; }
  if (typeof key !== 'string' || v === null || typeof v !== 'object' || Array.isArray(v)) throw new Error(`key ${String(key)}`);
  if (!(key in v)) v[key] = {};
  return v[key];
}
function applyEdit(root: J, e: J): void {
  const path: unknown[] = e.path; let at = root;
  for (const p of path.slice(0, -1)) at = step(at, p);
  const last = path[path.length - 1], i = strictIndex(last);
  if (e.op === 'remove') {
    if (i !== null) { if (!Array.isArray(at) || i >= at.length) throw new Error('remove index'); at.splice(i, 1); }
    else { if (typeof last !== 'string' || at === null || typeof at !== 'object') throw new Error('remove key'); delete at[last]; }
  } else if (e.op === 'set') {
    if (i !== null) { if (!Array.isArray(at) || i >= at.length) throw new Error('set index'); at[i] = structuredClone(e.value); }
    else { if (typeof last !== 'string' || at === null || typeof at !== 'object') throw new Error('set key'); at[last] = structuredClone(e.value); }
  } else throw new Error(`op ${e.op}`);
}
const h = (domain: string, payload: unknown) => canonicalSha256HexCheckedV1({ domain, payload });
/** 07e order: preparation hashes; selected source identities; publication; receipt. */
async function rehashAll(source: J): Promise<void> {
  const body = source?.retained_precision?.body;
  if (!body || typeof body !== 'object' || Array.isArray(body)) return;
  const attempts = Array.isArray(body.product_attempts) ? body.product_attempts : [];
  for (const s of Array.isArray(body.sources) ? body.sources : []) {
    const ai = strictIndex(s?.preparation?.attempt_ref); if (ai === null) continue;
    const a = attempts[ai]; if (!a || typeof a !== 'object') continue;
    const members: J[] = Array.isArray(a.preparation?.members) ? a.preparation.members : [];
    if (!members.every(m => m.result.kind === 'prepared')) continue;
    s.preparation.sha256 = await h('retained_precision_preparation_v1', { definition_id: a.definition_id, definition_sha256: DEFINITION_SHA256,
      owner_ref: a.owner_ref, ordinary_attempt_ref: a.ordinary_attempt_ref, material_basis_ref: a.material_basis_ref,
      members: members.map(m => ({ member: m.member, old_source: m.old_source, old_facts: m.old_facts, section: m.result.section })) });
  }
  const sources = Array.isArray(body.sources) ? body.sources : [];
  for (const c of Array.isArray(body.cases) ? body.cases : []) {
    if (c.status !== 'selected') continue;
    const si = strictIndex(c.source_ref); if (si === null || si >= sources.length || !sources[si] || typeof sources[si] !== 'object') continue;
    const s = structuredClone(sources[si]); delete s.index;
    c.source_identity_sha256 = await h('retained_precision_source_mp_v2', s);
  }
  const publication = structuredClone(source); delete publication.retained_precision;
  body.publication_sha256 = await h('retained_precision_publication_mp_v2', publication);
  source.retained_precision.receipt_sha256 = await h('retained_precision_receipt_mp_v2', body);
}
async function materialize(entry: J): Promise<{ source: J; invocation: J }> {
  const base = corpus.cases.find((c: J) => c.id === entry.base); if (!base) throw new Error('unknown base');
  const source = structuredClone(base.source), invocation = structuredClone(base.invocation);
  for (const e of entry.edits ?? []) applyEdit(source, e);
  const invEdits: J[] = entry.invocation_edits ?? [];
  for (const e of invEdits) applyEdit(invocation, e);
  if (invEdits.length) source.retained_precision.body.invocation.value = await h('source_blocks_invocation_v1', invocation);
  if (entry.rehash !== 'all') throw new Error('rehash is not all');
  await rehashAll(source);
  for (const e of entry.after_rehash ?? []) applyEdit(source, e);
  return { source, invocation };
}
const sha = (text: string) => createHash('sha256').update(text).digest('hex');
async function verdict(run: () => Promise<J>): Promise<J> {
  try {
    const v = await run();
    return { ok: { invocation_bound: v.invocation_bound, numerical_eligible: v.numerical_eligible, standing: v.standing, publication_sha256: v.publication_sha256,
      classifications: v.classifications.length, classifications_sha256: sha(JSON.stringify(v.classifications)) } };
  } catch (e) {
    if (e instanceof RetainedPrecisionError) return { err: { gate: e.gate, code: e.code, detail: e.detail } };
    return { throw: String(e) };
  }
}
async function evaluate(kind: string, i: number, entry: J, raw = false): Promise<J> {
  let m: { source: J; invocation: J };
  try { m = raw ? { source: structuredClone(entry.source), invocation: structuredClone(entry.invocation) } : await materialize(entry); }
  catch (e) { return { set: kind, i, id: entry.id, materialize_error: String(e) }; }
  const input = JSON.stringify([m.source, m.invocation]);
  return { set: kind, i, id: entry.id, input_sha256: sha(input),
    bound: await verdict(() => validateRetainedPrecision(structuredClone(m.source), structuredClone(m.invocation))),
    unbound: await verdict(() => validateRetainedPrecision(structuredClone(m.source))),
    transport: await verdict(() => validateRetainedPrecisionTransport(structuredClone(m.source))) };
}
const write = (path: string, lines: J[]) => writeFileSync(path, lines.map(l => JSON.stringify(l)).join('\n') + '\n');

it('rv113 census', async () => {
  const out = process.env.RV113_OUT; if (!out) return;
  const lines: J[] = [];
  for (const [i, c] of corpus.cases.entries()) lines.push(await evaluate('base', i, c, true));
  for (const [i, m] of corpus.mutations.entries()) lines.push(await evaluate('mutation', i, m));
  for (const [i, m] of corpus.must_pass.entries()) lines.push(await evaluate('must_pass', i, m));
  write(out, lines);
}, 3_600_000);

it('rv113 probes', async () => {
  const path = process.env.RV113_PROBES, out = process.env.RV113_PROBES_OUT; if (!path || !out) return;
  const probes: J[] = JSON.parse(readFileSync(path, 'utf8'));
  const lines: J[] = [];
  for (const [i, p] of probes.entries()) lines.push(await evaluate('probe', i, p));
  write(out, lines);
}, 3_600_000);
