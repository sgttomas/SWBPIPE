// W-17 (DEL-02-02 Design `project-unit-selection.md` §3) in the entity editors:
// a field whose default unit is a `project.units` key gets no default when the
// key is absent, and nothing is assumed in its place. Editing the field is
// refused by name until a unit is stated: set the key in Project units on the
// Project page (which queues `set_project_unit`), or, where the field has its
// own unit selector and the applier does not need the key, choose the field's
// unit there. Node positions are stored in `project.units.length`, so missing
// `length` with nodes is §3's hard lock, with its re-author text.
import type { PreviewModel } from "../../types";
import { PROJECT_UNIT_KEYS, type ProjectUnitKey } from "../../services/projectService";
import { projectUnitRow } from "./ProjectUnitsSection";
import { storedProjectUnit } from "./projectUnitIntents";

/** How the field can get a unit: `positions` (node positions: the length key alone),
 * `required` (the field has no unit selector) or `choose` (its own selector). */
export type MissingProjectUnitMode = "positions" | "required" | "choose";

export function isProjectUnitKey(key: string): key is ProjectUnitKey {
  return (PROJECT_UNIT_KEYS as readonly string[]).includes(key);
}

/** True when the key is absent from the document (or not a usable symbol). */
export function projectUnitMissing(model: PreviewModel, key: ProjectUnitKey): boolean {
  const stored = storedProjectUnit(model, key);
  return typeof stored !== "string" || stored.trim() === "";
}

/** The refusal shown on a field whose unit would come from the missing key. */
export function missingProjectUnitText(model: PreviewModel, key: ProjectUnitKey, mode: MissingProjectUnitMode): string {
  if (mode === "positions") {
    const notice = projectUnitRow(model, key, null).notice;
    if (notice?.kind === "missing-hard-lock") return notice.text;
  }
  return mode === "choose"
    ? `Missing project unit: ${key}. This field has no default unit, and nothing is assumed. Choose its unit here, or set ${key} in Project units on the Project page.`
    : `Missing project unit: ${key}. This field takes its unit from project.units.${key}, and nothing is assumed. Set ${key} in Project units on the Project page before editing it.`;
}
