// The comparison overlay's pure view model (DEL-07-08 REQ-011, AC-003, VER-003). The differences
// are built into the fixtures by hand, so the expected marks are written out here, not taken from
// the comparison engines: the overlay reads the comparison result and classifies nothing itself.
// All data are invented.
import { describe, expect, it } from "vitest";
import { compareModelDocuments } from "./stateComparison";
import { compareRunInputs, type FlatRunRow, type RunComparisonInput, type RunRowIdentity } from "./runComparison";
import { comparisonOverlayScene, runOverlayMarks, stateOverlayMarks, type OverlayMark } from "./comparisonOverlay";

type Json = any; // eslint-disable-line @typescript-eslint/no-explicit-any

function document(overrides: Json = {}): Json {
  return {
    schema_version: "0.9.0",
    document_kind: "model",
    project: { id: "project:invented", name: "Invented", description: "", units: { length: "m", force: "N" } },
    data_boundary: {},
    analysis_status: {},
    materials: [{ id: "material:M-1", label: "Invented steel", elastic_modulus: { value: 200, unit: "GPa" }, provenance: "invented" }],
    nodes: [
      { id: "node:N-1", label: "A", position: { x: 0, y: 0, z: 0 }, provenance: "invented" },
      { id: "node:N-2", label: "B", position: { x: 2, y: 0, z: 0 }, provenance: "invented" },
      { id: "node:N-3", label: "C", position: { x: 2, y: 1, z: 0 }, provenance: "invented" },
      { id: "node:N-OLD", label: "D", position: { x: 0, y: 1, z: 0 }, provenance: "invented" }
    ],
    pipe_segments: [{ id: "pipe:P-1", label: "P", from: "node:N-1", to: "node:N-2", section: {}, material: "material:M-1", provenance: "invented" }],
    supports: [{ id: "support:S-1", label: "Anchor", node: "node:N-1", provenance: "invented" }],
    ...overrides
  };
}

// The hand-built pair (AC-003): the right state adds node N-4, removes node N-3, modifies support
// S-1 (its label), changes material M-1's unit-bearing elastic modulus (200 GPa to 210 GPa), and
// gives node N-OLD the new identifier N-NEW (same position and fields).
const LEFT = document();
const RIGHT = (() => {
  const model = document();
  model.nodes = model.nodes
    .filter((node: Json) => node.id !== "node:N-3")
    .map((node: Json) => (node.id === "node:N-OLD" ? { ...node, id: "node:N-NEW" } : node));
  model.nodes.push({ id: "node:N-4", label: "E", position: { x: 4, y: 0, z: 0 }, provenance: "invented" });
  model.supports[0].label = "Anchor, relabelled";
  model.materials[0].elastic_modulus = { value: 210, unit: "GPa" };
  return model;
})();

const byRef = (marks: readonly OverlayMark[]) => Object.fromEntries(marks.map((mark) => [`${mark.ref}|${mark.markClass}`, mark.side]));

describe("the state comparison overlay", () => {
  const marks = stateOverlayMarks(compareModelDocuments(LEFT, RIGHT));

  it("marks exactly the entities the hand-built states differ by, each with its class and side", () => {
    expect(byRef(marks)).toEqual({
      "node:N-4|added": "right",
      // The renamed entity, without a mapping: removed on one side and added on the other.
      "node:N-NEW|added": "right",
      "node:N-OLD|removed": "left",
      "node:N-3|removed": "left",
      "support:S-1|changed": "both",
      "material:M-1|changed": "both",
      // The single model entity holds the ordered id list of every collection (DEL-14-03), which
      // the added, removed and renamed nodes change.
      "model|changed": "both",
      "node:N-1|unchanged": "both",
      "node:N-2|unchanged": "both",
      "pipe:P-1|unchanged": "both"
    });
  });

  it("draws the later state, the entities only in the earlier one as ghosts, and counts what it does not draw", () => {
    const scene = comparisonOverlayScene("states", marks, LEFT, RIGHT);
    const drawn = Object.fromEntries(scene.drawn.map((entity) => [entity.ref, { classes: entity.classes, ghost: entity.ghost }]));
    expect(drawn).toEqual({
      "node:N-1": { classes: ["unchanged"], ghost: false },
      "node:N-2": { classes: ["unchanged"], ghost: false },
      "node:N-NEW": { classes: ["added"], ghost: false },
      "node:N-4": { classes: ["added"], ghost: false },
      "pipe:P-1": { classes: ["unchanged"], ghost: false },
      "support:S-1": { classes: ["changed"], ghost: false },
      "node:N-3": { classes: ["removed"], ghost: true },
      "node:N-OLD": { classes: ["removed"], ghost: true }
    });
    // A ghost is drawn in its own position in the earlier state.
    expect(scene.drawn.find((entity) => entity.ref === "node:N-3")!.geometry).toEqual({ kind: "point", at: { x: 2, y: 1, z: 0 } });
    expect(scene.drawn.find((entity) => entity.ref === "pipe:P-1")!.geometry).toEqual({ kind: "segment", from: { x: 0, y: 0, z: 0 }, to: { x: 2, y: 0, z: 0 } });
    // The material and the model entity have no drawing: counted, with the reason, not dropped.
    expect(scene.notDrawn.map((entry) => [entry.mark.ref, entry.reason])).toEqual([["model", "no_drawing"], ["material:M-1", "no_drawing"]]);
    expect(scene.counts.map(({ markClass, total, drawn: shown, notDrawn }) => [markClass, total, shown, notDrawn])).toEqual([
      ["added", 2, 2, 0],
      ["removed", 2, 2, 0],
      ["changed", 3, 1, 2],
      ["unchanged", 3, 3, 0]
    ]);
  });

  it("does not draw an earlier-only entity when the two states' length units differ, and says why", () => {
    const left = document({ project: { ...LEFT.project, units: { length: "mm", force: "N" } } });
    const scene = comparisonOverlayScene("states", stateOverlayMarks(compareModelDocuments(left, RIGHT)), left, RIGHT);
    expect(scene.drawn.some((entity) => entity.ghost)).toBe(false);
    expect(scene.notDrawn.filter((entry) => entry.reason === "length_unit_differs").map((entry) => entry.mark.ref).sort()).toEqual(["node:N-3", "node:N-OLD"]);
  });

  it("counts an entity whose position cannot be read rather than placing it anywhere", () => {
    const right = structuredClone(RIGHT);
    right.nodes.find((node: Json) => node.id === "node:N-4").position = { x: 4, y: null, z: 0 };
    const scene = comparisonOverlayScene("states", stateOverlayMarks(compareModelDocuments(LEFT, right)), LEFT, right);
    expect(scene.drawn.some((entity) => entity.ref === "node:N-4")).toBe(false);
    expect(scene.notDrawn).toContainEqual({ mark: { ref: "node:N-4", family: "node", markClass: "added", side: "right" }, reason: "no_position" });
  });
});

