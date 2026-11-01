// Generator of fixtures/model_operations/run_sheet_authoring.json (DEL-07-02
// Design run-sheet-entry.md §7, slice S1). It hand-builds each expected batch
// and applied model from the design's rules (§3.3 payloads, §6.2 arithmetic,
// the engine's stated corner construction for axis-aligned 90° corners). It
// imports nothing from the desktop or the engine, so the fixture is independent
// of the builder (runSheetBatch.ts) and of the applier it is checked against.
// Every value is invented; it reads no input (`--check` reads only the fixture).
//
//   node tools/fixtures/run_sheet_authoring.mjs           rewrite the fixture
//   node tools/fixtures/run_sheet_authoring.mjs --check   exit 1 if it differs
import { readFileSync, writeFileSync } from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";
import { createHash } from "node:crypto";

const FIXTURE = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "../../fixtures/model_operations/run_sheet_authoring.json");
const check = process.argv.includes("--check");
const SOURCE = "invented run-sheet input (S1 fixture); not library, catalog or code-rule data";
const BASIS_PROV = "invented_run_sheet_basis_not_library_data";
const K_SOURCE = "invented k for the S1 fixture (user entry)";
const clone = (v) => JSON.parse(JSON.stringify(v));
// RFC 8785 for these values: sorted keys, ECMAScript numbers and strings.
const jcs = (v) => Array.isArray(v) ? `[${v.map(jcs).join(",")}]`
  : v && typeof v === "object" ? `{${Object.keys(v).sort().map((k) => `${JSON.stringify(k)}:${jcs(v[k])}`).join(",")}}`
  : JSON.stringify(v);

const m = (value) => ({ value, unit: "m" });
const SEC = { outside_diameter: m(0.1143), wall_thickness: m(0.00602) };
const node = (id, x, y, z, label) => ({ id, label, position: { x, y, z }, provenance: BASIS_PROV });
const pipe = (id, from, to, material) => ({ id, label: id.slice(5), from, to, section: clone(SEC), material, y_reference: { x: 0, y: 0, z: 1 }, provenance: BASIS_PROV });
const basis = {
  schema_version: "0.9.0",
  document_kind: "openpipestress.product_preview.model",
  project: {
    id: "project:run-sheet-authoring",
    name: "Invented run-sheet authoring basis",
    description: "Invented S1 basis for run-sheet entry (DEL-07-02 run-sheet-entry.md §7); synthetic, no library, catalog or code-rule data.",
    units: { length: "m", force: "N", angle: "rad", pressure: "Pa", stress: "Pa", temperature: "degC" },
  },
  analysis_status: { mechanics: "ready_for_preview_diagnostics", rule_check: "not_performed_user_rule_inputs_missing", professional_acceptance: "not_provided" },
  nodes: [
    node("node:A", 0, 0, 0, "A"), node("node:B", 4, 0, 0, "B"), node("node:C", 8, 0, 0, "C"), node("node:D", 8, 4, 0, "D"),
    node("node:E", 0, 6, 0, "E"), node("node:H", 0.1, 10, 0, "H"),
  ],
  pipe_segments: [pipe("pipe:AB", "node:A", "node:B", "material:steel"), pipe("pipe:BC", "node:B", "node:C", "material:steel"), pipe("pipe:CD", "node:C", "node:D", "material:alloy")],
  materials: [
    { id: "material:steel", label: "Invented steel", elastic_modulus: { value: 200000000000, unit: "Pa" }, shear_modulus: { value: 77000000000, unit: "Pa" }, provenance: BASIS_PROV },
    { id: "material:alloy", label: "Invented alloy", elastic_modulus: { value: 190000000000, unit: "Pa" }, shear_modulus: { value: 73000000000, unit: "Pa" }, provenance: BASIS_PROV },
  ],
  sections: [
    { id: "section:sch-a", name: "Invented shared pipe section A", section_type: "pipe", properties: { outside_diameter: m(0.1683), wall_thickness: m(0.00711) }, provenance: BASIS_PROV },
  ],
  supports: [{ id: "support:A", label: "A", node: "node:A", family: "anchor", restraints: ["UX", "UY", "UZ", "RX", "RY", "RZ"], provenance: BASIS_PROV }],
  components: [],
  load_cases: [{ id: "case:sustained", label: "Invented sustained case", kind: "primitive_user_load", status: "preview_only", primitive_loads: [], provenance: BASIS_PROV }],
  combinations: [],
  data_boundary: { source: BASIS_PROV },
  diagnostics: [],
};

