import { createHash } from "node:crypto";
import { mkdir, readFile, stat, writeFile } from "node:fs/promises";
import path from "node:path";
import { fileURLToPath } from "node:url";
import { expectedHitForProbe, fixtureCamera, projectPointToNdc } from "./point-hit-oracle.mjs";

const ROOT = path.dirname(fileURLToPath(import.meta.url));
const FIXTURE_DIR = path.join(ROOT, "fixtures");
const SAMPLE_DIR = path.join(ROOT, "samples");
const SEED = 20260915;
const SIZES = [1_000, 10_000];
const COLLISION_INDEX = 1_000;
const PROVENANCE = "invented_generated_ui_benchmark_seed_20260915_no_standard_or_catalog_data";
const POINT_HIT_POLICY_SHA256 = "bc0cb19948bec94fedddec4b8ea21005f32d011a16ca00e3d78c2ac9bb19e63c";
const OBSERVABILITY_ATTACHMENT_SHA256 = "ef009d11139fc5dab4639586092a44741ced49ac80d7108f22eecd264a6b5c3a";
const OBSERVER_BINDING_SHA256 = "8ab87d0d0d9088df085cecef7e88c4d6712b179c93dc2c6172197d4f5e9b2fa4";
const BOX_SELECTION_POLICY_SHA256 = "8195cd971146d337323dd791992884ce670b766f29bbdde6abd76b82da73b310";
const UI_TEST_BINDINGS_SHA256 = "12ea9947ff0251713b733e980f8ea7e5c536069f35a86791d82cd6300bebfbaa";
const BASELINE_CANVAS_CSS = [919, 628];

// Invocation (first-profile repair of 2026-09; see REPAIR_2026-09_FIRST_PROFILE.md):
//   node generate-fixtures.mjs --check [--protocol-history-dir <absolute dir>]
//     regenerates every frozen file in memory, compares it by SHA-256 and byte count with the
//     frozen bytes on disk, and WRITES NOTHING. This is the only mode the frozen first profile needs.
//   node generate-fixtures.mjs [--protocol-history-dir <absolute dir>]
//     the original writing mode. It needs the nine preserved protocol-history files, and it now
//     verifies them BEFORE the first write, so a missing history can no longer leave a partial write.
// The original closeout kept the nine preimages as evidence, not packaged method source.
// Recovered run evidence can now be supplied explicitly with --protocol-history-dir; the default
// location remains supported for checkouts that hold it. No pinned hash below has changed.
const ARGS = process.argv.slice(2);
const CHECK = ARGS.includes("--check");
const HISTORY_FLAG = "--protocol-history-dir";
const checkFindings = [];
function protocolHistoryArgument() {
  const rest = ARGS.filter((arg) => arg !== "--check");
  if (rest.length === 0) return null;
  if (rest.length !== 2 || rest[0] !== HISTORY_FLAG || !path.isAbsolute(rest[1])) {
    throw new Error(`usage: generate-fixtures.mjs [--check] [${HISTORY_FLAG} <absolute directory>]`);
  }
  return rest[1];
}

function round(value, places = 6) {
  const scale = 10 ** places;
  return Math.round(value * scale) / scale;
}

function pad(value, width = 5) {
  return String(value).padStart(width, "0");
}

function seededRandom(seed) {
  let state = seed >>> 0;
  return () => {
    state = (state + 0x6d2b79f5) >>> 0;
    let value = state;
    value = Math.imul(value ^ (value >>> 15), value | 1);
    value ^= value + Math.imul(value ^ (value >>> 7), value | 61);
    return ((value ^ (value >>> 14)) >>> 0) / 4294967296;
  };
}

function entityId(kind, index) {
  if (index === COLLISION_INDEX && ["node", "pipe", "support", "component"].includes(kind)) {
    return "entity:UIF-TYPED-COLLISION-01000";
  }
  return `${kind}:UIF-${pad(index)}`;
}

function latticePosition(index, random) {
  if (index === 0) return { x: 0, y: 0, z: 0 };
  const rowLength = 101;
  const layerRows = 10;
  const row = Math.floor(index / rowLength);
  const within = index % rowLength;
  const xIndex = row % 2 === 0 ? within : rowLength - 1 - within;
  const yIndex = row % layerRows;
  const zIndex = Math.floor(row / layerRows);
  const jitterY = (random() - 0.5) * 0.018;
  const jitterZ = (random() - 0.5) * 0.018;
  return {
    x: round(xIndex * 0.1),
    y: round(yIndex * 0.14 + jitterY),
    z: round(zIndex * 0.11 + jitterZ)
  };
}

function nodeParentIndex(index) {
  if (index === 0) return null;
  // A regular back-edge creates explicit branches while preserving one connected,
  // acyclic N-edge/N+1-node graph. The intervening predecessor becomes a leaf.
  return index % 64 === 0 ? index - 17 : index - 1;
}