// Two runs with hand-set rows: N-1's displacement differs (1 mm then 1.5 mm), N-2's is equal
// (2 mm both), support S-1's reaction is only in the left run (the support was removed in the later
// model), pipe P-1's moment is only in the right run, and N-2's rotation has a different unit on
// each side (not compared). A solver row names no model entity.
function identity(result_id: string, object_ref: string, family: string): RunRowIdentity {
  return { result_id, family, object_ref, basis_ref: { ref_type: "load_case", ref_id: "load:L-1" }, dimension: null, component: "x", location: null, basis: null, coordinate_system: "global", sign_convention: null };
}
function row(result_id: string, object_ref: string, family: string, value: unknown, unit: string, row_index = 0): FlatRunRow {
  return { identity: identity(result_id, object_ref, family), value, unit, row_index };
}
function input(rows: FlatRunRow[]): RunComparisonInput {
  return {
    rows,
    standing: { eligible: true, status: "integrity_checked", contract: "preview_physics", findings: [] },
    semantic_identity: { route: "invented", contract_id: null, contract_sha256: null },
    context: {},
    diagnostics: []
  };
}
const RUN_LEFT = input([
  row("r:n1:dx", "node:N-1", "displacement", 1, "mm"),
  row("r:n2:dx", "node:N-2", "displacement", 2, "mm"),
  row("r:n2:rx", "node:N-2", "rotation", 0.1, "rad"),
  row("r:s1:fz", "support:S-1", "reaction", -10, "N"),
  row("r:solver", "solver:invented", "displacement", 0, "mm")
]);
const RUN_RIGHT = input([
  row("r:n1:dx", "node:N-1", "displacement", 1.5, "mm"),
  row("r:n2:dx", "node:N-2", "displacement", 2, "mm"),
  row("r:n2:rx", "node:N-2", "rotation", 5.7, "deg"),
  row("r:p1:mz", "pipe:P-1", "moment", 3, "N*m"),
  row("r:solver", "solver:invented", "displacement", 0, "mm")
]);
const RUN_LEFT_MODEL = document();
const RUN_RIGHT_MODEL = document({ supports: [] });

describe("the run comparison overlay", () => {
  const marks = runOverlayMarks(compareRunInputs(RUN_LEFT, RUN_RIGHT));

  it("marks the entities whose rows differ, are equal, are not compared or are in one run only", () => {
    expect(byRef(marks)).toEqual({
      "node:N-1|nonzero_delta": "both",
      "node:N-2|zero_delta": "both",
      "node:N-2|not_compared": "both",
      "solver:invented|zero_delta": "both",
      "support:S-1|left_only": "left",
      "pipe:P-1|right_only": "right"
    });
    expect(marks.every((mark) => mark.family === null)).toBe(true);
  });

  it("draws the later run's model, the support only in the earlier run's model as a ghost, and counts the rest", () => {
    const scene = comparisonOverlayScene("runs", marks, RUN_LEFT_MODEL, RUN_RIGHT_MODEL);
    const marked = Object.fromEntries(scene.drawn.filter((entity) => entity.classes.length).map((entity) => [entity.ref, { classes: entity.classes, ghost: entity.ghost }]));
    expect(marked).toEqual({
      "node:N-1": { classes: ["nonzero_delta"], ghost: false },
      // Classes in legend order: non-zero delta, not compared, left only, right only, zero delta.
      "node:N-2": { classes: ["not_compared", "zero_delta"], ghost: false },
      "pipe:P-1": { classes: ["right_only"], ghost: false },
      "support:S-1": { classes: ["left_only"], ghost: true }
    });
    // The later model's unclassified entities are drawn as context, unmarked.
    expect(scene.drawn.filter((entity) => !entity.classes.length).map((entity) => entity.ref).sort()).toEqual(["node:N-3", "node:N-OLD"]);
    expect(scene.notDrawn).toEqual([{ mark: { ref: "solver:invented", family: null, markClass: "zero_delta", side: "both" }, reason: "not_in_either_model" }]);
    expect(scene.counts.map(({ markClass, total, notDrawn }) => [markClass, total, notDrawn])).toEqual([
      ["nonzero_delta", 1, 0],
      ["not_compared", 1, 0],
      ["left_only", 1, 0],
      ["right_only", 1, 0],
      ["zero_delta", 2, 1]
    ]);
  });
});
