import type { PreviewModel } from "../../types";
import type { OperationBatch } from "../../services/operationBatchService";
import { makeRichIntent } from "../rich-authoring/formSupport";
import { number, text } from "../geometry-tools/batchFormSupport";
export const DOFS = ["UX", "UY", "UZ", "RX", "RY", "RZ"] as const;
export type Dof = typeof DOFS[number];
export type BoundaryAssociation = {
  boundary_id: string;
  kind: "equipment" | "equipment_nozzle";
  equipment_reference: string;
  nozzle_reference?: string;
  coordinate_system: "global";
};
export type BoundaryDraft = {
  boundaryId: string; label: string; kind: "" | "equipment" | "equipment_nozzle";
  equipmentReference: string; nozzleReference: string; node: string; provenance: string; coordinateSystem: string;
  dofs: Record<Dof, { mode: "" | "free" | "rigid" | "spring"; value: string; unit: string }>;
};
export function emptyBoundaryDraft(): BoundaryDraft {
  return { boundaryId: "", label: "", kind: "", equipmentReference: "", nozzleReference: "", node: "", provenance: "", coordinateSystem: "",
    dofs: Object.fromEntries(DOFS.map(dof => [dof, { mode: "", value: "", unit: "" }])) as BoundaryDraft["dofs"] };
}
export type BoundaryMember = {
  id: string; label: string; node: string; provenance: string; family: "anchor" | "spring";
  restraints: Dof[]; stiffness?: { dof: Dof; value: { value: number; unit: string } };
  boundary_association: BoundaryAssociation;
};
export function boundaryMembers(model: PreviewModel, draft: BoundaryDraft): BoundaryMember[] {
  const boundaryId = text(draft.boundaryId, "Boundary ID");
  const label = text(draft.label, "Boundary label");
  if (draft.kind !== "equipment" && draft.kind !== "equipment_nozzle") throw new Error("Choose a boundary kind.");
  if (draft.coordinateSystem !== "global") throw new Error("Confirm the global coordinate system.");
  const association: BoundaryAssociation = { boundary_id: boundaryId, kind: draft.kind,
    equipment_reference: text(draft.equipmentReference, "Equipment reference"), coordinate_system: "global" };
  if (draft.kind === "equipment_nozzle") association.nozzle_reference = text(draft.nozzleReference, "Nozzle reference");
  const node = text(draft.node, "Boundary node");
  if (model.nodes.filter(n => n.id === node).length !== 1) throw new Error("Choose an existing unique boundary node.");
  const provenance = text(draft.provenance, "Boundary provenance");
  const supports = model.supports as Array<PreviewModel["supports"][number] & { boundary_association?: BoundaryAssociation }>;
  if (supports.some(s => s.boundary_association?.boundary_id === boundaryId)) throw new Error("Boundary ID already exists. Inspect the actual members; appending to an existing group is unsupported.");
  for (const dof of DOFS) if (!["free", "rigid", "spring"].includes(draft.dofs[dof].mode)) throw new Error(`Choose an explicit mode for ${dof}.`);
  const rigid = DOFS.filter(dof => draft.dofs[dof].mode === "rigid");
  const members: BoundaryMember[] = [];
  if (rigid.length) members.push({ id: `${boundaryId}:rigid`, label: `${label} / rigid ${rigid.join(",")}`, node, provenance,
    family: "anchor", restraints: rigid, boundary_association: association });
  for (const dof of DOFS) {
    const row = draft.dofs[dof];
    if (row.mode !== "spring") continue;
    const value = number(row.value, `${dof} stiffness`);
    if (value <= 0) throw new Error(`${dof} stiffness must be positive.`);
    members.push({ id: `${boundaryId}:spring:${dof}`, label: `${label} / spring ${dof}`, node, provenance,
      family: "spring", restraints: [dof], stiffness: { dof, value: { value, unit: text(row.unit, `${dof} stiffness unit`) } }, boundary_association: association });
  }
  if (!members.length) throw new Error("All DOFs are free. Choose at least one rigid or spring DOF to create a boundary.");
  if (members.some(m => supports.some(s => s.id === m.id))) throw new Error("A generated support ID already exists. Choose a new boundary ID.");
  return members;
}
export function buildBoundaryBatch(model: PreviewModel, draft: BoundaryDraft, id: string): OperationBatch {
  const members = boundaryMembers(model, draft);
  return { batch_id: id, operations: members.map((member, index) => {
    const intent = makeRichIntent({ object_type: "Support", ref: member.id }, "create_support", "supports", "not_present", member, member.label);
    intent.operation_id = `op:${id}:${index}`;
    intent.change.change_id = `change:${id}:${index}`;
    intent.source!.source_ref = "apps/desktop/src/features/boundary-authoring/BoundaryAuthoringPanel.tsx";
    intent.rationale = `User-authored boundary at ${member.boundary_association.equipment_reference}${member.boundary_association.nozzle_reference ? ` / nozzle ${member.boundary_association.nozzle_reference}` : ""}; global ${member.family} DOFs ${member.restraints.join(",")}.`;
    return intent;
  }) };
}
