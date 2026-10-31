// The desktop model document shape: the members the desktop's `PreviewModel`
// type requires and the desktop itself always writes (projectService's blank
// documents, saved projects). A model document opened from disk must carry
// them. A document that lacks one is refused by name with its JSON path; the
// desktop never fills a default in, because the session's model hash must be
// the hash of the document the user opened. Physics validity is the solver's:
// it reports its own named diagnostics on solve. Every member the model index
// reads with a string method is checked here (`pipe_segments[].section_ref`), as
// is every member the workspace dereferences on adoption (`project.units.*`,
// `components[].completeness`, the component descriptors, `diagnostics[].id`; pinned by
// `modelDocumentAdoption.test.tsx`), so a shape-valid document indexes and renders; as a last guard the session still builds the
// model index for the candidate and refuses it if that throws. The native save
// runs a port of this check (`src-tauri/src/model_document_shape.rs`).
import type { PreviewModel } from "../../types";
import { loadedWasmEngine, WASM_ENGINE_ABSENT_DIAGNOSTIC, WASM_ENGINE_BUILD_COMMAND } from "../../services/wasmEngine/loadWasmEngine";

export type ModelFileDiagnostic = { code: string; path: string; message: string };

export const MODEL_FILE_SHAPE = "MODEL-FILE-SHAPE";

type Kind = "string" | "object" | "array" | "number";

const ENTITY_STRINGS: ReadonlyArray<[member: string, required: ReadonlyArray<string>, optional?: boolean]> = [
  ["materials", ["id", "label", "provenance"], true],
  ["sections", ["id", "name", "section_type"], true],
  ["nodes", ["id", "label", "provenance"]],
  ["pipe_segments", ["id", "label", "from", "to", "material", "provenance"]],
  ["supports", ["id", "label", "node", "provenance"]],
  ["components", ["id", "label", "kind", "node", "provenance"]],
  ["load_cases", ["id", "label", "kind", "status", "provenance"]],
  ["combinations", ["id", "label", "basis", "provenance"], true],
  ["diagnostics", ["code", "severity", "message"]]
];

/** The combination bases whose operands are not terms (`minuend_id`/`subtrahend_id`, `operand_ids`). */
const TERMLESS_BASES: ReadonlySet<string> = new Set(["result_state_subtraction", "range_envelope"]);

/** The component descriptor members the workspace formats or trims on adoption
 * (the model tree's search text, the rule-check and editor-contract panels), typed
 * as `PreviewComponent` declares them. A present, non-null member must be of its type. */
const COMPONENT_DESCRIPTORS: Record<string, Record<string, "string" | "quantity" | "point">> = {
  geometry: { bend_radius: "quantity", bend_angle: "quantity", bend_plane_orientation: "string", bend_pipe_ref: "string", bend_geometry_source_reference: "string", branch_header_pipe_ref: "string", branch_branch_pipe_ref: "string", branch_run_size: "quantity", branch_header_size: "quantity", branch_connection_angle: "quantity", branch_connection_type: "string", branch_reinforcement_area: "quantity", branch_reinforcement_reference: "string", branch_geometry_source_reference: "string", rigid_pipe_ref: "string", rigid_body_length: "quantity", end_a_size: "quantity", end_b_size: "quantity", weight: "quantity", center_of_gravity: "point", eccentric_offset: "point", connection_end_a_reference: "string", connection_end_b_reference: "string", stiffness_behavior_reference: "string", rigid_component_source_reference: "string", expansion_joint_pipe_ref: "string", effective_area: "quantity", movement_limit: "quantity", hardware_reference: "string", manufacturer_reference: "string", pressure_thrust_reference: "string", expansion_joint_source_reference: "string" },
  modifiers: { sif_user_value: "quantity", branch_header_sif_user_value: "quantity", branch_branch_sif_user_value: "quantity", flexibility_factor_user_value: "quantity", sif_in_plane_user_value: "quantity", sif_out_of_plane_user_value: "quantity", stiffness_scaling_user_value: "quantity", linear_stiffness_user_value: "quantity", rotational_stiffness_user_value: "quantity", axial_stiffness_user_value: "quantity", lateral_stiffness_user_value: "quantity", angular_stiffness_user_value: "quantity", torsional_stiffness_user_value: "quantity", source_reference: "string" },
  mechanics_interface: { solver_consumption: "string", rule_check_consumption: "string" },
};

