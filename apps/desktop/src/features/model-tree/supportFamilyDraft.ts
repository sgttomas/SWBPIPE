import type { CompactSelectOption } from "../workspace/CompactSelect";
import { SUPPORT_FAMILIES } from "../results/displacementExtremes";

/** The six rigid DOFs whose full restraint suggests an anchor. */
const RIGID_DOFS = ["UX", "UY", "UZ", "RX", "RY", "RZ"] as const;

/** Why the "New support" form cannot supply a family. The operation applier refuses spring and the
 * hangers from this form's `create_support` (explicit stiffness with its DOF; a hanger object), as
 * `PropertyInspector.supportFamilyEngine.test.tsx` checks against the Wasm engine. The applier admits
 * `nonlinear` without a nonlinear law, but the solve then builds no nonlinear support and lowers it
 * to a rigid restraint (`core/product_physics` support lowering), so the model would declare a
 * nonlinear support while solving a linear one (review r1 P2). */
export const SUPPORT_FAMILY_ELSEWHERE: Readonly<Record<string, string>> = Object.freeze({
  spring: "Needs stiffness with its DOF: use Support configuration",
  variable_spring_hanger: "Needs hanger data: use Support configuration",
  spring_hanger: "Needs hanger data: use Support configuration",
  constant_effort_support: "Needs hanger data: use Support configuration",
  nonlinear: "Needs a nonlinear law: use Support configuration"
});

/** The "New support" form's family options: no family, then the model document's
 * `SupportFamily` values in the shared list's order and labels (the applier's `SUPPORT_FAMILIES`).
 * A family this form cannot supply stays listed, disabled, with its reason. */
export const SUPPORT_FAMILY_OPTIONS: readonly CompactSelectOption[] = Object.freeze([
  Object.freeze({ value: "", label: "Not stated" }),
  ...SUPPORT_FAMILIES.map((family) => {
    const elsewhere = SUPPORT_FAMILY_ELSEWHERE[family.value];
    return elsewhere ? Object.freeze({ value: family.value, label: `${family.label} (${elsewhere})`, disabled: true }) : family;
  })
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
