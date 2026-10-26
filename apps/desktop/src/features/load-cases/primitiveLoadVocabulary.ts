import type { PreviewModel } from "../../types";
import type { UnitCatalogRoute } from "../../services/unitCatalogService";
import { composedDefault, declaredOrComposedDefault, projectTemperatureDefault, type CheckedComposedDefault } from "../project-units/projectUnitSelection";

// Primitive-load category vocabulary of the Load Case Manager: which target a
// category uses, its direction choices and default, the dimension sent to core
// unit validation, and the project-unit default offered for a new primitive.
// Moved from LoadCaseManagerPanel.tsx so the mapping is testable on its own.

export type PrimitiveLoadCategory =
  | "concentrated_force"
  | "distributed_force"
  | "concentrated_moment"
  | "thermal"
  | "imposed_displacement";

export function primitiveLoadCategoryFromValue(value: string): PrimitiveLoadCategory {
  if (value === "concentrated_moment") return "concentrated_moment";
  if (value === "thermal") return "thermal";
  if (value === "imposed_displacement") return "imposed_displacement";
  return value === "distributed_force" ? "distributed_force" : "concentrated_force";
}

export function primitiveLoadUsesPipeTarget(category: PrimitiveLoadCategory): boolean {
  return category === "distributed_force" || category === "thermal";
}

export function primitiveLoadUsesSupportTarget(category: PrimitiveLoadCategory): boolean {
  return category === "imposed_displacement";
}

export function primitiveLoadDirectionOptions(category: PrimitiveLoadCategory): string[] {
  if (category === "imposed_displacement") return ["UX", "UY", "UZ", "RX", "RY", "RZ"];
  return category === "concentrated_moment" ? ["rotation_x", "rotation_y", "rotation_z"] : ["global_x", "global_y", "global_z"];
}

export function defaultPrimitiveLoadDirection(category: PrimitiveLoadCategory): string {
  if (category === "concentrated_moment") return "rotation_z";
  if (category === "thermal") return "global_z";
  if (category === "imposed_displacement") return "UZ";
  return "global_y";
}

export function primitiveLoadDofIsRotational(direction: string): boolean {
  return direction === "RX" || direction === "RY" || direction === "RZ";
}

export function primitiveLoadDraftDimension(category: PrimitiveLoadCategory, direction: string): string {
  if (category === "distributed_force") return "force_per_length";
  if (category === "concentrated_moment") return "moment";
  if (category === "thermal") return "temperature_interval";
  if (category === "imposed_displacement") return primitiveLoadDofIsRotational(direction) ? "rotation" : "displacement";
  return "force";
}

/**
 * The default unit of a new primitive (W-17 §2.4). The moment reads
 * `project.units.moment` when the key is present, and only when it is absent
 * falls back to `<force>*<length>`. The distributed force is composed as
 * `<force>/<length>`. A present but invalid key, or (once the catalog is loaded)
 * a composition or declared moment the catalog lacks, gives "" (unselected)
 * with a notice, never a substitute.
 */
export function primitiveLoadDefaultUnitChoice(
  model: PreviewModel,
  category: PrimitiveLoadCategory,
  direction: string,
  route: UnitCatalogRoute | null = null
): CheckedComposedDefault {
  if (category === "distributed_force") return composedDefault(route, "distributed_force", model.project.units);
  // A present moment key is the default and is never replaced by a composition;
  // a present but invalid one gives no default. Only an absent key composes.
  if (category === "concentrated_moment") return declaredOrComposedDefault(route, "moment", "moment", model.project.units, true);
  return { symbol: primitiveLoadDefaultUnit(model, category, direction, route), notice: null };
}

export function primitiveLoadDefaultUnit(
  model: PreviewModel,
  category: PrimitiveLoadCategory,
  direction: string,
  route: UnitCatalogRoute | null = null
): string {
  if (category === "distributed_force") return primitiveLoadDefaultUnitChoice(model, category, direction, route).symbol;
  if (category === "concentrated_moment") return primitiveLoadDefaultUnitChoice(model, category, direction, route).symbol;
  if (category === "thermal") return projectTemperatureUnit(model);
  if (category === "imposed_displacement") return primitiveLoadDofIsRotational(direction) ? projectRotationUnit(model) : projectLengthUnit(model);
  return projectForceUnit(model);
}

function projectForceUnit(model: PreviewModel): string {
  return optionalString(model.project.units.force) ?? "TBD";
}

function projectLengthUnit(model: PreviewModel): string {
  return optionalString(model.project.units.length) ?? "TBD";
}


function projectTemperatureUnit(model: PreviewModel): string {
  return projectTemperatureDefault(model.project.units) || "TBD";
}

function projectRotationUnit(model: PreviewModel): string {
  return optionalString(model.project.units.angle) ?? "TBD";
}

export function optionalString(value: unknown): string | null {
  return typeof value === "string" && value.trim() ? value : null;
}
