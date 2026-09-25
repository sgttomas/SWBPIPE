// Independent direct validator probes. Node24 strips types; no build or UI run.
import { readFileSync, writeFileSync, existsSync } from 'node:fs';
import { dirname, join } from 'node:path';
import { fileURLToPath, pathToFileURL } from 'node:url';
import { createHash } from 'node:crypto';
let project = dirname(fileURLToPath(import.meta.url));
while (!existsSync(join(project, 'core/analysis_runs/compatibility.py'))) project = dirname(project);
const reader = join(project, 'apps/desktop/src/features/results/physicsResultEvidence.ts');
const { validatePhysicsEvidence } = await import(pathToFileURL(reader).href);
const fixture = join(project, 'fixtures/results/physics_connected_mechanics_sparse.json');
const source = JSON.parse(readFileSync(fixture, 'utf8'));
const duplicateGroup = s => { const g = s.contract_evidence.exact_cases[0].pressure_rhs_assembly.groups; g.push(structuredClone(g[0])); };
const mutations = {
  unmodified_control: s => {},
  rhs_term_coefficient_999: s => { s.contract_evidence.exact_cases[0].pressure_rhs_assembly.groups[0].terms[0].coefficient = 999; },
  rhs_pressure_moment_999: s => { s.contract_evidence.exact_cases[0].pressure_rhs_assembly.assembled_pressure_rhs_global[3] = 999; },
  duplicate_rhs_group: duplicateGroup,
  source_annulus_ro_contradiction: s => {
    s.contract_evidence.exact_cases[0].pipe_sections[0].ro_m = 0.1;
    s.contract_evidence.pressure[0].geometry[0].ro_m = 0.1;
  },
  producer_shape_thermal_consumed_with_pressure_ledger_excluding_thermal: s => {
    for (const m of [s.contract_evidence.exact_cases[0].pipe_materials[0], s.contract_evidence.pressure[0].materials[0]]) {
      m.thermal_consumed = true; m.alpha_per_kelvin = 0.000012;
    }
    // This is an admission probe of producer shape, not a new solved source.
    // Actual producer pressure_runtime.rs emits thermal_included:false in this
    // pressure-only ledger independently of thermal_consumed.
  },
  unknown_v2_row: s => {
    const row = structuredClone(s.results.find(r => r.kind.endsWith('_v2')));
    row.id = 'unknown:v2'; row.kind = 'pipe_unknown_v2'; s.results.push(row);
  },
};
const results = Object.entries(mutations).map(([name, change]) => {
  const candidate = structuredClone(source); change(candidate);
  try { validatePhysicsEvidence(candidate); return {name, admission:'accepted'}; }
  catch (e) { return {name, error:String(e)}; }
});
const sha = path => createHash('sha256').update(readFileSync(path)).digest('hex');
const report = {scope:'Direct validator only; no actual fresh producer, Current, native or export claim.', runtime:process.version, reader_sha256:sha(reader), fixture_sha256:sha(fixture), results};
const output = JSON.stringify(report, null, 2) + '\n';
if (process.argv[2]) writeFileSync(process.argv[2], output);
process.stdout.write(output);
