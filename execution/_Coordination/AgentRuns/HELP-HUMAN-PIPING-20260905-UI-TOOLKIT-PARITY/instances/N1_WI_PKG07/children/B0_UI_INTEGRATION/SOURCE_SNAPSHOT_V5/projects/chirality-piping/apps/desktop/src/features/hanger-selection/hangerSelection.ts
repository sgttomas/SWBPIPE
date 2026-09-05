import { canonicalJsonString } from "../../services/hashService";
import type { PreviewModel } from "../../types";
import type { OperationBatch } from "../../services/operationBatchService";
import { draftIntent } from "../offline-proposal-intake/workflowSupport";

type Provenance = Record<string, unknown>;
type ImportedQuantity = { magnitude: number; unit: string; dimension: string; provenance: Provenance };
export type HangerRecord = {
  hanger_id: string; name: string; provenance: Provenance;
  hanger: {
    hanger_type: "variable_spring_hanger" | "constant_effort_support";
    source_reference: string; manufacturer_reference?: string; load_side_review_reference?: string; mechanics_consumption?: string;
    stiffness?: { dof: string; value: ImportedQuantity };
    installed_load?: ImportedQuantity; cold_load?: ImportedQuantity; hot_load?: ImportedQuantity;
    constant_load?: ImportedQuantity; travel_range?: ImportedQuantity; movement_limit?: ImportedQuantity;
  };
};
export type HangerDocument = {
  schema_version: string;
  hanger_library: { library_id: string; name: string; provenance: Provenance; [key: string]: unknown };
  hanger_records: HangerRecord[];
};
export type HangerTarget = { supportId: string; restraints: string[]; newSupport?: { label: string; node: string } };
// This maps a record AFTER native import validation. It is not a parallel library validator.
export async function buildHangerSelectionBatch(model: PreviewModel, document: HangerDocument, recordId: string, target: HangerTarget): Promise<OperationBatch> {
  const selected = document.hanger_records.find(record => record.hanger_id === recordId);
  if (!selected) throw new Error("The selected hanger record is missing. Refresh the library.");
  if (!target.supportId.trim()) throw new Error("Select or enter a support ID.");
  if (!target.restraints.length || target.restraints.some(dof => !["UX", "UY", "UZ"].includes(dof)))
    throw new Error("Explicitly select the translational restraint directions for this hanger.");
  const old = model.supports.find(support => support.id === target.supportId);
  if (!target.newSupport && !old) throw new Error("The target support no longer exists.");
  if (target.newSupport && (!target.newSupport.label.trim() || !model.nodes.some(node => node.id === target.newSupport?.node)))
    throw new Error("A new support needs a label and an existing node.");
  const source = await canonicalJsonString({
    kind: "user_imported_hanger_selection", library_kind: "hanger",
    library_id: document.hanger_library.library_id, hanger_id: selected.hanger_id,
    library_metadata: document.hanger_library, record_snapshot: selected
  });
  const imported = selected.hanger;
  const hanger: Record<string, unknown> = { hanger_type: imported.hanger_type, source_reference: imported.source_reference };
  for (const key of ["manufacturer_reference", "load_side_review_reference", "mechanics_consumption"] as const)
    if (imported[key] !== undefined) hanger[key] = imported[key];
  for (const key of ["installed_load", "cold_load", "hot_load", "constant_load", "travel_range", "movement_limit"] as const) {
    const q = imported[key];
    if (q !== undefined) hanger[key] = { value: q.magnitude, unit: q.unit };
  }
  if (imported.stiffness !== undefined) hanger.stiffness = {
    dof: imported.stiffness.dof,
    value: { value: imported.stiffness.value.magnitude, unit: imported.stiffness.value.unit }
  };
  const configuration = { family: imported.hanger_type, restraints: [...target.restraints], hanger, provenance: source };
  const after = target.newSupport ? { id: target.supportId, ...target.newSupport, ...configuration } : configuration;
  const before = old ? Object.fromEntries(["family", "restraints", "stiffness", "hanger", "nonlinear", "provenance"]
    .filter(key => (old as unknown as Record<string, unknown>)[key] !== undefined)
    .map(key => [key, (old as unknown as Record<string, unknown>)[key]])) : undefined;
  return { batch_id: `batch:hanger:${crypto.randomUUID()}`, operations: [draftIntent({
    operation_kind: target.newSupport ? "create" : "modify",
    target: { object_type: "Support", ref: target.supportId },
    change: {
      change_id: `change:${crypto.randomUUID()}`, change_kind: target.newSupport ? "create_support" : "update_support",
      field_label: "Imported hanger selection", field_path: target.newSupport ? "supports" : "configuration",
      before: target.newSupport ? "not_present" : await canonicalJsonString(before),
      after: await canonicalJsonString(after), unit: "none", dimension: "dimensionless", source_note: source
    },
    rationale: "Manual user selection of an imported hanger record; replaces the support configuration and clears previous top-level stiffness and nonlinear settings."
  })] };
}
