// The Loads page's Wind and Seismic sections (DEC-068 item 2, on the DEL-07-06 presentation):
// the case's `equivalent_static` record, edited field by field with the operations the
// inspector already queues (`model-tree/PropertyInspector.tsx`, its load-case field rows, and
// the model tree's grid columns): `update_load` on `equivalent_static.wind.pressure.value`,
// `.shape_factor.value`, `.direction`, `.exposed_pipe_refs`, and
// `equivalent_static.seismic.gravity_acceleration.value`, `.g_factor_x|y|z.value`. The
// before-value is the stored value as the inspector shows it (`TBD` when absent); a quantity
// with its own unit is the explicit `{value, unit}` payload; a factor is the entered number
// in the stored unit (`none` when absent); the direction is its global-axis token; the exposed
// pipes are their ids, comma-separated. No value, unit or coefficient is defaulted.
import type { EditorOperationIntent, EquivalentStaticGenerationInput, PreviewModel, QuantityValue } from "../../types";

type LoadCase = PreviewModel["load_cases"][number];

export const WIND_DIRECTIONS = ["global_x", "global_y", "global_z"] as const;
export const WIND_DIRECTION_LABEL: Record<string, string> = { global_x: "+X", global_y: "+Y", global_z: "+Z" };

/** One field of the record, as the operation names it. */
export type StaticField = {
  label: string;
  fieldPath: string;
  /** "quantity": a value with its own unit; "factor": a number in the stored unit; "token"/"refs": text. */
  kind: "quantity" | "factor" | "token" | "refs";
  /** The operation's dimension for a quantity or factor. */
  dimension: string;
};

export const WIND_FIELDS = {
  pressure: { label: "Wind pressure", fieldPath: "equivalent_static.wind.pressure.value", kind: "quantity", dimension: "pressure" },
  shapeFactor: { label: "Shape factor", fieldPath: "equivalent_static.wind.shape_factor.value", kind: "factor", dimension: "dimensionless" },
  direction: { label: "Direction", fieldPath: "equivalent_static.wind.direction", kind: "token", dimension: "dimensionless" },
  exposedPipes: { label: "Exposed pipes", fieldPath: "equivalent_static.wind.exposed_pipe_refs", kind: "refs", dimension: "dimensionless" }
} as const satisfies Record<string, StaticField>;

export const SEISMIC_FIELDS = {
  gravity: { label: "Gravity acceleration", fieldPath: "equivalent_static.seismic.gravity_acceleration.value", kind: "quantity", dimension: "acceleration" },
  x: { label: "X factor", fieldPath: "equivalent_static.seismic.g_factor_x.value", kind: "factor", dimension: "dimensionless" },
  y: { label: "Y factor", fieldPath: "equivalent_static.seismic.g_factor_y.value", kind: "factor", dimension: "dimensionless" },
  z: { label: "Z factor", fieldPath: "equivalent_static.seismic.g_factor_z.value", kind: "factor", dimension: "dimensionless" }
} as const satisfies Record<string, StaticField>;

const isQuantity = (value: unknown): value is QuantityValue =>
  Boolean(value) && typeof value === "object" && typeof (value as QuantityValue).value === "number" && typeof (value as QuantityValue).unit === "string";

/** The stored value at a field path of the case (the record below `equivalent_static`). */
export function storedAt(loadCase: LoadCase, field: StaticField): unknown {
  const segments = field.fieldPath.split(".");
  let at: unknown = loadCase;
  for (const segment of segments) at = at && typeof at === "object" ? (at as Record<string, unknown>)[segment] : undefined;
  return at;
}

/** The stored quantity whose `.value` the field names, or undefined. */
export function storedQuantity(loadCase: LoadCase, field: StaticField): QuantityValue | undefined {
  const parent = storedAt(loadCase, { ...field, fieldPath: field.fieldPath.replace(/\.value$/, "") });
  return isQuantity(parent) ? parent : undefined;
}

/** The before-value the operation compares: the stored value as the inspector shows it. */
export function beforeText(loadCase: LoadCase, field: StaticField): string {
  if (field.kind === "quantity" || field.kind === "factor") {
    const quantity = storedQuantity(loadCase, field);
    return quantity ? String(quantity.value) : "TBD";
  }
  const stored = storedAt(loadCase, field);
  if (field.kind === "refs") return Array.isArray(stored) && stored.length ? stored.join(", ") : "TBD";
  return typeof stored === "string" ? stored : "TBD";
}