// ---- Hand-built operations (the §3.3 envelope).
function op(rowKey, index, kind, objectType, ref, changeKind, fieldLabel, fieldPath, before, after, unit, dimension) {
  return {
    operation_id: `op:run-sheet:${rowKey}:${index}`,
    operation_kind: kind,
    operation_status: "proposed",
    author_type: "user",
    source: { source_ref: "apps/desktop/src/features/model-tree/runSheet/runSheetBatch.ts", source_channel: "local_desktop_preview", source_role: "gui_editor" },
    target: { object_type: objectType, ref },
    change: {
      change_id: `change:run-sheet:${rowKey}:${index}`,
      change_kind: changeKind, field_label: fieldLabel, field_path: fieldPath, before, after: JSON.stringify(after), unit, dimension,
      source_note: "Run-sheet row values as entered or shown inherited; no value is suggested or defaulted.",
    },
    validation: { schema_validation: "not_run", constraint_validation: "not_run", unit_validation: "not_run", diff_preview_status: "not_generated", application_status: "not_applied" },
    audit_boundary: { mutation_route: "structured_operations_only", direct_model_mutation_allowed: false, requires_user_acceptance: true, mutates_accepted_model_state: false },
    professional_boundary: { human_review_required: true, software_makes_compliance_claim: false, software_makes_certification_claim: false, software_makes_sealing_claim: false, software_makes_approval_claim: false, software_makes_authentication_claim: false },
    rationale: "Run-sheet entry: explicit user rows mapped to structured operations; requires review before apply.",
  };
}

