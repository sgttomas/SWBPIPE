// I61 07h probe harness (scratch lane only): validate pre-applied entries through the public entry.
import { it } from 'vitest';
import { readFileSync, writeFileSync } from 'node:fs';
import { validateRetainedPrecision } from './retainedPrecision';
it('i61 dump', async () => {
  const lines = readFileSync(process.env.I61_APPLIED as string, 'utf8').split('\n').filter(Boolean);
  const res: Record<string, unknown> = {};
  for (const line of lines) {
    const d = JSON.parse(line);
    let got: unknown;
    try {
      const v: any = await validateRetainedPrecision(d.source, d.invocation);
      got = { pass: true, eligible: v.numerical_eligible, standing: v.standing, classes: ['relative_verified', 'absolute_verified', 'input_derived', 'non_quantity'].map(k => v.classifications.filter((c: any) => c.class === k).length) };
    } catch (e: any) { got = [e.gate, e.code]; }
    res[`${d.kind}:${d.id}`] = got;
  }
  writeFileSync(process.env.I61_OUT as string, JSON.stringify(res, null, 0));
}, 1_800_000);
