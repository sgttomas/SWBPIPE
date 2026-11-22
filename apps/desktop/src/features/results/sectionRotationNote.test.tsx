// The rotation sentence: it shows under the Rotation group's heading and at the end of the
// Displacement Extremes boundary text exactly when the result on screen records shear deformation
// in straight members, read from the result itself and never from the document's present setting.
// The stored results are the product's own; invented data only.
import { cleanup, render, screen, within } from "@testing-library/react";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { afterEach, describe, expect, it } from "vitest";
import { healthStanding, overlayModel, overlayResult, row } from "../../test-support/resultOverlayFixtures";
import type { MechanicsResult, PreviewModel } from "../../types";
import { DISPLACEMENT_EXTREMES_BOUNDARY_TEXT, DisplacementExtremesPanel } from "./DisplacementExtremesPanel";
import { PREVIEW_PHYSICS_CONTRACT_ID, PREVIEW_PHYSICS_TIMOSHENKO_CONTRACT_ID } from "./numericalResultQuality";
import { ResultsPanel } from "./ResultsPanel";
import { recordedBeamTheory, REALIZED_ARCS_DIAGNOSTIC, SECTION_ROTATION_NOTE, showsSectionRotationNote, withSectionRotationNote } from "./sectionRotationNote";

afterEach(cleanup);

const stored = (path: string): MechanicsResult => JSON.parse(readFileSync(resolve(__dirname, "../../../../../", path), "utf8")) as MechanicsResult;

/** Results whose straight members were solved with shear deformation, one per route and headline. */
const TIMOSHENKO = [
  ["pressure-free, straight members", "fixtures/results/preview_physics_timoshenko_straight_sparse.json"],
  ["pressure-free, with realized bends (Timoshenko; bends Euler–Bernoulli)", "fixtures/results/preview_physics_timoshenko_arc_sparse.json"],
  ["exact, with realized bends (Timoshenko; bends Euler–Bernoulli)", "fixtures/results/pressure_u4b_arc_sparse.json"]
] as const;
/** Euler–Bernoulli results of each route. */
const EULER_BERNOULLI = [
  ["pressure-free", "fixtures/results/preview_physics_connected_sparse.json"],
  ["pressure-free, realized bends", "fixtures/results/preview_physics_u4b_arc_sparse.json"],
  ["exact", "fixtures/results/pressure_connected_mechanics_sparse.json"],
  ["exact source", "fixtures/product_preview/pressure_source/physics_source/n05-sparse_interactive.raw.json"]
] as const;

/** The result with its rotation rows only, so the Rotation group is on the first page. */
const rotations = (result: MechanicsResult): MechanicsResult => ({ ...result, results: result.results.filter((item) => item.kind.includes("rotation")) });

const section = (beamTheory: Record<string, unknown> | undefined) => ({ pipe_id: "pipe:invented", ...(beamTheory ? { beam_theory: beamTheory } : {}) });
const arc = (beamTheory: Record<string, unknown>) => ({ ...section(beamTheory), member_kind: "realized_arc" });
const exact = (...sections: Record<string, unknown>[]) => ({ exact_cases: [{ load_case_id: "case:invented", pipe_sections: sections }] });
const SOLVED = { mechanics: "MECHANICS_SOLVED" } as const;

const TIMOSHENKO_DOCUMENT = { model_settings: { shear_model: "timoshenko_annulus_energy_v1" } } as unknown as PreviewModel;
const EULER_BERNOULLI_DOCUMENT = { model_settings: { shear_model: "euler_bernoulli" } } as unknown as PreviewModel;

function panel(result: MechanicsResult, model: PreviewModel | null = null) {
  return render(<ResultsPanel result={result} model={model} knowledge={null} analysisRun={null} selectedResultId={null} onSelectResult={() => {}} />);
}

function expectNote(shown: boolean) {
  const group = screen.getByTestId("result-group-rotation");
  expect(within(group).getByRole("heading", { level: 3 }).textContent).toBe("Rotation");
  const line = within(group).queryByTestId("result-group-rotation-note");
  if (!shown) {
    expect(line).toBeNull();
    expect(group.hasAttribute("aria-describedby")).toBe(false);
    return;
  }
  expect(line?.textContent).toBe(SECTION_ROTATION_NOTE);
  // Directly under the heading, and it describes its group.
  expect(within(group).getByRole("heading", { level: 3 }).nextElementSibling).toBe(line);
  expect(group.getAttribute("aria-describedby")).toBe(line!.id);
  expect(line!.getAttribute("role")).toBeNull();
}

