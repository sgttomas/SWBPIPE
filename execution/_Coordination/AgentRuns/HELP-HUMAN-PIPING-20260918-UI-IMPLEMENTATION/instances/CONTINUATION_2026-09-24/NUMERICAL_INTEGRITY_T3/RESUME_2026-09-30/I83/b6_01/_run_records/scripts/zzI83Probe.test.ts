// I83 B6 probe (scratch only): TS first failure on the probe entries in I83_PROBES (a JSON list of {label, base, edits}).
import { it } from 'vitest';
import { readFileSync, writeFileSync } from 'node:fs';
import corpusText from '../../../../../fixtures/results/retained_precision_cases.json?raw';
import { validateRetainedPrecision, RetainedPrecisionError } from './retainedPrecision';
import { canonicalSha256HexCheckedV1 } from '../../services/hashService';
const corpus = JSON.parse(corpusText);
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
  for (const edit of edits ?? []) {
    let value = root; for (const key of edit.path.slice(0, -1)) value = value[key];
    const key = edit.path.at(-1);
    if (edit.op === 'remove') { if (Array.isArray(value)) value.splice(key, 1); else delete value[key]; }
    else value[key] = structuredClone(edit.value);
  }
}
it('probe', async () => {
  const probes = JSON.parse(readFileSync(process.env.I83_PROBES!, 'utf8'));
  const out: any[] = [];
  for (const p of probes) {
    const base = corpus.cases.find((c: any) => c.id === p.base); const source = structuredClone(base.source), invocation = structuredClone(base.invocation);
    applyEdits(source, p.edits); await rehash(source);
    let r = 'pass';
    try { await validateRetainedPrecision(source, invocation); } catch (e) { r = e instanceof RetainedPrecisionError ? `${e.gate} ${e.code}` : `HARNESS ${String(e)}`; }
    out.push({ label: p.label, typescript: r });
  }
  writeFileSync(process.env.I83_OUT!, JSON.stringify(out, null, 1));
}, 600000);
