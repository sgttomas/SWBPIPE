// Component rows carry the same explicit inputs as the creation forms. No
// engineering value inherits. Unit choices alone use the existing project keys.
import type { PreviewComponent, PreviewModel } from "../../../types";
import { defaultComponentDraft, rigidGeometryIssues, type ComponentDraft } from "../../component-creation/componentIntent";

export const COMPONENT_FIELDS = [
  { key: "id", label: "Component id" },
  { key: "label", label: "Component label" },
  { key: "rigidBodyLength", label: "Body length" },
  { key: "lengthUnit", label: "Body length unit", dimension: "length" },
  { key: "endASize", label: "End A size" },
  { key: "endASizeUnit", label: "End A size unit", dimension: "length" },
  { key: "endBSize", label: "End B size" },
  { key: "endBSizeUnit", label: "End B size unit", dimension: "length" },
  { key: "weight", label: "Weight" },
  { key: "forceUnit", label: "Weight unit", dimension: "force" },
  { key: "centerOfGravityX", label: "COG X" },
  { key: "centerOfGravityY", label: "COG Y" },
  { key: "centerOfGravityZ", label: "COG Z" },
  { key: "centerOfGravityUnit", label: "COG unit", dimension: "length" },
  { key: "connectionEndAReference", label: "End A reference" },
  { key: "connectionEndBReference", label: "End B reference" },
  { key: "stiffnessBehaviorReference", label: "Stiffness behavior reference" },
  { key: "rigidStiffnessScale", label: "Stiffness scale" },
  { key: "rigidScaleSourceReference", label: "Stiffness scale source" },
  { key: "rigidSolverConsumption", label: "Solver consumption" },
  { key: "geometrySourceReference", label: "Component geometry source" },
  { key: "provenance", label: "Component provenance" },
] as const;
export type ComponentField = typeof COMPONENT_FIELDS[number]["key"];
export type ComponentRowInput = Partial<Pick<ComponentDraft, ComponentField>>;
export type ComponentColumnKey = `component.${ComponentField}`;
export const componentColumn = (key: ComponentField): ComponentColumnKey => `component.${key}`;
export const componentField = (column: string): ComponentField | null => COMPONENT_FIELDS.find((field) => componentColumn(field.key) === column)?.key ?? null;

export function componentRowDraft(model: PreviewModel, kind: "valve" | "flange", values: ComponentRowInput, node: string, pipe: string): ComponentDraft {
  const draft = defaultComponentDraft(model, null, [], kind);
  const units = model.project.units as Record<string, string | undefined> | undefined;
  // The form's id/label/source defaults are deliberately not inputs to a row:
  // every field is entered, except unit choices derived visibly below.
  for (const field of COMPONENT_FIELDS) draft[field.key] = values[field.key]?.trim() ?? "";
  for (const field of COMPONENT_FIELDS) {
    if ("dimension" in field && !draft[field.key]) draft[field.key] = units?.[field.dimension] ?? "";
  }
  return { ...draft, node, primaryPipeRef: pipe };
}

export function componentRowIssues(draft: ComponentDraft): Array<{ field: ComponentField; message: string }> {
  const issues = rigidGeometryIssues(draft) as Array<{ field: ComponentField; message: string }>;
  for (const field of ["id", "label", "geometrySourceReference", "provenance"] as const) {
    if (!draft[field].trim()) issues.push({ field, message: "Enter the value." });
  }
  return issues;
}

/** Applied rows read their component's actual quantities, not current header
 * units or historical buffers. A foreign change is still marked detached. */
export function componentCellsFromRecord(component: PreviewComponent): ComponentRowInput {
  const g = component.geometry ?? {};
  const str = (value: unknown) => value === undefined || value === null ? "" : String(value);
  return {
    id: component.id, label: component.label,
    rigidBodyLength: str(g.rigid_body_length?.value), lengthUnit: str(g.rigid_body_length?.unit),
    endASize: str(g.end_a_size?.value), endASizeUnit: str(g.end_a_size?.unit),
    endBSize: str(g.end_b_size?.value), endBSizeUnit: str(g.end_b_size?.unit),
    weight: str(g.weight?.value), forceUnit: str(g.weight?.unit),
    centerOfGravityX: str(g.center_of_gravity?.x), centerOfGravityY: str(g.center_of_gravity?.y), centerOfGravityZ: str(g.center_of_gravity?.z), centerOfGravityUnit: str(g.center_of_gravity?.unit),
    connectionEndAReference: str(g.connection_end_a_reference), connectionEndBReference: str(g.connection_end_b_reference), stiffnessBehaviorReference: str(g.stiffness_behavior_reference),
    rigidStiffnessScale: str(component.modifiers?.stiffness_scaling_user_value?.value), rigidScaleSourceReference: str(component.modifiers?.source_reference), rigidSolverConsumption: str(component.mechanics_interface?.solver_consumption),
    geometrySourceReference: str(g.rigid_component_source_reference), provenance: component.provenance,
  };
}
