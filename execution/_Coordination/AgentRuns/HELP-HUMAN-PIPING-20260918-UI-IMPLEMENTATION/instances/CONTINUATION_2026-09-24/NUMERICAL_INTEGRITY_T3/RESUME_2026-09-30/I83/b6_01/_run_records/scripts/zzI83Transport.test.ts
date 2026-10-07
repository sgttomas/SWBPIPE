// I83 B6 probe (scratch only): TS transport route (sourceContractTransport) on every RV92 probe in I83_PROBE_DIR.
import { it } from 'vitest';
import { readFileSync, writeFileSync } from 'node:fs';
import { join } from 'node:path';
import { sourceContractTransport } from './numericalResultQuality';
it('transport', async () => {
  const dir = process.env.I83_PROBE_DIR!; const out: Record<string, string> = {};
  for (const item of JSON.parse(readFileSync(join(dir, 'index.json'), 'utf8'))) {
    const probe = JSON.parse(readFileSync(join(dir, item.file), 'utf8'));
    out[probe.id] = await sourceContractTransport(probe.source).then(() => 'ok', (e: any) => String(e?.message ?? e));
  }
  writeFileSync(process.env.I83_OUT!, JSON.stringify(out, null, 0));
}, 600000);