// A hand-run "case" state: the expected model, built by the stated effects.
class Case {
  constructor(id) { this.id = id; this.ops = []; this.model = clone(basis); this.counters = {}; }
  next(row) { const n = this.counters[row] ?? 0; this.counters[row] = n + 1; return n; }
  createNode(row, id, label, x, y, z) {
    const rec = { id, label, position: { x, y, z }, provenance: SOURCE };
    this.ops.push(op(row, this.next(row), "create", "Node", id, "create_node", "Run-sheet end node", "nodes", "not_present", rec, "m", "length"));
    this.model.nodes.push(clone(rec));
  }
  connect(row, id, from, to, section, material, y) {
    const rec = { id, label: id.replace(/^[^:]*:/, ""), from, to, section, material, y_reference: y, provenance: SOURCE };
    this.ops.push(op(row, this.next(row), "connect", "Element", id, "connect_pipe_run", "Run-sheet pipe", "pipe_segments", "not_present", rec, section.outside_diameter.unit, "length"));
    this.model.pipe_segments.push(clone(rec));
  }
  assign(row, pipeId, sectionId) {
    const p = this.model.pipe_segments.find((x) => x.id === pipeId);
    this.ops.push(op(row, this.next(row), "modify", "Element", pipeId, "assign_section", "Run-sheet shared section", "section_ref", jcs(p), { section_ref: sectionId }, "none", "dimensionless"));
    const s = this.model.sections.find((x) => x.id === sectionId);
    p.section_ref = sectionId;
    p.section.outside_diameter = clone(s.properties.outside_diameter);
    p.section.wall_thickness = clone(s.properties.wall_thickness);
  }
  support(row, id, nodeId, family, restraints) {
    const rec = { id, label: id.replace(/^[^:]*:/, ""), node: nodeId, restraints, family, provenance: SOURCE };
    this.ops.push(op(row, this.next(row), "create", "Support", id, "create_support", "Run-sheet support", "supports", "not_present", rec, "none", "dimensionless"));
    this.model.supports.push({ id, label: rec.label, node: nodeId, restraints, provenance: SOURCE, family });
  }
  // The engine's construction for axis-aligned 90° corners: L_t = R, T1 = C − R·u_in, T2 = C + R·u_out.
  bend(row, corner, radius, k, { applies = true } = {}) {
    const token = corner.replace(/^[^:]*:/, "");
    const ids = { t1: `node:${token}-T1`, t2: `node:${token}-T2`, arc: `pipe:${token}-bend`, comp: `component:${token}-bend` };
    const pipes = this.model.pipe_segments.filter((p) => p.from === corner || p.to === corner);
    const c = this.model.nodes.find((n) => n.id === corner);
    const before = jcs({ node: c, pipes, markers: [] });
    const after = {
      radius: { value: radius, unit: "m" }, flexibility_factor: { value: k, unit: "none" }, flexibility_source_reference: K_SOURCE,
      new_nodes: [{ id: ids.t1, label: "Bend start", provenance: SOURCE }, { id: ids.t2, label: "Bend end", provenance: SOURCE }],
      new_pipe: { id: ids.arc, label: "Bend arc", provenance: SOURCE },
      component: { id: ids.comp, label: "Bend", provenance: SOURCE, geometry_source_reference: SOURCE },
    };
    this.ops.push(op(row, this.next(row), "modify", "Node", corner, "insert_bend_at_corner", "Run-sheet bend at corner", "bend_at_corner", before, after, "none", "dimensionless"));
    if (!applies) return;
    const [pin, pout] = pipes;
    const pos = (id) => this.model.nodes.find((n) => n.id === id).position;
    const far = (p) => (p.from === corner ? p.to : p.from);
    const unit = (a, b) => { const d = [b.x - a.x, b.y - a.y, b.z - a.z]; const n = Math.hypot(...d); return d.map((v) => v / n); };
    const C = c.position;
    const uin = unit(pos(far(pin)), C);
    const uout = unit(C, pos(far(pout)));
    for (const u of [uin, uout]) if (u.filter((v) => v !== 0).length !== 1) throw new Error("fixture bends are axis-aligned");
    const t1 = { x: C.x - radius * uin[0], y: C.y - radius * uin[1], z: C.z - radius * uin[2] };
    const t2 = { x: C.x + radius * uout[0], y: C.y + radius * uout[1], z: C.z + radius * uout[2] };
    const arc = clone(pin);
    Object.assign(arc, { id: ids.arc, label: "Bend arc", provenance: SOURCE, from: ids.t1, to: ids.t2, y_reference: { x: uin[0], y: uin[1], z: uin[2] } });
    this.model.nodes = this.model.nodes.filter((n) => n.id !== corner);
    this.model.nodes.push({ id: ids.t1, label: "Bend start", provenance: SOURCE, position: t1 }, { id: ids.t2, label: "Bend end", provenance: SOURCE, position: t2 });
    for (const [p, to] of [[pin, ids.t1], [pout, ids.t2]]) for (const end of ["from", "to"]) if (p[end] === corner) p[end] = to;
    this.model.pipe_segments.push(arc);
    this.model.components.push({
      id: ids.comp, label: "Bend", kind: "bend", node: ids.t1,
      geometry: { bend_pipe_ref: ids.arc, bend_radius: { value: radius, unit: "m" }, bend_geometry_source_reference: SOURCE },
      modifiers: { flexibility_factor_user_value: { value: k, unit: "none" }, source_reference: K_SOURCE },
      mechanics_interface: { solver_consumption: "curved_bend_macro_element" },
      provenance: SOURCE,
    });
  }
  batch() { return { batch_id: `batch:run-sheet-${this.id}-1`, operations: this.ops }; }
}

