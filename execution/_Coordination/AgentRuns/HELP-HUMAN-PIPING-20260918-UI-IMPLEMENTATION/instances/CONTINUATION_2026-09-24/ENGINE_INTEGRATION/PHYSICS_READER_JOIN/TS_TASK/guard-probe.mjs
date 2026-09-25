// Bounded source-contract probe. Run from the integration repository root with Node24.
import fs from 'node:fs';
import assert from 'node:assert/strict';
import { pathToFileURL } from 'node:url';
const root = process.cwd() + '/projects/chirality-piping/';
const { validatePhysicsEvidence } = await import(pathToFileURL(root + 'apps/desktop/src/features/results/physicsResultEvidence.ts'));
const model = JSON.parse(fs.readFileSync(root + 'core/product_physics/tests/fixtures/exact_pressure_connected_request.json')).model;
const read = mode => JSON.parse(fs.readFileSync(root + `fixtures/results/physics_connected_mechanics_${mode}.json`));
let count = 0;
for (const mode of ['sparse', 'dense']) {
  const source = read(mode), before = JSON.stringify(source);
  validatePhysicsEvidence(source, model); assert.equal(JSON.stringify(source), before);
  console.log(`PASS actual ${mode}: ${source.results.length} rows; source unchanged`); count++;
}
for (const [label, mutate] of [
  ['missing exact evidence', s => delete s.contract_evidence],
  ['duplicate case', s => s.contract_evidence.exact_cases.push(s.contract_evidence.exact_cases[0])],
  ['missing unpressurized material', s => s.contract_evidence.exact_cases[1].pipe_materials = []],
  ['cross-case region', s => s.contract_evidence.pressure[0].load_case_id = s.contract_evidence.exact_cases[1].load_case_id],
  ['missing result binding', s => s.contract_evidence.pressure[0].result_ids.pop()],
  ['duplicate result binding', s => s.contract_evidence.pressure[0].result_ids.push(s.contract_evidence.pressure[0].result_ids[0])],
  ['unknown result binding', s => s.contract_evidence.pressure[0].result_ids[0] = 'missing'],
  ['contradictory source pressure bits', s => s.contract_evidence.pressure[0].p_pa += 1],
  ['contradictory duplicate geometry', s => s.contract_evidence.pressure[0].geometry[0].As_m2 *= 2],
  ['nonfinite material', s => s.contract_evidence.exact_cases[0].pipe_materials[0].E_pa = Infinity],
  ['unsupported derived row', s => s.results[0].source_result_refs = ['invented']],
  ['source-block namespace', s => s.contract_evidence.source_block_recovery = {}],
]) {
  const source = read('sparse'); mutate(source);
  assert.throws(() => validatePhysicsEvidence(source, model));
  console.log(`PASS rejects ${label}`); count++;
}
console.log(`PASS ${count} bounded physical evidence controls`);
