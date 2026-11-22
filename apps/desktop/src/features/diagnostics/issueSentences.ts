// Plain sentences for findings whose engine text reads as program text: eleven pressure-region and
// joint codes of the exact pressure route, and fourteen solver codes. Each sentence is formed from the
// finding's code and its typed references (in the slot order the engine writes them), and where it
// carries a value or a reason, from text found strictly between the engine's own fixed markers. A
// finding whose references or markers do not match keeps the engine's own message. The row that shows
// a plain sentence keeps the engine's message readable when opened ("Also stated as"). No sentence
// says a check passed or that a result is verified, bounded, confirmed or established: the solver's
// messages rest on its own error estimate, which SWBPIPE does not check.
//
// Reference orders, as written by core/product_physics/src:
// - pressure_runtime.rs: NONCOLLINEAR, TOPOLOGY_INVALID and INTERNAL_BRANCH [case, region];
//   BORE_MISMATCH [case, region, pipe]; MEMBER_UNKNOWN [case, region, member]; MITRE_UNSUPPORTED
//   [case, region, node, pipe_in, pipe_out]; SHARED_TERMINAL_JUNCTION [case, node, ...compensated
//   regions]; JOINT_PRESSURE_INTERFACE_UNRESOLVED [case, region, joint] with a bare reason;
// - exact_admission.rs: FAMILY_NOT_ADMITTED [component or support]; REDUCER_IN_REGION
//   [case, region, reducer, pipe];
// - joint.rs: LEGACY_FINITE_CONNECTOR_REAUTHOR_REQUIRED [joint] or [joint, pipe];
//   JOINT_PRESSURE_INTERFACE_UNRESOLVED [case, region, connector(, member)] with the message
//   "load case {case}, region {region}, connector {connector}: {reason}".
// Solver codes, core/product_physics/src: validation.rs NODE/PIPE/LOAD_INPUT_MISSING ["nodes"],
// ["pipe_segments"], ["load_cases"] (and case_state/resolve.rs's "no load case applies an action:"
// form); lib.rs NUMERICAL_INTEGRITY_CHECKS_PASSED [case] at severity info, NONLINEAR_SUPPORT_STATE_REVIEW
// [support, case], NONLINEAR_SUPPORT_LOOP_CONVERGED [case, "DEC-046"], SUPPORT_CONSTANT_EFFORT_NOT_CONSUMED
// [support, node]; validation.rs SPRING_HANGER_LOAD_NOT_APPLIED_AS_PRELOAD [support, field],
// SPRING_HANGER_USER_DATA_REVIEWED and CONSTANT_EFFORT_USER_DATA_REVIEWED [support, "hanger"],
// BEND_GEOMETRY_ONLY_SOLVED_AS_CHORD [component] (`bend_geometry_only_on_preview`); preview_physics.rs
// CONSTANT_EFFORT_NOT_CONSUMED [support] and COMPONENT_EQUAL_FACTOR_INTENSIFICATION_APPLIED
// [component, pipe, row, source, case]; rigid_component.rs RIGID_COMPONENT_STIFFNESS_SCALING_APPLIED
// [component, pipe].
import type { Diagnostic, PreviewModel } from "../../types";

function joinNames(names: readonly string[]): string {
  return names.length <= 1 ? names.join("") : `${names.slice(0, -1).join(", ")} and ${names[names.length - 1]}`;
}

/** The plain sentence for one exact-pressure family refusal, mirroring `exact_admission.rs`
 * `component_family` on the model's component of that id; null where the engine's text should stand. */
function familySentence(model: PreviewModel, id: string): string {
  const component = model.components.find((item) => item.id === id) as
    | (PreviewModel["components"][number] & { objective_connector?: unknown })
    | undefined;
  const generic = `${id} is a kind of component that cannot yet be solved with pressure activated.`;
  if (!component || component.objective_connector != null) return generic;
  const consumption = component.mechanics_interface?.solver_consumption;
  if ((component.kind === "bend" || component.kind === "elbow") && consumption !== "curved_bend_macro_element") {
    return `${id} is a geometry-only bend and cannot be solved with pressure activated. Realize the bend.`;
  }
  if (component.kind === "expansion_joint" && consumption === "not_solver_consumed") {
    return `${id} is an expansion joint kept as an annotation only and cannot be solved with pressure activated.`;
  }
  return generic;
}

/** The reason clause of a joint pressure finding, where it is recovered whole: the text after the
 * engine's exact prefix, or the whole message where the engine wrote the reason alone. */
function jointReason(message: string, refs: readonly string[]): string | null {
  const [lcase, region, connector] = refs;
  const prefix = `load case ${lcase}, region ${region}, connector ${connector}: `;
  if (message.startsWith(prefix)) return message.slice(prefix.length) || null;
  // pressure_runtime.rs writes the bare reason (three references, no prefix).
  if (refs.length === 3 && !message.startsWith("load case ")) return message || null;
  return null;
}