function buildModel(pipeCount) {
  const random = seededRandom(SEED ^ pipeCount);
  const nodes = Array.from({ length: pipeCount + 1 }, (_, index) => ({
    id: entityId("node", index),
    label: `UI benchmark node ${pad(index)}`,
    position: latticePosition(index, random),
    provenance: PROVENANCE
  }));
  const sections = [
    ["section:UIF-OD-060", "Invented OD 60 mm", 0.06, 0.004],
    ["section:UIF-OD-090", "Invented OD 90 mm", 0.09, 0.006],
    ["section:UIF-OD-120", "Invented OD 120 mm", 0.12, 0.008],
    ["section:UIF-OD-180", "Invented OD 180 mm", 0.18, 0.01]
  ].map(([id, name, outsideDiameter, wallThickness]) => ({
    id,
    name,
    section_type: "pipe",
    properties: {
      outside_diameter: { value: outsideDiameter, unit: "m" },
      wall_thickness: { value: wallThickness, unit: "m" }
    },
    provenance: PROVENANCE
  }));
  const sectionById = new Map(sections.map((section) => [section.id, section]));
  const pipeSegments = Array.from({ length: pipeCount }, (_, zeroIndex) => {
    const index = zeroIndex + 1;
    const sectionRef = sections[(index - 1) % sections.length].id;
    const section = sectionById.get(sectionRef);
    const parentIndex = nodeParentIndex(index);
    return {
      id: entityId("pipe", index),
      label: `UI benchmark pipe ${pad(index)}`,
      from: entityId("node", parentIndex),
      to: entityId("node", index),
      section: {
        outside_diameter: { ...section.properties.outside_diameter },
        wall_thickness: { ...section.properties.wall_thickness }
      },
      section_ref: sectionRef,
      material: "material:UIF-INVENTED-01",
      y_reference: { x: 0, y: 1, z: 0 },
      provenance: PROVENANCE
    };
  });
  const supports = Array.from({ length: pipeCount / 20 }, (_, zeroIndex) => {
    const index = (zeroIndex + 1) * 20;
    return {
      id: entityId("support", index),
      label: `UI benchmark support ${pad(index)}`,
      node: entityId("node", index),
      family: zeroIndex % 4 === 0 ? "anchor" : "guide",
      restraints: zeroIndex % 4 === 0 ? ["UX", "UY", "UZ", "RX", "RY", "RZ"] : ["UY", "UZ"],
      provenance: PROVENANCE
    };
  });
  const components = Array.from({ length: pipeCount / 100 }, (_, zeroIndex) => {
    const index = (zeroIndex + 1) * 100;
    return {
      id: entityId("component", index),
      label: `UI benchmark valve ${pad(index)}`,
      kind: "valve",
      node: entityId("node", index),
      geometry: {
        rigid_pipe_ref: entityId("pipe", index),
        rigid_body_length: { value: 0.24, unit: "m" },
        end_a_size: { value: 0.09, unit: "m" },
        end_b_size: { value: 0.09, unit: "m" },
        weight: { value: 125, unit: "N" },
        center_of_gravity: { x: 0, y: 0.04, z: 0, unit: "m" },
        connection_end_a_reference: "invented_ui_benchmark_end_a",
        connection_end_b_reference: "invented_ui_benchmark_end_b",
        stiffness_behavior_reference: "invented_ui_benchmark_display_only",
        rigid_component_source_reference: PROVENANCE
      },
      modifiers: {
        linear_stiffness_user_value: { value: 12000000, unit: "N/m" },
        rotational_stiffness_user_value: { value: 750000, unit: "N*m/rad" },
        source_reference: PROVENANCE
      },
      mechanics_interface: {
        solver_consumption: "not_asserted_ui_workload_only",
        rule_check_consumption: "not_asserted_ui_workload_only"
      },
      provenance: PROVENANCE
    };
  });
  const primitiveLoads = Array.from({ length: pipeCount / 10 }, (_, zeroIndex) => {
    const index = (zeroIndex + 1) * 10;
    return {
      id: `primitive:UIF-NODAL-FORCE-${pad(index)}`,
      category: "occasional",
      target: { type: "node", node: entityId("node", index) },
      direction: ["global_x", "global_y", "global_z"][zeroIndex % 3],
      magnitude: { value: 100 + (zeroIndex % 17) * 5, unit: "N" },
      dimension: "force",
      provenance: PROVENANCE
    };
  });
  return {
    schema_version: "0.2.0",
    document_kind: "openpipestress.product_preview.model",
    data_boundary: {
      public_examples_policy: "invented_generated_benchmark_data_only",
      protected_source_policy: "no_bundled_protected_owner_or_standards_data",
      private_data_policy: "no_private_project_data",
      solver_capacity_claim: "none_ui_workload_only",
      professional_boundary: "generated_ui_workload_not_for_engineering_reliance"
    },
    project: {
      id: `project:UIF-${pipeCount}`,
      name: `Generated UI foundation ${pipeCount}-pipe model`,
      description: "Deterministic branched and densely overlapping UI workload; not a solver-capacity fixture.",
      units: {
        length: "m",
        force: "N",
        angle: "rad",
        pressure: "Pa",
        temperature: "degC",
        stress: "MPa"
      }
    },
    analysis_status: {
      mechanics: "not_run_generated_ui_workload",
      rule_check: "not_run_generated_ui_workload",
      professional_acceptance: "not_provided"
    },
    materials: [{
      id: "material:UIF-INVENTED-01",
      label: "Invented UI benchmark elastic material",
      elastic_modulus: { value: 200000000000, unit: "Pa" },
      shear_modulus: { value: 77000000000, unit: "Pa" },
      thermal_expansion_coefficient: { value: 0.000012, unit: "1/degC" },
      provenance: PROVENANCE
    }],
    sections,
    nodes,
    pipe_segments: pipeSegments,
    supports,
    components,
    load_cases: [{
      id: "load-case:UIF-DISPLAY-01",
      label: "Generated UI nodal arrow workload",
      kind: "primitive_user_load",
      status: "generated_ui_data_not_solved",
      primitive_loads: primitiveLoads,
      provenance: PROVENANCE
    }],
    combinations: [],
    diagnostics: [{
      id: "diagnostic:UIF-WORKLOAD-ONLY",
      code: "UI_BENCHMARK_WORKLOAD_ONLY",
      severity: "info",
      message: "This deterministic generated model exercises UI rendering and interaction only; it is not solver-capacity or engineering-validation evidence.",
      provenance: PROVENANCE
    }]
  };
}

