// W-17 slice (c), DEL-02-02 Design `project-unit-selection.md` §7.4: an export that
// writes a unit taken from `project.units` refuses by name when that key is missing
// (absent, empty or whitespace-only). Nothing is assumed in its place. Missing
// `length` with nodes present is §3's hard lock, quoted with its re-author text.
import type { PreviewModel } from "../../types";
import type { ProjectUnitKey } from "../../services/projectService";
import { projectUnitMissing } from "./missingProjectUnit";
import { projectUnitRow } from "./ProjectUnitsSection";

/** A project key an export reads, and what it writes from it. */
export type ExportProjectUnitUse = { key: ProjectUnitKey; writes: string };

/** True when the key is `length`, missing, and the model has nodes: §3's hard lock. */
function lengthHardLock(model: PreviewModel, key: ProjectUnitKey): string | null {
  if (key !== "length") return null;
  const notice = projectUnitRow(model, "length", null).notice;
  return notice?.kind === "missing-hard-lock" ? notice.text : null;
}

/** The refusal text for one missing key, without its code. */
export function exportProjectUnitMissingText(model: PreviewModel, use: ExportProjectUnitUse): string {
  return lengthHardLock(model, use.key) ??
    `Missing project unit: ${use.key}. This export writes ${use.writes} from project.units.${use.key}, and nothing is assumed. Set ${use.key} in Project units on the Project page.`;
}

/** The remediation for one missing key: set it, or, under the hard lock, re-author the model. */
export function exportProjectUnitRemediation(model: PreviewModel, use: ExportProjectUnitUse, exportAction: string): string {
  return lengthHardLock(model, use.key)
    ? `Re-author the model with a length unit before ${exportAction}; the positions have no stated unit, and no fix is offered.`
    : `Set ${use.key} in Project units on the Project page before ${exportAction}.`;
}

/** The uses whose key is missing, each with its refusal text and remediation, in the order given. */
export function missingExportProjectUnits(model: PreviewModel, uses: readonly ExportProjectUnitUse[], exportAction = "exporting"): Array<ExportProjectUnitUse & { text: string; remediation: string }> {
  return uses
    .filter((use) => projectUnitMissing(model, use.key))
    .map((use) => ({ ...use, text: exportProjectUnitMissingText(model, use), remediation: exportProjectUnitRemediation(model, use, exportAction) }));
}

/** The stored symbol of a key the export has checked, or "" when it is missing (never a substitute). */
export function storedExportProjectUnit(model: PreviewModel, key: ProjectUnitKey): string {
  return projectUnitMissing(model, key) ? "" : (model.project.units as Record<string, string>)[key];
}

/** The panel's status line for project-unit refusals recorded as diagnostics, or null. */
export function projectUnitRefusalText(exportLabel: string, code: string, diagnostics: ReadonlyArray<{ code: string; message: string }>): string | null {
  const refused = diagnostics.filter((item) => item.code === code);
  if (refused.length === 0) return null;
  return `${exportLabel} export refused. ${refused.map((item) => `${item.code}: ${item.message}`).join(" ")}`;
}
