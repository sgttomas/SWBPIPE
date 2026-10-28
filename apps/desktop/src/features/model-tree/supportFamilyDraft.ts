import type { CompactSelectOption } from "../workspace/CompactSelect";
import { SUPPORT_FAMILIES } from "../results/displacementExtremes";

/** The six rigid DOFs whose full restraint suggests an anchor. */
const RIGID_DOFS = ["UX", "UY", "UZ", "RX", "RY", "RZ"] as const;

/** The "New support" form's family options: no family, then the model document's
 * `SupportFamily` values in the shared list's order and labels (the applier's `SUPPORT_FAMILIES`). */
export const SUPPORT_FAMILY_OPTIONS: readonly CompactSelectOption[] = Object.freeze([
  Object.freeze({ value: "", label: "Not stated" }),
  ...SUPPORT_FAMILIES
]);

/**
 * The family the form suggests from the user's own entries: "anchor" when all six rigid DOFs
 * are restrained with no stiffness entered (the form has no hanger), otherwise none. It is only
 * a suggestion: it is shown as one, can be changed, and is written only when the form is queued.
 */
export function suggestedSupportFamily(restraints: readonly string[], linearStiffness: string): string | null {
  return RIGID_DOFS.every((dof) => restraints.includes(dof)) && linearStiffness.trim() === "" ? "anchor" : null;
}

/** The family shown and written: an explicit pick (`""` is an explicit "Not stated") wins over the suggestion. */
export function draftSupportFamily(choice: string | null, suggestion: string | null): string {
  return choice ?? suggestion ?? "";
}

export function supportFamilySuggestionText(suggestion: string): string {
  const label = SUPPORT_FAMILIES.find((family) => family.value === suggestion)?.label ?? suggestion;
  return `Suggested from restraints: ${label.toLowerCase()}`;
}
