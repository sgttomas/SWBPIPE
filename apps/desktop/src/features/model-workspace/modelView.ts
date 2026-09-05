import type { EntityRef, PreviewComponent, PreviewModel } from "../../types";

export function defaultSelection(model: PreviewModel): EntityRef {
  return { type: "project", id: model.project.id };
}

export function entityLabel(model: PreviewModel, id: string): string {
  const entity =
    model.materials?.find((item) => item.id === id) ??
    model.sections?.find((item) => item.id === id) ??
    model.nodes.find((item) => item.id === id) ??
    model.pipe_segments.find((item) => item.id === id) ??
    model.supports.find((item) => item.id === id) ??
    model.components.find((item) => item.id === id) ??
    model.load_cases.find((item) => item.id === id) ??
    model.combinations?.find((item) => item.id === id);
  return entityDisplayLabel(entity) ?? id;
}

export type PropertyQuantity = { value: number | string; unit: string; dimension_id: string; label?: string };
export type PropertyReadout = { text: string; quantities?: PropertyQuantity[] };
type PropertyValue = string | PropertyReadout;

/** Typed readout values keep source quantities intact; the legacy text API remains exact. */
export function selectedPropertyRows(model: PreviewModel, selection: EntityRef): Array<[string, PropertyReadout]> {
  return selectedPropertyValues(model, selection).map(([label, value]) => [label, typeof value === "string" ? { text: value } : value]);
}

export function selectedProperties(model: PreviewModel, selection: EntityRef): Array<[string, string]> {
  return selectedPropertyRows(model, selection).map(([label, value]) => [label, value.text]);
}

