// RV97 round-1 probe (review copy only; never committed): the TS reader on RV97's saved documents.
import { describe, it } from 'vitest';
import { appendFileSync, readFileSync, readdirSync } from 'node:fs';
import { join } from 'node:path';
import { validateRetainedPrecision } from './retainedPrecision';

const dir = process.env.RV97_DOCS ?? '';
const out = process.env.RV97_TS_LOG ?? '';
describe('RV97: the TS reader on the probe documents', () => {
  it('reads each document, bound and unbound', async () => {
    const files = readdirSync(dir).filter((f) => f.endsWith('.json')).sort();
    if (files.length === 0) throw new Error('no documents');
    for (const f of files) {
      const doc = JSON.parse(readFileSync(join(dir, f), 'utf8'));
      for (const [label, inv] of [['bound', doc.invocation], ['unbound', undefined]] as const) {
        let line: string;
        try {
          const r = await validateRetainedPrecision(doc.source, inv);
          const counts: Record<string, number> = {};
          for (const c of r.classifications) counts[c.class] = (counts[c.class] ?? 0) + 1;
          line = JSON.stringify({ file: f, label, ok: true, bound: r.invocation_bound, eligible: r.numerical_eligible, standing: r.standing,
            rows: r.classifications.length, classes: Object.fromEntries(Object.entries(counts).sort()), publication_sha256: r.publication_sha256 });
        } catch (e) {
          const err = e as { gate?: string; code?: string; message?: string };
          line = JSON.stringify({ file: f, label, ok: false, gate: err.gate ?? null, code: err.code ?? null, message: err.message ?? null });
        }
        appendFileSync(out, `RV97_TS ${line}\n`);
      }
    }
  }, 600000);
});
