/** Issue 382: each member of the stored value/unit quantity family as it stands in
 * `unitPairFamilyModel`, with the invented values the surface regressions enter. The rule
 * (decided, binding): a bare number must name the stored unit or the edit is refused; an
 * explicit value/unit pair is written together after its unit is judged against the field's own
 * dimension; the dimension is the field's, never the intent's. */
import type { PreviewModel } from "../types";
import { FAMILY } from "./unitPairFamilyModel";

export type Quantity = { value: number; unit: string };
export type FamilyMember = {
  name: string;
  objectType: "Material" | "Element" | "Section" | "Load";
  ref: string;
  fieldPath: string;
  changeKind: "set_field" | "update_load";
  /** The field's own dimension. */
  dimension: string;
  stored: Quantity;
  /** The same quantity entered in another unit of the field's dimension. */
  otherUnit: Quantity;
  /** A new number in the stored unit. */
  inStoredUnit: number;
  /** A unit of another dimension, sent with the field's dimension. */
  wrongDimensionUnit: string;
  /** An intent dimension that is not the field's, with a unit of that dimension. */
  wrongIntentDimension: { dimension: string; unit: string };
  read: (model: PreviewModel) => unknown;
};

type Doc = Record<string, any>; // eslint-disable-line @typescript-eslint/no-explicit-any
const material = (model: PreviewModel): Doc => (model.materials as Doc[]).find((item) => item.id === FAMILY.material)!;
const pipe = (model: PreviewModel): Doc => (model.pipe_segments as Doc[]).find((item) => item.id === FAMILY.inlinePipe)!;
const section = (model: PreviewModel): Doc => ((model as Doc).sections as Doc[]).find((item) => item.id === FAMILY.section)!;
const primitive = (model: PreviewModel, index: number): Doc => (model.load_cases as Doc[]).find((item) => item.id === FAMILY.loadCase)!.primitive_loads[index];