function buildOverlay(model) {
  return {
    schema: "openpipestress.ui-foundation.generated-deformation-overlay/v1",
    fixture_ref: model.project.id,
    seed: SEED,
    status: "generated_ui_data_only_not_solver_output",
    unit: model.project.units.length,
    publication_boundary: "Offsets exercise display ownership and lifecycle only; they are not computed mechanics or engineering evidence.",
    nodes: model.nodes.map((node, index) => ({
      node_ref: node.id,
      delta: {
        x: round(Math.sin(index * 0.013) * 0.0025, 9),
        y: round((index / Math.max(1, model.nodes.length - 1)) ** 2 * 0.035, 9),
        z: round(Math.cos(index * 0.017) * 0.0015, 9)
      }
    }))
  };
}

function sampleUniqueIndices(count, maximum, seed) {
  const random = seededRandom(seed);
  const chosen = new Set();
  while (chosen.size < count) {
    const index = 1 + Math.floor(random() * maximum);
    if (index !== COLLISION_INDEX) chosen.add(index);
  }
  return [...chosen];
}

function buildSamples(pipeCount) {
  const pointIndices = sampleUniqueIndices(200, pipeCount, SEED ^ pipeCount ^ 0x51ec7);
  const boxes = Array.from({ length: 20 }, (_, index) => {
    const leftToRight = index % 2 === 0;
    const xA = round(0.08 + (index % 5) * 0.13, 4);
    const yA = round(0.1 + Math.floor(index / 5) * 0.16, 4);
    const xB = round(Math.min(0.94, xA + 0.21 + (index % 3) * 0.04), 4);
    const yB = round(Math.min(0.9, yA + 0.18 + (index % 2) * 0.05), 4);
    return {
      sample: index + 1,
      direction: leftToRight ? "left_to_right_contained" : "right_to_left_intersecting",
      filter: ["all", "pipes", "nodes", "supports", "components"][index % 5],
      start_normalized: leftToRight ? [xA, yA] : [xB, yA],
      end_normalized: leftToRight ? [xB, yB] : [xA, yB]
    };
  });
  const filters = [
    "UI benchmark pipe 00001", "UI benchmark pipe 00100", "UI benchmark pipe 00500",
    "UI benchmark node 00001", "UI benchmark node 00100", "UI benchmark node 00500",
    "UI benchmark support", "UI benchmark valve", "section:UIF-OD-060", "section:UIF-OD-180",
    "pipe:UIF-00077", "node:UIF-00088", "support:UIF-00120", "component:UIF-00200",
    "generated UI nodal arrow", "invented OD 90", "invented elastic material",
    "no-match-ui-foundation", "UIF-00", "generated ui"
  ];
  const selectedSetIndices = {
    empty: [],
    one_pipe: [pointIndices[0]],
    one_hundred_pipes: pointIndices.slice(0, 100),
    all_pipes: Array.from({ length: pipeCount }, (_, index) => index + 1)
  };
  return {
    schema: "openpipestress.ui-foundation.interaction-samples/v2",
    fixture_pipe_count: pipeCount,
    seed: SEED,
    point_selection: pointIndices.map((index, sample) => ({
      sample: sample + 1,
      probe_anchor_ref: { type: "pipe", id: entityId("pipe", index) },
      anchor: "pipe_centerline_midpoint",
      untimed_precondition: "activate the real project tree row and observe exclusive typed Project selection before this gesture",
      action: "real_canvas_pointer_pick_labels_off",
      presented_feedback: ["selection_membership", "primary_highlight", "property_identity", "browser_capture_post_clock_upper_bound"]
    })),
    box_selection: boxes,
    tree_filters: filters.map((query, index) => ({ sample: index + 1, query })),
    selected_sets: Object.fromEntries(Object.entries(selectedSetIndices).map(([name, indices]) => [
      name,
      indices.map((index) => entityId("pipe", index))
    ])),
    orbit_pointer_path: {
      warmup_ms: 2_000,
      measured_ms: 10_000,
      event_rate_hz: 60,
      normalized_path: "x=0.5+0.31*sin(2*pi*t/2400ms); y=0.5+0.23*sin(2*pi*t/1700ms+pi/3)",
      button: "left",
      expected_control: "orbit"
    }
  };
}

function pointAnchor(model, anchorRef) {
  const pipe = model.pipe_segments.find((candidate) => candidate.id === anchorRef.id);
  if (!pipe) throw new Error(`missing probe anchor pipe ${anchorRef.id}`);
  const from = model.nodes.find((node) => node.id === pipe.from)?.position;
  const to = model.nodes.find((node) => node.id === pipe.to)?.position;
  if (!from || !to) throw new Error(`unresolved probe anchor endpoints ${anchorRef.id}`);
  return [round((from.x + to.x) / 2, 12), round((from.y + to.y) / 2, 12), round((from.z + to.z) / 2, 12)];
}

