// I68 U8-0 probe (disposable archive copy only; never committed): the TS reader on each saved document.
import { describe, it } from 'vitest';
import { appendFileSync, readFileSync, readdirSync } from 'node:fs';
import { join } from 'node:path';
import { validateRetainedPrecision } from './retainedPrecision';

const dir = process.env.I68_OUT ?? '';
const log = (line: string) => { console.log(line); if (process.env.I68_TS_LOG) appendFileSync(process.env.I68_TS_LOG, line + '\n'); };
describe('I68 U8-0 probe: the TS reader on the saved documents', () => {
  it('reads each document, bound and unbound', async () => {
    const files = dir ? readdirSync(dir).filter((f) => f.endsWith('.json')).sort() : [];
    if (files.length === 0) throw new Error('I68_OUT has no documents');
    for (const f of files) {
      const doc = JSON.parse(readFileSync(join(dir, f), 'utf8'));
      for (const [label, inv] of [['bound', doc.invocation], ['unbound', undefined]] as const) {
        try {
          const r = await validateRetainedPrecision(doc.source, inv);
          const counts: Record<string, number> = {};
          for (const c of r.classifications) counts[c.class] = (counts[c.class] ?? 0) + 1;
          const classes = Object.keys(counts).sort().map((k) => `${k}:${counts[k]}`).join(',');
          log(`I68_TS ${f} ${label} PASS invocation_bound=${r.invocation_bound} numerical_eligible=${r.numerical_eligible} standing=${r.standing} classifications=${r.classifications.length} classes={${classes}} publication_sha256=${r.publication_sha256}`);
        } catch (e) {
          const err = e as { gate?: string; code?: string; detail?: string | null; message?: string };
          log(`I68_TS ${f} ${label} FAIL gate=${err.gate} code=${err.code} detail=${err.detail ?? null} message=${err.message}`);
        }
      }
    }
  }, 600000);
});