describe("the rotation sentence's condition", () => {
  it.each(TIMOSHENKO)("shows for a stored %s result", (_, path) => {
    expect(showsSectionRotationNote(stored(path))).toBe(true);
  });

  it.each(EULER_BERNOULLI)("does not show for a stored %s Euler–Bernoulli result", (_, path) => {
    expect(showsSectionRotationNote(stored(path))).toBe(false);
  });

  it("reads the exact routes' per-member record: Timoshenko straight members show it, a realized arc's record never does", () => {
    const timoshenko = { model: "timoshenko_annulus_energy_v1", shear_factor: 0.5, shear_area_m2: 0.001 };
    const eb = { model: "euler_bernoulli" };
    expect(showsSectionRotationNote({ status: SOLVED, contract_evidence: exact(section(timoshenko)) })).toBe(true);
    expect(showsSectionRotationNote({ status: SOLVED, contract_evidence: exact(arc(eb), section(timoshenko)) })).toBe(true);
    expect(showsSectionRotationNote({ status: SOLVED, contract_evidence: exact(section(eb), arc(eb)) })).toBe(false);
    expect(showsSectionRotationNote({ status: SOLVED, contract_evidence: exact(arc(eb)) })).toBe(false);
    // Two straight members recording different theories: no route publishes that, and it records none.
    expect(showsSectionRotationNote({ status: SOLVED, contract_evidence: exact(section(eb), section(timoshenko)) })).toBe(false);
  });

  it("does not show for a result that is not solved, though it records Timoshenko", () => {
    const blocked = stored("fixtures/results/preview_physics_timoshenko_partial_span_blocked_sparse.json");
    expect(blocked.status.mechanics).toBe("MODEL_INCOMPLETE");
    expect(recordedBeamTheory(blocked).straightMembers).toBe("timoshenko_annulus_energy_v1");
    expect(showsSectionRotationNote(blocked)).toBe(false);
    expect(withSectionRotationNote("Boundary.", blocked)).toBe("Boundary.");
    expect(showsSectionRotationNote({ status: { mechanics: "MODEL_INCOMPLETE" }, contract_evidence: exact(section({ model: "timoshenko_annulus_energy_v1" })) })).toBe(false);
    expect(showsSectionRotationNote({ contract_evidence: exact(section({ model: "timoshenko_annulus_energy_v1" })) })).toBe(false);
  });

  it("does not show for a result that records no theory", () => {
    expect(showsSectionRotationNote(null)).toBe(false);
    expect(showsSectionRotationNote({})).toBe(false);
    expect(showsSectionRotationNote({ producer: { semantic_contract_id: "openpipestress.result_semantics/0.3.0/pressure-3" }, contract_evidence: exact(section(undefined)) })).toBe(false);
    expect(showsSectionRotationNote({ contract_evidence: { exact_cases: [{ load_case_id: "case:invented" }] } })).toBe(false);
  });

  it("appends the sentence to a text only when it shows", () => {
    expect(withSectionRotationNote("Boundary.", { status: SOLVED, producer: { semantic_contract_id: PREVIEW_PHYSICS_TIMOSHENKO_CONTRACT_ID } })).toBe(`Boundary. ${SECTION_ROTATION_NOTE}`);
    expect(withSectionRotationNote("Boundary.", { status: SOLVED, producer: { semantic_contract_id: PREVIEW_PHYSICS_CONTRACT_ID } })).toBe("Boundary.");
  });
});

