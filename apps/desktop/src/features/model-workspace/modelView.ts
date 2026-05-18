import type { EntityRef, PreviewModel } from "../../types";

export function defaultSelection(model: PreviewModel): EntityRef {
  return { type: "project", id: model.project.id };
}

export function entityLabel(model: PreviewModel, id: string): string {
  const entity =
    model.nodes.find((item) => item.id === id) ??
    model.pipe_segments.find((item) => item.id === id) ??
    model.supports.find((item) => item.id === id) ??
    model.components.find((item) => item.id === id) ??
    model.load_cases.find((item) => item.id === id);
  return entity?.label ?? id;
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
    return [
      ["ID", support.id],
      ["Node", support.node],
      ["Restraints", support.restraints.join(", ")],
      ["Provenance", support.provenance]
    ];
  }
  const component = model.components.find((item) => item.id === selection.id);
  if (component) {
    return [
      ["ID", component.id],
      ["Kind", component.kind],
      ["Node", component.node],
      ["Provenance", component.provenance]
    ];
  }
  return [["Selection", selection.id]];
}
