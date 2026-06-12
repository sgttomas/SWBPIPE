import type { MechanicsResult, ObjectRef, PreviewModel } from "../types";

export type ExportUnitSystemDisclosure = {
  unit_system_ref: ObjectRef;
  source_model_ref: ObjectRef;
  storage_convention: "entered_units_preserved";
  model_units: Record<string, string>;
  result_units: string[];
  target_export_units: Record<string, string>;
  conversion_policy: string;
  conversion_performed: boolean;
  conversion_scope: string[];
  decision_basis_refs: ObjectRef[];
  protected_content_included: false;
  private_payload_included: false;
  provenance: {
    source_name: string;
    source_location: string;
    source_license: string;
    contributor: string;
    contributor_certification: string;
    redistribution_status: string;
    review_status: string;
    privacy_classification: string;
  };
};

export function buildExportUnitSystemDisclosure({
  model,
  result,
  targetExportUnits = {},
  conversionPolicy,
  conversionPerformed,
  conversionScope,
  sourceLocation
}: {
  model: PreviewModel;
  result?: MechanicsResult | null;
  targetExportUnits?: Record<string, string>;
  conversionPolicy: string;
  conversionPerformed: boolean;
  conversionScope: string[];
  sourceLocation: string;
}): ExportUnitSystemDisclosure {
  return {
    unit_system_ref: reference("UnitSystem", "unit-system:dec-018-si-dual-display"),
    source_model_ref: reference("Model", model.project.id),
    storage_convention: "entered_units_preserved",
    model_units: sortedStringRecord(model.project.units),
    result_units: Array.from(new Set((result?.results ?? []).map((item) => item.unit).filter(Boolean))).sort(),
    target_export_units: sortedStringRecord(targetExportUnits),
    conversion_policy: conversionPolicy,
    conversion_performed: conversionPerformed,
    conversion_scope: conversionScope.slice().sort(),
    decision_basis_refs: [reference("Decision", "DEC-018"), reference("Deliverable", "DEL-02-02")],
    protected_content_included: false,
    private_payload_included: false,
    provenance: {
      source_name: "OpenPipeStress desktop export unit disclosure",
      source_location: sourceLocation,
      source_license: "project-governed",
      contributor: "OpenPipeStress app integration tranche",
      contributor_certification: "DEC-018 unit metadata disclosure only; no protected standards or private payloads.",
      redistribution_status: "public_permissive",
      review_status: "desktop_preview",
      privacy_classification: "public_metadata"
    }
  };
}

export function unitDisclosureSummary(disclosure: ExportUnitSystemDisclosure): string {
  const resultUnits = disclosure.result_units.length > 0 ? disclosure.result_units.join(",") : "none";
  return `source=${formatUnitRecord(disclosure.model_units)}; target=${formatUnitRecord(
    disclosure.target_export_units
  )}; results=${resultUnits}; conversion=${String(disclosure.conversion_performed)}`;
}

export function formatUnitRecord(units: Record<string, string>): string {
  const entries = Object.entries(units).sort(([left], [right]) => left.localeCompare(right));
  if (entries.length === 0) return "none";
  return entries.map(([key, value]) => `${key}=${value}`).join(",");
}

function sortedStringRecord(record: Record<string, string>): Record<string, string> {
  return Object.fromEntries(
    Object.entries(record)
      .filter(([, value]) => typeof value === "string" && value.length > 0)
      .sort(([left], [right]) => left.localeCompare(right))
  );
}

function reference(objectType: string, ref: string): ObjectRef {
  return { object_type: objectType, ref };
}
