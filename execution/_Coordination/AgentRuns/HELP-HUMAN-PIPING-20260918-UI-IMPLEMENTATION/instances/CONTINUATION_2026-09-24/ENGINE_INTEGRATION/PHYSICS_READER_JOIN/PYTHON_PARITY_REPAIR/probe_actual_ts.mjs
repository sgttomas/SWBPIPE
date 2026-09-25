// Direct reader controls from actual producer bytes; no Current/authentication claim.
import { readFileSync, writeFileSync, existsSync } from 'node:fs';
import { dirname, join } from 'node:path';
import { pathToFileURL, fileURLToPath } from 'node:url';
import { createHash } from 'node:crypto';
const out = dirname(fileURLToPath(import.meta.url));
let project = out;
while (!existsSync(join(project, 'core/analysis_runs/compatibility.py'))) project = dirname(project);
const reader = join(project, 'apps/desktop/src/features/results/physicsResultEvidence.ts');
const { validatePhysicsEvidence } = await import(pathToFileURL(reader).href);
const blocked = join(out, '../RULE_SOURCE_IMMUTABILITY/BLOCKED_ACTUAL');
const fixtures = [
  ...['sparse', 'dense'].flatMap(mode => [join(project, `fixtures/results/physics_connected_mechanics_${mode}.json`), join(project, `fixtures/results/physics_thermal_ui_mechanics_${mode}.json`)]),
  ...['missing_nu', 'missing_closure'].map(name => join(blocked, `${name}.raw.json`)),
];
const sha = path => createHash('sha256').update(readFileSync(path)).digest('hex');
const results = fixtures.map(path => {
  const source = JSON.parse(readFileSync(path, 'utf8')), before = JSON.stringify(source);
  try { validatePhysicsEvidence(source); return { path, sha256:sha(path), mechanics:source.status.mechanics, admission:'accepted', unchanged:before===JSON.stringify(source) }; }
  catch (error) { return { path, sha256:sha(path), error:String(error) }; }
});
const record = {scope:'Actual producer outputs used only as reader controls, not independent physical oracles or Current authentication.', runtime:process.version, reader_sha256:sha(reader), results};
writeFileSync(join(out, 'ACTUAL_TS_AFTER.json'), JSON.stringify(record, null, 2)+'\n');
process.stdout.write(JSON.stringify(record, null, 2)+'\n');
if (results.some(result => result.error || !result.unchanged)) process.exitCode=1;
