// The desktop model document shape: the members the desktop's `PreviewModel`
// type requires and the desktop itself always writes (projectService's blank
// documents, saved projects). A model document opened from disk must carry
// them. A document that lacks one is refused by name with its JSON path; the
// desktop never fills a default in, because the session's model hash must be
// the hash of the document the user opened. Physics validity is the solver's:
// it reports its own named diagnostics on solve.
import type { PreviewModel } from "../../types";

export type ModelFileDiagnostic = { code: string; path: string; message: string };

export const MODEL_FILE_SHAPE = "MODEL-FILE-SHAPE";

type Kind = "string" | "object" | "array" | "number";

const ENTITY_STRINGS: ReadonlyArray<[member: string, required: ReadonlyArray<string>, optional?: boolean]> = [
  ["materials", ["id", "label", "provenance"], true],
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
      if (member === "pipe_segments") need(record.section, `${at}.section`, "object");
      if (member === "supports") need(record.restraints, `${at}.restraints`, "array");
      if (member === "combinations") need(record.terms, `${at}.terms`, "array");
    });
  }
  return found;
}

export function isDesktopModelDocument(document: unknown): document is PreviewModel {
  return modelDocumentShapeDiagnostics(document).length === 0;
}