export const FAMILY_MEMBERS: readonly FamilyMember[] = [
  { name: "material elastic modulus", objectType: "Material", ref: FAMILY.material, fieldPath: "elastic_modulus.value", changeKind: "set_field", dimension: "stress",
    stored: { value: 200000000000, unit: "Pa" }, otherUnit: { value: 210, unit: "GPa" }, inStoredUnit: 210000000000,
    wrongDimensionUnit: "m", wrongIntentDimension: { dimension: "length", unit: "m" }, read: (model) => material(model).elastic_modulus },
  { name: "material shear modulus", objectType: "Material", ref: FAMILY.material, fieldPath: "shear_modulus.value", changeKind: "set_field", dimension: "stress",
    stored: { value: 77000000000, unit: "Pa" }, otherUnit: { value: 79.3, unit: "GPa" }, inStoredUnit: 79300000000,
    wrongDimensionUnit: "N", wrongIntentDimension: { dimension: "force", unit: "N" }, read: (model) => material(model).shear_modulus },
  { name: "material expansion coefficient", objectType: "Material", ref: FAMILY.material, fieldPath: "thermal_expansion_coefficient.value", changeKind: "set_field", dimension: "thermal_expansion_coefficient",
    stored: { value: 0.000012, unit: "1/K" }, otherUnit: { value: 0.0000065, unit: "1/degF" }, inStoredUnit: 0.000013,
    wrongDimensionUnit: "Pa", wrongIntentDimension: { dimension: "stress", unit: "Pa" }, read: (model) => material(model).thermal_expansion_coefficient },
  { name: "pipe inline outside diameter", objectType: "Element", ref: FAMILY.inlinePipe, fieldPath: "section.outside_diameter.value", changeKind: "set_field", dimension: "length",
    stored: { value: 0.1143, unit: "m" }, otherUnit: { value: 114.3, unit: "mm" }, inStoredUnit: 0.12,
    wrongDimensionUnit: "Pa", wrongIntentDimension: { dimension: "stress", unit: "Pa" }, read: (model) => pipe(model).section.outside_diameter },
  { name: "pipe inline wall thickness", objectType: "Element", ref: FAMILY.inlinePipe, fieldPath: "section.wall_thickness.value", changeKind: "set_field", dimension: "length",
    stored: { value: 0.006, unit: "m" }, otherUnit: { value: 6.35, unit: "mm" }, inStoredUnit: 0.007,
    wrongDimensionUnit: "N", wrongIntentDimension: { dimension: "force", unit: "N" }, read: (model) => pipe(model).section.wall_thickness },
  { name: "shared section outside diameter", objectType: "Section", ref: FAMILY.section, fieldPath: "properties.outside_diameter.value", changeKind: "set_field", dimension: "length",
    stored: { value: 0.1683, unit: "m" }, otherUnit: { value: 168.3, unit: "mm" }, inStoredUnit: 0.2,
    wrongDimensionUnit: "Pa", wrongIntentDimension: { dimension: "stress", unit: "Pa" }, read: (model) => section(model).properties.outside_diameter },
  { name: "shared section wall thickness", objectType: "Section", ref: FAMILY.section, fieldPath: "properties.wall_thickness.value", changeKind: "set_field", dimension: "length",
    stored: { value: 0.00711, unit: "m" }, otherUnit: { value: 7.11, unit: "mm" }, inStoredUnit: 0.008,
    wrongDimensionUnit: "N", wrongIntentDimension: { dimension: "force", unit: "N" }, read: (model) => section(model).properties.wall_thickness },
  { name: "temperature point shear modulus", objectType: "Material", ref: FAMILY.material, fieldPath: `temperature_points.${FAMILY.point}.shear_modulus.value`, changeKind: "set_field", dimension: "stress",
    stored: { value: 73, unit: "GPa" }, otherUnit: { value: 73500, unit: "MPa" }, inStoredUnit: 74,
    wrongDimensionUnit: "m", wrongIntentDimension: { dimension: "length", unit: "m" },
    read: (model) => (material(model).temperature_points as Doc[]).find((point) => point.id === FAMILY.point)!.shear_modulus },
  { name: "distributed force magnitude", objectType: "Load", ref: FAMILY.loadCase, fieldPath: "primitive_loads.0.magnitude.value", changeKind: "update_load", dimension: "force_per_length",
    stored: { value: -190, unit: "N/m" }, otherUnit: { value: -0.2, unit: "kN/m" }, inStoredUnit: -200,
    wrongDimensionUnit: "N", wrongIntentDimension: { dimension: "force", unit: "N" }, read: (model) => primitive(model, 0).magnitude },
  { name: "concentrated force magnitude", objectType: "Load", ref: FAMILY.loadCase, fieldPath: "primitive_loads.1.magnitude.value", changeKind: "update_load", dimension: "force",
    stored: { value: 1000, unit: "N" }, otherUnit: { value: 1.2, unit: "kN" }, inStoredUnit: 1100,
    wrongDimensionUnit: "N/m", wrongIntentDimension: { dimension: "force_per_length", unit: "N/m" }, read: (model) => primitive(model, 1).magnitude }
];

/** A structured modify operation on a family member, as an imported file states it. */
export function familyOperation(member: FamilyMember, after: string, unit: string, dimension: string, token: string) {
  return {
    operation_id: `op:uf-${token}`,
    operation_kind: "modify",
    operation_status: "proposed",
    author_type: "user",
    source: { source_ref: `file:uf-${token}`, source_channel: "offline_user_supplied", source_role: "imported" },
    target: { object_type: member.objectType, ref: member.ref },
    change: {
      change_id: `change:uf-${token}`, change_kind: member.changeKind, field_label: member.name, field_path: member.fieldPath,
      before: String(member.stored.value), after, unit, dimension, source_note: "invented issue 382 surface regression"
    },
    validation: { schema_validation: "not_run", constraint_validation: "not_run", unit_validation: "not_run", diff_preview_status: "not_generated", application_status: "not_applied" },
    audit_boundary: { mutation_route: "structured_operations_only", direct_model_mutation_allowed: false, requires_user_acceptance: true, mutates_accepted_model_state: false },
    professional_boundary: { human_review_required: true, software_makes_compliance_claim: false, software_makes_certification_claim: false,
      software_makes_sealing_claim: false, software_makes_approval_claim: false, software_makes_authentication_claim: false },
    rationale: `invented issue 382 regression: ${member.name}`
  };
}

export const pair = (quantity: Quantity) => JSON.stringify(quantity);