describe("the result's recorded beam theory (the headline's and the sentence's one reading)", () => {
  it("reads a solved pressure-free Timoshenko result's arcs from the solve's own diagnostic, present and absent", () => {
    const codes = (result: MechanicsResult) => result.diagnostics.map((item) => item.code);
    // Maintained stored results, the product's output (`pf_timoshenko_envelopes.rs`).
    const withArc = stored("fixtures/results/preview_physics_timoshenko_arc_sparse.json");
    expect(withArc.status.mechanics).toBe("MECHANICS_SOLVED");
    expect(codes(withArc)).toContain(REALIZED_ARCS_DIAGNOSTIC);
    expect(recordedBeamTheory(withArc)).toEqual({ straightMembers: "timoshenko_annulus_energy_v1", realizedArc: true });
    const straight = stored("fixtures/results/preview_physics_timoshenko_straight_sparse.json");
    expect(straight.status.mechanics).toBe("MECHANICS_SOLVED");
    expect(codes(straight)).not.toContain(REALIZED_ARCS_DIAGNOSTIC);
    expect(recordedBeamTheory(straight)).toEqual({ straightMembers: "timoshenko_annulus_energy_v1", realizedArc: false });
  });

  it.each([
    // Not solved: the solve states nothing about arcs on a blocked result.
    ["blocked pressure-free Timoshenko", "fixtures/results/preview_physics_timoshenko_partial_span_blocked_sparse.json", "timoshenko_annulus_energy_v1"],
    // Euler–Bernoulli: the route states no arcs, though this one has them.
    ["pressure-free Euler–Bernoulli, with realized arcs", "fixtures/results/preview_physics_u4b_arc_sparse.json", "euler_bernoulli"],
    ["pressure-free Euler–Bernoulli", "fixtures/results/preview_physics_connected_sparse.json", "euler_bernoulli"]
  ] as const)("a %s result gives its identity's theory and an arc that is not known", (_, path, theory) => {
    const result = stored(path);
    expect(result.diagnostics.map((item) => item.code)).not.toContain(REALIZED_ARCS_DIAGNOSTIC);
    expect(recordedBeamTheory(result)).toEqual({ straightMembers: theory, realizedArc: "not_known" });
  });

  it("is never read from the document", () => {
    const result = stored("fixtures/results/preview_physics_timoshenko_straight_sparse.json");
    const withDocument = { ...result, model: { model_settings: { shear_model: "euler_bernoulli" } }, model_settings: { shear_model: "euler_bernoulli" } } as unknown as MechanicsResult;
    expect(recordedBeamTheory(withDocument)).toEqual({ straightMembers: "timoshenko_annulus_energy_v1", realizedArc: false });
  });

  it.each([
    ["exact Timoshenko with realized arcs (Timoshenko; bends Euler–Bernoulli)", "fixtures/results/pressure_u4b_arc_sparse.json", "timoshenko_annulus_energy_v1", true],
    ["exact Euler–Bernoulli, straight members", "fixtures/results/pressure_connected_mechanics_sparse.json", "euler_bernoulli", false],
    ["exact Euler–Bernoulli with a realized bend", "fixtures/reports/invented/exact_report_witness_inputs/pressure_2_realized_bend-sparse_interactive.raw.json", "euler_bernoulli", true],
    ["exact source Euler–Bernoulli", "fixtures/product_preview/pressure_source/physics_source/n05-sparse_interactive.raw.json", "euler_bernoulli", false]
  ] as const)("an %s result gives its members' records", (_, path, theory, realizedArc) => {
    expect(recordedBeamTheory(stored(path))).toEqual({ straightMembers: theory, realizedArc });
  });

  it("gives no straight theory where the exact records hold no straight member, and nothing where nothing is recorded", () => {
    // The corpus's replaced-span case: its one span is a connector, so its exact case has no member record.
    const corpus = JSON.parse(readFileSync(resolve(__dirname, "../../../../../fixtures/results/pressure_v3_arc_reader_corpus.json"), "utf8")) as { cases: { label: string; envelope: MechanicsResult }[] };
    const replaced = corpus.cases.find((entry) => entry.label === "replaced span only 0.3.0 sparse_interactive")!.envelope;
    expect(recordedBeamTheory(replaced)).toEqual({ straightMembers: null, realizedArc: false });
    expect(recordedBeamTheory({ contract_evidence: exact(arc({ model: "euler_bernoulli" })) })).toEqual({ straightMembers: null, realizedArc: true });
    expect(recordedBeamTheory({ contract_evidence: exact(section(undefined)) })).toEqual({ straightMembers: null, realizedArc: false });
    expect(recordedBeamTheory(null)).toEqual({ straightMembers: null, realizedArc: "not_known" });
    expect(recordedBeamTheory({ producer: { semantic_contract_id: "openpipestress.result_semantics/0.3.0/unknown" } })).toEqual({ straightMembers: null, realizedArc: "not_known" });
  });
});