function kindOf(value: unknown): Kind | "null" | "other" {
  if (value === null) return "null";
  if (Array.isArray(value)) return "array";
  if (typeof value === "string") return "string";
  if (typeof value === "number" && Number.isFinite(value)) return "number";
  if (typeof value === "object") return "object";
  return "other";
}

/** Named shape diagnostics for `document`; an empty list means the desktop can open it. */
export function modelDocumentShapeDiagnostics(document: unknown): ModelFileDiagnostic[] {
  const found: ModelFileDiagnostic[] = [];
  const need = (value: unknown, path: string, kind: Kind) => {
    const actual = kindOf(value);
    if (actual === kind) return true;
    found.push({
      code: MODEL_FILE_SHAPE,
      path,
      message: value === undefined ? `${path} is missing (${kind} required).` : `${path} is ${actual}; ${kind} required.`
    });
    return false;
  };
  const quantity = (value: unknown, path: string) => {
    if (!need(value, path, "object")) return;
    need((value as Record<string, unknown>).value, `${path}.value`, "number");
    need((value as Record<string, unknown>).unit, `${path}.unit`, "string");
  };
  if (!need(document, "$", "object")) return found;
  const doc = document as Record<string, unknown>;
  need(doc.schema_version, "schema_version", "string");
  need(doc.document_kind, "document_kind", "string");
  if (need(doc.data_boundary, "data_boundary", "object")) {
    for (const [key, value] of Object.entries(doc.data_boundary as Record<string, unknown>)) need(value, `data_boundary.${key}`, "string");
  }
  if (need(doc.project, "project", "object")) {
    const project = doc.project as Record<string, unknown>;
    for (const key of ["id", "name", "description"]) need(project[key], `project.${key}`, "string");
    // Every unit symbol is a string: the unit catalogue trims each (`normalizeSymbol`).
    if (need(project.units, "project.units", "object")) {
      for (const [key, value] of Object.entries(project.units as Record<string, unknown>)) need(value, `project.units.${key}`, "string");
    }
  }
  if (need(doc.analysis_status, "analysis_status", "object")) {
    const status = doc.analysis_status as Record<string, unknown>;
    for (const key of ["mechanics", "rule_check", "professional_acceptance"]) need(status[key], `analysis_status.${key}`, "string");
  }
  for (const [member, required, optional] of ENTITY_STRINGS) {
    const list = doc[member];
    if (optional && list === undefined) continue;
    if (!need(list, member, "array")) continue;
    (list as unknown[]).forEach((entry, index) => {
      const at = `${member}[${index}]`;
      if (!need(entry, at, "object")) return;
      const record = entry as Record<string, unknown>;
      for (const key of required) need(record[key], `${at}.${key}`, "string");
      if (member === "nodes" && need(record.position, `${at}.position`, "object")) {
        for (const axis of ["x", "y", "z"]) need((record.position as Record<string, unknown>)[axis], `${at}.position.${axis}`, "number");
      }
      if (member === "materials") quantity(record.elastic_modulus, `${at}.elastic_modulus`);
      if (member === "sections" && need(record.properties, `${at}.properties`, "object")) {
        for (const [key, value] of Object.entries(record.properties as Record<string, unknown>)) quantity(value, `${at}.properties.${key}`);
      }
      if (member === "sections" && kindOf(record.provenance) !== "string" && kindOf(record.provenance) !== "object") {
        need(record.provenance, `${at}.provenance`, "string");
      }
      if (member === "pipe_segments" && need(record.section, `${at}.section`, "object")) {
        for (const [key, value] of Object.entries(record.section as Record<string, unknown>)) quantity(value, `${at}.section.${key}`);
      }
      // A present shared-section reference is a string: the model index trims it
      // (`modelIndex.ts` buildModelIndex). Absent or null opens without one.
      if (member === "pipe_segments" && record.section_ref !== undefined && record.section_ref !== null) {
        need(record.section_ref, `${at}.section_ref`, "string");
      }
      // The model tree maps a present completeness list and formats each finding's code and status.
      if (member === "components" && record.completeness !== undefined && record.completeness !== null
        && need(record.completeness, `${at}.completeness`, "array")) {
        (record.completeness as unknown[]).forEach((finding, item) => {
          const findingAt = `${at}.completeness[${item}]`;
          if (!need(finding, findingAt, "object")) return;
          need((finding as Record<string, unknown>).diagnostic_code, `${findingAt}.diagnostic_code`, "string");
          need((finding as Record<string, unknown>).status, `${findingAt}.status`, "string");
        });
      }
      if (member === "components") {
        for (const [group, members] of Object.entries(COMPONENT_DESCRIPTORS)) {
          const descriptor = record[group];
          if (kindOf(descriptor) !== "object") continue;
          for (const [key, type] of Object.entries(members)) {
            const value = (descriptor as Record<string, unknown>)[key];
            if (value === undefined || value === null) continue;
            const path = `${at}.${group}.${key}`;
            if (type === "string") need(value, path, "string");
            else if (type === "quantity") quantity(value, path);
            else if (need(value, path, "object")) {
              for (const axis of ["x", "y", "z"]) need((value as Record<string, unknown>)[axis], `${path}.${axis}`, "number");
              need((value as Record<string, unknown>).unit, `${path}.unit`, "string");
            }
          }
        }
      }
      // A present diagnostic id is the tree row's id, rendered as text (absent or null falls back to the code).
      if (member === "diagnostics" && record.id !== undefined && record.id !== null) {
        need(record.id, `${at}.id`, "string");
      }
      if (member === "supports" && need(record.restraints, `${at}.restraints`, "array")) {
        (record.restraints as unknown[]).forEach((value, item) => need(value, `${at}.restraints[${item}]`, "string"));
      }
      // A term names exactly one operand, a load case or (model document 0.9.0) a
      // combination, as a string; the views interpolate it with its factor. A
      // subtraction or range envelope names its operands in its own members and
      // may omit `terms` (the views show it absent); every other basis carries it.
      const termsOptional = record.terms === undefined && TERMLESS_BASES.has(record.basis as string);
      if (member === "combinations" && !termsOptional && need(record.terms, `${at}.terms`, "array")) {
        (record.terms as unknown[]).forEach((term, item) => {
          const termAt = `${at}.terms[${item}]`;
          if (!need(term, termAt, "object")) return;
          const operand = term as Record<string, unknown>;
          const named = (["load_case", "combination"] as const).filter((key) => operand[key] !== undefined);
          if (named.length === 0) {
            found.push({ code: MODEL_FILE_SHAPE, path: termAt, message: `${termAt} names neither load_case nor combination (exactly one string required).` });
          } else if (named.length === 2) {
            found.push({ code: MODEL_FILE_SHAPE, path: termAt, message: `${termAt} names both load_case and combination (exactly one required).` });
          }
          for (const key of named) need(operand[key], `${termAt}.${key}`, "string");
          need(operand.factor, `${termAt}.factor`, "number");
        });
      }
      // The subtraction and envelope operands, interpolated and joined by the load
      // manager's rows: a present, non-null member is a string, or an array of them.
      if (member === "combinations") {
        for (const key of ["minuend_id", "subtrahend_id", "mode"]) {
          if (record[key] !== undefined && record[key] !== null) need(record[key], `${at}.${key}`, "string");
        }
        if (record.operand_ids !== undefined && record.operand_ids !== null && need(record.operand_ids, `${at}.operand_ids`, "array")) {
          (record.operand_ids as unknown[]).forEach((value, item) => need(value, `${at}.operand_ids[${item}]`, "string"));
        }
      }
    });
  }
  return found;
}

