import { coordinateError, type TableColumn } from "./tableState";
import type { EditorOperationIntent, EditorOperationObjectType, EntityRef, PreviewModel } from "../../../types";

export type GridColumn = {
  key: string;
  label: string;
  fieldPath: string;
  objectType: EditorOperationObjectType;
  changeKind: EditorOperationIntent["change"]["change_kind"];
  dimension: string;
  sourceNote: string;
  unit: (row: GridRow) => string;
  value: (row: GridRow) => string;
  unitEditable?: boolean;
  quantity?: boolean;
  readonly?: boolean;
  options?: readonly string[];
};

export type GridRow = {
  id: string;
  label: string;
  type: EntityRef["type"];
  searchText: string;
  raw: unknown;
};

export function buildGridOperationIntent({
  column,
  model,
  row,
  sequence,
  value,
  interaction = "review"
}: {
  column: GridColumn;
  model: PreviewModel;
  row: GridRow;
  sequence: number;
  value: string;
  interaction?: "review" | "cell";
}): EditorOperationIntent {
  const operationToken = `${safeToken(row.id)}-${safeToken(column.fieldPath)}-${sequence.toString().padStart(2, "0")}`;
  const unit = column.unit(row);
  const after =
    column.dimension === "dimensionless"
      ? value.trim() || "TBD"
      : JSON.stringify({ value: parseQuantityPayloadValue(value), unit });

  return {
    operation_id: `op:grid-intent-${operationToken}`,
    operation_kind: "modify",
    operation_status: "proposed",
    author_type: "user",
    source: {
      source_ref: `grid:${model.project.id}:${row.id}`,
      source_channel: "local_desktop_preview",
      source_role: "gui_editor"
    },
    target: {
      object_type: column.objectType,
      ref: row.id
    },
    change: {
      change_id: `change:grid:${operationToken}`,
      change_kind: column.changeKind,
      field_label: column.label,
      field_path: column.fieldPath,
      before: column.value(row),
      after,
      unit,
      dimension: column.dimension,
      source_note: `${interaction === "cell" ? "layout_table_cell" : "layout_grid_bulk_tabular"}; ${column.sourceNote}`
    },
    validation: {
      schema_validation: "not_run",
      constraint_validation: "not_run",
      unit_validation:
        column.dimension === "dimensionless" ? "not_required_dimensionless" : column.objectType === "Material" ? "not_run" : "model_metadata_unit_dimension_declared",
      diff_preview_status: "not_generated",
      application_status: "not_applied"
    },
    audit_boundary: {
      mutation_route: "structured_operations_only",
      direct_model_mutation_allowed: false,
      requires_user_acceptance: true,
      mutates_accepted_model_state: false
    },
    professional_boundary: {
      human_review_required: true,
      software_makes_compliance_claim: false,
      software_makes_certification_claim: false,
      software_makes_sealing_claim: false,
      software_makes_approval_claim: false,
      software_makes_authentication_claim: false
    },
    rationale: interaction === "cell" ? "layout_table_cell_apply" : "layout_grid_bulk_tabular_review_change"
  };
}

function parseQuantityPayloadValue(raw: string): number | string {
  const trimmed = raw.trim();
  if (!trimmed) return "TBD";
  const parsed = Number(trimmed);
  return Number.isFinite(parsed) ? parsed : trimmed;
}

function safeToken(value: string): string {
  return value.replace(/[^a-zA-Z0-9:_-]+/g, "-").replace(/^-+|-+$/g, "") || "entity";
}


/** Entered-coordinate grammar belongs to this adapter, not the interaction core. */
export function nodeCoordinateColumns(unit: string): TableColumn[] {
  return ["x", "y", "z"].map((key) => ({ key, kind: "quantity", label: key.toUpperCase(), unit: unit || "unit missing",
    validate: (text) => unit.trim() ? coordinateError(text) : "The model has no declared length unit. Direct coordinate Apply is unavailable.",
    equivalent: (before, after) => Number(before) === Number(after),
    compare: (a, b) => Number(a) - Number(b)
  }));
}

/** Text is dimensionless; only the replacement is normalized by the engine. */
export function nodeTableColumns(unit: string, review = false): TableColumn[] {
  const textColumn = (key: string, label: string): TableColumn => ({ key, label, unit: "", kind: "text",
    validate: review ? undefined : (text) => text.trim() ? undefined : "Enter text or explicitly enter TBD.",
    equivalent: review ? undefined : (before, after) => before === after.trim()
  });
  return [textColumn("label", "Label"), ...nodeCoordinateColumns(unit).map((column) => review
    ? { ...column, validate: undefined, equivalent: undefined } : column), textColumn("provenance", "Provenance")];
}

/** Materials edit existing quantities in their actual sibling unit. */
export function materialTableColumns(review = false): TableColumn[] {
  const text = (key: string, label: string): TableColumn => ({ key, label, unit: "", kind: "text",
    validate: review ? undefined : (value) => value.trim() ? undefined : "Enter text or explicitly enter TBD.",
    equivalent: review ? undefined : (before, after) => before === after.trim()
  });
  return [text("label", "Label"), ...["elastic", "shear", "thermal"].map((key): TableColumn => ({
    key, label: key === "elastic" ? "Elastic" : key === "shear" ? "Shear" : "Thermal", unit: "per-row entered unit", kind: "quantity",
    projectedSort: true, minWidth: 180,
    validate: review ? undefined : (value) => coordinateError(value) ?? (key !== "thermal" && Number(value) <= 0 ? "Enter a finite number greater than zero in the entered unit." : undefined),
    equivalent: review ? undefined : (before, after) => Number(before) === Number(after)
  })), text("provenance", "Provenance")];
}

/** Section geometry stays engine-validated, including coupled OD/wall constraints. */
export function sectionTableColumns(review = false): TableColumn[] {
  const text = (key: string, label: string): TableColumn => ({ key, label, unit: "", kind: "text",
    validate: review ? undefined : (value) => value.trim() ? undefined : "Enter text or explicitly enter TBD.",
    equivalent: review ? undefined : (before, after) => before === after.trim()
  });
  return [text("name", "Name"), { ...text("type", "Type"), options: ["pipe"],
    validate: (value) => value.trim() === "pipe" ? undefined : "Choose or enter pipe." },
    ...["outside", "wall"].map((key): TableColumn => ({ key, label: key === "outside" ? "Outside dia." : "Wall",
      unit: "per-row entered unit", kind: "quantity", projectedSort: true, minWidth: 180,
      validate: review ? undefined : coordinateError,
      equivalent: review ? undefined : (before, after) => Number(before) === Number(after)
    })), text("provenance", "Provenance")];
}