describe("results panel: the line under the Rotation heading", () => {
  it.each(TIMOSHENKO)("is shown for a %s result", (_, path) => {
    panel(rotations(stored(path)));
    expectNote(true);
  });

  it.each(EULER_BERNOULLI)("is not shown for a %s Euler–Bernoulli result", (_, path) => {
    panel(rotations(stored(path)));
    expectNote(false);
  });

  it("follows the result on screen, not the document's present setting", () => {
    // The document now states Timoshenko; the result was solved Euler–Bernoulli.
    panel(rotations(stored("fixtures/results/preview_physics_connected_sparse.json")), TIMOSHENKO_DOCUMENT);
    expectNote(false);
    cleanup();
    // The reverse: the document now states Euler–Bernoulli; the result was solved Timoshenko.
    panel(rotations(stored("fixtures/results/preview_physics_timoshenko_straight_sparse.json")), EULER_BERNOULLI_DOCUMENT);
    expectNote(true);
  });

  it("is on no other group", () => {
    panel(stored("fixtures/results/preview_physics_timoshenko_straight_sparse.json"));
    expect(screen.getAllByTestId("result-group-rotation-note")).toHaveLength(1);
  });
});

describe("displacement extremes: the boundary text", () => {
  const rows = [
    row("ux:A", "global_nodal_displacement_x", "node:A", 1.5, "mm", { basis_ref: { ref_type: "load_case", ref_id: "load:L-1" } }),
    row("rz:A", "global_nodal_rotation_z", "node:A", 0.01, "rad", { basis_ref: { ref_type: "load_case", ref_id: "load:L-1" } })
  ];
  const extremes = (result: MechanicsResult, model: PreviewModel = overlayModel()) =>
    render(<DisplacementExtremesPanel model={model} result={result} standing={healthStanding({ eligible: true })} />);
  const withProducer = (id: string): MechanicsResult => ({ ...overlayResult(rows), producer: { component_name: "invented", component_version: "0", semantic_contract_id: id } });
  const boundary = () => screen.getByText((_, element) => element?.tagName === "SMALL" && (element.textContent ?? "").startsWith(DISPLACEMENT_EXTREMES_BOUNDARY_TEXT));

  it("gains the sentence for a pressure-free Timoshenko result and for an exact one", () => {
    extremes(withProducer(PREVIEW_PHYSICS_TIMOSHENKO_CONTRACT_ID));
    expect(boundary().textContent).toBe(`${DISPLACEMENT_EXTREMES_BOUNDARY_TEXT} ${SECTION_ROTATION_NOTE}`);
    cleanup();
    // One straight Timoshenko member beside a realized arc, which records Euler–Bernoulli.
    extremes(overlayResult(rows, exact(arc({ model: "euler_bernoulli" }), section({ model: "timoshenko_annulus_energy_v1" }))));
    expect(boundary().textContent).toBe(`${DISPLACEMENT_EXTREMES_BOUNDARY_TEXT} ${SECTION_ROTATION_NOTE}`);
  });

  it("stays as it is for straight members recording different theories, and for a result that is not solved", () => {
    extremes(overlayResult(rows, exact(section({ model: "euler_bernoulli" }), section({ model: "timoshenko_annulus_energy_v1" }))));
    expect(boundary().textContent).toBe(DISPLACEMENT_EXTREMES_BOUNDARY_TEXT);
    cleanup();
    // A result that is not solved: the panel says so and gives no boundary text, and no sentence.
    const unsolved = { ...withProducer(PREVIEW_PHYSICS_TIMOSHENKO_CONTRACT_ID), status: { ...overlayResult(rows).status, mechanics: "MODEL_INCOMPLETE" } } as MechanicsResult;
    const view = extremes(unsolved);
    expect(screen.getByTestId("displacement-extremes-unavailable").textContent).toBe("Mechanics not solved");
    expect(view.container.textContent).not.toContain(SECTION_ROTATION_NOTE);
  });

  it("stays exactly as it is for an Euler–Bernoulli result, whatever the document states now", () => {
    extremes(withProducer(PREVIEW_PHYSICS_CONTRACT_ID), { ...overlayModel(), ...TIMOSHENKO_DOCUMENT } as PreviewModel);
    expect(boundary().textContent).toBe(DISPLACEMENT_EXTREMES_BOUNDARY_TEXT);
    cleanup();
    extremes(withProducer(PREVIEW_PHYSICS_TIMOSHENKO_CONTRACT_ID), { ...overlayModel(), ...EULER_BERNOULLI_DOCUMENT } as PreviewModel);
    expect(boundary().textContent).toBe(`${DISPLACEMENT_EXTREMES_BOUNDARY_TEXT} ${SECTION_ROTATION_NOTE}`);
  });
});