/** The shape check's `project.units` findings alone (W-17): the map itself is
 * required at every entry into the session (File > Open Model Document, an
 * opened local project, an opened saved state); a missing key is not a finding,
 * and the Project page offers to set it. */
export function projectUnitsShapeDiagnostics(document: unknown): ModelFileDiagnostic[] {
  return modelDocumentShapeDiagnostics(document).filter((item) => item.path === "project.units" || item.path.startsWith("project.units."));
}

/** One refusal text naming each `project.units` finding, or null when there is none. */
export function projectUnitsShapeRefusal(document: unknown): string | null {
  const found = projectUnitsShapeDiagnostics(document);
  if (found.length === 0) return null;
  return `Model document refused: ${found.map((item) => `${item.code} at ${item.path}: ${item.message}`).join(" | ")}`;
}

export function isDesktopModelDocument(document: unknown): document is PreviewModel {
  return modelDocumentShapeDiagnostics(document).length === 0;
}

/** The one document validator's findings for `document`, split by severity. */
export type ModelDocumentSlotValidation = { blocking: ModelFileDiagnostic[]; warnings: ModelFileDiagnostic[] };

/** One finding: the validator's own (`origin: "validator"`), or the engine's
 * failure to judge the document (`origin: "engine"`: the engine is not loaded
 * or reported an input error), which is blocking because nothing was checked. */
