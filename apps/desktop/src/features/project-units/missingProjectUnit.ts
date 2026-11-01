// W-17 (DEL-02-02 Design `project-unit-selection.md` §3) in the entity editors:
// a field whose default unit is a `project.units` key gets no default when the
// key is absent, and nothing is assumed in its place. Editing the field is
// refused by name until a unit is stated: set the key in Project units on the
// Project page (which queues `set_project_unit`), or, where the field has its
// own unit selector and the applier does not need the key, choose the field's
// unit there. Node positions are stored in `project.units.length`, so missing
// `length` with nodes is §3's hard lock, with its re-author text.
import type { PreviewModel } from "../../types";
import type { ComponentDraft } from "../component-creation/componentIntent";
import { PROJECT_UNIT_KEYS, type ProjectUnitKey } from "../../services/projectService";
import { projectUnitRow } from "./ProjectUnitsSection";
import { storedProjectUnit } from "./projectUnitIntents";

/** How the field can get a unit: `positions` (node positions: the length key alone),
 * `required` (the field has no unit selector), `choose` (its own selector) or
 * `inspector` (a grid cell, whose field has a selector in the property inspector). */
export type MissingProjectUnitMode = "positions" | "required" | "choose" | "inspector";

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
  if (mode === "choose") return `Missing project unit: ${key}. This field has no default unit, and nothing is assumed. Choose its unit here, or set ${key} in Project units on the Project page.`;
  if (mode === "inspector") return `Missing project unit: ${key}. This cell has no default unit, and nothing is assumed. Choose its unit in the property inspector, or set ${key} in Project units on the Project page.`;
  return `Missing project unit: ${key}. This field takes its unit from project.units.${key}, and nothing is assumed. Set ${key} in Project units on the Project page before editing it.`;
}

/** A grid cell whose composed default is not a catalog unit (§7.3, §7.6): the grid
 * has no unit selector, so the notice sends the user to the property inspector. */
export function unavailableDefaultUnitCellText(notice: string): string {
  return `${notice}. Choose its unit in the property inspector.`;
}

/** The keys a component creation draft (the property inspector's and the
 * viewport's) leaves unselected because the project key is absent. */
export function componentDraftMissingUnitKeys(model: PreviewModel, draft: Pick<ComponentDraft, "kind" | "lengthUnit" | "angleUnit" | "forceUnit" | "endASizeUnit" | "endBSizeUnit" | "centerOfGravityUnit">): ProjectUnitKey[] {
  if (draft.kind === "expansion_joint") return [];
  const unset = (unit: string) => unit.trim() === "" || unit.trim() === "TBD";
  const keys: ProjectUnitKey[] = [];
  const rigid = draft.kind !== "bend" && draft.kind !== "tee";
  if (unset(draft.lengthUnit) || (rigid && [draft.endASizeUnit, draft.endBSizeUnit, draft.centerOfGravityUnit].some(unset))) keys.push("length");
  if ((draft.kind === "bend" || draft.kind === "tee") && unset(draft.angleUnit)) keys.push("angle");
  if (draft.kind !== "bend" && draft.kind !== "tee" && unset(draft.forceUnit)) keys.push("force");
  return keys.filter((key) => projectUnitMissing(model, key));
}

/** The component creation form's notices: one per unselected unit whose key is absent. */
export function componentDraftMissingUnitNotices(model: PreviewModel, draft: Pick<ComponentDraft, "kind" | "lengthUnit" | "angleUnit" | "forceUnit" | "endASizeUnit" | "endBSizeUnit" | "centerOfGravityUnit">): Array<{ key: ProjectUnitKey; text: string }> {
  return componentDraftMissingUnitKeys(model, draft).map((key) => ({ key, text: missingProjectUnitText(model, key, "choose") }));
}