/** The entry a field's draft starts from. */
export function entryOf(loadCase: LoadCase, field: StaticField): { value: string; unit: string } {
  if (field.kind === "quantity" || field.kind === "factor") {
    const quantity = storedQuantity(loadCase, field);
    return { value: quantity ? String(quantity.value) : "", unit: quantity?.unit ?? "" };
  }
  const stored = storedAt(loadCase, field);
  if (field.kind === "refs") return { value: Array.isArray(stored) ? stored.join(", ") : "", unit: "" };
  return { value: typeof stored === "string" ? stored : "", unit: "" };
}

/** A finite number from entered text, or null. */
export function finiteNumber(raw: string): number | null {
  const text = raw.trim();
  if (!text) return null;
  const parsed = Number(text);
  return Number.isFinite(parsed) ? parsed : null;
}

/** The `update_load` intent for one field: the inspector's operation, with the entered value. */
export function staticFieldIntent(loadCase: LoadCase, field: StaticField, entry: { value: string; unit: string }): EditorOperationIntent {
  const token = `${safeToken(loadCase.id)}-${safeToken(field.fieldPath)}`;
  const storedUnit = storedQuantity(loadCase, field)?.unit;
  const unit = field.kind === "quantity" ? entry.unit.trim() : field.kind === "factor" ? storedUnit ?? "none" : "none";
  const after = field.kind === "quantity"
    ? JSON.stringify({ value: finiteNumber(entry.value) ?? entry.value.trim(), unit })
    : field.kind === "refs"
    ? entry.value.split(",").map((ref) => ref.trim()).filter(Boolean).join(", ")
    : entry.value.trim();
  return {
    operation_id: `op:loads-page-${token}`,
    operation_kind: "modify",
    operation_status: "proposed",
    author_type: "user",
    target: { object_type: "Load", ref: loadCase.id },
    change: {
      change_id: `change:loads-page-${token}`,
      change_kind: "update_load",
      field_label: field.label,
      field_path: field.fieldPath,
      before: beforeText(loadCase, field),
      after,
      unit,
      dimension: field.dimension,
      source_note: "User-entered static-equivalent generation input; no code coefficient or default."
    },
    validation: {
      schema_validation: "not_run",
      constraint_validation: "not_run",
      unit_validation: field.kind === "quantity" ? "not_run" : "not_required_dimensionless",
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
    rationale: `User-entered ${field.label.toLowerCase()} of load case ${loadCase.id}.`
  };
}

/** True when the entry differs from what the case stores. */
export function entryChanged(loadCase: LoadCase, field: StaticField, entry: { value: string; unit: string }): boolean {
  const stored = entryOf(loadCase, field);
  // An absent quantity with no value entered is unchanged, whatever unit stands ready beside it.
  if (field.kind === "quantity") return entry.value.trim() || stored.value ? entry.value.trim() !== stored.value || entry.unit.trim() !== stored.unit : false;
  if (field.kind === "factor") return entry.value.trim() !== stored.value;
  if (field.kind === "refs") return normalizedRefs(entry.value) !== normalizedRefs(stored.value);
  return entry.value.trim() !== stored.value;
}

const normalizedRefs = (text: string) => text.split(",").map((ref) => ref.trim()).filter(Boolean).join(",");

function safeToken(value: string): string {
  return value.replace(/[^A-Za-z0-9_.:-]+/g, "-");
}

const quantityText = (quantity: QuantityValue | undefined) => (quantity ? `${quantity.value} ${quantity.unit}` : null);

/** The Wind section's closed line: "none", or what the case's wind record holds. */
export function windSummary(record: EquivalentStaticGenerationInput | undefined): string {
  const wind = record?.wind;
  if (!wind) return "none";
  const pipes = wind.exposed_pipe_refs?.length ?? 0;
  const spans = wind.exposed_spans?.length ?? 0;
  const parts = [
    quantityText(wind.pressure),
    wind.direction ? WIND_DIRECTION_LABEL[wind.direction] ?? wind.direction : null,
    pipes ? `${pipes} ${pipes === 1 ? "pipe" : "pipes"}` : null,
    spans ? `${spans} partial ${spans === 1 ? "span" : "spans"}` : null
  ].filter(Boolean);
  return parts.length ? parts.join(", ") : "record with no values";
}

/** The Seismic section's closed line: "none", or the case's factors ("0.3 g X, 0.3 g Y"). */
export function seismicSummary(record: EquivalentStaticGenerationInput | undefined): string {
  const seismic = record?.seismic;
  if (!seismic) return "none";
  const factors = (["x", "y", "z"] as const)
    .map((axis) => [axis, seismic[`g_factor_${axis}`]] as const)
    .filter(([, factor]) => factor !== undefined)
    .map(([axis, factor]) => `${factor!.value} g ${axis.toUpperCase()}`);
  if (factors.length) return factors.join(", ");
  return seismic.gravity_acceleration ? `gravity ${quantityText(seismic.gravity_acceleration)}, no factors` : "record with no values";
}