export type ModelDocumentSlotFinding = ModelFileDiagnostic & {
  severity: "blocking" | "warning";
  affected_refs: string[];
  origin: "validator" | "engine";
};

type ValidatorFinding = { code: string; severity: string; path: string; message: string; affected_refs?: string[] };

/**
 * W-13 §5.3: the model-data slots (tags, notes, lines, assumptions, constraints,
 * equipment interfaces, rule-pack references, provenance records), checked by the
 * applier's Rust validator through the wasm engine. The webview never re-implements
 * it; without the engine, or when the engine reports an input error, the result is
 * a blocking finding that names why, never an unchecked pass.
 */
export function modelDocumentSlotFindings(document: unknown): ModelDocumentSlotFinding[] {
  const engine = loadedWasmEngine();
  if (!engine) {
    return [{
      code: WASM_ENGINE_ABSENT_DIAGNOSTIC,
      severity: "blocking",
      path: "",
      message: `The document validator is in the wasm engine, which is not loaded; build it with \`${WASM_ENGINE_BUILD_COMMAND}\`.`,
      affected_refs: [],
      origin: "engine"
    }];
  }
  const report = JSON.parse(engine.validateModelDocumentJson(JSON.stringify(document))) as
    { diagnostics?: ValidatorFinding[]; error?: { code: string; message: string } };
  if (report.error) {
    return [{ code: report.error.code, severity: "blocking", path: "", message: report.error.message, affected_refs: [], origin: "engine" }];
  }
  return (report.diagnostics ?? []).map((item) => ({
    code: item.code,
    severity: item.severity === "blocking" ? "blocking" : "warning",
    path: item.path,
    message: item.message,
    affected_refs: item.affected_refs ?? [],
    origin: "validator"
  }));
}

export function modelDocumentSlotValidation(document: unknown): ModelDocumentSlotValidation {
  const findings = modelDocumentSlotFindings(document);
  const named = (severity: string) => findings
    .filter((item) => item.severity === severity)
    .map(({ code, path, message }) => ({ code, path, message }));
  return { blocking: named("blocking"), warnings: named("warning") };
}
