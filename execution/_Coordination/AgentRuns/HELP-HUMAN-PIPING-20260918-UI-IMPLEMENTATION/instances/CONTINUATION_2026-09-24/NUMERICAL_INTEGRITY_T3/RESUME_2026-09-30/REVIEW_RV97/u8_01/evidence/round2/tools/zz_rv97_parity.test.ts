// RV97 round-2 probe (review copy only; never committed): the TS reader on RV97's identical documents.
import { describe, it } from 'vitest';
import { appendFileSync, readFileSync, readdirSync, writeFileSync } from 'node:fs';
import { join } from 'node:path';
import { validateRetainedPrecision } from './retainedPrecision';

const dir = process.env.RV97_DOCS ?? '', out = process.env.RV97_OUT ?? '';
describe('RV97 round 2: the TS reader on identical documents', () => {
  it('reads every document', async () => {
    const files = readdirSync(dir).filter(f => f.endsWith('.json')).sort();
    if (files.length === 0) throw new Error('no documents');
    writeFileSync(out, '');
    for (const f of files) {
      const d = JSON.parse(readFileSync(join(dir, f), 'utf8'));
      let line: unknown;
      try {
        const r: any = await validateRetainedPrecision(d.source, d.invocation ?? undefined);
        const rows = r.classifications.map((x: any) => [x.result_id, x.class, x.normalized_bits, x.scale_bits ?? null, x.bound_bits ?? null]);
        line = { label: d.label, ok: true, bound: r.invocation_bound, eligible: r.numerical_eligible, standing: r.standing, publication_sha256: r.publication_sha256, classifications: rows };
      } catch (e) {
        const err = e as { gate?: string; code?: string; message?: string };
        line = { label: d.label, ok: false, gate: err.gate ?? null, code: err.code ?? null, message: err.gate ? undefined : err.message };
      }
      appendFileSync(out, JSON.stringify(line) + '\n');
    }
  }, 1800000);
});