/** The text strictly between two markers of `message`, or null where either is missing or it is empty. */
function between(message: string, before: string, after: string): string | null {
  const start = message.indexOf(before);
  if (start === -1) return null;
  const from = start + before.length;
  const end = message.indexOf(after, from);
  return end === -1 || end === from ? null : message.slice(from, end);
}

const HANGER_FIELDS: Readonly<Record<string, string>> = {
  "hanger.installed_load": "installed load (hanger.installed_load)",
  "hanger.cold_load": "cold load (hanger.cold_load)",
  "hanger.hot_load": "hot load (hanger.hot_load)"
};

/** A geometry-only bend's sentence, mirroring validation.rs `bend_geometry_only_on_preview` on the
 * model's component of that id; null where the model holds no such bend or it is realized. */
function bendSentence(model: PreviewModel, id: string): string | null {
  const component = model.components.find((item) => item.id === id);
  if (!component) return null;
  const consumption = component.mechanics_interface?.solver_consumption ?? "mechanics_geometry_only";
  if (consumption === "curved_bend_macro_element") return null;
  const sif = component.modifiers?.sif_user_value?.value;
  const scalar = typeof sif === "number" && Number.isFinite(sif) && sif > 0;
  const lead = `${id} is a geometry-only bend solved as a straight chord without flexibility factor k`;
  if (!scalar) return `${lead} (no scalar SIF entered). Realize the bend to apply k.`;
  return consumption === "mechanics_geometry_only"
    ? `${lead} (scalar SIF is applied as equal-factor intensification). Realize the bend to apply k.`
    : `${lead} (its scalar SIF is not applied). Realize the bend to apply k.`;
}

