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

function isBendComponent(component: PreviewComponent): boolean {
  return component.kind === "bend" || component.kind === "elbow";
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
