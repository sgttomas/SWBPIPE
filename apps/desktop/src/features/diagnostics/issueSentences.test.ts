import { describe, expect, it } from "vitest";
import type { Diagnostic, PreviewModel } from "../../types";
import { plainIssueSentence } from "./issueSentences";
import { demoSession } from "./issueRows.testSessions";

// The reference orders are the engine's (core/product_physics/src pressure_runtime.rs,
// exact_admission.rs, joint.rs); the region and junction ids here are invented.
const model = demoSession().model;
const finding = (code: string, refs: string[], message = "engine text"): Diagnostic => ({ code, severity: "blocking", message, affected_refs: refs });
const sentence = (code: string, refs: string[], message?: string, on: PreviewModel = model) => plainIssueSentence(on, finding(code, refs, message));

function withComponent(id: string, fields: Record<string, unknown>): PreviewModel {
  const copy = structuredClone(model);
  copy.components = [...copy.components, { ...structuredClone(copy.components[0]!), id, ...fields } as PreviewModel["components"][number]];
  return copy;
}

describe("plainIssueSentence", () => {
  it("words each pressure-region refusal from its references", () => {
    expect(sentence("PRESSURE_REGION_NONCOLLINEAR", ["load:L-100", "region:R-1"])).toBe("A straight run in region region:R-1 of load:L-100 is not collinear or backtracks.");
    expect(sentence("PRESSURE_REGION_TOPOLOGY_INVALID", ["load:L-100", "region:R-1"])).toBe("Region region:R-1 of load:L-100 is not one unbroken run between its two terminals.");
    expect(sentence("PRESSURE_REGION_INTERNAL_BRANCH", ["load:L-100", "region:R-1"])).toBe("Region region:R-1 of load:L-100 has a branch inside it. End the region at the branch.");
    expect(sentence("PRESSURE_REGION_BORE_MISMATCH", ["load:L-100", "region:R-1", "pipe:P-110"])).toBe(
      "pipe:P-110 has a different pressure bore from the rest of region region:R-1 of load:L-100. End the region where the bore changes."
    );
    expect(sentence("PRESSURE_REGION_MEMBER_UNKNOWN", ["load:L-100", "region:R-1", "pipe:P-999"])).toBe(
      "Region region:R-1 of load:L-100 lists pipe:P-999, which is not a built pipe of this model."
    );
    expect(sentence("PRESSURE_REGION_MITRE_UNSUPPORTED", ["load:L-100", "region:R-1", "node:N-110", "pipe:P-100", "pipe:P-110"])).toBe(
      "pipe:P-100 and pipe:P-110 meet at node:N-110 at an angle where a bend ends (a mitre), which is not modelled. Make the bend tangent, or end region region:R-1 at node:N-110."
    );
    expect(sentence("EXACT_PRESSURE_REDUCER_IN_REGION", ["load:L-100", "region:R-1", "component:C-130", "pipe:P-130"])).toBe(
      "component:C-130 on pipe:P-130 is inside region region:R-1 of load:L-100 without forming an admitted reducer step. Give pipe:P-130 its own single-member region between two straight regions that transfer to the wall at both ends, or leave it outside every region."
    );
  });

  it("keeps the engine's message where a reference slot is missing or extra", () => {
    expect(sentence("PRESSURE_REGION_NONCOLLINEAR", ["load:L-100"])).toBeNull();
    expect(sentence("PRESSURE_REGION_BORE_MISMATCH", ["load:L-100", "region:R-1"])).toBeNull();
    expect(sentence("PRESSURE_REGION_MITRE_UNSUPPORTED", ["load:L-100", "region:R-1", "node:N-110", "pipe:P-100"])).toBeNull();
    expect(sentence("EXACT_PRESSURE_REDUCER_IN_REGION", ["load:L-100", "region:R-1", "", "pipe:P-130"])).toBeNull();
    expect(sentence("EXACT_PRESSURE_FAMILY_NOT_ADMITTED", ["load:L-100", "equivalent_static"])).toBeNull();
    expect(sentence("LEGACY_FINITE_CONNECTOR_REAUTHOR_REQUIRED", ["component:J-1", "pipe:P-100", "extra"])).toBeNull();
    expect(plainIssueSentence(model, { code: "PRESSURE_REGION_NONCOLLINEAR", severity: "blocking", message: "engine text" })).toBeNull();
    expect(sentence("ANOTHER_CODE", ["load:L-100", "region:R-1"])).toBeNull();
  });

  it("words a family refusal by the component the model holds under that id", () => {
    // The demonstration's C-110 is a bend consumed as geometry only.
    expect(sentence("EXACT_PRESSURE_FAMILY_NOT_ADMITTED", ["component:C-110"])).toBe(
      "component:C-110 is a geometry-only bend and cannot be solved with pressure activated. Realize the bend."
    );
    const annotation = withComponent("component:J-1", { kind: "expansion_joint", mechanics_interface: { solver_consumption: "not_solver_consumed" } });
    expect(sentence("EXACT_PRESSURE_FAMILY_NOT_ADMITTED", ["component:J-1"], undefined, annotation)).toBe(
      "component:J-1 is an expansion joint kept as an annotation only and cannot be solved with pressure activated."
    );
    // An objective connector takes precedence over the kind, as the engine's family does.
    const connector = withComponent("component:J-2", { kind: "bend", objective_connector: { schema_version: "1.0.0" } });
    expect(sentence("EXACT_PRESSURE_FAMILY_NOT_ADMITTED", ["component:J-2"], undefined, connector)).toBe(
      "component:J-2 is a kind of component that cannot yet be solved with pressure activated."
    );
    const realized = withComponent("component:B-2", { kind: "elbow", mechanics_interface: { solver_consumption: "curved_bend_macro_element" } });
    expect(sentence("EXACT_PRESSURE_FAMILY_NOT_ADMITTED", ["component:B-2"], undefined, realized)).toBe(
      "component:B-2 is a kind of component that cannot yet be solved with pressure activated."
    );
    // A support id, or a component the model does not hold.
    expect(sentence("EXACT_PRESSURE_FAMILY_NOT_ADMITTED", ["support:NL-140"])).toBe(
      "support:NL-140 is a kind of component that cannot yet be solved with pressure activated."
    );
  });

  it("names the compensated regions at a shared junction, one or several", () => {
    expect(sentence("EXACT_PRESSURE_SHARED_TERMINAL_JUNCTION_UNSUPPORTED", ["load:L-100", "node:N-120", "region:R-2"])).toBe(
      "Regions meet at node:N-120, and region:R-2 is marked separately supported or compensated there. Where regions meet, every end transfers to the wall."
    );
    expect(sentence("EXACT_PRESSURE_SHARED_TERMINAL_JUNCTION_UNSUPPORTED", ["load:L-100", "node:N-120", "region:R-2", "region:R-3", "region:R-4"])).toBe(
      "Regions meet at node:N-120, and region:R-2, region:R-3 and region:R-4 are marked separately supported or compensated there. Where regions meet, every end transfers to the wall."
    );
    expect(sentence("EXACT_PRESSURE_SHARED_TERMINAL_JUNCTION_UNSUPPORTED", ["load:L-100", "node:N-120"])).toBeNull();
  });

  it("words a legacy joint with or without its pipe", () => {
    const words = "component:J-1 is a legacy or incomplete expansion joint and is not solved. Enter it again as an objective connector.";
    expect(sentence("LEGACY_FINITE_CONNECTOR_REAUTHOR_REQUIRED", ["component:J-1"])).toBe(words);
    expect(sentence("LEGACY_FINITE_CONNECTOR_REAUTHOR_REQUIRED", ["component:J-1", "pipe:P-100"])).toBe(words);
  });

  it("words a joint pressure finding only where its reason comes out whole", () => {
    const refs = ["load:L-100", "region:R-1", "connector:X-1"];
    expect(sentence("JOINT_PRESSURE_INTERFACE_UNRESOLVED", refs, "load case load:L-100, region region:R-1, connector connector:X-1: the replaced span cannot be a member pipe of the region")).toBe(
      "connector:X-1 in region region:R-1 of load:L-100: the replaced span cannot be a member pipe of the region"
    );
    expect(sentence("JOINT_PRESSURE_INTERFACE_UNRESOLVED", [...refs, "pipe:P-110"], "load case load:L-100, region region:R-1, connector connector:X-1: the replaced span cannot be a member pipe of the region")).toBe(
      "connector:X-1 in region region:R-1 of load:L-100: the replaced span cannot be a member pipe of the region"
    );
    // pressure_runtime.rs writes the bare reason.
    expect(sentence("JOINT_PRESSURE_INTERFACE_UNRESOLVED", refs, "listed joint was not built")).toBe("connector:X-1 in region region:R-1 of load:L-100: listed joint was not built");
    // A prefix that does not match the references, or an empty reason, keeps the engine's text.
    expect(sentence("JOINT_PRESSURE_INTERFACE_UNRESOLVED", refs, "load case load:L-200, region region:R-1, connector connector:X-1: a reason")).toBeNull();
    expect(sentence("JOINT_PRESSURE_INTERFACE_UNRESOLVED", refs, "load case load:L-100, region region:R-1, connector connector:X-1: ")).toBeNull();
    expect(sentence("JOINT_PRESSURE_INTERFACE_UNRESOLVED", refs.slice(0, 2), "listed joint was not built")).toBeNull();
  });
});

