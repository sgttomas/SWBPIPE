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

export function selectedProperties(model: PreviewModel, selection: EntityRef): Array<[string, string]> {
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
      ["Elastic modulus", `${material.elastic_modulus.value} ${material.elastic_modulus.unit}`],
      ["Shear modulus", `${material.shear_modulus.value} ${material.shear_modulus.unit}`],
      [
        "Thermal expansion",
        material.thermal_expansion_coefficient
          ? `${material.thermal_expansion_coefficient.value} ${material.thermal_expansion_coefficient.unit}`
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
      ["OD", quantityDisplay(section.properties.outside_diameter)],
      ["Wall", quantityDisplay(section.properties.wall_thickness)],
      ["Provenance", provenanceDisplay(section.provenance)]
    ];
  }
  const node = model.nodes.find((item) => item.id === selection.id);
  if (node) {
    return [
      ["ID", node.id],
      ["Label", node.label],
      ["Position", `${node.position.x}, ${node.position.y}, ${node.position.z} m`],
      ["Provenance", node.provenance]
    ];
  }
  const pipe = model.pipe_segments.find((item) => item.id === selection.id);
  if (pipe) {
    return [
      ["ID", pipe.id],
      ["From", pipe.from],
      ["To", pipe.to],
      ["OD", `${pipe.section.outside_diameter.value} ${pipe.section.outside_diameter.unit}`],
      ["Wall", `${pipe.section.wall_thickness.value} ${pipe.section.wall_thickness.unit}`],
      ["Material", pipe.material],
      ["Provenance", pipe.provenance]
    ];
  }
  const support = model.supports.find((item) => item.id === selection.id);
  if (support) {
    const rows: Array<[string, string]> = [
      ["ID", support.id],
      ["Node", support.node],
      ["Restraints", support.restraints.join(", ")]
    ];
    const linearStiffness = support.properties?.linear_stiffness;
    if (linearStiffness) {
      rows.push(["Linear stiffness", `${linearStiffness.value} ${linearStiffness.unit}`]);
    }
    rows.push(["Provenance", support.provenance]);
    return rows;
  }
  const component = model.components.find((item) => item.id === selection.id);
  if (component) {
    const rows: Array<[string, string]> = [
      ["ID", component.id],
      ["Kind", component.kind],
      ["Node", component.node]
    ];
    if (isBendComponent(component)) {
      rows.push(
        ["Bend radius", quantityDisplay(component.geometry?.bend_radius)],
        ["Bend angle", quantityDisplay(component.geometry?.bend_angle)],
        ["Bend plane", component.geometry?.bend_plane_orientation ?? "TBD"],
        ["Geometry source", component.geometry?.bend_geometry_source_reference ?? "TBD"],
        ["Solver consumption", component.mechanics_interface?.solver_consumption ?? "TBD"],
        ["Rule input consumption", component.mechanics_interface?.rule_check_consumption ?? "TBD"],
        ["SIF user value", quantityDisplay(component.modifiers?.sif_user_value)],
        ["Flexibility user value", quantityDisplay(component.modifiers?.flexibility_factor_user_value)],
        ["Modifier source", component.modifiers?.source_reference ?? "TBD"],
        ["Completeness", componentCompletenessDisplay(component)]
      );
    }
    if (isBranchComponent(component)) {
      rows.push(
        ["Header pipe", component.geometry?.branch_header_pipe_ref ?? "TBD"],
        ["Branch pipe", component.geometry?.branch_branch_pipe_ref ?? "TBD"],
        ["Run size", quantityDisplay(component.geometry?.branch_run_size)],
        ["Header size", quantityDisplay(component.geometry?.branch_header_size)],
        ["Branch angle", quantityDisplay(component.geometry?.branch_connection_angle)],
        ["Connection type", component.geometry?.branch_connection_type ?? "TBD"],
        ["Reinforcement", component.geometry?.branch_reinforcement_reference ?? "TBD"],
        ["Geometry source", component.geometry?.branch_geometry_source_reference ?? "TBD"],
        ["Solver consumption", component.mechanics_interface?.solver_consumption ?? "TBD"],
        ["Rule input consumption", component.mechanics_interface?.rule_check_consumption ?? "TBD"],
        ["Header SIF user value", quantityDisplay(component.modifiers?.branch_header_sif_user_value)],
        ["Branch SIF user value", quantityDisplay(component.modifiers?.branch_branch_sif_user_value)],
        ["Flexibility user value", quantityDisplay(component.modifiers?.flexibility_factor_user_value)],
        ["Modifier source", component.modifiers?.source_reference ?? "TBD"],
        ["Completeness", componentCompletenessDisplay(component)]
      );
    }
    if (isRigidComponent(component)) {
      rows.push(
        ["Mapped pipe", component.geometry?.rigid_pipe_ref ?? "TBD"],
        ["Rigid body length", quantityDisplay(component.geometry?.rigid_body_length)],
        ["End A size", quantityDisplay(component.geometry?.end_a_size)],
        ["End B size", quantityDisplay(component.geometry?.end_b_size)],
        ["Weight", quantityDisplay(component.geometry?.weight)],
        ["Center of gravity", vectorQuantityDisplay(component.geometry?.center_of_gravity)],
        ["End A reference", component.geometry?.connection_end_a_reference ?? "TBD"],
        ["End B reference", component.geometry?.connection_end_b_reference ?? "TBD"],
        ["Stiffness behavior", component.geometry?.stiffness_behavior_reference ?? "TBD"],
        ["Geometry source", component.geometry?.rigid_component_source_reference ?? "TBD"],
        ["Solver consumption", component.mechanics_interface?.solver_consumption ?? "TBD"],
        ["Rule input consumption", component.mechanics_interface?.rule_check_consumption ?? "TBD"],
        ["Stiffness scale", quantityDisplay(component.modifiers?.stiffness_scaling_user_value)],
        ["Linear stiffness", quantityDisplay(component.modifiers?.linear_stiffness_user_value)],
        ["Rotational stiffness", quantityDisplay(component.modifiers?.rotational_stiffness_user_value)],
        ["Modifier source", component.modifiers?.source_reference ?? "TBD"],
        ["Completeness", componentCompletenessDisplay(component)]
      );
    }
    if (isExpansionJointComponent(component)) {
      rows.push(
        ["Mapped pipe", component.geometry?.expansion_joint_pipe_ref ?? "TBD"],
        ["Effective area", quantityDisplay(component.geometry?.effective_area)],
        ["Movement limit", quantityDisplay(component.geometry?.movement_limit)],
        ["Hardware reference", component.geometry?.hardware_reference ?? "TBD"],
        ["Manufacturer reference", component.geometry?.manufacturer_reference ?? "TBD"],
        ["Pressure thrust", component.geometry?.pressure_thrust_reference ?? "TBD"],
        ["Geometry source", component.geometry?.expansion_joint_source_reference ?? "TBD"],
        ["Solver consumption", component.mechanics_interface?.solver_consumption ?? "TBD"],
        ["Rule input consumption", component.mechanics_interface?.rule_check_consumption ?? "TBD"],
        ["Axial stiffness", quantityDisplay(component.modifiers?.axial_stiffness_user_value)],
        ["Lateral stiffness", quantityDisplay(component.modifiers?.lateral_stiffness_user_value)],
        ["Angular stiffness", quantityDisplay(component.modifiers?.angular_stiffness_user_value)],
        ["Torsional stiffness", quantityDisplay(component.modifiers?.torsional_stiffness_user_value)],
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
    return [
      ["ID", loadCase.id],
      ["Kind", loadCase.kind],
      ["Status", loadCase.status],
      ["Load records", primitiveLoads.length.toString()],
      ["Categories", primitiveLoadCategories(primitiveLoads)],
      ["Targets", primitiveLoadTargets(primitiveLoads)],
      ["Provenance", loadCase.provenance]
    ];
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

function quantityDisplay(quantity: { value: number; unit: string } | undefined): string {
  return quantity ? `${quantity.value} ${quantity.unit}` : "TBD";
}

function vectorQuantityDisplay(quantity: { x: number; y: number; z: number; unit: string } | undefined): string {
  return quantity ? `x=${quantity.x}, y=${quantity.y}, z=${quantity.z} ${quantity.unit}` : "TBD";
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