function selectedPropertyValues(model: PreviewModel, selection: EntityRef): Array<[string, PropertyValue]> {
  if (selection.type === "project") {
    return [
      ["Project ID", model.project.id],
      ["Schema", model.schema_version],
      ["Mechanics", model.analysis_status.mechanics],
      ["Rule check", model.analysis_status.rule_check],
      ["Professional acceptance", model.analysis_status.professional_acceptance]
    ];
  }
  const material = model.materials?.find((item) => item.id === selection.id);
  if (material) {
    return [
      ["ID", material.id],
      ["Elastic modulus", quantityDisplay(material.elastic_modulus, "stress")],
      ["Shear modulus", quantityDisplay(material.shear_modulus, "stress")],
      [
        "Thermal expansion",
        material.thermal_expansion_coefficient
          ? quantityDisplay(material.thermal_expansion_coefficient, "thermal_expansion_coefficient")
          : "TBD"
      ],
      ["Provenance", material.provenance]
    ];
  }
  const section = model.sections?.find((item) => item.id === selection.id);
  if (section) {
    return [
      ["ID", section.id],
      ["Name", section.name],
      ["Type", section.section_type],
      ["OD", quantityDisplay(section.properties.outside_diameter, "length")],
      ["Wall", quantityDisplay(section.properties.wall_thickness, "length")],
      ["Provenance", provenanceDisplay(section.provenance)]
    ];
  }
  const node = model.nodes.find((item) => item.id === selection.id);
  if (node) {
    return [
      ["ID", node.id],
      ["Label", node.label],
      ["Position", vectorQuantityDisplay({ ...node.position, unit: model.project.units.length }, `${node.position.x}, ${node.position.y}, ${node.position.z} ${model.project.units.length}`)],
      ["Provenance", node.provenance]
    ];
  }
  const pipe = model.pipe_segments.find((item) => item.id === selection.id);
  if (pipe) {
    return [
      ["ID", pipe.id],
      ["From", pipe.from],
      ["To", pipe.to],
      ["OD", quantityDisplay(pipe.section.outside_diameter, "length")],
      ["Wall", quantityDisplay(pipe.section.wall_thickness, "length")],
      // Optional user-entered mill-tolerance reduction: absence means no
      // reduction (not a zero default), so the row appears only when the
      // user entered a value.
      ...(pipe.section.mill_tolerance
        ? ([["Mill tolerance", quantityDisplay(pipe.section.mill_tolerance, "length")]] as Array<[string, PropertyValue]>)
        : []),
      ["Material", pipe.material],
      ["Provenance", pipe.provenance]
    ];
  }
  const support = model.supports.find((item) => item.id === selection.id);
  if (support) {
    const rows: Array<[string, PropertyValue]> = [
      ["ID", support.id],
      ["Node", support.node],
      ["Family", support.family ?? "TBD"],
      ["Restraints", support.restraints.join(", ")]
    ];
    // Keep the selected record's DOF with its quantity; the unit alone does
    // not establish whether the entered stiffness is linear or rotational.
    const stiffness = support.stiffness?.value ? support.stiffness : support.hanger?.stiffness;
    if (stiffness?.value) {
      const linear = ["UX", "Ux", "ux", "UY", "Uy", "uy", "UZ", "Uz", "uz"].includes(stiffness.dof);
      const rotational = ["RX", "Rx", "rx", "RY", "Ry", "ry", "RZ", "Rz", "rz"].includes(stiffness.dof);
      const label = linear ? "Linear stiffness" : rotational ? "Rotational stiffness" : "Stiffness";
      const dimension = linear ? "linear_stiffness" : rotational ? "rotational_stiffness" : "unknown";
      rows.push([label, quantityDisplay(stiffness.value, dimension)]);
    } else if (support.properties?.linear_stiffness) {
      rows.push(["Linear stiffness", quantityDisplay(support.properties.linear_stiffness, "linear_stiffness")]);
    }
    if (support.hanger) {
      rows.push(
        ["Hanger type", support.hanger.hanger_type ?? "TBD"],
        ["Installed load", quantityDisplay(support.hanger.installed_load, "force")],
        ["Cold load", quantityDisplay(support.hanger.cold_load, "force")],
        ["Hot load", quantityDisplay(support.hanger.hot_load, "force")],
        ["Constant load", quantityDisplay(support.hanger.constant_load, "force")],
        ["Travel range", quantityDisplay(support.hanger.travel_range, "length")],
        ["Movement limit", quantityDisplay(support.hanger.movement_limit, "length")],
        ["Source", support.hanger.source_reference ?? "TBD"],
        ["Manufacturer", support.hanger.manufacturer_reference ?? "TBD"],
        ["Load-side review", support.hanger.load_side_review_reference ?? "TBD"],
        ["Mechanics consumption", support.hanger.mechanics_consumption ?? "TBD"]
      );
    }
    rows.push(["Provenance", support.provenance]);
    return rows;
  }
  const component = model.components.find((item) => item.id === selection.id);
  if (component) {
    const rows: Array<[string, PropertyValue]> = [
      ["ID", component.id],
      ["Kind", component.kind],
      ["Node", component.node]
    ];
    if (isBendComponent(component)) {
      rows.push(
        ["Bend radius", quantityDisplay(component.geometry?.bend_radius, "length")],
        ["Bend angle", quantityDisplay(component.geometry?.bend_angle, "angle")],
        ["Bend plane", component.geometry?.bend_plane_orientation ?? "TBD"]
      );
      // User-entered curved-bend span mapping (DEC-070). The slot is
      // optional for the straight-chord modes, so an absent value is shown
      // (and flagged) only when the component requests the assembled
      // curved-bend realization, where its absence is engine-blocking.
      if (
        component.geometry?.bend_pipe_ref ||
        component.mechanics_interface?.solver_consumption === "curved_bend_macro_element"
      ) {
        rows.push(["Bend pipe", component.geometry?.bend_pipe_ref ?? "TBD"]);
      }
      rows.push(
        ["Geometry source", component.geometry?.bend_geometry_source_reference ?? "TBD"],
        ["Solver consumption", component.mechanics_interface?.solver_consumption ?? "TBD"],
        ["Rule input consumption", component.mechanics_interface?.rule_check_consumption ?? "TBD"],
        ["SIF user value", quantityDisplay(component.modifiers?.sif_user_value, "dimensionless")],
        ["Flexibility user value", quantityDisplay(component.modifiers?.flexibility_factor_user_value, "dimensionless")],
        ["Modifier source", component.modifiers?.source_reference ?? "TBD"],
        ["Completeness", componentCompletenessDisplay(component)]
      );
    }
    if (isBranchComponent(component)) {
      rows.push(
        ["Header pipe", component.geometry?.branch_header_pipe_ref ?? "TBD"],
        ["Branch pipe", component.geometry?.branch_branch_pipe_ref ?? "TBD"],
        ["Run size", quantityDisplay(component.geometry?.branch_run_size, "length")],
        ["Header size", quantityDisplay(component.geometry?.branch_header_size, "length")],
        ["Branch angle", quantityDisplay(component.geometry?.branch_connection_angle, "angle")],
        ["Connection type", component.geometry?.branch_connection_type ?? "TBD"],
        ["Reinforcement", component.geometry?.branch_reinforcement_reference ?? "TBD"],
        ["Geometry source", component.geometry?.branch_geometry_source_reference ?? "TBD"],
        ["Solver consumption", component.mechanics_interface?.solver_consumption ?? "TBD"],
        ["Rule input consumption", component.mechanics_interface?.rule_check_consumption ?? "TBD"],
        ["Header SIF user value", quantityDisplay(component.modifiers?.branch_header_sif_user_value, "dimensionless")],
        ["Branch SIF user value", quantityDisplay(component.modifiers?.branch_branch_sif_user_value, "dimensionless")],
        ["Flexibility user value", quantityDisplay(component.modifiers?.flexibility_factor_user_value, "dimensionless")],
        ["Modifier source", component.modifiers?.source_reference ?? "TBD"],
        ["Completeness", componentCompletenessDisplay(component)]
      );
    }
    if (isRigidComponent(component)) {
      rows.push(
        ["Mapped pipe", component.geometry?.rigid_pipe_ref ?? "TBD"],
        ["Rigid body length", quantityDisplay(component.geometry?.rigid_body_length, "length")],
        ["End A size", quantityDisplay(component.geometry?.end_a_size, "length")],
        ["End B size", quantityDisplay(component.geometry?.end_b_size, "length")],
        ["Weight", quantityDisplay(component.geometry?.weight, "force")],
        ["Center of gravity", vectorQuantityDisplay(component.geometry?.center_of_gravity)],
        ["End A reference", component.geometry?.connection_end_a_reference ?? "TBD"],
        ["End B reference", component.geometry?.connection_end_b_reference ?? "TBD"],
        ["Stiffness behavior", component.geometry?.stiffness_behavior_reference ?? "TBD"],
        ["Geometry source", component.geometry?.rigid_component_source_reference ?? "TBD"],
        ["Solver consumption", component.mechanics_interface?.solver_consumption ?? "TBD"],
        ["Rule input consumption", component.mechanics_interface?.rule_check_consumption ?? "TBD"],
        ["Stiffness scale", quantityDisplay(component.modifiers?.stiffness_scaling_user_value, "dimensionless")],
        ["Linear stiffness", quantityDisplay(component.modifiers?.linear_stiffness_user_value, "linear_stiffness")],
        ["Rotational stiffness", quantityDisplay(component.modifiers?.rotational_stiffness_user_value, "rotational_stiffness")],
        ["Modifier source", component.modifiers?.source_reference ?? "TBD"],
        ["Completeness", componentCompletenessDisplay(component)]
      );
    }
    if (isExpansionJointComponent(component)) {
      rows.push(
        ["Mapped pipe", component.geometry?.expansion_joint_pipe_ref ?? "TBD"],
        ["Effective area", quantityDisplay(component.geometry?.effective_area, "area")],
        ["Movement limit", quantityDisplay(component.geometry?.movement_limit, "length")],
        ["Hardware reference", component.geometry?.hardware_reference ?? "TBD"],
        ["Manufacturer reference", component.geometry?.manufacturer_reference ?? "TBD"],
        ["Pressure thrust", component.geometry?.pressure_thrust_reference ?? "TBD"],
        ["Geometry source", component.geometry?.expansion_joint_source_reference ?? "TBD"],
        ["Solver consumption", component.mechanics_interface?.solver_consumption ?? "TBD"],
        ["Rule input consumption", component.mechanics_interface?.rule_check_consumption ?? "TBD"],
        ["Axial stiffness", quantityDisplay(component.modifiers?.axial_stiffness_user_value, "linear_stiffness")],
        ["Lateral stiffness", quantityDisplay(component.modifiers?.lateral_stiffness_user_value, "linear_stiffness")],
        ["Angular stiffness", quantityDisplay(component.modifiers?.angular_stiffness_user_value, "rotational_stiffness")],
        ["Torsional stiffness", quantityDisplay(component.modifiers?.torsional_stiffness_user_value, "rotational_stiffness")],
        ["Modifier source", component.modifiers?.source_reference ?? "TBD"],
        ["Completeness", componentCompletenessDisplay(component)]
      );
    }
    rows.push(["Provenance", component.provenance]);
    return rows;
  }
  const loadCase = model.load_cases.find((item) => item.id === selection.id);
  if (loadCase) {
    const primitiveLoads = loadCase.primitive_loads ?? [];
    const rows: Array<[string, PropertyValue]> = [
      ["ID", loadCase.id],
      ["Kind", loadCase.kind],
      ["Status", loadCase.status],
      ["Load records", primitiveLoads.length.toString()],
      ["Categories", primitiveLoadCategories(primitiveLoads)],
      ["Targets", primitiveLoadTargets(primitiveLoads)]
    ];
    // Mutually exclusive user-entered modulus basis selectors (DEC-068 /
    // DEC-077). Absence of both means material base values.
    if (loadCase.modulus_basis_ref) {
      rows.push(["Modulus basis", loadCase.modulus_basis_ref]);
    }
    if (loadCase.modulus_basis_temperature) {
      rows.push([
        "Modulus basis solve temperature",
        quantityDisplay(loadCase.modulus_basis_temperature, "temperature")
      ]);
    }
    if (loadCase.equivalent_static?.seismic) {
      const seismic = loadCase.equivalent_static.seismic;
      rows.push(
        ["Seismic gravity", quantityDisplay(seismic.gravity_acceleration, "acceleration")],
        ["Seismic g-factor X", quantityDisplay(seismic.g_factor_x, "dimensionless")],
        ["Seismic g-factor Y", quantityDisplay(seismic.g_factor_y, "dimensionless")],
        ["Seismic g-factor Z", quantityDisplay(seismic.g_factor_z, "dimensionless")]
      );
    }
    if (loadCase.equivalent_static?.wind) {
      const wind = loadCase.equivalent_static.wind;
      rows.push(
        ["Wind pressure", quantityDisplay(wind.pressure, "pressure")],
        ["Wind shape factor", quantityDisplay(wind.shape_factor, "dimensionless")],
        ["Wind direction", wind.direction ?? "TBD"],
        ["Wind exposed spans", (wind.exposed_pipe_refs ?? []).join(", ") || "TBD"]
      );
    }
    if (loadCase.equivalent_static?.provenance) {
      rows.push(["Generation provenance", loadCase.equivalent_static.provenance]);
    }
    rows.push(["Provenance", loadCase.provenance]);
    return rows;
  }
  const combination = model.combinations?.find((item) => item.id === selection.id);
  if (combination) {
    return [
      ["ID", combination.id],
      ["Basis", combination.basis],
      ["Terms", combination.terms.map((term) => `${term.load_case} x ${term.factor}`).join("; ")],
      ["Provenance", combination.provenance]
    ];
  }
  return [["Selection", selection.id]];
}

