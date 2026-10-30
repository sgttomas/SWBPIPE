import assert from "node:assert/strict";
import { createHash } from "node:crypto";
import { readFile } from "node:fs/promises";
import path from "node:path";
import { fileURLToPath } from "node:url";

const root = path.dirname(fileURLToPath(import.meta.url));
const manifestBytes = await readFile(path.join(root, "fixture-manifest.json"));
const manifest = JSON.parse(manifestBytes);

function hash(bytes) {
  return createHash("sha256").update(bytes).digest("hex");
}

for (const entry of manifest.files) {
  const bytes = await readFile(path.join(root, entry.path));
  assert.equal(bytes.length, entry.bytes, `${entry.path}: byte count`);
  assert.equal(hash(bytes), entry.sha256, `${entry.path}: sha256`);
}

for (const fixture of manifest.fixtures) {
  const model = JSON.parse(await readFile(path.join(root, fixture.model_path), "utf8"));
  const overlay = JSON.parse(await readFile(path.join(root, fixture.overlay_path), "utf8"));
  const samples = JSON.parse(await readFile(path.join(root, fixture.samples_path), "utf8"));
  const expected = fixture.pipe_count;
  const primitiveLoads = model.load_cases.flatMap((loadCase) => loadCase.primitive_loads ?? []);
  assert.equal(model.nodes.length, expected + 1, `${expected}: N+1 nodes`);
  assert.equal(model.pipe_segments.length, expected, `${expected}: N pipes`);
  assert.equal(model.supports.length, expected / 20, `${expected}: N/20 supports`);
  assert.equal(model.components.length, expected / 100, `${expected}: N/100 components`);
  assert.equal(primitiveLoads.length, expected / 10, `${expected}: N/10 nodal arrows`);
  assert.equal(overlay.nodes.length, expected + 1, `${expected}: overlay node count`);
  assert.equal(overlay.status, "generated_ui_data_only_not_solver_output");
  assert.equal(samples.point_selection.length, 200, `${expected}: selection samples`);
  assert.equal(samples.box_selection.length, 20, `${expected}: box samples`);
  assert.equal(samples.tree_filters.length, 20, `${expected}: filter samples`);
  assert.deepEqual(
    Object.fromEntries(Object.entries(samples.selected_sets).map(([key, values]) => [key, values.length])),
    fixture.selected_set_sizes,
    `${expected}: selection-set sizes`
  );
  assert.equal(new Set(model.nodes.map((node) => node.id)).size, model.nodes.length, `${expected}: node ids unique within type`);
  assert.equal(new Set(model.pipe_segments.map((pipe) => pipe.id)).size, model.pipe_segments.length, `${expected}: pipe ids unique within type`);
  const nodeIds = new Set(model.nodes.map((node) => node.id));
  for (const pipe of model.pipe_segments) {
    assert(nodeIds.has(pipe.from), `${expected}: ${pipe.id} from reference`);
    assert(nodeIds.has(pipe.to), `${expected}: ${pipe.id} to reference`);
  }
  const sectionById = new Map(model.sections.map((section) => [section.id, section]));
  for (const pipe of model.pipe_segments) {
    const section = sectionById.get(pipe.section_ref);
    assert(section, `${expected}: ${pipe.id} section_ref`);
    assert.deepEqual(pipe.section.outside_diameter, section.properties.outside_diameter, `${expected}: ${pipe.id} OD binding`);
    assert.deepEqual(pipe.section.wall_thickness, section.properties.wall_thickness, `${expected}: ${pipe.id} wall binding`);
  }
  assert.equal(model.data_boundary.solver_capacity_claim, "none_ui_workload_only");
}

const base = JSON.parse(await readFile(path.join(root, manifest.precision_regression.base_path), "utf8"));
const translated = JSON.parse(await readFile(path.join(root, manifest.precision_regression.translated_path), "utf8"));
const precision = JSON.parse(await readFile(path.join(root, manifest.precision_regression.expectations_path), "utf8"));
assert.equal(base.nodes.length, translated.nodes.length);
for (let index = 0; index < base.nodes.length; index += 1) {
  assert.equal(base.nodes[index].id, translated.nodes[index].id);
  assert.deepEqual(
    ["x", "y", "z"].map((axis, axisIndex) => Number((translated.nodes[index].position[axis] - base.nodes[index].position[axis] - precision.translation_project_length_units[axisIndex]).toFixed(6))),
    [0, 0, 0],
    `precision node ${index}: exact translation`
  );
}

process.stdout.write(`${JSON.stringify({
  status: "PASS",
  manifest_sha256: hash(manifestBytes),
  bound_file_count: manifest.files.length,
  fixture_pipe_counts: manifest.fixtures.map((fixture) => fixture.pipe_count)
})}\n`);