function pointOracle(model, samples) {
  const aspect = BASELINE_CANVAS_CSS[0] / BASELINE_CANVAS_CSS[1];
  const baselineCamera = fixtureCamera("baseline_iso", aspect);
  const candidateNominalCamera = fixtureCamera("candidate_overview_iso", aspect);
  const probes = samples.point_selection.map((sample) => {
    const authoredAnchor = pointAnchor(model, sample.probe_anchor_ref);
    const baselineProjection = projectPointToNdc(baselineCamera, authoredAnchor);
    const baselineInFrustum = baselineProjection.ndc.every((value) => value >= -1 && value <= 1);
    const candidateProjection = projectPointToNdc(candidateNominalCamera, authoredAnchor);
    return {
      sample: sample.sample,
      probe_anchor_ref: sample.probe_anchor_ref,
      authored_anchor: authoredAnchor,
      baseline: {
        ndc: baselineProjection.ndc.map((value) => round(value, 12)),
        axial_depth: round(baselineProjection.axialDepth, 12),
        actionability: baselineInFrustum ? "ACTIONABLE_IN_BASELINE_FRUSTUM" : "NOT_ATTEMPTED_OUTSIDE_BASELINE_FRUSTUM",
        oracle: baselineInFrustum
          ? expectedHitForProbe(model, baselineCamera, BASELINE_CANVAS_CSS[1], baselineProjection.ndc)
          : { status: "NO_RUNTIME_ACTION", reason: "ANCHOR_PROJECTION_OUTSIDE_CLOSED_NDC_SQUARE", expectedHitRef: null }
      },
      candidate_nominal_preflight_only: {
        ndc: candidateProjection.ndc.map((value) => round(value, 12)),
        axial_depth: round(candidateProjection.axialDepth, 12),
        actionability: candidateProjection.ndc.every((value) => value >= -1 && value <= 1)
          ? "NOMINALLY_IN_FRUSTUM_REQUIRES_RUNTIME_READBACK"
          : "NOMINALLY_OUTSIDE_FRUSTUM_PRODUCT_FIT_REPAIR_REQUIRED",
        oracle: expectedHitForProbe(model, candidateNominalCamera, BASELINE_CANVAS_CSS[1], candidateProjection.ndc)
      }
    };
  });
  return {
    schema: "openpipestress.ui-foundation.point-hit-oracle/v3",
    point_hit_policy_sha256: POINT_HIT_POLICY_SHA256,
    expected_values_boundary: "Baseline expectations use source-bound deterministic Iso state. Candidate values here are nominal preflight only; full candidate expectations are regenerated offline from an accepted read-only camera/canvas preflight before measured candidate sessions and never inferred from product selection output.",
    probe_order_sha256: sha256(stableBytes(samples.point_selection.map((sample) => sample.probe_anchor_ref))),
    baseline_camera: baselineCamera,
    baseline_canvas_css: BASELINE_CANVAS_CSS,
    baseline_canvas_evidence: "BASELINE_V2 Playwright after@call@14 snapshot canvas bounding rect left=228 top=220 right=1147 bottom=848 and CSS size 919x628",
    candidate_nominal_camera: candidateNominalCamera,
    probes
  };
}

function precisionModel(translation) {
  const base = buildModel(20);
  base.project.id = "project:UIF-PRECISION-ORIGIN";
  base.project.name = "Generated UI local-render-origin precision probe";
  base.project.description = "Small authored-coordinate model used in original and translated forms for Fit, pick, and measurement equivalence.";
  base.nodes = base.nodes.map((node) => ({
    ...node,
    id: node.id.replace("UIF-", "UIF-PRECISION-"),
    position: {
      x: round(node.position.x + translation[0], 6),
      y: round(node.position.y + translation[1], 6),
      z: round(node.position.z + translation[2], 6)
    }
  }));
  const nodeId = (id) => id.replace("UIF-", "UIF-PRECISION-");
  base.pipe_segments = base.pipe_segments.map((pipe) => ({
    ...pipe,
    id: nodeId(pipe.id),
    from: nodeId(pipe.from),
    to: nodeId(pipe.to)
  }));
  base.supports = base.supports.map((support) => ({ ...support, id: nodeId(support.id), node: nodeId(support.node) }));
  base.components = [];
  base.load_cases = [];
  base.diagnostics = [];
  return base;
}

function stableBytes(value) {
  return Buffer.from(`${JSON.stringify(value)}\n`, "utf8");
}

function sha256(bytes) {
  return createHash("sha256").update(bytes).digest("hex");
}

async function compareWithFrozen(relativePath, bytes) {
  let frozen = null;
  try { frozen = await readFile(path.join(ROOT, relativePath)); } catch (error) { if (error.code !== "ENOENT") throw error; }
  if (!frozen || frozen.length !== bytes.length || sha256(frozen) !== sha256(bytes)) {
    checkFindings.push({ path: relativePath, generated_sha256: sha256(bytes), generated_bytes: bytes.length,
      frozen_sha256: frozen ? sha256(frozen) : null, frozen_bytes: frozen ? frozen.length : null });
  }
}

async function emit(relativePath, value, inventory) {
  const bytes = stableBytes(value);
  if (CHECK) {
    await compareWithFrozen(relativePath, bytes);
  } else {
    const outputPath = path.join(ROOT, relativePath);
    await mkdir(path.dirname(outputPath), { recursive: true });
    await writeFile(outputPath, bytes);
  }
  inventory.push({ path: relativePath, sha256: sha256(bytes), bytes: bytes.length });
}