const Z = { x: 0, y: 0, z: 1 };
const inline = () => clone(SEC);
const shared = () => ({ outside_diameter: m(0.1683), wall_thickness: m(0.00711) });
const cases = [];
const sha = (model) => `sha256:${createHash("sha256").update(jcs(model), "utf8").digest("hex")}`;
const add = (id, description, sheet, built, extra = {}) => cases.push({ id, description, sheet: { sheetKey: id, batchSeq: 1, source: SOURCE, ...sheet }, expected_batch: built.batch(),
  expected_applied_model: extra.refused ? null : built.model, expected_applied_model_sha256: extra.refused ? null : sha(built.model), ...extra });

{ // 1. Straight chain from a stored node with no pipes, ending in an anchor.
  const c = new Case("straight-chain");
  c.createNode("r2", "node:N20", "N20", 2, 6, 0); c.connect("r2", "pipe:P20", "node:E", "node:N20", inline(), "material:steel", Z);
  c.createNode("r3", "node:N30", "Corner", 2, 9, 0); c.connect("r3", "pipe:P30", "node:N20", "node:N30", inline(), "material:steel", Z);
  c.createNode("r4", "node:N40", "N40", 2, 9, -1.5); c.connect("r4", "pipe:P40", "node:N30", "node:N40", inline(), "material:steel", { x: 0, y: 1, z: 0 });
  c.support("r4", "support:N40", "node:N40", "anchor", ["UX", "UY", "UZ", "RX", "RY", "RZ"]);
  add("straight-chain", "F from a stored node with no pipes; three runs; Orientation re-entered where the run turns vertical; an anchor with `*` restraints.", { rows: [
    { key: "r1", kind: "F", node: "node:E" },
    { key: "r2", dx: "2", od: "0.1143", wall: "0.00602", material: "material:steel", orientation: "0 0 1" },
    { key: "r3", dy: "3", label: "Corner" },
    { key: "r4", dz: "-1.5", orientation: "0 1 0", support: "A", restraints: "*" },
  ] }, c);
}
const bendRows = (radii) => [
  { key: "r1", kind: "F", node: "node:E" },
  { key: "r2", kind: "B", dx: "3", od: "0.1143", wall: "0.00602", material: "material:steel", orientation: "0 0 1", bendRadius: radii[0], k: "1.5", kSource: K_SOURCE },
  { key: "r3", kind: "B", dy: "3", ...(radii[1] ? { bendRadius: radii[1] } : {}), k: "1.5", kSource: K_SOURCE },
  { key: "r4", dz: "3", orientation: "1 0 0" },
];
{ // 2. Two consecutive bends.
  const c = new Case("two-bend-chain");
  c.createNode("r2", "node:N20", "N20", 3, 6, 0); c.connect("r2", "pipe:P20", "node:E", "node:N20", inline(), "material:steel", Z);
  c.createNode("r3", "node:N30", "N30", 3, 9, 0); c.connect("r3", "pipe:P30", "node:N20", "node:N30", inline(), "material:steel", Z);
  c.bend("r2", "node:N20", 0.5, 1.5);
  c.createNode("r4", "node:N40", "N40", 3, 9, 3); c.connect("r4", "pipe:P40", "node:N30", "node:N40", inline(), "material:steel", { x: 1, y: 0, z: 0 });
  c.bend("r3", "node:N30", 0.5, 1.5);
  add("two-bend-chain", "Two consecutive bend runs; Bend R inherited by the second bend; k entered on each.", { rows: bendRows(["0.5"]) }, c);
}
{ // 3. Three consecutive bends on a shared section.
  const c = new Case("three-bend-chain");
  const k = (row) => ({ k: "2", kSource: K_SOURCE, ...row });
  c.createNode("r2", "node:N20", "N20", 3, 6, 0); c.connect("r2", "pipe:P20", "node:E", "node:N20", shared(), "material:steel", Z); c.assign("r2", "pipe:P20", "section:sch-a");
  c.createNode("r3", "node:N30", "N30", 3, 9, 0); c.connect("r3", "pipe:P30", "node:N20", "node:N30", shared(), "material:steel", Z); c.assign("r3", "pipe:P30", "section:sch-a");
  c.bend("r2", "node:N20", 0.5, 2);
  c.createNode("r4", "node:N40", "N40", 3, 9, 3); c.connect("r4", "pipe:P40", "node:N30", "node:N40", shared(), "material:steel", { x: 1, y: 0, z: 0 }); c.assign("r4", "pipe:P40", "section:sch-a");
  c.bend("r3", "node:N30", 0.5, 2);
  c.createNode("r5", "node:N50", "N50", 0, 9, 3); c.connect("r5", "pipe:P50", "node:N40", "node:N50", shared(), "material:steel", { x: 0, y: 1, z: 0 }); c.assign("r5", "pipe:P50", "section:sch-a");
  c.bend("r4", "node:N40", 0.5, 2);
  add("three-bend-chain", "Three consecutive bend runs on a shared section; each corner context carries the earlier bends' retargeted ends and section_ref.", { rows: [
    { key: "r1", kind: "F", node: "node:E" },
    k({ key: "r2", kind: "B", dx: "3", section: "section:sch-a", material: "material:steel", orientation: "0 0 1", bendRadius: "0.5" }),
    k({ key: "r3", kind: "B", dy: "3" }),
    k({ key: "r4", kind: "B", dz: "3", orientation: "1 0 0" }),
    { key: "r5", dx: "-3", orientation: "0 1 0" },
  ] }, c);
}
{ // 4. Shared-section run.
  const c = new Case("shared-section-run");
  c.createNode("r2", "node:N20", "N20", 2, 6, 0); c.connect("r2", "pipe:P20", "node:E", "node:N20", shared(), "material:steel", Z); c.assign("r2", "pipe:P20", "section:sch-a");
  c.createNode("r3", "node:N30", "N30", 2, 8, 0); c.connect("r3", "pipe:P30", "node:N20", "node:N30", shared(), "material:steel", Z); c.assign("r3", "pipe:P30", "section:sch-a");
  add("shared-section-run", "A Section entered once and inherited; each pipe is connected with the section's OD and wall, then assigned.", { rows: [
    { key: "r1", kind: "F", node: "node:E" },
    { key: "r2", dx: "2", section: "section:sch-a", material: "material:steel", orientation: "0 0 1" },
    { key: "r3", dy: "2" },
  ] }, c);
}
{ // 5. Insulation ended by `-`.
  const c = new Case("insulation-ended");
  const kg = (value) => ({ value, unit: "kg/m^3" });
  const ins = () => ({ ...inline(), insulation_thickness: m(0.05), insulation_density: kg(130), contents_density: kg(1000) });
  const bare = () => ({ ...inline(), contents_density: kg(1000) });
  c.createNode("r2", "node:N20", "N20", 2, 6, 0); c.connect("r2", "pipe:P20", "node:E", "node:N20", ins(), "material:steel", Z);
  c.createNode("r3", "node:N30", "N30", 2, 8, 0); c.connect("r3", "pipe:P30", "node:N20", "node:N30", ins(), "material:steel", Z);
  c.createNode("r4", "node:N40", "N40", 0, 8, 0); c.connect("r4", "pipe:P40", "node:N30", "node:N40", bare(), "material:steel", Z);
  c.createNode("r5", "node:N50", "N50", 0, 7, 0); c.connect("r5", "pipe:P50", "node:N40", "node:N50", bare(), "material:steel", Z);
  add("insulation-ended", "Insulation entered, inherited, ended by `-` and the absence inherited; contents density continues.", {
    columnUnits: { insulationDensity: "kg/m^3", contentsDensity: "kg/m^3" },
    rows: [
      { key: "r1", kind: "F", node: "node:E" },
      { key: "r2", dx: "2", od: "0.1143", wall: "0.00602", material: "material:steel", orientation: "0 0 1", insulationThickness: "0.05", insulationDensity: "130", contentsDensity: "1000" },
      { key: "r3", dy: "2" },
      { key: "r4", dx: "-2", insulationThickness: "-", insulationDensity: "-" },
      { key: "r5", dy: "-1" },
    ] }, c);
}
{ // 6. F branch, stored pipes agreeing.
  const c = new Case("f-branch-agreeing");
  c.createNode("r2", "node:N20", "Branch end", 4, 2, 0); c.connect("r2", "pipe:P20", "node:B", "node:N20", inline(), "material:steel", Z);
  c.support("r2", "support:N20", "node:N20", "guide", ["UX", "UZ"]);
  add("f-branch-agreeing", "F from a stored node whose pipes agree: every element field is inherited from pipe:AB; a guide with typed lower-case restraints.", { rows: [
    { key: "r1", kind: "F", node: "node:B" },
    { key: "r2", dy: "2", label: "Branch end", support: "G", restraints: "ux, uz" },
  ] }, c);
}
{ // 7. F branch, stored pipes disagreeing in material.
  const c = new Case("f-branch-disagreeing");
  c.createNode("r2", "node:N20", "N20", 10, 0, 0); c.connect("r2", "pipe:P20", "node:C", "node:N20", inline(), "material:steel", Z);
  add("f-branch-disagreeing", "F from a stored node whose pipes differ in material: Material is blank and entered on the run; the agreeing fields are inherited.", { rows: [
    { key: "r1", kind: "F", node: "node:C" },
    { key: "r2", dx: "2", material: "material:steel" },
  ] }, c);
}
{ // 8. P start.
  const c = new Case("point-start");
  c.createNode("r1", "node:N10", "Anchor point", 10, -4, 0);
  c.support("r1", "support:N10", "node:N10", "anchor", ["UX", "UY", "UZ", "RX", "RY", "RZ"]);
  c.createNode("r2", "node:N20", "N20", 13, -4, 0); c.connect("r2", "pipe:P20", "node:N10", "node:N20", inline(), "material:alloy", { x: 0, y: 1, z: 0 });
  add("point-start", "P at an absolute point with an anchor; a run from it with every field entered.", { rows: [
    { key: "r1", kind: "P", dx: "10", dy: "-4", dz: "0", label: "Anchor point", support: "A", restraints: "UX UY UZ RX RY RZ" },
    { key: "r2", dx: "3", od: "0.1143", wall: "0.00602", material: "material:alloy", orientation: "0 1 0" },
  ] }, c);
}
{ // 9. Run to an existing node.
  const c = new Case("run-to-existing-node");
  c.createNode("r1", "node:N10", "N10", 0, -3, 0);
  c.connect("r2", "pipe:P20", "node:N10", "node:A", inline(), "material:steel", Z);
  add("run-to-existing-node", "A run whose Node names a stored node: no create_node; the pipe ends on node:A.", { rows: [
    { key: "r1", kind: "P", dx: "0", dy: "-3", dz: "0" },
    { key: "r2", node: "node:A", od: "0.1143", wall: "0.00602", material: "material:steel", orientation: "0 0 1" },
  ] }, c);
}
{ // 10. Non-identity units: sheet in, model m. x = 0.1 + c(0.3 in), c the facade's convert_for_dimension.
  const c = new Case("inch-increments");
  const cin = (v) => v * 0.0254; // ((v·f_e + 0) − 0) / 1 for in → m
  const x = 0.1 + cin(0.3);
  if (x !== 0.10762000000000001) throw new Error(`unexpected ${x}`);
  c.createNode("r2", "node:N20", "N20", x, 10, 0); c.connect("r2", "pipe:P20", "node:H", "node:N20", inline(), "material:steel", Z);
  c.createNode("r3", "node:N30", "N30", x, 10 + cin(12), 0); c.connect("r3", "pipe:P30", "node:N20", "node:N30", inline(), "material:steel", Z);
  add("inch-increments", "Sheet unit in, model unit m: each increment is converted, then added (0.1 m + 0.3 in = 0.10762000000000001 m; absolute entry would give 0.10762000000000002 m).", {
    lengthUnit: "in",
    rows: [
      { key: "r1", kind: "F", node: "node:H" },
      { key: "r2", dx: "0.3", od: "0.1143", wall: "0.00602", material: "material:steel", orientation: "0 0 1" },
      { key: "r3", dy: "12" },
    ] }, c, { arithmetic: { predecessor_m: 0.1, increment_in: 0.3, converted_then_added_m: x, absolute_entry_m: 0.10762000000000002 } });
}
{ // 11. A -0 entry.
  const c = new Case("negative-zero");
  c.createNode("r1", "node:N10", "N10", 0, 12, 0);
  c.createNode("r2", "node:N20", "N20", 0, 12, 1.5); c.connect("r2", "pipe:P20", "node:N10", "node:N20", inline(), "material:steel", { x: 1, y: 0, z: 0 });
  add("negative-zero", "Typed -0 in a P coordinate and in run increments: every computed coordinate is normalized to +0 before display, projection and emission.", { rows: [
    { key: "r1", kind: "P", dx: "-0", dy: "12", dz: "-0" },
    { key: "r2", dx: "-0", dy: "-0", dz: "1.5", od: "0.1143", wall: "0.00602", material: "material:steel", orientation: "1 0 0" },
  ] }, c);
}
{ // 12. The last step refused: radius too large at the final corner.
  const c = new Case("refused-last-step");
  c.createNode("r2", "node:N20", "N20", 3, 6, 0); c.connect("r2", "pipe:P20", "node:E", "node:N20", inline(), "material:steel", Z);
  c.createNode("r3", "node:N30", "N30", 3, 9, 0); c.connect("r3", "pipe:P30", "node:N20", "node:N30", inline(), "material:steel", Z);
  c.bend("r2", "node:N20", 0.5, 1.5);
  c.createNode("r4", "node:N40", "N40", 3, 9, 3); c.connect("r4", "pipe:P40", "node:N30", "node:N40", inline(), "material:steel", { x: 1, y: 0, z: 0 });
  c.bend("r3", "node:N30", 5, 1.5, { applies: false });
  add("refused-last-step", "Two bends; the second radius (5 m) needs more straight than either pipe at node:N30 has, so the batch's last step is refused and nothing is published.", { rows: bendRows(["0.5", "5"]) }, c, {
    refused: true,
    expected_refusal: { simulation_disposition: "rolled_back_no_model_published", step_index: c.ops.length - 1, code: "OP-BEND-CORNER-RADIUS-INVALID", row: "r3" },
  });
}

