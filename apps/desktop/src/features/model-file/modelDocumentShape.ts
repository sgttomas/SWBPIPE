// The desktop model document shape: the members the desktop's `PreviewModel`
// type requires and the desktop itself always writes (projectService's blank
// documents, saved projects). A model document opened from disk must carry
// them. A document that lacks one is refused by name with its JSON path; the
// desktop never fills a default in, because the session's model hash must be
// the hash of the document the user opened. Physics validity is the solver's:
// it reports its own named diagnostics on solve. As a last guard the session
// also builds the model index for the candidate and refuses it if that throws.
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
    need(project.units, "project.units", "object");
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
      if (member === "supports" && need(record.restraints, `${at}.restraints`, "array")) {
        (record.restraints as unknown[]).forEach((value, item) => need(value, `${at}.restraints[${item}]`, "string"));
      }
      if (member === "combinations" && need(record.terms, `${at}.terms`, "array")) {
        (record.terms as unknown[]).forEach((term, item) => {
          const termAt = `${at}.terms[${item}]`;
          if (!need(term, termAt, "object")) return;
          need((term as Record<string, unknown>).load_case, `${termAt}.load_case`, "string");
          need((term as Record<string, unknown>).factor, `${termAt}.factor`, "number");
        });
      }
    });
  }
  return found;
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