const PRESERVED_PROTOCOL_HISTORY = [
  ["fixture-manifest-v1-superseded-before-timed-run.json", "285b918088cae236e38d8e9af31945782e14aed26e68440a612aef336ba939c1"],
  ["fixture-manifest-v2-superseded-after-baseline-load-filter-orbit.json", "df35d5291c0cd39f6cc137c7af7e20758811410e8cf35f7308077aa7e62bc5b6"],
  ["fixture-manifest-v3-superseded-before-canvas-point-timing.json", "8ca81f73de7267c66b1d2c07778e6e08f71778a151e008edb0c4e248ad234575"],
  ["fixture-manifest-v4-superseded-before-candidate-camera-preflight.json", "d6dcc757d1024da020e4333e1f7f624243920821c04946c418384875573f72bc"],
  ["fixture-manifest-v5-superseded-before-resource-route-calibration.json", "d086af9e6301ba5d2f55d6363576c46296713a18e19ced3dbc8666d9892309aa"],
  ["fixture-manifest-v6-superseded-before-box-policy-binding.json", "3fe26201623736b7810ce9885a43f1ea0edb0400b0f65b4e104b842e2b5254df"],
  ["fixture-manifest-v7-superseded-before-final-control-binding.json", "962b7970428a08eab1ff25583d85e548147220b9118d931b16d32385f95bb926"],
  ["ui-foundation-1000.interactions-v1-label-proxy.json", "0a68da5acf202c943ad6441e8731429ed1c156fb0f7cee02493d2f9b6ede3bed"],
  ["ui-foundation-10000.interactions-v1-label-proxy.json", "97ce2688aa79492402664ec091f2da43cb8f696fefc3c64d6b5d7b59690ac3a5"]
];

// Reads only. A supplied (or present default) directory must hold all nine files with the pinned
// hashes, in either mode. With no directory, the check mode says so in its report and the writing
// mode refuses before its first write.
async function verifyPreservedProtocolHistory() {
  const supplied = protocolHistoryArgument();
  const fallback = path.join(ROOT, "protocol-history");
  let directory = supplied;
  if (!directory) {
    try { if ((await stat(fallback)).isDirectory()) directory = fallback; } catch (error) { if (error.code !== "ENOENT") throw error; }
  }
  if (!directory) {
    if (!CHECK) throw new Error(`preserved protocol history was not supplied and the default directory is absent: supply the nine files with ${HISTORY_FLAG} <absolute directory>, or use --check (which writes nothing); nothing was written`);
    return { status: "NOT_SUPPLIED_NINE_PINNED_FILES", verified_files: 0, required_files: PRESERVED_PROTOCOL_HISTORY.length };
  }
  for (const [name, expectedHash] of PRESERVED_PROTOCOL_HISTORY) {
    let bytes;
    try { bytes = await readFile(path.join(directory, name)); } catch (error) {
      if (error.code === "ENOENT") throw new Error(`preserved protocol history file absent: ${name}; nothing was written`);
      throw error;
    }
    if (sha256(bytes) !== expectedHash) throw new Error(`preserved protocol history mismatch for ${name}: ${sha256(bytes)}; nothing was written`);
  }
  return { status: "VERIFIED_NINE_PINNED_HASHES", source: supplied ? "supplied-directory" : "default-directory",
    verified_files: PRESERVED_PROTOCOL_HISTORY.length, required_files: PRESERVED_PROTOCOL_HISTORY.length };
}

function modelCounts(model) {
  return {
    nodes: model.nodes.length,
    pipe_segments: model.pipe_segments.length,
    supports: model.supports.length,
    components: model.components.length,
    nodal_force_arrows: model.load_cases.flatMap((loadCase) => loadCase.primitive_loads ?? []).filter((load) => load.target?.type === "node" && load.dimension === "force").length,
    materials: model.materials?.length ?? 0,
    sections: model.sections?.length ?? 0
  };
}