const fixture = {
  fixture_kind: "openpipestress.fixture.run_sheet_authoring",
  description: "Run-sheet entry S1 (DEL-07-02 Design run-sheet-entry.md §7): sheets, hand-built expected batches and expected applied models over one invented basis model. Every row is invented, synthetic data; no section, material, radius or k is library, catalog or code-rule data.",
  data_boundary: "invented_synthetic_fixture_not_library_data",
  consumers: [
    "apps/desktop/src/features/model-tree/runSheet/runSheet.test.ts",
    "core/model_operations/operation_applier/tests/run_sheet_authoring.rs",
  ],
  generator: "tools/fixtures/run_sheet_authoring.mjs",
  id_assignment: "Each sheet's rows take numbers in row order from a fresh counter (node:N, pipe:P, start 10, step 10), skipping numbers that yield an id of the basis model.",
  basis_model: basis,
  cases,
};
const text = JSON.stringify(fixture, null, 2) + "\n";
if (check) {
  const current = readFileSync(FIXTURE, "utf8");
  if (current !== text) {
    console.error(`${path.relative(process.cwd(), FIXTURE)} differs from the generator's output`);
    process.exit(1);
  }
  console.log(`${cases.length} cases current`);
} else {
  writeFileSync(FIXTURE, text);
  console.log(`wrote ${cases.length} cases`);
}