function quantityDisplay(quantity: { value: number | string; unit: string } | undefined, dimension: string): PropertyValue {
  return quantity ? { text: `${quantity.value} ${quantity.unit}`, quantities: [{ ...quantity, dimension_id: dimension }] } : "TBD";
}

function vectorQuantityDisplay(quantity: { x: number; y: number; z: number; unit: string } | undefined, text?: string): PropertyValue {
  return quantity ? {
    text: text ?? `x=${quantity.x}, y=${quantity.y}, z=${quantity.z} ${quantity.unit}`,
    quantities: (["x", "y", "z"] as const).map((axis) => ({ value: quantity[axis], unit: quantity.unit, dimension_id: "length", label: axis }))
  } : "TBD";
}

function isBendComponent(component: PreviewComponent): boolean {
  return component.kind === "bend" || component.kind === "elbow";
}

function isBranchComponent(component: PreviewComponent): boolean {
  return component.kind === "branch" || component.kind === "tee" || component.kind === "branch_connection";
}

function isRigidComponent(component: PreviewComponent): boolean {
  return ["valve", "flange", "reducer", "rigid", "specialty"].includes(component.kind);
}

function isExpansionJointComponent(component: PreviewComponent): boolean {
  return component.kind === "expansion_joint";
}

function componentCompletenessDisplay(component: PreviewComponent): string {
  const findings = component.completeness ?? [];
  if (findings.length === 0) return "TBD";
  return findings
    .map((finding) => {
      const missing = finding.missing_field_kinds?.length ? `; missing=${finding.missing_field_kinds.join(",")}` : "";
      return `${finding.diagnostic_code}:${finding.status}${missing}`;
    })
    .join("; ");
}

