// I61 disposable harness (archive copy only): runs the unchanged TS reader on emitted receipts.
import { it } from 'vitest';
import { readFileSync, writeFileSync } from 'node:fs';
import { validateRetainedPrecision, RetainedPrecisionError } from './retainedPrecision';
const LIST = 'WT/scratch/i61_receipt_experiment_03/out/ts_receipts.txt';
const OUT = 'WT/scratch/i61_receipt_experiment_03/out/ts_results.txt';
it('i61 TS reader on emitted receipts', async () => {
  const lines: string[] = [];
  for (const path of readFileSync(LIST, 'utf8').split('\n').filter(Boolean)) {
    const c = JSON.parse(readFileSync(path, 'utf8'));
    try {
      const v = await validateRetainedPrecision(c.source, c.invocation);
      lines.push(`I61_TS ${c.id} PASS standing=${v.standing} eligible=${v.numerical_eligible} classes=${v.classifications.length}`);
      writeFileSync(path + '.ts_classes.txt', v.classifications.map((x) => `${x.result_id}|${x.normalized_bits}|${x.scale_bits ?? 'null'}|${x.class}`).join('\n'));
    } catch (e) {
      if (e instanceof RetainedPrecisionError) lines.push(`I61_TS ${c.id} FIRST ${e.gate} ${e.code} detail=${e.detail}`);
      else lines.push(`I61_TS ${c.id} THROW ${String(e)}`);
    }
  }
  writeFileSync(OUT, lines.join('\n') + '\n');
}, 120000);
