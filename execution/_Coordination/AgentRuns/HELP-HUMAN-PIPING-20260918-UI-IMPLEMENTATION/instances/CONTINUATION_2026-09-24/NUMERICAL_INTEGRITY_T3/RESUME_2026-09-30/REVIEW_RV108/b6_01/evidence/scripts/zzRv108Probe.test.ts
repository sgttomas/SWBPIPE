// RV108 probe harness (reviewer scratch only; never committed): each entry point over each probe.
import { it } from 'vitest';
import { readFileSync, writeFileSync } from 'node:fs';
import { createHash } from 'node:crypto';
import { validateRetainedPrecision, validateRetainedPrecisionTransport, RetainedPrecisionError } from './retainedPrecision';
import { sourceContract, sourceContractTransport } from './numericalResultQuality';

function stable(v: unknown): string {
  if (typeof v === 'bigint') return JSON.stringify('bigint:' + v.toString());
  if (Array.isArray(v)) return '[' + v.map(stable).join(',') + ']';
  if (v && typeof v === 'object') return '{' + Object.keys(v).sort().map(k => JSON.stringify(k) + ':' + stable((v as any)[k])).join(',') + '}';
  return JSON.stringify(v) ?? 'undefined';
}
async function run(fn: () => Promise<any> | any) {
  try {
    const r = await fn();
    return typeof r === 'string' ? { ok: true, route: r } : { ok: true, eligible: r?.numerical_eligible ?? null, sha: createHash('sha256').update(stable(r)).digest('hex') };
  } catch (e) {
    if (e instanceof RetainedPrecisionError) return { gate: e.gate, code: e.code, detail: e.detail };
    return { error: e instanceof Error ? e.message : String(e) };
  }
}
it('rv108 probes', async () => {
  const probes = JSON.parse(readFileSync(process.env.RV108_IN!, 'utf8'));
  const out: string[] = [];
  for (const p of probes) {
    const s = p.source, inv = p.invocation;
    out.push(JSON.stringify({ id: p.id, family: p.family,
      raw_inv: await run(() => validateRetainedPrecision(structuredClone(s), structuredClone(inv))),
      raw_none: await run(() => validateRetainedPrecision(structuredClone(s))),
      transport: await run(() => validateRetainedPrecisionTransport(structuredClone(s))),
      c_raw: await run(() => sourceContract(structuredClone(s))),
      c_transport: await run(() => sourceContractTransport(structuredClone(s))) }));
  }
  writeFileSync(process.env.RV108_OUT!, out.join('\n') + '\n');
}, 7_200_000);