describe("plainIssueSentence: the solver's codes", () => {
  // The demonstration result's own diagnostics, as the solver wrote them.
  const result = demoSession().resultDiagnostics;
  const first = (code: string) => result.find((item) => item.code === code)!;
  const of = (diagnostic: Diagnostic, on: PreviewModel = model) => plainIssueSentence(on, diagnostic);

  it("words the three empty-model findings, and the load-state form of the missing load", () => {
    expect(sentence("NODE_INPUT_MISSING", ["nodes"])).toBe("The model needs at least two nodes before it can be solved.");
    expect(sentence("PIPE_INPUT_MISSING", ["pipe_segments"])).toBe("The model needs at least one pipe segment before it can be solved.");
    expect(sentence("LOAD_INPUT_MISSING", ["load_cases"], "linear-static preview requires at least one explicit primitive load; no hidden loads are applied")).toBe(
      "The model needs at least one load before it can be solved; no hidden loads are applied."
    );
    expect(sentence("LOAD_INPUT_MISSING", ["load_cases"], "no load case applies an action: every case is a reference state with no listed stored primitive load, boundary motion, pressure region or member eigenstrain, so a solve would publish only all-zero results; no hidden loads are applied")).toBe(
      "No load case applies a load, boundary motion, pressure region, or eigenstrain; no hidden loads are applied."
    );
    expect(sentence("NODE_INPUT_MISSING", ["model"])).toBeNull();
    expect(sentence("PIPE_INPUT_MISSING", ["pipe_segments", "extra"])).toBeNull();
    expect(sentence("LOAD_INPUT_MISSING", [])).toBeNull();
  });

  it("says what the integrity checks found without saying they passed or bound the error", () => {
    const words = of(first("NUMERICAL_INTEGRITY_CHECKS_PASSED"));
    expect(words).toBe("The solver's residual and conditioning checks found no problem with load:L-100; they do not measure the error.");
    for (const forbidden of [/passed/i, /verified/i, /bounded/i]) expect(words).not.toMatch(forbidden);
    // A demoted record keeps the engine's text; so does one naming anything but its case.
    expect(of({ ...first("NUMERICAL_INTEGRITY_CHECKS_PASSED"), severity: "warning" })).toBeNull();
    expect(of({ ...first("NUMERICAL_INTEGRITY_CHECKS_PASSED"), affected_refs: ["load:L-100", "extra"] })).toBeNull();
  });

  it("says a nonlinear support's end state and the loop's count as the solver loop reported them", () => {
    const state = of(first("NONLINEAR_SUPPORT_STATE_REVIEW"));
    expect(state).toBe("Nonlinear support support:NL-130-FRIC ended in the sliding state in load:L-100 (reported by the solver loop only).");
    const loop = of(first("NONLINEAR_SUPPORT_LOOP_CONVERGED"));
    expect(loop).toBe("Nonlinear support solver loop for load:L-100 completed in 2 iterations (final residual count 0).");
    for (const words of [state, loop]) for (const forbidden of [/confirmed/i, /established/i, /verified/i]) expect(words).not.toMatch(forbidden);
    const one = { ...first("NONLINEAR_SUPPORT_LOOP_CONVERGED"), message: "sparse_interactive nonlinear support active-set preview completed 1 iteration(s); final residual count 3; rest" };
    expect(of(one)).toBe("Nonlinear support solver loop for load:L-100 completed in 1 iteration (final residual count 3).");
    // Markers or references that do not match keep the engine's text.
    expect(of({ ...first("NONLINEAR_SUPPORT_STATE_REVIEW"), affected_refs: ["support:NL-140", "load:L-100"] })).toBeNull();
    expect(of({ ...first("NONLINEAR_SUPPORT_STATE_REVIEW"), message: "nonlinear support support:NL-130-FRIC ended in  state for load case load:L-100; x" })).toBeNull();
    expect(of({ ...first("NONLINEAR_SUPPORT_LOOP_CONVERGED"), affected_refs: ["load:L-100", "DEC-047"] })).toBeNull();
    expect(of({ ...first("NONLINEAR_SUPPORT_LOOP_CONVERGED"), message: "completed two iterations" })).toBeNull();
  });

  it("words each hanger-load finding at its field, so a support's loads join at the member", () => {
    expect(of(first("SPRING_HANGER_LOAD_NOT_APPLIED_AS_PRELOAD"))).toBe(
      "Variable spring hanger support:SH-140: entered installed load (hanger.installed_load) — not applied as preload; treated as a linear spring from zero load (review evidence only)."
    );
    expect(sentence("SPRING_HANGER_LOAD_NOT_APPLIED_AS_PRELOAD", ["support:SH-140", "hanger.travel"])).toBeNull();
    expect(sentence("SPRING_HANGER_LOAD_NOT_APPLIED_AS_PRELOAD", ["support:SH-140"])).toBeNull();
  });

  it("words the constant-effort and user-data findings", () => {
    expect(of(first("CONSTANT_EFFORT_NOT_CONSUMED"))).toBe("Constant-effort support support:CE-120 is not applied or consumed by the solve; no support action is published for it.");
    expect(of(first("SUPPORT_CONSTANT_EFFORT_NOT_CONSUMED"))).toBe(
      "Constant-effort support support:CE-120 at node:N-120 is not applied or consumed by the solve (no translational restraint DOF is declared, so no acting direction is user-entered); it stays review evidence only."
    );
    expect(of({ ...first("SUPPORT_CONSTANT_EFFORT_NOT_CONSUMED"), message: "constant-effort support is not consumed" })).toBeNull();
    expect(of(first("SPRING_HANGER_USER_DATA_REVIEWED"))).toBe(
      "Variable spring hanger support:SH-140 records explicit user-entered stiffness, load, travel, and source data (no catalog default or verification)."
    );
    expect(of(first("CONSTANT_EFFORT_USER_DATA_REVIEWED"))).toBe(
      "Constant-effort support support:CE-120 records explicit user-entered constant load, travel, and source data (no catalog default or verification)."
    );
    expect(sentence("SPRING_HANGER_USER_DATA_REVIEWED", ["support:SH-140", "hanger.stiffness"])).toBeNull();
    expect(sentence("CONSTANT_EFFORT_NOT_CONSUMED", ["support:CE-120", "node:N-120"])).toBeNull();
  });

  it("words a geometry-only bend by the model's bend, as the engine chooses its text", () => {
    const lead = "component:C-110 is a geometry-only bend solved as a straight chord without flexibility factor k";
    expect(of(first("BEND_GEOMETRY_ONLY_SOLVED_AS_CHORD"))).toBe(`${lead} (scalar SIF is applied as equal-factor intensification). Realize the bend to apply k.`);
    const noSif = structuredClone(model);
    noSif.components[0]!.modifiers = { ...noSif.components[0]!.modifiers, sif_user_value: undefined };
    expect(of(first("BEND_GEOMETRY_ONLY_SOLVED_AS_CHORD"), noSif)).toBe(`${lead} (no scalar SIF entered). Realize the bend to apply k.`);
    const other = structuredClone(model);
    other.components[0]!.mechanics_interface = { solver_consumption: "invented_other_consumption" };
    expect(of(first("BEND_GEOMETRY_ONLY_SOLVED_AS_CHORD"), other)).toBe(`${lead} (its scalar SIF is not applied). Realize the bend to apply k.`);
    expect(sentence("BEND_GEOMETRY_ONLY_SOLVED_AS_CHORD", ["component:C-999"])).toBeNull();
  });

  it("words the rigid scale and the intensification from the values the engine wrote", () => {
    expect(of(first("RIGID_COMPONENT_STIFFNESS_SCALING_APPLIED"))).toBe(
      "Rigid component component:C-130 applies user-entered stiffness scale 15 to pipe pipe:P-130; published stresses are on the mapped pipe section, not the component body."
    );
    expect(of({ ...first("RIGID_COMPONENT_STIFFNESS_SCALING_APPLIED"), message: "scaled" })).toBeNull();
    expect(of(first("COMPONENT_EQUAL_FACTOR_INTENSIFICATION_APPLIED"))).toBe(
      "component:C-110 applies user-entered bend SIF 1.15 to bending stress on pipe:P-100 in load:L-100 (no flexibility factor, axial, or torsion term; not a code stress)."
    );
    const intensification = first("COMPONENT_EQUAL_FACTOR_INTENSIFICATION_APPLIED");
    expect(of({ ...intensification, affected_refs: intensification.affected_refs!.slice(0, 4) })).toBeNull();
    expect(of({ ...intensification, message: "applies a factor" })).toBeNull();
  });
});