async function main() {
  const protocolHistory = await verifyPreservedProtocolHistory();
  const inventory = [];
  const fixtureSummaries = [];
  for (const size of SIZES) {
    const model = buildModel(size);
    const overlay = buildOverlay(model);
    const samples = buildSamples(size);
    const oracle = pointOracle(model, samples);
    await emit(`fixtures/ui-foundation-${size}.model.json`, model, inventory);
    await emit(`fixtures/ui-foundation-${size}.deformation-overlay.json`, overlay, inventory);
    await emit(`samples/ui-foundation-${size}.interactions.json`, samples, inventory);
    await emit(`samples/ui-foundation-${size}.point-oracle-v3.json`, oracle, inventory);
    fixtureSummaries.push({
      pipe_count: size,
      model_path: `fixtures/ui-foundation-${size}.model.json`,
      overlay_path: `fixtures/ui-foundation-${size}.deformation-overlay.json`,
      samples_path: `samples/ui-foundation-${size}.interactions.json`,
      point_oracle_path: `samples/ui-foundation-${size}.point-oracle-v3.json`,
      counts: modelCounts(model),
      selected_set_sizes: Object.fromEntries(Object.entries(samples.selected_sets).map(([name, values]) => [name, values.length])),
      typed_collision: {
        raw_id: entityId("node", COLLISION_INDEX),
        types: size >= COLLISION_INDEX ? ["node", "pipe", "support", "component"] : []
      }
    });
  }

  const precisionBase = precisionModel([0, 0, 0]);
  const precisionTranslated = precisionModel([1_000_000, 2_000_000, -3_000_000]);
  await emit("fixtures/precision-origin-base.model.json", precisionBase, inventory);
  await emit("fixtures/precision-origin-translated.model.json", precisionTranslated, inventory);
  const precisionExpectations = {
    schema: "openpipestress.ui-foundation.precision-origin-expectations/v1",
    translation_project_length_units: [1_000_000, 2_000_000, -3_000_000],
    project_length_unit: "m",
    model_identity: "same semantic model except every authored node coordinate is translated by the exact vector",
    fit_mutation_allowed: false,
    pick_equivalence_css_pixel_tolerance: 1,
    relative_geometry_project_unit_tolerance: 1e-9,
    measurement_pairs: [
      { from: precisionBase.nodes[0].id, to: precisionBase.nodes[1].id },
      { from: precisionBase.nodes[3].id, to: precisionBase.nodes[11].id },
      { from: precisionBase.nodes[9].id, to: precisionBase.nodes[20].id }
    ].map((pair) => {
      const from = precisionBase.nodes.find((node) => node.id === pair.from).position;
      const to = precisionBase.nodes.find((node) => node.id === pair.to).position;
      const delta = { x: round(to.x - from.x, 9), y: round(to.y - from.y, 9), z: round(to.z - from.z, 9) };
      return { ...pair, delta, distance: round(Math.hypot(delta.x, delta.y, delta.z), 9), unit: "m" };
    }),
    route_capture_boundary: "Any local render-origin transform changes display coordinates only; route points remain authored project coordinates."
  };
  await emit("fixtures/precision-origin-expectations.json", precisionExpectations, inventory);

  const candidateControlBinding = {
    schema: "openpipestress.ui-foundation.candidate-control-binding/v1",
    status: "FROZEN_PROSPECTIVE_PENDING_PRODUCT_SOURCE_INTEGRATION",
    source: "UI manager checkpoint-2/full-workspace verifier coordination",
    governedBindingSha256: UI_TEST_BINDINGS_SHA256,
    controls: {
      labels: { role: "button", name: "Labels", testid: "toggle-viewport-labels", state: "aria-pressed" },
      boxSelect: { role: "button", name: "Box Select", testid: "viewport-box-select", state: "aria-pressed" },
      selectionFilter: { role: "combobox", name: "Selection filter", testid: "viewport-selection-filter", options: ["All", "Pipes", "Nodes", "Supports", "Components"] },
      fitModel: { role: "button", name: "Fit Model", testid: "viewport-fit-model" },
      isometric: { role: "button", name: "Isometric", testid: "viewport-view-isometric" },
      schematic: { role: "button", name: "Schematic", testid: "viewport-geometry-schematic", state: "aria-pressed" },
      actualOd: { role: "button", name: "Actual OD", testid: "viewport-geometry-actual-od", state: "aria-pressed" },
      openLocal: { role: "button", name: "Open local", testid: "open-local-project" }
    },
    feedback: {
      selectionSummary: { role: "status", name: "Selection summary", testid: "command-selection-readout" },
      propertyInspector: { role: "region", name: "Property inspector", testid: "property-inspector" },
      typedTreeRow: "tree-row-${encodeURIComponent(type)}-${encodeURIComponent(id)}"
    },
    sourceOnlyResourceReader: {
      module: "src/features/viewport/viewportResource.ts",
      export: "currentOwnedViewportResourceSnapshot",
      signature: "(): OwnedViewportResourceSnapshot | null",
      currentOnly: true,
      readOnly: true
    },
    mutation_surface: "none",
    qualification_rule: "Every binding must be present in the integrated production/source bundle before use; absence or semantic drift fails qualification."
  };
  await emit("candidate-control-binding-v1.json", candidateControlBinding, inventory);

  const manifest = {
    schema: "openpipestress.ui-foundation.fixture-manifest/v2",
    status: "frozen",
    seed: SEED,
    generator: { path: "generate-fixtures.mjs", algorithm: "mulberry32_uint32_and_integer_lattice_plus_point_hit_oracle_v3" },
    purpose: "Deterministic UI workload only; no solver-capacity, engineering-validation, professional-acceptance, or minimum-hardware claim.",
    fixtures: fixtureSummaries,
    precision_regression: {
      base_path: "fixtures/precision-origin-base.model.json",
      translated_path: "fixtures/precision-origin-translated.model.json",
      expectations_path: "fixtures/precision-origin-expectations.json",
      benchmark_samples_added: 0
    },
    candidate_observability: {
      governed_attachment_sha256: OBSERVABILITY_ATTACHMENT_SHA256,
      verifier_binding_path: "observer-contract-binding-v1.md",
      verifier_binding_sha256: OBSERVER_BINDING_SHA256,
      global_name: "__openPipeStressUiDiagnosticsV1",
      mutation_surface: "none",
      missing_or_stale_evidence: "FAIL_QUALIFICATION"
    },
    candidate_control_binding: {
      path: "candidate-control-binding-v1.json",
      governed_binding_sha256: UI_TEST_BINDINGS_SHA256,
      status: "FROZEN_PROSPECTIVE_PENDING_PRODUCT_SOURCE_INTEGRATION",
      absent_or_drifted_binding: "FAIL_QUALIFICATION"
    },
    point_hit_protocol: {
      policy_sha256: POINT_HIT_POLICY_SHA256,
      local_render_origin: "Per-axis midpoint of the bounds union of all N+1 finite authored node positions only; excludes marker radii, load arrows, labels, OD envelopes, results, and visibility masks.",
      independent_oracle_organization: "Exhaustive primitive evaluation with conservative per-primitive AABB rejection, using the sealed V3 analytic primitives, global minimum, and tie rule; product chunk partitioning is intentionally not emulated.",
      labels_state_during_point_actions: "OFF through the existing real Labels control in baseline and candidate",
      labels_restore_boundary: "ON before label-budget, visual, and orbit conditions",
      per_point_reset: "Before every timed point gesture, activate the real project tree row outside the timer and observe exclusive typed Project selection. Record reset timing separately. A repeated expected hit must require a new action- and generation-associated render submission, never stale selection state.",
      input_path: "real Playwright pointer input through the ordinary canvas path; synthetic dispatchEvent and DOM/app-handler invocation prohibited",
      expected_hit_boundary: "independent analytic V3 oracle computed before measured candidate sessions; never inferred from candidate selection",
      baseline_outside_frustum: "Fixed baseline Iso recipe leaves 20/200 N=1000 and 19/200 N=10000 anchor projections outside the closed NDC square; those are NOT_ATTEMPTED_OUTSIDE_BASELINE_FRUSTUM and never successful actions.",
      candidate_actionability: "Approved new-model Fit behavior and real preset readback must make all 200 prescribed probes actionable before full qualification; failure is a product Fit defect, not a benchmark waiver.",
      candidate_preflight: "One separately evidenced untimed preflight uses only real Fit/preset controls, freezes independent runtime camera/canvas expectations before the five fresh measured sessions, and contributes no measured sample. Each measured run revalidates actual camera, frustum, and canvas before point actions."
    },
    box_selection_protocol: {
      policy_sha256: BOX_SELECTION_POLICY_SHA256,
      candidate_set: "Unhidden valid pipes/nodes/supports/components selected by the frozen filter; project excluded; occlusion irrelevant.",
      exact_test: "Closed six-plane perspective-frustum clipping precedes projection. Left-to-right contains both clipped pipe endpoints; right-to-left intersects or touches the rectangle. Points use authored anchors and the same closed rectangle for either direction.",
      order_and_primary: "Independent full typed model-index treeOrder; frozen samples are plain replace and last hit is primary.",
      measurement: "Real primary-pointer canvas drag after explicit Box Select/filter, exact independent ordered refs and primary, new box action sequence and generation-associated main render, then browser capture conservative upper bound."
    },
    cameras: {
      baseline_reachable_recipe: {
        controls: ["click visible Iso preset", "apply orbit_pointer_path through the real viewport canvas"],
        exact_state_observable: false,
        state_claim: "unobserved; current product exposes no camera readback and its preset uses product-owned constants",
        coverage: "real reachable preset and gesture path only; nominal candidate poses are not claimed as observed baseline state",
        source_bound_iso_state: { position: [7.6, 7, 8], target: [3.8, 1.2, 0.7], up: [0, 1, 0], fov_degrees: 42, near: 0.1, far: 1000 },
        baseline_canvas_css: BASELINE_CANVAS_CSS
      },
      candidate_workload_pose_drafts: {
        overview_iso: { ui_command: "Fit Visible then Iso", position: [12.4, 8.1, 13.7], target: [5, 0.65, 0.5], up: [0, 1, 0], fov_degrees: 42 },
        dense_overlap: { draft_pose_only: true, position: [5, 7.5, 0.56], target: [5, 0.56, 0.56], up: [0, 0, -1], fov_degrees: 42 },
        box_selection: { draft_pose_only: true, position: [5, 9.5, 0.56], target: [5, 0.56, 0.56], up: [0, 0, -1], fov_degrees: 42 },
        status: "REFERENCE_DRAFTS_ONLY_NOT_CLAIMED_REACHABLE_AND_NOT_PRODUCT_CONTROL_REQUIREMENTS"
      },
      candidate_standard_command_preflight: {
        allowed_camera_path: "Use only real professional Fit and standard preset controls such as Top and Iso; no hidden camera mutation and no benchmark-only control.",
        freeze_boundary: "Record actual camera/canvas readback and seal the derived recipe and independent oracle in the untimed preflight before five measured sessions.",
        unreachable_draft_rule: "If an exact draft pose is not reachable, preserve it as a draft and use the sealed actual standard-command recipe without changing fixture, visibility, extents, probe anchors, order, or action count.",
        per_run_validation: "Every measured run applies the same sealed real controls and validates actual camera, frustum, and canvas before interaction.",
        mutation_boundary: "Standard view commands change viewport state only and must not change model bytes or hashes."
      }
    },
    benchmark: {
      viewport: { css_pixels: [1440, 920], device_pixel_ratio_cap: 2 },
      repetitions_per_fixture: 5,
      fresh_app_session_per_repetition: true,
      production_bundle_required: true,
      samples_per_repetition: {
        point_selection: 200,
        box_selection: 20,
        tree_filter: 20,
        orbit_warmup_ms: 2000,
        orbit_measured_ms: 10000
      },
      acceptance_targets_ms: {
        assignment_to_presented_viewport_and_responsive_tree_max: 2000,
        point_selection_presented_feedback_p95_max: 100,
        box_selection_presented_feedback_p95_max: 200,
        tree_filter_presented_feedback_p95_max: 200,
        orbit_centerline_frame_interval_p95_max: 16.7,
        orbit_actual_od_frame_interval_p95_max: 33.3
      },
      selection_set_sizes_required_on_10000_fixture: [0, 1, 100, 10000],
      selected_label_cap_candidate: 80,
      resource_replacement_cycles: 20,
      baseline_unsupported: ["box_selection", "actual_od", "in_session_model_replacement_generation_and_resource_observability", "renderer_resource_counters", "exact_app_assignment_start"],
      replacement_route_boundary: "Real browser-session local snapshot routes exist. Candidate first creates/saves the fixture snapshot through ordinary product controls, then repeats the visible `Open local` button (`data-testid=open-local-project`, calling openLocalProject) twenty times. There is no user-facing file input and setInputFiles is not accepted. Baseline replacement control availability is distinct from absent generation/resource observability and was not measured; N=10000 is additionally unavailable after renderer OOM.",
      remount_boundary: "Production browser has no accepted viewport-unmount command. Whole-page reload is the only production-browser owner teardown; real React component unmount tests and native process quit/reopen are separate evidence classes. No injected React/app mutation bridge is allowed in E2E.",
      baseline_common_actions: ["model_fixture_module_load", "real_canvas_pointer_pick_labels_off", "tree_filter", "orbit"],
      unsupported_rule: "Unsupported actions and metrics are N/A, never PASS. Outside-frustum and renderer-OOM actions are unavailable, never PASS. All 200 candidate point actions remain required."
    },
    od_conversion_states: {
      cold: "new project-session generation; zero cached OD conversions; submit one batched request for every distinct authored valid section quantity",
      warm: "same model generation and display unit; all distinct valid section conversions resolved and cached; no conversion request may be outstanding",
      obsolete_response: "advance model generation before delayed conversion resolves; returned values must be rejected and must not publish geometry"
    },
    publication_timing: {
      model_assignment_start: "Candidate strict: performance.now at entry to app-level assignment of the already parsed fixture. Baseline proxy: routed fixture module evaluation after object construction; includes unresolved Promise.all delay and cannot earn strict PASS.",
      viewport_presented: "Baseline proxy: a bound WebGL canvas exists and a following requestAnimationFrame paint opportunity occurs; this does not prove compositor presentation. Candidate strict evidence adds a Playwright/browser-observed screenshot after the generation-bound frame and reports the post-capture timestamp as a conservative upper bound.",
      tree_responsive: "The bound tree generation is browser-observed, and an identity-preserving probe selection publishes tree, primary highlight and property identity; a following paint opportunity is a baseline proxy unless a browser-observed capture is retained.",
      point_start: "Capturing pointerdown event timestamp for the real point-pick gesture.",
      point_end: "The expected ordered selection/primary state, viewport highlight, and property identity are observed; a browser screenshot is retained and the post-capture page clock is the conservative upper bound. Capture overhead is never subtracted.",
      box_start: "Capturing pointerdown event timestamp after explicit Box Select is active with the frozen filter.",
      box_end: "The expected membership/primary state, viewport feedback, tree feedback, and a following paint are presented.",
      filter_start: "The input event timestamp that changes the real tree query.",
      filter_end: "Filter summary and virtualized visible rows for that query are committed and a following paint is presented.",
      handler_only_duration_accepted: false,
      raw_sample_clock: "window.performance.now monotonic milliseconds",
      p95_method: "nearest-rank: sorted_samples[ceil(0.95*n)-1]"
    },
    separate_metrics: [
      "process_to_first_window_or_browser_navigation",
      "fixture_file_read",
      "fixture_json_parse",
      "parsed_fixture_assignment_to_first_usable",
      "javascript_heap",
      "native_rss",
      "renderer_resources"
    ],
    raw_sample_layout: {
      directory: "<evidence-dir>/raw/run-<01..05>/<fixture-pipe-count>/",
      files: [
        "environment.json", "startup.json", "model-assignment.json", "point-selection.jsonl",
        "box-selection.jsonl", "tree-filter.jsonl", "orbit-centerline.jsonl", "orbit-actual-od.jsonl",
        "settled-raf.json", "resources.json", "heap-rss.json", "failures.jsonl"
      ],
      aggregation: "<evidence-dir>/summary.json retains every status, sample count, nearest-rank p95, source hash, command, and raw relative path"
    },
    files: inventory.sort((a, b) => a.path.localeCompare(b.path))
  };
  const generatedManifestBytes = stableBytes(manifest);
  if (CHECK) {
    await compareWithFrozen("fixture-manifest.json", generatedManifestBytes);
    const status = checkFindings.length === 0 ? "PASS_CHECK_REPRODUCES_FROZEN_BYTES_NOTHING_WRITTEN" : "FAIL_CHECK_GENERATED_BYTES_DIFFER_NOTHING_WRITTEN";
    process.stdout.write(`${JSON.stringify({ status, mode: "check", manifest_sha256: sha256(generatedManifestBytes), files: inventory.length,
      compared_files: inventory.length + 1, differing: checkFindings, protocol_history: protocolHistory, fixtures: fixtureSummaries }, null, 2)}\n`);
    if (checkFindings.length > 0) process.exitCode = 1;
    return;
  }
  await writeFile(path.join(ROOT, "fixture-manifest.json"), generatedManifestBytes);
  const manifestBytes = await readFile(path.join(ROOT, "fixture-manifest.json"));
  process.stdout.write(`${JSON.stringify({ manifest_sha256: sha256(manifestBytes), files: inventory.length, protocol_history: protocolHistory, fixtures: fixtureSummaries }, null, 2)}\n`);
}

await main();
