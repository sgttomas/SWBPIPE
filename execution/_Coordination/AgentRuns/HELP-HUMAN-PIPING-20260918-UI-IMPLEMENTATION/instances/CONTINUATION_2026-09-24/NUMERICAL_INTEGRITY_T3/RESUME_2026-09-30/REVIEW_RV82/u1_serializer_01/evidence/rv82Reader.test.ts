// RV82 disposable lane (copy only): the unchanged accepted TS reader on the serializer's output.
import { it } from 'vitest';
import { readFileSync, writeFileSync, readdirSync } from 'node:fs';
import { join } from 'node:path';
import { validateRetainedPrecision, RetainedPrecisionError } from './retainedPrecision';
it('rv82 TS reader on U1 milestone receipts', async () => {
  const dir = process.env.RV82_READ_DIR as string;
  const lines: string[] = [];
  for (const f of readdirSync(dir).filter((x) => x.startsWith('u1_milestone_') && x.endsWith('.json')).sort()) {
    const c = JSON.parse(readFileSync(join(dir, f), 'utf8'));
    try {
      const v = await validateRetainedPrecision(c.source, c.invocation);
      lines.push(`TS ${f} PASS standing=${v.standing} eligible=${v.numerical_eligible} invocation_bound=${(v as any).invocation_bound} classes=${v.classifications.length}`);
      writeFileSync(join(dir, f + '.ts_classes.txt'), v.classifications.map((x) => `${x.result_id}|${x.normalized_bits}|${x.scale_bits ?? 'null'}|${x.class}`).join('\n') + '\n');
    } catch (e) {
      if (e instanceof RetainedPrecisionError) lines.push(`TS ${f} FIRST ${e.gate} ${e.code} detail=${e.detail}`);
      else lines.push(`TS ${f} THROW ${String(e)}`);
    }
  }
  writeFileSync(join(dir, 'ts_results.txt'), lines.join('\n') + '\n');
}, 300000);