function provenanceDisplay(provenance: string | Record<string, unknown>): string {
  return typeof provenance === "string" ? provenance : JSON.stringify(provenance);
}

function entityDisplayLabel(entity: { label?: string; name?: string } | undefined): string | undefined {
  return entity?.label ?? entity?.name;
}

function primitiveLoadCategories(loads: Array<Record<string, unknown>>): string {
  if (loads.length === 0) return "none";
  return loads.map((load) => stringField(load, "category")).join(", ");
}

function primitiveLoadTargets(loads: Array<Record<string, unknown>>): string {
  if (loads.length === 0) return "none";
  return loads.map(formatPrimitiveLoadTarget).join(", ");
}

function formatPrimitiveLoadTarget(load: Record<string, unknown>): string {
  const target = load.target;
  if (!target || typeof target !== "object" || Array.isArray(target)) return "target:TBD";
  const record = target as Record<string, unknown>;
  const targetType = stringField(record, "type");
  const node = optionalStringField(record, "node");
  if (node) return `${targetType}:${node}`;
  const pipe = optionalStringField(record, "pipe");
  if (pipe) return `${targetType}:${pipe}`;
  return `${targetType}:TBD`;
}

function stringField(record: Record<string, unknown>, key: string): string {
  return optionalStringField(record, key) ?? "TBD";
}

function optionalStringField(record: Record<string, unknown>, key: string): string | null {
  const value = record[key];
  return typeof value === "string" && value.trim() ? value : null;
}