/** The plain sentence for a finding, or null where the engine's own message stands. */
export function plainIssueSentence(model: PreviewModel, diagnostic: Diagnostic): string | null {
  const refs = diagnostic.affected_refs ?? [];
  const exactly = (count: number) => refs.length === count && refs.every((ref) => typeof ref === "string" && ref.length > 0);
  switch (diagnostic.code) {
    case "PRESSURE_REGION_NONCOLLINEAR":
      return exactly(2) ? `A straight run in region ${refs[1]} of ${refs[0]} is not collinear or backtracks.` : null;
    case "PRESSURE_REGION_TOPOLOGY_INVALID":
      return exactly(2) ? `Region ${refs[1]} of ${refs[0]} is not one unbroken run between its two terminals.` : null;
    case "PRESSURE_REGION_INTERNAL_BRANCH":
      return exactly(2) ? `Region ${refs[1]} of ${refs[0]} has a branch inside it. End the region at the branch.` : null;
    case "PRESSURE_REGION_BORE_MISMATCH":
      return exactly(3) ? `${refs[2]} has a different pressure bore from the rest of region ${refs[1]} of ${refs[0]}. End the region where the bore changes.` : null;
    case "PRESSURE_REGION_MEMBER_UNKNOWN":
      return exactly(3) ? `Region ${refs[1]} of ${refs[0]} lists ${refs[2]}, which is not a built pipe of this model.` : null;
    case "PRESSURE_REGION_MITRE_UNSUPPORTED":
      return exactly(5)
        ? `${refs[3]} and ${refs[4]} meet at ${refs[2]} at an angle where a bend ends (a mitre), which is not modelled. Make the bend tangent, or end region ${refs[1]} at ${refs[2]}.`
        : null;
    case "EXACT_PRESSURE_FAMILY_NOT_ADMITTED":
      return exactly(1) ? familySentence(model, refs[0]!) : null;
    case "EXACT_PRESSURE_SHARED_TERMINAL_JUNCTION_UNSUPPORTED": {
      if (refs.length < 3 || !refs.every((ref) => typeof ref === "string" && ref.length > 0)) return null;
      const compensated = refs.slice(2);
      return `Regions meet at ${refs[1]}, and ${joinNames(compensated)} ${compensated.length === 1 ? "is" : "are"} marked separately supported or compensated there. Where regions meet, every end transfers to the wall.`;
    }
    case "LEGACY_FINITE_CONNECTOR_REAUTHOR_REQUIRED":
      return exactly(1) || exactly(2) ? `${refs[0]} is a legacy or incomplete expansion joint and is not solved. Enter it again as an objective connector.` : null;
    case "JOINT_PRESSURE_INTERFACE_UNRESOLVED": {
      if (!(exactly(3) || exactly(4))) return null;
      const reason = jointReason(diagnostic.message, refs);
      return reason === null ? null : `${refs[2]} in region ${refs[1]} of ${refs[0]}: ${reason}`;
    }
    case "EXACT_PRESSURE_REDUCER_IN_REGION":
      return exactly(4)
        ? `${refs[2]} on ${refs[3]} is inside region ${refs[1]} of ${refs[0]} without forming an admitted reducer step. Give ${refs[3]} its own single-member region between two straight regions that transfer to the wall at both ends, or leave it outside every region.`
        : null;
    case "NODE_INPUT_MISSING":
      return exactly(1) && refs[0] === "nodes" ? "The model needs at least two nodes before it can be solved." : null;
    case "PIPE_INPUT_MISSING":
      return exactly(1) && refs[0] === "pipe_segments" ? "The model needs at least one pipe segment before it can be solved." : null;
    case "LOAD_INPUT_MISSING":
      if (!(exactly(1) && refs[0] === "load_cases")) return null;
      return diagnostic.message.startsWith("no load case applies an action:")
        ? "No load case applies a load, boundary motion, pressure region, or eigenstrain; no hidden loads are applied."
        : "The model needs at least one load before it can be solved; no hidden loads are applied.";
    case "NUMERICAL_INTEGRITY_CHECKS_PASSED":
      return exactly(1) && diagnostic.severity === "info"
        ? `The solver's residual and conditioning checks found no problem with ${refs[0]}; they do not measure the error.`
        : null;
    case "NONLINEAR_SUPPORT_STATE_REVIEW": {
      if (!exactly(2)) return null;
      const state = between(diagnostic.message, `nonlinear support ${refs[0]} ended in `, ` state for load case ${refs[1]}; `);
      return state === null ? null : `Nonlinear support ${refs[0]} ended in the ${state} state in ${refs[1]} (reported by the solver loop only).`;
    }
    case "NONLINEAR_SUPPORT_LOOP_CONVERGED": {
      if (!(exactly(2) && refs[1] === "DEC-046")) return null;
      const found = /nonlinear support active-set preview completed (\d+) iteration\(s\); final residual count (\d+);/.exec(diagnostic.message);
      if (!found) return null;
      const [, count, residual] = found;
      return `Nonlinear support solver loop for ${refs[0]} completed in ${count} ${count === "1" ? "iteration" : "iterations"} (final residual count ${residual}).`;
    }
    case "SPRING_HANGER_LOAD_NOT_APPLIED_AS_PRELOAD": {
      if (!exactly(2) || !Object.hasOwn(HANGER_FIELDS, refs[1]!)) return null;
      return `Variable spring hanger ${refs[0]}: entered ${HANGER_FIELDS[refs[1]!]} — not applied as preload; treated as a linear spring from zero load (review evidence only).`;
    }
    case "CONSTANT_EFFORT_NOT_CONSUMED":
      return exactly(1) ? `Constant-effort support ${refs[0]} is not applied or consumed by the solve; no support action is published for it.` : null;
    case "SUPPORT_CONSTANT_EFFORT_NOT_CONSUMED": {
      if (!exactly(2)) return null;
      const reason = between(diagnostic.message, "remains user-entered review evidence: ", "; assembled-solve consumption requires ");
      return reason === null ? null : `Constant-effort support ${refs[0]} at ${refs[1]} is not applied or consumed by the solve (${reason}); it stays review evidence only.`;
    }
    case "BEND_GEOMETRY_ONLY_SOLVED_AS_CHORD":
      return exactly(1) ? bendSentence(model, refs[0]!) : null;
    case "RIGID_COMPONENT_STIFFNESS_SCALING_APPLIED": {
      if (!exactly(2)) return null;
      const scale = between(diagnostic.message, "applies its user-entered stiffness scale ", " to mapped member ");
      return scale === null ? null : `Rigid component ${refs[0]} applies user-entered stiffness scale ${scale} to pipe ${refs[1]}; published stresses are on the mapped pipe section, not the component body.`;
    }
    case "COMPONENT_EQUAL_FACTOR_INTENSIFICATION_APPLIED": {
      if (!exactly(5)) return null;
      const found = /applies the user-entered (.+?) SIF ([^ ]+) to the member-Z bending stress at /.exec(diagnostic.message);
      if (!found) return null;
      const [, side, sif] = found;
      return `${refs[0]} applies user-entered ${side} SIF ${sif} to bending stress on ${refs[1]} in ${refs[4]} (no flexibility factor, axial, or torsion term; not a code stress).`;
    }
    case "SPRING_HANGER_USER_DATA_REVIEWED":
      return exactly(2) && refs[1] === "hanger"
        ? `Variable spring hanger ${refs[0]} records explicit user-entered stiffness, load, travel, and source data (no catalog default or verification).`
        : null;
    case "CONSTANT_EFFORT_USER_DATA_REVIEWED":
      return exactly(2) && refs[1] === "hanger"
        ? `Constant-effort support ${refs[0]} records explicit user-entered constant load, travel, and source data (no catalog default or verification).`
        : null;
    default:
      return null;
  }
}
